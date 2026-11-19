//! Pressmark Vector Typst Synthesis Engine
//!
//! Author: Daniel Hallman
//!
//! Executes in-memory Typst PDF and PNG preview compilation via stdin/stdout streaming.
//! Zero headless Chrome, zero ReportLab, zero WeasyPrint overhead.

use crate::models::{DocumentRequest, DocumentType};
use crate::templates::TYPST_THEME;
use std::time::Instant;

#[path = "engine_archetypes.rs"]
mod engine_archetypes;
#[path = "engine_compiler.rs"]
mod engine_compiler;
pub use engine_compiler::*;

pub(super) fn format_party_typst(party: &crate::models::PartyDetails) -> String {
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

pub(super) fn open_styled_table(
    columns_spec: &str,
    align_spec: &str,
    headers: &[&str],
    req: &DocumentRequest,
    primary_hex: &str,
    secondary_hex: &str,
    surface_hex: &str,
    default_inset: &str,
) -> String {
    let table_style = req.effective_table_style();
    let header_cells: Vec<String> = headers
        .iter()
        .map(|h| match table_style {
            "modern_cards" => format!("[#text(fill: white, weight: \"bold\")[{}]]", h),
            "minimal_lines" => format!(
                "[#text(fill: rgb(\"{}\"), weight: \"bold\")[{}]]",
                primary_hex, h
            ),
            "compact_mono" => format!(
                "[#text(font: font-mono, size: 7.5pt, weight: \"bold\", fill: rgb(\"{}\"))[{}]]",
                primary_hex, h
            ),
            _ => format!("[*{}*]", h),
        })
        .collect();
    let header_joined = header_cells.join(", ");

    match table_style {
        "minimal_lines" => format!(
            "#table(\n  columns: {},\n  stroke: (col, row) => if row == 0 {{ (bottom: 1.2pt + rgb(\"{}\")) }} else {{ (bottom: 0.4pt + rgb(\"{}\").lighten(70%)) }},\n  fill: none,\n  inset: (x: 6pt, y: 6.5pt),\n  align: {},\n  table.header({}),\n",
            columns_spec, primary_hex, secondary_hex, align_spec, header_joined
        ),
        "bordered_ledger" => format!(
            "#table(\n  columns: {},\n  stroke: (col, row) => if row == 0 {{ 0.85pt + rgb(\"{}\") }} else {{ 0.55pt + color-slate-400 }},\n  fill: (col, row) => if row == 0 {{ color-slate-100 }} else {{ white }},\n  inset: (x: 7.5pt, y: 6pt),\n  align: {},\n  table.header({}),\n",
            columns_spec, primary_hex, align_spec, header_joined
        ),
        "modern_cards" => format!(
            "#table(\n  columns: {},\n  stroke: (col, row) => if row == 0 {{ none }} else {{ (bottom: 0.5pt + rgb(\"{}\").lighten(72%)) }},\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\") }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: {},\n  align: {},\n  table.header({}),\n",
            columns_spec, secondary_hex, primary_hex, surface_hex, default_inset, align_spec, header_joined
        ),
        "compact_mono" => format!(
            "#table(\n  columns: {},\n  stroke: 0.45pt + rgb(\"{}\").lighten(60%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\") }} else {{ white }},\n  inset: (x: 5.5pt, y: 4.5pt),\n  align: {},\n  table.header({}),\n",
            columns_spec, secondary_hex, surface_hex, align_spec, header_joined
        ),
        _ => format!(
            "#table(\n  columns: {},\n  stroke: 0.5pt + rgb(\"{}\").lighten(65%),\n  fill: (col, row) => if row == 0 {{ rgb(\"{}\").lighten(90%) }} else if calc.even(row) {{ rgb(\"{}\") }} else {{ white }},\n  inset: {},\n  align: {},\n  table.header({}),\n",
            columns_spec, secondary_hex, secondary_hex, surface_hex, default_inset, align_spec, header_joined
        ),
    }
}

pub(super) fn render_totals_box(markup: &mut String, req: &DocumentRequest, primary_hex: &str, accent_hex: &str, surface_hex: &str, total_label: &str) {
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
    let totals_style = req.effective_totals_style();

    let mut rows_markup = String::new();
    if let Some((prior_rent, net_increase, _new_rent)) = req.rent_increase_breakdown() {
        let pct = if prior_rent > 0.0 {
            (net_increase / prior_rent) * 100.0
        } else {
            0.0
        };
        rows_markup.push_str(&format!(
            "    [#text(fill: color-slate-500)[Prior Monthly Rent:]], [#text(weight: \"bold\")[{}]],\n",
            req.format_amount_typst(prior_rent)
        ));
        rows_markup.push_str(&format!(
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
        rows_markup.push_str(&format!(
            "    [#text(fill: color-slate-500)[Subtotal:]], [#text(weight: \"bold\")[{}]],\n",
            req.format_amount_typst(subtotal)
        ));
    }
    if let Some(rate) = req.tax_rate {
        if rate > 0.0 {
            rows_markup.push_str(&format!(
                "    [#text(fill: color-slate-500)[Tax ({:.1}%):]], [#text(weight: \"bold\")[{}]],\n",
                rate, req.format_amount_typst(tax_total)
            ));
        }
    }
    if discount > 0.0 {
        rows_markup.push_str(&format!(
            "    [#text(fill: color-slate-500)[Discount / Credits:]], [#text(weight: \"bold\", fill: rgb(\"{}\"))[−{}]],\n",
            accent_hex, req.format_amount_typst(discount)
        ));
    }

    let mut paid_rows_markup = String::new();
    if req.is_paid_state() || req.amount_paid.is_some() {
        let paid = req.amount_paid_effective();
        let balance = req.balance_due();
        paid_rows_markup.push_str(&format!(
            "    [#text(size: 8.5pt, fill: rgb(\"#059669\"))[Amount Paid:]], [#text(size: 8.5pt, weight: \"bold\", fill: rgb(\"#059669\"))[{}]],\n",
            req.format_amount_typst(paid)
        ));
        let bal_color = if balance <= 0.001 { "#059669" } else { "#dc2626" };
        paid_rows_markup.push_str(&format!(
            "    [#text(size: 9pt, weight: 900, fill: rgb(\"{}\"))[Balance Due:]], [#text(size: 9pt, weight: 900, fill: rgb(\"{}\"))[{}]],\n",
            bal_color, bal_color, req.format_amount_typst(balance)
        ));
    }

    let table_markup = match totals_style {
        "card_highlight" => format!(
            "#box(width: 310pt)[\n  #rect(width: 100%, fill: rgb(\"{surface}\"), stroke: 0.75pt + rgb(\"{primary}\").lighten(45%), radius: 5pt, inset: (x: 10pt, y: 7pt))[\n    #table(\n      columns: (1fr, auto),\n      stroke: none,\n      inset: (x: 3pt, y: 3.5pt),\n      align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n{rows}      table.hline(stroke: 1pt + rgb(\"{primary}\")),\n      [#text(size: 10.5pt, weight: 900, fill: rgb(\"{primary}\"))[{label}]], [#text(size: 11.5pt, weight: 900, fill: rgb(\"{primary}\"))[{total}]],\n{paid}    )\n  ]\n]",
            surface = surface_hex,
            primary = primary_hex,
            rows = rows_markup,
            label = total_label,
            total = req.format_amount_typst(grand_total),
            paid = paid_rows_markup
        ),
        "banner_bar" => format!(
            "#box(width: 310pt)[\n  #table(\n    columns: (1fr, auto),\n    stroke: none,\n    inset: (x: 4pt, y: 3.5pt),\n    align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n{rows}    table.cell(colspan: 2, fill: rgb(\"{primary}\"), inset: (x: 8pt, y: 6pt))[\n      #grid(\n        columns: (1fr, auto),\n        align: (left + horizon, right + horizon),\n        text(size: 10pt, weight: 900, fill: white)[{label}],\n        text(size: 11.5pt, weight: 900, fill: white)[{total}]\n      )\n    ],\n{paid}  )\n]",
            primary = primary_hex,
            rows = rows_markup,
            label = total_label,
            total = req.format_amount_typst(grand_total),
            paid = paid_rows_markup
        ),
        "compact" => format!(
            "#box(width: 310pt)[\n  #rect(width: 100%, fill: rgb(\"{surface}\"), stroke: (top: 0.75pt + rgb(\"{primary}\"), bottom: 0.75pt + rgb(\"{primary}\")), inset: (x: 8pt, y: 5pt))[\n    #table(\n      columns: (1fr, auto),\n      stroke: none,\n      inset: (x: 2pt, y: 2.5pt),\n      align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n{rows}      [#text(size: 10pt, weight: 900, fill: rgb(\"{primary}\"))[{label}]], [#text(size: 10.5pt, weight: 900, fill: rgb(\"{primary}\"))[{total}]],\n{paid}    )\n  ]\n]",
            surface = surface_hex,
            primary = primary_hex,
            rows = rows_markup,
            label = total_label,
            total = req.format_amount_typst(grand_total),
            paid = paid_rows_markup
        ),
        "split_ledger" => format!(
            "#box(width: 310pt)[\n  #table(\n    columns: (1fr, auto),\n    stroke: 0.5pt + color-slate-300,\n    fill: (col, row) => if col == 0 {{ rgb(\"{surface}\") }} else {{ white }},\n    inset: (x: 7pt, y: 4.5pt),\n    align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n{rows}    [#text(size: 10pt, weight: 900, fill: rgb(\"{primary}\"))[{label}]], [#text(size: 11pt, weight: 900, fill: rgb(\"{primary}\"))[{total}]],\n{paid}  )\n]",
            surface = surface_hex,
            primary = primary_hex,
            rows = rows_markup,
            label = total_label,
            total = req.format_amount_typst(grand_total),
            paid = paid_rows_markup
        ),
        _ => format!(
            "#box(width: 310pt)[\n  #table(\n    columns: (1fr, auto),\n    stroke: none,\n    inset: (x: 4pt, y: 3.5pt),\n    align: (col, row) => if col == 0 {{ left + horizon }} else {{ right + horizon }},\n{rows}    table.hline(stroke: 1pt + rgb(\"{primary}\")),\n    [#text(size: 10.5pt, weight: 900, fill: rgb(\"{primary}\"))[{label}]], [#text(size: 11.5pt, weight: 900, fill: rgb(\"{primary}\"))[{total}]],\n{paid}  )\n]",
            primary = primary_hex,
            rows = rows_markup,
            label = total_label,
            total = req.format_amount_typst(grand_total),
            paid = paid_rows_markup
        ),
    };

    // 1. Paid in full attestation watermark stamp (if marked as paid, placed alongside Totals box for zero vertical collision)
    if req.is_paid_state() {
        let method = req.payment_method.as_deref().unwrap_or_default();
        let paid_date = req.paid_at.as_deref().unwrap_or_default();
        let tx_ref = req.transaction_reference.as_deref().unwrap_or_default();
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
        && non_empty_payment_link.is_some()
    {
        let raw_url = non_empty_payment_link.expect("payment link checked above");
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
        let payment_method = req.payment_method.as_deref().unwrap_or_default();
        let due_date_arg = match req.due_date.as_deref() {
            Some(d) => format!("\"{}\"", escape_typst_string_literal(d)),
            None => "none".to_string(),
        };
        let effective_amount_due = req.format_amount(req.balance_due());
        let payment_style = req.effective_payment_style();

        markup.push_str(&format!(
            "#payment-qr-block(\n  qr-path: {},\n  payment-url: \"{}\",\n  payment-status: \"{}\",\n  amount-due: \"{}\",\n  due-date: {},\n  payment-method: \"{}\",\n  style: \"{}\",\n  primary-color: rgb(\"{}\"),\n  accent-color: rgb(\"{}\"),\n  surface-color: rgb(\"{}\")\n)\n\n",
            qr_arg,
            escape_typst_string_literal(&payment_url),
            escape_typst_string_literal(payment_status),
            escape_typst_string_literal(&effective_amount_due),
            due_date_arg,
            escape_typst_string_literal(payment_method),
            escape_typst_string_literal(payment_style),
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

    let footer_text = req.footer_text.as_deref().map(escape_typst_text).unwrap_or_default();

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

    let issue_date = req.issue_date.as_deref().unwrap_or_default();
    let doc_num = req.effective_doc_number();
    let doc_title = req.effective_title();

    markup.push_str(&format!(
        "#set page(\n  paper: \"{}\",\n  fill: rgb(\"{}\"),\n  margin: {},\n  header: context {{\n    if counter(page).get().first() > 1 [\n      #grid(\n        columns: (1fr, 1fr),\n        text(font: {}, size: 7.5pt, fill: rgb(\"#94a3b8\"), weight: \"bold\")[{}],\n        align(right, text(font: {}, size: 7.5pt, fill: rgb(\"#94a3b8\"), weight: \"bold\")[\n          REF: {} · DATE: {}\n        ])\n      )\n      #v(-4pt)\n      #line(length: 100%, stroke: 0.5pt + rgb(\"#e2e8f0\"))\n    ]\n  }},\n  footer: context [\n    {}#grid(\n      columns: (1fr, auto),\n      text(size: 7.5pt, fill: rgb(\"#94a3b8\"))[{}],\n      {}\n    )\n  ]{}\n)\n",
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
        "PREPARED"
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
            .unwrap_or_default();
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
            .unwrap_or_default();

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
        escape_typst_string_literal(&req.format_date(req.issue_date.as_deref().unwrap_or_default())),
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

    engine_archetypes::render_parties_disclosures_and_body(
        &mut markup,
        req,
        primary_hex,
        secondary_hex,
        accent_hex,
        surface_hex,
        company_name,
    );

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
