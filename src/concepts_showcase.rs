//! Dedicated SEO Landing Pages, Element Variations Showcase & Live Interactive Customizer
//! for Pressmark (`pressmark.io/elements`, `pressmark.io/concepts/:slug`).
//!
//! Author: Daniel Hallman

use std::path::PathBuf;

use axum::extract::Path;
use axum::extract::Query;
use axum::http::header;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use hallman_doc_engine::catalog::find_concept_by_slug;
use hallman_doc_engine::catalog::render_concept_typst_markup;
use hallman_doc_engine::catalog::render_element_showcase_sheet_typst;
use hallman_doc_engine::catalog::ArchetypeConceptSpec;
use hallman_doc_engine::catalog::CustomRenderOptions;
use hallman_doc_engine::catalog::ARCHETYPE_CONCEPTS_50;
use hallman_doc_engine::compile_typst_to_pdf_bytes;
use hallman_doc_engine::compile_typst_to_png_bytes;

fn resolve_pressmark_root_dir() -> (PathBuf, &'static str) {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.join("static/assets/seals").exists() {
        return (cwd, "/static/assets/seals");
    }
    let fallback = PathBuf::from("/var/lib/pressmark");
    if fallback.join("static/assets/seals").exists() {
        return (fallback, "/static/assets/seals");
    }
    (cwd, "/static/assets/seals")
}

fn html_escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 16);
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Serve Nano Banana vertical certification seals and showcase sheets from `static/assets/seals/`.
pub async fn handle_pressmark_seal_asset(Path(filename): Path<String>) -> Response {
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return (StatusCode::BAD_REQUEST, "Invalid asset filename").into_response();
    }
    let (root, _) = resolve_pressmark_root_dir();
    let file_path = root.join("static/assets/seals").join(&filename);
    match tokio::fs::read(&file_path).await {
        Ok(bytes) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "image/png"),
                (header::CACHE_CONTROL, "public, max-age=86400"),
            ],
            bytes,
        )
            .into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Seal asset not found").into_response(),
    }
}

/// JSON API returning all 50 profession & archetype concepts.
pub async fn handle_pressmark_concepts_catalog_json() -> impl IntoResponse {
    Json(ARCHETYPE_CONCEPTS_50)
}

/// Live 144-DPI PNG preview endpoint supporting query-string style & content overrides.
pub async fn handle_pressmark_concept_preview_png(
    Path(slug): Path<String>,
    Query(opts): Query<CustomRenderOptions>,
) -> Response {
    let clean_slug = slug.trim_end_matches(".png");
    let (root, seal_prefix) = resolve_pressmark_root_dir();

    let markup = match clean_slug {
        "showcase-layouts" => render_element_showcase_sheet_typst("page_layouts", seal_prefix),
        "showcase-mastheads" => render_element_showcase_sheet_typst("mastheads", seal_prefix),
        "showcase-tables" => render_element_showcase_sheet_typst("tables_and_kpis", seal_prefix),
        "showcase-diagrams" => {
            render_element_showcase_sheet_typst("diagrams_and_stamps", seal_prefix)
        }
        "showcase-seals" => {
            render_element_showcase_sheet_typst("signatures_and_seals", seal_prefix)
        }
        _ => {
            let Some(spec) = find_concept_by_slug(clean_slug) else {
                return (StatusCode::NOT_FOUND, "Concept not found").into_response();
            };
            render_concept_typst_markup(spec, Some(&opts), seal_prefix)
        }
    };

    match tokio::task::spawn_blocking(move || compile_typst_to_png_bytes(&markup, Some(&root), 144))
        .await
    {
        Ok(Ok(png_bytes)) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "image/png"),
                (header::CACHE_CONTROL, "public, max-age=300"),
            ],
            png_bytes,
        )
            .into_response(),
        Ok(Err(err)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("PNG preview error: {}", err),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Task join error: {}", err),
        )
            .into_response(),
    }
}

/// Live fillable `/AcroForm` PDF compilation endpoint supporting query-string overrides.
pub async fn handle_pressmark_concept_sample_pdf(
    Path(slug): Path<String>,
    Query(opts): Query<CustomRenderOptions>,
) -> Response {
    let clean_slug = slug.trim_end_matches(".pdf");
    let Some(spec) = find_concept_by_slug(clean_slug) else {
        return (StatusCode::NOT_FOUND, "Concept not found").into_response();
    };

    let (root, seal_prefix) = resolve_pressmark_root_dir();
    let markup = render_concept_typst_markup(spec, Some(&opts), seal_prefix);
    let filename = format!("pressmark-{:02}-{}.pdf", spec.id, spec.slug);

    match tokio::task::spawn_blocking(move || compile_typst_to_pdf_bytes(&markup, Some(&root))).await
    {
        Ok(Ok(pdf_bytes)) => {
            let disposition = format!("inline; filename=\"{}\"", filename);
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "application/pdf".to_string()),
                    (header::CONTENT_DISPOSITION, disposition),
                    (header::CACHE_CONTROL, "public, max-age=300".to_string()),
                ],
                pdf_bytes,
            )
                .into_response()
        }
        Ok(Err(err)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("PDF compilation error: {}", err),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Task join error: {}", err),
        )
            .into_response(),
    }
}

/// Main `/elements` & `/concepts` Hub on Pressmark.
pub async fn handle_pressmark_elements_hub() -> impl IntoResponse {
    let mut concept_options_html = String::with_capacity(16384);
    let mut cards_html = String::with_capacity(65536);

    for spec in ARCHETYPE_CONCEPTS_50 {
        concept_options_html.push_str(&format!(
            r#"<option value="{slug}" data-layout="{layout}" data-masthead="{masthead}" data-kpi="{kpi}" data-table="{table}" data-diagram="{diagram}" data-preset="{preset}" data-exec="{exec}" data-title="{title}" data-org="{org}" data-counterparty="{cp}" data-stamp="{stamp}" data-kpi0-val="{kpi0_val}" data-item0-desc="{item0_desc}" data-item0-amt="{item0_amt}">#{id:02} — {title} ({vertical})</option>"#,
            slug = spec.slug,
            layout = spec.default_page_layout,
            masthead = spec.default_masthead,
            kpi = spec.default_kpi_style,
            table = spec.default_table_style,
            diagram = spec.default_diagram_kind,
            preset = spec.default_readability.as_typst_id(),
            exec = spec.default_execution_mode,
            org = html_escape(spec.sample_org),
            cp = html_escape(spec.sample_counterparty),
            stamp = html_escape(spec.stamp_label),
            kpi0_val = html_escape(spec.kpi_metrics[0].1),
            item0_desc = html_escape(spec.line_items[0].1),
            item0_amt = html_escape(spec.line_items[0].3),
            id = spec.id,
            title = html_escape(spec.title),
            vertical = html_escape(spec.vertical),
        ));

        cards_html.push_str(&format!(
            r#"<article class="pm-card" data-vertical="{vslug}" data-search="{search_blob}">
  <div style="display:flex;align-items:center;gap:10px;margin-bottom:10px;">
    <img src="/assets/seals/{seal}" alt="{vertical} seal" width="44" height="44" loading="lazy" style="border:1.5px solid #cbd5e1;border-radius:50%;background:#fff;">
    <div>
      <span class="pm-badge">CONCEPT #{id:02} · {vertical}</span>
      <div style="font-family:monospace;font-size:0.74rem;color:#64748b;margin-top:2px;">{citation}</div>
    </div>
  </div>
  <h3 style="margin:6px 0;font-size:1.05rem;font-weight:800;"><a href="/concepts/{slug}" style="color:#0f172a;text-decoration:none;">{title}</a></h3>
  <p style="margin:0 0 6px;font-size:0.82rem;color:#334155;"><strong>Profession:</strong> {profession}</p>
  <p style="margin:0 0 12px;font-size:0.8rem;color:#475569;line-height:1.45;">{signal}</p>
  <div style="display:flex;flex-wrap:wrap;gap:6px;margin-bottom:14px;">
    <span class="pm-pill" style="background:#dbeafe;color:#1e3a8a;font-weight:700;">Layout: {layout}</span>
    <span class="pm-pill">Masthead: {masthead}</span>
    <span class="pm-pill">Diagram: {diagram}</span>
    <span class="pm-pill">Preset: {preset}</span>
  </div>
  <div style="display:flex;gap:8px;flex-wrap:wrap;margin-top:auto;">
    <a href="/concepts/{slug}" class="pm-btn pm-btn--primary">Customize Live</a>
    <a href="/api/v1/concepts/{slug}/sample.pdf" target="_blank" rel="noopener" class="pm-btn pm-btn--secondary">Fillable PDF ↗</a>
  </div>
</article>"#,
            vslug = spec.vertical_slug,
            search_blob = html_escape(&format!(
                "{} {} {} {} {} {}",
                spec.title.to_lowercase(),
                spec.vertical.to_lowercase(),
                spec.target_profession.to_lowercase(),
                spec.primary_keyword.to_lowercase(),
                spec.statutory_citation.to_lowercase(),
                spec.default_page_layout.to_lowercase()
            )),
            seal = spec.seal_filename,
            vertical = html_escape(spec.vertical),
            id = spec.id,
            citation = html_escape(spec.statutory_citation),
            slug = spec.slug,
            title = html_escape(spec.title),
            profession = html_escape(spec.target_profession),
            signal = html_escape(spec.demand_signal),
            layout = spec.default_page_layout,
            masthead = spec.default_masthead,
            diagram = spec.default_diagram_kind,
            preset = spec.default_readability.as_typst_id(),
        ));
    }

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>100 Profession Document Concepts &amp; 10 Page Layout Blueprints Studio | Pressmark</title>
  <meta name="description" content="Interactive vector PDF element customizer and 100-profession template library across 10 structural page archetypes and 10 verticals.">
  <link rel="canonical" href="https://pressmark.dev/elements">
  <meta property="og:type" content="website">
  <meta property="og:title" content="100 Profession Document Concepts &amp; 10 Page Layout Blueprints Studio | Pressmark">
  <meta property="og:description" content="Switch between 10 structural page layouts, 6 mastheads, KPI strips, 10 embedded technical SVG schematics, readability presets, and fillable /AcroForm signatures live.">
  <meta property="og:url" content="https://pressmark.dev/elements">
  <meta property="og:image" content="https://pressmark.dev/assets/seals/showcase_sheet_page_layouts.png">
  <script type="application/ld+json">
  {{
    "@context": "https://schema.org",
    "@graph": [
      {{
        "@type": "CollectionPage",
        "@id": "https://pressmark.dev/elements#webpage",
        "url": "https://pressmark.dev/elements",
        "name": "100 Profession Document Concepts & 10 Page Layout Blueprints Studio",
        "description": "Vector PDF synthesis library spanning 100 profession archetypes, 10 structural page layouts, embedded SVG schematics, and interactive /AcroForm signature blocks."
      }},
      {{
        "@type": "SoftwareApplication",
        "name": "Pressmark Vector Document Studio",
        "applicationCategory": "BusinessApplication",
        "operatingSystem": "Web, macOS, Windows, iOS, Android",
        "offers": {{ "@type": "Offer", "price": "0.00", "priceCurrency": "USD" }}
      }}
    ]
  }}
  </script>
  <style>
    body {{ background:#f8fafc; color:#0f172a; font-family:'Inter', -apple-system, BlinkMacSystemFont, sans-serif; margin:0; }}
    .pm-shell {{ max-width:1280px; margin:0 auto; padding:28px 16px 64px; }}
    .pm-hero {{ background:#0f172a; color:#fff; border-radius:10px; padding:32px; margin-bottom:32px; border-bottom:4px solid #0284c7; }}
    .pm-badge {{ background:#dbeafe; color:#1e3a8a; font-family:monospace; font-size:0.7rem; font-weight:800; padding:2px 8px; border-radius:4px; text-transform:uppercase; }}
    .pm-studio {{ display:grid; grid-template-columns:390px 1fr; gap:24px; background:#fff; border:1px solid #cbd5e1; border-radius:10px; padding:24px; margin-bottom:40px; box-shadow:0 4px 16px rgba(15,23,42,0.06); }}
    @media (max-width: 960px) {{ .pm-studio {{ grid-template-columns:1fr; }} }}
    .pm-field {{ margin-bottom:11px; }}
    .pm-field label {{ display:block; font-family:monospace; font-size:0.74rem; font-weight:700; text-transform:uppercase; margin-bottom:4px; color:#334155; }}
    .pm-field select, .pm-field input {{ width:100%; padding:8px 10px; border:1.5px solid #94a3b8; border-radius:6px; font-size:0.86rem; font-weight:600; box-sizing:border-box; }}
    .pm-preview {{ background:#0f172a; border-radius:8px; padding:16px; display:flex; flex-direction:column; align-items:center; }}
    .pm-preview img {{ max-width:100%; height:auto; border-radius:4px; background:#fff; }}
    .pm-showcase-grid {{ display:grid; grid-template-columns:repeat(auto-fit, minmax(240px, 1fr)); gap:18px; margin-bottom:40px; }}
    .pm-showcase-card {{ background:#fff; border:1px solid #cbd5e1; border-radius:8px; padding:14px; }}
    .pm-showcase-card img {{ width:100%; height:auto; border:1px solid #e2e8f0; border-radius:4px; display:block; margin-top:8px; }}
    .pm-grid {{ display:grid; grid-template-columns:repeat(auto-fill, minmax(340px, 1fr)); gap:18px; }}
    .pm-card {{ background:#fff; border:1px solid #cbd5e1; border-radius:8px; padding:18px; display:flex; flex-direction:column; }}
    .pm-pill {{ background:#f1f5f9; border:1px solid #cbd5e1; font-family:monospace; font-size:0.7rem; padding:2px 6px; border-radius:4px; }}
    .pm-btn {{ display:inline-block; padding:8px 14px; border-radius:6px; font-weight:700; font-size:0.82rem; text-decoration:none; cursor:pointer; border:none; }}
    .pm-btn--primary {{ background:#1e3a8a; color:#fff; }}
    .pm-btn--secondary {{ background:#e2e8f0; color:#0f172a; }}
    .pm-filter-btn {{ border:1px solid #cbd5e1; background:#fff; color:#0f172a; padding:7px 12px; border-radius:6px; font-weight:700; font-size:0.8rem; cursor:pointer; }}
    .pm-filter-btn.active {{ background:#1e3a8a; color:#fff; }}
  </style>
</head>
<body>
  <header style="background:#0f172a;color:#fff;padding:14px 20px;border-bottom:1px solid #1e293b;">
    <div style="max-width:1280px;margin:0 auto;display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:12px;">
      <a href="/" style="color:#fff;font-weight:900;font-size:1.2rem;text-decoration:none;">PRESSMARK STUDIO</a>
      <nav style="display:flex;gap:18px;font-size:0.85rem;font-weight:700;">
        <a href="/" style="color:#cbd5e1;text-decoration:none;">Interactive Builder (57 Archetypes)</a>
        <a href="/elements" style="color:#38bdf8;text-decoration:underline;">100 Profession Concepts &amp; 10 Page Layouts</a>
        <a href="/pricing" style="color:#cbd5e1;text-decoration:none;">Pricing</a>
      </nav>
    </div>
  </header>

  <main class="pm-shell">
    <section class="pm-hero">
      <span style="background:#0284c7;color:#fff;font-family:monospace;font-size:0.76rem;font-weight:800;padding:3px 10px;border-radius:4px;">SHARED HALLMAN DOC ENGINE · 100 PROFESSION CONCEPTS · 10 PAGE LAYOUTS</span>
      <h1 style="font-size:clamp(1.8rem, 3.2vw, 2.5rem);margin:12px 0 8px;">10 Structural Page Archetype Blueprints, Embedded SVG Schematics &amp; 100 Profession Templates</h1>
      <p style="max-width:74ch;color:#cbd5e1;line-height:1.55;margin:0;">Switch between 10 structural page layouts (Asymmetric Sidebar, Pleading Caption, 3-Tier Packages, Inspection Checklist, Blueprint Schematic, Split Settlement, Clinical Box Grid, Certificate Broadsheet, Dark Tactile Brief, and Executive Draw Ledger), 6 masthead architectures, 10 domain SVG diagrams, and fillable /AcroForm signature widgets in real time.</p>
    </section>

    <section class="pm-studio">
      <div>
        <div class="pm-field">
          <label for="pm-concept">1. Profession Concept (1–100)</label>
          <select id="pm-concept">{concept_options_html}</select>
        </div>
        <div class="pm-field">
          <label for="pm-layout">2. Structural Page Layout Blueprint (10 Archetypes)</label>
          <select id="pm-layout">
            <option value="executive_draw_ledger">10 · Executive Draw &amp; SOV Ledger</option>
            <option value="asymmetric_sidebar">01 · Asymmetric Sidebar Ledger</option>
            <option value="pleading_caption">02 · Numbered Legal Pleading Caption</option>
            <option value="three_tier_packages">03 · 3-Tier Package Comparison Cards</option>
            <option value="inspection_checklist">04 · Multi-Point Inspection Checklist</option>
            <option value="blueprint_schematic">05 · Engineering Blueprint &amp; CAD Sheet</option>
            <option value="split_settlement">06 · Bilateral T-Account Split Settlement</option>
            <option value="clinical_box_grid">07 · Clinical / Regulatory Boxed Form Grid</option>
            <option value="certificate_broadsheet">08 · Ceremonial Certificate Broadsheet</option>
            <option value="dark_tactile_brief">09 · Dark Tactile 3D Depth Brief</option>
          </select>
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="pm-masthead">3. Masthead Style</label>
            <select id="pm-masthead">
              <option value="trade_banner">Trade Command (Dark)</option>
              <option value="neo_brutalist">Neo-Brutalist (#FFE500)</option>
              <option value="judicial_caption">Judicial Caption</option>
              <option value="swiss_grid">Swiss Precision Grid</option>
              <option value="editorial_serif">Editorial Serif</option>
              <option value="executive_split">Clinical / Executive Split</option>
            </select>
          </div>
          <div class="pm-field">
            <label for="pm-kpi">4. KPI Strip Style</label>
            <select id="pm-kpi">
              <option value="metric_cards">Executive Metric Cards</option>
              <option value="dark_terminal">Dark Terminal Telemetry</option>
              <option value="neo_brutalist">Neo-Brutalist Hard Border</option>
            </select>
          </div>
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="pm-diagram">5. Embedded SVG Diagram</label>
            <select id="pm-diagram">
              <option value="floorplan_spec">Commercial Suite RSF Zoning</option>
              <option value="parcel_lot">Cadastral Parcel &amp; Setback Plat</option>
              <option value="roof_pitch">Roof Pitch &amp; Hail Impact</option>
              <option value="hvac_diagnostic">HVAC Superheat &amp; Airflow</option>
              <option value="solar_array">Solar PV + LFP Storage</option>
              <option value="vehicle_inspection">Fleet Chassis Diagnostic</option>
              <option value="soil_topography">Soil Horizon &amp; LiDAR Slope</option>
              <option value="cap_table_waterfall">Cap Table &amp; SAFE Waterfall</option>
              <option value="chain_of_custody">ISO Lab Chain of Custody</option>
              <option value="logistics_route">Intermodal Freight Corridor</option>
            </select>
          </div>
          <div class="pm-field">
            <label for="pm-table">6. Schedule Table Style</label>
            <select id="pm-table">
              <option value="dark_header">Dark Header + Zebra</option>
              <option value="striped">Primary Header + Striped</option>
              <option value="minimal_lines">Minimal Editorial Rules</option>
            </select>
          </div>
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="pm-preset">7. Readability Preset</label>
            <select id="pm-preset">
              <option value="standard">Standard (9.5pt Body)</option>
              <option value="high_legibility">High-Legibility (10.4pt Large)</option>
              <option value="executive_compact">Executive Compact (8.8pt)</option>
              <option value="judicial_formal">Judicial Formal (9.5pt)</option>
            </select>
          </div>
          <div class="pm-field">
            <label for="pm-exec">8. Signature Mode</label>
            <select id="pm-exec">
              <option value="wet-ink">Fillable PDF (/AcroForm)</option>
              <option value="esign">Electronic Counter-Sign (/s/)</option>
              <option value="notary_jurat">Sworn Notary Public Jurat</option>
            </select>
          </div>
        </div>
        <div class="pm-field">
          <label for="pm-title">9. Document Title Override</label>
          <input type="text" id="pm-title" value="AIA G702/G703 Style Progress Billing &amp; Schedule of Values">
        </div>
        <div class="pm-field">
          <label for="pm-org">10. Issuing Organization</label>
          <input type="text" id="pm-org" value="Lookout Valley Commercial Builders LLC">
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="pm-cp">11. Counterparty / Client</label>
            <input type="text" id="pm-cp" value="Riverfront Mixed-Use DevelopmentSPE LLC">
          </div>
          <div class="pm-field">
            <label for="pm-stamp">12. Vector Rubber Stamp</label>
            <input type="text" id="pm-stamp" value="DRAW #04 APPROVED">
          </div>
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="pm-kpi0-val">13. Primary KPI Value</label>
            <input type="text" id="pm-kpi0-val" value="$1,480,000.00">
          </div>
          <div class="pm-field">
            <label for="pm-item0-amt">14. Line-Item #1 Amount</label>
            <input type="text" id="pm-item0-amt" value="$240,000.00">
          </div>
        </div>
        <div class="pm-field">
          <label for="pm-item0-desc">15. Line-Item #1 Description</label>
          <input type="text" id="pm-item0-desc" value="Div 03 — Cast-in-Place Structural Concrete &amp; Post-Tension Slabs (100% Complete)">
        </div>
        <div style="display:flex;gap:10px;flex-wrap:wrap;margin-top:14px;">
          <button type="button" id="pm-render-btn" class="pm-btn pm-btn--primary" style="flex:1;">Update Preview</button>
          <a id="pm-pdf-link" href="/api/v1/concepts/aia-g702-g703-progress-billing-draw-schedule/sample.pdf" target="_blank" rel="noopener" class="pm-btn pm-btn--secondary">Download Fillable PDF ↗</a>
        </div>
      </div>
      <div class="pm-preview">
        <div style="width:100%;display:flex;justify-content:space-between;color:#94a3b8;font-family:monospace;font-size:0.76rem;margin-bottom:10px;">
          <span id="pm-status">LIVE 144-DPI VECTOR PREVIEW · PAGE 1 OF 1</span>
          <span style="color:#38bdf8;">INTERACTIVE /ACROFORM WIDGETS</span>
        </div>
        <img id="pm-preview-img" src="/api/v1/concepts/aia-g702-g703-progress-billing-draw-schedule/preview.png" alt="Live compiled document preview" width="680" height="880">
      </div>
    </section>

    <section style="margin-bottom:44px;">
      <h2 style="font-size:1.45rem;font-weight:800;margin:0 0 14px;">5 Structural Page Layout &amp; Element Comparison Sheets</h2>
      <div class="pm-showcase-grid">
        <div class="pm-showcase-card">
          <span class="pm-badge">SHEET 00 · 10 PAGE ARCHETYPES</span>
          <a href="/api/v1/concepts/showcase-layouts/preview.png" target="_blank" rel="noopener">
            <img src="/api/v1/concepts/showcase-layouts/preview.png" alt="10 Structural Page Archetype Blueprints" loading="lazy">
          </a>
        </div>
        <div class="pm-showcase-card">
          <span class="pm-badge">SHEET 01 · 6 MASTHEAD ARCHITECTURES</span>
          <a href="/api/v1/concepts/showcase-mastheads/preview.png" target="_blank" rel="noopener">
            <img src="/api/v1/concepts/showcase-mastheads/preview.png" alt="6 Masthead Variations" loading="lazy">
          </a>
        </div>
        <div class="pm-showcase-card">
          <span class="pm-badge">SHEET 02 · KPI STRIPS, TABLES &amp; CHARTS</span>
          <a href="/api/v1/concepts/showcase-tables/preview.png" target="_blank" rel="noopener">
            <img src="/api/v1/concepts/showcase-tables/preview.png" alt="KPI Strips and Tables" loading="lazy">
          </a>
        </div>
        <div class="pm-showcase-card">
          <span class="pm-badge">SHEET 03 · 10 SVG SCHEMATICS &amp; STAMPS</span>
          <a href="/api/v1/concepts/showcase-diagrams/preview.png" target="_blank" rel="noopener">
            <img src="/api/v1/concepts/showcase-diagrams/preview.png" alt="SVG Schematics and Stamps" loading="lazy">
          </a>
        </div>
        <div class="pm-showcase-card">
          <span class="pm-badge">SHEET 04 · 14 SEALS &amp; 3 EXECUTION MODES</span>
          <a href="/api/v1/concepts/showcase-seals/preview.png" target="_blank" rel="noopener">
            <img src="/api/v1/concepts/showcase-seals/preview.png" alt="Vertical Seals and 3 Execution Modes" loading="lazy">
          </a>
        </div>
      </div>
    </section>

    <section>
      <h2 style="font-size:1.45rem;font-weight:800;margin:0 0 12px;">All 100 Profession &amp; Archetype Concepts</h2>
      <div style="display:flex;flex-wrap:wrap;gap:8px;margin-bottom:20px;">
        <input type="search" id="pm-search" placeholder="Search 100 concepts by job, layout, keyword, or statutory citation..." style="flex:1;min-width:250px;padding:8px 12px;border:1.5px solid #94a3b8;border-radius:6px;">
        <button type="button" class="pm-filter-btn active" data-filter="all">All (100)</button>
        <button type="button" class="pm-filter-btn" data-filter="construction-trades">Construction (10)</button>
        <button type="button" class="pm-filter-btn" data-filter="real-estate-leasing">Real Estate (10)</button>
        <button type="button" class="pm-filter-btn" data-filter="property-tax-appeals">Property Tax &amp; Ag (10)</button>
        <button type="button" class="pm-filter-btn" data-filter="software-saas-ai">SaaS &amp; Startup (10)</button>
        <button type="button" class="pm-filter-btn" data-filter="creative-events">Creative &amp; Hospitality (10)</button>
        <button type="button" class="pm-filter-btn" data-filter="legal-cpa-advisory">Legal &amp; Estate (10)</button>
        <button type="button" class="pm-filter-btn" data-filter="medical-clinical">Medical &amp; Biotech (10)</button>
        <button type="button" class="pm-filter-btn" data-filter="logistics-freight">Logistics, Aviation &amp; Auto (10)</button>
        <button type="button" class="pm-filter-btn" data-filter="solar-inspection">Solar &amp; Engineering (10)</button>
        <button type="button" class="pm-filter-btn" data-filter="consulting-govcon">Education, Non-Profit &amp; GovCon (10)</button>
      </div>
      <div class="pm-grid">
        {cards_html}
      </div>
    </section>
  </main>

  <script>
    (function() {{
      const conceptSel = document.getElementById('pm-concept');
      const layoutSel = document.getElementById('pm-layout');
      const mastheadSel = document.getElementById('pm-masthead');
      const kpiSel = document.getElementById('pm-kpi');
      const diagramSel = document.getElementById('pm-diagram');
      const tableSel = document.getElementById('pm-table');
      const presetSel = document.getElementById('pm-preset');
      const execSel = document.getElementById('pm-exec');
      const titleInp = document.getElementById('pm-title');
      const orgInp = document.getElementById('pm-org');
      const cpInp = document.getElementById('pm-cp');
      const stampInp = document.getElementById('pm-stamp');
      const kpi0ValInp = document.getElementById('pm-kpi0-val');
      const item0DescInp = document.getElementById('pm-item0-desc');
      const item0AmtInp = document.getElementById('pm-item0-amt');
      const renderBtn = document.getElementById('pm-render-btn');
      const previewImg = document.getElementById('pm-preview-img');
      const pdfLink = document.getElementById('pm-pdf-link');
      const statusSpan = document.getElementById('pm-status');

      function refresh() {{
        const slug = conceptSel.value;
        const p = new URLSearchParams();
        p.set('layout', layoutSel.value);
        p.set('masthead', mastheadSel.value);
        p.set('kpi', kpiSel.value);
        p.set('diagram', diagramSel.value);
        p.set('table', tableSel.value);
        p.set('preset', presetSel.value);
        p.set('execution', execSel.value);
        if (titleInp.value.trim()) p.set('custom_title', titleInp.value.trim());
        if (orgInp.value.trim()) p.set('custom_org', orgInp.value.trim());
        if (cpInp.value.trim()) p.set('custom_counterparty', cpInp.value.trim());
        if (stampInp.value.trim()) p.set('stamp', stampInp.value.trim());
        if (kpi0ValInp.value.trim()) p.set('kpi0_val', kpi0ValInp.value.trim());
        if (item0DescInp.value.trim()) p.set('item0_desc', item0DescInp.value.trim());
        if (item0AmtInp.value.trim()) p.set('item0_amt', item0AmtInp.value.trim());
        const qs = p.toString();
        statusSpan.textContent = 'COMPILING VECTOR PREVIEW...';
        previewImg.onload = function() {{ statusSpan.textContent = 'LIVE 144-DPI VECTOR PREVIEW · PAGE 1 OF 1'; }};
        previewImg.src = '/api/v1/concepts/' + encodeURIComponent(slug) + '/preview.png?' + qs;
        pdfLink.href = '/api/v1/concepts/' + encodeURIComponent(slug) + '/sample.pdf?' + qs;
      }}

      conceptSel.addEventListener('change', function() {{
        const opt = conceptSel.options[conceptSel.selectedIndex];
        if (opt) {{
          layoutSel.value = opt.getAttribute('data-layout') || 'executive_draw_ledger';
          mastheadSel.value = opt.getAttribute('data-masthead') || 'executive_split';
          kpiSel.value = opt.getAttribute('data-kpi') || 'metric_cards';
          tableSel.value = opt.getAttribute('data-table') || 'dark_header';
          diagramSel.value = opt.getAttribute('data-diagram') || 'floorplan_spec';
          presetSel.value = opt.getAttribute('data-preset') || 'standard';
          execSel.value = opt.getAttribute('data-exec') || 'wet-ink';
          titleInp.value = opt.getAttribute('data-title') || '';
          orgInp.value = opt.getAttribute('data-org') || '';
          cpInp.value = opt.getAttribute('data-counterparty') || '';
          stampInp.value = opt.getAttribute('data-stamp') || '';
          kpi0ValInp.value = opt.getAttribute('data-kpi0-val') || '';
          item0DescInp.value = opt.getAttribute('data-item0-desc') || '';
          item0AmtInp.value = opt.getAttribute('data-item0-amt') || '';
        }}
        refresh();
      }});
      [layoutSel, mastheadSel, kpiSel, diagramSel, tableSel, presetSel, execSel].forEach(function(el) {{
        el.addEventListener('change', refresh);
      }});
      renderBtn.addEventListener('click', refresh);

      const btns = document.querySelectorAll('.pm-filter-btn');
      const search = document.getElementById('pm-search');
      const cards = document.querySelectorAll('.pm-card');
      let vert = 'all';
      function filter() {{
        const q = (search.value || '').toLowerCase().trim();
        cards.forEach(function(c) {{
          const v = c.getAttribute('data-vertical');
          const s = c.getAttribute('data-search') || '';
          c.style.display = ((vert === 'all' || v === vert) && (!q || s.indexOf(q) !== -1)) ? 'flex' : 'none';
        }});
      }}
      btns.forEach(function(b) {{
        b.addEventListener('click', function() {{
          btns.forEach(function(x) {{ x.classList.remove('active'); }});
          b.classList.add('active');
          vert = b.getAttribute('data-filter') || 'all';
          filter();
        }});
      }});
      search.addEventListener('input', filter);
    }})();
  </script>
</body>
</html>"##,
        concept_options_html = concept_options_html,
        cards_html = cards_html,
    );

    Html(html)
}

/// Dedicated SEO Landing Page for each of the 100 concepts on Pressmark (`/concepts/:slug`).
pub async fn handle_pressmark_concept_detail(Path(slug): Path<String>) -> Response {
    let Some(spec) = find_concept_by_slug(&slug) else {
        return (StatusCode::NOT_FOUND, "Concept not found").into_response();
    };
    render_pressmark_concept_detail_html(spec).into_response()
}

fn render_pressmark_concept_detail_html(spec: &ArchetypeConceptSpec) -> Html<String> {
    let related_html = ARCHETYPE_CONCEPTS_50
        .iter()
        .filter(|c| c.vertical_slug == spec.vertical_slug && c.id != spec.id)
        .map(|c| {
            format!(
                r#"<li style="margin-bottom:8px;"><a href="/concepts/{slug}" style="font-weight:700;color:#1e3a8a;text-decoration:underline;">#{id:02} — {title}</a> <span style="font-family:monospace;font-size:0.78rem;color:#475569;">({citation})</span></li>"#,
                slug = c.slug,
                id = c.id,
                title = html_escape(c.title),
                citation = html_escape(c.statutory_citation),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let kpi_html = spec
        .kpi_metrics
        .iter()
        .map(|(k, v)| {
            format!(
                r#"<div style="border:1px solid #cbd5e1;border-radius:8px;padding:12px;background:#f8fafc;">
  <div style="font-family:monospace;font-size:0.7rem;font-weight:800;color:#475569;">{k}</div>
  <div style="font-size:1.1rem;font-weight:800;color:#0f172a;margin-top:4px;">{v}</div>
</div>"#,
                k = html_escape(k),
                v = html_escape(v),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let line_items_html = spec
        .line_items
        .iter()
        .map(|(code, desc, qty, amt)| {
            format!(
                r#"<tr>
  <td style="font-family:monospace;font-weight:700;padding:8px;border-bottom:1px solid #e2e8f0;">{code}</td>
  <td style="padding:8px;border-bottom:1px solid #e2e8f0;">{desc}</td>
  <td style="font-family:monospace;padding:8px;border-bottom:1px solid #e2e8f0;text-align:right;">{qty}</td>
  <td style="font-family:monospace;font-weight:800;padding:8px;border-bottom:1px solid #e2e8f0;text-align:right;">{amt}</td>
</tr>"#,
                code = html_escape(code),
                desc = html_escape(desc),
                qty = html_escape(qty),
                amt = html_escape(amt),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let keywords_joined = spec.secondary_keywords.join(", ");

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title} — Fillable PDF Template &amp; Live Customizer | Pressmark</title>
  <meta name="description" content="Download and customize a fillable PDF {title} ({citation}) for {profession}. Includes 10 structural page archetypes, embedded SVG schematics, and /AcroForm signature fields.">
  <link rel="canonical" href="https://pressmark.dev/concepts/{slug}">
  <meta property="og:type" content="article">
  <meta property="og:title" content="{title} — Fillable PDF Template | Pressmark">
  <meta property="og:description" content="{signal}">
  <meta property="og:url" content="https://pressmark.dev/concepts/{slug}">
  <meta property="og:image" content="https://pressmark.dev/api/v1/concepts/{slug}/preview.png">
  <script type="application/ld+json">
  {{
    "@context": "https://schema.org",
    "@graph": [
      {{
        "@type": "BreadcrumbList",
        "itemListElement": [
          {{ "@type": "ListItem", "position": 1, "name": "Pressmark", "item": "https://pressmark.dev/" }},
          {{ "@type": "ListItem", "position": 2, "name": "100 Document Concepts", "item": "https://pressmark.dev/elements" }},
          {{ "@type": "ListItem", "position": 3, "name": "{title_json}", "item": "https://pressmark.dev/concepts/{slug}" }}
        ]
      }},
      {{
        "@type": "SoftwareApplication",
        "name": "{title_json} Generator",
        "applicationCategory": "BusinessApplication",
        "operatingSystem": "Web, PDF 1.7 (/AcroForm)",
        "description": "Fillable PDF and live layout customizer for {title_json} ({citation_json}) built for {prof_json}.",
        "keywords": "{kw_json}",
        "offers": {{ "@type": "Offer", "price": "0.00", "priceCurrency": "USD" }}
      }}
    ]
  }}
  </script>
  <style>
    body {{ background:#f8fafc; color:#0f172a; font-family:'Inter', -apple-system, BlinkMacSystemFont, sans-serif; margin:0; line-height:1.5; }}
    .pm-shell {{ max-width:1240px; margin:0 auto; padding:28px 16px 64px; }}
    .pm-grid-2 {{ display:grid; grid-template-columns:400px 1fr; gap:24px; align-items:start; }}
    @media (max-width: 960px) {{ .pm-grid-2 {{ grid-template-columns:1fr; }} }}
    .pm-field {{ margin-bottom:10px; }}
    .pm-field label {{ display:block; font-family:monospace; font-size:0.74rem; font-weight:700; text-transform:uppercase; margin-bottom:4px; color:#334155; }}
    .pm-field select, .pm-field input {{ width:100%; padding:8px 10px; border:1.5px solid #94a3b8; border-radius:6px; font-weight:600; font-size:0.86rem; box-sizing:border-box; }}
    .pm-btn {{ display:inline-block; padding:9px 16px; border-radius:6px; font-weight:700; font-size:0.85rem; text-decoration:none; cursor:pointer; border:none; }}
    .pm-btn--primary {{ background:#1e3a8a; color:#fff; }}
    .pm-btn--secondary {{ background:#0284c7; color:#fff; }}
  </style>
</head>
<body>
  <header style="background:#0f172a;color:#fff;padding:14px 20px;">
    <div style="max-width:1240px;margin:0 auto;display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:10px;">
      <a href="/elements" style="color:#38bdf8;font-weight:800;text-decoration:none;">← All 100 Profession Concepts</a>
      <a href="/api/v1/concepts/{slug}/sample.pdf" target="_blank" rel="noopener" style="color:#fff;font-family:monospace;font-weight:700;">Download Fillable PDF ↗</a>
    </div>
  </header>
  <main class="pm-shell">
    <div style="background:#fff;border:1px solid #cbd5e1;border-radius:10px;padding:24px;margin-bottom:24px;box-shadow:0 4px 12px rgba(15,23,42,0.05);">
      <div style="display:flex;align-items:center;gap:14px;flex-wrap:wrap;">
        <img src="/assets/seals/{seal}" alt="{vertical} seal" width="56" height="56" style="border:1.5px solid #cbd5e1;border-radius:50%;background:#fff;">
        <div>
          <span style="background:#dbeafe;color:#1e3a8a;font-family:monospace;font-size:0.74rem;font-weight:800;padding:3px 8px;border-radius:4px;">CONCEPT #{id:02} OF 100 · {vertical} · LAYOUT: {def_layout}</span>
          <h1 style="margin:8px 0 4px;font-size:clamp(1.45rem, 2.8vw, 2.05rem);">{title}</h1>
          <div style="font-family:monospace;font-size:0.82rem;color:#475569;">{citation} · Built for {profession}</div>
        </div>
      </div>
      <p style="margin:14px 0 0;color:#334155;max-width:78ch;"><strong>Why professionals use this template:</strong> {signal}</p>
    </div>

    <div class="pm-grid-2" style="margin-bottom:32px;">
      <div style="background:#fff;border:1px solid #cbd5e1;border-radius:10px;padding:20px;box-shadow:0 4px 12px rgba(15,23,42,0.05);">
        <h2 style="margin:0 0 12px;font-size:1.1rem;font-weight:800;">Customize Layout &amp; Line Items Live</h2>
        <div class="pm-field">
          <label for="dt-layout">Structural Page Layout Blueprint (10 Archetypes)</label>
          <select id="dt-layout">
            <option value="{def_layout}">Default ({def_layout})</option>
            <option value="asymmetric_sidebar">01 · Asymmetric Sidebar Ledger</option>
            <option value="pleading_caption">02 · Numbered Legal Pleading Caption</option>
            <option value="three_tier_packages">03 · 3-Tier Package Comparison Cards</option>
            <option value="inspection_checklist">04 · Multi-Point Inspection Checklist</option>
            <option value="blueprint_schematic">05 · Engineering Blueprint &amp; CAD Sheet</option>
            <option value="split_settlement">06 · Bilateral T-Account Split Settlement</option>
            <option value="clinical_box_grid">07 · Clinical / Regulatory Boxed Form Grid</option>
            <option value="certificate_broadsheet">08 · Ceremonial Certificate Broadsheet</option>
            <option value="dark_tactile_brief">09 · Dark Tactile 3D Depth Brief</option>
            <option value="executive_draw_ledger">10 · Executive Draw &amp; SOV Ledger</option>
          </select>
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="dt-masthead">Masthead Style</label>
            <select id="dt-masthead">
              <option value="{def_masthead}">Default ({def_masthead})</option>
              <option value="trade_banner">Trade Command (Dark)</option>
              <option value="neo_brutalist">Neo-Brutalist (#FFE500)</option>
              <option value="judicial_caption">Judicial Caption</option>
              <option value="swiss_grid">Swiss Precision Grid</option>
              <option value="editorial_serif">Editorial Serif</option>
              <option value="executive_split">Clinical / Executive Split</option>
            </select>
          </div>
          <div class="pm-field">
            <label for="dt-kpi">KPI Strip Style</label>
            <select id="dt-kpi">
              <option value="{def_kpi}">Default ({def_kpi})</option>
              <option value="metric_cards">Executive Metric Cards</option>
              <option value="dark_terminal">Dark Terminal Telemetry</option>
              <option value="neo_brutalist">Neo-Brutalist Hard Border</option>
            </select>
          </div>
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="dt-diagram">Embedded SVG Schematic</label>
            <select id="dt-diagram">
              <option value="{def_diagram}">Default ({def_diagram})</option>
              <option value="parcel_lot">Cadastral Parcel &amp; Setback Plat</option>
              <option value="roof_pitch">Roof Pitch &amp; Hail Impact</option>
              <option value="hvac_diagnostic">HVAC Superheat &amp; Airflow</option>
              <option value="solar_array">Solar PV + LFP Storage</option>
              <option value="vehicle_inspection">Fleet Chassis Diagnostic</option>
              <option value="floorplan_spec">Commercial Suite RSF Zoning</option>
              <option value="soil_topography">Soil Horizon &amp; LiDAR Slope</option>
              <option value="cap_table_waterfall">Cap Table &amp; SAFE Waterfall</option>
              <option value="chain_of_custody">ISO Lab Chain of Custody</option>
              <option value="logistics_route">Intermodal Freight Corridor</option>
            </select>
          </div>
          <div class="pm-field">
            <label for="dt-table">Schedule Table Style</label>
            <select id="dt-table">
              <option value="{def_table}">Default ({def_table})</option>
              <option value="dark_header">Dark Header + Zebra</option>
              <option value="striped">Primary Header + Striped</option>
              <option value="minimal_lines">Minimal Editorial Rules</option>
            </select>
          </div>
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="dt-preset">Readability Preset</label>
            <select id="dt-preset">
              <option value="{def_preset}">Default ({def_preset})</option>
              <option value="standard">Standard (9.5pt Body)</option>
              <option value="high_legibility">High-Legibility (10.4pt Large)</option>
              <option value="executive_compact">Executive Compact (8.8pt)</option>
              <option value="judicial_formal">Judicial Formal (9.5pt)</option>
            </select>
          </div>
          <div class="pm-field">
            <label for="dt-exec">Signature Execution Mode</label>
            <select id="dt-exec">
              <option value="{def_exec}">Default ({def_exec})</option>
              <option value="wet-ink">Fillable PDF (/AcroForm)</option>
              <option value="esign">Electronic Counter-Sign (/s/)</option>
              <option value="notary_jurat">Sworn Notary Public Jurat</option>
            </select>
          </div>
        </div>
        <div class="pm-field">
          <label for="dt-title">Document Title Override</label>
          <input type="text" id="dt-title" value="{title}">
        </div>
        <div class="pm-field">
          <label for="dt-org">Issuing Organization / Firm</label>
          <input type="text" id="dt-org" value="{org}">
        </div>
        <div class="pm-field">
          <label for="dt-cp">Counterparty / Client / Recipient</label>
          <input type="text" id="dt-cp" value="{cp}">
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="dt-stamp">Vector Rubber Stamp Text</label>
            <input type="text" id="dt-stamp" value="{stamp}">
          </div>
          <div class="pm-field">
            <label for="dt-kpi0-val">Primary KPI Value ({kpi0_lbl})</label>
            <input type="text" id="dt-kpi0-val" value="{kpi0_val}">
          </div>
        </div>
        <div style="display:grid;grid-template-columns:2fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="dt-item0-desc">Schedule Line-Item #1 Description</label>
            <input type="text" id="dt-item0-desc" value="{item0_desc}">
          </div>
          <div class="pm-field">
            <label for="dt-item0-amt">Line-Item #1 Amount</label>
            <input type="text" id="dt-item0-amt" value="{item0_amt}">
          </div>
        </div>
        <div style="display:flex;gap:10px;flex-wrap:wrap;margin-top:14px;">
          <button type="button" id="dt-render-btn" class="pm-btn pm-btn--primary" style="flex:1;">Update Preview</button>
          <a id="dt-pdf-btn" href="/api/v1/concepts/{slug}/sample.pdf" target="_blank" rel="noopener" class="pm-btn pm-btn--secondary">Download Fillable PDF ↗</a>
        </div>
      </div>

      <div style="background:#0f172a;border-radius:10px;padding:16px;display:flex;flex-direction:column;align-items:center;">
        <div style="width:100%;display:flex;justify-content:space-between;color:#cbd5e1;font-family:monospace;font-size:0.76rem;margin-bottom:10px;">
          <span id="dt-status">144-DPI VECTOR PREVIEW · PAGE 1 OF 1</span>
          <span style="color:#38bdf8;">ACROFORM READY</span>
        </div>
        <img id="dt-preview-img" src="/api/v1/concepts/{slug}/preview.png" alt="{title} preview" style="max-width:100%;height:auto;border-radius:4px;background:#fff;">
      </div>
    </div>

    <section style="background:#fff;border:1px solid #cbd5e1;border-radius:10px;padding:24px;margin-bottom:24px;">
      <h2 style="margin:0 0 10px;font-size:1.25rem;font-weight:800;">Included Financial Metrics &amp; Line-Item Schedule</h2>
      <p style="margin:0 0 14px;color:#475569;font-size:0.9rem;"><strong>Target Roles:</strong> {profession} · <strong>Search Keywords:</strong> {primary_kw}, {secondary_kws}</p>
      <div style="display:grid;grid-template-columns:repeat(auto-fit, minmax(210px, 1fr));gap:12px;margin-bottom:20px;">
        {kpi_html}
      </div>
      <div style="overflow-x:auto;">
        <table style="width:100%;border-collapse:collapse;border:1px solid #cbd5e1;font-size:0.88rem;">
          <thead>
            <tr style="background:#0f172a;color:#fff;text-align:left;font-family:monospace;font-size:0.76rem;">
              <th style="padding:8px;">CODE</th>
              <th style="padding:8px;">DESCRIPTION &amp; TECHNICAL SPECIFICATION</th>
              <th style="padding:8px;text-align:right;">BASIS / QTY</th>
              <th style="padding:8px;text-align:right;">AMOUNT / DELTA</th>
            </tr>
          </thead>
          <tbody>
            {line_items_html}
          </tbody>
        </table>
      </div>
    </section>

    <section style="background:#fff;border:1px solid #cbd5e1;border-radius:10px;padding:24px;">
      <h2 style="margin:0 0 10px;font-size:1.15rem;font-weight:800;">Related {vertical} Templates</h2>
      <ul style="margin:0;padding-left:20px;">
        {related_html}
      </ul>
    </section>
  </main>

  <script>
    (function() {{
      const slug = "{slug}";
      const layout = document.getElementById('dt-layout');
      const masthead = document.getElementById('dt-masthead');
      const kpi = document.getElementById('dt-kpi');
      const diagram = document.getElementById('dt-diagram');
      const table = document.getElementById('dt-table');
      const preset = document.getElementById('dt-preset');
      const exec = document.getElementById('dt-exec');
      const titleInp = document.getElementById('dt-title');
      const org = document.getElementById('dt-org');
      const cp = document.getElementById('dt-cp');
      const stamp = document.getElementById('dt-stamp');
      const kpi0Val = document.getElementById('dt-kpi0-val');
      const item0Desc = document.getElementById('dt-item0-desc');
      const item0Amt = document.getElementById('dt-item0-amt');
      const btn = document.getElementById('dt-render-btn');
      const img = document.getElementById('dt-preview-img');
      const pdf = document.getElementById('dt-pdf-btn');
      const status = document.getElementById('dt-status');

      function update() {{
        const p = new URLSearchParams();
        p.set('layout', layout.value);
        p.set('masthead', masthead.value);
        p.set('kpi', kpi.value);
        p.set('diagram', diagram.value);
        p.set('table', table.value);
        p.set('preset', preset.value);
        p.set('execution', exec.value);
        if (titleInp.value.trim()) p.set('custom_title', titleInp.value.trim());
        if (org.value.trim()) p.set('custom_org', org.value.trim());
        if (cp.value.trim()) p.set('custom_counterparty', cp.value.trim());
        if (stamp.value.trim()) p.set('stamp', stamp.value.trim());
        if (kpi0Val.value.trim()) p.set('kpi0_val', kpi0Val.value.trim());
        if (item0Desc.value.trim()) p.set('item0_desc', item0Desc.value.trim());
        if (item0Amt.value.trim()) p.set('item0_amt', item0Amt.value.trim());
        const qs = p.toString();
        status.textContent = 'RENDERING LIVE PREVIEW...';
        img.onload = function() {{ status.textContent = '144-DPI VECTOR PREVIEW · PAGE 1 OF 1'; }};
        img.src = '/api/v1/concepts/' + slug + '/preview.png?' + qs;
        pdf.href = '/api/v1/concepts/' + slug + '/sample.pdf?' + qs;
      }}

      [layout, masthead, kpi, diagram, table, preset, exec].forEach(function(el) {{
        el.addEventListener('change', update);
      }});
      btn.addEventListener('click', update);
    }})();
  </script>
</body>
</html>"##,
        title = html_escape(spec.title),
        title_json = spec.title.replace('"', "\\\""),
        citation = html_escape(spec.statutory_citation),
        citation_json = spec.statutory_citation.replace('"', "\\\""),
        profession = html_escape(spec.target_profession),
        prof_json = spec.target_profession.replace('"', "\\\""),
        slug = spec.slug,
        signal = html_escape(spec.demand_signal),
        kw_json = format!("{}, {}", spec.primary_keyword, keywords_joined).replace('"', "\\\""),
        seal = spec.seal_filename,
        vertical = html_escape(spec.vertical),
        id = spec.id,
        def_layout = spec.default_page_layout,
        def_masthead = spec.default_masthead,
        def_kpi = spec.default_kpi_style,
        def_diagram = spec.default_diagram_kind,
        def_table = spec.default_table_style,
        def_preset = spec.default_readability.as_typst_id(),
        def_exec = spec.default_execution_mode,
        org = html_escape(spec.sample_org),
        cp = html_escape(spec.sample_counterparty),
        stamp = html_escape(spec.stamp_label),
        kpi0_lbl = html_escape(spec.kpi_metrics[0].0),
        kpi0_val = html_escape(spec.kpi_metrics[0].1),
        item0_desc = html_escape(spec.line_items[0].1),
        item0_amt = html_escape(spec.line_items[0].3),
        primary_kw = html_escape(spec.primary_keyword),
        secondary_kws = html_escape(&keywords_joined),
        kpi_html = kpi_html,
        line_items_html = line_items_html,
        related_html = related_html,
    );
    Html(html)
}
