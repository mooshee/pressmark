// Author: Daniel Hallman
// Checkout intent and settlement regressions use local fixtures only.
use axum::{body::{to_bytes, Body}, http::{Request, StatusCode}};
use pressmark::{server::{create_router, AppState}, storage::{CheckoutStart, Storage}};
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn checkout_get_and_head_never_record_or_redirect() {
    let storage = Arc::new(Storage::new(":memory:").unwrap());
    let router = create_router(Arc::new(AppState::default().with_storage(storage.clone())));
    for method in ["GET", "HEAD"] {
        for agent in ["Mozilla/5.0", "ClaudeBot/1.0", "curl/8.0"] {
            let request = Request::builder().method(method)
                .uri("/api/v1/checkout?plan=plan_solo&redirect=true&email=qa%40example.com")
                .header("user-agent", agent).body(Body::empty()).unwrap();
            let response = router.clone().oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert!(response.headers().get("location").is_none());
            assert_eq!(response.headers()["cache-control"], "no-store");
            let bytes = to_bytes(response.into_body(), 16384).await.unwrap();
            if method == "GET" {
                let html = String::from_utf8(bytes.to_vec()).unwrap();
                assert!(html.contains("method=\"post\""));
                assert!(html.contains("Continue to payment"));
                assert!(html.contains("$15 USD per month"));
                assert!(!html.contains("buy.stripe.com"));
            }
        }
    }
    assert_eq!(storage.analytics_summary()["events_by_type"]["checkout_init"].as_u64().unwrap_or(0), 0);
}

#[tokio::test]
async fn checkout_post_reuses_key_and_rejects_changed_request() {
    let storage = Arc::new(Storage::new(":memory:").unwrap());
    let router = create_router(Arc::new(AppState::default().with_storage(storage.clone())));
    let preview = router.clone().oneshot(Request::builder()
        .uri("/api/v1/checkout?plan=plan_solo").body(Body::empty()).unwrap()).await.unwrap();
    let body = to_bytes(preview.into_body(), 16384).await.unwrap();
    let preview: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let key = preview["request_id"].as_str().unwrap();
    assert!(uuid::Uuid::parse_str(key).is_ok());
    for plan in ["plan_solo", "plan_solo", "plan_business"] {
        let request = Request::builder().method("POST").uri("/api/v1/checkout")
            .header("content-type", "application/json").header("user-agent", "Mozilla/5.0")
            .body(Body::from(serde_json::json!({"plan":plan,"request_id":key}).to_string())).unwrap();
        let response = router.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), if plan == "plan_business" { StatusCode::CONFLICT } else { StatusCode::OK });
    }
    assert_eq!(storage.analytics_summary()["events_by_type"]["checkout_init"], 1);
    let missing_key = router.clone().oneshot(Request::builder().method("POST").uri("/api/v1/checkout")
        .header("content-type", "application/json").body(Body::from("{\"plan\":\"plan_solo\"}")).unwrap()).await.unwrap();
    assert_eq!(missing_key.status(), StatusCode::BAD_REQUEST);
    let form = format!("plan_id=plan_solo&request_id={key}&email=&client_reference_id=");
    let response = router.oneshot(Request::builder().method("POST").uri("/api/v1/checkout/continue")
        .header("origin", "http://127.0.0.1:3842")
        .header("content-type", "application/x-www-form-urlencoded").header("user-agent", "Mozilla/5.0")
        .body(Body::from(form)).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert!(response.headers()["location"].to_str().unwrap().starts_with("https://buy.stripe.com/"));
    assert_eq!(storage.analytics_summary()["events_by_type"]["checkout_init"], 1);
}

#[tokio::test]
async fn review_form_roundtrip_preserves_empty_fields_and_blocks_cross_origin() {
    let storage = Arc::new(Storage::new(":memory:").unwrap());
    let router = create_router(Arc::new(AppState::default().with_storage(storage.clone())));
    let response = router.clone().oneshot(Request::builder()
        .uri("/api/v1/checkout?plan=plan_solo&redirect=true")
        .body(Body::empty()).unwrap()).await.unwrap();
    let html = String::from_utf8(to_bytes(response.into_body(), 16384).await.unwrap().to_vec()).unwrap();
    let fields = regex::Regex::new(r#"<input type="hidden" name="([a-z_]+)" value="([^"]*)">"#).unwrap();
    let mut encoded = reqwest::Url::parse("https://fixture.invalid/").unwrap();
    for field in fields.captures_iter(&html) {
        encoded.query_pairs_mut().append_pair(&field[1], &field[2]);
    }
    let form = encoded.query().unwrap();
    assert!(form.contains("email=&" ) || form.ends_with("email="));
    assert!(form.contains("client_reference_id="));
    for origin in ["https://unrelated.invalid", "http://127.0.0.1:3842", "http://127.0.0.1:3842"] {
        let response = router.clone().oneshot(Request::builder().method("POST")
            .uri("/api/v1/checkout/continue").header("origin", origin)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(form.to_string())).unwrap()).await.unwrap();
        assert_eq!(response.status(), if origin.contains("unrelated") { StatusCode::FORBIDDEN } else { StatusCode::SEE_OTHER });
    }
    assert_eq!(storage.analytics_summary()["events_by_type"]["checkout_init"], 1);
}

#[test]
fn checkout_key_is_atomic_and_survives_reopening_storage() {
    let path = std::env::temp_dir().join(format!("pressmark-checkout-{}.sqlite3", uuid::Uuid::new_v4()));
    let storage = Storage::new(&path).unwrap();
    assert_eq!(storage.record_checkout_start("fixture-key", "same-payload", "plan_solo", "stripe_probe").unwrap(), CheckoutStart::Created);
    drop(storage);
    let storage = Arc::new(Storage::new(&path).unwrap());
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8).map(|_| {
            let storage = storage.clone();
            scope.spawn(move || storage.record_checkout_start("fixture-key", "same-payload", "plan_solo", "stripe_probe").unwrap())
        }).collect();
        for handle in handles { assert_eq!(handle.join().unwrap(), CheckoutStart::Replayed); }
    });
    assert_eq!(storage.analytics_summary()["events_by_type"]["checkout_init"], 1);
    drop(storage);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn checkout_fulfillment_requires_paid_status_and_handles_async_success() {
    use pressmark::stripe::{process_stripe_event, FulfillmentAction, StripeEvent};
    for event_type in ["checkout.session.completed", "checkout.session.async_payment_succeeded", "checkout.session.expired", "checkout.session.async_payment_failed"] {
        for payment_status in [Some("paid"), Some("unpaid"), Some("no_payment_required"), None] {
            let event: StripeEvent = serde_json::from_value(serde_json::json!({
                "id":"evt_fixture", "type":event_type, "data":{"object":{
                    "id":"cs_test_fixture", "amount_total":1500,
                    "customer_details":{"email":"qa@example.com"}, "payment_status":payment_status
                }}
            })).unwrap();
            let action = process_stripe_event(event).unwrap();
            let should_fulfill = payment_status == Some("paid") && matches!(event_type, "checkout.session.completed" | "checkout.session.async_payment_succeeded");
            assert_eq!(matches!(action, FulfillmentAction::CheckoutCompleted { .. }), should_fulfill, "{event_type} / {payment_status:?}");
        }
    }
}
