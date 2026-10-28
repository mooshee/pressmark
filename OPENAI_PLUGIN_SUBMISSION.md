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

## Reviewer test cases (NOT RUN for this package)

Use a guest session and synthetic sender and recipient details for these checks. Document generation can update customer records and start configured follow-up; it uses the signed-in email before document email fields. The [privacy policy](https://pressmark.dev/privacy) lists the fields shared. These examples describe expected results, not evidence that a payment occurred or a document was legally reviewed.

### Positive Test Cases (5)

#### Test Case 1: Consulting invoice with a payment link and QR code
* **User Prompt:**  
  `"Generate a freelance consulting invoice from Sample Studio (sender@example.com) to Sample Client (recipient@example.com) for 15 hours of Systems Architecture at $250/hr, due in 14 days, with payment link https://example.com/pay/invoice and a mobile QR code."`
* **Target Skill / Tool:** `invoice-generation` -> `preview_document` / `generate_document`
* **Expected Arguments:**
  - `doc_type`: `"invoice_freelance"`
  - `from`: `{"name": "Sample Studio", "email": "sender@example.com"}`
  - `to`: `{"name": "Sample Client", "email": "recipient@example.com"}`
  - `items`: `[{"description": "Systems Architecture", "quantity": 15, "unit_price": 250.0}]`
  - `payment_link`: `"https://example.com/pay/invoice"`
  - `payment_qr_code`: `true`
* **Expected Result / Assertions:**
  - Returns a PDF of the requested invoice.
  - Grand total calculates to `$3,750.00`.
  - High-res PNG preview rendered inline in chat.
  - Embedded vector SVG QR code rendered directly in document footer.

#### Test Case 2: Estimate-to-Invoice Lifecycle Conversion
* **User Prompt:**  
  `"Convert my project estimate EST-2026-0042 ($5,000 for Mobile App Design) into an official Tax Invoice due in 30 days with payment link https://example.com/pay/project."`
* **Target Skill / Tool:** `document-lifecycle` -> `convert_document`
* **Expected Arguments:**
  - `target_type`: `"invoice"`
  - `due_days`: `30`
  - `payment_link`: `"https://example.com/pay/project"`
  - `payment_qr_code`: `true`
  - `document`: valid estimate payload
* **Expected Result / Assertions:**
  - Document prefix re-sequences from `EST-` to `INV-`.
  - Issue date updates to current date; due date calculated to +30 days.
  - Status updates to `pending`.
  - Returns an updated PDF and summary with the supplied payment link.

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
  `"Record full payment of $3,750 for invoice INV-2026-001 received via Stripe Card on October 7, 2026 (transaction ID: sample-payment-001) and generate an official paid receipt."`
* **Target Skill / Tool:** `document-lifecycle` -> `record_payment`
* **Expected Arguments:**
  - `amount_paid`: `3750.0`
  - `payment_method`: `"Stripe Card"`
  - `transaction_reference`: `"sample-payment-001"`
  - `paid_date`: `"2026-10-07"`
* **Expected Result / Assertions:**
  - Balance due updates to `$0.00`.
  - Payment status transitions to `paid`.
  - Document stamped with `PAID IN FULL ✅`.
  - Generates a receipt with the supplied transaction reference.

#### Test Case 5: Calculate totals and check pricing
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
