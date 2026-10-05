//! Pressmark Stripe Live Webhook & Fulfillment Engine
//!
//! Author: Daniel Hallman

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use subtle::ConstantTimeEq;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub enum StripeWebhookError {
    MissingHeader,
    MalformedHeader,
    InvalidTimestamp,
    TimestampTooOld(u64),
    InvalidSignature,
    PayloadUtf8Error,
    SecretNotConfigured,
}

impl std::fmt::Display for StripeWebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingHeader => write!(f, "Missing Stripe-Signature header"),
            Self::MalformedHeader => write!(f, "Malformed Stripe-Signature header"),
            Self::InvalidTimestamp => write!(f, "Invalid timestamp in Stripe-Signature"),
            Self::TimestampTooOld(age) => write!(f, "Timestamp too old ({}s)", age),
            Self::InvalidSignature => write!(f, "Invalid Stripe webhook HMAC-SHA256 signature"),
            Self::PayloadUtf8Error => write!(f, "Webhook payload is not valid UTF-8"),
            Self::SecretNotConfigured => write!(f, "STRIPE_WEBHOOK_SECRET is not configured"),
        }
    }
}

impl std::error::Error for StripeWebhookError {}

/// Check if Stripe live mode is active (via STRIPE_LIVE_MODE=true/1)
pub fn is_live_mode() -> bool {
    std::env::var("STRIPE_LIVE_MODE")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false)
}

/// Dynamically resolve Stripe Secret Key according to STRIPE_LIVE_MODE
pub fn get_stripe_secret_key() -> Option<String> {
    if is_live_mode() {
        std::env::var("STRIPE_SECRET_KEY_LIVE").or_else(|_| std::env::var("STRIPE_SECRET_KEY")).ok()
    } else {
        std::env::var("STRIPE_SECRET_KEY_TEST").or_else(|_| std::env::var("STRIPE_SECRET_KEY")).ok()
    }
}

/// Dynamically resolve Stripe Publishable Key according to STRIPE_LIVE_MODE
pub fn get_stripe_publishable_key() -> Option<String> {
    if is_live_mode() {
        std::env::var("STRIPE_PUBLISHABLE_KEY_LIVE").or_else(|_| std::env::var("STRIPE_PUBLISHABLE_KEY")).ok()
    } else {
        std::env::var("STRIPE_PUBLISHABLE_KEY_TEST").or_else(|_| std::env::var("STRIPE_PUBLISHABLE_KEY")).ok()
    }
}

/// Dynamically resolve Stripe Webhook Secret according to STRIPE_LIVE_MODE
pub fn get_stripe_webhook_secret() -> Option<String> {
    if is_live_mode() {
        std::env::var("STRIPE_WEBHOOK_SECRET_LIVE").or_else(|_| std::env::var("STRIPE_WEBHOOK_SECRET")).ok()
    } else {
        std::env::var("STRIPE_WEBHOOK_SECRET_TEST").or_else(|_| std::env::var("STRIPE_WEBHOOK_SECRET")).ok()
    }
}

/// Verify the Stripe-Signature header against the raw body payload.
/// Format: `t=1492774577,v1=5257a869e7ecebeda32affa62cd323080cd477e62cf525e80cc1e88297e286add`
pub fn verify_stripe_signature(
    payload: &[u8],
    sig_header: &str,
    secret: &str,
    tolerance_secs: u64,
) -> Result<u64, StripeWebhookError> {
    if secret.is_empty() {
        return Err(StripeWebhookError::SecretNotConfigured);
    }

    let mut timestamp: Option<u64> = None;
    let mut signatures: Vec<&str> = Vec::new();

    for item in sig_header.split(',') {
        let parts: Vec<&str> = item.trim().splitn(2, '=').collect();
        if parts.len() == 2 {
            match parts[0] {
                "t" => {
                    timestamp = parts[1].parse::<u64>().ok();
                }
                "v1" => {
                    signatures.push(parts[1]);
                }
                _ => {}
            }
        }
    }

    let t = timestamp.ok_or(StripeWebhookError::InvalidTimestamp)?;
    if signatures.is_empty() {
        return Err(StripeWebhookError::MalformedHeader);
    }

    // Verify timestamp tolerance (anti-replay attack)
    if tolerance_secs > 0 {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let age = current_time.saturating_sub(t);
        let future_drift = t.saturating_sub(current_time);

        if age > tolerance_secs || future_drift > tolerance_secs {
            return Err(StripeWebhookError::TimestampTooOld(age));
        }
    }

    let payload_str = std::str::from_utf8(payload).map_err(|_| StripeWebhookError::PayloadUtf8Error)?;
    let signed_payload = format!("{}.{}", t, payload_str);

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| StripeWebhookError::SecretNotConfigured)?;
    mac.update(signed_payload.as_bytes());
    let expected_bytes = mac.finalize().into_bytes();

    let matched = signatures.iter().any(|&sig| {
        if let Ok(sig_bytes) = hex::decode(sig.trim()) {
            expected_bytes.as_slice().ct_eq(&sig_bytes).into()
        } else {
            false
        }
    });

    if !matched {
        return Err(StripeWebhookError::InvalidSignature);
    }

    Ok(t)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeEvent {
    pub id: String,
    #[serde(default)]
    pub object: String,
    pub r#type: String,
    pub data: StripeEventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeEventData {
    pub object: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "details")]
pub enum FulfillmentAction {
    PaymentUnlocked {
        customer_email: String,
        amount_cents: u64,
        currency: String,
        document_number: Option<String>,
        receipt_number: String,
        unlocked_tier: Option<String>,
    },
    CheckoutCompleted {
        customer_email: String,
        session_id: String,
        amount_total_cents: u64,
        document_number: Option<String>,
    },
    ChargeRefunded {
        customer_email: String,
        refund_amount_cents: u64,
        credit_note_number: String,
        original_document_number: Option<String>,
        reason: Option<String>,
    },
    Ignored {
        event_type: String,
    },
}

/// Process a verified Stripe event and trigger business fulfillment
pub fn process_stripe_event(event: StripeEvent) -> Result<FulfillmentAction, String> {
    match event.r#type.as_str() {
        "payment_intent.succeeded" => {
            let obj = &event.data.object;
            let amount_cents = obj["amount"].as_u64().unwrap_or(0);
            let currency = obj["currency"].as_str().unwrap_or("usd").to_uppercase();
            let customer_email = obj["receipt_email"]
                .as_str()
                .or_else(|| obj["charges"]["data"][0]["billing_details"]["email"].as_str())
                .unwrap_or("customer@pressmark.io")
                .to_string();

            let doc_num = obj["metadata"]["document_number"]
                .as_str()
                .map(|s| s.to_string());
            let tier = obj["metadata"]["tier"].as_str().map(|s| s.to_string());

            let receipt_number = if let Some(ref d) = doc_num {
                d.replace("INV-", "REC-").replace("EST-", "REC-")
            } else {
                format!("REC-{}", &event.id[..8.min(event.id.len())])
            };

            tracing::info!(
                "Stripe payment_intent.succeeded: ${:.2} {} from {} (Doc: {:?}, Receipt: {})",
                (amount_cents as f64) / 100.0,
                currency,
                customer_email,
                doc_num,
                receipt_number
            );

            Ok(FulfillmentAction::PaymentUnlocked {
                customer_email,
                amount_cents,
                currency,
                document_number: doc_num,
                receipt_number,
                unlocked_tier: tier,
            })
        }
        "checkout.session.completed" => {
            let obj = &event.data.object;
            let session_id = obj["id"].as_str().unwrap_or(&event.id).to_string();
            let amount_total_cents = obj["amount_total"].as_u64().unwrap_or(0);
            let customer_email = obj["customer_details"]["email"]
                .as_str()
                .or_else(|| obj["customer_email"].as_str())
                .unwrap_or("customer@pressmark.io")
                .to_string();

            let doc_num = obj["metadata"]["document_number"]
                .as_str()
                .map(|s| s.to_string());

            tracing::info!(
                "Stripe checkout.session.completed: session {} for {}",
                session_id,
                customer_email
            );

            Ok(FulfillmentAction::CheckoutCompleted {
                customer_email,
                session_id,
                amount_total_cents,
                document_number: doc_num,
            })
        }
        "charge.refunded" => {
            let obj = &event.data.object;
            let refund_cents = obj["amount_refunded"].as_u64().unwrap_or(0);
            let customer_email = obj["billing_details"]["email"]
                .as_str()
                .unwrap_or("customer@pressmark.io")
                .to_string();

            let original_doc = obj["metadata"]["document_number"]
                .as_str()
                .map(|s| s.to_string());

            let credit_note_number = if let Some(ref d) = original_doc {
                d.replace("INV-", "CRD-").replace("REC-", "CRD-")
            } else {
                format!("CRD-{}", &event.id[..8.min(event.id.len())])
            };

            let reason = obj["refunds"]["data"][0]["reason"]
                .as_str()
                .map(|s| s.to_string());

            tracing::info!(
                "Stripe charge.refunded: refunded ${:.2} to {} (Credit Note: {}, Original: {:?})",
                (refund_cents as f64) / 100.0,
                customer_email,
                credit_note_number,
                original_doc
            );

            Ok(FulfillmentAction::ChargeRefunded {
                customer_email,
                refund_amount_cents: refund_cents,
                credit_note_number,
                original_document_number: original_doc,
                reason,
            })
        }
        other => Ok(FulfillmentAction::Ignored {
            event_type: other.to_string(),
        }),
    }
}

/// Thread-safe idempotency store for Stripe webhook event IDs, backed by memory & SQLite
#[derive(Clone, Default)]
pub struct StripeIdempotencyStore {
    processed: Arc<tokio::sync::Mutex<std::collections::HashSet<String>>>,
    storage: Option<Arc<crate::storage::Storage>>,
}

impl StripeIdempotencyStore {
    pub fn new() -> Self {
        Self {
            processed: Arc::new(tokio::sync::Mutex::new(std::collections::HashSet::new())),
            storage: Some(Arc::new(crate::storage::Storage::default_db())),
        }
    }

    pub fn with_storage(mut self, storage: Arc<crate::storage::Storage>) -> Self {
        self.storage = Some(storage);
        self
    }

    pub async fn is_processed(&self, event_id: &str) -> bool {
        let lock = self.processed.lock().await;
        if lock.contains(event_id) {
            return true;
        }
        drop(lock);

        if let Some(storage) = &self.storage {
            let storage = Arc::clone(storage);
            let id = event_id.to_string();
            let is_proc = tokio::task::spawn_blocking(move || storage.is_stripe_event_processed(&id))
                .await
                .unwrap_or(false);
            if is_proc {
                return true;
            }
        }
        false
    }

    pub async fn record(&self, event_id: &str) {
        let mut lock = self.processed.lock().await;
        lock.insert(event_id.to_string());
        drop(lock);

        if let Some(storage) = &self.storage {
            let storage = Arc::clone(storage);
            let id = event_id.to_string();
            let _ = tokio::task::spawn_blocking(move || storage.record_stripe_event(&id)).await;
        }
    }
}

