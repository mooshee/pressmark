//! Visual QA High-Resolution Artifact Generation Test Suite
//!
//! Author: Daniel Hallman
//! Product: Pressmark Vector Engine
//!
//! Renders production-fidelity 144 DPI PNG visual artifacts for optical and layout QA.

use pressmark::engine::generate_preview_png;
use pressmark::models::{DocumentRequest, DocumentType, LineItem, PartyDetails};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_generate_visual_qa_artifacts() {
    let artifact_dir = PathBuf::from("/Users/dhallman/.gemini/antigravity/brain/db484a2f-1c16-4cec-a780-3792e1b1f23f");
    let _ = fs::create_dir_all(&artifact_dir);

    // 1. Tax Invoice with Vector QR Code & Embedded Payment URL
    let inv = DocumentRequest {
        doc_type: DocumentType::Invoice,
        doc_number: Some("INV-2026-QA-01".into()),
        title: Some("TAX INVOICE".into()),
        header_subtitle: Some("High-Throughput Vector Synthesis & Infrastructure".into()),
        issue_date: Some("2026-10-02".into()),
        due_date: Some("2026-10-16".into()),
        theme: Some("executive_navy".into()),
        layout_style: Some("modern".into()),
        from: PartyDetails {
            name: "Pressmark Technologies LLC".into(),
            company: Some("High-Throughput Vector Platforms".into()),
            address: Some("742 Evergreen Terrace, Suite 400\nSan Francisco, CA 94107".into()),
            email: Some("billing@pressmark.io".into()),
            phone: Some("+1 (415) 890-4421".into()),
            tax_id: Some("US-94-3829104".into()),
            website: Some("https://pressmark.io".into()),
            registration_number: Some("REG-84920".into()),
        },
        to: PartyDetails {
            name: "Acme Corporation Global".into(),
            company: Some("Enterprise Cloud Operations".into()),
            address: Some("100 Innovation Parkway, Floor 12\nAustin, TX 78701".into()),
            email: Some("accounts-payable@acmeglobal.com".into()),
            phone: Some("+1 (512) 555-0199".into()),
            tax_id: Some("US-74-8891024".into()),
            website: Some("https://acmeglobal.com".into()),
            registration_number: Some("CRN-09281".into()),
        },
        items: vec![
            LineItem {
                description: "Sub-Millisecond Vector Typesetting Engine (Annual Enterprise License)".into(),
                quantity: 1.0,
                unit_price: 4500.0,
                ..Default::default()
            },
            LineItem {
                description: "Zero-Latency Real-Time Canvas Synchronization Middleware".into(),
                quantity: 1.0,
                unit_price: 1500.0,
                ..Default::default()
            },
            LineItem {
                description: "Dedicated High-Throughput SLA & Architecture Support (Q4 2026)".into(),
                quantity: 3.0,
                unit_price: 800.0,
                ..Default::default()
            },
        ],
        tax_rate: Some(8.5),
        discount_amount: Some(400.0),
        payment_qr_code: Some(true),
        payment_link: Some("https://buy.stripe.com/test_28EcN4eQu9MJfaR1ADgbm00".into()),
        payment_method: Some("Stripe / Apple Pay / Cards".into()),
        notes: Some("All payments processed securely via Stripe. Scan QR code or click link above for instant 1-tap checkout.".into()),
        terms: Some("Net 14 payment terms. 1.5% interest per month on late invoices.".into()),
        ..Default::default()
    };

    let (inv_png, _) = generate_preview_png(&inv, 144).expect("Invoice PNG should compile");
    fs::write(artifact_dir.join("visual_qa_1_invoice_vector_qr.png"), inv_png).unwrap();

    // 2. Official Payment Receipt with PAID Watermark Stamp
    let receipt = inv.to_receipt(
        "Stripe / Apple Pay",
        "ch_3Mwj4x2eZvKYlo2C1g9J",
        "2026-10-02",
    );
    let (rec_png, _) = generate_preview_png(&receipt, 144).expect("Receipt PNG should compile");
    fs::write(artifact_dir.join("visual_qa_2_paid_receipt_watermark.png"), rec_png).unwrap();

    // 3. Credit Note with Reason and Adjusted Balance
    let credit = inv.to_credit_note(Some("Annual enterprise volume rebate and SLA credit"));
    let (crd_png, _) = generate_preview_png(&credit, 144).expect("Credit Note PNG should compile");
    fs::write(artifact_dir.join("visual_qa_3_credit_note_refund.png"), crd_png).unwrap();

    // 4. Partial Payment Invoice with Split Totals
    let mut partial = inv.clone();
    partial.doc_number = Some("INV-2026-QA-02".into());
    partial.amount_paid = Some(4000.0);
    partial.payment_status = Some("partial".into());
    partial.is_paid = Some(false);
    partial.paid_at = Some("2026-10-02".into());
    partial.transaction_reference = Some("ch_partial_wire_891".into());
    partial.payment_method = Some("Corporate ACH Transfer".into());
    let (partial_png, _) = generate_preview_png(&partial, 144).expect("Partial Invoice PNG should compile");
    fs::write(artifact_dir.join("visual_qa_4_partial_payment_invoice.png"), partial_png).unwrap();
}
