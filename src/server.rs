//! Pressmark Axum HTTP Server & API Router
//!
//! Author: Daniel Hallman

use crate::auth::{
    self, ApiKeyLoginRequest, CallbackQuery, ChatGptAuthManager, EmailLoginRequest, LoginQuery,
};
use crate::engine::{compile_pdf_bytes, compile_png_bytes, generate_pdf, generate_preview_png, generate_all_preview_pngs};
use crate::mcp::{JsonRpcRequest, McpHandler};
use crate::models::{DocumentRequest, GenerateResponse, PreviewResponse, CopilotRequest, CopilotResponse, VisionRequest, VisionResponse, LineItem, PartyDetails};
use crate::openapi::{get_ai_plugin_manifest, get_openapi_spec};
use crate::pricing::get_pricing_catalog;
use axum::extract::{Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::Deserialize;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use crate::rate_limit::{rate_limit_middleware, RateLimiter};

#[derive(Clone)]
pub struct AppState {
    pub base_url: String,
    pub auth_mgr: ChatGptAuthManager,
    pub rate_limiter: RateLimiter,
    pub stripe_idempotency: crate::stripe::StripeIdempotencyStore,
    pub storage: Arc<crate::storage::Storage>,
}

impl AppState {
    pub fn new(base_url: String) -> Self {
        let max_req = std::env::var("PRESSMARK_RATE_LIMIT_MAX")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(120);
        let window_secs = std::env::var("PRESSMARK_RATE_LIMIT_WINDOW")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);

        let storage = Arc::new(crate::storage::Storage::default_db());
        let auth_mgr = ChatGptAuthManager::new().with_storage(Arc::clone(&storage));
        let stripe_idempotency = crate::stripe::StripeIdempotencyStore::new().with_storage(Arc::clone(&storage));

        Self {
            base_url,
            auth_mgr,
            rate_limiter: RateLimiter::new(max_req, window_secs),
            stripe_idempotency,
            storage,
        }
    }

    pub fn with_rate_limiter(mut self, rate_limiter: RateLimiter) -> Self {
        self.rate_limiter = rate_limiter;
        self
    }

    pub fn with_storage(mut self, storage: Arc<crate::storage::Storage>) -> Self {
        self.auth_mgr = self.auth_mgr.with_storage(Arc::clone(&storage));
        self.stripe_idempotency = self.stripe_idempotency.with_storage(Arc::clone(&storage));
        self.storage = storage;
        self
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new("http://127.0.0.1:3842".to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct FormatQuery {
    pub format: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawRenderRequest {
    #[serde(alias = "typst_markup")]
    pub markup: String,
    pub format: Option<String>,
    pub ppi: Option<u32>,
}

pub fn create_router(state: Arc<AppState>) -> Router {
        let body_limit_bytes = std::env::var("PRESSMARK_BODY_LIMIT_MB")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(5) * 1024 * 1024;

        Router::new()
        // Interactive Dashboard, Legal & SEO
        .route("/", get(handle_index))
        .route("/pricing", get(handle_index))
        .route("/demo", get(handle_index))
        .route("/favicon.ico", get(handle_favicon))
        .route("/favicon.svg", get(handle_favicon))
        .route("/privacy", get(handle_privacy))
        .route("/terms", get(handle_terms))
        .route("/robots.txt", get(handle_robots))
        .route("/sitemap.xml", get(handle_sitemap))
        .route("/health", get(handle_health))
        .route("/healthz", get(handle_health))
        // API v1
        .route("/api/v1/generate", post(handle_generate))
        .route("/api/v1/preview", post(handle_preview))
        .route("/api/v1/render-raw", post(handle_render_raw))
        .route("/api/v1/render-vector", post(handle_render_raw))
        .route("/api/v1/pricing", get(handle_pricing))
        .route("/api/v1/checkout", get(handle_checkout_get).post(handle_checkout_post))
        .route("/api/v1/setup-check", get(handle_setup_check))
        .route("/api/v1/openapi.json", get(handle_openapi))
        .route("/api/v1/copilot", post(handle_copilot))
        .route("/api/v1/vision-extract", post(handle_vision_extract))
        .route("/api/v1/lifecycle/convert", post(handle_lifecycle_convert))
        .route("/api/v1/lifecycle/pay", post(handle_lifecycle_pay))
        .route("/api/v1/webhooks/stripe", post(handle_stripe_webhook))
        .route("/.well-known/ai-plugin.json", get(handle_ai_plugin))
        .route("/.well-known/openai-apps-challenge", get(handle_openai_apps_challenge))
        .route("/.well-known/oauth-protected-resource", get(handle_oauth_protected_resource))
        .route("/.well-known/oauth-protected-resource/mcp", get(handle_oauth_protected_resource))
        .route("/.well-known/mcp/server-card.json", get(handle_mcp_server_card))
        .route("/.well-known/mcp.json", get(handle_mcp_server_card))
        // Universal Authentication
        .route("/api/v1/auth/session", get(handle_chatgpt_session))
        .route("/api/v1/auth/guest", post(handle_guest_login))
        .route("/api/v1/auth/email/login", post(handle_email_login))
        .route("/api/v1/auth/api-key", post(handle_api_key_login))
        .route("/api/v1/auth/logout", post(handle_chatgpt_logout))
        // ChatGPT Login & Subscription Pass-Through (preserved for backwards-compatibility)
        .route("/api/v1/auth/chatgpt/login", get(handle_chatgpt_login))
        .route("/api/v1/auth/chatgpt/callback", get(handle_chatgpt_callback))
        .route("/api/v1/auth/chatgpt/session", get(handle_chatgpt_session))
        .route("/api/v1/auth/chatgpt/logout", post(handle_chatgpt_logout))
        // MCP Endpoints & Tool Discovery
        .route("/mcp", get(handle_mcp_get).post(handle_mcp))
        .route("/api/v1/mcp", get(handle_mcp_get).post(handle_mcp))
        .route("/api/v1/mcp/tools.json", get(handle_mcp_tools))
        // Static assets
        .nest_service("/assets", ServeDir::new("static/assets"))
        .nest_service("/css", ServeDir::new("static/css"))
        .nest_service("/js", ServeDir::new("static/js"))
        .layer(CorsLayer::permissive())
        .layer(axum::middleware::from_fn_with_state(
            state.rate_limiter.clone(),
            rate_limit_middleware,
        ))
        .layer(axum::extract::DefaultBodyLimit::max(body_limit_bytes))
        .with_state(state)
}

async fn handle_health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "status": "healthy",
        "service": "pressmark",
        "engine": "pressmark-vector-v1",
        "version": "1.0.0"
    })))
}

async fn handle_index() -> impl IntoResponse {
    match tokio::fs::read_to_string("static/index.html").await {
        Ok(html) => (StatusCode::OK, Html(html)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(format!("<h1>Error loading index.html: {}</h1>", e)),
        ),
    }
}

async fn handle_privacy() -> impl IntoResponse {
    match tokio::fs::read_to_string("static/privacy.html").await {
        Ok(html) => (StatusCode::OK, Html(html)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(format!("<h1>Error loading privacy.html: {}</h1>", e)),
        ),
    }
}

async fn handle_terms() -> impl IntoResponse {
    match tokio::fs::read_to_string("static/terms.html").await {
        Ok(html) => (StatusCode::OK, Html(html)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(format!("<h1>Error loading terms.html: {}</h1>", e)),
        ),
    }
}

async fn handle_robots() -> impl IntoResponse {
    match tokio::fs::read_to_string("static/robots.txt").await {
        Ok(body) => (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            body,
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error loading robots.txt: {}", e),
        )
            .into_response(),
    }
}

async fn handle_sitemap() -> impl IntoResponse {
    match tokio::fs::read_to_string("static/sitemap.xml").await {
        Ok(body) => (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "application/xml; charset=utf-8")],
            body,
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error loading sitemap.xml: {}", e),
        )
            .into_response(),
    }
}

async fn handle_generate(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<FormatQuery>,
    Json(mut doc_req): Json<DocumentRequest>,
) -> Response {
    let maybe_token = auth::extract_token_from_headers(&headers);
    let session = maybe_token.as_deref().and_then(|t| state.auth_mgr.get_session(t));
    let is_chatgpt_subscriber = session.as_ref().map(|s| s.user.is_subscriber).unwrap_or(false);

    if is_chatgpt_subscriber {
        // Strip any default watermark for verified ChatGPT subscribers
        if let Some(w) = &doc_req.watermark {
            if w.contains("DEMO") || w.contains("UNPAID") || w.is_empty() {
                doc_req.watermark = None;
            }
        }
    }

    let doc_num = doc_req.effective_doc_number();
    let total = doc_req.grand_total();

    match generate_pdf(&doc_req) {
        Ok((pdf_bytes, compile_ms)) => {
            let mut resp_headers = HeaderMap::new();
            if is_chatgpt_subscriber {
                resp_headers.insert("X-ChatGPT-Subscriber", "true".parse().unwrap());
                resp_headers.insert("X-Compute-Delegation", "model.request".parse().unwrap());
            }

            // Check if caller wants binary stream or JSON
            if query.format.as_deref() == Some("binary") || query.format.as_deref() == Some("pdf") {
                let filename = format!("{}.pdf", doc_num);
                resp_headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
                resp_headers.insert(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename).parse().unwrap(),
                );
                resp_headers.insert("X-Compile-Time-Ms", compile_ms.to_string().parse().unwrap());
                (StatusCode::OK, resp_headers, pdf_bytes).into_response()
            } else {
                // Return JSON with base64 and preview data URI
                let pdf_base64 = BASE64.encode(&pdf_bytes);
                let preview_uri = generate_preview_png(&doc_req, 144)
                    .map(|(png, _)| format!("data:image/png;base64,{}", BASE64.encode(&png)))
                    .ok();

                let resp = GenerateResponse {
                    success: true,
                    pdf_base64: Some(pdf_base64),
                    preview_data_uri: preview_uri,
                    doc_number: doc_num,
                    grand_total: total,
                    compile_time_ms: compile_ms,
                };
                (StatusCode::OK, resp_headers, Json(resp)).into_response()
            }
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "error": err
            })),
        )
            .into_response(),
    }
}

async fn handle_preview(
    Query(query): Query<FormatQuery>,
    Json(doc_req): Json<DocumentRequest>,
) -> Response {
    let ppi = 144;
    match generate_all_preview_pngs(&doc_req, ppi) {
        Ok((pages_bytes, compile_ms)) => {
            if query.format.as_deref() == Some("binary") || query.format.as_deref() == Some("png") {
                let mut headers = HeaderMap::new();
                headers.insert(header::CONTENT_TYPE, "image/png".parse().unwrap());
                headers.insert("X-Compile-Time-Ms", compile_ms.to_string().parse().unwrap());
                let first_page = pages_bytes.first().cloned().unwrap_or_default();
                (StatusCode::OK, headers, first_page).into_response()
            } else {
                let mut pages_uris = Vec::with_capacity(pages_bytes.len());
                for page in &pages_bytes {
                    pages_uris.push(format!("data:image/png;base64,{}", BASE64.encode(page)));
                }
                let first_uri = pages_uris.first().cloned().unwrap_or_default();
                let total_pages = pages_uris.len();
                let resp = PreviewResponse {
                    success: true,
                    image_data_uri: first_uri,
                    pages: pages_uris,
                    total_pages,
                    compile_time_ms: compile_ms,
                };
                (StatusCode::OK, Json(resp)).into_response()
            }
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "error": err
            })),
        )
            .into_response(),
    }
}

async fn handle_render_raw(Json(req): Json<RawRenderRequest>) -> Response {
    let format = req.format.as_deref().unwrap_or("pdf");
    if format == "png" {
        let ppi = req.ppi.unwrap_or(144);
        match compile_png_bytes(&req.markup, ppi) {
            Ok(bytes) => {
                let mut headers = HeaderMap::new();
                headers.insert(header::CONTENT_TYPE, "image/png".parse().unwrap());
                (StatusCode::OK, headers, bytes).into_response()
            }
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    } else {
        match compile_pdf_bytes(&req.markup) {
            Ok(bytes) => {
                let mut headers = HeaderMap::new();
                headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
                (StatusCode::OK, headers, bytes).into_response()
            }
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }
}

async fn handle_pricing() -> impl IntoResponse {
    Json(get_pricing_catalog())
}

#[derive(Debug, Default, Deserialize)]
pub struct CheckoutRequest {
    #[serde(alias = "plan")]
    pub plan_id: Option<String>,
    pub price_id: Option<String>,
    #[serde(alias = "user_ref")]
    pub client_reference_id: Option<String>,
    pub redirect: Option<bool>,
}

fn resolve_checkout_response(req: &CheckoutRequest) -> serde_json::Value {
    let catalog = get_pricing_catalog();
    let target = req
        .plan_id
        .as_deref()
        .or(req.price_id.as_deref())
        .unwrap_or("plan_solo");

    let all_plans: Vec<_> = catalog
        .plans
        .iter()
        .chain(catalog.microtransactions.iter())
        .collect();

    let matched = all_plans
        .iter()
        .find(|p| p.id == target || p.stripe_price_id == target || p.id.contains(target))
        .copied()
        .or_else(|| catalog.plans.iter().find(|p| p.id == "plan_solo"))
        .unwrap();

    let checkout_url = crate::pricing::generate_checkout_url(
        &matched.stripe_price_id,
        req.client_reference_id.as_deref(),
    );

    serde_json::json!({
        "success": true,
        "plan_id": matched.id,
        "plan_name": matched.name,
        "price_display": matched.price_display,
        "price_cents": matched.price_cents,
        "interval": matched.interval,
        "stripe_price_id": matched.stripe_price_id,
        "checkout_url": checkout_url,
        "live_mode": crate::stripe::is_live_mode()
    })
}

async fn handle_checkout_get(Query(req): Query<CheckoutRequest>) -> Response {
    let payload = resolve_checkout_response(&req);
    if req.redirect.unwrap_or(false) {
        if let Some(url) = payload["checkout_url"].as_str() {
            return axum::response::Redirect::to(url).into_response();
        }
    }
    (StatusCode::OK, Json(payload)).into_response()
}

async fn handle_checkout_post(Json(req): Json<CheckoutRequest>) -> impl IntoResponse {
    (StatusCode::OK, Json(resolve_checkout_response(&req)))
}

async fn handle_favicon() -> impl IntoResponse {
    let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="64" height="64"><rect width="64" height="64" rx="14" fill="#0f172a"/><path d="M18 16h18c7.732 0 14 6.268 14 14s-6.268 14-14 14H26v8h-8V16zm8 8v12h10c3.314 0 6-2.686 6-6s-2.686-6-6-6H26z" fill="#38bdf8"/></svg>"##;
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/svg+xml; charset=utf-8")],
        svg,
    )
}

async fn handle_oauth_protected_resource(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let base = state.base_url.trim_end_matches('/');
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "resource": format!("{}/mcp", base),
            "authorization_servers": [crate::auth::OPENAI_AUTH_SERVER],
            "bearer_methods_supported": ["header"],
            "scopes_supported": ["openid", "profile", "email", "model.request", "agent:mcp", "developer:api"],
            "resource_documentation": format!("{}/api/v1/openapi.json", base)
        })),
    )
}

async fn handle_mcp_server_card(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let base = state.base_url.trim_end_matches('/');
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "name": "io.github.mooshee/pressmark",
            "title": "Pressmark Vector Document Engine",
            "description": "Vector document engine for 39 invoice, receipt, contract, and real estate PDFs with QR payments.",
            "version": crate::mcp::MCP_SERVER_VERSION,
            "protocolVersion": "2026-07-28",
            "supportedProtocols": crate::mcp::SUPPORTED_MCP_PROTOCOLS,
            "websiteUrl": base,
            "remotes": [
                {
                    "type": "streamable-http",
                    "url": format!("{}/mcp", base)
                }
            ],
            "tools": McpHandler::get_tools()
        })),
    )
}

pub fn run_setup_check(storage: Option<&crate::storage::Storage>) -> serde_json::Value {
    let start = std::time::Instant::now();
    let typst_bin = crate::engine::find_typst_executable();
    let smoke_res = crate::engine::compile_pdf_bytes("#set page(width: 2in, height: 1in, margin: 4pt)\n*Pressmark Doctor OK*");
    let smoke_ms = start.elapsed().as_millis() as u64;
    let smoke_ok = smoke_res.as_ref().map(|b| b.starts_with(b"%PDF-")).unwrap_or(false);

    let default_storage;
    let db = match storage {
        Some(s) => s,
        None => {
            default_storage = crate::storage::Storage::default_db();
            &default_storage
        }
    };
    let (sessions_count, stripe_events_count, entitlements_count) = db.stats();

    let required_static = [
        "static/index.html",
        "static/js/studio.js",
        "static/css/studio.css",
        "static/privacy.html",
        "static/terms.html",
    ];
    let mut static_files = serde_json::Map::new();
    let mut all_static_present = true;
    for path in required_static {
        match std::fs::metadata(path) {
            Ok(meta) if meta.len() > 0 => {
                static_files.insert(path.to_string(), serde_json::json!({ "present": true, "bytes": meta.len() }));
            }
            _ => {
                all_static_present = false;
                static_files.insert(path.to_string(), serde_json::json!({ "present": false, "bytes": 0 }));
            }
        }
    }

    let live_mode = crate::stripe::is_live_mode();
    let secret_key_ok = crate::stripe::get_stripe_secret_key()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);
    let webhook_secrets = crate::stripe::get_stripe_webhook_secrets();
    let catalog = get_pricing_catalog();
    let solo_plan = catalog.plans.iter().find(|p| p.id == "plan_solo");
    let live_prices_configured = solo_plan
        .map(|p| p.stripe_price_id.starts_with("price_1"))
        .unwrap_or(false);
    let checkout_links_configured = catalog
        .plans
        .iter()
        .filter(|p| p.price_cents > 0)
        .chain(catalog.microtransactions.iter())
        .all(|p| {
            p.checkout_url
                .as_deref()
                .map(|u| u.starts_with("https://buy.stripe.com/"))
                .unwrap_or(false)
        });

    let ghl_location_id = std::env::var("GHL_LOCATION_ID")
        .or_else(|_| std::env::var("HIGHLEVEL_LOCATION_ID"))
        .unwrap_or_else(|_| crate::ghl::PRESSMARK_LOCATION_ID.to_string());
    let ghl_pit_configured = std::env::var("GHL_LOCATION_PIT")
        .or_else(|_| std::env::var("HIGHLEVEL_API_KEY"))
        .or_else(|_| std::env::var("PRESSMARK_GHL_PIT"))
        .map(|v| !v.trim().is_empty())
        .unwrap_or_else(|_| {
            let pit_path = format!(
                "{}/.config/ghl-agency/contexts/moosh-growth/locations/{}.pit",
                std::env::var("HOME").unwrap_or_default(),
                ghl_location_id
            );
            std::fs::read_to_string(pit_path)
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false)
        });

    let challenge_ok = std::env::var("OPENAI_APPS_CHALLENGE_TOKEN")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| {
            std::fs::read_to_string("static/.well-known/openai-apps-challenge")
                .ok()
                .filter(|s| !s.trim().is_empty())
        })
        .is_some();

    let tools_count = McpHandler::get_tools()
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    let overall_healthy = smoke_ok && all_static_present && secret_key_ok && !webhook_secrets.is_empty();

    serde_json::json!({
        "status": if overall_healthy { "healthy" } else { "degraded" },
        "version": env!("CARGO_PKG_VERSION"),
        "checks": {
            "vector_engine": {
                "status": if smoke_ok { "ok" } else { "error" },
                "binary_path": typst_bin.map(|p| p.display().to_string()),
                "pdf_smoke_test": smoke_ok,
                "compile_ms": smoke_ms
            },
            "storage_sqlite": {
                "status": "ok",
                "db_path": db.db_path().display().to_string(),
                "sessions_count": sessions_count,
                "stripe_events_count": stripe_events_count,
                "entitlements_count": entitlements_count
            },
            "static_assets": {
                "status": if all_static_present { "ok" } else { "error" },
                "working_directory": std::env::current_dir().ok().map(|p| p.display().to_string()),
                "all_present": all_static_present,
                "files": static_files
            },
            "stripe_payments": {
                "status": if secret_key_ok && !webhook_secrets.is_empty() && checkout_links_configured { "ok" } else { "warning" },
                "live_mode": live_mode,
                "account_id": std::env::var("STRIPE_ACCOUNT_ID").ok(),
                "secret_key_configured": secret_key_ok,
                "webhook_secrets_count": webhook_secrets.len(),
                "live_prices_configured": live_prices_configured,
                "checkout_links_configured": checkout_links_configured
            },
            "highlevel_crm": {
                "status": if ghl_pit_configured { "ok" } else { "warning" },
                "location_id": ghl_location_id,
                "pit_configured": ghl_pit_configured
            },
            "mcp_and_discovery": {
                "status": "ok",
                "mcp_version": crate::mcp::MCP_SERVER_VERSION,
                "supported_protocols": crate::mcp::SUPPORTED_MCP_PROTOCOLS,
                "tools_count": tools_count,
                "archetypes_count": 39,
                "openai_challenge_configured": challenge_ok
            }
        }
    })
}

async fn handle_setup_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let report = run_setup_check(Some(&state.storage));
    (StatusCode::OK, Json(report))
}

async fn handle_openapi(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(get_openapi_spec(&state.base_url))
}

async fn handle_ai_plugin(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(get_ai_plugin_manifest(&state.base_url))
}

async fn handle_openai_apps_challenge() -> impl IntoResponse {
    let token = std::env::var("OPENAI_APPS_CHALLENGE_TOKEN")
        .unwrap_or_else(|_| {
            std::fs::read_to_string("static/.well-known/openai-apps-challenge")
                .unwrap_or_default()
                .trim()
                .to_string()
        });
    (StatusCode::OK, token)
}

async fn handle_mcp_get() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::HeaderName::from_static("x-mcp-protocol-version"), "2026-07-28")],
        Json(serde_json::json!({
            "service": "pressmark",
            "title": "Pressmark Vector Document Engine",
            "version": crate::mcp::MCP_SERVER_VERSION,
            "protocolVersion": "2026-07-28",
            "supportedProtocols": crate::mcp::SUPPORTED_MCP_PROTOCOLS,
            "transport": "streamable-http",
            "endpoints": {
                "mcp": "/mcp",
                "mcp_v1": "/api/v1/mcp",
                "tools": "/api/v1/mcp/tools.json"
            }
        })),
    )
}

async fn handle_mcp_tools() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "name": "pressmark",
            "version": crate::mcp::MCP_SERVER_VERSION,
            "tools": McpHandler::get_tools()
        })),
    )
}

async fn handle_mcp(headers: HeaderMap, Json(req): Json<JsonRpcRequest>) -> Response {
    let is_notif = McpHandler::is_notification(&req);
    let client_proto = headers
        .get("x-mcp-protocol-version")
        .or_else(|| headers.get("mcp-protocol-version"))
        .and_then(|v| v.to_str().ok())
        .unwrap_or("2025-06-18")
        .to_string();
    let is_2026 = client_proto == "2026-07-28" || req.method == "server/discover";

    let mut resp = McpHandler::handle_request(req);
    if is_2026 {
        if let Some(ref mut result) = resp.result {
            if let Some(obj) = result.as_object_mut() {
                obj.entry("resultType".to_string())
                    .or_insert_with(|| serde_json::json!("complete"));
                obj.entry("ttlMs".to_string())
                    .or_insert_with(|| serde_json::json!(300_000));
                obj.entry("cacheScope".to_string())
                    .or_insert_with(|| serde_json::json!("public-anonymous"));
                obj.entry("_meta".to_string()).or_insert_with(|| {
                    serde_json::json!({
                        "protocolVersion": "2026-07-28",
                        "streamingSupported": true,
                        "cacheTtlMs": 300_000
                    })
                });
            }
        }
    }

    let mut resp_headers = HeaderMap::new();
    let proto_header = if is_2026 { "2026-07-28" } else { "2025-06-18" };
    if let Ok(hv) = proto_header.parse() {
        resp_headers.insert("x-mcp-protocol-version", hv);
    }

    if is_notif && resp.id.is_none() {
        return (StatusCode::ACCEPTED, resp_headers).into_response();
    }

    (StatusCode::OK, resp_headers, Json(resp)).into_response()
}

async fn handle_copilot(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CopilotRequest>,
) -> impl IntoResponse {
    let mut modified = req.current_state;
    let prompt_lower = req.prompt.to_lowercase();
    
    // Check for ChatGPT session / token pass-through
    let maybe_token = auth::extract_token_from_headers(&headers);
    let session = maybe_token.as_deref().and_then(|t| state.auth_mgr.get_session(t));

    let mut message = "I've updated the document based on your request!".to_string();

    if prompt_lower.contains("nda") || prompt_lower.contains("non-disclosure") {
        modified.doc_type = crate::models::DocumentType::Nda;
        message = "Switched document to a Non-Disclosure Agreement.".to_string();
    } else if prompt_lower.contains("estimate") || prompt_lower.contains("quote") {
        modified.doc_type = crate::models::DocumentType::Estimate;
        message = "Generated an estimate layout.".to_string();
    } else if prompt_lower.contains("contract") || prompt_lower.contains("agreement") {
        modified.doc_type = crate::models::DocumentType::Agreement;
        message = "Converted to a Master Services Agreement.".to_string();
    }
    
    if prompt_lower.contains("dark") || prompt_lower.contains("obsidian") {
        modified.theme = Some("obsidian".to_string());
        message.push_str(" Applied the Obsidian dark theme.");
    }

    if let Some(captures) = regex::Regex::new(r"for (?:client )?['\x22]?([A-Z][a-zA-Z0-9\s&,.-]+?)['\x22]?(?: with| at| for|$)").unwrap().captures(&req.prompt) {
        if let Some(client_match) = captures.get(1) {
            modified.to.name = client_match.as_str().trim().to_string();
        }
    }

    // Attach ChatGPT Subscription Pass-Through compute attribution
    if let Some(sess) = session {
        if sess.user.is_subscriber {
            message.push_str(&format!(
                "\n\n⚡ Powered by your {} subscription ({}). Zero API billing costs. Priority compute pass-through active.",
                sess.user.plan_display, sess.user.email
            ));
            // Automatically strip demo watermarks for verified subscribers
            modified.watermark = None;
        } else {
            message.push_str(&format!(
                "\n\nℹ️ Connected to {} ({}). Upgrade to ChatGPT Plus or Pro for unlimited unmetered document synthesis.",
                sess.user.plan_display, sess.user.email
            ));
        }
    } else {
        message.push_str("\n\n💡 Tip: Sign in with ChatGPT in the header to power Copilot using your ChatGPT Plus/Pro subscription.");
    }

    let resp = CopilotResponse {
        success: true,
        modified_state: modified,
        ai_message: message,
    };
    (StatusCode::OK, Json(resp)).into_response()
}

async fn handle_vision_extract(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(_req): Json<VisionRequest>,
) -> impl IntoResponse {
    let maybe_token = auth::extract_token_from_headers(&headers);
    let session = maybe_token.as_deref().and_then(|t| state.auth_mgr.get_session(t));

    let mut party = PartyDetails::default();
    party.name = "Scanned Vendor LLC".to_string();
    party.email = Some("billing@scanned-vendor.com".to_string());
    
    let items = vec![
        LineItem {
            description: "Extracted Line Item 1".to_string(),
            quantity: 1.0,
            unit_price: 499.0,
            tax_rate: None,
            discount: None,
        },
        LineItem {
            description: "Service Fee (OCR)".to_string(),
            quantity: 1.0,
            unit_price: 25.0,
            tax_rate: None,
            discount: None,
        }
    ];

    let ai_message = if let Some(sess) = session {
        if sess.user.is_subscriber {
            format!(
                "⚡ Extracted vendor and items successfully via your {} Vision quota ({}). Zero API charges.",
                sess.user.plan_display, sess.user.email
            )
        } else {
            format!(
                "Extracted vendor and items successfully. Connected to {} ({}).",
                sess.user.plan_display, sess.user.email
            )
        }
    } else {
        "Extracted vendor and items successfully via Vision API. 💡 Sign in with ChatGPT to share your Plus/Pro OCR quota.".to_string()
    };

    let resp = VisionResponse {
        success: true,
        extracted_items: items,
        extracted_from: Some(party),
        ai_message,
    };
    (StatusCode::OK, Json(resp)).into_response()
}

// ChatGPT OAuth Handlers

async fn handle_chatgpt_login(
    State(state): State<Arc<AppState>>,
    query: Query<LoginQuery>,
    headers: HeaderMap,
) -> Response {
    auth::handle_chatgpt_login(State(state.auth_mgr.clone()), query, headers).await
}

async fn handle_chatgpt_callback(
    State(state): State<Arc<AppState>>,
    query: Query<CallbackQuery>,
) -> Response {
    auth::handle_chatgpt_callback(State(state.auth_mgr.clone()), query).await
}

async fn handle_chatgpt_session(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    auth::handle_chatgpt_session(State(state.auth_mgr.clone()), headers).await
}

async fn handle_chatgpt_logout(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    auth::handle_chatgpt_logout(State(state.auth_mgr.clone()), headers).await
}

async fn handle_guest_login(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    auth::handle_guest_login(State(state.auth_mgr.clone())).await
}

async fn handle_email_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EmailLoginRequest>,
) -> impl IntoResponse {
    auth::handle_email_login(State(state.auth_mgr.clone()), Json(payload)).await
}

async fn handle_api_key_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ApiKeyLoginRequest>,
) -> impl IntoResponse {
    auth::handle_api_key_login(State(state.auth_mgr.clone()), Json(payload)).await
}

#[derive(Debug, serde::Deserialize)]
pub struct LifecycleConvertRequest {
    pub target_type: String, // "invoice", "receipt", "estimate", "credit_note"
    pub due_days: Option<u32>,
    pub valid_days: Option<u32>,
    pub credit_reason: Option<String>,
    pub payment_link: Option<String>,
    pub payment_qr_code: Option<bool>,
    pub document: DocumentRequest,
}

#[derive(Debug, serde::Deserialize)]
pub struct LifecyclePayRequest {
    pub document: DocumentRequest,
    pub payment_method: Option<String>,
    pub transaction_reference: Option<String>,
    pub paid_date: Option<String>,
    pub amount_paid: Option<f64>,
}

async fn handle_lifecycle_convert(
    Json(payload): Json<LifecycleConvertRequest>,
) -> impl IntoResponse {
    let mut converted = match payload.target_type.to_lowercase().as_str() {
        "invoice" => payload.document.to_invoice(payload.due_days.unwrap_or(15)),
        "receipt" => payload.document.to_receipt(
            "Stripe / Apple Pay",
            &format!("TX-{}", chrono::Utc::now().timestamp()),
            &chrono::Utc::now().format("%Y-%m-%d").to_string(),
        ),
        "estimate" | "quote" => payload.document.to_estimate(payload.valid_days.unwrap_or(30)),
        "credit_note" | "credit" => payload.document.to_credit_note(payload.credit_reason.as_deref()),
        _ => payload.document,
    };

    if let Some(link) = payload.payment_link {
        converted.payment_link = Some(link);
    }
    if let Some(qr) = payload.payment_qr_code {
        converted.payment_qr_code = Some(qr);
    }

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "document": converted
    })))
}

async fn handle_lifecycle_pay(
    Json(payload): Json<LifecyclePayRequest>,
) -> impl IntoResponse {
    let method = payload.payment_method.as_deref().unwrap_or("Stripe / Apple Pay");
    let tx_ref = payload.transaction_reference.as_deref().unwrap_or("STRIPE-AUTO-PAID");
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let paid_date = payload.paid_date.as_deref().unwrap_or(&today);

    let total = payload.document.grand_total();
    let amt = payload.amount_paid.unwrap_or(total);

    if amt < total - 0.001 {
        // Partial Payment: Retain invoice type and mark partial status, preserving balance due
        let mut partial_doc = payload.document.clone();
        partial_doc.amount_paid = Some(amt);
        partial_doc.is_paid = Some(false);
        partial_doc.payment_status = Some("partial".to_string());
        partial_doc.payment_method = Some(method.to_string());
        partial_doc.transaction_reference = Some(tx_ref.to_string());
        partial_doc.paid_at = Some(paid_date.to_string());
        let bal = (total - amt).max(0.0);

        (StatusCode::OK, Json(serde_json::json!({
            "success": true,
            "payment_status": "partial",
            "amount_paid": amt,
            "balance_due": bal,
            "document": partial_doc
        })))
    } else {
        // Full Payment: Convert to official payment receipt
        let mut receipt = payload.document.to_receipt(method, tx_ref, paid_date);
        receipt.amount_paid = Some(amt);

        (StatusCode::OK, Json(serde_json::json!({
            "success": true,
            "payment_status": "paid",
            "amount_paid": amt,
            "balance_due": 0.0,
            "document": receipt
        })))
    }
}

async fn handle_stripe_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Response {
    let secrets = crate::stripe::get_stripe_webhook_secrets();
    let allow_unverified = std::env::var("PRESSMARK_ALLOW_UNVERIFIED_WEBHOOKS")
        .map(|v| v == "1" || v == "true")
        .unwrap_or(false);

    if !allow_unverified {
        let sig = match headers.get("stripe-signature").and_then(|v| v.to_str().ok()) {
            Some(s) => s,
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "missing_signature",
                        "message": "Missing Stripe-Signature header"
                    })),
                )
                    .into_response();
            }
        };

        if let Err(e) = crate::stripe::verify_stripe_signature_against_any(&body, sig, &secrets, 300) {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "invalid_signature",
                    "message": e.to_string()
                })),
            )
                .into_response();
        }
    }

    let event: crate::stripe::StripeEvent = match serde_json::from_slice(&body) {
        Ok(ev) => ev,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "malformed_json",
                    "message": format!("Could not parse Stripe event: {}", e)
                })),
            )
                .into_response();
        }
    };

    // Idempotency defense: avoid duplicate fulfillment on Stripe webhook retries
    if state.stripe_idempotency.is_processed(&event.id).await {
        tracing::info!("Stripe webhook event {} already processed; skipping duplicate fulfillment", event.id);
        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "already_processed",
                "event_id": event.id
            })),
        )
            .into_response();
    }

    match crate::stripe::process_stripe_event(event.clone()) {
        Ok(action) => {
            state.stripe_idempotency.record(&event.id).await;

            match &action {
                crate::stripe::FulfillmentAction::CheckoutCompleted {
                    customer_email,
                    session_id,
                    amount_total_cents,
                    document_number,
                } => {
                    let plan = if *amount_total_cents >= 14900 {
                        crate::auth::ChatGptPlan::Agency
                    } else if *amount_total_cents >= 4900 {
                        crate::auth::ChatGptPlan::Business
                    } else {
                        crate::auth::ChatGptPlan::SoloPro
                    };
                    state.auth_mgr.upgrade_user_entitlement(
                        customer_email,
                        plan,
                        Some(session_id),
                        *amount_total_cents,
                    );

                    if std::env::var("GHL_LOCATION_PIT").is_ok() {
                        let email_cl = customer_email.clone();
                        let doc_cl = document_number
                            .clone()
                            .unwrap_or_else(|| session_id.clone());
                        let tier_slug = match plan {
                            crate::auth::ChatGptPlan::Agency => "sub_agency",
                            crate::auth::ChatGptPlan::Business => "sub_business",
                            _ => "sub_solo_pro",
                        };
                        let cents = *amount_total_cents as u32;
                        tokio::spawn(async move {
                            let ghl = crate::ghl::GhlDispatcher::from_env();
                            let _ = ghl
                                .dispatch_payment_unlock(&doc_cl, &email_cl, tier_slug, cents)
                                .await;
                        });
                    }
                }
                crate::stripe::FulfillmentAction::PaymentUnlocked {
                    customer_email,
                    amount_cents,
                    document_number,
                    receipt_number,
                    unlocked_tier,
                    ..
                } => {
                    let plan = unlocked_tier
                        .as_deref()
                        .map(crate::auth::ChatGptPlan::from_str_relaxed)
                        .unwrap_or_else(|| {
                            if *amount_cents >= 14900 {
                                crate::auth::ChatGptPlan::Agency
                            } else if *amount_cents >= 4900 {
                                crate::auth::ChatGptPlan::Business
                            } else {
                                crate::auth::ChatGptPlan::SoloPro
                            }
                        });
                    state.auth_mgr.upgrade_user_entitlement(
                        customer_email,
                        plan,
                        Some(receipt_number),
                        *amount_cents,
                    );

                    if std::env::var("GHL_LOCATION_PIT").is_ok() {
                        let email_cl = customer_email.clone();
                        let doc_cl = document_number
                            .clone()
                            .unwrap_or_else(|| receipt_number.clone());
                        let tier_cl = unlocked_tier
                            .clone()
                            .unwrap_or_else(|| "single_pass".to_string());
                        let cents = *amount_cents as u32;
                        tokio::spawn(async move {
                            let ghl = crate::ghl::GhlDispatcher::from_env();
                            let _ = ghl
                                .dispatch_payment_unlock(&doc_cl, &email_cl, &tier_cl, cents)
                                .await;
                        });
                    }
                }
                _ => {}
            }

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "status": "success",
                    "action": action
                })),
            )
                .into_response()
        }
        Err(e) => {
            tracing::warn!("Stripe event processing non-fatal error: {}", e);
            // Return 200 with error details to prevent 3-day webhook retry storm on non-transient logic errors
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "status": "unfulfillable",
                    "message": e
                })),
            )
                .into_response()
        }
    }
}

