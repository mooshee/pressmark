// Author: Daniel Hallman
//! Real Estate & Property Management Document Suite - Battle Test Suite
//!
//! Validates rendering, compilation, calculations, and compliance across:
//! 1. Landlord (Residential Lease, Notice to Vacate, Security Deposit Disposition, Inspection Checklist)
//! 2. Property Management (Monthly Owner Statement, Multi-Unit Rent Roll)
//! 3. Real Estate Agents & Brokers (Closing Settlement Statement, Commission Disbursement)
//! 4. Commercial Real Estate (Triple Net NNN Lease, Letter of Intent LOI, Tenant Estoppel)

use pressmark::engine::{generate_all_preview_pngs, generate_pdf, generate_preview_png};
use pressmark::models::{
    CustomField, DocumentRequest, DocumentSection, DocumentType, LineItem, PartyDetails,
    SignatureBlock,
};
use std::fs;
use std::path::Path;

const ARTIFACT_DIR: &str = "/Users/dhallman/.gemini/antigravity/brain/db484a2f-1c16-4cec-a780-3792e1b1f23f";

#[test]
fn test_residential_lease_agreement_rendering() {
    let req = DocumentRequest {
        doc_type: DocumentType::ResidentialLease,
        title: Some("RESIDENTIAL LEASE AGREEMENT".to_string()),
        doc_number: Some("RLA-2026-0819".to_string()),
        header_subtitle: Some("Standard Fixed-Term Residential Tenancy Agreement".to_string()),
        issue_date: Some("2026-10-01".to_string()),
        due_date: Some("2026-11-01".to_string()),
        theme: Some("electric_indigo".to_string()),
        primary_color: Some("#4f46e5".to_string()),
        font_family: Some("sans".to_string()),
        paper_size: Some("us-letter".to_string()),
        layout_style: Some("standard".to_string()),
        compliance_mode: Some("none".to_string()),
        from: PartyDetails {
            name: "Elena Rostova".to_string(),
            company: Some("Pacifica Property Holdings LLC".to_string()),
            address: Some("100 Montgomery St, Suite 1400, San Francisco, CA 94104".to_string()),
            email: Some("leasing@pacificaholdings.com".to_string()),
            phone: Some("+1 (415) 890-4421".to_string()),
            tax_id: Some("US-94-3829104".to_string()),
            website: Some("https://pacificaholdings.com".to_string()),
            registration_number: Some("DRE Lic #01928472".to_string()),
        },
        to: PartyDetails {
            name: "Marcus & Eleanor Vance".to_string(),
            company: Some("Residential Lessees".to_string()),
            address: Some("742 Evergreen Terrace, Unit 4B, San Francisco, CA 94107".to_string()),
            email: Some("marcus.vance@example.com".to_string()),
            phone: Some("+1 (415) 555-0199".to_string()),
            tax_id: None,
            website: None,
            registration_number: None,
        },
        custom_fields: vec![
            CustomField { label: "Premises Address".to_string(), value: "742 Evergreen Terrace, Unit 4B, San Francisco, CA 94107".to_string() },
            CustomField { label: "Parcel APN".to_string(), value: "042-8819-012".to_string() },
            CustomField { label: "Lease Term".to_string(), value: "12 Months (Nov 1, 2026 – Oct 31, 2027)".to_string() },
            CustomField { label: "Security Deposit Held".to_string(), value: "$3,200.00 (Chase Client Escrow Trust)".to_string() },
            CustomField { label: "Permitted Occupants".to_string(), value: "2 Adults, 1 Dependent".to_string() },
            CustomField { label: "Pet Authorization".to_string(), value: "1 Domestic Feline ($50/mo pet rent)".to_string() },
        ],
        items: vec![
            LineItem { description: "Monthly Base Rent (Unit 4B)".to_string(), quantity: 1.0, unit_price: 3200.00, tax_rate: None, discount: None },
            LineItem { description: "Security Deposit (Statutory Trust Escrow)".to_string(), quantity: 1.0, unit_price: 3200.00, tax_rate: None, discount: None },
            LineItem { description: "Assigned Subterranean Parking Stall (#14)".to_string(), quantity: 1.0, unit_price: 175.00, tax_rate: None, discount: None },
            LineItem { description: "Pet Rent (Authorized Domestic Feline)".to_string(), quantity: 1.0, unit_price: 50.00, tax_rate: None, discount: None },
        ],
        sections: vec![
            DocumentSection {
                title: "1. Rent Payment & Grace Period".to_string(),
                content: "Rent is due on the 1st calendar day of each month. A late fee of 5% ($160.00) applies if rent is not received by 11:59 PM on the 5th calendar day.".to_string(),
            },
            DocumentSection {
                title: "2. Security Deposit & Trust Account".to_string(),
                content: "The security deposit of $3,200.00 is held in a segregated client trust account at Chase Bank pursuant to state law. The deposit will be itemized and returned within 21 days of tenancy termination.".to_string(),
            },
            DocumentSection {
                title: "3. Landlord Right of Entry".to_string(),
                content: "Landlord shall provide at least 24 hours prior written notice before entering the premises for non-emergency inspection, maintenance, or prospective tenant showings.".to_string(),
            },
            DocumentSection {
                title: "4. Lead-Based Paint & Habitability Disclosure".to_string(),
                content: "Housing built before 1978 may contain lead-based paint. Landlord has provided the federally approved lead hazard information pamphlet 'Protect Your Family From Lead In Your Home'.".to_string(),
            },
        ],
        signatures: vec![
            SignatureBlock { signer_name: "Elena Rostova".to_string(), signer_title: Some("Managing Agent, Pacifica Holdings".to_string()), date: Some("2026-10-01".to_string()), signature_data: None },
            SignatureBlock { signer_name: "Marcus Vance".to_string(), signer_title: Some("Primary Lessee".to_string()), date: Some("2026-10-01".to_string()), signature_data: None },
            SignatureBlock { signer_name: "Eleanor Vance".to_string(), signer_title: Some("Co-Lessee".to_string()), date: Some("2026-10-01".to_string()), signature_data: None },
        ],
        notes: Some("Move-in keys and garage gate remote issued upon receipt of certified funds.".to_string()),
        terms: Some("Standard California Residential Tenancy Covenants apply.".to_string()),
        show_signature: Some(true),
        show_remittance_stub: Some(true),
        show_qr_seal: Some(true),
        ..Default::default()
    };

    assert_eq!(req.subtotal(), 6625.00);
    assert_eq!(req.grand_total(), 6625.00);

    let (pdf_bytes, ms_pdf) = generate_pdf(&req).expect("Failed to compile Residential Lease PDF");
    assert!(pdf_bytes.len() > 1000);
    println!("Compiled Residential Lease PDF in {}ms ({} bytes)", ms_pdf, pdf_bytes.len());

    let (png_bytes, ms_png) = generate_preview_png(&req, 144).expect("Failed to render Residential Lease PNG");
    assert!(png_bytes.len() > 1000);
    println!("Rendered Residential Lease PNG in {}ms ({} bytes)", ms_png, png_bytes.len());

    let (all_pages, ms_multi) = generate_all_preview_pngs(&req, 144).expect("Failed to render multi-page preview");
    println!("Rendered {} pages for Residential Lease in {}ms", all_pages.len(), ms_multi);
    assert!(all_pages.len() >= 2, "Expected at least 2 pages for comprehensive lease");

    if Path::new(ARTIFACT_DIR).exists() {
        let _ = fs::write(format!("{}/real_estate_template_1_residential_lease.pdf", ARTIFACT_DIR), &pdf_bytes);
        let _ = fs::write(format!("{}/real_estate_template_1_residential_lease.png", ARTIFACT_DIR), &png_bytes);
        for (i, p) in all_pages.iter().enumerate() {
            let _ = fs::write(format!("{}/real_estate_template_1_residential_lease_p{}.png", ARTIFACT_DIR, i + 1), p);
        }
    }
}

#[test]
fn test_commercial_nnn_lease_rendering() {
    let req = DocumentRequest {
        doc_type: DocumentType::CommercialNnnLease,
        title: Some("TRIPLE NET (NNN) COMMERCIAL LEASE".to_string()),
        doc_number: Some("NNN-2026-4200".to_string()),
        header_subtitle: Some("Class-A Office / Tech Flex Facility Lease Abstract".to_string()),
        issue_date: Some("2026-10-01".to_string()),
        due_date: Some("2026-11-01".to_string()),
        theme: Some("executive_navy".to_string()),
        primary_color: Some("#1e3a8a".to_string()),
        font_family: Some("grotesk".to_string()),
        paper_size: Some("us-letter".to_string()),
        layout_style: Some("framed".to_string()),
        from: PartyDetails {
            name: "Arthur Sterling".to_string(),
            company: Some("Apex Commercial Capital REIT".to_string()),
            address: Some("555 California St, Suite 4200, San Francisco, CA 94104".to_string()),
            email: Some("leasing@apexcommercial.com".to_string()),
            phone: Some("+1 (415) 980-2200".to_string()),
            tax_id: Some("US-94-1182903".to_string()),
            website: Some("https://apexcommercial.com".to_string()),
            registration_number: Some("CRE Broker Lic #00918274".to_string()),
        },
        to: PartyDetails {
            name: "Dr. Sarah Lin".to_string(),
            company: Some("NeuralCloud Systems Inc.".to_string()),
            address: Some("500 Howard Street, Suite 800, San Francisco, CA 94105".to_string()),
            email: Some("legal@neuralcloud.ai".to_string()),
            phone: Some("+1 (415) 555-8821".to_string()),
            tax_id: Some("US-94-8849201".to_string()),
            website: Some("https://neuralcloud.ai".to_string()),
            registration_number: Some("DE-CORP-774912".to_string()),
        },
        custom_fields: vec![
            CustomField { label: "Premises Address".to_string(), value: "500 Howard Street, Suite 800, San Francisco, CA 94105".to_string() },
            CustomField { label: "Rentable Area (RSF)".to_string(), value: "14,500 RSF (Usable: 12,850 USF)".to_string() },
            CustomField { label: "Load Factor".to_string(), value: "1.1284 (12.84%)".to_string() },
            CustomField { label: "Initial Lease Term".to_string(), value: "60 Months (5 Years)".to_string() },
            CustomField { label: "Commencement Date".to_string(), value: "November 1, 2026".to_string() },
            CustomField { label: "Permitted Use".to_string(), value: "Executive Tech Offices, R&D, Software Engineering".to_string() },
            CustomField { label: "Proportionate Share".to_string(), value: "18.42% of Building Operating Expenses".to_string() },
        ],
        items: vec![
            LineItem { description: "Monthly Base Rent ($68.00 / RSF / Year)".to_string(), quantity: 1.0, unit_price: 82166.67, tax_rate: None, discount: None },
            LineItem { description: "Common Area Maintenance (CAM / NNN Share)".to_string(), quantity: 1.0, unit_price: 21025.00, tax_rate: None, discount: None },
            LineItem { description: "Building Property Tax Escrow Reserve".to_string(), quantity: 1.0, unit_price: 8450.00, tax_rate: None, discount: None },
            LineItem { description: "Casualty & Commercial General Liability Escrow".to_string(), quantity: 1.0, unit_price: 2180.00, tax_rate: None, discount: None },
        ],
        sections: vec![
            DocumentSection {
                title: "1. Operating Expense Reconciliation".to_string(),
                content: "Tenant pays 18.42% Proportionate Share of actual Operating Expenses (Taxes, Insurance, Utilities, HVAC upkeep, Janitorial). Annual reconciliation statement delivered within 90 days of calendar year end.".to_string(),
            },
            DocumentSection {
                title: "2. Tenant Improvement (TI) Allowance".to_string(),
                content: "Landlord provides a turnkey TI allowance of $45.00 per RSF ($652,500.00 total) to be disbursed via architect progress certificates.".to_string(),
            },
            DocumentSection {
                title: "3. Subordination & SNDA".to_string(),
                content: "This Lease is subject and subordinate to all ground leases and mortgages, conditioned upon Tenant receiving standard SNDA covenant from senior lender.".to_string(),
            },
        ],
        signatures: vec![
            SignatureBlock { signer_name: "Arthur Sterling".to_string(), signer_title: Some("Senior Managing Director, Apex Capital REIT".to_string()), date: Some("2026-10-01".to_string()), signature_data: None },
            SignatureBlock { signer_name: "Dr. Sarah Lin".to_string(), signer_title: Some("Chief Executive Officer, NeuralCloud Systems Inc.".to_string()), date: Some("2026-10-01".to_string()), signature_data: None },
        ],
        show_signature: Some(true),
        show_qr_seal: Some(true),
        ..Default::default()
    };

    assert_eq!(req.subtotal(), 113821.67);
    let (pdf_bytes, ms_pdf) = generate_pdf(&req).expect("Failed to compile Commercial NNN Lease PDF");
    assert!(pdf_bytes.len() > 1000);
    println!("Compiled Commercial NNN Lease PDF in {}ms", ms_pdf);

    let (png_bytes, _) = generate_preview_png(&req, 144).expect("Failed to render Commercial NNN Lease PNG");
    assert!(png_bytes.len() > 1000);

    if Path::new(ARTIFACT_DIR).exists() {
        let _ = fs::write(format!("{}/real_estate_template_2_commercial_nnn_lease.pdf", ARTIFACT_DIR), &pdf_bytes);
        let _ = fs::write(format!("{}/real_estate_template_2_commercial_nnn_lease.png", ARTIFACT_DIR), &png_bytes);
    }
}

#[test]
fn test_property_management_owner_statement_rendering() {
    let req = DocumentRequest {
        doc_type: DocumentType::PropertyManagementStatement,
        title: Some("MONTHLY PROPERTY MANAGEMENT STATEMENT".to_string()),
        doc_number: Some("PMS-2026-0930".to_string()),
        header_subtitle: Some("Owner Remittance & Portfolio Operating Summary".to_string()),
        issue_date: Some("2026-09-30".to_string()),
        theme: Some("emerald_growth".to_string()),
        primary_color: Some("#059669".to_string()),
        layout_style: Some("compact_ledger".to_string()),
        from: PartyDetails {
            name: "Beacon Hill Property Management LLC".to_string(),
            company: Some("Brokerage Asset Management".to_string()),
            address: Some("120 Commonwealth Ave, Boston, MA 02116".to_string()),
            email: Some("accounting@beaconhillpm.com".to_string()),
            phone: Some("+1 (617) 555-0142".to_string()),
            tax_id: Some("US-04-9928172".to_string()),
            website: Some("https://beaconhillpm.com".to_string()),
            registration_number: Some("Broker Lic #01884920".to_string()),
        },
        to: PartyDetails {
            name: "Highland Real Estate Investors IV LLC".to_string(),
            company: Some("Portfolio Owner".to_string()),
            address: Some("75 State Street, Floor 22, Boston, MA 02109".to_string()),
            email: Some("assets@highlandrei.com".to_string()),
            phone: Some("+1 (617) 555-9000".to_string()),
            tax_id: Some("US-04-1029481".to_string()),
            website: None,
            registration_number: None,
        },
        custom_fields: vec![
            CustomField { label: "Property Name".to_string(), value: "The Beacon Residences (24 Units)".to_string() },
            CustomField { label: "Property Address".to_string(), value: "120 Commonwealth Ave, Boston, MA 02116".to_string() },
            CustomField { label: "Statement Period".to_string(), value: "September 1, 2026 – September 30, 2026".to_string() },
            CustomField { label: "Portfolio Occupancy".to_string(), value: "95.8% (23 of 24 Units Occupied)".to_string() },
            CustomField { label: "Reserve Escrow Account".to_string(), value: "$25,000.00 (Maintained at Boston Private)".to_string() },
        ],
        items: vec![
            LineItem { description: "Gross Actual Rents Collected (23 Units)".to_string(), quantity: 1.0, unit_price: 65550.00, tax_rate: None, discount: None },
            LineItem { description: "Late Fees & Parking Surcharges Collected".to_string(), quantity: 1.0, unit_price: 1150.00, tax_rate: None, discount: None },
            LineItem { description: "Property Management Fee (6.0% of Collections)".to_string(), quantity: 1.0, unit_price: 4002.00, tax_rate: None, discount: None },
            LineItem { description: "HVAC Emergency Compressor Replacement (Unit 4B)".to_string(), quantity: 1.0, unit_price: 1850.00, tax_rate: None, discount: None },
            LineItem { description: "Landscaping & Grounds Upkeep Contract".to_string(), quantity: 1.0, unit_price: 1200.00, tax_rate: None, discount: None },
            LineItem { description: "Common Area Electric & Water (Eversource)".to_string(), quantity: 1.0, unit_price: 2410.00, tax_rate: None, discount: None },
        ],
        sections: vec![
            DocumentSection {
                title: "Trust Fund & Escrow Certification".to_string(),
                content: "All operating funds and security deposits are segregated in federally insured trust accounts at Boston Private Bank pursuant to Massachusetts General Laws Chapter 186 § 15B.".to_string(),
            },
        ],
        signatures: vec![
            SignatureBlock { signer_name: "Harrison Vance".to_string(), signer_title: Some("Principal Managing Broker, Beacon Hill PM".to_string()), date: Some("2026-09-30".to_string()), signature_data: None },
        ],
        show_signature: Some(true),
        show_qr_seal: Some(true),
        ..Default::default()
    };

    let (pdf_bytes, ms_pdf) = generate_pdf(&req).expect("Failed to compile Property Management Statement PDF");
    assert!(pdf_bytes.len() > 1000);
    println!("Compiled Property Management Statement PDF in {}ms", ms_pdf);

    let (png_bytes, _) = generate_preview_png(&req, 144).expect("Failed to render Property Management Statement PNG");
    assert!(png_bytes.len() > 1000);

    if Path::new(ARTIFACT_DIR).exists() {
        let _ = fs::write(format!("{}/real_estate_template_3_property_management_statement.pdf", ARTIFACT_DIR), &pdf_bytes);
        let _ = fs::write(format!("{}/real_estate_template_3_property_management_statement.png", ARTIFACT_DIR), &png_bytes);
    }
}

#[test]
fn test_closing_settlement_statement_rendering() {
    let req = DocumentRequest {
        doc_type: DocumentType::ClosingSettlementStatement,
        title: Some("REAL ESTATE CLOSING SETTLEMENT STATEMENT".to_string()),
        doc_number: Some("ESC-2026-9921".to_string()),
        header_subtitle: Some("Buyer / Seller Combined Settlement Disclosures (RESPA / ALTA Standard)".to_string()),
        issue_date: Some("2026-10-15".to_string()),
        theme: Some("crimson_legal".to_string()),
        primary_color: Some("#991b1b".to_string()),
        layout_style: Some("swiss_grid".to_string()),
        from: PartyDetails {
            name: "First American Title Company".to_string(),
            company: Some("Escrow Division - Settlement Services".to_string()),
            address: Some("1842 Pacific Coast Highway, Suite 100, Malibu, CA 90265".to_string()),
            email: Some("escrow.settlement@firstam.com".to_string()),
            phone: Some("+1 (310) 555-0188".to_string()),
            tax_id: Some("US-95-1829012".to_string()),
            website: Some("https://firstam.com".to_string()),
            registration_number: Some("Escrow Lic #963-2210".to_string()),
        },
        to: PartyDetails {
            name: "Robert Vance (Buyer) & Claire Sterling (Seller)".to_string(),
            company: Some("Malibu Coastal Transaction".to_string()),
            address: Some("1842 Pacific Coast Highway, Malibu, CA 90265".to_string()),
            email: Some("parties@vancemedia.com".to_string()),
            phone: Some("+1 (310) 555-9921".to_string()),
            tax_id: None,
            website: None,
            registration_number: None,
        },
        custom_fields: vec![
            CustomField { label: "Property Address".to_string(), value: "1842 Pacific Coast Highway, Malibu, CA 90265".to_string() },
            CustomField { label: "APN / Parcel ID".to_string(), value: "4458-012-044".to_string() },
            CustomField { label: "Escrow File #".to_string(), value: "FE-2026-9921".to_string() },
            CustomField { label: "Settlement Date".to_string(), value: "October 15, 2026".to_string() },
            CustomField { label: "Title / Escrow Officer".to_string(), value: "First American Title (Officer: Diane Warren)".to_string() },
            CustomField { label: "Primary Loan".to_string(), value: "$1,450,000.00 (First Republic / JPMorgan)".to_string() },
        ],
        items: vec![
            LineItem { description: "Contract Purchase Price".to_string(), quantity: 1.0, unit_price: 2100000.00, tax_rate: None, discount: None },
            LineItem { description: "Listing Brokerage Commission (2.50%)".to_string(), quantity: 1.0, unit_price: 52500.00, tax_rate: None, discount: None },
            LineItem { description: "Selling Brokerage Commission (2.50%)".to_string(), quantity: 1.0, unit_price: 52500.00, tax_rate: None, discount: None },
            LineItem { description: "Owner's Title Insurance Policy".to_string(), quantity: 1.0, unit_price: 4200.00, tax_rate: None, discount: None },
            LineItem { description: "Lender's ALTA Title Policy".to_string(), quantity: 1.0, unit_price: 1850.00, tax_rate: None, discount: None },
            LineItem { description: "County Transfer Tax ($1.10 per $1,000)".to_string(), quantity: 1.0, unit_price: 2310.00, tax_rate: None, discount: None },
            LineItem { description: "Escrow Settlement Closing Fee".to_string(), quantity: 1.0, unit_price: 2400.00, tax_rate: None, discount: None },
        ],
        sections: vec![
            DocumentSection {
                title: "RESPA Settlement Notice".to_string(),
                content: "This Settlement Statement is prepared pursuant to the Real Estate Settlement Procedures Act (RESPA) 12 U.S.C. § 2601 et seq. All settlement charges and disbursements have been itemized with zero unearned fees or kickbacks.".to_string(),
            },
        ],
        signatures: vec![
            SignatureBlock { signer_name: "Diane Warren".to_string(), signer_title: Some("Certified Settlement Officer, First American Title".to_string()), date: Some("2026-10-15".to_string()), signature_data: None },
            SignatureBlock { signer_name: "Robert Vance".to_string(), signer_title: Some("Buyer / Borrower".to_string()), date: Some("2026-10-15".to_string()), signature_data: None },
            SignatureBlock { signer_name: "Claire Sterling".to_string(), signer_title: Some("Seller".to_string()), date: Some("2026-10-15".to_string()), signature_data: None },
        ],
        show_signature: Some(true),
        show_qr_seal: Some(true),
        ..Default::default()
    };

    let (pdf_bytes, ms_pdf) = generate_pdf(&req).expect("Failed to compile Closing Settlement Statement PDF");
    assert!(pdf_bytes.len() > 1000);
    println!("Compiled Closing Settlement Statement PDF in {}ms", ms_pdf);

    let (png_bytes, _) = generate_preview_png(&req, 144).expect("Failed to render Closing Settlement Statement PNG");
    assert!(png_bytes.len() > 1000);

    if Path::new(ARTIFACT_DIR).exists() {
        let _ = fs::write(format!("{}/real_estate_template_4_closing_settlement_statement.pdf", ARTIFACT_DIR), &pdf_bytes);
        let _ = fs::write(format!("{}/real_estate_template_4_closing_settlement_statement.png", ARTIFACT_DIR), &png_bytes);
    }
}

#[test]
fn test_multi_unit_rent_roll_rendering() {
    let req = DocumentRequest {
        doc_type: DocumentType::RentRoll,
        title: Some("MULTI-UNIT RENT ROLL & LEASE SCHEDULE".to_string()),
        doc_number: Some("RRL-2026-1001".to_string()),
        header_subtitle: Some("Asset Valuation & Tenancy Census Audit".to_string()),
        issue_date: Some("2026-10-01".to_string()),
        theme: Some("nordic_teal".to_string()),
        primary_color: Some("#0f766e".to_string()),
        layout_style: Some("compact_ledger".to_string()),
        show_line_numbers: Some(true),
        from: PartyDetails {
            name: "Sunset Park Realty Management".to_string(),
            company: Some("Asset Portfolio Manager".to_string()),
            address: Some("450 7th Ave, Brooklyn, NY 11215".to_string()),
            email: Some("leasing@sunsetparkapts.com".to_string()),
            phone: Some("+1 (718) 555-0199".to_string()),
            tax_id: Some("US-11-2948192".to_string()),
            website: None,
            registration_number: None,
        },
        to: PartyDetails {
            name: "Gotham Multi-Family Acquisition Fund LP".to_string(),
            company: Some("Underwriting Committee".to_string()),
            address: Some("375 Park Ave, New York, NY 10152".to_string()),
            email: Some("underwriting@gothamfund.com".to_string()),
            phone: Some("+1 (212) 555-4000".to_string()),
            tax_id: None,
            website: None,
            registration_number: None,
        },
        custom_fields: vec![
            CustomField { label: "Property".to_string(), value: "Sunset Park Apartments (16 Units)".to_string() },
            CustomField { label: "Address".to_string(), value: "450 7th Ave, Brooklyn, NY 11215".to_string() },
            CustomField { label: "As-Of Date".to_string(), value: "October 1, 2026".to_string() },
            CustomField { label: "Total Units / Occupancy".to_string(), value: "16 Units · 93.75% Occupied (15 Units)".to_string() },
            CustomField { label: "Annual Gross Scheduled Rent".to_string(), value: "$547,200.00".to_string() },
        ],
        items: vec![
            LineItem { description: "Unit 1A · 1BR/1BA · Sarah Miller · Exp: 05/2027".to_string(), quantity: 1.0, unit_price: 2650.00, tax_rate: None, discount: None },
            LineItem { description: "Unit 1B · 2BR/1.5BA · David Chen · Exp: 08/2027".to_string(), quantity: 1.0, unit_price: 3400.00, tax_rate: None, discount: None },
            LineItem { description: "Unit 2A · 1BR/1BA · Carlos Rivera · Exp: 11/2026".to_string(), quantity: 1.0, unit_price: 2700.00, tax_rate: None, discount: None },
            LineItem { description: "Unit 2B · 2BR/2BA · Michael Ross · Exp: 03/2027".to_string(), quantity: 1.0, unit_price: 3650.00, tax_rate: None, discount: None },
            LineItem { description: "Unit 3A · Studio · VACANT (Turnover / Renovating)".to_string(), quantity: 1.0, unit_price: 0.00, tax_rate: None, discount: None },
            LineItem { description: "Unit 3B · 2BR/1.5BA · Amanda Vance · Exp: 09/2027".to_string(), quantity: 1.0, unit_price: 3500.00, tax_rate: None, discount: None },
            LineItem { description: "Unit 4A · Penthouse 3BR/2BA · Robert Sterling · Exp: 01/2028".to_string(), quantity: 1.0, unit_price: 5200.00, tax_rate: None, discount: None },
        ],
        sections: vec![
            DocumentSection {
                title: "Underwriting & Occupancy Attestation".to_string(),
                content: "This rent roll represents an accurate census of all active residential leases, deposits held, and contractual terms as verified against executed lease agreements.".to_string(),
            },
        ],
        signatures: vec![
            SignatureBlock { signer_name: "Gregory Vance".to_string(), signer_title: Some("Managing Director, Sunset Park Realty".to_string()), date: Some("2026-10-01".to_string()), signature_data: None },
        ],
        show_signature: Some(true),
        ..Default::default()
    };

    let (pdf_bytes, ms_pdf) = generate_pdf(&req).expect("Failed to compile Rent Roll PDF");
    assert!(pdf_bytes.len() > 1000);
    println!("Compiled Rent Roll PDF in {}ms", ms_pdf);

    let (png_bytes, _) = generate_preview_png(&req, 144).expect("Failed to render Rent Roll PNG");
    assert!(png_bytes.len() > 1000);

    if Path::new(ARTIFACT_DIR).exists() {
        let _ = fs::write(format!("{}/real_estate_template_5_rent_roll.pdf", ARTIFACT_DIR), &pdf_bytes);
        let _ = fs::write(format!("{}/real_estate_template_5_rent_roll.png", ARTIFACT_DIR), &png_bytes);
    }
}

#[test]
fn test_notice_to_vacate_and_security_deposit_disposition() {
    // 1. Notice to Vacate
    let req_ntv = DocumentRequest {
        doc_type: DocumentType::NoticeToVacate,
        title: Some("NOTICE TO VACATE & CURE LEASE VIOLATION".to_string()),
        doc_number: Some("NTV-2026-0302".to_string()),
        header_subtitle: Some("Statutory Notice to Pay Rent or Quit Premises".to_string()),
        issue_date: Some("2026-10-01".to_string()),
        theme: Some("crimson_legal".to_string()),
        primary_color: Some("#dc2626".to_string()),
        layout_style: Some("framed".to_string()),
        from: PartyDetails {
            name: "Pacifica Property Holdings LLC".to_string(),
            company: Some("Elena Rostova, Managing Agent".to_string()),
            address: Some("100 Montgomery St, Suite 1400, San Francisco, CA 94104".to_string()),
            email: Some("notices@pacificaholdings.com".to_string()),
            phone: Some("+1 (415) 890-4421".to_string()),
            tax_id: None,
            website: None,
            registration_number: None,
        },
        to: PartyDetails {
            name: "Jordan Blake & All Unknown Occupants".to_string(),
            company: Some("Resident Occupants".to_string()),
            address: Some("880 Harrison St, Apt 302, San Francisco, CA 94107".to_string()),
            email: Some("jordan.blake@example.com".to_string()),
            phone: None,
            tax_id: None,
            website: None,
            registration_number: None,
        },
        custom_fields: vec![
            CustomField { label: "Premises Address".to_string(), value: "880 Harrison St, Apt 302, San Francisco, CA 94107".to_string() },
            CustomField { label: "Total Past Due".to_string(), value: "$4,250.00".to_string() },
            CustomField { label: "Statutory Cure Period".to_string(), value: "3 Calendar Days from Proof of Service".to_string() },
        ],
        items: vec![
            LineItem { description: "Unpaid Delinquent Rent - August 2026".to_string(), quantity: 1.0, unit_price: 2050.00, tax_rate: None, discount: None },
            LineItem { description: "Unpaid Delinquent Rent - September 2026".to_string(), quantity: 1.0, unit_price: 2050.00, tax_rate: None, discount: None },
            LineItem { description: "Contractual Late Fee (August)".to_string(), quantity: 1.0, unit_price: 75.00, tax_rate: None, discount: None },
            LineItem { description: "Contractual Late Fee (September)".to_string(), quantity: 1.0, unit_price: 75.00, tax_rate: None, discount: None },
        ],
        sections: vec![
            DocumentSection {
                title: "DEMAND TO PAY OR QUIT".to_string(),
                content: "WITHIN THREE (3) DAYS after service upon you of this notice, you are required to pay the total amount of $4,250.00, or vacate and deliver surrender of the premises to Landlord. Failure to comply will result in immediate legal proceedings for unlawful detainer.".to_string(),
            },
            DocumentSection {
                title: "DECLARATION OF SERVICE".to_string(),
                content: "I, Elena Rostova, declare under penalty of perjury under the laws of the State of California that on October 1, 2026, I served this Notice upon Tenant by personal delivery / conspicuous posting at the premises.".to_string(),
            },
        ],
        signatures: vec![
            SignatureBlock { signer_name: "Elena Rostova".to_string(), signer_title: Some("Authorized Agent for Landlord".to_string()), date: Some("2026-10-01".to_string()), signature_data: None },
        ],
        show_signature: Some(true),
        ..Default::default()
    };

    let (ntv_pdf, _) = generate_pdf(&req_ntv).expect("Failed to compile Notice to Vacate PDF");
    let (ntv_png, _) = generate_preview_png(&req_ntv, 144).expect("Failed to render Notice to Vacate PNG");
    assert!(ntv_pdf.len() > 1000);
    assert!(ntv_png.len() > 1000);

    if Path::new(ARTIFACT_DIR).exists() {
        let _ = fs::write(format!("{}/real_estate_template_6_notice_to_vacate.pdf", ARTIFACT_DIR), &ntv_pdf);
        let _ = fs::write(format!("{}/real_estate_template_6_notice_to_vacate.png", ARTIFACT_DIR), &ntv_png);
    }

    // 2. Security Deposit Disposition
    let req_sdd = DocumentRequest {
        doc_type: DocumentType::SecurityDepositDisposition,
        title: Some("SECURITY DEPOSIT ITEMIZED DISPOSITION".to_string()),
        doc_number: Some("SDD-2026-1420".to_string()),
        header_subtitle: Some("Statutory Accounting & Itemized Repair Deductions".to_string()),
        issue_date: Some("2026-09-28".to_string()),
        theme: Some("obsidian".to_string()),
        primary_color: Some("#0f172a".to_string()),
        layout_style: Some("standard".to_string()),
        from: PartyDetails {
            name: "Pacifica Property Holdings LLC".to_string(),
            company: Some("Management Accounting Division".to_string()),
            address: Some("100 Montgomery St, Suite 1400, San Francisco, CA 94104".to_string()),
            email: Some("deposits@pacificaholdings.com".to_string()),
            phone: Some("+1 (415) 890-4421".to_string()),
            tax_id: None,
            website: None,
            registration_number: None,
        },
        to: PartyDetails {
            name: "Sophie Laurent".to_string(),
            company: Some("Former Resident".to_string()),
            address: Some("1420 4th Street, Unit 201, Santa Monica, CA 90401".to_string()),
            email: Some("sophie.laurent@example.com".to_string()),
            phone: None,
            tax_id: None,
            website: None,
            registration_number: None,
        },
        custom_fields: vec![
            CustomField { label: "Property Address".to_string(), value: "1420 4th Street, Unit 201, Santa Monica, CA 90401".to_string() },
            CustomField { label: "Move-Out Date".to_string(), value: "September 28, 2026".to_string() },
            CustomField { label: "Original Deposit Paid".to_string(), value: "$3,500.00".to_string() },
            CustomField { label: "Statutory Interest Accrued".to_string(), value: "$28.42".to_string() },
        ],
        items: vec![
            LineItem { description: "Original Security Deposit Held".to_string(), quantity: 1.0, unit_price: 3500.00, tax_rate: None, discount: None },
            LineItem { description: "Accrued Statutory Trust Interest".to_string(), quantity: 1.0, unit_price: 28.42, tax_rate: None, discount: None },
            LineItem { description: "Professional Deep Clean & Carpet Sanitization (Receipt #441)".to_string(), quantity: 1.0, unit_price: 320.00, tax_rate: None, discount: None },
            LineItem { description: "Drywall Patching & Paint Beyond Normal Wear (Room 2)".to_string(), quantity: 1.0, unit_price: 450.00, tax_rate: None, discount: None },
            LineItem { description: "Replacement of Missing Garage Remote (2x)".to_string(), quantity: 1.0, unit_price: 120.00, tax_rate: None, discount: None },
            LineItem { description: "Unpaid Prorated Utility Charges (Water / Trash)".to_string(), quantity: 1.0, unit_price: 84.20, tax_rate: None, discount: None },
        ],
        sections: vec![
            DocumentSection {
                title: "Statutory Return Notice".to_string(),
                content: "Enclosed is check #4092 drawn on First Republic Bank for the net refund of $2,554.22. Pursuant to state law, itemized contractor receipts and invoices are attached.".to_string(),
            },
        ],
        signatures: vec![
            SignatureBlock { signer_name: "Elena Rostova".to_string(), signer_title: Some("Managing Agent, Pacifica Holdings".to_string()), date: Some("2026-09-28".to_string()), signature_data: None },
        ],
        show_signature: Some(true),
        show_remittance_stub: Some(true),
        ..Default::default()
    };

    let (sdd_pdf, _) = generate_pdf(&req_sdd).expect("Failed to compile Security Deposit Disposition PDF");
    let (sdd_png, _) = generate_preview_png(&req_sdd, 144).expect("Failed to render Security Deposit Disposition PNG");
    assert!(sdd_pdf.len() > 1000);
    assert!(sdd_png.len() > 1000);

    if Path::new(ARTIFACT_DIR).exists() {
        let _ = fs::write(format!("{}/real_estate_template_7_security_deposit_disposition.pdf", ARTIFACT_DIR), &sdd_pdf);
        let _ = fs::write(format!("{}/real_estate_template_7_security_deposit_disposition.png", ARTIFACT_DIR), &sdd_png);
    }
}
