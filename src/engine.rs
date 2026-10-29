//! Pressmark Sub-Millisecond Typst Synthesis Engine
//!
//! Author: Daniel Hallman
//!
//! Executes pure in-memory Typst compilation (< 25ms) via stdin/stdout streaming.
//! Zero headless Chrome, zero ReportLab, zero WeasyPrint overhead.

use crate::models::{DocumentRequest, DocumentType};
use crate::templates::TYPST_THEME;
use std::time::Instant;

#[path = "engine_compiler.rs"]
mod engine_compiler;
pub use engine_compiler::*;

fn format_party_typst(party: &crate::models::PartyDetails) -> String {
    let mut lines: Vec<String> = Vec::new();
    if let Some(ref c) = party.company {
        let trimmed = c.trim();
        if !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case(party.name.trim()) {
            lines.push(format!(
                "#text(size: 8pt, weight: \"medium\", fill: color-slate-800)[{}]",
                escape_typst_text(trimmed)
            ));
        }
    }
    if let Some(ref a) = party.address {
        for addr_line in a.lines().map(str::trim).filter(|s| !s.is_empty()) {
            lines.push(format!(
                "#text(size: 7.8pt, fill: color-slate-600)[{}]",
                escape_typst_text(addr_line)
            ));
        }
    }
    let email_opt = party.email.as_deref().map(str::trim).filter(|s| !s.is_empty());
    let phone_opt = party.phone.as_deref().map(str::trim).filter(|s| !s.is_empty());
    match (email_opt, phone_opt) {
        (Some(e), Some(p)) if e.len() + p.len() <= 44 => {
            lines.push(format!(
                "#text(size: 7.4pt, fill: color-slate-600)[#text(size: 6.8pt, weight: \"bold\", fill: color-slate-400)[EMAIL:] {} #h(5pt) #text(fill: color-slate-300)[·] #h(5pt) #text(size: 6.8pt, weight: \"bold\", fill: color-slate-400)[TEL:] {}]",
                escape_typst_text(e),
                escape_typst_text(p)
            ));
        }
        (e_opt, p_opt) => {
            if let Some(e) = e_opt {
                lines.push(format!(
                    "#text(size: 7.4pt, fill: color-slate-600)[#text(size: 6.8pt, weight: \"bold\", fill: color-slate-400)[EMAIL:] {}]",
                    escape_typst_text(e)
                ));
            }
            if let Some(p) = p_opt {
                lines.push(format!(
                    "#text(size: 7.4pt, fill: color-slate-600)[#text(size: 6.8pt, weight: \"bold\", fill: color-slate-400)[TEL:] {}]",
                    escape_typst_text(p)
                ));
            }
        }
    }
    let mut meta_parts: Vec<String> = Vec::new();
    if let Some(t) = party.tax_id.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        meta_parts.push(format!(
            "#text(size: 6.8pt, weight: \"bold\", fill: color-slate-400)[TIN:] #text(size: 7.2pt, fill: color-slate-600)[{}]",
            escape_typst_text(t)
        ));
    }
    if let Some(r) = party.registration_number.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        meta_parts.push(format!(
            "#text(size: 6.8pt, weight: \"bold\", fill: color-slate-400)[REG:] #text(size: 7.2pt, fill: color-slate-600)[{}]",
            escape_typst_text(r)
        ));
    }
    if !meta_parts.is_empty() {
        lines.push(meta_parts.join(" #h(5pt) #text(size: 7pt, fill: color-slate-300)[·] #h(5pt) "));
    }
    if let Some(w) = party.website.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        lines.push(format!(
            "#text(size: 7.2pt, fill: color-slate-500)[#text(size: 6.8pt, weight: \"bold\", fill: color-slate-400)[WEB:] {}]",
            escape_typst_text(w)
        ));
    }
    lines.join(" \\\n")
}

fn render_totals_box(markup: &mut String, req: &DocumentRequest, primary_hex: &str, accent_hex: &str, surface_hex: &str, total_label: &str) {
    if req.items.is_empty()
        && matches!(
            req.doc_type,
            DocumentType::NoticeOfEntry
                | DocumentType::RentalApplication
                | DocumentType::AdverseActionNotice
                | DocumentType::TenantEstoppel
                | DocumentType::PropertyInspectionChecklist
                | DocumentType::Nda
                | DocumentType::LeaseAmendment
                | DocumentType::MutualLeaseTermination
                | DocumentType::PropertyDeed
        )
    {
        return;
    }

    let subtotal = req.subtotal();
    let tax_total = req.tax_total();
    let discount = req.discount_amount.unwrap_or(0.0);
    let grand_total = req.grand_total();

    let mut table_markup = String::new();
    table_markup.push_str("#box(width: 310pt)[\n");
    table_markup.push_str("  #table(\n");
    table_markup.push_str("    columns: (1fr, auto),\n");
    table_markup.push_str("    stroke: none,\n");
    table_markup.push_str("    inset: (x: 4pt, y: 3.5pt),\n");
    table_markup.push_str("    align: (col, row) => if col == 0 { left + horizon } else { right + horizon },\n");
    if let Some((prior_rent, net_increase, _new_rent)) = req.rent_increase_breakdown() {
        let pct = if prior_rent > 0.0 {
            (net_increase / prior_rent) * 100.0
        } else {
            0.0
        };
        table_markup.push_str(&format!(
            "    [#text(fill: color-slate-500)[Prior Monthly Rent:]], [#text(weight: \"bold\")[{}]],\n",
            req.format_amount_typst(prior_rent)
        ));
        table_markup.push_str(&format!(
            "    [#text(fill: color-slate-500)[Net Monthly Increase:]], [#text(weight: \"bold\", fill: rgb(\"{}\"))[+{} (+{:.1}\\%)]],\n",
            accent_hex,
            req.format_amount_typst(net_increase),
            pct
        ));
    } else if !matches!(
        req.doc_type,
        DocumentType::RealEstateDealAnalysis
            | DocumentType::ScheduleOfRealEstateOwned
            | DocumentType::ScheduleETaxWorksheet
    ) || req.tax_rate.is_some()
        || discount > 0.0
    {
        table_markup.push_str(&format!(
            "    [#text(fill: color-slate-500)[Subtotal:]], [#text(weight: \"bold\")[{}]],\n",
            req.format_amount_typst(subtotal)
        ));
    }
    if let Some(rate) = req.tax_rate {
        if rate > 0.0 {
            table_markup.push_str(&format!(
                "    [#text(fill: color-slate-500)[Tax ({:.1}%):]], [#text(weight: \"bold\")[{}]],\n",
                rate, req.format_amount_typst(tax_total)
            ));
        }
    }
    if discount > 0.0 {
        table_markup.push_str(&format!(
            "    [#text(fill: color-slate-500)[Discount / Credits:]], [#text(weight: \"bold\", fill: rgb(\"{}\"))[−{}]],\n",
            accent_hex, req.format_amount_typst(discount)
        ));
    }
    table_markup.push_str(&format!(
        "    table.hline(stroke: 1pt + rgb(\"{}\")),\n",
        primary_hex
    ));
    table_markup.push_str(&format!(
        "    [#text(size: 10.5pt, weight: 900, fill: rgb(\"{}\"))[{}]], [#text(size: 11.5pt, weight: 900, fill: rgb(\"{}\"))[{}]],\n",
        primary_hex, total_label, primary_hex, req.format_amount_typst(grand_total)
    ));
    if req.is_paid_state() || req.amount_paid.is_some() {
        let paid = req.amount_paid_effective();
        let balance = req.balance_due();
        table_markup.push_str(&format!(
            "    [#text(size: 8.5pt, fill: rgb(\"#059669\"))[Amount Paid:]], [#text(size: 8.5pt, weight: \"bold\", fill: rgb(\"#059669\"))[{}]],\n",
            req.format_amount_typst(paid)
        ));
        let bal_color = if balance <= 0.001 { "#059669" } else { "#dc2626" };
        table_markup.push_str(&format!(
            "    [#text(size: 9pt, weight: 900, fill: rgb(\"{}\"))[Balance Due:]], [#text(size: 9pt, weight: 900, fill: rgb(\"{}\"))[{}]],\n",
            bal_color, bal_color, req.format_amount_typst(balance)
        ));
    }
    table_markup.push_str("  )\n");
    table_markup.push_str("]");

    // 1. Paid in full attestation watermark stamp (if marked as paid, placed alongside Totals box for zero vertical collision)
    if req.is_paid_state() {
        let method = req.payment_method.as_deref().unwrap_or("Stripe / Apple Pay");
        let paid_date = req.paid_at.as_deref().unwrap_or(req.issue_date.as_deref().unwrap_or("2026-10-02"));
        let tx_ref = req.transaction_reference.as_deref().unwrap_or("STRIPE-PAID-001");
        markup.push_str(&format!(
            "#v(4pt)\n#block(breakable: false, width: 100%)[\n  #grid(\n    columns: (1fr, auto),\n    gutter: 16pt,\n    align: (center + horizon, right + horizon),\n    [#paid-watermark-stamp(tx-ref: \"{}\", paid-date: \"{}\", method: \"{}\")],\n    [\n{}\n    ]\n  )\n]\n#v(4pt)\n\n",
            escape_typst_string_literal(tx_ref),
            escape_typst_string_literal(paid_date),
            escape_typst_string_literal(method),
            table_markup
        ));
    } else {
        markup.push_str("#v(4pt)\n#align(right)[\n");
        markup.push_str(&table_markup);
        markup.push_str("\n]\n\n");
    }

    // 2. Vector Payment QR Code & Checkout Block (only for payable archetypes that are NOT fully paid)
    let non_empty_payment_link = req
        .payment_link
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    if !req.is_paid_state()
        && req.doc_type.allows_payment_qr()
        && (req.payment_qr_code == Some(true) || non_empty_payment_link.is_some())
    {
        let raw_url = non_empty_payment_link.unwrap_or("https://buy.stripe.com/demo_checkout");
        let payment_url = if raw_url.starts_with("https://")
            || raw_url.starts_with("http://")
            || raw_url.starts_with("mailto:")
            || raw_url.starts_with("tel:")
        {
            raw_url.to_string()
        } else {
            format!("https://{}", raw_url)
        };
        let qr_path = match generate_qr_code_svg(&payment_url, primary_hex) {
            Ok(svg) => stage_svg_content("payment_qr", &svg),
            Err(_) => None,
        };
        let qr_arg = match qr_path {
            Some(p) => format!("\"{}\"", escape_typst_string_literal(&p)),
            None => "none".to_string(),
        };
        let payment_status = req.effective_payment_status();
        let payment_method = req.payment_method.as_deref().unwrap_or("Stripe / Card / Apple Pay");
        let due_date_arg = match req.due_date.as_deref() {
            Some(d) => format!("\"{}\"", escape_typst_string_literal(d)),
            None => "none".to_string(),
        };
        let effective_amount_due = req.format_amount(req.balance_due());

        markup.push_str(&format!(
            "#payment-qr-block(\n  qr-path: {},\n  payment-url: \"{}\",\n  payment-status: \"{}\",\n  amount-due: \"{}\",\n  due-date: {},\n  payment-method: \"{}\",\n  primary-color: rgb(\"{}\"),\n  accent-color: rgb(\"{}\"),\n  surface-color: rgb(\"{}\")\n)\n\n",
            qr_arg,
            escape_typst_string_literal(&payment_url),
            escape_typst_string_literal(payment_status),
            escape_typst_string_literal(&effective_amount_due),
            due_date_arg,
            escape_typst_string_literal(payment_method),
            primary_hex, accent_hex, surface_hex
        ));
    }

    if req.show_qr_seal.unwrap_or(false) {
        let seal_hash = req.compute_audit_sha256();
        let short_hash = &seal_hash[..seal_hash.len().min(40)];
        markup.push_str(&format!(
            "#v(4pt)\n#verification-seal(\"{}\", hash: \"{}...\", primary-color: rgb(\"{}\"), accent-color: rgb(\"{}\"), surface-color: rgb(\"{}\"))\n#v(4pt)\n",
            escape_typst_string_literal(&req.effective_doc_number()),
            escape_typst_string_literal(short_hash),
            primary_hex, accent_hex, surface_hex
        ));
    }
}

/// Generate comprehensive, publication-grade Typst source markup from a `DocumentRequest`.
pub fn generate_typst_markup(req: &DocumentRequest) -> String {
    let primary_hex = req.effective_primary_color();
    let secondary_hex = req.effective_secondary_color();
    let accent_hex = req.effective_accent_color();
    let surface_hex = req.effective_surface_color();
    let text_hex = req.effective_text_color();

    let paper_size = req.effective_paper_size();
    let font_family = req.effective_font_family();
    let font_size = req.effective_font_size();

    let logo_path = req
        .logo_data
        .as_deref()
        .and_then(|data| stage_image_asset("logo", data));

    let mut markup = String::new();

    // 1. Embed theme primitives
    markup.push_str(TYPST_THEME);
    markup.push_str("\n\n");
    markup.push_str(&format!(
        "#let theme-primary = rgb(\"{}\")\n\
         #let theme-secondary = rgb(\"{}\")\n\
         #let theme-accent = rgb(\"{}\")\n\
         #let theme-surface = rgb(\"{}\")\n\
         #let theme-text = rgb(\"{}\")\n\n",
        primary_hex, secondary_hex, accent_hex, surface_hex, text_hex
    ));

    // 2. Document Setup & Page Geometry
    let bg_fill = req.effective_background_fill();
    let bg_code = match req.background_style.as_deref() {
        Some("security_frame") => {
            ",\n  background: place(center + horizon, rect(width: 100% - 24pt, height: 100% - 24pt, stroke: 1.25pt + rgb(\"#cbd5e1\"), radius: 4pt))"
        }
        Some("blueprint") => {
            ",\n  background: place(top + left, rect(width: 100%, height: 100%, stroke: 2pt + rgb(\"#0891b2\")))"
        }
        _ => ""
    };

    let footer_text = if let Some(ref ft) = req.footer_text {
        if !ft.trim().is_empty() {
            escape_typst_text(ft)
        } else {
            "Pressmark Vector Engine · Certified Record".to_string()
        }
    } else {
        "Pressmark Vector Engine · Certified Commercial Record".to_string()
    };

    let page_numbers_snippet = if req.show_page_numbers.unwrap_or(true) {
        "align(right, text(size: 7.5pt, fill: rgb(\"#94a3b8\"))[\n        Page #counter(page).display() of #counter(page).final().first()\n      ])"
    } else {
        "[]"
    };

    let footer_rule = if req.show_footer_divider.unwrap_or(false) {
        "#line(length: 100%, stroke: 0.5pt + rgb(\"#e2e8f0\"))\n    #v(2pt)\n    "
    } else {
        ""
    };

    let issue_date = req.issue_date.as_deref().unwrap_or("2026-10-01");
    let doc_num = req.effective_doc_number();
    let doc_title = req.effective_title();

    markup.push_str(&format!(
        "#set page(\n  paper: \"{}\",\n  fill: rgb(\"{}\"),\n  margin: {},\n  header: context {{\n    if counter(page).get().first() > 1 [\n      #grid(\n        columns: (1fr, 1fr),\n        text(font: {}, size: 7.5pt, fill: rgb(\"#94a3b8\"), weight: \"bold\")[{} · OFFICIAL RECORD],\n        align(right, text(font: {}, size: 7.5pt, fill: rgb(\"#94a3b8\"), weight: \"bold\")[\n          REF: {} · DATE: {}\n        ])\n      )\n      #v(-4pt)\n      #line(length: 100%, stroke: 0.5pt + rgb(\"#e2e8f0\"))\n    ]\n  }},\n  footer: context [\n    {}#grid(\n      columns: (1fr, auto),\n      text(size: 7.5pt, fill: rgb(\"#94a3b8\"))[{}],\n      {}\n    )\n  ]{}\n)\n",
        paper_size,
        bg_fill,
        req.effective_margins(),
        font_family,
        escape_typst_text(&doc_title),
        font_family,
        escape_typst_text(&doc_num),
        escape_typst_text(&req.format_date(issue_date)),
        footer_rule,
        footer_text,
        page_numbers_snippet,
        bg_code
    ));
    markup.push_str(&format!(
        "#set text(font: {}, size: {}, fill: color-slate-900, features: (\"tnum\",))\n#set par(leading: {} * 0.65em)\n\n",
        font_family, font_size, req.effective_line_height()
    ));

    // 3. Watermark if present
    if let Some(ref wm) = req.watermark {
        if !wm.is_empty() {
            markup.push_str(&format!("#render-watermark(\"{}\")\n\n", escape_typst_string_literal(wm)));
        }
    }

    // 4. Header block
    let logo_typst_arg = match logo_path {
        Some(ref p) => format!("\"{}\"", escape_typst_string_literal(p)),
        None => "none".to_string(),
    };
    let due_date_typst_arg = match req.due_date {
        Some(ref d) if !d.is_empty() => format!("\"{}\"", escape_typst_string_literal(&req.format_date(d))),
        _ => "none".to_string(),
    };
    let subtitle_typst_arg = match req.header_subtitle {
        Some(ref s) if !s.is_empty() => format!("\"{}\"", escape_typst_string_literal(s)),
        _ => "none".to_string(),
    };
    let company_name = req
        .from
        .company
        .as_deref()
        .unwrap_or(req.from.name.as_str());
    let is_paid = req.is_paid_state();
    let status_label = if let Some(ref st) = req.status {
        st.as_str()
    } else if req.doc_type == DocumentType::CreditNote {
        if req.effective_payment_status() == "refunded" {
            "CREDIT / REFUNDED"
        } else {
            "CREDIT NOTE"
        }
    } else if req.doc_type == DocumentType::Estimate || req.doc_type == DocumentType::QuotationTender {
        "ESTIMATE / QUOTE"
    } else if req.due_date.is_none()
        && req.is_paid.is_none()
        && (matches!(
            req.doc_type,
            DocumentType::Certificate
                | DocumentType::CertificateCompletion
                | DocumentType::CertificateInsurance
                | DocumentType::Nda
                | DocumentType::PackingSlip
                | DocumentType::DeliveryReceipt
                | DocumentType::PropertyInspectionChecklist
                | DocumentType::RentRoll
                | DocumentType::TenantEstoppel
                | DocumentType::LetterOfIntentCre
                | DocumentType::ScheduleOfRealEstateOwned
                | DocumentType::ScheduleETaxWorksheet
                | DocumentType::RealEstateDealAnalysis
        ) || req.doc_type.is_contract_or_lease())
    {
        "OFFICIALLY ISSUED"
    } else if is_paid {
        "PAID / SETTLED"
    } else if req.effective_payment_status() == "partial" {
        "PARTIALLY PAID"
    } else {
        "PAYMENT PENDING"
    };

    let layout_style = req.layout_style.as_deref().unwrap_or("standard");

    // 3b. Recordable Deed / County Register of Deeds 3.0-Inch Top Margin Header (Bug C3 fix)
    if req.is_recordable_instrument() {
        let state_name = req
            .effective_state_profile()
            .map(|p| p.name)
            .unwrap_or("Tennessee");
        let county_name = req.effective_county();
        let custom_prepared_by = req
            .custom_fields
            .iter()
            .find(|cf| {
                let l = cf.label.to_ascii_lowercase();
                l.contains("prepared by") || l.contains("preparer") || l.contains("drafted by")
            })
            .map(|cf| cf.value.trim())
            .filter(|v| !v.is_empty());
        let prepared_by = match custom_prepared_by {
            Some(v) => v.to_string(),
            None => format!(
                "{}{}",
                req.from.name,
                req.from
                    .address
                    .as_deref()
                    .map(|a| format!(", {}", a.replace('\n', ", ")))
                    .unwrap_or_default()
            ),
        };
        let custom_return_to = req
            .custom_fields
            .iter()
            .find(|cf| {
                let l = cf.label.to_ascii_lowercase();
                l.contains("return to") || l.contains("recording return") || l.contains("after recording")
            })
            .map(|cf| cf.value.trim())
            .filter(|v| !v.is_empty());
        let return_to = match custom_return_to {
            Some(v) => v.to_string(),
            None => format!(
                "{}{}",
                req.to.name,
                req.to
                    .address
                    .as_deref()
                    .map(|a| format!(", {}", a.replace('\n', ", ")))
                    .unwrap_or_default()
            ),
        };
        let parcel_id = req
            .custom_fields
            .iter()
            .find(|cf| {
                let l = cf.label.to_ascii_lowercase();
                l.contains("parcel") || l.contains("apn") || l.contains("pin") || l.contains("tax id")
            })
            .map(|cf| cf.value.as_str())
            .unwrap_or(req.doc_number.as_deref().unwrap_or("APN-VERIFIED"));

        markup.push_str(&format!(
            "#recording-header-block(\n  prepared-by: \"{}\",\n  return-to: \"{}\",\n  parcel-id: \"{}\",\n  state-name: \"{}\",\n  county-name: \"{}\"\n)\n#v(6pt)\n\n",
            escape_typst_string_literal(&prepared_by),
            escape_typst_string_literal(&return_to),
            escape_typst_string_literal(parcel_id),
            escape_typst_string_literal(state_name),
            escape_typst_string_literal(&county_name)
        ));
    }

    markup.push_str(&format!(
        "#document-header(\n  title: \"{}\",\n  subtitle: {},\n  doc-number: \"{}\",\n  date: \"{}\",\n  due-date: {},\n  logo-path: {},\n  company-name: \"{}\",\n  primary-color: rgb(\"{}\"),\n  is-paid: {},\n  status: \"{}\",\n  layout-style: \"{}\"\n)\n\n",
        escape_typst_string_literal(&req.effective_title()),
        subtitle_typst_arg,
        escape_typst_string_literal(&req.effective_doc_number()),
        escape_typst_string_literal(&req.format_date(req.issue_date.as_deref().unwrap_or("2026-10-01"))),
        due_date_typst_arg,
        logo_typst_arg,
        escape_typst_string_literal(company_name),
        primary_hex,
        is_paid,
        escape_typst_string_literal(status_label),
        escape_typst_string_literal(layout_style)
    ));

    // Compliance Notice Banner
    let comp = req.normalized_compliance_mode();
    if comp != "none" && !comp.is_empty() {
        markup.push_str(&format!(
            "#compliance-banner(\"{}\", primary-color: rgb(\"{}\"))\n#v(6pt)\n\n",
            escape_typst_string_literal(comp), primary_hex
        ));
    }

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

    markup.push_str(&format!(
        "#parties-grid(\n  from-name: \"{}\",\n  from-details: [{}],\n  to-name: \"{}\",\n  to-details: [{}],\n  from-title: \"{}\",\n  to-title: \"{}\",\n  primary-color: rgb(\"{}\"),\n  secondary-color: rgb(\"{}\"),\n  surface-color: rgb(\"{}\")\n)\n\n",
        escape_typst_string_literal(&req.from.name),
        from_details_escaped,
        escape_typst_string_literal(&req.to.name),
        to_details_escaped,
        escape_typst_string_literal(from_title),
        escape_typst_string_literal(to_title),
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
        markup.push_str("#v(4pt)\n");
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
                "  #text(size: 11pt, weight: \"bold\", fill: rgb(\"{}\"))[{}] \\\n  #v(8pt)\n  #text(size: 13pt, style: \"italic\", fill: color-slate-500)[This certifies that] \\\n  #v(10pt)\n  #text(size: 24pt, weight: \"bold\", fill: rgb(\"{}\"))[{}] \\\n  #v(10pt)\n  #text(size: 10.5pt, fill: color-slate-700)[\n{}\n]  #v(16pt)\n",
                primary_hex, cert_title, primary_hex, escape_typst_text(&req.to.name),
                escape_typst_text(req.terms.as_deref().unwrap_or("Having fulfilled all criteria and professional requirements in full compliance with recognized standards."))
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
                markup.push_str(&format!(
                    "#table(\n  columns: (1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*Consideration / Milestone*], [*Retainer Amount*]),\n",
                    secondary_hex, secondary_hex, surface_hex
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
                markup.push_str(&format!(
                    "#table(\n  columns: (1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*Lease Covenant / Financial Term*], [*Contract Amount / Rate*]),\n",
                    secondary_hex, secondary_hex, surface_hex
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
                markup.push_str(&format!(
                    "#table(\n  columns: (1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*{}*], [*{}*]),\n",
                    secondary_hex, secondary_hex, surface_hex, col1, col2
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
                .unwrap_or("Tennessee");
            let premises_addr = req
                .to
                .address
                .as_deref()
                .unwrap_or("Premises Address on File");
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
                    } else {
                        "personal"
                    }
                })
                .unwrap_or("personal");
            markup.push_str(&format!(
                "#v(6pt)\n#certificate-of-service(\n  server-name: \"{}\",\n  service-date: \"{}\",\n  recipient-name: \"{}\",\n  premises-address: \"{}\",\n  state-name: \"{}\",\n  service-method: \"{}\"\n)\n#v(4pt)\n",
                escape_typst_string_literal(&req.from.name),
                escape_typst_string_literal(req.issue_date.as_deref().unwrap_or("2026-10-01")),
                escape_typst_string_literal(&req.to.name),
                escape_typst_string_literal(&premises_addr.replace('\n', ", ")),
                escape_typst_string_literal(state_name),
                service_method
            ));
        }
        DocumentType::SecurityDepositDisposition => {
            if !req.items.is_empty() {
                markup.push_str("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Security Deposit Itemized Accounting & Refund Statement]]\n");
                markup.push_str(&format!(
                    "#table(\n  columns: (1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*Security Deposit Credit / Itemized Deduction*], [*Amount (Credit / Deduction)*]),\n",
                    secondary_hex, secondary_hex, surface_hex
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
                markup.push_str(&format!(
                    "#table(\n  columns: (1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*Commission / Split / Brokerage Fee Line Item*], [*Disbursement Amount*]),\n",
                    secondary_hex, secondary_hex, surface_hex
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
                markup.push_str("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Certified Lease Financial & Deposit Summary]]\n");
                markup.push_str(&format!(
                    "#table(\n  columns: (1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*Certified Lease Financial Component*], [*Verified Amount*]),\n",
                    secondary_hex, secondary_hex, surface_hex
                ));
                for item in &req.items {
                    markup.push_str(&format!(
                        "  [{}], [{}],\n",
                        escape_typst_text(&item.description),
                        req.format_amount_typst(item.amount())
                    ));
                }
                markup.push_str(")\n#v(4pt)\n");
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, "Total Certified Monthly & Deposit Exposure:");
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
                markup.push_str(&format!(
                    "#table(\n  columns: (1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*{}*], [*{}*]),\n",
                    secondary_hex, secondary_hex, surface_hex, col1, col2
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
                markup.push_str("#block(sticky: true, above: 8pt, below: 4pt)[#text(size: 10pt, weight: \"bold\", fill: color-slate-900)[Chronological Certified Tenant Rent, Fee & Payment Ledger]]\n");
                markup.push_str(&format!(
                    "#table(\n  columns: (auto, 1fr, auto, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col <= 1 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*\\#*], [*Date & Transaction Description*], [*Entry Amount*], [*Running Balance*]),\n",
                    secondary_hex, secondary_hex, surface_hex
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
                render_totals_box(&mut markup, req, &primary_hex, &accent_hex, &surface_hex, "Certified Ending Ledger Balance Due:");
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
                    "Verified Schedule".to_string(),
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
                markup.push_str(&format!(
                    "#table(\n  columns: (1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*{}*], [*{}*]),\n",
                    secondary_hex, secondary_hex, surface_hex, col1, col2
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
                markup.push_str(&format!(
                    "#table(\n  columns: (1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*{}*], [*{}*]),\n",
                    secondary_hex, secondary_hex, surface_hex, col1, col2
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
                markup.push_str(&format!(
                    "#table(\n  columns: (auto, 1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col <= 1 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*Unit*], [*Tenant & Lease Expiration*], [*Monthly Contract Rent*]),\n",
                    secondary_hex, secondary_hex, surface_hex
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
                markup.push_str(&format!(
                    "#table(\n  columns: (1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*Settlement Item / Escrow Consideration*], [*Amount (Debit / Credit)*]),\n",
                    secondary_hex, secondary_hex, surface_hex
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
                markup.push_str(&format!(
                    "#table(\n  columns: (1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*Ledger Description (Income / Expense)*], [*Net Amount*]),\n",
                    secondary_hex, secondary_hex, surface_hex
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
                markup.push_str(&format!(
                    "#table(\n  columns: (auto, 1fr, auto),\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 8pt, y: 7pt),\n  align: (col, row) => if col <= 1 {{ left + horizon }} else {{ right + horizon }},\n  table.header([*Item*], [*Premises Area / Fixture*], [*Condition / Notes*]),\n",
                    secondary_hex, secondary_hex, surface_hex
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
                    markup.push_str("#table(\n");
                    markup.push_str("  columns: (auto, 1fr, auto, auto, auto),\n");
                    markup.push_str(&format!("  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n", secondary_hex));
                    markup.push_str(&format!("  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n", secondary_hex, surface_hex));
                    markup.push_str("  inset: (x: 7pt, y: 6.5pt),\n");
                    markup.push_str("  align: (col, row) => if col <= 1 { left + horizon } else { right + horizon },\n");
                    markup.push_str("  table.header(\n");
                    markup.push_str("    [*\\#*], [*Description*], [*Qty / Hrs*], [*Unit Price*], [*Total Amount*]\n");
                    markup.push_str("  ),\n");

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
                    markup.push_str("#table(\n");
                    markup.push_str("  columns: (1fr, auto, auto, auto),\n");
                    markup.push_str(&format!("  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n", secondary_hex));
                    markup.push_str(&format!("  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n", secondary_hex, surface_hex));
                    markup.push_str("  inset: (x: 8pt, y: 7pt),\n");
                    markup.push_str("  align: (col, row) => if col == 0 { left + horizon } else { right + horizon },\n");
                    markup.push_str("  table.header(\n");
                    markup.push_str("    [*Description*], [*Qty / Hrs*], [*Unit Price*], [*Total Amount*]\n");
                    markup.push_str("  ),\n");

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
                            "*Federal Pre-1978 Lead-Based Paint Disclosure (42 U.S.C. § 4852d · Constructed {}):* Housing built before 1978 may contain lead-based paint. Landlord discloses `[X]` No known lead-based paint hazards OR `[ ]` Known hazards attached; Tenant acknowledges receipt of EPA pamphlet *Protect Your Family From Lead in Your Home*.",
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
                        .unwrap_or("Segregated Statutory Escrow Account at FDIC-Insured Depository (See Schedule)");
                    disclosures_vec.push(format!(
                        "*Mandatory Escrow Bank Depository Disclosure ({}):* Security deposit funds are held at: *{}*.",
                        escape_typst_text(state_profile.governing_statute),
                        escape_typst_text(bank_info)
                    ));
                }
                if state_profile.requires_radon_disclosure {
                    disclosures_vec.push("*Statutory Radon Gas Warning:* Radon is a naturally occurring radioactive gas that, when accumulated in a building in sufficient quantities, may present health risks (lung cancer) to persons exposed over time.".to_string());
                }
                if state_profile.requires_mold_disclosure {
                    disclosures_vec.push("*Statutory Mold & Moisture Prevention Disclosure:* Landlord has inspected the Premises and knows of no damp or wet building materials or visible mold contamination; Tenant agrees to maintain ventilation and report water leaks immediately.".to_string());
                }
                if state_profile.requires_bed_bug_disclosure {
                    disclosures_vec.push("*Statutory Bed Bug Infestation History & Educational Notice:* Landlord certifies the Premises and adjacent units are free of known active bed bug infestation; Tenant agrees to inspect personal furnishings prior to move-in.".to_string());
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
            markup.push_str(&format!(
                "#v(4pt)\n#card-box([\n  #text(size: 8pt, weight: \"bold\", fill: color-slate-400)[NOTES & INSTRUCTIONS] \\\n  #v(2pt)\n  #text(size: 8.5pt, fill: color-slate-700)[{}]\n])\n\n",
                escape_typst_text(notes).replace('\n', "\\\n")
            ));
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
        let remittance_due_date = req.due_date.as_deref().unwrap_or("2026-10-15");
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
    for section in &req.sections {
        if !section.title.is_empty() || !section.content.is_empty() {
            markup.push_str(&format!(
                "#custom-section(\"{}\", [{}])\n\n",
                escape_typst_string_literal(&section.title),
                escape_typst_text(&section.content).replace('\n', "\\\n")
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
                let date_str = sig.date.as_deref().unwrap_or(req.issue_date.as_deref().unwrap_or("2026-10-01"));
                let formatted_date = req.format_date(date_str);
                let title_str = sig.signer_title.as_deref().unwrap_or("Authorized Signatory");

                markup.push_str(&format!(
                    "  signature-item(signature-path: {}, signer-name: \"{}\", signer-title: \"{}\", date: \"{}\", primary-color: rgb(\"{}\")),\n",
                    sig_arg,
                    escape_typst_string_literal(&sig.signer_name),
                    escape_typst_string_literal(title_str),
                    escape_typst_string_literal(&formatted_date),
                    primary_hex
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
        let signer_name = first_sig.map(|s| s.signer_name.as_str()).unwrap_or(req.from.name.as_str());
        let signer_title = first_sig.and_then(|s| s.signer_title.as_deref()).unwrap_or("Authorized Signatory");
        let date_str = first_sig.and_then(|s| s.date.as_deref()).unwrap_or(req.issue_date.as_deref().unwrap_or("2026-10-01"));
        let sig_img = first_sig.and_then(|s| s.signature_data.as_deref())
            .and_then(|data| stage_image_asset("jurat_sig", data));
        let sig_img_str = match sig_img {
            Some(ref p) => format!("\"{}\"", escape_typst_string_literal(p)),
            None => "none".to_string(),
        };
        let include_clerk = req.include_clerk_stamp.unwrap_or(false);
        let state_profile = req.effective_state_profile();
        let state_name = state_profile.map(|p| p.name).unwrap_or("Tennessee");
        let county_name = req.effective_county();
        let ron_citation = state_profile
            .map(|p| p.notary_ron_statute)
            .unwrap_or("T.C.A. § 8-16-301 et seq.");

        markup.push_str(&format!(
            "\n#v(8pt)\n#jurat-box(\n  signature-type: \"{}\",\n  notary-required: {},\n  signer-name: \"{}\",\n  signer-title: \"{}\",\n  signature-img: {},\n  execution-date: \"{}\",\n  esign-audit-id: \"PRESSMARK-ESIGN-8492-7A1B\",\n  esign-timestamp: \"2026-10-01T02:00:00Z\",\n  esign-ip: \"192.0.2.14\",\n  ron-statute-citation: \"{}\",\n  ron-session-id: \"RON-SESSION-2026-9941\",\n  include-clerk-stamp: {},\n  state: \"{}\",\n  county: \"{}\"\n)\n",
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

    // 11. Tamper-Evident Cryptographic Audit Slug (real SHA-256 hex digest)
    let audit_hash = req.compute_audit_sha256();
    let issue_date = req.issue_date.as_deref().unwrap_or("2026-10-01");
    markup.push_str(&format!(
        "\n#v(4pt)\n#audit-slug(parcel-id: \"{}\", hash: \"{}\", engine-version: \"Pressmark Vector Engine v1.0.0\", timestamp: \"{}T00:00:00Z\")\n",
        escape_typst_string_literal(&req.effective_doc_number()),
        escape_typst_string_literal(&audit_hash),
        escape_typst_string_literal(issue_date)
    ));

    markup
}

/// Execute end-to-end PDF generation for a `DocumentRequest`.
pub fn generate_pdf(req: &DocumentRequest) -> Result<(Vec<u8>, u128), String> {
    let start = Instant::now();
    let markup = generate_typst_markup(req);
    let pdf_bytes = compile_pdf_bytes(&markup)?;
    let elapsed = start.elapsed().as_millis();
    Ok((pdf_bytes, elapsed))
}

/// Execute end-to-end PNG preview generation for a `DocumentRequest` (Page 1).
pub fn generate_preview_png(req: &DocumentRequest, ppi: u32) -> Result<(Vec<u8>, u128), String> {
    let start = Instant::now();
    let markup = generate_typst_markup(req);
    let png_bytes = compile_png_bytes(&markup, ppi)?;
    let elapsed = start.elapsed().as_millis();
    Ok((png_bytes, elapsed))
}

/// Execute end-to-end multi-page PNG preview generation for a `DocumentRequest` (All Pages).
pub fn generate_all_preview_pngs(req: &DocumentRequest, ppi: u32) -> Result<(Vec<Vec<u8>>, u128), String> {
    let start = Instant::now();
    let markup = generate_typst_markup(req);
    let pages = compile_all_png_pages(&markup, ppi)?;
    let elapsed = start.elapsed().as_millis();
    Ok((pages, elapsed))
}
