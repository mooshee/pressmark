// Author: Daniel Hallman
// Regression tests for Pressmark v1.4.0 growth pages, share links, and security hardening.

use axum::body::{to_bytes, Body};
use axum::http::{header, Request, StatusCode};
use hmac::{Hmac, Mac};
use pressmark::auth::ChatGptAuthManager;
use pressmark::ghl::is_sample_or_fixture_email;
use pressmark::rate_limit::RateLimiter;
use pressmark::server::{create_router, AppState};
use pressmark::storage::Storage;
use pressmark::stripe::{process_stripe_event, FulfillmentAction, StripeEvent, StripeEventData, StripeIdempotencyStore};
use serde_json::{json, Value};
use sha2::Sha256;
use std::sync::Arc;
use tower::ServiceExt;

fn build_app(allow_demo_oauth: bool) -> (axum::Router, Arc<Storage>, ChatGptAuthManager) {
    let storage = Arc::new(Storage::new(":memory:").expect("In-memory storage failed"));
    let auth_mgr = ChatGptAuthManager::in_memory()
        .with_storage(Arc::clone(&storage))
        .with_demo_oauth(allow_demo_oauth);
    let state = Arc::new(AppState {
        base_url: "https://pressmark.dev".to_string(),
        downloads: Default::default(),
        feedback: None,
        auth_mgr: auth_mgr.clone(),
        rate_limiter: RateLimiter::new(1000, 60),
        stripe_idempotency: StripeIdempotencyStore::new().with_storage(Arc::clone(&storage)),
        storage: Arc::clone(&storage),
    });
    (create_router(state), storage, auth_mgr)
}

#[tokio::test]
async fn test_demo_oauth_disabled_vision_unavailable_and_copilot_422() {
    let (app, _storage, _auth_mgr) = build_app(false);

    let res_demo = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/chatgpt/login?demo=plus")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res_demo.status(), StatusCode::FORBIDDEN);
    let demo_json: Value =
        serde_json::from_slice(&to_bytes(res_demo.into_body(), 65536).await.unwrap()).unwrap();
    assert_eq!(demo_json["error"], "demo_auth_disabled");

    let res_cb = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/chatgpt/callback?code=fake&demo=plus")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res_cb.status(), StatusCode::FORBIDDEN);

    let res_vision = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/vision-extract")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "image_data": "data:image/png;base64,AA==" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res_vision.status(), StatusCode::SERVICE_UNAVAILABLE);
    let vision_text = String::from_utf8(to_bytes(res_vision.into_body(), 65536).await.unwrap().to_vec()).unwrap();
    let vision_json: Value = serde_json::from_str(&vision_text).unwrap();
    assert_eq!(vision_json["success"], false);
    assert_eq!(vision_json["error"], "vision_not_configured");
    assert!(!vision_text.contains("Scanned Vendor LLC"));

    let res_copilot = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/copilot")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "prompt": "xyzzy unrecognized prompt with no fields",
                        "current_state": {
                            "doc_type": "invoice",
                            "from": { "name": "Issuer LLC" },
                            "to": { "name": "Client LLC" },
                            "items": [{ "description": "Consulting", "quantity": 1.0, "unit_price": 500.0 }]
                        }
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res_copilot.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_stripe_missing_customer_identity_and_export_credit_fulfillment() {
    let ev = StripeEvent {
        id: "evt_missing_identity".to_string(),
        object: "event".to_string(),
        r#type: "checkout.session.completed".to_string(),
        data: StripeEventData {
            object: json!({
                "id": "cs_test_no_email",
                "amount_total": 299,
                "currency": "usd",
                "payment_status": "paid",
                "customer_details": {},
                "metadata": {}
            }),
        },
    };
    let action = process_stripe_event(ev).unwrap();
    assert!(matches!(
        action,
        FulfillmentAction::MissingCustomerIdentity { .. }
    ));

    std::env::set_var("STRIPE_WEBHOOK_SECRET", "whsec_v140_credit_secret");
    let (app, storage, _auth_mgr) = build_app(false);

    // Log in via email as an unentitled free user
    let login_res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/email/login")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "email": "credit.buyer@harborview.co", "name": "Credit Buyer" })
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(login_res.status(), StatusCode::OK);
    let login_json: Value =
        serde_json::from_slice(&to_bytes(login_res.into_body(), 65536).await.unwrap()).unwrap();
    let session_id = login_json["session_id"].as_str().unwrap().to_string();
    assert_eq!(login_json["user"]["plan"], "free");
    assert_eq!(login_json["export_credits"], 0);

    // Signed $2.99 one-time checkout webhook grants 1 export credit
    let payload = json!({
        "id": "evt_credit_grant_299",
        "object": "event",
        "type": "checkout.session.completed",
        "data": {
            "object": {
                "id": "cs_credit_299",
                "amount_total": 299,
                "currency": "usd",
                "payment_status": "paid",
                "customer_details": { "email": "credit.buyer@harborview.co" },
                "metadata": {}
            }
        }
    })
    .to_string();

    type HmacSha256 = Hmac<Sha256>;
    let now = chrono::Utc::now().timestamp();
    let mut mac = HmacSha256::new_from_slice(b"whsec_v140_credit_secret").unwrap();
    mac.update(format!("{now}.{payload}").as_bytes());
    let sig_header = format!("t={now},v1={}", hex::encode(mac.finalize().into_bytes()));

    let wh_res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/webhooks/stripe")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/json")
                .header("stripe-signature", sig_header)
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(wh_res.status(), StatusCode::OK);
    assert_eq!(storage.get_export_credits("credit.buyer@harborview.co"), 1);

    let doc_body = json!({
        "doc_type": "invoice",
        "doc_number": "INV-CREDIT-01",
        "from": { "name": "Harborview LLC" },
        "to": { "name": "Client Co" },
        "items": [{ "description": "Design Audit", "quantity": 1.0, "unit_price": 450.0 }]
    })
    .to_string();

    // First binary PDF export consumes the single credit (unlocked, no upgrade header)
    let gen1 = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/generate?format=pdf")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, format!("Bearer {session_id}"))
                .body(Body::from(doc_body.clone()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(gen1.status(), StatusCode::OK);
    assert_eq!(
        gen1.headers()
            .get("x-chatgpt-subscriber")
            .and_then(|v| v.to_str().ok()),
        Some("true")
    );
    assert!(gen1.headers().get("x-upgrade-checkout-url").is_none());
    assert_eq!(storage.get_export_credits("credit.buyer@harborview.co"), 0);

    // Second binary PDF export falls back to free tier (upgrade header present)
    let gen2 = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/generate?format=pdf")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, format!("Bearer {session_id}"))
                .body(Body::from(doc_body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(gen2.status(), StatusCode::OK);
    assert!(gen2.headers().get("x-chatgpt-subscriber").is_none());
    assert!(gen2.headers().get("x-upgrade-checkout-url").is_some());
}

#[test]
fn test_ghl_sample_and_fixture_email_filter() {
    for sample in [
        "billing@pressmark.dev",
        "ap@vancedynamics.com",
        "billing@scanned-vendor.com",
        "demo@example.org",
    ] {
        assert!(
            is_sample_or_fixture_email(sample),
            "Expected sample email to be filtered: {sample}"
        );
    }
    for real in ["integration-test@pressmark.dev", "owner@harborview.co"] {
        assert!(
            !is_sample_or_fixture_email(real),
            "Expected non-sample email to pass: {real}"
        );
    }
}

#[tokio::test]
async fn test_public_growth_pages_calculators_and_share_lifecycle() {
    let (app, _storage, _auth_mgr) = build_app(false);

    // 1. Home Studio branding/payload check & empty default payment link
    let home_res = app
        .clone()
        .oneshot(Request::builder().uri("/").method("GET").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(home_res.status(), StatusCode::OK);
    let home_html =
        String::from_utf8(to_bytes(home_res.into_body(), usize::MAX).await.unwrap().to_vec())
            .unwrap();
    assert!(home_html.contains("/assets/icon.svg"));
    assert!(!home_html.contains("/assets/logo.jpg"));
    assert!(home_html.contains("id=\"paymentLinkInput\""));
    assert!(!home_html.contains("buy.stripe.com/eVq28q60a1pX4Jsc1a5J600"));

    // 2. Server-rendered catalog, pricing, docs, and calculator routes
    for path in [
        "/templates",
        "/pricing",
        "/docs",
        "/calculators",
        "/calculators/security-deposit-return",
        "/calculators/rent-increase-notice",
        "/calculators/commercial-nnn-lease",
        "/calculators/late-fee-invoice",
    ] {
        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK, "Expected 200 OK on {path}");
    }

    // 3. Shareable recipient view, preview PNG, e-sign, and PDF download
    let share_res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/share")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "doc_type": "invoice",
                        "doc_number": "INV-SHARE-99",
                        "from": { "name": "Harborview Studio LLC" },
                        "to": { "name": "Jordan Avery" },
                        "items": [{ "description": "Architecture Retainer", "quantity": 2.0, "unit_price": 600.0 }]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(share_res.status(), StatusCode::OK);
    let share_json: Value =
        serde_json::from_slice(&to_bytes(share_res.into_body(), 65536).await.unwrap()).unwrap();
    assert_eq!(share_json["success"], true);
    let token = share_json["token"].as_str().unwrap().to_string();
    let initial_sha = share_json["sha256_hex"].as_str().unwrap().to_string();

    let view_res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/v/{token}"))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(view_res.status(), StatusCode::OK);
    assert_eq!(
        view_res.headers().get("x-robots-tag").and_then(|v| v.to_str().ok()),
        Some("noindex, nofollow, noarchive")
    );

    let png_res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/v/{token}/preview.png"))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(png_res.status(), StatusCode::OK);
    let png_bytes = to_bytes(png_res.into_body(), usize::MAX).await.unwrap();
    assert!(png_bytes.starts_with(b"\x89PNG"));

    let sign_res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/v/{token}/sign"))
                .method("POST")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "signer_name": "Jordan Avery",
                        "signer_title": "Authorized Signatory"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(sign_res.status(), StatusCode::OK);
    let sign_json: Value =
        serde_json::from_slice(&to_bytes(sign_res.into_body(), 65536).await.unwrap()).unwrap();
    assert_eq!(sign_json["success"], true);
    assert_eq!(sign_json["signed_by"], "Jordan Avery");
    assert_ne!(sign_json["sha256_hex"].as_str().unwrap(), initial_sha);

    let pdf_res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/v/{token}/pdf"))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(pdf_res.status(), StatusCode::OK);
    let pdf_bytes = to_bytes(pdf_res.into_body(), usize::MAX).await.unwrap();
    assert!(pdf_bytes.starts_with(b"%PDF-"));
}
