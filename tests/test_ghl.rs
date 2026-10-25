//! Integration Test for Pressmark HighLevel (GHL) CRM Dispatcher
//!
//! Author: Daniel Hallman

use pressmark::ghl::{GhlContactPayload, GhlDispatcher, PRESSMARK_LOCATION_ID};
use pressmark::models::{DocumentRequest, LineItem, PartyDetails};
use serde_json::json;
use std::collections::HashMap;

#[tokio::test]
async fn test_ghl_dispatcher_initialization_and_live_sync() {
    let dispatcher = GhlDispatcher::from_env();
    assert_eq!(dispatcher.location_id, PRESSMARK_LOCATION_ID);

    // If no token is present in the environment or 1Password SA, gracefully skip live HTTP call
    if dispatcher.token.is_empty() {
        println!("Skipping live GHL test: No Location PIT resolved.");
        return;
    }

    let mut custom_fields = HashMap::new();
    custom_fields.insert("pressmark_document_number".to_string(), json!("TEST-DOC-999"));
    custom_fields.insert("pressmark_document_title".to_string(), json!("Integration Test Invoice"));
    custom_fields.insert("pressmark_total_amount".to_string(), json!(750.0));
    custom_fields.insert("pressmark_currency".to_string(), json!("USD"));
    custom_fields.insert("pressmark_source_channel".to_string(), json!("integration-test"));

    let payload = GhlContactPayload {
        email: Some("integration-test@pressmark.dev".to_string()),
        phone: Some("+14238222232".to_string()),
        name: Some("Pressmark Test Runner".to_string()),
        first_name: Some("Pressmark".to_string()),
        last_name: Some("Tester".to_string()),
        company_name: Some("Pressmark Studio Lab".to_string()),
        address1: Some("100 Market St".to_string()),
        city: Some("Chattanooga".to_string()),
        state: Some("TN".to_string()),
        postal_code: Some("37402".to_string()),
        tags: vec![
            "pressmark:lead:test-runner".to_string(),
            "pressmark:channel:integration-test".to_string(),
            "pressmark:status:verified".to_string(),
        ],
        custom_fields,
    };

    let result = dispatcher.upsert_contact(&payload).await;
    assert!(result.is_ok(), "GHL Contact Upsert failed: {:?}", result.err());

    let receipt = result.unwrap();
    assert!(receipt.success);
    assert!(receipt.contact_id.is_some());
    println!("Successfully synced contact to GHL! Contact ID: {:?}", receipt.contact_id);

    // Also test dispatch_document_generation
    let mut doc = DocumentRequest::default();
    doc.title = Some("Test Contract".to_string());
    doc.doc_number = Some("TEST-DOC-1000".to_string());
    doc.issue_date = Some("2026-10-01".to_string());
    doc.watermark = Some("PRESSMARK FREE TIER".to_string());
    doc.doc_type = pressmark::models::DocumentType::Agreement;
    doc.from = PartyDetails {
        name: "Pressmark Test Corp".to_string(),
        company: Some("Pressmark Corp".to_string()),
        address: Some("100 Innovation Way, Chattanooga, TN 37402".to_string()),
        phone: Some("+14238222232".to_string()),
        email: Some("integration-test@pressmark.dev".to_string()),
        tax_id: None,
        website: None,
        registration_number: None,
    };
    doc.items = vec![LineItem {
        description: "Test Vector Document".to_string(),
        quantity: 1.0,
        unit_price: 150.0,
        tax_rate: None,
        discount: None,
    }];

    let doc_res = dispatcher.dispatch_document_generation(&doc, None, "integration-test").await;
    assert!(doc_res.is_ok(), "Dispatch document failed: {:?}", doc_res.err());
    let doc_receipt = doc_res.unwrap();
    assert_eq!(doc_receipt.workflows_enrolled.len(), 2);

    if let Some(ref cid) = doc_receipt.contact_id {
        let opp_res = dispatcher
            .upsert_opportunity(
                cid,
                pressmark::ghl::STAGE_ACTIVE_CREATOR_FREE,
                "Pressmark Test Corp — Free Creator",
                "open",
                Some(2.99),
            )
            .await;
        assert!(opp_res.is_ok(), "Opportunity upsert failed: {:?}", opp_res.err());

        let wf_res = dispatcher
            .enroll_in_workflow(cid, pressmark::ghl::WF1_FREE_CREATOR_NURTURE_ID)
            .await;
        assert!(wf_res.is_ok(), "Workflow enroll failed: {:?}", wf_res.err());

        // Clean up test contact from active workflows so test runs do not send delayed nurture emails
        let _ = dispatcher
            .remove_from_workflow(cid, pressmark::ghl::WF1_FREE_CREATOR_NURTURE_ID)
            .await;
        let _ = dispatcher
            .remove_from_workflow(cid, pressmark::ghl::WF4_HIGH_VALUE_B2B_ID)
            .await;
    }
}
