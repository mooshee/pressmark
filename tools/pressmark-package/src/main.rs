//! Package reviewed Pressmark files without generator defaults or runtime data.
//! Author: Daniel Hallman.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

type Files = BTreeMap<String, Vec<u8>>;
const VERSION: &str = "1.3.0";
const MANIFESTS: &[&str] = &[
    "plugin.json",
    ".codex-plugin/plugin.json",
    ".claude-plugin/plugin.json",
    ".cursor-plugin/plugin.json",
    "gemini-extension.json",
    "server.json",
];
const TOOLS: &[&str] = &[
    "calculate_totals",
    "convert_document",
    "generate_document",
    "get_feedback_status",
    "preview_document",
    "record_payment",
    "submit_feedback",
];

fn err(message: impl Into<String>) -> std::io::Error {
    std::io::Error::other(message.into())
}
fn checked_bytes(path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if fs::symlink_metadata(path)?.file_type().is_symlink() {
        return Err(err(format!("symlink refused: {}", path.display())).into());
    }
    let bytes = fs::read(path)?;
    if let Ok(text) = std::str::from_utf8(&bytes) {
        check_text(text)?;
    }
    Ok(bytes)
}
fn check_text(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    for marker in [
        "REQUIRED BEFORE SUBMISSION",
        "-----BEGIN PRIVATE KEY-----",
        "-----BEGIN RSA PRIVATE KEY-----",
        "sk_live_",
        "sk_test_",
        "ghp_",
        "github_pat_",
        "xoxb-",
        "xoxp-",
        "op://",
        "/Users/",
        "/home/",
        "OP_SERVICE_ACCOUNT_TOKEN=",
    ] {
        if text.contains(marker) {
            return Err(err(format!("private data or draft marker refused: {marker}")).into());
        }
    }
    Ok(())
}
fn add(
    files: &mut Files,
    root: &Path,
    source: &str,
    dest: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    files.insert(dest.into(), checked_bytes(&root.join(source))?);
    Ok(())
}
fn walk_skills(
    files: &mut Files,
    root: &Path,
    dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if fs::symlink_metadata(dir)?.file_type().is_symlink() {
        return Err(err("skill directory symlink refused").into());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let relative = path
            .strip_prefix(root)?
            .to_string_lossy()
            .replace('\\', "/");
        if path.file_name().unwrap().to_string_lossy().starts_with('.') {
            return Err(err(format!("hidden skill entry refused: {relative}")).into());
        }
        if fs::symlink_metadata(&path)?.file_type().is_symlink() {
            return Err(err(format!("symlink refused: {relative}")).into());
        }
        if path.is_dir() {
            walk_skills(files, root, &path)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            add(files, root, &relative, &relative)?;
        } else {
            return Err(err(format!("unexpected skill file refused: {relative}")).into());
        }
    }
    Ok(())
}
fn collect(root: &Path) -> Result<Files, Box<dyn std::error::Error>> {
    let mut files = Files::new();
    for path in MANIFESTS.iter().copied().chain([
        ".claude-plugin/marketplace.json",
        ".cursor/mcp.json",
        ".mcp.json",
        "mcp.json",
        "chatgpt-app-submission.json",
        "POWER.md",
        "smithery.yaml",
        "research-instructions.md",
        "RELEASE_NOTES.md",
        "LICENSE",
    ]) {
        add(&mut files, root, path, path)?;
    }
    add(&mut files, root, "plugin/README.md", "README.md")?;
    for asset in [
        "banner.png",
        "icon-64.png",
        "icon-256.png",
        "icon-512.png",
        "icon.png",
        "icon.svg",
        "logo.jpg",
    ] {
        let p = format!("assets/{asset}");
        add(&mut files, root, &p, &p)?;
    }
    walk_skills(&mut files, root, &root.join("skills"))?;
    for path in MANIFESTS {
        let value: Value = serde_json::from_slice(&files[*path])?;
        if value["version"] != VERSION {
            return Err(err(format!("version mismatch in {path}")).into());
        }
    }
    let marketplace: Value = serde_json::from_slice(&files[".claude-plugin/marketplace.json"])?;
    if marketplace["metadata"]["version"] != VERSION
        || marketplace["plugins"][0]["version"] != VERSION
    {
        return Err(err("Claude marketplace version mismatch").into());
    }
    for path in [
        "POWER.md",
        "smithery.yaml",
        "skills/pressmark/SKILL.md",
        "skills/invoice-generation/SKILL.md",
        "skills/document-lifecycle/SKILL.md",
        "skills/real-estate-agreements/SKILL.md",
    ] {
        if !std::str::from_utf8(&files[path])?.contains(&format!("version: {VERSION}\n")) {
            return Err(err(format!("version mismatch in {path}")).into());
        }
    }
    let submission: Value = serde_json::from_slice(&files["chatgpt-app-submission.json"])?;
    let names: Vec<&str> = submission["tools"]
        .as_object()
        .ok_or_else(|| err("missing tools"))?
        .keys()
        .map(String::as_str)
        .collect();
    if names != TOOLS {
        return Err(err("submission tool list mismatch").into());
    }
    Ok(files)
}
fn write_zip(files: &Files, output: &Path, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut archive = zip::ZipWriter::new(fs::File::create(output)?);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .last_modified_time(
            zip::DateTime::from_date_and_time(2026, 1, 1, 0, 0, 0)
                .map_err(|_| err("invalid timestamp"))?,
        )
        .unix_permissions(0o644);
    for (name, content) in files {
        archive.start_file(format!("{prefix}{name}"), options)?;
        archive.write_all(content)?;
    }
    archive.finish()?;
    let mut saved = zip::ZipArchive::new(fs::File::open(output)?)?;
    if saved.len() != files.len() {
        return Err(err("archive file count mismatch").into());
    }
    for (name, content) in files {
        let mut bytes = Vec::new();
        saved
            .by_name(&format!("{prefix}{name}"))?
            .read_to_end(&mut bytes)?;
        if &bytes != content {
            return Err(err(format!("archive byte mismatch: {name}")).into());
        }
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err(err("usage: pressmark-package REPO_ROOT OUTPUT_DIR").into());
    }
    let root = PathBuf::from(&args[1]).canonicalize()?;
    let out = PathBuf::from(&args[2]);
    let files = collect(&root)?;
    fs::create_dir_all(&out)?;
    let mut artifacts = BTreeMap::new();
    for (name, prefix) in [
        ("pressmark-flat.zip", ""),
        ("pressmark-agent-plugin.zip", "pressmark/"),
    ] {
        let path = out.join(name);
        write_zip(&files, &path, prefix)?;
        artifacts.insert(name, format!("{:x}", Sha256::digest(fs::read(&path)?)));
    }
    fs::write(out.join("server.json"), &files["server.json"])?;
    fs::write(out.join("RELEASE_NOTES.md"), &files["RELEASE_NOTES.md"])?;
    for name in ["server.json", "RELEASE_NOTES.md"] {
        artifacts.insert(
            name,
            format!("{:x}", Sha256::digest(fs::read(out.join(name))?)),
        );
    }
    let inventory: BTreeMap<_, _> = files
        .iter()
        .map(|(n, b)| (n, format!("{:x}", Sha256::digest(b))))
        .collect();
    let receipt = json!({"version": VERSION, "author": "Daniel Hallman", "artifacts": artifacts, "files": inventory});
    fs::write(
        out.join("SHA256.json"),
        format!("{}\n", serde_json::to_string_pretty(&receipt)?),
    )?;
    println!(
        "Verified {} allowlisted files in both archives; wrote {}",
        files.len(),
        out.display()
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn draft_markers_are_rejected() {
        assert!(check_text("REQUIRED BEFORE SUBMISSION: finish me").is_err());
    }
    #[test]
    fn private_keys_and_local_paths_are_rejected() {
        assert!(check_text("-----BEGIN PRIVATE KEY-----").is_err());
        assert!(check_text("/Users/example/.env").is_err());
    }
    #[test]
    fn public_instructions_are_allowed() {
        assert!(check_text("Connect https://pressmark.dev/mcp; never share credentials.").is_ok());
    }
    #[test]
    fn archives_are_deterministic_and_exact() {
        let temp = tempfile::tempdir().unwrap();
        let mut files = Files::new();
        files.insert("plugin.json".into(), b"{}".to_vec());
        files.insert("skills/example/SKILL.md".into(), b"# Example".to_vec());
        let a = temp.path().join("a.zip");
        let b = temp.path().join("b.zip");
        write_zip(&files, &a, "").unwrap();
        write_zip(&files, &b, "").unwrap();
        assert_eq!(fs::read(a).unwrap(), fs::read(b).unwrap());
    }
    #[test]
    fn symbolic_links_are_rejected() {
        let temp = tempfile::tempdir().unwrap();
        let p = temp.path().join("secret");
        fs::write(&p, "secret").unwrap();
        let link = temp.path().join("link");
        std::os::unix::fs::symlink(p, &link).unwrap();
        assert!(checked_bytes(&link).is_err());
    }
}
