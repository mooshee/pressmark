// Author: Daniel Hallman
use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "pressmark-release-test-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn file(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, bytes).unwrap();
        path
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn tree_hash_is_order_independent_and_binds_paths_and_bytes() {
    let a = Fixture::new();
    let b = Fixture::new();
    a.file("privacy.html", b"privacy");
    a.file("index.html", b"home");
    b.file("index.html", b"home");
    b.file("privacy.html", b"privacy");
    assert_eq!(
        tree_digest(&tree(&a.0).unwrap()).unwrap(),
        tree_digest(&tree(&b.0).unwrap()).unwrap()
    );
    b.file("index.html", b"changed");
    assert_ne!(
        tree_digest(&tree(&a.0).unwrap()).unwrap(),
        tree_digest(&tree(&b.0).unwrap()).unwrap()
    );
    fs::rename(b.0.join("index.html"), b.0.join("other.html")).unwrap();
    assert!(!tree(&b.0).unwrap().contains_key("index.html"));
}

#[test]
fn overlay_only_changes_the_existing_privacy_page() {
    let f = Fixture::new();
    f.file("privacy.html", b"old");
    f.file("index.html", b"home");
    let before = tree(&f.0).unwrap();
    let expected = overlay(&before, b"new").unwrap();
    f.file("privacy.html", b"new");
    assert_eq!(expected, tree_digest(&tree(&f.0).unwrap()).unwrap());
    assert_eq!(before["index.html"], tree(&f.0).unwrap()["index.html"]);
    let mut missing = before;
    missing.remove("privacy.html");
    assert!(overlay(&missing, b"new").is_err());
}

#[test]
fn file_reads_reject_symlinks_directories_and_large_files() {
    let f = Fixture::new();
    let original = f.file("original", b"data");
    std::os::unix::fs::symlink(&original, f.0.join("link")).unwrap();
    assert!(read_regular(&f.0.join("link"), 100).is_err());
    assert!(read_regular(&f.0, 100).is_err());
    assert!(read_regular(&original, 3).is_err());
    assert!(tree(&f.0).is_err());
}

#[test]
fn swap_rejects_drift_and_preserves_exact_bytes_and_permissions() {
    let f = Fixture::new();
    let path = f.file("binary", b"old");
    let mut changed = false;
    assert!(swap(
        &path,
        &digest(b"other"),
        b"new",
        0o755,
        "test",
        &mut changed
    )
    .is_err());
    assert!(!changed);
    assert_eq!(fs::read(&path).unwrap(), b"old");
    swap(&path, &digest(b"old"), b"new", 0o755, "test", &mut changed).unwrap();
    assert!(changed);
    assert_eq!(fs::read(&path).unwrap(), b"new");
    assert_eq!(fs::metadata(&path).unwrap().mode() & 0o777, 0o755);
    assert!(!f.0.join(".binary.release-test").exists());
}

#[test]
fn rollback_restores_only_the_bytes_this_release_installed() {
    let f = Fixture::new();
    let binary = f.file("binary", b"old-binary");
    let privacy = f.file("privacy.html", b"old-page");
    let mut changed = false;
    swap(
        &binary,
        &digest(b"old-binary"),
        b"new-binary",
        0o755,
        "test",
        &mut changed,
    )
    .unwrap();
    f.file("privacy.html", b"concurrent-page");
    assert!(swap(
        &privacy,
        &digest(b"old-page"),
        b"new-page",
        0o644,
        "test",
        &mut false
    )
    .is_err());
    swap(
        &binary,
        &digest(b"new-binary"),
        b"old-binary",
        0o755,
        "rollback",
        &mut false,
    )
    .unwrap();
    assert_eq!(fs::read(binary).unwrap(), b"old-binary");
    assert_eq!(fs::read(&privacy).unwrap(), b"concurrent-page");
    assert!(swap(
        &privacy,
        &digest(b"new-page"),
        b"old-page",
        0o644,
        "rollback",
        &mut false
    )
    .is_err());
}

#[test]
fn private_outputs_are_never_overwritten() {
    let f = Fixture::new();
    let path = f.0.join("receipt.json");
    write_new(&path, b"first", 0o600).unwrap();
    assert!(write_new(&path, b"second", 0o600).is_err());
    assert_eq!(fs::read(&path).unwrap(), b"first");
    assert_eq!(fs::metadata(path).unwrap().mode() & 0o777, 0o600);
}

#[test]
fn elf_validation_rejects_foreign_platform_and_architecture() {
    let mut native = vec![0; 32];
    native[..4].copy_from_slice(b"\x7fELF");
    native[4] = 2;
    native[5] = 1;
    native[6] = 1;
    native[18] = 62;
    assert!(same_elf_architecture(&native, &native));
    assert!(!same_elf_architecture(&native, b"not an executable"));
    let mut foreign = native.clone();
    foreign[18] = 183;
    assert!(!same_elf_architecture(&native, &foreign));
}

#[test]
fn service_readback_rejects_stopped_or_missing_processes() {
    assert_eq!(
        parse_service("MainPID=42\nActiveState=active\nSubState=running\n")
            .unwrap()
            .0,
        42
    );
    for value in [
        "MainPID=0\nActiveState=active\nSubState=running",
        "MainPID=42\nActiveState=failed\nSubState=failed",
        "MainPID=42\nActiveState=active",
    ] {
        assert!(parse_service(value).is_err());
    }
}

#[test]
fn apply_requires_a_reviewed_plan_hash_and_full_revisions() {
    assert!(!hex("../release", 40));
    assert!(!hex(&"A".repeat(40), 40));
    assert!(hex(&"a".repeat(40), 40));
    assert!(Cli::try_parse_from(["deploy", "release", "--apply"]).is_err());
    let base = [
        "deploy",
        "release",
        "--revision",
        "a",
        "--candidate-source",
        "/source",
        "--candidate-binary",
        "/candidate",
        "--expected-source-revision",
        "b",
        "--expected-current-binary-sha",
        "c",
        "--expected-current-static-sha",
        "d",
        "--expected-binary-sha",
        "e",
        "--expected-privacy-sha",
        "f",
    ];
    assert!(Cli::try_parse_from(base).is_ok());
    assert!(Cli::try_parse_from(base.into_iter().chain(["--apply"])).is_err());
    assert!(Cli::try_parse_from(
        base.into_iter()
            .chain(["--apply", "--expected-plan-sha", "g"])
    )
    .is_ok());
}
