//! HighLevel (GHL) Subaccount Custom Field Deployment CLI for Pressmark
//!
//! Author: Daniel Hallman

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use std::fs;
use std::process::Command;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CustomFieldResponse {
    #[serde(default, rename = "customFields")]
    custom_fields: Vec<CustomFieldItem>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
struct CustomFieldItem {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default, rename = "fieldKey")]
    field_key: Option<String>,
    #[serde(default, rename = "dataType")]
    data_type: Option<String>,
}

struct FieldSpec {
    name: &'static str,
    data_type: &'static str,
}

const REQUIRED_FIELDS: &[FieldSpec] = &[
    FieldSpec { name: "Pressmark Document ID", data_type: "TEXT" },
    FieldSpec { name: "Pressmark Document Number", data_type: "TEXT" },
    FieldSpec { name: "Pressmark Document Title", data_type: "TEXT" },
    FieldSpec { name: "Pressmark Document Archetype", data_type: "TEXT" },
    FieldSpec { name: "Pressmark Total Amount", data_type: "MONETORY" },
    FieldSpec { name: "Pressmark Currency", data_type: "TEXT" },
    FieldSpec { name: "Pressmark PDF Download Link", data_type: "TEXT" },
    FieldSpec { name: "Pressmark Stripe Price ID", data_type: "TEXT" },
    FieldSpec { name: "Pressmark Subscription Tier", data_type: "TEXT" },
    FieldSpec { name: "Pressmark Last Action", data_type: "TEXT" },
    FieldSpec { name: "Pressmark Generation Count", data_type: "NUMERICAL" },
    FieldSpec { name: "Pressmark Source Channel", data_type: "TEXT" },
];

fn resolve_location_token(location_id: &str) -> Result<String, String> {
    if let Ok(tok) = std::env::var("GHL_LOCATION_PIT") {
        if !tok.trim().is_empty() {
            return Ok(tok.trim().to_string());
        }
    }

    // Try reading local PIT file
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

    // Headlessly resolve from 1Password via service account
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

    if !output.status.success() {
        return Err(format!(
            "op error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let token = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if token.is_empty() {
        return Err("Resolved token from 1Password was empty.".to_string());
    }
    Ok(token)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let dry_run = args.iter().any(|a| a == "--dry-run");
    let reconcile_contact_id = args
        .windows(2)
        .find(|w| w[0] == "--reconcile-free-creator")
        .map(|w| w[1].clone());

    let location_id = std::env::var("GHL_LOCATION_ID")
        .or_else(|_| std::env::var("PRESSMARK_GHL_LOCATION_ID"))
        .unwrap_or_else(|_| pressmark::ghl::PRESSMARK_LOCATION_ID.to_string());

    let token = resolve_location_token(&location_id)?;
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    headers.insert("Version", HeaderValue::from_static("2021-07-28"));

    if let Some(contact_id) = reconcile_contact_id {
        println!("============================================================");
        println!("Pressmark HighLevel Free-Creator Opportunity Reconciliation");
        println!("Target Location: {}", location_id);
        println!("Target Contact:  {}", contact_id);
        println!("Mode:            {}", if dry_run { "DRY-RUN" } else { "LIVE" });
        println!("============================================================");

        // 1. Read-back Contact
        let contact_url = format!("https://services.leadconnectorhq.com/contacts/{}", contact_id);
        let c_resp = client.get(&contact_url).headers(headers.clone()).send().await?;
        if !c_resp.status().is_success() {
            let err = c_resp.text().await?;
            return Err(format!("Failed to read contact {}: {}", contact_id, err).into());
        }
        let c_json: serde_json::Value = c_resp.json().await?;
        let contact = &c_json["contact"];
        let c_loc = contact["locationId"].as_str().unwrap_or("");
        if c_loc != location_id {
            return Err(format!(
                "Contact location mismatch: expected {}, got {}",
                location_id, c_loc
            )
            .into());
        }
        let c_email = contact["email"].as_str().unwrap_or("");
        let full_fl = match (
            contact["firstName"].as_str().filter(|s| !s.is_empty()),
            contact["lastName"].as_str().filter(|s| !s.is_empty()),
        ) {
            (Some(f), Some(l)) => Some(format!("{} {}", f, l)),
            (Some(f), None) => Some(f.to_string()),
            (None, Some(l)) => Some(l.to_string()),
            (None, None) => None,
        };
        let c_name = contact["contactName"]
            .as_str()
            .filter(|s| !s.is_empty())
            .or_else(|| contact["name"].as_str().filter(|s| !s.is_empty()))
            .or_else(|| full_fl.as_deref())
            .or_else(|| contact["companyName"].as_str().filter(|s| !s.is_empty()))
            .unwrap_or("Creator");
        println!("Verified Contact: {} <{}>", c_name, c_email);

        let opp_name = format!("{} — Free Creator", c_name);
        if dry_run {
            println!(
                "[DRY-RUN] Would upsert opportunity '{}' in stage {} ($2.99) and enroll in workflow {}",
                opp_name,
                pressmark::ghl::STAGE_ACTIVE_CREATOR_FREE,
                pressmark::ghl::WF1_FREE_CREATOR_NURTURE_ID
            );
            return Ok(());
        }

        let ghl = pressmark::ghl::GhlDispatcher::new(location_id.clone(), token.clone());
        // Add pressmark:status:watermarked tag and remove stale unwatermarked tag
        let _ = client
            .post(format!(
                "https://services.leadconnectorhq.com/contacts/{}/tags",
                contact_id
            ))
            .headers(headers.clone())
            .json(&json!({ "tags": ["pressmark:status:watermarked"] }))
            .send()
            .await?;
        let _ = client
            .delete(format!(
                "https://services.leadconnectorhq.com/contacts/{}/tags",
                contact_id
            ))
            .headers(headers.clone())
            .json(&json!({ "tags": ["pressmark:status:unwatermarked"] }))
            .send()
            .await?;

        ghl.upsert_opportunity(
            &contact_id,
            pressmark::ghl::STAGE_ACTIVE_CREATOR_FREE,
            &opp_name,
            "open",
            Some(2.99),
        )
        .await?;
        println!("✓ Upserted opportunity '{}' ($2.99) in stage {}", opp_name, pressmark::ghl::STAGE_ACTIVE_CREATOR_FREE);

        if let Err(e) = ghl
            .enroll_in_workflow(&contact_id, pressmark::ghl::WF1_FREE_CREATOR_NURTURE_ID)
            .await
        {
            println!("! Workflow enrollment note: {}", e);
        } else {
            println!("✓ Enrolled in workflow {}", pressmark::ghl::WF1_FREE_CREATOR_NURTURE_ID);
        }

        // Read-back verification of opportunity
        let search_url = format!(
            "https://services.leadconnectorhq.com/opportunities/search?location_id={}",
            location_id
        );
        let opp_resp = client.get(&search_url).headers(headers.clone()).send().await?;
        let opp_json: serde_json::Value = opp_resp.json().await?;
        let matching: Vec<&serde_json::Value> = opp_json["opportunities"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter(|o| o["contactId"].as_str() == Some(contact_id.as_str()))
                    .collect()
            })
            .unwrap_or_default();
        println!(
            "✓ Read-back verified {} opportunity record(s) for contact {}",
            matching.len(),
            contact_id
        );
        if let Some(first) = matching.first() {
            println!(
                "  Opportunity ID: {}, Name: {}, Stage: {}, Value: ${}",
                first["id"].as_str().unwrap_or(""),
                first["name"].as_str().unwrap_or(""),
                first["pipelineStageId"].as_str().unwrap_or(""),
                first["monetaryValue"]
            );
        }
        return Ok(());
    }

    println!("============================================================");
    println!("Pressmark HighLevel Subaccount Custom Fields Deployment");
    println!("Target Location: {}", location_id);
    println!("============================================================");

    // 1. Fetch Existing Custom Fields
    let list_url = format!(
        "https://services.leadconnectorhq.com/locations/{}/customFields",
        location_id
    );
    let resp = client.get(&list_url).headers(headers.clone()).send().await?;

    if !resp.status().is_success() {
        let err_text = resp.text().await?;
        eprintln!("Failed to fetch custom fields: {}", err_text);
        return Err("Custom fields query failed".into());
    }

    let parsed: serde_json::Value = resp.json().await?;
    let mut existing_names = HashSet::new();

    if let Some(arr) = parsed.get("customFields").and_then(|v| v.as_array()) {
        for item in arr {
            if let Some(n) = item.get("name").and_then(|v| v.as_str()) {
                existing_names.insert(n.to_lowercase());
            }
        }
    }

    println!("Discovered {} existing custom fields.", existing_names.len());

    // 2. Provision Missing Custom Fields
    let mut created_count = 0;
    let create_url = format!(
        "https://services.leadconnectorhq.com/locations/{}/customFields",
        location_id
    );

    for spec in REQUIRED_FIELDS {
        let name_lower = spec.name.to_lowercase();

        if existing_names.contains(&name_lower) {
            println!("  [EXISTS]  {} ({})", spec.name, spec.data_type);
            continue;
        }

        print!("  [CREATING] {} ({})... ", spec.name, spec.data_type);
        let body = json!({
            "name": spec.name,
            "dataType": spec.data_type,
            "model": "contact"
        });

        let post_resp = client
            .post(&create_url)
            .headers(headers.clone())
            .json(&body)
            .send()
            .await?;

        if post_resp.status().is_success() {
            println!("✓ OK");
            created_count += 1;
        } else {
            let err = post_resp.text().await?;
            println!("✗ FAILED: {}", err);
        }
    }

    println!("============================================================");
    println!("Deployment Complete: {} new custom fields created.", created_count);
    println!("============================================================");

    Ok(())
}
