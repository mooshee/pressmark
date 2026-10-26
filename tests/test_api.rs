//! API, MCP & Plugin Integration Tests
//!
//! Author: Daniel Hallman

use pressmark::mcp::{JsonRpcRequest, McpHandler};
use pressmark::openapi::{get_ai_plugin_manifest, get_openapi_spec};
use pressmark::pricing::get_pricing_catalog;
use serde_json::json;

#[test]
fn test_openapi_spec_structure() {
    let spec = get_openapi_spec("http://localhost:3842");
    assert_eq!(spec["openapi"], "3.1.0");
    assert!(spec["paths"]["/api/v1/generate"].is_object());
    assert!(spec["paths"]["/api/v1/preview"].is_object());
    assert!(spec["paths"]["/api/v1/pricing"].is_object());
}

#[test]
fn test_chatgpt_ai_plugin_manifest() {
    let manifest = get_ai_plugin_manifest("http://localhost:3842");
    assert_eq!(manifest["schema_version"], "v1");
    assert_eq!(manifest["name_for_model"], "pressmark_doc_engine");
    assert!(manifest["description_for_model"].as_str().unwrap().contains("Pressmark Vector Engine"));
}

#[test]
fn test_pricing_catalog_has_all_tiers_and_microtransactions() {
    let catalog = get_pricing_catalog();
    assert_eq!(catalog.currency, "USD");
    assert_eq!(catalog.plans.len(), 4);
    assert_eq!(catalog.microtransactions.len(), 2);

    let solo = catalog.plans.iter().find(|p| p.id == "plan_solo").unwrap();
    assert_eq!(solo.price_cents, 1500);
    assert!(solo.popular);

    let single_pass = catalog.microtransactions.iter().find(|p| p.id == "pass_single_unlock").unwrap();
    assert_eq!(single_pass.price_cents, 299);
}

#[test]
fn test_mcp_tools_list_and_call() {
    let list_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "tools/list".to_string(),
        params: None,
    };
    let list_resp = McpHandler::handle_request(list_req);
    assert!(list_resp.result.is_some());
    let tools = list_resp.result.unwrap()["tools"].as_array().unwrap().clone();
    assert!(tools.iter().any(|t| t["name"] == "generate_document"));
    assert!(tools.iter().any(|t| t["name"] == "preview_document"));
    assert!(tools.iter().any(|t| t["name"] == "get_pricing"));
    assert!(tools.iter().any(|t| t["name"] == "calculate_totals"));

    // Test call calculate_totals
    let calc_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(2)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "calculate_totals",
            "arguments": {
                "document": {
                    "doc_type": "invoice",
                    "from": { "name": "Seller" },
                    "to": { "name": "Buyer" },
                    "items": [
                        { "description": "Widget A", "quantity": 2.0, "unit_price": 50.0 },
                        { "description": "Widget B", "quantity": 1.0, "unit_price": 100.0 }
                    ],
                    "tax_rate": 10.0,
                    "discount_amount": 20.0
                }
            }
        })),
    };
    let calc_resp = McpHandler::handle_request(calc_req);
    assert!(calc_resp.result.is_some());
    let calc_res = calc_resp.result.unwrap();
    assert_eq!(calc_res["subtotal"], 200.0);
    assert_eq!(calc_res["tax_total"], 20.0);
    assert_eq!(calc_res["discount_amount"], 20.0);
    assert_eq!(calc_res["grand_total"], 200.0);
    assert_eq!(calc_res["balance_due"], 200.0);
    assert_eq!(calc_res["is_paid"], false);

    // Test call get_pricing
    let call_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(3)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "get_pricing",
            "arguments": {}
        })),
    };
    let call_resp = McpHandler::handle_request(call_req);
    assert!(call_resp.result.is_some());
    let price_res = call_resp.result.unwrap();
    assert_eq!(price_res["currency"], "USD");
    assert!(price_res["content"].is_array());
}

#[test]
fn test_mcp_lifecycle_handshake_and_negative_validation() {
    // 1. initialize handshake
    let init_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "initialize".to_string(),
        params: Some(json!({
            "protocolVersion": "2026-07-28",
            "capabilities": {},
            "clientInfo": { "name": "test-client", "version": "1.0.0" }
        })),
    };
    let init_resp = McpHandler::handle_request(init_req);
    let init_res = init_resp.result.expect("initialize should return result");
    assert_eq!(init_res["protocolVersion"], "2026-07-28");
    assert_eq!(init_res["serverInfo"]["name"], "pressmark");
    assert_eq!(init_res["serverInfo"]["version"], "1.2.0");

    // 2. server/discover handshake
    let disc_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(2)),
        method: "server/discover".to_string(),
        params: Some(json!({})),
    };
    let disc_resp = McpHandler::handle_request(disc_req);
    let disc_res = disc_resp.result.expect("server/discover should return result");
    assert_eq!(disc_res["protocolVersion"], "2026-07-28");
    assert_eq!(disc_res["resultType"], "complete");
    assert_eq!(disc_res["cacheScope"], "public-anonymous");

    // 3. notifications/initialized & ping
    let notif_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: None,
        method: "notifications/initialized".to_string(),
        params: None,
    };
    assert!(McpHandler::is_notification(&notif_req));
    let notif_resp = McpHandler::handle_request(notif_req);
    assert!(notif_resp.error.is_none());

    // 4. Negative Test Case 1: Unsupported convert_document target_type
    let bad_convert = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(4)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "convert_document",
            "arguments": {
                "target_type": "mortgage_deed",
                "document": {
                    "doc_type": "estimate",
                    "from": { "name": "Seller" },
                    "to": { "name": "Buyer" }
                }
            }
        })),
    };
    let bad_convert_resp = McpHandler::handle_request(bad_convert);
    assert!(bad_convert_resp.error.is_some());
    assert_eq!(bad_convert_resp.error.unwrap()["code"], -32602);

    // 5. Negative Test Case 2: Missing counterparty name
    let missing_party = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(5)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "generate_document",
            "arguments": {
                "doc_type": "invoice",
                "from": { "name": "" },
                "to": { "name": "Buyer" },
                "items": [{ "description": "Widget", "quantity": 5.0, "unit_price": 10.0 }]
            }
        })),
    };
    let missing_party_resp = McpHandler::handle_request(missing_party);
    assert!(missing_party_resp.error.is_some());
    assert_eq!(missing_party_resp.error.unwrap()["code"], -32602);

    // 6. Negative Test Case 3: Negative quantity on non-credit-note
    let neg_qty = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(6)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "generate_document",
            "arguments": {
                "doc_type": "invoice",
                "from": { "name": "Seller" },
                "to": { "name": "Buyer" },
                "items": [{ "description": "Consulting", "quantity": -5.0, "unit_price": -100.0 }]
            }
        })),
    };
    let neg_qty_resp = McpHandler::handle_request(neg_qty);
    assert!(neg_qty_resp.error.is_some());
    assert_eq!(neg_qty_resp.error.unwrap()["code"], -32602);
}

#[test]
fn test_studio_ui_components_and_selectors() {
    let html = std::fs::read_to_string("static/index.html").expect("index.html should exist");
    assert!(html.contains("id=\"viewModeSelector\""));
    assert!(html.contains("id=\"dropdownTools\""));
    assert!(html.contains("id=\"dropdownMore\""));
    assert!(html.contains("id=\"dropdownExport\""));
    assert!(html.contains("class=\"sidebar-tab-strip\""));
    assert!(html.contains("id=\"tabContent-content\""));
    assert!(html.contains("id=\"tabContent-design\""));
    assert!(html.contains("id=\"tabContent-legal\""));
    assert!(html.contains("id=\"tabContent-ai\""));
    assert!(html.contains("id=\"archetypeSearchInput\""));
    assert!(html.contains("class=\"selection-floating-dock\""));
    assert!(html.contains("id=\"chatgptAuthContainer\""));
    assert!(html.contains("id=\"btnChatGptLogin\""));
    assert!(html.contains("id=\"chatgptUserDropdown\""));
    assert!(html.contains("id=\"chatgptComputeBanner\""));
    // Archetype primary header & multi-page preview
    assert!(html.contains("id=\"archetypePrimaryHeader\""));
    assert!(html.contains("id=\"currentArchetypeHeading\""));
    assert!(html.contains("id=\"btnOpenArchetypeModal\""));
    assert!(html.contains("id=\"archetypeCategoryChips\""));
    assert!(html.contains("id=\"pageNavControls\""));
    assert!(html.contains("id=\"btnPrevPage\""));
    assert!(html.contains("id=\"btnNextPage\""));
    assert!(html.contains("id=\"btnViewStack\""));
    assert!(html.contains("id=\"btnViewSingle\""));
    assert!(html.contains("id=\"previewPagesContainer\""));
    assert!(html.contains("id=\"fromPartyBadge\""));
    assert!(html.contains("id=\"toPartyBadge\""));
    // Data Mode & Party Directory Controls
    assert!(html.contains("id=\"dataModeControl\""));
    assert!(html.contains("id=\"btnModeMyData\""));
    assert!(html.contains("id=\"btnModeSampleData\""));
    assert!(html.contains("id=\"btnSaveIssuerProfile\""));
    assert!(html.contains("id=\"btnResetIssuerProfile\""));
    assert!(html.contains("id=\"counterpartyQuickSelect\""));
    assert!(html.contains("id=\"btnSaveCounterparty\""));
    assert!(html.contains("id=\"btnClearCounterparty\""));
    // Theme Palette & Custom Color System
    assert!(html.contains("id=\"themePresetSelect\""));
    assert!(html.contains("id=\"themePalettePreviewBar\""));
    assert!(html.contains("id=\"customPalettePanel\""));
    assert!(html.contains("id=\"customPrimaryColor\""));
    assert!(html.contains("id=\"secondaryColorInput\""));
    assert!(html.contains("id=\"accentColorInput\""));
    assert!(html.contains("id=\"surfaceColorInput\""));
    assert!(html.contains("class=\"theme-chip theme-chip-custom\""));

    let css = std::fs::read_to_string("static/css/studio.css").expect("studio.css should exist");
    assert!(css.contains("view-mode-split"));
    assert!(css.contains("view-mode-editor"));
    assert!(css.contains("view-mode-preview"));
    assert!(css.contains("selection-floating-dock"));
    assert!(css.contains("overflow: visible;"));
    assert!(css.contains(".btn-chatgpt-auth"));
    assert!(css.contains(".chatgpt-status-dot"));
    assert!(css.contains(".chatgpt-compute-banner"));
    // Archetype & multi-page CSS
    assert!(css.contains(".archetype-primary-header"));
    assert!(css.contains(".archetype-hero-card"));
    assert!(css.contains(".cat-chip"));
    assert!(css.contains(".party-from-subcard"));
    assert!(css.contains(".party-to-subcard"));
    assert!(css.contains(".page-nav-controls"));
    assert!(css.contains(".preview-pages-container"));
    assert!(css.contains(".preview-page-frame"));
    assert!(css.contains(".data-mode-control"));
    assert!(css.contains(".data-mode-btn"));
    assert!(css.contains(".party-subcard-header"));
    assert!(css.contains(".party-actions-group"));
    // Theme Palette CSS
    assert!(css.contains(".theme-palette-preview-bar"));
    assert!(css.contains(".palette-swatch-slot"));
    assert!(css.contains(".custom-palette-panel"));
    assert!(css.contains(".color-picker-swatch"));

    let js = std::fs::read_to_string("static/js/studio.js").expect("studio.js should exist");
    assert!(js.contains("bindDropdowns"));
    assert!(js.contains("bindViewModes"));
    assert!(js.contains("bindSidebarTabs"));
    assert!(js.contains("bindGlobalAccordionToggles"));
    assert!(js.contains("bindArchetypeSearch"));
    assert!(js.contains("bindFloatingDockActions"));
    assert!(js.contains("checkChatGptSession"));
    assert!(js.contains("bindChatGptAuth"));
    assert!(js.contains("renderLoggedInChatGptUI"));
    // Archetype adaptation & multi-page JS
    assert!(js.contains("adaptFormToArchetype"));
    assert!(js.contains("bindPrimaryArchetypeHeader"));
    assert!(js.contains("bindMultiPageControls"));
    assert!(js.contains("renderMultiPagePreview"));
    // Data persistence & non-destructive switching
    assert!(js.contains("bindDataModeControls"));
    assert!(js.contains("saveIssuerProfile"));
    assert!(js.contains("loadIssuerProfile"));
    assert!(js.contains("saveCounterparty"));
    assert!(js.contains("populateCounterpartyDropdown"));
    assert!(js.contains("autoSaveUserDocument"));
    // Multi-Color Palette Tokens & Custom Studio
    assert!(js.contains("THEME_PALETTES"));
    assert!(js.contains("applyThemePalette"));
    assert!(js.contains("syncPaletteControlsFromState"));
    assert!(js.contains("secondary_color"));
    assert!(js.contains("surface_color"));
}

#[test]
fn test_multi_color_theme_palettes_and_custom_tokens() {
    use pressmark::models::DocumentRequest;

    // Test default palette tokens for named themes
    let mut req = DocumentRequest::default();
    req.theme = Some("executive_navy".into());
    assert_eq!(req.effective_primary_color(), "#1e3a8a");
    assert_eq!(req.effective_secondary_color(), "#d4af37");
    assert_eq!(req.effective_accent_color(), "#c59b27");
    assert_eq!(req.effective_surface_color(), "#fdfbf7");

    // Test custom override tokens
    req.theme = Some("custom".into());
    req.primary_color = Some("#112233".into());
    req.secondary_color = Some("#445566".into());
    req.accent_color = Some("#778899".into());
    req.surface_color = Some("#aabbcc".into());

    assert_eq!(req.effective_primary_color(), "#112233");
    assert_eq!(req.effective_secondary_color(), "#445566");
    assert_eq!(req.effective_accent_color(), "#778899");
    assert_eq!(req.effective_surface_color(), "#aabbcc");

    // Test compilation with custom palette
    let result = pressmark::engine::generate_pdf(&req);
    match result {
        Ok((pdf_bytes, _duration)) => {
            assert!(pdf_bytes.starts_with(b"%PDF-"));
        }
        Err(e) => {
            panic!("PDF generation with custom multi-color palette failed: {}", e);
        }
    }
}

#[test]
fn test_chatgpt_oauth_pkce_generation_and_auth_url() {
    use pressmark::auth::{ChatGptAuthManager, OPENAI_AUTHORIZE_ENDPOINT, CHATGPT_AUTH_SCOPES};

    let verifier = ChatGptAuthManager::generate_pkce_verifier();
    assert!(verifier.len() >= 43, "PKCE verifier must be at least 43 chars");
    assert!(verifier.len() <= 128, "PKCE verifier must be at most 128 chars");

    let challenge = ChatGptAuthManager::generate_pkce_challenge(&verifier);
    assert!(!challenge.is_empty(), "Challenge should not be empty");
    assert!(!challenge.contains('+'), "Base64URL must not contain +");
    assert!(!challenge.contains('/'), "Base64URL must not contain /");
    assert!(!challenge.contains('='), "Base64URL must be unpadded");

    let auth_mgr = ChatGptAuthManager::new();
    let (auth_url, state) = auth_mgr.start_auth(
        "pressmark-client-id",
        "https://pressmark.dev/api/v1/auth/chatgpt/callback",
        Some("/studio".to_string()),
    );

    assert!(auth_url.starts_with(OPENAI_AUTHORIZE_ENDPOINT));
    assert!(auth_url.contains("client_id=pressmark-client-id"));
    assert!(auth_url.contains("response_type=code"));
    assert!(auth_url.contains("code_challenge_method=S256"));
    assert!(auth_url.contains(&format!("state={}", state)));
    assert!(auth_url.contains("model.request"));
    assert_eq!(CHATGPT_AUTH_SCOPES, "openid profile email model.request");
}

#[test]
fn test_chatgpt_subscription_tiers_and_entitlements() {
    use pressmark::auth::ChatGptPlan;

    let plus = ChatGptPlan::Plus;
    assert!(plus.is_subscriber());
    assert!(plus.watermark_free());
    assert!(plus.priority_compilation());
    assert_eq!(plus.display_name(), "ChatGPT Plus");
    assert_eq!(plus.compute_allowance(), "unlimited_delegated_compute");

    let pro = ChatGptPlan::Pro;
    assert!(pro.is_subscriber());
    assert!(pro.watermark_free());
    assert!(pro.priority_compilation());
    assert_eq!(pro.display_name(), "ChatGPT Pro");

    let team = ChatGptPlan::Team;
    assert!(team.is_subscriber());
    assert!(team.watermark_free());

    let enterprise = ChatGptPlan::Enterprise;
    assert!(enterprise.is_subscriber());

    let free = ChatGptPlan::Free;
    assert!(!free.is_subscriber());
    assert!(!free.watermark_free());
    assert!(!free.priority_compilation());
}

#[test]
fn test_chatgpt_session_store_and_token_extraction() {
    use pressmark::auth::{extract_token_from_headers, ChatGptAuthManager, ChatGptPlan};
    use axum::http::{header, HeaderMap};

    let auth_mgr = ChatGptAuthManager::new();
    let session = auth_mgr.create_demo_session(
        ChatGptPlan::Plus,
        Some("daniel@mooshworks.com".to_string()),
        Some("Daniel Hallman".to_string()),
    );

    assert!(session.session_id.starts_with("cpt_sess_"));
    assert_eq!(session.user.plan, ChatGptPlan::Plus);
    assert!(session.delegated_compute_active);

    // Lookup by session_id
    let found = auth_mgr.get_session(&session.session_id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().user.email, "daniel@mooshworks.com");

    // Lookup by access_token
    let found_by_token = auth_mgr.get_session(&session.access_token);
    assert!(found_by_token.is_some());

    // Test token extraction from Authorization header
    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, format!("Bearer {}", session.session_id).parse().unwrap());
    let extracted = extract_token_from_headers(&headers);
    assert_eq!(extracted.as_deref(), Some(session.session_id.as_str()));

    // Test token extraction from Cookie header
    let mut cookie_headers = HeaderMap::new();
    cookie_headers.insert(header::COOKIE, format!("other_cookie=123; chatgpt_session={}; test=abc", session.session_id).parse().unwrap());
    let extracted_cookie = extract_token_from_headers(&cookie_headers);
    assert_eq!(extracted_cookie.as_deref(), Some(session.session_id.as_str()));

    // Test session removal (Logout)
    let removed = auth_mgr.remove_session(&session.session_id);
    assert!(removed);
    assert!(auth_mgr.get_session(&session.session_id).is_none());
}

#[test]
fn test_openapi_and_plugin_manifest_chatgpt_oauth_spec() {
    let manifest = get_ai_plugin_manifest("https://pressmark.dev");
    assert_eq!(manifest["auth"]["type"], "oauth");
    assert_eq!(manifest["auth"]["client_url"], "https://auth.openai.com/api/accounts/authorize");
    assert_eq!(manifest["auth"]["authorization_url"], "https://auth.openai.com/api/accounts/oauth/token");
    assert_eq!(manifest["auth"]["scope"], "openid profile email model.request");

    let spec = get_openapi_spec("https://pressmark.dev");
    assert!(spec["paths"]["/api/v1/auth/chatgpt/session"].is_object());
    assert!(spec["paths"]["/api/v1/copilot"].is_object());
    assert!(spec["components"]["securitySchemes"]["chatgpt_oauth"].is_object());
    assert!(spec["components"]["securitySchemes"]["bearerAuth"].is_object());
    assert_eq!(spec["security"][0]["chatgpt_oauth"][3], "model.request");
}

#[test]
fn test_component_system_and_motion_choreography() {
    let html = std::fs::read_to_string("static/index.html").expect("index.html should exist");
    let css = std::fs::read_to_string("static/css/studio.css").expect("studio.css should exist");
    let js = std::fs::read_to_string("static/js/studio.js").expect("studio.js should exist");

    // 1. Floating Dock Variant Switcher HTML
    assert!(html.contains("id=\"dockVariantWrap\""));
    assert!(html.contains("id=\"dockVariantSelect\""));
    assert!(html.contains("id=\"selectionActionBar\""));

    // 2. Component System Specification in JS
    assert!(js.contains("const COMPONENT_SYSTEM = {"));
    assert!(js.contains("header: {"));
    assert!(js.contains("variants: ["));
    assert!(js.contains("standard"));
    assert!(js.contains("editorial_centered"));
    assert!(js.contains("split_sidebar"));
    assert!(js.contains("swiss_grid"));

    // 3. Tactile Add (Emerald) & Delete (Crimson) Motion Tokens in CSS
    assert!(css.contains(".anim-row-add-emerald"));
    assert!(css.contains("@keyframes emeraldAddReveal"));
    assert!(css.contains(".anim-card-add-emerald"));
    assert!(css.contains("@keyframes emeraldCardReveal"));
    assert!(css.contains(".anim-row-delete-crimson"));
    assert!(css.contains("@keyframes crimsonDeleteRow"));
    assert!(css.contains(".anim-card-delete-crimson"));
    assert!(css.contains("@keyframes crimsonDeleteCard"));
    assert!(css.contains(".hotspot-add-emerald"));
    assert!(css.contains("@keyframes hotspotRadarEmerald"));
    assert!(css.contains(".hotspot-delete-crimson"));
    assert!(css.contains("@keyframes hotspotGhostCrimson"));

    // 4. Double-Buffered Preview Layer Architecture
    assert!(css.contains(".preview-stage-buffer"));
    assert!(css.contains(".preview-layer-current"));
    assert!(css.contains(".preview-layer-next"));
    assert!(css.contains(".preview-layer-fade-out"));
    assert!(css.contains(".preview-layer-fade-in"));
    assert!(js.contains("function updatePreviewFrame"));
    assert!(js.contains("preview-layer-fade-in"));

    // 5. Deletion Choreography in JS (320ms deferred splice)
    assert!(js.contains("anim-row-delete-crimson"));
    assert!(js.contains("anim-card-delete-crimson"));
    assert!(js.contains("hotspot-delete-crimson"));
}

#[test]
fn test_invoicing_lifecycle_and_stripe_qr_elements() {
    use pressmark::models::{DocumentRequest, DocumentType};
    use pressmark::engine::{generate_qr_code_svg, generate_pdf};

    // 1. Vector QR code generation test
    let qr_svg = generate_qr_code_svg("https://buy.stripe.com/test_checkout", "#4f46e5");
    assert!(qr_svg.is_ok(), "Vector QR SVG should generate successfully");
    let svg_content = qr_svg.unwrap();
    assert!(svg_content.contains("<svg"), "QR output must be valid SVG");
    assert!(svg_content.contains("#4f46e5"), "QR output must respect brand primary color");

    // 2. Lifecycle transitions (Estimate -> Invoice -> Receipt)
    let mut quote = DocumentRequest::default();
    quote.doc_type = DocumentType::Estimate;
    quote.doc_number = Some("EST-2026-0099".into());
    quote.title = Some("ESTIMATE & SCOPE".into());

    let invoice = quote.to_invoice(14);
    assert_eq!(invoice.doc_type, DocumentType::Invoice);
    assert_eq!(invoice.title.as_deref(), Some("TAX INVOICE"));
    assert_eq!(invoice.doc_number.as_deref(), Some("INV-2026-0099"));
    assert_eq!(invoice.effective_payment_status(), "pending");

    let receipt = invoice.to_receipt(
        "Stripe / Apple Pay",
        "ch_3Mwj4x2eZvKYlo2C1g9J",
        "2026-10-02"
    );
    assert_eq!(receipt.doc_type, DocumentType::Receipt);
    assert_eq!(receipt.title.as_deref(), Some("OFFICIAL PAYMENT RECEIPT"));
    assert_eq!(receipt.doc_number.as_deref(), Some("REC-2026-0099"));
    assert_eq!(receipt.effective_payment_status(), "paid");
    assert!(receipt.is_paid_state());

    // 3. Vector compilation of Invoice with QR code & Receipt with watermark
    let mut inv_req = invoice.clone();
    inv_req.payment_qr_code = Some(true);
    inv_req.payment_link = Some("https://buy.stripe.com/test_checkout".into());
    let inv_pdf = generate_pdf(&inv_req);
    if let Err(ref e) = inv_pdf {
        panic!("Invoice compilation failed: {}", e);
    }
    assert!(inv_pdf.unwrap().0.starts_with(b"%PDF-"));

    let rec_pdf = generate_pdf(&receipt);
    assert!(rec_pdf.is_ok(), "Receipt with paid watermark stamp should compile cleanly");
    assert!(rec_pdf.unwrap().0.starts_with(b"%PDF-"));

    // 4. Studio Frontend Verification
    let html = std::fs::read_to_string("static/index.html").expect("index.html must exist");
    let css = std::fs::read_to_string("static/css/studio.css").expect("studio.css must exist");
    let js = std::fs::read_to_string("static/js/studio.js").expect("studio.js must exist");

    // Stepper & Payment Card in HTML
    assert!(html.contains("id=\"lifecycleStepperBar\""));
    assert!(html.contains("id=\"stepBtnEstimate\""));
    assert!(html.contains("id=\"stepBtnInvoice\""));
    assert!(html.contains("id=\"stepBtnReceipt\""));
    assert!(html.contains("id=\"btnLifecycleConvertAction\""));
    assert!(html.contains("id=\"cardPaymentCheckout\""));
    assert!(html.contains("id=\"paymentLinkInput\""));
    assert!(html.contains("id=\"paymentStatusSelect\""));
    assert!(html.contains("id=\"togglePaymentQr\""));
    assert!(html.contains("id=\"btnQuickRecordPayment\""));

    // CSS Styling
    assert!(css.contains(".lifecycle-stepper-bar"));
    assert!(css.contains(".lifecycle-step-btn"));
    assert!(css.contains(".btn-lifecycle-action"));
    assert!(css.contains(".btn-emerald-action"));

    // JS Controller Bindings
    assert!(js.contains("bindLifecycleStepper"));
    assert!(js.contains("updateLifecycleUI"));
    assert!(js.contains("executeLifecycleTransition"));
    assert!(js.contains("payment_link:"));
}

#[test]
fn test_deliverables_pro_review_fixes_and_lifecycle_enhancements() {
    use pressmark::engine::generate_qr_code_svg;
    use pressmark::models::{DocumentRequest, DocumentType, LineItem, PartyDetails};

    // 1. Deliverable 2: High EC Vector QR Code Engine
    let svg = generate_qr_code_svg("https://buy.stripe.com/live_pressmark_test_ec_h", "#1e293b").expect("QR should generate");
    assert!(svg.contains("<svg"));
    assert!(svg.contains("viewBox="));
    assert!(svg.contains("#1e293b"));

    // 2. Deliverable 3: Document Lifecycle - Credit Note & Base ID Preservation
    let mut inv = DocumentRequest {
        doc_type: DocumentType::Invoice,
        doc_number: Some("INV-2026-0819".into()),
        from: PartyDetails { name: "Apex Design Co.".into(), ..Default::default() },
        to: PartyDetails { name: "Acme Corp".into(), ..Default::default() },
        items: vec![
            LineItem { description: "Design Sprint".into(), quantity: 1.0, unit_price: 3500.0, ..Default::default() },
            LineItem { description: "Cloud Hosting".into(), quantity: 2.0, unit_price: 250.0, ..Default::default() },
        ],
        tax_rate: Some(10.0),
        discount_amount: Some(500.0),
        ..Default::default()
    };

    assert_eq!(inv.subtotal(), 4000.0);
    assert_eq!(inv.tax_total(), 400.0);
    assert_eq!(inv.grand_total(), 3900.0);
    assert_eq!(inv.balance_due(), 3900.0);
    assert!(!inv.is_paid_state());

    // Partial payment
    inv.amount_paid = Some(1500.0);
    assert_eq!(inv.balance_due(), 2400.0);
    assert!(!inv.is_paid_state());

    // Convert to Credit Note
    let credit = inv.to_credit_note(Some("Project scope reduced by client agreement"));
    assert_eq!(credit.doc_type, DocumentType::CreditNote);
    assert_eq!(credit.title.as_deref(), Some("CREDIT NOTE"));
    assert_eq!(credit.doc_number.as_deref(), Some("CRD-2026-0819"));
    assert_eq!(credit.effective_payment_status(), "refunded");
    assert!(credit.notes.as_deref().unwrap().contains("Reason: Project scope reduced"));

    // Test vector PDF compilation of credit note
    let pdf_res = pressmark::engine::generate_pdf(&credit);
    assert!(pdf_res.is_ok(), "Credit note should compile cleanly to vector PDF");
    assert!(pdf_res.unwrap().0.starts_with(b"%PDF-"));

    // 3. Deliverable 4: Conversational MCP Tools with Rich Markdown Output
    let convert_mcp_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(101)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "convert_document",
            "arguments": {
                "document": inv,
                "target_type": "credit_note",
                "credit_reason": "Billing adjustment",
                "payment_link": "https://buy.stripe.com/refund_claim"
            }
        })),
    };
    let convert_mcp_resp = McpHandler::handle_request(convert_mcp_req);
    assert!(convert_mcp_resp.result.is_some());
    let res_obj = convert_mcp_resp.result.unwrap();
    let text_content = res_obj["content"][0]["text"].as_str().unwrap();
    assert!(text_content.contains("CREDIT NOTE"));
    assert!(text_content.contains("CRD-2026-0819"));
    assert!(text_content.contains("| Description | Qty | Rate | Amount |"));
    assert!(text_content.contains("Click Here to Pay Online"));
    assert!(res_obj["pdf_base64"].is_string());

    // Test MCP record_payment with amount_paid
    let pay_mcp_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(102)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "record_payment",
            "arguments": {
                "document": inv,
                "payment_method": "Apple Pay",
                "transaction_reference": "TX_APPLE_98214",
                "amount_paid": 3900.0
            }
        })),
    };
    let pay_mcp_resp = McpHandler::handle_request(pay_mcp_req);
    assert!(pay_mcp_resp.result.is_some());
    let pay_obj = pay_mcp_resp.result.unwrap();
    let pay_text = pay_obj["content"][0]["text"].as_str().unwrap();
    assert!(pay_text.contains("PAID IN FULL"));
    assert!(pay_text.contains("REC-2026-0819"));

    // 4. Deliverable 5: Studio UI Accessibility & Components
    let html = std::fs::read_to_string("static/index.html").expect("index.html must exist");
    let css = std::fs::read_to_string("static/css/studio.css").expect("studio.css must exist");
    let js = std::fs::read_to_string("static/js/studio.js").expect("studio.js must exist");

    assert!(html.contains("id=\"stepBtnCredit\""));
    assert!(html.contains("id=\"btnCopyPaymentLink\""));
    assert!(html.contains("id=\"amountPaidRow\""));
    assert!(html.contains("id=\"balanceDueRow\""));
    assert!(html.contains("id=\"amountPaidInput\""));
    assert!(html.contains("role=\"tablist\""));
    assert!(html.contains("role=\"tab\""));

    assert!(css.contains(".btn-copy-input"));
    assert!(css.contains(".badge-credit"));
    assert!(css.contains("@media (max-width: 640px)"));

    assert!(js.contains("btnCopyPaymentLink"));
    assert!(js.contains("stepBtnCredit"));
    assert!(js.contains("amountPaidRow"));
    assert!(js.contains("balanceDueRow"));
}

#[tokio::test]
async fn test_p0_blocker_1_security_guards_and_rate_limiting() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use pressmark::rate_limit::RateLimiter;
    use pressmark::server::{create_router, AppState};
    use std::sync::Arc;
    use tower::ServiceExt;

    // Test with limit: 2 requests per 60 seconds
    let rate_limiter = RateLimiter::new(2, 60);
    let app_state = Arc::new(AppState::default().with_rate_limiter(rate_limiter));
    let router = create_router(app_state);

    // Request 1: Health check should be bypassed from rate limiting
    for _ in 0..5 {
        let req = Request::builder()
            .uri("/health")
            .method("GET")
            .body(Body::empty())
            .unwrap();
        let res = router.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    // Request 1 on regular endpoint
    let req1 = Request::builder()
        .uri("/api/v1/pricing")
        .method("GET")
        .header("cf-connecting-ip", "203.0.113.195")
        .body(Body::empty())
        .unwrap();
    let res1 = router.clone().oneshot(req1).await.unwrap();
    assert_eq!(res1.status(), StatusCode::OK);
    assert_eq!(res1.headers().get("x-ratelimit-remaining").unwrap(), "1");

    // Request 2 on regular endpoint
    let req2 = Request::builder()
        .uri("/api/v1/pricing")
        .method("GET")
        .header("cf-connecting-ip", "203.0.113.195")
        .body(Body::empty())
        .unwrap();
    let res2 = router.clone().oneshot(req2).await.unwrap();
    assert_eq!(res2.status(), StatusCode::OK);
    assert_eq!(res2.headers().get("x-ratelimit-remaining").unwrap(), "0");

    // Request 3 should be 429 Too Many Requests
    let req3 = Request::builder()
        .uri("/api/v1/pricing")
        .method("GET")
        .header("cf-connecting-ip", "203.0.113.195")
        .body(Body::empty())
        .unwrap();
    let res3 = router.clone().oneshot(req3).await.unwrap();
    assert_eq!(res3.status(), StatusCode::TOO_MANY_REQUESTS);
    assert!(res3.headers().contains_key("retry-after"));

    // Different IP should still succeed
    let req4 = Request::builder()
        .uri("/api/v1/pricing")
        .method("GET")
        .header("x-forwarded-for", "198.51.100.42, 10.0.0.1")
        .body(Body::empty())
        .unwrap();
    let res4 = router.clone().oneshot(req4).await.unwrap();
    assert_eq!(res4.status(), StatusCode::OK);

    // Verify /health and /healthz endpoints both succeed and bypass rate limit
    let req_hz = Request::builder()
        .uri("/healthz")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_hz = router.clone().oneshot(req_hz).await.unwrap();
    assert_eq!(res_hz.status(), StatusCode::OK);

    // Test Body Limit: 5MB default limit (Sending 6MB payload to /api/v1/render-vector)
    let huge_body = vec![b'a'; 6 * 1024 * 1024]; // 6MB exceeds 5MB limit
    let req_huge = Request::builder()
        .uri("/api/v1/render-vector")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(huge_body))
        .unwrap();
    let res_huge = router.clone().oneshot(req_huge).await.unwrap();
    assert_eq!(res_huge.status(), StatusCode::PAYLOAD_TOO_LARGE);

    // Test Cache Pruning
    let cache_dir = std::path::Path::new("/tmp/pressmark_test_prune");
    let _ = std::fs::create_dir_all(cache_dir);
    let test_file = cache_dir.join("test_prune_old_file.tmp");
    std::fs::write(&test_file, b"test").unwrap();
    let pruned = pressmark::engine::clean_cache_directory_at(cache_dir, 0);
    assert!(pruned >= 1);
    assert!(!test_file.exists());
    let _ = std::fs::remove_dir_all(cache_dir);

    // Test LFI Path Traversal Protection
    let lfi_attempt = "/etc/passwd";
    let staged = pressmark::engine::stage_image_asset("logo", lfi_attempt);
    assert_ne!(staged, Some("/etc/passwd".to_string()));
}

#[tokio::test]
async fn test_p0_blocker_2_stripe_webhook_and_fulfillment() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use hmac::{Hmac, Mac};
    use pressmark::server::{create_router, AppState};
    use pressmark::stripe::{process_stripe_event, verify_stripe_signature, FulfillmentAction, StripeEvent};
    use sha2::Sha256;
    use std::sync::Arc;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tower::ServiceExt;

    let secret = "whsec_test_secret_for_pressmark_98214";
    std::env::set_var("STRIPE_WEBHOOK_SECRET", secret);

    // 1. Test Signature Verification helper
    let payload = br#"{"id":"evt_123","type":"payment_intent.succeeded","data":{"object":{"amount":1500,"currency":"usd","receipt_email":"daniel@example.com","metadata":{"document_number":"INV-2026-9021","tier":"solo"}}}}"#;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    let signed_payload = format!("{}.{}", now, std::str::from_utf8(payload).unwrap());
    mac.update(signed_payload.as_bytes());
    let sig_hex = hex::encode(mac.finalize().into_bytes());
    let sig_header = format!("t={},v1={}", now, sig_hex);

    // Valid verification
    let verified_t = verify_stripe_signature(payload, &sig_header, secret, 300).expect("Signature should verify");
    assert_eq!(verified_t, now);

    // Tampered payload fails
    let tampered_payload = br#"{"id":"evt_123","type":"payment_intent.succeeded","data":{"object":{"amount":999999}}}"#;
    assert!(verify_stripe_signature(tampered_payload, &sig_header, secret, 300).is_err());

    // Expired timestamp fails
    let expired_header = format!("t={},v1={}", now - 400, sig_hex);
    assert!(verify_stripe_signature(payload, &expired_header, secret, 300).is_err());

    // 2. Test Fulfillment Logic: payment_intent.succeeded
    let event: StripeEvent = serde_json::from_slice(payload).unwrap();
    let action = process_stripe_event(event).unwrap();
    match action {
        FulfillmentAction::PaymentUnlocked { customer_email, amount_cents, currency, document_number, receipt_number, .. } => {
            assert_eq!(customer_email, "daniel@example.com");
            assert_eq!(amount_cents, 1500);
            assert_eq!(currency, "USD");
            assert_eq!(document_number, Some("INV-2026-9021".to_string()));
            assert_eq!(receipt_number, "REC-2026-9021");
        }
        _ => panic!("Expected PaymentUnlocked action"),
    }

    // 3. Test Fulfillment Logic: charge.refunded generates Credit Note
    let refund_json = br#"{"id":"ch_123","type":"charge.refunded","data":{"object":{"amount_refunded":1500,"billing_details":{"email":"daniel@example.com"},"metadata":{"document_number":"INV-2026-9021"},"refunds":{"data":[{"reason":"requested_by_customer"}]}}}}"#;
    let refund_event: StripeEvent = serde_json::from_slice(refund_json).unwrap();
    let refund_action = process_stripe_event(refund_event).unwrap();
    match refund_action {
        FulfillmentAction::ChargeRefunded { customer_email, refund_amount_cents, credit_note_number, original_document_number, .. } => {
            assert_eq!(customer_email, "daniel@example.com");
            assert_eq!(refund_amount_cents, 1500);
            assert_eq!(credit_note_number, "CRD-2026-9021");
            assert_eq!(original_document_number, Some("INV-2026-9021".to_string()));
        }
        _ => panic!("Expected ChargeRefunded action"),
    }

    // 4. Test Webhook HTTP Endpoint via Axum Router
    let app_state = Arc::new(AppState::default());
    let router = create_router(app_state);

    // Request missing signature
    let req_no_sig = Request::builder()
        .uri("/api/v1/webhooks/stripe")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_vec()))
        .unwrap();
    let res_no_sig = router.clone().oneshot(req_no_sig).await.unwrap();
    assert_eq!(res_no_sig.status(), StatusCode::BAD_REQUEST);

    // Request with valid signature
    let req_valid = Request::builder()
        .uri("/api/v1/webhooks/stripe")
        .method("POST")
        .header("content-type", "application/json")
        .header("stripe-signature", sig_header.clone())
        .body(Body::from(payload.to_vec()))
        .unwrap();
    let res_valid = router.clone().oneshot(req_valid).await.unwrap();
    assert_eq!(res_valid.status(), StatusCode::OK);

    // Duplicate webhook delivery should be safely recognized as already_processed (idempotent)
    let req_duplicate = Request::builder()
        .uri("/api/v1/webhooks/stripe")
        .method("POST")
        .header("content-type", "application/json")
        .header("stripe-signature", sig_header)
        .body(Body::from(payload.to_vec()))
        .unwrap();
    let res_duplicate = router.clone().oneshot(req_duplicate).await.unwrap();
    assert_eq!(res_duplicate.status(), StatusCode::OK);
    let dup_bytes = axum::body::to_bytes(res_duplicate.into_body(), 1024 * 1024).await.unwrap();
    let dup_json: serde_json::Value = serde_json::from_slice(&dup_bytes).unwrap();
    assert_eq!(dup_json["status"], "already_processed");

    // 4b. Test Multi-Secret Verification: Sandbox webhook arrives while STRIPE_LIVE_MODE=true
    std::env::set_var("STRIPE_LIVE_MODE", "true");
    std::env::set_var("STRIPE_WEBHOOK_SECRET_LIVE", "whsec_live_production_vault_secret");
    std::env::set_var("STRIPE_WEBHOOK_SECRET_TEST", "whsec_test_sandbox_vault_secret");

    let unique_event_id = format!("evt_sandbox_{}_{}", now, std::process::id());
    let test_payload_str = format!(
        r#"{{"id":"{}","type":"payment_intent.succeeded","data":{{"object":{{"amount":2000,"currency":"usd","receipt_email":"test@example.com","metadata":{{"document_number":"INV-2026-TEST"}}}}}}}}"#,
        unique_event_id
    );
    let test_payload = test_payload_str.as_bytes();
    let mut mac_sandbox = HmacSha256::new_from_slice(b"whsec_test_sandbox_vault_secret").unwrap();
    let signed_sandbox = format!("{}.{}", now, test_payload_str);
    mac_sandbox.update(signed_sandbox.as_bytes());
    let sig_sandbox_hex = hex::encode(mac_sandbox.finalize().into_bytes());
    let sig_sandbox_header = format!("t={},v1={}", now, sig_sandbox_hex);

    let req_sandbox = Request::builder()
        .uri("/api/v1/webhooks/stripe")
        .method("POST")
        .header("content-type", "application/json")
        .header("stripe-signature", sig_sandbox_header)
        .body(Body::from(test_payload.to_vec()))
        .unwrap();
    let res_sandbox = router.clone().oneshot(req_sandbox).await.unwrap();
    assert_eq!(res_sandbox.status(), StatusCode::OK);
    let sandbox_bytes = axum::body::to_bytes(res_sandbox.into_body(), 1024 * 1024).await.unwrap();
    let sandbox_json: serde_json::Value = serde_json::from_slice(&sandbox_bytes).unwrap();
    assert_eq!(sandbox_json["status"], "success");

    // 5. Test Live Stripe Pricing Env Overrides
    std::env::set_var("STRIPE_SOLO_PRICE_ID", "price_live_solo_production_override");
    let catalog = pressmark::pricing::get_pricing_catalog();
    let solo_plan = catalog.plans.iter().find(|p| p.id == "plan_solo").unwrap();
    assert_eq!(solo_plan.stripe_price_id, "price_live_solo_production_override");

    // 6. Test Direct Stripe Payment Link Generation with Plan Overrides
    std::env::set_var("STRIPE_SOLO_CHECKOUT_URL", "https://buy.stripe.com/live_pressmark_solo");
    let url = pressmark::pricing::generate_checkout_url("price_solo_123", Some("usr_danny99"));
    assert_eq!(url, "https://buy.stripe.com/live_pressmark_solo?client_reference_id=usr_danny99");
}

#[tokio::test]
async fn test_p0_blocker_3_persistent_session_and_auth_storage() {
    use pressmark::auth::{ChatGptAuthManager, ChatGptPlan};
    use pressmark::storage::Storage;
    use pressmark::stripe::StripeIdempotencyStore;
    use std::sync::Arc;

    let db_path = "/tmp/pressmark_test_session_storage.db";
    let _ = std::fs::remove_file(db_path);

    let storage1 = Arc::new(Storage::new(db_path).expect("Failed to initialize SQLite storage"));
    let auth_mgr1 = ChatGptAuthManager::new().with_storage(storage1.clone());

    // 1. Create a session on instance 1
    let session = auth_mgr1.create_demo_session(
        ChatGptPlan::Plus,
        Some("daniel@mooshworks.com".to_string()),
        Some("Daniel Hallman".to_string()),
    );
    let session_id = session.session_id.clone();
    let access_token = session.access_token.clone();

    // Verify session retrieval on instance 1
    let retrieved1 = auth_mgr1.get_session(&session_id).expect("Session must exist");
    assert_eq!(retrieved1.user.name, "Daniel Hallman");
    assert_eq!(retrieved1.user.email, "daniel@mooshworks.com");
    assert!(retrieved1.delegated_compute_active);

    // 2. Simulate server crash / restart: drop instance 1 completely
    drop(auth_mgr1);
    drop(storage1);

    // 3. Instance 2 starts up, reconnects to the SQLite database file
    let storage2 = Arc::new(Storage::new(db_path).expect("Failed to reopen SQLite storage"));
    let auth_mgr2 = ChatGptAuthManager::new().with_storage(storage2.clone());

    // Verify session was persisted across restarts and can be fetched by session_id or access_token
    let retrieved2 = auth_mgr2.get_session(&session_id).expect("Session must survive restart");
    assert_eq!(retrieved2.user.name, "Daniel Hallman");
    assert_eq!(retrieved2.user.email, "daniel@mooshworks.com");
    assert_eq!(retrieved2.user.plan, ChatGptPlan::Plus);

    let retrieved_by_token = auth_mgr2.get_session(&access_token).expect("Token lookup must survive restart");
    assert_eq!(retrieved_by_token.session_id, session_id);

    // 4. Test PKCE pending authorization persistence
    let (_auth_url, state) = auth_mgr2.start_auth("client_test_id", "http://localhost:3842/callback", Some("/studio".to_string()));
    drop(auth_mgr2);
    drop(storage2);

    // Instance 3 starts up and retrieves the pending PKCE state
    let storage3 = Arc::new(Storage::new(db_path).expect("Failed to reopen storage 3"));
    let pending = storage3.take_pending_auth(&state).expect("Pending auth must survive restart");
    assert_eq!(pending.redirect_uri, "http://localhost:3842/callback");
    assert_eq!(pending.return_to, "/studio");

    // Once taken, pending auth is consumed
    assert!(storage3.take_pending_auth(&state).is_none());

    // 5. Test Stripe Webhook Idempotency persistence
    let stripe_store = StripeIdempotencyStore::new().with_storage(storage3.clone());
    assert!(!stripe_store.is_processed("evt_test_persistent_99").await);
    stripe_store.record("evt_test_persistent_99").await;
    assert!(stripe_store.is_processed("evt_test_persistent_99").await);

    drop(stripe_store);
    drop(storage3);

    // Instance 4 checks Stripe event
    let storage4 = Arc::new(Storage::new(db_path).expect("Failed to reopen storage 4"));
    let stripe_store4 = StripeIdempotencyStore::new().with_storage(storage4.clone());
    assert!(stripe_store4.is_processed("evt_test_persistent_99").await);

    // 6. Test pruning stale data
    let pruned = storage4.prune_stale_data(0).expect("Prune should succeed");
    // With 0 days cutoff, the stripe event recorded just now should be pruned
    assert!(pruned >= 1, "At least 1 stale record should be pruned");
    assert!(!storage4.is_stripe_event_processed("evt_test_persistent_99"));

    // Cleanup
    let _ = std::fs::remove_file(db_path);
}

#[test]
fn test_p0_blocker_4_production_daemon_and_reverse_proxy_topology() {
    // 1. Systemd Service Unit Verification
    let service_content = std::fs::read_to_string("deploy/pressmark.service")
        .expect("deploy/pressmark.service must exist");
    assert!(service_content.contains("NoNewPrivileges=true"));
    assert!(service_content.contains("ProtectSystem=strict"));
    assert!(service_content.contains("ProtectHome=read-only"));
    assert!(service_content.contains("PrivateTmp=true"));
    assert!(service_content.contains("LimitNOFILE=65536"));
    assert!(service_content.contains("Restart=always"));
    assert!(service_content.contains("RestartSec=5s"));
    assert!(service_content.contains("KillMode=control-group"));
    assert!(service_content.contains("CapabilityBoundingSet="));
    assert!(service_content.contains("ProtectClock=true"));
    assert!(service_content.contains("ProtectHostname=true"));
    assert!(service_content.contains("ReadWritePaths=/var/lib/pressmark /var/log/pressmark /var/cache/pressmark"));
    assert!(service_content.contains("EnvironmentFile=-/etc/pressmark/pressmark.env"));
    assert!(service_content.contains("Daniel Hallman"));

    // 2. Production Environment Template Verification
    let env_content = std::fs::read_to_string(".env.production.example")
        .expect(".env.production.example must exist");
    assert!(env_content.contains("PRESSMARK_HOST=127.0.0.1"));
    assert!(env_content.contains("PRESSMARK_PORT=3842"));
    assert!(env_content.contains("PRESSMARK_BASE_URL=https://pressmark.dev"));
    assert!(env_content.contains("PRESSMARK_DB_PATH=/var/lib/pressmark/pressmark.db"));
    assert!(env_content.contains("PRESSMARK_CACHE_DIR=/var/cache/pressmark"));
    assert!(env_content.contains("PRESSMARK_ENGINE_BIN="));
    assert!(env_content.contains("PRESSMARK_TRUST_PROXY_HEADERS=true"));
    assert!(env_content.contains("OPENAI_CHATGPT_CLIENT_ID="));
    assert!(env_content.contains("GHL_LOCATION_ID=TnDcaRLbCxm7LnOUF173"));
    assert!(env_content.contains("HIGHLEVEL_LOCATION_ID=TnDcaRLbCxm7LnOUF173"));
    assert!(env_content.contains("STRIPE_LIVE_MODE=false"));
    assert!(env_content.contains("STRIPE_SINGLE_PASS_PRICE_ID_TEST="));
    assert!(env_content.contains("STRIPE_SINGLE_PASS_PRICE_ID_LIVE="));

    // 3. Caddyfile Verification
    let caddy_content = std::fs::read_to_string("deploy/Caddyfile")
        .expect("deploy/Caddyfile must exist");
    assert!(caddy_content.contains("pressmark.dev"));
    assert!(caddy_content.contains("reverse_proxy 127.0.0.1:3842"));
    assert!(caddy_content.contains("max_size 25MB"));
    assert!(caddy_content.contains("flush_interval -1"));
    assert!(caddy_content.contains("Strict-Transport-Security"));

    // 4. Nginx Configuration Verification
    let nginx_content = std::fs::read_to_string("deploy/nginx.conf")
        .expect("deploy/nginx.conf must exist");
    assert!(nginx_content.contains("upstream pressmark_backend"));
    assert!(nginx_content.contains("server 127.0.0.1:3842;"));
    assert!(nginx_content.contains("client_max_body_size 25M;"));
    assert!(nginx_content.contains("location /mcp"));
    assert!(nginx_content.contains("proxy_buffering off;"));
    assert!(nginx_content.contains("proxy_buffers 16 32k;"));
    assert!(nginx_content.contains("proxy_set_header Upgrade $http_upgrade;"));
    assert!(nginx_content.contains("Strict-Transport-Security"));
}

#[tokio::test]
async fn test_openai_apps_challenge_endpoint() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use pressmark::server::{create_router, AppState};
    use std::sync::Arc;
    use tower::ServiceExt;

    // Test with environment variable token
    std::env::set_var("OPENAI_APPS_CHALLENGE_TOKEN", "openai-challenge-test-token-xyz");

    let app_state = Arc::new(AppState::default());
    let router = create_router(app_state);

    let req = Request::builder()
        .uri("/.well-known/openai-apps-challenge")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res = router.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = axum::body::to_bytes(res.into_body(), 1024).await.unwrap();
    assert_eq!(&body[..], b"openai-challenge-test-token-xyz");

    std::env::remove_var("OPENAI_APPS_CHALLENGE_TOKEN");
}

#[tokio::test]
async fn test_setup_check_checkout_entitlement_and_discovery_routes() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use hmac::{Hmac, Mac};
    use pressmark::server::{create_router, AppState};
    use pressmark::storage::Storage;
    use sha2::Sha256;
    use std::sync::Arc;
    use tower::ServiceExt;

    let storage = Arc::new(Storage::new(":memory:").unwrap());
    let app_state = Arc::new(AppState::default().with_storage(storage.clone()));
    let router = create_router(app_state);

    // 1. Verify /api/v1/setup-check
    let req_setup = Request::builder()
        .uri("/api/v1/setup-check")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_setup = router.clone().oneshot(req_setup).await.unwrap();
    assert_eq!(res_setup.status(), StatusCode::OK);
    let setup_bytes = axum::body::to_bytes(res_setup.into_body(), 1024 * 1024).await.unwrap();
    let setup_json: serde_json::Value = serde_json::from_slice(&setup_bytes).unwrap();
    assert_eq!(setup_json["checks"]["vector_engine"]["pdf_smoke_test"], true);
    assert_eq!(setup_json["checks"]["mcp_and_discovery"]["tools_count"], 6);
    assert_eq!(setup_json["checks"]["mcp_and_discovery"]["archetypes_count"], 57);

    // 2. Verify /api/v1/checkout & /api/v1/pricing checkout_url
    let req_checkout = Request::builder()
        .uri("/api/v1/checkout?plan=plan_solo&client_reference_id=usr_checkout_99")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_checkout = router.clone().oneshot(req_checkout).await.unwrap();
    assert_eq!(res_checkout.status(), StatusCode::OK);
    let checkout_bytes = axum::body::to_bytes(res_checkout.into_body(), 1024 * 1024).await.unwrap();
    let checkout_json: serde_json::Value = serde_json::from_slice(&checkout_bytes).unwrap();
    assert_eq!(checkout_json["success"], true);
    assert_eq!(checkout_json["plan_id"], "plan_solo");
    assert!(checkout_json["checkout_url"]
        .as_str()
        .unwrap()
        .contains("client_reference_id=usr_checkout_99"));

    // 3. Verify /pricing, /demo, /favicon.ico, /.well-known/oauth-protected-resource, /.well-known/mcp/server-card.json
    for route in [
        "/pricing",
        "/demo",
        "/favicon.ico",
        "/.well-known/oauth-protected-resource",
        "/.well-known/oauth-protected-resource/mcp",
        "/.well-known/mcp/server-card.json",
        "/.well-known/agent-card.json",
        "/.well-known/agent.json",
    ] {
        let req = Request::builder()
            .uri(route)
            .method("GET")
            .body(Body::empty())
            .unwrap();
        let res = router.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK, "Route {} failed", route);
    }

    // 4. Verify Stripe Checkout webhook persists customer entitlement and upgrades email login
    std::env::set_var("STRIPE_WEBHOOK_SECRET", "whsec_entitlement_test_secret");
    let now = chrono::Utc::now().timestamp();
    let webhook_payload = serde_json::json!({
        "id": "evt_entitlement_business_001",
        "type": "checkout.session.completed",
        "data": {
            "object": {
                "id": "cs_live_business_001",
                "amount_total": 4900,
                "customer_details": {
                    "email": "billing.upgrade@pressmark.dev"
                }
            }
        }
    })
    .to_string();

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(b"whsec_entitlement_test_secret").unwrap();
    mac.update(format!("{}.{}", now, webhook_payload).as_bytes());
    let sig_hex = hex::encode(mac.finalize().into_bytes());
    let sig_header = format!("t={},v1={}", now, sig_hex);

    let req_wh = Request::builder()
        .uri("/api/v1/webhooks/stripe")
        .method("POST")
        .header("content-type", "application/json")
        .header("stripe-signature", sig_header)
        .body(Body::from(webhook_payload))
        .unwrap();
    let res_wh = router.clone().oneshot(req_wh).await.unwrap();
    assert_eq!(res_wh.status(), StatusCode::OK);

    assert_eq!(
        storage.get_customer_entitlement("billing.upgrade@pressmark.dev").as_deref(),
        Some("business")
    );
}

#[tokio::test]
async fn test_usage_telemetry_analytics_copilot_and_mcp_aliases() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use pressmark::server::{create_router, AppState};
    use pressmark::storage::Storage;
    use std::sync::Arc;
    use tower::ServiceExt;

    let storage = Arc::new(Storage::new(":memory:").unwrap());
    let app_state = Arc::new(AppState::default().with_storage(storage.clone()));
    let router = create_router(app_state);

    // 1. Verify MCP & A2A scanner route aliases return 200
    for route in [
        "/mcp/",
        "/mcp/mcp",
        "/mcp/sse",
        "/sse",
        "/mcp/.well-known/agent.json",
        "/mcp/.well-known/agent-card.json",
    ] {
        let req = Request::builder()
            .uri(route)
            .method("GET")
            .body(Body::empty())
            .unwrap();
        let res = router.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK, "Alias route {} failed", route);
    }

    // 2. Call POST /mcp/ with calculate_totals and verify usage telemetry records tool + doc_type
    let mcp_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 42,
        "method": "tools/call",
        "params": {
            "name": "calculate_totals",
            "arguments": {
                "document": {
                    "doc_type": "residential_lease",
                    "from": { "name": "Apex Property Mgmt" },
                    "to": { "name": "Jane Tenant" },
                    "items": [{ "description": "Monthly Base Rent", "quantity": 1.0, "unit_price": 2400.0 }]
                }
            }
        }
    });
    let req_mcp = Request::builder()
        .uri("/mcp/")
        .method("POST")
        .header("content-type", "application/json")
        .header("user-agent", "MCP-Test-Agent/2.0")
        .body(Body::from(mcp_body.to_string()))
        .unwrap();
    let res_mcp = router.clone().oneshot(req_mcp).await.unwrap();
    assert_eq!(res_mcp.status(), StatusCode::OK);

    // 3. Test natural-language /api/v1/copilot parsing (entities, line items, tax rate, theme, signature)
    let copilot_payload = serde_json::json!({
        "prompt": "Create a professional invoice for client \"Vance Dynamics\" from \"Pressmark Studio\" for 32 hours of Rust backend engineering at $185/hr and a $500 setup fee, plus 8.25% tax. Apply the Modern Slate theme and include a signature block.",
        "current_state": {
            "doc_type": "estimate",
            "from": { "name": "Old Sender" },
            "to": { "name": "Old Client" },
            "items": []
        }
    });
    let req_copilot = Request::builder()
        .uri("/api/v1/copilot")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(copilot_payload.to_string()))
        .unwrap();
    let res_copilot = router.clone().oneshot(req_copilot).await.unwrap();
    assert_eq!(res_copilot.status(), StatusCode::OK);
    let copilot_bytes = axum::body::to_bytes(res_copilot.into_body(), 1024 * 1024).await.unwrap();
    let copilot_json: serde_json::Value = serde_json::from_slice(&copilot_bytes).unwrap();
    assert_eq!(copilot_json["success"], true);
    assert_eq!(copilot_json["modified_state"]["doc_type"], "invoice");
    assert_eq!(copilot_json["modified_state"]["to"]["name"], "Vance Dynamics");
    assert_eq!(copilot_json["modified_state"]["from"]["name"], "Pressmark Studio");
    assert_eq!(copilot_json["modified_state"]["tax_rate"], 8.25);
    assert_eq!(copilot_json["modified_state"]["theme"], "slate");
    assert_eq!(copilot_json["modified_state"]["show_signature"], true);
    let items = copilot_json["modified_state"]["items"].as_array().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["quantity"], 32.0);
    assert_eq!(items[0]["unit_price"], 185.0);
    assert_eq!(items[1]["unit_price"], 500.0);

    // 4. Test parenthesized milestone items preset in /api/v1/copilot
    let milestone_payload = serde_json::json!({
        "prompt": "Add 3 milestone line items for Backend Architecture ($4,800), API Gateway ($2,400), and CI/CD Pipeline ($1,600)",
        "current_state": {
            "doc_type": "invoice",
            "from": { "name": "Pressmark Studio" },
            "to": { "name": "Vance Dynamics" },
            "items": []
        }
    });
    let req_ms = Request::builder()
        .uri("/api/v1/copilot")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(milestone_payload.to_string()))
        .unwrap();
    let res_ms = router.clone().oneshot(req_ms).await.unwrap();
    assert_eq!(res_ms.status(), StatusCode::OK);
    let ms_bytes = axum::body::to_bytes(res_ms.into_body(), 1024 * 1024).await.unwrap();
    let ms_json: serde_json::Value = serde_json::from_slice(&ms_bytes).unwrap();
    let ms_items = ms_json["modified_state"]["items"].as_array().unwrap();
    assert_eq!(ms_items.len(), 3);
    assert_eq!(ms_items[0]["unit_price"], 4800.0);
    assert_eq!(ms_items[1]["unit_price"], 2400.0);
    assert_eq!(ms_items[2]["unit_price"], 1600.0);

    // 4b. Test MCP hyphenated doc_type normalization + compile_ms + crawler bot / probe segmentation
    let mcp_gen_payload = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 99,
        "method": "tools/call",
        "params": {
            "name": "generate_document",
            "arguments": {
                "doc_type": "security-deposit-disposition",
                "from": { "name": "Harbor Property Mgmt" },
                "to": { "name": "Alex Tenant" },
                "items": [
                    { "description": "Original Security Deposit Held", "quantity": 1.0, "unit_price": 2000.0 },
                    { "description": "Less: Carpet Steam Cleaning", "quantity": 1.0, "unit_price": -150.0 }
                ]
            }
        }
    });
    let req_mcp_gen = Request::builder()
        .uri("/mcp")
        .method("POST")
        .header("content-type", "application/json")
        .header("user-agent", "claude-code/1.0")
        .body(Body::from(mcp_gen_payload.to_string()))
        .unwrap();
    let res_mcp_gen = router.clone().oneshot(req_mcp_gen).await.unwrap();
    assert_eq!(res_mcp_gen.status(), StatusCode::OK);

    // Crawler bot hitting /api/v1/checkout should NOT record checkout_init, while organic browser does
    let req_bot_checkout = Request::builder()
        .uri("/api/v1/checkout?plan=pass_single_unlock&redirect=true")
        .method("GET")
        .header("user-agent", "Mozilla/5.0 (compatible; ClaudeBot/1.0; +claudebot@anthropic.com)")
        .body(Body::empty())
        .unwrap();
    let res_bot_checkout = router.clone().oneshot(req_bot_checkout).await.unwrap();
    assert_eq!(res_bot_checkout.status(), StatusCode::SEE_OTHER);

    let req_org_checkout = Request::builder()
        .uri("/api/v1/checkout?plan=pass_single_unlock&redirect=true")
        .method("GET")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)")
        .body(Body::empty())
        .unwrap();
    let res_org_checkout = router.clone().oneshot(req_org_checkout).await.unwrap();
    assert_eq!(res_org_checkout.status(), StatusCode::SEE_OTHER);

    // Crawler bot and internal probe hitting /templates/residential-lease
    let req_bot_tpl = Request::builder()
        .uri("/templates/residential-lease")
        .method("GET")
        .header("user-agent", "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)")
        .body(Body::empty())
        .unwrap();
    let _ = router.clone().oneshot(req_bot_tpl).await.unwrap();

    let req_probe_tpl = Request::builder()
        .uri("/templates/residential-lease")
        .method("GET")
        .header("user-agent", "HallmanLab-ProductDoctor/1.0")
        .body(Body::empty())
        .unwrap();
    let _ = router.clone().oneshot(req_probe_tpl).await.unwrap();

    // 5. Query /api/v1/analytics and verify recorded usage events, canonical archetype, compile_ms, and actor breakdown
    let req_analytics = Request::builder()
        .uri("/api/v1/analytics")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_analytics = router.clone().oneshot(req_analytics).await.unwrap();
    assert_eq!(res_analytics.status(), StatusCode::OK);
    let analytics_bytes = axum::body::to_bytes(res_analytics.into_body(), 1024 * 1024).await.unwrap();
    let analytics_json: serde_json::Value = serde_json::from_slice(&analytics_bytes).unwrap();
    let total_ev = analytics_json["total_usage_events"].as_i64().unwrap();
    let organic_ev = analytics_json["external_organic_events"].as_i64().unwrap();
    let bot_ev = analytics_json["crawler_bot_events"].as_i64().unwrap();
    let probe_ev = analytics_json["internal_probe_events"].as_i64().unwrap();
    assert!(total_ev >= 7);
    assert_eq!(total_ev, organic_ev + bot_ev + probe_ev);
    assert_eq!(bot_ev, 1);
    assert_eq!(probe_ev, 1);
    assert_eq!(analytics_json["events_by_type"]["checkout_init"], 1);
    assert_eq!(analytics_json["mcp_tool_calls"]["calculate_totals"], 1);
    assert_eq!(analytics_json["mcp_tool_calls"]["generate_document"], 1);
    assert_eq!(analytics_json["top_archetypes"]["residential_lease"], 3);
    assert_eq!(analytics_json["top_archetypes"]["security_deposit_disposition"], 1);
    assert!(analytics_json["top_archetypes"].get("security-deposit-disposition").is_none());

    // Verify classification of real production User-Agents (HallmanLab-ProductDoctor/1.0 & MCP directory scanners)
    assert!(Storage::is_internal_probe_ua("HallmanLab-ProductDoctor/1.0"));
    assert_eq!(
        Storage::classify_usage_actor("rest_api", Some("HallmanLab-ProductDoctor/1.0")),
        "internal_probe"
    );
    assert_eq!(
        Storage::classify_usage_actor(
            "mcpbeat/0.1 (+https://mcpbeat.com/bot/; liveness check)",
            Some("tools/list")
        ),
        "crawler_bot"
    );
    assert_eq!(
        Storage::classify_usage_actor(
            "Go-http-client/2.0",
            Some("__verifymcp_auth_probe_e4d987cedfba3bef__")
        ),
        "crawler_bot"
    );

    // Verify free-tier vs paid-entitlement GHL watermark edge cases (None, "", "none", "false", "DRAFT")
    for empty_sentinel in [None, Some(""), Some("none"), Some("false")] {
        let mut free_doc = pressmark::models::DocumentRequest::default();
        free_doc.watermark = empty_sentinel.map(str::to_string);
        free_doc.apply_free_tier_ghl_watermark(false);
        assert_eq!(
            free_doc.watermark.as_deref(),
            Some("PRESSMARK FREE TIER"),
            "Unentitled doc with watermark {:?} must default to PRESSMARK FREE TIER",
            empty_sentinel
        );

        let mut paid_doc = pressmark::models::DocumentRequest::default();
        paid_doc.watermark = empty_sentinel.map(str::to_string);
        paid_doc.apply_free_tier_ghl_watermark(true);
        assert_eq!(
            paid_doc.watermark, None,
            "Paid doc with watermark {:?} must remain None",
            empty_sentinel
        );
    }
    let mut custom_free_doc = pressmark::models::DocumentRequest::default();
    custom_free_doc.watermark = Some("DRAFT".to_string());
    custom_free_doc.apply_free_tier_ghl_watermark(false);
    assert_eq!(
        custom_free_doc.watermark.as_deref(),
        Some("DRAFT"),
        "Custom watermark DRAFT on free doc must never be overwritten"
    );
    let mut custom_paid_doc = pressmark::models::DocumentRequest::default();
    custom_paid_doc.watermark = Some("DRAFT".to_string());
    custom_paid_doc.apply_free_tier_ghl_watermark(true);
    assert_eq!(
        custom_paid_doc.watermark.as_deref(),
        Some("DRAFT"),
        "Custom watermark DRAFT on paid doc must never be overwritten"
    );

    // Verify all 3 SKILL.md files use canonical DocumentType slugs, state_code, and REC- prefix
    let re_skill = std::fs::read_to_string("skills/real-estate-agreements/SKILL.md").unwrap();
    let inv_skill = std::fs::read_to_string("skills/invoice-generation/SKILL.md").unwrap();
    let lc_skill = std::fs::read_to_string("skills/document-lifecycle/SKILL.md").unwrap();
    assert!(re_skill.contains("state_code") && !re_skill.contains("governing_state"));
    assert!(inv_skill.contains("state_code") && !inv_skill.contains("governing_state"));
    assert!(lc_skill.contains("REC-") && !lc_skill.contains("RCT-"));
    for dt in pressmark::models::DocumentType::all() {
        assert!(
            inv_skill.contains(&format!("`{}`", dt.as_str())),
            "skills/invoice-generation/SKILL.md missing canonical archetype `{}`",
            dt.as_str()
        );
    }

    // 6. Verify static/index.html uses tracked /api/v1/checkout links and dialog ARIA attributes
    let html = std::fs::read_to_string("static/index.html").unwrap();
    assert!(html.contains("/api/v1/checkout?plan=pass_single_unlock&redirect=true"));
    assert!(html.contains("/api/v1/checkout?plan=plan_solo&redirect=true"));
    assert!(html.contains("/api/v1/checkout?plan=plan_business&redirect=true"));
    assert!(html.contains("id=\"pricingModal\" role=\"dialog\" aria-modal=\"true\""));
    assert!(html.contains("id=\"scanModal\" role=\"dialog\" aria-modal=\"true\""));
}

#[tokio::test]
async fn test_all_copilot_quick_presets_auth_redirect_and_storage_pruning() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use pressmark::auth::sanitize_return_to;
    use pressmark::server::{create_router, AppState};
    use pressmark::storage::Storage;
    use std::sync::Arc;
    use tower::ServiceExt;

    let storage = Arc::new(Storage::new(":memory:").unwrap());
    let state = Arc::new(AppState::new("http://localhost:3842".to_string()).with_storage(storage.clone()));
    let router = create_router(state.clone());

    let send_copilot = |prompt: &str, current_state: serde_json::Value| {
        let r = router.clone();
        let payload = serde_json::json!({
            "prompt": prompt,
            "current_state": current_state
        });
        async move {
            let req = Request::builder()
                .uri("/api/v1/copilot")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap();
            let res = r.oneshot(req).await.unwrap();
            assert_eq!(res.status(), StatusCode::OK);
            let bytes = axum::body::to_bytes(res.into_body(), 1024 * 1024).await.unwrap();
            serde_json::from_slice::<serde_json::Value>(&bytes).unwrap()
        }
    };

    let base_state = serde_json::json!({
        "doc_type": "estimate",
        "issue_date": "2026-10-01",
        "from": { "name": "Pressmark Studio" },
        "to": { "name": "Old Client" },
        "items": [{ "description": "Initial Consulting", "quantity": 1.0, "unit_price": 1000.0 }]
    });

    // Preset 1: "Draft a $4,500 strategic retainer invoice for Acme Corp with EU VAT reverse charge notice"
    let p1 = send_copilot(
        "Draft a $4,500 strategic retainer invoice for Acme Corp with EU VAT reverse charge notice",
        base_state.clone(),
    )
    .await;
    assert_eq!(p1["modified_state"]["doc_type"], "invoice");
    assert_eq!(p1["modified_state"]["to"]["name"], "Acme Corp");
    assert_eq!(p1["modified_state"]["compliance_mode"], "eu_vat");
    assert_eq!(p1["modified_state"]["items"][0]["unit_price"], 4500.0);

    // Preset 2: "Add 3 milestone line items for Backend Architecture ($4,800), API Gateway ($2,400), and CI/CD Pipeline ($1,600)"
    let p2 = send_copilot(
        "Add 3 milestone line items for Backend Architecture ($4,800), API Gateway ($2,400), and CI/CD Pipeline ($1,600)",
        base_state.clone(),
    )
    .await;
    assert_eq!(p2["modified_state"]["items"].as_array().unwrap().len(), 3);
    assert_eq!(p2["modified_state"]["to"]["name"], "Old Client");

    // Preset 3: "Apply UK 20% VAT rate, change currency to GBP (£), and format dates in Military style"
    let p3 = send_copilot(
        "Apply UK 20% VAT rate, change currency to GBP (£), and format dates in Military style",
        base_state.clone(),
    )
    .await;
    assert_eq!(p3["modified_state"]["tax_rate"], 20.0);
    assert_eq!(p3["modified_state"]["currency"], "GBP");
    assert_eq!(p3["modified_state"]["date_format"], "short_abbrev");

    // Preset 4: "Convert document into Independent Contractor Agreement with dispute arbitration clause"
    let p4 = send_copilot(
        "Convert document into Independent Contractor Agreement with dispute arbitration clause",
        base_state.clone(),
    )
    .await;
    assert_eq!(p4["modified_state"]["doc_type"], "contractor_agreement");
    assert!(
        p4["modified_state"]["terms"]
            .as_str()
            .unwrap_or("")
            .to_lowercase()
            .contains("arbitration")
    );

    // Preset 5: "Add IRS W-9 certification with taxpayer identification number attestation"
    let p5 = send_copilot(
        "Add IRS W-9 certification with taxpayer identification number attestation",
        base_state.clone(),
    )
    .await;
    assert_eq!(p5["modified_state"]["compliance_mode"], "irs_w9");

    // Preset 6: "Apply 15% promotional discount with 14-day payment settlement terms"
    let p6 = send_copilot(
        "Apply 15% promotional discount with 14-day payment settlement terms",
        base_state.clone(),
    )
    .await;
    assert_eq!(p6["modified_state"]["discount_amount"], 150.0);
    assert_eq!(p6["modified_state"]["due_date"], "2026-10-15");
    assert!(
        p6["modified_state"]["terms"]
            .as_str()
            .unwrap_or("")
            .contains("Net 14")
    );

    // Verify theme change updates primary_color/secondary_color/accent_color/surface_color even when current_state has stale primary_color,
    // and verify words like "credit", "European date", "audit", "Decade" do not trigger false-positive theme/currency changes
    let mut state_with_colors = base_state.clone();
    state_with_colors["primary_color"] = serde_json::json!("#4f46e5");
    state_with_colors["currency"] = serde_json::json!("USD");
    state_with_colors["theme"] = serde_json::json!("modern_slate");

    let p_theme = send_copilot(
        "Switch to Emerald theme and format dates in European date style",
        state_with_colors.clone(),
    )
    .await;
    assert_eq!(p_theme["modified_state"]["theme"], "emerald");
    assert_eq!(p_theme["modified_state"]["primary_color"], "#059669");
    assert_eq!(p_theme["modified_state"]["date_format"], "euro");
    assert_eq!(p_theme["modified_state"]["currency"], "USD");

    let p_no_false_pos = send_copilot(
        "Create a credit note for client \"Decade Motors\" for 10 hours of security audit at $200/hr",
        state_with_colors.clone(),
    )
    .await;
    assert_eq!(p_no_false_pos["modified_state"]["doc_type"], "credit_note");
    assert_eq!(p_no_false_pos["modified_state"]["to"]["name"], "Decade Motors");
    assert_eq!(p_no_false_pos["modified_state"]["theme"], "modern_slate");
    assert_eq!(p_no_false_pos["modified_state"]["currency"], "USD");

    // Verify sanitize_return_to and /api/v1/auth/chatgpt/login + callback open-redirect protection
    assert_eq!(sanitize_return_to(Some("https://evil.example/phish"), "/?auth=success"), "/?auth=success");
    assert_eq!(sanitize_return_to(Some("//evil.example/phish"), "/?auth=success"), "/?auth=success");
    assert_eq!(sanitize_return_to(Some("/\\evil.example"), "/?auth=success"), "/?auth=success");
    assert_eq!(sanitize_return_to(Some("/valid/path?tab=1"), "/?auth=success"), "/valid/path?tab=1");

    let req_login = Request::builder()
        .uri("/api/v1/auth/chatgpt/login?demo=plus&return_to=https://evil.example/steal")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_login = router.clone().oneshot(req_login).await.unwrap();
    assert_eq!(res_login.status(), StatusCode::FOUND);
    assert_eq!(res_login.headers().get("location").unwrap(), "/?auth=success");

    let (_auth_url, oauth_state) = state.auth_mgr.start_auth(
        "pressmark-client",
        "https://pressmark.dev/api/v1/auth/chatgpt/callback",
        Some("/studio?tab=billing".to_string()),
    );
    let req_cb = Request::builder()
        .uri(format!("/api/v1/auth/chatgpt/callback?code=abc&state={}", oauth_state))
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_cb = router.clone().oneshot(req_cb).await.unwrap();
    assert_eq!(res_cb.status(), StatusCode::FOUND);
    assert_eq!(res_cb.headers().get("location").unwrap(), "/studio?tab=billing");

    // Verify Content-Disposition filename sanitization on /api/v1/generate?format=pdf
    let req_pdf = Request::builder()
        .uri("/api/v1/generate?format=pdf")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({
                "doc_type": "invoice",
                "doc_number": "INV-\"2026\"/..\\99",
                "from": { "name": "Sender" },
                "to": { "name": "Client" },
                "items": []
            })
            .to_string(),
        ))
        .unwrap();
    let res_pdf = router.clone().oneshot(req_pdf).await.unwrap();
    assert_eq!(res_pdf.status(), StatusCode::OK);
    let cd = res_pdf.headers().get("content-disposition").unwrap().to_str().unwrap();
    assert_eq!(cd, "attachment; filename=\"INV-2026..99.pdf\"");

    // Verify static/index.html has 57 template-pill buttons and updated copy
    let html = std::fs::read_to_string("static/index.html").unwrap();
    assert_eq!(html.matches("class=\"template-pill").count(), 57);
    assert!(html.contains("Switch Archetype (57)"));
    assert!(!html.contains("Neural Receipt"));

    // Verify storage prune_stale_data runs cleanly with usage_events retention
    let pruned = storage.prune_stale_data(30).unwrap();
    assert_eq!(pruned, 0);
}

#[tokio::test]
async fn test_seo_template_landings_mcp_conversion_and_refund_revocation() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use pressmark::auth::{ChatGptAuthManager, ChatGptPlan};
    use pressmark::mcp::{JsonRpcRequest, McpHandler};
    use pressmark::rate_limit::RateLimiter;
    use pressmark::server::{create_router, AppState};
    use pressmark::storage::Storage;
    use pressmark::stripe::StripeIdempotencyStore;
    use std::sync::Arc;
    use tower::ServiceExt;

    let storage = Arc::new(Storage::new(":memory:").unwrap());
    let auth_mgr = ChatGptAuthManager::in_memory().with_storage(Arc::clone(&storage));
    let stripe_idempotency = StripeIdempotencyStore::new().with_storage(Arc::clone(&storage));
    let state = Arc::new(AppState {
        base_url: "https://pressmark.dev".to_string(),
        auth_mgr: auth_mgr.clone(),
        rate_limiter: RateLimiter::new(1000, 60),
        stripe_idempotency,
        storage: Arc::clone(&storage),
    });
    let router = create_router(Arc::clone(&state));

    // 1. Verify GET /api/v1/templates returns all 57 document archetypes
    let req_cat = Request::builder()
        .uri("/api/v1/templates")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_cat = router.clone().oneshot(req_cat).await.unwrap();
    assert_eq!(res_cat.status(), StatusCode::OK);
    let body_cat = axum::body::to_bytes(res_cat.into_body(), usize::MAX).await.unwrap();
    let cat_json: serde_json::Value = serde_json::from_slice(&body_cat).unwrap();
    assert_eq!(cat_json["count"], 57);
    assert_eq!(cat_json["templates"].as_array().unwrap().len(), 57);

    // 2. Verify GET /templates/residential-lease and /templates/commercial-nnn-lease customize SEO tags and deep-link archetype
    let req_lease = Request::builder()
        .uri("/templates/residential-lease")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_lease = router.clone().oneshot(req_lease).await.unwrap();
    assert_eq!(res_lease.status(), StatusCode::OK);
    let html_lease = String::from_utf8(
        axum::body::to_bytes(res_lease.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    assert!(html_lease.contains("<title>Residential Lease Agreement Template & PDF Generator · Pressmark Studio</title>"));
    assert!(html_lease.contains("<link rel=\"canonical\" href=\"https://pressmark.dev/templates/residential-lease\">"));
    assert!(html_lease.contains("window.__PRESSMARK_INITIAL_ARCHETYPE__ = \"residential_lease\";"));

    // Verify unknown template slug returns 404
    let req_404 = Request::builder()
        .uri("/templates/non-existent-archetype-xyz")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_404 = router.clone().oneshot(req_404).await.unwrap();
    assert_eq!(res_404.status(), StatusCode::NOT_FOUND);

    // 3. Verify sitemap.xml includes all 57 /templates/<slug> routes + 51 state-prefixed residential lease routes + 5 state real estate slugs
    let req_sm = Request::builder()
        .uri("/sitemap.xml")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_sm = router.clone().oneshot(req_sm).await.unwrap();
    assert_eq!(res_sm.status(), StatusCode::OK);
    let sm_xml = String::from_utf8(
        axum::body::to_bytes(res_sm.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    assert_eq!(sm_xml.matches("https://pressmark.dev/templates/").count(), 113);

    // 4. Verify REST /api/v1/generate and MCP generate_document include 1-click checkout URLs
    let req_gen = Request::builder()
        .uri("/api/v1/generate")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({
                "doc_type": "invoice",
                "from": { "name": "Apex Studio" },
                "to": { "name": "Client Co" },
                "items": [{ "description": "Design Sprint", "quantity": 1.0, "unit_price": 1200.0 }]
            })
            .to_string(),
        ))
        .unwrap();
    let res_gen = router.clone().oneshot(req_gen).await.unwrap();
    assert_eq!(res_gen.status(), StatusCode::OK);
    assert_eq!(
        res_gen.headers().get("x-upgrade-checkout-url").unwrap(),
        "https://pressmark.dev/api/v1/checkout?plan=pass_single_unlock&redirect=true"
    );
    let gen_json: serde_json::Value = serde_json::from_slice(
        &axum::body::to_bytes(res_gen.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert_eq!(
        gen_json["unlock_checkout_url"],
        "https://pressmark.dev/api/v1/checkout?plan=pass_single_unlock&redirect=true"
    );
    assert_eq!(
        gen_json["pro_checkout_url"],
        "https://pressmark.dev/api/v1/checkout?plan=plan_solo&redirect=true"
    );

    let mcp_resp = McpHandler::handle_request(JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(77)),
        method: "tools/call".to_string(),
        params: Some(serde_json::json!({
            "name": "generate_document",
            "arguments": {
                "doc_type": "invoice",
                "from": { "name": "Apex Studio" },
                "to": { "name": "Client Co" },
                "items": [{ "description": "Design Sprint", "quantity": 1.0, "unit_price": 1200.0 }]
            }
        })),
    });
    let mcp_res = mcp_resp.result.unwrap();
    assert_eq!(
        mcp_res["unlock_checkout_url"],
        "https://pressmark.dev/api/v1/checkout?plan=pass_single_unlock&redirect=true"
    );
    assert_eq!(
        mcp_res["pro_checkout_url"],
        "https://pressmark.dev/api/v1/checkout?plan=plan_solo&redirect=true"
    );

    // 5. Verify entitlement upgrade and revocation upon refund
    auth_mgr.upgrade_user_entitlement("refund.test@example.org", ChatGptPlan::SoloPro, Some("cs_123"), 1500);
    assert_eq!(
        storage.get_customer_entitlement("refund.test@example.org").as_deref(),
        Some("solo_pro")
    );
    let sess = auth_mgr.create_email_session("refund.test@example.org", Some("Refund Tester"));
    assert!(sess.user.is_subscriber);

    auth_mgr.revoke_user_entitlement("refund.test@example.org");
    assert!(storage.get_customer_entitlement("refund.test@example.org").is_none());
    let updated_sess = auth_mgr.get_session(&sess.session_id).unwrap();
    assert!(!updated_sess.user.is_subscriber);
    assert_eq!(updated_sess.user.plan, ChatGptPlan::Free);
}

#[tokio::test]
async fn test_pressmark_normalization_audit_all_9_items() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use pressmark::engine::generate_pdf;
    use pressmark::mcp::{JsonRpcRequest, McpHandler};
    use pressmark::models::{DocumentRequest, DocumentType};
    use pressmark::openapi::get_openapi_spec;
    use pressmark::server::{create_router, AppState};
    use pressmark::state_laws::infer_state_from_address;
    use pressmark::storage::Storage;
    use std::sync::Arc;
    use tower::ServiceExt;

    let storage = Arc::new(Storage::new(":memory:").unwrap());
    let app_state = Arc::new(AppState::new("https://pressmark.dev".to_string()).with_storage(storage));
    let router = create_router(app_state);

    // -------------------------------------------------------------------------
    // ITEM 1: Negative Line Items & Deductions Across Accounting/RE Archetypes
    // -------------------------------------------------------------------------
    assert!(DocumentType::ScheduleETaxWorksheet.allows_negative_items());
    assert!(DocumentType::PropertyManagementStatement.allows_negative_items());
    assert!(DocumentType::SecurityDepositDisposition.allows_negative_items());
    assert!(DocumentType::TenantRentLedger.allows_negative_items());
    assert!(DocumentType::RealEstateDealAnalysis.allows_negative_items());
    assert!(DocumentType::RealEstatePurchaseAgreement.allows_negative_items());
    assert!(DocumentType::CreditNote.allows_negative_items());
    assert!(!DocumentType::Invoice.allows_negative_items());

    // MCP validate_document_request & generate_document accept negative unit_price on schedule_e_tax_worksheet
    let sched_e_doc: DocumentRequest = serde_json::from_value(serde_json::json!({
        "doc_type": "schedule_e_tax_worksheet",
        "from": { "name": "Hallman Capital Properties LLC" },
        "to": { "name": "Vance CPAs" },
        "items": [
            { "description": "Line 3 Gross Rents", "quantity": 1.0, "unit_price": 148200.0 },
            { "description": "Line 12 Mortgage Interest", "quantity": 1.0, "unit_price": -21400.0 },
            { "description": "Line 18 Depreciation", "quantity": 1.0, "unit_price": -38900.0 }
        ]
    }))
    .unwrap();
    assert!(McpHandler::validate_document_request(&sched_e_doc).is_ok());

    let mcp_sched_e = McpHandler::handle_request(JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(901)),
        method: "tools/call".to_string(),
        params: Some(serde_json::json!({
            "name": "generate_document",
            "arguments": sched_e_doc
        })),
    });
    assert!(
        mcp_sched_e.error.is_none() && mcp_sched_e.result.is_some(),
        "Schedule E negative items should succeed in MCP generate_document: {:?}",
        mcp_sched_e.error
    );

    // MCP generate_document still rejects negative unit_price on standard invoice
    let mcp_inv_neg = McpHandler::handle_request(JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(902)),
        method: "tools/call".to_string(),
        params: Some(serde_json::json!({
            "name": "generate_document",
            "arguments": {
                "doc_type": "invoice",
                "from": { "name": "Issuer" },
                "to": { "name": "Client" },
                "items": [
                    { "description": "Bad Negative Item", "quantity": 1.0, "unit_price": -500.0 }
                ]
            }
        })),
    });
    assert!(mcp_inv_neg.error.is_some(), "Standard invoice with negative unit_price must fail MCP validation");

    // Negative net grand_total preserved on credit_note / real_estate_deal_analysis
    let neg_credit: DocumentRequest = serde_json::from_value(serde_json::json!({
        "doc_type": "credit_note",
        "from": { "name": "Vendor" },
        "to": { "name": "Client" },
        "items": [{ "description": "SLA Credit Refund", "quantity": 1.0, "unit_price": -450.0 }]
    }))
    .unwrap();
    assert_eq!(neg_credit.grand_total(), -450.0);

    // -------------------------------------------------------------------------
    // ITEM 2: State & Municipal Statutory Jurisdiction Inference & Cross-Validation
    // -------------------------------------------------------------------------
    assert_eq!(infer_state_from_address("100 Main St Chattanooga TN 37402").map(|p| p.code), Some("TN"));
    assert_eq!(infer_state_from_address("100 Main St, Austin, TX, USA").map(|p| p.code), Some("TX"));
    assert_eq!(infer_state_from_address("742 Evergreen Terrace, san francisco, ca 94107").map(|p| p.code), Some("CA"));
    // Street suffix false-positive guards
    assert!(infer_state_from_address("100 Oak CT").is_none());
    assert!(infer_state_from_address("123 Martin Luther King IN").is_none());

    // Jurisdiction inference on expanded archetypes & custom field labels
    let deal_doc: DocumentRequest = serde_json::from_value(serde_json::json!({
        "doc_type": "real_estate_deal_analysis",
        "from": { "name": "Sponsor" },
        "to": { "name": "Investor" },
        "custom_fields": [{ "label": "Managed Portfolio", "value": "412 Northshore Dr, Chattanooga, TN 37405" }]
    }))
    .unwrap();
    assert_eq!(deal_doc.effective_state_profile().map(|p| p.code), Some("TN"));

    // Municipal cross-validation against explicit state_code
    let mismatched_muni: DocumentRequest = serde_json::from_value(serde_json::json!({
        "doc_type": "residential_lease",
        "state_code": "TX",
        "municipality": "NYC",
        "from": { "name": "Landlord" },
        "to": { "name": "Tenant" }
    }))
    .unwrap();
    assert!(
        mismatched_muni.effective_municipal_profile().is_none(),
        "NYC municipal profile must not apply when state_code is TX"
    );

    let matched_muni: DocumentRequest = serde_json::from_value(serde_json::json!({
        "doc_type": "residential_lease",
        "state_code": "NY",
        "municipality": "NYC",
        "from": { "name": "Landlord" },
        "to": { "name": "Tenant" }
    }))
    .unwrap();
    assert_eq!(matched_muni.effective_municipal_profile().map(|m| m.slug), Some("new-york-city"));

    // -------------------------------------------------------------------------
    // ITEM 3: Currency Normalization (ISO 4217 vs Symbols)
    // -------------------------------------------------------------------------
    let sym_curr_doc: DocumentRequest = serde_json::from_value(serde_json::json!({
        "doc_type": "invoice",
        "currency": "$",
        "currency_symbol_mode": "code",
        "from": { "name": "Issuer" },
        "to": { "name": "Client" },
        "items": [{ "description": "Consulting", "quantity": 1.0, "unit_price": 1250.0 }]
    }))
    .unwrap();
    assert_eq!(sym_curr_doc.normalized_currency_code(), "USD");
    assert_eq!(sym_curr_doc.format_amount(1250.0), "USD 1,250.00");

    let lower_curr_doc: DocumentRequest = serde_json::from_value(serde_json::json!({
        "doc_type": "invoice",
        "currency": "gbp",
        "currency_symbol_mode": "symbol",
        "from": { "name": "Issuer" },
        "to": { "name": "Client" }
    }))
    .unwrap();
    assert_eq!(lower_curr_doc.normalized_currency_code(), "GBP");
    assert_eq!(lower_curr_doc.format_amount(500.0), "£500.00");

    // -------------------------------------------------------------------------
    // ITEM 4: Compliance Banners & Execution Mode Normalization in Engine/Typst
    // -------------------------------------------------------------------------
    for (compliance, exec_mode) in [
        ("peppol", "wet-ink"),
        ("factur-x", "notarized_wet"),
        ("hipaa", "notarized_ron"),
        ("zatca", "ron"),
        ("pci_dss", "esign"),
        ("soc2", "esign"),
        ("statutory_sworn", "wet_ink"),
    ] {
        let mut comp_doc: DocumentRequest = serde_json::from_value(serde_json::json!({
            "doc_type": "property_deed",
            "compliance_mode": compliance,
            "execution_mode": exec_mode,
            "notary_required": true,
            "recordable": true,
            "state_code": "TN",
            "from": { "name": "Grantor LLC" },
            "to": { "name": "Grantee LLC" },
            "items": [{ "description": "Consideration", "quantity": 1.0, "unit_price": 250000.0 }]
        }))
        .unwrap();
        assert!(comp_doc.normalize_and_validate().is_ok());
        let (pdf, _ms) = generate_pdf(&comp_doc)
            .unwrap_or_else(|e| panic!("Failed to compile compliance={} exec={}: {}", compliance, exec_mode, e));
        assert!(pdf.starts_with(b"%PDF-"), "Expected valid PDF for compliance={}", compliance);
    }

    // -------------------------------------------------------------------------
    // ITEM 5: MCP & OpenAPI Schema Parity
    // -------------------------------------------------------------------------
    let openapi = get_openapi_spec("https://pressmark.dev");
    let oa_props = &openapi["components"]["schemas"]["DocumentRequest"]["properties"];
    assert_eq!(oa_props["currency"]["default"], "USD");
    for prop in [
        "state_code",
        "municipality",
        "county",
        "execution_mode",
        "notary_required",
        "recordable",
        "compliance_mode",
        "date_format",
        "currency_symbol_mode",
        "currency_placement",
        "currency_separators",
        "number_decimals",
        "custom_fields",
        "sections",
        "signatures",
    ] {
        assert!(oa_props.get(prop).is_some(), "OpenAPI DocumentRequest missing property {}", prop);
    }

    let mcp_tools = McpHandler::handle_request(JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(905)),
        method: "tools/list".to_string(),
        params: None,
    })
    .result
    .unwrap();
    let gen_tool = mcp_tools["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["name"] == "generate_document")
        .unwrap();
    let mcp_props = &gen_tool["inputSchema"]["properties"];
    for prop in [
        "state_code",
        "municipality",
        "county",
        "execution_mode",
        "notary_required",
        "recordable",
        "compliance_mode",
        "date_format",
        "currency_symbol_mode",
        "currency_placement",
        "currency_separators",
        "number_decimals",
        "custom_fields",
        "sections",
        "signatures",
    ] {
        assert!(mcp_props.get(prop).is_some(), "MCP generate_document missing property {}", prop);
    }

    // -------------------------------------------------------------------------
    // ITEM 6: Date Formatting & Numeric Sanitization
    // -------------------------------------------------------------------------
    let mut bad_tax_doc: DocumentRequest = serde_json::from_value(serde_json::json!({
        "doc_type": "invoice",
        "tax_rate": -15.0,
        "from": { "name": "Sender" },
        "to": { "name": "Receiver" }
    }))
    .unwrap();
    assert!(bad_tax_doc.normalize_and_validate().is_err());

    let mut date_doc: DocumentRequest = serde_json::from_value(serde_json::json!({
        "doc_type": "invoice",
        "date_format": "us_long",
        "tax_rate": 8.25,
        "number_decimals": 9,
        "from": { "name": "Sender" },
        "to": { "name": "Receiver" }
    }))
    .unwrap();
    assert!(date_doc.normalize_and_validate().is_ok());
    assert_eq!(date_doc.tax_rate, Some(8.25));
    assert_eq!(date_doc.number_decimals, Some(4));
    assert_eq!(date_doc.format_date("2026-10-25T14:30:00Z"), "October 25, 2026");
    assert_eq!(date_doc.format_date("10/5/2026"), "October 05, 2026");
    assert_eq!(date_doc.format_date("2026/10/05"), "October 05, 2026");

    // -------------------------------------------------------------------------
    // ITEM 7: Flexible DocumentType Deserialization & Field Aliases
    // -------------------------------------------------------------------------
    let aliased_doc: DocumentRequest = serde_json::from_value(serde_json::json!({
        "archetype": "tennessee-residential-lease",
        "number": "RLA-2026-7777",
        "jurisdiction": "tn",
        "landlord": { "name": "Lookout Mountain Properties LLC" },
        "tenant": { "name": "Caleb Montgomery" },
        "line_items": [
            { "title": "Monthly Base Rent", "qty": 1.0, "rate": 2450.0 }
        ]
    }))
    .unwrap();
    assert_eq!(aliased_doc.doc_type, DocumentType::ResidentialLease);
    assert_eq!(aliased_doc.doc_number.as_deref(), Some("RLA-2026-7777"));
    assert_eq!(aliased_doc.from.name, "Lookout Mountain Properties LLC");
    assert_eq!(aliased_doc.to.name, "Caleb Montgomery");
    assert_eq!(aliased_doc.items.len(), 1);
    assert_eq!(aliased_doc.items[0].description, "Monthly Base Rent");
    assert_eq!(aliased_doc.items[0].unit_price, 2450.0);
    assert_eq!(aliased_doc.effective_state_profile().map(|s| s.code), Some("TN"));

    for (raw_type, expected) in [
        ("ResidentialLease", DocumentType::ResidentialLease),
        ("Quitclaim Deed", DocumentType::PropertyDeed),
        ("Schedule E", DocumentType::ScheduleETaxWorksheet),
        ("msa", DocumentType::Agreement),
        ("sow", DocumentType::AgreementSow),
        ("loi", DocumentType::LetterOfIntentCre),
        ("psa", DocumentType::RealEstatePurchaseAgreement),
    ] {
        let parsed: DocumentType = serde_json::from_value(serde_json::json!(raw_type)).unwrap();
        assert_eq!(parsed, expected, "Failed alias deserialization for {}", raw_type);
    }

    // -------------------------------------------------------------------------
    // ITEM 8: Document Lifecycle Prefix Stripping, REC- Prefix & Refund Target
    // -------------------------------------------------------------------------
    assert_eq!(DocumentType::Receipt.default_number_prefix(), "REC-");
    let custom_prefix_doc: DocumentRequest = serde_json::from_value(serde_json::json!({
        "doc_type": "quotation_tender",
        "doc_number": "QUO-2026-0099",
        "from": { "name": "Contractor" },
        "to": { "name": "Authority" },
        "items": [{ "description": "Bid Item", "quantity": 1.0, "unit_price": 5000.0 }]
    }))
    .unwrap();
    let inv = custom_prefix_doc.to_invoice(14);
    assert_eq!(inv.doc_number.as_deref(), Some("INV-2026-0099"));
    let rec = inv.to_receipt("Stripe", "ch_123", "2026-10-25");
    assert_eq!(rec.doc_number.as_deref(), Some("REC-2026-0099"));
    let crd = rec.to_credit_note(Some("Project cancelled"));
    assert_eq!(crd.doc_number.as_deref(), Some("CRD-2026-0099"));

    // REST /api/v1/lifecycle/convert with target_type = "refund" -> 200 OK (credit_note)
    let req_refund = Request::builder()
        .uri("/api/v1/lifecycle/convert")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({
                "document": inv,
                "target_type": "refund",
                "reason": "Full customer refund"
            })
            .to_string(),
        ))
        .unwrap();
    let res_refund = router.clone().oneshot(req_refund).await.unwrap();
    assert_eq!(res_refund.status(), StatusCode::OK);
    let refund_json: serde_json::Value = serde_json::from_slice(
        &axum::body::to_bytes(res_refund.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert_eq!(refund_json["document"]["doc_type"], "credit_note");
    assert_eq!(refund_json["document"]["doc_number"], "CRD-2026-0099");

    // Verify "reason" alias populated credit_reason in converted credit note notes
    assert!(
        refund_json["document"]["notes"]
            .as_str()
            .unwrap_or("")
            .contains("Full customer refund"),
        "Expected 'reason' alias to populate credit note reason in notes"
    );

    // REST /api/v1/lifecycle/convert with invalid target_type -> 400 BAD_REQUEST
    let req_bad_lc = Request::builder()
        .uri("/api/v1/lifecycle/convert")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({
                "document": inv,
                "target_type": "bogus_lifecycle_target"
            })
            .to_string(),
        ))
        .unwrap();
    let res_bad_lc = router.clone().oneshot(req_bad_lc).await.unwrap();
    assert_eq!(res_bad_lc.status(), StatusCode::BAD_REQUEST);

    // -------------------------------------------------------------------------
    // REGRESSION CHECKS FOR REVIEWER FIXES:
    // 1) normalize_and_validate() clears item.discount when unit_price < 0.0
    //    and is enforced on /api/v1/generate and /api/v1/preview
    // -------------------------------------------------------------------------
    let mut neg_with_discount: DocumentRequest = serde_json::from_value(serde_json::json!({
        "doc_type": "rent_increase_notice",
        "from": { "name": "Landlord LLC" },
        "to": { "name": "Tenant" },
        "items": [
            { "description": "New Monthly Base Rent", "quantity": 1.0, "unit_price": 2100.0, "discount": 0.0 },
            { "description": "Prior Base Rent Offset", "quantity": 1.0, "unit_price": -1950.0, "discount": 1950.0 }
        ]
    }))
    .unwrap();
    assert!(neg_with_discount.allows_negative_items());
    // Even before normalize_and_validate, LineItem::amount() ignores discount when unit_price < 0
    assert!((neg_with_discount.items[1].amount() - (-1950.0)).abs() < 0.01);
    assert!((neg_with_discount.subtotal() - 150.0).abs() < 0.01);
    assert!(neg_with_discount.normalize_and_validate().is_ok());
    assert_eq!(neg_with_discount.items[1].discount, None);

    // POST /api/v1/generate rejects negative tax_rate (-15.0) with 400 BAD_REQUEST
    let req_bad_tax = Request::builder()
        .uri("/api/v1/generate")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({
                "doc_type": "invoice",
                "tax_rate": -15.0,
                "from": { "name": "Seller" },
                "to": { "name": "Buyer" },
                "items": [{ "description": "Service", "quantity": 1.0, "unit_price": 100.0 }]
            })
            .to_string(),
        ))
        .unwrap();
    let res_bad_tax = router.clone().oneshot(req_bad_tax).await.unwrap();
    assert_eq!(res_bad_tax.status(), StatusCode::BAD_REQUEST);

    // POST /api/v1/preview rejects negative unit_price on standard invoice with 400 BAD_REQUEST
    let req_bad_neg = Request::builder()
        .uri("/api/v1/preview")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({
                "doc_type": "invoice",
                "from": { "name": "Seller" },
                "to": { "name": "Buyer" },
                "items": [{ "description": "Illicit Deduction", "quantity": 1.0, "unit_price": -50.0 }]
            })
            .to_string(),
        ))
        .unwrap();
    let res_bad_neg = router.clone().oneshot(req_bad_neg).await.unwrap();
    assert_eq!(res_bad_neg.status(), StatusCode::BAD_REQUEST);

    // Per-item tax_rate with mixed-sign line items does not tax negative deductions
    let mixed_item_tax_doc: DocumentRequest = serde_json::from_value(serde_json::json!({
        "doc_type": "property_management_statement",
        "from": { "name": "PM Co" },
        "to": { "name": "Owner" },
        "items": [
            { "description": "Taxable Maintenance Fee", "quantity": 1.0, "unit_price": 500.0, "tax_rate": 10.0 },
            { "description": "Owner Reserve Credit", "quantity": 1.0, "unit_price": -200.0 }
        ]
    }))
    .unwrap();
    assert!((mixed_item_tax_doc.tax_total() - 50.0).abs() < 0.01);

    // Address state inference with US country suffixes, non-US countries, and street names
    assert_eq!(
        pressmark::state_laws::infer_state_from_address("100 Main St, Austin, TX, US").map(|s| s.code),
        Some("TX")
    );
    assert_eq!(
        pressmark::state_laws::infer_state_from_address("100 Main St, Austin, TX, United States").map(|s| s.code),
        Some("TX")
    );
    assert_eq!(
        pressmark::state_laws::infer_state_from_address("100 Main St Chattanooga TN 37402, USA").map(|s| s.code),
        Some("TN")
    );
    assert_eq!(
        pressmark::state_laws::infer_state_from_address("10 Rue de Rivoli, Paris, ID, France")
            .map(|s| s.code),
        None
    );
    assert_eq!(
        pressmark::state_laws::infer_state_from_address("100 Main St, Ontario, CA, Canada")
            .map(|s| s.code),
        None
    );
    assert_eq!(
        pressmark::state_laws::infer_state_from_address("100 Indiana Ave").map(|s| s.code),
        None
    );

    // MCP municipality schema description preserves all 6 municipalities including Cook County
    let mun_desc = mcp_props["municipality"]["description"].as_str().unwrap_or("");
    assert!(
        mun_desc.contains("Cook County"),
        "Expected MCP municipality description to include Cook County, got: {}",
        mun_desc
    );
}



