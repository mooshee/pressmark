//! Pressmark Axum HTTP Server & API Router
//!
//! Author: Daniel Hallman

use crate::auth::{
    self, ApiKeyLoginRequest, CallbackQuery, ChatGptAuthManager, EmailLoginRequest, LoginQuery,
};
use crate::engine::{compile_pdf_bytes, compile_png_bytes, generate_pdf, generate_preview_png, generate_all_preview_pngs};
use crate::mcp::{JsonRpcRequest, JsonRpcResponse, McpHandler};
use crate::models::{DocumentRequest, DocumentType, GenerateResponse, PreviewResponse, CopilotRequest, CopilotResponse, VisionRequest, VisionResponse, LineItem, PartyDetails};
use crate::openapi::{get_ai_plugin_manifest, get_openapi_spec};
use crate::pricing::get_pricing_catalog;
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{Form, Json, Router};
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
    pub downloads: crate::downloads::DocumentDownloadCache,
}

impl AppState {
    pub fn new(base_url: String) -> Self {
        let max_req = std::env::var("PRESSMARK_RATE_LIMIT_MAX")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(300);
        let window_secs = std::env::var("PRESSMARK_RATE_LIMIT_WINDOW")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);

        let storage = Arc::new(crate::storage::Storage::default_db());
        let auth_mgr = ChatGptAuthManager::new().with_storage(Arc::clone(&storage));
        let stripe_idempotency = crate::stripe::StripeIdempotencyStore::new().with_storage(Arc::clone(&storage));

        Self {
            base_url,
            downloads: crate::downloads::DocumentDownloadCache::default(),
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
        .route("/", get(handle_index).post(handle_mcp))
        .route("/pricing", get(handle_index))
        .route("/demo", get(handle_index))
        .route("/docs", get(handle_index))
        .route("/templates", get(handle_index))
        .route("/templates/", get(handle_index))
        .route("/templates/:slug", get(handle_template_landing))
        .route("/favicon.ico", get(handle_favicon))
        .route("/favicon.svg", get(handle_favicon))
        .route("/favicon.png", get(handle_favicon))
        .route("/privacy", get(handle_privacy))
        .route("/terms", get(handle_terms))
        .route("/robots.txt", get(handle_robots))
        .route("/sitemap.xml", get(handle_sitemap))
        .route("/llms.txt", get(handle_llms_txt))
        .route("/llms-full.txt", get(handle_llms_full_txt))
        .route("/health", get(handle_health))
        .route("/healthz", get(handle_health))
        .route("/ready", get(handle_ready).post(handle_ready))
        .route("/openapi.json", get(handle_openapi))
        // Informational, Company & Docs Redirects
        .route("/about", get(|| async { Redirect::permanent("/") }))
        .route("/about-us", get(|| async { Redirect::permanent("/") }))
        .route("/company", get(|| async { Redirect::permanent("/") }))
        .route("/contact", get(|| async { Redirect::permanent("/") }))
        .route("/support", get(|| async { Redirect::permanent("/") }))
        .route("/help", get(|| async { Redirect::permanent("/") }))
        .route("/faq", get(|| async { Redirect::permanent("/") }))
        .route("/features", get(|| async { Redirect::permanent("/#pricing") }))
        .route("/product", get(|| async { Redirect::permanent("/#pricing") }))
        .route("/documentation", get(|| async { Redirect::permanent("/docs") }))
        .route("/security", get(|| async { Redirect::permanent("/.well-known/security.txt") }))
        .route("/security.txt", get(handle_security_txt))
        .route("/.well-known/security.txt", get(handle_security_txt))
        .route("/AGENTS.md", get(|| async { Redirect::permanent("/llms.txt") }))
        .route("/AGENT.md", get(|| async { Redirect::permanent("/llms.txt") }))
        .route("/QWEN.md", get(|| async { Redirect::permanent("/llms.txt") }))
        .route("/CLAUDE.md", get(|| async { Redirect::permanent("/llms.txt") }))
        // API v1
        .route("/api/v1/generate", post(handle_generate))
        .route("/api/v1/downloads/:token", get(handle_document_download))
        .route("/api/v1/preview", post(handle_preview))
        .route("/api/v1/render-raw", post(handle_render_raw))
        .route("/api/v1/render-vector", post(handle_render_raw))
        .route("/api/v1/templates", get(handle_templates_catalog))
        .route("/api/v1/states", get(handle_states_catalog))
        .route("/api/v1/pricing", get(handle_pricing))
        .route("/api/v1/checkout", get(handle_checkout_get).post(handle_checkout_post))
        .route("/api/v1/checkout/continue", post(handle_checkout_continue))
        .route("/api/v1/setup-check", get(handle_setup_check).post(handle_setup_check))
        .route("/api/v1/analytics", get(handle_analytics).post(handle_analytics))
        .route("/api/v1/openapi.json", get(handle_openapi))
        .route("/api/v1/copilot", post(handle_copilot))
        .route("/api/v1/vision-extract", post(handle_vision_extract))
        .route("/api/v1/lifecycle/convert", post(handle_lifecycle_convert))
        .route("/api/v1/lifecycle/pay", post(handle_lifecycle_pay))
        .route("/api/v1/webhooks/stripe", post(handle_stripe_webhook))
        .route("/.well-known/ai-plugin.json", get(handle_ai_plugin))
        .route("/.well-known/openai-apps-challenge", get(handle_openai_apps_challenge))
        .route("/.well-known/owners.json", get(handle_owners_json))
        .route("/mcp/.well-known/owners.json", get(handle_mcp_owners_json))
        .route("/api/v1/mcp/.well-known/owners.json", get(handle_mcp_owners_json))
        .route("/.well-known/glama.json", get(handle_glama_json))
        .route("/.well-known/oauth-protected-resource", get(handle_oauth_protected_resource))
        .route("/.well-known/oauth-protected-resource/mcp", get(handle_oauth_protected_resource))
        .route("/.well-known/oauth-protected-resource/api/v1/mcp", get(handle_oauth_protected_resource))
        .route("/mcp/.well-known/oauth-protected-resource", get(handle_oauth_protected_resource))
        .route("/.well-known/oauth-authorization-server", get(handle_oauth_authorization_server))
        .route("/.well-known/oauth-authorization-server/mcp", get(handle_oauth_authorization_server))
        .route("/.well-known/oauth-authorization-server/api/v1/mcp", get(handle_oauth_authorization_server))
        .route("/mcp/.well-known/oauth-authorization-server", get(handle_oauth_authorization_server))
        .route("/.well-known/openid-configuration", get(handle_oauth_authorization_server))
        .route("/.well-known/openid-configuration/mcp", get(handle_oauth_authorization_server))
        .route("/.well-known/openid-configuration/api/v1/mcp", get(handle_oauth_authorization_server))
        .route("/mcp/.well-known/openid-configuration", get(handle_oauth_authorization_server))
        .route("/.well-known/mcp/server-card.json", get(handle_mcp_server_card))
        .route("/.well-known/mcp.json", get(handle_mcp_server_card))
        .route("/.mcp.json", get(handle_mcp_server_card))
        .route("/.cursor/mcp.json", get(handle_mcp_server_card))
        .route("/.well-known/agent-card.json", get(handle_agent_card))
        .route("/.well-known/agent.json", get(handle_agent_card))
        .route("/.well-known/agents.json", get(handle_agent_card))
        .route("/.well-known/ard.json", get(handle_agent_card))
        .route("/.well-known/ai-catalog.json", get(handle_agent_card))
        .route("/mcp/.well-known/agent.json", get(handle_agent_card))
        .route("/mcp/.well-known/agent-card.json", get(handle_agent_card))
        .route("/api/v1/mcp/.well-known/agent.json", get(handle_agent_card))
        .route("/api/v1/mcp/.well-known/agent-card.json", get(handle_agent_card))
        .route("/mcp/server-card", get(handle_mcp_server_card))
        .route("/mcp/server-card.json", get(handle_mcp_server_card))
        .route("/api/v1/mcp/server-card", get(handle_mcp_server_card))
        .route("/api/v1/mcp/server-card.json", get(handle_mcp_server_card))
        .route("/.well-known/capabilities.txt", get(handle_capabilities_txt))
        .route("/.well-known/did.json", get(handle_did_json))
        .route("/.well-known/brick-blue.json", get(handle_brick_blue_json))
        .route("/discovery/resources", get(handle_discovery_resources))
        .route("/.well-known/x402", get(handle_x402_discovery))
        .route("/.well-known/x402.json", get(handle_x402_discovery))
        .route("/a2a", get(handle_agent_card).post(handle_a2a_post))
        .route("/a2a/", get(handle_agent_card).post(handle_a2a_post))
        .route("/api/v1/mcp/a2a", get(handle_agent_card).post(handle_a2a_post))
        .route("/api/v1/mcp/a2a/", get(handle_agent_card).post(handle_a2a_post))
        .route("/v1/message:send", post(handle_a2a_post))
        .route("/api/v1/mcp/v1/message:send", post(handle_a2a_post))
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
        .route("/mcp", get(handle_mcp_get).post(handle_mcp).delete(handle_mcp_delete))
        .route("/mcp/", get(handle_mcp_get).post(handle_mcp).delete(handle_mcp_delete))
        .route("/mcp/mcp", get(handle_mcp_get).post(handle_mcp).delete(handle_mcp_delete))
        .route("/mcp/v1", get(handle_mcp_get).post(handle_mcp).delete(handle_mcp_delete))
        .route("/api/mcp", get(handle_mcp_get).post(handle_mcp).delete(handle_mcp_delete))
        .route("/mcp.", get(|| async { Redirect::permanent("/mcp") }).post(|| async { Redirect::permanent("/mcp") }))
        .route("/mcp\"", get(|| async { Redirect::permanent("/mcp") }).post(|| async { Redirect::permanent("/mcp") }))
        .route("/mcp%22", get(|| async { Redirect::permanent("/mcp") }).post(|| async { Redirect::permanent("/mcp") }))
        .route("/api/v1/mcp.", get(|| async { Redirect::permanent("/api/v1/mcp") }).post(|| async { Redirect::permanent("/api/v1/mcp") }))
        .route("/api/v1/mcp\"", get(|| async { Redirect::permanent("/api/v1/mcp") }).post(|| async { Redirect::permanent("/api/v1/mcp") }))
        .route("/api/v1/mcp%22", get(|| async { Redirect::permanent("/api/v1/mcp") }).post(|| async { Redirect::permanent("/api/v1/mcp") }))
        .route("/mcp/sse", get(handle_mcp_get).post(handle_mcp))
        .route("/sse", get(handle_mcp_get).post(handle_mcp))
        .route("/api/v1/mcp", get(handle_mcp_get).post(handle_mcp).delete(handle_mcp_delete))
        .route("/api/v1/mcp/", get(handle_mcp_get).post(handle_mcp).delete(handle_mcp_delete))
        .route("/api/v1/mcp/mcp", get(handle_mcp_get).post(handle_mcp).delete(handle_mcp_delete))
        .route("/api/v1/mcp/sse", get(handle_mcp_get).post(handle_mcp))
        .route("/api/v1/mcp/tools.json", get(handle_mcp_tools))
        // Static assets
        .nest_service("/static", ServeDir::new("static"))
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
        "version": crate::mcp::MCP_SERVER_VERSION
    })))
}

async fn handle_document_download(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> Response {
    let mut response = match state.downloads.get(&token) {
        Some(file) => {
            let length = file.bytes.len();
            let mut response = (StatusCode::OK, file.bytes).into_response();
            let headers = response.headers_mut();
            headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static(file.mime_type));
            headers.insert(header::CONTENT_LENGTH, header::HeaderValue::from(length));
            // The cache only emits ASCII letters, digits, hyphens, and underscores in filenames.
            headers.insert(header::CONTENT_DISPOSITION,
                format!("{}; filename=\"{}\"", if file.mime_type == "image/png" { "inline" } else { "attachment" }, file.filename).parse().expect("safe document filename"));
            response
        }
        None => (StatusCode::NOT_FOUND,
            "This download link is unavailable or has expired. Generate the PDF again, or request a new preview.").into_response(),
    };
    let headers = response.headers_mut();
    headers.insert(header::CACHE_CONTROL, header::HeaderValue::from_static("private, no-store, max-age=0"));
    headers.insert("cdn-cache-control", header::HeaderValue::from_static("no-store"));
    headers.insert("cloudflare-cdn-cache-control", header::HeaderValue::from_static("no-store"));
    headers.insert("x-robots-tag", header::HeaderValue::from_static("noindex, nofollow, noarchive"));
    headers.insert("referrer-policy", header::HeaderValue::from_static("no-referrer"));
    headers.insert("x-content-type-options", header::HeaderValue::from_static("nosniff"));
    headers.insert(header::CONTENT_SECURITY_POLICY, header::HeaderValue::from_static("default-src 'none'; sandbox"));
    response
}

async fn handle_ready(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let db_ok = state.storage.check_ready().is_ok();
    let typst_path = crate::engine::find_typst_executable();
    let typst_ok = typst_path.is_some();

    let all_ok = db_ok && typst_ok;
    let status_code = if all_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        status_code,
        Json(serde_json::json!({
            "service": "pressmark",
            "status": if all_ok { "ready" } else { "degraded" },
            "version": crate::mcp::MCP_SERVER_VERSION,
            "checks": {
                "database": if db_ok { "ok" } else { "error" },
                "typst": if typst_ok { "ok" } else { "missing_binary" }
            }
        })),
    )
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
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Response {
    let clean_slug = slug.trim_end_matches([':', ')', ']', '}', '"', '\'', '.', ',', ';', '>']);
    if clean_slug != slug {
        if DocumentType::from_slug(clean_slug).is_some() || DocumentType::parse_state_prefixed_slug(clean_slug).is_some() {
            return Redirect::permanent(&format!("/templates/{}", clean_slug)).into_response();
        }
    }
    let (doc_type, maybe_state_profile) = if let Some(dt) = DocumentType::from_slug(&slug) {
        (dt, None)
    } else if let Some((sp, dt)) = DocumentType::parse_state_prefixed_slug(&slug) {
        (dt, Some(sp))
    } else {
        return (
            StatusCode::NOT_FOUND,
            Html("<h1>404 · Template Not Found</h1><p><a href=\"/templates\">Browse all 57 document templates</a></p>".to_string()),
        )
            .into_response();
    };

    let archetype_key = doc_type.as_str();
    let (canonical_slug, seo_title, seo_desc) = if let Some(sp) = maybe_state_profile {
        let (st_title, st_desc) = doc_type.state_seo_metadata(sp);
        (
            format!("{}-{}", sp.slug, doc_type.url_slug()),
            st_title,
            st_desc,
        )
    } else {
        let (base_title, base_desc) = doc_type.seo_metadata();
        (
            doc_type.url_slug().to_string(),
            base_title.to_string(),
            base_desc.to_string(),
        )
    };
    let canonical_url = format!("https://pressmark.dev/templates/{}", canonical_slug);
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let landing_channel = if crate::storage::Storage::is_crawler_bot_ua(user_agent) {
        "web_seo_bot"
    } else if crate::storage::Storage::is_internal_probe_ua(user_agent) {
        "web_seo_probe"
    } else {
        "web_seo"
    };

    state.storage.record_usage_event(
        "template_landing_view",
        Some(archetype_key),
        landing_channel,
        Some(&canonical_slug),
        None,
    );

    match tokio::fs::read_to_string("static/index.html").await {
        Ok(raw_html) => {
            let mut html = raw_html
                .replace(
                    "<title>Pressmark · Invoices, agreements and documents</title>",
                    &format!("<title>{}</title>", seo_title),
                )
                .replace(
                    "<meta name=\"description\" content=\"Create invoices, agreements and business documents. Add your details, choose a design, and review your document with a live preview before exporting.\">",
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
                    "<meta property=\"og:title\" content=\"Pressmark · Invoices, agreements and documents\">",
                    &format!("<meta property=\"og:title\" content=\"{}\">", seo_title),
                )
                .replace(
                    "<meta property=\"og:description\" content=\"Create your document, add your details, and review every page before exporting.\">",
                    &format!("<meta property=\"og:description\" content=\"{}\">", seo_desc),
                );

            let boot_script = if let Some(sp) = maybe_state_profile {
                format!(
                    "  <script>window.__PRESSMARK_INITIAL_ARCHETYPE__ = \"{}\"; window.__PRESSMARK_INITIAL_STATE__ = \"{}\";</script>\n</head>",
                    archetype_key, sp.code
                )
            } else {
                format!(
                    "  <script>window.__PRESSMARK_INITIAL_ARCHETYPE__ = \"{}\";</script>\n</head>",
                    archetype_key
                )
            };
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
            "success": true,
            "count": templates.len(),
            "templates": templates
        })),
    )
}

async fn handle_states_catalog() -> impl IntoResponse {
    let states = crate::state_laws::all_state_lease_profiles();
    let municipalities = crate::state_laws::all_municipal_sub_profiles();
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "states_count": states.len(),
            "municipalities_count": municipalities.len(),
            "states": states,
            "municipalities": municipalities,
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
    let mut xml = String::with_capacity(16384);
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    xml.push_str("  <url><loc>https://pressmark.dev/</loc><changefreq>daily</changefreq><priority>1.0</priority></url>\n");
    xml.push_str("  <url><loc>https://pressmark.dev/pricing</loc><changefreq>weekly</changefreq><priority>0.9</priority></url>\n");
    xml.push_str("  <url><loc>https://pressmark.dev/privacy</loc><changefreq>monthly</changefreq><priority>0.5</priority></url>\n");
    xml.push_str("  <url><loc>https://pressmark.dev/terms</loc><changefreq>monthly</changefreq><priority>0.5</priority></url>\n");

    for item in crate::models::DocumentType::all() {
        xml.push_str(&format!(
            "  <url><loc>https://pressmark.dev/templates/{}</loc><changefreq>weekly</changefreq><priority>0.8</priority></url>\n",
            item.url_slug()
        ));
    }

    for sp in crate::state_laws::all_state_lease_profiles() {
        xml.push_str(&format!(
            "  <url><loc>https://pressmark.dev/templates/{}-residential-lease</loc><changefreq>weekly</changefreq><priority>0.8</priority></url>\n",
            sp.slug
        ));
    }

    for extra_slug in &[
        "florida-eviction-notice",
        "texas-quitclaim-deed",
        "new-york-room-rental-lease",
        "california-eviction-notice",
        "tennessee-property-deed",
    ] {
        xml.push_str(&format!(
            "  <url><loc>https://pressmark.dev/templates/{}</loc><changefreq>weekly</changefreq><priority>0.8</priority></url>\n",
            extra_slug
        ));
    }

    xml.push_str("</urlset>\n");
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        xml,
    )
        .into_response()
}

async fn handle_llms_txt() -> impl IntoResponse {
    let mut lines = vec![
        "# Pressmark Studio · Sub-Millisecond Vector Document Engine".to_string(),
        "".to_string(),
        "> High-performance Rust vector PDF document synthesis engine, interactive browser studio, and Model Context Protocol (MCP) server at https://pressmark.dev/mcp.".to_string(),
        "".to_string(),
        "## Core Endpoints & Agent Integration".to_string(),
        "- [Interactive Studio](https://pressmark.dev/): Design, preview, and export 57 business, billing, legal, and real estate document archetypes in sub-25ms.".to_string(),
        "- [Remote MCP Endpoint](https://pressmark.dev/mcp): Streamable HTTP MCP server (`io.github.mooshee/pressmark`) exposing `generate_document`, `preview_document`, `calculate_totals`, `get_pricing`, `convert_document`, and `record_payment`.".to_string(),
        "- [OpenAPI 3.1 Specification](https://pressmark.dev/api/v1/openapi.json): REST API schema for `/api/v1/generate`, `/api/v1/preview`, `/api/v1/templates`, `/api/v1/states`, and `/api/v1/pricing`.".to_string(),
        "- [Pricing & Single Unlock Passes](https://pressmark.dev/pricing): Free community tier, $2.99 Single Document Pass, and $15/mo Solo Pro unlimited exports.".to_string(),
        "- [Full LLM Corpus](https://pressmark.dev/llms-full.txt): Complete 57-archetype catalog and MCP tool documentation.".to_string(),
        "".to_string(),
        "## Interactive Document Templates (57 Archetypes)".to_string(),
    ];
    for dt in DocumentType::all() {
        let (_, desc) = dt.seo_metadata();
        lines.push(format!(
            "- [{}](https://pressmark.dev/templates/{}) — {}",
            dt.display_title(),
            dt.url_slug(),
            desc
        ));
    }
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        lines.join("\n"),
    )
}

async fn handle_llms_full_txt() -> impl IntoResponse {
    let mut lines = vec![
        "# Pressmark Studio — Full AI & LLM Reference Corpus".to_string(),
        "".to_string(),
        "Website: https://pressmark.dev".to_string(),
        "MCP Server (Streamable HTTP): https://pressmark.dev/mcp".to_string(),
        "MCP Registry ID: io.github.mooshee/pressmark".to_string(),
        "OpenAPI Spec: https://pressmark.dev/api/v1/openapi.json".to_string(),
        "".to_string(),
        "## MCP Tools (6 Tools)".to_string(),
        "1. `generate_document`: Synthesizes a publication-grade vector PDF with embedded 1-scan SVG QR code mobile payments and returns `pdf_base64`, `checkout_url`, and `pricing_url`.".to_string(),
        "2. `preview_document`: Renders a 144-PPI PNG preview as a Base64 Data URI for immediate visual inspection in chat.".to_string(),
        "3. `calculate_totals`: Computes line-item subtotals, tax rates, discounts, and currency-specific grand totals.".to_string(),
        "4. `get_pricing`: Returns subscription tiers ($15/mo Solo Pro, $49/mo Business, $149/mo Enterprise) and microtransaction passes ($2.99 Single Unlock).".to_string(),
        "5. `convert_document`: Transitions documents across their billing lifecycle (`estimate` -> `invoice` -> `receipt` / `credit_note`).".to_string(),
        "6. `record_payment`: Records partial or full payments, recalculates `balance_due`, and synthesizes official paid receipts.".to_string(),
        "".to_string(),
        "## All 57 Document Archetypes".to_string(),
    ];
    for dt in DocumentType::all() {
        let (title, desc) = dt.seo_metadata();
        lines.push(format!(
            "### {} (`{}`)\n- URL: https://pressmark.dev/templates/{}\n- SEO Title: {}\n- Summary: {}\n",
            dt.display_title(),
            dt.as_str(),
            dt.url_slug(),
            title,
            desc
        ));
    }
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        lines.join("\n"),
    )
}

async fn handle_generate(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<FormatQuery>,
    Json(mut doc_req): Json<DocumentRequest>,
) -> Response {
    if let Err(err) = doc_req.normalize_and_validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "error": err
            })),
        )
            .into_response();
    }

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
                    .filter(|e| {
                        !e.ends_with("@pressmark.local")
                            && !e.ends_with("@example.com")
                            && !e.ends_with("@example.org")
                    })
                    .or_else(|| doc_req.from.email.as_deref())
                    .or_else(|| doc_req.to.email.as_deref())
                    .map(str::trim)
                    .filter(|e| {
                        e.contains('@')
                            && !e.ends_with("@pressmark.local")
                            && !e.ends_with("@example.com")
                            && !e.ends_with("@example.org")
                    })
                    .map(|s| s.to_string());
                if let Some(email_cl) = candidate_email {
                    let mut doc_cl = doc_req.clone();
                    let is_paid = is_chatgpt_subscriber
                        || state
                            .storage
                            .get_customer_entitlement(&email_cl)
                            .is_some();
                    doc_cl.apply_free_tier_ghl_watermark(is_paid);
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
    Json(mut doc_req): Json<DocumentRequest>,
) -> Response {
    if let Err(err) = doc_req.normalize_and_validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "error": err
            })),
        )
            .into_response();
    }
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
    pub request_id: Option<String>,
    #[serde(alias = "plan")]
    pub plan_id: Option<String>,
    pub price_id: Option<String>,
    #[serde(alias = "user_ref")]
    pub client_reference_id: Option<String>,
    #[serde(alias = "customer_email")]
    pub email: Option<String>,
    pub redirect: Option<bool>,
}

fn dispatch_checkout_to_ghl_if_configured(
    state: &AppState,
    headers: &HeaderMap,
    req: &CheckoutRequest,
    payload: &serde_json::Value,
) {
    if !crate::ghl::GhlDispatcher::has_env_or_pit_file() {
        return;
    }
    let session_email = auth::extract_token_from_headers(headers)
        .and_then(|t| state.auth_mgr.get_session(&t))
        .map(|s| s.user.email);
    let candidate_email = req
        .email
        .as_deref()
        .map(str::trim)
        .filter(|email| !email.is_empty())
        .or_else(|| {
            req.client_reference_id
                .as_deref()
                .filter(|r| r.contains('@'))
        })
        .or(session_email.as_deref())
        .map(str::trim)
        .filter(|e| {
            e.contains('@')
                && !e.ends_with("@pressmark.local")
                && !e.ends_with("@example.com")
        })
        .map(str::to_lowercase);

    if let Some(email_cl) = candidate_email {
        let plan_id_cl = payload["plan_id"]
            .as_str()
            .unwrap_or("plan_solo")
            .to_string();
        let stripe_price_id_cl = payload["stripe_price_id"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let price_cents = payload["price_cents"].as_u64().unwrap_or(0) as u32;
        tokio::spawn(async move {
            let ghl = crate::ghl::GhlDispatcher::from_env();
            if ghl.is_configured() {
                let _ = ghl
                    .dispatch_checkout_started(
                        &email_cl,
                        &plan_id_cl,
                        &stripe_price_id_cl,
                        price_cents,
                        "web_checkout",
                    )
                    .await;
            }
        });
    }
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
        req.client_reference_id.as_deref().filter(|reference| !reference.is_empty()),
    );

    serde_json::json!({
        "success": true,
        "request_id": req.request_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
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
    Query(req): Query<CheckoutRequest>,
) -> Response {
    let payload = resolve_checkout_response(&req);
    // Link previews, crawlers and browser prefetches must not start a checkout or notify CRM.
    let mut response = if req.redirect.unwrap_or(false) {
        Html(checkout_review_page(&req, &payload)).into_response()
    } else {
        Json(payload).into_response()
    };
    response.headers_mut().insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
    response.headers_mut().insert("x-robots-tag", "noindex, nofollow".parse().unwrap());
    response
}

fn checkout_html(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
        .replace('"', "&quot;").replace('\'', "&#39;")
}

fn checkout_review_page(req: &CheckoutRequest, payload: &serde_json::Value) -> String {
    let value = |key: &str| checkout_html(payload[key].as_str().unwrap_or(""));
    let reference = checkout_html(req.client_reference_id.as_deref().unwrap_or(""));
    let email = checkout_html(req.email.as_deref().unwrap_or(""));
    let price = match payload["interval"].as_str() {
        Some("month") => format!("{} USD per month", value("price_display")),
        Some("one-time") => format!("{} USD one time", value("price_display")),
        _ => format!("{} USD", value("price_display")),
    };
    format!(r#"<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Review your plan | Pressmark</title><style>body{{margin:0;background:#f6f8fb;color:#182235;font:16px/1.5 system-ui,sans-serif}}main{{max-width:460px;margin:10vh auto;padding:32px;background:white;border:1px solid #dce2eb;border-radius:16px}}h1{{font-size:28px;line-height:1.2}}.brand{{font-weight:700;color:#2541a8}}.price{{font-size:24px;font-weight:600}}button{{width:100%;padding:14px;border:0;border-radius:8px;background:#2847bf;color:white;font:inherit;font-weight:600;cursor:pointer}}button:focus-visible,a:focus-visible{{outline:3px solid #4876e8;outline-offset:3px}}a{{color:#2541a8}}@media(max-width:540px){{main{{margin:24px 16px;padding:24px}}}}</style></head><body><main><p class="brand">Pressmark</p><h1>Review your plan</h1><h2>{name}</h2><p class="price">{price}</p><p>Continue to Stripe to enter your payment details and confirm your purchase.</p><form method="post" action="/api/v1/checkout/continue"><input type="hidden" name="request_id" value="{request_id}"><input type="hidden" name="plan_id" value="{plan_id}"><input type="hidden" name="client_reference_id" value="{reference}"><input type="hidden" name="email" value="{email}"><button type="submit">Continue to payment</button></form><p><a href="/pricing">Back to Pressmark</a></p></main></body></html>"#,
        name = value("plan_name"), price = price,
        request_id = value("request_id"), plan_id = value("plan_id"),
    )
}

async fn handle_checkout_post(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CheckoutRequest>,
) -> Response {
    continue_checkout(&state, &headers, req, false)
}

async fn handle_checkout_continue(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Form(req): Form<CheckoutRequest>,
) -> Response {
    continue_checkout(&state, &headers, req, true)
}

fn continue_checkout(state: &AppState, headers: &HeaderMap, req: CheckoutRequest, redirect: bool) -> Response {
    let origin = headers.get(header::ORIGIN).and_then(|value| value.to_str().ok());
    let same_origin = origin.and_then(|value| reqwest::Url::parse(value).ok())
        .zip(reqwest::Url::parse(&state.base_url).ok())
        .is_some_and(|(origin, base)| origin.origin() == base.origin());
    if (origin.is_some() && !same_origin) || (redirect && !same_origin) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error":"checkout_origin_rejected"}))).into_response();
    }
    let Some(request_id) = req.request_id.as_deref().filter(|id| uuid::Uuid::parse_str(id).is_ok()) else {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "checkout_request_id_required", "message": "Read the checkout endpoint first and reuse its request_id when submitting or retrying."
        }))).into_response();
    };
    let payload = resolve_checkout_response(&req);
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !crate::storage::Storage::is_crawler_bot_ua(user_agent) {
        let checkout_channel = if crate::storage::Storage::is_internal_probe_ua(user_agent) {
            "stripe_probe"
        } else {
            "stripe"
        };
        use sha2::{Digest, Sha256};
        let email = req.email.as_deref().map(str::trim).filter(|email| !email.is_empty()).map(str::to_lowercase);
        let fingerprint = hex::encode(Sha256::digest(serde_json::json!([
            payload["plan_id"], payload["checkout_url"], email
        ]).to_string().as_bytes()));
        match state.storage.record_checkout_start(request_id, &fingerprint, payload["plan_id"].as_str().unwrap_or(""), checkout_channel) {
            Ok(crate::storage::CheckoutStart::Created) => dispatch_checkout_to_ghl_if_configured(state, headers, &req, &payload),
            Ok(crate::storage::CheckoutStart::Replayed) => {},
            Ok(crate::storage::CheckoutStart::Conflict) => return (StatusCode::CONFLICT, Json(serde_json::json!({"error":"checkout_request_changed"}))).into_response(),
            Err(_) => return (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({"error":"checkout_record_unavailable"}))).into_response(),
        }
    }
    if redirect {
        if let Some(url) = payload["checkout_url"].as_str() {
            return axum::response::Redirect::to(url).into_response();
        }
    }
    (StatusCode::OK, Json(payload)).into_response()
}

async fn handle_favicon() -> impl IntoResponse {
    let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="64" height="64"><rect width="64" height="64" rx="14" fill="#0f172a"/><path d="M18 16h18c7.732 0 14 6.268 14 14s-6.268 14-14 14H26v8h-8V16zm8 8v12h10c3.314 0 6-2.686 6-6s-2.686-6-6-6H26z" fill="#38bdf8"/></svg>"##;
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/svg+xml; charset=utf-8")],
        svg,
    )
}

async fn handle_owners_json() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "owners": [
                {
                    "type": "github",
                    "id": "mooshee"
                }
            ]
        })),
    )
}

async fn handle_mcp_owners_json() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "$schema": "https://verifymcp.io/schemas/owners.json",
            "owners": ["daniel@mooshworks.com", "daniel.hallman@gmail.com"]
        })),
    )
}

async fn handle_glama_json() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "$schema": "https://glama.ai/mcp/schemas/connector.json",
            "maintainers": [
                {
                    "email": "daniel@mooshworks.com",
                    "github": "mooshee"
                }
            ]
        })),
    )
}

async fn handle_oauth_protected_resource() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "resource": "https://pressmark.dev/mcp",
            "authorization_servers": ["https://pressmark.dev"],
            "bearer_methods_supported": ["header"],
            "resource_documentation": "https://pressmark.dev/docs"
        })),
    )
}

async fn handle_oauth_authorization_server() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "issuer": "https://pressmark.dev",
            "authorization_endpoint": "https://pressmark.dev/api/v1/auth/chatgpt/login",
            "token_endpoint": "https://pressmark.dev/api/v1/auth/api-key",
            "userinfo_endpoint": "https://pressmark.dev/api/v1/auth/session",
            "response_types_supported": ["code", "token"],
            "grant_types_supported": ["authorization_code", "client_credentials"],
            "token_endpoint_auth_methods_supported": ["none", "client_secret_post", "private_key_jwt"],
            "code_challenge_methods_supported": ["S256"],
            "scopes_supported": ["openid", "profile", "email", "mcp:tools"],
            "service_documentation": "https://pressmark.dev/docs"
        })),
    )
}

async fn handle_x402_discovery() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "version": "1.0",
            "service": "pressmark",
            "url": "https://pressmark.dev/mcp",
            "pricing_url": "https://pressmark.dev/api/v1/pricing",
            "checkout_url": "https://pressmark.dev/api/v1/checkout",
            "free_tier": true,
            "currency": "USD"
        })),
    )
}

async fn handle_agent_card() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "id": "io.github.mooshee/pressmark",
            "name": "Pressmark",
            "description": "High-performance Rust vector PDF document synthesis engine and MCP server for 57 invoice, estimate, receipt, contract, and 50-state statutory real estate archetypes.",
            "url": "https://pressmark.dev",
            "version": env!("CARGO_PKG_VERSION"),
            "provider": {
                "organization": "Moosh Works",
                "url": "https://pressmark.dev"
            },
            "documentationUrl": "https://pressmark.dev/docs",
            "capabilities": {
                "streaming": true,
                "pushNotifications": false,
                "stateTransitionHistory": false
            },
            "defaultInputModes": ["application/json", "text/plain"],
            "defaultOutputModes": ["application/pdf", "image/png", "application/json"],
            "skills": [
                {
                    "id": "invoice-generation",
                    "name": "Invoice & Document Generation",
                    "description": "Synthesize vector PDF invoices, estimates, receipts, contracts, NDAs, and 57 business and real estate document archetypes in sub-25ms with 144-PPI PNG previews and QR code mobile payments.",
                    "tags": ["invoice", "pdf", "estimate", "receipt", "contract", "typst"]
                },
                {
                    "id": "document-lifecycle",
                    "name": "Document Lifecycle & Billing",
                    "description": "Statelessly transition documents across their billing lifecycle (estimate to invoice, receipt, or credit note) and record partial or full payments.",
                    "tags": ["billing", "lifecycle", "invoice", "receipt", "credit-note"]
                },
                {
                    "id": "real-estate-agreements",
                    "name": "Real Estate & Statutory Leases",
                    "description": "Typeset 29 leasing, property management, investor analysis, and deed archetypes with a 50-state + DC + 6 municipal 4-layer statutory lease engine.",
                    "tags": ["real-estate", "lease", "deed", "property-management", "statutory"]
                }
            ]
        })),
    )
}

async fn handle_a2a_post(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Response {
    if let Ok(req) = serde_json::from_slice::<JsonRpcRequest>(&body) {
        return handle_mcp(State(state), headers, Json(req)).await;
    }
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "ok",
            "agent": "io.github.mooshee/pressmark",
            "version": env!("CARGO_PKG_VERSION"),
            "message": "Pressmark vector document engine is active and ready for MCP tool invocations."
        })),
    )
        .into_response()
}

async fn handle_mcp_delete() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

async fn handle_security_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "Contact: mailto:daniel@mooshworks.com\nExpires: 2027-12-31T23:59:59.000Z\nPreferred-Languages: en\nCanonical: https://pressmark.dev/.well-known/security.txt\nPolicy: https://pressmark.dev/privacy\n",
    )
}

async fn handle_capabilities_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "name: Pressmark\ncapabilities:\n  - vector_pdf_synthesis\n  - 57_document_archetypes\n  - mcp_streamable_http\n  - png_preview_rendering\n  - temporary_pdf_downloads\nendpoints:\n  mcp: https://pressmark.dev/mcp\n  studio: https://pressmark.dev\n  openapi: https://pressmark.dev/openapi.json\nprivacy: https://pressmark.dev/privacy\n",
    )
}

async fn handle_did_json() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "@context": ["https://www.w3.org/ns/did/v1"],
            "id": "did:web:pressmark.dev",
            "service": [{
                "id": "did:web:pressmark.dev#mcp",
                "type": "ModelContextProtocolService",
                "serviceEndpoint": "https://pressmark.dev/mcp"
            }]
        })),
    )
}

async fn handle_brick_blue_json() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "name": "Pressmark",
            "domain": "pressmark.dev",
            "agent_url": "https://pressmark.dev/mcp",
            "protocol": "mcp",
            "version": env!("CARGO_PKG_VERSION"),
            "verified": true
        })),
    )
}

async fn handle_discovery_resources() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "resources": [
                {
                    "uri": "pressmark://templates/catalog",
                    "name": "Document Archetypes Catalog",
                    "mimeType": "application/json",
                    "description": "Catalog of 57 business and real estate document templates"
                },
                {
                    "uri": "pressmark://pricing/catalog",
                    "name": "Pricing & Entitlements",
                    "mimeType": "application/json",
                    "description": "Pressmark subscription and single-pass export pricing"
                }
            ]
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
            "description": "Vector document engine for 57 invoice, receipt, contract, and real estate PDFs with QR payments.",
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
                "archetypes_count": DocumentType::all().len(),
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
        [
            (header::HeaderName::from_static("mcp-protocol-version"), "2026-07-28"),
            (header::HeaderName::from_static("x-mcp-protocol-version"), "2026-07-28"),
        ],
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
    let private_result = matches!(mcp_method.as_str(), "tools/call" | "resources/read");
    let tool_name = req
        .params
        .as_ref()
        .and_then(|p| p.get("name"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| (mcp_method != "tools/call").then(|| mcp_method.clone()));
    fn extract_doc_type_field(obj: &serde_json::Value) -> Option<&serde_json::Value> {
        obj.get("doc_type")
            .or_else(|| obj.get("archetype"))
            .or_else(|| obj.get("type"))
            .or_else(|| obj.get("template"))
            .or_else(|| obj.get("document_type"))
    }
    let doc_type = req
        .params
        .as_ref()
        .and_then(|p| p.get("arguments"))
        .and_then(|a| {
            a.get("target_type")
                .or_else(|| extract_doc_type_field(a))
                .or_else(|| a.get("document").and_then(extract_doc_type_field))
        })
        .and_then(|v| v.as_str())
        .map(|s| {
            DocumentType::from_slug(s)
                .map(|dt| dt.as_str().to_string())
                .or_else(|| {
                    DocumentType::parse_state_prefixed_slug(s)
                        .map(|(_, dt)| dt.as_str().to_string())
                })
                .unwrap_or_else(|| s.trim().to_lowercase().replace('-', "_"))
        });

    let header_proto = headers
        .get("mcp-protocol-version")
        .or_else(|| headers.get("x-mcp-protocol-version"))
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    let client_proto = header_proto
        .clone()
        .unwrap_or_else(|| "2026-07-28".to_string());
    let is_2026 = header_proto.as_deref() == Some("2026-07-28") || req.method == "server/discover";

    let maybe_token = auth::extract_token_from_headers(&headers);
    let session = maybe_token
        .as_deref()
        .and_then(|t| state.auth_mgr.get_session(t));
    let is_paid_session = session.as_ref().map(|s| s.user.is_subscriber).unwrap_or(false);
    let session_email = session.as_ref().map(|s| s.user.email.as_str());

    let attempt_id = if mcp_method == "tools/call" {
        state.storage.begin_mcp_tool_attempt(doc_type.as_deref(), user_agent.unwrap_or("mcp"), tool_name.as_deref())
    } else {
        None
    };

    let mut resp = if let Some(response) = read_download_resource(&state, &req) {
        response
    } else {
        McpHandler::handle_request_with_context(req, Some(&state.storage), is_paid_session, session_email)
    };
    if resp.tool_succeeded()
        && matches!(tool_name.as_deref(), Some("generate_document" | "convert_document" | "record_payment"))
    {
        if let Some(result) = resp.result.as_mut() {
            attach_pdf_download(&state, result, doc_type.as_deref().unwrap_or("document"), &client_proto);
        }
    }
    if resp.tool_succeeded() && tool_name.as_deref() == Some("preview_document") {
        if let Some(result) = resp.result.as_mut() {
            attach_preview_download(&state, result, doc_type.as_deref().unwrap_or("document"), &client_proto);
        }
    }
    let compile_ms = resp
        .result
        .as_ref()
        .and_then(|r| r.get("compile_time_ms"))
        .and_then(|v| v.as_u64());

    if mcp_method == "tools/call" {
        if let Some(id) = attempt_id {
            let success = resp.tool_succeeded();
            let error_code = resp.error.as_ref().and_then(|error| error.get("code")).and_then(|value| value.as_i64());
            state.storage.finish_mcp_tool_attempt(id, success, error_code, compile_ms);
        }
    } else {
        state.storage.record_usage_event("mcp_protocol", doc_type.as_deref(), user_agent.unwrap_or("mcp"), tool_name.as_deref(), compile_ms);
    }
    if is_2026 && mcp_method != "initialize" && mcp_method != "ping" && !mcp_method.starts_with("notifications/") {
        if let Some(ref mut result) = resp.result {
            if let Some(obj) = result.as_object_mut() {
                obj.entry("resultType".to_string())
                    .or_insert_with(|| serde_json::json!("complete"));
                if mcp_method != "tools/call" {
                    obj.entry("ttlMs".to_string())
                        .or_insert_with(|| serde_json::json!(if private_result { 0 } else { 300_000 }));
                    obj.entry("cacheScope".to_string())
                        .or_insert_with(|| serde_json::json!(if private_result { "private" } else { "public" }));
                }
                obj.entry("_meta".to_string()).or_insert_with(|| {
                    serde_json::json!({
                        "io.modelcontextprotocol/serverInfo": {
                            "name": "pressmark",
                            "title": "Pressmark Vector Document Engine",
                            "version": crate::mcp::MCP_SERVER_VERSION
                        },
                        "protocolVersion": "2026-07-28",
                        "streamingSupported": true,
                        "cacheTtlMs": if private_result { 0 } else { 300_000 }
                    })
                });
            }
        }
    }

    let mut resp_headers = HeaderMap::new();
    if private_result {
        resp_headers.insert(header::CACHE_CONTROL, header::HeaderValue::from_static("private, no-store"));
    }
    let negotiated_proto = resp
        .result
        .as_ref()
        .and_then(|r| r.get("protocolVersion"))
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let proto_header = if is_2026 {
        "2026-07-28"
    } else if let Some(ref np) = negotiated_proto {
        np.as_str()
    } else if crate::mcp::SUPPORTED_MCP_PROTOCOLS.contains(&client_proto.as_str()) {
        client_proto.as_str()
    } else {
        "2026-07-28"
    };
    if let Ok(hv) = proto_header.parse::<header::HeaderValue>() {
        resp_headers.insert("mcp-protocol-version", hv.clone());
        resp_headers.insert("x-mcp-protocol-version", hv);
    }

    if is_notif && resp.id.is_none() {
        return (StatusCode::ACCEPTED, resp_headers).into_response();
    }

    (StatusCode::OK, resp_headers, Json(resp)).into_response()
}


fn read_download_resource(state: &AppState, request: &JsonRpcRequest) -> Option<JsonRpcResponse> {
    if request.method != "resources/read" { return None; }
    let uri = request.params.as_ref()?.get("uri")?.as_str()?;
    // Catalogs are handled by the MCP dispatcher. Download resources are local cache lookups only.
    if uri.starts_with("pressmark://catalog/") { return None; }
    let file = reqwest::Url::parse(uri).ok().and_then(|url| {
        let base = reqwest::Url::parse(&state.base_url).ok()?;
        if url.origin() != base.origin() || !url.username().is_empty() || url.password().is_some()
            || url.query().is_some() || url.fragment().is_some() {
            return None;
        }
        let token = url.path().strip_prefix("/api/v1/downloads/")?;
        state.downloads.get(token)
    });
    Some(match file {
        Some(file) => JsonRpcResponse {
            jsonrpc: "2.0", id: request.id.clone(), error: None,
            result: Some(serde_json::json!({"contents": [{"uri": uri, "mimeType": file.mime_type, "blob": BASE64.encode(file.bytes)}]})),
        },
        None => JsonRpcResponse {
            jsonrpc: "2.0", id: request.id.clone(), result: None,
            error: Some(serde_json::json!({"code": -32002, "message": "Resource not found or expired. Generate a new document or preview."})),
        },
    })
}

fn attach_preview_download(state: &AppState, result: &mut serde_json::Value, doc_type: &str, _protocol: &str) {
    use crate::downloads::{DownloadError, MAX_DOWNLOAD_BYTES};
    let Some(encoded) = result["content"].as_array().and_then(|items| items.iter()
        .find(|item| item["type"] == "image" && item["mimeType"] == "image/png"))
        .and_then(|image| image["data"].as_str()) else { return; };
    let stored = if encoded.len() > ((MAX_DOWNLOAD_BYTES + 2) / 3) * 4 {
        Err(DownloadError::TooLarge)
    } else {
        BASE64.decode(encoded).map_err(|_| DownloadError::InvalidPng)
            .and_then(|bytes| state.downloads.insert_png(bytes, doc_type, "preview", &state.base_url))
    };
    match stored {
        Ok(link) => {
            let metadata = serde_json::to_value(&link).expect("serializable preview metadata");
            result["preview"] = metadata.clone();
            result["structuredContent"]["preview"] = metadata;
            if let Some(content) = result["content"].as_array_mut() {
                content.insert(0, serde_json::json!({"type": "text", "text": format!(
                    "[Open document preview]({})\n\nThe link expires in 15 minutes and may stop working sooner if the service restarts. Anyone with the link can view the preview while it is valid.", link.url)}));
            }
        },
        Err(error) => {
            result["preview_error"] = serde_json::json!(error.message());
        },
    }
}

fn attach_pdf_download(state: &AppState, result: &mut serde_json::Value, fallback_type: &str, _protocol: &str) {
    use crate::downloads::{DownloadError, MAX_DOWNLOAD_BYTES};
    let Some(encoded) = result.get("pdf_base64").and_then(|v| v.as_str()) else { return; };
    let stored = if encoded.len() > ((MAX_DOWNLOAD_BYTES + 2) / 3) * 4 {
        Err(DownloadError::TooLarge)
    } else {
        BASE64.decode(encoded).map_err(|_| DownloadError::InvalidPdf).and_then(|pdf| {
            let doc = &result["document"];
            let kind = doc.get("doc_type").and_then(|v| v.as_str()).unwrap_or(fallback_type);
            let number = doc.get("doc_number").or_else(|| result.get("doc_number"))
                .and_then(|v| v.as_str()).unwrap_or_default();
            state.downloads.insert(pdf, kind, number, &state.base_url)
        })
    };
    match stored {
        Ok(link) => {
            let message = format!("[Download PDF]({})\n\nThis link expires in 15 minutes and may stop working sooner if the service restarts. Anyone with the link can download the PDF while it is valid.", link.url);
            let metadata = serde_json::to_value(&link).expect("serializable download metadata");
            result["download"] = metadata.clone();
            result["structuredContent"]["download"] = metadata;
            if let Some(content) = result["content"].as_array_mut() {
                content.retain(|item| !(item["type"] == "resource" && item["resource"]["mimeType"] == "application/pdf"));
                content.insert(0, serde_json::json!({"type": "text", "text": message}));
            }
        }
        Err(error) => {
            result["download_error"] = serde_json::json!(error.message());
            result["structuredContent"]["download_error"] = serde_json::json!(error.message());
            if let Some(content) = result["content"].as_array_mut() {
                content.push(serde_json::json!({"type": "text", "text": error.message()}));
            }
        }
    }
}

#[path = "server_lifecycle_and_tests.rs"]
mod server_lifecycle_and_tests;
pub use server_lifecycle_and_tests::*;
