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

/// HighLevel Pipeline & Stage IDs in `TnDcaRLbCxm7LnOUF173` (`Pressmark Conversion & Subscriptions`)
pub const PRESSMARK_PIPELINE_ID: &str = "HYdsJf17lGH7rIhCwCnx";
pub const STAGE_WORKSPACE_SIGNED_IN: &str = "b985cdd5-7a12-4627-972f-491df6dfbd2e";
pub const STAGE_ACTIVE_CREATOR_FREE: &str = "b6214d99-9183-40d4-aa0e-ae41b301d217";
pub const STAGE_CHECKOUT_STARTED: &str = "007d26e6-a269-494e-8952-ac3201e81f0d";
pub const STAGE_PAID_SINGLE_PASS: &str = "78e16400-4b79-43db-9fda-82fd3b04f3c2";
pub const STAGE_ACTIVE_SUBSCRIBER: &str = "189f3036-2659-4d15-9dcc-0c16f8c5ffc6";
pub const STAGE_REFUNDED_CHURNED: &str = "1c3517fe-450c-4ce4-bed0-593252e9ff21";

/// Published HighLevel Workflow IDs in `TnDcaRLbCxm7LnOUF173`
pub const WF1_FREE_CREATOR_NURTURE_ID: &str = "355b939e-3480-4a06-b899-48e0f4f2e4e1";
pub const WF2_ABANDONED_CHECKOUT_ID: &str = "4de3d6d7-8d8f-4886-b0f5-08ae9944c02d";
pub const WF3_PAID_ONBOARDING_ID: &str = "bb45a7bc-bc2f-48bb-8c3d-09c7035cc384";
pub const WF4_HIGH_VALUE_B2B_ID: &str = "ee27eb6f-8372-48ad-9ad8-bed9b4e6f16d";
pub const WF5_REFUND_CHURN_ID: &str = "2f5e14dd-af14-4e55-a3c8-f4174b63e4ce";

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
    #[serde(default)]
    pub workflows_enrolled: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline_stage_id: Option<String>,
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

    pub fn is_configured(&self) -> bool {
        !self.token.trim().is_empty()
    }

    pub fn has_env_or_pit_file() -> bool {
        if std::env::var("GHL_LOCATION_PIT")
            .or_else(|_| std::env::var("HIGHLEVEL_API_KEY"))
            .or_else(|_| std::env::var("PRESSMARK_GHL_PIT"))
            .map(|v| !v.trim().is_empty())
            .unwrap_or(false)
        {
            return true;
        }
        let location_id = std::env::var("GHL_LOCATION_ID")
            .or_else(|_| std::env::var("PRESSMARK_GHL_LOCATION_ID"))
            .or_else(|_| std::env::var("HIGHLEVEL_LOCATION_ID"))
            .unwrap_or_else(|_| PRESSMARK_LOCATION_ID.to_string());
        let pit_path = format!(
            "{}/.config/ghl-agency/contexts/moosh-growth/locations/{}.pit",
            std::env::var("HOME").unwrap_or_default(),
            location_id
        );
        std::path::Path::new(&pit_path).exists()
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
        if self.token == "dry_run" {
            return Ok(GhlDispatchReceipt {
                success: true,
                contact_id: Some("dry_run_contact".to_string()),
                tags_applied: payload.tags.clone(),
                workflows_enrolled: Vec::new(),
                pipeline_stage_id: None,
                message: "Dry run contact payload verified".to_string(),
            });
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
            workflows_enrolled: Vec::new(),
            pipeline_stage_id: None,
            message: "Successfully synchronized contact to HighLevel CRM".to_string(),
        })
    }

    fn auth_headers(&self) -> Result<HeaderMap, String> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))
                .map_err(|e| format!("Invalid auth header: {}", e))?,
        );
        headers.insert("Version", HeaderValue::from_static("2021-07-28"));
        Ok(headers)
    }

    /// Deterministically enroll a contact into a published HighLevel workflow via Public API v2.
    pub async fn enroll_in_workflow(&self, contact_id: &str, workflow_id: &str) -> Result<(), String> {
        if self.token == "dry_run" {
            return Ok(());
        }
        let headers = self.auth_headers()?;
        let url = format!(
            "https://services.leadconnectorhq.com/contacts/{}/workflow/{}",
            contact_id, workflow_id
        );
        let resp = self
            .client
            .post(&url)
            .headers(headers)
            .json(&json!({}))
            .send()
            .await
            .map_err(|e| format!("GHL workflow enroll error: {}", e))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("GHL workflow enroll failed ({}): {}", status, text));
        }
        Ok(())
    }

    /// Remove a contact from an active HighLevel workflow via Public API v2.
    pub async fn remove_from_workflow(&self, contact_id: &str, workflow_id: &str) -> Result<(), String> {
        if self.token == "dry_run" {
            return Ok(());
        }
        let headers = self.auth_headers()?;
        let url = format!(
            "https://services.leadconnectorhq.com/contacts/{}/workflow/{}",
            contact_id, workflow_id
        );
        let _ = self
            .client
            .delete(&url)
            .headers(headers)
            .json(&json!({}))
            .send()
            .await;
        Ok(())
    }

    /// Upsert contact's pipeline opportunity stage in `Pressmark Conversion & Subscriptions`.
    pub async fn upsert_opportunity(
        &self,
        contact_id: &str,
        stage_id: &str,
        name: &str,
        status: &str,
        monetary_value: Option<f64>,
    ) -> Result<(), String> {
        if self.token == "dry_run" {
            return Ok(());
        }
        let headers = self.auth_headers()?;
        let mut body = json!({
            "pipelineId": PRESSMARK_PIPELINE_ID,
            "locationId": self.location_id,
            "contactId": contact_id,
            "pipelineStageId": stage_id,
            "name": name,
            "status": status,
        });
        if let Some(val) = monetary_value {
            body["monetaryValue"] = json!(val);
        }
        let resp = self
            .client
            .post("https://services.leadconnectorhq.com/opportunities/upsert")
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("GHL opportunity upsert error: {}", e))?;
        if !resp.status().is_success() {
            let st = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("GHL opportunity upsert failed ({}): {}", st, text));
        }
        Ok(())
    }

    /// Record document generation event in HighLevel CRM with archetype tags, pipeline progression, and workflow enrollment.
    pub async fn dispatch_document_generation(
        &self,
        doc: &DocumentRequest,
        user_email: Option<&str>,
        channel: &str,
    ) -> Result<GhlDispatchReceipt, String> {
        let archetype_slug = doc.doc_type.as_str();
        let archetype_title = doc.doc_type.display_title();
        let total_val = doc.grand_total();

        let mut tags = vec![
            format!("pressmark:channel:{}", channel),
            format!("pressmark:doc:{}-created", archetype_slug),
            "pressmark:lead:active-creator".to_string(),
        ];

        let is_watermarked = doc
            .watermark
            .as_deref()
            .map(|w| {
                let t = w.trim();
                !t.is_empty() && !t.eq_ignore_ascii_case("false") && !t.eq_ignore_ascii_case("none")
            })
            .unwrap_or(false);
        if is_watermarked {
            tags.push("pressmark:status:watermarked".to_string());
        } else {
            tags.push("pressmark:status:unwatermarked".to_string());
        }

        if !doc.signatures.is_empty() {
            tags.push("pressmark:feature:digital-signatures".to_string());
        }
        let comp = doc.normalized_compliance_mode();
        if comp != "none" && !comp.is_empty() {
            tags.push(format!("pressmark:compliance:{}", comp));
        }
        let is_high_value_b2b = total_val >= 10_000.0
            || matches!(
                doc.doc_type,
                crate::models::DocumentType::CommercialNnnLease
                    | crate::models::DocumentType::ClosingSettlementStatement
                    | crate::models::DocumentType::RentRoll
                    | crate::models::DocumentType::CommercialLease
                    | crate::models::DocumentType::Agreement
                    | crate::models::DocumentType::AgreementSow
                    | crate::models::DocumentType::ScheduleOfRealEstateOwned
                    | crate::models::DocumentType::RealEstateDealAnalysis
                    | crate::models::DocumentType::RealEstatePurchaseAgreement
                    | crate::models::DocumentType::PropertyDeed
                    | crate::models::DocumentType::PropertyManagementAgreement
                    | crate::models::DocumentType::PropertyManagementStatement
                    | crate::models::DocumentType::RealEstateCommission
                    | crate::models::DocumentType::LetterOfIntentCre
                    | crate::models::DocumentType::TenantEstoppel
                    | crate::models::DocumentType::MechanicsLienWaiver
            );
        if is_high_value_b2b {
            tags.push("pressmark:segment:high-value-b2b".to_string());
        }

        let mut custom_fields = HashMap::new();
        custom_fields.insert("pressmark_document_number".to_string(), json!(doc.effective_doc_number()));
        custom_fields.insert("pressmark_document_title".to_string(), json!(doc.title.as_deref().unwrap_or(archetype_title)));
        custom_fields.insert("pressmark_document_archetype".to_string(), json!(archetype_title));
        custom_fields.insert("pressmark_total_amount".to_string(), json!(total_val));
        custom_fields.insert("pressmark_currency".to_string(), json!(doc.normalized_currency_code()));
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

        let mut receipt = self.upsert_contact(&payload).await?;
        if let Some(ref cid) = receipt.contact_id {
            if is_watermarked {
                let opp_name = format!("{} — Free Creator", doc.from.name);
                let _ = self
                    .upsert_opportunity(cid, STAGE_ACTIVE_CREATOR_FREE, &opp_name, "open", Some(2.99))
                    .await;
                receipt.pipeline_stage_id = Some(STAGE_ACTIVE_CREATOR_FREE.to_string());
                let _ = self.enroll_in_workflow(cid, WF1_FREE_CREATOR_NURTURE_ID).await;
                receipt
                    .workflows_enrolled
                    .push(WF1_FREE_CREATOR_NURTURE_ID.to_string());
            }
            if is_high_value_b2b {
                let _ = self.enroll_in_workflow(cid, WF4_HIGH_VALUE_B2B_ID).await;
                receipt
                    .workflows_enrolled
                    .push(WF4_HIGH_VALUE_B2B_ID.to_string());
            }
        }
        Ok(receipt)
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

        let is_subscriber = tier_or_pass.contains("sub_");
        if is_subscriber {
            tags.push("pressmark:status:subscriber".to_string());
        } else {
            tags.push("pressmark:status:microtransaction-pass".to_string());
        }

        let amount_dollars = amount_cents as f64 / 100.0;
        let mut custom_fields = HashMap::new();
        custom_fields.insert("pressmark_document_number".to_string(), json!(doc_number));
        custom_fields.insert("pressmark_subscription_tier".to_string(), json!(tier_or_pass));
        custom_fields.insert("pressmark_total_amount".to_string(), json!(amount_dollars));
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

        let mut receipt = self.upsert_contact(&payload).await?;
        if let Some(ref cid) = receipt.contact_id {
            let _ = self.remove_from_workflow(cid, WF1_FREE_CREATOR_NURTURE_ID).await;
            let _ = self.remove_from_workflow(cid, WF2_ABANDONED_CHECKOUT_ID).await;
            let stage_id = if is_subscriber {
                STAGE_ACTIVE_SUBSCRIBER
            } else {
                STAGE_PAID_SINGLE_PASS
            };
            let opp_name = format!("{} — {}", user_email, tier_or_pass);
            let _ = self
                .upsert_opportunity(cid, stage_id, &opp_name, "won", Some(amount_dollars))
                .await;
            receipt.pipeline_stage_id = Some(stage_id.to_string());
            let _ = self.enroll_in_workflow(cid, WF3_PAID_ONBOARDING_ID).await;
            receipt
                .workflows_enrolled
                .push(WF3_PAID_ONBOARDING_ID.to_string());
        }
        Ok(receipt)
    }

    /// Record an email workspace sign-in event in HighLevel CRM.
    pub async fn dispatch_workspace_signin(
        &self,
        user_email: &str,
        name: Option<&str>,
        channel: &str,
    ) -> Result<GhlDispatchReceipt, String> {
        let tags = vec![
            "pressmark:lead:workspace-signin".to_string(),
            format!("pressmark:channel:{}", channel),
        ];

        let mut custom_fields = HashMap::new();
        custom_fields.insert("pressmark_source_channel".to_string(), json!(channel));
        custom_fields.insert("pressmark_last_action".to_string(), json!("workspace_signin"));

        let payload = GhlContactPayload {
            email: Some(user_email.to_string()),
            phone: None,
            name: name.map(|s| s.to_string()),
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

        let mut receipt = self.upsert_contact(&payload).await?;
        if let Some(ref cid) = receipt.contact_id {
            let opp_name = format!("{} — Workspace Lead", name.unwrap_or(user_email));
            let _ = self
                .upsert_opportunity(cid, STAGE_WORKSPACE_SIGNED_IN, &opp_name, "open", Some(2.99))
                .await;
            receipt.pipeline_stage_id = Some(STAGE_WORKSPACE_SIGNED_IN.to_string());
            let _ = self.enroll_in_workflow(cid, WF1_FREE_CREATOR_NURTURE_ID).await;
            receipt
                .workflows_enrolled
                .push(WF1_FREE_CREATOR_NURTURE_ID.to_string());
        }
        Ok(receipt)
    }

    /// Record a checkout initiation event in HighLevel CRM for abandoned-cart recovery.
    pub async fn dispatch_checkout_started(
        &self,
        user_email: &str,
        plan_id: &str,
        stripe_price_id: &str,
        price_cents: u32,
        channel: &str,
    ) -> Result<GhlDispatchReceipt, String> {
        let tags = vec![
            "pressmark:funnel:checkout-started".to_string(),
            format!("pressmark:intent:{}", plan_id),
            format!("pressmark:channel:{}", channel),
        ];

        let amount_dollars = price_cents as f64 / 100.0;
        let mut custom_fields = HashMap::new();
        custom_fields.insert("pressmark_subscription_tier".to_string(), json!(plan_id));
        if !stripe_price_id.is_empty() {
            custom_fields.insert(
                "pressmark_stripe_price_id".to_string(),
                json!(stripe_price_id),
            );
        }
        custom_fields.insert("pressmark_total_amount".to_string(), json!(amount_dollars));
        custom_fields.insert("pressmark_source_channel".to_string(), json!(channel));
        custom_fields.insert("pressmark_last_action".to_string(), json!("checkout_started"));

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

        let mut receipt = self.upsert_contact(&payload).await?;
        if let Some(ref cid) = receipt.contact_id {
            let opp_name = format!("{} — Checkout ({})", user_email, plan_id);
            let _ = self
                .upsert_opportunity(cid, STAGE_CHECKOUT_STARTED, &opp_name, "open", Some(amount_dollars))
                .await;
            receipt.pipeline_stage_id = Some(STAGE_CHECKOUT_STARTED.to_string());
            let _ = self.enroll_in_workflow(cid, WF2_ABANDONED_CHECKOUT_ID).await;
            receipt
                .workflows_enrolled
                .push(WF2_ABANDONED_CHECKOUT_ID.to_string());
        }
        Ok(receipt)
    }

    /// Record a Stripe charge refund event in HighLevel CRM.
    pub async fn dispatch_charge_refunded(
        &self,
        user_email: &str,
        credit_note_number: &str,
        refund_cents: u64,
    ) -> Result<GhlDispatchReceipt, String> {
        let tags = vec![
            "pressmark:status:refunded".to_string(),
            "pressmark:action:entitlement-revoked".to_string(),
        ];

        let mut custom_fields = HashMap::new();
        custom_fields.insert("pressmark_document_number".to_string(), json!(credit_note_number));
        custom_fields.insert("pressmark_subscription_tier".to_string(), json!("free"));
        custom_fields.insert("pressmark_total_amount".to_string(), json!(refund_cents as f64 / 100.0));
        custom_fields.insert("pressmark_last_action".to_string(), json!("charge_refunded"));

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

        let mut receipt = self.upsert_contact(&payload).await?;
        if let Some(ref cid) = receipt.contact_id {
            let _ = self.remove_from_workflow(cid, WF3_PAID_ONBOARDING_ID).await;
            let opp_name = format!("{} — Refunded ({})", user_email, credit_note_number);
            let _ = self
                .upsert_opportunity(cid, STAGE_REFUNDED_CHURNED, &opp_name, "lost", Some(0.0))
                .await;
            receipt.pipeline_stage_id = Some(STAGE_REFUNDED_CHURNED.to_string());
            let _ = self.enroll_in_workflow(cid, WF5_REFUND_CHURN_ID).await;
            receipt
                .workflows_enrolled
                .push(WF5_REFUND_CHURN_ID.to_string());
        }
        Ok(receipt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DocumentRequest;

    fn test_dispatcher() -> GhlDispatcher {
        GhlDispatcher::new(PRESSMARK_LOCATION_ID.to_string(), "dry_run".to_string())
    }

    #[tokio::test]
    async fn test_ghl_high_value_b2b_and_lifecycle_dispatches() {
        let ghl = test_dispatcher();

        let signin_res = ghl
            .dispatch_workspace_signin("founder@acme.io", Some("Jane Founder"), "web_studio")
            .await
            .expect("signin dry run should succeed");
        assert!(signin_res
            .tags_applied
            .contains(&"pressmark:lead:workspace-signin".to_string()));
        assert_eq!(
            signin_res.pipeline_stage_id.as_deref(),
            Some(STAGE_WORKSPACE_SIGNED_IN)
        );
        assert_eq!(
            signin_res.workflows_enrolled,
            vec![WF1_FREE_CREATOR_NURTURE_ID.to_string()]
        );

        let checkout_res = ghl
            .dispatch_checkout_started(
                "founder@acme.io",
                "pass_single_unlock",
                "price_1TOnY92c2efmg5sE3R8v6aN0",
                299,
                "web_checkout",
            )
            .await
            .expect("checkout dry run should succeed");
        assert!(checkout_res
            .tags_applied
            .contains(&"pressmark:funnel:checkout-started".to_string()));
        assert!(checkout_res
            .tags_applied
            .contains(&"pressmark:intent:pass_single_unlock".to_string()));
        assert_eq!(
            checkout_res.pipeline_stage_id.as_deref(),
            Some(STAGE_CHECKOUT_STARTED)
        );
        assert_eq!(
            checkout_res.workflows_enrolled,
            vec![WF2_ABANDONED_CHECKOUT_ID.to_string()]
        );

        let mut doc: DocumentRequest = serde_json::from_value(json!({
            "doc_type": "commercial_lease",
            "doc_number": "NNN-2026-001",
            "title": "Triple Net Commercial Lease",
            "watermark": "PRESSMARK PREVIEW",
            "from": {
                "name": "Apex Commercial Holdings",
                "company": "Apex REIT",
                "email": "leasing@apexreit.com"
            },
            "to": {
                "name": "Tenant Retail Corp",
                "email": "ap@tenantretail.com"
            },
            "items": [
                {
                    "description": "Monthly NNN Base Rent",
                    "quantity": 1.0,
                    "unit_price": 8500.0
                }
            ]
        }))
        .expect("valid DocumentRequest json");

        let gen_res = ghl
            .dispatch_document_generation(&doc, Some("leasing@apexreit.com"), "rest_api")
            .await
            .expect("doc generation dry run should succeed");
        assert!(gen_res
            .tags_applied
            .contains(&"pressmark:segment:high-value-b2b".to_string()));
        assert!(gen_res
            .tags_applied
            .contains(&"pressmark:status:watermarked".to_string()));
        assert_eq!(
            gen_res.pipeline_stage_id.as_deref(),
            Some(STAGE_ACTIVE_CREATOR_FREE)
        );
        assert_eq!(
            gen_res.workflows_enrolled,
            vec![
                WF1_FREE_CREATOR_NURTURE_ID.to_string(),
                WF4_HIGH_VALUE_B2B_ID.to_string()
            ]
        );

        doc.watermark = None;
        let unwatermarked_res = ghl
            .dispatch_document_generation(&doc, Some("leasing@apexreit.com"), "mcp_agent")
            .await
            .expect("unwatermarked doc dry run should succeed");
        assert!(unwatermarked_res
            .tags_applied
            .contains(&"pressmark:status:unwatermarked".to_string()));
        assert_eq!(
            unwatermarked_res.workflows_enrolled,
            vec![WF4_HIGH_VALUE_B2B_ID.to_string()]
        );

        let unlock_res = ghl
            .dispatch_payment_unlock("NNN-2026-001", "founder@acme.io", "sub_pro_monthly", 1500)
            .await
            .expect("payment unlock dry run should succeed");
        assert_eq!(
            unlock_res.pipeline_stage_id.as_deref(),
            Some(STAGE_ACTIVE_SUBSCRIBER)
        );
        assert_eq!(
            unlock_res.workflows_enrolled,
            vec![WF3_PAID_ONBOARDING_ID.to_string()]
        );

        let refund_res = ghl
            .dispatch_charge_refunded("founder@acme.io", "CN-2026-009", 1500)
            .await
            .expect("refund dry run should succeed");
        assert!(refund_res
            .tags_applied
            .contains(&"pressmark:status:refunded".to_string()));
        assert!(refund_res
            .tags_applied
            .contains(&"pressmark:action:entitlement-revoked".to_string()));
        assert_eq!(
            refund_res.pipeline_stage_id.as_deref(),
            Some(STAGE_REFUNDED_CHURNED)
        );
        assert_eq!(
            refund_res.workflows_enrolled,
            vec![WF5_REFUND_CHURN_ID.to_string()]
        );
    }
}
