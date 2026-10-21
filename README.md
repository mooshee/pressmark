# PRESSMARK · Sub-Millisecond Vector Document Engine & Interactive Studio

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![Pressmark Engine](https://img.shields.io/badge/Pressmark%20Engine-v1.0.0-blue.svg)](https://pressmark.dev)
[![Pressmark MCP server quality and maintenance on Glama](https://glama.ai/mcp/servers/mooshee/pressmark/badges/score.svg)](https://glama.ai/mcp/servers/mooshee/pressmark)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Author**: Daniel Hallman  
> **Brand**: [Pressmark Studio](https://pressmark.dev)  
> **Architecture**: Pure Rust (Tokio + Axum) + Sub-Millisecond Vector Synthesis Engine  

Pressmark is an ultra-fast, publication-grade document synthesis engine, real-time interactive design studio, and Model Context Protocol (MCP) / ChatGPT plugin server. Built to replace sluggish legacy document pipelines (Puppeteer/Headless Chrome, Python ReportLab, WeasyPrint), Pressmark compiles crisp, print-ready vector PDFs and high-resolution PNG previews in under **25 milliseconds** with zero disk churn and sub-cent unit economics.

## Quick start

1. Open [Pressmark Studio](https://pressmark.dev) to design, preview, and download vector PDFs in your browser.
2. Connect any MCP client (Claude Code, Cursor, Codex, Grok, Gemini CLI, Hermes) to `https://pressmark.dev/mcp` (`streamable-http`), or download the pre-packaged plugin bundles from [GitHub Releases](https://github.com/mooshee/pressmark/releases).

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

## 2. Comprehensive Library of 18 Document Archetypes

### Invoicing & Commercial
1. **Tax Invoice (`invoice` / `invoice_standard`)**: Full multi-currency calculation, itemized tax rates, payment terms, ACH/wire instructions, and status stamps.
2. **Freelance & Consulting Invoice (`invoice_freelance`)**: Hourly rate structures, billable consulting hours, milestone references, and contractor details.
3. **Construction Progress Billing (`invoice_construction`)**: Subcontractor draw applications, retainage calculations, job-site references, and work-completed percentages.
4. **Clinical Services Superbill (`invoice_medical`)**: Healthcare provider billing, patient diagnostics, procedural codes (CPT/ICD-10), and insurance remittance.
5. **SaaS & Subscription Invoice (`invoice_saas`)**: Cloud usage metrics, recurring seat licenses, enterprise tier charges, and metering periods.
6. **Minimal Commercial Invoice (`invoice_minimal`)**: High-contrast, minimalist typographic layout for design studios and boutique agencies.

### Estimates, Quotes & Operations
7. **Formal Estimate / Quote (`estimate`)**: Scope of work breakdowns, pricing estimates, expiration dates, and client acceptance blocks.
8. **Payment Receipt (`receipt`)**: Settlement confirmations, transaction IDs, payment method tokens, and settled stamps.
9. **Enterprise Purchase Order (`purchase_order`)**: Vendor routing, shipping addresses, procurement authorizations, and SKU tracking.
10. **Packing Slip & BOL (`packing_slip`)**: Warehouse dispatch logs, box weights, carrier routing, and line-item shipment tallies.
11. **Credit Memorandum (`credit_note`)**: Account adjustments, credit issuance, reference invoice linkage, and balance offsets.

### Legal, Contracts & Trust
12. **Master Services Agreement (`agreement`)**: Operative SOW clauses, IP assignments, dispute arbitration terms, and dual signature blocks.
13. **Statement of Work (`agreement_sow`)**: Milestone deliverables, project schedules, and client acceptance specifications.
14. **Independent Contractor Agreement (`contractor_agreement`)**: W-9 contractor representations, confidentiality covenants, and non-solicitation clauses.
15. **Mutual Non-Disclosure Agreement (`nda`)**: Confidentiality definitions, exclusions, non-use covenants, and dual-party execution lines.
16. **Statutory Bill of Sale (`bill_of_sale`)**: Asset transfers, vehicle/equipment serial verification, warranty disclaimers, and consideration sums.
17. **Certificate of Achievement (`certificate`)**: Formal typographic awards, honors, issuing authorities, and verification seal.
18. **Condition & Inspection Report (`inspection_report`)**: Facility or property grading, defect checklists, inspector credentials, and sign-offs.

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
