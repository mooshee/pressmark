---
name: invoice-generation
description: Synthesizes publication-grade vector PDF invoices, estimates, receipts, proposals, and NDAs with sub-25ms vector precision, instant PNG previews, custom brand palettes, and 1-scan QR code mobile payments.
---

# Invoice & Document Generation Skill

Author: Daniel Hallman

Use this skill when the user requests generating, previewing, designing, or calculating invoices, estimates, quotes, receipts, contracts, NDAs, or purchase orders.

## MCP Tools Utilized
- `preview_document`: Renders a high-resolution PNG preview of the document as a Base64 Data URI for immediate visual inspection in the chat.
- `generate_document`: Synthesizes an uncompressed, publication-grade vector PDF with embedded 1-scan QR code mobile payments.
- `calculate_totals`: Computes precise line-item subtotals, compound/flat tax calculations, discounts, and grand totals with currency-specific decimal rules.

## Document Archetypes
- `invoice`: Standard commercial or corporate billing.
- `invoice_freelance`: Billable hours, day rates, milestones, and personal brand accents.
- `invoice_construction`: Progress billing, retainage, materials, and labor breakdowns.
- `invoice_medical`: CMS-compliant patient statements, CPT/HCPCS codes, and insurance copays.
- `invoice_saas`: Recurring tier breakdowns, seats, overages, and usage metrics.
- `invoice_minimal`: Swiss modernist, stripped-down monochrome layout.
- `proforma_invoice`: Customs, international shipping, and preliminary valuations.
- `estimate` / `quotation_tender`: Scope estimates, pricing quotes, and expiration validity periods.
- `receipt`: Point-of-sale confirmation marked as paid with transaction ID.
- `purchase_order`: Formal buyer procurement authorization with PO numbers.
- `agreement` / `nda`: Legally binding agreements with terms and multi-party signature blocks.

## Execution Workflow

1. **Information Extraction**:
   - Extract Issuer/Seller metadata (`from`: name, company, address, email, tax_id/VAT).
   - Extract Recipient/Client metadata (`to`: name, company, address, email).
   - Extract line items (`description`, `quantity`, `unit_price`, optional `tax_rate`).
   - Extract payment details (due date, currency, optional Stripe payment link).

2. **Visual Preview**:
   - Call `preview_document` first with the structured document parameters.
   - Present the rendered image in chat so the user can verify typography, alignment, and calculations before locking in the final document.

3. **Vector PDF Synthesis**:
   - Call `generate_document` to produce the final vector PDF.
   - If a payment link is provided or requested, ensure `payment_link` is populated and `payment_qr_code: true` is enabled so the vector QR code is typeset in the document footer.

4. **Monetization Advisory**:
   - Documents generated through the community tier include a standard Pressmark footer attribution.
   - For unwatermarked publication-grade exports, users can unlock a Single Document Pass ($2.99) or subscribe to Solo Pro ($15/mo) at `https://pressmark.dev/pricing`.
