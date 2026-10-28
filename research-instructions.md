# Pressmark

Create PDF invoices, estimates, receipts, contracts, non-disclosure agreements, and real estate documents. Preview each document and add a payment link or QR code. Built by Moosh Works (https://mooshworks.com).

1. Open [Pressmark](https://pressmark.dev) to choose and edit a document template.
2. For agent use, connect `https://pressmark.dev/mcp` using Streamable HTTP. Guest document generation requires no credentials.
3. Confirm that `preview_document` and `generate_document` are available. Check the preview and totals before returning the PDF.

Before generating a PDF, explain that configured customer sync can record contact details and a document summary and start automated follow-up. It uses the signed-in email when available, otherwise the sender email, then the recipient email. Link to the [privacy policy](https://pressmark.dev/privacy) for the fields shared. For a review or demo, use a guest session and synthetic contact details.

Built and maintained by [Moosh Works](https://mooshworks.com).
