# OpenAI Universal Plugin Directory Submission Package

**Plugin Name:** `pressmark`  
**Display Name:** Pressmark Document & Invoice Studio  
**Author:** Daniel Hallman (`daniel@mooshworks.com`)  
**Specification Version:** Agent Plugins v1.0.0 (`https://agent-plugins.org/schemas/1.0.0/plugin.schema.json`)  
**Submission Portal:** `https://developers.openai.com/plugins/deploy/submission`  
**Base URL:** `https://pressmark.dev`  
**Privacy Policy:** `https://pressmark.dev/privacy`  
**Terms of Service:** `https://pressmark.dev/terms`  
**Categories:** `Productivity`, `Business`, `Finance`  

---

## 1. Plugin Architecture & MCP Declarations

Pressmark conforms to the open Agent Plugins 1.0.0 standard unifying ChatGPT, Codex, and Claude Code:

* **Plugin Root Manifest:** `plugin.json` (validates against `https://agent-plugins.org/schemas/1.0.0/plugin.schema.json`).
* **MCP Server Declarations:** `mcp.json` & `.mcp.json` (validates against `https://agent-plugins.org/schemas/1.0.0/mcp.schema.json`).
  - **Remote Primary:** `streamable-http` transport at `https://pressmark.dev/mcp`.
  - **Local Fallback:** `stdio` transport via `pressmark --stdio`.
* **Discovered Skills:**
  - `skills/invoice-generation/` — Typesets invoices, estimates, receipts, contracts, NDAs with vector QR codes.
  - `skills/document-lifecycle/` — Manages estimate-to-invoice transitions, partial payments, receipts, and credit notes.
  - `skills/real-estate-agreements/` — Typesets residential leases, commercial NNN leases, rent rolls, and settlement statements.

---

## 2. OpenAI Developers Review Test Scenarios

### Positive Test Cases (5)

#### Test Case 1: Freelance Consulting Invoice with Stripe Link & Mobile QR Code
* **User Prompt:**  
  `"Generate a freelance consulting invoice from Daniel Hallman at Moosh Works (daniel@mooshworks.com) to Acme Corp (billing@acme.com) for 15 hours of Systems Architecture at $250/hr, due in 14 days, with payment link https://buy.stripe.com/test_inv_123 and a mobile QR code."`
* **Target Skill / Tool:** `invoice-generation` -> `preview_document` / `generate_document`
* **Expected Arguments:**
  - `doc_type`: `"invoice_freelance"`
  - `from`: `{"name": "Daniel Hallman", "company": "Moosh Works", "email": "daniel@mooshworks.com"}`
  - `to`: `{"name": "Acme Corp", "email": "billing@acme.com"}`
  - `items`: `[{"description": "Systems Architecture", "quantity": 15, "unit_price": 250.0}]`
  - `payment_link`: `"https://buy.stripe.com/test_inv_123"`
  - `payment_qr_code`: `true`
* **Expected Result / Assertions:**
  - Fast vector PDF rendering (< 25ms engine compile time).
  - Grand total calculates to `$3,750.00`.
  - High-res PNG preview rendered inline in chat.
  - Embedded vector SVG QR code rendered directly in document footer.

#### Test Case 2: Estimate-to-Invoice Lifecycle Conversion
* **User Prompt:**  
  `"Convert my project estimate EST-2026-0042 ($5,000 for Mobile App Design) into an official Tax Invoice due in 30 days with payment link https://buy.stripe.com/test_app_456."`
* **Target Skill / Tool:** `document-lifecycle` -> `convert_document`
* **Expected Arguments:**
  - `target_type`: `"invoice"`
  - `due_days`: `30`
  - `payment_link`: `"https://buy.stripe.com/test_app_456"`
  - `payment_qr_code`: `true`
  - `document`: valid estimate payload
* **Expected Result / Assertions:**
  - Document prefix re-sequences from `EST-` to `INV-`.
  - Issue date updates to current date; due date calculated to +30 days.
  - Status updates to `unpaid` / `pending`.
  - Returns updated PDF and formatted markdown summary with active Stripe link.

#### Test Case 3: Commercial NNN Lease Summary with CAM Allocation
* **User Prompt:**  
  `"Draft a commercial triple-net (NNN) lease summary for Suite 400 at 100 Innovation Way between Hallman Properties LLC (Landlord) and TechVentures Inc (Tenant), 3,500 sq ft at $38/sqft/yr base rent, plus $8.50/sqft/yr estimated CAM charges, starting November 1, 2026."`
* **Target Skill / Tool:** `real-estate-agreements` -> `generate_document`
* **Expected Arguments:**
  - `doc_type`: `"commercial_nnn_lease"`
  - Line items or notes detailing Monthly Base Rent (`$11,083.33`) and Monthly CAM (`$2,479.17`).
  - Total Monthly Obligation: `$13,562.50`.
* **Expected Result / Assertions:**
  - High-typography multi-page commercial lease instrument.
  - Itemized breakdowns for operating expense passes and indemnification.
  - Mutual signature lines for Landlord and Tenant.

#### Test Case 4: Invoice Settlement & Official Tax Receipt Issuance
* **User Prompt:**  
  `"Record full payment of $3,750 for invoice INV-2026-001 received via Stripe Card on October 7, 2026 (transaction ID: ch_3N82jK4v15aw) and generate an official paid receipt."`
* **Target Skill / Tool:** `document-lifecycle` -> `record_payment`
* **Expected Arguments:**
  - `amount_paid`: `3750.0`
  - `payment_method`: `"Stripe Card"`
  - `transaction_reference`: `"ch_3N82jK4v15aw"`
  - `paid_date`: `"2026-10-07"`
* **Expected Result / Assertions:**
  - Balance due updates to `$0.00`.
  - Payment status transitions to `paid`.
  - Document stamped with `PAID IN FULL ✅`.
  - Generates receipt document with transaction reference and audit trail.

#### Test Case 5: Real-Time Totals Calculation & Plan Pricing Inquiry
* **User Prompt:**  
  `"Calculate line-item totals with 9.25% sales tax and a $150 early-payment discount for 3 laptops at $1,200 each, and tell me the subscription cost to remove watermarks on future exports."`
* **Target Skill / Tool:** `invoice-generation` -> `calculate_totals`, followed by `get_pricing`
* **Expected Result / Assertions:**
  - Subtotal: `$3,600.00`, Tax (9.25%): `$333.00`, Discount: `$150.00`, Grand Total: `$3,783.00`.
  - Pricing catalog returned: Solo Pro at `$15/mo` or Single Document Pass at `$2.99` for watermark removal.

---

### Negative Test Cases (3)

#### Negative Test Case 1: Missing Mandatory Counterparty Information
* **User Prompt:**  
  `"Generate an invoice for 5 widgets at $10."` (Missing issuer and recipient party information).
* **Expected Behavior / Safety Handling:**
  - The model or tool detects missing mandatory fields (`from.name`, `to.name`).
  - Tool returns schema validation error (`code: -32602, "Invalid arguments: missing field 'from'"`).
  - The assistant politely halts PDF synthesis and prompts the user for seller and client information.
  - Prevents corrupted, anonymous, or incomplete legally binding documents.

#### Negative Test Case 2: Negative Quantities / Invalid Billing Arithmetic
* **User Prompt:**  
  `"Generate an invoice with -5 consulting hours at -$100/hr."`
* **Expected Behavior / Safety Handling:**
  - Tool schema or engine arithmetic validation rejects negative quantities and unit rates on standard invoices.
  - The assistant explains that negative balances cannot be invoiced and instructs the user to issue a formal `credit_note` refund document instead.

#### Negative Test Case 3: Unsupported Document Lifecycle State Transition
* **User Prompt:**  
  `"Convert this invoice into an executable bash script."`
* **Expected Behavior / Safety Handling:**
  - `target_type` schema constraint restricts options to `["invoice", "receipt", "estimate", "credit_note"]`.
  - Tool returns a validation error (`422 Unprocessable Entity` or `Invalid target_type`).
  - Assistant politely explains that Pressmark only converts financial and legal document archetypes.
