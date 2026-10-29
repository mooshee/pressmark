//! MCP document delivery and missing-field regressions.
//!
//! Author: Daniel Hallman

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use pressmark::engine::generate_typst_markup;
use pressmark::mcp::{JsonRpcRequest, McpHandler};
use pressmark::models::DocumentRequest;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn sample_document() -> Value {
    json!({
        "doc_type": "invoice",
        "from": { "name": "Sample Studio" },
        "to": { "name": "Sample Client" },
        "items": [{ "description": "Design services", "quantity": 15, "unit_price": 250 }]
    })
}

fn call_tool(name: &str, arguments: Value) -> Value {
    let response = McpHandler::handle_request(JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "tools/call".to_string(),
        params: Some(json!({ "name": name, "arguments": arguments })),
    });
    assert!(response.error.is_none(), "{name}: {:?}", response.error);
    response.result.expect("successful tool result")
}

#[test]
fn generated_converted_and_payment_pdfs_are_in_mcp_content() {
    for (name, arguments) in [
        ("generate_document", sample_document()),
        (
            "convert_document",
            json!({ "document": sample_document(), "target_type": "estimate" }),
        ),
        (
            "record_payment",
            json!({
                "document": sample_document(), "amount_paid": 3750,
                "payment_method": "Check", "transaction_reference": "SAMPLE-0042", "paid_date": "2026-10-28"
            }),
        ),
    ] {
        let result = call_tool(name, arguments);
        let resource = result["content"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["type"] == "resource")
            .expect("PDF must be delivered in MCP content")["resource"]
            .clone();
        assert_eq!(resource["mimeType"], "application/pdf");
        let pdf = BASE64.decode(resource["blob"].as_str().unwrap()).unwrap();
        assert!(pdf.starts_with(b"%PDF-"));
        assert!(pdf.len() > 1000);
        assert_eq!(resource["blob"], result["pdf_base64"]);
        assert_eq!(resource["blob"], result["structuredContent"]["pdf_base64"]);
        assert_eq!(
            resource["uri"],
            format!(
                "pressmark://documents/{}.pdf",
                hex::encode(Sha256::digest(&pdf))
            )
        );
        for key in ["doc_number", "grand_total", "compile_time_ms", "document"] {
            if let Some(value) = result.get(key) {
                assert_eq!(Some(value), result["structuredContent"].get(key));
            }
        }
    }
}

#[test]
fn omitted_dates_and_number_stay_blank_without_certification_or_engine_footer() {
    let doc: DocumentRequest = serde_json::from_value(sample_document()).unwrap();
    assert_eq!(doc.effective_doc_number(), "");
    let markup = generate_typst_markup(&doc);
    let body = markup
        .strip_prefix(pressmark::templates::TYPST_THEME)
        .unwrap();
    for unwanted in [
        "2026-0042",
        "2026-10-01",
        "Certified",
        "#audit-slug(",
        "Pressmark Vector Engine",
    ] {
        assert!(
            !body.contains(unwanted),
            "unexpected default in document: {unwanted}"
        );
    }
    assert!(body.contains("doc-number: \"\""));
    assert!(body.contains("date: \"\""));
    let result = call_tool("generate_document", sample_document());
    assert_eq!(result["doc_number"], "");
    let summary = result["content"][0]["text"].as_str().unwrap();
    assert!(summary.contains("**Document #:** `Not provided`"));
    assert!(summary.contains("**Date:** `Not provided`"));
    assert!(!summary.contains("Vector Engine"));
}

#[test]
fn supplied_document_fields_and_custom_footer_are_preserved() {
    let mut value = sample_document();
    value["doc_number"] = json!("INV-DEMO-0042");
    value["issue_date"] = json!("2026-10-28");
    value["due_date"] = json!("2026-11-27");
    value["footer_text"] = json!("Thank you for your business.");
    let doc: DocumentRequest = serde_json::from_value(value.clone()).unwrap();
    let markup = generate_typst_markup(&doc);
    let body = markup
        .strip_prefix(pressmark::templates::TYPST_THEME)
        .unwrap();
    for expected in [
        "INV-DEMO-0042",
        "2026-10-28",
        "2026-11-27",
        "Thank you for your business.",
    ] {
        assert!(body.contains(expected), "missing user field: {expected}");
    }
    let preview = call_tool("preview_document", value);
    let png = BASE64
        .decode(preview["content"][0]["data"].as_str().unwrap())
        .unwrap();
    assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));
    assert_eq!(
        preview["structuredContent"]["image_data_uri"],
        preview["image_data_uri"]
    );
}

#[test]
fn document_schemas_expose_dates_and_number_and_describe_generation_side_effects() {
    let tools = McpHandler::get_tools();
    for name in ["generate_document", "preview_document"] {
        let tool = tools
            .as_array()
            .unwrap()
            .iter()
            .find(|tool| tool["name"] == name)
            .unwrap();
        for field in ["doc_number", "issue_date", "due_date"] {
            assert_eq!(tool["inputSchema"]["properties"][field]["type"], "string");
            assert!(!tool["inputSchema"]["required"]
                .as_array()
                .unwrap()
                .contains(&json!(field)));
        }
        assert!(!tool["description"].as_str().unwrap().contains("<25ms"));
    }
    let generate = &tools[0];
    assert_eq!(generate["annotations"]["readOnlyHint"], false);
    assert_eq!(generate["annotations"]["idempotentHint"], false);
    assert_eq!(generate["annotations"]["openWorldHint"], true);
}
