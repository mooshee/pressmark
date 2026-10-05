//! Pressmark Model Context Protocol (MCP) Server
//!
//! Author: Daniel Hallman

use crate::engine::{generate_pdf, generate_preview_png};
use crate::models::DocumentRequest;
use crate::pricing::get_pricing_catalog;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

pub struct McpHandler;

impl McpHandler {
    pub fn get_tools() -> Value {
        json!([
            {
                "name": "generate_document",
                "description": "Generate a publication-ready vector PDF document (invoice, estimate, receipt, agreement, NDA, purchase order, certificate) in under 25ms.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "doc_type": {
                            "type": "string",
                            "enum": ["invoice", "estimate", "receipt", "agreement", "nda", "purchase_order", "certificate"],
                            "description": "The category of document to generate"
                        },
                        "title": { "type": "string" },
                        "doc_number": { "type": "string" },
                        "from": {
                            "type": "object",
                            "properties": {
                                "name": { "type": "string" },
                                "company": { "type": "string" },
                                "address": { "type": "string" },
                                "email": { "type": "string" }
                            },
                            "required": ["name"]
                        },
                        "to": {
                            "type": "object",
                            "properties": {
                                "name": { "type": "string" },
                                "company": { "type": "string" },
                                "address": { "type": "string" },
                                "email": { "type": "string" }
                            },
                            "required": ["name"]
                        },
                        "items": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "description": { "type": "string" },
                                    "quantity": { "type": "number" },
                                    "unit_price": { "type": "number" }
                                },
                                "required": ["description", "quantity", "unit_price"]
                            }
                        },
                        "theme": { "type": "string" },
                        "primary_color": { "type": "string" },
                        "notes": { "type": "string" },
                        "terms": { "type": "string" },
                        "is_paid": { "type": "boolean" }
                    },
                    "required": ["doc_type", "from", "to"]
                }
            },
            {
                "name": "preview_document",
                "description": "Render a high-resolution PNG preview of the document as a Base64 Data URI for immediate visual inspection in chat.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "doc_type": { "type": "string" },
                        "from": { "type": "object", "required": ["name"] },
                        "to": { "type": "object", "required": ["name"] }
                    },
                    "required": ["doc_type", "from", "to"]
                }
            },
            {
                "name": "calculate_totals",
                "description": "Calculate subtotal, tax amounts, discounts, grand total, and balance due for a document without full PDF synthesis.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "document": { "type": "object", "description": "The document payload containing items, tax rates, and discounts" }
                    },
                    "required": ["document"]
                }
            },
            {
                "name": "get_pricing",
                "description": "Get current subscription tiers, single-pass microtransactions, and checkout links.",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "convert_document",
                "description": "Convert an existing document (e.g. Estimate or Quote) to a Tax Invoice, Receipt, or Credit Note, preserving all line items, parties, and calculations.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "document": { "type": "object", "description": "The current document payload" },
                        "target_type": { "type": "string", "enum": ["invoice", "receipt", "estimate", "credit_note"], "description": "Target lifecycle document type (invoice, receipt, estimate, credit_note)" },
                        "due_days": { "type": "integer", "description": "Number of days until payment is due (for invoice conversion)" },
                        "payment_link": { "type": "string", "description": "Optional customer checkout/payment URL to embed as vector QR code and clickable link" },
                        "payment_qr_code": { "type": "boolean", "description": "Whether to render an SVG vector QR code for 1-scan mobile payment" },
                        "credit_reason": { "type": "string", "description": "Reason for issuing credit note / refund (e.g. Overbilling, Customer cancellation)" }
                    },
                    "required": ["document", "target_type"]
                }
            },
            {
                "name": "record_payment",
                "description": "Record a payment against an invoice and instantly generate an official Receipt with payment status and remaining balance tracking.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "document": { "type": "object", "description": "The current invoice payload" },
                        "payment_method": { "type": "string", "description": "Method of payment, e.g. Stripe, Apple Pay, Bank Wire" },
                        "transaction_reference": { "type": "string", "description": "Transaction or checkout reference ID" },
                        "paid_date": { "type": "string", "description": "Date payment was received (YYYY-MM-DD)" },
                        "amount_paid": { "type": "number", "description": "Amount received. Defaults to grand total (marks PAID IN FULL). If less, records partial payment." }
                    },
                    "required": ["document"]
                }
            }
        ])
    }

    pub fn handle_request(req: JsonRpcRequest) -> JsonRpcResponse {
        match req.method.as_str() {
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
                            Ok(doc_req) => match generate_pdf(&doc_req) {
                                Ok((pdf_bytes, ms)) => {
                                    let b64 = BASE64.encode(&pdf_bytes);
                                    JsonRpcResponse {
                                        jsonrpc: "2.0",
                                        id: req.id,
                                        result: Some(json!({
                                            "content": [
                                                {
                                                    "type": "text",
                                                    "text": format_document_markdown(&doc_req, "Generated", ms)
                                                }
                                            ],
                                            "pdf_base64": b64,
                                            "doc_number": doc_req.effective_doc_number(),
                                            "grand_total": doc_req.grand_total(),
                                            "compile_time_ms": ms
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
                            },
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
                            Ok(doc_req) => match generate_preview_png(&doc_req, 144) {
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
                                            "compile_time_ms": ms
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
                            },
                            Err(e) => JsonRpcResponse {
                                jsonrpc: "2.0",
                                id: req.id,
                                result: None,
                                error: Some(json!({ "code": -32602, "message": format!("Invalid arguments: {}", e) })),
                            },
                        }
                    }
                    "get_pricing" => JsonRpcResponse {
                        jsonrpc: "2.0",
                        id: req.id,
                        result: Some(json!(get_pricing_catalog())),
                        error: None,
                    },
                    "convert_document" => {
                        let target_type = arguments.get("target_type").and_then(|t| t.as_str()).unwrap_or("invoice");
                        let due_days = arguments.get("due_days").and_then(|d| d.as_u64()).unwrap_or(15) as u32;
                        let payment_link = arguments.get("payment_link").and_then(|l| l.as_str());
                        let payment_qr_code = arguments.get("payment_qr_code").and_then(|q| q.as_bool());
                        let credit_reason = arguments.get("credit_reason").and_then(|r| r.as_str());
                        let doc_val = arguments.get("document").cloned().unwrap_or(Value::Null);

                        match serde_json::from_value::<DocumentRequest>(doc_val) {
                            Ok(doc_req) => {
                                let mut converted = match target_type.to_lowercase().as_str() {
                                    "credit_note" | "credit" | "refund" => doc_req.to_credit_note(credit_reason),
                                    "receipt" => doc_req.to_receipt(
                                        "Stripe / Apple Pay",
                                        "TX-PAID",
                                        &chrono::Utc::now().format("%Y-%m-%d").to_string(),
                                    ),
                                    "estimate" | "quote" => doc_req.to_estimate(30),
                                    _ => doc_req.to_invoice(due_days),
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
                                                    }
                                                ],
                                                "document": converted,
                                                "pdf_base64": b64,
                                                "compile_time_ms": ms
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
                            Ok(doc_req) => {
                                let subtotal = doc_req.subtotal();
                                let tax_total = doc_req.tax_total();
                                let discount = doc_req.discount_amount.unwrap_or(0.0);
                                let grand_total = doc_req.grand_total();
                                let balance_due = doc_req.balance_due();
                                let amount_paid = doc_req.amount_paid.unwrap_or(0.0);
                                let is_paid = doc_req.is_paid_state();

                                JsonRpcResponse {
                                    jsonrpc: "2.0",
                                    id: req.id,
                                    result: Some(json!({
                                        "content": [
                                            {
                                                "type": "text",
                                                "text": format!(
                                                    "**Document Totals Calculation:**\n- Subtotal: ${:.2}\n- Tax: ${:.2}\n- Discount: ${:.2}\n- **Grand Total:** ${:.2}\n- Amount Paid: ${:.2}\n- **Balance Due:** ${:.2}\n- Status: {}",
                                                    subtotal, tax_total, discount, grand_total, amount_paid, balance_due, if is_paid { "PAID" } else { "UNPAID" }
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
                        let method = arguments.get("payment_method").and_then(|m| m.as_str()).unwrap_or("Stripe / Apple Pay");
                        let tx_ref = arguments.get("transaction_reference").and_then(|r| r.as_str()).unwrap_or("STRIPE-AUTO-PAID");
                        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
                        let paid_date = arguments.get("paid_date").and_then(|d| d.as_str()).unwrap_or(&today);
                        let amount_paid = arguments.get("amount_paid").and_then(|a| a.as_f64());

                        match serde_json::from_value::<DocumentRequest>(doc_val) {
                            Ok(doc_req) => {
                                let grand_total = doc_req.grand_total();
                                let (payment_doc, action_label) = if let Some(paid) = amount_paid {
                                    if paid < grand_total - 0.001 {
                                        let mut partial_doc = doc_req.clone();
                                        partial_doc.amount_paid = Some(paid);
                                        partial_doc.is_paid = Some(false);
                                        partial_doc.payment_status = Some("partial".to_string());
                                        partial_doc.payment_method = Some(method.to_string());
                                        partial_doc.transaction_reference = Some(tx_ref.to_string());
                                        partial_doc.paid_at = Some(paid_date.to_string());
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
                                                    }
                                                ],
                                                "document": payment_doc,
                                                "pdf_base64": b64,
                                                "compile_time_ms": ms
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
            _ => JsonRpcResponse {
                jsonrpc: "2.0",
                id: req.id,
                result: None,
                error: Some(json!({ "code": -32601, "message": "Method not found" })),
            },
        }
    }
}

fn format_document_markdown(doc: &DocumentRequest, action_title: &str, compile_ms: u128) -> String {
    let status_badge = if doc.is_paid_state() {
        "**PAID IN FULL** ✅"
    } else if doc.amount_paid.is_some() && doc.balance_due() > 0.0 {
        "**PARTIALLY PAID** 💳"
    } else {
        "**PAYMENT PENDING** ⏳"
    };

    let issue_date_str = doc.issue_date.as_deref().unwrap_or("Current");

    let mut md = format!(
        "### {} {}\n\n**Document #:** `{}` | **Date:** `{}` | **Status:** {}\n\n",
        action_title,
        doc.effective_title(),
        doc.effective_doc_number(),
        issue_date_str,
        status_badge
    );
    md.push_str(&format!("- **From:** {} ({})\n", doc.from.name, doc.from.company.as_deref().unwrap_or("-")));
    md.push_str(&format!("- **To:** {} ({})\n\n", doc.to.name, doc.to.company.as_deref().unwrap_or("-")));

    if !doc.items.is_empty() {
        md.push_str("| Description | Qty | Rate | Amount |\n|---|---:|---:|---:|\n");
        for item in &doc.items {
            md.push_str(&format!("| {} | {} | ${:.2} | ${:.2} |\n", item.description, item.quantity, item.unit_price, item.amount()));
        }
        md.push('\n');
    }

    md.push_str(&format!("- **Subtotal:** ${:.2}\n", doc.subtotal()));
    if let Some(rate) = doc.tax_rate {
        if rate > 0.0 {
            md.push_str(&format!("- **Tax ({:.1}%):** ${:.2}\n", rate, doc.tax_total()));
        }
    }
    md.push_str(&format!("- **Grand Total:** ${:.2}\n", doc.grand_total()));
    if let Some(paid) = doc.amount_paid {
        md.push_str(&format!("- **Amount Paid:** ${:.2}\n", paid));
        md.push_str(&format!("- **Balance Due:** ${:.2}\n", doc.balance_due()));
    }

    if let Some(link) = &doc.payment_link {
        md.push_str(&format!("\n👉 [💳 Click Here to Pay Online]({})\n", link));
        if doc.payment_qr_code.unwrap_or(false) {
            md.push_str("\n*Vector SVG QR code embedded in document for 1-scan mobile checkout.*\n");
        }
    }
    md.push_str(&format!("\n*Rendered in {}ms via Pressmark Vector Engine.*", compile_ms));
    md
}
