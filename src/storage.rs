//! Pressmark Persistent Storage Engine (SQLite)
//!
//! Author: Daniel Hallman
//!
//! Provides durable, restart-safe persistence for:
//! 1. ChatGPT OAuth user sessions and subscription tokens
//! 2. PKCE pending authorizations
//! 3. Stripe webhook idempotency event tracking

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension, Result};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::auth::{ChatGptSession, PendingAuth};

#[derive(Clone)]
pub struct Storage {
    conn: Arc<Mutex<Connection>>,
    db_path: PathBuf,
    mcp_attempt_write_failures: Arc<AtomicU64>,
    mcp_outcome_write_failures: Arc<AtomicU64>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CheckoutStart {
    Created,
    Replayed,
    Conflict,
}

impl Storage {
    /// Initialize storage at the given path (or in-memory if ":memory:").
    /// Automatically applies WAL mode and creates required tables.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db_path = path.as_ref().to_path_buf();

        if let Some(parent) = db_path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                let _ = std::fs::create_dir_all(parent);
            }
        }

        let conn = if db_path.to_str() == Some(":memory:") {
            Connection::open_in_memory()?
        } else {
            Connection::open(&db_path)?
        };

        // Enable Write-Ahead Logging (WAL) for concurrent read performance
        let _ = conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA busy_timeout = 5000;",
        );

        let storage = Self {
            conn: Arc::new(Mutex::new(conn)),
            db_path,
            mcp_attempt_write_failures: Arc::new(AtomicU64::new(0)),
            mcp_outcome_write_failures: Arc::new(AtomicU64::new(0)),
        };

        storage.init_schema()?;
        Ok(storage)
    }

    /// Initialize default persistent database from environment variable `PRESSMARK_DB_PATH`
    /// or defaulting to `.local/pressmark_sessions.db`.
    pub fn default_db() -> Self {
        let path = std::env::var("PRESSMARK_DB_PATH")
            .unwrap_or_else(|_| ".local/pressmark_sessions.db".to_string());
        Self::new(path).unwrap_or_else(|e| {
            tracing::warn!("Failed to open persistent DB at path, falling back to memory: {}", e);
            Self::new(":memory:").expect("In-memory SQLite DB must always succeed")
        })
    }

    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    pub fn check_ready(&self) -> Result<()> {
        let lock = self.conn.lock().unwrap();
        lock.prepare_cached("SELECT 1;")?
            .query_row([], |_| Ok(()))?;
        Ok(())
    }

    fn init_schema(&self) -> Result<()> {
        let lock = self.conn.lock().unwrap();

        lock.execute_batch(
            "CREATE TABLE IF NOT EXISTS sessions (
                session_id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                user_json TEXT NOT NULL,
                access_token TEXT NOT NULL,
                refresh_token TEXT,
                scope TEXT NOT NULL,
                expires_at TEXT NOT NULL,
                created_at TEXT NOT NULL,
                delegated_compute_active INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_sessions_access_token ON sessions(access_token);
            CREATE INDEX IF NOT EXISTS idx_sessions_expires ON sessions(expires_at);

            CREATE TABLE IF NOT EXISTS pending_auths (
                state TEXT PRIMARY KEY,
                code_verifier TEXT NOT NULL,
                redirect_uri TEXT NOT NULL,
                return_to TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_pending_auths_created ON pending_auths(created_at);

            CREATE TABLE IF NOT EXISTS processed_stripe_events (
                event_id TEXT PRIMARY KEY,
                processed_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_processed_stripe_at ON processed_stripe_events(processed_at);

            CREATE TABLE IF NOT EXISTS checkout_starts (
                request_id TEXT PRIMARY KEY,
                request_fingerprint TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS customer_entitlements (
                email TEXT PRIMARY KEY,
                plan TEXT NOT NULL,
                stripe_session_id TEXT,
                amount_cents INTEGER NOT NULL DEFAULT 0,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS usage_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                event_type TEXT NOT NULL,
                archetype TEXT,
                channel TEXT NOT NULL,
                detail TEXT,
                compile_ms INTEGER,
                created_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_usage_events_type ON usage_events(event_type);
            CREATE INDEX IF NOT EXISTS idx_usage_events_created_at ON usage_events(created_at);
            CREATE INDEX IF NOT EXISTS idx_usage_events_channel_detail ON usage_events(channel, detail);
            CREATE INDEX IF NOT EXISTS idx_usage_events_archetype ON usage_events(archetype);
            CREATE INDEX IF NOT EXISTS idx_usage_events_type_detail ON usage_events(event_type, detail);
            UPDATE usage_events SET archetype = REPLACE(archetype, '-', '_') WHERE archetype IS NOT NULL AND archetype LIKE '%-%';",
        )?;

        // Nullable columns preserve the unknown outcome of historical attempts.
        let columns: Vec<String> = lock.prepare_cached("PRAGMA table_info(usage_events)")?
            .query_map([], |row| row.get(1))?.collect::<Result<_>>()?;
        for (name, kind) in [("outcome", "TEXT"), ("completed_at", "TEXT"), ("error_code", "INTEGER")] {
            if !columns.iter().any(|column| column == name) {
                lock.execute_batch(&format!("ALTER TABLE usage_events ADD COLUMN {name} {kind};"))?;
            }
        }
        lock.execute_batch("CREATE TABLE IF NOT EXISTS usage_measurement_metadata (key TEXT PRIMARY KEY, value TEXT NOT NULL);")?;
        lock.prepare_cached(
            "INSERT OR IGNORE INTO usage_measurement_metadata (key, value) VALUES ('mcp_outcomes_started_at', ?1)",
        )?
        .execute([Utc::now().to_rfc3339()])?;

        Ok(())
    }

    // --- ChatGPT Sessions ---

    pub fn save_session(&self, session: &ChatGptSession) -> Result<()> {
        let lock = self.conn.lock().unwrap();
        let user_json = serde_json::to_string(&session.user).unwrap_or_default();

        lock.prepare_cached(
            "INSERT INTO sessions (
                session_id, user_id, user_json, access_token, refresh_token,
                scope, expires_at, created_at, delegated_compute_active
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(session_id) DO UPDATE SET
                user_id = excluded.user_id,
                user_json = excluded.user_json,
                access_token = excluded.access_token,
                refresh_token = excluded.refresh_token,
                scope = excluded.scope,
                expires_at = excluded.expires_at,
                delegated_compute_active = excluded.delegated_compute_active;",
        )?
        .execute(params![
            session.session_id,
            session.user.id,
            user_json,
            session.access_token,
            session.refresh_token,
            session.scope,
            session.expires_at.to_rfc3339(),
            session.created_at.to_rfc3339(),
            if session.delegated_compute_active { 1 } else { 0 },
        ])?;

        Ok(())
    }

    pub fn get_session(&self, token_or_id: &str) -> Option<ChatGptSession> {
        let clean = token_or_id.trim_start_matches("Bearer ").trim();
        let lock = self.conn.lock().unwrap();

        let mut stmt = lock.prepare_cached(
            "SELECT session_id, user_json, access_token, refresh_token,
                    scope, expires_at, created_at, delegated_compute_active
             FROM sessions
             WHERE session_id = ?1 OR access_token = ?1
             LIMIT 1;",
        ).ok()?;

        let mut rows = stmt.query(params![clean]).ok()?;

        if let Ok(Some(row)) = rows.next() {
            let session_id: String = row.get(0).ok()?;
            let user_json: String = row.get(1).ok()?;
            let access_token: String = row.get(2).ok()?;
            let refresh_token: Option<String> = row.get(3).ok();
            let scope: String = row.get(4).ok()?;
            let expires_at_str: String = row.get(5).ok()?;
            let created_at_str: String = row.get(6).ok()?;
            let delegated_compute: i64 = row.get(7).ok()?;

            let user = serde_json::from_str(&user_json).ok()?;
            let expires_at = DateTime::parse_from_rfc3339(&expires_at_str).ok()?.with_timezone(&Utc);
            let created_at = DateTime::parse_from_rfc3339(&created_at_str).ok()?.with_timezone(&Utc);

            if expires_at > Utc::now() {
                return Some(ChatGptSession {
                    session_id,
                    user,
                    access_token,
                    refresh_token,
                    scope,
                    expires_at,
                    created_at,
                    delegated_compute_active: delegated_compute != 0,
                });
            }
        }

        None
    }

    pub fn remove_session(&self, token_or_id: &str) -> bool {
        let clean = token_or_id.trim_start_matches("Bearer ").trim();
        let lock = self.conn.lock().unwrap();

        let rows_affected = lock
            .prepare_cached("DELETE FROM sessions WHERE session_id = ?1 OR access_token = ?1;")
            .and_then(|mut stmt| stmt.execute(params![clean]))
            .unwrap_or(0);

        rows_affected > 0
    }

    // --- PKCE Pending Authorizations ---

    pub fn save_pending_auth(&self, auth: &PendingAuth) -> Result<()> {
        let lock = self.conn.lock().unwrap();

        lock.prepare_cached(
            "INSERT INTO pending_auths (state, code_verifier, redirect_uri, return_to, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(state) DO UPDATE SET
                code_verifier = excluded.code_verifier,
                redirect_uri = excluded.redirect_uri,
                return_to = excluded.return_to,
                created_at = excluded.created_at;",
        )?
        .execute(params![
            auth.state,
            auth.code_verifier,
            auth.redirect_uri,
            auth.return_to,
            auth.created_at.to_rfc3339(),
        ])?;

        Ok(())
    }

    /// Atomically retrieves and removes a pending PKCE authorization request in a single statement
    pub fn take_pending_auth(&self, state: &str) -> Option<PendingAuth> {
        let lock = self.conn.lock().unwrap();

        let mut stmt = lock.prepare_cached(
            "DELETE FROM pending_auths
             WHERE state = ?1
             RETURNING state, code_verifier, redirect_uri, return_to, created_at;",
        ).ok()?;

        let mut rows = stmt.query(params![state]).ok()?;

        if let Ok(Some(row)) = rows.next() {
            let state: String = row.get(0).ok()?;
            let code_verifier: String = row.get(1).ok()?;
            let redirect_uri: String = row.get(2).ok()?;
            let return_to: String = row.get(3).ok()?;
            let created_at_str: String = row.get(4).ok()?;

            let created_at = DateTime::parse_from_rfc3339(&created_at_str).ok()?.with_timezone(&Utc);

            return Some(PendingAuth {
                state,
                code_verifier,
                redirect_uri,
                return_to,
                created_at,
            });
        }

        None
    }

    // --- Stripe Webhook Idempotency ---

    pub fn is_stripe_event_processed(&self, event_id: &str) -> bool {
        let lock = self.conn.lock().unwrap();
        let mut stmt = match lock.prepare_cached("SELECT 1 FROM processed_stripe_events WHERE event_id = ?1 LIMIT 1;") {
            Ok(s) => s,
            Err(_) => return false,
        };

        stmt.exists(params![event_id]).unwrap_or(false)
    }

    pub fn record_stripe_event(&self, event_id: &str) -> Result<()> {
        let lock = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        lock.prepare_cached(
            "INSERT INTO processed_stripe_events (event_id, processed_at) VALUES (?1, ?2)
             ON CONFLICT(event_id) DO NOTHING;",
        )?
        .execute(params![event_id, now])?;

        Ok(())
    }

    pub fn record_customer_entitlement(
        &self,
        email: &str,
        plan: &str,
        stripe_session_id: Option<&str>,
        amount_cents: u64,
    ) -> Result<()> {
        let clean_email = email.trim().to_lowercase();
        let lock = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        lock.prepare_cached(
            "INSERT INTO customer_entitlements (email, plan, stripe_session_id, amount_cents, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(email) DO UPDATE SET
                plan = excluded.plan,
                stripe_session_id = COALESCE(excluded.stripe_session_id, customer_entitlements.stripe_session_id),
                amount_cents = excluded.amount_cents,
                updated_at = excluded.updated_at;",
        )?
        .execute(params![clean_email, plan, stripe_session_id, amount_cents as i64, now])?;

        Ok(())
    }

    pub fn get_customer_entitlement(&self, email: &str) -> Option<String> {
        let clean_email = email.trim().to_lowercase();
        let lock = self.conn.lock().unwrap();
        let mut stmt = lock
            .prepare_cached("SELECT plan FROM customer_entitlements WHERE email = ?1 LIMIT 1;")
            .ok()?;
        let mut rows = stmt.query(params![clean_email]).ok()?;
        if let Ok(Some(row)) = rows.next() {
            return row.get(0).ok();
        }
        None
    }

    pub fn revoke_customer_entitlement(&self, email: &str) -> Result<bool> {
        let clean_email = email.trim().to_lowercase();
        let lock = self.conn.lock().unwrap();
        let rows = lock
            .prepare_cached("DELETE FROM customer_entitlements WHERE email = ?1;")?
            .execute(params![clean_email])?;
        Ok(rows > 0)
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        let lock = self.conn.lock().unwrap();
        let sessions: i64 = lock
            .prepare_cached("SELECT count(*) FROM sessions;")
            .and_then(|mut s| s.query_row([], |r| r.get(0)))
            .unwrap_or(0);
        let stripe_events: i64 = lock
            .prepare_cached("SELECT count(*) FROM processed_stripe_events;")
            .and_then(|mut s| s.query_row([], |r| r.get(0)))
            .unwrap_or(0);
        let entitlements: i64 = lock
            .prepare_cached("SELECT count(*) FROM customer_entitlements;")
            .and_then(|mut s| s.query_row([], |r| r.get(0)))
            .unwrap_or(0);
        (sessions as usize, stripe_events as usize, entitlements as usize)
    }

    pub fn is_crawler_bot_ua(ua: &str) -> bool {
        let lower = ua.to_ascii_lowercase();
        const BOT_MARKERS: &[&str] = &[
            "web_seo_bot",
            "claudebot",
            "googlebot",
            "googleother",
            "gptbot",
            "oai-searchbot",
            "bingbot",
            "ahrefsbot",
            "semrushbot",
            "applebot",
            "perplexitybot",
            "bytespider",
            "petalbot",
            "yandexbot",
            "baiduspider",
            "duckduckbot",
            "facebookexternalhit",
            "twitterbot",
            "linkedinbot",
            "slackbot",
            "discordbot",
            "amazonbot",
            "mj12bot",
            "dotbot",
            "crawler",
            "spider",
            "mcpbeat",
            "faceabot",
            "agentrankprobe",
            "everymcp",
            "mcphub-probe",
            "catalog-probe",
            "smallprint-probe",
            "teppi-probe",
            "agent-collector",
            "mcp-health",
            "registry-survey",
            "sarnai.dev/bot",
            "verifymcp",
            "histor/",
            "mcptry.com",
            "golemreach",
            "squatchmcp",
            "protogrid-probe",
            "moonglade/",
            "mcpwatch/",
            "ziwei",
            "heldfast",
            "legion-tlog",
            "agentprobe/",
            "mcp2-research",
            "sealkeeper",
            "reliability-bureau",
            "agenttoolindex",
            "proofbench/",
            "toolvetprobe",
            "invokerankbot",
            "taifoonharvester",
            "priviscan/",
            "anlyon.dev-indexer",
            "mcp-rugpull-research",
            "dss-mcp-survey",
            "mcpgw-indexer",
            "sniffmcp",
            "alphaproxy-mcp-probe",
            "aptlynx",
            "brickbluebot",
            "agent-evidence-scanner",
            "agent-index-prober",
            "agent-tools.cloud",
            "atpp-scan",
            "flame-moisson",
            "mcp-inventory",
            "mcpcost/",
            "hultra-link",
            "mcplayer-gateway",
            "censysinspect",
            "blackall-allmcp",
            "mcp-harbor-probe",
            "botsrated-mcp-probe",
            "canmyaiuse-healthcheck",
            "endpointaudit/",
            "mcpstationbot",
            "pluginboardbot",
            "agent-world-probe",
            "mcp-observatory",
            "referencesource-mcp",
            "stackduke-mcp-scanner",
            "7maps-bot",
            "agora-probe",
            "capkeybot",
            "mcpcensus",
            "annalistchecks",
            "exorails-gateway",
            "mcp-stats-prober",
            "mcpmeter/",
            "proofhousebaseline",
            "wellknownbot",
            "agentalog-sonda",
            "creora-mcp-registry",
            "mcp-history/",
            "mcp-registry-census",
            "mcpqueen-grader",
            "rootz-mcp-registry",
            "safemcp-research",
            "warden-survey",
            "agentcommonsdoctor",
            "agentindexbot",
            "builtwith-mcpregistry",
            "gaip-observatory",
            "roninforgeobservatory",
            "allagents-verify",
            "catalog-health/",
            "lastseen-schema-probe",
            "mcpwell.com",
            "musedirectory.ai",
            "spiron-mcp",
            "aive-mcp",
            "apeye/",
            "glidemcpindex",
            "inite-atlas",
            "pulsefeed",
            "thewhiterock-registrycheck",
            "addrbook-probe",
            "ai-pulse-mcp-probe",
            "free-remote-mcp-servers",
            "mcp-checker/",
            "mcp-scraper/",
            "mcpforcopilot-probe",
            "capabilityindexbot",
            "mcpstatus/",
            "modernmustardseed",
            "agentid-scout",
            "rokha-probe",
            "eda-claw-runtime-probe",
            "mcp-server.io-healthcheck",
            "mcpserver-lol-probe",
            "tanod-referee",
            "anchorterminalbot",
            "agent-readiness-check",
            "aisec-registry",
            "awesome-remote-mcp-servers-ci",
            "mcp-probe/",
            "tokenrip",
            "402explorer",
            "factanker",
            "talaria",
            "7maps",
            "spanly",
            "mcp-cloud",
            "pryno",
            "mcpcentral",
            "agentservicedirectory",
            "ariesview",
            "agentalmanac",
            "kell-tool-router",
            "lowdown-scanner",
            "mcpmon",
            "mcp-discovery-measurement",
            "whoelse-probe",
            "sarnai",
            "telosnex",
            "stack-install-probe",
            "tacuri",
            "aisonar",
            "aiworkforce",
            "aven-mcp",
            "cre-brain",
            "deusintus",
            "electraindex",
            "lazaretto",
            "leash-index",
            "lurq-probe",
            "magazin-ai",
            "mcp-checked",
            "mcplookup",
            "mcprush",
            "mcpsentinel",
            "patentmax",
            "pawnly",
            "plug.directory",
            "syndai",
            "ventru",
            "agenttested",
            "agentforum",
            "assay",
        ];
        BOT_MARKERS.iter().any(|m| lower.contains(m))
    }

    pub fn is_internal_probe_ua(ua: &str) -> bool {
        let lower = ua.to_ascii_lowercase();
        const PROBE_MARKERS: &[&str] = &[
            "web_seo_probe",
            "stripe_probe",
            "productdoctor",
            "product-doctor",
            "hallmanlab",
            "seo-audit",
            "seo_audit",
            "pressmark-smoke",
            "pressmark-measurement-qa/",
            "hallman-lab-ops-heartbeats",
        ];
        PROBE_MARKERS.iter().any(|m| lower.contains(m))
    }

    pub fn classify_usage_actor(channel: &str, detail: Option<&str>) -> &'static str {
        if Self::is_internal_probe_ua(channel)
            || detail.map(Self::is_internal_probe_ua).unwrap_or(false)
        {
            "internal_probe"
        } else if Self::is_crawler_bot_ua(channel)
            || detail.map(Self::is_crawler_bot_ua).unwrap_or(false)
        {
            "crawler_bot"
        } else {
            "unclassified"
        }
    }

    /// Atomically record an explicit checkout action once. Only a request hash is retained.
    pub fn record_checkout_start(
        &self,
        request_id: &str,
        request_fingerprint: &str,
        plan_id: &str,
        channel: &str,
    ) -> Result<CheckoutStart> {
        let mut lock = self.conn.lock().unwrap();
        let transaction = lock.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let existing: Option<String> = transaction
            .prepare_cached(
                "SELECT request_fingerprint FROM checkout_starts WHERE request_id = ?1",
            )?
            .query_row([request_id], |row| row.get(0))
            .optional()?;
        if let Some(existing) = existing {
            return Ok(if existing == request_fingerprint {
                CheckoutStart::Replayed
            } else {
                CheckoutStart::Conflict
            });
        }
        let now = Utc::now().to_rfc3339();
        transaction
            .prepare_cached(
                "INSERT INTO checkout_starts (request_id, request_fingerprint, created_at) VALUES (?1, ?2, ?3)",
            )?
            .execute(params![request_id, request_fingerprint, now])?;
        transaction
            .prepare_cached(
                "INSERT INTO usage_events (event_type, channel, detail, created_at) VALUES ('checkout_init', ?1, ?2, ?3)",
            )?
            .execute(params![channel, plan_id, now])?;
        transaction.commit()?;
        Ok(CheckoutStart::Created)
    }

    pub fn record_usage_event(
        &self,
        event_type: &str,
        archetype: Option<&str>,
        channel: &str,
        detail: Option<&str>,
        compile_ms: Option<u64>,
    ) {
        let now = Utc::now().to_rfc3339();
        let normalized_archetype = archetype.map(|a| {
            crate::models::DocumentType::from_slug(a)
                .map(|dt| dt.as_str().to_string())
                .or_else(|| {
                    crate::models::DocumentType::parse_state_prefixed_slug(a)
                        .map(|(_, dt)| dt.as_str().to_string())
                })
                .unwrap_or_else(|| a.trim().to_lowercase().replace('-', "_"))
        });
        if let Ok(lock) = self.conn.lock() {
            if let Ok(mut stmt) = lock.prepare_cached(
                "INSERT INTO usage_events (event_type, archetype, channel, detail, compile_ms, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
            ) {
                let _ = stmt.execute(params![
                    event_type,
                    normalized_archetype.as_deref(),
                    channel,
                    detail,
                    compile_ms.map(|v| v as i64),
                    now
                ]);
            }
        }
    }

    /// Persist an attempt before execution; interrupted work stays pending, never successful.
    pub fn begin_mcp_tool_attempt(&self, archetype: Option<&str>, channel: &str, tool: Option<&str>) -> Option<i64> {
        let tool = crate::mcp::McpHandler::measurement_tool_name(tool);
        let result = self.conn.lock().map_err(|_| "storage lock poisoned".to_string()).and_then(|lock| {
            lock.prepare_cached(
                "INSERT INTO usage_events (event_type, archetype, channel, detail, created_at, outcome) VALUES ('mcp_tool_call', ?1, ?2, ?3, ?4, 'pending')",
            )
            .and_then(|mut s| s.execute(params![archetype, channel, tool, Utc::now().to_rfc3339()]))
            .map_err(|error| error.to_string())?;
            Ok(lock.last_insert_rowid())
        });
        match result {
            Ok(id) => Some(id),
            Err(error) => {
                self.mcp_attempt_write_failures.fetch_add(1, Ordering::Relaxed);
                tracing::warn!("MCP attempt measurement write failed: {}", error);
                None
            }
        }
    }

    pub fn finish_mcp_tool_attempt(&self, id: i64, success: bool, error_code: Option<i64>, compile_ms: Option<u64>) {
        let result = self.conn.lock().map_err(|_| "storage lock poisoned".to_string()).and_then(|lock| {
            lock.prepare_cached(
                "UPDATE usage_events SET outcome = ?1, completed_at = ?2, error_code = ?3, compile_ms = ?4 WHERE id = ?5 AND event_type = 'mcp_tool_call' AND outcome = 'pending'",
            )
            .and_then(|mut s| s.execute(params![if success { "success" } else { "failure" }, Utc::now().to_rfc3339(), error_code, compile_ms.map(|v| v as i64), id]))
            .map_err(|error| error.to_string())
        });
        if !matches!(result, Ok(1)) {
            self.mcp_outcome_write_failures.fetch_add(1, Ordering::Relaxed);
            tracing::warn!("MCP outcome measurement write failed; the attempt remains unresolved");
        }
    }

    pub fn analytics_summary(&self) -> serde_json::Value {
        let lock = self.conn.lock().unwrap();
        let total_events: i64 = lock
            .prepare_cached("SELECT count(*) FROM usage_events;")
            .and_then(|mut s| s.query_row([], |r| r.get(0)))
            .unwrap_or(0);

        let mut unclassified_events: i64 = 0;
        let mut crawler_bot_events: i64 = 0;
        let mut internal_probe_events: i64 = 0;
        let mut harness_client_events: i64 = 0;
        if let Ok(mut stmt) = lock.prepare_cached(
            "SELECT channel, detail, count(*) FROM usage_events GROUP BY channel, detail;",
        ) {
            if let Ok(mut rows) = stmt.query([]) {
                while let Ok(Some(row)) = rows.next() {
                    let ch: String = row.get(0).unwrap_or_default();
                    let det: Option<String> = row.get(1).ok();
                    let cnt: i64 = row.get(2).unwrap_or(0);
                    let ch_lower = ch.to_ascii_lowercase();
                    if ch_lower.contains("codex-mcp-client")
                        || ch_lower.contains("claude-code")
                        || ch_lower.contains("grok-cli")
                    {
                        harness_client_events += cnt;
                    }
                    match Self::classify_usage_actor(&ch, det.as_deref()) {
                        "crawler_bot" => crawler_bot_events += cnt,
                        "internal_probe" => internal_probe_events += cnt,
                        _ => unclassified_events += cnt,
                    }
                }
            }
        }

        let distinct_mcp_channels: i64 = lock
            .prepare_cached(
                "SELECT count(DISTINCT channel) FROM usage_events WHERE event_type IN ('mcp_protocol', 'mcp_tool_call');",
            )
            .and_then(|mut s| s.query_row([], |r| r.get(0)))
            .unwrap_or(0);

        let mut by_type = serde_json::Map::new();
        if let Ok(mut stmt) = lock.prepare_cached(
            "SELECT event_type, count(*) FROM usage_events GROUP BY event_type ORDER BY count(*) DESC;",
        ) {
            if let Ok(mut rows) = stmt.query([]) {
                while let Ok(Some(row)) = rows.next() {
                    if let (Ok(k), Ok(v)) = (row.get::<_, String>(0), row.get::<_, i64>(1)) {
                        by_type.insert(k, serde_json::json!(v));
                    }
                }
            }
        }

        let mut by_archetype = serde_json::Map::new();
        if let Ok(mut stmt) = lock.prepare_cached(
            "SELECT archetype, count(*) FROM usage_events WHERE archetype IS NOT NULL GROUP BY archetype ORDER BY count(*) DESC LIMIT 20;",
        ) {
            if let Ok(mut rows) = stmt.query([]) {
                while let Ok(Some(row)) = rows.next() {
                    if let (Ok(k), Ok(v)) = (row.get::<_, String>(0), row.get::<_, i64>(1)) {
                        by_archetype.insert(k, serde_json::json!(v));
                    }
                }
            }
        }

        let mut by_mcp_tool = serde_json::Map::new();
        if let Ok(mut stmt) = lock.prepare_cached(
            "SELECT detail, count(*) FROM usage_events WHERE event_type = 'mcp_tool_call' AND detail IS NOT NULL GROUP BY detail ORDER BY count(*) DESC;",
        ) {
            if let Ok(mut rows) = stmt.query([]) {
                while let Ok(Some(row)) = rows.next() {
                    if let (Ok(k), Ok(v)) = (row.get::<_, String>(0), row.get::<_, i64>(1)) {
                        let key = if k == "unnamed_tool" { k } else { crate::mcp::McpHandler::measurement_tool_name(Some(&k)) };
                        let previous = by_mcp_tool.get(&key).and_then(serde_json::Value::as_i64).unwrap_or(0);
                        by_mcp_tool.insert(key, serde_json::json!(previous + v));
                    }
                }
            }
        }

        let mut mcp_outcomes = serde_json::json!({"success": 0, "failure": 0, "pending": 0, "unknown": 0});
        let mut mcp_failure_breakdown = serde_json::json!({"unknown_tool": 0, "invalid_arguments": 0, "execution_error": 0, "unclassified": 0});
        let mut mcp_by_tool = serde_json::Map::new();
        let mut mcp_by_actor = serde_json::Map::new();
        let mut mcp_attempts = 0_i64;
        let mut mcp_query_ok = false;
        if let Ok(mut stmt) = lock.prepare_cached(
            "SELECT COALESCE(detail, '(unnamed)'), channel, COALESCE(outcome, 'unknown'), error_code, count(*) FROM usage_events WHERE event_type = 'mcp_tool_call' GROUP BY detail, channel, outcome, error_code;",
        ) {
            if let Ok(mut rows) = stmt.query([]) {
                mcp_query_ok = true;
                loop {
                    match rows.next() {
                        Ok(Some(row)) => {
                            let values = (row.get::<_, String>(0), row.get::<_, String>(1), row.get::<_, String>(2), row.get::<_, Option<i64>>(3), row.get::<_, i64>(4));
                            let (Ok(tool), Ok(channel), Ok(raw_outcome), Ok(error_code), Ok(count)) = values else { mcp_query_ok = false; break; };
                            let outcome = match raw_outcome.as_str() { "success" => "success", "failure" => "failure", "pending" => "pending", _ => "unknown" };
                            mcp_attempts += count;
                            mcp_outcomes[outcome] = serde_json::json!(mcp_outcomes[outcome].as_i64().unwrap_or(0) + count);
                            if outcome == "failure" {
                                let reason = match error_code {
                                    Some(-32601) => "unknown_tool",
                                    Some(-32602) => "invalid_arguments",
                                    Some(-32000 | -32603) => "execution_error",
                                    _ => "unclassified",
                                };
                                mcp_failure_breakdown[reason] = serde_json::json!(mcp_failure_breakdown[reason].as_i64().unwrap_or(0) + count);
                            }
                            let tool = if tool == "unnamed_tool" || tool == "(unnamed)" { "unnamed_tool".to_string() } else { crate::mcp::McpHandler::measurement_tool_name(Some(&tool)) };
                            for (map, key) in [(&mut mcp_by_tool, tool), (&mut mcp_by_actor, Self::classify_usage_actor(&channel, None).to_string())] {
                                let entry = map.entry(key).or_insert_with(|| serde_json::json!({"attempts": 0, "success": 0, "failure": 0, "pending": 0, "unknown": 0}));
                                entry["attempts"] = serde_json::json!(entry["attempts"].as_i64().unwrap_or(0) + count);
                                entry[outcome] = serde_json::json!(entry[outcome].as_i64().unwrap_or(0) + count);
                            }
                        }
                        Ok(None) => break,
                        Err(_) => { mcp_query_ok = false; break; }
                    }
                }
            }
        }
        let retained_window: Option<(Option<String>, Option<String>)> = lock
            .prepare_cached("SELECT MIN(created_at), MAX(created_at) FROM usage_events")
            .and_then(|mut s| s.query_row([], |row| Ok((row.get(0)?, row.get(1)?))))
            .ok();
        let outcomes_started_at: Option<String> = lock
            .prepare_cached(
                "SELECT value FROM usage_measurement_metadata WHERE key = 'mcp_outcomes_started_at'",
            )
            .and_then(|mut s| s.query_row([], |row| row.get(0)))
            .ok();

        let avg_compile_ms: f64 = lock
            .prepare_cached(
                "SELECT COALESCE(AVG(compile_ms), 0.0) FROM usage_events WHERE compile_ms IS NOT NULL;",
            )
            .and_then(|mut s| s.query_row([], |r| r.get(0)))
            .unwrap_or(0.0);

        let checkout_counts: Option<(i64, i64)> = lock
            .prepare_cached(
                "SELECT (SELECT count(*) FROM checkout_starts), (SELECT count(*) FROM usage_events WHERE event_type = 'checkout_init');",
            )
            .and_then(|mut statement| statement.query_row([], |row| Ok((row.get(0)?, row.get(1)?))))
            .ok();

        let (sessions_count, stripe_events_count, entitlements_count, paid_entitlements_count) = {
            let s: i64 = lock
                .prepare_cached("SELECT count(*) FROM sessions;")
                .and_then(|mut st| st.query_row([], |r| r.get(0)))
                .unwrap_or(0);
            let st: i64 = lock
                .prepare_cached("SELECT count(*) FROM processed_stripe_events;")
                .and_then(|mut st| st.query_row([], |r| r.get(0)))
                .unwrap_or(0);
            let e: i64 = lock
                .prepare_cached("SELECT count(*) FROM customer_entitlements;")
                .and_then(|mut st| st.query_row([], |r| r.get(0)))
                .unwrap_or(0);
            let paid: i64 = lock
                .prepare_cached("SELECT count(*) FROM customer_entitlements WHERE amount_cents > 0;")
                .and_then(|mut st| st.query_row([], |r| r.get(0)))
                .unwrap_or(0);
            (s, st, e, paid)
        };

        serde_json::json!({
            "total_usage_events": total_events,
            "measurement_schema_version": 2,
            "unclassified_events": unclassified_events,
            "crawler_bot_events": crawler_bot_events,
            "internal_probe_events": internal_probe_events,
            "harness_client_events": harness_client_events,
            "distinct_mcp_channels": distinct_mcp_channels,
            "actor_breakdown": {
                "unclassified": unclassified_events,
                "crawler_bot": crawler_bot_events,
                "internal_probe": internal_probe_events
            },
            "attribution": {
                "basis": "User-agent and named probe hints; caller supplied, not verified identities",
                "unique_users": null,
                "customer_count": null,
                "note": "Unclassified includes generic command-line, browser and agent clients. Harness client events overlap actor categories. Distinct MCP channels count client strings, not identities. Neither metric identifies customers."
            },
            "retained_usage_window": {
                "first_event_at": retained_window.as_ref().and_then(|window| window.0.as_ref()),
                "last_event_at": retained_window.as_ref().and_then(|window| window.1.as_ref()),
                "query_scope": "all_retained_events",
                "retention_days": 90,
                "durable_storage": self.db_path.to_str() != Some(":memory:"),
                "full_window_coverage_established": false
            },
            "mcp_measurement": {
                "query_complete": mcp_query_ok,
                "outcomes_started_at": outcomes_started_at,
                "attempts": if mcp_query_ok { serde_json::json!(mcp_attempts) } else { serde_json::Value::Null },
                "outcomes": if mcp_query_ok { mcp_outcomes } else { serde_json::Value::Null },
                "failure_breakdown": if mcp_query_ok { mcp_failure_breakdown } else { serde_json::Value::Null },
                "by_tool": if mcp_query_ok { serde_json::json!(mcp_by_tool) } else { serde_json::Value::Null },
                "by_actor": if mcp_query_ok { serde_json::json!(mcp_by_actor) } else { serde_json::Value::Null },
                "attempt_write_failures_since_process_start": self.mcp_attempt_write_failures.load(Ordering::Relaxed),
                "outcome_write_failures_since_process_start": self.mcp_outcome_write_failures.load(Ordering::Relaxed),
                "definitions": "Attempts are parsed tools/call requests reaching the handler. Success is a tool result without a JSON-RPC or tool error. Pending means completion was not recorded. Historical outcomes are unknown. Failure reasons use recorded error codes: unknown_tool (-32601), invalid_arguments (-32602), execution_error (-32000 or -32603), otherwise unclassified. Rejected requests are not execution errors. Protocol discovery and malformed HTTP/JSON requests are excluded. Counts do not establish users, customers, payment or delivery."
            },
            "checkout_measurement": {
                "query_complete": checkout_counts.is_some(),
                "recorded_starts": checkout_counts.map(|counts| counts.0),
                "initialization_events": checkout_counts.map(|counts| counts.1),
                "definitions": "Recorded starts count deduplicated explicit checkout actions in checkout_starts. Initialization events count retained checkout_init events, including older automatic requests and probes; they do not establish explicit starts. Recorded starts include test actions and are not unique people, completed purchases or payments. The start table and retained event table have different retention scopes."
            },
            "events_by_type": by_type,
            "top_archetypes": by_archetype,
            "mcp_tool_calls": by_mcp_tool,
            "avg_compile_ms": (avg_compile_ms * 10.0).round() / 10.0,
            "sessions_count": sessions_count,
            "stripe_events_count": stripe_events_count,
            "entitlements_count": entitlements_count,
            "paid_entitlements_count": paid_entitlements_count
        })
    }

    /// Prune expired sessions, stale pending auths (>1h), old Stripe events (>N days), and usage_events (>90 days)
    pub fn prune_stale_data(&self, max_stripe_event_age_days: u32) -> Result<usize> {
        let lock = self.conn.lock().unwrap();
        let now = Utc::now();
        let now_rfc3339 = now.to_rfc3339();
        let one_hour_ago = (now - chrono::Duration::hours(1)).to_rfc3339();
        let stripe_cutoff = (now - chrono::Duration::days(max_stripe_event_age_days as i64)).to_rfc3339();
        let usage_cutoff = (now - chrono::Duration::days(90)).to_rfc3339();

        let expired_sessions = lock
            .prepare_cached("DELETE FROM sessions WHERE expires_at < ?1;")?
            .execute(params![now_rfc3339])?;

        let stale_auths = lock
            .prepare_cached("DELETE FROM pending_auths WHERE created_at < ?1;")?
            .execute(params![one_hour_ago])?;

        let old_events = lock
            .prepare_cached("DELETE FROM processed_stripe_events WHERE processed_at < ?1;")?
            .execute(params![stripe_cutoff])?;

        let old_usage = lock
            .prepare_cached("DELETE FROM usage_events WHERE created_at < ?1;")?
            .execute(params![usage_cutoff])?;

        let total = expired_sessions + stale_auths + old_events + old_usage;
        if total > 0 {
            tracing::info!(
                "Pruned stale DB records: {} sessions, {} auths, {} stripe events, {} usage events",
                expired_sessions, stale_auths, old_events, old_usage
            );
        }
        Ok(total)
    }
}

/// Spawns a background pruning worker that runs every `interval_secs` seconds
pub fn start_db_pruner_task(storage: Arc<Storage>, interval_secs: u64, max_stripe_event_age_days: u32) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(interval_secs));
        loop {
            interval.tick().await;
            let s = Arc::clone(&storage);
            let _ = tokio::task::spawn_blocking(move || {
                s.prune_stale_data(max_stripe_event_age_days)
            }).await;
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_covering_indexes_query_plan() {
        let storage = Storage::new(":memory:").expect("in-memory storage");
        let lock = storage.conn.lock().unwrap();

        let mut stmt_channel = lock
            .prepare_cached(
                "EXPLAIN QUERY PLAN SELECT channel, detail, count(*) FROM usage_events GROUP BY channel, detail;",
            )
            .expect("prepare_cached explain channel");
        let channel_plan: Vec<String> = stmt_channel
            .query_map([], |row| row.get::<_, String>(3))
            .expect("query_map")
            .filter_map(|r| r.ok())
            .collect();
        assert!(
            channel_plan
                .iter()
                .any(|d| d.contains("idx_usage_events_channel_detail")),
            "expected idx_usage_events_channel_detail in plan: {:?}",
            channel_plan
        );

        let mut stmt_mcp = lock
            .prepare_cached(
                "EXPLAIN QUERY PLAN SELECT detail, count(*) FROM usage_events WHERE event_type = ?1 AND detail IS NOT NULL GROUP BY detail;",
            )
            .expect("prepare_cached explain mcp");
        let mcp_plan: Vec<String> = stmt_mcp
            .query_map(["mcp_tool_call"], |row| row.get::<_, String>(3))
            .expect("query_map")
            .filter_map(|r| r.ok())
            .collect();
        assert!(
            mcp_plan
                .iter()
                .any(|d| d.contains("idx_usage_events_type_detail")),
            "expected idx_usage_events_type_detail in plan: {:?}",
            mcp_plan
        );

        let mut stmt_arch = lock
            .prepare_cached(
                "EXPLAIN QUERY PLAN SELECT archetype, count(*) FROM usage_events WHERE archetype IS NOT NULL AND archetype != '' GROUP BY archetype ORDER BY count(*) DESC LIMIT 20;",
            )
            .expect("prepare_cached explain archetype");
        let arch_plan: Vec<String> = stmt_arch
            .query_map([], |row| row.get::<_, String>(3))
            .expect("query_map")
            .filter_map(|r| r.ok())
            .collect();
        assert!(
            arch_plan
                .iter()
                .any(|d| d.contains("idx_usage_events_archetype")),
            "expected idx_usage_events_archetype in plan: {:?}",
            arch_plan
        );
    }
}
