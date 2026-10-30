//! Pressmark Model Context Protocol (MCP) Server
//!
//! Author: Daniel Hallman

use crate::engine::{generate_pdf, generate_preview_png};
use crate::models::{DocumentRequest, DocumentType};
use crate::pricing::get_pricing_catalog;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const MCP_SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SUPPORTED_MCP_PROTOCOLS: &[&str] = &[
    "2026-07-28",
    "2025-11-25",
    "2025-06-18",
    "2025-03-26",
    "2024-11-05",
];

#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: &'static str,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

impl JsonRpcResponse {
    pub fn tool_succeeded(&self) -> bool {
        self.error.is_none() && self.result.as_ref()
            .map(|result| result.get("isError").and_then(Value::as_bool) != Some(true))
            .unwrap_or(false)
    }
}

pub struct McpHandler;

impl McpHandler {
    /// Only canonical registry names become public metric keys; unknown input is never a label.
    pub fn measurement_tool_name(name: Option<&str>) -> String {
        let Some(name) = name.filter(|name| !name.is_empty()) else { return "unnamed_tool".to_string(); };
        let tools = Self::get_tools();
        tools.as_array().into_iter().flatten()
            .filter_map(|tool| tool.get("name").and_then(Value::as_str))
            .find(|known| *known == name)
            .unwrap_or("unknown_tool")
            .to_string()
    }

    pub fn is_notification(req: &JsonRpcRequest) -> bool {
        req.id.is_none() || req.method.starts_with("notifications/")
    }

    pub fn document_type_enum() -> Value {
        Value::Array(
            DocumentType::all()
                .iter()
                .map(|dt| Value::String(dt.as_str().to_string()))
                .collect(),
        )
    }

    fn document_input_properties() -> Value {
        json!({
            "doc_type": { "type": "string", "description": "Archetype slug (invoice, receipt, residential_lease, nda, etc.)." },
            "doc_number": { "type": "string", "description": "Document number. Left blank when omitted." },
            "issue_date": { "type": "string", "description": "Issue date (YYYY-MM-DD)." },
            "due_date": { "type": "string", "description": "Payment due date (YYYY-MM-DD)." },
            "state_code": { "type": "string", "description": "2-letter US state or DC code." },
            "municipality": { "type": "string", "description": "Municipal profile (Chicago, Cook County, NYC, SF, Seattle, Portland)." },
            "county": { "type": "string", "description": "County name for jurats and headers." },
            "recordable": { "type": "boolean", "description": "Enable County Recorder 3-inch top margin." },
            "notary_required": { "type": "boolean", "description": "Add blank section for notary completion." },
            "execution_mode": { "type": "string", "description": "Signature layout (esign, wet_ink, notarized_wet, ron)." },
            "compliance_mode": { "type": "string", "description": "Compliance topic note (none, eu_vat, peppol_bis, hipaa_baa)." },
            "date_format": { "type": "string", "description": "Date format (iso, us_slash, euro, formal_prose)." },
            "currency_symbol_mode": { "type": "string", "description": "Display currency symbol or ISO code." },
            "currency_placement": { "type": "string", "description": "Currency prefix or suffix placement." },
            "currency_separators": { "type": "string", "description": "Number separators (comma_dot, dot_comma)." },
            "number_decimals": { "type": "integer", "description": "Decimal places for amounts (default 2)." },
            "from": {
                "type": "object",
                "description": "Issuer or landlord details.",
                "properties": {
                    "name": { "type": "string", "description": "Issuer name." },
                    "email": { "type": "string", "description": "Issuer email." },
                    "address": { "type": "string", "description": "Issuer address." }
                },
                "required": ["name"]
            },
            "to": {
                "type": "object",
                "description": "Recipient or tenant details.",
                "properties": {
                    "name": { "type": "string", "description": "Recipient name." },
                    "email": { "type": "string", "description": "Recipient email." },
                    "address": { "type": "string", "description": "Recipient address." }
                },
                "required": ["name"]
            },
            "items": {
                "type": "array",
                "description": "Line items or rent schedule.",
                "items": {
                    "type": "object",
                    "properties": {
                        "description": { "type": "string", "description": "Item description." },
                        "quantity": { "type": "number", "description": "Item quantity." },
                        "unit_price": { "type": "number", "description": "Unit price." }
                    },
                    "required": ["description", "quantity", "unit_price"]
                }
            },
            "currency": { "type": "string", "default": "USD", "description": "ISO 4217 currency code." },
            "tax_rate": { "type": "number", "description": "Percentage tax rate." },
            "payment_link": { "type": "string", "description": "HTTPS checkout URL for QR code." },
            "payment_qr_code": { "type": "boolean", "description": "Embed vector SVG QR code." },
            "custom_fields": { "type": "array", "description": "Custom key-value metadata fields." },
            "sections": { "type": "array", "description": "Structured legal clause sections." },
            "signatures": {
                "type": "array", "description": "Signature fields with signer names, dates, and optional signature images.",
                "items": {
                    "type": "object",
                    "properties": {
                        "signer_name": { "type": "string" },
                        "signer_title": { "type": "string" },
                        "date": { "type": "string", "description": "Signing date." },
                        "signature_data": { "type": "string", "description": "Base64 signature image data URI." }
                    },
                    "required": ["signer_name"]
                }
            }
        })
    }

    fn preview_input_properties() -> Value {
        Self::document_input_properties()
    }

    fn download_output_schema() -> Value {
        json!({
            "type": "object", "description": "Temporary file link for HTTP tool calls.",
            "properties": {
                "url": { "type": "string", "format": "uri" },
                "filename": { "type": "string" },
                "expires_at": { "type": "string", "format": "date-time" },
                "expires_in_seconds": { "type": "integer" }
            },
            "required": ["url", "filename", "expires_at", "expires_in_seconds"]
        })
    }

    pub fn tools_list() -> Value {
        Self::get_tools()
    }

    pub fn get_tools() -> Value {
        let doc_props = Self::document_input_properties();
        let preview_props = Self::preview_input_properties();
        json!([
            {
                "name": "generate_document",
                "title": "Generate PDF Document",
                "description": "Generate a vector PDF from document details. HTTP calls can return a temporary download link. Configured customer sync can save contact details and start follow-up. See https://pressmark.dev/privacy.",
                "annotations": {
                    "title": "Generate PDF Document",
                    "readOnlyHint": false,
                    "destructiveHint": false,
                    "idempotentHint": false,
                    "openWorldHint": true
                },
                "inputSchema": {
                    "type": "object",
                    "properties": doc_props,
                    "required": ["doc_type", "from", "to"],
                    "examples": [
                        {
                            "doc_type": "invoice",
                            "from": { "name": "Acme Corp" },
                            "to": { "name": "Client LLC" },
                            "items": [{ "description": "Design retainer", "quantity": 1, "unit_price": 1500 }]
                        }
                    ]
                },
                "outputSchema": {
                    "type": "object",
                    "properties": {
                        "pdf_base64": { "type": "string", "description": "Base64 vector PDF." },
                        "download": Self::download_output_schema(),
                        "download_error": { "type": "string", "description": "Why a temporary download link could not be created. The embedded PDF is still available." },
                        "doc_number": { "type": "string", "description": "User-supplied document number, or an empty string when omitted." },
                        "grand_total": { "type": "number", "description": "Computed grand total." },
                        "compile_time_ms": { "type": "integer", "description": "Compile time in ms." }
                    },
                    "required": ["pdf_base64", "doc_number", "grand_total", "compile_time_ms"]
                }
            },
            {
                "name": "preview_document",
                "title": "Preview Document as PNG",
                "description": "Render a PNG preview from supplied document details. HTTP calls can return a temporary image link.",
                "annotations": {
                    "title": "Preview Document as PNG",
                    "readOnlyHint": false,
                    "destructiveHint": false,
                    "idempotentHint": false,
                    "openWorldHint": false
                },
                "inputSchema": {
                    "type": "object",
                    "properties": preview_props,
                    "required": ["doc_type", "from", "to"],
                    "examples": [
                        {
                            "doc_type": "invoice",
                            "from": { "name": "Acme Corp" },
                            "to": { "name": "Client LLC" }
                        }
                    ]
                },
                "outputSchema": {
                    "type": "object",
                    "properties": {
                        "image_data_uri": { "type": "string", "description": "Base64 PNG data URI." },
                        "preview": Self::download_output_schema(),
                        "preview_error": { "type": "string", "description": "Why a temporary preview link could not be created. The PNG remains in the tool result." },
                        "compile_time_ms": { "type": "integer", "description": "Compile time in ms." }
                    },
                    "required": ["image_data_uri", "compile_time_ms"]
                }
            },
            {
                "name": "calculate_totals",
                "title": "Calculate Document Totals",
                "description": "Compute subtotal, tax, discount, grand total, and balance due from document items. Stateless.",
                "annotations": {
                    "title": "Calculate Document Totals",
                    "readOnlyHint": true,
                    "destructiveHint": false,
                    "idempotentHint": true,
                    "openWorldHint": false
                },
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "document": {
                            "type": "object",
                            "description": "Document JSON with items, tax_rate, discount_amount, and optional amount_paid."
                        }
                    },
                    "required": ["document"],
                    "examples": [
                        { "document": { "items": [{ "description": "Service", "quantity": 2, "unit_price": 50 }] } }
                    ]
                },
                "outputSchema": {
                    "type": "object",
                    "properties": {
                        "subtotal": { "type": "number", "description": "Subtotal before tax/discount." },
                        "tax_total": { "type": "number", "description": "Tax amount." },
                        "discount_amount": { "type": "number", "description": "Discount applied." },
                        "grand_total": { "type": "number", "description": "Payable grand total." },
                        "amount_paid": { "type": "number", "description": "Amount paid." },
                        "balance_due": { "type": "number", "description": "Remaining balance due." },
                        "is_paid": { "type": "boolean", "description": "True when paid in full." }
                    },
                    "required": ["subtotal", "tax_total", "discount_amount", "grand_total", "amount_paid", "balance_due", "is_paid"]
                }
            },
            {
                "name": "get_pricing",
                "title": "Get Pressmark Pricing Catalog",
                "description": "Read subscription tiers, single-pass unlocks, and checkout links. Stateless.",
                "annotations": {
                    "title": "Get Pressmark Pricing Catalog",
                    "readOnlyHint": true,
                    "destructiveHint": false,
                    "idempotentHint": true,
                    "openWorldHint": false
                },
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "examples": [{}]
                },
                "outputSchema": {
                    "type": "object",
                    "properties": {
                        "currency": { "type": "string", "description": "Catalog currency (USD)." },
                        "plans": { "type": "array", "description": "Subscription tiers." },
                        "microtransactions": { "type": "array", "description": "Single-pass unlocks." }
                    },
                    "required": ["currency", "plans", "microtransactions"]
                }
            },
            {
                "name": "convert_document",
                "title": "Convert Document",
                "description": "Convert document details to an invoice, estimate, receipt, credit note, or refund PDF. HTTP calls can return a temporary download link.",
                "annotations": {
                    "title": "Convert Document",
                    "readOnlyHint": false,
                    "destructiveHint": false,
                    "idempotentHint": false,
                    "openWorldHint": false
                },
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "document": { "type": "object", "description": "Stateless input document JSON payload to transform." },
                        "target_type": { "type": "string", "enum": ["invoice", "receipt", "estimate", "credit_note", "refund"], "description": "Target type (invoice, receipt, estimate, credit_note, refund)." },
                        "due_days": { "type": "integer", "description": "Days until payment due." },
                        "payment_link": { "type": "string", "description": "HTTPS checkout URL for QR code." },
                        "payment_qr_code": { "type": "boolean", "description": "Embed SVG vector QR code." },
                        "credit_reason": { "type": "string", "description": "Credit note or refund reason." }
                    },
                    "required": ["document", "target_type"],
                    "examples": [
                        { "document": { "doc_type": "estimate", "from": { "name": "Acme" }, "to": { "name": "Client" } }, "target_type": "invoice" }
                    ]
                },
                "outputSchema": {
                    "type": "object",
                    "properties": {
                        "document": { "type": "object", "description": "Converted document JSON." },
                        "pdf_base64": { "type": "string", "description": "Base64 vector PDF." },
                        "download": Self::download_output_schema(),
                        "download_error": { "type": "string", "description": "Why a temporary download link could not be created. The embedded PDF is still available." },
                        "compile_time_ms": { "type": "integer", "description": "Compile time in ms." }
                    },
                    "required": ["document", "pdf_base64", "compile_time_ms"]
                }
            },
            {
                "name": "record_payment",
                "title": "Create Payment Record",
                "description": "Create a receipt or updated invoice from supplied payment details. Does not charge or verify payment. HTTP calls can return a temporary download link.",
                "annotations": {
                    "title": "Create Payment Record",
                    "readOnlyHint": false,
                    "destructiveHint": false,
                    "idempotentHint": false,
                    "openWorldHint": false
                },
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "document": { "type": "object", "description": "Stateless input invoice JSON payload." },
                        "payment_method": { "type": "string", "description": "Payment method (Stripe, Apple Pay, Wire)." },
                        "transaction_reference": { "type": "string", "description": "Transaction reference ID." },
                        "paid_date": { "type": "string", "description": "Payment date (YYYY-MM-DD)." },
                        "amount_paid": { "type": "number", "description": "Amount paid toward balance." }
                    },
                    "required": ["document"],
                    "examples": [
                        { "document": { "doc_type": "invoice", "from": { "name": "Acme" }, "to": { "name": "Client" } }, "amount_paid": 100 }
                    ]
                },
                "outputSchema": {
                    "type": "object",
                    "properties": {
                        "document": { "type": "object", "description": "Receipt or updated invoice JSON." },
                        "pdf_base64": { "type": "string", "description": "Base64 vector PDF." },
                        "download": Self::download_output_schema(),
                        "download_error": { "type": "string", "description": "Why a temporary download link could not be created. The embedded PDF is still available." },
                        "compile_time_ms": { "type": "integer", "description": "Compile time in ms." }
                    },
                    "required": ["document", "pdf_base64", "compile_time_ms"]
                }
            }
        ])
    }


    pub fn validate_document_request(doc_req: &DocumentRequest) -> Result<(), String> {
        if doc_req.from.name.trim().is_empty() || doc_req.to.name.trim().is_empty() {
            return Err("Validation error: both 'from.name' (seller/issuer name) and 'to.name' (buyer/recipient name) are required.".to_string());
        }
        let mut cloned = doc_req.clone();
        cloned.normalize_and_validate()
    }

    pub fn handle_request(req: JsonRpcRequest) -> JsonRpcResponse {
        Self::handle_request_with_context(req, None, false, None)
    }

    pub fn handle_request_with_context(
        req: JsonRpcRequest,
        storage: Option<&crate::storage::Storage>,
        is_paid_session: bool,
        session_email: Option<&str>,
    ) -> JsonRpcResponse {
        let is_tool_call = req.method == "tools/call";
        let mut response = match req.method.as_str() {
            "initialize" => {
                let requested = req
                    .params
                    .as_ref()
                    .and_then(|p| p.get("protocolVersion"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("2026-07-28");
                let negotiated = if SUPPORTED_MCP_PROTOCOLS.contains(&requested) {
                    requested
                } else {
                    "2026-07-28"
                };
                JsonRpcResponse {
                    jsonrpc: "2.0",
                    id: req.id,
                    result: Some(json!({
                        "protocolVersion": negotiated,
                        "capabilities": {
                            "tools": { "listChanged": false },
                            "prompts": { "listChanged": false },
                            "resources": { "subscribe": false, "listChanged": false }
                        },
                        "serverInfo": {
                            "name": "pressmark",
                            "title": "Pressmark Vector Document Engine",
                            "version": MCP_SERVER_VERSION
                        }
                    })),
                    error: None,
                }
            }
            "server/discover" => JsonRpcResponse {
                jsonrpc: "2.0",
                id: req.id,
                result: Some(json!({
                    "resultType": "complete",
                    "supportedVersions": SUPPORTED_MCP_PROTOCOLS,
                    "protocolVersion": "2026-07-28",
                    "protocolVersions": SUPPORTED_MCP_PROTOCOLS,
                    "supportedProtocolVersions": SUPPORTED_MCP_PROTOCOLS,
                    "capabilities": {
                        "tools": { "listChanged": false },
                        "prompts": { "listChanged": false },
                        "resources": { "subscribe": false, "listChanged": false }
                    },
                    "serverInfo": {
                        "name": "pressmark",
                        "title": "Pressmark Vector Document Engine",
                        "version": MCP_SERVER_VERSION
                    },
                    "ttlMs": 300_000,
                    "cacheScope": "public",
                    "_meta": {
                        "io.modelcontextprotocol/serverInfo": {
                            "name": "pressmark",
                            "title": "Pressmark Vector Document Engine",
                            "version": MCP_SERVER_VERSION
                        },
                        "protocolVersion": "2026-07-28",
                        "streamingSupported": true,
                        "cacheTtlMs": 300_000
                    }
                })),
                error: None,
            },
            "ping" => JsonRpcResponse {
                jsonrpc: "2.0",
                id: req.id,
                result: Some(json!({})),
                error: None,
            },
            "notifications/initialized" | "notifications/cancelled" => JsonRpcResponse {
                jsonrpc: "2.0",
                id: req.id,
                result: Some(json!({})),
                error: None,
            },
            "prompts/list" => JsonRpcResponse {
                jsonrpc: "2.0",
                id: req.id,
                result: Some(json!({ "prompts": [] })),
                error: None,
            },
            "resources/list" => JsonRpcResponse {
                jsonrpc: "2.0",
                id: req.id,
                result: Some(json!({
                    "resources": [
                        {
                            "uri": "pressmark://catalog/archetypes",
                            "name": "Document Archetypes Catalog",
                            "description": "All 57 supported document archetypes.",
                            "mimeType": "application/json"
                        },
                        {
                            "uri": "pressmark://catalog/states",
                            "name": "US State Statutory Lease Profiles",
                            "description": "50-state, DC, and municipal statutory lease profiles.",
                            "mimeType": "application/json"
                        },
                        {
                            "uri": "pressmark://catalog/pricing",
                            "name": "Pricing & Unlock Tiers",
                            "description": "Subscription plans and document unlock pricing.",
                            "mimeType": "application/json"
                        },
                        {
                            "uri": "pressmark://catalog/themes",
                            "name": "Visual Theme Presets",
                            "description": "Visual appearance presets and document colors.",
                            "mimeType": "application/json"
                        },
                        {
                            "uri": "pressmark://catalog/compliance",
                            "name": "Compliance Notes",
                            "description": "Compliance topic notes and disclosures.",
                            "mimeType": "application/json"
                        },
                        {
                            "uri": "pressmark://catalog/execution",
                            "name": "Signature Layouts",
                            "description": "Signature layout templates.",
                            "mimeType": "application/json"
                        }
                    ]
                })),
                error: None,
            },
            "resources/read" => {
                let uri = req
                    .params
                    .as_ref()
                    .and_then(|p| p.get("uri"))
                    .and_then(|u| u.as_str())
                    .unwrap_or("");
                let text = match uri {
                    "pressmark://catalog/states" => {
                        serde_json::to_string(&crate::state_laws::all_state_lease_profiles())
                            .unwrap_or_default()
                    }
                    "pressmark://catalog/pricing" => {
                        serde_json::to_string(&get_pricing_catalog()).unwrap_or_default()
                    }
                    "pressmark://catalog/themes" => json!({
                        "studio_url": "https://pressmark.dev",
                        "description": "Choose appearance presets in the Studio. Use the document preview to check colors before exporting."
                    }).to_string(),
                    "pressmark://catalog/compliance" => Self::document_input_properties()["compliance_mode"].to_string(),
                    "pressmark://catalog/execution" => Self::document_input_properties()["execution_mode"].to_string(),
                    "pressmark://catalog/archetypes" => serde_json::to_string(&Self::document_type_enum()).unwrap_or_default(),
                    _ => return JsonRpcResponse {
                        jsonrpc: "2.0",
                        id: req.id,
                        result: None,
                        error: Some(json!({"code": -32002, "message": "Resource not found."})),
                    },
                };
                JsonRpcResponse {
                    jsonrpc: "2.0",
                    id: req.id,
                    result: Some(json!({
                        "contents": [
                            {
                                "uri": uri,
                                "mimeType": "application/json",
                                "text": text
                            }
                        ]
                    })),
                    error: None,
                }
            },
            "resources/templates/list" => JsonRpcResponse {
                jsonrpc: "2.0",
                id: req.id,
                result: Some(json!({ "resourceTemplates": [] })),
                error: None,
            },
            "tools/list" => JsonRpcResponse {
                jsonrpc: "2.0",
                id: req.id,
                result: Some(json!({ "tools": Self::get_tools() })),
                error: None,
            },
            "tools/call" => {
                let params = req.params.unwrap_or(Value::Null);
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let arguments = params.get("arguments").cloned().unwrap_or(Value::Null);

                match tool_name {
                    "generate_document" => {
                        match serde_json::from_value::<DocumentRequest>(arguments) {
                            Ok(mut doc_req) => {
                                if let Err(msg) = Self::validate_document_request(&doc_req)
                                    .and_then(|()| doc_req.normalize_and_validate())
                                {
                                    return JsonRpcResponse {
                                        jsonrpc: "2.0",
                                        id: req.id,
                                        result: None,
                                        error: Some(json!({ "code": -32602, "message": msg })),
                                    };
                                }
                                match generate_pdf(&doc_req) {
                                    Ok((pdf_bytes, ms)) => {
                                        if crate::ghl::GhlDispatcher::has_env_or_pit_file() {
                                            let candidate_email = session_email
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
                                                .map(str::to_string);
                                            if let Some(email_cl) = candidate_email {
                                                let is_paid = is_paid_session
                                                    || match storage {
                                                        Some(st) => st
                                                            .get_customer_entitlement(&email_cl)
                                                            .is_some(),
                                                        None => crate::storage::Storage::default_db()
                                                            .get_customer_entitlement(&email_cl)
                                                            .is_some(),
                                                    };
                                                let mut doc_cl = doc_req.clone();
                                                doc_cl.apply_free_tier_ghl_watermark(is_paid);
                                                if let Ok(rt) = tokio::runtime::Handle::try_current() {
                                                    rt.spawn(async move {
                                                        let ghl = crate::ghl::GhlDispatcher::from_env();
                                                        if ghl.is_configured() {
                                                            let _ = ghl
                                                                .dispatch_document_generation(
                                                                    &doc_cl,
                                                                    Some(&email_cl),
                                                                    "mcp_agent",
                                                                )
                                                                .await;
                                                        }
                                                    });
                                                }
                                            }
                                        }
                                        let b64 = BASE64.encode(&pdf_bytes);
                                        JsonRpcResponse {
                                            jsonrpc: "2.0",
                                            id: req.id,
                                            result: Some(json!({
                                                "content": [
                                                    {
                                                        "type": "text",
                                                        "text": format_document_markdown(&doc_req, "Generated", ms)
                                                    },
                                                    pdf_resource(&pdf_bytes, &b64)
                                                ],
                                                "pdf_base64": b64,
                                                "doc_number": doc_req.effective_doc_number(),
                                                "grand_total": doc_req.grand_total(),
                                                "compile_time_ms": ms,
                                                "unlock_checkout_url": "https://pressmark.dev/api/v1/checkout?plan=pass_single_unlock&redirect=true",
                                                "pro_checkout_url": "https://pressmark.dev/api/v1/checkout?plan=plan_solo&redirect=true"
                                            })),
                                            error: None,
                                        }
                                    }
                                    Err(err) => JsonRpcResponse {
                                        jsonrpc: "2.0",
                                        id: req.id,
                                        result: None,
                                        error: Some(json!({ "code": -32000, "message": err })),
                                    },
                                }
                            }
                            Err(e) => JsonRpcResponse {
                                jsonrpc: "2.0",
                                id: req.id,
                                result: None,
                                error: Some(json!({ "code": -32602, "message": format!("Invalid arguments: {}", e) })),
                            },
                        }
                    }
                    "preview_document" => {
                        match serde_json::from_value::<DocumentRequest>(arguments) {
                            Ok(mut doc_req) => {
                                if let Err(msg) = Self::validate_document_request(&doc_req)
                                    .and_then(|()| doc_req.normalize_and_validate())
                                {
                                    return JsonRpcResponse {
                                        jsonrpc: "2.0",
                                        id: req.id,
                                        result: None,
                                        error: Some(json!({ "code": -32602, "message": msg })),
                                    };
                                }
                                match generate_preview_png(&doc_req, 144) {
                                    Ok((png_bytes, ms)) => {
                                        let b64 = BASE64.encode(&png_bytes);
                                        let uri = format!("data:image/png;base64,{}", b64);
                                        JsonRpcResponse {
                                            jsonrpc: "2.0",
                                            id: req.id,
                                            result: Some(json!({
                                                "content": [
                                                    {
                                                        "type": "image",
                                                        "data": b64,
                                                        "mimeType": "image/png"
                                                    }
                                                ],
                                                "image_data_uri": uri,
                                                "compile_time_ms": ms,
                                                "unlock_checkout_url": "https://pressmark.dev/api/v1/checkout?plan=pass_single_unlock&redirect=true",
                                                "pro_checkout_url": "https://pressmark.dev/api/v1/checkout?plan=plan_solo&redirect=true"
                                            })),
                                            error: None,
                                        }
                                    }
                                    Err(err) => JsonRpcResponse {
                                        jsonrpc: "2.0",
                                        id: req.id,
                                        result: None,
                                        error: Some(json!({ "code": -32000, "message": err })),
                                    },
                                }
                            }
                            Err(e) => JsonRpcResponse {
                                jsonrpc: "2.0",
                                id: req.id,
                                result: None,
                                error: Some(json!({ "code": -32602, "message": format!("Invalid arguments: {}", e) })),
                            },
                        }
                    }
                    "get_pricing" => {
                        let catalog = get_pricing_catalog();
                        let mut val = serde_json::to_value(&catalog).unwrap_or_else(|_| json!({}));
                        if let Some(obj) = val.as_object_mut() {
                            obj.insert(
                                "content".to_string(),
                                json!([
                                    {
                                        "type": "text",
                                        "text": serde_json::to_string_pretty(&catalog).unwrap_or_default()
                                    }
                                ]),
                            );
                        }
                        JsonRpcResponse {
                            jsonrpc: "2.0",
                            id: req.id,
                            result: Some(val),
                            error: None,
                        }
                    }
                    "convert_document" => {
                        let target_type = arguments.get("target_type").and_then(|t| t.as_str()).unwrap_or("");
                        let due_days = arguments.get("due_days").and_then(|d| d.as_u64()).unwrap_or(15) as u32;
                        let payment_link = arguments.get("payment_link").and_then(|l| l.as_str());
                        let payment_qr_code = arguments.get("payment_qr_code").and_then(|q| q.as_bool());
                        let credit_reason = arguments
                            .get("credit_reason")
                            .or_else(|| arguments.get("reason"))
                            .and_then(|r| r.as_str());
                        let doc_val = arguments.get("document").cloned().unwrap_or(Value::Null);

                        match serde_json::from_value::<DocumentRequest>(doc_val) {
                            Ok(mut doc_req) => {
                                if let Err(msg) = Self::validate_document_request(&doc_req)
                                    .and_then(|()| doc_req.normalize_and_validate())
                                {
                                    return JsonRpcResponse {
                                        jsonrpc: "2.0",
                                        id: req.id,
                                        result: None,
                                        error: Some(json!({ "code": -32602, "message": msg })),
                                    };
                                }
                                let mut converted = match target_type.trim().to_lowercase().as_str() {
                                    "credit_note" | "credit" | "refund" => doc_req.to_credit_note(credit_reason),
                                    "receipt" => doc_req.to_receipt(
                                        doc_req.payment_method.as_deref().unwrap_or_default(),
                                        doc_req.transaction_reference.as_deref().unwrap_or_default(),
                                        doc_req.paid_at.as_deref().unwrap_or_default(),
                                    ),
                                    "estimate" | "quote" => doc_req.to_estimate(30),
                                    "invoice" => doc_req.to_invoice(due_days),
                                    other => {
                                        return JsonRpcResponse {
                                            jsonrpc: "2.0",
                                            id: req.id,
                                            result: None,
                                            error: Some(json!({
                                                "code": -32602,
                                                "message": format!(
                                                    "Unsupported target_type '{}'. Supported lifecycle target types are: invoice, receipt, estimate, credit_note.",
                                                    other
                                                )
                                            })),
                                        };
                                    }
                                };
                                if let Some(link) = payment_link {
                                    converted.payment_link = Some(link.to_string());
                                    converted.payment_qr_code = Some(payment_qr_code.unwrap_or(true));
                                } else if let Some(qr) = payment_qr_code {
                                    converted.payment_qr_code = Some(qr);
                                }
                                match generate_pdf(&converted) {
                                    Ok((pdf_bytes, ms)) => {
                                        let b64 = BASE64.encode(&pdf_bytes);
                                        JsonRpcResponse {
                                            jsonrpc: "2.0",
                                            id: req.id,
                                            result: Some(json!({
                                                "content": [
                                                    {
                                                        "type": "text",
                                                        "text": format_document_markdown(&converted, "Document Converted to", ms)
                                                    },
                                                    pdf_resource(&pdf_bytes, &b64)
                                                ],
                                                "document": converted,
                                                "pdf_base64": b64,
                                                "compile_time_ms": ms,
                                                "unlock_checkout_url": "https://pressmark.dev/api/v1/checkout?plan=pass_single_unlock&redirect=true",
                                                "pro_checkout_url": "https://pressmark.dev/api/v1/checkout?plan=plan_solo&redirect=true"
                                            })),
                                            error: None,
                                        }
                                    }
                                    Err(err) => JsonRpcResponse {
                                        jsonrpc: "2.0",
                                        id: req.id,
                                        result: None,
                                        error: Some(json!({ "code": -32000, "message": err })),
                                    },
                                }
                            }
                            Err(e) => JsonRpcResponse {
                                jsonrpc: "2.0",
                                id: req.id,
                                result: None,
                                error: Some(json!({ "code": -32602, "message": format!("Invalid document payload: {}", e) })),
                            },
                        }
                    }
                    "calculate_totals" => {
                        let doc_val = if arguments.get("document").is_some() {
                            arguments.get("document").cloned().unwrap_or(Value::Null)
                        } else {
                            arguments.clone()
                        };
                        match serde_json::from_value::<DocumentRequest>(doc_val) {
                            Ok(mut doc_req) => {
                                if let Err(msg) = doc_req.normalize_and_validate() {
                                    return JsonRpcResponse {
                                        jsonrpc: "2.0",
                                        id: req.id,
                                        result: None,
                                        error: Some(json!({ "code": -32602, "message": msg })),
                                    };
                                }
                                let subtotal = doc_req.subtotal();
                                let tax_total = doc_req.tax_total();
                                let discount = doc_req.discount_amount.unwrap_or(0.0);
                                let grand_total = doc_req.grand_total();
                                let balance_due = doc_req.balance_due();
                                let amount_paid = doc_req.amount_paid_effective();
                                let is_paid = doc_req.is_paid_state();

                                JsonRpcResponse {
                                    jsonrpc: "2.0",
                                    id: req.id,
                                    result: Some(json!({
                                        "content": [
                                            {
                                                "type": "text",
                                                "text": format!(
                                                    "**Document Totals Calculation:**\n- Subtotal: {}\n- Tax: {}\n- Discount: {}\n- **Grand Total:** {}\n- Amount Paid: {}\n- **Balance Due:** {}\n- Status: {}",
                                                    doc_req.format_amount(subtotal),
                                                    doc_req.format_amount(tax_total),
                                                    doc_req.format_amount(discount),
                                                    doc_req.format_amount(grand_total),
                                                    doc_req.format_amount(amount_paid),
                                                    doc_req.format_amount(balance_due),
                                                    doc_req.effective_payment_status().to_ascii_uppercase()
                                                )
                                            }
                                        ],
                                        "subtotal": subtotal,
                                        "tax_total": tax_total,
                                        "discount_amount": discount,
                                        "grand_total": grand_total,
                                        "amount_paid": amount_paid,
                                        "balance_due": balance_due,
                                        "is_paid": is_paid
                                    })),
                                    error: None,
                                }
                            }
                            Err(e) => JsonRpcResponse {
                                jsonrpc: "2.0",
                                id: req.id,
                                result: None,
                                error: Some(json!({ "code": -32602, "message": format!("Invalid document payload: {}", e) })),
                            },
                        }
                    }
                    "record_payment" => {
                        let doc_val = arguments.get("document").cloned().unwrap_or(Value::Null);
                        let method = arguments.get("payment_method").and_then(|m| m.as_str()).unwrap_or_default();
                        let tx_ref = arguments.get("transaction_reference").and_then(|r| r.as_str()).unwrap_or_default();
                        let paid_date = arguments.get("paid_date").and_then(|d| d.as_str()).unwrap_or_default();
                        let amount_paid = arguments.get("amount_paid").and_then(|a| a.as_f64());

                        if let Some(paid) = amount_paid {
                            if paid < 0.0 {
                                return JsonRpcResponse {
                                    jsonrpc: "2.0",
                                    id: req.id,
                                    result: None,
                                    error: Some(json!({
                                        "code": -32602,
                                        "message": "Validation error: amount_paid cannot be negative. Use convert_document with target_type='credit_note' to prepare a credit note; this does not send a refund."
                                    })),
                                };
                            }
                        }

                        match serde_json::from_value::<DocumentRequest>(doc_val) {
                            Ok(mut doc_req) => {
                                if let Err(msg) = Self::validate_document_request(&doc_req)
                                    .and_then(|()| doc_req.normalize_and_validate())
                                {
                                    return JsonRpcResponse {
                                        jsonrpc: "2.0",
                                        id: req.id,
                                        result: None,
                                        error: Some(json!({ "code": -32602, "message": msg })),
                                    };
                                }
                                let grand_total = doc_req.grand_total();
                                let (payment_doc, action_label) = if let Some(paid) = amount_paid {
                                    if paid < grand_total - 0.001 {
                                        let mut partial_doc = doc_req.clone();
                                        partial_doc.amount_paid = Some(paid);
                                        partial_doc.is_paid = Some(false);
                                        partial_doc.payment_status = Some("partial".to_string());
                                        partial_doc.payment_method = (!method.trim().is_empty()).then(|| method.to_string());
                                        partial_doc.transaction_reference = (!tx_ref.trim().is_empty()).then(|| tx_ref.to_string());
                                        partial_doc.paid_at = (!paid_date.trim().is_empty()).then(|| paid_date.to_string());
                                        (partial_doc, "Partial Payment Recorded & Invoice Updated")
                                    } else {
                                        let mut receipt = doc_req.to_receipt(method, tx_ref, paid_date);
                                        receipt.amount_paid = Some(paid);
                                        (receipt, "Payment In Full Recorded & Receipt Generated")
                                    }
                                } else {
                                    (doc_req.to_receipt(method, tx_ref, paid_date), "Payment Recorded & Generated")
                                };

                                match generate_pdf(&payment_doc) {
                                    Ok((pdf_bytes, ms)) => {
                                        let b64 = BASE64.encode(&pdf_bytes);
                                        JsonRpcResponse {
                                            jsonrpc: "2.0",
                                            id: req.id,
                                            result: Some(json!({
                                                "content": [
                                                    {
                                                        "type": "text",
                                                        "text": format_document_markdown(&payment_doc, action_label, ms)
                                                    },
                                                    pdf_resource(&pdf_bytes, &b64)
                                                ],
                                                "document": payment_doc,
                                                "pdf_base64": b64,
                                                "compile_time_ms": ms,
                                                "unlock_checkout_url": "https://pressmark.dev/api/v1/checkout?plan=pass_single_unlock&redirect=true",
                                                "pro_checkout_url": "https://pressmark.dev/api/v1/checkout?plan=plan_solo&redirect=true"
                                            })),
                                            error: None,
                                        }
                                    }
                                    Err(err) => JsonRpcResponse {
                                        jsonrpc: "2.0",
                                        id: req.id,
                                        result: None,
                                        error: Some(json!({ "code": -32000, "message": err })),
                                    },
                                }
                            }
                            Err(e) => JsonRpcResponse {
                                jsonrpc: "2.0",
                                id: req.id,
                                result: None,
                                error: Some(json!({ "code": -32602, "message": format!("Invalid document payload: {}", e) })),
                            },
                        }
                    }
                    _ => JsonRpcResponse {
                        jsonrpc: "2.0",
                        id: req.id,
                        result: None,
                        error: Some(json!({ "code": -32601, "message": format!("Tool '{}' not found", tool_name) })),
                    },
                }
            }
            m if m.starts_with("notifications/") => JsonRpcResponse {
                jsonrpc: "2.0",
                id: req.id,
                result: Some(json!({})),
                error: None,
            },
            _ => JsonRpcResponse {
                jsonrpc: "2.0",
                id: req.id,
                result: None,
                error: Some(json!({ "code": -32601, "message": "Method not found" })),
            },
        };
        if is_tool_call {
            if let Some(result) = response.result.as_mut().and_then(Value::as_object_mut) {
                let mut structured = result.clone();
                structured.remove("content");
                structured.remove("isError");
                result.insert("structuredContent".to_string(), Value::Object(structured));
            }
        }
        response
    }
}

fn pdf_resource(pdf_bytes: &[u8], base64_pdf: &str) -> Value {
    let digest = hex::encode(Sha256::digest(pdf_bytes));
    json!({
        "type": "resource",
        "resource": {
            "uri": format!("pressmark://documents/{digest}.pdf"),
            "mimeType": "application/pdf",
            "blob": base64_pdf
        }
    })
}

fn format_document_markdown(doc: &DocumentRequest, action_title: &str, _compile_ms: u128) -> String {
    let status_badge = match doc.effective_payment_status() {
        "paid" => "**PAID IN FULL** ✅",
        "partial" => "**PARTIALLY PAID** 💳",
        "refunded" => "**REFUNDED**",
        _ => "**PAYMENT PENDING** ⏳",
    };

    let issue_date_str = doc
        .issue_date
        .as_deref()
        .map(|d| doc.format_date(d))
        .unwrap_or_else(|| "Not provided".to_string());
    let doc_number = doc.doc_number.as_deref().filter(|value| !value.trim().is_empty())
        .unwrap_or("Not provided");

    let mut md = format!(
        "### {} {}\n\n**Document #:** `{}` | **Date:** `{}` | **Status:** {}\n\n",
        action_title,
        doc.effective_title(),
        doc_number,
        issue_date_str,
        status_badge
    );
    md.push_str(&format!("- **From:** {} ({})\n", doc.from.name, doc.from.company.as_deref().unwrap_or("-")));
    md.push_str(&format!("- **To:** {} ({})\n\n", doc.to.name, doc.to.company.as_deref().unwrap_or("-")));

    if !doc.items.is_empty() {
        md.push_str("| Description | Qty | Rate | Amount |\n|---|---:|---:|---:|\n");
        for item in &doc.items {
            md.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                item.description,
                item.quantity,
                doc.format_amount(item.unit_price),
                doc.format_amount(item.amount())
            ));
        }
        md.push('\n');
    }

    md.push_str(&format!("- **Subtotal:** {}\n", doc.format_amount(doc.subtotal())));
    if let Some(rate) = doc.tax_rate {
        if rate > 0.0 {
            md.push_str(&format!("- **Tax ({:.1}%):** {}\n", rate, doc.format_amount(doc.tax_total())));
        }
    }
    md.push_str(&format!("- **Grand Total:** {}\n", doc.format_amount(doc.grand_total())));
    if doc.amount_paid.is_some() || doc.is_paid_state() {
        md.push_str(&format!("- **Amount Paid:** {}\n", doc.format_amount(doc.amount_paid_effective())));
        md.push_str(&format!("- **Balance Due:** {}\n", doc.format_amount(doc.balance_due())));
    }

    if let Some(link) = &doc.payment_link {
        md.push_str(&format!("\n[Pay online]({})\n", link));
        if doc.payment_qr_code.unwrap_or(false) {
            md.push_str("\nThe document includes a QR code for this payment link.\n");
        }
    }
    md.push_str("\n[Single Document Pass ($2.99 one-time)](https://pressmark.dev/api/v1/checkout?plan=pass_single_unlock&redirect=true) · [Solo Pro ($15/month)](https://pressmark.dev/api/v1/checkout?plan=plan_solo&redirect=true)");
    md
}
