# PRESSMARK · Sub-Millisecond Vector Document Engine & Interactive Studio

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![Pressmark Engine](https://img.shields.io/badge/Pressmark%20Engine-v1.2.1-blue.svg)](https://pressmark.dev)
[![Pressmark MCP server quality and maintenance on Glama](https://glama.ai/mcp/servers/mooshee/pressmark/badges/score.svg)](https://glama.ai/mcp/servers/mooshee/pressmark)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Author**: Daniel Hallman  
> **Brand**: [Pressmark Studio](https://pressmark.dev)  
> **Architecture**: Pure Rust (Tokio + Axum) + Sub-Millisecond Vector Synthesis Engine  

Pressmark is an ultra-fast, publication-grade document synthesis engine, real-time interactive design studio, and Model Context Protocol (MCP) / ChatGPT plugin server. Built to replace sluggish legacy document pipelines (Puppeteer/Headless Chrome, Python ReportLab, WeasyPrint), Pressmark compiles crisp, print-ready vector PDFs and high-resolution PNG previews in under **25 milliseconds** with zero disk churn and sub-cent unit economics.

## Quick start

1. Open [Pressmark Studio](https://pressmark.dev) to design, preview, and download vector PDFs in your browser.
2. Connect any MCP client to `https://pressmark.dev/mcp` (`streamable-http`), or install directly:
   - **VS Code (1-Click MCP)**: [Install Pressmark MCP in VS Code](https://insiders.vscode.dev/redirect/mcp/install?name=pressmark&config=%7B%22type%22%3A%22http%22%2C%22url%22%3A%22https%3A%2F%2Fpressmark.dev%2Fmcp%22%7D)
   - **Claude Code**: `/plugin marketplace add mooshee/pressmark` then `/plugin install pressmark@pressmark`
   - **Gemini CLI**: `gemini extensions install https://github.com/mooshee/pressmark`
   - **Devin CLI**: `devin plugins install mooshee/pressmark`
   - **Vercel Skills (`skills.sh`)**: `npx skills add mooshee/pressmark`
   - **Homebrew**: `brew install mooshee/tap/pressmark`

---

## 1. System Architecture & The Pressmark Vector Advantage

```
+-------------------------------------------------------------------------+
|                  CLIENT SURFACES & PROTOCOL ADAPTERS                    |
|                                                                         |
|  +--------------------+  +--------------------+  +--------------------+ |
|  | Interactive Studio |  |   ChatGPT Plugin   |  |     Model Context  | |
|  | Web UI (Dark Dev)  |  | Custom GPT Action  |  |     Protocol (MCP) | |
|  +---------+----------+  +---------+----------+  +---------+----------+ |
+------------|-----------------------|-----------------------|------------+
             | REST JSON             | OpenAPI 3.1           | JSON-RPC 2.0
             v                       v                       v
+-------------------------------------------------------------------------+
|                     PRESSMARK CORE SERVICE (AXUM + TOKIO)               |
|                                                                         |
|  +-------------------------------------------------------------------+  |
|  | REST API Router & Content Negotiation (/api/v1/generate, preview) |  |
|  +---------------------------------+---------------------------------+  |
|                                    |                                    |
|  +---------------------------------v---------------------------------+  |
|  | Precision Vector Synthesis & Parameter Injection                 |  |
|  |  - Unified Theme Primitives (theme engine)                       |  |
|  |  - Dynamic Geometry (US Letter, A4, Legal)                        |  |
|  |  - Color Schemes & Font Fallbacks (Sans, Serif, Mono)             |  |
|  |  - Compliance Notice Injection & Cryptographic QR Verification    |  |
|  +---------------------------------+---------------------------------+  |
|                                    |                                    |
|  +---------------------------------v---------------------------------+  |
|  | Pure In-Memory Streaming Pipeline (stdin -> stdout)               |  |
|  |   Sub-millisecond Vector PDF & 144/300 PPI PNG Rendering           |  |
|  +---------------------------------+---------------------------------+  |
+------------------------------------|------------------------------------+
                                     v
+-------------------------------------------------------------------------+
|                         OUTPUT ARTIFACTS (< 25ms)                       |
|                                                                         |
|     * Crisp Vector PDF (application/pdf)                                |
|     * Zero-CORS Base64 PNG Preview Data URI (data:image/png;base64)    |
|     * Direct Browser Print Stream                                       |
+-------------------------------------------------------------------------+
```

### Eliminating the Legacy Document Tooling Trap
| Metric | Legacy Chrome / Puppeteer | Legacy Python WeasyPrint | Pressmark Vector Engine |
| :--- | :--- | :--- | :--- |
| **Generation Latency** | 2,500ms – 5,500ms | 3,500ms – 6,000ms | **15ms – 25ms** (100x faster) |
| **Memory Footprint** | 500MB – 1.2GB / process | 250MB – 450MB / process | **< 30MB** total server resident |
| **Concurrency Lock** | Global browser mutex pool | Global GIL & C-lib locks | **Full async Tokio non-blocking** |
| **Unit Compute Cost** | ~$0.005 – $0.02 / document | ~$0.003 / document | **~$0.00001** (99%+ Gross Margin) |
| **Typography Precision** | Browser CSS approximate | Fragile Pango/Cairo | **Pure TeX-grade Microtypography** |

---

## 2. Comprehensive Library of 57 Document Archetypes & 50-State Statutory Lease Engine

### Invoicing & Commercial Billing (14 Archetypes)
- `invoice`, `invoice_freelance`, `invoice_construction`, `invoice_medical`, `invoice_saas`, `invoice_minimal`, `proforma_invoice`, `recurring_invoice`, `retainer_invoice`, `timesheet_invoice`, `legal_retainer`, `real_estate_commission`, `hoa_assessment`, `utility_submetering`

### Estimates, Quotes, Logistics & Operations (13 Archetypes)
- `estimate`, `quotation_tender`, `receipt`, `purchase_order`, `packing_slip`, `credit_note`, `change_order`, `work_order`, `expense_report`, `statement_of_account`, `payment_demand`, `delivery_note`, `commercial_customs_invoice`

### Legal, Corporate Governance & Contracts (11 Archetypes)
- `agreement`, `agreement_sow`, `contractor_agreement`, `nda`, `bill_of_sale`, `certificate`, `inspection_report`, `mechanics_lien_waiver`, `board_resolution`, `promissory_note`, `employment_offer`

### Real Estate, Leasing, Property Management & Investor Underwriting (29 Templates + 50-State + DC Engine)
- **Leasing & Tenancy**: `residential_lease`, `month_to_month_rental_agreement`, `room_rental_agreement`, `sublease_agreement`, `lease_renewal_agreement`, `lease_amendment`, `lease_termination_agreement`, `commercial_lease`, `commercial_nnn_lease`, `short_term_vacation_rental`
- **Addenda & Tenant Screening**: `rental_application`, `lease_guarantor_agreement`, `pet_addendum`, `lead_paint_disclosure`
- **Statutory Property Notices**: `eviction_notice`, `notice_to_vacate`, `late_rent_notice`, `rent_increase_notice`, `notice_to_enter`, `lease_violation_notice`
- **Move-In, Deposit & Operations**: `move_in_out_checklist`, `security_deposit_disposition`, `property_management_statement`, `property_management_agreement`, `rent_roll`
- **Closing, Deeds & Investor Sheets**: `closing_settlement_statement`, `schedule_of_real_estate_owned`, `real_estate_deal_analysis`, `real_estate_purchase_agreement`, `property_deed` (`quitclaim_deed`, `warranty_deed`), `tenant_estoppel`, `letter_of_intent_cre`
- **4-Layer Statutory Lease Engine (`GET /api/v1/states`)**: Compiles Federal HUD/EPA Lead-Based Paint & Fair Housing disclosures, 50-state + DC statutory security deposit caps, return windows, escrow rules, late fee caps, entry notice windows, state-conditional mandatory disclosures, and 6 municipal rent/housing ordinance riders (NYC, SF, LA, Chicago, Seattle, DC).

---

## 3. Interactive Web Studio Capabilities

- **✨ In-Studio AI Copilot**: Type natural language commands directly into the studio bar (e.g. *"Generate a $4,500 retainer invoice for Acme Corp with EU VAT"* or *"Draft an NDA for Heidelberg BioPharma"*) to instantly configure the document.
- **🎲 Surprise Me / Randomize**: Instant 1-click scene rotation through curated real-world enterprise documents across multiple currencies (USD, EUR, GBP, CHF), industries, and themes.
- **📷 Scan & Extract from Photo**: Drag & drop or upload scanned receipt or invoice images with automatic item and total extraction.
- **📋 Clone & Duplicate**: 1-click duplicate document state with fresh unique sequence IDs.
- **{ } JSON State Import/Export**: Full bi-directional JSON schema editing for headless orchestration.
- **🛡️ Enterprise Compliance Toggles**:
  - EU VAT Reverse Charge (Directive 2006/112/EC Art. 196)
  - IRS W-9 Certification & TIN Attestation
  - PCI-DSS Tokenized Settlement Attestation
  - SOC-2 Type II Confidentiality & Trust Seal
  - Statutory Sworn Legal Affidavit & Jurisdiction Block
  - Cryptographic Verification QR Seal (SHA-256 tamper-evident stamp)
  - Perforated Remittance Payment Stub
  - Sequential Line Item Numbering (#1, #2, #3...)
- **HTML5 Canvas Digital Signature Pad**: Draw with mouse or touch, instant embedded vector export.
- **1-Click Export**: Instant Vector PDF download, native print stylesheet trigger, and live PNG previews.

---

## 4. ChatGPT Plugin & Custom GPT Action Integration

Pressmark serves full ChatGPT Plugin and OpenAPI 3.1 action specs natively out of the box:

- **ChatGPT Plugin Manifest**: `GET https://pressmark.dev/.well-known/ai-plugin.json`
- **OpenAPI 3.1 Specification**: `GET https://pressmark.dev/api/v1/openapi.json`
- **Model Context Protocol (MCP)**: `POST https://pressmark.dev/mcp` (and `POST https://pressmark.dev/api/v1/mcp`)
- **MCP Tool Catalog**: `GET https://pressmark.dev/api/v1/mcp/tools.json`

### Connecting to Custom GPT:
1. In ChatGPT GPT Editor, navigate to **Configure > Actions > Create new action**.
2. Paste the Pressmark OpenAPI URL: `https://pressmark.dev/api/v1/openapi.json`
3. Authentication: `None` (or API Key header).
4. Prompt Example:
   > *"Create an invoice for client 'Vance Dynamics' from 'Pressmark Studio' for 32 hours of Rust backend engineering at $185/hr, plus 8.25% tax. Apply the Modern Slate theme and include a signature block."*

---

## 5. Commercial Pricing & Stripe Architecture

| Plan | Price | Monthly Limit | Target Persona | Features |
| :--- | :--- | :--- | :--- | :--- |
| **Free Community** | **$0** | 3 docs / mo | Occasional Users | All archetypes, subtle watermark |
| **Single Unlock Pass** | **$2.99** one-time | 1 document | Impulse / 1-Tap | Zero watermark, 1-tap Apple/Google Pay |
| **Solo Pro** | **$15** / mo | 50 docs / mo | Freelancers & Solo | Full custom logos, signatures, ChatGPT Actions |
| **Business Studio** | **$49** / mo | 250 docs / mo | Growing Agencies | High-res 300 PPI PNGs, API Keys, Priority queue |
| **Agency / API Scale** | **$149** / mo | 2,000 docs / mo | Enterprise & Platforms | Complete white-labeling, batch webhooks |

---

## 6. Local Development & Self-Hosting

### Prerequisites
- Rust 1.80+ (`cargo`)

### Run Locally
```bash
# Clone and enter workspace
git clone https://github.com/mooshee/pressmark.git
cd pressmark

# Run isolated parallel test suite
cargo nextest run

# Launch high-performance Axum service on port 3842
cargo run --release
```

Open `http://localhost:3842` in your browser for local development, or visit [https://pressmark.dev](https://pressmark.dev) for the live production studio.

See [usage measurement](docs/measurement.md) for analytics definitions, historical-data limits and operator verification.
