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
    assert_eq!(call_resp.result.unwrap()["currency"], "USD");
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
        "https://pressmark.io/api/v1/auth/chatgpt/callback",
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
    let manifest = get_ai_plugin_manifest("https://pressmark.io");
    assert_eq!(manifest["auth"]["type"], "oauth");
    assert_eq!(manifest["auth"]["client_url"], "https://auth.openai.com/api/accounts/authorize");
    assert_eq!(manifest["auth"]["authorization_url"], "https://auth.openai.com/api/accounts/oauth/token");
    assert_eq!(manifest["auth"]["scope"], "openid profile email model.request");

    let spec = get_openapi_spec("https://pressmark.io");
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
    assert!(caddy_content.contains("pressmark.io"));
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

