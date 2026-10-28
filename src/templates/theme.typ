// Pressmark Unified Vector Design System & Document Primitives
// Author: Daniel Hallman
//
// Sub-millisecond vector document synthesis primitives for commercial invoices,
// estimates, receipts, corporate agreements, non-disclosure agreements,
// purchase orders, inspection checklists, and certificates.

#let color-slate-950 = rgb("#020617")
#let color-slate-900 = rgb("#0f172a")
#let color-slate-800 = rgb("#1e293b")
#let color-slate-700 = rgb("#334155")
#let color-slate-600 = rgb("#475569")
#let color-slate-500 = rgb("#64748b")
#let color-slate-400 = rgb("#94a3b8")
#let color-slate-300 = rgb("#cbd5e1")
#let color-slate-200 = rgb("#e2e8f0")
#let color-slate-100 = rgb("#f1f5f9")
#let color-slate-50 = rgb("#f8fafc")

#let color-indigo-600 = rgb("#4f46e5")
#let color-indigo-50 = rgb("#eef2ff")
#let color-emerald-700 = rgb("#047857")
#let color-emerald-600 = rgb("#059669")
#let color-emerald-500 = rgb("#10b981")
#let color-emerald-50 = rgb("#ecfdf5")
#let color-crimson-600 = rgb("#dc2626")
#let color-crimson-50 = rgb("#fef2f2")
#let color-amber-600 = rgb("#d97706")
#let color-amber-50 = rgb("#fffbeb")
#let color-cyan-600 = rgb("#0891b2")
#let color-cyan-50 = rgb("#ecfeff")
#let color-blue-600 = rgb("#2563eb")
#let color-blue-50 = rgb("#eff6ff")

#let font-serif = ("Georgia", "Libertinus Serif", "DejaVu Serif", "Times New Roman")
#let font-sans  = ("Helvetica Neue", "Arial", "Inter", "Liberation Sans", "DejaVu Sans")
#let font-mono  = ("Menlo", "DejaVu Sans Mono", "Liberation Mono", "Courier New")

// =============================================================================
// ATOMIC BADGES & UI PRIMITIVES
// =============================================================================

#let badge(body, bg: color-indigo-50, fg: color-indigo-600) = {
  box(
    fill: bg,
    inset: (x: 8pt, y: 3.5pt),
    radius: 4pt,
    text(size: 7.5pt, weight: "bold", fill: fg)[#body]
  )
}

#let status-pill(status, paid: false) = {
  if status == "none" or status == "" or status == "NONE" {
    // Suppressed status badge
  } else if paid or status == "PAID / SETTLED" or status == "OFFICIALLY ISSUED" or status == "EXECUTED / ACTIVE" {
    badge(status, bg: color-emerald-50, fg: color-emerald-600)
  } else if status == "PARTIALLY PAID" or status == "PARTIAL PAYMENT" {
    badge(status, bg: color-blue-50, fg: color-blue-600)
  } else if status == "CREDIT / REFUNDED" or status == "CREDIT ISSUED" or status == "REFUNDED" {
    badge(status, bg: color-slate-100, fg: color-slate-600)
  } else if status == "ESTIMATE / QUOTE" or status == "PROPOSAL" or status == "DRAFT" {
    badge(status, bg: color-cyan-50, fg: color-cyan-600)
  } else {
    badge(status, bg: color-amber-50, fg: color-amber-600)
  }
}

#let divider(color: color-slate-200) = {
  line(length: 100%, stroke: 0.75pt + color)
}

#let card-box(body, bg: color-slate-50, border: color-slate-200) = {
  rect(
    width: 100%,
    fill: bg,
    stroke: 0.5pt + border,
    radius: 6pt,
    inset: 9pt,
    body
  )
}

// =============================================================================
// COMPLIANCE & TRUST NOTICES
// =============================================================================

#let compliance-banner(compliance-type, primary-color: color-indigo-600) = {
  if compliance-type == "eu_vat" or compliance-type == "factur-x" or compliance-type == "factur_x" {
    card-box(
      grid(
        columns: (auto, 1fr),
        gutter: 8pt,
        text(size: 9pt)[🏛️],
        [
          #text(size: 8pt, weight: "bold", fill: color-slate-900)[EU VAT DIRECTIVE 2006/112/EC & EN 16931 FACTUR-X COMPLIANT] \
          #text(size: 7pt, fill: color-slate-500)[Intra-community supply of services. Reverse charge applies where customer VAT ID is verified.]
        ]
      ),
      bg: rgb("#f0fdf4"),
      border: rgb("#86efac")
    )
  } else if compliance-type == "peppol" or compliance-type == "peppol_bis" {
    card-box(
      grid(
        columns: (auto, 1fr),
        gutter: 8pt,
        text(size: 9pt)[🌐],
        [
          #text(size: 8pt, weight: "bold", fill: color-slate-900)[PEPPOL BIS BILLING 3.0 · UBL 2.1 E-INVOICING STANDARD] \
          #text(size: 7pt, fill: color-slate-500)[Structured electronic billing payload aligned with Pan-European Public Procurement On-Line (EN 16931) specifications.]
        ]
      ),
      bg: rgb("#eff6ff"),
      border: rgb("#93c5fd")
    )
  } else if compliance-type == "hipaa" or compliance-type == "hipaa_baa" {
    card-box(
      grid(
        columns: (auto, 1fr),
        gutter: 8pt,
        text(size: 9pt)[⚕️],
        [
          #text(size: 8pt, weight: "bold", fill: color-slate-900)[HIPAA 45 CFR § 164.504(e) PROTECTED HEALTH INFORMATION NOTICE] \
          #text(size: 7pt, fill: color-slate-500)[Confidential clinical billing record subject to HIPAA Privacy & Security Rules. Minimum necessary disclosure safeguard active.]
        ]
      ),
      bg: rgb("#f0fdfa"),
      border: rgb("#5eead4")
    )
  } else if compliance-type == "zatca" or compliance-type == "zatca_ksa" {
    card-box(
      grid(
        columns: (auto, 1fr),
        gutter: 8pt,
        text(size: 9pt)[🧾],
        [
          #text(size: 8pt, weight: "bold", fill: color-slate-900)[ZATCA PHASE 2 (FATOORA) KSA E-INVOICING COMPLIANT] \
          #text(size: 7pt, fill: color-slate-500)[Issued in accordance with Zakat, Tax and Customs Authority e-invoicing integration and cryptographic stamp regulations.]
        ]
      ),
      bg: rgb("#f0fdf4"),
      border: rgb("#86efac")
    )
  } else if compliance-type == "irs_w9" or compliance-type == "w9" {
    card-box(
      grid(
        columns: (auto, 1fr),
        gutter: 8pt,
        text(size: 9pt)[⚖️],
        [
          #text(size: 8pt, weight: "bold", fill: color-slate-900)[IRS W-9 TAXPAYER ATTESTATION] \
          #text(size: 7pt, fill: color-slate-500)[Under penalties of perjury, the issuer certifies that the Taxpayer Identification Number (TIN) provided is correct and the entity is not subject to backup withholding.]
        ]
      ),
      bg: rgb("#eff6ff"),
      border: rgb("#93c5fd")
    )
  } else if compliance-type == "pci_dss" or compliance-type == "pci" {
    card-box(
      grid(
        columns: (auto, 1fr),
        gutter: 8pt,
        text(size: 9pt)[🔒],
        [
          #text(size: 8pt, weight: "bold", fill: color-slate-900)[PCI-DSS LEVEL 1 TOKENIZED CLEARING] \
          #text(size: 7pt, fill: color-slate-500)[Zero raw cardholder data storage. Cryptographic settlement executed via end-to-end tokenized processing.]
        ]
      ),
      bg: rgb("#faf5ff"),
      border: rgb("#d8b4fe")
    )
  } else if compliance-type == "soc2" or compliance-type == "soc2_badge" or compliance-type == "iso27001" {
    card-box(
      grid(
        columns: (auto, 1fr),
        gutter: 8pt,
        text(size: 9pt)[🛡️],
        [
          #text(size: 8pt, weight: "bold", fill: color-slate-900)[SOC-2 TYPE II AUDITED & ISO/IEC 27001 VERIFIED] \
          #text(size: 7pt, fill: color-slate-500)[Enterprise controls, security auditing, and continuous compliance telemetry active.]
        ]
      ),
      bg: color-slate-50,
      border: color-slate-300
    )
  } else if compliance-type == "statutory_sworn" or compliance-type == "sworn" {
    card-box(
      grid(
        columns: (auto, 1fr),
        gutter: 8pt,
        text(size: 9pt)[📜],
        [
          #text(size: 8pt, weight: "bold", fill: color-slate-900)[STATUTORY SWORN DECLARATION] \
          #text(size: 7pt, fill: color-slate-500)[I declare under penalty of perjury under the laws of the applicable jurisdiction that the foregoing statements, amounts, and deliverables are true and correct.]
        ]
      ),
      bg: rgb("#fef2f2"),
      border: rgb("#fca5a5")
    )
  }
}

// =============================================================================
// CRYPTOGRAPHIC VERIFICATION SEAL / QR BLOCK
// =============================================================================

// =============================================================================
// VECTOR PAYMENT QR CODE & CHECKOUT BLOCK
// =============================================================================

#let payment-qr-block(
  qr-path: none,
  payment-url: none,
  payment-status: "unpaid",
  amount-due: "$0.00",
  due-date: none,
  payment-method: "Stripe / Card / Apple Pay",
  primary-color: color-indigo-600,
  accent-color: color-cyan-600,
  surface-color: color-slate-50
) = {
  let is-paid = (payment-status == "paid")
  v(8pt)
  rect(
    width: 100%,
    stroke: if is-paid { 1pt + color-emerald-500 } else { 0.75pt + accent-color.lighten(30%) },
    fill: if is-paid { rgb("#f0fdf4") } else { surface-color },
    radius: 4pt,
    inset: 8pt,
    [
      #grid(
        columns: (auto, 1fr, auto),
        gutter: 12pt,
        align: (center + horizon, left + horizon, right + horizon),
        [
          #if qr-path != none and qr-path != "" {
            let qr-box = box(
              width: 52pt,
              height: 52pt,
              fill: white,
              radius: 3pt,
              stroke: 0.5pt + color-slate-200,
              inset: 3pt,
              align(center + horizon)[
                #image(qr-path, width: 46pt, height: 46pt, fit: "contain")
              ]
            )
            if payment-url != none and payment-url != "" {
              link(payment-url)[#qr-box]
            } else {
              qr-box
            }
          } else {
            rect(
              width: 52pt,
              height: 52pt,
              fill: if is-paid { color-emerald-600 } else { primary-color },
              radius: 3pt,
              align(center + horizon)[
                #text(size: 6.5pt, weight: "bold", fill: white)[SCAN\ TO PAY]
              ]
            )
          }
        ],
        [
          #if is-paid {
            stack(
              dir: ttb,
              spacing: 3pt,
              text(size: 8.5pt, weight: "bold", fill: color-emerald-700)[✓ PAYMENT RECEIVED IN FULL],
              text(size: 7pt, fill: color-slate-600)[Processed via #payment-method. Official receipt generated.]
            )
          } else {
            stack(
              dir: ttb,
              spacing: 3pt,
              text(size: 8.5pt, weight: "bold", fill: primary-color)[SCAN TO PAY VIA MOBILE · #upper(payment-method)],
              text(size: 7pt, fill: color-slate-600)[Instant 1-tap checkout. Scan the QR code with your mobile camera.],
              ..if payment-url != none and payment-url != "" {
                (text(size: 6.5pt, font: font-mono, weight: "bold", fill: primary-color)[#link(payment-url)[#payment-url]],)
              } else {
                ()
              }
            )
          }
        ],
        [
          #if is-paid {
            box(
              fill: color-emerald-600,
              radius: 3pt,
              inset: (x: 8pt, y: 5pt),
              text(size: 8pt, weight: "bold", fill: white)[PAID · \$0.00 DUE]
            )
          } else {
            align(right)[
              #stack(
                dir: ttb,
                spacing: 3pt,
                text(size: 6.5pt, weight: "bold", fill: color-slate-500)[AMOUNT DUE],
                text(size: 11pt, weight: 900, fill: primary-color)[#amount-due],
                ..if due-date != none and due-date != "" {
                  (text(size: 6.5pt, fill: color-slate-500)[Due: #due-date],)
                } else {
                  ()
                }
              )
            ]
          }
        ]
      )
    ]
  )
}

// =============================================================================
// PAID IN FULL ATTESTATION WATERMARK STAMP
// =============================================================================

#let paid-watermark-stamp(
  tx-ref: "STRIPE-PAID",
  paid-date: "2026-10-02",
  method: "Stripe / Apple Pay"
) = {
  rotate(-14deg, box(
    stroke: 2pt + rgb("#059669"),
    radius: 4pt,
    inset: (x: 10pt, y: 6pt),
    fill: rgb("#f0fdf4"),
    align(center)[
      #text(size: 13pt, weight: 900, fill: rgb("#047857"), tracking: 1.5pt)[PAID IN FULL] \
      #text(size: 6pt, font: font-mono, weight: "bold", fill: rgb("#065f46"))[#method · #paid-date] \
      #text(size: 5.5pt, font: font-mono, fill: rgb("#059669"))[REF: #tx-ref]
    ]
  ))
}

#let verification-seal(doc-number, hash: "", primary-color: color-indigo-600, accent-color: color-cyan-600, surface-color: color-slate-50) = {
  let effective-hash = if hash != "" { hash } else { doc-number }
  box(
    stroke: 0.75pt + accent-color.lighten(40%),
    radius: 4pt,
    fill: surface-color,
    inset: 6pt,
    width: 100%,
    [
      #grid(
        columns: (auto, 1fr),
        gutter: 8pt,
        align: (center + horizon, left + horizon),
        [
          #rect(
            width: 34pt,
            height: 34pt,
            fill: primary-color,
            radius: 2pt,
            align(center + horizon)[
              #text(size: 6pt, font: font-mono, fill: white)[PRESS\ MARK\ SECURE]
            ]
          )
        ],
        [
          #text(size: 7pt, weight: "bold", fill: primary-color)[DIGITALLY VERIFIED DOCUMENT] \
          #text(size: 6pt, font: font-mono, fill: color-slate-500)[ID: #doc-number] \
          #text(size: 5.5pt, font: font-mono, fill: color-slate-400)[SHA-256: #effective-hash · Pressmark Audit Node]
        ]
      )
    ]
  )
}

// =============================================================================
// REMITTANCE STUB (TEAR-OFF PAYMENT SLIP)
// =============================================================================

#let remittance-stub(
  doc-number: "INV-2026-0042",
  client-name: "Acme Corp",
  amount-due: "$1,500.00",
  due-date: "2026-10-15",
  issuer-name: "Pressmark Technologies"
) = {
  block(breakable: false, width: 100%, above: 10pt, below: 8pt)[
    #grid(
      columns: (1fr, auto, 1fr),
      gutter: 8pt,
      align: horizon,
      line(length: 100%, stroke: (paint: color-slate-300, thickness: 0.75pt, dash: "dashed")),
      text(size: 7pt, weight: "bold", fill: color-slate-400, tracking: 0.4pt)[✂  DETACH & RETURN WITH REMITTANCE  ✂],
      line(length: 100%, stroke: (paint: color-slate-300, thickness: 0.75pt, dash: "dashed"))
    )
    #v(5pt)
    #rect(
      width: 100%,
      stroke: 0.5pt + color-slate-300,
      fill: color-slate-50,
      radius: 4pt,
      inset: (x: 10pt, y: 8pt),
      [
        #grid(
          columns: (1fr, 1fr, 1fr),
          gutter: 12pt,
          align: (left + horizon, left + horizon, right + horizon),
          stack(
            dir: ttb,
            spacing: 3pt,
            text(size: 6.5pt, weight: "bold", fill: color-slate-400)[REMIT TO:],
            text(size: 8pt, weight: "bold", fill: color-slate-900)[#issuer-name],
            text(size: 7pt, fill: color-slate-500)[Invoice Reference: #doc-number]
          ),
          stack(
            dir: ttb,
            spacing: 3pt,
            text(size: 6.5pt, weight: "bold", fill: color-slate-400)[BILLED CUSTOMER:],
            text(size: 8pt, weight: "bold", fill: color-slate-900)[#client-name],
            text(size: 7pt, fill: color-slate-500)[Due Date: #due-date]
          ),
          align(right)[
            #stack(
              dir: ttb,
              spacing: 3pt,
              text(size: 6.5pt, weight: "bold", fill: color-slate-400)[TOTAL REMITTANCE DUE:],
              text(size: 11pt, weight: 900, fill: color-slate-950)[#amount-due],
              text(size: 6.5pt, fill: color-slate-500)[Amount Enclosed: \$ #box(line(length: 60pt, stroke: 0.5pt + color-slate-400))]
            )
          ]
        )
      ]
    )
  ]
}

// =============================================================================
// DOCUMENT HEADER COMPONENT
// =============================================================================

#let document-header(
  title: "INVOICE",
  subtitle: none,
  doc-number: "INV-2026-001",
  date: "2026-10-01",
  due-date: none,
  logo-path: none,
  company-name: "Pressmark Technologies",
  primary-color: color-indigo-600,
  is-paid: false,
  status: "DUE UPON RECEIPT",
  layout-style: "standard"
) = {
  block(width: 100%, below: 6pt)[
    #set par(leading: 0.38em)
    #if layout-style == "modern_banner" {
      rect(
        width: 100%,
        fill: primary-color,
        radius: 6pt,
        inset: (x: 14pt, y: 11pt),
        [
          #grid(
            columns: (1.1fr, 1fr),
            gutter: 12pt,
            align: (left + horizon, right + horizon),
            stack(
              dir: ttb,
              spacing: 3.5pt,
              ..if logo-path != none and logo-path != "" {
                (box(height: 30pt, image(logo-path, fit: "contain")),)
              } else { () },
              text(size: 15pt, weight: "bold", fill: white)[#company-name],
              ..if subtitle != none and subtitle != "" {
                (text(size: 8pt, fill: white.transparentize(20%))[#subtitle],)
              } else { () }
            ),
            align(right)[
              #stack(
                dir: ttb,
                spacing: 3.5pt,
                text(size: 16.5pt, weight: 900, fill: white)[#title],
                text(size: 9pt, weight: "bold", fill: white.transparentize(18%))[#doc-number]
              )
            ]
          )
        ]
      )
      v(7pt)
    } else if layout-style == "classic_letterhead" {
      align(center)[
        #stack(
          dir: ttb,
          spacing: 4pt,
          ..if logo-path != none and logo-path != "" {
            (box(height: 34pt, image(logo-path, fit: "contain")),)
          } else { () },
          text(size: 16.5pt, weight: "bold", fill: primary-color)[#company-name],
          ..if subtitle != none and subtitle != "" {
            (text(size: 8.5pt, fill: color-slate-500)[#subtitle],)
          } else { () },
          line(length: 48%, stroke: 1.25pt + primary-color),
          text(size: 15pt, weight: 900, fill: color-slate-900)[#title],
          box[
            #text(size: 9pt, weight: "bold", fill: color-slate-500)[#doc-number]
            #h(6pt)
            #status-pill(status, paid: is-paid)
          ]
        )
      ]
      v(6pt)
    } else if layout-style == "swiss_grid" {
      line(length: 100%, stroke: 3pt + primary-color)
      v(6pt)
      grid(
        columns: (1.15fr, 1fr),
        gutter: 14pt,
        align: (left + horizon, right + horizon),
        stack(
          dir: ttb,
          spacing: 3.5pt,
          ..if logo-path != none and logo-path != "" {
            (box(height: 32pt, image(logo-path, fit: "contain")),)
          } else { () },
          text(size: 16pt, weight: "bold", fill: color-slate-950)[#company-name],
          ..if subtitle != none and subtitle != "" {
            (text(size: 8pt, fill: color-slate-500)[#subtitle],)
          } else { () }
        ),
        align(right)[
          #stack(
            dir: ttb,
            spacing: 3.5pt,
            text(size: 16.5pt, weight: 900, fill: primary-color)[#title],
            box[
              #text(size: 9pt, font: font-mono, weight: "bold", fill: color-slate-600)[#doc-number]
              #h(6pt)
              #status-pill(status, paid: is-paid)
            ]
          )
        ]
      )
      v(6pt)
      line(length: 100%, stroke: 0.5pt + color-slate-300)
      v(6pt)
    } else if layout-style == "split_sidebar" {
      rect(
        width: 100%,
        fill: color-slate-50,
        stroke: 0.5pt + color-slate-200,
        radius: 5pt,
        inset: (x: 12pt, y: 10pt),
        [
          #grid(
            columns: (1.1fr, 1fr),
            gutter: 12pt,
            align: (left + horizon, right + horizon),
            stack(
              dir: ttb,
              spacing: 3.5pt,
              ..if logo-path != none and logo-path != "" {
                (box(height: 30pt, image(logo-path, fit: "contain")),)
              } else { () },
              text(size: 15pt, weight: "bold", fill: primary-color)[#company-name],
              ..if subtitle != none and subtitle != "" {
                (text(size: 8pt, fill: color-slate-500)[#subtitle],)
              } else { () }
            ),
            align(right)[
              #stack(
                dir: ttb,
                spacing: 3.5pt,
                text(size: 16.5pt, weight: 900, fill: color-slate-900)[#title],
                box[
                  #text(size: 9pt, weight: "bold", fill: primary-color)[#doc-number]
                  #h(6pt)
                  #status-pill(status, paid: is-paid)
                ]
              )
            ]
          )
        ]
      )
      v(6pt)
    } else if layout-style == "compact_ledger" {
      grid(
        columns: (1.1fr, 1fr),
        gutter: 12pt,
        align: (left + horizon, right + horizon),
        stack(
          dir: ttb,
          spacing: 3pt,
          ..if logo-path != none and logo-path != "" {
            (box(height: 24pt, image(logo-path, fit: "contain")),)
          } else { () },
          text(size: 13.5pt, weight: "bold", fill: primary-color)[#company-name],
          ..if subtitle != none and subtitle != "" {
            (text(size: 8pt, fill: color-slate-500)[#subtitle],)
          } else { () }
        ),
        align(right)[
          #stack(
            dir: ttb,
            spacing: 3.5pt,
            text(size: 14.5pt, weight: 900, fill: color-slate-900)[#title],
            box[
              #text(size: 8.5pt, font: font-mono, weight: "bold", fill: color-slate-500)[#doc-number]
              #h(6pt)
              #status-pill(status, paid: is-paid)
            ]
          )
        ]
      )
      v(5pt)
      line(length: 100%, stroke: 1pt + color-slate-200)
      v(6pt)
    } else if layout-style == "editorial_centered" {
      align(center)[
        #stack(
          dir: ttb,
          spacing: 4pt,
          ..if logo-path != none and logo-path != "" {
            (box(height: 34pt, image(logo-path, fit: "contain")),)
          } else { () },
          text(size: 18pt, weight: 900, fill: color-slate-900)[#title],
          text(size: 11pt, weight: "medium", fill: primary-color)[#company-name],
          ..if subtitle != none and subtitle != "" {
            (text(size: 8pt, fill: color-slate-500)[#subtitle],)
          } else { () },
          box[
            #text(size: 8.5pt, fill: color-slate-500)[Reference: #doc-number]
            #h(6pt)
            #status-pill(status, paid: is-paid)
          ],
          line(length: 100%, stroke: 0.5pt + color-slate-200)
        )
      ]
      v(6pt)
    } else if layout-style == "framed" {
      rect(
        width: 100%,
        fill: color-slate-50,
        stroke: 1pt + primary-color,
        radius: 6pt,
        inset: (x: 12pt, y: 10pt),
        [
          #grid(
            columns: (1.1fr, 1fr),
            gutter: 12pt,
            align: (left + horizon, right + horizon),
            stack(
              dir: ttb,
              spacing: 3.5pt,
              ..if logo-path != none and logo-path != "" {
                (box(height: 30pt, image(logo-path, fit: "contain")),)
              } else { () },
              text(size: 15pt, weight: "bold", fill: primary-color)[#company-name],
              ..if subtitle != none and subtitle != "" {
                (text(size: 8pt, fill: color-slate-500)[#subtitle],)
              } else { () }
            ),
            align(right)[
              #stack(
                dir: ttb,
                spacing: 3.5pt,
                text(size: 16pt, weight: 900, fill: color-slate-900)[#title],
                box[
                  #text(size: 9pt, font: font-mono, weight: "bold", fill: primary-color)[#doc-number]
                  #h(6pt)
                  #status-pill(status, paid: is-paid)
                ]
              )
            ]
          )
        ]
      )
      v(6pt)
    } else if layout-style == "minimal" {
      grid(
        columns: (1.1fr, 1fr),
        gutter: 12pt,
        align: (left + horizon, right + horizon),
        stack(
          dir: ttb,
          spacing: 3pt,
          ..if logo-path != none and logo-path != "" {
            (box(height: 26pt, image(logo-path, fit: "contain")),)
          } else { () },
          text(size: 15pt, weight: "bold", fill: color-slate-900)[#company-name],
          ..if subtitle != none and subtitle != "" {
            (text(size: 8pt, fill: color-slate-500)[#subtitle],)
          } else { () }
        ),
        align(right)[
          #stack(
            dir: ttb,
            spacing: 3.5pt,
            text(size: 15.5pt, weight: 800, fill: color-slate-900)[#title],
            box[
              #text(size: 8.5pt, fill: color-slate-500)[#doc-number]
              #h(6pt)
              #status-pill(status, paid: is-paid)
            ]
          )
        ]
      )
      v(5pt)
      line(length: 100%, stroke: 0.5pt + color-slate-200)
      v(6pt)
    } else {
      grid(
        columns: (1.1fr, 1fr),
        gutter: 14pt,
        align: (left + horizon, right + horizon),
        stack(
          dir: ttb,
          spacing: 3.5pt,
          ..if logo-path != none and logo-path != "" {
            (box(height: 34pt, image(logo-path, fit: "contain")),)
          } else { () },
          text(size: 16pt, weight: "bold", fill: primary-color)[#company-name],
          ..if subtitle != none and subtitle != "" {
            (text(size: 8.5pt, fill: color-slate-500)[#subtitle],)
          } else { () }
        ),
        align(right)[
          #stack(
            dir: ttb,
            spacing: 3.5pt,
            text(size: 17.5pt, weight: 900, fill: color-slate-900)[#title],
            box[
              #text(size: 9pt, font: font-mono, weight: "bold", fill: color-slate-500)[#doc-number]
              #h(6pt)
              #status-pill(status, paid: is-paid)
            ]
          )
        ]
      )

      v(6pt)
      divider(color: primary-color)
      v(6pt)
    }

    #grid(
      columns: (1fr, 1fr),
      gutter: 20pt,
      align: (left + horizon, right + horizon),
      stack(
        dir: ttb,
        spacing: 2.5pt,
        text(size: 7.5pt, weight: "bold", fill: color-slate-400)[ISSUE / EFFECTIVE DATE],
        text(size: 9pt, weight: "bold", fill: color-slate-900)[#date]
      ),
      align(right)[
        #if due-date != none and due-date != "" {
          stack(
            dir: ttb,
            spacing: 2.5pt,
            text(size: 7.5pt, weight: "bold", fill: color-slate-400)[PAYMENT DUE DATE],
            text(size: 9pt, weight: "bold", fill: primary-color)[#due-date]
          )
        } else {
          stack(
            dir: ttb,
            spacing: 2.5pt,
            text(size: 7.5pt, weight: "bold", fill: color-slate-400)[DOCUMENT REFERENCE],
            text(size: 9pt, font: font-mono, weight: "bold", fill: color-slate-700)[#doc-number]
          )
        }
      ]
    )
  ]
}

// =============================================================================
// PARTIES GRID (FROM / TO)
// =============================================================================

#let parties-grid(
  from-name: "Acme Studio Inc.",
  from-details: "123 Innovation Way\nSan Francisco, CA 94105\nbilling@acme.io",
  to-name: "Global Logistics Corp",
  to-details: "456 Enterprise Blvd\nAustin, TX 78701\nap@globallogistics.com",
  from-title: "ISSUED BY",
  to-title: "BILLED TO",
  primary-color: rgb("4f46e5"),
  secondary-color: rgb("475569"),
  surface-color: rgb("f8fafc")
) = {
  let from-inner = [
    #set par(leading: 0.48em)
    #stack(
      dir: ttb,
      spacing: 4pt,
      box(
        fill: primary-color.lighten(88%),
        inset: (x: 5pt, y: 2.5pt),
        radius: 3pt,
        text(size: 6.5pt, weight: "bold", fill: primary-color)[● #from-title]
      ),
      text(size: 10pt, weight: "bold", fill: color-slate-900)[#from-name],
      text(size: 8pt, fill: color-slate-700)[#from-details]
    )
  ]
  let to-inner = [
    #set par(leading: 0.48em)
    #stack(
      dir: ttb,
      spacing: 4pt,
      box(
        fill: secondary-color.lighten(88%),
        inset: (x: 5pt, y: 2.5pt),
        radius: 3pt,
        text(size: 6.5pt, weight: "bold", fill: secondary-color)[○ #to-title]
      ),
      text(size: 10pt, weight: "bold", fill: color-slate-900)[#to-name],
      text(size: 8pt, fill: color-slate-700)[#to-details]
    )
  ]
  layout(size => {
    let col-w = (size.width - 14pt) / 2
    let inner-w = col-w - 20pt
    let h1 = measure(width: inner-w, from-inner).height
    let h2 = measure(width: inner-w, to-inner).height
    let card-h = calc.max(h1, h2) + 17pt
    grid(
      columns: (1fr, 1fr),
      gutter: 14pt,
      rect(
        width: 100%,
        height: card-h,
        fill: surface-color,
        stroke: (
          left: 3.5pt + primary-color,
          top: 0.5pt + secondary-color.lighten(70%),
          right: 0.5pt + secondary-color.lighten(70%),
          bottom: 0.5pt + secondary-color.lighten(70%)
        ),
        radius: (right: 5pt, left: 2pt),
        inset: (x: 10pt, y: 8.5pt),
        from-inner
      ),
      rect(
        width: 100%,
        height: card-h,
        fill: rgb("#ffffff"),
        stroke: (
          left: 3.5pt + secondary-color,
          top: 0.5pt + secondary-color.lighten(70%),
          right: 0.5pt + secondary-color.lighten(70%),
          bottom: 0.5pt + secondary-color.lighten(70%)
        ),
        radius: (right: 5pt, left: 2pt),
        inset: (x: 10pt, y: 8.5pt),
        to-inner
      )
    )
  })
}

// =============================================================================
// CUSTOM SECTIONS
// =============================================================================

#let custom-section(title, body) = {
  block(sticky: true, above: 9pt, below: 4pt)[
    #stack(
      dir: ttb,
      spacing: 3pt,
      text(size: 9.5pt, weight: "bold", fill: color-slate-900)[#title],
      line(length: 100%, stroke: 0.5pt + color-slate-300)
    )
  ]
  block(above: 4pt, below: 6pt)[
    #text(size: 8.5pt, fill: color-slate-700)[#body]
  ]
}

// =============================================================================
// SIGNATURE BLOCK & MULTI-SIGNATORY GRID
// =============================================================================

#let signature-item(
  signature-path: none,
  signer-name: "Daniel Hallman",
  signer-title: "Authorized Representative",
  date: "2026-10-01",
  primary-color: color-indigo-600
) = {
  box(width: 100%)[
    #stack(
      dir: ttb,
      spacing: 3.5pt,
      if signature-path != none and signature-path != "" {
        align(center)[#box(height: 28pt, image(signature-path, fit: "contain"))]
      } else {
        v(22pt)
      },
      line(length: 100%, stroke: 0.75pt + color-slate-700),
      text(size: 8.5pt, weight: "bold", fill: color-slate-900)[#signer-name],
      text(size: 7.5pt, fill: color-slate-500)[#signer-title],
      text(size: 7pt, fill: color-slate-400)[Date: #date]
    )
  ]
}

#let signature-grid(items) = {
  block(breakable: false, width: 100%, above: 10pt, below: 6pt)[
    #let count = items.len()
    #if count == 1 {
      align(right)[
        #box(width: 200pt)[#items.at(0)]
      ]
    } else if count == 2 {
      grid(
        columns: (1fr, 1fr),
        gutter: 24pt,
        items.at(0),
        items.at(1)
      )
    } else if count == 3 {
      grid(
        columns: (1fr, 1fr, 1fr),
        gutter: 16pt,
        items.at(0),
        items.at(1),
        items.at(2)
      )
    } else {
      grid(
        columns: (1fr, 1fr),
        gutter: 18pt,
        ..items
      )
    }
  ]
}

#let signature-block(
  signature-path: none,
  signer-name: "Daniel Hallman",
  signer-title: "Authorized Representative",
  date: "2026-10-01",
  primary-color: color-indigo-600
) = {
  block(breakable: false, width: 100%, above: 10pt, below: 6pt)[
    #align(right)[
      #box(width: 210pt)[
        #stack(
          dir: ttb,
          spacing: 3.5pt,
          if signature-path != none and signature-path != "" {
            align(center)[#box(height: 30pt, image(signature-path, fit: "contain"))]
          } else {
            v(24pt)
          },
          line(length: 100%, stroke: 0.75pt + color-slate-700),
          text(size: 9pt, weight: "bold", fill: color-slate-900)[#signer-name],
          text(size: 8pt, fill: color-slate-500)[#signer-title],
          text(size: 7.5pt, fill: color-slate-400)[Date: #date]
        )
      ]
    ]
  ]
}

// =============================================================================
// WATERMARK PRIMITIVE (FOR FREE TIER)
// =============================================================================

#let render-watermark(text-val) = {
  if text-val != none and text-val != "" {
    place(
      center + horizon,
      rotate(
        -30deg,
        text(size: 38pt, weight: "bold", fill: color-slate-400.transparentize(82%))[#text-val]
      )
    )
  }
}

// =============================================================================
// INSTITUTIONAL, JUDICIAL & WHITE-LABEL DESIGN PRIMITIVES
// Inspired by Architectural Specifications in Untax Engine
// =============================================================================

#let ink-primary       = rgb("#0f172a")
#let ink-secondary     = rgb("#475569")
#let ink-muted         = rgb("#94a3b8")
#let ink-subtle        = rgb("#64748b")

#let navy-judicial     = rgb("#1e3a8a")
#let green-savings     = rgb("#15803d")
#let green-savings-bg  = rgb("#dcfce7")
#let amber-warning     = rgb("#b45309")
#let amber-warning-bg  = rgb("#fef3c7")
#let red-statutory     = rgb("#b91c1c")
#let red-statutory-bg  = rgb("#fee2e2")

#let bg-tint-subtle    = rgb("#f8fafc")
#let bg-tint-highlight = rgb("#f1f5f9")
#let bg-tint-card      = rgb("#ffffff")
#let border-color      = rgb("#cbd5e1")
#let border-dark       = rgb("#0f172a")

#let font-serif = ("Georgia", "Libertinus Serif", "DejaVu Serif", "Times New Roman")
#let font-sans  = ("Helvetica Neue", "Arial", "Inter", "Liberation Sans", "DejaVu Sans")
#let font-mono  = ("Menlo", "DejaVu Sans Mono", "Liberation Mono", "Courier New")

/// County Clerk / Reception walk-in time/date receipt clearance box
#let date-stamp-zone(width: 2.2in, height: 1.1in) = {
  rect(
    width: width,
    height: height,
    stroke: (paint: ink-muted, dash: "dashed", thickness: 0.75pt),
    radius: 3pt,
    fill: bg-tint-subtle,
    inset: 4pt,
    align(center + horizon)[
      #text(font: font-sans, size: 6.5pt, weight: "bold", fill: ink-secondary)[
        OFFICIAL RECEIVED DATE-STAMP \
        #v(2pt)
        #text(size: 5.5pt, fill: ink-muted, weight: "regular")[DO NOT WRITE OR SIGN IN THIS SPACE]
      ]
    ]
  )
}

/// Master document layout with running header on Page 2+ and deterministic Page X of Y footer
#let document-layout(
  title: "COMMERCIAL INSTRUMENT",
  subtitle: "",
  jurisdiction: "OFFICIAL JURISDICTION",
  docket: "",
  parcel-id: "",
  tax-year: "2026",
  branding: none,
  recordable: false,
  body
) = {
  let org-name = if branding != none and branding.org-name != "" {
    branding.org-name
  } else {
    "PRESSMARK STUDIO"
  }
  let header-left-text = if branding != none and branding.header-left != "" {
    branding.header-left
  } else {
    upper(org-name) + " · OFFICIAL VECTOR INSTRUMENT"
  }
  let footer-left-text = if branding != none and branding.footer-left != "" {
    branding.footer-left
  } else {
    org-name + " · Pressmark Vector Engine"
  }
  let footer-center-text = if branding != none and branding.footer-center != "" {
    branding.footer-center
  } else {
    "Certified Verifiable Document"
  }
  let primary-color = if branding != none { branding.accent-primary } else { navy-judicial }

  let page-margins = if recordable {
    (x: 1.0in, top: 1.0in, bottom: 1.0in)
  } else {
    (x: 0.55in, top: 0.55in, bottom: 0.6in)
  }

  set page(
    paper: "us-letter",
    margin: page-margins,
    header: context {
      if counter(page).get().first() > 1 [
        #grid(
          columns: (1fr, 1fr),
          text(font: font-sans, size: 7.5pt, fill: ink-muted, weight: "bold")[#header-left-text],
          align(right, text(font: font-sans, size: 7.5pt, fill: ink-muted, weight: "bold")[
            REF: #parcel-id #sym.bullet DATE: #tax-year
          ])
        )
        #v(-4pt)
        #line(length: 100%, stroke: 0.5pt + rgb("#e2e8f0"))
      ]
    },
    footer: context [
      #line(length: 100%, stroke: 0.5pt + rgb("#e2e8f0"))
      #v(2pt)
      #grid(
        columns: (1fr, 1fr, 1fr),
        text(font: font-sans, size: 8pt, fill: ink-muted)[#footer-left-text],
        align(center, text(font: font-sans, size: 8pt, fill: ink-muted)[#footer-center-text]),
        align(right, text(font: font-sans, size: 8pt, fill: ink-muted)[
          Page #counter(page).display() of #counter(page).final().first()
        ])
      )
      #if branding != none and branding.disclaimer != "" [
        #v(-2pt)
        #align(center, text(font: font-sans, size: 6.5pt, fill: ink-muted)[#branding.disclaimer])
      ]
    ]
  )

  set text(font: font-sans, size: 9pt, fill: ink-primary)
  set par(leading: 0.6em, justify: false)

  if recordable [
    // 3.0-inch top margin clearance for official recording stamp
    #v(2.0in)
  ]

  body
}

/// Official legal, institutional or corporate masthead
#let masthead(
  jurisdiction: "COMMERCIAL & REGULATORY DOCUMENTATION DIVISION",
  document-title: "OFFICIAL INSTRUMENT & DECLARATION",
  subtitle: "Binding Record & Certified Vector Filing",
  docket: none,
  parcel-id: "",
  tax-year: "",
  branding: none,
) = {
  let org-name = if branding != none and "org-name" in branding and branding.org-name != "" {
    branding.org-name
  } else {
    "PRESSMARK VECTOR INSTRUMENT"
  }
  let primary-color = if branding != none and "accent-primary" in branding {
    branding.accent-primary
  } else {
    navy-judicial
  }

  block(width: 100%, spacing: 10pt)[
    // Top jurisdiction and organizational banner
    #grid(
      columns: (1fr, auto),
      gutter: 12pt,
      [
        #text(font: font-sans, size: 8pt, weight: "bold", fill: primary-color)[#upper(jurisdiction)]
        #v(1pt)
        #text(font: font-sans, size: 7.5pt, fill: ink-secondary)[#org-name]
      ],
      align(right)[
        #if docket != none and docket != "" [
          #badge(upper(docket), bg: color-indigo-50, fg: primary-color)
        ] else [
          #badge("CERTIFIED RECORD", bg: color-slate-100, fg: color-slate-700)
        ]
      ]
    )

    #v(3pt)
    #line(length: 100%, stroke: 1.5pt + primary-color)
    #v(4pt)

    // Document Title
    #text(font: font-serif, size: 16pt, weight: "bold", fill: ink-primary)[#document-title]
    #if subtitle != "" [
      #v(2pt)
      #text(font: font-sans, size: 8.5pt, fill: ink-secondary)[#subtitle]
    ]

    #v(6pt)

    // Metadata bar with reference/parcel ID, year, and appearance
    #rect(
      width: 100%,
      fill: bg-tint-subtle,
      stroke: 0.5pt + border-color,
      radius: 3pt,
      inset: (x: 8pt, y: 5pt),
      [
        #grid(
          columns: (1.2fr, 1fr, 1fr),
          [
            #text(font: font-sans, size: 7pt, weight: "bold", fill: ink-subtle)[REFERENCE / RECORD ID] \
            #text(font: font-mono, size: 9pt, weight: "bold", fill: ink-primary)[#parcel-id]
          ],
          [
            #text(font: font-sans, size: 7pt, weight: "bold", fill: ink-subtle)[EFFECTIVE PERIOD / YEAR] \
            #text(font: font-mono, size: 9pt, weight: "bold", fill: primary-color)[#tax-year]
          ],
          align(right)[
            #text(font: font-sans, size: 7pt, weight: "bold", fill: ink-subtle)[INSTRUMENT STATUS] \
            #text(font: font-sans, size: 8.5pt, weight: "bold", fill: ink-primary)[Executed & Verified]
          ]
        )
      ]
    )
  ]
}

/// Highlighted statutory callout citing exact legal authority and burden of proof
#let statutory-box(
  citation: "UCC § 2-201 / UNIFORM COMMERCIAL CODE",
  burden: "Enforceability & Commercial Record Standing",
  body
) = {
  rect(
    width: 100%,
    stroke: (left: 3.5pt + navy-judicial, rest: 0.5pt + border-color),
    radius: (right: 4pt, left: 0pt),
    fill: bg-tint-subtle,
    inset: (x: 9pt, y: 6.5pt),
    [
      #set par(leading: 0.48em)
      #stack(
        dir: ttb,
        spacing: 3.5pt,
        grid(
          columns: (1fr, auto),
          gutter: 10pt,
          align: (left + horizon, right + top),
          text(font: font-mono, size: 7.2pt, weight: "bold", fill: navy-judicial)[#citation],
          badge("STATUTORY GROUND", bg: rgb("#dbeafe"), fg: navy-judicial)
        ),
        text(font: font-sans, size: 7pt, fill: ink-secondary)[
          *Evidentiary Scope:* #burden
        ],
        body
      )
    ]
  )
}

/// Side-by-side financial valuation, appraisal & tax liability delta grid
#let financial-delta-grid(
  current-appraised: "$0",
  proposed-appraised: "$0",
  current-assessed: "$0",
  proposed-assessed: "$0",
  net-appraised-reduction: "$0",
  net-assessed-reduction: "$0",
  annual-savings: "$0",
  ratio: "25%",
  levy-breakdown: ()
) = {
  rect(
    width: 100%,
    stroke: 0.75pt + border-color,
    radius: 4pt,
    fill: bg-tint-card,
    inset: 8pt,
    [
      #grid(
        columns: (1fr, 1fr, 1fr),
        gutter: 8pt,
        [
          #rect(
            width: 100%,
            fill: bg-tint-subtle,
            stroke: 0.5pt + border-color,
            radius: 3pt,
            inset: 6pt,
            [
              #text(font: font-sans, size: 6.5pt, weight: "bold", fill: ink-subtle)[ORIGINAL BASELINE] \
              #text(font: font-mono, size: 10pt, weight: "bold", fill: ink-primary)[#current-appraised] \
              #v(1pt)
              #text(font: font-sans, size: 6.5pt, fill: ink-secondary)[Ratio: #ratio · #current-assessed]
            ]
          )
        ],
        [
          #rect(
            width: 100%,
            fill: rgb("#f0fdf4"),
            stroke: 0.5pt + rgb("#86efac"),
            radius: 3pt,
            inset: 6pt,
            [
              #text(font: font-sans, size: 6.5pt, weight: "bold", fill: green-savings)[EVIDENTIARY CONTENDED] \
              #text(font: font-mono, size: 10pt, weight: "bold", fill: green-savings)[#proposed-appraised] \
              #v(1pt)
              #text(font: font-sans, size: 6.5pt, fill: ink-secondary)[Ratio: #ratio · #proposed-assessed]
            ]
          )
        ],
        [
          #rect(
            width: 100%,
            fill: rgb("#dcfce7"),
            stroke: 1pt + green-savings,
            radius: 3pt,
            inset: 6pt,
            [
              #text(font: font-sans, size: 6.5pt, weight: "bold", fill: green-savings)[ANNUAL NET BENEFIT] \
              #text(font: font-mono, size: 11pt, weight: "bold", fill: green-savings)[#annual-savings] \
              #v(1pt)
              #text(font: font-mono, size: 6.5pt, weight: "bold", fill: green-savings)[#net-appraised-reduction]
            ]
          )
        ]
      )
    ]
  )
}

/// Sworn affidavit jurat, penalty of perjury statement, and 3-way execution block
#let jurat-box(
  heading: "EXECUTION & SWORN ATTESTATION",
  attestation: "",
  notary-required: false,
  pro-se: true,
  signer-title: "Authorized Representative / Managing Principal",
  signer-capacity: "",
  seal-box: false,
  cross-reference: "",
  signature-type: "esign", // "wet-ink", "esign", "ron"
  signer-name: "",
  signature-img: none,
  execution-date: "",
  esign-audit-id: "",
  esign-timestamp: "",
  esign-ip: "",
  ron-statute-citation: "T.C.A. § 8-16-301 et seq.",
  ron-session-id: "",
  include-clerk-stamp: false,
  state: "Tennessee",
  county: "Hamilton",
) = {
  let default-attestation = "I declare under penalty of perjury that I am authorized to execute this instrument, and that the statements, figures, and attached schedules herein are true, correct, and complete."
  let body-text = if attestation != "" { attestation } else { default-attestation }
  let effective-title = if signer-capacity != "" {
    signer-title + " (" + signer-capacity + ")"
  } else {
    signer-title
  }

  block(breakable: false, width: 100%)[
    #rect(
      width: 100%,
      stroke: (dash: "dashed", paint: ink-muted, thickness: 0.75pt),
      radius: 4pt,
      fill: bg-tint-subtle,
      inset: (x: 8pt, y: 6.5pt),
      [
        #if cross-reference != "" [
          #rect(
            width: 100%,
            fill: bg-tint-highlight,
            stroke: 0.5pt + navy-judicial,
            radius: 2pt,
            inset: (x: 4pt, y: 2.5pt),
            text(font: font-mono, size: 6pt, weight: "bold", fill: navy-judicial)[
              ATTACHMENT BINDING: #cross-reference
            ]
          )
          #v(3pt)
        ]

        #text(font: font-sans, size: 7.5pt, weight: "bold", fill: ink-primary)[#upper(heading)]
        #v(2pt)
        #text(font: font-sans, size: 7pt, fill: ink-secondary)[#body-text]
        #v(8pt)

        #let is-wet = signature-type == "wet-ink" or signature-type == "wet_ink" or signature-type == "notarized_wet"
        #let is-ron = signature-type == "ron" or signature-type == "notarized_ron"
        #let is-esign = signature-type == "esign" or (not is-wet and not is-ron)

        #grid(
          columns: if include-clerk-stamp and is-wet {
            (1.2fr, 0.8fr, auto)
          } else {
            (1.5fr, 1fr)
          },
          column-gutter: if include-clerk-stamp and is-wet { 14pt } else { 24pt },
          [
            #if is-esign [
              #if signature-img != none [
                #image(signature-img, height: 20pt)
              ] else if signer-name != "" [
                #text(font: font-serif, style: "italic", size: 12pt, fill: navy-judicial)[#signer-name]
              ] else [
                #text(font: font-serif, style: "italic", size: 12pt, fill: navy-judicial)[#signer-title]
              ]
            ] else [
              #v(14pt)
            ]
            #line(length: 100%, stroke: 1pt + border-dark)
            #v(2pt)
            #text(font: font-sans, size: 6.5pt, fill: ink-subtle)[#effective-title]
          ],
          [
            #if is-esign and execution-date != "" [
              #text(font: font-mono, size: 8.5pt, weight: "bold", fill: navy-judicial)[#execution-date]
            ] else [
              #v(14pt)
            ]
            #line(length: 100%, stroke: 1pt + border-dark)
            #v(2pt)
            #text(font: font-sans, size: 6.5pt, fill: ink-subtle)[Execution Date (YYYY-MM-DD)]
          ],
          if include-clerk-stamp and is-wet [
            #date-stamp-zone(width: 1.8in, height: 0.75in)
          ]
        )

        #if is-esign and esign-audit-id != "" [
          #v(5pt)
          #rect(
            width: 100%,
            fill: rgb("#f0fdf4"),
            stroke: 0.5pt + rgb("#86efac"),
            radius: 2pt,
            inset: (x: 6pt, y: 3pt),
            [
              #grid(
                columns: (1fr, auto),
                [
                  #text(font: font-mono, size: 5.5pt, weight: "bold", fill: green-savings)[
                    CERTIFIED ELECTRONIC EXECUTION (15 U.S.C. § 7001 / UETA) · AUDIT ID: #esign-audit-id
                  ] \
                  #text(font: font-mono, size: 5pt, fill: ink-secondary)[
                    DIGITALLY ATTESTED BY #upper(if signer-name != "" { signer-name } else { effective-title }) · IP: #if esign-ip != "" { esign-ip } else { "VERIFIED" }
                  ]
                ],
                align(right + horizon)[
                  #text(font: font-mono, size: 5pt, fill: ink-subtle)[#esign-timestamp]
                ]
              )
            ]
          )
        ]

        #if is-ron or signature-type == "notarized_wet" or notary-required [
          #v(6pt)
          #line(length: 100%, stroke: 0.5pt + rgb("#e2e8f0"))
          #v(4pt)
          #text(font: font-sans, size: 6.5pt, weight: "bold", fill: navy-judicial)[NOTARIAL ACKNOWLEDGMENT (STATE OF #upper(state), COUNTY OF #upper(county))] \
          #text(font: font-sans, size: 6pt, fill: ink-secondary)[
            Subscribed, acknowledged, and sworn to before me this #if execution-date != "" { execution-date } else { "____ day of _________, 20____" }, by the subscriber proved to me through satisfactory evidence of identification.
            #if is-ron [
              Executed via two-way audio-video Remote Online Notarization (RON) pursuant to #ron-statute-citation #if ron-session-id != "" [ · Session ID: #ron-session-id].
            ]
          ]
          #v(6pt)
          #grid(
            columns: (1.2fr, 0.8fr, 1.2fr),
            gutter: 8pt,
            [
              #v(14pt)
              #line(length: 100%, stroke: 0.75pt + border-dark)
              #text(font: font-sans, size: 5.5pt, fill: ink-subtle)[Signature of Notary Public]
            ],
            [
              #v(14pt)
              #line(length: 100%, stroke: 0.75pt + border-dark)
              #text(font: font-sans, size: 5.5pt, fill: ink-subtle)[Commission Expiration]
            ],
            align(center + horizon)[
              #rect(
                width: 100%,
                height: 28pt,
                stroke: 0.5pt + ink-muted,
                radius: 2pt,
                align(center + horizon)[
                  #text(font: font-mono, size: 5pt, fill: ink-subtle)[AFFIX OFFICIAL NOTARY SEAL]
                ]
              )
            ]
          )
        ]
      ]
    )
  ]
}

/// Cryptographic verification slug at document base
#let audit-slug(
  parcel-id: "",
  hash: "",
  engine-version: "Pressmark Vector Engine v1.0.0",
  timestamp: ""
) = {
  let display-hash = if hash.len() > 28 { hash.slice(0, 28) + "…" } else { hash }
  rect(
    width: 100%,
    fill: bg-tint-subtle,
    stroke: 0.5pt + rgb("#cbd5e1"),
    radius: 3pt,
    inset: (x: 8pt, y: 4.5pt),
    [
      #grid(
        columns: (auto, 1fr, auto),
        gutter: 8pt,
        align: (left + horizon, left + horizon, right + horizon),
        text(font: font-mono, size: 6pt, weight: "bold", fill: ink-primary)[VERIFICATION SLUG:],
        text(font: font-mono, size: 6pt, fill: ink-secondary)[
          REF: #parcel-id · SHA-256: #display-hash
        ],
        align(right, text(font: font-mono, size: 6pt, fill: ink-subtle)[
          #engine-version · #timestamp
        ])
      )
    ]
  )
}

/// Physical submission checklist
#let clerk-checklist(title: "SUBMISSION CHECKLIST", items: ()) = {
  rect(
    width: 100%,
    fill: bg-tint-subtle,
    stroke: 0.5pt + border-color,
    radius: 3pt,
    inset: 8pt,
    [
      #text(font: font-sans, size: 7.5pt, weight: "bold", fill: ink-primary)[#title]
      #v(4pt)
      #for item in items [
        #grid(
          columns: (12pt, 1fr),
          [#rect(width: 7pt, height: 7pt, stroke: 0.75pt + ink-muted, radius: 1pt)],
          text(font: font-sans, size: 7pt, fill: ink-secondary)[#item]
        )
        #v(2pt)
      ]
    ]
  )
}

/// Walk-in counter facility routing box
#let walk-in-box(
  facility-title: "CUSTOMER SERVICE & RECORDING DIVISION",
  room-window: "Service Counter #3",
  physical-address: "6135 Heritage Park Dr, Chattanooga, TN",
  hours: "Monday – Friday · 8:00 AM – 4:00 PM",
  clerk-phone: "(423) 209-7300"
) = {
  rect(
    width: 100%,
    fill: rgb("#eff6ff"),
    stroke: 0.5pt + rgb("#93c5fd"),
    radius: 4pt,
    inset: 8pt,
    [
      #grid(
        columns: (1fr, auto),
        [
          #text(font: font-sans, size: 8pt, weight: "bold", fill: navy-judicial)[#facility-title] \
          #text(font: font-sans, size: 7pt, fill: ink-secondary)[#physical-address · #room-window] \
          #text(font: font-sans, size: 6.5pt, fill: ink-subtle)[Hours: #hours · Tel: #clerk-phone]
        ],
        align(right + horizon)[
          #badge("WALK-IN SERVICE", bg: rgb("#dbeafe"), fg: navy-judicial)
        ]
      )
    ]
  )
}

/// Real Estate Investment & Underwriting KPI Grid (BiggerPockets / Stessa Pro)
#let deal-kpi-grid(
  metrics: (),
  accent: navy-judicial,
) = {
  if metrics.len() > 0 {
    let cols = calc.min(metrics.len(), 4)
    rect(
      width: 100%,
      stroke: 0.75pt + border-color,
      radius: 4pt,
      fill: bg-tint-card,
      inset: 8pt,
      [
        #grid(
          columns: (1fr,) * cols,
          gutter: 6pt,
          ..metrics.map(m => rect(
            width: 100%,
            fill: bg-tint-subtle,
            stroke: (top: 2pt + accent, rest: 0.5pt + border-color),
            radius: 3pt,
            inset: (x: 6pt, y: 5pt),
            [
              #text(font: font-sans, size: 6.5pt, weight: "bold", fill: ink-subtle)[#upper(m.at(0))] \
              #v(1pt)
              #text(font: font-mono, size: 9.5pt, weight: "bold", fill: ink-primary)[#m.at(1)]
              #if m.len() > 2 and m.at(2) != "" [
                \ #text(font: font-sans, size: 6pt, fill: ink-secondary)[#m.at(2)]
              ]
            ]
          ))
        )
      ]
    )
  }
}

/// Court-Ready Proof / Certificate of Service Block for Statutory Landlord-Tenant Notices
#let certificate-of-service(
  server-name: "Authorized Landlord / Property Manager",
  service-date: "",
  recipient-name: "Tenant(s) and All Occupants",
  premises-address: "",
  state-name: "Tennessee",
  service-method: "personal",
) = {
  let chk-personal = if service-method == "personal" or service-method == "personal_and_mail" { "[X]" } else { "[ ]" }
  let chk-substitute = if service-method == "substitute" { "[X]" } else { "[ ]" }
  let chk-mail = if service-method == "mail" or service-method == "personal_and_mail" { "[X]" } else { "[ ]" }
  let chk-posting = if service-method == "posting" { "[X]" } else { "[ ]" }

  block(breakable: false, width: 100%)[
    #rect(
      width: 100%,
      stroke: 0.75pt + border-dark,
      radius: 3pt,
      fill: bg-tint-subtle,
      inset: 8pt,
      [
        #grid(
          columns: (1fr, auto),
          text(font: font-sans, size: 7.5pt, weight: "bold", fill: navy-judicial)[
            CERTIFICATE & PROOF OF STATUTORY SERVICE (STATE OF #upper(state-name))
          ],
          badge("EVIDENTIARY PROOF OF SERVICE", bg: rgb("#dbeafe"), fg: navy-judicial)
        )
        #v(3pt)
        #text(font: font-sans, size: 7pt, fill: ink-secondary)[
          I, *#server-name*, hereby certify under penalty of perjury that on *#if service-date != "" { service-date } else { "____/____/20____" }*, I served a true and correct copy of the foregoing statutory notice upon *#recipient-name* at *#if premises-address != "" { premises-address } else { "the Premises" }* by the following statutorily authorized method(s):
        ]
        #v(4pt)
        #grid(
          columns: (1fr, 1fr),
          gutter: 4pt,
          text(font: font-sans, size: 6.8pt, fill: ink-primary)[#raw(chk-personal) Personal Hand Delivery to Named Tenant],
          text(font: font-sans, size: 6.8pt, fill: ink-primary)[#raw(chk-substitute) Substitute Service on Competent Household Member],
          text(font: font-sans, size: 6.8pt, fill: ink-primary)[#raw(chk-mail) First-Class / Certified U.S. Mail (Return Receipt)],
          text(font: font-sans, size: 6.8pt, fill: ink-primary)[#raw(chk-posting) Conspicuous Posting on Main Entry Door + Mail],
        )
        #v(8pt)
        #grid(
          columns: (1.5fr, 1fr),
          gutter: 20pt,
          [
            #line(length: 100%, stroke: 0.75pt + border-dark)
            #v(1pt)
            #text(font: font-sans, size: 6pt, fill: ink-subtle)[Signature of Server / Declarant (#server-name)]
          ],
          [
            #line(length: 100%, stroke: 0.75pt + border-dark)
            #v(1pt)
            #text(font: font-sans, size: 6pt, fill: ink-subtle)[Date & Time of Service]
          ]
        )
      ]
    )
  ]
}

/// County Recorder 3.0-Inch Top Margin Header & Stamp Reserve for Recordable Deeds & Liens
#let recording-header-block(
  prepared-by: "",
  return-to: "",
  parcel-id: "",
  state-name: "Tennessee",
  county-name: "Hamilton",
) = {
  rect(
    width: 100%,
    height: 2.4in,
    stroke: 0.75pt + border-dark,
    radius: 3pt,
    fill: rgb("#ffffff"),
    inset: 8pt,
    [
      #grid(
        columns: (1fr, 3.5in),
        gutter: 12pt,
        [
          #text(font: font-sans, size: 7pt, weight: "bold", fill: navy-judicial)[
            COUNTY REGISTER OF DEEDS RECORDING COVER HEADER (#upper(county-name) COUNTY, #upper(state-name))
          ]
          #v(4pt)
          #text(font: font-sans, size: 6.5pt, weight: "bold", fill: ink-subtle)[INSTRUMENT PREPARED BY:] \
          #text(font: font-sans, size: 7.5pt, fill: ink-primary)[#prepared-by]
          #v(6pt)
          #text(font: font-sans, size: 6.5pt, weight: "bold", fill: ink-subtle)[AFTER RECORDING RETURN TO & SEND TAX STATEMENTS TO:] \
          #text(font: font-sans, size: 7.5pt, fill: ink-primary)[#return-to]
          #if parcel-id != "" [
            #v(6pt)
            #text(font: font-mono, size: 7pt, weight: "bold", fill: navy-judicial)[
              TAX PARCEL / APN: #parcel-id
            ]
          ]
        ],
        align(right + top)[
          #date-stamp-zone(width: 3.4in, height: 1.95in)
        ]
      )
      #v(1fr)
      #line(length: 100%, stroke: 1pt + border-dark)
      #v(1pt)
      #align(center, text(font: font-mono, size: 6pt, weight: "bold", fill: ink-subtle)[
        --- SPACE ABOVE THIS LINE RESERVED FOR OFFICIAL COUNTY RECORDER STAMP (3.0-INCH STATUTORY MARGIN) ---
      ])
    ]
  )
}


