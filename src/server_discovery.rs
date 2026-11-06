//! Pressmark Server Discovery & Health Check Metadata
//!
//! Author: Daniel Hallman

use std::sync::Arc;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Json},
};
use crate::{
    mcp::McpHandler,
    models::DocumentType,
    pricing::get_pricing_catalog,
    server::AppState,
};

pub async fn handle_owners_json() -> impl IntoResponse {
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

pub async fn handle_mcp_owners_json() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "$schema": "https://verifymcp.io/schemas/owners.json",
            "owners": ["daniel@mooshworks.com", "daniel.hallman@gmail.com"]
        })),
    )
}

pub async fn handle_glama_json() -> impl IntoResponse {
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

pub async fn handle_oauth_protected_resource() -> impl IntoResponse {
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

pub async fn handle_oauth_authorization_server() -> impl IntoResponse {
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

pub async fn handle_x402_discovery() -> impl IntoResponse {
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

pub async fn handle_agent_card() -> impl IntoResponse {
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
                "stateTransitionHistory": true
            },
            "protocols": [
                {
                    "type": "mcp",
                    "version": "2026-07-28",
                    "transport": "streamable-http",
                    "endpoint": "https://pressmark.dev/mcp",
                    "toolsEndpoint": "https://pressmark.dev/api/v1/mcp/tools.json"
                },
                {
                    "type": "a2a",
                    "version": "1.0",
                    "endpoint": "https://pressmark.dev/a2a"
                }
            ],
            "skills": [
                {
                    "name": "Vector PDF Synthesis",
                    "description": "Generate publication-grade PDFs with sub-25ms compilation and 1-scan QR code mobile payments.",
                    "tags": ["pdf", "typst", "invoice", "receipt", "vector"]
                },
                {
                    "name": "Document Lifecycle Management",
                    "description": "Transition documents from estimate to invoice, record payments, recalculate balances, and generate credit notes.",
                    "tags": ["billing", "accounting", "payments", "lifecycle"]
                },
                {
                    "name": "Real Estate & Statutory Leases",
                    "description": "Typeset 29 leasing, property management, investor analysis, and deed archetypes with a 50-state + DC + 6 municipal 4-layer statutory lease engine.",
                    "tags": ["real-estate", "lease", "deed", "property-management", "statutory"]
                }
            ]
        })),
    )
}

pub async fn handle_security_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "Contact: mailto:daniel@mooshworks.com\nExpires: 2027-12-31T23:59:59.000Z\nPreferred-Languages: en\nCanonical: https://pressmark.dev/.well-known/security.txt\nPolicy: https://pressmark.dev/privacy\n",
    )
}

pub async fn handle_ads_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "# Pressmark ad inventory declaration\n# Pressmark does not sell programmatic advertising.\ncontact=daniel@mooshworks.com\n",
    )
}

pub async fn handle_sellers_json() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        Json(serde_json::json!({
            "sellerId": "mooshworks",
            "name": "Moosh Works LLC",
            "domain": "pressmark.dev",
            "seller_type": "PUBLISHER",
            "is_passthrough": false
        })),
    )
}

pub async fn handle_mcpprobe_opt_out() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        Json(serde_json::json!({
            "opt_out": false,
            "contact": "daniel@mooshworks.com",
            "policy": "https://pressmark.dev/privacy",
            "server": "https://pressmark.dev/mcp"
        })),
    )
}

pub async fn handle_humans_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "/* TEAM */\nFounder & Lead Architect: Daniel Hallman\nContact: daniel@mooshworks.com\nLocation: Chattanooga, Tennessee, USA\n\n/* SITE */\nName: Pressmark\nURL: https://pressmark.dev\nEngine: Typst & Rust (Axum, Tokio)\nStandards: Model Context Protocol (MCP), OpenAPI 3.1, A2A\n",
    )
}

pub async fn handle_tdmrep_json() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        Json(serde_json::json!({
            "tdm-reservation": 1,
            "policy": "https://pressmark.dev/terms"
        })),
    )
}

pub async fn handle_capabilities_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "name: Pressmark\ncapabilities:\n  - vector_pdf_synthesis\n  - 57_document_archetypes\n  - mcp_streamable_http\n  - png_preview_rendering\n  - temporary_pdf_downloads\nendpoints:\n  mcp: https://pressmark.dev/mcp\n  studio: https://pressmark.dev\n  openapi: https://pressmark.dev/openapi.json\nprivacy: https://pressmark.dev/privacy\n",
    )
}

pub async fn handle_did_json() -> impl IntoResponse {
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

pub async fn handle_brick_blue_json() -> impl IntoResponse {
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

pub async fn handle_discovery_resources() -> impl IntoResponse {
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

pub fn build_mcp_server_card_json(base: &str) -> serde_json::Value {
    serde_json::json!({
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
    })
}

pub async fn handle_mcp_server_card(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let base = state.base_url.trim_end_matches('/');
    (StatusCode::OK, Json(build_mcp_server_card_json(base)))
}

pub async fn handle_mcp_server_cards_json(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let base = state.base_url.trim_end_matches('/');
    (StatusCode::OK, Json(serde_json::json!([build_mcp_server_card_json(base)])))
}

pub async fn handle_toolvet_verify_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "server: Pressmark\nmaintainer: Daniel Hallman\nemail: daniel@mooshworks.com\nendpoint: https://pressmark.dev/mcp\nprotocol: mcp\nverified: true\npolicy: https://pressmark.dev/privacy\n",
    )
}

pub async fn handle_skill_md() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/markdown; charset=utf-8")],
        include_str!("../skills/pressmark/SKILL.md"),
    )
}

pub async fn handle_auth_md() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/markdown; charset=utf-8")],
        concat!(
            "# Pressmark Authentication & Authorization\n\n",
            "Author: Daniel Hallman\n\n",
            "Pressmark is an open-access vector PDF document synthesis engine and Model Context Protocol (MCP) server.\n\n",
            "## Access Tiers & Credentials\n\n",
            "1. **Guest Mode (Default & Zero-Credential)**:\n",
            "   - Synthesis (`/api/v1/generate`, `/api/v1/preview`, `/api/v1/render-vector`), template listing (`/api/v1/templates`), state lookup (`/api/v1/states`), and MCP tools (`/mcp` and `/api/v1/mcp`) require no API keys, accounts, or auth headers.\n",
            "   - Designed for zero-friction invocation by autonomous AI agents, LLM copilots, and web visitors.\n\n",
            "2. **Solo Pro & Subscriptions**:\n",
            "   - Web studio and automated workflows unlock persistent session storage and unlimited watermarked-free PDF rendering via Stripe Checkout passes (`https://pressmark.dev/pricing`).\n\n",
            "3. **OAuth 2.0 PKCE & Protected Resource Discovery**:\n",
            "   - OAuth authorization metadata: `https://pressmark.dev/.well-known/oauth-authorization-server/mcp`\n",
            "   - OAuth protected resource metadata: `https://pressmark.dev/.well-known/oauth-protected-resource/mcp`\n",
            "   - OpenID configuration: `https://pressmark.dev/.well-known/openid-configuration/mcp`\n\n",
            "For inquiries and support, contact `daniel@mooshworks.com`.\n"
        ),
    )
}

pub async fn handle_http_signatures_directory() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        Json(serde_json::json!({
            "version": "1.0",
            "issuer": "https://pressmark.dev",
            "directory": "https://pressmark.dev/.well-known/http-message-signatures-directory",
            "endpoints": [
                "https://pressmark.dev/mcp",
                "https://pressmark.dev/api/v1/mcp"
            ],
            "verification": {
                "status": "active",
                "mode": "open_read_write"
            }
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
                "tools_sha256": McpHandler::tools_sha256(),
                "archetypes_count": DocumentType::all().len(),
                "openai_challenge_configured": challenge_ok
            },
            "usage_telemetry": db.analytics_summary()
        }
    })
}

pub async fn handle_setup_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut report = run_setup_check(Some(&state.storage));
    let ready = match &state.feedback { Some(client) => client.healthy().await, None => false };
    report["checks"]["feedback"] = serde_json::json!({"status": if ready { "ok" } else { "unavailable" }, "url": "/feedback"});
    if !ready { report["status"] = serde_json::json!("degraded"); }
    (StatusCode::OK, Json(report))
}
