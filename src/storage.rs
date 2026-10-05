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
            CREATE INDEX IF NOT EXISTS idx_processed_stripe_at ON processed_stripe_events(processed_at);",
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

    /// Prune expired sessions, stale pending auths (>1h), and old Stripe events (>N days)
    pub fn prune_stale_data(&self, max_stripe_event_age_days: u32) -> Result<usize> {
        let lock = self.conn.lock().unwrap();
        let now = Utc::now();
        let now_rfc3339 = now.to_rfc3339();
        let one_hour_ago = (now - chrono::Duration::hours(1)).to_rfc3339();
        let stripe_cutoff = (now - chrono::Duration::days(max_stripe_event_age_days as i64)).to_rfc3339();

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

        let total = expired_sessions + stale_auths + old_events;
        if total > 0 {
            tracing::info!(
                "Pruned stale DB records: {} sessions, {} auths, {} stripe events",
                expired_sessions, stale_auths, old_events
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
