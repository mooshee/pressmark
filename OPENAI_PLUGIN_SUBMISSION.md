# Pressmark plugin submission

**Plugin Name:** `pressmark`  
**Display Name:** Pressmark Document & Invoice Studio  
**Author:** Daniel Hallman (`daniel@mooshworks.com`)  
**Declared plugin schema:** `https://agent-plugins.org/schemas/1.0.0/plugin.schema.json`\
**Base URL:** `https://pressmark.dev`  
**Privacy Policy:** `https://pressmark.dev/privacy`  
**Terms of Service:** `https://pressmark.dev/terms`  
**Categories:** `Productivity`, `Business`, `Finance`  

---

## Package files

The package contains these manifests and connection settings:

* **Plugin Root Manifest:** `plugin.json` declares `https://agent-plugins.org/schemas/1.0.0/plugin.schema.json`.
* **MCP Server Declarations:** `mcp.json` and `.mcp.json`. `mcp.json` declares `https://agent-plugins.org/schemas/1.0.0/mcp.schema.json`.
  - **Remote Primary:** `streamable-http` transport at `https://pressmark.dev/mcp`.
* **Discovered Skills:**
  - `skills/pressmark/` — Connects the server and introduces the available document tools.
  - `skills/invoice-generation/` — Typesets invoices, estimates, receipts, contracts, NDAs with vector QR codes.
  - `skills/document-lifecycle/` — Manages estimate-to-invoice transitions, partial payments, receipts, and credit notes.
  - `skills/real-estate-agreements/` — Typesets residential leases, commercial NNN leases, rent rolls, and settlement statements.

---

## Prepare an OpenAI submission

1. Use Plugin Creator to prepare a submission for the existing Pressmark plugin. Keep its current identity and publisher.
2. Check the listing, links, demo, reviewer access, live tool annotations, and publication details against the supported behavior.
3. Inspect the final ZIP and run the applicable review cases against the saved version. Record the result of each case.
4. Complete the submission workflow for that existing plugin. An uploaded draft, submission for review, and an approved publication are separate states.

## Development examples

The submitted cases are in `plugin.json` and `chatgpt-app-submission.json`.

Use a guest session and synthetic sender and recipient details for these checks. Document generation can update customer records and start configured follow-up; it uses the signed-in email before document email fields. The [privacy policy](https://pressmark.dev/privacy) lists the fields shared. These examples describe expected results, not evidence that a payment occurred or a document was legally reviewed.

### Positive Test Cases (5)

#### Test Case 1: Consulting invoice and preview
* **User Prompt:**  
  `"Create and preview a sample invoice from Sample Studio to Sample Client for 15 consulting hours at $250 per hour. Leave out email addresses and payment links."`
* **Target Skill / Tool:** `invoice-generation` -> `preview_document` / `generate_document`
* **Expected Arguments:**
  - `doc_type`: `"invoice_freelance"`
  - `from`: `{"name": "Sample Studio"}`
  - `to`: `{"name": "Sample Client"}`
  - `items`: `[{"description": "Consulting", "quantity": 15, "unit_price": 250.0}]`
* **Expected Result / Assertions:**
  - Returns a PDF of the requested invoice.
  - Grand total calculates to `$3,750.00`.
  - Returns a PNG preview and a temporary PDF download link.

#### Test Case 2: Estimate-to-Invoice Lifecycle Conversion
* **User Prompt:**  
  `"Convert this sample estimate to an invoice due in 30 days: EST-2026-0042 from Sample Studio to Sample Client for $5,000 in mobile app design."`
* **Target Skill / Tool:** `document-lifecycle` -> `convert_document`
* **Expected Arguments:**
  - `target_type`: `"invoice"`
  - `due_days`: `30`
  - `document`: valid estimate payload
* **Expected Result / Assertions:**
  - Document prefix re-sequences from `EST-` to `INV-`.
  - Issue date updates to current date; due date calculated to +30 days.
  - Status updates to `pending`.
  - Returns an updated PDF and summary.

#### Test Case 3: Commercial NNN Lease Summary with CAM Allocation
* **User Prompt:**  
  `"Draft a commercial triple-net (NNN) lease summary for Suite 400 at 100 Innovation Way between Hallman Properties LLC (Landlord) and TechVentures Inc (Tenant), 3,500 sq ft at $38/sqft/yr base rent, plus $8.50/sqft/yr estimated CAM charges, starting November 1, 2026."`
* **Target Skill / Tool:** `real-estate-agreements` -> `generate_document`
* **Expected Arguments:**
  - `doc_type`: `"commercial_nnn_lease"`
  - Line items or notes detailing Monthly Base Rent (`$11,083.33`) and Monthly CAM (`$2,479.17`).
  - Total Monthly Obligation: `$13,562.50`.
* **Expected Result / Assertions:**
  - A multi-page commercial lease document.
  - Itemized breakdowns for operating expense passes and indemnification.
  - Mutual signature lines for Landlord and Tenant.

#### Test Case 4: Record a payment and create a receipt
* **User Prompt:**  
  `"Create a receipt for sample invoice INV-2026-001 from Sample Studio to Sample Client for 15 consulting hours at $250 per hour. Use a fictional $3,750 payment on October 7, 2026, with reference sample-payment-001."`
* **Target Skill / Tool:** `document-lifecycle` -> `record_payment`
* **Expected Arguments:**
  - `amount_paid`: `3750.0`
  - `transaction_reference`: `"sample-payment-001"`
  - `paid_date`: `"2026-10-07"`
* **Expected Result / Assertions:**
  - Balance due updates to `$0.00`.
  - Payment status transitions to `paid`.
  - Document stamped with `PAID IN FULL ✅`.
  - Generates a receipt with the supplied transaction reference.

#### Test Case 5: Calculate totals
* **User Prompt:**  
  `"Calculate the total for three laptops at $1,200 each with 9.25% sales tax and a $150 discount."`
* **Target Skill / Tool:** `invoice-generation` -> `calculate_totals`
* **Expected Result / Assertions:**
  - Subtotal: `$3,600.00`, Tax (9.25%): `$333.00`, Discount: `$150.00`, Grand Total: `$3,783.00`.

---

## Tool annotations (live server, aligned with `chatgpt-app-submission.json`)

Every tool declares all four hints explicitly (never null). Artifact-producing tools are not read-only; pure computation and reads are.

| Tool | readOnly | destructive | idempotent | openWorld |
|---|---|---|---|---|
| `generate_document` | false | true | false | true |
| `preview_document` | false | false | false | false |
| `calculate_totals` | true | false | true | false |
| `convert_document` | false | false | false | false |
| `record_payment` | false | false | false | false |
| `submit_feedback` | false | false | false | true |
| `get_feedback_status` | true | false | true | true |

When customer sync is enabled, `generate_document` can replace contact details, update an opportunity's stage, and start follow-up workflows. `submit_feedback` can create a new report. The plugin does not list Pressmark plans or start checkout. A PDF can include a link supplied by the user.

### Negative Test Cases (3)

#### Negative Test Case 1: Missing Mandatory Counterparty Information
* **User Prompt:**  
  `"Generate an invoice for 5 widgets at $10."` (Missing issuer and recipient party information).
* **Expected Behavior / Safety Handling:**
  - The model or tool detects missing mandatory fields (`from.name`, `to.name`).
  - The service rejects a request with missing or empty required party details.
  - Ask for the missing seller and client details before generating the PDF.
  - Do not fill missing party details with invented information.

#### Negative Test Case 2: Negative Quantities / Invalid Billing Arithmetic
* **User Prompt:**  
  `"Generate an invoice with -5 consulting hours at -$100/hr."`
* **Expected Behavior / Safety Handling:**
  - The service rejects negative line-item quantities or unit prices on a standard invoice.
  - Ask whether the user intended a credit note before changing the document type. Creating a credit note does not send a refund.

#### Negative Test Case 3: Unsupported Document Lifecycle State Transition
* **User Prompt:**  
  `"Convert this invoice into an executable bash script."`
* **Expected Behavior / Safety Handling:**
  - `target_type` schema constraint restricts options to `["invoice", "receipt", "estimate", "credit_note"]`.
  - The service rejects the unsupported target type.
  - Explain that Pressmark converts documents, not executable code.
