//! Pressmark MCP, Discovery, A2A, OAuth Metadata & Setup Check Handlers
//!
//! Author: Daniel Hallman

use super::*;

pub(crate) async fn handle_mcp_owners_json() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "$schema": "https://verifymcp.io/schemas/owners.json",
            "owners": ["daniel@mooshworks.com", "daniel.hallman@gmail.com"]
        })),
    )
}

pub(crate) async fn handle_glama_json() -> impl IntoResponse {
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

pub(crate) async fn handle_oauth_protected_resource() -> impl IntoResponse {
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

pub(crate) async fn handle_oauth_authorization_server() -> impl IntoResponse {
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

pub(crate) async fn handle_x402_discovery() -> impl IntoResponse {
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

pub(crate) async fn handle_agent_card() -> impl IntoResponse {
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

pub(crate) async fn handle_a2a_post(
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

pub(crate) async fn handle_mcp_delete() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

pub(crate) async fn handle_security_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "Contact: mailto:daniel@mooshworks.com\nExpires: 2027-12-31T23:59:59.000Z\nPreferred-Languages: en\nCanonical: https://pressmark.dev/.well-known/security.txt\nPolicy: https://pressmark.dev/privacy\n",
    )
}

pub(crate) async fn handle_ads_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "# Pressmark Vector Document Engine (https://pressmark.dev)\n# Contact: daniel@mooshworks.com\n",
    )
}

pub(crate) async fn handle_sellers_json() -> impl IntoResponse {
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

pub(crate) async fn handle_mcpprobe_opt_out() -> impl IntoResponse {
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

pub(crate) async fn handle_humans_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "/* TEAM */\nFounder & Lead Architect: Daniel Hallman\nContact: daniel@mooshworks.com\nLocation: Chattanooga, Tennessee, USA\n\n/* SITE */\nName: Pressmark\nURL: https://pressmark.dev\nEngine: Typst & Rust (Axum, Tokio)\nStandards: Model Context Protocol (MCP), OpenAPI 3.1, A2A\n",
    )
}

pub(crate) async fn handle_tdmrep_json() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        Json(serde_json::json!({
            "tdm-reservation": 1,
            "policy": "https://pressmark.dev/terms"
        })),
    )
}

pub(crate) async fn handle_capabilities_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "name: Pressmark\ncapabilities:\n  - vector_pdf_synthesis\n  - 57_document_archetypes\n  - mcp_streamable_http\n  - png_preview_rendering\n  - temporary_pdf_downloads\nendpoints:\n  mcp: https://pressmark.dev/mcp\n  studio: https://pressmark.dev\n  openapi: https://pressmark.dev/openapi.json\nprivacy: https://pressmark.dev/privacy\n",
    )
}

pub(crate) async fn handle_did_json() -> impl IntoResponse {
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

pub(crate) async fn handle_brick_blue_json() -> impl IntoResponse {
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

pub(crate) async fn handle_discovery_resources() -> impl IntoResponse {
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

pub(crate) async fn handle_mcp_server_card(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
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
    let smoke_res = crate::engine::compile_pdf_bytes(
        "#set page(width: 2in, height: 1in, margin: 4pt)\n*Pressmark Doctor OK*",
    );
    let smoke_ms = start.elapsed().as_millis() as u64;
    let smoke_ok = smoke_res
        .as_ref()
        .map(|b| b.starts_with(b"%PDF-"))
        .unwrap_or(false);

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
                static_files.insert(
                    path.to_string(),
                    serde_json::json!({ "present": true, "bytes": meta.len() }),
                );
            }
            _ => {
                all_static_present = false;
                static_files.insert(
                    path.to_string(),
                    serde_json::json!({ "present": false, "bytes": 0 }),
                );
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
    let overall_healthy =
        smoke_ok && all_static_present && secret_key_ok && !webhook_secrets.is_empty();

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

pub(crate) async fn handle_setup_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut report = run_setup_check(Some(&state.storage));
    let ready = match &state.feedback {
        Some(client) => client.healthy().await,
        None => false,
    };
    report["checks"]["feedback"] =
        serde_json::json!({"status": if ready { "ok" } else { "unavailable" }, "url": "/feedback"});
    if !ready {
        report["status"] = serde_json::json!("degraded");
    }
    (StatusCode::OK, Json(report))
}

pub(crate) async fn handle_analytics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.storage.analytics_summary()))
}

pub(crate) async fn handle_openapi(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(get_openapi_spec(&state.base_url))
}

pub(crate) async fn handle_ai_plugin(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(get_ai_plugin_manifest(&state.base_url))
}

pub(crate) async fn handle_openai_apps_challenge() -> impl IntoResponse {
    let token = std::env::var("OPENAI_APPS_CHALLENGE_TOKEN").unwrap_or_else(|_| {
        std::fs::read_to_string("static/.well-known/openai-apps-challenge")
            .unwrap_or_default()
            .trim()
            .to_string()
    });
    (StatusCode::OK, token)
}

pub(crate) async fn handle_mcp_get() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (
                header::HeaderName::from_static("mcp-protocol-version"),
                "2026-07-28",
            ),
            (
                header::HeaderName::from_static("x-mcp-protocol-version"),
                "2026-07-28",
            ),
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

pub(crate) async fn handle_mcp_tools() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "name": "pressmark",
            "version": crate::mcp::MCP_SERVER_VERSION,
            "tools": McpHandler::get_tools()
        })),
    )
}

pub(crate) async fn handle_mcp(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<JsonRpcRequest>,
) -> Response {
    let is_notif = McpHandler::is_notification(&req);
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok());
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
    let is_paid_session = session
        .as_ref()
        .map(|s| s.user.is_subscriber)
        .unwrap_or(false);
    let session_email = session.as_ref().map(|s| s.user.email.as_str());

    let attempt_id = if mcp_method == "tools/call" {
        state.storage.begin_mcp_tool_attempt(
            doc_type.as_deref(),
            user_agent.unwrap_or("mcp"),
            tool_name.as_deref(),
        )
    } else {
        None
    };

    let mut resp = if crate::feedback::is_tool(&req) {
        crate::feedback::handle_tool(&state, &headers, req).await
    } else if let Some(response) = read_download_resource(&state, &req) {
        response
    } else {
        McpHandler::handle_request_with_context(
            req,
            Some(&state.storage),
            is_paid_session,
            session_email,
        )
    };
    if resp.tool_succeeded()
        && matches!(
            tool_name.as_deref(),
            Some("generate_document" | "convert_document" | "record_payment")
        )
    {
        if let Some(result) = resp.result.as_mut() {
            attach_pdf_download(
                &state,
                result,
                doc_type.as_deref().unwrap_or("document"),
                &client_proto,
            );
        }
    }
    if resp.tool_succeeded() && tool_name.as_deref() == Some("preview_document") {
        if let Some(result) = resp.result.as_mut() {
            attach_preview_download(
                &state,
                result,
                doc_type.as_deref().unwrap_or("document"),
                &client_proto,
            );
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
            let error_code = resp
                .error
                .as_ref()
                .and_then(|error| error.get("code"))
                .and_then(|value| value.as_i64());
            state
                .storage
                .finish_mcp_tool_attempt(id, success, error_code, compile_ms);
        }
    } else {
        state.storage.record_usage_event(
            "mcp_protocol",
            doc_type.as_deref(),
            user_agent.unwrap_or("mcp"),
            tool_name.as_deref(),
            compile_ms,
        );
    }
    if is_2026
        && mcp_method != "initialize"
        && mcp_method != "ping"
        && !mcp_method.starts_with("notifications/")
    {
        if let Some(ref mut result) = resp.result {
            if let Some(obj) = result.as_object_mut() {
                obj.entry("resultType".to_string())
                    .or_insert_with(|| serde_json::json!("complete"));
                if mcp_method != "tools/call" {
                    obj.entry("ttlMs".to_string()).or_insert_with(|| {
                        serde_json::json!(if private_result { 0 } else { 300_000 })
                    });
                    obj.entry("cacheScope".to_string()).or_insert_with(|| {
                        serde_json::json!(if private_result { "private" } else { "public" })
                    });
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
        resp_headers.insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("private, no-store"),
        );
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

pub(crate) fn read_download_resource(
    state: &AppState,
    request: &JsonRpcRequest,
) -> Option<JsonRpcResponse> {
    if request.method != "resources/read" {
        return None;
    }
    let uri = request.params.as_ref()?.get("uri")?.as_str()?;
    // Catalogs are handled by the MCP dispatcher. Download resources are local cache lookups only.
    if uri.starts_with("pressmark://catalog/") {
        return None;
    }
    let file = reqwest::Url::parse(uri).ok().and_then(|url| {
        let base = reqwest::Url::parse(&state.base_url).ok()?;
        if url.origin() != base.origin()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return None;
        }
        let token = url.path().strip_prefix("/api/v1/downloads/")?;
        state.downloads.get(token)
    });
    Some(match file {
        Some(file) => JsonRpcResponse {
            jsonrpc: "2.0",
            id: request.id.clone(),
            error: None,
            result: Some(
                serde_json::json!({"contents": [{"uri": uri, "mimeType": file.mime_type, "blob": BASE64.encode(file.bytes)}]}),
            ),
        },
        None => JsonRpcResponse {
            jsonrpc: "2.0",
            id: request.id.clone(),
            result: None,
            error: Some(
                serde_json::json!({"code": -32002, "message": "Resource not found or expired. Generate a new document or preview."}),
            ),
        },
    })
}

pub(crate) fn attach_preview_download(
    state: &AppState,
    result: &mut serde_json::Value,
    doc_type: &str,
    _protocol: &str,
) {
    use crate::downloads::{DownloadError, MAX_DOWNLOAD_BYTES};
    let Some(encoded) = result["content"]
        .as_array()
        .and_then(|items| {
            items
                .iter()
                .find(|item| item["type"] == "image" && item["mimeType"] == "image/png")
        })
        .and_then(|image| image["data"].as_str())
    else {
        return;
    };
    let stored = if encoded.len() > MAX_DOWNLOAD_BYTES.div_ceil(3) * 4 {
        Err(DownloadError::TooLarge)
    } else {
        BASE64
            .decode(encoded)
            .map_err(|_| DownloadError::InvalidPng)
            .and_then(|bytes| {
                state
                    .downloads
                    .insert_png(bytes, doc_type, "preview", &state.base_url)
            })
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
        }
        Err(error) => {
            result["preview_error"] = serde_json::json!(error.message());
        }
    }
}

pub(crate) fn attach_pdf_download(
    state: &AppState,
    result: &mut serde_json::Value,
    fallback_type: &str,
    _protocol: &str,
) {
    use crate::downloads::{DownloadError, MAX_DOWNLOAD_BYTES};
    let Some(encoded) = result.get("pdf_base64").and_then(|v| v.as_str()) else {
        return;
    };
    let stored = if encoded.len() > MAX_DOWNLOAD_BYTES.div_ceil(3) * 4 {
        Err(DownloadError::TooLarge)
    } else {
        BASE64
            .decode(encoded)
            .map_err(|_| DownloadError::InvalidPdf)
            .and_then(|pdf| {
                let doc = &result["document"];
                let kind = doc
                    .get("doc_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or(fallback_type);
                let number = doc
                    .get("doc_number")
                    .or_else(|| result.get("doc_number"))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
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
                content.retain(|item| {
                    !(item["type"] == "resource"
                        && item["resource"]["mimeType"] == "application/pdf")
                });
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
