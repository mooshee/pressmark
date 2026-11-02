//! Pressmark feedback adapter regressions. Author: Daniel Hallman.
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    routing::{get, post},
    Json, Router,
};
use pressmark::{
    feedback::FeedbackClient,
    server::{create_router, AppState},
    storage::Storage,
};
use serde_json::{json, Value};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tower::ServiceExt;

async fn fixture() -> (
    axum::Router,
    Arc<Mutex<Vec<Value>>>,
    tokio::task::JoinHandle<()>,
) {
    let seen = Arc::new(Mutex::new(Vec::<Value>::new()));
    let seen2 = seen.clone();
    let backend=Router::new().route("/health",get(||async{Json(json!({"service":"platform-feedback","status":"healthy"}))}))
        .route("/v1/products/pressmark/submit/:source",post(move|Json(input):Json<Value>|{let seen=seen2.clone();async move{seen.lock().unwrap().push(input);Json(json!({"report_id":"00000000-0000-4000-8000-000000000010","access_key":"a".repeat(64),"status":"new","created_at":"2026-11-01T00:00:00Z","replayed":false}))}}))
        .route("/v1/products/pressmark/status",post(||async{Json(json!({"report_id":"00000000-0000-4000-8000-000000000010","status":"reviewed","updated_at":"2026-11-01T00:00:01Z"}))}));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        axum::serve(listener, backend).await.unwrap();
    });
    let mut state = AppState::default().with_storage(Arc::new(Storage::new(":memory:").unwrap()));
    state.base_url = "https://pressmark.dev".into();
    state.feedback = Some(FeedbackClient::new(&format!("http://{address}")).unwrap());
    (create_router(Arc::new(state)), seen, task)
}

fn report() -> Value {
    json!({"request_id":"00000000-0000-4000-8000-000000000001","kind":"bug","summary":"Synthetic feedback test","description":"Local fixture only."})
}
async fn call(app: Router, path: &str, input: Value) -> (StatusCode, Value) {
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(path)
                .header("content-type", "application/json")
                .body(Body::from(input.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let code = response.status();
    assert!(response.headers()["cache-control"]
        .to_str()
        .unwrap()
        .split(',')
        .any(|value| value.trim() == "no-store"));
    let bytes = to_bytes(response.into_body(), 65536).await.unwrap();
    (code, serde_json::from_slice(&bytes).unwrap())
}

#[tokio::test]
async fn web_and_mcp_use_the_same_private_backend_and_status_contract() {
    let (app, seen, task) = fixture().await;
    let (code, receipt) = call(app.clone(), "/api/v1/feedback", report()).await;
    assert_eq!(code, StatusCode::OK);
    assert_eq!(receipt["status"], "new");
    let(code,result)=call(app.clone(),"/mcp",json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"submit_feedback","arguments":report()}})).await;
    assert_eq!(code, StatusCode::OK);
    assert_eq!(result["result"]["isError"], false);
    assert_eq!(
        result["result"]["structuredContent"]["report_id"],
        receipt["report_id"]
    );
    assert!(!result["result"]["content"]
        .to_string()
        .contains(&"a".repeat(64)));
    assert_eq!(seen.lock().unwrap().len(), 2);
    let (code, status) = call(
        app.clone(),
        "/api/v1/feedback/status",
        json!({"report_id":receipt["report_id"],"access_key":receipt["access_key"]}),
    )
    .await;
    assert_eq!(code, StatusCode::OK);
    assert_eq!(status["status"], "reviewed");
    assert!(status.get("access_key").is_none());
    let health = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/feedback/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(health.status(), StatusCode::OK);
    task.abort();
}

#[tokio::test]
async fn origin_spam_and_unavailable_service_never_claim_receipt() {
    let (app, seen, task) = fixture().await;
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/feedback")
                .header("origin", "https://unrelated.invalid")
                .header("content-type", "application/json")
                .body(Body::from(report().to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let mut spam = report();
    spam["website"] = json!("spam");
    assert_eq!(
        call(app.clone(), "/api/v1/feedback", spam).await.0,
        StatusCode::BAD_REQUEST
    );
    assert!(seen.lock().unwrap().is_empty());
    task.abort();
    tokio::time::sleep(Duration::from_millis(10)).await;
    let (code, value) = call(app.clone(), "/api/v1/feedback", report()).await;
    assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    assert!(value.get("report_id").is_none());
    let(_,result)=call(app,"/mcp",json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"submit_feedback","arguments":report()}})).await;
    assert_eq!(result["result"]["isError"], true);
}

#[tokio::test]
async fn form_discovery_and_safe_target_validation() {
    for target in [
        "https://example.com",
        "http://127.0.0.1/other",
        "http://user@127.0.0.1:3844",
        "http://127.0.0.1:3844?secret=x",
    ] {
        assert!(FeedbackClient::new(target).is_err());
    }
    let (app, _, task) = fixture().await;
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/feedback")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let html = String::from_utf8(
        to_bytes(response.into_body(), 65536)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    assert!(html.contains("Help improve Pressmark"));
    assert!(html.contains("reply_email"));
    let tools = pressmark::mcp::McpHandler::get_tools();
    for name in ["submit_feedback", "get_feedback_status"] {
        assert!(tools
            .as_array()
            .unwrap()
            .iter()
            .any(|tool| tool["name"] == name));
    }
    assert!(
        pressmark::openapi::get_openapi_spec("https://pressmark.dev")["paths"]["/api/v1/feedback"]
            .is_object()
    );
    task.abort();
}
