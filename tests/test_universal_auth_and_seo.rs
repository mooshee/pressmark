//! Pressmark Universal Authentication & SEO Integration Test Suite
//!
//! Author: Daniel Hallman

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use pressmark::auth::ChatGptAuthManager;
use pressmark::rate_limit::RateLimiter;
use pressmark::server::{create_router, AppState};
use pressmark::storage::Storage;
use pressmark::stripe::StripeIdempotencyStore;
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

fn setup_test_app() -> axum::Router {
    let storage = Arc::new(Storage::new(":memory:").expect("In-memory storage failed"));
    let auth_mgr = ChatGptAuthManager::in_memory().with_storage(Arc::clone(&storage));
    let rate_limiter = RateLimiter::new(1000, 60);
    let stripe_idempotency = StripeIdempotencyStore::new();

    let state = Arc::new(AppState {
        base_url: "https://pressmark.dev".to_string(),
        auth_mgr,
        rate_limiter,
        stripe_idempotency,
        storage,
    });

    create_router(state)
}

#[tokio::test]
async fn test_seo_robots_and_sitemap_endpoints() {
    let app = setup_test_app();

    // 1. Probe /robots.txt
    let req = Request::builder()
        .uri("/robots.txt")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let content_type = res
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    assert!(content_type.contains("text/plain"));

    let body_bytes = axum::body::to_bytes(res.into_body(), 1024 * 1024).await.unwrap();
    let body_str = String::from_utf8_lossy(&body_bytes);
    assert!(body_str.contains("User-agent: *"));
    assert!(body_str.contains("Allow: /"));
    assert!(body_str.contains("Sitemap: https://pressmark.dev/sitemap.xml"));
    assert!(body_str.contains("Disallow: /api/v1/auth/"));

    // 2. Probe /sitemap.xml
    let req2 = Request::builder()
        .uri("/sitemap.xml")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res2 = app.clone().oneshot(req2).await.unwrap();
    assert_eq!(res2.status(), StatusCode::OK);
    let content_type2 = res2
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    assert!(content_type2.contains("application/xml"));

    let body2_bytes = axum::body::to_bytes(res2.into_body(), 1024 * 1024).await.unwrap();
    let body2_str = String::from_utf8_lossy(&body2_bytes);
    assert!(body2_str.contains("<urlset"));
    assert!(body2_str.contains("<loc>https://pressmark.dev/</loc>"));
    assert!(body2_str.contains("<loc>https://pressmark.dev/privacy</loc>"));
    assert!(body2_str.contains("<loc>https://pressmark.dev/terms</loc>"));
}

#[tokio::test]
async fn test_html_head_seo_opengraph_and_breadcrumbs() {
    let app = setup_test_app();

    let req = Request::builder()
        .uri("/")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(res.into_body(), 1024 * 1024).await.unwrap();
    let html = String::from_utf8_lossy(&body_bytes);

    // Verify Title and Meta tags
    assert!(html.contains("<title>Pressmark Studio · Sub-Millisecond Vector Invoices, Leases & Documents</title>"));
    assert!(html.contains("<meta name=\"description\""));
    assert!(html.contains("Sub-millisecond Rust-native vector document engine"));
    assert!(html.contains("<meta name=\"author\" content=\"Daniel Hallman\">"));
    assert!(html.contains("<link rel=\"canonical\" href=\"https://pressmark.dev/\">"));

    // Verify OpenGraph
    assert!(html.contains("<meta property=\"og:title\" content=\"Pressmark Studio · Sub-Millisecond Vector Document Engine\">"));
    assert!(html.contains("<meta property=\"og:image\" content=\"https://pressmark.dev/assets/hero.jpg\">"));

    // Verify Twitter Card
    assert!(html.contains("<meta name=\"twitter:card\" content=\"summary_large_image\">"));

    // Verify JSON-LD Structured Data
    assert!(html.contains("\"@type\": \"WebApplication\""));
    assert!(html.contains("\"name\": \"Pressmark Document Studio\""));

    // Verify Studio Breadcrumbs & Mobile Navigation Trigger
    assert!(html.contains("id=\"studioBreadcrumbs\""));
    assert!(html.contains("id=\"btnMobileNavToggle\""));

    // Verify Universal Auth Modal
    assert!(html.contains("id=\"modalAuth\""));
    assert!(html.contains("id=\"btnAuthGuest\""));
    assert!(html.contains("id=\"formEmailAuth\""));
    assert!(html.contains("id=\"formApiKeyAuth\""));
}

#[tokio::test]
async fn test_universal_auth_guest_workspace() {
    let app = setup_test_app();

    // 1. Post to /api/v1/auth/guest
    let req = Request::builder()
        .uri("/api/v1/auth/guest")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::empty())
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Extract cookie
    let cookie_header = res
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .expect("Must set session cookie");
    assert!(cookie_header.contains("pressmark_session=guest_sess_"));

    let body_bytes = axum::body::to_bytes(res.into_body(), 1024 * 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(json["authenticated"], true);
    assert_eq!(json["user"]["provider"], "guest");
    assert_eq!(json["user"]["name"], "Guest Workspace");
    assert_eq!(json["user"]["plan"], "free");

    let session_id = json["session_id"].as_str().unwrap();

    // 2. Query /api/v1/auth/session using the session_id as Bearer token
    let req2 = Request::builder()
        .uri("/api/v1/auth/session")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {}", session_id))
        .body(Body::empty())
        .unwrap();

    let res2 = app.clone().oneshot(req2).await.unwrap();
    assert_eq!(res2.status(), StatusCode::OK);
    let bytes2 = axum::body::to_bytes(res2.into_body(), 1024 * 1024).await.unwrap();
    let json2: Value = serde_json::from_slice(&bytes2).unwrap();
    assert_eq!(json2["authenticated"], true);
    assert_eq!(json2["user"]["name"], "Guest Workspace");

    // 3. Query /api/v1/auth/session using the Cookie header
    let req3 = Request::builder()
        .uri("/api/v1/auth/session")
        .method("GET")
        .header(header::COOKIE, format!("pressmark_session={}", session_id))
        .body(Body::empty())
        .unwrap();

    let res3 = app.clone().oneshot(req3).await.unwrap();
    assert_eq!(res3.status(), StatusCode::OK);
    let bytes3 = axum::body::to_bytes(res3.into_body(), 1024 * 1024).await.unwrap();
    let json3: Value = serde_json::from_slice(&bytes3).unwrap();
    assert_eq!(json3["authenticated"], true);
}

#[tokio::test]
async fn test_universal_auth_email_and_api_key_flows() {
    let app = setup_test_app();

    // 1. Email Login
    let email_payload = serde_json::json!({
        "email": "sarah.architect@studio.design",
        "name": "Sarah Architect"
    });

    let req = Request::builder()
        .uri("/api/v1/auth/email/login")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(email_payload.to_string()))
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(res.into_body(), 1024 * 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(json["authenticated"], true);
    assert_eq!(json["user"]["email"], "sarah.architect@studio.design");
    assert_eq!(json["user"]["name"], "Sarah Architect");
    assert_eq!(json["user"]["plan"], "solo_pro");
    assert_eq!(json["user"]["watermark_free"], true);

    let email_session_id = json["session_id"].as_str().unwrap().to_string();

    // 2. Developer API Key Login
    let key_payload = serde_json::json!({
        "api_key": "prsm_live_9a8f7e6d5c4b3a21"
    });

    let req_key = Request::builder()
        .uri("/api/v1/auth/api-key")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(key_payload.to_string()))
        .unwrap();

    let res_key = app.clone().oneshot(req_key).await.unwrap();
    assert_eq!(res_key.status(), StatusCode::OK);

    let key_bytes = axum::body::to_bytes(res_key.into_body(), 1024 * 1024).await.unwrap();
    let key_json: Value = serde_json::from_slice(&key_bytes).unwrap();
    assert_eq!(key_json["authenticated"], true);
    assert_eq!(key_json["user"]["provider"], "api_key");
    assert_eq!(key_json["user"]["plan"], "developer");
    assert_eq!(key_json["user"]["watermark_free"], true);

    // 3. Logout
    let req_logout = Request::builder()
        .uri("/api/v1/auth/logout")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {}", email_session_id))
        .body(Body::empty())
        .unwrap();

    let res_logout = app.clone().oneshot(req_logout).await.unwrap();
    assert_eq!(res_logout.status(), StatusCode::OK);

    // Verify session is now invalidated
    let req_check = Request::builder()
        .uri("/api/v1/auth/session")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {}", email_session_id))
        .body(Body::empty())
        .unwrap();

    let res_check = app.clone().oneshot(req_check).await.unwrap();
    let check_bytes = axum::body::to_bytes(res_check.into_body(), 1024 * 1024).await.unwrap();
    let check_json: Value = serde_json::from_slice(&check_bytes).unwrap();
    assert_eq!(check_json["authenticated"], false);
}
