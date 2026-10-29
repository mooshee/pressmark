// Author: Daniel Hallman
// Fixed-target binary and privacy-page release; customer data is never restored.
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::{
        fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
        io::AsRawFd,
    },
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    thread,
    time::{Duration, Instant},
};

type Result<T> = std::result::Result<T, String>;
const SOURCE: &str = "/home/ubuntu/pressmark";
const BINARY: &str = "/usr/local/bin/pressmark";
const STATIC: &str = "/var/lib/pressmark/static";
const SERVICE: &str = "pressmark.service";
const LOCK: &str = "/var/lock/pressmark-release.lock";
const BACKUPS: &str = "/var/backups/pressmark/releases";
const MAX_BINARY: u64 = 128 * 1024 * 1024;
const MAX_PAGE: u64 = 512 * 1024;
const PROTECTED: &[&str] = &[
    "/etc/pressmark/pressmark.env",
    "/etc/systemd/system/pressmark.service",
    "/etc/caddy/Caddyfile",
    "/etc/caddy/products/pressmark.caddy",
];

#[derive(Parser)]
#[command(
    about = "Plan by default; release only the existing Pressmark service binary and privacy page"
)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Read fixed production paths and return hashes, source revision and process state.
    Inspect,
    /// Hash one regular file without printing its contents.
    HashFile { path: PathBuf },
    /// Validate a candidate. --apply also requires the exact reviewed plan hash.
    Release(ReleaseArgs),
}

#[derive(clap::Args)]
struct ReleaseArgs {
    #[arg(long)]
    revision: String,
    #[arg(long)]
    candidate_source: PathBuf,
    #[arg(long)]
    candidate_binary: PathBuf,
    #[arg(long)]
    expected_source_revision: String,
    #[arg(long)]
    expected_current_binary_sha: String,
    #[arg(long)]
    expected_current_static_sha: String,
    #[arg(long)]
    expected_binary_sha: String,
    #[arg(long)]
    expected_privacy_sha: String,
    #[arg(long, requires = "apply")]
    expected_plan_sha: Option<String>,
    #[arg(long, requires = "expected_plan_sha")]
    apply: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Entry {
    bytes: u64,
    sha256: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
struct Runtime {
    pid: u32,
    active: String,
    sub: String,
    binary_sha256: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
struct Baseline {
    source_revision: String,
    binary_sha256: String,
    binary_mode: u32,
    static_sha256: String,
    privacy_sha256: String,
    privacy_mode: u32,
    protected_sha256: BTreeMap<String, String>,
    runtime: Runtime,
}

#[derive(Debug, Serialize)]
struct Plan {
    revision: String,
    candidate_source: PathBuf,
    candidate_binary: PathBuf,
    baseline: Baseline,
    binary_sha256: String,
    privacy_sha256: String,
    resulting_static_sha256: String,
    changed_paths: [&'static str; 2],
    backup_directory: PathBuf,
    source_checkout_changed: bool,
    configuration_changed: bool,
    customer_data_changed: bool,
}

struct Loaded {
    plan: Plan,
    old_binary: Vec<u8>,
    old_privacy: Vec<u8>,
    new_binary: Vec<u8>,
    new_privacy: Vec<u8>,
    binary_mode: u32,
    privacy_mode: u32,
}

fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn read_regular(path: &Path, limit: u64) -> Result<Vec<u8>> {
    let metadata =
        fs::symlink_metadata(path).map_err(|_| format!("Cannot inspect {}", path.display()))?;
    require(
        metadata.is_file() && metadata.len() <= limit,
        "Expected a bounded regular file",
    )?;
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(|_| format!("Cannot open {}", path.display()))?;
    let opened = file.metadata().map_err(|_| "Cannot inspect opened file")?;
    require(
        opened.ino() == metadata.ino() && opened.dev() == metadata.dev(),
        "File changed while opening",
    )?;
    let mut bytes = Vec::with_capacity(opened.len() as usize);
    file.take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Cannot read file")?;
    require(bytes.len() as u64 <= limit, "File exceeds size limit")?;
    Ok(bytes)
}

fn file_hash(path: &Path) -> Result<String> {
    Ok(digest(&read_regular(path, MAX_BINARY)?))
}

fn tree(path: &Path) -> Result<BTreeMap<String, Entry>> {
    require(
        fs::symlink_metadata(path)
            .map_err(|_| "Cannot inspect static root")?
            .is_dir(),
        "Static root must be a real directory",
    )?;
    fn visit(root: &Path, current: &Path, result: &mut BTreeMap<String, Entry>) -> Result<()> {
        for item in fs::read_dir(current).map_err(|_| "Cannot read static directory")? {
            let path = item.map_err(|_| "Cannot read static entry")?.path();
            let metadata =
                fs::symlink_metadata(&path).map_err(|_| "Cannot inspect static entry")?;
            if metadata.is_dir() {
                visit(root, &path, result)?;
            } else {
                require(
                    metadata.is_file(),
                    "Static tree contains a symlink or special file",
                )?;
                let relative = path
                    .strip_prefix(root)
                    .map_err(|_| "Invalid static path")?
                    .to_str()
                    .ok_or("Static path is not UTF-8")?
                    .to_owned();
                let bytes = read_regular(&path, MAX_BINARY)?;
                result.insert(
                    relative,
                    Entry {
                        bytes: bytes.len() as u64,
                        sha256: digest(&bytes),
                    },
                );
            }
        }
        Ok(())
    }
    let mut result = BTreeMap::new();
    visit(path, path, &mut result)?;
    require(!result.is_empty(), "Static tree is empty")?;
    Ok(result)
}

fn tree_digest(entries: &BTreeMap<String, Entry>) -> Result<String> {
    Ok(digest(
        &serde_json::to_vec(entries).map_err(|_| "Cannot hash static manifest")?,
    ))
}

fn overlay(entries: &BTreeMap<String, Entry>, privacy: &[u8]) -> Result<String> {
    require(
        entries.contains_key("privacy.html"),
        "Live privacy page is missing",
    )?;
    let mut candidate = entries.clone();
    candidate.insert(
        "privacy.html".into(),
        Entry {
            bytes: privacy.len() as u64,
            sha256: digest(privacy),
        },
    );
    tree_digest(&candidate)
}

fn command(program: &str, args: &[&str], cwd: Option<&Path>) -> Result<Vec<u8>> {
    // Production source belongs to ubuntu. Read Git state as that owner rather
    // than trusting its configuration or filesystem helpers with root access.
    let mut cmd = if program == "git" && unsafe { libc::geteuid() } == 0 {
        let mut command = Command::new("runuser");
        command.args(["-u", "ubuntu", "--", "git"]);
        command
    } else {
        Command::new(program)
    };
    if program == "git" {
        cmd.args([
            "-c",
            "core.fsmonitor=false",
            "-c",
            "core.hooksPath=/dev/null",
        ]);
        for (key, _) in std::env::vars_os() {
            if key.to_str().is_some_and(|key| key.starts_with("GIT_")) {
                cmd.env_remove(key);
            }
        }
        cmd.env("GIT_OPTIONAL_LOCKS", "0");
    }
    cmd.args(args);
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }
    let output = cmd.output().map_err(|_| format!("Cannot run {program}"))?;
    require(
        output.status.success(),
        &format!("{program} did not complete successfully"),
    )?;
    require(
        output.stdout.len() <= 1024 * 1024,
        "Command output exceeds limit",
    )?;
    Ok(output.stdout)
}

fn source_revision(path: &Path) -> Result<String> {
    require(
        path.is_absolute()
            && fs::canonicalize(path).map_err(|_| "Source directory is unavailable")? == path,
        "Source directory must be an absolute path without symlinks",
    )?;
    let revision = String::from_utf8(command("git", &["rev-parse", "HEAD"], Some(path))?)
        .map_err(|_| "Invalid source revision")?
        .trim()
        .to_owned();
    require(hex(&revision, 40), "Invalid source revision")?;
    require(
        command(
            "git",
            &["status", "--porcelain=v1", "--untracked-files=all"],
            Some(path),
        )?
        .is_empty(),
        "Source checkout contains local changes",
    )?;
    Ok(revision)
}

fn protected_hashes() -> Result<BTreeMap<String, String>> {
    PROTECTED
        .iter()
        .map(|path| Ok((path.to_string(), file_hash(Path::new(path))?)))
        .collect()
}

fn parse_service(text: &str) -> Result<(u32, String, String)> {
    let fields: BTreeMap<_, _> = text
        .lines()
        .filter_map(|line| line.split_once('='))
        .collect();
    let pid = fields
        .get("MainPID")
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or("Service PID is missing")?;
    let active = fields
        .get("ActiveState")
        .ok_or("Service state is missing")?
        .to_string();
    let sub = fields
        .get("SubState")
        .ok_or("Service substate is missing")?
        .to_string();
    require(
        pid > 1 && active == "active" && sub == "running",
        "Service is not running",
    )?;
    Ok((pid, active, sub))
}

fn runtime() -> Result<Runtime> {
    let bytes = command(
        "systemctl",
        &[
            "show",
            SERVICE,
            "--property=MainPID",
            "--property=ActiveState",
            "--property=SubState",
        ],
        None,
    )?;
    let (pid, active, sub) =
        parse_service(std::str::from_utf8(&bytes).map_err(|_| "Invalid service state")?)?;
    let executable = PathBuf::from(format!("/proc/{pid}/exe"));
    require(
        fs::read_link(&executable).map_err(|_| "Cannot inspect service executable")?
            == Path::new(BINARY),
        "Service executable is not the fixed production path",
    )?;
    // /proc/<verified systemd PID>/exe is a kernel-owned symlink, unlike candidate files.
    let bytes = fs::read(executable).map_err(|_| "Cannot read running executable")?;
    require(
        bytes.len() as u64 <= MAX_BINARY,
        "Running executable exceeds limit",
    )?;
    Ok(Runtime {
        pid,
        active,
        sub,
        binary_sha256: digest(&bytes),
    })
}

fn baseline() -> Result<Baseline> {
    let binary_sha256 = file_hash(Path::new(BINARY))?;
    let runtime = runtime()?;
    require(
        runtime.binary_sha256 == binary_sha256,
        "Running executable differs from the installed binary",
    )?;
    Ok(Baseline {
        source_revision: source_revision(Path::new(SOURCE))?,
        binary_sha256,
        binary_mode: active_mode(Path::new(BINARY))?,
        static_sha256: tree_digest(&tree(Path::new(STATIC))?)?,
        privacy_sha256: file_hash(&Path::new(STATIC).join("privacy.html"))?,
        privacy_mode: active_mode(&Path::new(STATIC).join("privacy.html"))?,
        protected_sha256: protected_hashes()?,
        runtime,
    })
}

fn health() -> Result<()> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(3))
        .redirect(reqwest::redirect::Policy::none())
        .no_proxy()
        .build()
        .map_err(|_| "Cannot create health client")?;
    let response = client
        .get("http://127.0.0.1:3842/health")
        .send()
        .map_err(|_| "Health request failed")?;
    require(
        response.status().is_success(),
        "Health endpoint returned an error",
    )?;
    require(
        response.content_length().is_none_or(|n| n <= 4096),
        "Health reply exceeds limit",
    )?;
    let mut bytes = Vec::new();
    response
        .take(4097)
        .read_to_end(&mut bytes)
        .map_err(|_| "Cannot read health response")?;
    require(bytes.len() <= 4096, "Health reply exceeds limit")?;
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|_| "Health reply is not JSON")?;
    require(
        value["status"] == "healthy" && value["service"] == "pressmark",
        "Unexpected health identity",
    )
}

fn active_mode(path: &Path) -> Result<u32> {
    let metadata = fs::symlink_metadata(path).map_err(|_| "Cannot inspect active permissions")?;
    require(
        metadata.is_file() && metadata.uid() == 0 && metadata.mode() & 0o6022 == 0,
        "Active file must be root-owned without write access for other users or special bits",
    )?;
    Ok(metadata.mode() & 0o777)
}

fn same_elf_architecture(previous: &[u8], candidate: &[u8]) -> bool {
    previous.len() >= 20
        && candidate.len() >= 20
        && previous.starts_with(b"\x7fELF")
        && candidate.starts_with(b"\x7fELF")
        && previous[4..7] == candidate[4..7]
        && previous[18..20] == candidate[18..20]
}

fn load(args: &ReleaseArgs) -> Result<Loaded> {
    require(
        hex(&args.revision, 40) && hex(&args.expected_source_revision, 40),
        "Source revisions must be full lowercase commit hashes",
    )?;
    for value in [
        &args.expected_current_binary_sha,
        &args.expected_current_static_sha,
        &args.expected_binary_sha,
        &args.expected_privacy_sha,
    ] {
        require(
            hex(value, 64),
            "Expected hashes must be lowercase SHA-256 values",
        )?;
    }
    let baseline = baseline()?;
    require(
        baseline.source_revision == args.expected_source_revision,
        "Production source revision drifted",
    )?;
    require(
        baseline.binary_sha256 == args.expected_current_binary_sha,
        "Production binary drifted",
    )?;
    require(
        baseline.static_sha256 == args.expected_current_static_sha,
        "Production static tree drifted",
    )?;
    require(
        source_revision(&args.candidate_source)? == args.revision,
        "Candidate source revision differs",
    )?;
    let old_binary = read_regular(Path::new(BINARY), MAX_BINARY)?;
    let privacy = Path::new(STATIC).join("privacy.html");
    let old_privacy = read_regular(&privacy, MAX_PAGE)?;
    let new_binary = read_regular(&args.candidate_binary, MAX_BINARY)?;
    let new_privacy = read_regular(&args.candidate_source.join("static/privacy.html"), MAX_PAGE)?;
    let tracked_privacy = command(
        "git",
        &["show", &format!("{}:static/privacy.html", args.revision)],
        Some(&args.candidate_source),
    )?;
    require(
        new_privacy == tracked_privacy,
        "Candidate privacy page differs from its commit",
    )?;
    require(
        digest(&new_binary) == args.expected_binary_sha
            && digest(&new_privacy) == args.expected_privacy_sha,
        "Candidate artifact hash differs",
    )?;
    require(
        digest(&old_binary) == baseline.binary_sha256
            && digest(&old_privacy) == baseline.privacy_sha256,
        "Production artifacts changed during plan construction",
    )?;
    require(
        same_elf_architecture(&old_binary, &new_binary),
        "Candidate must be a native ELF executable for the current architecture",
    )?;
    require(
        args.expected_binary_sha != args.expected_current_binary_sha,
        "Candidate binary is unchanged",
    )?;
    let binary_mode = baseline.binary_mode;
    let privacy_mode = baseline.privacy_mode;
    require(binary_mode & 0o111 != 0, "Active binary is not executable")?;
    health()?;
    let static_entries = tree(Path::new(STATIC))?;
    require(
        tree_digest(&static_entries)? == baseline.static_sha256,
        "Static tree changed while constructing the plan",
    )?;
    let plan = Plan {
        revision: args.revision.clone(),
        candidate_source: args.candidate_source.clone(),
        candidate_binary: args.candidate_binary.clone(),
        resulting_static_sha256: overlay(&static_entries, &new_privacy)?,
        baseline,
        binary_sha256: args.expected_binary_sha.clone(),
        privacy_sha256: args.expected_privacy_sha.clone(),
        changed_paths: [BINARY, "/var/lib/pressmark/static/privacy.html"],
        backup_directory: Path::new(BACKUPS).join(&args.revision),
        source_checkout_changed: false,
        configuration_changed: false,
        customer_data_changed: false,
    };
    require(
        !plan.backup_directory.exists(),
        "Release backup already exists; use its receipt instead of repeating the release",
    )?;
    Ok(Loaded {
        plan,
        old_binary,
        old_privacy,
        new_binary,
        new_privacy,
        binary_mode,
        privacy_mode,
    })
}

fn plan_digest(plan: &Plan) -> Result<String> {
    Ok(digest(
        &serde_json::to_vec(plan).map_err(|_| "Cannot hash release plan")?,
    ))
}

fn private_directory(path: &Path, create_new: bool) -> Result<()> {
    if create_new {
        fs::create_dir(path)
            .map_err(|_| "Private release directory already exists or cannot be created")?;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .map_err(|_| "Cannot protect release directory")?;
        File::open(path.parent().ok_or("Private directory has no parent")?)
            .and_then(|parent| parent.sync_all())
            .map_err(|_| "Cannot sync private directory entry")?;
    } else if !path.exists() {
        private_directory(path, true)?;
    }
    let metadata = fs::symlink_metadata(path).map_err(|_| "Cannot inspect private directory")?;
    require(
        metadata.is_dir() && metadata.uid() == 0 && metadata.mode() & 0o077 == 0,
        "Backup directory must be private and root-owned",
    )
}

fn write_new(path: &Path, bytes: &[u8], mode: u32) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(|_| "Output already exists or cannot be created")?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(|_| "Cannot write durable output")?;
    file.set_permissions(fs::Permissions::from_mode(mode))
        .map_err(|_| "Cannot set output permissions")?;
    file.sync_all()
        .map_err(|_| "Cannot sync output metadata".into())
}

fn swap(
    path: &Path,
    expected: &str,
    bytes: &[u8],
    mode: u32,
    revision: &str,
    changed: &mut bool,
) -> Result<()> {
    require(
        file_hash(path)? == expected,
        "Target changed before replacement",
    )?;
    let parent = path.parent().ok_or("Target has no parent")?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Invalid target name")?;
    let temporary = parent.join(format!(".{name}.release-{revision}"));
    write_new(&temporary, bytes, mode)?;
    let result = (|| {
        require(
            file_hash(path)? == expected,
            "Target changed while preparing replacement",
        )?;
        require(
            file_hash(&temporary)? == digest(bytes),
            "Replacement bytes changed",
        )?;
        fs::rename(&temporary, path).map_err(|_| "Cannot activate replacement")?;
        *changed = true;
        File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| "Cannot sync target directory")?;
        require(
            file_hash(path)? == digest(bytes),
            "Active file differs from replacement",
        )
    })();
    if temporary.exists() && file_hash(&temporary).ok().as_deref() == Some(digest(bytes).as_str()) {
        let _ = fs::remove_file(temporary);
    }
    result
}

fn release_lock() -> Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(LOCK)
        .map_err(|_| "Cannot open release lock")?;
    let metadata = file.metadata().map_err(|_| "Cannot inspect release lock")?;
    require(
        metadata.is_file() && metadata.uid() == 0 && metadata.mode() & 0o077 == 0,
        "Release lock is not private and root-owned",
    )?;
    // The file owns this descriptor for the lock's entire lifetime.
    require(
        unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0,
        "Another release holds the service lock",
    )?;
    Ok(file)
}

fn restart_and_verify(expected: &str, previous_pid: u32) -> Result<Runtime> {
    command("systemctl", &["restart", SERVICE], None)?;
    let deadline = Instant::now() + Duration::from_secs(20);
    while Instant::now() < deadline {
        if let Ok(runtime) = runtime() {
            if runtime.pid != previous_pid && runtime.binary_sha256 == expected && health().is_ok()
            {
                return Ok(runtime);
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
    Err("Service did not become healthy with the approved executable".into())
}

fn verify_fixed_state(plan: &Plan, binary_sha: &str, static_sha: &str) -> Result<()> {
    require(
        source_revision(Path::new(SOURCE))? == plan.baseline.source_revision,
        "Production source changed during release",
    )?;
    require(
        protected_hashes()? == plan.baseline.protected_sha256,
        "Protected configuration changed during release",
    )?;
    require(
        file_hash(Path::new(BINARY))? == binary_sha,
        "Installed executable differs",
    )?;
    require(
        tree_digest(&tree(Path::new(STATIC))?)? == static_sha,
        "Static tree differs from approved result",
    )
}

fn apply(loaded: Loaded, expected_plan: &str) -> Result<serde_json::Value> {
    require(
        cfg!(target_os = "linux") && unsafe { libc::geteuid() } == 0,
        "Release application requires root on the production Linux host",
    )?;
    require(
        hex(expected_plan, 64) && plan_digest(&loaded.plan)? == expected_plan,
        "Plan changed; review a fresh dry-run before applying",
    )?;
    let _lock = release_lock()?;
    let plan = &loaded.plan;
    require(
        baseline()? == plan.baseline,
        "Production changed after plan validation",
    )?;
    require(
        source_revision(&plan.candidate_source)? == plan.revision,
        "Candidate source changed after plan validation",
    )?;
    private_directory(Path::new("/var/backups/pressmark"), false)?;
    private_directory(Path::new(BACKUPS), false)?;
    private_directory(&plan.backup_directory, true)?;
    write_new(
        &plan.backup_directory.join("previous-pressmark"),
        &loaded.old_binary,
        0o600,
    )?;
    write_new(
        &plan.backup_directory.join("previous-privacy.html"),
        &loaded.old_privacy,
        0o600,
    )?;
    write_new(
        &plan.backup_directory.join("candidate-pressmark"),
        &loaded.new_binary,
        0o600,
    )?;
    write_new(
        &plan.backup_directory.join("candidate-privacy.html"),
        &loaded.new_privacy,
        0o600,
    )?;
    write_new(
        &plan.backup_directory.join("plan.json"),
        &serde_json::to_vec_pretty(plan).map_err(|_| "Cannot encode plan")?,
        0o600,
    )?;
    File::open(&plan.backup_directory)
        .and_then(|d| d.sync_all())
        .map_err(|_| "Cannot sync backup directory")?;
    require(
        baseline()? == plan.baseline,
        "Production changed before activation",
    )?;
    let privacy = Path::new(STATIC).join("privacy.html");
    let mut binary_changed = false;
    let mut privacy_changed = false;
    let mut restarted_pid = None;
    let activation: Result<serde_json::Value> = (|| {
        swap(
            Path::new(BINARY),
            &plan.baseline.binary_sha256,
            &loaded.new_binary,
            loaded.binary_mode,
            &plan.revision,
            &mut binary_changed,
        )?;
        swap(
            &privacy,
            &plan.baseline.privacy_sha256,
            &loaded.new_privacy,
            loaded.privacy_mode,
            &plan.revision,
            &mut privacy_changed,
        )?;
        let runtime = restart_and_verify(&plan.binary_sha256, plan.baseline.runtime.pid)?;
        restarted_pid = Some(runtime.pid);
        verify_fixed_state(plan, &plan.binary_sha256, &plan.resulting_static_sha256)?;
        require(
            runtime == self::runtime()?,
            "Service process changed during final verification",
        )?;
        health()?;
        let receipt = serde_json::json!({"status":"activated","plan_sha256":expected_plan,"plan":plan,"runtime":runtime,
            "rollback_directory":plan.backup_directory,"configuration_unchanged":true,"customer_data_restored":false});
        Ok(receipt)
    })();
    match activation {
        Ok(mut receipt) => {
            let saved = write_new(
                &plan.backup_directory.join("receipt.json"),
                &serde_json::to_vec_pretty(&receipt).map_err(|_| "Cannot encode receipt")?,
                0o600,
            );
            if saved.is_err() {
                receipt["status"] = serde_json::json!("activated_receipt_requires_attention");
                println!(
                    "{}",
                    serde_json::to_string_pretty(&receipt).map_err(|_| "Cannot encode receipt")?
                );
                return Err("Release is active and verified, but saving the receipt failed; retain the returned result and inspect the backup directory".into());
            }
            Ok(receipt)
        }
        Err(error) => {
            let rollback: Result<Runtime> = (|| {
                let privacy_restore = if privacy_changed {
                    swap(
                        &privacy,
                        &plan.privacy_sha256,
                        &loaded.old_privacy,
                        loaded.privacy_mode,
                        &format!("{}-rollback", plan.revision),
                        &mut false,
                    )
                } else {
                    Ok(())
                };
                let binary_restore = if binary_changed {
                    swap(
                        Path::new(BINARY),
                        &plan.binary_sha256,
                        &loaded.old_binary,
                        loaded.binary_mode,
                        &format!("{}-rollback", plan.revision),
                        &mut false,
                    )
                } else {
                    Ok(())
                };
                // A foreign privacy change must not prevent restoring our own executable.
                // Never restart after an executable restore failure: its bytes may be foreign.
                binary_restore?;
                let runtime = if binary_changed {
                    restart_and_verify(
                        &plan.baseline.binary_sha256,
                        restarted_pid.unwrap_or(plan.baseline.runtime.pid),
                    )?
                } else {
                    runtime()?
                };
                privacy_restore?;
                verify_fixed_state(
                    plan,
                    &plan.baseline.binary_sha256,
                    &plan.baseline.static_sha256,
                )?;
                Ok(runtime)
            })();
            let receipt = serde_json::json!({"status":if rollback.is_ok(){"rolled_back"}else{"rollback_requires_attention"},
                "activation_error":error,"rollback_error":rollback.as_ref().err(),"runtime":rollback.as_ref().ok(),
                "plan_sha256":expected_plan,"plan":plan,"customer_data_restored":false});
            let _ = write_new(
                &plan.backup_directory.join("receipt.json"),
                &serde_json::to_vec_pretty(&receipt)
                    .map_err(|_| "Cannot encode failure receipt")?,
                0o600,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&receipt)
                    .map_err(|_| "Cannot encode failure receipt")?
            );
            Err(if rollback.is_ok() {
                format!("{error}; previous binary and static bytes restored and verified")
            } else {
                format!("{error}; rollback requires attention; see the private release receipt")
            })
        }
    }
}

fn run() -> Result<()> {
    let value = match Cli::parse().action {
        Action::Inspect => {
            serde_json::to_value(baseline()?).map_err(|_| "Cannot encode inspection")?
        }
        Action::HashFile { path } => serde_json::json!({"sha256":file_hash(&path)?}),
        Action::Release(args) => {
            let loaded = load(&args)?;
            if args.apply {
                apply(
                    loaded,
                    args.expected_plan_sha
                        .as_deref()
                        .ok_or("Reviewed plan hash is required")?,
                )?
            } else {
                serde_json::json!({"status":"validated","mode":"plan","plan_sha256":plan_digest(&loaded.plan)?,"plan":loaded.plan})
            }
        }
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&value).map_err(|_| "Cannot encode result")?
    );
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("pressmark-deploy: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests;
