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
7. **Shared Document Engine (`crates/hallman-doc-engine`) & 3-Mode Signable PDFs**:
   - Pressmark shares `crates/hallman-doc-engine` with Untax (`untax.io`) for `ReadabilityPreset` (`Standard` `9.5pt`, `HighLegibility` `10.4pt` with `0.65em` leading and `#000000` ink, `ExecutiveCompact` `8.8pt`, `JudicialFormal` `9.5pt`), Typst escaping, and `lopdf`-based interactive `/AcroForm` signature/date field injection (`inject_fillable_signature_acroform`).
   - All unsigned / wet-ink contracts, leases, notices, and signature blocks automatically embed clickable `/AcroForm` `/Widget` (`/FT /Tx`) fields (`taxpayer_signature`, `execution_date`, `counterparty_signature`, `counterparty_date`, and notary fields when applicable) while keeping financial line items and totals immutable as flattened vector text.
   - Maintain minimum print/screen legibility floors (`>= 9.5pt` Standard body, `>= 10.4pt` HighLegibility body, `>= 8pt` clauses/headers, `>= 10.5pt` KPI figures, and `13pt` italic serif e-Sign signatures).
