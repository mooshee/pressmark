//! Pressmark Interactive Statutory & Financial Calculators with 1-Click PDF Export
//!
//! Author: Daniel Hallman

use crate::seo_pages::{html_escape, render_seo_shell};
use crate::state_laws::all_state_lease_profiles;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct CalcQuery {
    pub state: Option<String>,
}

struct CalcInfo {
    slug: &'static str,
    title: &'static str,
    short_desc: &'static str,
    seo_title: &'static str,
    seo_desc: &'static str,
}

const CALCULATORS: &[CalcInfo] = &[
    CalcInfo {
        slug: "security-deposit-return",
        title: "Security Deposit Return & Deadline Calculator (50 States + DC)",
        short_desc: "Check statutory return deadlines, state deposit caps, and itemized deductions, then download a statutory disposition PDF.",
        seo_title: "Security Deposit Return Calculator (50 States + DC) · Pressmark",
        seo_desc: "Calculate your state's statutory security deposit return deadline, deposit cap, itemized repair deductions, and export an official disposition PDF.",
    },
    CalcInfo {
        slug: "rent-increase-notice",
        title: "Rent Increase Percentage & Notice Period Calculator",
        short_desc: "Compute the exact percentage increase, check state rent stabilization notes, and generate a state rent increase notice PDF.",
        seo_title: "Rent Increase Percentage & State Notice Calculator · Pressmark",
        seo_desc: "Compute rent increase percentages, review 50-state notice rules and rent control limits, and generate a rent increase notice PDF.",
    },
    CalcInfo {
        slug: "commercial-nnn-lease",
        title: "Commercial Triple-Net (NNN) Lease Cost Calculator",
        short_desc: "Calculate monthly base rent, CAM, property taxes, insurance pass-throughs per SqFt, and export a Commercial NNN Lease PDF.",
        seo_title: "Commercial Triple-Net (NNN) Lease Calculator · Pressmark",
        seo_desc: "Calculate monthly and annual commercial Triple-Net (NNN) occupancy cost from base rent, CAM, property taxes, and insurance per SqFt.",
    },
    CalcInfo {
        slug: "late-fee-invoice",
        title: "Invoice Late Fee, Due Date & Tax Calculator",
        short_desc: "Compute exact invoice totals with discounts, tax, Net payment due date, overdue late fees, and export an updated invoice PDF.",
        seo_title: "Invoice Late Fee, Due Date & Tax Calculator · Pressmark",
        seo_desc: "Calculate invoice subtotals, discounts, sales/VAT tax, net payment due dates, and late fees, then download a vector PDF invoice.",
    },
];

pub async fn handle_calculators_hub() -> impl IntoResponse {
    let cards: String = CALCULATORS
        .iter()
        .map(|c| {
            format!(
                "<a class=\"pm-card\" href=\"/calculators/{slug}\" style=\"color:inherit;\">\
                 <span class=\"pm-badge-ok\">Free Tool · PDF Export</span>\
                 <h3>{title}</h3>\
                 <p style=\"margin:0;color:var(--muted);font-size:14px;\">{desc}</p>\
                 <span style=\"font-size:13px;font-weight:600;color:var(--brand);\">Open Calculator →</span>\
                 </a>",
                slug = c.slug,
                title = html_escape(c.title),
                desc = html_escape(c.short_desc)
            )
        })
        .collect();

    let body = format!(
        r#"<section class="pm-hero">
      <span class="pm-eyebrow">Statutory &amp; Financial Calculators</span>
      <h1>Free Lease, Deposit &amp; Billing Calculators</h1>
      <p class="pm-lead">Calculate security deposit returns, rent increases, commercial NNN pass-throughs, and invoice late fees in your browser, then export a vector PDF.</p>
    </section>
    <div class="pm-grid-2">{cards}</div>"#
    );

    Html(render_seo_shell(
        "Free Lease, Deposit & Invoice Calculators · Pressmark",
        "Free 50-state security deposit return, rent increase notice, commercial NNN lease, and invoice late fee calculators with direct PDF export.",
        "https://pressmark.dev/calculators",
        "calculators",
        "",
        &body,
    ))
}

fn state_options_and_json(selected_code: &str) -> (String, String) {
    let profiles = all_state_lease_profiles();
    let options: String = profiles
        .iter()
        .map(|sp| {
            let sel = if sp.code.eq_ignore_ascii_case(selected_code) {
                " selected"
            } else {
                ""
            };
            format!(
                "<option value=\"{code}\"{sel}>{name} ({code})</option>",
                code = sp.code,
                sel = sel,
                name = html_escape(sp.name)
            )
        })
        .collect();

    let compact: Vec<serde_json::Value> = profiles
        .iter()
        .map(|sp| {
            serde_json::json!({
                "code": sp.code,
                "name": sp.name,
                "slug": sp.slug,
                "statute": sp.governing_statute,
                "deposit_cap": sp.deposit_cap,
                "return_deadline": sp.return_deadline,
                "grace_period": sp.grace_period,
                "late_fee_cap": sp.late_fee_cap,
                "rent_control": sp.rent_control_note,
                "pay_or_quit": sp.pay_or_quit_notice
            })
        })
        .collect();

    (
        options,
        serde_json::to_string(&compact).unwrap_or_else(|_| "[]".to_string()),
    )
}

fn render_deposit_calculator(selected_state: &str) -> String {
    let (state_opts, states_json) = state_options_and_json(selected_state);
    format!(
        r#"<div class="pm-grid-2">
      <section class="pm-card">
        <h2>Deposit &amp; Deductions</h2>
        <div class="pm-form-grid">
          <div class="pm-field"><label for="depState">State / Jurisdiction</label><select id="depState">{state_opts}</select></div>
          <div class="pm-field"><label for="depMoveOut">Move-Out Date</label><input id="depMoveOut" type="date"></div>
          <div class="pm-field"><label for="depHeld">Deposit Held ($)</label><input id="depHeld" type="number" step="0.01" value="2500"></div>
          <div class="pm-field"><label for="depDamages">Cleaning &amp; Damage Repairs ($)</label><input id="depDamages" type="number" step="0.01" value="325"></div>
          <div class="pm-field"><label for="depUnpaid">Unpaid Rent / Utilities ($)</label><input id="depUnpaid" type="number" step="0.01" value="0"></div>
          <div class="pm-field"><label for="depTenant">Tenant Name (for PDF)</label><input id="depTenant" value="Jordan Avery"></div>
          <div class="pm-field" style="grid-column:1 / -1;"><label for="depLandlord">Landlord / Property Manager Name</label><input id="depLandlord" value="Harborview Properties LLC"></div>
        </div>
        <div class="pm-cta-row" style="margin-top:8px;">
          <button type="button" id="btnExportDepositPdf" class="pm-btn pm-btn-primary">Download Disposition PDF</button>
          <a id="depStateTemplateLink" href="/templates/security-deposit-disposition?studio=1" class="pm-btn pm-btn-secondary">Open in Full Editor</a>
        </div>
        <span id="depStatus" style="font-size:13px;color:var(--muted);"></span>
      </section>
      <section class="pm-card">
        <h2>Result &amp; Statutory Rules</h2>
        <div style="font-size:28px;font-weight:700;" id="depRefundResult">$2,175.00 Refund Due</div>
        <div class="pm-table-wrap">
          <table>
            <tbody>
              <tr><th>Governing Statute</th><td id="depRuleStatute"></td></tr>
              <tr><th>Statutory Deposit Cap</th><td id="depRuleCap"></td></tr>
              <tr><th>Statutory Return Window</th><td id="depRuleDeadline"></td></tr>
              <tr><th>Estimated Postmark Due Date</th><td id="depDueDateResult">Select move-out date</td></tr>
            </tbody>
          </table>
        </div>
      </section>
    </div>
    <script>
      (function(){{
        var states = {states_json};
        var stateEl = document.getElementById('depState');
        var moveOutEl = document.getElementById('depMoveOut');
        if (!moveOutEl.value) moveOutEl.value = new Date().toISOString().split('T')[0];
        function fmt(n) {{ return '$' + Number(n || 0).toLocaleString('en-US', {{minimumFractionDigits:2, maximumFractionDigits:2}}); }}
        function recalc() {{
          var sp = states.find(function(s){{ return s.code === stateEl.value; }}) || states[0];
          var held = parseFloat(document.getElementById('depHeld').value) || 0;
          var dmg = parseFloat(document.getElementById('depDamages').value) || 0;
          var unp = parseFloat(document.getElementById('depUnpaid').value) || 0;
          var net = held - dmg - unp;
          document.getElementById('depRefundResult').textContent = net >= 0
            ? fmt(net) + ' Refund Due to Tenant'
            : fmt(Math.abs(net)) + ' Balance Owed by Tenant';
          document.getElementById('depRuleStatute').textContent = sp.statute;
          document.getElementById('depRuleCap').textContent = sp.deposit_cap;
          document.getElementById('depRuleDeadline').textContent = sp.return_deadline;
          document.getElementById('depStateTemplateLink').href = '/templates/' + sp.slug + '-security-deposit-disposition?studio=1';
          var m = (sp.return_deadline || '').match(/(\d+)\s*(business\s*)?days?/i);
          if (m && moveOutEl.value) {{
            var days = parseInt(m[1], 10);
            var dt = new Date(moveOutEl.value + 'T00:00:00');
            dt.setDate(dt.getDate() + days);
            document.getElementById('depDueDateResult').textContent = dt.toISOString().split('T')[0] + ' (' + days + ' days after move-out)';
          }} else {{
            document.getElementById('depDueDateResult').textContent = sp.return_deadline;
          }}
        }}
        ['depState','depMoveOut','depHeld','depDamages','depUnpaid'].forEach(function(id){{
          document.getElementById(id).addEventListener('input', recalc);
          document.getElementById(id).addEventListener('change', recalc);
        }});
        recalc();
        document.getElementById('btnExportDepositPdf').addEventListener('click', async function(){{
          var sp = states.find(function(s){{ return s.code === stateEl.value; }}) || states[0];
          var held = parseFloat(document.getElementById('depHeld').value) || 0;
          var dmg = parseFloat(document.getElementById('depDamages').value) || 0;
          var unp = parseFloat(document.getElementById('depUnpaid').value) || 0;
          var st = document.getElementById('depStatus');
          st.textContent = 'Generating PDF...';
          var items = [{{ description: 'Original Security Deposit Held', quantity: 1, unit_price: held }}];
          if (dmg > 0) items.push({{ description: 'Less Itemized Cleaning & Repair Deductions', quantity: 1, unit_price: -dmg }});
          if (unp > 0) items.push({{ description: 'Less Unpaid Rent / Utilities', quantity: 1, unit_price: -unp }});
          var body = {{
            doc_type: 'security_deposit_disposition',
            state_code: sp.code,
            from: {{ name: document.getElementById('depLandlord').value || 'Landlord' }},
            to: {{ name: document.getElementById('depTenant').value || 'Tenant' }},
            items: items,
            custom_fields: [
              {{ label: 'Move-Out Date', value: moveOutEl.value }},
              {{ label: 'Statutory Return Window', value: sp.return_deadline }}
            ]
          }};
          var r = await fetch('/api/v1/generate?format=pdf', {{ method:'POST', headers:{{'Content-Type':'application/json'}}, body: JSON.stringify(body) }});
          if (!r.ok) {{ st.textContent = 'Failed to generate PDF.'; return; }}
          var blob = await r.blob();
          var u = URL.createObjectURL(blob);
          var a = document.createElement('a'); a.href = u; a.download = sp.code + '-deposit-disposition.pdf';
          document.body.appendChild(a); a.click(); URL.revokeObjectURL(u); a.remove();
          st.textContent = 'PDF downloaded.';
          if (typeof gtag === 'function') gtag('event', 'calculator_used', {{ calculator: 'security-deposit-return', state: sp.code }});
        }});
      }})();
    </script>"#
    )
}

fn render_rent_increase_calculator(selected_state: &str) -> String {
    let (state_opts, states_json) = state_options_and_json(selected_state);
    format!(
        r#"<div class="pm-grid-2">
      <section class="pm-card">
        <h2>Rent Adjustment Inputs</h2>
        <div class="pm-form-grid">
          <div class="pm-field"><label for="riState">State / Jurisdiction</label><select id="riState">{state_opts}</select></div>
          <div class="pm-field"><label for="riDate">Effective Date</label><input id="riDate" type="date"></div>
          <div class="pm-field"><label for="riCurrent">Current Monthly Rent ($)</label><input id="riCurrent" type="number" step="0.01" value="2200"></div>
          <div class="pm-field"><label for="riNew">Proposed Monthly Rent ($)</label><input id="riNew" type="number" step="0.01" value="2350"></div>
          <div class="pm-field"><label for="riLandlord">Landlord Name</label><input id="riLandlord" value="Harborview Properties LLC"></div>
          <div class="pm-field"><label for="riTenant">Tenant Name</label><input id="riTenant" value="Jordan Avery"></div>
        </div>
        <div class="pm-cta-row" style="margin-top:8px;">
          <button type="button" id="btnExportRentPdf" class="pm-btn pm-btn-primary">Download Rent Increase PDF</button>
          <a id="riStateTemplateLink" href="/templates/rent-increase-notice?studio=1" class="pm-btn pm-btn-secondary">Open in Full Editor</a>
        </div>
        <span id="riStatus" style="font-size:13px;color:var(--muted);"></span>
      </section>
      <section class="pm-card">
        <h2>Adjustment Summary &amp; Rules</h2>
        <div style="font-size:28px;font-weight:700;" id="riSummary">+$150.00 / mo (+6.82%)</div>
        <div class="pm-table-wrap">
          <table>
            <tbody>
              <tr><th>Annualized Difference</th><td id="riAnnual">+$1,800.00 / yr</td></tr>
              <tr><th>Governing Statute</th><td id="riStatute"></td></tr>
              <tr><th>State Rent Regulation</th><td id="riRentControl"></td></tr>
            </tbody>
          </table>
        </div>
      </section>
    </div>
    <script>
      (function(){{
        var states = {states_json};
        var stateEl = document.getElementById('riState');
        var effEl = document.getElementById('riDate');
        if (!effEl.value) {{ var d = new Date(); d.setDate(d.getDate() + 45); effEl.value = d.toISOString().split('T')[0]; }}
        function recalc() {{
          var sp = states.find(function(s){{ return s.code === stateEl.value; }}) || states[0];
          var cur = parseFloat(document.getElementById('riCurrent').value) || 0;
          var nxt = parseFloat(document.getElementById('riNew').value) || 0;
          var diff = nxt - cur;
          var pct = cur > 0 ? (diff / cur) * 100 : 0;
          document.getElementById('riSummary').textContent = (diff >= 0 ? '+' : '-') + '$' + Math.abs(diff).toFixed(2) + ' / mo (' + (pct >= 0 ? '+' : '') + pct.toFixed(2) + '%)';
          document.getElementById('riAnnual').textContent = (diff >= 0 ? '+' : '-') + '$' + (Math.abs(diff) * 12).toFixed(2) + ' / yr';
          document.getElementById('riStatute').textContent = sp.statute;
          document.getElementById('riRentControl').textContent = sp.rent_control;
          document.getElementById('riStateTemplateLink').href = '/templates/' + sp.slug + '-rent-increase-notice?studio=1';
        }}
        ['riState','riCurrent','riNew'].forEach(function(id){{
          document.getElementById(id).addEventListener('input', recalc);
          document.getElementById(id).addEventListener('change', recalc);
        }});
        recalc();
        document.getElementById('btnExportRentPdf').addEventListener('click', async function(){{
          var sp = states.find(function(s){{ return s.code === stateEl.value; }}) || states[0];
          var cur = parseFloat(document.getElementById('riCurrent').value) || 0;
          var nxt = parseFloat(document.getElementById('riNew').value) || 0;
          var st = document.getElementById('riStatus');
          st.textContent = 'Generating PDF...';
          var body = {{
            doc_type: 'rent_increase_notice',
            state_code: sp.code,
            from: {{ name: document.getElementById('riLandlord').value || 'Landlord' }},
            to: {{ name: document.getElementById('riTenant').value || 'Tenant' }},
            items: [
              {{ description: 'Prior Monthly Base Rent', quantity: 1, unit_price: cur }},
              {{ description: 'New Adjusted Monthly Base Rent (Effective ' + effEl.value + ')', quantity: 1, unit_price: nxt }}
            ]
          }};
          var r = await fetch('/api/v1/generate?format=pdf', {{ method:'POST', headers:{{'Content-Type':'application/json'}}, body: JSON.stringify(body) }});
          if (!r.ok) {{ st.textContent = 'Failed to generate PDF.'; return; }}
          var blob = await r.blob();
          var u = URL.createObjectURL(blob);
          var a = document.createElement('a'); a.href = u; a.download = sp.code + '-rent-increase-notice.pdf';
          document.body.appendChild(a); a.click(); URL.revokeObjectURL(u); a.remove();
          st.textContent = 'PDF downloaded.';
          if (typeof gtag === 'function') gtag('event', 'calculator_used', {{ calculator: 'rent-increase-notice', state: sp.code }});
        }});
      }})();
    </script>"#
    )
}

fn render_nnn_calculator() -> String {
    r#"<div class="pm-grid-2">
      <section class="pm-card">
        <h2>Premises &amp; NNN Inputs</h2>
        <div class="pm-form-grid">
          <div class="pm-field"><label for="nnnSqft">Rentable Area (SqFt)</label><input id="nnnSqft" type="number" value="3200"></div>
          <div class="pm-field"><label for="nnnBase">Annual Base Rent ($ / SqFt)</label><input id="nnnBase" type="number" step="0.01" value="28.00"></div>
          <div class="pm-field"><label for="nnnCam">CAM ($ / SqFt / yr)</label><input id="nnnCam" type="number" step="0.01" value="5.50"></div>
          <div class="pm-field"><label for="nnnTax">Property Tax ($ / SqFt / yr)</label><input id="nnnTax" type="number" step="0.01" value="4.25"></div>
          <div class="pm-field"><label for="nnnIns">Insurance ($ / SqFt / yr)</label><input id="nnnIns" type="number" step="0.01" value="1.75"></div>
          <div class="pm-field"><label for="nnnTenant">Tenant Company</label><input id="nnnTenant" value="Summit Retail Partners LLC"></div>
        </div>
        <div class="pm-cta-row" style="margin-top:8px;">
          <button type="button" id="btnExportNnnPdf" class="pm-btn pm-btn-primary">Download NNN Lease PDF</button>
          <a href="/templates/commercial-nnn-lease?studio=1" class="pm-btn pm-btn-secondary">Open in Full Editor</a>
        </div>
        <span id="nnnStatus" style="font-size:13px;color:var(--muted);"></span>
      </section>
      <section class="pm-card">
        <h2>Monthly &amp; Annual Occupancy Cost</h2>
        <div style="font-size:28px;font-weight:700;" id="nnnMonthlyTotal">$10,533.33 / mo</div>
        <div class="pm-table-wrap">
          <table>
            <tbody>
              <tr><th>Monthly Base Rent</th><td id="nnnMonthlyBase"></td></tr>
              <tr><th>Monthly CAM + Tax + Ins (NNN)</th><td id="nnnMonthlyPass"></td></tr>
              <tr><th>All-In Rate ($ / SqFt / yr)</th><td id="nnnAllInRate"></td></tr>
              <tr><th>Total Annual Lease Cost</th><td id="nnnAnnualTotal"></td></tr>
            </tbody>
          </table>
        </div>
      </section>
    </div>
    <script>
      (function(){
        function fmt(n) { return '$' + Number(n || 0).toLocaleString('en-US', {minimumFractionDigits:2, maximumFractionDigits:2}); }
        function vals() {
          var sqft = parseFloat(document.getElementById('nnnSqft').value) || 0;
          var base = parseFloat(document.getElementById('nnnBase').value) || 0;
          var cam = parseFloat(document.getElementById('nnnCam').value) || 0;
          var tax = parseFloat(document.getElementById('nnnTax').value) || 0;
          var ins = parseFloat(document.getElementById('nnnIns').value) || 0;
          return { sqft: sqft, base: base, cam: cam, tax: tax, ins: ins, moBase: (sqft * base)/12, moNnn: (sqft * (cam+tax+ins))/12 };
        }
        function recalc() {
          var v = vals();
          var moTotal = v.moBase + v.moNnn;
          document.getElementById('nnnMonthlyTotal').textContent = fmt(moTotal) + ' / mo';
          document.getElementById('nnnMonthlyBase').textContent = fmt(v.moBase);
          document.getElementById('nnnMonthlyPass').textContent = fmt(v.moNnn);
          document.getElementById('nnnAllInRate').textContent = '$' + (v.base + v.cam + v.tax + v.ins).toFixed(2) + ' / SqFt / yr';
          document.getElementById('nnnAnnualTotal').textContent = fmt(moTotal * 12) + ' / yr';
        }
        ['nnnSqft','nnnBase','nnnCam','nnnTax','nnnIns'].forEach(function(id){ document.getElementById(id).addEventListener('input', recalc); });
        recalc();
        document.getElementById('btnExportNnnPdf').addEventListener('click', async function(){
          var v = vals();
          var st = document.getElementById('nnnStatus');
          st.textContent = 'Generating PDF...';
          var body = {
            doc_type: 'commercial_nnn_lease',
            from: { name: 'Commercial Property Holdings LLC' },
            to: { name: document.getElementById('nnnTenant').value || 'Tenant Corp' },
            items: [
              { description: 'Monthly Base Rent (' + v.sqft + ' SqFt @ $' + v.base.toFixed(2) + '/SqFt/yr)', quantity: 1, unit_price: Math.round(v.moBase * 100)/100 },
              { description: 'Estimated CAM, Property Tax & Insurance (NNN Pass-Through)', quantity: 1, unit_price: Math.round(v.moNnn * 100)/100 }
            ]
          };
          var r = await fetch('/api/v1/generate?format=pdf', { method:'POST', headers:{'Content-Type':'application/json'}, body: JSON.stringify(body) });
          if (!r.ok) { st.textContent = 'Failed to generate PDF.'; return; }
          var blob = await r.blob();
          var u = URL.createObjectURL(blob);
          var a = document.createElement('a'); a.href = u; a.download = 'commercial-nnn-lease.pdf';
          document.body.appendChild(a); a.click(); URL.revokeObjectURL(u); a.remove();
          st.textContent = 'PDF downloaded.';
          if (typeof gtag === 'function') gtag('event', 'calculator_used', { calculator: 'commercial-nnn-lease' });
        });
      })();
    </script>"#.to_string()
}

fn render_late_fee_calculator() -> String {
    r#"<div class="pm-grid-2">
      <section class="pm-card">
        <h2>Invoice &amp; Late Fee Inputs</h2>
        <div class="pm-form-grid">
          <div class="pm-field"><label for="lfSub">Invoice Subtotal ($)</label><input id="lfSub" type="number" step="0.01" value="2500"></div>
          <div class="pm-field"><label for="lfTax">Tax Rate (%)</label><input id="lfTax" type="number" step="0.01" value="8.25"></div>
          <div class="pm-field"><label for="lfPct">Monthly Late Fee (%)</label><input id="lfPct" type="number" step="0.1" value="1.5"></div>
          <div class="pm-field"><label for="lfDays">Days Overdue (0 if on time)</label><input id="lfDays" type="number" value="15"></div>
          <div class="pm-field"><label for="lfNet">Net Payment Terms (Days)</label><input id="lfNet" type="number" value="14"></div>
          <div class="pm-field"><label for="lfClient">Client Name</label><input id="lfClient" value="Acme Corp"></div>
        </div>
        <div class="pm-cta-row" style="margin-top:8px;">
          <button type="button" id="btnExportLfPdf" class="pm-btn pm-btn-primary">Download Updated Invoice PDF</button>
          <a href="/templates/invoice?studio=1" class="pm-btn pm-btn-secondary">Open in Full Editor</a>
        </div>
        <span id="lfStatus" style="font-size:13px;color:var(--muted);"></span>
      </section>
      <section class="pm-card">
        <h2>Calculated Total &amp; Fee</h2>
        <div style="font-size:28px;font-weight:700;" id="lfGrandTotal">$2,726.55 Total Due</div>
        <div class="pm-table-wrap">
          <table>
            <tbody>
              <tr><th>Tax Amount</th><td id="lfTaxAmt"></td></tr>
              <tr><th>Prorated Late Fee</th><td id="lfFeeAmt"></td></tr>
              <tr><th>Net Due Date (from today)</th><td id="lfDueDate"></td></tr>
            </tbody>
          </table>
        </div>
      </section>
    </div>
    <script>
      (function(){
        function fmt(n) { return '$' + Number(n || 0).toLocaleString('en-US', {minimumFractionDigits:2, maximumFractionDigits:2}); }
        function calc() {
          var sub = parseFloat(document.getElementById('lfSub').value) || 0;
          var taxRate = parseFloat(document.getElementById('lfTax').value) || 0;
          var pct = parseFloat(document.getElementById('lfPct').value) || 0;
          var days = Math.max(0, parseFloat(document.getElementById('lfDays').value) || 0);
          var net = Math.max(0, parseInt(document.getElementById('lfNet').value, 10) || 14);
          var tax = sub * (taxRate / 100);
          var fee = (sub + tax) * (pct / 100) * (days / 30);
          var due = new Date(); due.setDate(due.getDate() + net);
          return { sub: sub, taxRate: taxRate, tax: tax, fee: fee, total: sub + tax + fee, dueStr: due.toISOString().split('T')[0], days: days };
        }
        function recalc() {
          var c = calc();
          document.getElementById('lfGrandTotal').textContent = fmt(c.total) + ' Total Due';
          document.getElementById('lfTaxAmt').textContent = fmt(c.tax);
          document.getElementById('lfFeeAmt').textContent = fmt(c.fee);
          document.getElementById('lfDueDate').textContent = c.dueStr;
        }
        ['lfSub','lfTax','lfPct','lfDays','lfNet'].forEach(function(id){ document.getElementById(id).addEventListener('input', recalc); });
        recalc();
        document.getElementById('btnExportLfPdf').addEventListener('click', async function(){
          var c = calc();
          var st = document.getElementById('lfStatus');
          st.textContent = 'Generating PDF...';
          var items = [{ description: 'Invoiced Services Principal', quantity: 1, unit_price: c.sub }];
          if (c.fee > 0) items.push({ description: 'Late Payment Surcharge (' + c.days + ' days overdue)', quantity: 1, unit_price: Math.round(c.fee * 100)/100 });
          var body = {
            doc_type: 'invoice',
            due_date: c.dueStr,
            tax_rate: c.taxRate,
            from: { name: 'Pressmark User Workspace' },
            to: { name: document.getElementById('lfClient').value || 'Client' },
            items: items
          };
          var r = await fetch('/api/v1/generate?format=pdf', { method:'POST', headers:{'Content-Type':'application/json'}, body: JSON.stringify(body) });
          if (!r.ok) { st.textContent = 'Failed to generate PDF.'; return; }
          var blob = await r.blob();
          var u = URL.createObjectURL(blob);
          var a = document.createElement('a'); a.href = u; a.download = 'invoice-with-late-fee.pdf';
          document.body.appendChild(a); a.click(); URL.revokeObjectURL(u); a.remove();
          st.textContent = 'PDF downloaded.';
          if (typeof gtag === 'function') gtag('event', 'calculator_used', { calculator: 'late-fee-invoice' });
        });
      })();
    </script>"#.to_string()
}

pub async fn handle_calculator_trailing(Path(slug): Path<String>) -> Response {
    let clean = slug.trim_end_matches('/');
    Redirect::permanent(&format!("/calculators/{}", clean)).into_response()
}

pub async fn handle_calculator_page(
    Path(slug): Path<String>,
    Query(query): Query<CalcQuery>,
) -> Response {
    let Some(info) = CALCULATORS.iter().find(|c| c.slug == slug) else {
        return (
            StatusCode::NOT_FOUND,
            Html("<h1>404 · Calculator Not Found</h1><p><a href=\"/calculators\">View all calculators</a></p>".to_string()),
        )
            .into_response();
    };

    let selected_state = query
        .state
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_ascii_uppercase())
        .unwrap_or_else(|| "CA".to_string());

    let tool_html = match info.slug {
        "security-deposit-return" => render_deposit_calculator(&selected_state),
        "rent-increase-notice" => render_rent_increase_calculator(&selected_state),
        "commercial-nnn-lease" => render_nnn_calculator(),
        _ => render_late_fee_calculator(),
    };

    let body = format!(
        r#"<section class="pm-hero">
      <span class="pm-eyebrow">Free Online Calculator · PDF Export</span>
      <h1>{title}</h1>
      <p class="pm-lead">{desc}</p>
    </section>
    {tool_html}"#,
        title = html_escape(info.title),
        desc = html_escape(info.short_desc),
        tool_html = tool_html
    );

    let canonical = format!("https://pressmark.dev/calculators/{}", info.slug);
    let json_ld = format!(
        r#"<script type="application/ld+json">{{"@context":"https://schema.org","@type":"WebApplication","name":"{}","url":"{}","applicationCategory":"FinanceApplication","operatingSystem":"Any","offers":{{"@type":"Offer","price":"0.00","priceCurrency":"USD"}}}}</script>"#,
        html_escape(info.title),
        html_escape(&canonical)
    );

    Html(render_seo_shell(
        info.seo_title,
        info.seo_desc,
        &canonical,
        "calculators",
        &json_ld,
        &body,
    ))
    .into_response()
}
