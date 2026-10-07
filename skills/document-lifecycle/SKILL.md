---
name: document-lifecycle
description: Transitions documents through their operational billing lifecycle—converting estimates into tax invoices, recording partial or full payments, recalculating balances due, issuing paid receipts, and generating credit notes.
---

# Document Lifecycle & Billing Skill

Author: Daniel Hallman

Use this skill when the user asks to convert an existing estimate or quote into an invoice, record a customer payment, mark an invoice as paid, or issue a refund credit note.

## MCP Tools Utilized
- `convert_document`: Transforms document lifecycle state (e.g. `estimate` -> `invoice`, `invoice` -> `receipt`, `invoice` -> `credit_note`). Updates document numbering sequences (`EST-` -> `INV-` -> `REC-` -> `CRD-`), adjusts issue/due dates, and transitions payment status flags.
- `record_payment`: Records payment transactions against an outstanding invoice. Recalculates `balance_due = grand_total - total_paid`, updates status (`unpaid`, `partially_paid`, or `paid`), and returns the updated document object.

## Lifecycle Transition Matrix

1. **Estimate / Quote Accepted**:
   - Call `convert_document` with `target_type: "invoice"` and optional `due_date_days: 14` or `30`.
   - The document ID transitions from `EST-XXXX` to `INV-XXXX`.
   - Issue date updates to the current date, due date is calculated, and payment status defaults to `unpaid`.

2. **Partial Payment Received**:
   - Call `record_payment` with `payment_amount: <amount>`, `payment_method: "stripe"` (or `"bank_transfer"` / `"card"`), and `reference: <tx_id>`.
   - If `payment_amount < grand_total`, status transitions to `partially_paid`, and `balance_due` reflects the remaining owed balance.

3. **Settled / Full Payment Received**:
   - Call `record_payment` with `payment_amount: balance_due`.
   - Payment status transitions to `paid`, `balance_due` becomes `0.00`, and a stamped paid watermark is enabled.
   - User can optionally convert to an official tax receipt (`REC-XXXX`) via `convert_document` (`target_type: "receipt"`).

4. **Refund / Adjustment / Credit Note**:
   - Call `convert_document` with `target_type: "credit_note"`.
   - Numbering updates to `CRD-XXXX`, amounts are framed as credit/refund offsets referencing the original invoice number.
