# Usage measurement

Author: Daniel Hallman

`GET /api/v1/analytics` returns aggregates from the retained SQLite usage table. Measurement schema version 2 separates tool attempts, recorded outcomes and caller hints. It does not report unique users or customers.

For MCP traffic:

- A parsed `tools/call` request is recorded as pending before execution. A successful tool result completes it as success. JSON-RPC errors and results with `isError: true` complete it as failure.
- A pending attempt has no recorded completion. It can represent work in progress, interrupted work or a failed outcome write; it is not counted as success or failure.
- Calls retained before outcome instrumentation have an unknown outcome. The migration does not infer or backfill their results.
- Discovery, initialization and other protocol requests remain separate from tool attempts. Invalid HTTP or JSON bodies that never reach the MCP handler are outside this measurement.
- Public tool labels use the canonical tool registry. Unknown or missing names use fixed buckets; arbitrary client input is not exposed as a metric key.

The `mcp_measurement` object includes outcomes, per-tool and per-actor counts, the instrumentation start time, query completeness and process-local write-failure counters. Those failure counters reset on service restart. `mcp_tool_calls` retains its original meaning: attempted calls by tool, including calls whose outcome is unknown.

`mcp_measurement.failure_breakdown` separates rejected requests from execution errors using recorded JSON-RPC codes:

- `unknown_tool`: code `-32601`.
- `invalid_arguments`: code `-32602`.
- `execution_error`: codes `-32000` and `-32603`.
- `unclassified`: a missing or other code, including tool-error results without a recorded code.

These buckets sum to the failure count. Pending and historical unknown outcomes stay separate. An unknown tool or invalid argument does not establish a service outage. If the MCP query is incomplete, the breakdown is `null`.

The mutually exclusive actor buckets are `internal_probe`, `crawler_bot` and `unclassified`. They use caller-supplied user-agent hints, not verified identity. Generic command-line clients, browsers and agent clients remain unclassified. `external_organic_events` was removed because the service has no evidence that establishes that classification. `harness_client_events` is a separate, overlapping descriptor. `distinct_mcp_channels` counts different client strings, not people.

`retained_usage_window` reports the earliest and latest retained event and whether storage is durable. The normal retention limit is 90 days. These aggregates cover all retained rows; they are not a complete 7-day or 30-day report. The API does not infer complete coverage from the age of the oldest row.

Session rows, entitlements and paid entitlements remain separate counts. Paid entitlements require a positive recorded amount; complimentary access does not count as paid. None of these totals establishes an attributable customer count.

Use `checkout_measurement.recorded_starts` for deduplicated explicit checkout actions. It counts the durable `checkout_starts` records, including test actions. It does not count completed purchases or unique people. Repeating the same request does not add another start.

`checkout_measurement.initialization_events` preserves the retained `checkout_init` event count. Older versions recorded automatic requests and probes in that counter, so it must not be reported as explicit checkout starts. For example, 47 initialization events with zero recorded starts do not establish 47 people starting checkout. The start records are not pruned with the 90-day usage events; do not subtract or divide these counts to infer a historical funnel. If either checkout query fails, `query_complete` is false and both counts are `null`.

For operator verification, send `User-Agent: Pressmark-Measurement-QA/1.0`. A `get_pricing` call and an intentionally unknown tool call verify success and failure recording without creating a document, checkout or payment. Read back the `internal_probe` outcome delta and preserve the before/after snapshots. Such calls are verification traffic, not adoption.
