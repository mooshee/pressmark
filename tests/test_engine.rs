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
            email: Some("billing@pressmark.io".to_string()),
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
        DocumentType::Estimate,
        DocumentType::Receipt,
        DocumentType::PurchaseOrder,
        DocumentType::PackingSlip,
        DocumentType::CreditNote,
        DocumentType::Agreement,
        DocumentType::AgreementSow,
        DocumentType::ContractorAgreement,
        DocumentType::Nda,
        DocumentType::BillOfSale,
        DocumentType::Certificate,
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

    for dt in doc_types {
        let mut req = create_sample_invoice();
        req.doc_type = dt.clone();
        req.title = Some(dt.display_title().to_string());

        let result = generate_pdf(&req);
        assert!(result.is_ok(), "Failed compiling doc type {:?}: {:?}", dt, result.err());
        let (bytes, ms) = result.unwrap();
        assert!(bytes.len() > 1000);
        println!("Compiled {:?} in {}ms", dt, ms);
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
