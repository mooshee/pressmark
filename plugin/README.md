# Pressmark

Author: Daniel Hallman

Create PDF invoices, estimates, receipts, contracts, and real estate documents. Preview the layout and check the names, dates, amounts, and terms before downloading the PDF.

## Quick start

1. Open [Pressmark Studio](https://pressmark.dev) to edit a document in your browser.
2. For agent use, install this package in a compatible host or connect `https://pressmark.dev/mcp` using Streamable HTTP. Guest generation requires no credentials. Installing a skill alone does not connect the server.
3. Confirm that the client lists `preview_document`, `generate_document`, `calculate_totals`, `get_pricing`, `convert_document`, and `record_payment`.
4. Ask for a document, check its preview and totals, then download the PDF. Use a guest session and synthetic contact details for a demo.

When customer sync is enabled, generating a PDF can record contact details and a document summary and start automated follow-up. It uses the signed-in email when available, otherwise the sender email, then the recipient email. The [privacy policy](https://pressmark.dev/privacy) lists the fields shared and records retained.

Use `get_pricing` for current prices and document limits before choosing a paid export. Recording a payment prepares a document from the supplied details; it does not move money.

Version **1.2.4**, October 28, 2026. See the [release notes](https://github.com/mooshee/pressmark/releases/tag/v1.2.4) and [installation instructions](https://github.com/mooshee/pressmark#use-pressmark-with-an-agent).

Built by [Moosh Works](https://mooshworks.com).
