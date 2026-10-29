# Pressmark

[![Rust](https://img.shields.io/badge/Rust-orange.svg)](https://www.rust-lang.org/)
[![Pressmark Engine](https://img.shields.io/badge/Pressmark%20Engine-v1.2.4-blue.svg)](https://pressmark.dev)
[![Pressmark MCP server quality and maintenance on Glama](https://glama.ai/mcp/servers/mooshee/pressmark/badges/score.svg)](https://glama.ai/mcp/servers/mooshee/pressmark)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Author: Daniel Hallman

Create PDF invoices, estimates, receipts, contracts, and real estate documents. Preview the layout, check the details, and download the PDF. You can add your logo, signature, payment link, and a QR code.

## Quick start

1. Open [Pressmark Studio](https://pressmark.dev).
2. Choose a template and enter the parties, dates, amounts, and terms.
3. Check the preview and totals.
4. Download the PDF or print it.

When customer sync is enabled, generating a PDF can update Pressmark's customer records and start automated follow-up. It uses the signed-in email when available, otherwise the sender email, then the recipient email. The [privacy policy](https://pressmark.dev/privacy) lists the contact details and document summary fields shared.

Package version **1.2.4**, released October 28, 2026. Read the [release notes](https://github.com/mooshee/pressmark/releases/tag/v1.2.4) for the updated instructions and data-use disclosures. The service version is tracked separately.

## Use Pressmark with an agent

Connect a client that supports the Model Context Protocol (MCP) to `https://pressmark.dev/mcp` using `streamable-http`. Guest document generation requires no credentials.

Choose the installation method for your client:

- **VS Code:** [Install Pressmark MCP in VS Code](https://insiders.vscode.dev/redirect/mcp/install?name=pressmark&config=%7B%22type%22%3A%22http%22%2C%22url%22%3A%22https%3A%2F%2Fpressmark.dev%2Fmcp%22%7D)
- **Claude Code:** `/plugin marketplace add mooshee/pressmark`, then `/plugin install pressmark@pressmark`
- **Gemini CLI:** `gemini extensions install https://github.com/mooshee/pressmark`
- **Devin CLI:** `devin plugins install mooshee/pressmark`
- **Vercel Skills:** `npx skills add mooshee/pressmark`
- **Homebrew:** `brew install mooshee/tap/pressmark`

Installing a skill alone does not connect the server. After connecting, confirm that your client lists these tools:

| Tool | What it does |
| --- | --- |
| `preview_document` | Shows a PNG preview. |
| `generate_document` | Creates a PDF from the supplied document. |
| `calculate_totals` | Calculates tax, discounts, totals, and balances. |
| `get_pricing` | Shows plans, document limits, and checkout links. |
| `convert_document` | Converts an estimate, invoice, receipt, or credit note. |
| `record_payment` | Prepares a receipt or updated invoice from payment details. It does not move money. |

Try: “Create an invoice for 15 hours of consulting at $250 per hour, due in 14 days. Show me a preview first.”

## Documents and editing

Pressmark includes templates for invoices, estimates, quotes, receipts, purchase orders, contracts, non-disclosure agreements, and real estate documents. See the [document type reference](skills/invoice-generation/references/archetypes.md) for common choices.

In the Studio, you can:

- Describe a document in the assistant bar.
- Upload an invoice or receipt image to extract its items and totals.
- Duplicate a document or import and export its JSON.
- Add a logo, draw a signature, and change the theme.
- Preview, print, or download the document.

For real estate work, use the [real estate skill](skills/real-estate-agreements/SKILL.md). Templates include leases, notices, rent rolls, property management statements, purchase agreements, and deeds. State profiles are available at `GET https://pressmark.dev/api/v1/states`. Check the applicable profile and the resulting text; a template or notice option does not certify legal compliance.

## Connect a Custom GPT action

1. In the GPT editor, select **Configure > Actions > Create new action**.
2. Import `https://pressmark.dev/api/v1/openapi.json`.
3. Choose **None** for guest authentication, or configure the API key header for your account.
4. Ask for a sample invoice and check that the action returns the requested document.

Integration endpoints:

- Plugin manifest: `GET https://pressmark.dev/.well-known/ai-plugin.json`
- OpenAPI specification: `GET https://pressmark.dev/api/v1/openapi.json`
- MCP: `POST https://pressmark.dev/mcp` or `POST https://pressmark.dev/api/v1/mcp`
- Tool catalog: `GET https://pressmark.dev/api/v1/mcp/tools.json`

## Pricing

| Plan | Price | Document limit | Included options |
| --- | --- | --- | --- |
| Free Community | $0 | 3 per month | Templates and exports with Pressmark attribution |
| Single Document Pass | $2.99 once | 1 document | Export without Pressmark attribution |
| Solo Pro | $15 per month | 50 per month | Custom logos, signatures, and exports without attribution |
| Business Studio | $49 per month | 250 per month | 300-PPI PNG previews, API keys, and priority processing |
| Agency & API Scale | $149 per month | 2,000 per month | Custom branding, batch generation, and webhooks |

Check [pricing](https://pressmark.dev/pricing) or `get_pricing` for the available plans and checkout links.

## Run locally

Install Rust and Cargo, the Typst CLI for PDF and PNG generation, and `cargo-nextest` for the test command below. Make `typst` available on `PATH`, or set `PRESSMARK_ENGINE_BIN` to its executable path.

Run:

```bash
git clone https://github.com/mooshee/pressmark.git
cd pressmark
cargo nextest run
cargo run --release
```

Open `http://localhost:3842` to check that the Studio loads.

See [usage measurement](docs/measurement.md) for analytics definitions, historical-data limits, and verification steps.

Built and maintained by [Moosh Works](https://mooshworks.com).
