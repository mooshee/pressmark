//! HighLevel (GHL) CRM, Contact Synchronization, and Tagging Engine for Pressmark
//!
//! Author: Daniel Hallman
//!
//! Synchronizes document generation lifecycle events, paid tier unlocks, digital
//! signatures, and channel attribution with the Pressmark HighLevel CRM subaccount.

use crate::models::DocumentRequest;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

pub const PRESSMARK_LOCATION_ID: &str = "TnDcaRLbCxm7LnOUF173";

/// Contact synchronization payload for HighLevel CRM contacts/upsert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GhlContactPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub custom_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhlDispatchReceipt {
    pub success: bool,
    pub contact_id: Option<String>,
    pub tags_applied: Vec<String>,
    pub message: String,
}

#[derive(Clone)]
pub struct GhlDispatcher {
    pub location_id: String,
    pub token: String,
    pub client: reqwest::Client,
}

impl GhlDispatcher {
    /// Initialize dispatcher from environment, local PIT file, or 1Password service account.
    pub fn from_env() -> Self {
        let location_id = std::env::var("GHL_LOCATION_ID")
            .or_else(|_| std::env::var("PRESSMARK_GHL_LOCATION_ID"))
            .or_else(|_| std::env::var("HIGHLEVEL_LOCATION_ID"))
            .unwrap_or_else(|_| PRESSMARK_LOCATION_ID.to_string());

        let token = Self::resolve_token(&location_id).unwrap_or_default();
        Self {
            location_id,
            token,
            client: reqwest::Client::new(),
        }
    }

    pub fn new(location_id: String, token: String) -> Self {
        Self {
            location_id,
            token,
            client: reqwest::Client::new(),
        }
    }

    fn resolve_token(location_id: &str) -> Result<String, String> {
        if let Ok(tok) = std::env::var("GHL_LOCATION_PIT")
            .or_else(|_| std::env::var("HIGHLEVEL_API_KEY"))
            .or_else(|_| std::env::var("PRESSMARK_GHL_PIT"))
        {
            if !tok.trim().is_empty() {
                return Ok(tok.trim().to_string());
            }
        }

        let pit_path = format!(
            "{}/.config/ghl-agency/contexts/moosh-growth/locations/{}.pit",
            std::env::var("HOME").unwrap_or_default(),
            location_id
        );
        if let Ok(tok) = fs::read_to_string(&pit_path) {
            if !tok.trim().is_empty() {
                return Ok(tok.trim().to_string());
            }
        }

        let op_script = "/Users/dhallman/Codes/personal/hallman-lab/scripts/with-op-sa.sh";
        let output = Command::new(op_script)
            .args([
                "op",
                "item",
                "get",
                "Operator Key — HighLevel Location PIT — Pressmark",
                "--vault",
                "Moosh Works Operator",
                "--fields",
                "credential",
                "--reveal",
            ])
            .output()
            .map_err(|e| format!("Failed to run with-op-sa.sh: {}", e))?;

        if output.status.success() {
            let tok = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !tok.is_empty() {
                return Ok(tok);
            }
        }

        Err("No HighLevel Location PIT resolved for Pressmark".to_string())
    }

    /// Upsert contact in HighLevel CRM with tags and custom fields.
    pub async fn upsert_contact(&self, payload: &GhlContactPayload) -> Result<GhlDispatchReceipt, String> {
        if self.token.is_empty() {
            return Err("HighLevel Location PIT is missing or unauthenticated".to_string());
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))
                .map_err(|e| format!("Invalid auth header: {}", e))?,
        );
        headers.insert("Version", HeaderValue::from_static("2021-07-28"));

        let mut body = json!({
            "locationId": self.location_id,
            "tags": payload.tags,
        });

        if let Some(ref em) = payload.email {
            body["email"] = json!(em);
        }
        if let Some(ref ph) = payload.phone {
            body["phone"] = json!(ph);
        }
        if let Some(ref n) = payload.name {
            body["name"] = json!(n);
        }
        if let Some(ref fn_) = payload.first_name {
            body["firstName"] = json!(fn_);
        }
        if let Some(ref ln) = payload.last_name {
            body["lastName"] = json!(ln);
        }
        if let Some(ref co) = payload.company_name {
            body["companyName"] = json!(co);
        }
        if let Some(ref a1) = payload.address1 {
            body["address1"] = json!(a1);
        }
        if let Some(ref ci) = payload.city {
            body["city"] = json!(ci);
        }
        if let Some(ref st) = payload.state {
            body["state"] = json!(st);
        }
        if let Some(ref pc) = payload.postal_code {
            body["postalCode"] = json!(pc);
        }

        if !payload.custom_fields.is_empty() {
            let mut custom_fields_arr = Vec::new();
            for (key, val) in &payload.custom_fields {
                custom_fields_arr.push(json!({
                    "key": key,
                    "field_value": val
                }));
            }
            body["customFields"] = json!(custom_fields_arr);
        }

        let url = "https://services.leadconnectorhq.com/contacts/upsert";
        let resp = self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("GHL request error: {}", e))?;

        let status = resp.status();
        let resp_text = resp.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(format!("GHL upsert failed ({}): {}", status, resp_text));
        }

        let contact_id = serde_json::from_str::<serde_json::Value>(&resp_text)
            .ok()
            .and_then(|v| {
                v.get("contact")
                    .and_then(|c| c.get("id"))
                    .and_then(|id| id.as_str())
                    .map(|s| s.to_string())
            });

        Ok(GhlDispatchReceipt {
            success: true,
            contact_id,
            tags_applied: payload.tags.clone(),
            message: "Successfully synchronized contact to HighLevel CRM".to_string(),
        })
    }

    /// Record document generation event in HighLevel CRM with archetype tags.
    pub async fn dispatch_document_generation(
        &self,
        doc: &DocumentRequest,
        user_email: Option<&str>,
        channel: &str,
    ) -> Result<GhlDispatchReceipt, String> {
        let archetype_slug = format!("{:?}", doc.doc_type).to_lowercase();
        let archetype_title = doc.doc_type.display_title();
        let total_val = doc.grand_total();

        let mut tags = vec![
            format!("pressmark:channel:{}", channel),
            format!("pressmark:doc:{}-created", archetype_slug),
            "pressmark:lead:active-creator".to_string(),
        ];

        let is_watermarked = doc.watermark.as_deref() != Some("false");
        if is_watermarked {
            tags.push("pressmark:status:watermarked".to_string());
        } else {
            tags.push("pressmark:status:unwatermarked".to_string());
        }

        if !doc.signatures.is_empty() {
            tags.push("pressmark:feature:digital-signatures".to_string());
        }
        if doc.compliance_mode.as_deref().unwrap_or("none") != "none" {
            tags.push(format!(
                "pressmark:compliance:{}",
                doc.compliance_mode.as_deref().unwrap_or("none")
            ));
        }

        let mut custom_fields = HashMap::new();
        custom_fields.insert("pressmark_document_number".to_string(), json!(doc.effective_doc_number()));
        custom_fields.insert("pressmark_document_title".to_string(), json!(doc.title.as_deref().unwrap_or(archetype_title)));
        custom_fields.insert("pressmark_document_archetype".to_string(), json!(archetype_title));
        custom_fields.insert("pressmark_total_amount".to_string(), json!(total_val));
        custom_fields.insert("pressmark_currency".to_string(), json!(doc.currency.as_deref().unwrap_or("USD")));
        custom_fields.insert("pressmark_source_channel".to_string(), json!(channel));
        custom_fields.insert("pressmark_last_action".to_string(), json!("document_generated"));

        let email = user_email
            .or_else(|| doc.from.email.as_deref())
            .map(|s| s.to_string());

        let payload = GhlContactPayload {
            email,
            phone: doc.from.phone.clone(),
            name: Some(doc.from.name.clone()),
            first_name: None,
            last_name: None,
            company_name: doc.from.company.clone(),
            address1: doc.from.address.clone(),
            city: None,
            state: None,
            postal_code: None,
            tags,
            custom_fields,
        };

        self.upsert_contact(&payload).await
    }

    /// Record a payment unlock (Single Pass or Subscription) in HighLevel CRM.
    pub async fn dispatch_payment_unlock(
        &self,
        doc_number: &str,
        user_email: &str,
        tier_or_pass: &str,
        amount_cents: u32,
    ) -> Result<GhlDispatchReceipt, String> {
        let mut tags = vec![
            "pressmark:customer:paying".to_string(),
            format!("pressmark:unlock:{}", tier_or_pass),
            "pressmark:action:watermark-removed".to_string(),
        ];

        if tier_or_pass.contains("sub_") {
            tags.push("pressmark:status:subscriber".to_string());
        } else {
            tags.push("pressmark:status:microtransaction-pass".to_string());
        }

        let mut custom_fields = HashMap::new();
        custom_fields.insert("pressmark_document_number".to_string(), json!(doc_number));
        custom_fields.insert("pressmark_subscription_tier".to_string(), json!(tier_or_pass));
        custom_fields.insert("pressmark_total_amount".to_string(), json!(amount_cents as f64 / 100.0));
        custom_fields.insert("pressmark_last_action".to_string(), json!("payment_unlocked"));

        let payload = GhlContactPayload {
            email: Some(user_email.to_string()),
            phone: None,
            name: None,
            first_name: None,
            last_name: None,
            company_name: None,
            address1: None,
            city: None,
            state: None,
            postal_code: None,
            tags,
            custom_fields,
        };

        self.upsert_contact(&payload).await
    }
}
