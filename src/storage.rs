//! Pressmark Persistent Storage Engine (SQLite)
//!
//! Author: Daniel Hallman
//!
//! Provides durable, restart-safe persistence for:
//! 1. ChatGPT OAuth user sessions and subscription tokens
//! 2. PKCE pending authorizations
//! 3. Stripe webhook idempotency event tracking

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::auth::{ChatGptSession, PendingAuth};

#[derive(Clone)]
pub struct Storage {
    conn: Arc<Mutex<Connection>>,
    db_path: PathBuf,
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
            CREATE INDEX IF NOT EXISTS idx_usage_events_created ON usage_events(created_at);
            CREATE INDEX IF NOT EXISTS idx_usage_events_created_at ON usage_events(created_at);
            UPDATE usage_events SET archetype = REPLACE(archetype, '-', '_') WHERE archetype IS NOT NULL AND archetype LIKE '%-%';
            UPDATE usage_events SET channel = 'web_seo_probe' WHERE event_type = 'template_landing_view' AND channel = 'web_seo' AND substr(created_at, 1, 19) IN (SELECT DISTINCT substr(created_at, 1, 19) FROM usage_events WHERE detail LIKE '%ProductDoctor%' OR channel LIKE '%ProductDoctor%');
            UPDATE usage_events SET channel = 'stripe_probe' WHERE event_type = 'checkout_init' AND channel = 'stripe' AND substr(created_at, 1, 19) IN (SELECT DISTINCT substr(created_at, 1, 19) FROM usage_events WHERE detail LIKE '%ProductDoctor%' OR channel LIKE '%ProductDoctor%');",
        )?;

        Ok(())
    }

    // --- ChatGPT Sessions ---

    pub fn save_session(&self, session: &ChatGptSession) -> Result<()> {
        let lock = self.conn.lock().unwrap();
        let user_json = serde_json::to_string(&session.user).unwrap_or_default();

        lock.execute(
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
            params![
                session.session_id,
                session.user.id,
                user_json,
                session.access_token,
                session.refresh_token,
                session.scope,
                session.expires_at.to_rfc3339(),
                session.created_at.to_rfc3339(),
                if session.delegated_compute_active { 1 } else { 0 },
            ],
        )?;

        Ok(())
    }

    pub fn get_session(&self, token_or_id: &str) -> Option<ChatGptSession> {
        let clean = token_or_id.trim_start_matches("Bearer ").trim();
        let lock = self.conn.lock().unwrap();

        let mut stmt = lock.prepare(
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

        let rows_affected = lock.execute(
            "DELETE FROM sessions WHERE session_id = ?1 OR access_token = ?1;",
            params![clean],
        ).unwrap_or(0);

        rows_affected > 0
    }

    // --- PKCE Pending Authorizations ---

    pub fn save_pending_auth(&self, auth: &PendingAuth) -> Result<()> {
        let lock = self.conn.lock().unwrap();

        lock.execute(
            "INSERT INTO pending_auths (state, code_verifier, redirect_uri, return_to, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(state) DO UPDATE SET
                code_verifier = excluded.code_verifier,
                redirect_uri = excluded.redirect_uri,
                return_to = excluded.return_to,
                created_at = excluded.created_at;",
            params![
                auth.state,
                auth.code_verifier,
                auth.redirect_uri,
                auth.return_to,
                auth.created_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    /// Atomically retrieves and removes a pending PKCE authorization request in a single statement
    pub fn take_pending_auth(&self, state: &str) -> Option<PendingAuth> {
        let lock = self.conn.lock().unwrap();

        let mut stmt = lock.prepare(
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
        let mut stmt = match lock.prepare("SELECT 1 FROM processed_stripe_events WHERE event_id = ?1 LIMIT 1;") {
            Ok(s) => s,
            Err(_) => return false,
        };

        let exists = stmt.exists(params![event_id]).unwrap_or(false);
        exists
    }

    pub fn record_stripe_event(&self, event_id: &str) -> Result<()> {
        let lock = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        lock.execute(
            "INSERT INTO processed_stripe_events (event_id, processed_at) VALUES (?1, ?2)
             ON CONFLICT(event_id) DO NOTHING;",
            params![event_id, now],
        )?;

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

        lock.execute(
            "INSERT INTO customer_entitlements (email, plan, stripe_session_id, amount_cents, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(email) DO UPDATE SET
                plan = excluded.plan,
                stripe_session_id = COALESCE(excluded.stripe_session_id, customer_entitlements.stripe_session_id),
                amount_cents = excluded.amount_cents,
                updated_at = excluded.updated_at;",
            params![clean_email, plan, stripe_session_id, amount_cents as i64, now],
        )?;

        Ok(())
    }

    pub fn get_customer_entitlement(&self, email: &str) -> Option<String> {
        let clean_email = email.trim().to_lowercase();
        let lock = self.conn.lock().unwrap();
        let mut stmt = lock
            .prepare("SELECT plan FROM customer_entitlements WHERE email = ?1 LIMIT 1;")
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
        let rows = lock.execute(
            "DELETE FROM customer_entitlements WHERE email = ?1;",
            params![clean_email],
        )?;
        Ok(rows > 0)
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        let lock = self.conn.lock().unwrap();
        let sessions: i64 = lock
            .query_row("SELECT count(*) FROM sessions;", [], |r| r.get(0))
            .unwrap_or(0);
        let stripe_events: i64 = lock
            .query_row("SELECT count(*) FROM processed_stripe_events;", [], |r| r.get(0))
            .unwrap_or(0);
        let entitlements: i64 = lock
            .query_row("SELECT count(*) FROM customer_entitlements;", [], |r| r.get(0))
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
            "verifymcp_auth_probe",
            "io.verifymcp",
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
            "aive-mcp-discover",
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
            "xh/",
            "curl/",
            "pressmark-smoke",
            "hallman-lab-ops-heartbeats",
            "uptime",
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
            "external_organic"
        }
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
            let _ = lock.execute(
                "INSERT INTO usage_events (event_type, archetype, channel, detail, compile_ms, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
                params![
                    event_type,
                    normalized_archetype.as_deref(),
                    channel,
                    detail,
                    compile_ms.map(|v| v as i64),
                    now
                ],
            );
        }
    }

    pub fn analytics_summary(&self) -> serde_json::Value {
        let lock = self.conn.lock().unwrap();
        let total_events: i64 = lock
            .query_row("SELECT count(*) FROM usage_events;", [], |r| r.get(0))
            .unwrap_or(0);

        let mut external_organic_events: i64 = 0;
        let mut crawler_bot_events: i64 = 0;
        let mut internal_probe_events: i64 = 0;
        let mut harness_client_events: i64 = 0;
        if let Ok(mut stmt) = lock.prepare(
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
                        _ => external_organic_events += cnt,
                    }
                }
            }
        }

        let distinct_mcp_channels: i64 = lock
            .query_row(
                "SELECT count(DISTINCT channel) FROM usage_events WHERE channel LIKE 'mcp%';",
                [],
                |r| r.get(0),
            )
            .unwrap_or(0);

        let mut by_type = serde_json::Map::new();
        if let Ok(mut stmt) = lock.prepare(
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
        if let Ok(mut stmt) = lock.prepare(
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
        if let Ok(mut stmt) = lock.prepare(
            "SELECT detail, count(*) FROM usage_events WHERE event_type = 'mcp_tool_call' AND detail IS NOT NULL GROUP BY detail ORDER BY count(*) DESC;",
        ) {
            if let Ok(mut rows) = stmt.query([]) {
                while let Ok(Some(row)) = rows.next() {
                    if let (Ok(k), Ok(v)) = (row.get::<_, String>(0), row.get::<_, i64>(1)) {
                        by_mcp_tool.insert(k, serde_json::json!(v));
                    }
                }
            }
        }

        let avg_compile_ms: f64 = lock
            .query_row(
                "SELECT COALESCE(AVG(compile_ms), 0.0) FROM usage_events WHERE compile_ms IS NOT NULL;",
                [],
                |r| r.get(0),
            )
            .unwrap_or(0.0);

        let (sessions_count, stripe_events_count, entitlements_count) = {
            let s: i64 = lock.query_row("SELECT count(*) FROM sessions;", [], |r| r.get(0)).unwrap_or(0);
            let st: i64 = lock.query_row("SELECT count(*) FROM processed_stripe_events;", [], |r| r.get(0)).unwrap_or(0);
            let e: i64 = lock.query_row("SELECT count(*) FROM customer_entitlements;", [], |r| r.get(0)).unwrap_or(0);
            (s, st, e)
        };

        serde_json::json!({
            "total_usage_events": total_events,
            "external_organic_events": external_organic_events,
            "crawler_bot_events": crawler_bot_events,
            "internal_probe_events": internal_probe_events,
            "harness_client_events": harness_client_events,
            "distinct_mcp_channels": distinct_mcp_channels,
            "actor_breakdown": {
                "external_organic": external_organic_events,
                "crawler_bot": crawler_bot_events,
                "internal_probe": internal_probe_events,
                "harness_client": harness_client_events
            },
            "events_by_type": by_type,
            "top_archetypes": by_archetype,
            "mcp_tool_calls": by_mcp_tool,
            "avg_compile_ms": (avg_compile_ms * 10.0).round() / 10.0,
            "sessions_count": sessions_count,
            "stripe_events_count": stripe_events_count,
            "paid_entitlements_count": entitlements_count
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

        let expired_sessions = lock.execute(
            "DELETE FROM sessions WHERE expires_at < ?1;",
            params![now_rfc3339],
        )?;

        let stale_auths = lock.execute(
            "DELETE FROM pending_auths WHERE created_at < ?1;",
            params![one_hour_ago],
        )?;

        let old_events = lock.execute(
            "DELETE FROM processed_stripe_events WHERE processed_at < ?1;",
            params![stripe_cutoff],
        )?;

        let old_usage = lock.execute(
            "DELETE FROM usage_events WHERE created_at < ?1;",
            params![usage_cutoff],
        )?;

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
