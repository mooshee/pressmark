//! Payment metadata precedence and receipt compatibility regression tests.
//! Author: Daniel Hallman

use pressmark::engine::generate_typst_markup;
use pressmark::mcp::{JsonRpcRequest, McpHandler};
use pressmark::models::{DocumentRequest, DocumentType, LineItem, PartyDetails};
use serde_json::json;

fn document(doc_type: DocumentType, total: f64) -> DocumentRequest {
    DocumentRequest {
        doc_type,
        from: PartyDetails {
            name: "Seller".into(),
            ..Default::default()
        },
        to: PartyDetails {
            name: "Buyer".into(),
            ..Default::default()
        },
        items: vec![LineItem {
            description: "Services".into(),
            quantity: 1.0,
            unit_price: total,
            ..Default::default()
        }],
        ..Default::default()
    }
}

fn assert_payment(doc: &DocumentRequest, status: &str, paid: f64, balance: f64) {
    assert_eq!(doc.effective_payment_status(), status);
    assert_eq!(doc.is_paid_state(), status == "paid");
    assert!((doc.amount_paid_effective() - paid).abs() < 0.001);
    assert!((doc.balance_due() - balance).abs() < 0.001);
}

#[test]
fn receipt_type_preserves_explicit_payment_metadata() {
    for (status, amount, expected_balance) in [
        ("unpaid", None, 1000.0),
        ("partial", Some(250.0), 750.0),
        ("paid", Some(1000.0), 0.0),
    ] {
        let mut invoice = document(DocumentType::Invoice, 1000.0);
        invoice.payment_status = Some(status.into());
        invoice.is_paid = Some(status == "paid");
        invoice.amount_paid = amount;
        let mut receipt = invoice.clone();
        receipt.doc_type = DocumentType::Receipt;
        assert_payment(&receipt, status, amount.unwrap_or(0.0), expected_balance);
        assert_eq!(
            receipt.effective_payment_status(),
            invoice.effective_payment_status()
        );
        assert_eq!(receipt.balance_due(), invoice.balance_due());

        let markup = generate_typst_markup(&receipt);
        assert_eq!(
            markup.contains("#paid-watermark-stamp(tx-ref:"),
            status == "paid"
        );
        assert_eq!(
            markup.contains("\n  status: \"PAID / SETTLED\","),
            status == "paid"
        );
        assert_eq!(
            markup.contains("\n  status: \"PARTIALLY PAID\","),
            status == "partial"
        );
    }
}

#[test]
fn recorded_amount_prevents_false_full_payment_claims() {
    for (status, flag, amount, expected_status, expected_balance) in [
        (Some("paid"), Some(true), 250.0, "partial", 750.0),
        (Some("paid"), Some(false), 0.0, "unpaid", 1000.0),
        (None, Some(true), 250.0, "partial", 750.0),
        (Some("unpaid"), Some(true), 1000.0, "unpaid", 0.0),
        (Some("partial"), Some(true), 250.0, "partial", 750.0),
        (None, Some(false), 1000.0, "unpaid", 0.0),
        (None, None, 1000.0, "paid", 0.0),
        (None, None, 1200.0, "paid", 0.0),
        (None, None, 250.0, "partial", 750.0),
    ] {
        let mut receipt = document(DocumentType::Receipt, 1000.0);
        receipt.payment_status = status.map(str::to_owned);
        receipt.is_paid = flag;
        receipt.amount_paid = Some(amount);
        assert_payment(&receipt, expected_status, amount, expected_balance);
        let markup = generate_typst_markup(&receipt);
        assert_eq!(
            markup.contains("#paid-watermark-stamp(tx-ref:"),
            expected_status == "paid"
        );
    }
}

#[test]
fn omitted_receipt_metadata_keeps_legacy_full_payment_default() {
    let receipt = document(DocumentType::Receipt, 1000.0);
    assert_payment(&receipt, "paid", 1000.0, 0.0);

    let mut explicit_unpaid = receipt.clone();
    explicit_unpaid.is_paid = Some(false);
    assert_payment(&explicit_unpaid, "unpaid", 0.0, 1000.0);
    explicit_unpaid.is_paid = Some(true);
    explicit_unpaid.payment_status = Some("unpaid".into());
    assert_payment(&explicit_unpaid, "unpaid", 0.0, 1000.0);

    let mut paid_invoice = document(DocumentType::Invoice, 1000.0);
    paid_invoice.payment_status = Some("paid".into());
    assert_payment(&paid_invoice, "paid", 1000.0, 0.0);
    let recorded = paid_invoice.to_receipt("Recorded manually", "RECEIPT-1", "2026-10-28");
    assert_payment(&recorded, "paid", 1000.0, 0.0);
}

#[test]
fn zero_and_negative_totals_do_not_imply_paid_or_refunded() {
    for amount in [None, Some(0.0), Some(50.0)] {
        let mut credit = document(DocumentType::CreditNote, -1000.0);
        credit.amount_paid = amount;
        let paid = amount.unwrap_or(0.0);
        assert_payment(&credit, "unpaid", paid, -1000.0 - paid);
        let markup = generate_typst_markup(&credit);
        assert!(!markup.contains("#paid-watermark-stamp(tx-ref:"));
        assert!(markup.contains("\n  status: \"CREDIT NOTE\","));
        credit.payment_status = Some("refunded".into());
        credit.is_paid = Some(false);
        assert_payment(&credit, "refunded", paid, -1000.0 - paid);
        assert!(generate_typst_markup(&credit).contains("\n  status: \"CREDIT / REFUNDED\","));
    }
    let mut zero_invoice = document(DocumentType::Invoice, 0.0);
    zero_invoice.amount_paid = Some(0.0);
    assert_payment(&zero_invoice, "unpaid", 0.0, 0.0);
}

#[test]
fn mcp_totals_agree_with_resolved_receipt_payment_state() {
    for (status, amount, expected_status, expected_paid, expected_balance) in [
        (None, None, "PAID", 1000.0, 0.0),
        (Some("unpaid"), None, "UNPAID", 0.0, 1000.0),
        (Some("partial"), Some(250.0), "PARTIAL", 250.0, 750.0),
        (Some("paid"), Some(250.0), "PARTIAL", 250.0, 750.0),
        (Some("paid"), Some(1000.0), "PAID", 1000.0, 0.0),
    ] {
        let mut receipt = document(DocumentType::Receipt, 1000.0);
        receipt.payment_status = status.map(str::to_owned);
        receipt.amount_paid = amount;
        let response = McpHandler::handle_request(JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(json!(1)),
            method: "tools/call".into(),
            params: Some(
                json!({ "name": "calculate_totals", "arguments": { "document": receipt } }),
            ),
        });
        assert!(response.error.is_none(), "{:?}", response.error);
        let result = response.result.unwrap();
        assert_eq!(result["amount_paid"], expected_paid);
        assert_eq!(result["balance_due"], expected_balance);
        assert_eq!(result["is_paid"], expected_status == "PAID");
        assert!(result["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains(&format!("Status: {expected_status}")));
    }
}

#[test]
fn receipt_remittance_stub_uses_remaining_balance() {
    for (status, amount, balance) in [
        ("unpaid", 0.0, 100.0),
        ("partial", 40.0, 60.0),
        ("paid", 100.0, 0.0),
    ] {
        let mut receipt = document(DocumentType::Receipt, 100.0);
        receipt.payment_status = Some(status.into());
        receipt.amount_paid = Some(amount);
        receipt.show_remittance_stub = Some(true);
        let markup = generate_typst_markup(&receipt);
        assert!(markup.contains("[Document total:]"));
        assert!(!markup.contains("[Total Due:]"));
        let remittance = markup
            .split_once("#remittance-stub(\n")
            .expect("Remittance invocation must be present")
            .1
            .split_once("\n)\n")
            .unwrap()
            .0;
        assert!(
            remittance.contains(&format!(
                "  amount-due: \"{}\",",
                receipt.format_amount(balance)
            )),
            "Incorrect {status} remittance amount: {remittance}"
        );
    }
}
