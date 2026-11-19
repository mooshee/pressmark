//! Pressmark Server-Rendered SEO Pages, Catalogs, Docs & Compiled-In Assets
//!
//! Author: Daniel Hallman

use crate::models::{DocumentType, LineItem, PartyDetails};
use crate::pricing::get_pricing_catalog;
use crate::server::AppState;
use crate::state_laws::{all_municipal_sub_profiles, all_state_lease_profiles, StateLeaseProfile};
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Redirect, Response};
use serde::Deserialize;
use std::collections::BTreeSet;
use std::sync::Arc;

const STUDIO_HTML: &str = include_str!("../static/index.html");
const STUDIO_CSS: &str = include_str!("../static/css/studio.css");
const STUDIO_JS: &str = include_str!("../static/js/studio.js");
const TERMS_HTML: &str = include_str!("../static/terms.html");
const PRIVACY_HTML_FALLBACK: &str = include_str!("../static/privacy.html");
const ROBOTS_TXT: &str = include_str!("../static/robots.txt");
const ICON_SVG: &[u8] = include_bytes!("../assets/icon.svg");
const ICON_512_PNG: &[u8] = include_bytes!("../assets/icon-512.png");

#[derive(Debug, Default, Deserialize)]
pub struct TemplatePageQuery {
    pub studio: Option<String>,
}

pub fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn shared_page_css() -> &'static str {
    r#"
    :root {
      --bg: #f6f7fb;
      --surface: #ffffff;
      --border: #dfe3ec;
      --text: #151923;
      --muted: #556075;
      --brand: #3b49df;
      --brand-hover: #2d3ac4;
      --accent-soft: #eef2ff;
      --emerald: #059669;
    }
    * { box-sizing: border-box; }
    body {
      margin: 0;
      background: var(--bg);
      color: var(--text);
      font-family: -apple-system, BlinkMacSystemFont, "Inter", "Segoe UI", Roboto, sans-serif;
      line-height: 1.55;
      -webkit-font-smoothing: antialiased;
    }
    a { color: var(--brand); text-decoration: none; }
    a:hover { text-decoration: underline; }
    .pm-nav {
      position: sticky; top: 0; z-index: 20;
      background: rgba(255,255,255,0.94); backdrop-filter: blur(10px);
      border-bottom: 1px solid var(--border);
    }
    .pm-nav-inner {
      max-width: 1120px; margin: 0 auto; padding: 12px 20px;
      display: flex; align-items: center; justify-content: space-between; gap: 16px; flex-wrap: wrap;
    }
    .pm-brand {
      display: inline-flex; align-items: center; gap: 10px;
      font-weight: 700; font-size: 16px; color: var(--text);
    }
    .pm-brand img { width: 26px; height: 26px; border-radius: 6px; }
    .pm-links { display: flex; align-items: center; gap: 18px; flex-wrap: wrap; font-size: 14px; }
    .pm-links a { color: var(--muted); font-weight: 500; }
    .pm-links a:hover, .pm-links a.active { color: var(--text); text-decoration: none; }
    .pm-btn {
      display: inline-flex; align-items: center; justify-content: center; gap: 8px;
      min-height: 42px; padding: 10px 18px; border-radius: 8px; border: 1px solid transparent;
      font-size: 14px; font-weight: 600; cursor: pointer; text-decoration: none;
    }
    .pm-btn-primary { background: var(--brand); color: #fff; }
    .pm-btn-primary:hover { background: var(--brand-hover); text-decoration: none; }
    .pm-btn-secondary { background: var(--surface); color: var(--text); border-color: var(--border); }
    .pm-btn-secondary:hover { background: #f0f2f7; text-decoration: none; }
    .pm-wrap { max-width: 1120px; margin: 0 auto; padding: 36px 20px 72px; }
    .pm-hero {
      background: var(--surface); border: 1px solid var(--border); border-radius: 16px;
      padding: 32px; margin-bottom: 28px;
    }
    .pm-eyebrow {
      display: inline-block; padding: 4px 10px; border-radius: 999px;
      background: var(--accent-soft); color: var(--brand);
      font-size: 12px; font-weight: 600; margin-bottom: 12px;
    }
    h1 { font-size: clamp(26px, 4vw, 38px); line-height: 1.15; letter-spacing: -0.02em; margin: 0 0 12px; }
    h2 { font-size: 22px; line-height: 1.25; letter-spacing: -0.015em; margin: 0 0 14px; }
    h3 { font-size: 16px; margin: 0 0 8px; }
    .pm-lead { font-size: 17px; color: var(--muted); max-width: 66ch; margin: 0 0 22px; }
    .pm-cta-row { display: flex; flex-wrap: wrap; gap: 12px; align-items: center; }
    .pm-grid-2 { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 20px; }
    .pm-grid-3 { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 18px; }
    .pm-card {
      background: var(--surface); border: 1px solid var(--border); border-radius: 14px;
      padding: 22px; display: flex; flex-direction: column; gap: 10px;
    }
    .pm-card-popular { border-color: var(--brand); box-shadow: 0 0 0 1px var(--brand); }
    .pm-table-wrap { overflow-x: auto; background: var(--surface); border: 1px solid var(--border); border-radius: 14px; }
    table { width: 100%; border-collapse: collapse; font-size: 14px; }
    th, td { text-align: left; padding: 12px 16px; border-bottom: 1px solid #edf0f6; vertical-align: top; }
    th { font-size: 12px; text-transform: uppercase; letter-spacing: 0.04em; color: var(--muted); background: #fafbfd; }
    tr:last-child td { border-bottom: 0; }
    .pm-form-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 12px; }
    .pm-field { display: flex; flex-direction: column; gap: 6px; }
    .pm-field label { font-size: 12px; font-weight: 600; color: var(--muted); }
    .pm-field input, .pm-field select, .pm-field textarea {
      width: 100%; padding: 10px 12px; border: 1px solid var(--border); border-radius: 8px;
      font: inherit; font-size: 15px; background: #fff; color: var(--text);
    }
    pre {
      background: #141824; color: #e8ecf4; padding: 16px; border-radius: 10px;
      overflow-x: auto; font-size: 13px; line-height: 1.5; margin: 10px 0 0;
    }
    code { font-family: ui-monospace, SFMono-Regular, Menlo, monospace; }
    .pm-badge-ok { display: inline-block; padding: 2px 8px; border-radius: 6px; background: #ecfdf5; color: #047857; font-size: 12px; font-weight: 600; }
    .pm-chip-grid { display: flex; flex-wrap: wrap; gap: 8px; }
    .pm-chip {
      display: inline-flex; padding: 7px 12px; border-radius: 8px; background: var(--surface);
      border: 1px solid var(--border); color: var(--text); font-size: 13px; font-weight: 500;
    }
    .pm-chip:hover { border-color: var(--brand); text-decoration: none; }
    .pm-faq details { border-bottom: 1px solid var(--border); padding: 14px 0; }
    .pm-faq details:last-child { border-bottom: 0; }
    .pm-faq summary { font-weight: 600; cursor: pointer; }
    .pm-footer {
      border-top: 1px solid var(--border); padding: 24px 20px; background: var(--surface);
      color: var(--muted); font-size: 13px;
    }
    .pm-footer-inner {
      max-width: 1120px; margin: 0 auto; display: flex; justify-content: space-between;
      flex-wrap: wrap; gap: 14px;
    }
    @media (max-width: 820px) {
      .pm-grid-2, .pm-grid-3, .pm-form-grid { grid-template-columns: 1fr; }
      .pm-hero { padding: 22px; }
      .pm-wrap { padding: 20px 16px 56px; }
    }
    "#
}

pub fn render_seo_shell(
    title: &str,
    description: &str,
    canonical_url: &str,
    active_nav: &str,
    head_extra: &str,
    body_html: &str,
) -> String {
    let nav_cls = |k: &str| if active_nav == k { " class=\"active\"" } else { "" };
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title}</title>
  <meta name="description" content="{description}">
  <meta name="author" content="Daniel Hallman">
  <meta name="robots" content="index, follow">
  <link rel="canonical" href="{canonical_url}">
  <meta property="og:type" content="website">
  <meta property="og:url" content="{canonical_url}">
  <meta property="og:title" content="{title}">
  <meta property="og:description" content="{description}">
  <meta property="og:image" content="https://pressmark.dev/assets/hero.jpg">
  <meta property="og:site_name" content="Pressmark">
  <meta name="twitter:card" content="summary_large_image">
  <meta name="twitter:title" content="{title}">
  <meta name="twitter:description" content="{description}">
  <link rel="icon" type="image/svg+xml" href="/assets/icon.svg">
  <link rel="apple-touch-icon" href="/assets/icon-512.png">
  <script async src="https://www.googletagmanager.com/gtag/js?id=G-4ZM1ZEHH43"></script>
  <script>window.dataLayer=window.dataLayer||[];function gtag(){{dataLayer.push(arguments);}}gtag('js',new Date());gtag('config','G-4ZM1ZEHH43');</script>
  <style>{css}</style>
  {head_extra}
</head>
<body>
  <header class="pm-nav">
    <div class="pm-nav-inner">
      <a href="/" class="pm-brand">
        <img src="/assets/icon.svg" alt="Pressmark">
        <span>Pressmark</span>
      </a>
      <nav class="pm-links" aria-label="Main navigation">
        <a href="/"{nav_studio}>Studio</a>
        <a href="/templates"{nav_templates}>Templates</a>
        <a href="/calculators"{nav_calculators}>Calculators</a>
        <a href="/pricing"{nav_pricing}>Pricing</a>
        <a href="/docs"{nav_docs}>API &amp; MCP</a>
        <a href="/" class="pm-btn pm-btn-primary">Open Studio</a>
      </nav>
    </div>
  </header>
  <main class="pm-wrap">
    {body_html}
  </main>
  <footer class="pm-footer">
    <div class="pm-footer-inner">
      <div>© 2026 Pressmark · Built by <a href="https://mooshworks.com/">Moosh Works</a></div>
      <div class="pm-links">
        <a href="/templates">57 Templates</a>
        <a href="/calculators">Calculators</a>
        <a href="/pricing">Pricing</a>
        <a href="/docs">Docs</a>
        <a href="/privacy">Privacy</a>
        <a href="/terms">Terms</a>
        <a href="/feedback">Feedback</a>
      </div>
    </div>
  </footer>
</body>
</html>"#,
        title = html_escape(title),
        description = html_escape(description),
        canonical_url = html_escape(canonical_url),
        css = shared_page_css(),
        head_extra = head_extra,
        nav_studio = nav_cls("studio"),
        nav_templates = nav_cls("templates"),
        nav_calculators = nav_cls("calculators"),
        nav_pricing = nav_cls("pricing"),
        nav_docs = nav_cls("docs"),
        body_html = body_html
    )
}

pub async fn handle_index() -> impl IntoResponse {
    let html = STUDIO_HTML.replace(
        "<div class=\"studio-help\">",
        "<div class=\"studio-help\"><a href=\"/feedback\">Give feedback</a>",
    );
    (StatusCode::OK, Html(html))
}

pub async fn handle_studio_css() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/css; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=300"),
        ],
        STUDIO_CSS,
    )
}

pub async fn handle_studio_js() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=300"),
        ],
        STUDIO_JS,
    )
}

pub async fn handle_icon_svg() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/svg+xml; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        ICON_SVG,
    )
}

pub async fn handle_icon_512_png() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        ICON_512_PNG,
    )
}

pub async fn handle_privacy() -> impl IntoResponse {
    let html = tokio::fs::read_to_string("static/privacy.html")
        .await
        .unwrap_or_else(|_| PRIVACY_HTML_FALLBACK.to_string());
    (StatusCode::OK, Html(html))
}

pub async fn handle_terms() -> impl IntoResponse {
    (StatusCode::OK, Html(TERMS_HTML))
}

pub async fn handle_robots() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        ROBOTS_TXT,
    )
}

pub async fn handle_templates_page() -> impl IntoResponse {
    let mut cards = String::new();
    for dt in DocumentType::all() {
        let (seo_title, seo_desc) = dt.seo_metadata();
        let short_desc = seo_desc.replace(" in milliseconds", "");
        cards.push_str(&format!(
            "<a class=\"pm-card\" href=\"/templates/{slug}\" style=\"color:inherit;\">\
             <span class=\"pm-badge-ok\">{prefix}</span>\
             <h3>{name}</h3>\
             <p style=\"margin:0;font-size:13.5px;color:var(--muted);\">{desc}</p>\
             <span style=\"font-size:13px;font-weight:600;color:var(--brand);\">Open {title} →</span>\
             </a>",
            slug = dt.url_slug(),
            prefix = html_escape(dt.default_number_prefix().trim_end_matches('-')),
            name = html_escape(seo_title.trim_end_matches(" · Pressmark Studio")),
            desc = html_escape(&short_desc),
            title = html_escape(&dt.display_title()),
        ));
    }

    let mut state_links = String::new();
    for sp in all_state_lease_profiles() {
        state_links.push_str(&format!(
            "<a class=\"pm-chip\" href=\"/templates/{}-residential-lease\">{} ({}) Lease</a>",
            sp.slug,
            html_escape(sp.name),
            sp.code
        ));
    }

    let json_ld = r#"<script type="application/ld+json">{"@context":"https://schema.org","@type":"CollectionPage","name":"Pressmark Document Templates","url":"https://pressmark.dev/templates","description":"57 invoice, contract, and 50-state residential lease PDF templates generated in Rust with Typst."}</script>"#;

    let body = format!(
        r#"<section class="pm-hero">
      <span class="pm-eyebrow">57 Document Types · 51 Jurisdictions</span>
      <h1>Invoice, Contract &amp; State Lease PDF Templates</h1>
      <p class="pm-lead">Pick any template to inspect required fields, check state landlord-tenant rules, export a PDF, or open the interactive editor. Previews and calculations are free.</p>
      <div class="pm-cta-row">
        <a href="/calculators/security-deposit-return" class="pm-btn pm-btn-secondary">Deposit Return Calculator</a>
        <a href="/calculators/rent-increase-notice" class="pm-btn pm-btn-secondary">Rent Increase Calculator</a>
        <a href="/pricing" class="pm-btn pm-btn-secondary">Compare Plans</a>
      </div>
    </section>
    <section style="margin-bottom:36px;">
      <h2>All 57 Business &amp; Real Estate Templates</h2>
      <div class="pm-grid-3">{cards}</div>
    </section>
    <section class="pm-card">
      <h2>50-State + DC Statutory Residential Lease Templates</h2>
      <p style="margin:0 0 12px;color:var(--muted);font-size:14px;">Each state lease page includes deposit limits, return deadlines, pay-or-quit periods, and required inline disclosures from verified state statutes.</p>
      <div class="pm-chip-grid">{state_links}</div>
    </section>"#
    );

    Html(render_seo_shell(
        "57 Business, Legal & State Lease PDF Templates · Pressmark",
        "Browse 57 print-ready invoice, receipt, NDA, commercial lease, and 50-state statutory residential lease PDF templates.",
        "https://pressmark.dev/templates",
        "templates",
        json_ld,
        &body,
    ))
}

pub async fn handle_pricing_page() -> impl IntoResponse {
    let catalog = get_pricing_catalog();
    let mut plan_cards = String::new();
    for plan in catalog.plans.iter().chain(catalog.microtransactions.iter()) {
        let pop_cls = if plan.popular { " pm-card pm-card-popular" } else { "pm-card" };
        let features_html: String = plan
            .features
            .iter()
            .map(|f| format!("<li>{}</li>", html_escape(f)))
            .collect();
        let interval_lbl = match plan.interval {
            "month" => "/ month",
            "one-time" => "one-time",
            _ => "free forever",
        };
        let cta = if plan.price_cents == 0 {
            "<a href=\"/\" class=\"pm-btn pm-btn-secondary\" style=\"margin-top:auto;\">Start in Studio</a>".to_string()
        } else {
            format!(
                "<a href=\"/api/v1/checkout?plan={}&amp;redirect=true\" class=\"pm-btn pm-btn-primary\" style=\"margin-top:auto;\">Select {}</a>",
                html_escape(plan.id),
                html_escape(plan.name)
            )
        };
        plan_cards.push_str(&format!(
            "<div class=\"{pop_cls}\">\
             <div><strong>{name}</strong></div>\
             <div style=\"font-size:30px;font-weight:700;\">{price} <span style=\"font-size:14px;font-weight:500;color:var(--muted);\">{interval}</span></div>\
             <div style=\"font-size:13px;color:var(--brand);font-weight:600;\">{limit}</div>\
             <p style=\"margin:0;font-size:13.5px;color:var(--muted);\">{desc}</p>\
             <ul style=\"margin:4px 0 12px;padding-left:18px;font-size:13.5px;\">{features}</ul>\
             {cta}\
             </div>",
            pop_cls = pop_cls,
            name = html_escape(plan.name),
            price = html_escape(plan.price_display),
            interval = interval_lbl,
            limit = html_escape(plan.document_limit),
            desc = html_escape(plan.description),
            features = features_html,
            cta = cta
        ));
    }

    let json_ld = r#"<script type="application/ld+json">{"@context":"https://schema.org","@type":"FAQPage","mainEntity":[{"@type":"Question","name":"What is free on Pressmark?","acceptedAnswer":{"@type":"Answer","text":"All 57 document templates, interactive live previews, statutory calculators, and MCP agent calls are available at zero cost. Paid plans or single passes remove the Pressmark footer attribution and unlock API keys and higher monthly quotas."}},{"@type":"Question","name":"Do single unlock credits expire?","acceptedAnswer":{"@type":"Answer","text":"Single document passes ($2.99) and 5-document packs ($9.99) stay on your account until used and do not require a monthly subscription."}}]}</script>"#;

    let body = format!(
        r#"<section class="pm-hero">
      <span class="pm-eyebrow">Transparent Pricing · No Lock-In</span>
      <h1>Simple Pricing for PDFs &amp; Team Documents</h1>
      <p class="pm-lead">Build and preview any document for free. Upgrade with a one-time $2.99 pass or monthly plan when you want attribution-free exports, custom logo branding, or API keys.</p>
    </section>
    <div class="pm-grid-3">{plan_cards}</div>
    <section class="pm-card pm-faq" style="margin-top:28px;">
      <h2>Common Questions</h2>
      <details open>
        <summary>What is included in the free tier?</summary>
        <p style="margin:8px 0 0;color:var(--muted);font-size:14px;">All 57 templates, unlimited live previews, all 4 calculators, shareable /v/ recipient links, and anonymous MCP access. Free web PDF downloads include a small footer line ("Prepared with Pressmark · pressmark.dev") unless unlocked.</p>
      </details>
      <details>
        <summary>How does Stripe fulfillment work?</summary>
        <p style="margin:8px 0 0;color:var(--muted);font-size:14px;">Sign in with your email before checkout or use the same email at Stripe Checkout. Verified Stripe webhooks credit your account right after payment settles.</p>
      </details>
    </section>"#
    );

    Html(render_seo_shell(
        "Pricing · Free Tier, $2.99 Pass & Pro Plans · Pressmark",
        "Compare Pressmark Free, $2.99 Single Document Pass, $9.99 5-Document Pack, Solo Pro ($15/mo), Business ($49/mo), and Agency ($149/mo).",
        "https://pressmark.dev/pricing",
        "pricing",
        json_ld,
        &body,
    ))
}

pub async fn handle_docs_page() -> impl IntoResponse {
    let body = r#"<section class="pm-hero">
      <span class="pm-eyebrow">REST API · OpenAPI 3.1 · Streamable HTTP MCP</span>
      <h1>Pressmark Developer &amp; MCP Docs</h1>
      <p class="pm-lead">Generate vector PDFs, 144-PPI previews, totals, and billing conversions from REST endpoints or any Model Context Protocol client with zero setup keys.</p>
      <div class="pm-cta-row">
        <a href="/api/v1/openapi.json" class="pm-btn pm-btn-primary">OpenAPI 3.1 JSON</a>
        <a href="/mcp/tools.json" class="pm-btn pm-btn-secondary">MCP Tool Schemas (7 tools)</a>
        <a href="/SKILL.md" class="pm-btn pm-btn-secondary">Agent SKILL.md</a>
      </div>
    </section>
    <div class="pm-grid-2">
      <section class="pm-card">
        <h2>Connect via MCP (Streamable HTTP)</h2>
        <p style="margin:0;color:var(--muted);font-size:14px;">Endpoint: <code>https://pressmark.dev/mcp</code> (no API key or OAuth required for guest generation).</p>
        <pre><code>claude mcp add --transport http pressmark https://pressmark.dev/mcp</code></pre>
        <pre><code>{
  "mcpServers": {
    "pressmark": {
      "type": "http",
      "url": "https://pressmark.dev/mcp"
    }
  }
}</code></pre>
      </section>
      <section class="pm-card">
        <h2>Generate a PDF over REST</h2>
        <p style="margin:0;color:var(--muted);font-size:14px;">Send JSON to <code>POST /api/v1/generate?format=pdf</code> for raw PDF bytes, or omit <code>?format=pdf</code> for Base64 + PNG preview.</p>
        <pre><code>xh POST https://pressmark.dev/api/v1/generate?format=pdf \
  doc_type=invoice \
  doc_number=INV-2026-101 \
  from:='{"name":"Northstar Electrical LLC"}' \
  to:='{"name":"Summit Property Group"}' \
  items:='[{"description":"Panel upgrade","quantity":1,"unit_price":1450}]' \
  &gt; invoice.pdf</code></pre>
      </section>
    </div>
    <section class="pm-card" style="margin-top:20px;">
      <h2>7 Built-In MCP Tools</h2>
      <div class="pm-table-wrap">
        <table>
          <thead><tr><th>Tool</th><th>Purpose</th></tr></thead>
          <tbody>
            <tr><td><code>generate_document</code></td><td>Compile any of 57 archetypes into PDF and issue a 15-minute HTTP download link.</td></tr>
            <tr><td><code>preview_document</code></td><td>Render a 144-PPI PNG preview image for visual inspection.</td></tr>
            <tr><td><code>calculate_totals</code></td><td>Calculate line-item subtotals, tax rates, discounts, and currency-formatted totals.</td></tr>
            <tr><td><code>convert_document</code></td><td>Convert estimates, invoices, receipts, and credit notes across lifecycle stages.</td></tr>
            <tr><td><code>record_payment</code></td><td>Record partial or full payment details and compute remaining balance or paid receipt.</td></tr>
            <tr><td><code>submit_feedback</code></td><td>Send bug reports or feature requests to the Pressmark engineering inbox.</td></tr>
            <tr><td><code>get_feedback_status</code></td><td>Poll private report review status by report ID and access key.</td></tr>
          </tbody>
        </table>
      </div>
    </section>"#;

    Html(render_seo_shell(
        "API & MCP Documentation · Pressmark Developer Hub",
        "Connect Claude Code, Cursor, Gemini CLI, or custom REST services to Pressmark PDF generation via OpenAPI 3.1 and https://pressmark.dev/mcp.",
        "https://pressmark.dev/docs",
        "docs",
        "",
        body,
    ))
}

pub async fn handle_template_landing_trailing(Path(slug): Path<String>) -> Response {
    let clean_slug = slug.trim_end_matches([':', ')', ']', '}', '"', '\'', '.', ',', ';', '>', '/']);
    Redirect::permanent(&format!("/templates/{}", clean_slug)).into_response()
}

fn render_state_rules_html(sp: &StateLeaseProfile, dt: &DocumentType) -> String {
    let munis: Vec<_> = all_municipal_sub_profiles()
        .iter()
        .filter(|m| m.state_code == sp.code)
        .collect();
    let muni_html = if munis.is_empty() {
        String::new()
    } else {
        let list: String = munis
            .iter()
            .map(|m| {
                format!(
                    "<li><strong>{} ({}):</strong> Deposit Cap: {} · Grace: {} · Pay/Quit: {} · Disclosures: {}</li>",
                    html_escape(m.name),
                    html_escape(m.ordinance_citation),
                    html_escape(m.deposit_cap),
                    html_escape(m.grace_period),
                    html_escape(m.pay_or_quit_notice),
                    html_escape(m.key_disclosures)
                )
            })
            .collect();
        format!(
            "<div class=\"pm-card\" style=\"margin-top:20px;\"><h3>Municipal Overrides in {}</h3><ul style=\"margin:0;padding-left:18px;font-size:14px;\">{}</ul></div>",
            html_escape(sp.name),
            list
        )
    };

    let sibling_slugs = [
        ("Residential Lease", "residential-lease"),
        ("Security Deposit Itemization", "security-deposit-disposition"),
        ("Rent Increase Notice", "rent-increase-notice"),
        ("Notice to Vacate", "notice-to-vacate"),
    ];
    let sibling_html: String = sibling_slugs
        .iter()
        .filter(|(_, suf)| *suf != dt.url_slug())
        .map(|(lbl, suf)| {
            format!(
                "<a class=\"pm-chip\" href=\"/templates/{}-{}\">{} {}</a>",
                sp.slug,
                suf,
                html_escape(sp.name),
                lbl
            )
        })
        .collect();

    format!(
        r#"<section style="margin-top:24px;">
      <h2>{} ({}) Statutory Rules Enforced in This Template</h2>
      <div class="pm-table-wrap">
        <table>
          <tbody>
            <tr><th>Governing Statute</th><td>{} ({})</td></tr>
            <tr><th>Security Deposit Limit</th><td>{}</td></tr>
            <tr><th>Deposit Return Deadline</th><td>{}</td></tr>
            <tr><th>Grace Period &amp; Late Fee Cap</th><td>Grace: {} · Late Fee: {}</td></tr>
            <tr><th>Pay-or-Quit Notice</th><td>{}</td></tr>
            <tr><th>Landlord Entry Notice</th><td>{}</td></tr>
            <tr><th>Remote Online Notarization (RON)</th><td>{}</td></tr>
            <tr><th>Rent Stabilization Rule</th><td>{}</td></tr>
            <tr><th>Mandatory Statutory Disclosures</th><td>{}</td></tr>
          </tbody>
        </table>
      </div>
      {muni_html}
      <div class="pm-card" style="margin-top:20px;">
        <h3>Related {} Landlord-Tenant Documents &amp; Calculators</h3>
        <div class="pm-chip-grid">
          {sibling_html}
          <a class="pm-chip" href="/calculators/security-deposit-return?state={code}">Calculate {code} Deposit Return →</a>
          <a class="pm-chip" href="/calculators/rent-increase-notice?state={code}">Calculate {code} Rent Increase →</a>
        </div>
      </div>
    </section>"#,
        html_escape(sp.name),
        sp.code,
        html_escape(sp.governing_statute),
        html_escape(sp.framework),
        html_escape(sp.deposit_cap),
        html_escape(sp.return_deadline),
        html_escape(sp.grace_period),
        html_escape(sp.late_fee_cap),
        html_escape(sp.pay_or_quit_notice),
        html_escape(sp.entry_notice),
        html_escape(sp.notary_ron_statute),
        html_escape(sp.rent_control_note),
        html_escape(sp.key_disclosures),
        html_escape(sp.name),
        code = sp.code
    )
}

pub async fn handle_template_landing(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<TemplatePageQuery>,
    Path(slug): Path<String>,
) -> Response {
    let clean_slug = slug.trim_end_matches([':', ')', ']', '}', '"', '\'', '.', ',', ';', '>']);
    if clean_slug != slug
        && (DocumentType::from_slug(clean_slug).is_some()
            || DocumentType::parse_state_prefixed_slug(clean_slug).is_some())
    {
        return Redirect::permanent(&format!("/templates/{}", clean_slug)).into_response();
    }

    let (doc_type, maybe_state_profile) = if let Some(dt) = DocumentType::from_slug(&slug) {
        (dt, None)
    } else if let Some((sp, dt)) = DocumentType::parse_state_prefixed_slug(&slug) {
        (dt, Some(sp))
    } else {
        return (
            StatusCode::NOT_FOUND,
            Html("<h1>404 · Template Not Found</h1><p><a href=\"/templates\">Browse all 57 document templates</a></p>".to_string()),
        )
            .into_response();
    };

    let archetype_key = doc_type.as_str();
    let (canonical_slug, seo_title, seo_desc) = if let Some(sp) = maybe_state_profile {
        let (st_title, st_desc) = doc_type.state_seo_metadata(sp);
        (
            format!("{}-{}", sp.slug, doc_type.url_slug()),
            st_title,
            st_desc,
        )
    } else {
        let (base_title, base_desc) = doc_type.seo_metadata();
        (
            doc_type.url_slug().to_string(),
            base_title.to_string(),
            base_desc.to_string(),
        )
    };
    let canonical_url = format!("https://pressmark.dev/templates/{}", canonical_slug);
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let landing_channel = if crate::storage::Storage::is_crawler_bot_ua(user_agent) {
        "web_seo_bot"
    } else if crate::storage::Storage::is_internal_probe_ua(user_agent) {
        "web_seo_probe"
    } else {
        "web_seo"
    };

    state.storage.record_usage_event(
        "template_landing_view",
        Some(archetype_key),
        landing_channel,
        Some(&canonical_slug),
        None,
    );

    let boot_script = if let Some(sp) = maybe_state_profile {
        format!(
            "  <script>window.__PRESSMARK_INITIAL_ARCHETYPE__ = \"{}\"; window.__PRESSMARK_INITIAL_STATE__ = \"{}\";</script>",
            archetype_key, sp.code
        )
    } else {
        format!(
            "  <script>window.__PRESSMARK_INITIAL_ARCHETYPE__ = \"{}\"; window.__PRESSMARK_INITIAL_STATE__ = \"\";</script>",
            archetype_key
        )
    };

    let wants_studio = query
        .studio
        .as_deref()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"));
    if wants_studio {
        let mut html = STUDIO_HTML
            .replace(
                "<div class=\"studio-help\">",
                "<div class=\"studio-help\"><a href=\"/feedback\">Give feedback</a>",
            )
            .replace(
                "<title>Pressmark · Invoices, agreements and documents</title>",
                &format!("<title>{}</title>", html_escape(&seo_title)),
            )
            .replace(
                "<meta name=\"description\" content=\"Create invoices, agreements and business documents. Add your details, choose a design, and review your document with a live preview before exporting.\">",
                &format!("<meta name=\"description\" content=\"{}\">", html_escape(&seo_desc)),
            )
            .replace(
                "<link rel=\"canonical\" href=\"https://pressmark.dev/\">",
                &format!("<link rel=\"canonical\" href=\"{}\">", html_escape(&canonical_url)),
            )
            .replace(
                "<meta property=\"og:url\" content=\"https://pressmark.dev/\">",
                &format!("<meta property=\"og:url\" content=\"{}\">", html_escape(&canonical_url)),
            )
            .replace(
                "<meta property=\"og:title\" content=\"Pressmark · Invoices, agreements and documents\">",
                &format!("<meta property=\"og:title\" content=\"{}\">", html_escape(&seo_title)),
            )
            .replace(
                "<meta property=\"og:description\" content=\"Create your document, add your details, and review every page before exporting.\">",
                &format!("<meta property=\"og:description\" content=\"{}\">", html_escape(&seo_desc)),
            );
        html = html.replacen("</head>", &format!("{}\n</head>", boot_script), 1);
        return (StatusCode::OK, Html(html)).into_response();
    }

    let h1_title = seo_title
        .trim_end_matches(" · Pressmark Studio")
        .trim_end_matches(" · Statutory PDF Generator");
    let display_name = doc_type.display_title();
    let sample_number = format!("{}2026-001", doc_type.default_number_prefix());
    let sample_from = PartyDetails {
        name: maybe_state_profile
            .map(|sp| format!("{} Property Management LLC", sp.name))
            .unwrap_or_else(|| "Harborview Consulting LLC".to_string()),
        ..Default::default()
    };
    let state_rules_block = maybe_state_profile
        .map(|sp| render_state_rules_html(sp, &doc_type))
        .unwrap_or_default();
    let state_code_val = maybe_state_profile.map(|sp| sp.code).unwrap_or("");
    let json_ld = format!(
        r#"<script type="application/ld+json">{{"@context":"https://schema.org","@type":"BreadcrumbList","itemListElement":[{{"@type":"ListItem","position":1,"name":"Templates","item":"https://pressmark.dev/templates"}},{{"@type":"ListItem","position":2,"name":"{name}","item":"{url}"}}]}}</script>{boot}"#,
        name = html_escape(h1_title),
        url = html_escape(&canonical_url),
        boot = boot_script
    );

    let default_item = LineItem {
        description: format!("{} Standard Schedule", display_name),
        quantity: 1.0,
        unit_price: if doc_type.is_state_lease_instrument() { 1850.0 } else { 750.0 },
        tax_rate: None,
        discount: None,
    };

    let body = format!(
        r#"<section class="pm-hero">
      <span class="pm-eyebrow">Vector PDF Template · {prefix}</span>
      <h1>{h1_title}</h1>
      <p class="pm-lead">{seo_desc}</p>
      <div class="pm-cta-row">
        <a href="/templates/{canonical_slug}?studio=1" class="pm-btn pm-btn-primary">Open Full Studio Editor</a>
        <a href="/templates" class="pm-btn pm-btn-secondary">Browse All 57 Templates</a>
      </div>
    </section>
    <div class="pm-grid-2">
      <section class="pm-card">
        <h2>Quick PDF Export</h2>
        <p style="margin:0;color:var(--muted);font-size:14px;">Enter the main details below to compile and download a vector PDF directly, or open the full editor for multi-line items, signatures, and theme styling.</p>
        <form id="quickTemplateForm" class="pm-form-grid">
          <div class="pm-field"><label for="qFrom">Issuer / Landlord Name</label><input id="qFrom" required value="{from_name}"></div>
          <div class="pm-field"><label for="qTo">Recipient / Tenant Name</label><input id="qTo" required value="Jordan Avery"></div>
          <div class="pm-field"><label for="qNum">Document Number</label><input id="qNum" value="{sample_number}"></div>
          <div class="pm-field"><label for="qAmt">Primary Amount (USD)</label><input id="qAmt" type="number" step="0.01" value="{amt}"></div>
          <div class="pm-field" style="grid-column:1 / -1;"><label for="qItem">Line Item / Covenant Summary</label><input id="qItem" value="{item_desc}"></div>
          <div class="pm-cta-row" style="grid-column:1 / -1;">
            <button type="submit" class="pm-btn pm-btn-primary">Download Vector PDF</button>
            <span id="qStatus" style="font-size:13px;color:var(--muted);"></span>
          </div>
        </form>
      </section>
      <section class="pm-card">
        <h2>What This Template Includes</h2>
        <ul style="margin:0;padding-left:18px;font-size:14px;color:var(--text);">
          <li>Print-ready vector layout powered by Typst (<code>us-letter</code> and <code>a4</code>)</li>
          <li>Optional scan-to-pay Stripe or payment link QR block</li>
          <li>Digital signature and execution jurat formatting</li>
          <li>Shareable recipient view (<code>/v/token</code>) with online e-signing</li>
          <li>REST &amp; MCP API automation via <code>doc_type: "{archetype_key}"</code></li>
        </ul>
      </section>
    </div>
    {state_rules_block}
    <script>
      document.getElementById('quickTemplateForm').addEventListener('submit', async function(e) {{
        e.preventDefault();
        var st = document.getElementById('qStatus');
        st.textContent = 'Compiling PDF...';
        var payload = {{
          doc_type: {doc_type_json},
          doc_number: document.getElementById('qNum').value,
          state_code: {state_code_json} || undefined,
          from: {{ name: document.getElementById('qFrom').value }},
          to: {{ name: document.getElementById('qTo').value }},
          items: [{{ description: document.getElementById('qItem').value, quantity: 1, unit_price: parseFloat(document.getElementById('qAmt').value) || 0 }}]
        }};
        try {{
          var r = await fetch('/api/v1/generate?format=pdf', {{ method:'POST', headers:{{'Content-Type':'application/json'}}, body: JSON.stringify(payload) }});
          if (!r.ok) {{ st.textContent = 'Could not generate PDF.'; return; }}
          var blob = await r.blob();
          var url = URL.createObjectURL(blob);
          var a = document.createElement('a'); a.href = url; a.download = (payload.doc_number || 'document') + '.pdf';
          document.body.appendChild(a); a.click(); URL.revokeObjectURL(url); a.remove();
          st.textContent = 'PDF downloaded.';
          if (typeof gtag === 'function') gtag('event', 'pdf_downloaded', {{ doc_type: payload.doc_type, source: 'template_landing' }});
        }} catch (_) {{ st.textContent = 'Connection error.'; }}
      }});
    </script>"#,
        prefix = html_escape(doc_type.default_number_prefix().trim_end_matches('-')),
        h1_title = html_escape(h1_title),
        seo_desc = html_escape(&seo_desc),
        canonical_slug = html_escape(&canonical_slug),
        from_name = html_escape(&sample_from.name),
        sample_number = html_escape(&sample_number),
        amt = default_item.unit_price,
        item_desc = html_escape(&default_item.description),
        archetype_key = html_escape(archetype_key),
        doc_type_json = serde_json::to_string(archetype_key).unwrap_or_else(|_| "\"invoice\"".to_string()),
        state_code_json = serde_json::to_string(state_code_val).unwrap_or_else(|_| "\"\"".to_string()),
    );

    Html(render_seo_shell(
        &seo_title,
        &seo_desc,
        &canonical_url,
        "templates",
        &json_ld,
        &body,
    ))
    .into_response()
}

pub async fn handle_sitemap() -> impl IntoResponse {
    let mut xml = String::with_capacity(49152);
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    for (path, freq, prio) in [
        ("/", "daily", "1.0"),
        ("/templates", "weekly", "0.9"),
        ("/calculators", "weekly", "0.9"),
        ("/calculators/security-deposit-return", "weekly", "0.9"),
        ("/calculators/rent-increase-notice", "weekly", "0.9"),
        ("/calculators/commercial-nnn-lease", "weekly", "0.9"),
        ("/calculators/late-fee-invoice", "weekly", "0.9"),
        ("/pricing", "weekly", "0.9"),
        ("/docs", "weekly", "0.8"),
        ("/privacy", "monthly", "0.5"),
        ("/terms", "monthly", "0.5"),
    ] {
        xml.push_str(&format!(
            "  <url><loc>https://pressmark.dev{path}</loc><changefreq>{freq}</changefreq><priority>{prio}</priority></url>\n"
        ));
    }

    let mut template_slugs = BTreeSet::new();
    for item in DocumentType::all() {
        template_slugs.insert(item.url_slug().to_string());
    }
    for sp in all_state_lease_profiles() {
        for suf in [
            "residential-lease",
            "security-deposit-disposition",
            "rent-increase-notice",
            "notice-to-vacate",
        ] {
            template_slugs.insert(format!("{}-{}", sp.slug, suf));
        }
    }
    for extra_slug in [
        "florida-eviction-notice",
        "texas-quitclaim-deed",
        "new-york-room-rental-lease",
        "california-eviction-notice",
        "tennessee-property-deed",
    ] {
        template_slugs.insert(extra_slug.to_string());
    }
    for slug in template_slugs {
        xml.push_str(&format!(
            "  <url><loc>https://pressmark.dev/templates/{slug}</loc><changefreq>weekly</changefreq><priority>0.8</priority></url>\n"
        ));
    }
    xml.push_str("</urlset>\n");
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        xml,
    )
}

pub async fn handle_llms_txt() -> impl IntoResponse {
    let mut lines = vec![
        "# Pressmark Studio · Vector PDF Document Engine".to_string(),
        "".to_string(),
        "> Stateless Rust and Typst vector PDF document synthesis engine, interactive web studio, statutory calculators, and Model Context Protocol (MCP) server at https://pressmark.dev/mcp.".to_string(),
        "".to_string(),
        "## Core Endpoints & Agent Integration".to_string(),
        "- [Interactive Studio](https://pressmark.dev/): Design, preview, and export 57 business, billing, legal, and real estate document types.".to_string(),
        "- [Templates Catalog](https://pressmark.dev/templates): 57 templates and 51 state statutory lease guides.".to_string(),
        "- [Statutory Calculators](https://pressmark.dev/calculators): Deposit return, rent increase, NNN lease, and late-fee invoice calculators with direct PDF export.".to_string(),
        "- [Remote MCP Endpoint](https://pressmark.dev/mcp): Streamable HTTP MCP server (`io.github.mooshee/pressmark`) with seven tools: `generate_document`, `preview_document`, `calculate_totals`, `convert_document`, `record_payment`, `submit_feedback`, and `get_feedback_status`.".to_string(),
        "- [OpenAPI 3.1 Specification](https://pressmark.dev/api/v1/openapi.json): REST API schema for `/api/v1/generate`, `/api/v1/preview`, `/api/v1/share`, `/api/v1/templates`, `/api/v1/states`, and `/api/v1/pricing`.".to_string(),
        "- [Pricing & Unlock Passes](https://pressmark.dev/pricing): Free community tier, $2.99 Single Document Pass, $9.99 5-Document Pack, and $15/mo Solo Pro.".to_string(),
        "- [Developer & MCP Docs](https://pressmark.dev/docs): Connection snippets for Claude Code, Cursor, VS Code, Gemini CLI, and REST clients.".to_string(),
        "- [Full LLM Corpus](https://pressmark.dev/llms-full.txt): Complete 57-archetype catalog and MCP tool documentation.".to_string(),
        "".to_string(),
        "## Interactive Document Templates (57 Archetypes)".to_string(),
    ];
    for dt in DocumentType::all() {
        let (_, desc) = dt.seo_metadata();
        lines.push(format!(
            "- [{}](https://pressmark.dev/templates/{}) — {}",
            dt.display_title(),
            dt.url_slug(),
            desc
        ));
    }
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        lines.join("\n"),
    )
}

pub async fn handle_llms_full_txt() -> impl IntoResponse {
    let mut lines = vec![
        "# Pressmark Studio — Full AI & LLM Reference Corpus".to_string(),
        "".to_string(),
        "Website: https://pressmark.dev".to_string(),
        "MCP Server (Streamable HTTP): https://pressmark.dev/mcp".to_string(),
        "MCP Registry ID: io.github.mooshee/pressmark".to_string(),
        "OpenAPI Spec: https://pressmark.dev/api/v1/openapi.json".to_string(),
        "".to_string(),
        "## MCP Tools (7 Tools)".to_string(),
        "1. `generate_document`: Creates a vector PDF from supplied details. HTTP calls can return a temporary download link.".to_string(),
        "2. `preview_document`: Renders a 144-PPI PNG preview as a Base64 Data URI for immediate visual inspection in chat.".to_string(),
        "3. `calculate_totals`: Computes line-item subtotals, tax rates, discounts, and currency-specific grand totals.".to_string(),
        "4. `convert_document`: Converts an estimate, invoice, or receipt into another billing document.".to_string(),
        "5. `record_payment`: Uses supplied payment details to calculate the balance and create an updated invoice or receipt.".to_string(),
        "6. `submit_feedback`: Sends a private bug report, feature request, or general feedback to the Pressmark inbox.".to_string(),
        "7. `get_feedback_status`: Checks report status by report ID and access key without exposing private report text.".to_string(),
        "".to_string(),
        "## All 57 Document Archetypes".to_string(),
    ];
    for dt in DocumentType::all() {
        let (title, desc) = dt.seo_metadata();
        lines.push(format!(
            "### {} (`{}`)\n- URL: https://pressmark.dev/templates/{}\n- SEO Title: {}\n- Summary: {}\n",
            dt.display_title(),
            dt.as_str(),
            dt.url_slug(),
            title,
            desc
        ));
    }
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        lines.join("\n"),
    )
}
