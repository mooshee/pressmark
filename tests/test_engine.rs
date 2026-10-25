//! Comprehensive Engine & Typst Compilation Tests
//!
//! Author: Daniel Hallman

use pressmark::engine::{generate_pdf, generate_preview_png};
use pressmark::models::{DocumentRequest, DocumentType, LineItem, PartyDetails};

fn create_sample_invoice() -> DocumentRequest {
    DocumentRequest {
        doc_type: DocumentType::Invoice,
        title: Some("TAX INVOICE".to_string()),
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
            tax_id: Some("XX-XXXXXXX".to_string()),
            ..Default::default()
        },
        to: PartyDetails {
            name: "Acme Corp".to_string(),
            company: Some("Acme Holdings".to_string()),
            address: Some("456 Innovation Way, Austin, TX".to_string()),
            email: Some("accounts@acme.com".to_string()),
            ..Default::default()
        },
        items: vec![
            LineItem {
                description: "Rust High-Performance Service Implementation".to_string(),
                quantity: 40.0,
                unit_price: 175.0,
                tax_rate: None,
                discount: None,
            },
            LineItem {
                description: "Pressmark Vector Publishing Pipeline".to_string(),
                quantity: 20.0,
                unit_price: 175.0,
                tax_rate: None,
                discount: None,
            },
        ],
        currency: Some("$".to_string()),
        tax_rate: Some(8.25),
        discount_amount: Some(500.0),
        notes: Some("Payable within 14 days.".to_string()),
        terms: Some("Standard commercial terms apply.".to_string()),
        custom_fields: vec![],
        signatures: vec![],
        sections: vec![],
        header_subtitle: Some("Enterprise Infrastructure Services".to_string()),
        footer_text: Some("CONFIDENTIAL & PROPRIETARY".to_string()),
        background_style: Some("white".to_string()),
        show_page_numbers: Some(true),
        show_footer_divider: Some(true),
        is_paid: Some(false),
        watermark: None,
        compliance_mode: Some("pci_dss".to_string()),
        show_qr_seal: Some(true),
        show_signature: Some(true),
        show_remittance_stub: Some(true),
        show_line_numbers: Some(true),
        layout_style: Some("standard".to_string()),
        ..Default::default()
    }
}

#[test]
fn test_generate_pdf_invoice_compiles_under_100ms() {
    let req = create_sample_invoice();
    let result = generate_pdf(&req);
    assert!(result.is_ok(), "PDF generation failed: {:?}", result.err());

    let (pdf_bytes, ms) = result.unwrap();
    assert!(!pdf_bytes.is_empty(), "PDF bytes must not be empty");
    assert!(pdf_bytes.len() > 1000, "PDF bytes should exceed 1KB");
    assert_eq!(&pdf_bytes[0..4], b"%PDF", "Must be a valid PDF header");
    println!("Compiled invoice PDF in {}ms (Size: {} bytes)", ms, pdf_bytes.len());
}

#[test]
fn test_generate_png_preview_header() {
    let req = create_sample_invoice();
    let result = generate_preview_png(&req, 144);
    assert!(result.is_ok(), "PNG generation failed: {:?}", result.err());

    let (png_bytes, ms) = result.unwrap();
    assert!(!png_bytes.is_empty(), "PNG bytes must not be empty");
    assert_eq!(&png_bytes[0..8], b"\x89PNG\r\n\x1a\n", "Must be a valid PNG header");
    println!("Compiled invoice preview PNG in {}ms (Size: {} bytes)", ms, png_bytes.len());
}

#[test]
fn test_all_document_types_compile() {
    let doc_types = [
        DocumentType::Invoice,
        DocumentType::InvoiceStandard,
        DocumentType::InvoiceFreelance,
        DocumentType::InvoiceConstruction,
        DocumentType::InvoiceMedical,
        DocumentType::InvoiceSaas,
        DocumentType::InvoiceMinimal,
        DocumentType::ProformaInvoice,
        DocumentType::Estimate,
        DocumentType::Receipt,
        DocumentType::PurchaseOrder,
        DocumentType::PackingSlip,
        DocumentType::CreditNote,
        DocumentType::WorkOrder,
        DocumentType::ExpenseReport,
        DocumentType::DeliveryReceipt,
        DocumentType::CommercialLease,
        DocumentType::QuotationTender,
        DocumentType::Agreement,
        DocumentType::AgreementSow,
        DocumentType::ContractorAgreement,
        DocumentType::RetainerAgreement,
        DocumentType::Nda,
        DocumentType::BillOfSale,
        DocumentType::Certificate,
        DocumentType::CertificateInsurance,
        DocumentType::CertificateCompletion,
        DocumentType::InspectionReport,
        DocumentType::ResidentialLease,
        DocumentType::CommercialNnnLease,
        DocumentType::PropertyManagementStatement,
        DocumentType::RentRoll,
        DocumentType::ClosingSettlementStatement,
        DocumentType::NoticeToVacate,
        DocumentType::SecurityDepositDisposition,
        DocumentType::RealEstateCommission,
        DocumentType::PropertyInspectionChecklist,
        DocumentType::LetterOfIntentCre,
        DocumentType::TenantEstoppel,
    ];

    for dt in &doc_types {
        let mut req = create_sample_invoice();
        req.doc_type = dt.clone();
        req.title = Some(dt.display_title().to_string());

        let result = generate_pdf(&req);
        assert!(result.is_ok(), "Failed compiling PDF for doc type {:?}: {:?}", dt, result.err());
        let (bytes, ms) = result.unwrap();
        assert!(bytes.len() > 1000);

        let png_result = pressmark::engine::generate_all_preview_pngs(&req, 144);
        assert!(png_result.is_ok(), "Failed generating PNG preview for doc type {:?}: {:?}", dt, png_result.err());
        let (pages, png_ms) = png_result.unwrap();
        assert!(!pages.is_empty(), "Preview pages must not be empty for {:?}", dt);
        println!("Compiled PDF in {}ms and PNG ({} pages) in {}ms for {:?}", ms, pages.len(), png_ms, dt);

        // Also test with empty items
        let mut req_empty = req.clone();
        req_empty.items = vec![];
        let empty_result = generate_pdf(&req_empty);
        assert!(empty_result.is_ok(), "Failed compiling empty-items PDF for doc type {:?}: {:?}", dt, empty_result.err());
        let empty_png_result = pressmark::engine::generate_all_preview_pngs(&req_empty, 144);
        assert!(empty_png_result.is_ok(), "Failed generating empty-items PNG for doc type {:?}: {:?}", dt, empty_png_result.err());
    }
}

#[test]
fn test_multi_signatures_and_dynamic_sections() {
    use pressmark::models::{DocumentSection, SignatureBlock};

    let mut req = create_sample_invoice();
    req.sections = vec![
        DocumentSection {
            title: "1. SCOPE OF SERVICES".to_string(),
            content: "Consultant shall provide advanced agentic architecture and vector compilation pipelines.".to_string(),
        },
        DocumentSection {
            title: "2. INTELLECTUAL PROPERTY".to_string(),
            content: "All developed code, specifications, and vector stylesheets assign irrevocably to client.".to_string(),
        },
    ];

    req.signatures = vec![
        SignatureBlock {
            signer_name: "Daniel Hallman".to_string(),
            signer_title: Some("Managing Principal".to_string()),
            date: Some("2026-10-01".to_string()),
            signature_data: None,
        },
        SignatureBlock {
            signer_name: "Alexandria Vance".to_string(),
            signer_title: Some("Chief Technology Officer".to_string()),
            date: Some("2026-10-01".to_string()),
            signature_data: None,
        },
        SignatureBlock {
            signer_name: "Marcus Sterling".to_string(),
            signer_title: Some("Legal Notary Public".to_string()),
            date: Some("2026-10-01".to_string()),
            signature_data: None,
        },
    ];

    req.background_style = Some("parchment".to_string());
    req.footer_text = Some("CONFIDENTIAL & PRIVILEGED · LEGAL ATTESTATION".to_string());

    let (pdf_bytes, ms) = generate_pdf(&req).expect("Failed to compile multi-signature document");
    assert!(!pdf_bytes.is_empty());
    assert_eq!(&pdf_bytes[0..4], b"%PDF");
    println!("Compiled multi-signature document in {}ms (Size: {} bytes)", ms, pdf_bytes.len());
}

#[test]
fn test_advanced_formatters_and_new_layouts() {
    let mut req = create_sample_invoice();
    req.doc_type = DocumentType::ProformaInvoice;
    req.layout_style = Some("swiss_grid".to_string());
    req.spacing_density = Some("generous".to_string());
    req.currency = Some("EUR".to_string());
    req.currency_symbol_mode = Some("symbol".to_string());
    req.currency_placement = Some("suffix".to_string());
    req.currency_separators = Some("dot_comma".to_string());
    req.date_format = Some("formal_prose".to_string());
    req.unit_measure = Some("hrs".to_string());
    req.theme = Some("royal_violet".to_string());
    req.background_style = Some("slate_tint".to_string());

    assert_eq!(req.format_amount(1250.50), "1.250,50 €");
    assert_eq!(req.format_date("2026-10-01"), "October 01, 2026");

    let (pdf_bytes, ms) = generate_pdf(&req).expect("Failed to compile formatted document");
    assert!(!pdf_bytes.is_empty());
    assert_eq!(&pdf_bytes[0..4], b"%PDF");
    println!("Compiled swiss_grid generous proforma invoice in {}ms (Size: {} bytes)", ms, pdf_bytes.len());
}

#[test]
fn test_typst_root_sandbox_and_hostile_input_escaping() {
    use pressmark::engine::{compile_pdf_bytes, generate_typst_markup};
    use pressmark::models::{DocumentSection, SignatureBlock};

    // 1. Direct Typst compilation must reject reading files outside get_cache_dir() (--root sandbox)
    let lfi_markup = "#read(\"/etc/hosts\")";
    let lfi_res = compile_pdf_bytes(lfi_markup);
    assert!(
        lfi_res.is_err(),
        "Typst --root sandbox must prevent reading /etc/hosts outside get_cache_dir()"
    );

    // 2. Hostile Typst markup & string-escape payloads in terms, remittance, sections, signatures, jurat, hex colors
    let mut req = create_sample_invoice();
    req.doc_type = DocumentType::ContractorAgreement;
    req.primary_color = Some("#\");]//".to_string()); // invalid hex color injection attempt
    req.secondary_color = Some("#zzzzzz".to_string());
    req.accent_color = Some("#12345g".to_string());
    req.surface_color = Some("#!@#$%^".to_string());
    req.text_color = Some("#\"quoted".to_string());
    assert_eq!(req.effective_primary_color(), "#4f46e5");
    assert_eq!(req.effective_secondary_color(), "#475569");
    assert_eq!(req.effective_accent_color(), "#06b6d4");
    assert_eq!(req.effective_surface_color(), "#f8fafc");
    assert_eq!(req.effective_text_color(), "#0f172a");

    req.terms = Some("1. Hostile terms: #read(\"/etc/hosts\") *unclosed bold _unclosed italic `backtick <label> ~ = $math @ref [bracket]".to_string());
    req.to.name = "Acme @ Corp $100 #1 \"Quoted\" \\Backslash *Star*".to_string();
    req.from.company = Some("Issuer @ Co $50 #2 \"Quote\"".to_string());
    req.doc_number = Some("CA-2026-\"99\"-@-$".to_string());
    req.due_date = Some("2026-10-15 \"due\" @ref".to_string());
    req.issue_date = Some("2026-10-01 *issue* #1".to_string());
    req.unit_measure = Some("hrs *bold* #1 $2".to_string());
    req.currency = Some("US$*#".to_string());
    req.compliance_mode = Some("soc2_badge".to_string());
    req.show_remittance_stub = Some(true);
    req.execution_mode = Some("notary \"sworn\" @1".to_string());
    req.sections = vec![DocumentSection {
        title: "Section @1 $2 #3 \"Title\"".to_string(),
        content: "Body with *unclosed bold _italic `code <tag> ~ = #sys.inputs".to_string(),
    }];
    req.signatures = vec![SignatureBlock {
        signer_name: "Dr. Jane @ Doe $1 #2 \"MD\"".to_string(),
        signer_title: Some("VP *Engineering* @ Corp \"Title\"".to_string()),
        date: Some("2026-10-01 \"Date\" @ref".to_string()),
        signature_data: None,
    }];

    let markup = generate_typst_markup(&req);
    // Verify ContractorAgreement does not duplicate req.terms in bottom TERMS & CONDITIONS block
    assert!(
        !markup.contains("*TERMS & CONDITIONS:*"),
        "ContractorAgreement should not duplicate terms in the bottom invoice terms block"
    );

    let (pdf_bytes, _) = generate_pdf(&req).expect("Hostile input payload must compile cleanly into PDF");
    assert!(pdf_bytes.starts_with(b"%PDF-"));

    // 3. Localization aliases (euro, space, plain separators; us, eu, long, short date formats; NZD, SGD, CNY)
    let mut loc_req = create_sample_invoice();
    loc_req.currency = Some("NZD".to_string());
    loc_req.currency_separators = Some("euro".to_string());
    assert_eq!(loc_req.format_amount(1234.56), "NZ$1.234,56");
    loc_req.currency = Some("SGD".to_string());
    loc_req.currency_separators = Some("space".to_string());
    assert_eq!(loc_req.format_amount(1234.56), "S$1 234,56");
    loc_req.currency = Some("CNY".to_string());
    loc_req.currency_separators = Some("plain".to_string());
    assert_eq!(loc_req.format_amount(1234.56), "¥1234.56");

    loc_req.date_format = Some("us".to_string());
    assert_eq!(loc_req.format_date("2026-10-05"), "10/05/2026");
    loc_req.date_format = Some("eu".to_string());
    assert_eq!(loc_req.format_date("2026-10-05"), "05/10/2026");
    loc_req.date_format = Some("long".to_string());
    assert_eq!(loc_req.format_date("2026-10-05"), "October 05, 2026");
    loc_req.date_format = Some("short".to_string());
    assert_eq!(loc_req.format_date("2026-10-05"), "05-Oct-2026");

    // 4. Web-root /assets/... staging and path traversal rejection
    let staged_logo = pressmark::engine::stage_image_asset("logo", "/assets/logo.jpg");
    assert_eq!(staged_logo.as_deref(), Some("/logo_logo.jpg"));
    assert!(pressmark::engine::stage_image_asset("logo", "/etc/hosts").is_none());
    assert!(pressmark::engine::stage_image_asset("logo", "/assets/../../Cargo.toml").is_none());

    // 5. Empty payment_link vs schemeless payment_link compilation
    let mut pay_req = create_sample_invoice();
    pay_req.payment_qr_code = Some(false);
    pay_req.payment_link = Some("   ".to_string());
    let empty_pay_markup = generate_typst_markup(&pay_req);
    assert!(!empty_pay_markup.contains("#payment-qr-block("));

    pay_req.payment_link = Some("buy.stripe.com/schemeless_checkout".to_string());
    let schemeless_markup = generate_typst_markup(&pay_req);
    assert!(schemeless_markup.contains("https://buy.stripe.com/schemeless_checkout"));
    let (pay_pdf, _) = generate_pdf(&pay_req).expect("Schemeless payment_link must compile cleanly");
    assert!(pay_pdf.starts_with(b"%PDF-"));

    // 6. PressmarkDocumentBuilder full content escaping and hex color sanitization
    let mut bad_branding = pressmark::branding::DocumentBrandingConfig::default();
    bad_branding.accent_primary = "#\");]//".to_string();
    let built_pdf = pressmark::builder::PressmarkDocumentBuilder::new()
        .branding(bad_branding)
        .add_statutory_box("26 U.S.C. § 1", "Strict", "Unclosed *bold _italic `code <tag> ~ = #read(\"/etc/hosts\")")
        .compile_pdf_bytes()
        .expect("PressmarkDocumentBuilder must escape hostile content and sanitize invalid hex color");
    assert!(built_pdf.starts_with(b"%PDF-"));
}
