//! HTTP MCP download delivery, privacy headers, and byte identity.
//!
//! Author: Daniel Hallman

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use pressmark::{
    auth::ChatGptAuthManager,
    rate_limit::RateLimiter,
    server::{create_router, AppState},
    storage::Storage,
    stripe::StripeIdempotencyStore,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceExt;

fn app() -> axum::Router {
    let storage = Arc::new(Storage::new(":memory:").unwrap());
    create_router(Arc::new(AppState {
        base_url: "https://pressmark.dev".to_string(),
        auth_mgr: ChatGptAuthManager::in_memory().with_storage(Arc::clone(&storage)),
        rate_limiter: RateLimiter::new(1000, 60),
        stripe_idempotency: StripeIdempotencyStore::new().with_storage(Arc::clone(&storage)),
        storage,
        downloads: Default::default(),
    }))
}

fn document() -> Value {
    json!({
        "doc_type": "invoice", "doc_number": "INV-DOWNLOAD-42", "issue_date": "2026-10-29",
        "from": { "name": "Sample Studio" }, "to": { "name": "Sample Client" },
        "items": [{ "description": "Design services", "quantity": 3, "unit_price": 100 }]
    })
}

fn request(path: &str, method: &str) -> Request<Body> {
    Request::builder()
        .uri(path)
        .method(method)
        .header("user-agent", "Pressmark-Measurement-QA/1.0")
        .body(Body::empty())
        .unwrap()
}

fn assert_private_headers(headers: &axum::http::HeaderMap) {
    assert_eq!(headers["cache-control"], "private, no-store, max-age=0");
    assert_eq!(headers["cdn-cache-control"], "no-store");
    assert_eq!(headers["cloudflare-cdn-cache-control"], "no-store");
    assert_eq!(headers["x-robots-tag"], "noindex, nofollow, noarchive");
    assert_eq!(headers["referrer-policy"], "no-referrer");
    assert_eq!(headers["x-content-type-options"], "nosniff");
}

#[tokio::test]
async fn all_http_pdf_tools_return_working_links_to_the_exact_generated_bytes() {
    let router = app();
    for (tool, arguments) in [
        ("generate_document", document()),
        (
            "convert_document",
            json!({"document":document(),"target_type":"estimate"}),
        ),
        (
            "record_payment",
            json!({"document":document(),"amount_paid":300,"payment_method":"Check","paid_date":"2026-10-28","transaction_reference":"SAMPLE-CHECK-42"}),
        ),
    ] {
        let response = router.clone().oneshot(Request::builder().uri("/mcp").method("POST")
            .header("content-type", "application/json")
            .header("user-agent", "Pressmark-Measurement-QA/1.0")
            .body(Body::from(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":tool,"arguments":arguments}}).to_string())).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["cache-control"], "private, no-store");
        let bytes = axum::body::to_bytes(response.into_body(), 10 * 1024 * 1024)
            .await
            .unwrap();
        let response: Value = serde_json::from_slice(&bytes).unwrap();
        assert!(
            response.get("error").is_none() || response["error"].is_null(),
            "{response}"
        );
        let result = &response["result"];
        assert_eq!(result["download"], result["structuredContent"]["download"]);
        assert_eq!(result["download"]["expires_in_seconds"], 900);
        let url = result["download"]["url"].as_str().expect("download URL");
        let parsed = reqwest::Url::parse(url).unwrap();
        assert_eq!(parsed.host_str(), Some("pressmark.dev"));
        let content = result["content"].as_array().unwrap();
        assert!(content.iter().any(
            |item| item["type"] == "text" && item["text"].as_str().unwrap_or("").contains(url)
        ));
        assert!(content
            .iter()
            .any(|item| item["type"] == "resource_link" && item["uri"] == url));
        let expected_pdf = BASE64
            .decode(result["pdf_base64"].as_str().unwrap())
            .unwrap();
        let reread = read_resource(&router, url).await;
        assert!(reread["error"].is_null(), "{reread}");
        assert_eq!(reread["result"]["contents"][0]["mimeType"], "application/pdf");
        assert_eq!(BASE64.decode(reread["result"]["contents"][0]["blob"].as_str().unwrap()).unwrap(), expected_pdf);
        assert_eq!(result["content"].as_array().unwrap().iter().filter(|item| item["type"] == "resource" || item["type"] == "resource_link").count(), 1);

        let download = router
            .clone()
            .oneshot(request(parsed.path(), "GET"))
            .await
            .unwrap();
        assert_eq!(download.status(), StatusCode::OK);
        assert_private_headers(download.headers());
        assert_eq!(download.headers()["content-type"], "application/pdf");
        assert_eq!(
            download.headers()["content-length"],
            expected_pdf.len().to_string()
        );
        assert_eq!(
            download.headers()["content-disposition"],
            format!(
                "attachment; filename=\"{}\"",
                result["download"]["filename"].as_str().unwrap()
            )
        );
        let actual_pdf = axum::body::to_bytes(download.into_body(), 8 * 1024 * 1024)
            .await
            .unwrap();
        assert_eq!(actual_pdf.as_ref(), expected_pdf);

        let head = router
            .clone()
            .oneshot(request(parsed.path(), "HEAD"))
            .await
            .unwrap();
        assert_eq!(head.status(), StatusCode::OK);
        assert_private_headers(head.headers());
        assert_eq!(
            head.headers()["content-length"],
            expected_pdf.len().to_string()
        );
        assert!(axum::body::to_bytes(head.into_body(), 1024)
            .await
            .unwrap()
            .is_empty());

        let restarted = app().oneshot(request(parsed.path(), "GET")).await.unwrap();
        assert_eq!(restarted.status(), StatusCode::NOT_FOUND);
        assert_private_headers(restarted.headers());
    }
}

#[tokio::test]
async fn downloads_have_no_listing_and_unknown_links_do_not_leak_details() {
    let router = app();
    for path in ["/api/v1/downloads", "/api/v1/downloads/"] {
        assert_eq!(
            router
                .clone()
                .oneshot(request(path, "GET"))
                .await
                .unwrap()
                .status(),
            StatusCode::NOT_FOUND
        );
    }
    for token in ["invalid", &"0".repeat(64)] {
        let response = router
            .clone()
            .oneshot(request(&format!("/api/v1/downloads/{token}"), "GET"))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_private_headers(response.headers());
        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(!text.contains(token));
        assert!(text.contains("Generate the PDF again"));
    }
    let robots = router
        .clone()
        .oneshot(request("/robots.txt", "GET"))
        .await
        .unwrap();
    let robots = axum::body::to_bytes(robots.into_body(), 10240)
        .await
        .unwrap();
    assert!(String::from_utf8_lossy(&robots).contains("Disallow: /api/v1/downloads/"));
}

#[tokio::test]
async fn download_content_matches_the_negotiated_mcp_version() {
    let router = app();
    for (protocol, has_resource_link) in [
        ("2024-11-05", false),
        ("2025-03-26", false),
        ("2025-06-18", true),
        ("2025-11-25", true),
        ("2026-07-28", true),
    ] {
        let response = router.clone().oneshot(Request::builder().uri("/mcp").method("POST")
            .header("content-type", "application/json")
            .header("mcp-protocol-version", protocol)
            .header("user-agent", "Pressmark-Measurement-QA/1.0")
            .body(Body::from(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"generate_document","arguments":document()}}).to_string())).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["mcp-protocol-version"], protocol);
        let body = axum::body::to_bytes(response.into_body(), 10 * 1024 * 1024)
            .await
            .unwrap();
        let response: Value = serde_json::from_slice(&body).unwrap();
        let result = &response["result"];
        let content = result["content"].as_array().unwrap();
        assert_eq!(
            content.iter().any(|item| item["type"] == "resource_link"),
            has_resource_link
        );
        assert!(content.iter().any(|item| item["type"] == "text"
            && item["text"]
                .as_str()
                .unwrap_or_default()
                .contains("[Download PDF]")));
        assert_eq!(content
            .iter()
            .any(|item| item["type"] == "resource"
                && item["resource"]["mimeType"] == "application/pdf"), !has_resource_link);
        let uri = if has_resource_link {
            content.iter().find(|item| item["type"] == "resource_link").unwrap()["uri"].as_str().unwrap()
        } else {
            content.iter().find(|item| item["type"] == "resource").unwrap()["resource"]["uri"].as_str().unwrap()
        };
        let reread = read_resource(&router, uri).await;
        assert!(reread["error"].is_null(), "{reread}");
        assert_eq!(reread["result"]["contents"][0]["mimeType"], "application/pdf");
        assert_eq!(reread["result"]["contents"][0]["blob"], result["pdf_base64"]);
        if protocol == "2026-07-28" {
            assert_eq!(result["_meta"]["cacheTtlMs"], 0);
        }
    }
}

async fn read_resource(router: &axum::Router, uri: &str) -> Value {
    let response = router.clone().oneshot(Request::builder().uri("/mcp").method("POST")
        .header("content-type", "application/json")
        .header("mcp-protocol-version", "2026-07-28")
        .header("user-agent", "Pressmark-Measurement-QA/1.0")
        .body(Body::from(json!({"jsonrpc":"2.0","id":12,"method":"resources/read","params":{"uri":uri}}).to_string())).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["cache-control"], "private, no-store");
    let body = axum::body::to_bytes(response.into_body(), 12 * 1024 * 1024).await.unwrap();
    let result: Value = serde_json::from_slice(&body).unwrap();
    if result["error"].is_null() {
        assert_eq!(result["result"]["ttlMs"], 0);
        assert_eq!(result["result"]["cacheScope"], "private");
        assert_eq!(result["result"]["_meta"]["cacheTtlMs"], 0);
    }
    result
}

#[tokio::test]
async fn previews_are_exact_png_resources_and_invalid_or_expired_uris_never_return_catalogs() {
    let router = app();
    let response = router.clone().oneshot(Request::builder().uri("/mcp").method("POST")
        .header("content-type", "application/json")
        .header("user-agent", "Pressmark-Measurement-QA/1.0")
        .body(Body::from(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"preview_document","arguments":document()}}).to_string())).unwrap()).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), 12 * 1024 * 1024).await.unwrap();
    let response: Value = serde_json::from_slice(&body).unwrap();
    let result = &response["result"];
    assert_eq!(result["preview"], result["structuredContent"]["preview"]);
    assert_eq!(result["preview"]["expires_in_seconds"], 900);
    let uri = result["preview"]["url"].as_str().expect("preview link");
    let content = result["content"].as_array().unwrap();
    let image = content.iter().find(|item| item["type"] == "image").unwrap();
    let expected_png = BASE64.decode(image["data"].as_str().unwrap()).unwrap();
    assert!(expected_png.starts_with(b"\x89PNG\r\n\x1a\n"));
    assert!(content.iter().any(|item| item["type"] == "text" && item["text"].as_str().unwrap_or_default().contains(uri)));
    let reread = read_resource(&router, uri).await;
    assert!(reread["error"].is_null(), "{reread}");
    assert_eq!(reread["result"]["contents"][0]["mimeType"], "image/png");
    assert_eq!(BASE64.decode(reread["result"]["contents"][0]["blob"].as_str().unwrap()).unwrap(), expected_png);
    let parsed = reqwest::Url::parse(uri).unwrap();
    let download = router.clone().oneshot(request(parsed.path(), "GET")).await.unwrap();
    assert_eq!(download.status(), StatusCode::OK);
    assert_private_headers(download.headers());
    assert_eq!(download.headers()["content-type"], "image/png");
    assert_eq!(download.headers()["content-disposition"], "inline; filename=\"invoice-preview.png\"");
    assert_eq!(axum::body::to_bytes(download.into_body(), 8 * 1024 * 1024).await.unwrap().as_ref(), expected_png);
    let restarted = read_resource(&app(), uri).await;
    assert_eq!(restarted["error"]["code"], -32002);
    for invalid in [
        "pressmark://documents/unknown.pdf".to_owned(),
        "pressmark://catalog/missing".to_owned(),
        "https://example.com/api/v1/downloads/unknown".to_owned(),
        "http://127.0.0.1/private".to_owned(),
        format!("{uri}?other=1"),
        format!("{uri}#fragment"),
        uri.replace("https://", "https://other@"),
        "https://pressmark.dev/api/v1/downloads/../private".to_owned(),
    ] {
        let missing = read_resource(&router, &invalid).await;
        assert_eq!(missing["error"]["code"], -32002, "{missing}");
        assert!(missing["result"].is_null());
    }
    let catalog = read_resource(&router, "pressmark://catalog/archetypes").await;
    assert_eq!(catalog["result"]["contents"][0]["mimeType"], "application/json");
    assert!(catalog["result"]["contents"][0]["blob"].is_null());
    for name in ["states", "pricing", "themes", "compliance", "execution"] {
        let resource = read_resource(&router, &format!("pressmark://catalog/{name}")).await;
        assert!(resource["error"].is_null(), "{name}: {resource}");
        let text = resource["result"]["contents"][0]["text"].as_str().unwrap();
        assert_ne!(text, catalog["result"]["contents"][0]["text"].as_str().unwrap());
        assert!(serde_json::from_str::<Value>(text).is_ok());
    }
}

async fn lifecycle_document(
    router: &axum::Router,
    tool: &str,
    arguments: Value,
    mcp: bool,
) -> Value {
    let path = if mcp {
        "/mcp"
    } else if tool == "convert_document" {
        "/api/v1/lifecycle/convert"
    } else {
        "/api/v1/lifecycle/pay"
    };
    let payload = if mcp {
        json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":tool,"arguments":arguments}})
    } else {
        arguments
    };
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri(path)
                .method("POST")
                .header("content-type", "application/json")
                .header("user-agent", "Pressmark-Measurement-QA/1.0")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), 10 * 1024 * 1024)
        .await
        .unwrap();
    let response: Value = serde_json::from_slice(&bytes).unwrap();
    if mcp {
        assert!(response["error"].is_null(), "{response}");
        response["result"]["document"].clone()
    } else {
        assert_eq!(response["success"], true);
        response["document"].clone()
    }
}

#[tokio::test]
async fn lifecycle_paths_leave_missing_payment_metadata_blank_and_preserve_supplied_dates() {
    let router = app();
    for mcp in [false, true] {
        for (tool, arguments) in [
            (
                "convert_document",
                json!({"document":document(),"target_type":"receipt"}),
            ),
            (
                "record_payment",
                json!({"document":document(),"amount_paid":100}),
            ),
            (
                "record_payment",
                json!({"document":document(),"amount_paid":300}),
            ),
        ] {
            let returned = lifecycle_document(&router, tool, arguments, mcp).await;
            for field in ["payment_method", "transaction_reference", "paid_at"] {
                assert!(
                    returned[field].is_null(),
                    "{tool} invented {field}: {}",
                    returned[field]
                );
            }
        }

        let mut supplied = document();
        supplied["payment_method"] = json!("Check");
        supplied["transaction_reference"] = json!("SUPPLIED-CHECK-42");
        supplied["paid_at"] = json!("2026-10-28");
        for (tool, arguments) in [
            (
                "convert_document",
                json!({"document":supplied,"target_type":"receipt"}),
            ),
            (
                "record_payment",
                json!({"document":document(),"amount_paid":100,"payment_method":"Check","transaction_reference":"SUPPLIED-CHECK-42","paid_date":"2026-10-28"}),
            ),
            (
                "record_payment",
                json!({"document":document(),"amount_paid":300,"payment_method":"Check","transaction_reference":"SUPPLIED-CHECK-42","paid_date":"2026-10-28"}),
            ),
        ] {
            let returned = lifecycle_document(&router, tool, arguments, mcp).await;
            assert_eq!(returned["payment_method"], "Check");
            assert_eq!(returned["transaction_reference"], "SUPPLIED-CHECK-42");
            assert_eq!(returned["paid_at"], "2026-10-28");
            assert_eq!(returned["issue_date"], "2026-10-29");
        }
    }
}
