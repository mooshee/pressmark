//! Pressmark In-Memory Rate Limiting & DoS Defense Middleware
//!
//! Author: Daniel Hallman

use axum::extract::{Request, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
struct IpHistory {
    requests: Vec<Instant>,
}

const NUM_SHARDS: usize = 64;

#[derive(Clone)]
pub struct RateLimiter {
    shards: Arc<Vec<Mutex<HashMap<String, IpHistory>>>>,
    pub max_requests: usize,
    pub window: Duration,
    pub enabled: bool,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        let enabled = std::env::var("PRESSMARK_RATE_LIMIT")
            .map(|v| v != "0" && v != "false")
            .unwrap_or(true);

        let mut shards = Vec::with_capacity(NUM_SHARDS);
        for _ in 0..NUM_SHARDS {
            shards.push(Mutex::new(HashMap::new()));
        }

        Self {
            shards: Arc::new(shards),
            max_requests,
            window: Duration::from_secs(window_secs),
            enabled,
        }
    }

    fn shard_for(&self, client_ip: &str) -> &Mutex<HashMap<String, IpHistory>> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        client_ip.hash(&mut hasher);
        let idx = (hasher.finish() as usize) % self.shards.len();
        &self.shards[idx]
    }

    /// Check if the given client IP is allowed.
    /// Returns (allowed: bool, current_count: usize, reset_seconds: u64).
    pub async fn check(&self, client_ip: &str) -> (bool, usize, u64) {
        if !self.enabled {
            return (true, 0, 0);
        }

        let now = Instant::now();
        let cutoff = now.checked_sub(self.window).unwrap_or(now);

        let shard = self.shard_for(client_ip);
        let mut lock = shard.lock().await;

        // Clean up stale requests
        let history = lock.entry(client_ip.to_string()).or_insert_with(|| IpHistory {
            requests: Vec::new(),
        });
        history.requests.retain(|&t| t > cutoff);

        if history.requests.len() >= self.max_requests {
            let earliest = history.requests.first().cloned().unwrap_or(now);
            let elapsed = now.duration_since(earliest);
            let remaining_secs = self.window.saturating_sub(elapsed).as_secs().max(1);
            return (false, history.requests.len(), remaining_secs);
        }

        history.requests.push(now);
        let remaining_secs = self.window.as_secs();
        (true, history.requests.len(), remaining_secs)
    }

    /// Background eviction across all shards sequentially, avoiding monolithic lock stalls.
    pub fn start_cleanup_task(&self, interval_secs: u64) {
        let limiter = self.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs));
            loop {
                ticker.tick().await;
                let now = Instant::now();
                let cutoff = now.checked_sub(limiter.window).unwrap_or(now);
                for shard in limiter.shards.iter() {
                    let mut lock = shard.lock().await;
                    lock.retain(|_, history| {
                        history.requests.retain(|&t| t > cutoff);
                        !history.requests.is_empty()
                    });
                }
            }
        });
    }
}

pub fn extract_client_ip(headers: &HeaderMap, connect_info: Option<&SocketAddr>) -> String {
    let trust_proxy = std::env::var("PRESSMARK_TRUST_PROXY_HEADERS")
        .map(|v| v != "0" && v != "false")
        .unwrap_or(true);

    if trust_proxy {
        // 1. Cloudflare Connecting IP
        if let Some(cf_ip) = headers.get("cf-connecting-ip").and_then(|v| v.to_str().ok()) {
            let trimmed = cf_ip.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }

        // 2. X-Forwarded-For (First IP in list)
        if let Some(xff) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
            if let Some(first) = xff.split(',').next() {
                let trimmed = first.trim();
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }

        // 3. X-Real-IP
        if let Some(real_ip) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
            let trimmed = real_ip.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }

    // 4. Fallback to peer address or localhost
    connect_info
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "127.0.0.1".to_string())
}

pub async fn rate_limit_middleware(
    State(limiter): State<RateLimiter>,
    req: Request,
    next: Next,
) -> Response {
    let connect_info = req.extensions().get::<axum::extract::ConnectInfo<SocketAddr>>().map(|ci| &ci.0);
    let client_ip = extract_client_ip(req.headers(), connect_info);

    // Bypass rate limiting for internal health checks and static discovery metadata
    let path = req.uri().path();
    if path == "/health"
        || path == "/healthz"
        || path == "/ready"
        || path.starts_with("/.well-known/")
        || path.starts_with("/mcp/.well-known/")
        || path.starts_with("/api/v1/mcp/.well-known/")
        || path == "/robots.txt"
        || path == "/sitemap.xml"
        || path == "/llms.txt"
        || path == "/llms-full.txt"
        || path == "/favicon.ico"
        || path == "/favicon.svg"
        || path == "/favicon.png"
        || path == "/.mcp.json"
        || path == "/AGENTS.md"
        || path == "/AGENT.md"
        || path == "/QWEN.md"
        || path == "/CLAUDE.md"
    {
        return next.run(req).await;
    }

    let (allowed, current_count, reset_secs) = limiter.check(&client_ip).await;

    if !allowed {
        let body = json!({
            "error": "rate_limit_exceeded",
            "message": "Too many requests. Please wait before compiling again.",
            "client_ip": client_ip,
            "retry_after_seconds": reset_secs,
            "limit": limiter.max_requests
        });

        let mut res = (
            StatusCode::TOO_MANY_REQUESTS,
            [(header::CONTENT_TYPE, "application/json")],
            body.to_string(),
        )
            .into_response();

        let headers = res.headers_mut();
        headers.insert(header::RETRY_AFTER, HeaderValue::from(reset_secs));
        headers.insert("X-RateLimit-Limit", HeaderValue::from(limiter.max_requests));
        headers.insert("X-RateLimit-Remaining", HeaderValue::from(0));
        headers.insert("X-RateLimit-Reset", HeaderValue::from(reset_secs));

        return res;
    }

    let mut res = next.run(req).await;
    let remaining = limiter.max_requests.saturating_sub(current_count);
    let headers = res.headers_mut();
    headers.insert("X-RateLimit-Limit", HeaderValue::from(limiter.max_requests));
    headers.insert("X-RateLimit-Remaining", HeaderValue::from(remaining));
    headers.insert("X-RateLimit-Reset", HeaderValue::from(reset_secs));

    res
}
