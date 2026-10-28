//! MCP measurement regression coverage.
//! Author: Daniel Hallman

use axum::{body::Body, http::{Request, StatusCode}};
use pressmark::{server::{create_router, AppState}, storage::Storage};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceExt;

#[test]
fn tool_error_result_is_a_failure_and_unknown_metric_names_are_bounded() {
    use pressmark::mcp::{JsonRpcResponse, McpHandler};
    let storage = Storage::new(":memory:").unwrap();
    let response = JsonRpcResponse { jsonrpc: "2.0", id: Some(json!(1)), result: Some(json!({"isError": true, "content": []})), error: None };
    assert!(!response.tool_succeeded());
    let attempt = storage.begin_mcp_tool_attempt(None, "Pressmark-Measurement-QA/1.0", Some("secret-user-supplied-value@example.com")).unwrap();
    storage.finish_mcp_tool_attempt(attempt, response.tool_succeeded(), None, None);
    // Old arbitrary names also remain private when read through the public summary.
    storage.record_usage_event("mcp_tool_call", None, "curl/8.0", Some("private-legacy-name"), None);
    let summary = storage.analytics_summary();
    assert_eq!(summary["mcp_measurement"]["outcomes"]["failure"], 1);
    assert_eq!(summary["mcp_measurement"]["by_tool"]["unknown_tool"]["attempts"], 2);
    assert_eq!(summary["mcp_tool_calls"]["unknown_tool"], 2);
    assert!(!summary.to_string().contains("secret-user"));
    assert!(!summary.to_string().contains("private-legacy"));
    for tool in McpHandler::get_tools().as_array().unwrap() {
        let name = tool["name"].as_str().unwrap();
        assert_eq!(McpHandler::measurement_tool_name(Some(name)), name);
    }
}

#[test]
fn legacy_attempts_remain_unknown_across_migration_and_restart() {
    let path = std::env::temp_dir().join(format!("pressmark-measurement-{}.db", uuid::Uuid::new_v4()));
    let conn = rusqlite::Connection::open(&path).unwrap();
    conn.execute_batch("CREATE TABLE usage_events (id INTEGER PRIMARY KEY AUTOINCREMENT, event_type TEXT NOT NULL, archetype TEXT, channel TEXT NOT NULL, detail TEXT, compile_ms INTEGER, created_at TEXT NOT NULL);
        INSERT INTO usage_events (event_type, channel, detail, created_at) VALUES ('mcp_tool_call', 'curl/8.0', 'get_pricing', '2026-10-25T11:08:10+00:00');").unwrap();
    drop(conn);

    let storage = Storage::new(&path).unwrap();
    let first = storage.analytics_summary();
    assert_eq!(first["mcp_measurement"]["outcomes"]["unknown"], 1);
    assert_eq!(first["mcp_measurement"]["outcomes"]["success"], 0);
    assert_eq!(first["mcp_measurement"]["by_actor"]["unclassified"]["attempts"], 1);
    assert_eq!(first["retained_usage_window"]["first_event_at"], "2026-10-25T11:08:10+00:00");
    assert_eq!(first["retained_usage_window"]["full_window_coverage_established"], false);
    assert!(first.get("external_organic_events").is_none());
    assert!(first["attribution"]["customer_count"].is_null());
    assert!(first["attribution"]["unique_users"].is_null());

    let attempt = storage.begin_mcp_tool_attempt(None, "Pressmark-Measurement-QA/1.0", Some("get_pricing")).unwrap();
    drop(storage);
    let reopened = Storage::new(&path).unwrap();
    let pending = reopened.analytics_summary();
    assert_eq!(pending["mcp_measurement"]["outcomes_started_at"], first["mcp_measurement"]["outcomes_started_at"]);
    assert_eq!(pending["mcp_measurement"]["outcomes"]["pending"], 1);
    assert_eq!(pending["mcp_measurement"]["outcomes"]["unknown"], 1);
    reopened.finish_mcp_tool_attempt(attempt, false, Some(-32601), None);
    let final_summary = reopened.analytics_summary();
    assert_eq!(final_summary["mcp_measurement"]["outcomes"]["failure"], 1);
    assert_eq!(final_summary["mcp_measurement"]["outcomes"]["pending"], 0);
    let conn = rusqlite::Connection::open(&path).unwrap();
    let legacy: (String, String, Option<String>) = conn.query_row("SELECT channel, created_at, outcome FROM usage_events WHERE id = 1", [], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).unwrap();
    assert_eq!(legacy, ("curl/8.0".into(), "2026-10-25T11:08:10+00:00".into(), None));
    drop(conn);
    drop(reopened);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn generic_clients_are_unclassified_and_actor_buckets_do_not_overlap() {
    let storage = Storage::new(":memory:").unwrap();
    for client in ["curl/8.0", "xh/0.23", "Mozilla/5.0", "claude-code/1.0", "unknown", "", "uptime-client"] {
        assert_eq!(Storage::classify_usage_actor(client, None), "unclassified", "{client}");
        storage.record_usage_event("mcp_protocol", None, client, Some("tools/list"), None);
    }
    storage.record_usage_event("mcp_protocol", None, "Pressmark-Measurement-QA/1.0", Some("tools/list"), None);
    storage.record_usage_event("mcp_protocol", None, "Googlebot/2.1", Some("tools/list"), None);
    let summary = storage.analytics_summary();
    assert_eq!(summary["harness_client_events"], 1);
    assert_eq!(summary["actor_breakdown"].as_object().unwrap().values().map(|v| v.as_i64().unwrap()).sum::<i64>(), 9);
    assert_eq!(summary["unclassified_events"], 7);
    assert_eq!(summary["internal_probe_events"], 1);
    assert_eq!(summary["crawler_bot_events"], 1);
    assert_eq!(summary["mcp_measurement"]["attempts"], 0);
}

#[test]
fn write_failures_are_reported_without_claiming_success_and_free_access_is_not_paid() {
    let path = std::env::temp_dir().join(format!("pressmark-measurement-{}.db", uuid::Uuid::new_v4()));
    let storage = Storage::new(&path).unwrap();
    let conn = rusqlite::Connection::open(&path).unwrap();
    conn.execute_batch("CREATE TRIGGER fail_measurement_insert BEFORE INSERT ON usage_events BEGIN SELECT RAISE(ABORT, 'test failure'); END;").unwrap();
    assert!(storage.begin_mcp_tool_attempt(None, "Pressmark-Measurement-QA/1.0", Some("get_pricing")).is_none());
    conn.execute_batch("DROP TRIGGER fail_measurement_insert;").unwrap();
    let attempt = storage.begin_mcp_tool_attempt(None, "Pressmark-Measurement-QA/1.0", Some("get_pricing")).unwrap();
    conn.execute_batch("CREATE TRIGGER fail_measurement_update BEFORE UPDATE ON usage_events BEGIN SELECT RAISE(ABORT, 'test failure'); END;").unwrap();
    storage.finish_mcp_tool_attempt(attempt, true, None, None);
    storage.record_customer_entitlement("free@example.com", "complimentary", None, 0).unwrap();
    storage.record_customer_entitlement("paid@example.com", "paid", None, 1500).unwrap();
    let summary = storage.analytics_summary();
    assert_eq!(summary["mcp_measurement"]["attempt_write_failures_since_process_start"], 1);
    assert_eq!(summary["mcp_measurement"]["outcome_write_failures_since_process_start"], 1);
    assert_eq!(summary["mcp_measurement"]["outcomes"]["pending"], 1);
    assert_eq!(summary["mcp_measurement"]["outcomes"]["success"], 0);
    assert_eq!(summary["entitlements_count"], 2);
    assert_eq!(summary["paid_entitlements_count"], 1);
    drop(conn);
    drop(storage);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn coincident_timestamps_do_not_reclassify_historical_traffic() {
    let path = std::env::temp_dir().join(format!("pressmark-measurement-{}.db", uuid::Uuid::new_v4()));
    let storage = Storage::new(&path).unwrap();
    drop(storage);
    let conn = rusqlite::Connection::open(&path).unwrap();
    conn.execute_batch("INSERT INTO usage_events (event_type, channel, created_at) VALUES ('template_landing_view', 'web_seo', '2026-10-25T11:08:10+00:00'), ('mcp_protocol', 'HallmanLab-ProductDoctor/1.0', '2026-10-25T11:08:10+00:00');").unwrap();
    drop(conn);
    let storage = Storage::new(&path).unwrap();
    let summary = storage.analytics_summary();
    assert_eq!(summary["unclassified_events"], 1);
    assert_eq!(summary["internal_probe_events"], 1);
    drop(storage);
    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn http_tool_results_distinguish_success_failure_and_protocol_discovery() {
    let storage = Arc::new(Storage::new(":memory:").unwrap());
    let router = create_router(Arc::new(AppState::default().with_storage(storage.clone())));
    for (method, params, expected_error) in [
        ("tools/call", json!({"name": "get_pricing", "arguments": {}}), false),
        ("tools/call", json!({"name": "measurement_unknown_tool", "arguments": {}}), true),
        ("tools/call", json!({"name": "generate_document", "arguments": {}}), true),
        ("tools/list", json!({}), false),
    ] {
        let response = router.clone().oneshot(Request::builder().uri("/mcp").method("POST")
            .header("content-type", "application/json").header("user-agent", "Pressmark-Measurement-QA/1.0")
            .body(Body::from(json!({"jsonrpc": "2.0", "id": 1, "method": method, "params": params}).to_string())).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let response: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(response.get("error").is_some(), expected_error);
    }
    let summary = storage.analytics_summary();
    assert_eq!(summary["mcp_measurement"]["query_complete"], true);
    assert_eq!(summary["mcp_measurement"]["attempts"], 3);
    assert_eq!(summary["mcp_measurement"]["outcomes"], json!({"success": 1, "failure": 2, "pending": 0, "unknown": 0}));
    assert_eq!(summary["mcp_measurement"]["by_actor"]["internal_probe"]["attempts"], 3);
    assert_eq!(summary["mcp_measurement"]["by_tool"]["get_pricing"]["success"], 1);
    assert_eq!(summary["mcp_measurement"]["by_tool"]["generate_document"]["failure"], 1);
    assert_eq!(summary["events_by_type"]["mcp_protocol"], 1);
    assert_eq!(summary["mcp_measurement"]["attempt_write_failures_since_process_start"], 0);
    assert_eq!(summary["mcp_measurement"]["outcome_write_failures_since_process_start"], 0);
}
