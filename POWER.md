---
name: pressmark
displayName: Pressmark
version: 1.2.4
author: Daniel Hallman
description: Create PDF invoices, estimates, contracts, and real estate documents.
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

Create PDF invoices, estimates, receipts, contracts, non-disclosure agreements, and real estate documents. Preview each document and add a payment link or QR code. Built by Moosh Works (https://mooshworks.com).

## Quick start

1. Open [Pressmark](https://pressmark.dev) to edit a document in your browser.
2. For Kiro, connect `https://pressmark.dev/mcp` using the configuration below.
3. Confirm that Kiro lists `preview_document` and `generate_document`, then ask for a sample invoice and check its preview.

When customer sync is enabled, generating a PDF can record contact details and a document summary and start automated follow-up. It uses the signed-in email when available, otherwise the sender email, then the recipient email. Read the [privacy policy](https://pressmark.dev/privacy) before sending document details. Use a guest session and synthetic contact details for a demo.

Built and maintained by [Moosh Works](https://mooshworks.com).

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
