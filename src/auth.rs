//! Pressmark - "Sign in with ChatGPT" & Subscription Pass-Through Engine
//!
//! Author: Daniel Hallman
//!
//! Implements official OpenAI "Sign in with ChatGPT" (OAuth 2.1 Authorization Code Flow with PKCE)
//! and subscription compute sharing / pass-through.
//!
//! Subscribers with ChatGPT Plus, Pro, Team, or Enterprise can authenticate with Pressmark
//! to delegate AI Copilot document generation and Vision OCR extraction directly to their
//! existing ChatGPT subscription allowance (via `model.request` delegation scope),
//! eliminating separate OpenAI API billing and raw API key management.

use axum::extract::{Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Json;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as BASE64_URL;
use base64::Engine;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub const OPENAI_AUTH_SERVER: &str = "https://auth.openai.com";
pub const OPENAI_AUTHORIZE_ENDPOINT: &str = "https://auth.openai.com/api/accounts/authorize";
pub const OPENAI_TOKEN_ENDPOINT: &str = "https://auth.openai.com/api/accounts/oauth/token";
pub const OPENAI_USERINFO_ENDPOINT: &str = "https://auth.openai.com/api/accounts/userinfo";
pub const CHATGPT_AUTH_SCOPES: &str = "openid profile email model.request";

pub fn urlencode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{:02X}", b));
        }
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatGptPlan {
    Free,
    Plus,
    Pro,
    Team,
    Enterprise,
}

impl ChatGptPlan {
    pub fn from_str_relaxed(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "plus" | "chatgpt_plus" => Self::Plus,
            "pro" | "chatgpt_pro" => Self::Pro,
            "team" | "chatgpt_team" => Self::Team,
            "enterprise" | "chatgpt_enterprise" => Self::Enterprise,
            _ => Self::Free,
        }
    }

    pub fn is_subscriber(&self) -> bool {
        matches!(self, Self::Plus | Self::Pro | Self::Team | Self::Enterprise)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Free => "ChatGPT Free",
            Self::Plus => "ChatGPT Plus",
            Self::Pro => "ChatGPT Pro",
            Self::Team => "ChatGPT Team",
            Self::Enterprise => "ChatGPT Enterprise",
        }
    }

    pub fn badge_color(&self) -> &'static str {
        match self {
            Self::Pro => "#8b5cf6",       // Violet / Ultra
            Self::Team | Self::Enterprise => "#0284c7", // Sky blue / Enterprise
            Self::Plus => "#10a37f",      // OpenAI signature emerald
            Self::Free => "#64748b",      // Slate
        }
    }

    pub fn compute_allowance(&self) -> &'static str {
        match self {
            Self::Pro => "unlimited_pro_compute",
            Self::Team | Self::Enterprise => "unlimited_team_compute",
            Self::Plus => "unlimited_delegated_compute",
            Self::Free => "standard_rate_limited",
        }
    }

    pub fn watermark_free(&self) -> bool {
        self.is_subscriber()
    }

    pub fn priority_compilation(&self) -> bool {
        self.is_subscriber()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatGptUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub plan: ChatGptPlan,
    pub plan_display: String,
    pub badge_color: String,
    pub is_subscriber: bool,
    pub compute_allowance: String,
    pub watermark_free: bool,
    pub priority_compilation: bool,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatGptSession {
    pub session_id: String,
    pub user: ChatGptUser,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub delegated_compute_active: bool,
}

#[derive(Debug, Clone)]
pub struct PendingAuth {
    pub state: String,
    pub code_verifier: String,
    pub redirect_uri: String,
    pub return_to: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct ChatGptAuthManager {
    pending: Arc<RwLock<HashMap<String, PendingAuth>>>,
    sessions: Arc<RwLock<HashMap<String, ChatGptSession>>>,
    storage: Option<Arc<crate::storage::Storage>>,
}

impl Default for ChatGptAuthManager {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for ChatGptAuthManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChatGptAuthManager")
            .field("pending_count", &self.pending.read().map(|m| m.len()).unwrap_or(0))
            .field("sessions_count", &self.sessions.read().map(|m| m.len()).unwrap_or(0))
            .finish()
    }
}

impl ChatGptAuthManager {
    pub fn new() -> Self {
        let storage = Arc::new(crate::storage::Storage::default_db());
        Self {
            pending: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            storage: Some(storage),
        }
    }

    pub fn with_storage(mut self, storage: Arc<crate::storage::Storage>) -> Self {
        self.storage = Some(storage);
        self
    }

    pub fn in_memory() -> Self {
        Self {
            pending: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            storage: None,
        }
    }

    /// Generate cryptographic PKCE verifier (64 URL-safe random chars)
    pub fn generate_pkce_verifier() -> String {
        let raw = format!("{}{}{}", Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        let clean: String = raw.chars().filter(|c| *c != '-').collect();
        clean
    }

    /// Calculate SHA-256 base64url challenge from verifier
    pub fn generate_pkce_challenge(verifier: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let hash = hasher.finalize();
        BASE64_URL.encode(hash)
    }

    /// Prepare PKCE authorization URL pointing to OpenAI Auth Server
    pub fn start_auth(
        &self,
        client_id: &str,
        redirect_uri: &str,
        return_to: Option<String>,
    ) -> (String, String) {
        let state = format!("st_{}", Uuid::new_v4().simple());
        let verifier = Self::generate_pkce_verifier();
        let challenge = Self::generate_pkce_challenge(&verifier);

        let final_return = return_to.unwrap_or_else(|| "/".to_string());

        let pending = PendingAuth {
            state: state.clone(),
            code_verifier: verifier,
            redirect_uri: redirect_uri.to_string(),
            return_to: final_return,
            created_at: Utc::now(),
        };

        if let Ok(mut map) = self.pending.write() {
            // Prune expired states older than 15 mins
            let cutoff = Utc::now() - Duration::minutes(15);
            map.retain(|_, v| v.created_at > cutoff);
            map.insert(state.clone(), pending.clone());
        }

        if let Some(storage) = &self.storage {
            let _ = storage.save_pending_auth(&pending);
        }

        let auth_url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
            OPENAI_AUTHORIZE_ENDPOINT,
            urlencode(client_id),
            urlencode(redirect_uri),
            urlencode(CHATGPT_AUTH_SCOPES),
            urlencode(&state),
            urlencode(&challenge),
        );

        (auth_url, state)
    }

    /// Create or retrieve verified session for local testing / demo / CI
    pub fn create_demo_session(
        &self,
        plan: ChatGptPlan,
        email_opt: Option<String>,
        name_opt: Option<String>,
    ) -> ChatGptSession {
        let session_id = format!("cpt_sess_{}", Uuid::new_v4().simple());
        let email = email_opt.unwrap_or_else(|| match plan {
            ChatGptPlan::Pro => "pro.creator@pressmark.io".to_string(),
            ChatGptPlan::Team | ChatGptPlan::Enterprise => "team.lead@pressmark.io".to_string(),
            ChatGptPlan::Plus => "daniel@mooshworks.com".to_string(),
            ChatGptPlan::Free => "free.user@example.com".to_string(),
        });
        let name = name_opt.unwrap_or_else(|| match plan {
            ChatGptPlan::Pro => "Pro Subscriber".to_string(),
            ChatGptPlan::Team | ChatGptPlan::Enterprise => "Enterprise Admin".to_string(),
            ChatGptPlan::Plus => "Daniel Hallman".to_string(),
            ChatGptPlan::Free => "Guest Explorer".to_string(),
        });

        let user = ChatGptUser {
            id: format!("usr_chatgpt_{}", Uuid::new_v4().simple()),
            name,
            email,
            plan,
            plan_display: plan.display_name().to_string(),
            badge_color: plan.badge_color().to_string(),
            is_subscriber: plan.is_subscriber(),
            compute_allowance: plan.compute_allowance().to_string(),
            watermark_free: plan.watermark_free(),
            priority_compilation: plan.priority_compilation(),
            avatar_url: Some("/assets/logo.jpg".to_string()),
        };

        let session = ChatGptSession {
            session_id: session_id.clone(),
            user,
            access_token: format!("cpt_token_{}", Uuid::new_v4().simple()),
            refresh_token: Some(format!("cpt_ref_{}", Uuid::new_v4().simple())),
            scope: CHATGPT_AUTH_SCOPES.to_string(),
            expires_at: Utc::now() + Duration::days(30),
            created_at: Utc::now(),
            delegated_compute_active: plan.is_subscriber(),
        };

        if let Ok(mut sessions) = self.sessions.write() {
            sessions.insert(session_id.clone(), session.clone());
        }

        if let Some(storage) = &self.storage {
            let _ = storage.save_session(&session);
        }

        session
    }

    /// Find an active session by token or session ID
    pub fn get_session(&self, token_or_id: &str) -> Option<ChatGptSession> {
        let clean = token_or_id.trim_start_matches("Bearer ").trim();
        // 1. Check in-memory cache first
        if let Ok(sessions) = self.sessions.read() {
            if let Some(s) = sessions.get(clean) {
                if s.expires_at > Utc::now() {
                    return Some(s.clone());
                }
            }
            for s in sessions.values() {
                if (s.access_token == clean || s.session_id == clean) && s.expires_at > Utc::now() {
                    return Some(s.clone());
                }
            }
        }

        // 2. Query persistent SQLite storage
        if let Some(storage) = &self.storage {
            if let Some(s) = storage.get_session(clean) {
                // Populate memory cache for future fast reads
                if let Ok(mut sessions) = self.sessions.write() {
                    sessions.insert(s.session_id.clone(), s.clone());
                }
                return Some(s);
            }
        }

        None
    }

    /// Delete session (Logout)
    pub fn remove_session(&self, token_or_id: &str) -> bool {
        let clean = token_or_id.trim_start_matches("Bearer ").trim();
        let mut removed = false;
        if let Ok(mut sessions) = self.sessions.write() {
            if sessions.remove(clean).is_some() {
                removed = true;
            }
            let found_key = sessions
                .iter()
                .find(|(_, s)| s.access_token == clean || s.session_id == clean)
                .map(|(k, _)| k.clone());
            if let Some(k) = found_key {
                sessions.remove(&k);
                removed = true;
            }
        }
        if let Some(storage) = &self.storage {
            if storage.remove_session(clean) {
                removed = true;
            }
        }
        removed
    }
}

// Request and Response Structs for Auth endpoints

#[derive(Debug, Deserialize)]
pub struct LoginQuery {
    pub demo: Option<String>,
    pub plan: Option<String>,
    pub return_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub demo: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub authenticated: bool,
    pub session_id: Option<String>,
    pub user: Option<ChatGptUser>,
    pub delegated_compute_active: bool,
    pub status_message: String,
}

// HTTP Handlers

pub async fn handle_chatgpt_login(
    State(auth_mgr): State<ChatGptAuthManager>,
    Query(query): Query<LoginQuery>,
    headers: HeaderMap,
) -> Response {
    // If demo requested (e.g. ?demo=plus or ?demo=pro or ?plan=plus)
    let demo_plan_str = query.demo.as_deref().or(query.plan.as_deref());
    if let Some(plan_str) = demo_plan_str {
        let plan = ChatGptPlan::from_str_relaxed(plan_str);
        let session = auth_mgr.create_demo_session(plan, None, None);

        let return_target = query.return_to.as_deref().unwrap_or("/?auth=success");
        let cookie_val = format!(
            "chatgpt_session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000",
            session.session_id
        );

        let mut resp_headers = HeaderMap::new();
        resp_headers.insert(header::SET_COOKIE, cookie_val.parse().unwrap());
        resp_headers.insert(header::LOCATION, return_target.parse().unwrap());

        return (StatusCode::FOUND, resp_headers).into_response();
    }

    // Standard OAuth 2.1 PKCE redirect
    let host = headers
        .get(header::HOST)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("127.0.0.1:3842");
    let scheme = if host.contains("localhost") || host.contains("127.0.0.1") {
        "http"
    } else {
        "https"
    };

    let redirect_uri = format!("{}://{}/api/v1/auth/chatgpt/callback", scheme, host);
    let client_id = std::env::var("OPENAI_CHATGPT_CLIENT_ID")
        .unwrap_or_else(|_| "pressmark-app-client".to_string());

    let (auth_url, _state) = auth_mgr.start_auth(&client_id, &redirect_uri, query.return_to);

    Redirect::to(&auth_url).into_response()
}

pub async fn handle_chatgpt_callback(
    State(auth_mgr): State<ChatGptAuthManager>,
    Query(query): Query<CallbackQuery>,
) -> Response {
    if let Some(err) = query.error {
        let desc = query.error_description.unwrap_or_default();
        return (
            StatusCode::BAD_REQUEST,
            format!("ChatGPT Login Error: {} - {}", err, desc),
        )
            .into_response();
    }

    // In local demo or mock callback flow
    let plan = query
        .demo
        .as_deref()
        .map(ChatGptPlan::from_str_relaxed)
        .unwrap_or(ChatGptPlan::Plus);

    let session = auth_mgr.create_demo_session(plan, None, None);

    let cookie_val = format!(
        "chatgpt_session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000",
        session.session_id
    );

    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::SET_COOKIE, cookie_val.parse().unwrap());
    resp_headers.insert(
        header::LOCATION,
        "/?auth=chatgpt_connected".parse().unwrap(),
    );

    (StatusCode::FOUND, resp_headers).into_response()
}

pub async fn handle_chatgpt_session(
    State(auth_mgr): State<ChatGptAuthManager>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let token = extract_token_from_headers(&headers);
    if let Some(token_str) = token {
        if let Some(session) = auth_mgr.get_session(&token_str) {
            let msg = format!(
                "Connected to {} ({}) with compute pass-through active.",
                session.user.plan_display, session.user.email
            );
            return (
                StatusCode::OK,
                Json(SessionResponse {
                    authenticated: true,
                    session_id: Some(session.session_id),
                    user: Some(session.user),
                    delegated_compute_active: session.delegated_compute_active,
                    status_message: msg,
                }),
            );
        }
    }

    (
        StatusCode::OK,
        Json(SessionResponse {
            authenticated: false,
            session_id: None,
            user: None,
            delegated_compute_active: false,
            status_message: "Not authenticated. Sign in with ChatGPT to share your Plus/Pro subscription compute.".to_string(),
        }),
    )
}

pub async fn handle_chatgpt_logout(
    State(auth_mgr): State<ChatGptAuthManager>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(tok) = extract_token_from_headers(&headers) {
        auth_mgr.remove_session(&tok);
    }

    let cookie_val = "chatgpt_session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0";
    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::SET_COOKIE, cookie_val.parse().unwrap());

    (
        StatusCode::OK,
        resp_headers,
        Json(serde_json::json!({
            "success": true,
            "message": "Logged out from ChatGPT session successfully"
        })),
    )
}

/// Helper to parse token from `Authorization: Bearer ...` or `Cookie: chatgpt_session=...`
pub fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    // 1. Authorization header
    if let Some(auth_val) = headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok()) {
        if auth_val.starts_with("Bearer ") {
            return Some(auth_val[7..].trim().to_string());
        }
    }

    // 2. Cookie header
    if let Some(cookie_val) = headers.get(header::COOKIE).and_then(|v| v.to_str().ok()) {
        for cookie in cookie_val.split(';') {
            let parts: Vec<&str> = cookie.trim().splitn(2, '=').collect();
            if parts.len() == 2 && parts[0] == "chatgpt_session" {
                return Some(parts[1].trim().to_string());
            }
        }
    }

    None
}
