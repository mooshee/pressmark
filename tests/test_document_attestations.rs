//! Regression checks for supplied signing facts and blank document defaults.
//!
//! Author: Daniel Hallman

use pressmark::builder::{AuditSlugConfig, JuratConfig, MastheadConfig, PressmarkDocumentBuilder};
use pressmark::engine::{generate_pdf, generate_preview_png, generate_typst_markup};
use pressmark::models::{DocumentRequest, DocumentType};
use pressmark::templates::TYPST_THEME;
use serde_json::json;

fn sample() -> DocumentRequest {
    serde_json::from_value(json!({
        "doc_type": "invoice", "issue_date": "2026-10-29",
        "from": { "name": "Sample Studio" }, "to": { "name": "Sample Client" },
        "items": [{ "description": "Design services", "quantity": 3, "unit_price": 100 }]
    }))
    .unwrap()
}

fn rendered_body(req: &DocumentRequest) -> String {
    generate_typst_markup(req)
        .strip_prefix(TYPST_THEME)
        .unwrap()
        .to_string()
}

fn save_evidence(name: &str, extension: &str, bytes: &[u8]) {
    if let Some(dir) = std::env::var_os("PRESSMARK_SIGNING_EVIDENCE_DIR") {
        let dir = std::path::PathBuf::from(dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(format!("{name}.{extension}")), bytes).unwrap();
    }
}

#[test]
fn execution_layouts_do_not_invent_signing_events_or_jurisdictions() {
    for mode in ["esign", "wet_ink", "ron"] {
        let mut req = sample();
        req.execution_mode = Some(mode.to_string());
        let body = rendered_body(&req);
        let jurat = body.split("#jurat-box(").nth(1).unwrap();
        for unwanted in [
            "PRESSMARK-ESIGN-",
            "RON-SESSION-",
            "2026-10-01",
            "192.0.2.14",
            "Tennessee",
            "Hamilton",
            "2026-10-29",
        ] {
            assert!(
                !jurat.contains(unwanted),
                "invented execution field in {mode}: {unwanted}"
            );
        }
        assert!(jurat.contains("signer-name: \"\""));
        assert!(jurat.contains("execution-date: \"\""));
        assert!(jurat.contains("state: \"\""));
        assert!(jurat.contains("county: \"\""));
        let (pdf, _) = generate_pdf(&req).expect("execution PDF compiles");
        assert!(pdf.starts_with(b"%PDF-"));
        let (png, _) = generate_preview_png(&req, 120).expect("execution preview compiles");
        assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));
        save_evidence(mode, "pdf", &pdf);
        save_evidence(mode, "png", &png);
    }
}

#[test]
fn supplied_signature_dates_and_names_remain_separate_from_the_issue_date() {
    let mut req = sample();
    req.signer_name = Some("Sample Signer".to_string());
    assert_eq!(req.effective_signatures()[0].date, None);
    assert!(rendered_body(&req).contains("date: \"\""));
    req.execution_mode = Some("esign".to_string());
    req.signatures = serde_json::from_value(json!([{
        "signer_name": "Sample Signer", "date": "2026-10-28", "signer_title": "Owner"
    }]))
    .unwrap();
    let body = rendered_body(&req);
    assert!(body.contains("signer-name: \"Sample Signer\""));
    assert!(body.contains("execution-date: \"2026-10-28\""));
    let (png, _) = generate_preview_png(&req, 120).unwrap();
    save_evidence("supplied-signer", "png", &png);
    let (pdf, _) = generate_pdf(&req).unwrap();
    save_evidence("supplied-signer", "pdf", &pdf);
}

#[test]
fn all_archetypes_omit_fixed_signing_and_certification_metadata() {
    for doc_type in DocumentType::all() {
        let mut req = sample();
        req.doc_type = doc_type.clone();
        req.issue_date = None;
        let body = rendered_body(&req);
        for unwanted in [
            "PRESSMARK-ESIGN-8492-7A1B",
            "RON-SESSION-2026-9941",
            "192.0.2.14",
            "2026-10-01T02:00:00Z",
            "APN-VERIFIED",
            "CERTIFIED TENANT",
            "Certified Commercial Record",
            "OFFICIALLY ISSUED",
        ] {
            assert!(
                !body.contains(unwanted),
                "{:?} contains invented metadata: {unwanted}",
                doc_type
            );
        }
    }
}

#[test]
fn builder_defaults_are_blank_and_supplied_records_are_preserved() {
    let jurat = JuratConfig::default();
    assert!(jurat.signer_name.is_empty());
    assert!(jurat.execution_date.is_empty());
    assert!(jurat.esign_audit_id.is_empty());
    assert!(jurat.esign_timestamp.is_empty());
    assert!(jurat.esign_ip.is_empty());
    assert!(jurat.county_name.is_empty());
    let audit = AuditSlugConfig::default();
    assert!(audit.hash.is_empty());
    assert!(audit.timestamp.is_empty());
    let masthead = MastheadConfig::default();
    assert!(masthead.date_str.is_empty());
    assert!(masthead.reference_id.is_empty());

    let builder = PressmarkDocumentBuilder::new().add_jurat(JuratConfig {
        signer_name: "Sample Signer".to_string(),
        esign_audit_id: "SUPPLIED-RECORD-42".to_string(),
        esign_timestamp: "2026-10-28T18:30:00Z".to_string(),
        esign_ip: "192.0.2.99".to_string(),
        ..Default::default()
    });
    let built = builder.build();
    for supplied in [
        "Sample Signer",
        "SUPPLIED-RECORD-42",
        "2026-10-28T18:30:00Z",
        "192.0.2.99",
    ] {
        assert!(built.markup.contains(supplied));
    }
    let pdf = builder.compile_pdf_bytes().unwrap();
    let png = builder.compile_png_bytes(120).unwrap();
    save_evidence("supplied-record", "pdf", &pdf);
    save_evidence("supplied-record", "png", &png);
}

#[test]
fn missing_payment_links_and_due_dates_do_not_create_sample_instructions() {
    let mut req = sample();
    req.payment_qr_code = Some(true);
    req.show_qr_seal = Some(true);
    req.show_remittance_stub = Some(true);
    let body = rendered_body(&req);
    assert!(!body.contains("#payment-qr-block("));
    assert!(!body.contains("demo_checkout"));
    assert!(!body.contains("2026-10-15"));
    assert!(body.contains("#verification-seal("));
    assert!(body.contains("due-date: \"\""));
}

#[test]
fn payment_and_service_records_only_use_supplied_event_details() {
    let mut req = sample();
    req.is_paid = Some(true);
    let body = rendered_body(&req);
    assert!(body.contains("paid-watermark-stamp(tx-ref: \"\", paid-date: \"\", method: \"\")"));
    req.payment_method = Some("Check".to_string());
    req.paid_at = Some("2026-10-28".to_string());
    req.transaction_reference = Some("SUPPLIED-CHECK-42".to_string());
    let body = rendered_body(&req);
    assert!(body.contains("paid-watermark-stamp(tx-ref: \"SUPPLIED-CHECK-42\", paid-date: \"2026-10-28\", method: \"Check\")"));

    let mut notice = sample();
    notice.doc_type = DocumentType::NoticeToVacate;
    let body = rendered_body(&notice);
    let service = body.split("#certificate-of-service(").nth(1).unwrap();
    for expected in [
        "server-name: \"\"",
        "service-date: \"\"",
        "service-method: \"\"",
        "state-name: \"\"",
    ] {
        assert!(
            service.contains(expected),
            "missing blank service field: {expected}"
        );
    }
    notice.custom_fields = serde_json::from_value(json!([
        {"label": "Server Name", "value": "Sample Server"},
        {"label": "Service Date", "value": "2026-10-28"},
        {"label": "Service Method", "value": "Certified Mail"}
    ]))
    .unwrap();
    let body = rendered_body(&notice);
    let service = body.split("#certificate-of-service(").nth(1).unwrap();
    assert!(service.contains("server-name: \"Sample Server\""));
    assert!(service.contains("service-date: \"2026-10-28\""));
    assert!(service.contains("service-method: \"mail\""));
    let (pdf, _) = generate_pdf(&notice).unwrap();
    save_evidence("supplied-service", "pdf", &pdf);
}

#[test]
fn compliance_and_property_templates_do_not_attest_to_missing_facts() {
    for unsupported in [
        "CERTIFIED ELECTRONIC EXECUTION",
        "DIGITALLY VERIFIED DOCUMENT",
        "Executed & Verified",
        "SOC-2 TYPE II AUDITED",
        "EVIDENTIARY PROOF OF SERVICE",
        "FACTUR-X COMPLIANT",
        "KSA E-INVOICING COMPLIANT",
    ] {
        assert!(
            !TYPST_THEME.contains(unsupported),
            "unsupported template claim: {unsupported}"
        );
    }
    let mut req = sample();
    req.doc_type = DocumentType::ResidentialLease;
    req.state_code = Some("CA".to_string());
    req.year_built = Some(1965);
    let body = rendered_body(&req);
    for unsupported in [
        "[X]",
        "Landlord has inspected",
        "Landlord certifies the Premises",
        "FDIC-Insured Depository",
    ] {
        assert!(
            !body.contains(unsupported),
            "unsupported property assertion: {unsupported}"
        );
    }
    let mut invoice = sample();
    invoice.compliance_mode = Some("soc2".to_string());
    invoice.show_qr_seal = Some(true);
    let (pdf, _) = generate_pdf(&invoice).unwrap();
    let (png, _) = generate_preview_png(&invoice, 120).unwrap();
    save_evidence("compliance-note", "pdf", &pdf);
    save_evidence("compliance-note", "png", &png);
}
