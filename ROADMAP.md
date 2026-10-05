# Pressmark Strategic Product Roadmap (2026–2027)

**Author:** Daniel Hallman  
**Product:** Pressmark Vector Document Studio & Engine  
**Positioning:** The World's Fastest Vector Invoicing & Document Engine (Sub-15ms Rust Core)

---

## 1. Strategic Vision & The Winning Wedge

Pressmark's core competitive advantage is our **sub-15ms Pressmark Vector Engine**. Unlike legacy SaaS accounting platforms (FreshBooks, Wave, Invoice Simple, HoneyBook) that rely on bloated, multi-second Puppeteer/Chromium clusters or page reloads, Pressmark delivers real-time 60fps typing previews and sub-15ms vector PDF compilation.

### The Wedge: The "Conversational & Instant Invoice"
Rather than building a commoditized, sprawling accounting suite, Pressmark enters the market through a high-velocity, high-demand wedge: **Instant Conversational & Live-Preview Invoicing for Freelancers, Agencies, and Modern Operators**.

```
[ChatGPT / Claude MCP] ──┐
                         ├─► [Pressmark Sub-15ms Engine] ─► [Vector PDF + Stripe QR] ─► [Instant Payment]
[60fps Real-Time Studio] ┘                                           │
                                                                     ▼
                                                     [Headless Developer API]
```

### The Transition to the Moat: Headless Developer API
As our invoicing, estimate, and receipt engine matures, we expose the underlying infrastructure as the premier **Developer Headless Document API**—the high-performance, cost-effective alternative to CraftMyPDF, APITemplate.io, and memory-leaking headless Chrome fleets.

---

## 2. Competitive Matrix

| Dimension | Legacy SaaS (FreshBooks, Wave, Invoice Simple) | PDF Template APIs (CraftMyPDF, APITemplate) | Pressmark Vector Platform |
| :--- | :--- | :--- | :--- |
| **Generation Latency** | 2,000ms – 6,000ms (Heavy Chromium) | 800ms – 2,500ms | **Sub-15ms (Native Rust Vector Engine)** |
| **Studio Preview** | Modal spinners, page reloads | Stale static previews | **60fps Double-Buffered Live Vector Canvas** |
| **Interface** | Form-heavy web dashboard only | REST API / Webhooks only | **Multi-Modal: Real-time Studio + ChatGPT Action + MCP** |
| **Payments** | Saturated proprietary payment lock-in | No built-in payment rails | **Instant Stripe Payment Links & Vector QR Codes** |
| **Document Lifecycle** | Complex, multi-step accounting menus | Stateless manual JSON mapping | **1-Click Estimate ➔ Invoice ➔ Receipt Lifecycle** |

---

## 3. Phased Execution Roadmap

### Phase 1: High-Velocity Conversational Invoicing & Live Studio (Current Sprint)
*Focus: Maximum conversion speed, zero friction, instant Stripe checkout.*

1. **Stripe Payment Links & Vector QR Code Generation**:
   - Embed dynamic, crisp SVG QR codes directly onto invoices, receipts, and remittance stubs.
   - 1-scan mobile payment supporting Apple Pay, Google Pay, and credit cards via Stripe.
   - Visual toggle in Content tab and automatic generation from payment URLs.
2. **1-Click Document Lifecycle Converter**:
   - Seamless progression: `Estimate / Quote` ➔ `Tax Invoice` ➔ `Paid Receipt` ➔ `Credit Note (Reversal / Refund)`.
   - Preserves all client metadata, line items, and discounts without retyping.
   - Sequence numbering integrity: extracts canonical numeric base (`2026-0042`) across `EST-` ➔ `INV-` ➔ `REC-` ➔ `CRD-`.
   - Automatic document re-titling, dynamic balance-due recalculation, and "PAID IN FULL" attestation stamps.
3. **Conversational Billing via ChatGPT & MCP**:
   - Tools: `create_invoice`, `convert_document`, `record_payment`.
   - Natural language trigger: *"Invoice Acme Corp $3,500 for Sept branding, net 15 with Stripe link."*
   - Immediate in-chat image preview, clickable checkout links, and downloadable vector PDF.
4. **Cross-Border Tax & Global e-Invoicing Compliance (PEPPOL & Factur-X)**:
   - Dedicated presets for EU Reverse Charge, UK VAT, US Sales Tax, and Canadian GST/HST.
   - Statutory European & International e-Invoicing compliance:
     * **PEPPOL BIS Billing 3.0**: Native UBL 2.1 XML structure generation.
     * **Factur-X / ZUGFeRD 2.2**: Hybrid e-invoices with embedded machine-readable XML within PDF/A-3 vector documents.
     * Verified Tax Identification verification (VAT ID, EIN, ABN) with statutory seller/buyer legal entity attestations.

---

### Phase 2: Payment Webhooks, Automated Remittance & Client Portal (Days 31–60)
*Focus: Closed-loop billing, automated payment reconciliation, and recurring billing.*

1. **Stripe Webhook Listener & Automated Lifecycle Operations**:
   - Webhook endpoint `/api/v1/webhooks/stripe` with cryptographic signature verification (`stripe-signature`).
   - Event Handling:
     * `payment_intent.succeeded` / `checkout.session.completed`: Transitions invoice state from `pending` to `paid`, captures transaction reference, generates immutable `Receipt` vector PDF, and fires customer receipt email.
     * `charge.refunded`: Automatically triggers `to_credit_note()` lifecycle conversion, issuing a statutory Credit Note (`CRD-XXXX`) referencing the original invoice.
     * `payment_intent.payment_failed`: Transitions invoice to `overdue`, generates an automated dunning statement with updated 1-tap checkout link.
2. **1-Click Client View & Payment Landing Page**:
   - Lightweight public link `/pay/:invoice_id` displaying the vector preview with an interactive "Pay Now with Apple Pay" button.
3. **Saved Counterparty Directory & Recurring Invoicing**:
   - Store frequent clients with tax IDs, preferred currencies, and default terms.
   - Scheduled auto-generation for monthly retainers and recurring maintenance agreements.

---

### Phase 3: High-Throughput Developer Headless API (Days 61–90)
*Focus: B2B infrastructure expansion, developer adoption, and usage-based monetization.*

1. **Developer API & SDKs**:
   - Authenticated endpoints via Bearer API keys (`pk_live_...`).
   - Enterprise Headers:
     * `Idempotency-Key`: Atomic, deduplicated document compilation guarantees.
     * `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`: Real-time quota enforcement.
   - Official SDKs in Rust (`pressmark-rs`), TypeScript/Node (`@pressmark/sdk`), and Python (`pressmark-py`).
   - Zero-install CLI for CI/CD pipelines (`pressmark compile document.json -o invoice.pdf`).
2. **Unit Economics & High-Margin Scalability**:
   - Compute benchmark: ~12ms vector compilation consumes ~$0.0001 in compute on modern edge/bare-metal ARM64 instances.
   - Retail pricing: $0.05 – $0.15 per document render, delivering **> 99.8% gross margins** vs legacy Chromium/Puppeteer fleets that incur high memory and server infrastructure costs.
3. **Usage-Based Credit Metering & Packs**:
   - Microtransaction packs (100, 1,000, 10,000 document renders).
   - Enterprise dedicated instances with sub-5ms SLA.
4. **Template Registry & Community Marketplace**:
   - Curated gallery of verified invoices, quotes, receipts, bills of lading, and settlement statements.
   - Community submission and custom theme publishing.

---

## 4. Key Performance Indicators (KPIs)

- **Engine Performance**: Maintain < 25ms 99th percentile compile latency on production instances.
- **Conversion Velocity**: Time from prompt or studio open to final paid invoice link < 60 seconds.
- **Preview Smoothness**: 60fps interaction with zero white-flash redraws via double-buffering.
- **Test Coverage**: 100% unit and integration test passing rate across all document archetypes and formatters.
