//! Short-lived PDF downloads for HTTP MCP results.
//!
//! Author: Daniel Hallman

use axum::body::Bytes;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;

pub const DOWNLOAD_TTL_SECONDS: u64 = 15 * 60;
pub const MAX_DOWNLOAD_BYTES: usize = 8 * 1024 * 1024;
const MAX_CACHE_BYTES: usize = 64 * 1024 * 1024;
const MAX_CACHE_DOCUMENTS: usize = 64;

#[derive(Clone)]
pub struct PdfDownloadCache {
    inner: Arc<Mutex<Cache>>,
    ttl: Duration,
    max_file_bytes: usize,
    max_total_bytes: usize,
    max_documents: usize,
}

struct Cache {
    entries: HashMap<String, Entry>,
    total_bytes: usize,
}

struct Entry {
    document: DownloadedPdf,
    expires: Instant,
}

#[derive(Clone)]
pub struct DownloadedPdf {
    pub bytes: Bytes,
    pub filename: String,
}

#[derive(Serialize)]
pub struct DownloadLink {
    pub url: String,
    pub filename: String,
    pub expires_at: String,
    pub expires_in_seconds: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DownloadError {
    InvalidPdf,
    TooLarge,
    Capacity,
    InvalidOrigin,
    Unavailable,
}

impl DownloadError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::TooLarge => "This PDF exceeds the 8 MiB temporary-download limit. The PDF remains embedded in the tool result.",
            Self::Capacity => "Temporary download storage is full. Try generating the PDF again later. The PDF remains embedded in the tool result.",
            _ => "A temporary download link is unavailable. The PDF remains embedded in the tool result.",
        }
    }
}

impl Default for PdfDownloadCache {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Cache {
                entries: HashMap::new(),
                total_bytes: 0,
            })),
            ttl: Duration::from_secs(DOWNLOAD_TTL_SECONDS),
            max_file_bytes: MAX_DOWNLOAD_BYTES,
            max_total_bytes: MAX_CACHE_BYTES,
            max_documents: MAX_CACHE_DOCUMENTS,
        }
    }
}

impl Cache {
    fn purge(&mut self, now: Instant) {
        self.entries.retain(|_, entry| entry.expires > now);
        self.total_bytes = self
            .entries
            .values()
            .map(|entry| entry.document.bytes.len())
            .sum();
    }
}

impl PdfDownloadCache {
    pub fn insert(
        &self,
        bytes: Vec<u8>,
        doc_type: &str,
        doc_number: &str,
        base_url: &str,
    ) -> Result<DownloadLink, DownloadError> {
        self.insert_at(bytes, doc_type, doc_number, base_url, Instant::now())
    }

    fn insert_at(
        &self,
        bytes: Vec<u8>,
        doc_type: &str,
        doc_number: &str,
        base_url: &str,
        now: Instant,
    ) -> Result<DownloadLink, DownloadError> {
        if bytes.len() > self.max_file_bytes {
            return Err(DownloadError::TooLarge);
        }
        if !bytes.starts_with(b"%PDF-") {
            return Err(DownloadError::InvalidPdf);
        }
        let mut origin = reqwest::Url::parse(base_url).map_err(|_| DownloadError::InvalidOrigin)?;
        let loopback = matches!(origin.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"));
        if !(origin.scheme() == "https" || (origin.scheme() == "http" && loopback))
            || origin.host_str().is_none()
            || !origin.username().is_empty()
            || origin.password().is_some()
            || origin.query().is_some()
            || origin.fragment().is_some()
        {
            return Err(DownloadError::InvalidOrigin);
        }
        let mut cache = self.inner.lock().map_err(|_| DownloadError::Unavailable)?;
        cache.purge(now);
        if cache.entries.len() >= self.max_documents
            || cache.total_bytes.saturating_add(bytes.len()) > self.max_total_bytes
        {
            return Err(DownloadError::Capacity);
        }
        // Two independent UUID v4 values provide 244 random bits.
        let token = loop {
            let candidate = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
            if !cache.entries.contains_key(&candidate) {
                break candidate;
            }
        };
        let filename = pdf_filename(doc_type, doc_number);
        origin.set_path(&format!("/api/v1/downloads/{token}"));
        cache.total_bytes += bytes.len();
        cache.entries.insert(
            token,
            Entry {
                document: DownloadedPdf {
                    bytes: Bytes::from(bytes),
                    filename: filename.clone(),
                },
                expires: now + self.ttl,
            },
        );
        Ok(DownloadLink {
            url: origin.to_string(),
            filename,
            expires_at: (chrono::Utc::now() + chrono::Duration::seconds(self.ttl.as_secs() as i64))
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            expires_in_seconds: self.ttl.as_secs(),
        })
    }

    pub fn get(&self, token: &str) -> Option<DownloadedPdf> {
        self.get_at(token, Instant::now())
    }

    fn get_at(&self, token: &str, now: Instant) -> Option<DownloadedPdf> {
        if token.len() != 64
            || !token
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return None;
        }
        let mut cache = self.inner.lock().ok()?;
        cache.purge(now);
        cache.entries.get(token).map(|entry| entry.document.clone())
    }

    pub fn start_cleanup_task(&self) {
        let cache = Arc::downgrade(&self.inner);
        tokio::spawn(async move {
            let mut timer = tokio::time::interval(Duration::from_secs(30));
            loop {
                timer.tick().await;
                let Some(cache) = cache.upgrade() else {
                    break;
                };
                if let Ok(mut cache) = cache.lock() {
                    cache.purge(Instant::now());
                };
            }
        });
    }
}

fn pdf_filename(doc_type: &str, doc_number: &str) -> String {
    fn part(value: &str) -> String {
        value
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
            .take(60)
            .collect()
    }
    let kind = part(doc_type);
    let kind = if kind.is_empty() { "document" } else { &kind };
    let number = part(doc_number);
    if number.is_empty() {
        format!("{kind}.pdf")
    } else {
        format!("{kind}-{number}.pdf")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(link: &DownloadLink) -> &str {
        link.url.rsplit('/').next().unwrap()
    }
    fn pdf() -> Vec<u8> {
        b"%PDF-1.7\nfixture".to_vec()
    }

    #[test]
    fn expiration_invalid_tokens_and_restart_remove_access() {
        let cache = PdfDownloadCache::default();
        let now = Instant::now();
        let link = cache
            .insert_at(pdf(), "invoice", "INV-42", "https://pressmark.dev", now)
            .unwrap();
        assert_eq!(
            cache.get_at(token(&link), now).unwrap().bytes.as_ref(),
            pdf()
        );
        assert!(cache
            .get_at(
                token(&link),
                now + Duration::from_secs(DOWNLOAD_TTL_SECONDS)
            )
            .is_none());
        assert_eq!(cache.inner.lock().unwrap().total_bytes, 0);
        assert!(cache.get("../private").is_none());
        assert!(cache.get(&"0".repeat(64)).is_none());
        assert!(PdfDownloadCache::default().get(token(&link)).is_none());
    }

    #[test]
    fn capacity_and_size_limits_reject_without_evicting_valid_links() {
        let cache = PdfDownloadCache {
            max_documents: 1,
            max_total_bytes: 20,
            max_file_bytes: 20,
            ..Default::default()
        };
        let now = Instant::now();
        let link = cache
            .insert_at(pdf(), "invoice", "", "https://pressmark.dev", now)
            .unwrap();
        assert!(matches!(
            cache.insert_at(pdf(), "invoice", "", "https://pressmark.dev", now),
            Err(DownloadError::Capacity)
        ));
        assert!(cache.get_at(token(&link), now).is_some());
        assert!(matches!(
            cache.insert(vec![0; 21], "invoice", "", "https://pressmark.dev"),
            Err(DownloadError::TooLarge)
        ));
        assert!(cache
            .insert_at(
                pdf(),
                "invoice",
                "",
                "https://pressmark.dev",
                now + Duration::from_secs(DOWNLOAD_TTL_SECONDS)
            )
            .is_ok());
        let byte_limited = PdfDownloadCache {
            max_total_bytes: 20,
            ..Default::default()
        };
        byte_limited
            .insert(pdf(), "invoice", "", "https://pressmark.dev")
            .unwrap();
        assert!(matches!(
            byte_limited.insert(pdf(), "invoice", "", "https://pressmark.dev"),
            Err(DownloadError::Capacity)
        ));
    }

    #[test]
    fn links_are_unique_and_filenames_and_origins_are_safe() {
        let cache = PdfDownloadCache::default();
        let first = cache
            .insert(
                pdf(),
                "invoice",
                "../INV-42\r\n\".exe",
                "https://pressmark.dev",
            )
            .unwrap();
        let second = cache
            .insert(pdf(), "invoice", "INV-42", "https://pressmark.dev")
            .unwrap();
        assert_ne!(first.url, second.url);
        assert_eq!(token(&first).len(), 64);
        assert_eq!(first.filename, "invoice-INV-42exe.pdf");
        for origin in [
            "http://public.example",
            "https://user:secret@pressmark.dev",
            "https://pressmark.dev?secret=1",
            "https://pressmark.dev#fragment",
            "file:///tmp",
        ] {
            assert!(matches!(
                cache.insert(pdf(), "invoice", "", origin),
                Err(DownloadError::InvalidOrigin)
            ));
        }
        assert!(cache
            .insert(pdf(), "invoice", "", "http://127.0.0.1:3842")
            .is_ok());
        assert!(matches!(
            cache.insert(
                b"not a pdf".to_vec(),
                "invoice",
                "",
                "https://pressmark.dev"
            ),
            Err(DownloadError::InvalidPdf)
        ));
    }
}
