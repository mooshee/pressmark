# Pressmark release operator

Author: Daniel Hallman

This Rust command replaces the existing Linux service binary and `static/privacy.html` on the Pressmark host. It plans by default. It checks the current source revision, running executable, complete static file tree, and protected configuration before applying a reviewed plan. It does not update the source checkout, edit configuration, or restore customer data.

## Prepare

Use a clean candidate checkout at the reviewed commit. Build the application natively on the host, and record the candidate binary and privacy-page hashes. Build this operator separately:

```console
cargo build --release --locked --manifest-path tools/pressmark-deploy/Cargo.toml
sudo tools/pressmark-deploy/target/release/pressmark-deploy inspect
```

`inspect` returns the production source revision, binary/static hashes, protected-file hashes and running process. It does not print configuration contents. The target paths and service are fixed in the operator.

## Plan and apply

1. Run `release` with the approved current values and candidate hashes. Omit `--apply`. Required arguments are listed by `release --help`.
2. Review the returned plan. It names the two changed files, expected resulting static hash, source revision, protected hashes and private rollback directory.
3. Run the same command with `--apply --expected-plan-sha <plan_sha256>`. A changed source, process, binary, static tree, configuration or candidate rejects the old plan. Read the new state before preparing another plan.
4. Confirm `status: activated`, the new process and artifact hashes. The operator saves its plan, exact previous/candidate bytes and receipt under `/var/backups/pressmark/releases/<revision>/`.
5. Perform the required public and host-level product checks. A healthy process alone does not establish correct MCP downloads or a successful ChatGPT demonstration.

The apply step holds a service release lock, writes private backups, replaces each file atomically, restarts the service, and checks its executable and health response. It verifies that the full static tree differs only by the approved privacy page and that protected configuration and the source checkout stayed unchanged.

If activation or verification fails, the operator restores only files still matching its own installed bytes and verifies the previous release. It never overwrites a concurrent foreign change. A rollback failure returns `rollback_requires_attention` with a private receipt. Do not rerun a revision whose backup directory already exists; inspect its receipt and current state first.

If the release passes verification but its receipt cannot be saved, the command returns `activated_receipt_requires_attention` and exits with an error. The verified release remains active. Retain the returned result and inspect the backup directory before taking another action.

Tests use isolated temporary files to check drift refusal, exact-byte restoration, permission preservation, static-tree coverage, symlink rejection and command-line requirements. Run them with:

```console
cargo nextest run --manifest-path tools/pressmark-deploy/Cargo.toml
```
