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
    let mut concept_options_html = String::with_capacity(8192);
    let mut cards_html = String::with_capacity(32768);

    for spec in ARCHETYPE_CONCEPTS_50 {
        concept_options_html.push_str(&format!(
            r#"<option value="{slug}" data-masthead="{masthead}" data-kpi="{kpi}" data-table="{table}" data-diagram="{diagram}" data-preset="{preset}" data-exec="{exec}" data-org="{org}" data-counterparty="{cp}" data-stamp="{stamp}">#{id:02} — {title} ({vertical})</option>"#,
            slug = spec.slug,
            masthead = spec.default_masthead,
            kpi = spec.default_kpi_style,
            table = spec.default_table_style,
            diagram = spec.default_diagram_kind,
            preset = spec.default_readability.as_typst_id(),
            exec = spec.default_execution_mode,
            org = html_escape(spec.sample_org),
            cp = html_escape(spec.sample_counterparty),
            stamp = html_escape(spec.stamp_label),
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
                "{} {} {} {} {}",
                spec.title.to_lowercase(),
                spec.vertical.to_lowercase(),
                spec.target_profession.to_lowercase(),
                spec.primary_keyword.to_lowercase(),
                spec.statutory_citation.to_lowercase()
            )),
            seal = spec.seal_filename,
            vertical = html_escape(spec.vertical),
            id = spec.id,
            citation = html_escape(spec.statutory_citation),
            slug = spec.slug,
            title = html_escape(spec.title),
            profession = html_escape(spec.target_profession),
            signal = html_escape(spec.demand_signal),
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
  <title>50 Profession Document Concepts &amp; Element Variations Studio | Pressmark</title>
  <meta name="description" content="Interactive vector PDF element customizer and 50-profession template library across construction, real estate, property tax, SaaS, creative, legal, medical, logistics, solar, and GovCon.">
  <link rel="canonical" href="https://pressmark.dev/elements">
  <meta property="og:type" content="website">
  <meta property="og:title" content="50 Profession Document Concepts &amp; Element Variations Studio | Pressmark">
  <meta property="og:description" content="Switch mastheads, KPI strips, embedded technical SVG schematics, readability presets, and fillable /AcroForm signatures live.">
  <meta property="og:url" content="https://pressmark.dev/elements">
  <meta property="og:image" content="https://pressmark.dev/assets/seals/showcase_sheet_mastheads.png">
  <script type="application/ld+json">
  {{
    "@context": "https://schema.org",
    "@graph": [
      {{
        "@type": "CollectionPage",
        "@id": "https://pressmark.dev/elements#webpage",
        "url": "https://pressmark.dev/elements",
        "name": "50 Profession Document Concepts & Element Variations Studio",
        "description": "Vector PDF synthesis library spanning 50 profession archetypes with embedded SVG schematics and interactive /AcroForm signature blocks."
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
    .pm-studio {{ display:grid; grid-template-columns:380px 1fr; gap:24px; background:#fff; border:1px solid #cbd5e1; border-radius:10px; padding:24px; margin-bottom:40px; box-shadow:0 4px 16px rgba(15,23,42,0.06); }}
    @media (max-width: 960px) {{ .pm-studio {{ grid-template-columns:1fr; }} }}
    .pm-field {{ margin-bottom:12px; }}
    .pm-field label {{ display:block; font-family:monospace; font-size:0.75rem; font-weight:700; text-transform:uppercase; margin-bottom:4px; color:#334155; }}
    .pm-field select, .pm-field input {{ width:100%; padding:8px 10px; border:1.5px solid #94a3b8; border-radius:6px; font-size:0.88rem; font-weight:600; box-sizing:border-box; }}
    .pm-preview {{ background:#0f172a; border-radius:8px; padding:16px; display:flex; flex-direction:column; align-items:center; }}
    .pm-preview img {{ max-width:100%; height:auto; border-radius:4px; background:#fff; }}
    .pm-showcase-grid {{ display:grid; grid-template-columns:repeat(auto-fit, minmax(280px, 1fr)); gap:20px; margin-bottom:40px; }}
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
        <a href="/elements" style="color:#38bdf8;text-decoration:underline;">50 Profession Concepts &amp; Elements</a>
        <a href="/pricing" style="color:#cbd5e1;text-decoration:none;">Pricing</a>
      </nav>
    </div>
  </header>

  <main class="pm-shell">
    <section class="pm-hero">
      <span style="background:#0284c7;color:#fff;font-family:monospace;font-size:0.76rem;font-weight:800;padding:3px 10px;border-radius:4px;">SHARED HALLMAN DOC ENGINE · 50 PROFESSION CONCEPTS</span>
      <h1 style="font-size:clamp(1.8rem, 3.2vw, 2.5rem);margin:12px 0 8px;">Multi-Style Document Elements, Embedded SVG Schematics &amp; 50 Profession Templates</h1>
      <p style="max-width:74ch;color:#cbd5e1;line-height:1.55;margin:0;">Switch between 6 masthead architectures, 3 KPI metric strips, 6 domain SVG technical diagrams, 3 schedule table styles, 4 readability presets, and fillable /AcroForm signature widgets in real time.</p>
    </section>

    <section class="pm-studio">
      <div>
        <div class="pm-field">
          <label for="pm-concept">1. Profession Concept (1–50)</label>
          <select id="pm-concept">{concept_options_html}</select>
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="pm-masthead">2. Masthead Style</label>
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
            <label for="pm-kpi">3. KPI Strip Style</label>
            <select id="pm-kpi">
              <option value="metric_cards">Executive Metric Cards</option>
              <option value="dark_terminal">Dark Terminal Telemetry</option>
              <option value="neo_brutalist">Neo-Brutalist Hard Border</option>
            </select>
          </div>
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="pm-diagram">4. Embedded SVG Diagram</label>
            <select id="pm-diagram">
              <option value="floorplan_spec">Commercial Suite RSF Zoning</option>
              <option value="parcel_lot">Cadastral Parcel &amp; Setback Plat</option>
              <option value="roof_pitch">Roof Pitch &amp; Hail Impact</option>
              <option value="hvac_diagnostic">HVAC Superheat &amp; Airflow</option>
              <option value="solar_array">Solar PV + LFP Storage</option>
              <option value="vehicle_inspection">Fleet Chassis Diagnostic</option>
            </select>
          </div>
          <div class="pm-field">
            <label for="pm-table">5. Schedule Table Style</label>
            <select id="pm-table">
              <option value="dark_header">Dark Header + Zebra</option>
              <option value="striped">Primary Header + Striped</option>
              <option value="minimal_lines">Minimal Editorial Rules</option>
            </select>
          </div>
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="pm-preset">6. Readability Preset</label>
            <select id="pm-preset">
              <option value="standard">Standard (9.5pt Body)</option>
              <option value="high_legibility">High-Legibility (10.4pt Large)</option>
              <option value="executive_compact">Executive Compact (8.8pt)</option>
              <option value="judicial_formal">Judicial Formal (9.5pt)</option>
            </select>
          </div>
          <div class="pm-field">
            <label for="pm-exec">7. Signature Mode</label>
            <select id="pm-exec">
              <option value="wet-ink">Fillable PDF (/AcroForm)</option>
              <option value="esign">Electronic Counter-Sign (/s/)</option>
              <option value="notary_jurat">Sworn Notary Public Jurat</option>
            </select>
          </div>
        </div>
        <div class="pm-field">
          <label for="pm-org">8. Issuing Organization</label>
          <input type="text" id="pm-org" value="Lookout Valley Commercial Builders LLC">
        </div>
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;">
          <div class="pm-field">
            <label for="pm-cp">9. Counterparty / Client</label>
            <input type="text" id="pm-cp" value="Riverfront Mixed-Use DevelopmentSPE LLC">
          </div>
          <div class="pm-field">
            <label for="pm-stamp">10. Vector Rubber Stamp</label>
            <input type="text" id="pm-stamp" value="DRAW #04 APPROVED">
          </div>
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
      <h2 style="font-size:1.45rem;font-weight:800;margin:0 0 14px;">4 Multi-Variant Element Comparison Sheets</h2>
      <div class="pm-showcase-grid">
        <div class="pm-showcase-card">
          <span class="pm-badge">SHEET 01 · 6 MASTHEAD ARCHITECTURES</span>
          <a href="/assets/seals/showcase_sheet_mastheads.png" target="_blank" rel="noopener">
            <img src="/assets/seals/showcase_sheet_mastheads.png" alt="6 Masthead Variations" loading="lazy">
          </a>
        </div>
        <div class="pm-showcase-card">
          <span class="pm-badge">SHEET 02 · KPI STRIPS, TABLES &amp; CHARTS</span>
          <a href="/assets/seals/showcase_sheet_tables_and_kpis.png" target="_blank" rel="noopener">
            <img src="/assets/seals/showcase_sheet_tables_and_kpis.png" alt="KPI Strips and Tables" loading="lazy">
          </a>
        </div>
        <div class="pm-showcase-card">
          <span class="pm-badge">SHEET 03 · 6 SVG SCHEMATICS &amp; STAMPS</span>
          <a href="/assets/seals/showcase_sheet_diagrams_and_stamps.png" target="_blank" rel="noopener">
            <img src="/assets/seals/showcase_sheet_diagrams_and_stamps.png" alt="6 SVG Schematics and Stamps" loading="lazy">
          </a>
        </div>
        <div class="pm-showcase-card">
          <span class="pm-badge">SHEET 04 · 10 SEALS &amp; 3 EXECUTION MODES</span>
          <a href="/assets/seals/showcase_sheet_signatures_and_seals.png" target="_blank" rel="noopener">
            <img src="/assets/seals/showcase_sheet_signatures_and_seals.png" alt="10 Vertical Seals and 3 Execution Modes" loading="lazy">
          </a>
        </div>
      </div>
    </section>

    <section>
      <h2 style="font-size:1.45rem;font-weight:800;margin:0 0 12px;">All 50 Profession &amp; Archetype Concepts</h2>
      <div style="display:flex;flex-wrap:wrap;gap:8px;margin-bottom:20px;">
        <input type="search" id="pm-search" placeholder="Search 50 concepts by job, keyword, or statutory citation..." style="flex:1;min-width:250px;padding:8px 12px;border:1.5px solid #94a3b8;border-radius:6px;">
        <button type="button" class="pm-filter-btn active" data-filter="all">All (50)</button>
        <button type="button" class="pm-filter-btn" data-filter="construction-trades">Construction</button>
        <button type="button" class="pm-filter-btn" data-filter="real-estate-leasing">Real Estate</button>
        <button type="button" class="pm-filter-btn" data-filter="property-tax-appeals">Property Tax</button>
        <button type="button" class="pm-filter-btn" data-filter="software-saas-ai">SaaS &amp; AI</button>
        <button type="button" class="pm-filter-btn" data-filter="creative-events">Creative</button>
        <button type="button" class="pm-filter-btn" data-filter="legal-cpa-advisory">Legal &amp; CPA</button>
        <button type="button" class="pm-filter-btn" data-filter="medical-clinical">Medical</button>
        <button type="button" class="pm-filter-btn" data-filter="logistics-freight">Logistics</button>
        <button type="button" class="pm-filter-btn" data-filter="solar-inspection">Solar &amp; Appraisal</button>
        <button type="button" class="pm-filter-btn" data-filter="consulting-govcon">HR &amp; GovCon</button>
      </div>
      <div class="pm-grid">
        {cards_html}
      </div>
    </section>
  </main>

  <script>
    (function() {{
      const conceptSel = document.getElementById('pm-concept');
      const mastheadSel = document.getElementById('pm-masthead');
      const kpiSel = document.getElementById('pm-kpi');
      const diagramSel = document.getElementById('pm-diagram');
      const tableSel = document.getElementById('pm-table');
      const presetSel = document.getElementById('pm-preset');
      const execSel = document.getElementById('pm-exec');
      const orgInp = document.getElementById('pm-org');
      const cpInp = document.getElementById('pm-cp');
      const stampInp = document.getElementById('pm-stamp');
      const renderBtn = document.getElementById('pm-render-btn');
      const previewImg = document.getElementById('pm-preview-img');
      const pdfLink = document.getElementById('pm-pdf-link');
      const statusSpan = document.getElementById('pm-status');

      function refresh() {{
        const slug = conceptSel.value;
        const p = new URLSearchParams();
        p.set('masthead', mastheadSel.value);
        p.set('kpi', kpiSel.value);
        p.set('diagram', diagramSel.value);
        p.set('table', tableSel.value);
        p.set('preset', presetSel.value);
        p.set('execution', execSel.value);
        if (orgInp.value.trim()) p.set('custom_org', orgInp.value.trim());
        if (cpInp.value.trim()) p.set('custom_counterparty', cpInp.value.trim());
        if (stampInp.value.trim()) p.set('stamp', stampInp.value.trim());
        const qs = p.toString();
        statusSpan.textContent = 'COMPILING VECTOR PREVIEW...';
        previewImg.onload = function() {{ statusSpan.textContent = 'LIVE 144-DPI VECTOR PREVIEW · PAGE 1 OF 1'; }};
        previewImg.src = '/api/v1/concepts/' + encodeURIComponent(slug) + '/preview.png?' + qs;
        pdfLink.href = '/api/v1/concepts/' + encodeURIComponent(slug) + '/sample.pdf?' + qs;
      }}

      conceptSel.addEventListener('change', function() {{
        const opt = conceptSel.options[conceptSel.selectedIndex];
        if (opt) {{
          mastheadSel.value = opt.getAttribute('data-masthead') || 'executive_split';
          kpiSel.value = opt.getAttribute('data-kpi') || 'metric_cards';
          tableSel.value = opt.getAttribute('data-table') || 'dark_header';
          diagramSel.value = opt.getAttribute('data-diagram') || 'floorplan_spec';
          presetSel.value = opt.getAttribute('data-preset') || 'standard';
          execSel.value = opt.getAttribute('data-exec') || 'wet-ink';
          orgInp.value = opt.getAttribute('data-org') || '';
          cpInp.value = opt.getAttribute('data-counterparty') || '';
          stampInp.value = opt.getAttribute('data-stamp') || '';
        }}
        refresh();
      }});
      [mastheadSel, kpiSel, diagramSel, tableSel, presetSel, execSel].forEach(function(el) {{
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

/// Dedicated SEO Landing Page for each of the 50 concepts on Pressmark (`/concepts/:slug`).
pub async fn handle_pressmark_concept_detail(Path(slug): Path<String>) -> Response {
    let Some(spec) = find_concept_by_slug(&slug) else {
        return (StatusCode::NOT_FOUND, "Concept not found").into_response();
    };
    render_pressmark_concept_detail_html(spec).into_response()
}

fn render_pressmark_concept_detail_html(spec: &ArchetypeConceptSpec) -> Html<String> {
    let html = format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title} — Fillable PDF Template &amp; Live Customizer | Pressmark</title>
  <meta name="description" content="Customize and download a fillable PDF {title} ({citation}) for {profession}.">
  <link rel="canonical" href="https://pressmark.dev/concepts/{slug}">
  <meta property="og:type" content="article">
  <meta property="og:title" content="{title} — Fillable PDF Template | Pressmark">
  <meta property="og:description" content="{signal}">
  <meta property="og:url" content="https://pressmark.dev/concepts/{slug}">
  <meta property="og:image" content="https://pressmark.dev/api/v1/concepts/{slug}/preview.png">
  <style>
    body {{ background:#f8fafc; color:#0f172a; font-family:'Inter', -apple-system, BlinkMacSystemFont, sans-serif; margin:0; }}
    .pm-shell {{ max-width:1180px; margin:0 auto; padding:28px 16px 64px; }}
    .pm-btn {{ display:inline-block; padding:9px 16px; border-radius:6px; font-weight:700; font-size:0.85rem; text-decoration:none; background:#1e3a8a; color:#fff; }}
  </style>
</head>
<body>
  <header style="background:#0f172a;color:#fff;padding:14px 20px;">
    <div style="max-width:1180px;margin:0 auto;display:flex;justify-content:space-between;align-items:center;">
      <a href="/elements" style="color:#38bdf8;font-weight:800;text-decoration:none;">← All 50 Profession Concepts</a>
      <a href="/api/v1/concepts/{slug}/sample.pdf" target="_blank" rel="noopener" style="color:#fff;font-family:monospace;font-weight:700;">Download Fillable PDF ↗</a>
    </div>
  </header>
  <main class="pm-shell">
    <div style="background:#fff;border:1px solid #cbd5e1;border-radius:10px;padding:24px;margin-bottom:24px;">
      <div style="display:flex;align-items:center;gap:14px;flex-wrap:wrap;">
        <img src="/assets/seals/{seal}" alt="{vertical} seal" width="56" height="56">
        <div>
          <span style="background:#dbeafe;color:#1e3a8a;font-family:monospace;font-size:0.74rem;font-weight:800;padding:3px 8px;border-radius:4px;">CONCEPT #{id:02} · {vertical}</span>
          <h1 style="margin:8px 0 4px;font-size:1.75rem;">{title}</h1>
          <div style="font-family:monospace;font-size:0.82rem;color:#475569;">{citation} · Built for {profession}</div>
        </div>
      </div>
      <p style="margin:14px 0 16px;color:#334155;">{signal}</p>
      <div style="display:flex;gap:12px;flex-wrap:wrap;">
        <a href="/api/v1/concepts/{slug}/sample.pdf" target="_blank" rel="noopener" class="pm-btn">Download Fillable PDF (/AcroForm) ↗</a>
        <a href="/elements" class="pm-btn" style="background:#0f766e;">Open Live Element Customizer</a>
      </div>
    </div>
    <div style="background:#0f172a;border-radius:10px;padding:20px;text-align:center;">
      <img src="/api/v1/concepts/{slug}/preview.png" alt="{title} preview" style="max-width:100%;height:auto;border-radius:4px;background:#fff;">
    </div>
  </main>
</body>
</html>"##,
        title = html_escape(spec.title),
        citation = html_escape(spec.statutory_citation),
        profession = html_escape(spec.target_profession),
        slug = spec.slug,
        signal = html_escape(spec.demand_signal),
        seal = spec.seal_filename,
        vertical = html_escape(spec.vertical),
        id = spec.id,
    );
    Html(html)
}
