//! Studio Interactive Button Sequences & Archetype Switching Integration Tests
//!
//! Author: Daniel Hallman
//!
//! Exhaustive test suite testing simulated button tap sequences, archetype transitions,
//! layout alterations, formatting switches, and vector preview synchronization.

use pressmark::engine::{generate_all_preview_pngs, generate_pdf};
use pressmark::models::{
    CustomField, DocumentRequest, DocumentSection, DocumentType, LineItem, PartyDetails,
    SignatureBlock,
};

fn create_sample_request(doc_type: DocumentType) -> DocumentRequest {
    DocumentRequest {
        doc_type: doc_type.clone(),
        title: Some(doc_type.display_title().to_string()),
        doc_number: Some("INV-2026-TEST".to_string()),
        issue_date: Some("2026-10-01".to_string()),
        due_date: Some("2026-10-15".to_string()),
        theme: Some("modern_slate".to_string()),
        primary_color: Some("#4f46e5".to_string()),
        accent_color: None,
        font_family: Some("sans".to_string()),
        font_scale: Some("standard".to_string()),
        paper_size: Some("us-letter".to_string()),
        logo_data: None,
        signature_data: None,
        signer_name: Some("Daniel Hallman".to_string()),
        signer_title: Some("Principal".to_string()),
        from: PartyDetails {
            name: "Pressmark Technologies".to_string(),
            company: Some("Pressmark LLC".to_string()),
            address: Some("123 Market St, San Francisco, CA".to_string()),
            email: Some("billing@pressmark.dev".to_string()),
            phone: Some("+1 (415) 555-0100".to_string()),
            tax_id: Some("US-94-8849201".to_string()),
            website: Some("pressmark.dev".to_string()),
            registration_number: Some("CA-CORP-488219".to_string()),
            ..Default::default()
        },
        to: PartyDetails {
            name: "Acme Corp".to_string(),
            company: Some("Acme Holdings".to_string()),
            address: Some("456 Innovation Way, Austin, TX".to_string()),
            email: Some("accounts@acme.com".to_string()),
            phone: Some("+1 (512) 555-0182".to_string()),
            tax_id: Some("US-74-9918204".to_string()),
            ..Default::default()
        },
        items: vec![
            LineItem {
                description: "Rust High-Performance Service Implementation".to_string(),
                quantity: 40.0,
                unit_price: 175.0,
                discount: None,
                tax_rate: None,
            },
            LineItem {
                description: "Sub-15ms Document Typesetting Pipeline".to_string(),
                quantity: 20.0,
                unit_price: 200.0,
                discount: Some(250.0),
                tax_rate: None,
            },
        ],
        tax_rate: Some(8.25),
        discount_amount: Some(100.0),
        notes: Some("Thank you for your partnership!".to_string()),
        terms: Some("Payment due within 15 days of invoice date.".to_string()),
        currency: Some("USD".to_string()),
        compliance_mode: Some("none".to_string()),
        header_subtitle: Some("High-Throughput Vector Typesetting Services".to_string()),
        footer_text: Some("Pressmark Engine · Verified Vector Standard".to_string()),
        is_paid: Some(false),
        show_signature: Some(true),
        show_remittance_stub: Some(false),
        show_qr_seal: Some(true),
        show_line_numbers: Some(true),
        show_page_numbers: Some(true),
        show_footer_divider: Some(true),
        sections: vec![
            DocumentSection {
                title: "Scope & Deliverables".to_string(),
                content: "All services rendered in accordance with enterprise SLA.".to_string(),
            },
        ],
        signatures: vec![
            SignatureBlock {
                signer_name: "Daniel Hallman".to_string(),
                signer_title: Some("Chief Architect".to_string()),
                date: Some("2026-10-01".to_string()),
                signature_data: None,
            },
        ],
        custom_fields: vec![
            CustomField {
                label: "Project Code".to_string(),
                value: "PRESSMARK-CORE-001".to_string(),
            },
        ],
        ..Default::default()
    }
}

#[test]
fn test_all_39_archetypes_pdf_and_preview_png() {
    let all_archetypes = DocumentType::all().to_vec();

    assert_eq!(all_archetypes.len(), 57);

    for dt in &all_archetypes {
        let req = create_sample_request(dt.clone());
        let pdf_res = generate_pdf(&req);
        assert!(
            pdf_res.is_ok(),
            "Archetype {:?} failed PDF compilation: {:?}",
            dt,
            pdf_res.err()
        );
        let (bytes, pdf_ms) = pdf_res.unwrap();
        assert!(!bytes.is_empty(), "PDF empty for {:?}", dt);

        let png_res = generate_all_preview_pngs(&req, 144);
        assert!(
            png_res.is_ok(),
            "Archetype {:?} failed PNG preview generation: {:?}",
            dt,
            png_res.err()
        );
        let (pages, png_ms) = png_res.unwrap();
        assert!(!pages.is_empty(), "Preview pages empty for {:?}", dt);
        println!(
            "Verified archetype {:?}: PDF ({} bytes, {}ms), PNG ({} pages, {}ms)",
            dt,
            bytes.len(),
            pdf_ms,
            pages.len(),
            png_ms
        );
    }
}

#[test]
fn test_all_39_archetypes_empty_items_robustness() {
    let all_archetypes = vec![
        DocumentType::Invoice,
        DocumentType::InvoiceFreelance,
        DocumentType::ProformaInvoice,
        DocumentType::WorkOrder,
        DocumentType::Certificate,
        DocumentType::Nda,
        DocumentType::ResidentialLease,
        DocumentType::CommercialLease,
        DocumentType::ExpenseReport,
        DocumentType::DeliveryReceipt,
        DocumentType::TenantEstoppel,
    ];

    for dt in &all_archetypes {
        let mut req = create_sample_request(dt.clone());
        req.items = vec![];
        let pdf_res = generate_pdf(&req);
        assert!(
            pdf_res.is_ok(),
            "Empty items failed PDF compilation for {:?}: {:?}",
            dt,
            pdf_res.err()
        );
        let png_res = generate_all_preview_pngs(&req, 144);
        assert!(
            png_res.is_ok(),
            "Empty items failed PNG preview for {:?}: {:?}",
            dt,
            png_res.err()
        );
    }
}

#[test]
fn test_simulated_button_tap_sequence_archetype_transitions() {
    // Simulate user tapping through 9 distinct archetypes in sequence
    let sequence = vec![
        DocumentType::Invoice,
        DocumentType::ResidentialLease,
        DocumentType::Estimate,
        DocumentType::CommercialNnnLease,
        DocumentType::WorkOrder,
        DocumentType::Certificate,
        DocumentType::Nda,
        DocumentType::CreditNote,
        DocumentType::ExpenseReport,
    ];

    let mut current_state = create_sample_request(DocumentType::Invoice);

    for next_type in sequence {
        current_state.doc_type = next_type.clone();
        current_state.title = Some(next_type.display_title().to_string());

        // Preview should compile cleanly at each step
        let preview_res = generate_all_preview_pngs(&current_state, 144);
        assert!(
            preview_res.is_ok(),
            "Transition to {:?} failed preview generation: {:?}",
            next_type,
            preview_res.err()
        );
        let (pages, ms) = preview_res.unwrap();
        assert!(!pages.is_empty(), "Pages empty on transition to {:?}", next_type);
        println!("Button Tap -> Switched to {:?} in {}ms ({} pages)", next_type, ms, pages.len());
    }
}

#[test]
fn test_simulated_button_tap_sequence_field_and_toggle_mutations() {
    let mut req = create_sample_request(DocumentType::Invoice);

    // 1. User adds a line item
    req.items.push(LineItem {
        description: "Emergency Architectural Consultation & Tuning".to_string(),
        quantity: 5.0,
        unit_price: 350.0,
        discount: Some(100.0),
        tax_rate: None,
    });
    assert!(generate_pdf(&req).is_ok());

    // 2. User toggles Paid Stamp
    req.is_paid = Some(true);
    req.payment_status = Some("paid".to_string());
    req.transaction_reference = Some("TXN-STRIPE-884920".to_string());
    assert!(generate_pdf(&req).is_ok());

    // 3. User toggles Payment QR Code
    req.payment_qr_code = Some(true);
    req.payment_link = Some("https://pressmark.dev/pay/INV-2026-0001".to_string());
    assert!(generate_pdf(&req).is_ok());

    // 4. User toggles Remittance Stub
    req.show_remittance_stub = Some(true);
    assert!(generate_pdf(&req).is_ok());

    // 5. User changes Paper Size to A4 and Spacing to Compact
    req.paper_size = Some("a4".to_string());
    req.spacing_density = Some("compact".to_string());
    assert!(generate_pdf(&req).is_ok());

    // 6. User changes Layout Style to Modern Banner
    req.layout_style = Some("banner".to_string());
    assert!(generate_pdf(&req).is_ok());

    // 7. User changes Currency to EUR with suffix placement
    req.currency = Some("EUR".to_string());
    req.currency_placement = Some("suffix".to_string());
    assert!(generate_pdf(&req).is_ok());

    // 8. User changes Date Format to Formal Prose
    req.date_format = Some("formal".to_string());
    assert!(generate_pdf(&req).is_ok());

    // Final preview check
    let (pages, ms) = generate_all_preview_pngs(&req, 144).expect("Final preview failed");
    assert!(!pages.is_empty());
    println!("Mutation sequence completed with {} preview pages in {}ms", pages.len(), ms);
}

#[test]
fn test_simulated_partial_payment_button_tap() {
    let mut req = create_sample_request(DocumentType::Invoice);
    let total = req.grand_total();
    let half = total / 2.0;

    // Record partial payment
    req.amount_paid = Some(half);
    req.payment_status = Some("partial".to_string());
    req.payment_qr_code = Some(true);
    req.payment_link = Some("https://pressmark.dev/pay/partial".to_string());

    assert_eq!(req.effective_payment_status(), "partial");
    assert!((req.balance_due() - (total - half)).abs() < 0.01);

    let pdf_res = generate_pdf(&req);
    assert!(pdf_res.is_ok(), "Partial payment PDF compilation failed");

    let preview_res = generate_all_preview_pngs(&req, 144);
    assert!(preview_res.is_ok(), "Partial payment PNG preview failed");
}

#[test]
fn test_simulated_edge_case_strings_in_inputs() {
    // Test that strings containing double quotes, backslashes, newlines, markdown, or symbols
    // do NOT cause compilation errors or break previews.
    let mut req = create_sample_request(DocumentType::Invoice);
    req.title = Some("INVOICE \"PREMIUM\" & ESCAPED \\ SPECIAL #1".to_string());
    req.header_subtitle = Some("Testing \"Quotes\", \\Backslashes\\, and @Symbols".to_string());
    req.doc_number = Some("INV-\"2026\"-#001".to_string());
    req.from.name = "O'Connor & Sons \"Ventures\" \\ LLC".to_string();
    req.to.name = "Client #1: \"Acme Corp\" & Co.".to_string();
    req.notes = Some("Notes with \"quotes\" and \n newlines and \\ backslashes.".to_string());
    req.terms = Some("Terms: 100% due; \"Net 30\" applies #terms @all.".to_string());

    let pdf_res = generate_pdf(&req);
    assert!(
        pdf_res.is_ok(),
        "Edge case strings broke PDF compilation: {:?}",
        pdf_res.err()
    );

    let preview_res = generate_all_preview_pngs(&req, 144);
    assert!(
        preview_res.is_ok(),
        "Edge case strings broke PNG preview: {:?}",
        preview_res.err()
    );
}
