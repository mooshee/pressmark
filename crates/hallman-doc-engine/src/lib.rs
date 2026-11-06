//! Shared Vector Document & Interactive AcroForm Engine (`hallman-doc-engine`)
//!
//! Author: Daniel Hallman
//!
//! Provides unified Typst binary discovery, safe Typst string/content escaping,
//! minimum print/screen legibility presets (`ReadabilityPreset`), and interactive
//! PDF `/AcroForm` fillable signature/date widget injection across Untax (`untax.io`)
//! and Pressmark (`pressmark.io`).

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use lopdf::dictionary;
use lopdf::Document;
use lopdf::Object;
use lopdf::StringFormat;
use serde::Deserialize;
use serde::Serialize;

pub mod catalog;
pub mod graphics;

/// Readability and typography preset for tailoring font size, leading, and contrast
/// across statutory petitions, affidavits, contracts, leases, and invoices.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ReadabilityPreset {
    /// Standard 9.5pt body, balanced judicial/commercial spacing (Default).
    #[default]
    Standard,
    /// Large 10.4pt body, generous 0.65em leading, and pure `#000000` ink for seniors & disabled veterans.
    HighLegibility,
    /// Dense 8.8pt body, compact table padding for institutional portfolio schedules.
    ExecutiveCompact,
    /// Formal 9.5pt serif/sans styling with double-rule accents for quasi-judicial dockets.
    JudicialFormal,
}

impl ReadabilityPreset {
    /// Canonical Typst dictionary identifier.
    pub fn as_typst_id(&self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::HighLegibility => "high_legibility",
            Self::ExecutiveCompact => "executive_compact",
            Self::JudicialFormal => "judicial_formal",
        }
    }

    /// Base body font size enforcing the minimum print/screen legibility floor.
    pub fn base_font_size(&self) -> &'static str {
        match self {
            Self::Standard | Self::JudicialFormal => "9.5pt",
            Self::HighLegibility => "10.4pt",
            Self::ExecutiveCompact => "8.8pt",
        }
    }

    /// Line height multiplier relative to `0.65em` Typst paragraph leading.
    pub fn line_height_multiplier(&self) -> &'static str {
        match self {
            Self::Standard => "1.3",
            Self::HighLegibility => "1.45",
            Self::ExecutiveCompact => "1.18",
            Self::JudicialFormal => "1.35",
        }
    }
}

/// Specification for an interactive PDF `/AcroForm` text widget (`/FT /Tx`).
#[derive(Debug, Clone)]
pub struct AcroFormFieldSpec {
    /// Field identifier (`/T`), e.g. `"taxpayer_signature"` or `"execution_date"`.
    pub name: String,
    /// Accessible tooltip description (`/TU`).
    pub tooltip: String,
    /// Default value (`/V`), typically empty `""` for blank signature/date lines.
    pub default_value: String,
    /// PDF Default Appearance string (`/DA`), e.g. `"/TiIt 13 Tf 0.08 0.18 0.36 rg"`.
    pub default_appearance: String,
    /// 1-indexed target page number where the widget is placed.
    pub page_number: u32,
    /// Bounding box `[x1, y1, x2, y2]` in PDF points (origin at bottom-left of page).
    pub rect: [i64; 4],
}

/// Locate the Typst compiler binary on macOS or Linux systems.
pub fn find_typst_executable() -> Option<PathBuf> {
    if std::env::var("UNTAX_SKIP_PDF")
        .or_else(|_| std::env::var("PRESSMARK_SKIP_PDF"))
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return None;
    }

    for env_key in ["UNTAX_TYPST_BIN", "PRESSMARK_TYPST_BIN", "TYPST_BIN"] {
        if let Ok(path_str) = std::env::var(env_key) {
            let p = PathBuf::from(path_str);
            if p.exists() {
                return Some(p);
            }
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let cargo_bin = PathBuf::from(home).join(".cargo/bin/typst");
        if cargo_bin.exists() {
            return Some(cargo_bin);
        }
    }

    let candidates = [
        "/opt/homebrew/bin/typst",
        "/usr/local/bin/typst",
        "/usr/bin/typst",
        "typst",
    ];

    for path_str in candidates {
        if path_str == "typst" {
            if let Some(paths) = std::env::var_os("PATH") {
                for dir in std::env::split_paths(&paths) {
                    let candidate = dir.join("typst");
                    if candidate.is_file() {
                        return Some(candidate);
                    }
                }
            }
        } else {
            let p = PathBuf::from(path_str);
            if p.exists() {
                return Some(p);
            }
        }
    }

    None
}

/// Escape user-supplied strings so they can be safely embedded inside Typst content blocks `[...]`.
pub fn escape_typst_text(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 16);
    for ch in input.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '#' => out.push_str("\\#"),
            '$' => out.push_str("\\$"),
            '*' => out.push_str("\\*"),
            '_' => out.push_str("\\_"),
            '`' => out.push_str("\\`"),
            '<' => out.push_str("\\<"),
            '>' => out.push_str("\\>"),
            '@' => out.push_str("\\@"),
            '[' => out.push_str("\\["),
            ']' => out.push_str("\\]"),
            '~' => out.push_str("\\~"),
            '=' => out.push_str("\\="),
            _ => out.push(ch),
        }
    }
    out
}

/// Escape user-supplied strings so they can be safely embedded inside Typst string literals `"..."`.
pub fn escape_typst_string_literal(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 8);
    for ch in input.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            _ => out.push(ch),
        }
    }
    out
}

fn document_body_slice(typst_markup: &str) -> &str {
    typst_markup
        .rfind("#set page(")
        .or_else(|| typst_markup.rfind("#let hde-render-page-archetype("))
        .or_else(|| typst_markup.rfind("#let recording-header-block("))
        .or_else(|| typst_markup.rfind("#let jurat-box("))
        .map(|idx| &typst_markup[idx..])
        .unwrap_or(typst_markup)
}

/// Determine whether a Typst markup document contains an interactive signature or jurat block
/// that should receive fillable `/AcroForm` widgets.
pub fn should_inject_signature_acroform(typst_markup: &str) -> bool {
    let body = document_body_slice(typst_markup);
    let is_esign =
        body.contains("signature-type: \"esign\"") || body.contains("sig-type: \"esign\"");
    let has_wet_jurat = (body.contains("#jurat-box(")
        || body.contains("jurat-box(")
        || body.contains("#hde-render-page-archetype("))
        && (body.contains("signature-type: \"wet-ink\"")
            || body.contains("signature-type: \"wet_ink\"")
            || body.contains("sig-type: \"wet-ink\"")
            || body.contains("sig-type: \"wet_ink\"")
            || body.contains("signature-type: \"ron\"")
            || body.contains("notary-required: true")
            || !is_esign);
    let has_pressmark_sig_grid = (body.contains("#signature-grid(")
        || body.contains("#signature-block("))
        && body.contains("signature-path: none");
    let has_certificate_of_service = body.contains("#certificate-of-service(");
    has_wet_jurat || has_pressmark_sig_grid || has_certificate_of_service
}

/// Inject interactive `/AcroForm` fillable signature and execution-date fields into a Typst-compiled PDF
/// so taxpayers, clients, landlords, tenants, and notaries can click and type directly inside any PDF viewer
/// (Chrome, Safari, macOS Preview, Adobe Acrobat) while keeping valuation and billing tables immutable.
pub fn inject_fillable_signature_acroform(
    pdf_bytes: &[u8],
    typst_markup: &str,
) -> Result<Vec<u8>, String> {
    let body = document_body_slice(typst_markup);
    let target_page = estimate_signature_page_number(body);
    let mut specs = vec![
        AcroFormFieldSpec {
            name: "taxpayer_signature".to_string(),
            tooltip: "Type or sign your full legal name (Primary Signer / Petitioner)".to_string(),
            default_value: String::new(),
            default_appearance: "/TiIt 13 Tf 0.08 0.18 0.36 rg".to_string(),
            page_number: target_page,
            rect: [44, 196, 336, 218],
        },
        AcroFormFieldSpec {
            name: "execution_date".to_string(),
            tooltip: "Execution / Signing Date (MM/DD/YYYY)".to_string(),
            default_value: String::new(),
            default_appearance: "/Helv 10 Tf 0.06 0.09 0.16 rg".to_string(),
            page_number: target_page,
            rect: [360, 196, 555, 218],
        },
    ];

    if body.matches("signature-item(").count() >= 2 {
        specs.push(AcroFormFieldSpec {
            name: "counterparty_signature".to_string(),
            tooltip: "Counterparty / Second Signatory Legal Signature".to_string(),
            default_value: String::new(),
            default_appearance: "/TiIt 13 Tf 0.08 0.18 0.36 rg".to_string(),
            page_number: target_page,
            rect: [320, 152, 555, 174],
        });
        specs.push(AcroFormFieldSpec {
            name: "counterparty_date".to_string(),
            tooltip: "Counterparty Signing Date (MM/DD/YYYY)".to_string(),
            default_value: String::new(),
            default_appearance: "/Helv 10 Tf 0.06 0.09 0.16 rg".to_string(),
            page_number: target_page,
            rect: [320, 128, 460, 146],
        });
    }

    if body.contains("notary-required: true") {
        specs.push(AcroFormFieldSpec {
            name: "notary_signature".to_string(),
            tooltip: "Notary Public Signature".to_string(),
            default_value: String::new(),
            default_appearance: "/TiIt 12 Tf 0 0 0 rg".to_string(),
            page_number: target_page,
            rect: [44, 120, 290, 140],
        });
        specs.push(AcroFormFieldSpec {
            name: "notary_commission_expires".to_string(),
            tooltip: "My Commission Expires".to_string(),
            default_value: String::new(),
            default_appearance: "/Helv 9 Tf 0 0 0 rg".to_string(),
            page_number: target_page,
            rect: [135, 92, 290, 108],
        });
    }

    inject_acroform_fields(pdf_bytes, &specs)
}

/// Inject an arbitrary list of `/AcroForm` text widget specifications into a compiled PDF byte buffer.
pub fn inject_acroform_fields(
    pdf_bytes: &[u8],
    fields: &[AcroFormFieldSpec],
) -> Result<Vec<u8>, String> {
    if fields.is_empty() {
        return Ok(pdf_bytes.to_vec());
    }

    let mut doc = Document::load_mem(pdf_bytes).map_err(|e| e.to_string())?;
    let pages = doc.get_pages();
    if pages.is_empty() {
        return Err("PDF has no pages".to_string());
    }

    let max_page = *pages.keys().max().unwrap_or(&1);

    let helv_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });
    let tiit_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Times-Italic",
    });

    let mut all_field_refs = Vec::with_capacity(fields.len());

    for spec in fields {
        let clamped_page = spec.page_number.clamp(1, max_page);
        let page_id = pages
            .get(&clamped_page)
            .copied()
            .or_else(|| pages.values().next().copied())
            .ok_or_else(|| "Target page not found in PDF".to_string())?;

        let field_id = doc.add_object(dictionary! {
            "Type" => "Annot",
            "Subtype" => "Widget",
            "FT" => "Tx",
            "P" => page_id,
            "T" => Object::String(spec.name.as_bytes().to_vec(), StringFormat::Literal),
            "TU" => Object::String(spec.tooltip.as_bytes().to_vec(), StringFormat::Literal),
            "V" => Object::String(spec.default_value.as_bytes().to_vec(), StringFormat::Literal),
            "DA" => Object::String(spec.default_appearance.as_bytes().to_vec(), StringFormat::Literal),
            "F" => 4,
            "Rect" => vec![
                spec.rect[0].into(),
                spec.rect[1].into(),
                spec.rect[2].into(),
                spec.rect[3].into(),
            ],
        });

        let field_ref = Object::Reference(field_id);
        all_field_refs.push(field_ref.clone());

        if let Ok(page_dict) = doc.get_object_mut(page_id).and_then(Object::as_dict_mut) {
            match page_dict.get_mut(b"Annots") {
                Ok(Object::Array(annots)) => {
                    annots.push(field_ref);
                }
                _ => {
                    page_dict.set("Annots", Object::Array(vec![field_ref]));
                }
            }
        }
    }

    let acroform_id = doc.add_object(dictionary! {
        "Fields" => Object::Array(all_field_refs),
        "NeedAppearances" => true,
        "DA" => Object::String(b"/Helv 10 Tf 0 0 0 rg".to_vec(), StringFormat::Literal),
        "DR" => dictionary! {
            "Font" => dictionary! {
                "Helv" => Object::Reference(helv_id),
                "TiIt" => Object::Reference(tiit_id),
            },
        },
    });

    let catalog_id = doc
        .trailer
        .get(b"Root")
        .and_then(Object::as_reference)
        .map_err(|e| e.to_string())?;
    if let Ok(catalog_dict) = doc.get_object_mut(catalog_id).and_then(Object::as_dict_mut) {
        catalog_dict.set("AcroForm", Object::Reference(acroform_id));
    }

    let mut out = Vec::with_capacity(pdf_bytes.len() + 2048);
    doc.save_to(&mut out).map_err(|e| e.to_string())?;
    Ok(out)
}

fn estimate_signature_page_number(typst_markup: &str) -> u32 {
    let sig_pos = typst_markup
        .find("#jurat-box(")
        .or_else(|| typst_markup.find("jurat-box("))
        .or_else(|| typst_markup.find("#signature-grid("))
        .or_else(|| typst_markup.find("#signature-block("))
        .or_else(|| typst_markup.find("#certificate-of-service("));

    if let Some(pos) = sig_pos {
        let prefix = &typst_markup[..pos];
        let explicit_breaks = prefix.matches("#pagebreak()").count() as u32;
        1 + explicit_breaks
    } else {
        1
    }
}

/// Compile Typst markup directly to PDF bytes and automatically inject interactive `/AcroForm`
/// signature/date widgets when applicable.
pub fn compile_typst_to_pdf_bytes(
    typst_markup: &str,
    root_dir: Option<&Path>,
) -> Result<Vec<u8>, String> {
    use std::io::Write;
    use std::process::Stdio;

    let typst_bin =
        find_typst_executable().ok_or_else(|| "Typst executable not found".to_string())?;
    let unique_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp_dir = std::env::temp_dir().join(format!(
        "hde_pdf_{}_{}",
        std::process::id(),
        unique_id
    ));
    std::fs::create_dir_all(&tmp_dir).map_err(|e| e.to_string())?;
    let output_path = tmp_dir.join("doc.pdf");

    let mut cmd = Command::new(&typst_bin);
    cmd.arg("compile").arg("--pdf-standard").arg("1.7");
    if let Some(root) = root_dir {
        cmd.arg("--root").arg(root);
    }
    cmd.arg("-")
        .arg(&output_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(typst_markup.as_bytes())
            .map_err(|e| e.to_string())?;
    }

    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let _ = std::fs::remove_dir_all(&tmp_dir);
        return Err(format!("Typst PDF compile failed: {}", stderr));
    }

    let raw_pdf = std::fs::read(&output_path).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_dir_all(&tmp_dir);

    if should_inject_signature_acroform(typst_markup) {
        if let Ok(enriched) = inject_fillable_signature_acroform(&raw_pdf, typst_markup) {
            return Ok(enriched);
        }
    }
    Ok(raw_pdf)
}

/// Compile Typst markup to a first-page PNG preview image at the specified DPI (`ppi`, e.g. `144`).
pub fn compile_typst_to_png_bytes(
    typst_markup: &str,
    root_dir: Option<&Path>,
    ppi: u16,
) -> Result<Vec<u8>, String> {
    use std::io::Write;
    use std::process::Stdio;

    let typst_bin =
        find_typst_executable().ok_or_else(|| "Typst executable not found".to_string())?;
    let unique_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp_dir = std::env::temp_dir().join(format!(
        "hde_png_{}_{}",
        std::process::id(),
        unique_id
    ));
    std::fs::create_dir_all(&tmp_dir).map_err(|e| e.to_string())?;
    let out_pattern = tmp_dir.join("page-{p}.png");
    let page_1_path = tmp_dir.join("page-1.png");

    let mut cmd = Command::new(&typst_bin);
    cmd.arg("compile")
        .arg("--format")
        .arg("png")
        .arg("--ppi")
        .arg(ppi.to_string())
        .arg("--pages")
        .arg("1");
    if let Some(root) = root_dir {
        cmd.arg("--root").arg(root);
    }
    cmd.arg("-")
        .arg(&out_pattern)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(typst_markup.as_bytes())
            .map_err(|e| e.to_string())?;
    }

    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let _ = std::fs::remove_dir_all(&tmp_dir);
        return Err(format!("Typst PNG compile failed: {}", stderr));
    }

    let png_bytes = std::fs::read(&page_1_path).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_dir_all(&tmp_dir);
    Ok(png_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readability_preset_floors() {
        assert_eq!(ReadabilityPreset::Standard.base_font_size(), "9.5pt");
        assert_eq!(ReadabilityPreset::HighLegibility.base_font_size(), "10.4pt");
        assert_eq!(ReadabilityPreset::ExecutiveCompact.base_font_size(), "8.8pt");
        assert_eq!(ReadabilityPreset::JudicialFormal.base_font_size(), "9.5pt");
    }

    #[test]
    fn test_should_inject_signature_acroform_detection() {
        assert!(should_inject_signature_acroform(
            "#jurat-box(signature-type: \"wet-ink\", notary-required: true)"
        ));
        assert!(!should_inject_signature_acroform(
            "#jurat-box(signature-type: \"esign\", signer-name: \"Jane Doe\")"
        ));
        assert!(should_inject_signature_acroform(
            "#signature-grid((signature-item(signature-path: none, signer-name: \"Alice\"),))"
        ));
    }
}
