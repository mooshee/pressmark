//! Pressmark Archetype Body, Statutory Disclosure & Signature Renderers
//!
//! Author: Daniel Hallman

use super::{
    escape_typst_string_literal, escape_typst_text, format_party_typst, open_styled_table,
    render_totals_box, stage_image_asset,
};
use crate::models::{DocumentRequest, DocumentType};

pub(super) fn render_parties_disclosures_and_body(
    mut markup: &mut String,
    req: &DocumentRequest,
    primary_hex: &str,
    secondary_hex: &str,
    accent_hex: &str,
    surface_hex: &str,
    company_name: &str,
) {
    // 5. Parties Block
    let from_title = match req.doc_type {
        DocumentType::Estimate => "ESTIMATE PREPARED BY",
        DocumentType::Receipt => "PAYMENT RECEIVED BY",
        DocumentType::PurchaseOrder => "PURCHASER / BUYER",
        DocumentType::Agreement | DocumentType::Nda => "DISCLOSING / FIRST PARTY",
        DocumentType::Certificate => "ISSUING AUTHORITY",
        DocumentType::ResidentialLease
        | DocumentType::RoomRentalLease
        | DocumentType::LeaseRenewalAgreement
        | DocumentType::LeaseAmendment
        | DocumentType::CommercialLease
        | DocumentType::CommercialNnnLease
        | DocumentType::NoticeToVacate
        | DocumentType::NoticeOfEntry
        | DocumentType::RentIncreaseNotice
        | DocumentType::MutualLeaseTermination
        | DocumentType::SecurityDepositDisposition
        | DocumentType::TenantRentLedger => "LANDLORD / LESSOR",
        DocumentType::SubleaseAgreement => "SUBLESSOR (PRIME TENANT)",
        DocumentType::RentToOwnAgreement => "LANDLORD / OPTIONOR (SELLER)",
        DocumentType::RentalApplication | DocumentType::AdverseActionNotice => "LANDLORD / SCREENING AGENT",
        DocumentType::PropertyManagementStatement | DocumentType::RentRoll => "MANAGING AGENT / BROKERAGE",
        DocumentType::PropertyManagementAgreement => "PROPERTY OWNER / PRINCIPAL",
        DocumentType::ScheduleOfRealEstateOwned
        | DocumentType::ScheduleETaxWorksheet
        | DocumentType::RealEstateDealAnalysis => "PORTFOLIO OWNER / SPONSOR",
        DocumentType::RealEstatePurchaseAgreement => "SELLER / ASSIGNOR",
        DocumentType::PropertyDeed => "GRANTOR (CONVEYING OWNER)",
        DocumentType::MechanicsLienWaiver => "CLAIMANT / CONTRACTOR",
        DocumentType::ClosingSettlementStatement => "SETTLEMENT / ESCROW AGENT",
        DocumentType::RealEstateCommission => "DISBURSING BROKERAGE",
        DocumentType::PropertyInspectionChecklist => "INSPECTED BY (LANDLORD / AGENT)",
        DocumentType::LetterOfIntentCre => "PROSPECTIVE LESSEE / BUYER",
        DocumentType::TenantEstoppel => "TENANT (CERTIFYING LESSEE)",
        _ => "ISSUED BY",
    };
    let to_title = match req.doc_type {
        DocumentType::Estimate => "ESTIMATE PREPARED FOR",
        DocumentType::Receipt => "PAYMENT RECEIVED FROM",
        DocumentType::PurchaseOrder => "VENDOR / SUPPLIER",
        DocumentType::Agreement | DocumentType::Nda => "RECEIVING / SECOND PARTY",
        DocumentType::Certificate => "AWARDED / RECOGNIZED TO",
        DocumentType::ResidentialLease
        | DocumentType::RoomRentalLease
        | DocumentType::LeaseRenewalAgreement
        | DocumentType::LeaseAmendment
        | DocumentType::CommercialLease
        | DocumentType::CommercialNnnLease
        | DocumentType::NoticeToVacate
        | DocumentType::NoticeOfEntry
        | DocumentType::RentIncreaseNotice
        | DocumentType::MutualLeaseTermination
        | DocumentType::SecurityDepositDisposition
        | DocumentType::TenantRentLedger => "TENANT / LESSEE",
        DocumentType::SubleaseAgreement => "SUBLESSEE (SUBTENANT)",
        DocumentType::RentToOwnAgreement => "TENANT / OPTIONEE (BUYER)",
        DocumentType::RentalApplication | DocumentType::AdverseActionNotice => "APPLICANT / PROSPECTIVE TENANT",
        DocumentType::PropertyManagementStatement | DocumentType::RentRoll => "PROPERTY OWNER / CLIENT",
        DocumentType::PropertyManagementAgreement => "MANAGING BROKER / AGENT",
        DocumentType::ScheduleOfRealEstateOwned
        | DocumentType::ScheduleETaxWorksheet
        | DocumentType::RealEstateDealAnalysis => "LENDER / UNDERWRITING & CPA",
        DocumentType::RealEstatePurchaseAgreement => "BUYER / PURCHASER",
        DocumentType::PropertyDeed => "GRANTEE (VESTING RECIPIENT)",
        DocumentType::MechanicsLienWaiver => "PROPERTY OWNER / GENERAL CONTRACTOR",
        DocumentType::ClosingSettlementStatement => "BUYER / BORROWER & SELLER",
        DocumentType::RealEstateCommission => "AGENT / CO-BROKER",
        DocumentType::PropertyInspectionChecklist => "RESIDENT / TENANT",
        DocumentType::LetterOfIntentCre => "LANDLORD / LESSOR",
        DocumentType::TenantEstoppel => "LENDER / PURCHASER",
        _ => "BILLED TO",
    };

    let from_details_escaped = format_party_typst(&req.from);
    let to_details_escaped = format_party_typst(&req.to);
    let parties_style = req.effective_parties_style();
    let from_style = req.effective_from_style();
    let to_style = req.effective_to_style();

    markup.push_str(&format!(
        "#parties-grid(\n  from-name: \"{}\",\n  from-details: [{}],\n  to-name: \"{}\",\n  to-details: [{}],\n  from-title: \"{}\",\n  to-title: \"{}\",\n  parties-style: \"{}\",\n  from-style: \"{}\",\n  to-style: \"{}\",\n  primary-color: rgb(\"{}\"),\n  secondary-color: rgb(\"{}\"),\n  surface-color: rgb(\"{}\")\n)\n\n",
        escape_typst_string_literal(&req.from.name),
        from_details_escaped,
        escape_typst_string_literal(&req.to.name),
        to_details_escaped,
        escape_typst_string_literal(from_title),
        escape_typst_string_literal(to_title),
        escape_typst_string_literal(parties_style),
        escape_typst_string_literal(from_style),
        escape_typst_string_literal(to_style),
        primary_hex,
        secondary_hex,
        surface_hex
    ));

    // 5b. Layer 1: Page-1 State Statutory Mandates & Municipal Sub-Profile Box
    if req.doc_type.is_state_lease_instrument() {
        if let Some(state_profile) = req.effective_state_profile() {
            // Render Page-1 Statutory Mandates for VA (§ 55.1-1204.1) & MN (§ 504B.120) only on actual lease agreements
            if req.doc_type.is_lease_agreement() {
                if let Some(mandate_text) = state_profile.page1_mandate {
                    markup.push_str(&format!(
                        "#v(4pt)\n#rect(width: 100%, fill: rgb(\"#fef2f2\"), stroke: (left: 3.5pt + rgb(\"#dc2626\"), rest: 0.75pt + rgb(\"#fca5a5\")), radius: 3pt, inset: 8pt, [\n  #text(size: 8pt, weight: \"bold\", fill: rgb(\"#991b1b\"))[PAGE-1 STATUTORY MANDATE ({} · {})] \\\n  #v(2pt)\n  #text(size: 8pt, weight: \"medium\", fill: color-slate-900)[{}] \\\n  #v(3pt)\n  #text(size: 7.5pt, weight: \"bold\", fill: rgb(\"#991b1b\"))[Page-1 Itemized Total Commitment: {}]\n])\n#v(4pt)\n\n",
                        escape_typst_text(state_profile.name),
                        escape_typst_text(state_profile.governing_statute),
                        escape_typst_text(mandate_text),
                        req.format_amount_typst(req.grand_total())
                    ));
                }
            }

            let citation_str = format!(
                "{} STATUTORY FRAMEWORK · {} ({})",
                state_profile.name.to_uppercase(),
                state_profile.governing_statute,
                state_profile.framework
            );
            let burden_str = format!(
                "Deposit Cap: {} | Return: {} | Grace: {} | Late Fee Cap: {}",
                state_profile.deposit_cap,
                state_profile.return_deadline,
                state_profile.grace_period,
                state_profile.late_fee_cap
            );
            markup.push_str(&format!(
                "#v(2pt)\n#statutory-box(\n  citation: \"{}\",\n  burden: \"{}\",\n  [\n    #text(size: 7.5pt, fill: color-slate-700)[\n      *Governing Jurisdiction:* {} ({}) — {} \\\n      *Pay-or-Quit Notice:* {} #h(8pt) *Right of Entry Notice:* {} \\\n      *Rent Regulation:* {} \\\n      *Mandatory Statutory Disclosures:* {}\n    ]\n  ]\n)\n#v(4pt)\n\n",
                escape_typst_string_literal(&citation_str),
                escape_typst_string_literal(&burden_str),
                escape_typst_text(state_profile.name),
                escape_typst_text(state_profile.code),
                escape_typst_text(state_profile.governing_statute),
                escape_typst_text(state_profile.pay_or_quit_notice),
                escape_typst_text(state_profile.entry_notice),
                escape_typst_text(state_profile.rent_control_note),
                escape_typst_text(state_profile.key_disclosures)
            ));
        }

        if let Some(muni) = req.effective_municipal_profile() {
            let muni_cit = format!(
                "MUNICIPAL ORDINANCE OVERRIDE: {} ({})",
                muni.name.to_uppercase(),
                muni.ordinance_citation
            );
            let muni_burden = format!(
                "Deposit Cap: {} | Grace: {} | Late Fee Cap: {}",
                muni.deposit_cap, muni.grace_period, muni.late_fee_cap
            );
            markup.push_str(&format!(
                "#v(2pt)\n#statutory-box(\n  citation: \"{}\",\n  burden: \"{}\",\n  [\n    #text(size: 7.5pt, fill: color-slate-700)[\n      *Municipal Jurisdiction:* {} ({}) — {} \\\n      *Municipal Pay-or-Quit:* {} #h(8pt) *Municipal Entry Notice:* {} \\\n      *Required Municipal Attachments:* {}\n    ]\n  ]\n)\n#v(4pt)\n\n",
                escape_typst_string_literal(&muni_cit),
                escape_typst_string_literal(&muni_burden),
                escape_typst_text(muni.name),
                escape_typst_text(muni.state_code),
                escape_typst_text(muni.ordinance_citation),
                escape_typst_text(muni.pay_or_quit_notice),
                escape_typst_text(muni.entry_notice),
                escape_typst_text(muni.key_disclosures)
            ));
        }
    }

    // 6. Custom Fields Grid (if any; for SREO / Schedule E / Deal Analysis, the first 4 KPIs render inside #deal-kpi-grid)
    let cf_slice = if matches!(
        req.doc_type,
        DocumentType::ScheduleOfRealEstateOwned
            | DocumentType::ScheduleETaxWorksheet
            | DocumentType::RealEstateDealAnalysis
    ) {
        req.custom_fields.get(4..).unwrap_or(&[])
    } else {
        &req.custom_fields[..]
    };
    if !cf_slice.is_empty() {
        let fields_style = req.effective_fields_style();
        markup.push_str("#v(4pt)\n");
        match fields_style {
            "compact_strip" => {
                let col_count = cf_slice.len().clamp(1, 4);
                let cols_spec = std::iter::repeat("1fr").take(col_count).collect::<Vec<_>>().join(", ");
                markup.push_str(&format!(
                    "#rect(width: 100%, fill: rgb(\"{}\"), stroke: 0.5pt + color-slate-300, radius: 4pt, inset: (x: 10pt, y: 6.5pt))[\n  #grid(\n    columns: ({}),\n    gutter: 10pt,\n",
                    surface_hex, cols_spec
                ));
                for field in cf_slice {
                    markup.push_str(&format!(
                        "    [#stack(dir: ttb, spacing: 2.5pt, text(size: 6.5pt, weight: \"bold\", fill: rgb(\"{}\"))[{}], text(size: 8.5pt, weight: \"bold\", fill: color-slate-900)[{}])],\n",
                        primary_hex,
                        escape_typst_text(&field.label.to_uppercase()),
                        escape_typst_text(&field.value)
                    ));
                }
                markup.push_str("  )\n]\n#v(4pt)\n\n");
            }
            "ledger_table" => {
                markup.push_str(&format!(
                    "#table(\n  columns: (auto, 1fr, auto, 1fr),\n  stroke: 0.5pt + color-slate-300,\n  fill: (col, row) => if calc.even(col) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 7pt, y: 5pt),\n  align: (col, row) => left + horizon,\n",
                    surface_hex
                ));
                let len = cf_slice.len();
                let is_odd = len % 2 == 1;
                for (idx, field) in cf_slice.iter().enumerate() {
                    let lbl = format!(
                        "[#text(size: 7pt, font: font-mono, weight: \"bold\", fill: rgb(\"{}\"))[{}]]",
                        primary_hex,
                        escape_typst_text(&field.label.to_uppercase())
                    );
                    let val = format!(
                        "[#text(size: 8.5pt, weight: \"medium\", fill: color-slate-900)[{}]]",
                        escape_typst_text(&field.value)
                    );
                    if is_odd && idx + 1 == len {
                        markup.push_str(&format!("  {}, table.cell(colspan: 3, {}),\n", lbl, val));
                    } else {
                        markup.push_str(&format!("  {}, {},\n", lbl, val));
                    }
                }
                markup.push_str(")\n#v(4pt)\n\n");
            }
            "minimal_list" => {
                markup.push_str("#grid(\n  columns: (1fr, 1fr),\n  gutter: 12pt,\n");
                let len = cf_slice.len();
                let is_odd = len % 2 == 1;
                for (idx, field) in cf_slice.iter().enumerate() {
                    let cell_expr = format!(
                        "rect(width: 100%, fill: none, stroke: (bottom: 0.5pt + color-slate-200), inset: (x: 2pt, y: 4.5pt))[\n    #grid(\n      columns: (auto, 1fr),\n      gutter: 8pt,\n      align: (left + horizon, right + horizon),\n      text(size: 7pt, weight: \"bold\", fill: color-slate-500)[{}],\n      text(size: 8.5pt, weight: \"bold\", fill: color-slate-900)[{}]\n    )\n  ]",
                        escape_typst_text(&field.label.to_uppercase()),
                        escape_typst_text(&field.value)
                    );
                    if is_odd && idx + 1 == len {
                        markup.push_str(&format!("  grid.cell(colspan: 2, {}),\n", cell_expr));
                    } else {
                        markup.push_str(&format!("  {},\n", cell_expr));
                    }
                }
                markup.push_str(")\n#v(4pt)\n\n");
            }
            _ => {
                markup.push_str("#grid(\n  columns: (1fr, 1fr),\n  gutter: 8pt,\n");
                let len = cf_slice.len();
                let is_odd = len % 2 == 1;
                for (idx, field) in cf_slice.iter().enumerate() {
                    let card_expr = format!(
                        "card-box([\n    #stack(\n      dir: ttb,\n      spacing: 3pt,\n      text(size: 7pt, weight: \"bold\", fill: color-slate-400)[{}],\n      text(size: 9pt, weight: \"medium\", fill: color-slate-900)[{}]\n    )\n  ])",
                        escape_typst_text(&field.label.to_uppercase()),
                        escape_typst_text(&field.value)
                    );
                    if is_odd && idx + 1 == len {
                        markup.push_str(&format!("  grid.cell(colspan: 2, {}),\n", card_expr));
                    } else {
                        markup.push_str(&format!("  {},\n", card_expr));
                    }
                }
                markup.push_str(")\n#v(4pt)\n\n");
            }
        }
    }

    // 7. Body Section according to Document Type
    match req.doc_type {
        DocumentType::Certificate | DocumentType::CertificateCompletion | DocumentType::CertificateInsurance => {
            // Honorable Certificate Layout: Zero tables, centered attestation, official seal
            markup.push_str("#v(20pt)\n");
            markup.push_str("#align(center)[\n");
            let cert_title = match req.doc_type {
                DocumentType::CertificateCompletion => "CERTIFICATE OF COMPLETION",
                DocumentType::CertificateInsurance => "CERTIFICATE OF INSURANCE (COI)",
                _ => "CERTIFICATE OF RECOGNITION",
            };
            markup.push_str(&format!(
                "  #text(size: 11pt, weight: \"bold\", fill: rgb(\"{}\"))[{}] \\\n  #v(8pt)\n  #text(size: 13pt, style: \"italic\", fill: color-slate-500)[Issued to] \\\n  #v(10pt)\n  #text(size: 24pt, weight: \"bold\", fill: rgb(\"{}\"))[{}] \\\n  #v(10pt)\n  #text(size: 10.5pt, fill: color-slate-700)[\n{}\n]  #v(16pt)\n",
                primary_hex, cert_title, primary_hex, escape_typst_text(&req.to.name),
                escape_typst_text(req.terms.as_deref().unwrap_or("Certificate details not supplied."))
            ));
            markup.push_str("]\n#v(14pt)\n");
        }
        DocumentType::Agreement
        | DocumentType::AgreementSow
        | DocumentType::Nda
        | DocumentType::ContractorAgreement
        | DocumentType::RetainerAgreement
        | DocumentType::BillOfSale => {
            // Legal & Contractual layout
            markup.push_str("#v(8pt)\n");
            markup.push_str(&format!(
                "= Recitals & Operative Provisions\n\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n\n",
                escape_typst_text(req.terms.as_deref().unwrap_or(
                    "1. PURPOSE & SCOPE: The parties agree to collaborate in good faith according to the parameters specified herein.\n\n2. CONFIDENTIALITY: Each party agrees to safeguard proprietary technical, business, and financial information disclosed during the term.\n\n3. INTELLECTUAL PROPERTY: All custom deliverables, code, designs, and work product vest exclusively with the client upon receipt of full payment.\n\n4. GOVERNING LAW & ARBITRATION: This agreement is governed by applicable commercial laws, and disputes shall be resolved through expedited binding arbitration."
                ))
            ));

            if !req.items.is_empty() {
                markup.push_str("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Financial Consideration & Retainer Schedule]]\n");
                markup.push_str(&open_styled_table(
                    "(1fr, auto)",
                    "(col, row) => if col == 0 { left + horizon } else { right + horizon }",
                    &["Consideration / Milestone", "Retainer Amount"],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for item in &req.items {
                    markup.push_str(&format!(
                        "  [{}], [{}],\n",
                        escape_typst_text(&item.description),
                        req.format_amount_typst(item.amount())
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, "Total Contract Retainer:");
            }
        }
        DocumentType::ResidentialLease
        | DocumentType::RoomRentalLease
        | DocumentType::SubleaseAgreement
        | DocumentType::RentToOwnAgreement
        | DocumentType::CommercialNnnLease
        | DocumentType::CommercialLease
        | DocumentType::LeaseRenewalAgreement
        | DocumentType::LeaseAmendment
        | DocumentType::LetterOfIntentCre => {
            if !req.items.is_empty() {
                let schedule_title = match req.doc_type {
                    DocumentType::CommercialNnnLease => "Base Rent, NNN Operating Expenses & Deposit Schedule",
                    DocumentType::LetterOfIntentCre => "Proposed Base Rent & Consideration Schedule",
                    DocumentType::RoomRentalLease => "Co-Living Room Rent, Shared Utility Split & Deposit Schedule",
                    DocumentType::SubleaseAgreement => "Sublease Monthly Rent, Proration & Deposit Schedule",
                    DocumentType::RentToOwnAgreement => "Monthly Rent, Option Consideration & Rent Credit Schedule",
                    DocumentType::LeaseRenewalAgreement => "Prior Rent vs. Renewed Rent & Adjustment Schedule",
                    DocumentType::LeaseAmendment => "Lease Amendment Financial & Fee Adjustments",
                    _ => "Rent, Security Deposit & Fee Schedule",
                };
                markup.push_str(&format!("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[{}]]\n", schedule_title));
                markup.push_str(&open_styled_table(
                    "(1fr, auto)",
                    "(col, row) => if col == 0 { left + horizon } else { right + horizon }",
                    &["Lease Covenant / Financial Term", "Contract Amount / Rate"],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for item in &req.items {
                    markup.push_str(&format!(
                        "  [{}], [{}],\n",
                        escape_typst_text(&item.description),
                        req.format_amount_typst(item.amount())
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                let total_label = match req.doc_type {
                    DocumentType::CommercialNnnLease => "Initial Monthly & Deposit Total:",
                    DocumentType::RoomRentalLease => "Total Room Move-In Commitment:",
                    DocumentType::SubleaseAgreement => "Total Sublease Move-In Commitment:",
                    DocumentType::RentToOwnAgreement => "Total Initial Lease-Option Commitment:",
                    DocumentType::LeaseRenewalAgreement => "Renewed Monthly Lease Commitment:",
                    DocumentType::LeaseAmendment => "Net Amendment Adjustment:",
                    _ => "Total Initial Move-In Commitment:",
                };
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, total_label);
            }

            // Bug C5 fix: Render lease covenants at 9pt primary legibility in the body
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    markup.push_str(&format!(
                        "#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Operative Lease Covenants & Statutory Terms]]\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n#v(4pt)\n",
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }

            if req.doc_type == DocumentType::SubleaseAgreement {
                markup.push_str("#v(4pt)\n#card-box([\n  #text(size: 8pt, weight: \"bold\", fill: navy-judicial)[THREE-PARTY PRIME LANDLORD CONSENT & ATTORNMENT] \\\n  #v(2pt)\n  #text(size: 8pt, fill: color-slate-700)[The undersigned Prime Landlord hereby consents to the foregoing Sublease Agreement without releasing Prime Tenant (Sublessor) from primary liability under the Master Lease.] \\\n  #v(10pt)\n  #grid(columns: (1.5fr, 1fr), gutter: 20pt, [#line(length: 100%, stroke: 0.75pt + color-slate-400) \\ #text(size: 6.5pt, fill: color-slate-500)[Prime Landlord / Master Lessor Signature]], [#line(length: 100%, stroke: 0.75pt + color-slate-400) \\ #text(size: 6.5pt, fill: color-slate-500)[Date of Master Lessor Consent]])\n])\n");
            }
        }
        DocumentType::NoticeToVacate | DocumentType::NoticeOfEntry | DocumentType::RentIncreaseNotice => {
            if !req.items.is_empty() {
                let (tbl_title, col1, col2, tot_lbl) = match req.doc_type {
                    DocumentType::NoticeOfEntry => (
                        "Scheduled Premises Entry & Maintenance Scope",
                        "Entry Window / Inspection Scope",
                        "Tenant Fee / Charge",
                        "Total Scheduled Charge:",
                    ),
                    DocumentType::RentIncreaseNotice => (
                        "Current vs. Adjusted Monthly Rent Schedule",
                        "Rent Component / Effective Date",
                        "Monthly Amount",
                        "New Adjusted Monthly Rent:",
                    ),
                    _ => (
                        "Statutory Delinquency & Past-Due Rental Ledger",
                        "Delinquent Period / Contractual Charge",
                        "Amount Demanded to Cure",
                        "Total Delinquent Balance Demanded:",
                    ),
                };
                markup.push_str(&format!("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[{}]]\n", tbl_title));
                markup.push_str(&open_styled_table(
                    "(1fr, auto)",
                    "(col, row) => if col == 0 { left + horizon } else { right + horizon }",
                    &[col1, col2],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for item in &req.items {
                    markup.push_str(&format!(
                        "  [{}], [{}],\n",
                        escape_typst_text(&item.description),
                        req.format_amount_typst(item.amount())
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, tot_lbl);
            }
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    markup.push_str(&format!(
                        "#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Statutory Notice & Legal Demand]]\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n#v(4pt)\n",
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }
            let state_name = req
                .effective_state_profile()
                .map(|p| p.name)
                .unwrap_or_default();
            let premises_addr = req
                .to
                .address
                .as_deref()
                .unwrap_or_default();
            let service_field = |labels: &[&str]| {
                req.custom_fields.iter()
                    .find(|cf| labels.contains(&cf.label.trim().to_ascii_lowercase().as_str()))
                    .map(|cf| cf.value.as_str()).unwrap_or_default()
            };
            let service_method = req
                .custom_fields
                .iter()
                .find(|cf| {
                    let l = cf.label.to_ascii_lowercase();
                    l.contains("service method") || l.contains("delivery method") || l.contains("method of service")
                })
                .map(|cf| {
                    let v = cf.value.to_ascii_lowercase();
                    if (v.contains("personal") || v.contains("hand")) && v.contains("mail") {
                        "personal_and_mail"
                    } else if v.contains("post") || v.contains("door") || v.contains("nail") {
                        "posting"
                    } else if v.contains("sub") || v.contains("household") {
                        "substitute"
                    } else if v.contains("mail") || v.contains("certified") {
                        "mail"
                    } else if v.contains("personal") || v.contains("hand") {
                        "personal"
                    } else {
                        ""
                    }
                })
                .unwrap_or_default();
            markup.push_str(&format!(
                "#v(6pt)\n#certificate-of-service(\n  server-name: \"{}\",\n  service-date: \"{}\",\n  recipient-name: \"{}\",\n  premises-address: \"{}\",\n  state-name: \"{}\",\n  service-method: \"{}\"\n)\n#v(4pt)\n",
                escape_typst_string_literal(service_field(&["server name", "served by"])),
                escape_typst_string_literal(service_field(&["service date", "date of service", "served on"])),
                escape_typst_string_literal(&req.to.name),
                escape_typst_string_literal(&premises_addr.replace('\n', ", ")),
                escape_typst_string_literal(state_name),
                service_method
            ));
        }
        DocumentType::SecurityDepositDisposition => {
            if !req.items.is_empty() {
                markup.push_str("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Security Deposit Itemized Accounting & Refund Statement]]\n");
                markup.push_str(&open_styled_table(
                    "(1fr, auto)",
                    "(col, row) => if col == 0 { left + horizon } else { right + horizon }",
                    &["Security Deposit Credit / Itemized Deduction", "Amount (Credit / Deduction)"],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for item in &req.items {
                    markup.push_str(&format!(
                        "  [{}], [{}],\n",
                        escape_typst_text(&item.description),
                        req.format_amount_typst(item.amount())
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, "Net Security Deposit Refund Due:");
            }
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    markup.push_str(&format!(
                        "#v(6pt)\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n#v(4pt)\n",
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }
        }
        DocumentType::RealEstateCommission => {
            if !req.items.is_empty() {
                markup.push_str("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Commission Calculation, Co-Broker Split & Disbursement Authorization]]\n");
                markup.push_str(&open_styled_table(
                    "(1fr, auto)",
                    "(col, row) => if col == 0 { left + horizon } else { right + horizon }",
                    &["Commission / Split / Brokerage Fee Line Item", "Disbursement Amount"],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for item in &req.items {
                    markup.push_str(&format!(
                        "  [{}], [{}],\n",
                        escape_typst_text(&item.description),
                        req.format_amount_typst(item.amount())
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, "Net Payable Commission:");
            }
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    markup.push_str(&format!(
                        "#v(6pt)\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n#v(4pt)\n",
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }
        }
        DocumentType::TenantEstoppel => {
            if !req.items.is_empty() {
                markup.push_str("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Lease Financial & Deposit Summary]]\n");
                markup.push_str(&open_styled_table(
                    "(1fr, auto)",
                    "(col, row) => if col == 0 { left + horizon } else { right + horizon }",
                    &["Lease Financial Component", "Supplied Amount"],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for item in &req.items {
                    markup.push_str(&format!(
                        "  [{}], [{}],\n",
                        escape_typst_text(&item.description),
                        req.format_amount_typst(item.amount())
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, "Total Monthly & Deposit Amount:");
            }
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    markup.push_str(&format!(
                        "#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Tenant Estoppel Representations & Certifications]]\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n#v(4pt)\n",
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }
        }
        DocumentType::RentalApplication | DocumentType::AdverseActionNotice | DocumentType::MutualLeaseTermination => {
            if !req.items.is_empty() {
                let (tbl_title, col1, col2, tot_lbl) = match req.doc_type {
                    DocumentType::RentalApplication => (
                        "Application Screening Fee, Holding Deposit & Target Rent Schedule",
                        "Screening / Deposit / Target Rent Component",
                        "Amount",
                        "Total Application & Holding Commitment:",
                    ),
                    DocumentType::AdverseActionNotice => (
                        "Applicant Screening & Credit Evaluation Summary",
                        "Evaluation Factor / Screening Metric",
                        "Reported Value / Fee",
                        "Application Fee Status:",
                    ),
                    _ => (
                        "Mutual Lease Surrender & Cash-for-Keys Settlement Schedule",
                        "Surrender Consideration / Proration / Deposit Item",
                        "Settlement Amount",
                        "Net Surrender Settlement Due:",
                    ),
                };
                markup.push_str(&format!("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[{}]]\n", tbl_title));
                markup.push_str(&open_styled_table(
                    "(1fr, auto)",
                    "(col, row) => if col == 0 { left + horizon } else { right + horizon }",
                    &[col1, col2],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for item in &req.items {
                    markup.push_str(&format!(
                        "  [{}], [{}],\n",
                        escape_typst_text(&item.description),
                        req.format_amount_typst(item.amount())
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, tot_lbl);
            }
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    let sec_title = match req.doc_type {
                        DocumentType::RentalApplication => "FCRA Consumer Report Authorization & Applicant Representations",
                        DocumentType::AdverseActionNotice => "Statutory FCRA Adverse Action Notice (15 U.S.C. § 1681m)",
                        _ => "Mutual Lease Termination, Surrender & Release Covenants",
                    };
                    markup.push_str(&format!(
                        "#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[{}]]\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n#v(4pt)\n",
                        sec_title,
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }
        }
        DocumentType::TenantRentLedger => {
            if !req.items.is_empty() {
                markup.push_str("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Tenant Rent, Fee & Payment Ledger]]\n");
                markup.push_str(&open_styled_table(
                    "(auto, 1fr, auto, auto)",
                    "(col, row) => if col <= 1 { left + horizon } else { right + horizon }",
                    &["\\#", "Date & Transaction Description", "Entry Amount", "Running Balance"],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                let mut running = 0.0;
                for (idx, item) in req.items.iter().enumerate() {
                    let amt = item.amount();
                    running += amt;
                    markup.push_str(&format!(
                        "  [{}], [{}], [{}], [{}],\n",
                        idx + 1,
                        escape_typst_text(&item.description),
                        req.format_amount_typst(amt),
                        req.format_amount_typst(running)
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, "Ending Ledger Balance Due:");
            }
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    markup.push_str(&format!(
                        "#v(6pt)\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n#v(4pt)\n",
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }
        }
        DocumentType::ScheduleOfRealEstateOwned
        | DocumentType::ScheduleETaxWorksheet
        | DocumentType::RealEstateDealAnalysis => {
            // Render #deal-kpi-grid from up to 4 custom_fields or default underwriting KPIs
            let mut kpi_tuples: Vec<(String, String, String)> = Vec::new();
            for cf in req.custom_fields.iter().take(4) {
                kpi_tuples.push((cf.label.clone(), cf.value.clone(), String::new()));
            }
            if kpi_tuples.is_empty() {
                kpi_tuples.push((
                    "NET TOTAL / NOI".to_string(),
                    req.format_amount(req.grand_total()),
                    "Calculated from supplied line items".to_string(),
                ));
                kpi_tuples.push((
                    "LINE ITEMS".to_string(),
                    req.items.len().to_string(),
                    "Active Entries".to_string(),
                ));
            }
            markup.push_str("#v(4pt)\n#deal-kpi-grid(\n  metrics: (\n");
            for (k, v, s) in &kpi_tuples {
                markup.push_str(&format!(
                    "    (\"{}\", \"{}\", \"{}\"),\n",
                    escape_typst_string_literal(k),
                    escape_typst_string_literal(v),
                    escape_typst_string_literal(s)
                ));
            }
            markup.push_str(&format!("  ),\n  accent: rgb(\"{}\")\n)\n#v(6pt)\n", primary_hex));

            if !req.items.is_empty() {
                let (tbl_title, col1, col2, tot_lbl) = match req.doc_type {
                    DocumentType::ScheduleOfRealEstateOwned => (
                        "Schedule of Real Estate Owned (SREO) Portfolio Asset & Debt Matrix",
                        "Property Address, Units, Market Value, Debt, Rate & DSCR",
                        "Annual Net Operating Income (NOI)",
                        "Total Portfolio Annual NOI:",
                    ),
                    DocumentType::ScheduleETaxWorksheet => (
                        "IRS Form 1040 Schedule E (Part I) Rental Income & Expense Reconciliation",
                        "IRS Schedule E Line / Stessa Operating Category",
                        "Annual Tax Amount",
                        "Net Rental Income / (Loss) (Schedule E):",
                    ),
                    _ => (
                        "Real Estate Deal Underwriting, Acquisition, Rehab & Cash-Flow Pro-Forma",
                        "Underwriting Component / Pro-Forma Line Item",
                        "Projected Amount",
                        "Projected Annual Cash Flow / NOI:",
                    ),
                };
                markup.push_str(&format!("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[{}]]\n", tbl_title));
                markup.push_str(&open_styled_table(
                    "(1fr, auto)",
                    "(col, row) => if col == 0 { left + horizon } else { right + horizon }",
                    &[col1, col2],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for item in &req.items {
                    let formatted_val = if item.is_ratio_metric() {
                        let d = item.description.to_ascii_lowercase();
                        if d.contains("dscr")
                            || d.contains("coverage")
                            || d.contains("grm")
                            || d.contains("multiplier")
                        {
                            format!("{:.2}x", item.amount())
                        } else {
                            format!("{:.2}\\%", item.amount())
                        }
                    } else {
                        req.format_amount_typst(item.amount())
                    };
                    markup.push_str(&format!(
                        "  [{}], [{}],\n",
                        escape_typst_text(&item.description),
                        formatted_val
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, tot_lbl);
            }
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    markup.push_str(&format!(
                        "#v(6pt)\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n#v(4pt)\n",
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }
        }
        DocumentType::PropertyManagementAgreement
        | DocumentType::RealEstatePurchaseAgreement
        | DocumentType::PropertyDeed
        | DocumentType::MechanicsLienWaiver => {
            if !req.items.is_empty() {
                markup.push_str("#v(6pt)\n");
                let (tbl_title, col1, col2, tot_lbl) = match req.doc_type {
                    DocumentType::PropertyManagementAgreement => (
                        "Management Compensation, Leasing Fee & Repair Reserve Schedule",
                        "Management Service / Fee / Reserve Authority",
                        "Contract Amount / Fee",
                        "Initial Owner Reserve & Onboarding Total:",
                    ),
                    DocumentType::RealEstatePurchaseAgreement => (
                        "Purchase Price, Earnest Money Deposit & Financing Breakdown",
                        "Purchase Consideration / Escrow Credit / Financing Item",
                        "Contract Amount",
                        "Net Purchase Consideration:",
                    ),
                    DocumentType::PropertyDeed => (
                        "Conveyance Consideration & Transfer Tax Valuation",
                        "Conveyance Parcel / Consideration / Documentary Stamp Item",
                        "Stated Value",
                        "Total Stated Conveyance Consideration:",
                    ),
                    _ => (
                        "Construction Progress Billing & Statutory Lien Waiver Schedule",
                        "Contract Scope / Draw Milestone / Retention Item",
                        "Waiver Release Amount",
                        "Net Statutory Lien Waiver Amount:",
                    ),
                };
                markup.push_str(&format!("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[{}]]\n", tbl_title));
                markup.push_str(&open_styled_table(
                    "(1fr, auto)",
                    "(col, row) => if col == 0 { left + horizon } else { right + horizon }",
                    &[col1, col2],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for item in &req.items {
                    markup.push_str(&format!(
                        "  [{}], [{}],\n",
                        escape_typst_text(&item.description),
                        req.format_amount_typst(item.amount())
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, tot_lbl);
            }
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    let sec_title = match req.doc_type {
                        DocumentType::PropertyManagementAgreement => "Exclusive Property Management Agency Covenants & Authority",
                        DocumentType::RealEstatePurchaseAgreement => "Purchase & Sale Covenants, Contingencies & Closing Terms",
                        DocumentType::PropertyDeed => "Statutory Granting Clause, Legal Description & Habendum",
                        _ => "Statutory Mechanic's Lien Waiver & Release Covenants",
                    };
                    markup.push_str(&format!(
                        "#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[{}]]\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n#v(4pt)\n",
                        sec_title,
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }
        }
        DocumentType::RentRoll => {
            if !req.items.is_empty() {
                markup.push_str("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Multi-Unit Tenancy Schedule & Rent Roll Census]]\n");
                markup.push_str(&open_styled_table(
                    "(auto, 1fr, auto)",
                    "(col, row) => if col <= 1 { left + horizon } else { right + horizon }",
                    &["Unit", "Tenant & Lease Expiration", "Monthly Contract Rent"],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for (idx, item) in req.items.iter().enumerate() {
                    markup.push_str(&format!(
                        "  [{}], [{}], [{}],\n",
                        idx + 1,
                        escape_typst_text(&item.description),
                        req.format_amount_typst(item.amount())
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, "Total Scheduled Monthly Rent:");
            }
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    markup.push_str(&format!(
                        "#v(4pt)\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n\n",
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }
        }
        DocumentType::ClosingSettlementStatement => {
            if !req.items.is_empty() {
                markup.push_str("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Escrow Settlement Disbursements & Adjustments (RESPA/ALTA)]]\n");
                markup.push_str(&open_styled_table(
                    "(1fr, auto)",
                    "(col, row) => if col == 0 { left + horizon } else { right + horizon }",
                    &["Settlement Item / Escrow Consideration", "Amount (Debit / Credit)"],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for item in &req.items {
                    markup.push_str(&format!(
                        "  [{}], [{}],\n",
                        escape_typst_text(&item.description),
                        req.format_amount_typst(item.amount())
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, "Net Settlement Due / Escrow Total:");
            }
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    markup.push_str(&format!(
                        "#v(4pt)\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n\n",
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }
        }
        DocumentType::PropertyManagementStatement => {
            if !req.items.is_empty() {
                markup.push_str("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Property Income & Operating Expense Ledger]]\n");
                markup.push_str(&open_styled_table(
                    "(1fr, auto)",
                    "(col, row) => if col == 0 { left + horizon } else { right + horizon }",
                    &["Ledger Description (Income / Expense)", "Net Amount"],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for item in &req.items {
                    markup.push_str(&format!(
                        "  [{}], [{}],\n",
                        escape_typst_text(&item.description),
                        req.format_amount_typst(item.amount())
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, "Net Owner Distribution:");
            }
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    markup.push_str(&format!(
                        "#v(4pt)\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n\n",
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }
        }
        DocumentType::PropertyInspectionChecklist => {
            if !req.items.is_empty() {
                markup.push_str("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Property Condition & Fixture Audit Matrix]]\n");
                markup.push_str(&open_styled_table(
                    "(auto, 1fr, auto)",
                    "(col, row) => if col <= 1 { left + horizon } else { right + horizon }",
                    &["Item", "Premises Area / Fixture", "Condition / Notes"],
                    req,
                    &primary_hex,
                    &secondary_hex,
                    &surface_hex,
                    "(x: 8pt, y: 7pt)",
                ));
                for (idx, item) in req.items.iter().enumerate() {
                    let status = if item.amount() > 0.0 { "Repair Needed" } else { "Satisfactory / Clean" };
                    markup.push_str(&format!(
                        "  [{}], [{}], [{}],\n",
                        idx + 1,
                        escape_typst_text(&item.description),
                        status
                    ));
                }
                markup.push_str(")\n#v(6pt)\n");
            }
            if let Some(ref terms) = req.terms {
                if !terms.is_empty() {
                    markup.push_str(&format!(
                        "#v(4pt)\n#text(size: 9pt, fill: color-slate-700)[\n{}\n]\n\n",
                        escape_typst_text(terms).replace('\n', "\\\n")
                    ));
                }
            }
        }
        _ => {
            // Standard Invoicing, Quotes, POs, Receipts, Work Orders, Bill of Sale, Expense Report
            if !req.items.is_empty() {
                let show_num = req.show_line_numbers.unwrap_or(false);
                if show_num {
                    markup.push_str(&open_styled_table(
                        "(auto, 1fr, auto, auto, auto)",
                        "(col, row) => if col <= 1 { left + horizon } else { right + horizon }",
                        &["\\#", "Description", "Qty / Hrs", "Unit Price", "Total Amount"],
                        req,
                        &primary_hex,
                        &secondary_hex,
                        &surface_hex,
                        "(x: 7pt, y: 6.5pt)",
                    ));

                    for (idx, item) in req.items.iter().enumerate() {
                        let amount = item.amount();
                        let unit_label = req.unit_measure.as_deref().unwrap_or("");
                        let qty_str = if unit_label.is_empty() {
                            format!("{:.1}", item.quantity)
                        } else {
                            format!("{:.1} {}", item.quantity, escape_typst_text(unit_label))
                        };
                        let desc_str = if let Some(d) = item.discount.filter(|&d| d > 0.0) {
                            format!(
                                "{} #text(size: 7.5pt, fill: rgb(\"#64748b\"))[(Less {} discount)]",
                                escape_typst_text(&item.description),
                                req.format_amount_typst(d)
                            )
                        } else {
                            escape_typst_text(&item.description)
                        };
                        markup.push_str(&format!(
                            "  [{}], [{}], [{}], [{}], [{}],\n",
                            idx + 1, desc_str, qty_str, req.format_amount_typst(item.unit_price), req.format_amount_typst(amount)
                        ));
                    }

                    markup.push_str(")\n\n");
                } else {
                    markup.push_str(&open_styled_table(
                        "(1fr, auto, auto, auto)",
                        "(col, row) => if col == 0 { left + horizon } else { right + horizon }",
                        &["Description", "Qty / Hrs", "Unit Price", "Total Amount"],
                        req,
                        &primary_hex,
                        &secondary_hex,
                        &surface_hex,
                        "(x: 8pt, y: 7pt)",
                    ));

                    for item in &req.items {
                        let amount = item.amount();
                        let unit_label = req.unit_measure.as_deref().unwrap_or("");
                        let qty_str = if unit_label.is_empty() {
                            format!("{:.1}", item.quantity)
                        } else {
                            format!("{:.1} {}", item.quantity, escape_typst_text(unit_label))
                        };
                        let desc_str = if let Some(d) = item.discount.filter(|&d| d > 0.0) {
                            format!(
                                "{} #text(size: 7.5pt, fill: rgb(\"#64748b\"))[(Less {} discount)]",
                                escape_typst_text(&item.description),
                                req.format_amount_typst(d)
                            )
                        } else {
                            escape_typst_text(&item.description)
                        };
                        markup.push_str(&format!(
                            "  [{}], [{}], [{}], [{}],\n",
                            desc_str, qty_str, req.format_amount_typst(item.unit_price), req.format_amount_typst(amount)
                        ));
                    }

                    markup.push_str(")\n\n");
                }
            }

            let total_label = if req.amount_paid.is_some() || req.is_paid_state() {
                "Document total:"
            } else {
                "Total Due:"
            };
            render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, total_label);
        }
    }

    // 7b. Layers 2 & 3: Conspicuous Typography Enforcement & Property/State-Triggered Inline Disclosures
    if req.doc_type.is_state_lease_instrument() {
        if let Some(state_profile) = req.effective_state_profile() {
            // Layer 2: Conspicuous Typography Clause (MI 12pt bold, MA 12pt bold, NJ 10pt bold, SC/TX/OH bold/underlined)
            if let Some(ref cc) = state_profile.conspicuous_clause {
                let raw_clause_text = if cc.uppercase {
                    cc.verbatim_text.to_uppercase()
                } else {
                    cc.verbatim_text.to_string()
                };
                let weight_str = if cc.bold { "bold" } else { "semibold" };
                let text_body = if cc.underlined {
                    format!(
                        "#underline[#text(size: {}pt, weight: \"{}\", fill: color-slate-900)[{}]]",
                        cc.point_size,
                        weight_str,
                        escape_typst_text(&raw_clause_text).replace('\n', "\\\n")
                    )
                } else {
                    format!(
                        "#text(size: {}pt, weight: \"{}\", fill: color-slate-900)[{}]",
                        cc.point_size,
                        weight_str,
                        escape_typst_text(&raw_clause_text).replace('\n', "\\\n")
                    )
                };
                markup.push_str(&format!(
                    "#v(4pt)\n#rect(width: 100%, fill: rgb(\"#fffbeb\"), stroke: 1pt + rgb(\"#d97706\"), radius: 3pt, inset: 8pt, [\n  #text(size: 8pt, weight: \"bold\", fill: rgb(\"#b45309\"))[{} ({})] \\\n  #v(3pt)\n  {}\n])\n#v(4pt)\n\n",
                    escape_typst_text(cc.title),
                    escape_typst_text(cc.citation),
                    text_body
                ));
            }

            // Layer 3: Triggered Inline Disclosures for Leases
            if matches!(
                req.doc_type,
                DocumentType::ResidentialLease
                    | DocumentType::RoomRentalLease
                    | DocumentType::SubleaseAgreement
                    | DocumentType::RentToOwnAgreement
                    | DocumentType::LeaseRenewalAgreement
                    | DocumentType::LeaseAmendment
            ) {
                let mut disclosures_vec: Vec<String> = Vec::new();
                if let Some(yb) = req.effective_year_built() {
                    if yb < 1978 {
                        disclosures_vec.push(format!(
                            "*Federal Pre-1978 Lead-Based Paint Disclosure (42 U.S.C. § 4852d · Constructed {}):* Housing built before 1978 may contain lead-based paint. Landlord disclosure: `[ ]` No known lead-based paint hazards OR `[ ]` Known hazards attached. Tenant receipt of EPA pamphlet *Protect Your Family From Lead in Your Home*: `[ ]` Confirmed. Complete these fields from the actual disclosure and receipt.",
                            yb
                        ));
                    }
                }
                if state_profile.requires_escrow_bank_disclosure {
                    let bank_info = req
                        .custom_fields
                        .iter()
                        .find(|cf| {
                            let l = cf.label.to_ascii_lowercase();
                            l.contains("escrow") || l.contains("bank") || l.contains("depository")
                        })
                        .map(|cf| cf.value.as_str())
                        .unwrap_or("Not supplied");
                    disclosures_vec.push(format!(
                        "*Escrow Bank Depository Disclosure ({}):* Depository details supplied by the preparer: *{}*.",
                        escape_typst_text(state_profile.governing_statute),
                        escape_typst_text(bank_info)
                    ));
                }
                if state_profile.requires_radon_disclosure {
                    disclosures_vec.push("*Statutory Radon Gas Warning:* Radon is a naturally occurring radioactive gas that, when accumulated in a building in sufficient quantities, may present health risks (lung cancer) to persons exposed over time.".to_string());
                }
                if state_profile.requires_mold_disclosure {
                    disclosures_vec.push("*Mold & Moisture Disclosure:* Inspection date, known moisture conditions, and visible mold findings: Not supplied. Complete the property-specific disclosure before use.".to_string());
                }
                if state_profile.requires_bed_bug_disclosure {
                    disclosures_vec.push("*Bed Bug Infestation History & Educational Notice:* Known infestation history and related treatment details: Not supplied. Complete the property-specific disclosure before use.".to_string());
                }
                if state_profile.requires_flood_disclosure {
                    disclosures_vec.push("*Statutory Flood Zone & Prior Flooding History Disclosure:* Landlord discloses whether the Premises is located in a FEMA Special Flood Hazard Area (100-year floodplain) or has experienced prior flooding damage; standard renter's insurance does not cover flood loss.".to_string());
                }
                if state_profile.requires_sprinkler_disclosure {
                    disclosures_vec.push("*Operative Fire Sprinkler System Disclosure:* Landlord discloses whether the Premises is equipped with an operative automatic fire sprinkler system and its most recent inspection/maintenance date.".to_string());
                }

                if !disclosures_vec.is_empty() {
                    markup.push_str(&format!(
                        "#v(4pt)\n#card-box([\n  #text(size: 8pt, weight: \"bold\", fill: navy-judicial)[LAYER 3 MANDATORY {} INLINE DISCLOSURES] \\\n  #v(3pt)\n  #text(size: 7.8pt, fill: color-slate-700)[\n    {}\n  ]\n])\n#v(4pt)\n\n",
                        escape_typst_text(&state_profile.name.to_uppercase()),
                        disclosures_vec.join(" \\\n    #v(2pt)\n    ")
                    ));
                }
            }
        }
    }

    // 8. Notes & Payment Terms
    if let Some(ref notes) = req.notes {
        if !notes.is_empty() {
            let notes_style = req.effective_notes_style();
            let escaped_notes = escape_typst_text(notes).replace('\n', "\\\n");
            match notes_style {
                "left_accent" | "numbered_legal" | "callout_card" => {
                    markup.push_str(&format!(
                        "#custom-section(\"NOTES & INSTRUCTIONS\", [{}], style: \"{}\", primary-color: rgb(\"{}\"), surface-color: rgb(\"{}\"))\n\n",
                        escaped_notes, notes_style, primary_hex, surface_hex
                    ));
                }
                _ => {
                    markup.push_str(&format!(
                        "#v(4pt)\n#card-box([\n  #text(size: 8pt, weight: \"bold\", fill: color-slate-400)[NOTES & INSTRUCTIONS] \\\n  #v(2pt)\n  #text(size: 8.5pt, fill: color-slate-700)[{}]\n])\n\n",
                        escaped_notes
                    ));
                }
            }
        }
    }

    if let Some(ref terms) = req.terms {
        if !terms.is_empty()
            && !matches!(
                req.doc_type,
                DocumentType::Agreement
                    | DocumentType::Nda
                    | DocumentType::ContractorAgreement
                    | DocumentType::Certificate
                    | DocumentType::CertificateCompletion
                    | DocumentType::CertificateInsurance
                    | DocumentType::SecurityDepositDisposition
                    | DocumentType::RealEstateCommission
                    | DocumentType::TenantRentLedger
                    | DocumentType::ScheduleOfRealEstateOwned
                    | DocumentType::ScheduleETaxWorksheet
                    | DocumentType::RealEstateDealAnalysis
            )
            && !req.doc_type.is_contract_or_lease()
        {
            markup.push_str(&format!(
                "#v(2pt)\n#text(size: 7.5pt, fill: color-slate-400)[\n  *TERMS & CONDITIONS:* {}\n]\n\n",
                escape_typst_text(terms)
            ));
        }
    }

    // Remittance Stub if enabled
    if req.show_remittance_stub.unwrap_or(false) {
        let remittance_due_date = req.due_date.as_deref().unwrap_or_default();
        markup.push_str(&format!(
            "#remittance-stub(\n  doc-number: \"{}\",\n  client-name: \"{}\",\n  amount-due: \"{}\",\n  due-date: \"{}\",\n  issuer-name: \"{}\"\n)\n\n",
            escape_typst_string_literal(&req.effective_doc_number()),
            escape_typst_string_literal(&req.to.name),
            escape_typst_string_literal(&req.format_amount(req.balance_due())),
            escape_typst_string_literal(&req.format_date(remittance_due_date)),
            escape_typst_string_literal(company_name)
        ));
    }

    // Dynamic Custom Sections
    let sections_style = req.effective_sections_style();
    for section in &req.sections {
        if !section.title.is_empty() || !section.content.is_empty() {
            markup.push_str(&format!(
                "#custom-section(\"{}\", [{}], style: \"{}\", primary-color: rgb(\"{}\"), surface-color: rgb(\"{}\"))\n\n",
                escape_typst_string_literal(&section.title),
                escape_typst_text(&section.content).replace('\n', "\\\n"),
                escape_typst_string_literal(sections_style),
                primary_hex,
                surface_hex
            ));
        }
    }

    // 9. Signatures Block (Multi-Signatory Support; avoid duplicating primary signer when #jurat-box is active or inline consent card emitted)
    let notary_req = req.is_notary_required();
    let jurat_active = req.execution_mode.is_some() || notary_req;
    if req.show_signature.unwrap_or(true) {
        let all_signatures = req.effective_signatures();
        let grid_signatures: Vec<_> = all_signatures
            .iter()
            .enumerate()
            .skip(if jurat_active { 1 } else { 0 })
            .filter(|(_, sig)| {
                if req.doc_type == DocumentType::SubleaseAgreement {
                    let t = sig.signer_title.as_deref().unwrap_or("").to_ascii_lowercase();
                    let n = sig.signer_name.to_ascii_lowercase();
                    !(t.contains("prime landlord")
                        || t.contains("master lessor")
                        || n.contains("prime landlord")
                        || n.contains("master lessor"))
                } else {
                    true
                }
            })
            .collect();

        if !grid_signatures.is_empty() {
            let sig_style = req.effective_signature_style();
            markup.push_str("#signature-grid((\n");
            for (idx, sig) in grid_signatures {
                let sig_asset = sig
                    .signature_data
                    .as_deref()
                    .and_then(|data| stage_image_asset(&format!("sig_{}", idx), data));

                let sig_arg = match sig_asset {
                    Some(ref p) => format!("\"{}\"", escape_typst_string_literal(p)),
                    None => "none".to_string(),
                };
                let date_str = sig.date.as_deref().unwrap_or_default();
                let formatted_date = req.format_date(date_str);
                let title_str = sig.signer_title.as_deref().unwrap_or("Authorized Signatory");

                markup.push_str(&format!(
                    "  signature-item(signature-path: {}, signer-name: \"{}\", signer-title: \"{}\", date: \"{}\", style: \"{}\", primary-color: rgb(\"{}\"), surface-color: rgb(\"{}\")),\n",
                    sig_arg,
                    escape_typst_string_literal(&sig.signer_name),
                    escape_typst_string_literal(title_str),
                    escape_typst_string_literal(&formatted_date),
                    escape_typst_string_literal(sig_style),
                    primary_hex,
                    surface_hex
                ));
            }
            markup.push_str("))\n");
        }
    }

    // 10. Execution Jurat (if execution_mode is specified OR notary is required — fixes Bug C3)
    if jurat_active {
        let em = if req.execution_mode.is_some() {
            req.normalized_execution_mode()
        } else if notary_req {
            "wet-ink"
        } else {
            "esign"
        };
        let signatures = req.effective_signatures();
        let first_sig = signatures.first();
        let signer_name = first_sig.map(|s| s.signer_name.as_str()).unwrap_or_default();
        let signer_title = first_sig.and_then(|s| s.signer_title.as_deref()).unwrap_or("Authorized Signatory");
        let date_str = first_sig.and_then(|s| s.date.as_deref()).unwrap_or_default();
        let sig_img = first_sig.and_then(|s| s.signature_data.as_deref())
            .and_then(|data| stage_image_asset("jurat_sig", data));
        let sig_img_str = match sig_img {
            Some(ref p) => format!("\"{}\"", escape_typst_string_literal(p)),
            None => "none".to_string(),
        };
        let include_clerk = req.include_clerk_stamp.unwrap_or(false);
        let state_profile = req.effective_state_profile();
        let state_name = state_profile.map(|p| p.name).unwrap_or_default();
        let county_name = req.effective_county();
        let ron_citation = state_profile
            .map(|p| p.notary_ron_statute)
            .unwrap_or_default();

        markup.push_str(&format!(
            "\n#v(8pt)\n#jurat-box(\n  signature-type: \"{}\",\n  notary-required: {},\n  signer-name: \"{}\",\n  signer-title: \"{}\",\n  signature-img: {},\n  execution-date: \"{}\",\n  ron-statute-citation: \"{}\",\n  include-clerk-stamp: {},\n  state: \"{}\",\n  county: \"{}\"\n)\n",
            escape_typst_string_literal(em),
            notary_req,
            escape_typst_string_literal(signer_name),
            escape_typst_string_literal(signer_title),
            sig_img_str,
            escape_typst_string_literal(&req.format_date(date_str)),
            escape_typst_string_literal(ron_citation),
            include_clerk,
            escape_typst_string_literal(state_name),
            escape_typst_string_literal(&county_name)
        ));
    }

    // 10b. Layer 4: Mandatory Standalone Separate Attachments (#pagebreak() for FL § 83.512, WI ATCP 134.09, NY RPL § 231-c, or lease_package_mode)
    if req.doc_type.is_state_lease_instrument() {
        if let Some(state_profile) = req.effective_state_profile() {
            if let Some(ref att) = state_profile.standalone_attachment {
                let should_attach_standalone = if state_profile.code == "NY" {
                    req.doc_type.is_lease_agreement()
                        || matches!(
                            req.doc_type,
                            DocumentType::RentIncreaseNotice | DocumentType::NoticeToVacate
                        )
                } else {
                    req.doc_type.is_lease_agreement()
                };
                if should_attach_standalone {
                    markup.push_str(&format!(
                        "\n#pagebreak()\n#rect(width: 100%, fill: rgb(\"#f8fafc\"), stroke: 1pt + navy-judicial, radius: 4pt, inset: 12pt, [\n  #text(size: 11pt, weight: \"bold\", fill: navy-judicial)[{}] \\\n  #v(2pt)\n  #text(size: 8pt, weight: \"bold\", fill: color-slate-500)[STATUTORY AUTHORITY: {} · STANDALONE SEPARATE ATTACHMENT] \\\n  #v(8pt)\n  #line(length: 100%, stroke: 0.5pt + color-slate-300)\n  #v(8pt)\n  #text(size: 9pt, fill: color-slate-800)[\n    {}\n  ]\n  #v(16pt)\n  #grid(\n    columns: (1fr, 1fr),\n    gutter: 24pt,\n    [\n      #line(length: 100%, stroke: 0.75pt + color-slate-700)\n      #v(2pt)\n      #text(size: 7.5pt, weight: \"bold\", fill: color-slate-600)[Tenant / Lessee Separate Initials & Acknowledgment ({})]\n    ],\n    [\n      #line(length: 100%, stroke: 0.75pt + color-slate-700)\n      #v(2pt)\n      #text(size: 7.5pt, weight: \"bold\", fill: color-slate-600)[Landlord / Lessor Acknowledgment ({})]\n    ]\n  )\n])\n",
                        escape_typst_text(att.title),
                        escape_typst_text(att.citation),
                        escape_typst_text(att.body).replace('\n', "\\\n"),
                        escape_typst_text(&req.to.name),
                        escape_typst_text(&req.from.name)
                    ));
                }
            }

            if req.lease_package_mode.unwrap_or(false)
                && state_profile.requires_move_in_checklist
                && req.doc_type.is_lease_agreement()
            {
                markup.push_str(&format!(
                    "\n#pagebreak()\n#rect(width: 100%, fill: rgb(\"#ffffff\"), stroke: 0.75pt + color-slate-400, radius: 4pt, inset: 12pt, [\n  #text(size: 11pt, weight: \"bold\", fill: navy-judicial)[MANDATORY {} MOVE-IN / MOVE-OUT CONDITION INVENTORY CHECKLIST] \\\n  #v(2pt)\n  #text(size: 8pt, fill: color-slate-500)[Governing Statute: {} · Prerequisite to Security Deposit Collection / Retention] \\\n  #v(8pt)\n  #table(\n    columns: (1.2fr, 1fr, 1fr),\n    stroke: 0.5pt + color-slate-300,\n    inset: 6pt,\n    table.header([*Room / Fixture*], [*Move-In Condition*], [*Move-Out Condition*]),\n    [Living Room / Flooring / Walls], [Clean / Undamaged], [],\n    [Kitchen Appliances / Countertops / Sink], [Working / Clean], [],\n    [Bedrooms / Windows / Locks / Screens], [Working / Intact], [],\n    [Bathrooms / Plumbing / Exhaust Fan], [Working / Leak-Free], [],\n    [Smoke & Carbon Monoxide Detectors], [Tested & Operational], []\n  )\n])\n",
                    escape_typst_text(&state_profile.name.to_uppercase()),
                    escape_typst_text(state_profile.governing_statute)
                ));
            }
        }
    }
}
