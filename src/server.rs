//! Pressmark Axum HTTP Server & API Router
//!
//! Author: Daniel Hallman

use crate::auth::{
    self, ApiKeyLoginRequest, CallbackQuery, ChatGptAuthManager, EmailLoginRequest, LoginQuery,
};
use crate::engine::{
    compile_pdf_bytes, compile_png_bytes, generate_all_preview_pngs, generate_pdf,
    generate_preview_png,
};
use crate::mcp::{JsonRpcRequest, JsonRpcResponse, McpHandler};
use crate::models::{
    CopilotRequest, CopilotResponse, DocumentRequest, DocumentType, GenerateResponse, LineItem,
    PartyDetails, PreviewResponse, VisionRequest, VisionResponse,
};
use crate::openapi::{get_ai_plugin_manifest, get_openapi_spec};
use crate::pricing::get_pricing_catalog;
use crate::rate_limit::{rate_limit_middleware, RateLimiter};
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

#[derive(Clone)]
pub struct AppState {
    pub base_url: String,
    pub auth_mgr: ChatGptAuthManager,
    pub rate_limiter: RateLimiter,
    pub stripe_idempotency: crate::stripe::StripeIdempotencyStore,
    pub storage: Arc<crate::storage::Storage>,
    pub downloads: crate::downloads::DocumentDownloadCache,
    pub feedback: Option<crate::feedback::FeedbackClient>,
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
        let stripe_idempotency =
            crate::stripe::StripeIdempotencyStore::new().with_storage(Arc::clone(&storage));

        Self {
            base_url,
            downloads: crate::downloads::DocumentDownloadCache::default(),
            auth_mgr,
            rate_limiter: RateLimiter::new(max_req, window_secs),
            stripe_idempotency,
            storage,
            feedback: crate::feedback::FeedbackClient::from_env(),
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
        .unwrap_or(5)
        * 1024
        * 1024;

    Router::new()
        // Interactive Dashboard, Legal & SEO
        .route("/", get(handle_index).post(handle_mcp))
        .route("/pricing", get(handle_index))
        .route("/demo", get(handle_index))
        .route("/docs", get(handle_index))
        .route("/templates", get(handle_index))
        .route("/templates/", get(handle_index))
        .route("/templates/:slug", get(handle_template_landing))
        .route("/templates/:slug/", get(handle_template_landing_trailing))
        .route("/elements", get(crate::concepts_showcase::handle_pressmark_elements_hub))
        .route("/elements/", get(crate::concepts_showcase::handle_pressmark_elements_hub))
        .route("/concepts", get(crate::concepts_showcase::handle_pressmark_elements_hub))
        .route("/concepts/", get(crate::concepts_showcase::handle_pressmark_elements_hub))
        .route("/concepts/:slug", get(crate::concepts_showcase::handle_pressmark_concept_detail))
        .route("/api/v1/concepts/catalog.json", get(crate::concepts_showcase::handle_pressmark_concepts_catalog_json))
        .route("/api/v1/concepts/:slug/preview.png", get(crate::concepts_showcase::handle_pressmark_concept_preview_png))
        .route("/api/v1/concepts/:slug/sample.pdf", get(crate::concepts_showcase::handle_pressmark_concept_sample_pdf))
        .route("/assets/seals/:filename", get(crate::concepts_showcase::handle_pressmark_seal_asset))
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
        .route(
            "/contact",
            get(|| async { Redirect::temporary("/feedback") }),
        )
        .route(
            "/support",
            get(|| async { Redirect::temporary("/feedback") }),
        )
        .route("/feedback", get(crate::feedback::page))
        .route("/api/v1/feedback", post(crate::feedback::submit))
        .route("/api/v1/feedback/status", post(crate::feedback::status))
        .route("/api/v1/feedback/health", get(crate::feedback::health))
        .route("/help", get(|| async { Redirect::permanent("/") }))
        .route("/faq", get(|| async { Redirect::permanent("/") }))
        .route(
            "/features",
            get(|| async { Redirect::permanent("/#pricing") }),
        )
        .route(
            "/product",
            get(|| async { Redirect::permanent("/#pricing") }),
        )
        .route(
            "/documentation",
            get(|| async { Redirect::permanent("/docs") }),
        )
        .route(
            "/security",
            get(|| async { Redirect::permanent("/.well-known/security.txt") }),
        )
        .route("/security.txt", get(handle_security_txt))
        .route("/.well-known/security.txt", get(handle_security_txt))
        .route(
            "/.well-known/mcpprobe-opt-out",
            get(handle_mcpprobe_opt_out),
        )
        .route("/ads.txt", get(handle_ads_txt))
        .route("/sellers.json", get(handle_sellers_json))
        .route("/plans", get(|| async { Redirect::permanent("/pricing") }))
        .route("/plans/", get(|| async { Redirect::permanent("/pricing") }))
        .route(
            "/ai.txt",
            get(|| async { Redirect::permanent("/llms.txt") }),
        )
        .route("/humans.txt", get(handle_humans_txt))
        .route("/.well-known/tdmrep.json", get(handle_tdmrep_json))
        .route("/docs/mcp", get(|| async { Redirect::permanent("/mcp") }))
        .route("/docs/mcp/", get(|| async { Redirect::permanent("/mcp") }))
        .route(
            "/AGENTS.md",
            get(|| async { Redirect::permanent("/llms.txt") }),
        )
        .route(
            "/AGENT.md",
            get(|| async { Redirect::permanent("/llms.txt") }),
        )
        .route(
            "/QWEN.md",
            get(|| async { Redirect::permanent("/llms.txt") }),
        )
        .route(
            "/CLAUDE.md",
            get(|| async { Redirect::permanent("/llms.txt") }),
        )
        // API v1
        .route("/api/v1/generate", post(handle_generate))
        .route("/api/v1/downloads/:token", get(handle_document_download))
        .route("/api/v1/preview", post(handle_preview))
        .route("/api/v1/render-raw", post(handle_render_raw))
        .route("/api/v1/render-vector", post(handle_render_raw))
        .route("/api/v1/templates", get(handle_templates_catalog))
        .route("/api/v1/states", get(handle_states_catalog))
        .route("/api/v1/pricing", get(handle_pricing))
        .route(
            "/api/v1/checkout",
            get(handle_checkout_get).post(handle_checkout_post),
        )
        .route(
            "/api/v1/checkout/continue",
            get(|| async { Redirect::to("/#pricing") }).post(handle_checkout_continue),
        )
        .route(
            "/api/v1/setup-check",
            get(handle_setup_check).post(handle_setup_check),
        )
        .route(
            "/api/v1/analytics",
            get(handle_analytics).post(handle_analytics),
        )
        .route("/api/v1/openapi.json", get(handle_openapi))
        .route("/api/v1/copilot", post(handle_copilot))
        .route("/api/v1/vision-extract", post(handle_vision_extract))
        .route("/api/v1/lifecycle/convert", post(handle_lifecycle_convert))
        .route("/api/v1/lifecycle/pay", post(handle_lifecycle_pay))
        .route("/api/v1/webhooks/stripe", post(handle_stripe_webhook))
        .route("/.well-known/ai-plugin.json", get(handle_ai_plugin))
        .route(
            "/.well-known/openai-apps-challenge",
            get(handle_openai_apps_challenge),
        )
        .route("/.well-known/owners.json", get(handle_owners_json))
        .route("/mcp/.well-known/owners.json", get(handle_mcp_owners_json))
        .route(
            "/api/v1/mcp/.well-known/owners.json",
            get(handle_mcp_owners_json),
        )
        .route("/.well-known/glama.json", get(handle_glama_json))
        .route(
            "/.well-known/oauth-protected-resource",
            get(handle_oauth_protected_resource),
        )
        .route(
            "/.well-known/oauth-protected-resource/mcp",
            get(handle_oauth_protected_resource),
        )
        .route(
            "/.well-known/oauth-protected-resource/api/v1/mcp",
            get(handle_oauth_protected_resource),
        )
        .route(
            "/mcp/.well-known/oauth-protected-resource",
            get(handle_oauth_protected_resource),
        )
        .route(
            "/.well-known/oauth-authorization-server",
            get(handle_oauth_authorization_server),
        )
        .route(
            "/.well-known/oauth-authorization-server/mcp",
            get(handle_oauth_authorization_server),
        )
        .route(
            "/.well-known/oauth-authorization-server/api/v1/mcp",
            get(handle_oauth_authorization_server),
        )
        .route(
            "/mcp/.well-known/oauth-authorization-server",
            get(handle_oauth_authorization_server),
        )
        .route(
            "/.well-known/openid-configuration",
            get(handle_oauth_authorization_server),
        )
        .route(
            "/.well-known/openid-configuration/mcp",
            get(handle_oauth_authorization_server),
        )
        .route(
            "/.well-known/openid-configuration/api/v1/mcp",
            get(handle_oauth_authorization_server),
        )
        .route(
            "/mcp/.well-known/openid-configuration",
            get(handle_oauth_authorization_server),
        )
        .route(
            "/.well-known/mcp/server-card.json",
            get(handle_mcp_server_card),
        )
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
        .route(
            "/api/v1/mcp/.well-known/agent-card.json",
            get(handle_agent_card),
        )
        .route("/mcp/server-card", get(handle_mcp_server_card))
        .route("/mcp/server-card.json", get(handle_mcp_server_card))
        .route("/api/v1/mcp/server-card", get(handle_mcp_server_card))
        .route("/api/v1/mcp/server-card.json", get(handle_mcp_server_card))
        .route(
            "/.well-known/capabilities.txt",
            get(handle_capabilities_txt),
        )
        .route("/.well-known/did.json", get(handle_did_json))
        .route("/.well-known/brick-blue.json", get(handle_brick_blue_json))
        .route("/discovery/resources", get(handle_discovery_resources))
        .route("/.well-known/x402", get(handle_x402_discovery))
        .route("/.well-known/x402.json", get(handle_x402_discovery))
        .route("/a2a", get(handle_agent_card).post(handle_a2a_post))
        .route("/a2a/", get(handle_agent_card).post(handle_a2a_post))
        .route(
            "/api/v1/mcp/a2a",
            get(handle_agent_card).post(handle_a2a_post),
        )
        .route(
            "/api/v1/mcp/a2a/",
            get(handle_agent_card).post(handle_a2a_post),
        )
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
        .route(
            "/api/v1/auth/chatgpt/callback",
            get(handle_chatgpt_callback),
        )
        .route("/api/v1/auth/chatgpt/session", get(handle_chatgpt_session))
        .route("/api/v1/auth/chatgpt/logout", post(handle_chatgpt_logout))
        // MCP Endpoints & Tool Discovery
        .route(
            "/mcp",
            get(handle_mcp_get)
                .post(handle_mcp)
                .delete(handle_mcp_delete),
        )
        .route(
            "/mcp/",
            get(handle_mcp_get)
                .post(handle_mcp)
                .delete(handle_mcp_delete),
        )
        .route(
            "/mcp/mcp",
            get(handle_mcp_get)
                .post(handle_mcp)
                .delete(handle_mcp_delete),
        )
        .route(
            "/mcp/v1",
            get(handle_mcp_get)
                .post(handle_mcp)
                .delete(handle_mcp_delete),
        )
        .route(
            "/api/mcp",
            get(handle_mcp_get)
                .post(handle_mcp)
                .delete(handle_mcp_delete),
        )
        .route(
            "/mcp.",
            get(|| async { Redirect::permanent("/mcp") })
                .post(|| async { Redirect::permanent("/mcp") }),
        )
        .route(
            "/mcp\"",
            get(|| async { Redirect::permanent("/mcp") })
                .post(|| async { Redirect::permanent("/mcp") }),
        )
        .route(
            "/mcp%22",
            get(|| async { Redirect::permanent("/mcp") })
                .post(|| async { Redirect::permanent("/mcp") }),
        )
        .route(
            "/mcp`",
            get(|| async { Redirect::permanent("/mcp") })
                .post(|| async { Redirect::permanent("/mcp") }),
        )
        .route(
            "/mcp%60",
            get(|| async { Redirect::permanent("/mcp") })
                .post(|| async { Redirect::permanent("/mcp") }),
        )
        .route(
            "/mcp)",
            get(|| async { Redirect::permanent("/mcp") })
                .post(|| async { Redirect::permanent("/mcp") }),
        )
        .route(
            "/mcp%29",
            get(|| async { Redirect::permanent("/mcp") })
                .post(|| async { Redirect::permanent("/mcp") }),
        )
        .route(
            "/api/v1/mcp.",
            get(|| async { Redirect::permanent("/api/v1/mcp") })
                .post(|| async { Redirect::permanent("/api/v1/mcp") }),
        )
        .route(
            "/api/v1/mcp\"",
            get(|| async { Redirect::permanent("/api/v1/mcp") })
                .post(|| async { Redirect::permanent("/api/v1/mcp") }),
        )
        .route(
            "/api/v1/mcp%22",
            get(|| async { Redirect::permanent("/api/v1/mcp") })
                .post(|| async { Redirect::permanent("/api/v1/mcp") }),
        )
        .route(
            "/api/v1/mcp`",
            get(|| async { Redirect::permanent("/api/v1/mcp") })
                .post(|| async { Redirect::permanent("/api/v1/mcp") }),
        )
        .route(
            "/api/v1/mcp%60",
            get(|| async { Redirect::permanent("/api/v1/mcp") })
                .post(|| async { Redirect::permanent("/api/v1/mcp") }),
        )
        .route(
            "/api/v1/mcp)",
            get(|| async { Redirect::permanent("/api/v1/mcp") })
                .post(|| async { Redirect::permanent("/api/v1/mcp") }),
        )
        .route(
            "/api/v1/mcp%29",
            get(|| async { Redirect::permanent("/api/v1/mcp") })
                .post(|| async { Redirect::permanent("/api/v1/mcp") }),
        )
        .route(
            "/api/v1/mcp/tools.json`",
            get(|| async { Redirect::permanent("/api/v1/mcp/tools.json") }),
        )
        .route(
            "/api/v1/mcp/tools.json%60",
            get(|| async { Redirect::permanent("/api/v1/mcp/tools.json") }),
        )
        .route(
            "/api/v1/openapi.json`",
            get(|| async { Redirect::permanent("/api/v1/openapi.json") }),
        )
        .route(
            "/api/v1/openapi.json%60",
            get(|| async { Redirect::permanent("/api/v1/openapi.json") }),
        )
        .route(
            "/api/v1/states`",
            get(|| async { Redirect::permanent("/api/v1/states") }),
        )
        .route(
            "/api/v1/states%60",
            get(|| async { Redirect::permanent("/api/v1/states") }),
        )
        .route(
            "/.well-known/ai-plugin.json`",
            get(|| async { Redirect::permanent("/.well-known/ai-plugin.json") }),
        )
        .route(
            "/.well-known/ai-plugin.json%60",
            get(|| async { Redirect::permanent("/.well-known/ai-plugin.json") }),
        )
        .route("/docs\"", get(|| async { Redirect::permanent("/docs") }))
        .route("/docs%22", get(|| async { Redirect::permanent("/docs") }))
        .route("/docs`", get(|| async { Redirect::permanent("/docs") }))
        .route("/docs%60", get(|| async { Redirect::permanent("/docs") }))
        .route("/docs)", get(|| async { Redirect::permanent("/docs") }))
        .route("/docs%29", get(|| async { Redirect::permanent("/docs") }))
        .route("/docs/", get(|| async { Redirect::permanent("/docs") }))
        .route(
            "/privacy)",
            get(|| async { Redirect::permanent("/privacy") }),
        )
        .route(
            "/privacy%29",
            get(|| async { Redirect::permanent("/privacy") }),
        )
        .route(
            "/privacy`",
            get(|| async { Redirect::permanent("/privacy") }),
        )
        .route(
            "/privacy%60",
            get(|| async { Redirect::permanent("/privacy") }),
        )
        .route(
            "/privacy\"",
            get(|| async { Redirect::permanent("/privacy") }),
        )
        .route(
            "/privacy%22",
            get(|| async { Redirect::permanent("/privacy") }),
        )
        .route(
            "/privacy/",
            get(|| async { Redirect::permanent("/privacy") }),
        )
        .route("/terms)", get(|| async { Redirect::permanent("/terms") }))
        .route("/terms%29", get(|| async { Redirect::permanent("/terms") }))
        .route("/terms`", get(|| async { Redirect::permanent("/terms") }))
        .route("/terms%60", get(|| async { Redirect::permanent("/terms") }))
        .route("/terms\"", get(|| async { Redirect::permanent("/terms") }))
        .route("/terms%22", get(|| async { Redirect::permanent("/terms") }))
        .route("/terms/", get(|| async { Redirect::permanent("/terms") }))
        .route(
            "/pricing)",
            get(|| async { Redirect::permanent("/pricing") }),
        )
        .route(
            "/pricing%29",
            get(|| async { Redirect::permanent("/pricing") }),
        )
        .route(
            "/pricing`",
            get(|| async { Redirect::permanent("/pricing") }),
        )
        .route(
            "/pricing%60",
            get(|| async { Redirect::permanent("/pricing") }),
        )
        .route(
            "/pricing\"",
            get(|| async { Redirect::permanent("/pricing") }),
        )
        .route(
            "/pricing%22",
            get(|| async { Redirect::permanent("/pricing") }),
        )
        .route(
            "/pricing/",
            get(|| async { Redirect::permanent("/pricing") }),
        )
        .route("/mcp/sse", get(handle_mcp_get).post(handle_mcp))
        .route("/sse", get(handle_mcp_get).post(handle_mcp))
        .route(
            "/api/v1/mcp",
            get(handle_mcp_get)
                .post(handle_mcp)
                .delete(handle_mcp_delete),
        )
        .route(
            "/api/v1/mcp/",
            get(handle_mcp_get)
                .post(handle_mcp)
                .delete(handle_mcp_delete),
        )
        .route(
            "/api/v1/mcp/mcp",
            get(handle_mcp_get)
                .post(handle_mcp)
                .delete(handle_mcp_delete),
        )
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
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "healthy",
            "service": "pressmark",
            "engine": "pressmark-vector-v1",
            "version": crate::mcp::MCP_SERVER_VERSION
        })),
    )
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
    headers.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("private, no-store, max-age=0"),
    );
    headers.insert(
        "cdn-cache-control",
        header::HeaderValue::from_static("no-store"),
    );
    headers.insert(
        "cloudflare-cdn-cache-control",
        header::HeaderValue::from_static("no-store"),
    );
    headers.insert(
        "x-robots-tag",
        header::HeaderValue::from_static("noindex, nofollow, noarchive"),
    );
    headers.insert(
        "referrer-policy",
        header::HeaderValue::from_static("no-referrer"),
    );
    headers.insert(
        "x-content-type-options",
        header::HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        header::HeaderValue::from_static("default-src 'none'; sandbox"),
    );
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
        Ok(html) => (
            StatusCode::OK,
            Html(html.replace(
                "<div class=\"studio-help\">",
                "<div class=\"studio-help\"><a href=\"/feedback\">Give feedback</a>",
            )),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(format!("<h1>Error loading index.html: {}</h1>", e)),
        ),
    }
}

async fn handle_template_landing_trailing(Path(slug): Path<String>) -> Response {
    let clean_slug =
        slug.trim_end_matches([':', ')', ']', '}', '"', '\'', '.', ',', ';', '>', '/']);
    Redirect::permanent(&format!("/templates/{}", clean_slug)).into_response()
}

async fn handle_template_landing(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Response {
    let clean_slug = slug.trim_end_matches([':', ')', ']', '}', '"', '\'', '.', ',', ';', '>']);
    if clean_slug != slug
        && (DocumentType::from_slug(clean_slug).is_some()
            || DocumentType::parse_state_prefixed_slug(clean_slug).is_some())
    {
        return Redirect::permanent(&format!("/templates/{}", clean_slug)).into_response();
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
            [(
                axum::http::header::CONTENT_TYPE,
                "text/plain; charset=utf-8",
            )],
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
    xml.push_str("  <url><loc>https://pressmark.dev/elements</loc><changefreq>weekly</changefreq><priority>0.9</priority></url>\n");
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

    for concept in hallman_doc_engine::catalog::ARCHETYPE_CONCEPTS_50 {
        xml.push_str(&format!(
            "  <url><loc>https://pressmark.dev/concepts/{}</loc><changefreq>weekly</changefreq><priority>0.8</priority></url>\n",
            concept.slug
        ));
    }

    xml.push_str("</urlset>\n");
    (
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            "application/xml; charset=utf-8",
        )],
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
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; charset=utf-8",
        )],
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
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; charset=utf-8",
        )],
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
    let session = maybe_token
        .as_deref()
        .and_then(|t| state.auth_mgr.get_session(t));
    let is_chatgpt_subscriber = session
        .as_ref()
        .map(|s| s.user.is_subscriber)
        .unwrap_or(false);

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
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok());

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
                    .or(doc_req.from.email.as_deref())
                    .or(doc_req.to.email.as_deref())
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
                        || state.storage.get_customer_entitlement(&email_cl).is_some();
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
                let safe_base = if safe_doc_num.is_empty() {
                    "document"
                } else {
                    &safe_doc_num
                };
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
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok());
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
            e.contains('@') && !e.ends_with("@pressmark.local") && !e.ends_with("@example.com")
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
        req.client_reference_id
            .as_deref()
            .filter(|reference| !reference.is_empty()),
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

async fn handle_checkout_get(Query(req): Query<CheckoutRequest>) -> Response {
    let payload = resolve_checkout_response(&req);
    // Link previews, crawlers and browser prefetches must not start a checkout or notify CRM.
    let mut response = if req.redirect.unwrap_or(false) {
        Html(checkout_review_page(&req, &payload)).into_response()
    } else {
        Json(payload).into_response()
    };
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
    response
        .headers_mut()
        .insert("x-robots-tag", "noindex, nofollow".parse().unwrap());
    response
}

fn checkout_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
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
    format!(
        r#"<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Review your plan | Pressmark</title><style>body{{margin:0;background:#f6f8fb;color:#182235;font:16px/1.5 system-ui,sans-serif}}main{{max-width:460px;margin:10vh auto;padding:32px;background:white;border:1px solid #dce2eb;border-radius:16px}}h1{{font-size:28px;line-height:1.2}}.brand{{font-weight:700;color:#2541a8}}.price{{font-size:24px;font-weight:600}}button{{width:100%;padding:14px;border:0;border-radius:8px;background:#2847bf;color:white;font:inherit;font-weight:600;cursor:pointer}}button:focus-visible,a:focus-visible{{outline:3px solid #4876e8;outline-offset:3px}}a{{color:#2541a8}}@media(max-width:540px){{main{{margin:24px 16px;padding:24px}}}}</style></head><body><main><p class="brand">Pressmark</p><h1>Review your plan</h1><h2>{name}</h2><p class="price">{price}</p><p>Continue to Stripe to enter your payment details and confirm your purchase.</p><form method="post" action="/api/v1/checkout/continue"><input type="hidden" name="request_id" value="{request_id}"><input type="hidden" name="plan_id" value="{plan_id}"><input type="hidden" name="client_reference_id" value="{reference}"><input type="hidden" name="email" value="{email}"><button type="submit">Continue to payment</button></form><p><a href="/pricing">Back to Pressmark</a></p></main></body></html>"#,
        name = value("plan_name"),
        price = price,
        request_id = value("request_id"),
        plan_id = value("plan_id"),
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

fn continue_checkout(
    state: &AppState,
    headers: &HeaderMap,
    req: CheckoutRequest,
    redirect: bool,
) -> Response {
    let origin = headers
        .get(header::ORIGIN)
        .and_then(|value| value.to_str().ok());
    let same_origin = origin
        .and_then(|value| reqwest::Url::parse(value).ok())
        .zip(reqwest::Url::parse(&state.base_url).ok())
        .is_some_and(|(origin, base)| origin.origin() == base.origin());
    if (origin.is_some() && !same_origin) || (redirect && !same_origin) {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error":"checkout_origin_rejected"})),
        )
            .into_response();
    }
    let Some(request_id) = req
        .request_id
        .as_deref()
        .filter(|id| uuid::Uuid::parse_str(id).is_ok())
    else {
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
        let email = req
            .email
            .as_deref()
            .map(str::trim)
            .filter(|email| !email.is_empty())
            .map(str::to_lowercase);
        let fingerprint = hex::encode(Sha256::digest(
            serde_json::json!([payload["plan_id"], payload["checkout_url"], email])
                .to_string()
                .as_bytes(),
        ));
        match state.storage.record_checkout_start(
            request_id,
            &fingerprint,
            payload["plan_id"].as_str().unwrap_or(""),
            checkout_channel,
        ) {
            Ok(crate::storage::CheckoutStart::Created) => {
                dispatch_checkout_to_ghl_if_configured(state, headers, &req, &payload)
            }
            Ok(crate::storage::CheckoutStart::Replayed) => {}
            Ok(crate::storage::CheckoutStart::Conflict) => {
                return (
                    StatusCode::CONFLICT,
                    Json(serde_json::json!({"error":"checkout_request_changed"})),
                )
                    .into_response()
            }
            Err(_) => {
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(serde_json::json!({"error":"checkout_record_unavailable"})),
                )
                    .into_response()
            }
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

#[path = "server_mcp_and_discovery.rs"]
mod server_mcp_and_discovery;
pub use server_mcp_and_discovery::*;

#[path = "server_lifecycle_and_tests.rs"]
mod server_lifecycle_and_tests;
pub use server_lifecycle_and_tests::*;
