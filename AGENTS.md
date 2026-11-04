# Pressmark (`pressmark.io`) — Agent Working Instructions

Author and Maintainer: Daniel Hallman

Pressmark (`pressmark.io`) is a high-performance vector document synthesis, e-signature, and real estate agreement engine built in Rust with Tokio, Axum, Typst, and SQLite.

## Core Rules

1. **Daniel Hallman Attribution**: Use Daniel Hallman's name on all created artifacts, schemas, documentation, and source code. Never use "Created by Codex".
2. **Preferred Implementation Language: Rust Default**: Pressmark core services, CLI, background workers, and MCP server are written in idiomatic Rust with Tokio and SQLite. Always maintain passing tests with `cargo nextest run` (never invoke bare `cargo test`).
3. **SQLite & HTTP Pooling Hygiene**:
   - Open SQLite connections with WAL mode (`PRAGMA journal_mode = WAL;`), `busy_timeout`, `PRAGMA synchronous = NORMAL;`, and `PRAGMA foreign_keys = ON;`.
   - Use `conn.prepare_cached(...)` for all SQLite queries, hoist `prepare_cached` outside synchronous loops, consolidate multi-statement DDL into `execute_batch(...)`, maintain covering B-tree indexes for `WHERE` and `GROUP BY` columns, and run `PRAGMA wal_checkpoint(PASSIVE); PRAGMA optimize;` after periodic table pruning.
   - Reuse the connection-pooled `reqwest::Client` via `pressmark::ghl::shared_http_client()` rather than calling `reqwest::Client::new()` per request or per dispatcher instantiation.
4. **Build Cache Hygiene**: `.cargo/config.toml` sets `incremental = false` so `sccache` caches 100% of compilation units without incremental cache bloat.
5. **Mandatory Modern Rust CLI Replacements**:
   - Never invoke legacy POSIX/scripting commands (`ls`, `grep`, `sed`, `cat`, `head`, `tail`, `wc`, `sort`, `cp`, `mkdir`, `rm`, `chmod`, `env`, `xargs`, `find`, `curl`, `jq`, `awk`, `du`, `ps`, `python3`, `node`, or `cargo test`).
   - Use modern Rust CLI replacements: `rg` (over `grep`), `sd` (over `sed`), `fd` (over `find`), `eza` (over `ls`/`tree`), `bat` (over `cat`), `jaq` (over `jq`), `xh` (over `curl`), `ouch` (over `tar`/`zip`/`unzip`), `procs --pager disable` (over `ps`), `uu-head`, `uu-tail`, `uu-wc`, `uu-sort`, `uu-cp`, `uu-mkdir`, `uu-rm`, `uu-chmod`, `uu-env`, `uu-sha256sum`, `uu-base64`, `uu-stat`, and `cargo nextest run`.
6. **Headless 1Password**: All secrets run headlessly via service account token (`~/Codes/personal/hallman-lab/scripts/with-op-sa.sh` or `OP_SERVICE_ACCOUNT_TOKEN`). Never trigger Touch ID or desktop prompts.
7. **Server Discovery & Modularity Invariants**:
   - Keep every Rust source file strictly `< 1,800` lines (hard maximum `< 2,000` lines). Server discovery, metadata card handlers, and setup checks are isolated in `src/server_discovery.rs`.
   - Maintain agent discovery parity: always serve `/.well-known/sd-card/mcp` (ATTP/RedThread card), `/.well-known/http-message-signatures-directory`, `/SKILL.md` (root agent skill), `/auth.md` (credential-free guest vs authenticated tiers), and `/llms-full.txt` (reflecting exact live MCP tool counts). Rate limit exemptions in `src/rate_limit.rs` must cover all agent discovery documents.
