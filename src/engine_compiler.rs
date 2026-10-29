//! Pressmark Typst Compiler, Asset Cache & Escaping Primitives
//!
//! Author: Daniel Hallman

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Locate the Typst compiler binary on macOS or Linux systems.
pub fn find_typst_executable() -> Option<PathBuf> {
    if let Ok(path_str) = std::env::var("PRESSMARK_ENGINE_BIN").or_else(|_| std::env::var("PRESSMARK_TYPST_BIN")) {
        let p = PathBuf::from(path_str);
        if p.exists() {
            return Some(p);
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
            if let Ok(output) = Command::new("which").arg("typst").output() {
                if output.status.success() {
                    let p = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !p.is_empty() && Path::new(&p).exists() {
                        return Some(PathBuf::from(p));
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

/// Resolve the disk cache directory from `PRESSMARK_CACHE_DIR` or defaulting to `/tmp/pressmark_cache`.
pub fn get_cache_dir() -> PathBuf {
    std::env::var("PRESSMARK_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp/pressmark_cache"))
}

/// Save Base64 data URI to a fast disk cache so Typst's `#image(...)` function can load it.
/// Returns a root-relative path (`"/filename.ext"`) scoped to `get_cache_dir()`.
pub fn stage_image_asset(prefix: &str, data_uri: &str) -> Option<String> {
    if data_uri.is_empty() {
        return None;
    }

    let cache_dir = get_cache_dir();
    let _ = fs::create_dir_all(&cache_dir);

    let candidate_path = if let Some(rest) = data_uri.strip_prefix("/assets/") {
        std::path::PathBuf::from("static/assets").join(rest)
    } else if let Some(rest) = data_uri.strip_prefix("assets/") {
        std::path::PathBuf::from("static/assets").join(rest)
    } else {
        std::path::PathBuf::from(data_uri)
    };

    // Only allow pre-existing file paths if strictly within cache directory or static/assets
    if let Ok(canon) = std::fs::canonicalize(&candidate_path) {
        let canon_cache = std::fs::canonicalize(&cache_dir).unwrap_or_else(|_| cache_dir.clone());
        let is_cache = canon.starts_with(&canon_cache)
            || canon.starts_with(&cache_dir)
            || canon.starts_with("/tmp/pressmark_cache");
        let current_dir = std::env::current_dir().unwrap_or_default();
        let canon_assets = std::fs::canonicalize(current_dir.join("static/assets"))
            .unwrap_or_else(|_| current_dir.join("static/assets"));
        let is_assets = canon.starts_with(&canon_assets) || canon.starts_with(current_dir.join("static/assets"));
        let sanitize_fname = |raw: &str| -> String {
            raw.chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
                .collect()
        };
        if is_cache {
            if let Some(fname) = canon.file_name().and_then(|f| f.to_str()) {
                let safe_name = sanitize_fname(fname);
                if !safe_name.is_empty() {
                    let dst = cache_dir.join(&safe_name);
                    if !dst.exists() {
                        let _ = fs::copy(&canon, &dst);
                    }
                    return Some(format!("/{}", safe_name));
                }
            }
        } else if is_assets {
            if let Some(fname) = canon.file_name().and_then(|f| f.to_str()) {
                let safe_name = sanitize_fname(&format!("{}_{}", prefix, fname));
                if !safe_name.is_empty() {
                    let dst = cache_dir.join(&safe_name);
                    if !dst.exists() {
                        let _ = fs::copy(&canon, &dst);
                    }
                    return Some(format!("/{}", safe_name));
                }
            }
        }
        return None;
    }

    if !data_uri.starts_with("data:") && (data_uri.starts_with('/') || data_uri.starts_with('.')) {
        return None;
    }

    // Extract base64 payload
    let parts: Vec<&str> = data_uri.split(',').collect();
    let b64_str = if parts.len() == 2 { parts[1] } else { data_uri };

    let decoded = BASE64.decode(b64_str.trim()).ok()?;

    // Compute hash for deduplication
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    decoded.hash(&mut hasher);
    let hash_val = hasher.finish();

    let ext = if data_uri.contains("image/jpeg") || data_uri.contains("image/jpg") {
        "jpg"
    } else if data_uri.contains("image/svg") || data_uri.contains("<svg") {
        "svg"
    } else {
        "png"
    };

    let filename = format!("{}_{:x}.{}", prefix, hash_val, ext);
    let file_path = cache_dir.join(&filename);

    if !file_path.exists() {
        let _ = fs::write(&file_path, &decoded);
    }

    Some(format!("/{}", filename))
}

/// Save raw SVG XML string directly into the disk cache and return a root-relative path.
pub fn stage_svg_content(prefix: &str, svg_content: &str) -> Option<String> {
    if svg_content.is_empty() {
        return None;
    }

    let cache_dir = get_cache_dir();
    let _ = fs::create_dir_all(&cache_dir);

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    svg_content.hash(&mut hasher);
    let hash_val = hasher.finish();

    let filename = format!("{}_{:x}.svg", prefix, hash_val);
    let file_path = cache_dir.join(&filename);

    if !file_path.exists() {
        let _ = fs::write(&file_path, svg_content.as_bytes());
    }

    Some(format!("/{}", filename))
}

/// Prune temporary files in a specific directory that are older than `max_age_secs`.
/// Returns the number of files deleted.
pub fn clean_cache_directory_at(cache_dir: &std::path::Path, max_age_secs: u64) -> usize {
    if !cache_dir.exists() {
        return 0;
    }

    let mut deleted = 0;
    let now = std::time::SystemTime::now();
    let max_age = std::time::Duration::from_secs(max_age_secs);

    if let Ok(entries) = fs::read_dir(cache_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(meta) = entry.metadata() {
                    if let Ok(modified) = meta.modified() {
                        if let Ok(age) = now.duration_since(modified) {
                            if age > max_age {
                                if fs::remove_file(&path).is_ok() {
                                    deleted += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    deleted
}

/// Prune temporary files in the cache directory that are older than `max_age_secs`.
/// Returns the number of files deleted.
pub fn clean_cache_directory(max_age_secs: u64) -> usize {
    clean_cache_directory_at(&get_cache_dir(), max_age_secs)
}

/// Spawns a background task to periodically clean the disk cache.
pub fn start_cache_pruner_task(interval_secs: u64, max_age_secs: u64) {
    tokio::spawn(async move {
        let _ = tokio::task::spawn_blocking(move || clean_cache_directory(max_age_secs)).await;
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(interval_secs));
        loop {
            interval.tick().await;
            let pruned = tokio::task::spawn_blocking(move || clean_cache_directory(max_age_secs))
                .await
                .unwrap_or(0);
            if pruned > 0 {
                tracing::info!("Pressmark cache pruner: removed {} stale files from {}", pruned, get_cache_dir().display());
            }
        }
    });
}

/// Generate a vector SVG QR code with High error correction (EcLevel::H), quiet zone, and custom dark color.
pub fn generate_qr_code_svg(content: &str, dark_hex: &str) -> Result<String, String> {
    use qrcode::render::svg;
    use qrcode::{EcLevel, QrCode};

    let code = QrCode::with_error_correction_level(content.as_bytes(), EcLevel::H)
        .or_else(|_| QrCode::new(content.as_bytes()))
        .map_err(|e| format!("QR encoding error: {}", e))?;

    let svg_xml = code
        .render::<svg::Color>()
        .min_dimensions(180, 180)
        .quiet_zone(true)
        .dark_color(svg::Color(dark_hex))
        .light_color(svg::Color("#ffffff00"))
        .build();
    Ok(svg_xml)
}

/// Compile a Typst markup string directly into binary PDF bytes in memory.
pub fn compile_pdf_bytes(typst_markup: &str) -> Result<Vec<u8>, String> {
    let typst_bin = find_typst_executable()
        .ok_or_else(|| "Pressmark vector compiler executable not found on system.".to_string())?;

    let cache_dir = get_cache_dir();
    let _ = std::fs::create_dir_all(&cache_dir);

    let mut child = Command::new(&typst_bin)
        .arg("compile")
        .arg("--root")
        .arg(&cache_dir)
        .arg("-") // stdin
        .arg("-") // stdout
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn vector compiler {:?}: {}", typst_bin, e))?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin
            .write_all(typst_markup.as_bytes())
            .map_err(|e| format!("Failed to pipe markup to vector compiler stdin: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to await vector compilation: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Pressmark vector PDF synthesis notice: {}", stderr));
    }

    Ok(output.stdout)
}

/// Compile all pages of a vector markup string into individual PNG byte buffers.
pub fn compile_all_png_pages(typst_markup: &str, ppi: u32) -> Result<Vec<Vec<u8>>, String> {
    let typst_bin = find_typst_executable()
        .ok_or_else(|| "Pressmark vector compiler executable not found on system.".to_string())?;

    let cache_dir = get_cache_dir();
    let _ = std::fs::create_dir_all(&cache_dir);

    let session_id = uuid::Uuid::new_v4().to_string();
    let temp_dir = std::env::temp_dir().join(format!("pressmark_pages_{}", session_id));
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp dir for multi-page export: {}", e))?;

    let out_pattern = temp_dir.join("{p}.png");

    let mut child = Command::new(&typst_bin)
        .arg("compile")
        .arg("--root")
        .arg(&cache_dir)
        .arg("--format")
        .arg("png")
        .arg("--ppi")
        .arg(ppi.to_string())
        .arg("-") // stdin
        .arg(&out_pattern)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn vector preview process {:?}: {}", typst_bin, e))?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin
            .write_all(typst_markup.as_bytes())
            .map_err(|e| format!("Failed to pipe markup to vector compiler stdin: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to await vector preview execution: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let _ = std::fs::remove_dir_all(&temp_dir);
        return Err(format!("Pressmark vector preview synthesis notice: {}", stderr));
    }

    let mut page_files: Vec<(usize, std::path::PathBuf)> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&temp_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("png") {
                if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(page_num) = file_stem.parse::<usize>() {
                        page_files.push((page_num, path));
                    }
                }
            }
        }
    }

    page_files.sort_by_key(|(num, _)| *num);

    let mut pages_bytes = Vec::new();
    for (_, path) in page_files {
        if let Ok(bytes) = std::fs::read(&path) {
            pages_bytes.push(bytes);
        }
    }

    let _ = std::fs::remove_dir_all(&temp_dir);

    if pages_bytes.is_empty() {
        return Err("No page images were produced by the vector engine.".to_string());
    }

    Ok(pages_bytes)
}

/// Compile a vector markup string directly into high-resolution PNG bytes for page 1.
pub fn compile_png_bytes(typst_markup: &str, ppi: u32) -> Result<Vec<u8>, String> {
    let pages = compile_all_png_pages(typst_markup, ppi)?;
    pages
        .into_iter()
        .next()
        .ok_or_else(|| "Failed to extract page 1 from compiled vector pages.".to_string())
}

/// Escape special characters in text so Typst doesn't interpret @ as labels, $ as math, or [ ] # \ * _ ` < > ~ = as markup.
pub fn escape_typst_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '@' => out.push_str("\\@"),
            '$' => out.push_str("\\$"),
            '[' => out.push_str("\\["),
            ']' => out.push_str("\\]"),
            '#' => out.push_str("\\#"),
            '*' => out.push_str("\\*"),
            '_' => out.push_str("\\_"),
            '`' => out.push_str("\\`"),
            '<' => out.push_str("\\<"),
            '>' => out.push_str("\\>"),
            '~' => out.push_str("\\~"),
            '=' => out.push_str("\\="),
            _ => out.push(c),
        }
    }
    out
}

/// Escape string literals inside Typst function arguments `"..."`
pub fn escape_typst_string_literal(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            _ => out.push(c),
        }
    }
    out
}
