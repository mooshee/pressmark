//! Pressmark adapter for the shared feedback service. Author: Daniel Hallman.
use crate::{
    mcp::{JsonRpcRequest, JsonRpcResponse},
    rate_limit::{extract_client_ip, RateLimiter},
    server::AppState,
};
use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub struct FeedbackClient {
    base: String,
    client: reqwest::Client,
    limiter: RateLimiter,
}
impl FeedbackClient {
    pub fn new(base: &str) -> Result<Self, String> {
        let url = reqwest::Url::parse(base).map_err(|_| "Invalid feedback service address")?;
        let local = url
            .host_str()
            .and_then(|s| s.parse::<std::net::IpAddr>().ok())
            .is_some_and(|ip| ip.is_loopback());
        if !local
            || url.scheme() != "http"
            || !url.username().is_empty()
            || url.password().is_some()
            || url.path() != "/"
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err("Feedback service must use an explicit loopback HTTP address".into());
        }
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(8))
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| "Feedback client unavailable")?;
        Ok(Self {
            base: base.trim_end_matches('/').into(),
            client,
            limiter: RateLimiter::new(15, 3600),
        })
    }
    pub fn from_env() -> Option<Self> {
        Self::new(
            &std::env::var("PRESSMARK_FEEDBACK_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:3844".into()),
        )
        .ok()
    }
    pub async fn healthy(&self) -> bool {
        match self
            .client
            .get(format!("{}/health", self.base))
            .send()
            .await
        {
            Ok(r) if r.status().is_success() => r
                .json::<Value>()
                .await
                .is_ok_and(|v| v["service"] == "platform-feedback" && v["status"] == "healthy"),
            _ => false,
        }
    }
    async fn send(
        &self,
        status: bool,
        source: &str,
        input: Value,
        headers: &HeaderMap,
    ) -> (StatusCode, Value) {
        if !self
            .limiter
            .check(&extract_client_ip(headers, None))
            .await
            .0
        {
            return (
                StatusCode::TOO_MANY_REQUESTS,
                json!({"error":"rate_limited","message":"Please try again later."}),
            );
        }
        let path = if status {
            "status".to_owned()
        } else {
            format!("submit/{source}")
        };
        match self
            .client
            .post(format!("{}/v1/products/pressmark/{path}", self.base))
            .json(&input)
            .send()
            .await
        {
            Ok(response) => {
                let code = response.status();
                match response.json::<Value>().await {
                    Ok(value) if code.is_success() => (StatusCode::OK, value),
                    Ok(value) if matches!(code.as_u16(), 400 | 404 | 409 | 413 | 422 | 429) => (
                        StatusCode::from_u16(code.as_u16()).unwrap_or(StatusCode::BAD_REQUEST),
                        value,
                    ),
                    _ => (StatusCode::SERVICE_UNAVAILABLE, unavailable()),
                }
            }
            Err(_) => (StatusCode::SERVICE_UNAVAILABLE, unavailable()),
        }
    }
}
fn unavailable() -> Value {
    json!({"error":"feedback_unavailable","message":"Your feedback has not been confirmed. Keep your text and try again with the same request ID."})
}
fn private_response(code: StatusCode, value: Value) -> Response {
    (
        code,
        [
            (header::CACHE_CONTROL, "no-store"),
            (
                header::HeaderName::from_static("x-robots-tag"),
                "noindex, nofollow",
            ),
        ],
        Json(value),
    )
        .into_response()
}
fn valid_origin(headers: &HeaderMap, base: &str) -> bool {
    headers
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .is_none_or(|origin| {
            reqwest::Url::parse(origin)
                .ok()
                .zip(reqwest::Url::parse(base).ok())
                .is_some_and(|(a, b)| a.origin() == b.origin())
        })
}
pub async fn submit(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(mut input): Json<Value>,
) -> Response {
    if !valid_origin(&headers, &state.base_url) {
        return private_response(StatusCode::FORBIDDEN, json!({"error":"origin_rejected"}));
    }
    // A hidden field catches simple form spam without making agents solve a CAPTCHA.
    if input
        .get("website")
        .and_then(Value::as_str)
        .is_some_and(|v| !v.is_empty())
    {
        return private_response(StatusCode::BAD_REQUEST, json!({"error":"invalid_feedback"}));
    }
    if let Some(obj) = input.as_object_mut() {
        obj.remove("website");
    }
    match &state.feedback {
        Some(client) => {
            let (code, value) = client.send(false, "web", input, &headers).await;
            private_response(code, value)
        }
        None => private_response(StatusCode::SERVICE_UNAVAILABLE, unavailable()),
    }
}
#[derive(serde::Deserialize, Default)]
pub struct FeedbackStatusQuery {
    pub request_id: Option<String>,
    pub feedback_id: Option<String>,
}

pub async fn status_get(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<FeedbackStatusQuery>,
) -> Response {
    if !valid_origin(&headers, &state.base_url) {
        return private_response(StatusCode::FORBIDDEN, json!({"error":"origin_rejected"}));
    }
    let target_id = query.request_id.or(query.feedback_id);
    let Some(id) = target_id else {
        return private_response(
            StatusCode::BAD_REQUEST,
            json!({
                "error": "missing_request_id",
                "message": "Provide request_id in query parameter or POST request body"
            }),
        );
    };
    match &state.feedback {
        Some(client) => {
            let (code, value) = client
                .send(true, "web", json!({ "request_id": id }), &headers)
                .await;
            private_response(code, value)
        }
        None => private_response(StatusCode::SERVICE_UNAVAILABLE, unavailable()),
    }
}

pub async fn status(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(input): Json<Value>,
) -> Response {
    if !valid_origin(&headers, &state.base_url) {
        return private_response(StatusCode::FORBIDDEN, json!({"error":"origin_rejected"}));
    }
    match &state.feedback {
        Some(client) => {
            let (code, value) = client.send(true, "web", input, &headers).await;
            private_response(code, value)
        }
        None => private_response(StatusCode::SERVICE_UNAVAILABLE, unavailable()),
    }
}
pub async fn health(State(state): State<Arc<AppState>>) -> Response {
    let healthy = match &state.feedback {
        Some(client) => client.healthy().await,
        None => false,
    };
    private_response(
        if healthy {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        json!({"service":"pressmark-feedback","status":if healthy{"healthy"}else{"unavailable"}}),
    )
}
pub async fn page() -> impl IntoResponse {
    (
        [
            (header::CACHE_CONTROL, "no-store"),
            (
                header::HeaderName::from_static("x-robots-tag"),
                "noindex, nofollow",
            ),
        ],
        Html(include_str!("feedback.html")),
    )
}

pub fn tools() -> Vec<Value> {
    let properties = json!({
        "request_id":{"type":"string","format":"uuid","description":"A new UUID for this report. Reuse it when retrying unchanged feedback."},
        "kind":{"type":"string","enum":["bug","feature","general"]},
        "summary":{"type":"string","minLength":1,"maxLength":160},
        "description":{"type":"string","minLength":1,"maxLength":6000},
        "steps":{"type":"string","maxLength":3000},"expected":{"type":"string","maxLength":1500},"actual":{"type":"string","maxLength":1500},
        "version":{"type":"string","maxLength":80},"reply_email":{"type":"string","maxLength":254,"description":"Optional email supplied by the user for feedback follow-up."}
    });
    vec![
        json!({
            "name":"submit_feedback",
            "title":"Send Pressmark feedback",
            "description":"Send a bug report, feature request, or general feedback to the private Pressmark inbox when the user asks. Include only details they choose to share; omit documents, passwords, and tokens.",
            "annotations":{"readOnlyHint":false,"destructiveHint":false,"idempotentHint":false,"openWorldHint":true},
            "inputSchema":{
                "type":"object",
                "properties":properties,
                "required":["request_id","kind","summary","description"],
                "additionalProperties":false,
                "examples": [
                    {
                        "request_id": "a0000000-0000-0000-0000-000000000001",
                        "kind": "bug",
                        "summary": "Preview rendering error on custom font",
                        "description": "When exporting with custom font tokens, preview returns fallback styling."
                    }
                ]
            }
        }),
        json!({
            "name":"get_feedback_status",
            "title":"Check feedback status",
            "description":"Check one Pressmark report using its report ID and private access key. Returns its review status without exposing the report text or contact details.",
            "annotations":{"readOnlyHint":true,"destructiveHint":false,"idempotentHint":true,"openWorldHint":true},
            "inputSchema":{
                "type":"object",
                "properties":{"report_id":{"type":"string","format":"uuid"},"access_key":{"type":"string","minLength":64,"maxLength":64}},
                "required":["report_id","access_key"],
                "additionalProperties":false,
                "examples": [
                    {
                        "report_id": "a0000000-0000-0000-0000-000000000001",
                        "access_key": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    }
                ]
            }
        }),
    ]
}
pub fn is_tool(req: &JsonRpcRequest) -> bool {
    req.method == "tools/call"
        && matches!(
            req.params.as_ref().and_then(|p| p["name"].as_str()),
            Some("submit_feedback" | "get_feedback_status")
        )
}
fn tool_response(id: Option<Value>, code: StatusCode, value: Value) -> JsonRpcResponse {
    let ok = code.is_success();
    // Access keys stay in the structured receipt; they are never repeated in explanatory text.
    let message = if ok {
        if value.get("access_key").is_some() {
            "Feedback received. Keep the private receipt to check its status."
        } else {
            "Feedback status retrieved."
        }
    } else {
        value["message"]
            .as_str()
            .unwrap_or("Feedback could not be confirmed. Keep your text and retry later.")
    };
    JsonRpcResponse {
        jsonrpc: "2.0",
        id,
        error: None,
        result: Some(
            json!({"isError":!ok,"content":[{"type":"text","text":message}],"structuredContent":value}),
        ),
    }
}
pub async fn handle_tool(
    state: &AppState,
    headers: &HeaderMap,
    req: JsonRpcRequest,
) -> JsonRpcResponse {
    let check = req
        .params
        .as_ref()
        .is_some_and(|p| p["name"] == "get_feedback_status");
    let input = req
        .params
        .as_ref()
        .and_then(|p| p.get("arguments"))
        .cloned()
        .unwrap_or(json!({}));
    let (code, value) = match &state.feedback {
        Some(client) => client.send(check, "agent", input, headers).await,
        None => (StatusCode::SERVICE_UNAVAILABLE, unavailable()),
    };
    tool_response(req.id, code, value)
}
pub fn handle_stdio(req: JsonRpcRequest) -> JsonRpcResponse {
    // The synchronous stdio dispatcher runs in Tokio's entrypoint; isolate blocking I/O on its own thread.
    let id = req.id.clone();
    std::thread::spawn(move || {
        let Some(config) = FeedbackClient::from_env() else {
            return tool_response(req.id, StatusCode::SERVICE_UNAVAILABLE, unavailable());
        };
        let check = req
            .params
            .as_ref()
            .is_some_and(|p| p["name"] == "get_feedback_status");
        let path = if check { "status" } else { "submit/agent" };
        let input = req
            .params
            .as_ref()
            .and_then(|p| p.get("arguments"))
            .cloned()
            .unwrap_or(json!({}));
        let result = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(8))
            .no_proxy()
            .build()
            .ok()
            .and_then(|c| {
                c.post(format!("{}/v1/products/pressmark/{path}", config.base))
                    .json(&input)
                    .send()
                    .ok()
            })
            .and_then(|r| {
                let code = r.status();
                r.json::<Value>().ok().map(|v| (code, v))
            });
        match result {
            Some((code, value)) => tool_response(
                req.id,
                StatusCode::from_u16(code.as_u16()).unwrap_or(StatusCode::SERVICE_UNAVAILABLE),
                value,
            ),
            None => tool_response(req.id, StatusCode::SERVICE_UNAVAILABLE, unavailable()),
        }
    })
    .join()
    .unwrap_or_else(|_| tool_response(id, StatusCode::SERVICE_UNAVAILABLE, unavailable()))
}
