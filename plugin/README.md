# Pressmark

Author: Daniel Hallman

Create PDF invoices, estimates, receipts, contracts, and real estate documents. Preview the layout and check the names, dates, amounts, and terms before downloading the PDF.

## Quick start

1. Open [Pressmark Studio](https://pressmark.dev) to edit a document in your browser.
2. For agent use, install this package in a compatible host or connect `https://pressmark.dev/mcp` using Streamable HTTP. Guest generation requires no credentials. Installing a skill alone does not connect the server.
3. Confirm that the client lists `preview_document`, `generate_document`, `calculate_totals`, `convert_document`, `record_payment`, `submit_feedback`, and `get_feedback_status`.
4. Ask for a document, check its preview and totals, then download the PDF. Use a guest session and synthetic contact details for a demo.

When customer sync is enabled, generating a PDF can record contact details and a document summary and start automated follow-up. It uses the signed-in email when available, otherwise the sender email, then the recipient email. The [privacy policy](https://pressmark.dev/privacy) lists the fields shared and records retained.

Pressmark does not show its plans or start checkout in chat. For plan details, open the [pricing page](https://pressmark.dev/pricing) in a browser. Recording a payment prepares a document from the supplied details; it does not move money or verify a transaction.

Version **1.3.0**. See the [release notes](https://github.com/mooshee/pressmark/releases/tag/v1.3.0) and [installation instructions](https://github.com/mooshee/pressmark#use-pressmark-with-an-agent). Update or reinstall the package, then refresh the connection's tool list. The server address is unchanged.

Built by [Moosh Works](https://mooshworks.com).
