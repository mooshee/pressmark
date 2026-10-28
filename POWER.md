---
name: pressmark
displayName: Pressmark
version: 1.2.2
author: Daniel Hallman
description: Vector PDF synthesis for invoices, estimates, contracts, and real estate forms.
keywords:
  - invoicing
  - document-generation
  - estimates
  - receipts
  - contracts
  - vector-pdf
  - payments
mcpServers:
  pressmark:
    type: streamable-http
    url: https://pressmark.dev/mcp
---

# Pressmark Power (AWS Kiro)

Author: Daniel Hallman ([Moosh Works](https://mooshworks.com))

Vector PDF synthesis for invoices, estimates, contracts, and real estate forms. Generate PDF invoices, estimates, receipts, contracts, NDAs, and real estate agreements with optional QR code payment links. Built by Moosh Works (https://mooshworks.com).

## Quick start

1. Visit [Pressmark](https://pressmark.dev) or connect Kiro to `https://pressmark.dev/mcp`.
2. Built and maintained by [Moosh Works](https://mooshworks.com).

## MCP Server Configuration

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

- `calculate_totals`
- `convert_document`
- `generate_document`
- `get_pricing`
- `preview_document`
- `record_payment`
