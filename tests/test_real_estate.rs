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

#[test]
fn test_bug_c3_c4_c5_deduction_math_recordable_deed_and_9pt_lease_terms() {
    use pressmark::engine::generate_typst_markup;

    // 1. Bug C4: Verify negative deduction math on SecurityDepositDisposition yields exact $2,554.22 refund
    let req_sdd = DocumentRequest {
        doc_type: DocumentType::SecurityDepositDisposition,
        from: PartyDetails {
            name: "Pacifica Property Holdings LLC".to_string(),
            address: Some("100 Montgomery St, Suite 1400, San Francisco, CA 94104".to_string()),
            ..Default::default()
        },
        to: PartyDetails {
            name: "Sophie Laurent".to_string(),
            address: Some("1420 4th Street, Unit 201, Santa Monica, CA 90401".to_string()),
            ..Default::default()
        },
        items: vec![
            LineItem { description: "Original Security Deposit Held".to_string(), quantity: 1.0, unit_price: 3500.00, tax_rate: None, discount: None },
            LineItem { description: "Accrued Statutory Trust Interest".to_string(), quantity: 1.0, unit_price: 28.42, tax_rate: None, discount: None },
            LineItem { description: "Less: Professional Deep Clean (Receipt #441)".to_string(), quantity: 1.0, unit_price: 0.0, tax_rate: None, discount: Some(320.00) },
            LineItem { description: "Less: Drywall Patching & Paint Beyond Normal Wear".to_string(), quantity: 1.0, unit_price: 0.0, tax_rate: None, discount: Some(450.00) },
            LineItem { description: "Less: Replacement of Missing Garage Remote (2x)".to_string(), quantity: 1.0, unit_price: 0.0, tax_rate: None, discount: Some(120.00) },
            LineItem { description: "Less: Unpaid Prorated Utility Charges".to_string(), quantity: 1.0, unit_price: 0.0, tax_rate: None, discount: Some(84.20) },
        ],
        terms: Some("Statutory itemized disposition pursuant to Cal. Civ. Code § 1950.5 within 21 calendar days.".to_string()),
        ..Default::default()
    };

    assert!((req_sdd.subtotal() - 2554.22).abs() < 0.005, "Expected $2,554.22 net refund, got {}", req_sdd.subtotal());
    assert!((req_sdd.grand_total() - 2554.22).abs() < 0.005);
    let sdd_markup = generate_typst_markup(&req_sdd);
    assert!(sdd_markup.contains("Security Deposit Itemized Accounting & Refund Statement"));
    assert!(sdd_markup.contains("2,554.22"));
    // Bug C5: Verify terms on contract/lease/real-estate instruments render at 9pt primary legibility
    assert!(sdd_markup.contains("#text(size: 9pt, fill: color-slate-700)"));

    let (sdd_pdf, _) = generate_pdf(&req_sdd).expect("Failed to compile SecurityDepositDisposition with negative deductions");
    assert!(sdd_pdf.starts_with(b"%PDF-"));

    // 2. Bug C3: Verify PropertyDeed / recordable: true renders 3.0in County Recorder Stamp Margin, #recording-header-block, and dynamic RON jurat
    let req_deed = DocumentRequest {
        doc_type: DocumentType::PropertyDeed,
        state_code: Some("TN".to_string()),
        county: Some("Hamilton".to_string()),
        recordable: Some(true),
        notary_required: Some(true),
        from: PartyDetails {
            name: "Highland Crest Holdings LLC".to_string(),
            address: Some("412 Northshore Dr, Chattanooga, TN 37405".to_string()),
            ..Default::default()
        },
        to: PartyDetails {
            name: "Hallman Capital Properties LLC".to_string(),
            address: Some("1200 Market St, Suite 400, Chattanooga, TN 37402".to_string()),
            ..Default::default()
        },
        custom_fields: vec![
            CustomField { label: "APN".to_string(), value: "135K-A-014.00".to_string() },
        ],
        items: vec![
            LineItem { description: "Conveyance Consideration".to_string(), quantity: 1.0, unit_price: 1650000.00, tax_rate: None, discount: None },
        ],
        terms: Some("TO HAVE AND TO HOLD the said tract or parcel of land, with the appurtenances, estate, title, and interest thereto belonging to the Grantee forever.".to_string()),
        ..Default::default()
    };

    assert!(req_deed.is_recordable_instrument());
    assert!(req_deed.is_notary_required());
    let deed_markup = generate_typst_markup(&req_deed);
    assert!(deed_markup.contains("#recording-header-block("));
    assert!(deed_markup.contains("state: \"Tennessee\""));
    assert!(deed_markup.contains("county: \"Hamilton\""));
    assert!(deed_markup.contains("notary-required: true"));
    assert!(deed_markup.contains("T.C.A. § 8-16-301"));

    let (deed_pdf, _) = generate_pdf(&req_deed).expect("Failed to compile recordable PropertyDeed PDF");
    assert!(deed_pdf.starts_with(b"%PDF-"));
}

#[test]
fn test_50_state_plus_dc_and_municipal_statutory_lease_engine_all_4_layers() {
    use pressmark::engine::generate_typst_markup;
    use pressmark::state_laws::{all_municipal_sub_profiles, all_state_lease_profiles, get_state_lease_profile};

    let states = all_state_lease_profiles();
    assert_eq!(states.len(), 51, "Must include all 50 US states + District of Columbia");

    let munis = all_municipal_sub_profiles();
    assert_eq!(munis.len(), 6, "Must include all 6 priority municipal rent ordinances");

    // Verify all 51 jurisdictions produce valid Typst markup with their exact statutory citation
    for sp in states {
        let req = DocumentRequest {
            doc_type: DocumentType::ResidentialLease,
            state_code: Some(sp.code.to_string()),
            year_built: Some(1972),
            from: PartyDetails {
                name: "Vance Residential Holdings LLC".to_string(),
                address: Some(format!("100 Main St, {}", sp.code)),
                ..Default::default()
            },
            to: PartyDetails {
                name: "Alex Morgan".to_string(),
                address: Some(format!("200 Oak Ave, {}", sp.code)),
                ..Default::default()
            },
            items: vec![
                LineItem { description: "Monthly Base Rent".to_string(), quantity: 1.0, unit_price: 2200.0, tax_rate: None, discount: None },
                LineItem { description: "Security Deposit".to_string(), quantity: 1.0, unit_price: 2200.0, tax_rate: None, discount: None },
            ],
            ..Default::default()
        };

        let markup = generate_typst_markup(&req);
        assert!(
            markup.contains(sp.name),
            "State {} ({}) missing state name in markup",
            sp.code,
            sp.name
        );
        // Verify Layer 3 pre-1978 Lead-Based Paint disclosure triggered
        assert!(markup.contains("42 U.S.C. § 4852d"));
    }

    // Explicitly compile representative multi-layer states to PDF:
    // - Layer 1 Page-1 mandates: Virginia (VA) & Minnesota (MN)
    // - Layer 2 Conspicuous typography: Michigan (MI), Massachusetts (MA), New Jersey (NJ), South Carolina (SC), Texas (TX), Ohio (OH)
    // - Layer 4 Standalone pagebreak attachments: Florida (FL § 83.512), Wisconsin (WI ATCP 134.09), New York (NY RPL § 231-c)
    // - Priority States: Tennessee (TN), California (CA)
    let priority_codes = ["VA", "MN", "MI", "MA", "NJ", "SC", "TX", "OH", "FL", "WI", "NY", "TN", "CA"];
    for code in priority_codes {
        let sp = get_state_lease_profile(code).expect("State profile must exist");
        let req = DocumentRequest {
            doc_type: DocumentType::ResidentialLease,
            state_code: Some(code.to_string()),
            year_built: Some(1974),
            lease_package_mode: Some(true),
            from: PartyDetails {
                name: "Vance Residential Group".to_string(),
                address: Some(format!("100 Capitol Blvd, {}", code)),
                ..Default::default()
            },
            to: PartyDetails {
                name: "Jordan Taylor".to_string(),
                address: Some(format!("400 Elm St, {}", code)),
                ..Default::default()
            },
            items: vec![
                LineItem { description: "Monthly Base Rent".to_string(), quantity: 1.0, unit_price: 2400.0, tax_rate: None, discount: None },
                LineItem { description: "Security Deposit".to_string(), quantity: 1.0, unit_price: 2400.0, tax_rate: None, discount: None },
            ],
            ..Default::default()
        };

        let markup = generate_typst_markup(&req);
        if let Some(p1_mandate) = sp.page1_mandate {
            assert!(markup.contains(p1_mandate), "Layer 1 Page-1 mandate missing for {}", code);
        }
        if let Some(cc) = sp.conspicuous_clause {
            assert!(markup.contains(cc.title), "Layer 2 Conspicuous clause missing for {}", code);
            assert!(
                markup.contains(&format!("size: {}pt", cc.point_size)),
                "Layer 2 font size {}pt missing for {}",
                cc.point_size,
                code
            );
        }
        if let Some(att) = sp.standalone_attachment {
            assert!(markup.contains(att.title), "Layer 4 Standalone attachment missing for {}", code);
            assert!(markup.contains("#pagebreak()"), "Layer 4 #pagebreak() missing for {}", code);
        }

        let (pdf_bytes, _) = generate_pdf(&req).unwrap_or_else(|e| panic!("Failed to compile {} statutory lease PDF: {}", code, e));
        assert!(pdf_bytes.starts_with(b"%PDF-"));
    }

    // Verify all 6 municipal sub-profiles compile cleanly to PDF
    for mp in munis {
        let req = DocumentRequest {
            doc_type: DocumentType::ResidentialLease,
            state_code: Some(mp.state_code.to_string()),
            municipality: Some(mp.name.to_string()),
            from: PartyDetails { name: "Urban Core Properties LLC".to_string(), ..Default::default() },
            to: PartyDetails { name: "Casey Morgan".to_string(), ..Default::default() },
            items: vec![
                LineItem { description: "Monthly Base Rent".to_string(), quantity: 1.0, unit_price: 2800.0, tax_rate: None, discount: None },
            ],
            ..Default::default()
        };
        let markup = generate_typst_markup(&req);
        assert!(markup.contains(mp.name));
        let (pdf_bytes, _) = generate_pdf(&req).unwrap_or_else(|e| panic!("Failed municipal lease PDF for {}: {}", mp.name, e));
        assert!(pdf_bytes.starts_with(b"%PDF-"));
    }
}

#[test]
fn test_all_18_new_avail_stessa_biggerpockets_rocket_lawyer_archetypes() {
    let new_archetypes = [
        DocumentType::RoomRentalLease,
        DocumentType::LeaseRenewalAgreement,
        DocumentType::LeaseAmendment,
        DocumentType::RentalApplication,
        DocumentType::AdverseActionNotice,
        DocumentType::NoticeOfEntry,
        DocumentType::RentIncreaseNotice,
        DocumentType::MutualLeaseTermination,
        DocumentType::TenantRentLedger,
        DocumentType::ScheduleOfRealEstateOwned,
        DocumentType::ScheduleETaxWorksheet,
        DocumentType::RealEstateDealAnalysis,
        DocumentType::SubleaseAgreement,
        DocumentType::RentToOwnAgreement,
        DocumentType::PropertyManagementAgreement,
        DocumentType::RealEstatePurchaseAgreement,
        DocumentType::PropertyDeed,
        DocumentType::MechanicsLienWaiver,
    ];
    assert_eq!(new_archetypes.len(), 18);

    for dt in new_archetypes {
        let req = DocumentRequest {
            doc_type: dt.clone(),
            state_code: Some("TN".to_string()),
            county: Some("Hamilton".to_string()),
            from: PartyDetails {
                name: "Hallman Capital Properties LLC".to_string(),
                address: Some("1200 Market St, Suite 400, Chattanooga, TN 37402".to_string()),
                ..Default::default()
            },
            to: PartyDetails {
                name: "Counterparty Principal".to_string(),
                address: Some("412 Northshore Dr, Chattanooga, TN 37405".to_string()),
                ..Default::default()
            },
            custom_fields: vec![
                CustomField { label: "Property".to_string(), value: "412 Northshore Dr, Chattanooga, TN 37405".to_string() },
                CustomField { label: "DSCR".to_string(), value: "1.58x".to_string() },
            ],
            items: vec![
                LineItem { description: "Primary Line Item / Charge".to_string(), quantity: 1.0, unit_price: 2500.00, tax_rate: None, discount: None },
                LineItem { description: "Credit / Deduction Adjustment".to_string(), quantity: 1.0, unit_price: 0.00, tax_rate: None, discount: Some(250.00) },
            ],
            terms: Some("Executed under applicable state real property statutes.".to_string()),
            ..Default::default()
        };

        let (pdf_bytes, _) = generate_pdf(&req).unwrap_or_else(|e| panic!("Failed PDF for {:?}: {}", dt, e));
        assert!(pdf_bytes.starts_with(b"%PDF-"));
        let (png_bytes, _) = generate_preview_png(&req, 144).unwrap_or_else(|e| panic!("Failed PNG for {:?}: {}", dt, e));
        assert!(png_bytes.len() > 1000);
    }
}

#[tokio::test]
async fn test_programmatic_state_seo_landing_routes_and_states_api() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use pressmark::server::{create_router, AppState};
    use pressmark::storage::Storage;
    use std::sync::Arc;
    use tower::ServiceExt;

    let storage = Arc::new(Storage::new(":memory:").unwrap());
    let app_state = Arc::new(AppState::default().with_storage(storage));
    let router = create_router(app_state);

    // 1. Verify GET /api/v1/states returns all 51 states + 6 municipalities
    let req_states = Request::builder()
        .uri("/api/v1/states")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_states = router.clone().oneshot(req_states).await.unwrap();
    assert_eq!(res_states.status(), StatusCode::OK);
    let states_bytes = axum::body::to_bytes(res_states.into_body(), usize::MAX).await.unwrap();
    let states_json: serde_json::Value = serde_json::from_slice(&states_bytes).unwrap();
    assert_eq!(states_json["states_count"], 51);
    assert_eq!(states_json["municipalities_count"], 6);

    // 2. Verify programmatic state-prefixed SEO landing routes
    let seo_routes = [
        ("/templates/tennessee-residential-lease", "Tennessee", "T.C.A. § 66-28-101", "residential_lease", "TN"),
        ("/templates/california-residential-lease", "California", "Cal. Civ. Code § 1940", "residential_lease", "CA"),
        ("/templates/florida-eviction-notice", "Florida", "Fla. Stat. § 83.40", "notice_to_vacate", "FL"),
        ("/templates/texas-quitclaim-deed", "Texas", "Tex. Prop. Code Ch. 92", "property_deed", "TX"),
        ("/templates/new-york-room-rental-lease", "New York", "N.Y. Real Prop. Law § 220", "room_rental_lease", "NY"),
    ];

    for (uri, state_name, citation_prefix, expected_archetype, expected_state_code) in seo_routes {
        let req = Request::builder().uri(uri).method("GET").body(Body::empty()).unwrap();
        let res = router.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK, "Expected 200 OK for {}", uri);
        let html = String::from_utf8(axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(html.contains(state_name), "HTML for {} missing state name {}", uri, state_name);
        assert!(html.contains(citation_prefix), "HTML for {} missing citation {}", uri, citation_prefix);
        assert!(
            html.contains(&format!("window.__PRESSMARK_INITIAL_ARCHETYPE__ = \"{}\";", expected_archetype)),
            "HTML for {} missing archetype injection {}",
            uri,
            expected_archetype
        );
        assert!(
            html.contains(&format!("window.__PRESSMARK_INITIAL_STATE__ = \"{}\";", expected_state_code)),
            "HTML for {} missing state code injection {}",
            uri,
            expected_state_code
        );
    }

    // 3. Verify sub-variant SEO routes & West Virginia state-prefixed routes
    let subvariant_routes = [
        ("/templates/new-york-14-day-rent-demand", "New York", "notice_to_vacate", "NY"),
        ("/templates/california-3-day-notice-to-pay-or-quit", "California", "notice_to_vacate", "CA"),
        ("/templates/west-virginia-residential-lease", "West Virginia", "residential_lease", "WV"),
        ("/templates/pay-or-quit-notice", "Notice To Vacate", "notice_to_vacate", ""),
        ("/templates/reo-schedule", "Schedule Of Real Estate Owned", "schedule_of_real_estate_owned", ""),
    ];
    for (uri, expected_text, expected_archetype, expected_state_code) in subvariant_routes {
        let req = Request::builder().uri(uri).method("GET").body(Body::empty()).unwrap();
        let res = router.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK, "Expected 200 OK for {}", uri);
        let html = String::from_utf8(axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(html.contains(expected_text), "HTML for {} missing expected text {}", uri, expected_text);
        assert!(
            html.contains(&format!("window.__PRESSMARK_INITIAL_ARCHETYPE__ = \"{}\";", expected_archetype)),
            "HTML for {} missing archetype {}",
            uri,
            expected_archetype
        );
        if !expected_state_code.is_empty() {
            assert!(
                html.contains(&format!("window.__PRESSMARK_INITIAL_STATE__ = \"{}\";", expected_state_code)),
                "HTML for {} missing state code {}",
                uri,
                expected_state_code
            );
        }
    }

    // 4. Verify POST /api/v1/copilot resolves "West Virginia" as WV (not VA)
    let copilot_req = Request::builder()
        .uri("/api/v1/copilot")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"prompt":"Draft a West Virginia residential lease for 100 Capitol St, Charleston, WV 25301 at $1,650/mo"}"#))
        .unwrap();
    let copilot_res = router.clone().oneshot(copilot_req).await.unwrap();
    assert_eq!(copilot_res.status(), StatusCode::OK);
    let copilot_bytes = axum::body::to_bytes(copilot_res.into_body(), usize::MAX).await.unwrap();
    let copilot_json: serde_json::Value = serde_json::from_slice(&copilot_bytes).unwrap();
    assert_eq!(copilot_json["modified_state"]["state_code"], "WV");
    assert_eq!(copilot_json["modified_state"]["doc_type"], "residential_lease");

    // 5. Verify GET /sitemap.xml includes all 57 archetypes and all 51 state+DC residential lease routes
    let sitemap_req = Request::builder()
        .uri("/sitemap.xml")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let sitemap_res = router.clone().oneshot(sitemap_req).await.unwrap();
    assert_eq!(sitemap_res.status(), StatusCode::OK);
    let sitemap_xml = String::from_utf8(axum::body::to_bytes(sitemap_res.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
    for dt in DocumentType::all() {
        let expected_loc = format!("<loc>https://pressmark.dev/templates/{}</loc>", dt.url_slug());
        assert!(sitemap_xml.contains(&expected_loc), "Sitemap missing archetype URL {}", expected_loc);
    }
    for sp in pressmark::state_laws::all_state_lease_profiles() {
        let expected_state_loc = format!("<loc>https://pressmark.dev/templates/{}-residential-lease</loc>", sp.slug);
        assert!(sitemap_xml.contains(&expected_state_loc), "Sitemap missing state URL {}", expected_state_loc);
    }
}

#[test]
fn test_infer_state_from_address_directional_and_street_name_traps() {
    use pressmark::state_laws::infer_state_from_address;

    assert_eq!(infer_state_from_address("123 NE 45th St, Seattle, WA 98105").map(|p| p.code), Some("WA"));
    assert_eq!(infer_state_from_address("8500 LA Cienega Blvd, Los Angeles, CA 90048").map(|p| p.code), Some("CA"));
    assert_eq!(infer_state_from_address("500 CO Rd 12, Austin, TX 78701").map(|p| p.code), Some("TX"));
    assert_eq!(infer_state_from_address("100 MT Vernon Ave, Alexandria, VA 22301").map(|p| p.code), Some("VA"));
    assert_eq!(infer_state_from_address("300 DE Kalb Ave, Brooklyn, NY 11205").map(|p| p.code), Some("NY"));
    assert_eq!(infer_state_from_address("742 IN St, Denver, CO 80202").map(|p| p.code), Some("CO"));
    assert_eq!(infer_state_from_address("100 Capitol St, Charleston, West Virginia 25301").map(|p| p.code), Some("WV"));
    assert_eq!(infer_state_from_address("200 Main St, Richmond, Virginia 23219").map(|p| p.code), Some("VA"));
}

#[test]
fn test_michigan_uppercase_conspicuous_notice_and_kpi_deduplication_and_combined_edge_cases() {
    use pressmark::engine::generate_typst_markup;

    // 1. Michigan MCL 554.634 requires uppercase conspicuous Truth in Renting notice
    let mi_req = DocumentRequest {
        doc_type: DocumentType::ResidentialLease,
        state_code: Some("MI".to_string()),
        from: PartyDetails { name: "Great Lakes Management LLC".to_string(), ..Default::default() },
        to: PartyDetails { name: "Jordan Taylor".to_string(), ..Default::default() },
        items: vec![LineItem { description: "Monthly Rent".to_string(), quantity: 1.0, unit_price: 1800.0, tax_rate: None, discount: None }],
        ..Default::default()
    };
    let mi_markup = generate_typst_markup(&mi_req);
    assert!(
        mi_markup.contains("NOTICE: MICHIGAN LAW ESTABLISHES RIGHTS AND OBLIGATIONS FOR PARTIES TO RENTAL AGREEMENTS"),
        "Expected Michigan conspicuous clause to be rendered in uppercase"
    );

    // 2. Verify KPI fields [0..4] in RealEstateDealAnalysis / ScheduleOfRealEstateOwned / ScheduleETaxWorksheet are not duplicated
    let deal_req = DocumentRequest {
        doc_type: DocumentType::RealEstateDealAnalysis,
        from: PartyDetails { name: "Apex Capital".to_string(), ..Default::default() },
        to: PartyDetails { name: "Investment Committee".to_string(), ..Default::default() },
        custom_fields: vec![
            CustomField { label: "Purchase Price".to_string(), value: "$425,000".to_string() },
            CustomField { label: "Cap Rate".to_string(), value: "7.85%".to_string() },
            CustomField { label: "Cash-on-Cash Return".to_string(), value: "12.40%".to_string() },
            CustomField { label: "DSCR".to_string(), value: "1.42x".to_string() },
            CustomField { label: "Exit Horizon".to_string(), value: "5-Year Refi".to_string() },
        ],
        items: vec![LineItem { description: "Gross Scheduled Rent".to_string(), quantity: 12.0, unit_price: 4200.0, tax_rate: None, discount: None }],
        ..Default::default()
    };
    let deal_markup = generate_typst_markup(&deal_req);
    assert_eq!(
        deal_markup.matches("$425,000").count(),
        1,
        "First 4 KPI custom_fields should appear once in #deal-kpi-grid and not duplicate in card-box"
    );
    assert_eq!(deal_markup.matches("EXIT HORIZON").count(), 1);
    assert_eq!(deal_markup.matches("5-Year Refi").count(), 1);

    // 3. Verify 9pt contractual legibility on AgreementSow, RetainerAgreement, BillOfSale, RentRoll, ClosingSettlementStatement
    for dt in [
        DocumentType::AgreementSow,
        DocumentType::RetainerAgreement,
        DocumentType::BillOfSale,
        DocumentType::RentRoll,
        DocumentType::ClosingSettlementStatement,
        DocumentType::PropertyManagementStatement,
        DocumentType::PropertyInspectionChecklist,
    ] {
        let req = DocumentRequest {
            doc_type: dt.clone(),
            from: PartyDetails { name: "Issuer".to_string(), ..Default::default() },
            to: PartyDetails { name: "Counterparty".to_string(), ..Default::default() },
            terms: Some("Binding operative governance clause.".to_string()),
            ..Default::default()
        };
        let markup = generate_typst_markup(&req);
        assert_eq!(
            markup.matches("Binding operative governance clause.").count(),
            1,
            "Expected req.terms to appear once (not duplicated in footer) for {:?}",
            dt
        );
        assert!(
            !markup.contains("#text(size: 7.5pt, fill: rgb(color-muted))[Binding operative governance clause.]"),
            "Expected {:?} terms not to render in 7.5pt muted invoice footer",
            dt
        );
    }

    // 4. Combined edge case: recordable: true + lease_package_mode: true + multi-page sections on FL, WI, NY
    for state in ["FL", "WI", "NY"] {
        let combo_req = DocumentRequest {
            doc_type: DocumentType::ResidentialLease,
            state_code: Some(state.to_string()),
            county: Some("Test County".to_string()),
            recordable: Some(true),
            notary_required: Some(true),
            lease_package_mode: Some(true),
            custom_fields: vec![
                CustomField { label: "Parcel ID / APN".to_string(), value: "APN-991-002-44".to_string() },
            ],
            from: PartyDetails {
                name: "Institutional Landlord Holdings LLC".to_string(),
                address: Some("100 Brickell Ave, Suite 900, Miami, FL 33131".to_string()),
                ..Default::default()
            },
            to: PartyDetails {
                name: "Primary & Co-Tenant Lessees".to_string(),
                address: Some("500 Ocean Dr, Unit 12A, Miami, FL 33139".to_string()),
                ..Default::default()
            },
            items: vec![
                LineItem { description: "Monthly Base Rent".to_string(), quantity: 1.0, unit_price: 3400.0, tax_rate: None, discount: None },
                LineItem { description: "Security Deposit".to_string(), quantity: 1.0, unit_price: 3400.0, tax_rate: None, discount: None },
            ],
            sections: (1..=12)
                .map(|i| DocumentSection {
                    title: format!("Article {}. Extended Covenant & Statutory Undertaking", i),
                    content: "Landlord and Tenant mutually covenant and agree to abide by all state, county, and municipal habitability, security deposit escrow, and environmental disclosure laws throughout the initial lease term and any renewal thereof.".repeat(3),
                })
                .collect(),
            ..Default::default()
        };
        let (pdf_bytes, _) = generate_pdf(&combo_req).unwrap_or_else(|e| panic!("Failed combined recordable + lease_package_mode PDF for {}: {}", state, e));
        assert!(pdf_bytes.starts_with(b"%PDF-"));
        assert!(pdf_bytes.len() > 5000);
    }
}

#[tokio::test]
async fn test_pro_qa_all_defect_fixes_and_visual_verification() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use pressmark::engine::generate_typst_markup;
    use pressmark::models::SignatureBlock;
    use pressmark::server::{create_router, AppState};
    use pressmark::state_laws::get_state_lease_profile;
    use pressmark::storage::Storage;
    use std::sync::Arc;
    use tower::ServiceExt;

    // 1. Defect Group 1: Payment QR Gating on Non-Payable Archetypes
    let non_payable = [
        DocumentType::CreditNote,
        DocumentType::Receipt,
        DocumentType::PropertyDeed,
        DocumentType::RealEstatePurchaseAgreement,
        DocumentType::ScheduleETaxWorksheet,
        DocumentType::ScheduleOfRealEstateOwned,
        DocumentType::RealEstateDealAnalysis,
        DocumentType::SecurityDepositDisposition,
        DocumentType::MutualLeaseTermination,
        DocumentType::PropertyInspectionChecklist,
        DocumentType::RentalApplication,
        DocumentType::AdverseActionNotice,
        DocumentType::NoticeOfEntry,
        DocumentType::RentIncreaseNotice,
        DocumentType::TenantEstoppel,
        DocumentType::LetterOfIntentCre,
        DocumentType::MechanicsLienWaiver,
        DocumentType::Nda,
        DocumentType::Certificate,
        DocumentType::CertificateCompletion,
        DocumentType::CertificateInsurance,
    ];
    for dt in non_payable {
        assert!(!dt.allows_payment_qr(), "{:?} must not allow payment QR", dt);
        let req = DocumentRequest {
            doc_type: dt.clone(),
            payment_qr_code: Some(true),
            payment_link: Some("https://buy.stripe.com/test_leak".to_string()),
            from: PartyDetails { name: "Issuer".to_string(), ..Default::default() },
            to: PartyDetails { name: "Recipient".to_string(), ..Default::default() },
            items: vec![LineItem {
                description: "Item".to_string(),
                quantity: 1.0,
                unit_price: 100.0,
                tax_rate: None,
                discount: None,
            }],
            ..Default::default()
        };
        let markup = generate_typst_markup(&req);
        assert!(
            !markup.contains("#payment-qr-block("),
            "Non-payable archetype {:?} leaked #payment-qr-block",
            dt
        );
    }

    // Verify payable archetypes DO emit #payment-qr-block when requested
    for dt in [
        DocumentType::Invoice,
        DocumentType::NoticeToVacate,
        DocumentType::TenantRentLedger,
        DocumentType::RetainerAgreement,
    ] {
        assert!(dt.allows_payment_qr(), "{:?} should allow payment QR", dt);
        let req = DocumentRequest {
            doc_type: dt.clone(),
            payment_qr_code: Some(true),
            payment_link: Some("https://buy.stripe.com/test_valid".to_string()),
            from: PartyDetails { name: "Issuer".to_string(), ..Default::default() },
            to: PartyDetails { name: "Recipient".to_string(), ..Default::default() },
            items: vec![LineItem {
                description: "Balance Due".to_string(),
                quantity: 1.0,
                unit_price: 500.0,
                tax_rate: None,
                discount: None,
            }],
            ..Default::default()
        };
        let markup = generate_typst_markup(&req);
        assert!(
            markup.contains("#payment-qr-block("),
            "Payable archetype {:?} failed to emit #payment-qr-block",
            dt
        );
    }

    // 2. Defect Group 2: Layer 1 (VA/MN) & Layer 4 (FL/WI/NY) Statutory Scoping
    let non_lease_notices = [
        DocumentType::NoticeOfEntry,
        DocumentType::SecurityDepositDisposition,
        DocumentType::MutualLeaseTermination,
        DocumentType::LeaseAmendment,
    ];
    for dt in non_lease_notices {
        assert!(!dt.is_lease_agreement());
        for state in ["VA", "MN", "FL", "WI", "NY"] {
            let req = DocumentRequest {
                doc_type: dt.clone(),
                state_code: Some(state.to_string()),
                lease_package_mode: Some(true),
                from: PartyDetails { name: "Landlord".to_string(), ..Default::default() },
                to: PartyDetails { name: "Tenant".to_string(), ..Default::default() },
                ..Default::default()
            };
            let markup = generate_typst_markup(&req);
            assert!(
                !markup.contains("VIRGINIA STATUTORY PAGE-1 LEASE SUMMARY"),
                "VA Page-1 Lease Summary leaked onto {:?}",
                dt
            );
            assert!(
                !markup.contains("MINNESOTA STATUTORY PAGE-1 LEASE SUMMARY"),
                "MN Page-1 Lease Summary leaked onto {:?}",
                dt
            );
            assert!(
                !markup.contains("FLORIDA STATUTORY FLOOD DISCLOSURE"),
                "FL standalone lease flood addendum leaked onto {:?}",
                dt
            );
            assert!(
                !markup.contains("WISCONSIN STATUTORY NONSTANDARD RENTAL PROVISIONS"),
                "WI standalone lease addendum leaked onto {:?}",
                dt
            );
            assert!(
                !markup.contains("NEW YORK STATUTORY GOOD CAUSE EVICTION LAW NOTICE"),
                "NY RPL 231-c rider leaked onto {:?}",
                dt
            );
            assert!(
                !markup.contains("SCHEDULE A: JOINT MOVE-IN / MOVE-OUT CONDITION CHECKLIST"),
                "Move-In checklist leaked onto {:?}",
                dt
            );
        }
    }

    // NY RPL § 231-c SHOULD attach on RentIncreaseNotice and NoticeToVacate, while FL/WI/VA Page-1 must NOT
    for dt in [DocumentType::RentIncreaseNotice, DocumentType::NoticeToVacate] {
        let ny_req = DocumentRequest {
            doc_type: dt.clone(),
            state_code: Some("NY".to_string()),
            from: PartyDetails { name: "Landlord".to_string(), ..Default::default() },
            to: PartyDetails { name: "Tenant".to_string(), ..Default::default() },
            items: vec![LineItem { description: "Rent".to_string(), quantity: 1.0, unit_price: 2400.0, tax_rate: None, discount: None }],
            ..Default::default()
        };
        let ny_markup = generate_typst_markup(&ny_req);
        assert!(
            ny_markup.contains("NEW YORK STATUTORY GOOD CAUSE EVICTION LAW NOTICE"),
            "NY RPL 231-c rider should attach to {:?}",
            dt
        );

        let fl_req = DocumentRequest {
            doc_type: dt.clone(),
            state_code: Some("FL".to_string()),
            from: PartyDetails { name: "Landlord".to_string(), ..Default::default() },
            to: PartyDetails { name: "Tenant".to_string(), ..Default::default() },
            items: vec![LineItem { description: "Rent".to_string(), quantity: 1.0, unit_price: 2400.0, tax_rate: None, discount: None }],
            ..Default::default()
        };
        let fl_markup = generate_typst_markup(&fl_req);
        assert!(
            !fl_markup.contains("FLORIDA STATUTORY FLOOD DISCLOSURE"),
            "FL flood addendum must not attach to {:?}",
            dt
        );
    }

    // 3. Defect Group 4: RentIncreaseNotice Prior+New Rent vs Prior+Delta, Deal Analysis DSCR/Purchase Price, Schedule E Label
    let rent_inc_prior_new = DocumentRequest {
        doc_type: DocumentType::RentIncreaseNotice,
        from: PartyDetails { name: "Vance Residential Holdings LLC".to_string(), ..Default::default() },
        to: PartyDetails { name: "Marcus Sterling".to_string(), ..Default::default() },
        items: vec![
            LineItem { description: "Prior Monthly Base Rent".to_string(), quantity: 1.0, unit_price: 2300.0, tax_rate: None, discount: None },
            LineItem { description: "New Monthly Base Rent (Effective Jan 1, 2027)".to_string(), quantity: 1.0, unit_price: 2450.0, tax_rate: None, discount: None },
        ],
        ..Default::default()
    };
    assert!((rent_inc_prior_new.subtotal() - 2450.0).abs() < 0.005, "Expected $2,450.00 new rent, got {}", rent_inc_prior_new.subtotal());
    assert!((rent_inc_prior_new.grand_total() - 2450.0).abs() < 0.005);
    let rent_inc_markup = generate_typst_markup(&rent_inc_prior_new);
    assert!(rent_inc_markup.contains("Prior Monthly Rent:"));
    assert!(rent_inc_markup.contains("Net Monthly Increase:"));
    assert!(rent_inc_markup.contains("2,450.00"));
    assert!(!rent_inc_markup.contains("4,750.00"));

    // 3-row RentIncreaseNotice (Prior Rent + New Rent + Parking Fee) must not sum Prior Rent + New Rent
    let rent_inc_3row = DocumentRequest {
        doc_type: DocumentType::RentIncreaseNotice,
        from: PartyDetails { name: "Vance Residential Holdings LLC".to_string(), ..Default::default() },
        to: PartyDetails { name: "Marcus Sterling".to_string(), ..Default::default() },
        items: vec![
            LineItem { description: "Prior Monthly Base Rent".to_string(), quantity: 1.0, unit_price: 2300.0, tax_rate: None, discount: None },
            LineItem { description: "New Monthly Base Rent (Effective Jan 1, 2027)".to_string(), quantity: 1.0, unit_price: 2450.0, tax_rate: None, discount: None },
            LineItem { description: "Assigned Covered Parking Fee".to_string(), quantity: 1.0, unit_price: 50.0, tax_rate: None, discount: None },
        ],
        ..Default::default()
    };
    assert!((rent_inc_3row.subtotal() - 2500.0).abs() < 0.005, "Expected $2,500.00 total new rent, got {}", rent_inc_3row.subtotal());
    let rent_inc_3row_markup = generate_typst_markup(&rent_inc_3row);
    assert!(rent_inc_3row_markup.contains("(+8.7\\%)"), "Expected percentage increase (+8.7%) in markup");
    assert!(!rent_inc_3row_markup.contains("4,800.00"));

    // Deal Analysis with Purchase Price + Down Payment + Closing Costs + Rent + Expenses + DSCR + IRR + GRM
    let deal_mixed = DocumentRequest {
        doc_type: DocumentType::RealEstateDealAnalysis,
        from: PartyDetails { name: "Hallman Capital Properties LLC".to_string(), ..Default::default() },
        to: PartyDetails { name: "Investment Committee".to_string(), ..Default::default() },
        items: vec![
            LineItem { description: "Purchase Price".to_string(), quantity: 1.0, unit_price: 850000.0, tax_rate: None, discount: None },
            LineItem { description: "Down Payment (25% Equity)".to_string(), quantity: 1.0, unit_price: 212500.0, tax_rate: None, discount: None },
            LineItem { description: "Closing Costs & Title Escrow".to_string(), quantity: 1.0, unit_price: 18500.0, tax_rate: None, discount: None },
            LineItem { description: "Gross Scheduled Rent".to_string(), quantity: 1.0, unit_price: 96000.0, tax_rate: None, discount: None },
            LineItem { description: "Operating Expenses".to_string(), quantity: 1.0, unit_price: -34000.0, tax_rate: None, discount: None },
            LineItem { description: "Debt Service Coverage Ratio (DSCR)".to_string(), quantity: 1.0, unit_price: 1.48, tax_rate: None, discount: None },
            LineItem { description: "Gross Rent Multiplier (GRM)".to_string(), quantity: 1.0, unit_price: 8.85, tax_rate: None, discount: None },
            LineItem { description: "5-Year Projected IRR".to_string(), quantity: 1.0, unit_price: 17.4, tax_rate: None, discount: None },
        ],
        ..Default::default()
    };
    // Subtotal must only sum operating cash flow items (96,000 - 34,000 = 62,000), excluding capital and ratio metrics
    assert!(
        (deal_mixed.subtotal() - 62000.0).abs() < 0.005,
        "Expected $62,000.00 NOI, got {}",
        deal_mixed.subtotal()
    );
    let deal_markup = generate_typst_markup(&deal_mixed);
    assert!(deal_markup.contains("1.48x"), "DSCR ratio item must format as 1.48x, not $1.48");
    assert!(deal_markup.contains("8.85x"), "GRM ratio item must format as 8.85x, not $8.85");
    assert!(deal_markup.contains("17.40\\%"), "IRR ratio item must format as 17.40%, not $17.40");
    assert!(deal_markup.contains("Projected Annual Cash Flow / NOI:"));

    // Schedule E Tax Worksheet label check
    let sched_e_req = DocumentRequest {
        doc_type: DocumentType::ScheduleETaxWorksheet,
        from: PartyDetails { name: "Hallman Capital Properties LLC".to_string(), ..Default::default() },
        to: PartyDetails { name: "Vance & Associates CPAs LLP".to_string(), ..Default::default() },
        items: vec![
            LineItem { description: "Line 3 · Gross Rents Received".to_string(), quantity: 1.0, unit_price: 148200.0, tax_rate: None, discount: None },
            LineItem { description: "Line 12 · Mortgage Interest Paid".to_string(), quantity: 1.0, unit_price: -21400.0, tax_rate: None, discount: None },
            LineItem { description: "Line 18 · MACRS Depreciation".to_string(), quantity: 1.0, unit_price: -38900.0, tax_rate: None, discount: None },
        ],
        ..Default::default()
    };
    let sched_e_markup = generate_typst_markup(&sched_e_req);
    assert!(sched_e_markup.contains("Net Rental Income / (Loss) (Schedule E):"));
    assert!(!sched_e_markup.contains("Net Cash Remittance to Owner:"));

    // Empty items on NoticeOfEntry must suppress the totals box
    let notice_entry_req = DocumentRequest {
        doc_type: DocumentType::NoticeOfEntry,
        state_code: Some("TX".to_string()),
        custom_fields: vec![
            CustomField { label: "Service Method".to_string(), value: "Certified Mail".to_string() },
        ],
        from: PartyDetails { name: "Vance Property Management LLC".to_string(), ..Default::default() },
        to: PartyDetails { name: "Marcus Sterling".to_string(), ..Default::default() },
        items: vec![],
        ..Default::default()
    };
    let notice_entry_markup = generate_typst_markup(&notice_entry_req);
    assert!(!notice_entry_markup.contains("Total Due:"), "NoticeOfEntry with empty items must not render Total Due box");
    assert!(
        notice_entry_markup.contains("service-method: \"mail\""),
        "NoticeOfEntry must pass service-method from CustomField to #certificate-of-service"
    );

    // 4. Defect Group 5: PropertyDeed Prepared By / Return To, Single Jurat Signer Deduplication, Michigan 14pt
    let deed_req = DocumentRequest {
        doc_type: DocumentType::PropertyDeed,
        state_code: Some("TN".to_string()),
        county: Some("Hamilton".to_string()),
        recordable: Some(true),
        notary_required: Some(true),
        from: PartyDetails {
            name: "Highland Crest Holdings LLC".to_string(),
            address: Some("412 Northshore Dr, Chattanooga, TN 37405".to_string()),
            ..Default::default()
        },
        to: PartyDetails {
            name: "Hallman Capital Properties LLC".to_string(),
            address: Some("1200 Market St, Suite 400, Chattanooga, TN 37402".to_string()),
            ..Default::default()
        },
        custom_fields: vec![
            CustomField {
                label: "Prepared By".to_string(),
                value: "Chambliss, Bahner & Stophel, P.C., 605 Chestnut St, Chattanooga, TN 37450".to_string(),
            },
            CustomField {
                label: "Return To".to_string(),
                value: "Pioneer Title & Escrow, 701 Market St, Chattanooga, TN 37402".to_string(),
            },
            CustomField {
                label: "APN".to_string(),
                value: "135K-A-014.00".to_string(),
            },
        ],
        items: vec![
            LineItem { description: "Consideration".to_string(), quantity: 1.0, unit_price: 1650000.0, tax_rate: None, discount: None },
        ],
        signatures: vec![
            SignatureBlock {
                signer_name: "Arthur Highland".to_string(),
                signer_title: Some("Managing Member, Highland Crest Holdings LLC".to_string()),
                date: Some("2026-10-25".to_string()),
                signature_data: None,
            },
        ],
        ..Default::default()
    };
    let deed_markup = generate_typst_markup(&deed_req);
    assert!(deed_markup.contains("prepared-by: \"Chambliss, Bahner & Stophel, P.C., 605 Chestnut St, Chattanooga, TN 37450\""));
    assert!(deed_markup.contains("return-to: \"Pioneer Title & Escrow, 701 Market St, Chattanooga, TN 37402\""));
    assert!(deed_markup.contains("#jurat-box("));
    assert!(
        !deed_markup.contains("#signature-grid("),
        "Single-signer notarized instrument must not duplicate signer in both #jurat-box and #signature-grid"
    );

    let mi_profile = get_state_lease_profile("MI").expect("MI profile must exist");
    let mi_cc = mi_profile.conspicuous_clause.expect("MI conspicuous clause must exist");
    assert_eq!(mi_cc.point_size, 14, "MCL 554.603 requires >= 4pt larger than 10pt body text (14pt)");

    // 5. Defect Group 6: Real SHA-256 Audit Hash & API "success": true
    let hash1 = deed_req.compute_audit_sha256();
    let hash2 = sched_e_req.compute_audit_sha256();
    assert_eq!(hash1.len(), 64);
    assert!(hash1.chars().all(|c| c.is_ascii_hexdigit()));
    assert_ne!(hash1, hash2, "Different documents must produce different SHA-256 hashes");
    assert!(!hash1.starts_with("7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1f"));
    assert!(!deed_markup.contains("\n#audit-slug("), "Documents must not add an internal audit footer by default");

    // Verify /api/v1/templates and /api/v1/states include "success": true
    let storage = Arc::new(Storage::new(":memory:").unwrap());
    let app_state = Arc::new(AppState::default().with_storage(storage));
    let router = create_router(app_state);

    for uri in ["/api/v1/templates", "/api/v1/states"] {
        let req = Request::builder().uri(uri).method("GET").body(Body::empty()).unwrap();
        let res = router.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["success"], true, "Endpoint {} must return \"success\": true", uri);
    }

    // 6. Defect Group 3 & 1: Verify static/js/studio.js fixes
    let studio_js = std::fs::read_to_string("static/js/studio.js").expect("Failed to read static/js/studio.js");
    let state_idx = studio_js.find("const requestedState =").expect("requestedState missing");
    let tmpl_idx = studio_js.find("const requestedTemplate =").expect("requestedTemplate missing");
    assert!(state_idx < tmpl_idx, "requestedState must be assigned before requestedTemplate in init()");
    assert!(studio_js.contains("} else if (type === 'agreement') {"));
    assert!(studio_js.contains("} else if (type === 'agreement_sow') {"));
    assert!(studio_js.contains("} else if (type === 'contractor_agreement') {"));
    assert!(studio_js.contains("} else if (type === 'retainer_agreement') {"));
    assert!(studio_js.contains("} else if (type === 'nda') {"));
    assert!(!studio_js.contains("} else if (type === 'nda' || type.includes('agreement')) {"));
    assert!(studio_js.contains("state.amount_paid = null;"));
    assert!(studio_js.contains("state.notes = '';"));
    assert!(studio_js.contains("state.terms = '';"));
    assert!(studio_js.contains("function computeSubtotalUi()"));
    assert!(!studio_js.contains("company: 'Portfolio Owner'"));
    assert!(!studio_js.contains("company: 'Building Facilities Team'"));
    assert_eq!(
        studio_js.matches("document.getElementById('btnZoomFit')").count(),
        1,
        "btnZoomFit listener must be registered exactly once"
    );

    // Compile visual QA previews for key fixed documents
    for (name, req) in [
        ("deed", &deed_req),
        ("sched_e", &sched_e_req),
        ("rent_inc", &rent_inc_prior_new),
        ("rent_inc_3row", &rent_inc_3row),
        ("deal", &deal_mixed),
        ("notice_entry", &notice_entry_req),
    ] {
        let (pdf, _) = generate_pdf(req).unwrap_or_else(|e| panic!("PDF failed for {}: {}", name, e));
        assert!(pdf.starts_with(b"%PDF-"));
        let (png, _) = generate_preview_png(req, 144).unwrap_or_else(|e| panic!("PNG failed for {}: {}", name, e));
        assert!(png.len() > 5000);
    }
}



