---
name: pressmark
displayName: Pressmark Vector Document Engine
version: 1.2.1
author: Daniel Hallman
description: High-performance vector PDF document synthesis engine for 57 invoice, estimate, receipt, contract, NDA, and 50-state statutory real estate archetypes in sub-25ms.
keywords:
  - invoice
  - pdf
  - estimate
  - receipt
  - contract
  - real-estate
  - lease
  - mcp
mcpServers:
  pressmark:
    type: streamable-http
    url: https://pressmark.dev/mcp
---

# Pressmark Power (AWS Kiro)

Author: Daniel Hallman

Pressmark synthesizes vector PDF invoices, estimates, receipts, credit notes, contracts, NDAs, and 29 real estate and 50-state statutory lease archetypes in under 25ms.

## MCP Server Configuration

Connect Kiro to Pressmark's remote Streamable HTTP MCP server:

```json
{
  "mcpServers": {
    "pressmark": {
      "type": "streamable-http",
      "url": "https://pressmark.dev/mcp"
    }
  }
}
```

## Tools Available

- `generate_document`: Synthesize a vector PDF across 57 document archetypes with optional 50-state + DC statutory lease riders and SVG QR code payment links.
- `preview_document`: Render a 144-PPI PNG preview as a Base64 Data URI for immediate visual inspection.
- `calculate_totals`: Compute line-item subtotals, tax amounts, discounts, grand total, and balance due.
- `convert_document`: Statelessly convert an estimate or quote into a tax invoice, receipt, or credit note and synthesize its PDF.
- `record_payment`: Statelessly apply full or partial payment against an invoice and synthesize a paid receipt or updated balance-due invoice PDF.
- `get_pricing`: Retrieve subscription tiers and single-document unlock links.
