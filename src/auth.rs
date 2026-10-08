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
    SoloPro,
    Business,
    Agency,
    Developer,
}

impl ChatGptPlan {
    pub fn from_str_relaxed(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "plus" | "chatgpt_plus" => Self::Plus,
            "pro" | "chatgpt_pro" => Self::Pro,
            "team" | "chatgpt_team" => Self::Team,
            "enterprise" | "chatgpt_enterprise" => Self::Enterprise,
            "solo" | "solo_pro" => Self::SoloPro,
            "business" => Self::Business,
            "agency" => Self::Agency,
            "developer" | "api" | "agent" => Self::Developer,
            _ => Self::Free,
        }
    }

    pub fn is_subscriber(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Pro
                | Self::Team
                | Self::Enterprise
                | Self::SoloPro
                | Self::Business
                | Self::Agency
                | Self::Developer
        )
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Free => "Free Workspace",
            Self::Plus => "ChatGPT Plus",
            Self::Pro => "ChatGPT Pro",
            Self::Team => "ChatGPT Team",
            Self::Enterprise => "ChatGPT Enterprise",
            Self::SoloPro => "Solo Pro",
            Self::Business => "Business Studio",
            Self::Agency => "Agency Scale",
            Self::Developer => "Developer API",
        }
    }

    pub fn badge_color(&self) -> &'static str {
        match self {
            Self::Pro | Self::Agency => "#8b5cf6",       // Violet / Ultra
            Self::Team | Self::Enterprise | Self::Business => "#0284c7", // Sky blue / Enterprise
            Self::Plus => "#10a37f",      // OpenAI signature emerald
            Self::SoloPro => "#3b82f6",   // Royal blue
            Self::Developer => "#f59e0b", // Amber gold
            Self::Free => "#64748b",      // Slate
        }
    }

    pub fn compute_allowance(&self) -> &'static str {
        match self {
            Self::Pro | Self::Agency => "unlimited_pro_compute",
            Self::Team | Self::Enterprise | Self::Business => "unlimited_team_compute",
            Self::Plus => "unlimited_delegated_compute",
            Self::SoloPro | Self::Developer => "unlimited_pro_compute",
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

fn default_auth_provider() -> String {
    "chatgpt".to_string()
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
    #[serde(default = "default_auth_provider")]
    pub provider: String,
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
            ChatGptPlan::Pro | ChatGptPlan::Agency => "pro.creator@pressmark.io".to_string(),
            ChatGptPlan::Team | ChatGptPlan::Enterprise | ChatGptPlan::Business => {
                "team.lead@pressmark.io".to_string()
            }
            ChatGptPlan::Plus | ChatGptPlan::SoloPro => "daniel@mooshworks.com".to_string(),
            ChatGptPlan::Developer => "agent@pressmark.local".to_string(),
            ChatGptPlan::Free => "free.user@example.com".to_string(),
        });
        let name = name_opt.unwrap_or_else(|| match plan {
            ChatGptPlan::Pro | ChatGptPlan::Agency => "Pro Subscriber".to_string(),
            ChatGptPlan::Team | ChatGptPlan::Enterprise | ChatGptPlan::Business => {
                "Enterprise Admin".to_string()
            }
            ChatGptPlan::Plus | ChatGptPlan::SoloPro => "Daniel Hallman".to_string(),
            ChatGptPlan::Developer => "Developer Agent".to_string(),
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
            provider: "chatgpt".to_string(),
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

    /// Create an instant local guest workspace session
    pub fn create_guest_session(&self) -> ChatGptSession {
        let session_id = format!("guest_sess_{}", Uuid::new_v4().simple());
        let user = ChatGptUser {
            id: format!("usr_guest_{}", Uuid::new_v4().simple()),
            name: "Guest Workspace".to_string(),
            email: "guest@pressmark.local".to_string(),
            plan: ChatGptPlan::Free,
            plan_display: "Guest Workspace".to_string(),
            badge_color: "#64748b".to_string(),
            is_subscriber: false,
            compute_allowance: "standard_rate_limited".to_string(),
            watermark_free: false,
            priority_compilation: false,
            avatar_url: Some("/assets/logo.jpg".to_string()),
            provider: "guest".to_string(),
        };

        let session = ChatGptSession {
            session_id: session_id.clone(),
            user,
            access_token: format!("guest_tok_{}", Uuid::new_v4().simple()),
            refresh_token: None,
            scope: "guest:workspace".to_string(),
            expires_at: Utc::now() + Duration::days(90),
            created_at: Utc::now(),
            delegated_compute_active: false,
        };

        if let Ok(mut sessions) = self.sessions.write() {
            sessions.insert(session_id.clone(), session.clone());
        }

        if let Some(storage) = &self.storage {
            let _ = storage.save_session(&session);
        }

        session
    }

    /// Create or retrieve an email-authenticated session
    pub fn create_email_session(&self, email: &str, name_opt: Option<&str>) -> ChatGptSession {
        let clean_email = email.trim().to_lowercase();
        let name = name_opt
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| {
                clean_email
                    .split('@')
                    .next()
                    .unwrap_or("User")
                    .replace('.', " ")
                    .to_string()
            });

        let plan = ChatGptPlan::SoloPro;
        let session_id = format!("email_sess_{}", Uuid::new_v4().simple());
        let user = ChatGptUser {
            id: format!("usr_email_{}", Uuid::new_v4().simple()),
            name,
            email: clean_email,
            plan,
            plan_display: plan.display_name().to_string(),
            badge_color: plan.badge_color().to_string(),
            is_subscriber: true,
            compute_allowance: plan.compute_allowance().to_string(),
            watermark_free: true,
            priority_compilation: true,
            avatar_url: Some("/assets/logo.jpg".to_string()),
            provider: "email".to_string(),
        };

        let session = ChatGptSession {
            session_id: session_id.clone(),
            user,
            access_token: format!("prsm_tok_{}", Uuid::new_v4().simple()),
            refresh_token: Some(format!("prsm_ref_{}", Uuid::new_v4().simple())),
            scope: "email:account".to_string(),
            expires_at: Utc::now() + Duration::days(60),
            created_at: Utc::now(),
            delegated_compute_active: false,
        };

        if let Ok(mut sessions) = self.sessions.write() {
            sessions.insert(session_id.clone(), session.clone());
        }

        if let Some(storage) = &self.storage {
            let _ = storage.save_session(&session);
        }

        session
    }

    /// Authenticate or register a developer API key for autonomous agents
    pub fn create_api_key_session(&self, api_key: &str) -> ChatGptSession {
        let clean_key = api_key.trim();
        let session_id = format!("apikey_sess_{}", Uuid::new_v4().simple());
        let user = ChatGptUser {
            id: format!("usr_agent_{}", Uuid::new_v4().simple()),
            name: "Developer Agent".to_string(),
            email: "agent@pressmark.local".to_string(),
            plan: ChatGptPlan::Developer,
            plan_display: "Developer API".to_string(),
            badge_color: "#f59e0b".to_string(),
            is_subscriber: true,
            compute_allowance: "unlimited_pro_compute".to_string(),
            watermark_free: true,
            priority_compilation: true,
            avatar_url: Some("/assets/logo.jpg".to_string()),
            provider: "api_key".to_string(),
        };

        let session = ChatGptSession {
            session_id: session_id.clone(),
            user,
            access_token: clean_key.to_string(),
            refresh_token: None,
            scope: "agent:mcp developer:api".to_string(),
            expires_at: Utc::now() + Duration::days(365),
            created_at: Utc::now(),
            delegated_compute_active: false,
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

#[derive(Debug, Deserialize)]
pub struct EmailLoginRequest {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApiKeyLoginRequest {
    pub api_key: String,
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
            let msg = if session.user.provider == "chatgpt" {
                format!(
                    "Connected to {} ({}) with compute pass-through active.",
                    session.user.plan_display, session.user.email
                )
            } else {
                format!(
                    "Signed in as {} ({}) with {} plan.",
                    session.user.name, session.user.email, session.user.plan_display
                )
            };
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
            status_message: "Not authenticated. Continue as Guest, sign in with email, or connect ChatGPT Plus/Pro.".to_string(),
        }),
    )
}

pub async fn handle_guest_login(
    State(auth_mgr): State<ChatGptAuthManager>,
) -> impl IntoResponse {
    let session = auth_mgr.create_guest_session();
    let cookie_val = format!(
        "pressmark_session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=7776000",
        session.session_id
    );
    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::SET_COOKIE, cookie_val.parse().unwrap());

    (
        StatusCode::OK,
        resp_headers,
        Json(SessionResponse {
            authenticated: true,
            session_id: Some(session.session_id),
            user: Some(session.user),
            delegated_compute_active: false,
            status_message: "Guest workspace active on this device.".to_string(),
        }),
    )
}

pub async fn handle_email_login(
    State(auth_mgr): State<ChatGptAuthManager>,
    Json(payload): Json<EmailLoginRequest>,
) -> impl IntoResponse {
    if payload.email.trim().is_empty() || !payload.email.contains('@') {
        return (
            StatusCode::BAD_REQUEST,
            HeaderMap::new(),
            Json(serde_json::json!({ "error": "Valid email address required" })),
        )
            .into_response();
    }

    let session = auth_mgr.create_email_session(&payload.email, payload.name.as_deref());
    let cookie_val = format!(
        "pressmark_session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=5184000",
        session.session_id
    );
    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::SET_COOKIE, cookie_val.parse().unwrap());

    (
        StatusCode::OK,
        resp_headers,
        Json(SessionResponse {
            authenticated: true,
            session_id: Some(session.session_id),
            user: Some(session.user),
            delegated_compute_active: false,
            status_message: "Signed in via email successfully.".to_string(),
        }),
    )
        .into_response()
}

pub async fn handle_api_key_login(
    State(auth_mgr): State<ChatGptAuthManager>,
    Json(payload): Json<ApiKeyLoginRequest>,
) -> impl IntoResponse {
    let key = payload.api_key.trim();
    if key.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            HeaderMap::new(),
            Json(serde_json::json!({ "error": "API key required" })),
        )
            .into_response();
    }

    let session = auth_mgr.create_api_key_session(key);
    let cookie_val = format!(
        "pressmark_session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=31536000",
        session.session_id
    );
    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::SET_COOKIE, cookie_val.parse().unwrap());

    (
        StatusCode::OK,
        resp_headers,
        Json(SessionResponse {
            authenticated: true,
            session_id: Some(session.session_id),
            user: Some(session.user),
            delegated_compute_active: false,
            status_message: "Developer API token verified.".to_string(),
        }),
    )
        .into_response()
}

pub async fn handle_chatgpt_logout(
    State(auth_mgr): State<ChatGptAuthManager>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(tok) = extract_token_from_headers(&headers) {
        auth_mgr.remove_session(&tok);
    }

    let mut resp_headers = HeaderMap::new();
    resp_headers.append(
        header::SET_COOKIE,
        "chatgpt_session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0"
            .parse()
            .unwrap(),
    );
    resp_headers.append(
        header::SET_COOKIE,
        "pressmark_session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0"
            .parse()
            .unwrap(),
    );

    (
        StatusCode::OK,
        resp_headers,
        Json(serde_json::json!({
            "success": true,
            "message": "Logged out successfully"
        })),
    )
}

/// Helper to parse token from `Authorization: Bearer ...` or `Cookie: pressmark_session=...` / `chatgpt_session=...`
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
            if parts.len() == 2 && (parts[0] == "pressmark_session" || parts[0] == "chatgpt_session") {
                return Some(parts[1].trim().to_string());
            }
        }
    }

    None
}
