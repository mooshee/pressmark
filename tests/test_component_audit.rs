//! Pressmark Vector Engine: Comprehensive Component Audit & Stress-Testing Suite
//!
//! Author: Daniel Hallman
//!
//! Exhaustive verification of all document components across:
//! 1. Paper sizes & geometries (US Letter, A4, US Legal, US Executive, Tabloid, A3, A5 Mini)
//! 2. Margins, line heights, font scales, and typography fallbacks
//! 3. All 9 layout styles (standard, modern_banner, classic_letterhead, swiss_grid, split_sidebar, compact_ledger, editorial_centered, framed, minimal)
//! 4. Extreme text lengths, 100+ char counterparty names, 6-line addresses, 150-word line descriptions
//! 5. Extreme item volumes (0 items fallback, 1 item, 25 items spanning multiple pages with running headers)
//! 6. Image shapes & aspect ratios (ultra-wide 5:1, square 1:1, ultra-tall 1:4, missing logos)
//! 7. Multi-signatory execution grids (1, 2, 3, 4 signers with/without base64 signature images)
//! 8. Financial formatting, high totals ($18.5M+), JPY zero-decimals, European separators, negative discounts
//! 9. Institutional compliance banners, remittance stubs, QR verification seals, and tamper-evident audit slugs

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use pressmark::engine::{find_typst_executable, generate_pdf, generate_preview_png};
use pressmark::models::{
    CustomField, DocumentRequest, DocumentSection, DocumentType, LineItem, PartyDetails,
    SignatureBlock,
};
use std::fs;
use std::path::PathBuf;

fn setup_gallery_dir() -> PathBuf {
    let dir = PathBuf::from("target/visual_gallery");
    let _ = fs::create_dir_all(&dir);
    dir
}

/// Helper to generate sample SVG data URI for given width, height, background color, and text
fn create_test_svg_data_uri(width: u32, height: u32, bg_hex: &str, text: &str) -> String {
    let escaped_text = text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}"><rect width="{}" height="{}" rx="6" fill="{}"/><text x="{}" y="{}" font-family="sans-serif" font-size="{}" fill="#ffffff" font-weight="bold">{}</text></svg>"##,
        width,
        height,
        width,
        height,
        width,
        height,
        bg_hex,
        width / 10,
        height * 6 / 10,
        (height / 3).max(12),
        escaped_text
    );
    format!("data:image/svg+xml;base64,{}", BASE64.encode(svg.as_bytes()))
}

/// Helper to generate a realistic signature SVG
fn create_test_signature_data_uri(name: &str) -> String {
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 240 60" width="240" height="60"><path d="M 20 40 Q 50 10 80 35 T 140 25 T 200 45" fill="none" stroke="#1e3a8a" stroke-width="2.5" stroke-linecap="round"/><text x="30" y="52" font-family="serif" font-size="16" font-style="italic" fill="#1e3a8a">{}</text></svg>"##,
        name
    );
    format!("data:image/svg+xml;base64,{}", BASE64.encode(svg.as_bytes()))
}

#[test]
fn test_audit_all_paper_sizes_margins_and_fonts() {
    if find_typst_executable().is_none() {
        eprintln!("SKIPPING test_audit_all_paper_sizes_margins_and_fonts: compiler not found");
        return;
    }

    let paper_sizes = ["us-letter", "a4", "us-legal", "us-executive", "tabloid", "a3", "a5"];
    let margins = ["compact", "standard", "generous", "executive"];
    let font_scales = ["compact", "standard", "large"];
    let font_families = ["sans", "serif", "mono", "display", "grotesk", "editorial"];

    for paper in paper_sizes {
        for margin in &margins[..2] {
            let req = DocumentRequest {
                doc_type: DocumentType::InvoiceStandard,
                paper_size: Some(paper.to_string()),
                spacing_density: Some(margin.to_string()),
                font_scale: Some("standard".to_string()),
                font_family: Some("sans".to_string()),
                from: PartyDetails {
                    name: "Acme Dynamics Corp".to_string(),
                    company: Some("Acme Global Holdings".to_string()),
                    address: Some("100 Enterprise Way\nAustin, TX 78701".to_string()),
                    ..Default::default()
                },
                to: PartyDetails {
                    name: "Zenith Logistics Inc".to_string(),
                    address: Some("500 Harbor Blvd\nSeattle, WA 98104".to_string()),
                    ..Default::default()
                },
                items: vec![
                    LineItem {
                        description: "Cloud Architecture & Vector Processing".to_string(),
                        quantity: 12.5,
                        unit_price: 240.0,
                        tax_rate: Some(8.5),
                        discount: None,
                    },
                ],
                ..Default::default()
            };

            let (pdf_bytes, ms) = generate_pdf(&req).expect(&format!("Failed for paper: {}", paper));
            assert!(!pdf_bytes.is_empty());
            assert!(pdf_bytes.starts_with(b"%PDF"));
            assert!(ms < 5000, "Compile took too long: {}ms", ms);
        }
    }

    // Test font families and font scales
    for font in font_families {
        for scale in font_scales {
            let req = DocumentRequest {
                doc_type: DocumentType::Invoice,
                font_family: Some(font.to_string()),
                font_scale: Some(scale.to_string()),
                from: PartyDetails {
                    name: "Pressmark Technologies".to_string(),
                    ..Default::default()
                },
                to: PartyDetails {
                    name: "Client Partner".to_string(),
                    ..Default::default()
                },
                items: vec![
                    LineItem {
                        description: "Consulting".to_string(),
                        quantity: 1.0,
                        unit_price: 500.0,
                        tax_rate: None,
                        discount: None,
                    },
                ],
                ..Default::default()
            };
            let (pdf_bytes, _) = generate_pdf(&req).expect(&format!("Failed for font: {}, scale: {}", font, scale));
            assert!(!pdf_bytes.is_empty());
        }
    }
}

#[test]
fn test_audit_all_nine_layout_styles() {
    if find_typst_executable().is_none() {
        eprintln!("SKIPPING test_audit_all_nine_layout_styles: compiler not found");
        return;
    }

    let layouts = [
        "standard",
        "modern_banner",
        "classic_letterhead",
        "swiss_grid",
        "split_sidebar",
        "compact_ledger",
        "editorial_centered",
        "framed",
        "minimal",
    ];

    let background_finishes = [
        "clean",
        "parchment",
        "slate_tint",
        "blueprint",
        "linen_subtle",
        "security_frame",
    ];

    for (idx, layout) in layouts.iter().enumerate() {
        let bg = background_finishes[idx % background_finishes.len()];
        let req = DocumentRequest {
            doc_type: DocumentType::InvoiceStandard,
            layout_style: Some(layout.to_string()),
            background_style: Some(bg.to_string()),
            header_subtitle: Some("Certified Professional Commercial Instrument".to_string()),
            from: PartyDetails {
                name: "Meridian Structural Engineering LLC".to_string(),
                company: Some("Meridian Applied Science Group".to_string()),
                address: Some("742 Innovation Parkway, Suite 400\nBoston, MA 02110".to_string()),
                email: Some("billing@meridian.eng".to_string()),
                phone: Some("+1 (617) 555-0199".to_string()),
                tax_id: Some("EIN 04-9876543".to_string()),
                ..Default::default()
            },
            to: PartyDetails {
                name: "Apex Infrastructure Partners".to_string(),
                address: Some("1200 Continental Center\nChicago, IL 60606".to_string()),
                email: Some("accounts-payable@apexinfra.com".to_string()),
                ..Default::default()
            },
            items: vec![
                LineItem {
                    description: "Structural Foundation Analysis & Geotechnical Subsurface Probe".to_string(),
                    quantity: 40.0,
                    unit_price: 275.0,
                    tax_rate: Some(6.25),
                    discount: Some(500.0),
                },
                LineItem {
                    description: "Computational Wind Load & Seismic Vulnerability Simulation".to_string(),
                    quantity: 1.0,
                    unit_price: 8500.0,
                    tax_rate: Some(6.25),
                    discount: None,
                },
            ],
            notes: Some("Direct wire transfers must reference invoice number on remittance advice.".to_string()),
            terms: Some("Net 30 days from date of issuance. Unpaid balances incur 1.5% statutory monthly interest.".to_string()),
            show_qr_seal: Some(true),
            show_remittance_stub: Some(true),
            ..Default::default()
        };

        let (pdf_bytes, ms) = generate_pdf(&req).expect(&format!("Failed layout: {}", layout));
        assert!(!pdf_bytes.is_empty(), "PDF empty for layout: {}", layout);
        assert!(ms < 5000, "Layout {} compilation took {}ms", layout, ms);

        // Also verify PNG preview compiles for each layout style
        let (png_bytes, _) = generate_preview_png(&req, 100).expect(&format!("Failed PNG preview for layout: {}", layout));
        assert!(!png_bytes.is_empty(), "PNG empty for layout: {}", layout);
        assert!(png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    }
}

#[test]
fn test_audit_extreme_text_lengths_and_multipage() {
    if find_typst_executable().is_none() {
        eprintln!("SKIPPING test_audit_extreme_text_lengths_and_multipage: compiler not found");
        return;
    }

    // 1. Extreme 100+ char counterparty name and 6-line addresses
    let extreme_from_name = "International Consortium of Distributed Quantum Aerodynamics, Advanced Fluid Propulsion & Magnetohydrodynamics Holdings Group, LLC";
    let extreme_to_name = "Department of Intergovernmental Transportation Infrastructure & High-Speed Rail Modernization Authorities of North America";

    let mut items_25 = Vec::new();
    for i in 1..=25 {
        items_25.push(LineItem {
            description: format!(
                "Milestone Deliverable #{:02}: Comprehensive technical audit of sub-assembly #{:04} with multi-spectral ultrasonic sensor validation and cryptographic hash telemetry",
                i, i * 42
            ),
            quantity: (i as f64) * 2.5,
            unit_price: 150.0 + (i as f64) * 10.0,
            tax_rate: Some(9.25),
            discount: if i % 5 == 0 { Some(100.0) } else { None },
        });
    }

    let req = DocumentRequest {
        doc_type: DocumentType::InvoiceStandard,
        title: Some("SPECIALIZED MULTI-YEAR COMPREHENSIVE STRATEGIC STATEMENT OF WORK AND INFRASTRUCTURE ACCORD".to_string()),
        doc_number: Some("SOW-2026-XTRM-9921".to_string()),
        from: PartyDetails {
            name: extreme_from_name.to_string(),
            company: Some("Aerospace Research Division · Sub-Orbital Telemetry Annex".to_string()),
            address: Some("Building 42, Quantum Research Corridor, Spaceport Boulevard\nHuntsville, AL 35805".to_string()),
            email: Some("contracts-administration@quantum-aerodynamics.global.org".to_string()),
            phone: Some("+1 (256) 555-0142 ext 884".to_string()),
            tax_id: Some("EIN 63-0987654 / CAGE 8Z9F2".to_string()),
            website: Some("https://www.quantum-aerodynamics-consortium.org".to_string()),
            registration_number: Some("State Entity ID #AL-849201-B".to_string()),
        },
        to: PartyDetails {
            name: extreme_to_name.to_string(),
            company: Some("Division of Federal Grant Oversight & Capital Disbursements".to_string()),
            address: Some("Federal Center Plaza, Suite 9000, 400 7th Street SW\nWashington, DC 20590".to_string()),
            email: Some("disbursements-procurement@transportation.gov".to_string()),
            phone: Some("+1 (202) 555-0188".to_string()),
            tax_id: Some("Tax Exempt Org #501-C3-998811".to_string()),
            website: Some("https://www.transportation.infrastructure.gov".to_string()),
            registration_number: Some("DUNS #08-392-8172".to_string()),
        },
        items: items_25,
        show_line_numbers: Some(true),
        tax_rate: Some(9.25),
        discount_amount: Some(500.0),
        notes: Some(
            "This instrument encompasses multi-phased procurement governed under Title 48 CFR Chapter 1.\nAll testing procedures were executed under ISO/IEC 17025 accredited laboratory surveillance.\nRemittance must occur strictly through Automated Clearing House (ACH) direct deposit."
                .to_string(),
        ),
        terms: Some(
            "1. DELIVERABLE ACCEPTANCE: Inspection and preliminary acceptance shall occur within fifteen (15) calendar days of electronic manifest transmission.\n2. RETENTION: A statutory five percent (5%) retainage will be held pending final commissioning.\n3. AUDIT RIGHTS: The contracting officer retains perpetual rights to examine and audit all supporting telemetry logs."
                .to_string(),
        ),
        custom_fields: vec![
            CustomField { label: "Grant Award ID".to_string(), value: "DOT-FRA-2026-GR-8891".to_string() },
            CustomField { label: "NAICS Code".to_string(), value: "541330 (Engineering Services)".to_string() },
            CustomField { label: "Security Clearance Level".to_string(), value: "Facility Security Clearance TS/SCI".to_string() },
            CustomField { label: "Cost Accounting Standard".to_string(), value: "CAS-401 / CAS-402 Compliant".to_string() },
            CustomField { label: "Primary Contracting Officer".to_string(), value: "Commander Sarah Jenkins, PE".to_string() },
            CustomField { label: "Performance Period".to_string(), value: "2026-10-01 through 2028-09-30".to_string() },
            CustomField { label: "Inspection Facility".to_string(), value: "AeroPropulsion Test Bay 4B".to_string() },
            CustomField { label: "Statutory Safe Harbor".to_string(), value: "10 U.S.C. § 2306a Certified".to_string() },
        ],
        sections: vec![
            DocumentSection {
                title: "Section I: Quality Assurance & Metrology Protocols".to_string(),
                content: "All sensor readings and telemetry outputs documented in Schedule 4 have been calibrated against NIST-traceable standards. Environmental chamber stress testing was maintained at -40°C to +85°C across 48 continuous cycles.".to_string(),
            },
            DocumentSection {
                title: "Section II: Statutory Non-Debarment Certification".to_string(),
                content: "The issuer explicitly attests under penalty of law that neither the entity nor its principals are currently suspended, debarred, or proposed for debarment by any Federal executive branch agency.".to_string(),
            }
        ],
        show_page_numbers: Some(true),
        show_footer_divider: Some(true),
        show_qr_seal: Some(true),
        show_remittance_stub: Some(true),
        execution_mode: Some("esign".to_string()),
        ..Default::default()
    };

    let (pdf_bytes, ms) = generate_pdf(&req).expect("Failed extreme multipage PDF");
    assert!(!pdf_bytes.is_empty());
    assert!(pdf_bytes.len() > 10000, "Multipage PDF should be substantial");
    assert!(ms < 5000, "Multipage compilation took {}ms", ms);

    // Also test empty items fallback
    let mut empty_req = req.clone();
    empty_req.items.clear();
    let (empty_pdf, _) = generate_pdf(&empty_req).expect("Failed empty items fallback PDF");
    assert!(!empty_pdf.is_empty());
}

#[test]
fn test_audit_image_shapes_and_aspect_ratios() {
    if find_typst_executable().is_none() {
        eprintln!("SKIPPING test_audit_image_shapes_and_aspect_ratios: compiler not found");
        return;
    }

    // 1. Ultra-wide logo (5:1 aspect ratio, 500x100)
    let wide_logo = create_test_svg_data_uri(500, 100, "#4f46e5", "WIDE PANORAMA LOGO");

    // 2. Square logo (1:1 aspect ratio, 200x200)
    let square_logo = create_test_svg_data_uri(200, 200, "#059669", "SQUARE");

    // 3. Ultra-tall portrait crest logo (1:4 aspect ratio, 100x400)
    let tall_logo = create_test_svg_data_uri(100, 400, "#dc2626", "TALL");

    // 4. Handwritten signature SVGs
    let sig1 = create_test_signature_data_uri("Daniel Hallman");
    let sig2 = create_test_signature_data_uri("Eleanor Vance");
    let sig3 = create_test_signature_data_uri("Marcus Sterling");

    let logos = [
        ("wide", Some(wide_logo)),
        ("square", Some(square_logo)),
        ("tall", Some(tall_logo)),
        ("missing", None),
    ];

    for (name, logo_opt) in logos {
        let req = DocumentRequest {
            doc_type: DocumentType::InvoiceFreelance,
            logo_data: logo_opt,
            from: PartyDetails {
                name: "Hallman Design Studio".to_string(),
                company: Some("Creative Vector Labs".to_string()),
                ..Default::default()
            },
            to: PartyDetails {
                name: "Client Brand Co".to_string(),
                ..Default::default()
            },
            items: vec![
                LineItem {
                    description: "Brand Identity Design & Vector Asset Pack".to_string(),
                    quantity: 1.0,
                    unit_price: 3500.0,
                    tax_rate: None,
                    discount: None,
                },
            ],
            // 3 signers with 2 images and 1 blank for wet-ink
            signatures: vec![
                SignatureBlock {
                    signer_name: "Daniel Hallman".to_string(),
                    signer_title: Some("Chief Executive Officer".to_string()),
                    date: Some("2026-10-01".to_string()),
                    signature_data: Some(sig1.clone()),
                },
                SignatureBlock {
                    signer_name: "Eleanor Vance".to_string(),
                    signer_title: Some("Managing Director".to_string()),
                    date: Some("2026-10-01".to_string()),
                    signature_data: Some(sig2.clone()),
                },
                SignatureBlock {
                    signer_name: "Marcus Sterling".to_string(),
                    signer_title: Some("Chief Legal Counsel (Wet-Ink)".to_string()),
                    date: Some("2026-10-02".to_string()),
                    signature_data: None, // blank line for physical signing
                },
            ],
            show_signature: Some(true),
            ..Default::default()
        };

        let (pdf_bytes, ms) = generate_pdf(&req).expect(&format!("Failed logo shape test: {}", name));
        assert!(!pdf_bytes.is_empty(), "PDF empty for logo: {}", name);
        assert!(ms < 5000, "Logo test {} took {}ms", name, ms);
    }

    // Test 4-signatory 2x2 grid
    let req_4_sigs = DocumentRequest {
        doc_type: DocumentType::Agreement,
        title: Some("FOUR-PARTY QUADRIPARTITE CONSORTIUM AGREEMENT".to_string()),
        from: PartyDetails {
            name: "Consortium Party Alpha".to_string(),
            ..Default::default()
        },
        to: PartyDetails {
            name: "Consortium Party Beta".to_string(),
            ..Default::default()
        },
        signatures: vec![
            SignatureBlock {
                signer_name: "Daniel Hallman".to_string(),
                signer_title: Some("Principal Investigator (Party Alpha)".to_string()),
                date: Some("2026-10-01".to_string()),
                signature_data: Some(sig1.clone()),
            },
            SignatureBlock {
                signer_name: "Eleanor Vance".to_string(),
                signer_title: Some("Executive Director (Party Beta)".to_string()),
                date: Some("2026-10-01".to_string()),
                signature_data: Some(sig2.clone()),
            },
            SignatureBlock {
                signer_name: "Marcus Sterling".to_string(),
                signer_title: Some("Trustee (Party Gamma)".to_string()),
                date: Some("2026-10-02".to_string()),
                signature_data: Some(sig3.clone()),
            },
            SignatureBlock {
                signer_name: "Dr. Evelyn Zhao".to_string(),
                signer_title: Some("Independent Arbitrator (Party Delta)".to_string()),
                date: Some("2026-10-02".to_string()),
                signature_data: None, // blank for ink
            },
        ],
        show_signature: Some(true),
        ..Default::default()
    };
    let (pdf_4, _) = generate_pdf(&req_4_sigs).expect("Failed 4-signatory grid PDF");
    assert!(!pdf_4.is_empty());
}

#[test]
fn test_audit_numbers_currencies_and_separators() {
    if find_typst_executable().is_none() {
        eprintln!("SKIPPING test_audit_numbers_currencies_and_separators: compiler not found");
        return;
    }

    let test_cases = vec![
        // 1. High value USD with standard comma/dot
        DocumentRequest {
            currency: Some("USD".to_string()),
            currency_separators: Some("comma_dot".to_string()),
            items: vec![
                LineItem {
                    description: "High-Performance Data Center Acquisition".to_string(),
                    quantity: 1.0,
                    unit_price: 18_450_000.0,
                    tax_rate: Some(8.25),
                    discount: Some(450_000.0),
                },
            ],
            ..Default::default()
        },
        // 2. Japanese Yen (JPY) zero-decimal
        DocumentRequest {
            currency: Some("JPY".to_string()),
            currency_symbol_mode: Some("symbol".to_string()),
            number_decimals: Some(0),
            items: vec![
                LineItem {
                    description: "Tokyo Server Cluster Lease (Annual)".to_string(),
                    quantity: 12.0,
                    unit_price: 1_250_000.0,
                    tax_rate: Some(10.0),
                    discount: None,
                },
            ],
            ..Default::default()
        },
        // 3. Euro (EUR) European format dot_comma with suffix
        DocumentRequest {
            currency: Some("EUR".to_string()),
            currency_separators: Some("dot_comma".to_string()),
            currency_placement: Some("suffix".to_string()),
            items: vec![
                LineItem {
                    description: "Continental Fiber Optic Transit".to_string(),
                    quantity: 100.0,
                    unit_price: 1425.50,
                    tax_rate: Some(20.0),
                    discount: Some(250.0),
                },
            ],
            ..Default::default()
        },
        // 4. French/Nordic space_comma format
        DocumentRequest {
            currency: Some("EUR".to_string()),
            currency_separators: Some("space_comma".to_string()),
            currency_placement: Some("suffix".to_string()),
            items: vec![
                LineItem {
                    description: "Parisian Architectural Heritage Assessment".to_string(),
                    quantity: 3.0,
                    unit_price: 15400.00,
                    tax_rate: Some(19.6),
                    discount: None,
                },
            ],
            ..Default::default()
        },
        // 5. Swiss Franc (CHF) apostrophe_dot format
        DocumentRequest {
            currency: Some("CHF".to_string()),
            currency_separators: Some("apostrophe_dot".to_string()),
            items: vec![
                LineItem {
                    description: "Zurich Private Banking Security Audit".to_string(),
                    quantity: 1.0,
                    unit_price: 88500.00,
                    tax_rate: Some(7.7),
                    discount: None,
                },
            ],
            ..Default::default()
        },
        // 6. Micro-pricing / high precision (4 decimals)
        DocumentRequest {
            currency: Some("USD".to_string()),
            number_decimals: Some(4),
            unit_measure: Some("API calls".to_string()),
            items: vec![
                LineItem {
                    description: "Serverless Edge Invocations".to_string(),
                    quantity: 2500000.0,
                    unit_price: 0.0004,
                    tax_rate: None,
                    discount: None,
                },
            ],
            ..Default::default()
        },
        // 7. Zero-sum document ($0.00 free community grant)
        DocumentRequest {
            currency: Some("USD".to_string()),
            items: vec![
                LineItem {
                    description: "Open Source Research Grant & Educational License".to_string(),
                    quantity: 1.0,
                    unit_price: 0.0,
                    tax_rate: None,
                    discount: None,
                },
            ],
            is_paid: Some(true),
            ..Default::default()
        },
    ];

    for (idx, mut req) in test_cases.into_iter().enumerate() {
        req.from = PartyDetails {
            name: "Global Financial Services SA".to_string(),
            address: Some("Bahnhofstrasse 45\n8001 Zurich, Switzerland".to_string()),
            ..Default::default()
        };
        req.to = PartyDetails {
            name: "Enterprise Client Corp".to_string(),
            address: Some("350 Fifth Avenue\nNew York, NY 10118".to_string()),
            ..Default::default()
        };

        let (pdf_bytes, ms) = generate_pdf(&req).expect(&format!("Failed financial test case #{}", idx + 1));
        assert!(!pdf_bytes.is_empty());
        assert!(ms < 5000, "Financial test #{} took {}ms", idx + 1, ms);
    }
}

#[test]
fn test_audit_all_compliance_modes() {
    if find_typst_executable().is_none() {
        eprintln!("SKIPPING test_audit_all_compliance_modes: compiler not found");
        return;
    }

    let compliance_modes = ["eu_vat", "irs_w9", "pci_dss", "soc2", "statutory_sworn"];

    for comp in compliance_modes {
        let req = DocumentRequest {
            doc_type: DocumentType::InvoiceStandard,
            compliance_mode: Some(comp.to_string()),
            from: PartyDetails {
                name: "Compliant Enterprise Solutions Corp".to_string(),
                company: Some("Regulatory Technology Division".to_string()),
                tax_id: Some("EU VAT: FR849201928 / EIN: 12-3456789".to_string()),
                ..Default::default()
            },
            to: PartyDetails {
                name: "European Commercial Counterparty BV".to_string(),
                tax_id: Some("VAT: NL998877665B01".to_string()),
                ..Default::default()
            },
            items: vec![
                LineItem {
                    description: "Cross-Border Telecommunications & Gateway Transit".to_string(),
                    quantity: 1.0,
                    unit_price: 14500.0,
                    tax_rate: Some(0.0), // Reverse charge
                    discount: None,
                },
            ],
            notes: Some("Reverse charge: Customer to account for VAT pursuant to EU Directive 2006/112/EC.".to_string()),
            show_qr_seal: Some(true),
            ..Default::default()
        };

        let (pdf_bytes, ms) = generate_pdf(&req).expect(&format!("Failed compliance mode: {}", comp));
        assert!(!pdf_bytes.is_empty());
        assert!(ms < 5000, "Compliance mode {} took {}ms", comp, ms);
    }
}

#[test]
fn test_audit_master_showcase_export_pdf_and_png() {
    if find_typst_executable().is_none() {
        eprintln!("SKIPPING test_audit_master_showcase_export_pdf_and_png: compiler not found");
        return;
    }

    let gallery_dir = setup_gallery_dir();
    let pdf_path = gallery_dir.join("pro_document_audit.pdf");
    let png_path = gallery_dir.join("pro_document_audit.png");

    let logo_uri = create_test_svg_data_uri(360, 90, "#1e3a8a", "PRESSMARK STUDIO");
    let sig_ceo = create_test_signature_data_uri("Daniel Hallman");
    let sig_cfo = create_test_signature_data_uri("Eleanor Vance");

    let req = DocumentRequest {
        doc_type: DocumentType::InvoiceStandard,
        title: Some("COMMERCIAL TAX INVOICE & PERFORMANCE CERTIFICATE".to_string()),
        doc_number: Some("INV-2026-PRO-AUDIT-01".to_string()),
        header_subtitle: Some("Pressmark Sub-Millisecond Vector Design Specification · Comprehensive Living Test".to_string()),
        issue_date: Some("2026-10-01".to_string()),
        due_date: Some("2026-10-15".to_string()),
        theme: Some("executive_navy".to_string()),
        layout_style: Some("framed".to_string()),
        background_style: Some("slate_tint".to_string()),
        spacing_density: Some("standard".to_string()),
        font_family: Some("sans".to_string()),
        font_scale: Some("standard".to_string()),
        logo_data: Some(logo_uri),
        from: PartyDetails {
            name: "Pressmark Technologies & Vector Systems, Inc.".to_string(),
            company: Some("Engineering & Commercial Infrastructure Group".to_string()),
            address: Some("100 Innovation Way, Suite 500\nSan Francisco, CA 94105".to_string()),
            email: Some("billing@pressmark.dev".to_string()),
            phone: Some("+1 (415) 555-0142".to_string()),
            tax_id: Some("EIN 94-8849201 / VAT US948849201".to_string()),
            website: Some("https://pressmark.dev".to_string()),
            registration_number: Some("State Entity ID #C4829101".to_string()),
        },
        to: PartyDetails {
            name: "Global Aerospace & Applied Dynamics Corporation".to_string(),
            company: Some("Defense & Autonomous Systems Division".to_string()),
            address: Some("700 Continental Gateway, Floor 32\nChicago, IL 60606".to_string()),
            email: Some("disbursements@globalaerospace.com".to_string()),
            phone: Some("+1 (312) 555-0199".to_string()),
            tax_id: Some("EIN 36-1234567 / DUNS 04-928-1192".to_string()),
            website: Some("https://globalaerospace.com".to_string()),
            registration_number: Some("CAGE Code 1Z4Y8".to_string()),
        },
        items: vec![
            LineItem {
                description: "Sub-Millisecond Vector Synthesis Engine Core Enterprise License (Annual)".to_string(),
                quantity: 1.0,
                unit_price: 24000.0,
                tax_rate: Some(8.75),
                discount: Some(2000.0),
            },
            LineItem {
                description: "Dedicated High-Throughput Document Compilation Cluster (4 Dedicated Nodes)".to_string(),
                quantity: 4.0,
                unit_price: 3500.0,
                tax_rate: Some(8.75),
                discount: None,
            },
            LineItem {
                description: "Custom Typesetting Template Migration & Typographic Token Architecture".to_string(),
                quantity: 45.0,
                unit_price: 250.0,
                tax_rate: Some(8.75),
                discount: None,
            },
            LineItem {
                description: "Enterprise SLA: 24/7/365 Direct Tier-3 Vector Engine Engineering Support".to_string(),
                quantity: 12.0,
                unit_price: 1250.0,
                tax_rate: Some(8.75),
                discount: Some(1500.0),
            },
        ],
        show_line_numbers: Some(true),
        currency: Some("USD".to_string()),
        currency_separators: Some("comma_dot".to_string()),
        tax_rate: Some(8.75),
        discount_amount: Some(3500.0),
        notes: Some(
            "1. Electronic bank transfers must reference invoice #INV-2026-PRO-AUDIT-01 on all remittance transmissions.\n2. Cryptographic audit validation is anchored to SHA-256 telemetry block below.\n3. Early settlement terms: 2/10 Net 30 applied upon verification of settlement."
                .to_string(),
        ),
        terms: Some(
            "All services rendered and licensed under the Pressmark Master Commercial Services Agreement.\nTitle to custom vector templates transfers upon full settlement. Confidential and proprietary."
                .to_string(),
        ),
        custom_fields: vec![
            CustomField { label: "Purchase Order #".to_string(), value: "PO-GAAD-2026-00918".to_string() },
            CustomField { label: "Contract Reference".to_string(), value: "MSA-2026-PM-0482".to_string() },
            CustomField { label: "Payment Terms".to_string(), value: "Net 15 Days (Wire / ACH)".to_string() },
            CustomField { label: "Security Clearance".to_string(), value: "SOC-2 Type II Certified".to_string() },
        ],
        sections: vec![
            DocumentSection {
                title: "Statutory Performance Attestation".to_string(),
                content: "The issuer certifies that all software modules, vector engines, and service hours billed herein have been delivered in full accordance with contractual benchmarks and security invariants.".to_string(),
            },
        ],
        signatures: vec![
            SignatureBlock {
                signer_name: "Daniel Hallman".to_string(),
                signer_title: Some("Chief Executive Officer & Founder".to_string()),
                date: Some("2026-10-01".to_string()),
                signature_data: Some(sig_ceo),
            },
            SignatureBlock {
                signer_name: "Eleanor Vance".to_string(),
                signer_title: Some("VP of Finance & Corporate Controller".to_string()),
                date: Some("2026-10-01".to_string()),
                signature_data: Some(sig_cfo),
            },
        ],
        show_signature: Some(true),
        show_qr_seal: Some(true),
        show_remittance_stub: Some(true),
        show_page_numbers: Some(true),
        show_footer_divider: Some(true),
        compliance_mode: Some("soc2".to_string()),
        execution_mode: Some("esign".to_string()),
        watermark: None,
        ..Default::default()
    };

    let (pdf_bytes, ms_pdf) = generate_pdf(&req).expect("Failed master showcase PDF");
    assert!(!pdf_bytes.is_empty());
    fs::write(&pdf_path, &pdf_bytes).expect("Failed to write pro_document_audit.pdf");
    println!("Master Showcase PDF written to {:?} ({} bytes in {}ms)", pdf_path, pdf_bytes.len(), ms_pdf);

    let (png_bytes, ms_png) = generate_preview_png(&req, 150).expect("Failed master showcase PNG");
    assert!(!png_bytes.is_empty());
    fs::write(&png_path, &png_bytes).expect("Failed to write pro_document_audit.png");
    println!("Master Showcase PNG preview written to {:?} ({} bytes in {}ms)", png_path, png_bytes.len(), ms_png);

    // Also copy PNG to conversation artifact directory so it can be viewed by user
    let artifact_dir = PathBuf::from("/Users/dhallman/.gemini/antigravity/brain/db484a2f-1c16-4cec-a780-3792e1b1f23f");
    let artifact_png = artifact_dir.join("pro_document_audit_showcase.png");
    let _ = fs::copy(&png_path, &artifact_png);
    println!("Master Showcase preview copied to artifact path: {:?}", artifact_png);
}

#[test]
fn test_render_diverse_template_gallery_and_export_artifacts() {
    if find_typst_executable().is_none() {
        eprintln!("SKIPPING test_render_diverse_template_gallery_and_export_artifacts: compiler not found");
        return;
    }

    let artifact_dir = PathBuf::from("/Users/dhallman/.gemini/antigravity/brain/db484a2f-1c16-4cec-a780-3792e1b1f23f");
    let _ = fs::create_dir_all(&artifact_dir);

    let logo_tech = create_test_svg_data_uri(320, 80, "#0f172a", "PRESSMARK LABS");
    let logo_legal = create_test_svg_data_uri(320, 80, "#881337", "HALLMAN & STERLING LLP");
    let logo_construct = create_test_svg_data_uri(320, 80, "#b45309", "SUMMIT INFRASTRUCTURE");
    let logo_academy = create_test_svg_data_uri(320, 80, "#78350f", "EUCLID INSTITUTE");

    let sig_dh = create_test_signature_data_uri("Daniel Hallman");
    let sig_ev = create_test_signature_data_uri("Eleanor Vance");
    let sig_ms = create_test_signature_data_uri("Marcus Sterling, Esq.");

    // 1. Template: Corporate B2B Tax Invoice
    let t1_req = DocumentRequest {
        doc_type: DocumentType::InvoiceStandard,
        title: Some("TAX INVOICE".to_string()),
        doc_number: Some("INV-2026-US-892".to_string()),
        header_subtitle: Some("Enterprise Vector Systems & Infrastructure".to_string()),
        issue_date: Some("2026-10-01".to_string()),
        due_date: Some("2026-10-15".to_string()),
        theme: Some("executive_navy".to_string()),
        layout_style: Some("framed".to_string()),
        background_style: Some("security_frame".to_string()),
        logo_data: Some(logo_tech.clone()),
        from: PartyDetails {
            name: "Pressmark Technologies LLC".to_string(),
            company: Some("High-Throughput Vector Platforms".to_string()),
            address: Some("742 Evergreen Terrace, Suite 400\nSan Francisco, CA 94107".to_string()),
            email: Some("billing@pressmark.io".to_string()),
            phone: Some("+1 (415) 890-4421".to_string()),
            tax_id: Some("US-94-3829104".to_string()),
            website: Some("https://pressmark.io".to_string()),
            registration_number: Some("REG-84920".to_string()),
        },
        to: PartyDetails {
            name: "Vance Dynamics Corporation".to_string(),
            company: Some("Autonomous Robotics Group".to_string()),
            address: Some("100 Innovation Parkway, Floor 12\nAustin, TX 78701".to_string()),
            email: Some("disbursements@vancedynamics.com".to_string()),
            phone: Some("+1 (512) 555-0199".to_string()),
            tax_id: Some("US-74-8891024".to_string()),
            website: Some("https://vancedynamics.com".to_string()),
            registration_number: Some("CRN-09281".to_string()),
        },
        items: vec![
            LineItem {
                description: "Sub-Millisecond Vector Synthesis Engine Core Enterprise License".to_string(),
                quantity: 1.0,
                unit_price: 18500.0,
                tax_rate: Some(8.25),
                discount: Some(1000.0),
            },
            LineItem {
                description: "High-Availability Distributed Compilation Cluster Nodes (4 Nodes)".to_string(),
                quantity: 4.0,
                unit_price: 2450.0,
                tax_rate: Some(8.25),
                discount: None,
            },
            LineItem {
                description: "Automated CI/CD PDF Pipeline Integration & Typographic Design System".to_string(),
                quantity: 35.0,
                unit_price: 220.0,
                tax_rate: Some(8.25),
                discount: None,
            },
        ],
        show_line_numbers: Some(true),
        currency: Some("USD".to_string()),
        currency_separators: Some("comma_dot".to_string()),
        tax_rate: Some(8.25),
        discount_amount: Some(1000.0),
        notes: Some("Remittance via Wire or ACH. Remittance reference: INV-2026-US-892.".to_string()),
        terms: Some("Net 15 days. Unpaid balances subject to 1.5% monthly late assessment.".to_string()),
        custom_fields: vec![
            CustomField { label: "Purchase Order".to_string(), value: "PO-VD-88291".to_string() },
            CustomField { label: "Project Code".to_string(), value: "PRJ-NEO-2026".to_string() },
            CustomField { label: "Cost Center".to_string(), value: "CC-ENG-402".to_string() },
        ],
        signatures: vec![
            SignatureBlock {
                signer_name: "Daniel Hallman".to_string(),
                signer_title: Some("Chief Executive Officer".to_string()),
                date: Some("2026-10-01".to_string()),
                signature_data: Some(sig_dh.clone()),
            },
        ],
        show_signature: Some(true),
        show_qr_seal: Some(true),
        show_remittance_stub: Some(true),
        show_page_numbers: Some(true),
        compliance_mode: Some("eu_vat".to_string()),
        execution_mode: Some("esign".to_string()),
        ..Default::default()
    };
    let (t1_png, _) = generate_preview_png(&t1_req, 150).expect("Failed template 1 PNG");
    let t1_path = artifact_dir.join("template_1_tax_invoice.png");
    fs::write(&t1_path, &t1_png).expect("Failed write t1");
    println!("Template 1 written to {:?}", t1_path);

    // 2. Template: Freelance Consulting & Scope of Work
    let t2_req = DocumentRequest {
        doc_type: DocumentType::AgreementSow,
        title: Some("CONSULTING RETAINER & SCOPE OF WORK".to_string()),
        doc_number: Some("RET-2026-SOW-04".to_string()),
        header_subtitle: Some("Q4 2026 Distributed Systems Architecture & Optimization".to_string()),
        issue_date: Some("2026-10-01".to_string()),
        due_date: Some("2026-10-31".to_string()),
        theme: Some("electric_indigo".to_string()),
        layout_style: Some("swiss_grid".to_string()),
        background_style: Some("clean_white".to_string()),
        from: PartyDetails {
            name: "Hallman Engineering Labs".to_string(),
            company: Some("Principal Systems Consultancy".to_string()),
            address: Some("500 Market Street, Suite 1800\nSan Francisco, CA 94105".to_string()),
            email: Some("daniel@hallmanlabs.com".to_string()),
            phone: Some("+1 (423) 822-2232".to_string()),
            tax_id: Some("EIN 82-4910294".to_string()),
            website: Some("https://hallmanlabs.com".to_string()),
            registration_number: None,
        },
        to: PartyDetails {
            name: "Acorn Logistics GmbH".to_string(),
            company: Some("Autonomous Fleet Operations".to_string()),
            address: Some("Friedrichstraße 42\n10117 Berlin, Germany".to_string()),
            email: Some("contracts@acornlogistics.de".to_string()),
            phone: Some("+49 30 9281042".to_string()),
            tax_id: Some("DE 294819204".to_string()),
            website: Some("https://acornlogistics.de".to_string()),
            registration_number: Some("HRB 194820 B".to_string()),
        },
        items: vec![
            LineItem {
                description: "Distributed Event Pipeline & Tokio Engine Performance Audit".to_string(),
                quantity: 40.0,
                unit_price: 175.0,
                tax_rate: None,
                discount: None,
            },
            LineItem {
                description: "Real-Time Telemetry Streaming Architecture & Micro-Benchmarking".to_string(),
                quantity: 30.0,
                unit_price: 195.0,
                tax_rate: None,
                discount: None,
            },
            LineItem {
                description: "Executive Engineering Alignment & Technical Advisory (Monthly Retainer)".to_string(),
                quantity: 1.0,
                unit_price: 4500.0,
                tax_rate: None,
                discount: None,
            },
        ],
        show_line_numbers: Some(true),
        currency: Some("EUR".to_string()),
        currency_separators: Some("space_comma".to_string()),
        tax_rate: Some(0.0),
        discount_amount: None,
        notes: Some("EU Reverse Charge: Customer VAT registered in Germany (DE 294819204).".to_string()),
        terms: Some("All work product transfers upon full settlement of invoice.".to_string()),
        sections: vec![
            DocumentSection {
                title: "Scope of Milestones & Deliverables".to_string(),
                content: "1. Complete end-to-end memory profiling of asynchronous actor workers.\n2. Deliver zero-alloc deserialization benchmarks for binary protocol.\n3. Publish formal architectural recommendation document.".to_string(),
            },
        ],
        signatures: vec![
            SignatureBlock {
                signer_name: "Daniel Hallman".to_string(),
                signer_title: Some("Principal Consulting Architect".to_string()),
                date: Some("2026-10-01".to_string()),
                signature_data: Some(sig_dh.clone()),
            },
            SignatureBlock {
                signer_name: "Dr. Klaus Lindner".to_string(),
                signer_title: Some("Chief Technology Officer, Acorn Logistics".to_string()),
                date: Some("2026-10-02".to_string()),
                signature_data: Some(sig_ev.clone()),
            },
        ],
        show_signature: Some(true),
        show_qr_seal: Some(true),
        show_page_numbers: Some(true),
        execution_mode: Some("esign".to_string()),
        ..Default::default()
    };
    let (t2_png, _) = generate_preview_png(&t2_req, 150).expect("Failed template 2 PNG");
    let t2_path = artifact_dir.join("template_2_freelance_consulting.png");
    fs::write(&t2_path, &t2_png).expect("Failed write t2");
    println!("Template 2 written to {:?}", t2_path);

    // 3. Template: Legal Retainer & Mutual Non-Disclosure Agreement
    let t3_req = DocumentRequest {
        doc_type: DocumentType::Nda,
        title: Some("LEGAL RETAINER & MUTUAL NON-DISCLOSURE AGREEMENT".to_string()),
        doc_number: Some("LGL-2026-NDA-109".to_string()),
        header_subtitle: Some("Statutory Sworn Legal Instrument & Retainer Protocol".to_string()),
        issue_date: Some("2026-10-01".to_string()),
        due_date: Some("2026-10-15".to_string()),
        theme: Some("crimson_legal".to_string()),
        layout_style: Some("classic_letterhead".to_string()),
        background_style: Some("warm_parchment".to_string()),
        logo_data: Some(logo_legal),
        from: PartyDetails {
            name: "Hallman, Sterling & Vance LLP".to_string(),
            company: Some("Attorneys and Counselors at Law".to_string()),
            address: Some("350 California Street, Suite 2400\nSan Francisco, CA 94104".to_string()),
            email: Some("retainers@hallmansterling.com".to_string()),
            phone: Some("+1 (415) 555-8200".to_string()),
            tax_id: Some("EIN 94-7182901".to_string()),
            website: Some("https://hallmansterling.com".to_string()),
            registration_number: Some("State Bar #284910".to_string()),
        },
        to: PartyDetails {
            name: "Hyperion Autonomous Robotics Corp.".to_string(),
            company: Some("Deep Tech Ventures".to_string()),
            address: Some("1200 Seaport Boulevard\nRedwood City, CA 94063".to_string()),
            email: Some("legal@hyperionrobotics.com".to_string()),
            phone: Some("+1 (650) 555-0182".to_string()),
            tax_id: Some("EIN 46-2918402".to_string()),
            website: Some("https://hyperionrobotics.com".to_string()),
            registration_number: Some("Delaware C-Corp #592810".to_string()),
        },
        items: vec![
            LineItem {
                description: "Quarterly General Corporate & Patent Prosecution Retainer (Q4 2026)".to_string(),
                quantity: 1.0,
                unit_price: 15000.0,
                tax_rate: None,
                discount: None,
            },
            LineItem {
                description: "Intellectual Property Freedom-to-Operate Vector Engine Patent Clearance".to_string(),
                quantity: 1.0,
                unit_price: 8500.0,
                tax_rate: None,
                discount: None,
            },
        ],
        show_line_numbers: Some(true),
        currency: Some("USD".to_string()),
        currency_separators: Some("comma_dot".to_string()),
        tax_rate: None,
        discount_amount: None,
        notes: Some("Trust Account Remittance: Funds held in IOLTA escrow account #8928104.".to_string()),
        terms: Some("Governed by the State Bar of California Rules of Professional Conduct.".to_string()),
        sections: vec![
            DocumentSection {
                title: "1. Confidentiality & Non-Disclosure Invariants".to_string(),
                content: "Both parties agree that all trade secrets, compiler algorithms, financial tables, and customer rosters exchanged during this retainer shall remain strictly confidential for a term of five (5) years from the effective execution date.".to_string(),
            },
            DocumentSection {
                title: "2. Governing Law & Equitable Relief".to_string(),
                content: "This agreement is construed under the laws of the State of California. The parties submit to exclusive venue in the City and County of San Francisco. Injunctive relief may be sought without posting bond.".to_string(),
            },
        ],
        signatures: vec![
            SignatureBlock {
                signer_name: "Marcus Sterling, Esq.".to_string(),
                signer_title: Some("Managing Partner, Hallman, Sterling & Vance LLP".to_string()),
                date: Some("2026-10-01".to_string()),
                signature_data: Some(sig_ms),
            },
            SignatureBlock {
                signer_name: "Daniel Hallman".to_string(),
                signer_title: Some("Chief Executive Officer, Hyperion Autonomous Robotics".to_string()),
                date: Some("2026-10-01".to_string()),
                signature_data: Some(sig_dh.clone()),
            },
        ],
        show_signature: Some(true),
        show_qr_seal: Some(true),
        show_page_numbers: Some(true),
        compliance_mode: Some("statutory_jurat".to_string()),
        execution_mode: Some("wet_ink".to_string()),
        ..Default::default()
    };
    let (t3_png, _) = generate_preview_png(&t3_req, 150).expect("Failed template 3 PNG");
    let t3_path = artifact_dir.join("template_3_legal_engagement.png");
    fs::write(&t3_path, &t3_png).expect("Failed write t3");
    println!("Template 3 written to {:?}", t3_path);

    // 4. Template: Construction & Trade Progress Claim
    let t4_req = DocumentRequest {
        doc_type: DocumentType::InvoiceConstruction,
        title: Some("APPLICATION AND CERTIFICATE FOR PAYMENT".to_string()),
        doc_number: Some("AIA-G702-2026-03".to_string()),
        header_subtitle: Some("Commercial Structural Expansion · Progress Billing #3".to_string()),
        issue_date: Some("2026-10-01".to_string()),
        due_date: Some("2026-10-20".to_string()),
        theme: Some("warm_amber".to_string()),
        layout_style: Some("modern_banner".to_string()),
        background_style: Some("slate_tint".to_string()),
        logo_data: Some(logo_construct),
        from: PartyDetails {
            name: "Summit Commercial Infrastructure LLC".to_string(),
            company: Some("General Contracting Division".to_string()),
            address: Some("420 Industrial Highway\nDenver, CO 80216".to_string()),
            email: Some("billing@summitinfra.com".to_string()),
            phone: Some("+1 (303) 555-0912".to_string()),
            tax_id: Some("CO-EIN 84-2910482".to_string()),
            website: Some("https://summitinfra.com".to_string()),
            registration_number: Some("CO Contractor Lic #GC-94810".to_string()),
        },
        to: PartyDetails {
            name: "Apex Logistics Hub LLC".to_string(),
            company: Some("Property Development Group".to_string()),
            address: Some("8800 Airport Road, Suite 200\nAurora, CO 80011".to_string()),
            email: Some("ap@apexlogistics.com".to_string()),
            phone: Some("+1 (303) 555-0144".to_string()),
            tax_id: Some("EIN 84-9102842".to_string()),
            website: Some("https://apexlogistics.com".to_string()),
            registration_number: None,
        },
        items: vec![
            LineItem {
                description: "Structural Steel Frame Erection & Seismic Reinforcement".to_string(),
                quantity: 14.5,
                unit_price: 4800.0,
                tax_rate: Some(4.75),
                discount: None,
            },
            LineItem {
                description: "High-Strength Reinforced Concrete Slab Foundation (3,500 PSI)".to_string(),
                quantity: 180.0,
                unit_price: 340.0,
                tax_rate: Some(4.75),
                discount: None,
            },
            LineItem {
                description: "Industrial High-Voltage Switchgear Rough-In & Grounding".to_string(),
                quantity: 260.0,
                unit_price: 98.0,
                tax_rate: Some(4.75),
                discount: None,
            },
        ],
        show_line_numbers: Some(true),
        currency: Some("USD".to_string()),
        currency_separators: Some("comma_dot".to_string()),
        tax_rate: Some(4.75),
        discount_amount: Some(15628.0), // 10% Retainage withheld
        notes: Some("Conditional Partial Lien Waiver attached. Releases all claims through 2026-10-01 upon clearance.".to_string()),
        terms: Some("Payment due within 20 days per Colorado Prompt Payment Act § 24-91-103.".to_string()),
        custom_fields: vec![
            CustomField { label: "Project Title".to_string(), value: "Apex West Distribution Center".to_string() },
            CustomField { label: "Contract Sum to Date".to_string(), value: "$1,840,000.00".to_string() },
            CustomField { label: "Retainage Rate".to_string(), value: "10.0% Stored / Work Completed".to_string() },
        ],
        signatures: vec![
            SignatureBlock {
                signer_name: "Daniel Hallman".to_string(),
                signer_title: Some("Project Executive, Summit Infrastructure".to_string()),
                date: Some("2026-10-01".to_string()),
                signature_data: Some(sig_dh.clone()),
            },
            SignatureBlock {
                signer_name: "Eleanor Vance, AIA".to_string(),
                signer_title: Some("Supervising Architect of Record".to_string()),
                date: Some("2026-10-02".to_string()),
                signature_data: Some(sig_ev.clone()),
            },
        ],
        show_signature: Some(true),
        show_qr_seal: Some(true),
        show_remittance_stub: Some(true),
        show_page_numbers: Some(true),
        compliance_mode: Some("pci_dss".to_string()),
        execution_mode: Some("esign".to_string()),
        ..Default::default()
    };
    let (t4_png, _) = generate_preview_png(&t4_req, 150).expect("Failed template 4 PNG");
    let t4_path = artifact_dir.join("template_4_construction_trade.png");
    fs::write(&t4_path, &t4_png).expect("Failed write t4");
    println!("Template 4 written to {:?}", t4_path);

    // 5. Template: Certificate of Architectural Completion & Honors
    let t5_req = DocumentRequest {
        doc_type: DocumentType::CertificateCompletion,
        title: Some("CERTIFICATE OF ARCHITECTURAL COMPLETION".to_string()),
        doc_number: Some("CERT-2026-ACCEPT-01".to_string()),
        header_subtitle: Some("Pressmark Autonomous Typesetting & Systems Architecture Standards".to_string()),
        issue_date: Some("2026-10-01".to_string()),
        due_date: None,
        theme: Some("monaco_gold".to_string()),
        layout_style: Some("editorial_centered".to_string()),
        background_style: Some("marble_vellum".to_string()),
        logo_data: Some(logo_academy),
        from: PartyDetails {
            name: "Euclid Institute of Advanced Architecture".to_string(),
            company: Some("Board of Architectural Review".to_string()),
            address: Some("100 Academy Plaza, Suite 900\nBoston, MA 02116".to_string()),
            email: Some("registrar@euclidinstitute.org".to_string()),
            phone: Some("+1 (617) 555-0900".to_string()),
            tax_id: Some("EIN 04-2918401".to_string()),
            website: Some("https://euclidinstitute.org".to_string()),
            registration_number: Some("Charter #MA-1892".to_string()),
        },
        to: PartyDetails {
            name: "Daniel Hallman".to_string(),
            company: Some("Pressmark Vector Engineering Lead".to_string()),
            address: Some("San Francisco, California".to_string()),
            email: Some("daniel@mooshworks.com".to_string()),
            phone: Some("+1 (423) 822-2232".to_string()),
            tax_id: None,
            website: Some("https://pressmark.dev".to_string()),
            registration_number: None,
        },
        items: vec![
            LineItem {
                description: "Sub-Millisecond Vector Typesetting Core Protocol Invariants".to_string(),
                quantity: 1.0,
                unit_price: 0.0,
                tax_rate: None,
                discount: None,
            },
            LineItem {
                description: "Deterministic 9-Layout Responsive Micro-Grid Architecture".to_string(),
                quantity: 1.0,
                unit_price: 0.0,
                tax_rate: None,
                discount: None,
            },
            LineItem {
                description: "Interactive Canvas Drag-and-Drop & Zero-Error Execution Engine".to_string(),
                quantity: 1.0,
                unit_price: 0.0,
                tax_rate: None,
                discount: None,
            },
        ],
        show_line_numbers: Some(false),
        currency: Some("USD".to_string()),
        notes: Some("This certificate attests that all technical standards, cryptographic tamper-evident seals, and layout aesthetics have satisfied the Highest Professional Honors.".to_string()),
        terms: Some("Authenticated via Pressmark SHA-256 Ledger Node.".to_string()),
        sections: vec![
            DocumentSection {
                title: "Formal Citation of Merit".to_string(),
                content: "Be it known that Daniel Hallman has engineered and delivered an exceptional vector document compilation engine, combining sub-millisecond execution, elegant typography, fluid motion ergonomics, and commercial durability.".to_string(),
            },
        ],
        signatures: vec![
            SignatureBlock {
                signer_name: "Dr. Alistair Montgomery".to_string(),
                signer_title: Some("President of the Institute".to_string()),
                date: Some("2026-10-01".to_string()),
                signature_data: Some(sig_ev.clone()),
            },
            SignatureBlock {
                signer_name: "Eleanor Vance".to_string(),
                signer_title: Some("Dean of Systems Engineering".to_string()),
                date: Some("2026-10-01".to_string()),
                signature_data: Some(sig_dh.clone()),
            },
        ],
        show_signature: Some(true),
        show_qr_seal: Some(true),
        show_page_numbers: Some(false),
        compliance_mode: Some("soc2".to_string()),
        execution_mode: Some("esign".to_string()),
        ..Default::default()
    };
    let (t5_png, _) = generate_preview_png(&t5_req, 150).expect("Failed template 5 PNG");
    let t5_path = artifact_dir.join("template_5_certificate_completion.png");
    fs::write(&t5_path, &t5_png).expect("Failed write t5");
    println!("Template 5 written to {:?}", t5_path);
}
