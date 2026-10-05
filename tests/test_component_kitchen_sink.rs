//! Pressmark Vector Engine: Component Kitchen Sink & Living Style Guide Test
//!
//! Author: Daniel Hallman
//!
//! Inspired by the Untax Document Engine design system and judicial invariants:
//! 1. Verifies all atomic primitives (badges, status pills, statutory boxes, financial grids, walk-in boxes, checklists)
//! 2. Verifies all 3 execution modes (WetInk with date-stamp zone, Esign with ESIGN Act audit badge, NotarizedRon with notary seal box)
//! 3. Verifies multi-tenant white-labeling personas (PressmarkDefault, CorporateWhiteLabel, LegalCounsel, GovernmentInstitutional)
//! 4. Verifies type-safe fluent `PressmarkDocumentBuilder`
//! 5. Compiles output PDF and PNG raster pages in `target/visual_gallery/component_kitchen_sink.pdf`

use std::fs;
use std::path::PathBuf;
use pressmark::branding::{BrandingMode, DocumentBrandingConfig};
use pressmark::builder::{
    AuditSlugConfig, ClerkChecklistConfig, ExecutionMode, FinancialDeltaGridConfig,
    JuratConfig, MastheadConfig, PressmarkDocumentBuilder, TaxLevyDelta, WalkInBoxConfig,
};
use pressmark::engine::find_typst_executable;

fn setup_gallery_dir() -> PathBuf {
    let dir = PathBuf::from("target/visual_gallery");
    let _ = fs::create_dir_all(&dir);
    dir
}

#[test]
fn test_component_kitchen_sink_pdf_and_png() {
    if find_typst_executable().is_none() {
        eprintln!("SKIPPING test_component_kitchen_sink_pdf_and_png: Vector compiler binary not installed");
        return;
    }

    let gallery_dir = setup_gallery_dir();
    let pdf_path = gallery_dir.join("component_kitchen_sink.pdf");

    let doc = PressmarkDocumentBuilder::new()
        .title("PRESSMARK VECTOR ENGINE · COMPONENT KITCHEN SINK")
        .subtitle("Living Style Guide & Institutional Vector Design Specification")
        .jurisdiction("COMMERCIAL & REGULATORY DOCUMENTATION DIVISION")
        .docket("SPEC-2026-CATALOG")
        .reference_id("REF-742-EVG")
        .tax_year("2026")
        .branding(DocumentBrandingConfig::pressmark_default())
        .add_masthead(MastheadConfig {
            jurisdiction: "PRESSMARK VECTOR ENGINE · DESIGN SPECIFICATION".to_string(),
            document_title: "COMPONENT KITCHEN SINK & LIVING STYLE GUIDE".to_string(),
            subtitle: "Comprehensive Living Catalog for Enterprise, Commercial & Legal Vector Instruments".to_string(),
            docket: Some("SPEC-2026-CATALOG".to_string()),
            reference_id: "REF-742-EVG".to_string(),
            date_str: "2026-10-01".to_string(),
        })
        .add_statutory_box(
            "UCC § 2-201 / UNIFORM COMMERCIAL CODE & ESIGN ACT 15 U.S.C. § 7001",
            "Statutory Commercial Record & Self-Authenticating Invariant Standard",
            "This document catalog demonstrates the standardized design tokens, atomic layout primitives, financial delta grids, three-way execution modes, and tamper-evident audit slugs enforced across all Pressmark archetypes.",
        )
        .add_financial_delta_grid(FinancialDeltaGridConfig {
            current_appraised: "$485,000.00".to_string(),
            proposed_appraised: "$392,000.00".to_string(),
            current_assessed: "$121,250.00".to_string(),
            proposed_assessed: "$98,000.00".to_string(),
            net_appraised_reduction: "-$93,000.00 (-19.2%)".to_string(),
            net_assessed_reduction: "-$23,250.00".to_string(),
            annual_savings: "$1,082.67 / yr".to_string(),
            ratio: "25%".to_string(),
            levy_breakdown: vec![
                TaxLevyDelta {
                    jurisdiction: "Municipal Infrastructure Utility Service".to_string(),
                    millage_rate: "2.2373".to_string(),
                    billed_tax: "$2,712.72".to_string(),
                    contended_tax: "$2,192.55".to_string(),
                    net_savings: "-$520.17".to_string(),
                },
                TaxLevyDelta {
                    jurisdiction: "Regional Data Telemetry Assessment".to_string(),
                    millage_rate: "1.8000".to_string(),
                    billed_tax: "$2,182.50".to_string(),
                    contended_tax: "$1,764.00".to_string(),
                    net_savings: "-$418.50".to_string(),
                },
            ],
        })
        .add_walk_in_box(WalkInBoxConfig::default())
        .add_clerk_checklist(ClerkChecklistConfig {
            title: "OFFICIAL SUBMISSION CHECKLIST (FOR COUNTERPARTY & ARCHIVE)".to_string(),
            items: vec![
                "Original Signed Commercial Instrument (Owner Attestation Executed)".to_string(),
                "Schedule A: Certified Valuation & Performance Ledger".to_string(),
                "Schedule B: Cryptographic Verification Audit Certificate citing 15 U.S.C. § 7001".to_string(),
                "Two (2) identical copies for County Clerk / Notarial Date Stamp Receipt".to_string(),
            ],
        })
        .add_page_break()
        // Page 2: Demonstrate 3 Execution Modes
        .add_raw("#v(10pt)\n#text(font: font-sans, size: 10pt, weight: \"bold\", fill: navy-judicial)[DEMONSTRATION: 3-WAY EXECUTION STATE MACHINE]\n#v(4pt)")
        // Mode 1: Wet-Ink with Date Stamp clearance box
        .add_jurat(JuratConfig {
            heading: "EXECUTION MODE 1: PHYSICAL WET-INK & CLERK DATE-STAMP".to_string(),
            attestation: "I hereby attest that this instrument shall be physically executed in ink and hand-delivered for mechanical date-stamp registration.".to_string(),
            notary_required: false,
            pro_se: true,
            signer_title: "Managing Partner".to_string(),
            signer_capacity: "Managing Partner".to_string(),
            seal_box: false,
            cross_reference: "EXHIBIT WET-INK".to_string(),
            signature_type: ExecutionMode::WetInk,
            signer_name: "Daniel Hallman".to_string(),
            signature_img: None,
            execution_date: String::new(),
            esign_audit_id: String::new(),
            esign_timestamp: String::new(),
            esign_ip: String::new(),
            ron_statute_citation: String::new(),
            ron_session_id: String::new(),
            include_clerk_stamp: true,
            state_name: "Tennessee".to_string(),
            county_name: "Hamilton".to_string(),
        })
        .add_raw("#v(10pt)")
        // Mode 2: E-Sign with ESIGN Act audit badge
        .add_jurat(JuratConfig {
            heading: "EXECUTION MODE 2: CERTIFIED ELECTRONIC EXECUTION (15 U.S.C. § 7001)".to_string(),
            attestation: "I declare under penalty of perjury that I am authorized to digitally execute this instrument via certified cryptographic signature.".to_string(),
            notary_required: false,
            pro_se: true,
            signer_title: "Chief Executive Officer".to_string(),
            signer_capacity: "CEO / Managing Director".to_string(),
            seal_box: false,
            cross_reference: "EXHIBIT ESIGN-AUDIT".to_string(),
            signature_type: ExecutionMode::Esign,
            signer_name: "Alexandria Vance".to_string(),
            signature_img: None,
            execution_date: "10/01/2026".to_string(),
            esign_audit_id: "PRESSMARK-ESIGN-8492-7A1B".to_string(),
            esign_timestamp: "2026-10-01T02:15:32Z".to_string(),
            esign_ip: "198.51.100.42".to_string(),
            ron_statute_citation: String::new(),
            ron_session_id: String::new(),
            include_clerk_stamp: false,
            state_name: "California".to_string(),
            county_name: "San Francisco".to_string(),
        })
        .add_raw("#v(10pt)")
        // Mode 3: Remote Online Notary (RON) with Notary Seal box
        .add_jurat(JuratConfig {
            heading: "EXECUTION MODE 3: REMOTE ONLINE NOTARY (RON) ACKNOWLEDGMENT".to_string(),
            attestation: "I swear under penalty of perjury that this statutory instrument was executed before a commissioned notary public via interactive audio-video communication.".to_string(),
            notary_required: true,
            pro_se: false,
            signer_title: "Property Principal".to_string(),
            signer_capacity: "Sole Proprietor".to_string(),
            seal_box: true,
            cross_reference: "EXHIBIT NOTARIZED-OATH".to_string(),
            signature_type: ExecutionMode::NotarizedRon,
            signer_name: "Marcus Sterling".to_string(),
            signature_img: None,
            execution_date: "10/01/2026".to_string(),
            esign_audit_id: "PRESSMARK-RON-9921-8B2C".to_string(),
            esign_timestamp: "2026-10-01T02:22:11Z".to_string(),
            esign_ip: "203.0.113.88".to_string(),
            ron_statute_citation: "T.C.A. § 8-16-301 et seq.".to_string(),
            ron_session_id: "SESSION-RON-2026-4491".to_string(),
            include_clerk_stamp: false,
            state_name: "Tennessee".to_string(),
            county_name: "Hamilton".to_string(),
        })
        .add_audit_slug(AuditSlugConfig {
            reference_id: "REF-742-EVG".to_string(),
            hash: "c4ca4238a0b923820dcc509a6f75849b84927a1b8c8d9e0f1a2b3c4d5e6f7a8b".to_string(),
            engine_version: "Pressmark Vector Engine v1.0.0".to_string(),
            timestamp: "2026-10-01T02:30:00Z".to_string(),
        });

    let built = doc.build();
    assert!(built.markup.contains("#masthead("));
    assert!(built.markup.contains("#statutory-box("));
    assert!(built.markup.contains("#financial-delta-grid("));
    assert!(built.markup.contains("#walk-in-box("));
    assert!(built.markup.contains("#clerk-checklist("));
    assert!(built.markup.contains("#jurat-box("));
    assert!(built.markup.contains("#audit-slug("));

    // Compile to PDF
    let compiled_pdf = doc.compile_pdf(&pdf_path).expect("Failed to compile Component Kitchen Sink to PDF");
    assert!(compiled_pdf.exists(), "PDF must exist on disk");
    let pdf_bytes = fs::read(&compiled_pdf).expect("Failed to read PDF bytes");
    assert!(pdf_bytes.len() > 5000, "Component Kitchen Sink PDF must be substantial (> 5KB)");
    println!("SUCCESS: Rendered Component Kitchen Sink PDF ({} bytes) -> {:?}", pdf_bytes.len(), pdf_path);

    // Compile to PNG raster images
    let png_bytes = doc.compile_png_bytes(144).expect("Failed to compile Component Kitchen Sink to PNG");
    assert!(!png_bytes.is_empty(), "PNG bytes must be non-empty");
    let png_path = gallery_dir.join("component_kitchen_sink_page_1.png");
    fs::write(&png_path, png_bytes).expect("Failed to write PNG preview");
    println!("SUCCESS: Rendered Component Kitchen Sink PNG -> {:?}", png_path);
}

#[test]
fn test_all_white_label_branding_personas() {
    if find_typst_executable().is_none() {
        return;
    }

    let gallery_dir = setup_gallery_dir();

    let personas = vec![
        ("default", DocumentBrandingConfig::pressmark_default(), BrandingMode::PressmarkDefault),
        ("corporate", DocumentBrandingConfig::corporate_white_label("Apex Aerospace Technologies Corp", Some("Defense Systems Division")), BrandingMode::CorporateWhiteLabel),
        ("legal", DocumentBrandingConfig::legal_counsel("Cumberland Legal Counsel, PLLC", Some("SALT & Commercial Practice"), Some("Bar #038491")), BrandingMode::LegalCounsel),
        ("government", DocumentBrandingConfig::government_institutional(), BrandingMode::GovernmentInstitutional),
    ];

    for (name, branding, expected_mode) in personas {
        assert_eq!(branding.mode, expected_mode);
        let pdf_path = gallery_dir.join(format!("branding_persona_{}.pdf", name));
        let builder = PressmarkDocumentBuilder::new()
            .title(&format!("OFFICIAL FILING · {}", name.to_uppercase()))
            .branding(branding)
            .add_statutory_box("COMMERCIAL REGULATION 2026", "Binding Legal Accord", "Demonstrating white-label multi-tenant branding isolation.")
            .add_jurat(JuratConfig {
                heading: "OFFICIAL ATTESTATION".to_string(),
                signer_title: "Managing Authority".to_string(),
                signature_type: ExecutionMode::Esign,
                signer_name: "Daniel Hallman".to_string(),
                ..Default::default()
            });

        let pdf = builder.compile_pdf(&pdf_path).expect("Failed to compile branding persona");
        assert!(pdf.exists());
        println!("SUCCESS: Rendered Branding Persona '{}' -> {:?}", name, pdf_path);
    }
}
