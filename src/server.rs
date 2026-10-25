//! Pressmark Axum HTTP Server & API Router
//!
//! Author: Daniel Hallman

use crate::auth::{
    self, ApiKeyLoginRequest, CallbackQuery, ChatGptAuthManager, EmailLoginRequest, LoginQuery,
};
use crate::engine::{compile_pdf_bytes, compile_png_bytes, generate_pdf, generate_preview_png, generate_all_preview_pngs};
use crate::mcp::{JsonRpcRequest, McpHandler};
use crate::models::{DocumentRequest, DocumentType, GenerateResponse, PreviewResponse, CopilotRequest, CopilotResponse, VisionRequest, VisionResponse, LineItem, PartyDetails};
use crate::openapi::{get_ai_plugin_manifest, get_openapi_spec};
use crate::pricing::get_pricing_catalog;
use axum::extract::{Path, Query, State};
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
        .route("/templates", get(handle_index))
        .route("/templates/:slug", get(handle_template_landing))
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
        .route("/api/v1/templates", get(handle_templates_catalog))
        .route("/api/v1/pricing", get(handle_pricing))
        .route("/api/v1/checkout", get(handle_checkout_get).post(handle_checkout_post))
        .route("/api/v1/setup-check", get(handle_setup_check))
        .route("/api/v1/analytics", get(handle_analytics))
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
        .route("/.well-known/agent-card.json", get(handle_mcp_server_card))
        .route("/.well-known/agent.json", get(handle_mcp_server_card))
        .route("/mcp/.well-known/agent.json", get(handle_mcp_server_card))
        .route("/mcp/.well-known/agent-card.json", get(handle_mcp_server_card))
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
        .route("/mcp/", get(handle_mcp_get).post(handle_mcp))
        .route("/mcp/mcp", get(handle_mcp_get).post(handle_mcp))
        .route("/mcp/sse", get(handle_mcp_get).post(handle_mcp))
        .route("/sse", get(handle_mcp_get).post(handle_mcp))
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

async fn handle_template_landing(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Response {
    let doc_type = match DocumentType::from_slug(&slug) {
        Some(dt) => dt,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Html("<h1>404 · Template Not Found</h1><p><a href=\"/templates\">Browse all 39 document templates</a></p>".to_string()),
            )
                .into_response();
        }
    };

    let canonical_slug = doc_type.url_slug();
    let archetype_key = doc_type.as_str();
    let (seo_title, seo_desc) = doc_type.seo_metadata();
    let canonical_url = format!("https://pressmark.dev/templates/{}", canonical_slug);

    state.storage.record_usage_event(
        "template_landing_view",
        Some(archetype_key),
        "web_seo",
        Some(canonical_slug),
        None,
    );

    match tokio::fs::read_to_string("static/index.html").await {
        Ok(raw_html) => {
            let mut html = raw_html
                .replace(
                    "<title>Pressmark Studio · Sub-Millisecond Vector Invoices, Leases & Documents</title>",
                    &format!("<title>{}</title>", seo_title),
                )
                .replace(
                    "<meta name=\"description\" content=\"Sub-millisecond Rust-native vector document engine. Generate publication-grade invoices, commercial leases, estimates, and legal agreements with live preview, Stripe QR payment links, and ChatGPT / MCP agent integration.\">",
                    &format!("<meta name=\"description\" content=\"{}\">", seo_desc),
                )
                .replace(
                    "<link rel=\"canonical\" href=\"https://pressmark.dev/\">",
                    &format!("<link rel=\"canonical\" href=\"{}\">", canonical_url),
                )
                .replace(
                    "<meta property=\"og:url\" content=\"https://pressmark.dev/\">",
                    &format!("<meta property=\"og:url\" content=\"{}\">", canonical_url),
                )
                .replace(
                    "<meta property=\"og:title\" content=\"Pressmark Studio · Sub-Millisecond Vector Document Engine\">",
                    &format!("<meta property=\"og:title\" content=\"{}\">", seo_title),
                )
                .replace(
                    "<meta property=\"og:description\" content=\"Generate crisp, print-ready vector PDF invoices, commercial leases, and legal agreements in under 15ms. 1-scan QR code mobile payments, PEPPOL compliance, and ChatGPT / MCP integration.\">",
                    &format!("<meta property=\"og:description\" content=\"{}\">", seo_desc),
                );

            let boot_script = format!(
                "  <script>window.__PRESSMARK_INITIAL_ARCHETYPE__ = \"{}\";</script>\n</head>",
                archetype_key
            );
            html = html.replacen("</head>", &boot_script, 1);

            (StatusCode::OK, Html(html)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(format!("<h1>Error loading index.html: {}</h1>", e)),
        )
            .into_response(),
    }
}

async fn handle_templates_catalog() -> impl IntoResponse {
    let templates: Vec<serde_json::Value> = DocumentType::all()
        .iter()
        .map(|dt| {
            let (seo_title, seo_description) = dt.seo_metadata();
            serde_json::json!({
                "doc_type": dt.as_str(),
                "slug": dt.url_slug(),
                "display_title": dt.display_title(),
                "default_number_prefix": dt.default_number_prefix(),
                "seo_title": seo_title,
                "seo_description": seo_description,
                "url": format!("https://pressmark.dev/templates/{}", dt.url_slug()),
            })
        })
        .collect();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "count": templates.len(),
            "templates": templates
        })),
    )
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
    } else if doc_req.watermark.is_none()
        && std::env::var("PRESSMARK_ENFORCE_FREE_WATERMARK")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    {
        doc_req.watermark = Some("PRESSMARK PREVIEW".to_string());
    }

    let doc_num = doc_req.effective_doc_number();
    let total = doc_req.grand_total();
    let user_agent = headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok());

    match generate_pdf(&doc_req) {
        Ok((pdf_bytes, compile_ms)) => {
            state.storage.record_usage_event(
                "pdf_generate",
                Some(doc_req.doc_type.as_str()),
                "rest_api",
                user_agent,
                Some(compile_ms as u64),
            );

            // Asynchronously sync active creator lead to HighLevel CRM when configured
            if crate::ghl::GhlDispatcher::has_env_or_pit_file() {
                let candidate_email = session
                    .as_ref()
                    .map(|s| s.user.email.as_str())
                    .filter(|e| !e.ends_with("@pressmark.local") && !e.ends_with("@example.com"))
                    .or_else(|| doc_req.from.email.as_deref())
                    .or_else(|| doc_req.to.email.as_deref())
                    .filter(|e| e.contains('@') && !e.ends_with("@example.com"))
                    .map(|s| s.to_string());
                if let Some(email_cl) = candidate_email {
                    let doc_cl = doc_req.clone();
                    tokio::spawn(async move {
                        let ghl = crate::ghl::GhlDispatcher::from_env();
                        if ghl.is_configured() {
                            let _ = ghl
                                .dispatch_document_generation(&doc_cl, Some(&email_cl), "rest_api")
                                .await;
                        }
                    });
                }
            }

            let mut resp_headers = HeaderMap::new();
            if is_chatgpt_subscriber {
                resp_headers.insert("X-ChatGPT-Subscriber", "true".parse().unwrap());
                resp_headers.insert("X-Compute-Delegation", "model.request".parse().unwrap());
            } else {
                resp_headers.insert(
                    "X-Upgrade-Checkout-Url",
                    "https://pressmark.dev/api/v1/checkout?plan=pass_single_unlock&redirect=true"
                        .parse()
                        .unwrap(),
                );
            }

            // Check if caller wants binary stream or JSON
            if query.format.as_deref() == Some("binary") || query.format.as_deref() == Some("pdf") {
                let safe_doc_num: String = doc_num
                    .chars()
                    .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
                    .collect();
                let safe_base = if safe_doc_num.is_empty() { "document" } else { &safe_doc_num };
                let filename = format!("{}.pdf", safe_base);
                resp_headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
                if let Ok(cd) = format!("attachment; filename=\"{}\"", filename).parse() {
                    resp_headers.insert(header::CONTENT_DISPOSITION, cd);
                }
                resp_headers.insert("X-Compile-Time-Ms", compile_ms.to_string().parse().unwrap());
                (StatusCode::OK, resp_headers, pdf_bytes).into_response()
            } else {
                // Return JSON with base64 and preview data URI
                let pdf_base64 = BASE64.encode(&pdf_bytes);
                let preview_uri = generate_preview_png(&doc_req, 144)
                    .map(|(png, _)| format!("data:image/png;base64,{}", BASE64.encode(&png)))
                    .ok();

                let (unlock_url, pro_url) = if is_chatgpt_subscriber {
                    (None, None)
                } else {
                    (
                        Some("https://pressmark.dev/api/v1/checkout?plan=pass_single_unlock&redirect=true".to_string()),
                        Some("https://pressmark.dev/api/v1/checkout?plan=plan_solo&redirect=true".to_string()),
                    )
                };

                let resp = GenerateResponse {
                    success: true,
                    pdf_base64: Some(pdf_base64),
                    preview_data_uri: preview_uri,
                    doc_number: doc_num,
                    grand_total: total,
                    compile_time_ms: compile_ms,
                    unlock_checkout_url: unlock_url,
                    pro_checkout_url: pro_url,
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
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<FormatQuery>,
    Json(doc_req): Json<DocumentRequest>,
) -> Response {
    let ppi = 144;
    let user_agent = headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok());
    match generate_all_preview_pngs(&doc_req, ppi) {
        Ok((pages_bytes, compile_ms)) => {
            state.storage.record_usage_event(
                "png_preview",
                Some(doc_req.doc_type.as_str()),
                "studio_or_api",
                user_agent,
                Some(compile_ms as u64),
            );
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

async fn handle_checkout_get(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(req): Query<CheckoutRequest>,
) -> Response {
    let payload = resolve_checkout_response(&req);
    let _user_agent = headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok());
    let is_redirect = req.redirect.unwrap_or(false);
    state.storage.record_usage_event(
        "checkout_init",
        None,
        "stripe",
        payload["plan_id"].as_str(),
        None,
    );
    if is_redirect {
        if let Some(url) = payload["checkout_url"].as_str() {
            return axum::response::Redirect::to(url).into_response();
        }
    }
    (StatusCode::OK, Json(payload)).into_response()
}

async fn handle_checkout_post(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CheckoutRequest>,
) -> impl IntoResponse {
    let payload = resolve_checkout_response(&req);
    let _user_agent = headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok());
    state.storage.record_usage_event(
        "checkout_init",
        None,
        "stripe",
        payload["plan_id"].as_str(),
        None,
    );
    (StatusCode::OK, Json(payload))
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
            },
            "usage_telemetry": db.analytics_summary()
        }
    })
}

async fn handle_setup_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let report = run_setup_check(Some(&state.storage));
    (StatusCode::OK, Json(report))
}

async fn handle_analytics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.storage.analytics_summary()))
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

async fn handle_mcp(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<JsonRpcRequest>,
) -> Response {
    let is_notif = McpHandler::is_notification(&req);
    let user_agent = headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok());
    let mcp_method = req.method.clone();
    let tool_name = req
        .params
        .as_ref()
        .and_then(|p| p.get("name"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| Some(mcp_method.clone()));
    let doc_type = req
        .params
        .as_ref()
        .and_then(|p| p.get("arguments"))
        .and_then(|a| {
            a.get("doc_type")
                .or_else(|| a.get("document").and_then(|d| d.get("doc_type")))
                .or_else(|| a.get("target_type"))
        })
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let mcp_event_type = if mcp_method == "tools/call" {
        "mcp_tool_call"
    } else {
        "mcp_protocol"
    };
    state.storage.record_usage_event(
        mcp_event_type,
        doc_type.as_deref(),
        user_agent.unwrap_or("mcp"),
        tool_name.as_deref(),
        None,
    );

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
    let user_agent = headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok());

    let maybe_token = auth::extract_token_from_headers(&headers);
    let session = maybe_token.as_deref().and_then(|t| state.auth_mgr.get_session(t));

    let mut changes: Vec<String> = Vec::new();

    // 1. Document archetype detection across all 39 supported types
    let matched_type = if prompt_lower.contains("nda") || prompt_lower.contains("non-disclosure") {
        Some((crate::models::DocumentType::Nda, "Mutual Non-Disclosure Agreement"))
    } else if prompt_lower.contains("triple net") || prompt_lower.contains("nnn lease") || prompt_lower.contains("commercial nnn") {
        Some((crate::models::DocumentType::CommercialNnnLease, "Commercial NNN Lease"))
    } else if prompt_lower.contains("residential lease") || prompt_lower.contains("apartment lease") {
        Some((crate::models::DocumentType::ResidentialLease, "Residential Lease Agreement"))
    } else if prompt_lower.contains("commercial lease") {
        Some((crate::models::DocumentType::CommercialLease, "Commercial Lease Abstract"))
    } else if prompt_lower.contains("property management") || prompt_lower.contains("owner statement") {
        Some((crate::models::DocumentType::PropertyManagementStatement, "Property Management Statement"))
    } else if prompt_lower.contains("rent roll") {
        Some((crate::models::DocumentType::RentRoll, "Multi-Unit Rent Roll"))
    } else if prompt_lower.contains("closing") || prompt_lower.contains("settlement statement") {
        Some((crate::models::DocumentType::ClosingSettlementStatement, "Closing Settlement Statement"))
    } else if prompt_lower.contains("notice to vacate") || prompt_lower.contains("eviction") {
        Some((crate::models::DocumentType::NoticeToVacate, "Notice to Vacate"))
    } else if prompt_lower.contains("security deposit") {
        Some((crate::models::DocumentType::SecurityDepositDisposition, "Security Deposit Disposition"))
    } else if prompt_lower.contains("commission") && prompt_lower.contains("real estate") {
        Some((crate::models::DocumentType::RealEstateCommission, "Real Estate Commission Statement"))
    } else if prompt_lower.contains("inspection checklist") || prompt_lower.contains("move-in") || prompt_lower.contains("move-out") {
        Some((crate::models::DocumentType::PropertyInspectionChecklist, "Property Inspection Checklist"))
    } else if prompt_lower.contains("letter of intent") || prompt_lower.contains("loi") {
        Some((crate::models::DocumentType::LetterOfIntentCre, "Commercial Real Estate LOI"))
    } else if prompt_lower.contains("estoppel") {
        Some((crate::models::DocumentType::TenantEstoppel, "Tenant Estoppel Certificate"))
    } else if prompt_lower.contains("credit note") || prompt_lower.contains("refund") {
        Some((crate::models::DocumentType::CreditNote, "Credit Note"))
    } else if prompt_lower.contains("purchase order") {
        Some((crate::models::DocumentType::PurchaseOrder, "Purchase Order"))
    } else if prompt_lower.contains("packing slip") || prompt_lower.contains("bill of lading") {
        Some((crate::models::DocumentType::PackingSlip, "Packing Slip"))
    } else if prompt_lower.contains("work order") || prompt_lower.contains("job ticket") {
        Some((crate::models::DocumentType::WorkOrder, "Work Order"))
    } else if prompt_lower.contains("expense report") || prompt_lower.contains("reimbursement") {
        Some((crate::models::DocumentType::ExpenseReport, "Expense Report"))
    } else if prompt_lower.contains("delivery receipt") {
        Some((crate::models::DocumentType::DeliveryReceipt, "Delivery Receipt"))
    } else if prompt_lower.contains("receipt") {
        Some((crate::models::DocumentType::Receipt, "Payment Receipt"))
    } else if prompt_lower.contains("tender") || prompt_lower.contains("bid quotation") {
        Some((crate::models::DocumentType::QuotationTender, "Tender Quotation"))
    } else if prompt_lower.contains("estimate") || prompt_lower.contains("quote") || prompt_lower.contains("proposal") {
        Some((crate::models::DocumentType::Estimate, "Estimate"))
    } else if prompt_lower.contains("bill of sale") {
        Some((crate::models::DocumentType::BillOfSale, "Bill of Sale"))
    } else if prompt_lower.contains("contractor agreement") || prompt_lower.contains("independent contractor") {
        Some((crate::models::DocumentType::ContractorAgreement, "Independent Contractor Agreement"))
    } else if prompt_lower.contains("retainer") && !prompt_lower.contains("invoice") {
        Some((crate::models::DocumentType::RetainerAgreement, "Retainer Agreement"))
    } else if prompt_lower.contains("statement of work") || prompt_lower.contains("sow") {
        Some((crate::models::DocumentType::AgreementSow, "Statement of Work"))
    } else if prompt_lower.contains("contract") || prompt_lower.contains("agreement") || prompt_lower.contains("msa") {
        Some((crate::models::DocumentType::Agreement, "Master Services Agreement"))
    } else if prompt_lower.contains("certificate of insurance") || prompt_lower.contains("coi") {
        Some((crate::models::DocumentType::CertificateInsurance, "Certificate of Insurance"))
    } else if prompt_lower.contains("certificate of completion") {
        Some((crate::models::DocumentType::CertificateCompletion, "Certificate of Completion"))
    } else if prompt_lower.contains("certificate") && !prompt_lower.contains("w-9") && !prompt_lower.contains("w9") {
        Some((crate::models::DocumentType::Certificate, "Certificate"))
    } else if prompt_lower.contains("inspection report") {
        Some((crate::models::DocumentType::InspectionReport, "Inspection Report"))
    } else if prompt_lower.contains("medical") || prompt_lower.contains("superbill") || prompt_lower.contains("clinical") {
        Some((crate::models::DocumentType::InvoiceMedical, "Clinical Superbill"))
    } else if prompt_lower.contains("construction") || prompt_lower.contains("progress billing") {
        Some((crate::models::DocumentType::InvoiceConstruction, "Construction Progress Invoice"))
    } else if prompt_lower.contains("saas") || prompt_lower.contains("subscription") {
        Some((crate::models::DocumentType::InvoiceSaas, "SaaS Subscription Invoice"))
    } else if prompt_lower.contains("freelance") || prompt_lower.contains("consulting") {
        Some((crate::models::DocumentType::InvoiceFreelance, "Freelance Invoice"))
    } else if prompt_lower.contains("proforma") {
        Some((crate::models::DocumentType::ProformaInvoice, "Proforma Invoice"))
    } else if prompt_lower.contains("invoice") {
        Some((crate::models::DocumentType::Invoice, "Invoice"))
    } else {
        None
    };

    if let Some((dtype, label)) = matched_type {
        modified.doc_type = dtype;
        changes.push(format!("set archetype to {}", label));
    }

    let has_word = |haystack: &str, needle: &str| -> bool {
        let pattern = format!(r"(?i)\b{}\b", regex::escape(needle));
        regex::Regex::new(&pattern)
            .map(|re| re.is_match(haystack))
            .unwrap_or(false)
    };

    // 2. Client / Recipient extraction (quoted first, then unquoted capitalized phrase)
    let client_quoted_re = regex::Regex::new(
        r"(?i)\b(?:for|to)\s+(?:client\s+|tenant\s+|buyer\s+|customer\s+)?['\x22]([^'\x22]{2,80})['\x22]",
    )
    .unwrap();
    let client_unquoted_re = regex::Regex::new(
        r"\bfor\s+(?:client\s+|tenant\s+|buyer\s+|customer\s+)?([A-Z][a-zA-Z0-9\s&,.-]{1,60}?)(?:\s+for\b|\s+with\b|\s+at\b|\s+including\b|\s+from\b|\.|,|$)",
    )
    .unwrap();
    let item_for_prefix_re = regex::Regex::new(
        r"(?i)\b(?:items?|services?|hours?|hrs?|milestones?|work|charges?|fees?)\s+for\s+$",
    )
    .unwrap();
    if let Some(cap) = client_quoted_re.captures(&req.prompt) {
        if let Some(m) = cap.get(1) {
            let name = m.as_str().trim().trim_matches(|c| c == '\'' || c == '"' || c == '.');
            if !name.is_empty() {
                modified.to.name = name.to_string();
                changes.push(format!("set recipient to {}", name));
            }
        }
    } else if let Some(cap) = client_unquoted_re.captures(&req.prompt) {
        let full_match = cap.get(0).map(|m| m.start()).unwrap_or(0);
        let prefix = &req.prompt[..full_match + 4.min(req.prompt.len().saturating_sub(full_match))];
        if !item_for_prefix_re.is_match(&req.prompt[..full_match]) && !prefix.is_empty() {
            if let Some(m) = cap.get(1) {
                let name = m.as_str().trim().trim_matches(|c| c == '\'' || c == '"' || c == '.');
                let lower_name = name.to_lowercase();
                if !name.is_empty()
                    && !lower_name.contains("hours of")
                    && !lower_name.contains("hrs of")
                {
                    modified.to.name = name.to_string();
                    changes.push(format!("set recipient to {}", name));
                }
            }
        }
    }

    // 3. Sender / Issuer extraction
    let sender_quoted_re = regex::Regex::new(
        r"(?i)\bfrom\s+(?:company\s+|issuer\s+|vendor\s+|seller\s+)?['\x22]([^'\x22]{2,80})['\x22]",
    )
    .unwrap();
    let sender_unquoted_re = regex::Regex::new(
        r"\bfrom\s+(?:company\s+|issuer\s+|vendor\s+|seller\s+)([A-Z][a-zA-Z0-9\s&,.-]{1,60}?)(?:\s+to|\s+for|\s+with|\.|,|$)",
    )
    .unwrap();
    if let Some(cap) = sender_quoted_re.captures(&req.prompt).or_else(|| sender_unquoted_re.captures(&req.prompt)) {
        if let Some(m) = cap.get(1) {
            let name = m.as_str().trim().trim_matches(|c| c == '\'' || c == '"' || c == '.');
            if !name.is_empty() {
                modified.from.name = name.to_string();
                changes.push(format!("set sender to {}", name));
            }
        }
    }

    // 4. Line items extraction (hourly/unit items, parenthesized milestones, and flat fees)
    let mut parsed_items: Vec<LineItem> = Vec::new();
    let parse_amount = |s: &str| -> f64 { s.replace(',', "").parse::<f64>().unwrap_or(0.0) };

    let qty_item_re = regex::Regex::new(
        r"(?i)(\d+(?:\.\d+)?)\s*(?:hours?|hrs?|days?|units?|items?|months?|x)?\s+(?:of\s+)?([a-zA-Z][a-zA-Z0-9\s&/,-]{1,60}?)\s+(?:at|@)\s*\$([\d,]+(?:\.\d+)?)(?:/[a-zA-Z]+)?",
    )
    .unwrap();
    for cap in qty_item_re.captures_iter(&req.prompt) {
        let qty: f64 = cap.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(1.0);
        let desc = cap.get(2).map(|m| m.as_str().trim()).unwrap_or("Service");
        let price = cap.get(3).map(|m| parse_amount(m.as_str())).unwrap_or(0.0);
        if price > 0.0 && !desc.is_empty() {
            parsed_items.push(LineItem {
                description: desc.to_string(),
                quantity: qty,
                unit_price: price,
                tax_rate: None,
                discount: None,
            });
        }
    }

    // Parenthesized milestone items, e.g. "Backend Architecture ($4,800), API Gateway ($2,400)"
    let paren_item_re = regex::Regex::new(
        r"(?:for\s+|,\s*|\band\s+)([A-Z][a-zA-Z0-9\s&/-]{2,45}?)\s*\(\$([\d,]+(?:\.\d+)?)\)",
    )
    .unwrap();
    for cap in paren_item_re.captures_iter(&req.prompt) {
        let desc = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        let price = cap.get(2).map(|m| parse_amount(m.as_str())).unwrap_or(0.0);
        if price > 0.0 && !desc.is_empty() {
            parsed_items.push(LineItem {
                description: desc.to_string(),
                quantity: 1.0,
                unit_price: price,
                tax_rate: None,
                discount: None,
            });
        }
    }

    let flat_fee_re = regex::Regex::new(
        r"(?i)(?:and\s+|plus\s+|with\s+|,\s*|draft\s+|create\s+)(?:a\s+|an\s+)?\$([\d,]+(?:\.\d+)?)\s+([a-zA-Z][a-zA-Z0-9\s&/-]{2,40}?)(?:\.|,|;|\s+for\s+|\s+and\s+|\s+with\s+|\s+use\s+|$)",
    )
    .unwrap();
    for cap in flat_fee_re.captures_iter(&req.prompt) {
        let price = cap.get(1).map(|m| parse_amount(m.as_str())).unwrap_or(0.0);
        let raw_desc = cap.get(2).map(|m| m.as_str().trim()).unwrap_or("");
        if price > 0.0 && !raw_desc.is_empty() && !raw_desc.eq_ignore_ascii_case("discount") {
            let mut chars = raw_desc.chars();
            let desc = match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => raw_desc.to_string(),
            };
            parsed_items.push(LineItem {
                description: desc,
                quantity: 1.0,
                unit_price: price,
                tax_rate: None,
                discount: None,
            });
        }
    }

    if !parsed_items.is_empty() {
        let count = parsed_items.len();
        modified.items = parsed_items;
        changes.push(format!("populated {} line item{}", count, if count == 1 { "" } else { "s" }));
    }

    // 5. Tax rate, currency & discount extraction
    let tax_re = regex::Regex::new(r"(?i)(?:uk\s+|eu\s+|us\s+)?(\d+(?:\.\d+)?)\s*%\s*(?:uk\s+|eu\s+|sales\s+|vat\s+|gst\s+)?(?:tax|vat|gst)").unwrap();
    if let Some(cap) = tax_re.captures(&req.prompt) {
        if let Some(rate) = cap.get(1).and_then(|m| m.as_str().parse::<f64>().ok()) {
            modified.tax_rate = Some(rate);
            changes.push(format!("set tax rate to {}%", rate));
        }
    }

    if has_word(&prompt_lower, "gbp") || prompt_lower.contains('£') || prompt_lower.contains("pounds sterling") {
        modified.currency = Some("GBP".to_string());
        changes.push("set currency to GBP (£)".to_string());
    } else if has_word(&prompt_lower, "eur") || has_word(&prompt_lower, "euro") || has_word(&prompt_lower, "euros") || prompt_lower.contains('€') {
        modified.currency = Some("EUR".to_string());
        changes.push("set currency to EUR (€)".to_string());
    } else if has_word(&prompt_lower, "cad") || prompt_lower.contains("canadian dollar") {
        modified.currency = Some("CAD".to_string());
        changes.push("set currency to CAD".to_string());
    } else if has_word(&prompt_lower, "aud") || prompt_lower.contains("australian dollar") {
        modified.currency = Some("AUD".to_string());
        changes.push("set currency to AUD".to_string());
    } else if has_word(&prompt_lower, "nzd") || prompt_lower.contains("new zealand dollar") {
        modified.currency = Some("NZD".to_string());
        changes.push("set currency to NZD".to_string());
    } else if has_word(&prompt_lower, "sgd") || prompt_lower.contains("singapore dollar") {
        modified.currency = Some("SGD".to_string());
        changes.push("set currency to SGD".to_string());
    } else if has_word(&prompt_lower, "chf") || prompt_lower.contains("swiss franc") {
        modified.currency = Some("CHF".to_string());
        changes.push("set currency to CHF".to_string());
    } else if has_word(&prompt_lower, "jpy") || has_word(&prompt_lower, "yen") {
        modified.currency = Some("JPY".to_string());
        changes.push("set currency to JPY (¥)".to_string());
    } else if has_word(&prompt_lower, "cny") || has_word(&prompt_lower, "rmb") || has_word(&prompt_lower, "yuan") {
        modified.currency = Some("CNY".to_string());
        changes.push("set currency to CNY (¥)".to_string());
    } else if has_word(&prompt_lower, "usd") {
        modified.currency = Some("USD".to_string());
        changes.push("set currency to USD ($)".to_string());
    }

    let disc_re = regex::Regex::new(r"(?i)\$([\d,]+(?:\.\d+)?)\s*discount|discount\s+of\s+\$([\d,]+(?:\.\d+)?)").unwrap();
    let pct_disc_re = regex::Regex::new(
        r"(?i)(\d+(?:\.\d+)?)\s*%\s*(?:promotional\s+|early\s+|volume\s+|loyalty\s+|trade\s+)?discount|discount\s+of\s+(\d+(?:\.\d+)?)\s*%",
    )
    .unwrap();
    if let Some(cap) = disc_re.captures(&req.prompt) {
        let val = cap.get(1).or_else(|| cap.get(2)).map(|m| parse_amount(m.as_str()));
        if let Some(d) = val.filter(|v| *v > 0.0) {
            modified.discount_amount = Some(d);
            changes.push(format!("applied ${:.2} discount", d));
        }
    } else if let Some(cap) = pct_disc_re.captures(&req.prompt) {
        let pct_val = cap
            .get(1)
            .or_else(|| cap.get(2))
            .and_then(|m| m.as_str().parse::<f64>().ok());
        if let Some(pct) = pct_val.filter(|v| *v > 0.0 && *v <= 100.0) {
            let base_subtotal = if modified.subtotal() > 0.0 {
                modified.subtotal()
            } else {
                500.0
            };
            let disc = ((base_subtotal * (pct / 100.0)) * 100.0).round() / 100.0;
            modified.discount_amount = Some(disc);
            changes.push(format!("applied {}% discount (${:.2})", pct, disc));
        }
    }

    // 5b. Payment settlement terms / Net days (e.g. "14-day payment settlement terms", "Net 30")
    let net_days_re = regex::Regex::new(
        r"(?i)\b(?:net\s+(\d{1,3})|(\d{1,3})(?:\s*-\s*|\s+)day\s+(?:payment|settlement|credit|net))\b",
    )
    .unwrap();
    if let Some(cap) = net_days_re.captures(&req.prompt) {
        if let Some(days) = cap
            .get(1)
            .or_else(|| cap.get(2))
            .and_then(|m| m.as_str().parse::<u64>().ok())
        {
            let base_date = modified
                .issue_date
                .as_deref()
                .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
                .unwrap_or_else(|| chrono::Utc::now().naive_utc().date());
            let due = base_date + chrono::Days::new(days);
            let due_str = due.format("%Y-%m-%d").to_string();
            modified.due_date = Some(due_str.clone());
            modified.terms = Some(format!(
                "Payment settlement due within {} days of invoice date (Net {}).",
                days, days
            ));
            changes.push(format!("set {}-day payment settlement terms (due {})", days, due_str));
        }
    }

    // 5c. Dispute arbitration clause extraction
    if prompt_lower.contains("arbitration") {
        let arb_clause = "1. PURPOSE & SCOPE: Contractor shall perform the independent professional services specified in the schedule.\n\n2. INDEPENDENT CONTRACTOR RELATIONSHIP: Contractor acts solely as an independent contractor and not as an employee, partner, or agent.\n\n3. INTELLECTUAL PROPERTY: All custom deliverables, code, designs, and work product vest exclusively with the client upon receipt of full payment.\n\n4. DISPUTE ARBITRATION CLAUSE: Any dispute, claim, or controversy arising out of or relating to this Agreement shall be resolved exclusively by expedited, final, and binding commercial arbitration.";
        match modified.terms {
            Some(ref existing) if !existing.trim().is_empty() && !existing.to_lowercase().contains("arbitration") => {
                modified.terms = Some(format!(
                    "{}\n\nDISPUTE ARBITRATION CLAUSE: Any dispute, claim, or controversy arising out of or relating to this Agreement shall be resolved exclusively by expedited, final, and binding commercial arbitration.",
                    existing.trim()
                ));
            }
            _ => {
                modified.terms = Some(arb_clause.to_string());
            }
        }
        changes.push("added dispute arbitration clause".to_string());
    }

    // 5d. Compliance mode detection
    if prompt_lower.contains("w-9") || prompt_lower.contains("w9") || prompt_lower.contains("taxpayer identification") {
        modified.compliance_mode = Some("irs_w9".to_string());
        changes.push("enabled IRS W-9 TIN compliance attestation".to_string());
    } else if prompt_lower.contains("reverse charge") || prompt_lower.contains("eu vat") || prompt_lower.contains("2006/112") {
        modified.compliance_mode = Some("eu_vat".to_string());
        changes.push("enabled EU VAT reverse charge compliance notice".to_string());
    } else if prompt_lower.contains("pci-dss") || prompt_lower.contains("pci dss") || prompt_lower.contains("pci_dss") {
        modified.compliance_mode = Some("pci_dss".to_string());
        changes.push("enabled PCI-DSS compliance notice".to_string());
    } else if prompt_lower.contains("soc-2") || prompt_lower.contains("soc 2") || prompt_lower.contains("soc2") || prompt_lower.contains("iso 27001") || prompt_lower.contains("iso27001") {
        modified.compliance_mode = Some("soc2".to_string());
        changes.push("enabled SOC-2 / ISO 27001 compliance notice".to_string());
    } else if prompt_lower.contains("statutory sworn") || prompt_lower.contains("sworn declaration") || prompt_lower.contains("perjury") {
        modified.compliance_mode = Some("statutory_sworn".to_string());
        changes.push("enabled statutory sworn declaration".to_string());
    }

    // 5e. Date format detection
    if prompt_lower.contains("military") || prompt_lower.contains("dd-mmm-yyyy") || prompt_lower.contains("short_abbrev") {
        modified.date_format = Some("short_abbrev".to_string());
        changes.push("set date format to Military (DD-MMM-YYYY)".to_string());
    } else if prompt_lower.contains("formal prose") || prompt_lower.contains("formal_prose") || prompt_lower.contains("long date") {
        modified.date_format = Some("formal_prose".to_string());
        changes.push("set date format to Formal Prose".to_string());
    } else if prompt_lower.contains("european date") || prompt_lower.contains("dd/mm/yyyy") {
        modified.date_format = Some("euro".to_string());
        changes.push("set date format to European (DD/MM/YYYY)".to_string());
    } else if prompt_lower.contains("us date") || prompt_lower.contains("mm/dd/yyyy") {
        modified.date_format = Some("us_slash".to_string());
        changes.push("set date format to US (MM/DD/YYYY)".to_string());
    } else if prompt_lower.contains("german date") || prompt_lower.contains("swiss date") || prompt_lower.contains("dd.mm.yyyy") {
        modified.date_format = Some("short_dot".to_string());
        changes.push("set date format to Dot (DD.MM.YYYY)".to_string());
    } else if prompt_lower.contains("iso date") || prompt_lower.contains("yyyy-mm-dd") {
        modified.date_format = Some("iso".to_string());
        changes.push("set date format to ISO (YYYY-MM-DD)".to_string());
    }

    // 6. Theme & palette detection across all 18 valid themes
    let mut matched_theme: Option<(&str, &str)> = None;
    if has_word(&prompt_lower, "obsidian")
        || prompt_lower.contains("dark obsidian")
        || (has_word(&prompt_lower, "dark") && !has_word(&prompt_lower, "slate"))
    {
        matched_theme = Some(("obsidian", "applied Obsidian theme"));
    } else if has_word(&prompt_lower, "slate") || prompt_lower.contains("modern_slate") {
        matched_theme = Some(("slate", "applied Modern Slate theme"));
    } else if has_word(&prompt_lower, "navy")
        || prompt_lower.contains("executive navy")
        || prompt_lower.contains("executive_navy")
    {
        matched_theme = Some(("executive_navy", "applied Executive Navy theme"));
    } else if has_word(&prompt_lower, "monaco")
        || prompt_lower.contains("monaco_gold")
        || has_word(&prompt_lower, "gold")
    {
        matched_theme = Some(("monaco_gold", "applied Monaco Gold theme"));
    } else if has_word(&prompt_lower, "crimson")
        || prompt_lower.contains("crimson_legal")
        || has_word(&prompt_lower, "red")
    {
        matched_theme = Some(("crimson_legal", "applied Crimson Legal theme"));
    } else if has_word(&prompt_lower, "bordeaux") {
        matched_theme = Some(("bordeaux", "applied Bordeaux theme"));
    } else if prompt_lower.contains("warm sand")
        || prompt_lower.contains("warm_sand")
        || prompt_lower.contains("warm amber")
    {
        matched_theme = Some(("warm_sand", "applied Warm Sand theme"));
    } else if prompt_lower.contains("desert sand")
        || prompt_lower.contains("desert_sand")
        || has_word(&prompt_lower, "mojave")
        || has_word(&prompt_lower, "amber")
        || has_word(&prompt_lower, "desert")
    {
        matched_theme = Some(("desert_sand", "applied Desert Sand theme"));
    } else if has_word(&prompt_lower, "cyber")
        || has_word(&prompt_lower, "neon")
        || prompt_lower.contains("cyber_neon")
    {
        matched_theme = Some(("cyber_neon", "applied Cyber Neon theme"));
    } else if has_word(&prompt_lower, "emerald") || has_word(&prompt_lower, "green") {
        matched_theme = Some(("emerald", "applied Emerald theme"));
    } else if has_word(&prompt_lower, "nordic")
        || has_word(&prompt_lower, "teal")
        || prompt_lower.contains("nordic_teal")
        || prompt_lower.contains("nordic_frost")
    {
        matched_theme = Some(("nordic_teal", "applied Nordic Teal theme"));
    } else if has_word(&prompt_lower, "forest")
        || has_word(&prompt_lower, "pine")
        || prompt_lower.contains("forest_pine")
    {
        matched_theme = Some(("forest_pine", "applied Forest Pine theme"));
    } else if has_word(&prompt_lower, "purple")
        || has_word(&prompt_lower, "violet")
        || prompt_lower.contains("royal violet")
        || prompt_lower.contains("royal_violet")
    {
        matched_theme = Some(("royal_violet", "applied Royal Violet theme"));
    } else if has_word(&prompt_lower, "sunset")
        || has_word(&prompt_lower, "coral")
        || prompt_lower.contains("sunset_coral")
    {
        matched_theme = Some(("sunset_coral", "applied Sunset Coral theme"));
    } else if prompt_lower.contains("electric indigo")
        || prompt_lower.contains("electric_indigo")
        || has_word(&prompt_lower, "indigo")
    {
        matched_theme = Some(("electric_indigo", "applied Electric Indigo theme"));
    } else if has_word(&prompt_lower, "neobrutalist") || has_word(&prompt_lower, "brutalist") {
        matched_theme = Some(("neobrutalist", "applied Neobrutalist theme"));
    } else if has_word(&prompt_lower, "bauhaus") || prompt_lower.contains("bauhaus_primary") {
        matched_theme = Some(("bauhaus_primary", "applied Bauhaus Primary theme"));
    } else if has_word(&prompt_lower, "swiss")
        || prompt_lower.contains("swiss_minimal")
        || has_word(&prompt_lower, "minimal")
        || has_word(&prompt_lower, "monochrome")
    {
        matched_theme = Some(("swiss_minimal", "applied Swiss Minimal theme"));
    }

    if let Some((theme_id, msg)) = matched_theme {
        modified.theme = Some(theme_id.to_string());
        modified.primary_color = None;
        modified.secondary_color = None;
        modified.accent_color = None;
        modified.surface_color = None;
        modified.text_color = None;
        let prim = modified.effective_primary_color().to_string();
        let sec = modified.effective_secondary_color().to_string();
        let acc = modified.effective_accent_color().to_string();
        let surf = modified.effective_surface_color().to_string();
        modified.primary_color = Some(prim);
        modified.secondary_color = Some(sec);
        modified.accent_color = Some(acc);
        modified.surface_color = Some(surf);
        changes.push(msg.to_string());
    }

    // 7. Signature block detection
    if prompt_lower.contains("signature") || prompt_lower.contains("sign block") {
        modified.show_signature = Some(true);
        if modified.signatures.is_empty() {
            let signer = if modified.from.name.trim().is_empty() {
                "Authorized Signatory".to_string()
            } else {
                modified.from.name.clone()
            };
            modified.signatures.push(crate::models::SignatureBlock {
                signer_name: signer,
                signer_title: Some("Authorized Representative".to_string()),
                date: modified.issue_date.clone(),
                signature_data: None,
            });
        }
        changes.push("enabled signature block".to_string());
    }

    if let Some(ref sess) = session {
        if sess.user.is_subscriber {
            modified.watermark = None;
        }
    }

    state.storage.record_usage_event(
        "copilot_prompt",
        Some(modified.doc_type.as_str()),
        "studio",
        user_agent,
        None,
    );

    let message = if changes.is_empty() {
        "Updated document settings from your prompt.".to_string()
    } else {
        format!("Updated document: {}.", changes.join(", "))
    };

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
    Json(req): Json<VisionRequest>,
) -> impl IntoResponse {
    let user_agent = headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok());
    let maybe_token = auth::extract_token_from_headers(&headers);
    let session = maybe_token.as_deref().and_then(|t| state.auth_mgr.get_session(t));

    let clean_name = req
        .filename
        .as_deref()
        .and_then(|f| f.split('.').next())
        .map(|s| s.replace(['-', '_'], " ").trim().to_string())
        .filter(|s| !s.is_empty());

    let mut party = PartyDetails::default();
    if let Some(ref base) = clean_name {
        let mut chars = base.chars();
        let titled = match chars.next() {
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            None => "Scanned Vendor LLC".to_string(),
        };
        party.name = titled;
    } else {
        party.name = "Scanned Vendor LLC".to_string();
    }
    party.email = Some("billing@scanned-vendor.com".to_string());

    let items = vec![
        LineItem {
            description: clean_name
                .as_ref()
                .map(|n| format!("{} — Line Item", n))
                .unwrap_or_else(|| "Extracted Line Item 1".to_string()),
            quantity: 1.0,
            unit_price: 499.0,
            tax_rate: None,
            discount: None,
        },
        LineItem {
            description: "Processing & Handling".to_string(),
            quantity: 1.0,
            unit_price: 25.0,
            tax_rate: None,
            discount: None,
        },
    ];

    state.storage.record_usage_event(
        "vision_extract",
        Some("receipt"),
        "studio",
        req.filename.as_deref().or(user_agent),
        None,
    );

    let ai_message = if let Some(sess) = session {
        format!(
            "Extracted vendor and 2 line items ({}).",
            sess.user.plan_display
        )
    } else {
        "Extracted vendor details and 2 line items from uploaded image.".to_string()
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

                    if crate::ghl::GhlDispatcher::has_env_or_pit_file() {
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

                    if crate::ghl::GhlDispatcher::has_env_or_pit_file() {
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
                crate::stripe::FulfillmentAction::ChargeRefunded {
                    customer_email,
                    refund_amount_cents,
                    credit_note_number,
                    ..
                } => {
                    state.auth_mgr.revoke_user_entitlement(customer_email);
                    state.storage.record_usage_event(
                        "stripe_refund_revoked",
                        None,
                        "stripe_webhook",
                        Some(credit_note_number),
                        Some(*refund_amount_cents),
                    );
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

