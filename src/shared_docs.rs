//! Pressmark Shareable Recipient Document Links & E-Sign Flow (/v/:token)
//!
//! Author: Daniel Hallman

use crate::engine::{generate_pdf, generate_preview_png};
use crate::models::{DocumentRequest, SignatureBlock};
use crate::seo_pages::{html_escape, shared_page_css};
use crate::server::AppState;
use crate::storage::SharedDocumentRecord;
use axum::extract::{Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::Json;
use chrono::{Duration, Utc};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SignSharedRequest {
    pub signer_name: String,
    pub signer_title: Option<String>,
    pub signature_data: Option<String>,
}

fn noindex_headers(extra_cache: bool) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("x-robots-tag", header::HeaderValue::from_static("noindex, nofollow, noarchive"));
    headers.insert("referrer-policy", header::HeaderValue::from_static("no-referrer"));
    headers.insert("x-content-type-options", header::HeaderValue::from_static("nosniff"));
    if extra_cache {
        headers.insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("private, no-store, max-age=0"),
        );
    }
    headers
}

fn safe_payment_url(raw: Option<&str>) -> Option<String> {
    let trimmed = raw.map(str::trim).filter(|s| !s.is_empty())?;
    let candidate = if trimmed.starts_with("https://") || trimmed.starts_with("http://") {
        trimmed.to_string()
    } else if !trimmed.contains(':') && trimmed.contains('.') {
        format!("https://{}", trimmed)
    } else {
        return None;
    };
    reqwest::Url::parse(&candidate)
        .ok()
        .filter(|u| matches!(u.scheme(), "https" | "http"))
        .map(|u| u.to_string())
}

pub async fn handle_create_share(
    State(state): State<Arc<AppState>>,
    Json(mut doc_req): Json<DocumentRequest>,
) -> impl IntoResponse {
    if let Err(err) = doc_req.normalize_and_validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "error": err
            })),
        )
            .into_response();
    }

    let pdf_bytes = match generate_pdf(&doc_req) {
        Ok((bytes, _)) => bytes,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "success": false,
                    "error": err
                })),
            )
                .into_response();
        }
    };

    let sha256_hex = hex::encode(Sha256::digest(&pdf_bytes));
    let doc_json = match serde_json::to_string(&doc_req) {
        Ok(j) => j,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": e.to_string() })),
            )
                .into_response();
        }
    };

    let now = Utc::now();
    let expires_at = (now + Duration::days(30)).timestamp();
    let token_full = Uuid::new_v4().simple().to_string();
    let token = token_full[..22].to_string();
    let payment_link = safe_payment_url(doc_req.payment_link.as_deref());

    let record = SharedDocumentRecord {
        token: token.clone(),
        doc_json,
        doc_type: doc_req.doc_type.as_str().to_string(),
        doc_title: doc_req.effective_title(),
        doc_number: doc_req.effective_doc_number(),
        from_name: doc_req.from.name.clone(),
        to_name: doc_req.to.name.clone(),
        total_formatted: doc_req.format_amount(doc_req.grand_total()),
        payment_link,
        sha256_hex: sha256_hex.clone(),
        recipient_signed_name: None,
        recipient_signed_at: None,
        created_at: now.timestamp(),
        expires_at,
        view_count: 0,
    };

    if let Err(e) = state.storage.save_shared_document(&record) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "success": false, "error": e.to_string() })),
        )
            .into_response();
    }

    state.storage.record_usage_event(
        "share_link_created",
        Some(doc_req.doc_type.as_str()),
        "studio",
        Some(&token),
        None,
    );

    let base = state.base_url.trim_end_matches('/');
    let share_url = format!("{}/v/{}", base, token);
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "token": token,
            "share_url": share_url,
            "sha256_hex": sha256_hex,
            "expires_at": expires_at
        })),
    )
        .into_response()
}

pub async fn handle_view_shared(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> Response {
    let Some(record) = state.storage.get_shared_document(&token) else {
        return (
            StatusCode::NOT_FOUND,
            noindex_headers(true),
            Html("<h1>Document link expired or not found</h1><p>Ask the sender for a new Pressmark share link.</p>".to_string()),
        )
            .into_response();
    };

    state.storage.increment_shared_view(&record.token);

    let pay_cta = safe_payment_url(record.payment_link.as_deref())
        .map(|u| {
            format!(
                "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\" class=\"pm-btn pm-btn-primary\" style=\"background:#059669;\">Pay Now ({})</a>",
                html_escape(&u),
                html_escape(&record.total_formatted)
            )
        })
        .unwrap_or_default();

    let signed_banner = if let Some(ref signer) = record.recipient_signed_name {
        format!(
            "<div class=\"pm-card\" style=\"border-color:#10b981;background:#ecfdf5;margin-bottom:20px;\">\
             <strong style=\"color:#047857;\">Signed by {}</strong>\
             <span style=\"font-size:13px;color:#065f46;\">Verified SHA-256: <code>{}</code></span>\
             </div>",
            html_escape(signer),
            html_escape(&record.sha256_hex[..record.sha256_hex.len().min(24)])
        )
    } else {
        String::new()
    };

    let sign_section = if record.recipient_signed_name.is_some() {
        String::new()
    } else {
        format!(
            r#"<section class="pm-card" style="margin-top:20px;">
          <h2>Accept &amp; E-Sign Document</h2>
          <p style="margin:0;color:var(--muted);font-size:14px;">Draw your signature or type your legal name to execute and stamp an updated PDF copy.</p>
          <form id="signForm" style="display:flex;flex-direction:column;gap:12px;">
            <div class="pm-form-grid">
              <div class="pm-field"><label for="signerName">Full Legal Name</label><input id="signerName" required placeholder="{to_name}"></div>
              <div class="pm-field"><label for="signerTitle">Title (Optional)</label><input id="signerTitle" placeholder="Authorized Signatory"></div>
            </div>
            <div>
              <label style="font-size:12px;font-weight:600;color:var(--muted);display:block;margin-bottom:6px;">Draw Signature (Optional)</label>
              <canvas id="sigPad" width="460" height="120" style="width:100%;max-width:460px;height:120px;border:1px solid var(--border);border-radius:8px;background:#fff;touch-action:none;"></canvas>
              <div style="margin-top:6px;"><button type="button" id="btnClearSig" class="pm-btn pm-btn-secondary" style="min-height:32px;padding:4px 12px;font-size:12px;">Clear Signature</button></div>
            </div>
            <div class="pm-cta-row">
              <button type="submit" class="pm-btn pm-btn-primary">Sign &amp; Update Document</button>
              <span id="signStatus" style="font-size:13px;color:var(--muted);"></span>
            </div>
          </form>
        </section>
        <script>
          (function(){{
            var canvas = document.getElementById('sigPad');
            var ctx = canvas.getContext('2d');
            var drawing = false, hasStroke = false;
            ctx.strokeStyle = '#0f172a'; ctx.lineWidth = 2.2; ctx.lineCap = 'round';
            function pos(e) {{
              var r = canvas.getBoundingClientRect();
              var p = e.touches ? e.touches[0] : e;
              return {{ x: (p.clientX - r.left) * (canvas.width / r.width), y: (p.clientY - r.top) * (canvas.height / r.height) }};
            }}
            canvas.addEventListener('pointerdown', function(e){{ drawing = true; hasStroke = true; var p = pos(e); ctx.beginPath(); ctx.moveTo(p.x, p.y); }});
            canvas.addEventListener('pointermove', function(e){{ if (!drawing) return; var p = pos(e); ctx.lineTo(p.x, p.y); ctx.stroke(); }});
            window.addEventListener('pointerup', function(){{ drawing = false; }});
            document.getElementById('btnClearSig').addEventListener('click', function(){{ ctx.clearRect(0, 0, canvas.width, canvas.height); hasStroke = false; }});
            document.getElementById('signForm').addEventListener('submit', async function(e){{
              e.preventDefault();
              var st = document.getElementById('signStatus');
              st.textContent = 'Signing...';
              var payload = {{
                signer_name: document.getElementById('signerName').value,
                signer_title: document.getElementById('signerTitle').value || undefined,
                signature_data: hasStroke ? canvas.toDataURL('image/png') : undefined
              }};
              var r = await fetch('/v/{token}/sign', {{ method:'POST', headers:{{'Content-Type':'application/json'}}, body: JSON.stringify(payload) }});
              var d = await r.json().catch(function(){{ return {{}}; }});
              if (r.ok && d.success) {{ window.location.reload(); }}
              else {{ st.textContent = d.error || 'Could not record signature.'; }}
            }});
          }})();
        </script>"#,
            to_name = html_escape(&record.to_name),
            token = html_escape(&record.token)
        )
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title} (#{number}) · Pressmark</title>
  <meta name="robots" content="noindex, nofollow, noarchive">
  <link rel="icon" type="image/svg+xml" href="/assets/icon.svg">
  <style>{css}</style>
</head>
<body>
  <header class="pm-nav">
    <div class="pm-nav-inner">
      <a href="/" class="pm-brand"><img src="/assets/icon.svg" alt="Pressmark"><span>Pressmark</span></a>
      <div class="pm-cta-row">
        <a href="/v/{token}/pdf" class="pm-btn pm-btn-secondary">Download PDF</a>
        {pay_cta}
      </div>
    </div>
  </header>
  <main class="pm-wrap" style="max-width:860px;">
    {signed_banner}
    <section class="pm-hero">
      <span class="pm-eyebrow">{title} · #{number}</span>
      <h1>Document from {from_name}</h1>
      <p class="pm-lead" style="margin-bottom:14px;">Prepared for <strong>{to_name}</strong> · Total: <strong>{total}</strong></p>
      <div class="pm-cta-row">
        <a href="/v/{token}/pdf" class="pm-btn pm-btn-primary">Download Vector PDF</a>
        {pay_cta}
      </div>
    </section>
    <div class="pm-card" style="padding:16px;align-items:center;">
      <img src="/v/{token}/preview.png" alt="Document Preview" style="width:100%;max-width:680px;height:auto;border:1px solid var(--border);border-radius:8px;background:#fff;">
    </div>
    {sign_section}
    <p style="margin-top:28px;text-align:center;font-size:13px;color:var(--muted);">
      Sent via <a href="/">Pressmark</a> · Create free invoices, contracts &amp; 50-state leases at <a href="https://pressmark.dev/">pressmark.dev</a>
    </p>
  </main>
</body>
</html>"#,
        css = shared_page_css(),
        title = html_escape(&record.doc_title),
        number = html_escape(&record.doc_number),
        token = html_escape(&record.token),
        from_name = html_escape(&record.from_name),
        to_name = html_escape(&record.to_name),
        total = html_escape(&record.total_formatted),
        pay_cta = pay_cta,
        signed_banner = signed_banner,
        sign_section = sign_section,
    );

    (StatusCode::OK, noindex_headers(true), Html(html)).into_response()
}

pub async fn handle_shared_pdf(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> Response {
    let Some(record) = state.storage.get_shared_document(&token) else {
        return (StatusCode::NOT_FOUND, noindex_headers(true), "Share link expired or not found").into_response();
    };
    let Ok(doc_req) = serde_json::from_str::<DocumentRequest>(&record.doc_json) else {
        return (StatusCode::INTERNAL_SERVER_ERROR, noindex_headers(true), "Stored document corrupt").into_response();
    };
    match generate_pdf(&doc_req) {
        Ok((bytes, _)) => {
            let mut headers = noindex_headers(true);
            headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/pdf"));
            let safe_name: String = record
                .doc_number
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
                .collect();
            let filename = if safe_name.is_empty() { "document" } else { &safe_name };
            if let Ok(cd) = format!("attachment; filename=\"{}.pdf\"", filename).parse() {
                headers.insert(header::CONTENT_DISPOSITION, cd);
            }
            (StatusCode::OK, headers, bytes).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, noindex_headers(true), e).into_response(),
    }
}

pub async fn handle_shared_preview_png(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> Response {
    let Some(record) = state.storage.get_shared_document(&token) else {
        return (StatusCode::NOT_FOUND, noindex_headers(true), "Share link expired or not found").into_response();
    };
    let Ok(doc_req) = serde_json::from_str::<DocumentRequest>(&record.doc_json) else {
        return (StatusCode::INTERNAL_SERVER_ERROR, noindex_headers(true), "Stored document corrupt").into_response();
    };
    match generate_preview_png(&doc_req, 144) {
        Ok((bytes, _)) => {
            let mut headers = noindex_headers(true);
            headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("image/png"));
            (StatusCode::OK, headers, bytes).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, noindex_headers(true), e).into_response(),
    }
}

pub async fn handle_sign_shared(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
    Json(payload): Json<SignSharedRequest>,
) -> Response {
    let signer = payload.signer_name.trim();
    if signer.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "success": false, "error": "Signer name is required." })),
        )
            .into_response();
    }
    let Some(record) = state.storage.get_shared_document(&token) else {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "success": false, "error": "Share link expired or not found." })),
        )
            .into_response();
    };
    let Ok(mut doc_req) = serde_json::from_str::<DocumentRequest>(&record.doc_json) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "success": false, "error": "Stored document invalid." })),
        )
            .into_response();
    };

    let signed_date = Utc::now().format("%Y-%m-%d").to_string();
    doc_req.show_signature = Some(true);
    doc_req.signatures.push(SignatureBlock {
        signer_name: signer.to_string(),
        signer_title: payload
            .signer_title
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .or_else(|| Some("Recipient Counter-Signature".to_string())),
        date: Some(signed_date),
        signature_data: payload.signature_data.filter(|s| s.starts_with("data:image/")),
    });

    let pdf_bytes = match generate_pdf(&doc_req) {
        Ok((bytes, _)) => bytes,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "success": false, "error": e })),
            )
                .into_response();
        }
    };

    let new_sha256 = hex::encode(Sha256::digest(&pdf_bytes));
    let updated_json = match serde_json::to_string(&doc_req) {
        Ok(j) => j,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": e.to_string() })),
            )
                .into_response();
        }
    };

    let signed_at = Utc::now().timestamp();
    match state
        .storage
        .sign_shared_document(&record.token, &updated_json, &new_sha256, signer, signed_at)
    {
        Ok(true) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "success": true,
                "signed_by": signer,
                "signed_at": signed_at,
                "sha256_hex": new_sha256
            })),
        )
            .into_response(),
        _ => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "success": false, "error": "Could not update shared document." })),
        )
            .into_response(),
    }
}
