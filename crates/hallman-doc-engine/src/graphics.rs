//! Shared Typst Embedded Vector Graphics, 10 Structural Page Layouts, 10 Domain Schematics & Stamps
//!
//! Author: Daniel Hallman
//!
//! Pure-Typst + inline SVG vector graphics and 10 radically distinct structural page layout
//! engines shared across Untax (`untax.io`) and Pressmark (`pressmark.dev`).
//! Zero external runtime dependencies.

/// Shared Typst library of embedded SVG diagrams, charts, rubber stamps, element variations,
/// and 10 structural page layout blueprints.
pub const HALLMAN_EMBEDDED_GRAPHICS_TYP: &str = r##"
// =============================================================================
// HALLMAN DOC ENGINE: EMBEDDED VECTOR GRAPHICS & 10 PAGE LAYOUT ARCHITECTURES
// Author: Daniel Hallman
// =============================================================================

#let hde-font-sans  = ("Helvetica Neue", "Arial", "Inter", "Liberation Sans", "DejaVu Sans")
#let hde-font-serif = ("Georgia", "Libertinus Serif", "Times New Roman", "Liberation Serif", "DejaVu Serif")
#let hde-font-mono  = ("Menlo", "Courier New", "Liberation Mono", "DejaVu Sans Mono")

// -----------------------------------------------------------------------------
// 1. EMBEDDED VECTOR RUBBER STAMPS & TACTILE STICKERS (SVG + TYPST)
// -----------------------------------------------------------------------------
#let svg-rubber-stamp(
  label: "PAID IN FULL",
  sublabel: "VERIFIED RECORD",
  color-hex: "#059669",
  angle: -5deg,
  width-in: 1.52in,
) = {
  let safe-label = label.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
  let safe-sub = sublabel.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
  let main-fs = if safe-label.len() > 16 { "11.5" } else if safe-label.len() > 12 { "13" } else { "14.5" }
  let svg-str = (
    "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 228 66' fill='none'>"
    + "<rect x='3' y='3' width='222' height='60' rx='6' stroke='" + color-hex + "' stroke-width='3'/>"
    + "<rect x='7' y='7' width='214' height='52' rx='3.5' stroke='" + color-hex + "' stroke-width='1.1' stroke-dasharray='5 2.5'/>"
    + "<text x='114' y='32' text-anchor='middle' font-family='Arial, Helvetica, sans-serif' font-size='" + main-fs + "' font-weight='900' letter-spacing='1.3' fill='" + color-hex + "'>" + safe-label + "</text>"
    + "<line x1='20' y1='40' x2='208' y2='40' stroke='" + color-hex + "' stroke-width='1'/>"
    + "<text x='114' y='52' text-anchor='middle' font-family='Courier New, monospace' font-size='8.8' font-weight='700' letter-spacing='1.0' fill='" + color-hex + "'>" + safe-sub + "</text>"
    + "</svg>"
  )
  rotate(angle, image(bytes(svg-str), format: "svg", width: width-in))
}

#let hde-sticker-badge(
  label: "CERTIFIED",
  angle: -2.5deg,
  bg: rgb("#ffe500"),
  fg: rgb("#0f172a"),
) = {
  rotate(angle)[
    #box()[
      #place(top + left, dx: 2.5pt, dy: 3pt)[
        #box(fill: rgb("#0f172a"), radius: 99pt, inset: (x: 10pt, y: 4.5pt))[
          #text(font: hde-font-mono, size: 7.5pt, weight: "black", fill: rgb(0, 0, 0, 0))[#label]
        ]
      ]
      #box(fill: bg, stroke: 1.5pt + rgb("#0f172a"), radius: 99pt, inset: (x: 10pt, y: 4.5pt))[
        #text(font: hde-font-mono, size: 7.5pt, weight: "black", fill: fg, tracking: 0.4pt)[#label]
      ]
    ]
  ]
}

#let hde-depth-card(
  accent-border: rgb("#254b7c"),
  fill-color: rgb("#0f172a"),
  shadow-color: rgb("#0284c7"),
  inset-val: (x: 12pt, y: 10pt),
  body,
) = {
  rect(
    width: 100%,
    fill: fill-color,
    stroke: (
      top: 1.2pt + accent-border,
      left: 1.2pt + accent-border,
      right: 3.5pt + shadow-color,
      bottom: 4pt + shadow-color,
    ),
    radius: 6pt,
    inset: inset-val,
    body
  )
}

// -----------------------------------------------------------------------------
// 1B. NANO BANANA TYPOGRAPHIC SECTION HEADERS & ARCHITECTURAL DIVIDERS (6 MODES)
// -----------------------------------------------------------------------------
#let hde-section-header(
  variant: "numbered_pill_rule",
  sec-num: "01",
  kicker: "SECTION 01 · VERIFIED SCHEDULE",
  title: "SCOPE OF WORK & DELIVERABLES",
  subtitle: "Certified Counterparty Record",
  right-tag: "T.C.A. § 67-5-1407",
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
) = {
  block(breakable: false, width: 100%, above: 3pt, below: 2.5pt)[
    #if variant == "editorial_accent_bar" {
      rect(
        width: 100%,
        stroke: (left: 4pt + accent-color, bottom: 0.6pt + rgb("#cbd5e1")),
        fill: rgb("#f8fafc"),
        inset: (left: 8pt, right: 6pt, y: 3.5pt),
        grid(
          columns: (1fr, auto),
          gutter: 8pt,
          align: (left + horizon, right + bottom),
          [
            #text(font: hde-font-mono, size: 5.8pt, weight: "bold", tracking: 0.6pt, fill: accent-color)[#upper(kicker)] \
            #v(0.5pt)
            #text(font: hde-font-sans, size: 8.6pt, weight: "black", fill: rgb("#0f172a"))[#title]
          ],
          text(font: hde-font-serif, size: 7.2pt, style: "italic", fill: rgb("#334155"))[#subtitle · #right-tag]
        )
      )
    } else if variant == "dark_command_ribbon" {
      rect(
        width: 100%,
        fill: rgb("#0f172a"),
        stroke: (left: 4pt + primary-color),
        radius: (right: 3pt),
        inset: (x: 8pt, y: 4pt),
        grid(
          columns: (1fr, auto),
          gutter: 8pt,
          align: (left + horizon, right + horizon),
          text(font: hde-font-sans, size: 7.8pt, weight: "black", tracking: 0.4pt, fill: white)[
            § #sec-num · #upper(title)
          ],
          box(
            fill: accent-color,
            radius: 99pt,
            inset: (x: 7pt, y: 2pt),
            text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: white)[#upper(right-tag)]
          )
        )
      )
    } else if variant == "centered_double_rule" {
      grid(
        columns: (1fr, auto, 1fr),
        gutter: 8pt,
        align: (center + horizon, center + horizon, center + horizon),
        stack(
          spacing: 1.5pt,
          line(length: 100%, stroke: 1.1pt + primary-color),
          line(length: 100%, stroke: 0.5pt + accent-color),
        ),
        text(font: hde-font-serif, size: 7.8pt, weight: "bold", tracking: 0.7pt, fill: rgb("#0f172a"))[
          ◇ #upper(title) ◇
        ],
        stack(
          spacing: 1.5pt,
          line(length: 100%, stroke: 1.1pt + primary-color),
          line(length: 100%, stroke: 0.5pt + accent-color),
        ),
      )
    } else if variant == "neo_brutalist_tab" {
      stack(
        dir: ttb,
        spacing: 0pt,
        grid(
          columns: (auto, 1fr),
          align: (left + bottom, right + bottom),
          box(
            fill: rgb("#ffe500"),
            stroke: (top: 1.5pt + rgb("#0f172a"), left: 1.5pt + rgb("#0f172a"), right: 2.5pt + rgb("#0f172a")),
            radius: (top: 4pt),
            inset: (x: 8pt, y: 2.8pt),
            text(font: hde-font-mono, size: 6.8pt, weight: "black", fill: rgb("#0f172a"))[
              SEC #sec-num // #upper(title)
            ]
          ),
          box(inset: (bottom: 2pt))[
            #text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: rgb("#334155"))[[REF: #right-tag]]
          ]
        ),
        line(length: 100%, stroke: 1.6pt + rgb("#0f172a"))
      )
    } else if variant == "swiss_index_grid" {
      table(
        columns: (42pt, 1fr, auto),
        stroke: 0.7pt + rgb("#0f172a"),
        inset: (x: 7pt, y: 3.2pt),
        fill: (x, _) => if x == 0 { rgb("#f1f5f9") } else if x == 2 { rgb("#ecfdf5") } else { white },
        align: (center + horizon, left + horizon, right + horizon),
        text(font: hde-font-mono, size: 7.4pt, weight: "black", fill: primary-color)[#sec-num / 04],
        text(font: hde-font-sans, size: 7.6pt, weight: "black", tracking: 0.35pt, fill: rgb("#0f172a"))[#upper(title)],
        text(font: hde-font-mono, size: 7.2pt, weight: "bold", fill: accent-color)[#right-tag],
      )
    } else {
      // Default: "numbered_pill_rule"
      grid(
        columns: (auto, auto, 1fr, auto),
        gutter: 6pt,
        align: (left + horizon, left + horizon, center + horizon, right + horizon),
        box(
          fill: rgb("#0f172a"),
          radius: 99pt,
          inset: (x: 6.5pt, y: 2.2pt),
          text(font: hde-font-mono, size: 6.6pt, weight: "bold", fill: white)[§ #sec-num]
        ),
        text(font: hde-font-sans, size: 7.8pt, weight: "black", tracking: 0.3pt, fill: rgb("#0f172a"))[
          #upper(title)
        ],
        line(length: 100%, stroke: 0.85pt + primary-color),
        box(
          fill: primary-color,
          radius: 99pt,
          inset: (x: 6.5pt, y: 2pt),
          text(font: hde-font-mono, size: 6pt, weight: "bold", fill: white)[#right-tag]
        ),
      )
    }
  ]
}

// -----------------------------------------------------------------------------
// 1C. TACTILE FOLDED-CORNER BRIEF CARD & SIDE-BY-SIDE COMPARISON LANES
// -----------------------------------------------------------------------------
#let hde-folded-brief-card(
  kicker: "VERIFIED BRIEF",
  headline: "$184,350.00",
  items: (),
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
) = {
  let fold-size = 11pt
  block(
    width: 100%,
    fill: rgb("#f8fafc"),
    stroke: 0.75pt + primary-color,
    radius: (top-left: 4pt, bottom-left: 4pt, bottom-right: 4pt, top-right: 0pt),
    inset: (x: 8pt, y: 5pt),
  )[
    #place(top + right, dx: 8pt, dy: -5pt)[
      #polygon(
        fill: rgb("#cbd5e1"),
        stroke: 0.5pt + primary-color,
        (0pt, 0pt),
        (0pt, fold-size),
        (fold-size, fold-size),
      )
    ]
    #text(font: hde-font-mono, size: 5.8pt, weight: "bold", tracking: 0.5pt, fill: primary-color)[#upper(kicker)] \
    #v(1pt)
    #text(font: hde-font-mono, size: 9pt, weight: "black", fill: accent-color)[#headline]
    #v(2pt)
    #line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1"))
    #v(2pt)
    #for item in items [
      #grid(
        columns: (1fr, auto),
        gutter: 4pt,
        align: (left + horizon, right + horizon),
        text(font: hde-font-sans, size: 6.4pt, fill: rgb("#334155"))[
          #text(fill: accent-color, weight: "bold")[✓] #item.at(0)
        ],
        text(font: hde-font-mono, size: 6.5pt, weight: "bold", fill: rgb("#0f172a"))[#item.at(1)]
      )
      #v(1pt)
    ]
  ]
}

#let hde-comparison-lanes(
  left-label: "BASELINE / GROSS RECORD",
  left-val: "$485,000.00",
  left-sub: "Prior / Unadjusted Basis",
  right-label: "CONTENDED / NET CERTIFIED",
  right-val: "$362,500.00",
  right-sub: "Verified Audit Position",
  delta-badge: "VERIFIED DELTA",
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
) = {
  block(breakable: false, width: 100%, above: 2.5pt, below: 2.5pt)[
    #grid(
      columns: (1fr, auto, 1fr),
      gutter: 6pt,
      align: (left + horizon, center + horizon, right + horizon),
      rect(
        width: 100%,
        fill: rgb("#f1f5f9"),
        stroke: (top: 2.2pt + rgb("#64748b"), rest: 0.6pt + rgb("#cbd5e1")),
        radius: 3.5pt,
        inset: (x: 8pt, y: 4.5pt),
        grid(
          columns: (1fr, auto),
          align: (left + horizon, right + horizon),
          [
            #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: rgb("#475569"))[#upper(left-label)] \
            #text(font: hde-font-sans, size: 6.3pt, fill: rgb("#64748b"))[#left-sub]
          ],
          text(font: hde-font-mono, size: 8.8pt, weight: "bold", fill: rgb("#1e293b"))[#left-val]
        )
      ),
      box(
        fill: accent-color,
        radius: 99pt,
        inset: (x: 6.5pt, y: 3pt),
        text(font: hde-font-mono, size: 6pt, weight: "black", fill: white)[⇄ #delta-badge]
      ),
      rect(
        width: 100%,
        fill: rgb("#ecfdf5"),
        stroke: (top: 2.2pt + accent-color, rest: 0.6pt + accent-color),
        radius: 3.5pt,
        inset: (x: 8pt, y: 4.5pt),
        grid(
          columns: (1fr, auto),
          align: (left + horizon, right + horizon),
          [
            #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: accent-color)[#upper(right-label)] \
            #text(font: hde-font-sans, size: 6.3pt, fill: rgb("#047857"))[#right-sub]
          ],
          text(font: hde-font-mono, size: 8.8pt, weight: "black", fill: accent-color)[#right-val]
        )
      )
    )
  ]
}

// -----------------------------------------------------------------------------
// 1D. NANO BANANA TEXT ALIGNMENT, PROSE FLOW & DOT-LEADER ENGINE (5 MODES)
// -----------------------------------------------------------------------------
#let hde-text-alignment-block(
  variant: "two_column_justified",
  org-name: "Issuing Organization",
  counterparty: "Counterparty of Record",
  profession: "Authorized Practitioner",
  citation: "Statutory Standard",
  demand-signal: "Verified operational scope and technical compliance.",
  metrics: (),
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
) = {
  let m0-lbl = if metrics.len() > 0 { metrics.at(0).at(0) } else { "PRIMARY BASIS" }
  let m0-val = if metrics.len() > 0 { metrics.at(0).at(1) } else { "$0.00" }
  let m1-lbl = if metrics.len() > 1 { metrics.at(1).at(0) } else { "ALLOCATION" }
  let m1-val = if metrics.len() > 1 { metrics.at(1).at(1) } else { "$0.00" }
  let m2-lbl = if metrics.len() > 2 { metrics.at(2).at(0) } else { "ADJUSTMENT" }
  let m2-val = if metrics.len() > 2 { metrics.at(2).at(1) } else { "$0.00" }
  let m3-lbl = if metrics.len() > 3 { metrics.at(3).at(0) } else { "NET CERTIFIED" }
  let m3-val = if metrics.len() > 3 { metrics.at(3).at(1) } else { "$0.00" }

  block(breakable: false, width: 100%, above: 2.5pt, below: 2.5pt)[
    #if variant == "hanging_indent_clauses" {
      // Mode 2: Hanging-Indent Numbered Recitals + Right-Aligned Margin Citations
      rect(
        width: 100%,
        fill: rgb("#ffffff"),
        stroke: 0.6pt + rgb("#cbd5e1"),
        radius: 3.5pt,
        inset: (x: 8pt, y: 5pt),
        [
          #grid(
            columns: (26pt, 1fr, 108pt),
            row-gutter: 4pt,
            column-gutter: 6pt,
            align: (left + top, left + top, right + top),
            text(font: hde-font-mono, size: 6.8pt, weight: "black", fill: primary-color)[¶ 01],
            par(justify: true, leading: 0.44em)[
              #text(font: hde-font-sans, size: 6.8pt, fill: rgb("#1e293b"))[
                *JURISDICTIONAL & TECHNICAL BASIS:* *#org-name* (#profession) certifies that all line items and measurements for *#counterparty* conform to governing authority.
              ]
            ],
            box(fill: rgb("#f1f5f9"), radius: 2.5pt, inset: (x: 5pt, y: 2pt))[
              #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: primary-color)[[#citation]]
            ],
            text(font: hde-font-mono, size: 6.8pt, weight: "black", fill: accent-color)[¶ 02],
            par(justify: true, leading: 0.44em)[
              #text(font: hde-font-sans, size: 6.8pt, fill: rgb("#1e293b"))[
                *VERIFIED RECONCILIATION:* #demand-signal Baseline *#m0-lbl* (#m0-val) reconciles to net *#m3-lbl* (#m3-val).
              ]
            ],
            box(fill: rgb("#ecfdf5"), radius: 2.5pt, inset: (x: 5pt, y: 2pt))[
              #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: accent-color)[[NET: #m3-val]]
            ],
          )
        ]
      )
    } else if variant == "asymmetric_callout_split" {
      // Mode 3: Asymmetric 62% Left-Aligned Narrative + 38% Right Folded-Corner Brief Card
      grid(
        columns: (1.55fr, 1fr),
        gutter: 8pt,
        align: (left + horizon, left + horizon),
        rect(
          width: 100%,
          fill: rgb("#ffffff"),
          stroke: (left: 2.5pt + primary-color, rest: 0.5pt + rgb("#cbd5e1")),
          radius: (right: 3.5pt),
          inset: (x: 8pt, y: 5pt),
          [
            #text(font: hde-font-mono, size: 6pt, weight: "bold", tracking: 0.5pt, fill: primary-color)[
              EXECUTIVE SCOPE & ALIGNED NARRATIVE SUMMARY
            ] \
            #v(1.5pt)
            #par(justify: false, leading: 0.45em)[
              #text(font: hde-font-sans, size: 6.8pt, fill: rgb("#1e293b"))[
                Prepared by *#org-name* for *#counterparty* under *#citation*. #demand-signal
              ]
            ]
            #v(2pt)
            #text(font: hde-font-sans, size: 6.5pt, fill: rgb("#334155"))[
              #text(fill: primary-color, weight: "bold")[•] Primary Basis (*#m0-lbl*): *#m0-val* #h(6pt)
              #text(fill: accent-color, weight: "bold")[•] Target (*#m3-lbl*): *#m3-val*
            ]
          ]
        ),
        hde-folded-brief-card(
          kicker: m3-lbl,
          headline: m3-val,
          items: ((m0-lbl, m0-val), (m1-lbl, m1-val), (m2-lbl, m2-val)),
          primary-color: primary-color,
          accent-color: accent-color,
        )
      )
    } else if variant == "tabular_dot_leader_ledger" {
      // Mode 4: Tabular Dot-Leader Financial Ledger (Left Label + . . . + Right-Aligned Tabular Figures)
      rect(
        width: 100%,
        fill: rgb("#fffdf9"),
        stroke: 0.65pt + rgb("#cbd5e1"),
        radius: 3.5pt,
        inset: (x: 9pt, y: 5pt),
        [
          #grid(
            columns: (auto, 1fr, auto),
            row-gutter: 3pt,
            column-gutter: 4pt,
            align: (left + bottom, left + bottom, right + bottom),
            text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: rgb("#334155"))[[01] #m0-lbl (#org-name)],
            box(width: 100%, repeat(text(font: hde-font-mono, size: 6.4pt, fill: rgb("#94a3b8"))[. ])),
            text(font: hde-font-mono, size: 7pt, weight: "bold", fill: rgb("#0f172a"))[#m0-val],
            text(font: hde-font-sans, size: 6.8pt, fill: rgb("#334155"))[[02] #m1-lbl · #m2-lbl (#m2-val)],
            box(width: 100%, repeat(text(font: hde-font-mono, size: 6.4pt, fill: rgb("#94a3b8"))[. ])),
            text(font: hde-font-mono, size: 7pt, weight: "bold", fill: primary-color)[#m1-val],
            text(font: hde-font-sans, size: 7pt, weight: "black", fill: accent-color)[[03] VERIFIED NET POSITION — #upper(m3-lbl)],
            box(width: 100%, repeat(text(font: hde-font-mono, size: 6.4pt, fill: accent-color)[. ])),
            box(stroke: (top: 0.6pt + rgb("#0f172a"), bottom: 1.4pt + accent-color), inset: (y: 1pt))[
              #text(font: hde-font-mono, size: 7.4pt, weight: "black", fill: accent-color)[#m3-val]
            ],
          )
        ]
      )
    } else if variant == "centered_recital_quote" {
      // Mode 5: Centered Pull-Quote / Statutory Proclamation with Flanking Rules
      rect(
        width: 100%,
        fill: rgb("#f8fafc"),
        stroke: (top: 1pt + primary-color, bottom: 1pt + primary-color),
        inset: (x: 14pt, y: 4.5pt),
        align(center)[
          #text(font: hde-font-serif, size: 7.3pt, style: "italic", fill: rgb("#0f172a"))[
            “Pursuant to *#citation*, *#org-name* certifies this official schedule for *#counterparty*. #demand-signal”
          ] \
          #v(1.5pt)
          #text(font: hde-font-mono, size: 6pt, weight: "bold", tracking: 0.6pt, fill: accent-color)[
            — CERTIFIED RECORD · #upper(m0-lbl): #m0-val · #upper(m3-lbl): #m3-val —
          ]
        ]
      )
    } else {
      // Default Mode 1: "two_column_justified" (Two-Column Justified Covenants with Gutter Rule)
      rect(
        width: 100%,
        fill: rgb("#ffffff"),
        stroke: 0.6pt + rgb("#cbd5e1"),
        radius: 3.5pt,
        inset: (x: 8pt, y: 4.5pt),
        grid(
          columns: (1fr, 1fr),
          gutter: 10pt,
          rect(
            width: 100%,
            stroke: (right: 0.5pt + rgb("#cbd5e1")),
            inset: (right: 6pt, y: 0pt),
            par(justify: true, leading: 0.44em)[
              #text(font: hde-font-serif, size: 6.7pt, fill: rgb("#1e293b"))[
                *WHEREAS, THE UNDERSIGNED* (*#org-name*, acting as *#profession*) and *#counterparty* enter into this verified record pursuant to *#citation*, establishing *#m0-lbl* at *#m0-val*.
              ]
            ]
          ),
          par(justify: true, leading: 0.44em)[
            #text(font: hde-font-serif, size: 6.7pt, fill: rgb("#1e293b"))[
              *FURTHER COVENANTED:* #demand-signal Upon reconciliation of *#m1-lbl* (*#m1-val*) and *#m2-lbl* (*#m2-val*), the certified *#m3-lbl* is fixed at *#m3-val*.
            ]
          ]
        )
      )
    }
  ]
}

// -----------------------------------------------------------------------------
// 2. 10 EMBEDDED ARCHITECTURAL, TECHNICAL & DOMAIN SCHEMATICS (SVG)
// -----------------------------------------------------------------------------
#let svg-architectural-diagram(
  kind: "parcel_lot",
  title: "TECHNICAL SCHEMATIC & VERIFICATION DIAGRAM",
  primary-hex: "#1e3a8a",
  accent-hex: "#d97706",
  metric-a: "Primary Metric",
  metric-b: "Secondary Metric",
  hero-height: auto,
) = {
  let body-svg = if kind == "parcel_lot" or kind == "parcel_plat_svg" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 112' fill='none'>"
      + "<rect x='2' y='2' width='536' height='108' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<g stroke='#e2e8f0' stroke-width='0.8'>"
      + "<line x1='0' y1='28' x2='540' y2='28'/><line x1='0' y1='56' x2='540' y2='56'/><line x1='0' y1='84' x2='540' y2='84'/>"
      + "<line x1='90' y1='0' x2='90' y2='112'/><line x1='180' y1='0' x2='180' y2='112'/><line x1='270' y1='0' x2='270' y2='112'/><line x1='360' y1='0' x2='360' y2='112'/><line x1='450' y1='0' x2='450' y2='112'/>"
      + "</g>"
      + "<polygon points='42,18 338,15 354,94 30,96' fill='#dbeafe' fill-opacity='0.55' stroke='" + primary-hex + "' stroke-width='2.2'/>"
      + "<polygon points='84,32 290,30 296,80 80,82' fill='#ffffff' stroke='" + accent-hex + "' stroke-width='1.8' stroke-dasharray='4 2'/>"
      + "<text x='188' y='52' text-anchor='middle' font-family='Arial, sans-serif' font-size='9.2' font-weight='bold' fill='#0f172a'>IMPROVED GLA FOOTPRINT</text>"
      + "<text x='188' y='67' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='" + primary-hex + "'>SETBACK: 25 FT FRONT / 10 FT SIDE</text>"
      + "<text x='188' y='12' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='" + primary-hex + "'>N 88°14'20\" E — 142.50 FT</text>"
      + "<text x='190' y='106' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='" + primary-hex + "'>S 88°14'20\" W — 148.00 FT (R.O.W.)</text>"
      + "<circle cx='465' cy='42' r='20' fill='#ffffff' stroke='" + primary-hex + "' stroke-width='1.5'/>"
      + "<polygon points='465,24 470,44 465,40 460,44' fill='" + primary-hex + "'/>"
      + "<text x='465' y='19' text-anchor='middle' font-family='Arial, sans-serif' font-size='7.8' font-weight='bold' fill='" + primary-hex + "'>N</text>"
      + "<text x='465' y='82' text-anchor='middle' font-family='Courier New, monospace' font-size='8.2' font-weight='bold' fill='#0f172a'>NAD83 / STATE PLANE</text>"
      + "<text x='465' y='95' text-anchor='middle' font-family='Courier New, monospace' font-size='7.6' fill='#475569'>CADASTRAL LOT PLAT</text>"
      + "</svg>"
    )
  } else if kind == "roof_pitch" or kind == "roof_facet_map" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 112' fill='none'>"
      + "<rect x='2' y='2' width='536' height='108' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<polygon points='40,86 175,22 300,86' fill='#e0f2fe' stroke='" + primary-hex + "' stroke-width='2'/>"
      + "<polygon points='175,22 375,22 465,86 300,86' fill='#fef3c7' stroke='" + accent-hex + "' stroke-width='2'/>"
      + "<line x1='175' y1='22' x2='175' y2='86' stroke='#64748b' stroke-width='1.2' stroke-dasharray='4 3'/>"
      + "<circle cx='286' cy='38' r='5.5' fill='#dc2626' fill-opacity='0.22' stroke='#dc2626' stroke-width='1.5'/>"
      + "<circle cx='336' cy='46' r='6' fill='#dc2626' fill-opacity='0.22' stroke='#dc2626' stroke-width='1.5'/>"
      + "<circle cx='378' cy='38' r='5' fill='#dc2626' fill-opacity='0.22' stroke='#dc2626' stroke-width='1.5'/>"
      + "<text x='162' y='76' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.2' font-weight='bold' fill='" + primary-hex + "'>FACET A (7/12 PITCH)</text>"
      + "<text x='364' y='76' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.0' font-weight='bold' fill='#92400e'>FACET B — HAIL IMPACT (12 SQ)</text>"
      + "<text x='275' y='15' text-anchor='middle' font-family='Courier New, monospace' font-size='8.2' font-weight='bold' fill='#0f172a'>RIDGE VENT: 48 LF • DRIP EDGE &amp; ICE/WATER SHIELD</text>"
      + "<text x='275' y='102' text-anchor='middle' font-family='Courier New, monospace' font-size='8' fill='#334155'>TOTAL ROOF AREA: 3,420 SQ FT (34.2 SQUARES) + 12% WASTE FACTOR</text>"
      + "</svg>"
    )
  } else if kind == "hvac_diagnostic" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 112' fill='none'>"
      + "<rect x='2' y='2' width='536' height='108' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<rect x='24' y='20' width='126' height='72' rx='5' fill='#ffffff' stroke='" + primary-hex + "' stroke-width='2'/>"
      + "<text x='87' y='40' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.5' font-weight='bold' fill='" + primary-hex + "'>RETURN PLENUM</text>"
      + "<text x='87' y='57' text-anchor='middle' font-family='Courier New, monospace' font-size='8' fill='#0f172a'>TEMP: 74.2°F DB</text>"
      + "<text x='87' y='72' text-anchor='middle' font-family='Courier New, monospace' font-size='7.8' fill='#475569'>STATIC: -0.18 IWC</text>"
      + "<path d='M154 56 L198 56' stroke='" + accent-hex + "' stroke-width='3' stroke-dasharray='5 3'/>"
      + "<rect x='202' y='16' width='136' height='80' rx='6' fill='#eff6ff' stroke='" + primary-hex + "' stroke-width='2.2'/>"
      + "<text x='270' y='35' text-anchor='middle' font-family='Arial, sans-serif' font-size='9' font-weight='bold' fill='#0f172a'>AIR HANDLER / COIL</text>"
      + "<text x='270' y='51' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#059669'>R-410A SUBCOOL: 11.4°F</text>"
      + "<text x='270' y='66' text-anchor='middle' font-family='Courier New, monospace' font-size='8' fill='#0f172a'>BLOWER: 1,620 CFM</text>"
      + "<text x='270' y='81' text-anchor='middle' font-family='Courier New, monospace' font-size='7.5' fill='#dc2626'>CAPACITOR: 45/5 MFD PASS</text>"
      + "<path d='M342 56 L386 56' stroke='" + primary-hex + "' stroke-width='3'/>"
      + "<rect x='390' y='20' width='126' height='72' rx='5' fill='#ffffff' stroke='#059669' stroke-width='2'/>"
      + "<text x='453' y='40' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.5' font-weight='bold' fill='#059669'>SUPPLY REGISTER</text>"
      + "<text x='453' y='57' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#0f172a'>SUPPLY: 54.6°F DB</text>"
      + "<text x='453' y='72' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#059669'>DELTA-T: 19.6°F PASS</text>"
      + "</svg>"
    )
  } else if kind == "solar_array" or kind == "solar_string_layout" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 112' fill='none'>"
      + "<rect x='2' y='2' width='536' height='108' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<rect x='24' y='24' width='160' height='72' rx='4' fill='#1e3a8a' stroke='#0f172a' stroke-width='1.5'/>"
      + "<g stroke='#93c5fd' stroke-width='1'>"
      + "<line x1='64' y1='24' x2='64' y2='96'/><line x1='104' y1='24' x2='104' y2='96'/><line x1='144' y1='24' x2='144' y2='96'/>"
      + "<line x1='24' y1='48' x2='184' y2='48'/><line x1='24' y1='72' x2='184' y2='72'/>"
      + "</g>"
      + "<text x='104' y='16' text-anchor='middle' font-family='Courier New, monospace' font-size='7.4' font-weight='bold' fill='#0f172a'>24x 440W BIFACIAL ARRAY (10.56 kW)</text>"
      + "<rect x='214' y='28' width='120' height='62' rx='5' fill='#ffffff' stroke='#059669' stroke-width='2'/>"
      + "<text x='274' y='46' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.2' font-weight='bold' fill='#059669'>HYBRID INVERTER + LFP</text>"
      + "<text x='274' y='61' text-anchor='middle' font-family='Courier New, monospace' font-size='7.8' fill='#0f172a'>97.8% CEC EFFICIENCY</text>"
      + "<text x='274' y='76' text-anchor='middle' font-family='Courier New, monospace' font-size='7.8' fill='#475569'>13.5 kWh STORAGE</text>"
      + "<rect x='362' y='18' width='154' height='78' rx='5' fill='#ecfdf5' stroke='#059669' stroke-width='1.8'/>"
      + "<text x='439' y='36' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.2' font-weight='bold' fill='#047857'>25-YR PRODUCTION &amp; ITC</text>"
      + "<text x='439' y='52' text-anchor='middle' font-family='Courier New, monospace' font-size='9' font-weight='bold' fill='#0f172a'>15,840 kWh / YEAR 1</text>"
      + "<text x='439' y='68' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#059669'>30% IRC § 25D CREDIT</text>"
      + "<text x='439' y='83' text-anchor='middle' font-family='Courier New, monospace' font-size='7.5' fill='#334155'>OFFSET: 94% UTILITY LOAD</text>"
      + "</svg>"
    )
  } else if kind == "vehicle_inspection" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 112' fill='none'>"
      + "<rect x='2' y='2' width='536' height='108' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<rect x='155' y='24' width='230' height='64' rx='12' fill='#ffffff' stroke='" + primary-hex + "' stroke-width='2'/>"
      + "<rect x='185' y='32' width='170' height='48' rx='5' fill='#f1f5f9' stroke='#64748b' stroke-width='1.2'/>"
      + "<rect x='172' y='14' width='38' height='10' rx='2.5' fill='#059669'/><rect x='330' y='14' width='38' height='10' rx='2.5' fill='#059669'/>"
      + "<rect x='172' y='88' width='38' height='10' rx='2.5' fill='#d97706'/><rect x='330' y='88' width='38' height='10' rx='2.5' fill='#059669'/>"
      + "<text x='18' y='38' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#059669'>[PASS] LF BRAKE: 8mm</text>"
      + "<text x='18' y='82' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#d97706'>[WARN] LR ROTOR: 3mm</text>"
      + "<text x='396' y='38' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#059669'>[PASS] RF BRAKE: 8mm</text>"
      + "<text x='396' y='82' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#059669'>[PASS] BATT CCA: 740A</text>"
      + "<text x='270' y='59' text-anchor='middle' font-family='Arial, sans-serif' font-size='8' font-weight='bold' fill='#0f172a'>CHASSIS &amp; POWERTRAIN TELEMETRY</text>"
      + "</svg>"
    )
  } else if kind == "chain_of_custody" or kind == "chain_of_custody_flow" or kind == "medical_claim_flow" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 112' fill='none'>"
      + "<rect x='2' y='2' width='536' height='108' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<rect x='18' y='22' width='112' height='68' rx='5' fill='#ffffff' stroke='" + primary-hex + "' stroke-width='2'/>"
      + "<text x='74' y='44' text-anchor='middle' font-family='Arial, sans-serif' font-size='8' font-weight='bold' fill='" + primary-hex + "'>01. COLLECTION</text>"
      + "<text x='74' y='59' text-anchor='middle' font-family='Courier New, monospace' font-size='7.5' fill='#0f172a'>TAMPER SEAL #A91</text>"
      + "<text x='74' y='73' text-anchor='middle' font-family='Courier New, monospace' font-size='7.2' fill='#059669'>TEMP: 2.4°C PASS</text>"
      + "<line x1='130' y1='56' x2='152' y2='56' stroke='" + primary-hex + "' stroke-width='2.5'/>"
      + "<rect x='152' y='22' width='112' height='68' rx='5' fill='#eff6ff' stroke='" + primary-hex + "' stroke-width='2'/>"
      + "<text x='208' y='44' text-anchor='middle' font-family='Arial, sans-serif' font-size='8' font-weight='bold' fill='#0f172a'>02. INTAKE &amp; LOG</text>"
      + "<text x='208' y='59' text-anchor='middle' font-family='Courier New, monospace' font-size='7.5' fill='" + primary-hex + "'>ISO-17025 LIMS</text>"
      + "<text x='208' y='73' text-anchor='middle' font-family='Courier New, monospace' font-size='7.2' fill='#475569'>BARCODE VERIFIED</text>"
      + "<line x1='264' y1='56' x2='286' y2='56' stroke='" + accent-hex + "' stroke-width='2.5'/>"
      + "<rect x='286' y='22' width='112' height='68' rx='5' fill='#ffffff' stroke='" + accent-hex + "' stroke-width='2'/>"
      + "<text x='342' y='44' text-anchor='middle' font-family='Arial, sans-serif' font-size='8' font-weight='bold' fill='#92400e'>03. ASSAY / QA</text>"
      + "<text x='342' y='59' text-anchor='middle' font-family='Courier New, monospace' font-size='7.5' fill='#0f172a'>HPLC / MASS SPEC</text>"
      + "<text x='342' y='73' text-anchor='middle' font-family='Courier New, monospace' font-size='7.2' fill='#059669'>PURITY: 99.84%</text>"
      + "<line x1='398' y1='56' x2='416' y2='56' stroke='#059669' stroke-width='2.5'/>"
      + "<rect x='416' y='22' width='106' height='68' rx='5' fill='#ecfdf5' stroke='#059669' stroke-width='2'/>"
      + "<text x='469' y='44' text-anchor='middle' font-family='Arial, sans-serif' font-size='8' font-weight='bold' fill='#047857'>04. CERTIFIED</text>"
      + "<text x='469' y='59' text-anchor='middle' font-family='Courier New, monospace' font-size='7.5' font-weight='bold' fill='#0f172a'>COA RELEASED</text>"
      + "<text x='469' y='73' text-anchor='middle' font-family='Courier New, monospace' font-size='7.2' fill='#047857'>21 CFR PART 11</text>"
      + "</svg>"
    )
  } else if kind == "cap_table_waterfall" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 112' fill='none'>"
      + "<rect x='2' y='2' width='536' height='108' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<rect x='22' y='24' width='200' height='36' rx='4' fill='" + primary-hex + "'/>"
      + "<text x='122' y='46' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.5' font-weight='bold' fill='#ffffff'>FOUNDERS COMMON (68.0%)</text>"
      + "<rect x='226' y='24' width='110' height='36' rx='4' fill='" + accent-hex + "'/>"
      + "<text x='281' y='46' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.2' font-weight='bold' fill='#ffffff'>SAFE / SEED (20.0%)</text>"
      + "<rect x='340' y='24' width='80' height='36' rx='4' fill='#059669'/>"
      + "<text x='380' y='46' text-anchor='middle' font-family='Arial, sans-serif' font-size='8' font-weight='bold' fill='#ffffff'>ESOP (12.0%)</text>"
      + "<rect x='426' y='20' width='94' height='74' rx='5' fill='#ffffff' stroke='" + primary-hex + "' stroke-width='1.5'/>"
      + "<text x='473' y='40' text-anchor='middle' font-family='Courier New, monospace' font-size='7.5' font-weight='bold' fill='" + primary-hex + "'>POST-MONEY CAP</text>"
      + "<text x='473' y='56' text-anchor='middle' font-family='Arial, sans-serif' font-size='9.5' font-weight='bold' fill='#0f172a'>10,000,000</text>"
      + "<text x='473' y='71' text-anchor='middle' font-family='Courier New, monospace' font-size='7.2' fill='#475569'>FULLY DILUTED</text>"
      + "<text x='473' y='84' text-anchor='middle' font-family='Courier New, monospace' font-size='7.2' font-weight='bold' fill='#059669'>IRC § 409A SAFE</text>"
      + "<text x='220' y='84' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#0f172a'>PRO-RATA DILUTION WATERFALL &amp; 1X NON-PARTICIPATING PREF</text>"
      + "</svg>"
    )
  } else if kind == "logistics_route" or kind == "logistics_route_map" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 112' fill='none'>"
      + "<rect x='2' y='2' width='536' height='108' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<path d='M55 68 Q 180 22, 275 56 T 485 44' stroke='" + primary-hex + "' stroke-width='3' stroke-dasharray='6 4' fill='none'/>"
      + "<circle cx='55' cy='68' r='8' fill='" + primary-hex + "'/>"
      + "<text x='55' y='92' text-anchor='middle' font-family='Courier New, monospace' font-size='7.8' font-weight='bold' fill='#0f172a'>ORIGIN DOCK</text>"
      + "<circle cx='275' cy='56' r='8' fill='" + accent-hex + "'/>"
      + "<text x='275' y='80' text-anchor='middle' font-family='Courier New, monospace' font-size='7.8' font-weight='bold' fill='#92400e'>CUSTOMS / HUB</text>"
      + "<circle cx='485' cy='44' r='8' fill='#059669'/>"
      + "<text x='485' y='68' text-anchor='middle' font-family='Courier New, monospace' font-size='7.8' font-weight='bold' fill='#047857'>CONSIGNEE POD</text>"
      + "<text x='270' y='18' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.2' font-weight='bold' fill='" + primary-hex + "'>INTERMODAL CORRIDOR • SEAL VERIFIED • GPS TELEMETRY</text>"
      + "</svg>"
    )
  } else if kind == "soil_topography" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 112' fill='none'>"
      + "<rect x='2' y='2' width='536' height='108' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<path d='M16 78 Q 140 34, 270 62 T 524 40 L 524 96 L 16 96 Z' fill='#dcfce7' stroke='#15803d' stroke-width='2'/>"
      + "<path d='M16 90 Q 160 64, 290 78 T 524 68 L 524 96 L 16 96 Z' fill='#fef3c7' stroke='#b45309' stroke-width='1.5'/>"
      + "<text x='145' y='44' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.2' font-weight='bold' fill='#15803d'>CLASS II PRIME LOAM (48 AC)</text>"
      + "<text x='390' y='34' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.2' font-weight='bold' fill='#b45309'>CLASS VI STEEP KARST (94 AC)</text>"
      + "<text x='270' y='16' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#0f172a'>USDA NRCS SSURGO SOIL HORIZON &amp; USGS 3DEP LIDAR SLOPE PROFILE</text>"
      + "</svg>"
    )
  } else {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 112' fill='none'>"
      + "<rect x='2' y='2' width='536' height='108' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<rect x='24' y='18' width='190' height='76' fill='#ffffff' stroke='" + primary-hex + "' stroke-width='2'/>"
      + "<rect x='214' y='18' width='150' height='76' fill='#eff6ff' stroke='" + primary-hex + "' stroke-width='2'/>"
      + "<rect x='364' y='18' width='152' height='76' fill='#fef3c7' stroke='" + accent-hex + "' stroke-width='2'/>"
      + "<text x='119' y='50' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.8' font-weight='bold' fill='#0f172a'>ZONE A — MAIN SUITE</text>"
      + "<text x='119' y='66' text-anchor='middle' font-family='Courier New, monospace' font-size='8' fill='" + primary-hex + "'>1,850 RSF / 1,610 USF</text>"
      + "<text x='289' y='50' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.8' font-weight='bold' fill='#0f172a'>ZONE B — LAB / OPS</text>"
      + "<text x='289' y='66' text-anchor='middle' font-family='Courier New, monospace' font-size='8' fill='" + primary-hex + "'>1,120 RSF • MEP TIER 2</text>"
      + "<text x='440' y='50' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.8' font-weight='bold' fill='#92400e'>ZONE C — COMMON / ADA</text>"
      + "<text x='440' y='66' text-anchor='middle' font-family='Courier New, monospace' font-size='8' fill='#92400e'>640 RSF • LOAD FACTOR 14%</text>"
      + "</svg>"
    )
  }

  block(breakable: false, width: 100%, above: 4pt, below: 4pt)[
    #rect(
      width: 100%,
      stroke: 0.75pt + rgb("#cbd5e1"),
      radius: 4pt,
      fill: rgb("#ffffff"),
      inset: (x: 9pt, y: 6pt),
      [
        #grid(
          columns: (1fr, auto),
          gutter: 6pt,
          align: (left + horizon, right + horizon),
          text(font: hde-font-sans, size: 7.2pt, weight: "bold", tracking: 0.4pt, fill: rgb(primary-hex))[
            SCHEMATIC • #upper(title)
          ],
          box(
            fill: rgb("#ecfdf5"),
            radius: 3pt,
            inset: (x: 6pt, y: 2pt),
            text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: rgb("#047857"))[#metric-b]
          )
        )
        #v(3pt)
        #if hero-height != auto {
          image(bytes(body-svg), format: "svg", width: 100%, height: hero-height, fit: "contain")
        } else {
          image(bytes(body-svg), format: "svg", width: 100%)
        }
      ]
    )
  ]
}

// -----------------------------------------------------------------------------
// 3. EMBEDDED HORIZONTAL COMPARATIVE BAR CHART
// -----------------------------------------------------------------------------
#let hde-comparative-bar-chart(
  title: "COMPARATIVE VALUATION & COST ALLOCATION",
  rows: (),
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
) = {
  block(breakable: false, width: 100%, above: 4pt, below: 4pt)[
    #rect(
      width: 100%,
      stroke: 0.75pt + rgb("#cbd5e1"),
      radius: 4pt,
      fill: rgb("#f8fafc"),
      inset: (x: 10pt, y: 7pt),
      [
        #text(font: hde-font-sans, size: 7.2pt, weight: "bold", tracking: 0.4pt, fill: primary-color)[
          ALLOCATION • #upper(title)
        ]
        #v(4pt)
        #for row in rows [
          #let pct = row.at(2)
          #let bar-fill = if row.len() > 3 and row.at(3) == "accent" { accent-color } else { primary-color }
          #grid(
            columns: (1.2fr, 0.85fr, auto),
            gutter: 6pt,
            align: (left + horizon, left + horizon, right + horizon),
            text(font: hde-font-sans, size: 7.4pt, weight: "bold", fill: rgb("#1e293b"))[#row.at(0)],
            box(width: 100%, height: 7.5pt, fill: rgb("#e2e8f0"), radius: 3pt)[
              #box(width: pct * 1%, height: 7.5pt, fill: bar-fill, radius: 3pt)
            ],
            text(font: hde-font-mono, size: 7.4pt, weight: "bold", fill: rgb("#0f172a"))[#row.at(1)]
          )
          #v(3pt)
        ]
      ]
    )
  ]
}

// -----------------------------------------------------------------------------
// 4. EMBEDDED MILESTONE / PHASE TIMELINE STRIP
// -----------------------------------------------------------------------------
#let hde-milestone-timeline(
  title: "EXECUTION MILESTONES & PAYMENT / FILING SCHEDULE",
  steps: (),
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
) = {
  block(breakable: false, width: 100%, above: 4pt, below: 4pt)[
    #rect(
      width: 100%,
      stroke: 0.75pt + rgb("#cbd5e1"),
      radius: 4pt,
      fill: rgb("#ffffff"),
      inset: (x: 9pt, y: 6pt),
      [
        #text(font: hde-font-sans, size: 7.2pt, weight: "bold", tracking: 0.4pt, fill: primary-color)[
          TIMELINE • #upper(title)
        ]
        #v(4pt)
        #grid(
          columns: (1fr,) * steps.len(),
          gutter: 7pt,
          ..steps.enumerate().map(((idx, step)) => {
            let is-active = (idx == 0)
            rect(
              width: 100%,
              fill: if is-active { rgb("#eff6ff") } else { rgb("#f8fafc") },
              stroke: (top: 2.2pt + if is-active { primary-color } else { accent-color }, rest: 0.5pt + rgb("#cbd5e1")),
              radius: 3pt,
              inset: (x: 7pt, y: 5pt),
              [
                #text(font: hde-font-mono, size: 6.6pt, weight: "bold", fill: if is-active { primary-color } else { accent-color })[
                  PHASE #(idx + 1) • #step.at(2)
                ] \
                #v(1.5pt)
                #text(font: hde-font-sans, size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[#step.at(0)] \
                #v(1pt)
                #text(font: hde-font-sans, size: 7pt, fill: rgb("#475569"))[#step.at(1)]
              ]
            )
          })
        )
      ]
    )
  ]
}

// -----------------------------------------------------------------------------
// 5. 6-VARIANT MASTHEAD ENGINE
// -----------------------------------------------------------------------------
#let hde-masthead-variation(
  variant: "executive_split",
  title: "OFFICIAL INSTRUMENT",
  subtitle: "",
  org-name: "Hallman Document Engine",
  vertical-badge: "CERTIFIED ARCHETYPE",
  doc-ref: "DOC-2026-001",
  date-str: "2026-11-06",
  citation: "",
  seal-path: none,
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#d97706"),
) = {
  let seal-node = if seal-path != none and seal-path != "" {
    box(width: 46pt, height: 46pt, image(seal-path, width: 46pt, height: 46pt, fit: "contain"))
  } else {
    none
  }

  if variant == "neo_brutalist" {
    rect(
      width: 100%,
      fill: rgb("#ffe500"),
      stroke: 2.2pt + rgb("#000000"),
      inset: (x: 11pt, y: 8pt),
      [
        #grid(
          columns: (if seal-node != none { 50pt } else { 0pt }, 1fr, 150pt),
          gutter: if seal-node != none { 9pt } else { 0pt },
          align: (left + horizon, left + horizon, right + top),
          [#if seal-node != none { seal-node }],
          [
            #box(fill: rgb("#000000"), inset: (x: 6pt, y: 2.5pt))[
              #text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: rgb("#ffe500"))[#upper(vertical-badge)]
            ]
            #v(3pt)
            #text(font: hde-font-sans, size: 12.8pt, weight: "black", fill: rgb("#000000"))[#upper(title)] \
            #v(1pt)
            #text(font: hde-font-sans, size: 7.6pt, weight: "bold", fill: rgb("#1e293b"))[#subtitle]
          ],
          rect(
            width: 100%,
            fill: rgb("#ffffff"),
            stroke: 1.5pt + rgb("#000000"),
            inset: (x: 8pt, y: 6pt),
            align(right)[
              #text(font: hde-font-mono, size: 7pt, weight: "bold", fill: rgb("#000000"))[REF: #doc-ref] \
              #text(font: hde-font-mono, size: 6.6pt, fill: rgb("#334155"))[DATE: #date-str] \
              #text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: primary-color)[#org-name]
            ]
          )
        )
      ]
    )
  } else if variant == "swiss_grid" {
    table(
      columns: (if seal-node != none { 56pt } else { 0pt }, 1fr, 155pt),
      stroke: 0.75pt + rgb("#0f172a"),
      inset: (x: 9pt, y: 7pt),
      align: (center + horizon, left + horizon, left + top),
      [#if seal-node != none { seal-node }],
      [
        #text(font: hde-font-mono, size: 6.6pt, weight: "bold", tracking: 0.8pt, fill: primary-color)[01 :: #upper(vertical-badge)] \
        #v(2pt)
        #text(font: hde-font-sans, size: 12.5pt, weight: "bold", fill: rgb("#0f172a"))[#title] \
        #v(1.5pt)
        #text(font: hde-font-sans, size: 7.6pt, fill: rgb("#475569"))[#subtitle]
      ],
      [
        #text(font: hde-font-mono, size: 6.4pt, weight: "bold", fill: rgb("#64748b"))[ISSUING ENTITY] \
        #text(font: hde-font-sans, size: 7.6pt, weight: "bold", fill: rgb("#0f172a"))[#org-name] \
        #v(2.5pt)
        #text(font: hde-font-mono, size: 6.6pt, fill: primary-color)[ID: #doc-ref · #date-str]
      ]
    )
  } else if variant == "judicial_caption" {
    rect(
      width: 100%,
      stroke: (top: 2.2pt + primary-color, bottom: 1.2pt + primary-color, left: 0.75pt + rgb("#94a3b8"), right: 0.75pt + rgb("#94a3b8")),
      fill: rgb("#f8fafc"),
      inset: (x: 11pt, y: 7pt),
      [
        #grid(
          columns: (if seal-node != none { 50pt } else { 0pt }, 1fr, 160pt),
          gutter: 9pt,
          align: (center + horizon, left + horizon, left + horizon),
          [#if seal-node != none { seal-node }],
          [
            #text(font: hde-font-serif, size: 7.4pt, weight: "bold", tracking: 0.6pt, fill: primary-color)[#upper(vertical-badge)] \
            #v(2pt)
            #text(font: hde-font-serif, size: 12.4pt, weight: "bold", fill: rgb("#0f172a"))[#upper(title)] \
            #v(1.5pt)
            #text(font: hde-font-sans, size: 7.4pt, style: "italic", fill: rgb("#334155"))[#subtitle]
          ],
          rect(
            width: 100%,
            stroke: (left: 1.5pt + primary-color),
            inset: (left: 8pt, y: 2.5pt),
            [
              #text(font: hde-font-mono, size: 7pt, weight: "bold", fill: rgb("#0f172a"))[DOCKET / REF: #doc-ref] \
              #text(font: hde-font-mono, size: 6.6pt, fill: rgb("#475569"))[FILED: #date-str] \
              #text(font: hde-font-mono, size: 6.4pt, weight: "bold", fill: accent-color)[#citation]
            ]
          )
        )
      ]
    )
  } else if variant == "trade_banner" {
    rect(
      width: 100%,
      fill: rgb("#0f172a"),
      stroke: (bottom: 3pt + accent-color),
      radius: (top: 4pt, bottom: 2pt),
      inset: (x: 11pt, y: 8pt),
      [
        #grid(
          columns: (if seal-node != none { 50pt } else { 0pt }, 1fr, 155pt),
          gutter: 9pt,
          align: (center + horizon, left + horizon, right + horizon),
          [#if seal-node != none { box(fill: white, radius: 4pt, inset: 2pt, seal-node) }],
          [
            #text(font: hde-font-mono, size: 6.8pt, weight: "bold", tracking: 0.6pt, fill: rgb("#fbbf24"))[★ #upper(vertical-badge) • #upper(org-name)] \
            #v(2pt)
            #text(font: hde-font-sans, size: 12.5pt, weight: "black", fill: white)[#upper(title)] \
            #v(1.5pt)
            #text(font: hde-font-sans, size: 7.4pt, fill: rgb("#cbd5e1"))[#subtitle]
          ],
          box(
            width: 100%,
            fill: rgb("#1e293b"),
            stroke: 0.75pt + accent-color,
            radius: 3pt,
            inset: (x: 8pt, y: 5pt),
            align(right)[
              #text(font: hde-font-mono, size: 7.2pt, weight: "bold", fill: rgb("#fbbf24"))[#doc-ref] \
              #text(font: hde-font-mono, size: 6.6pt, fill: white)[#date-str] \
              #text(font: hde-font-mono, size: 6.2pt, fill: rgb("#94a3b8"))[#citation]
            ]
          )
        )
      ]
    )
  } else if variant == "editorial_serif" {
    block(width: 100%, below: 4pt)[
      #line(length: 100%, stroke: 1.8pt + primary-color)
      #v(1.5pt)
      #line(length: 100%, stroke: 0.5pt + accent-color)
      #v(5pt)
      #grid(
        columns: (if seal-node != none { 50pt } else { 0pt }, 1fr, 155pt),
        gutter: 9pt,
        align: (left + horizon, left + horizon, right + horizon),
        [#if seal-node != none { seal-node }],
        [
          #text(font: hde-font-sans, size: 6.8pt, weight: "bold", tracking: 1pt, fill: accent-color)[#upper(org-name) • #upper(vertical-badge)] \
          #v(2pt)
          #text(font: hde-font-serif, size: 13.5pt, weight: "bold", fill: rgb("#0f172a"))[#title] \
          #v(1.5pt)
          #text(font: hde-font-serif, size: 8pt, style: "italic", fill: rgb("#475569"))[#subtitle]
        ],
        align(right)[
          #text(font: hde-font-mono, size: 7.2pt, weight: "bold", fill: primary-color)[#doc-ref] \
          #text(font: hde-font-sans, size: 7pt, fill: rgb("#475569"))[Issued: #date-str] \
          #text(font: hde-font-mono, size: 6.4pt, fill: accent-color)[#citation]
        ]
      )
      #v(5pt)
      #line(length: 100%, stroke: 0.75pt + rgb("#cbd5e1"))
    ]
  } else {
    rect(
      width: 100%,
      stroke: (left: 3.5pt + primary-color, rest: 0.75pt + rgb("#cbd5e1")),
      fill: rgb("#f8fafc"),
      radius: (right: 4pt, left: 1pt),
      inset: (x: 11pt, y: 7pt),
      [
        #grid(
          columns: (if seal-node != none { 50pt } else { 0pt }, 1fr, 155pt),
          gutter: 9pt,
          align: (center + horizon, left + horizon, right + horizon),
          [#if seal-node != none { seal-node }],
          [
            #box(fill: rgb("#dbeafe"), radius: 3pt, inset: (x: 6pt, y: 2pt))[
              #text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: primary-color)[#upper(vertical-badge)]
            ]
            #h(6pt)
            #text(font: hde-font-mono, size: 6.6pt, weight: "bold", fill: accent-color)[#citation] \
            #v(2.5pt)
            #text(font: hde-font-sans, size: 12.5pt, weight: "bold", fill: rgb("#0f172a"))[#title] \
            #v(1.5pt)
            #text(font: hde-font-sans, size: 7.6pt, fill: rgb("#475569"))[#subtitle]
          ],
          rect(
            width: 100%,
            fill: white,
            stroke: 0.5pt + rgb("#cbd5e1"),
            radius: 3pt,
            inset: (x: 8pt, y: 5pt),
            align(right)[
              #text(font: hde-font-sans, size: 7.2pt, weight: "bold", fill: rgb("#0f172a"))[#org-name] \
              #text(font: hde-font-mono, size: 7pt, weight: "bold", fill: primary-color)[#doc-ref] \
              #text(font: hde-font-mono, size: 6.5pt, fill: rgb("#64748b"))[#date-str]
            ]
          )
        )
      ]
    )
  }
}

// -----------------------------------------------------------------------------
// 6. KPI & FINANCIAL METRIC STRIP
// -----------------------------------------------------------------------------
#let hde-kpi-strip-variation(
  variant: "metric_cards",
  metrics: (),
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
) = {
  block(breakable: false, width: 100%, above: 6pt, below: 6pt)[
    #if variant == "dark_terminal" {
      rect(
        width: 100%,
        fill: rgb("#0f172a"),
        radius: 4pt,
        inset: (x: 11pt, y: 7pt),
        grid(
          columns: (1fr,) * metrics.len(),
          gutter: 8pt,
          ..metrics.enumerate().map(((idx, m)) => [
            #text(font: hde-font-mono, size: 6.6pt, weight: "bold", fill: rgb("#94a3b8"))[#upper(m.at(0))] \
            #v(2pt)
            #text(font: hde-font-mono, size: 10pt, weight: "bold", fill: if idx == metrics.len() - 1 { rgb("#34d399") } else { rgb("#38bdf8") })[#m.at(1)]
          ])
        )
      )
    } else if variant == "neo_brutalist" {
      grid(
        columns: (1fr,) * metrics.len(),
        gutter: 7pt,
        ..metrics.enumerate().map(((idx, m)) => rect(
          width: 100%,
          fill: if idx == metrics.len() - 1 { rgb("#00f0ff") } else { rgb("#ffffff") },
          stroke: 1.5pt + rgb("#000000"),
          inset: (x: 8pt, y: 6pt),
          [
            #text(font: hde-font-mono, size: 6.6pt, weight: "bold", fill: rgb("#000000"))[#upper(m.at(0))] \
            #v(2pt)
            #text(font: hde-font-mono, size: 10pt, weight: "black", fill: rgb("#000000"))[#m.at(1)]
          ]
        ))
      )
    } else {
      grid(
        columns: (1fr,) * metrics.len(),
        gutter: 7pt,
        ..metrics.enumerate().map(((idx, m)) => rect(
          width: 100%,
          fill: if idx == metrics.len() - 1 { rgb("#ecfdf5") } else { rgb("#f8fafc") },
          stroke: (top: 2.2pt + if idx == metrics.len() - 1 { accent-color } else { primary-color }, rest: 0.5pt + rgb("#cbd5e1")),
          radius: 3pt,
          inset: (x: 8pt, y: 6pt),
          [
            #text(font: hde-font-sans, size: 6.6pt, weight: "bold", fill: rgb("#475569"))[#upper(m.at(0))] \
            #v(2pt)
            #text(font: hde-font-mono, size: 10pt, weight: "bold", fill: if idx == metrics.len() - 1 { accent-color } else { rgb("#0f172a") })[#m.at(1)]
          ]
        ))
      )
    }
  ]
}

// -----------------------------------------------------------------------------
// 7. LINE-ITEM & EVIDENCE SCHEDULE TABLE (WITH STATUS PILLS & SUMMARY FOOTER)
// -----------------------------------------------------------------------------
#let hde-schedule-table-variation(
  variant: "striped",
  headers: ("CODE", "DESCRIPTION & TECHNICAL SPECIFICATION", "BASIS / STATUS", "AMOUNT / DELTA"),
  rows: (),
  summary-label: "",
  summary-value: "",
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
) = {
  let row-count = rows.len()
  block(breakable: false, width: 100%, above: 2pt, below: 2pt)[
    #if variant == "dark_header" or variant == "striped" {
      table(
        columns: (56pt, 1fr, 88pt, 76pt),
        inset: (x: 5.5pt, y: 3.8pt),
        stroke: 0.5pt + rgb("#cbd5e1"),
        align: (left + horizon, left + horizon, right + horizon, right + horizon),
        fill: (x, y) => if y == 0 {
          if variant == "dark_header" { rgb("#0f172a") } else { primary-color }
        } else if summary-value != "" and y == row-count + 1 {
          rgb("#0f172a")
        } else if calc.odd(y) {
          rgb("#ffffff")
        } else {
          rgb("#f8fafc")
        },
        ..headers.map(h => text(font: hde-font-sans, size: 6.6pt, weight: "bold", fill: white)[#upper(h)]),
        ..rows.enumerate().map(((idx, r)) => (
          text(font: hde-font-mono, size: 7pt, weight: "bold", fill: primary-color)[#r.at(0)],
          text(font: hde-font-sans, size: 7.2pt, fill: rgb("#1e293b"))[#r.at(1)],
          box(
            fill: if calc.even(idx) { rgb("#ecfdf5") } else { rgb("#eff6ff") },
            stroke: 0.4pt + if calc.even(idx) { accent-color } else { primary-color },
            radius: 99pt,
            inset: (x: 5pt, y: 1.4pt),
            text(font: hde-font-mono, size: 6.3pt, weight: "bold", fill: if calc.even(idx) { accent-color } else { primary-color })[#r.at(2)]
          ),
          text(font: hde-font-mono, size: 7.4pt, weight: "bold", fill: rgb("#0f172a"))[#r.at(3)],
        )).flatten(),
        ..if summary-value != "" {
          (
            table.cell(colspan: 2, align: left + horizon)[
              #text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: rgb("#94a3b8"))[
                ✓ SCHEDULE VERIFIED · TABULAR RIGHT-ALIGNED RECONCILIATION
              ]
            ],
            text(font: hde-font-mono, size: 6.4pt, weight: "bold", fill: rgb("#ffe500"))[
              #upper(if summary-label != "" { summary-label } else { "NET TOTAL" })
            ],
            text(font: hde-font-mono, size: 7.6pt, weight: "black", fill: rgb("#34d399"))[#summary-value],
          )
        } else { () }
      )
    } else {
      table(
        columns: (56pt, 1fr, 88pt, 76pt),
        inset: (x: 5.5pt, y: 3.8pt),
        stroke: (x, y) => if y == 0 {
          (bottom: 1.4pt + primary-color, top: 1.4pt + primary-color)
        } else if summary-value != "" and y == row-count + 1 {
          (top: 1.2pt + primary-color, bottom: 1.6pt + accent-color)
        } else {
          (bottom: 0.5pt + rgb("#cbd5e1"))
        },
        fill: (x, y) => if summary-value != "" and y == row-count + 1 { rgb("#f8fafc") } else { none },
        align: (left + horizon, left + horizon, right + horizon, right + horizon),
        ..headers.map(h => text(font: hde-font-sans, size: 6.6pt, weight: "bold", fill: primary-color)[#upper(h)]),
        ..rows.enumerate().map(((idx, r)) => (
          text(font: hde-font-mono, size: 7pt, weight: "bold", fill: rgb("#334155"))[#r.at(0)],
          text(font: hde-font-sans, size: 7.2pt, fill: rgb("#0f172a"))[#r.at(1)],
          box(
            fill: rgb("#f1f5f9"),
            radius: 99pt,
            inset: (x: 5pt, y: 1.4pt),
            text(font: hde-font-mono, size: 6.3pt, weight: "bold", fill: primary-color)[#r.at(2)]
          ),
          text(font: hde-font-mono, size: 7.4pt, weight: "bold", fill: accent-color)[#r.at(3)],
        )).flatten(),
        ..if summary-value != "" {
          (
            table.cell(colspan: 2, align: left + horizon)[
              #text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: rgb("#475569"))[
                ✓ CERTIFIED SCHEDULE FOOTER · RIGHT-ALIGNED SUMMARY
              ]
            ],
            text(font: hde-font-mono, size: 6.4pt, weight: "bold", fill: primary-color)[
              #upper(if summary-label != "" { summary-label } else { "NET TOTAL" })
            ],
            text(font: hde-font-mono, size: 7.6pt, weight: "black", fill: accent-color)[#summary-value],
          )
        } else { () }
      )
    }
  ]
}

// -----------------------------------------------------------------------------
// 8. 3-MODE EXECUTION, E-SIGN & STATUTORY NOTARY JURAT BLOCK
// -----------------------------------------------------------------------------
#let jurat-box(
  signer-name: "Authorized Signatory",
  signer-role: "Principal / Petitioner / Contractor",
  counterparty-name: "Counterparty / Client / Reviewing Officer",
  signature-type: "wet-ink",
  notary-required: false,
  stamp-label: "VERIFIED RECORD",
  stamp-sublabel: "ACROFORM READY",
  stamp-color: "#059669",
  primary-color: rgb("#1e3a8a"),
) = {
  block(breakable: false, width: 100%, above: 3pt)[
    #rect(
      width: 100%,
      stroke: 0.85pt + primary-color,
      radius: 4pt,
      fill: rgb("#f8fafc"),
      inset: (x: 9pt, y: if notary-required { 4.5pt } else { 6pt }),
      [
        #grid(
          columns: (1fr, auto),
          gutter: 10pt,
          align: (left + top, right + horizon),
          [
            #text(font: hde-font-sans, size: 7pt, weight: "bold", tracking: 0.5pt, fill: primary-color)[
              EXECUTION • ATTESTATION & INTERACTIVE SIGNATURE BLOCK (#upper(signature-type))
            ]
            #v(1.5pt)
            #text(font: hde-font-sans, size: 7pt, fill: rgb("#334155"))[
              I hereby certify and affirm that the schedules, technical specifications, and monetary figures stated herein are true, accurate, and complete.
            ]
            #v(if notary-required { 3pt } else { 4pt })
            #grid(
              columns: (1.35fr, 0.9fr),
              gutter: 12pt,
              [
                #if signature-type == "esign" [
                  #text(font: hde-font-serif, size: 11.5pt, style: "italic", weight: "bold", fill: primary-color)[/s/ #signer-name]
                  #v(1.5pt)
                ] else [
                  #rect(
                    width: 100%,
                    height: if notary-required { 16.5pt } else { 19pt },
                    fill: white,
                    stroke: (bottom: 1.2pt + rgb("#0f172a"), rest: 0.5pt + rgb("#94a3b8")),
                    radius: 2pt,
                    inset: (x: 6pt, y: 2.5pt),
                    align(left + horizon)[
                      #text(font: hde-font-mono, size: 6.4pt, fill: rgb("#64748b"))[SIGN OR TYPE HERE (INTERACTIVE /ACROFORM FIELD)]
                    ]
                  )
                ]
                #v(1.5pt)
                #text(font: hde-font-sans, size: 7pt, weight: "bold", fill: rgb("#0f172a"))[#signer-name]
                #text(font: hde-font-sans, size: 6.6pt, fill: rgb("#475569"))[ — #signer-role]
              ],
              [
                #if signature-type == "esign" [
                  #text(font: hde-font-mono, size: 8.8pt, weight: "bold", fill: rgb("#0f172a"))[2026-11-06 · ESIGN-SHA256]
                  #v(1.5pt)
                ] else [
                  #rect(
                    width: 100%,
                    height: if notary-required { 16.5pt } else { 19pt },
                    fill: white,
                    stroke: (bottom: 1.2pt + rgb("#0f172a"), rest: 0.5pt + rgb("#94a3b8")),
                    radius: 2pt,
                    inset: (x: 6pt, y: 2.5pt),
                    align(left + horizon)[
                      #text(font: hde-font-mono, size: 6.4pt, fill: rgb("#64748b"))[DATE (MM/DD/YYYY)]
                    ]
                  )
                ]
                #v(1.5pt)
                #text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: rgb("#334155"))[Counterparty: #counterparty-name]
              ]
            )
            #if notary-required [
              #v(3pt)
              #line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1"))
              #v(2.5pt)
              #grid(
                columns: (1fr, 104pt),
                gutter: 8pt,
                align: (left + horizon, center + horizon),
                [
                  #text(font: hde-font-serif, size: 6.6pt, weight: "bold", fill: rgb("#0f172a"))[
                    STATE OF RECORD · COUNTY OF RECORD — NOTARY PUBLIC JURAT:
                  ] \
                  #text(font: hde-font-serif, size: 6.4pt, fill: rgb("#334155"))[
                    Subscribed and sworn to before me in 2026. Notary Signature & Commission Exp: #box(line(length: 76pt, stroke: 0.5pt + rgb("#475569")))
                  ]
                ],
                rect(
                  width: 100%,
                  height: 18pt,
                  stroke: (dash: "dashed", paint: rgb("#64748b"), thickness: 0.75pt),
                  radius: 2pt,
                  align(center + horizon)[
                    #text(font: hde-font-mono, size: 6pt, weight: "bold", fill: rgb("#475569"))[OFFICIAL NOTARY SEAL]
                  ]
                )
              )
            ]
          ],
          svg-rubber-stamp(label: stamp-label, sublabel: stamp-sublabel, color-hex: stamp-color, angle: -5deg, width-in: if notary-required { 1.38in } else { 1.48in })
        )
      ]
    )
  ]
}

// =============================================================================
// 9. THE 10 RADICALLY DISTINCT STRUCTURAL PAGE ARCHETYPE BLUEPRINTS
// =============================================================================

#let hde-render-page-archetype(
  page-layout: "executive_draw_ledger",
  masthead-variant: "executive_split",
  kpi-variant: "metric_cards",
  table-variant: "striped",
  diagram-kind: "parcel_lot",
  section-header-variant: "numbered_pill_rule",
  text-alignment-variant: "two_column_justified",
  title: "",
  org-name: "",
  counterparty: "",
  profession: "",
  vertical: "",
  doc-ref: "",
  date-str: "2026-11-06",
  citation: "",
  demand-signal: "",
  seal-path: "",
  primary-hex: "#1e3a8a",
  accent-hex: "#059669",
  stamp-label: "VERIFIED",
  stamp-sublabel: "OFFICIAL RECORD",
  sig-type: "wet-ink",
  notary-required: false,
  metrics: (),
  rows: (),
  chart-rows: (),
  milestones: (),
) = {
  let primary-color = rgb(primary-hex)
  let accent-color = rgb(accent-hex)
  let seal-node = if seal-path != "" {
    image(seal-path, width: 52pt, height: 52pt, fit: "contain")
  } else {
    none
  }
  let sec-hdr-2 = if section-header-variant == "numbered_pill_rule" {
    "editorial_accent_bar"
  } else if section-header-variant == "editorial_accent_bar" {
    "swiss_index_grid"
  } else if section-header-variant == "dark_command_ribbon" {
    "numbered_pill_rule"
  } else if section-header-variant == "centered_double_rule" {
    "editorial_accent_bar"
  } else if section-header-variant == "neo_brutalist_tab" {
    "dark_command_ribbon"
  } else {
    "numbered_pill_rule"
  }

  // ---------------------------------------------------------------------------
  // LAYOUT 1: ASYMMETRIC SIDEBAR LEDGER (`asymmetric_sidebar`)
  // Left full-height dark slate/accent anchor rail + Right open executive ledger
  // ---------------------------------------------------------------------------
  if page-layout == "asymmetric_sidebar" {
    grid(
      columns: (150pt, 1fr),
      gutter: 10pt,
      rect(
        width: 100%,
        fill: rgb("#0f172a"),
        stroke: (right: 3pt + accent-color),
        radius: 5pt,
        inset: (x: 8.5pt, y: 8pt),
        [
          #if seal-node != none [
            #align(center, box(fill: white, radius: 6pt, inset: 2.5pt, seal-node))
            #v(4pt)
          ]
          #box(fill: accent-color, radius: 3pt, inset: (x: 6pt, y: 2pt))[
            #text(font: hde-font-mono, size: 6pt, weight: "bold", fill: white)[#upper(vertical)]
          ]
          #v(3.5pt)
          #text(font: hde-font-sans, size: 9.2pt, weight: "black", fill: white)[#org-name]
          #v(1.5pt)
          #text(font: hde-font-sans, size: 6.8pt, fill: rgb("#94a3b8"))[#profession]
          #v(5pt)
          #line(length: 100%, stroke: 0.5pt + rgb("#334155"))
          #v(5pt)
          #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: rgb("#38bdf8"))[DOCUMENT REFERENCE] \
          #text(font: hde-font-mono, size: 7.2pt, weight: "bold", fill: white)[#doc-ref · #date-str]
          #v(4pt)
          #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: rgb("#38bdf8"))[COUNTERPARTY / CLIENT] \
          #text(font: hde-font-sans, size: 7pt, weight: "bold", fill: rgb("#f8fafc"))[#counterparty]
          #v(4pt)
          #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: rgb("#38bdf8"))[GOVERNANCE STANDARD] \
          #text(font: hde-font-mono, size: 6.3pt, fill: rgb("#cbd5e1"))[#citation]
          #v(5pt)
          #line(length: 100%, stroke: 0.5pt + rgb("#334155"))
          #v(5pt)
          #for (idx, m) in metrics.enumerate() [
            #rect(
              width: 100%,
              fill: if idx == metrics.len() - 1 { rgb("#064e3b") } else { rgb("#1e293b") },
              stroke: 0.6pt + if idx == metrics.len() - 1 { rgb("#10b981") } else { rgb("#334155") },
              radius: 3.5pt,
              inset: (x: 6pt, y: 3.8pt),
              [
                #text(font: hde-font-mono, size: 5.6pt, weight: "bold", fill: rgb("#94a3b8"))[#upper(m.at(0))] \
                #v(1pt)
                #text(font: hde-font-mono, size: 8.2pt, weight: "bold", fill: if idx == metrics.len() - 1 { rgb("#34d399") } else { white })[#m.at(1)]
              ]
            )
            #v(2.5pt)
          ]
          #v(3pt)
          #align(center, svg-rubber-stamp(label: stamp-label, sublabel: stamp-sublabel, color-hex: accent-hex, angle: -4deg, width-in: 1.28in))
        ]
      ),
      [
        #rect(
          width: 100%,
          fill: rgb("#f8fafc"),
          stroke: (bottom: 2pt + primary-color),
          inset: (x: 8pt, y: 5pt),
          [
            #text(font: hde-font-mono, size: 6.2pt, weight: "bold", tracking: 0.8pt, fill: primary-color)[
              EXECUTIVE ENGAGEMENT & COMMERCIAL LEDGER · #doc-ref
            ] \
            #v(1.5pt)
            #text(font: hde-font-sans, size: 11.8pt, weight: "black", fill: rgb("#0f172a"))[#title]
          ]
        )
        #hde-section-header(
          variant: section-header-variant,
          sec-num: "01",
          kicker: "SECTION 01 · COVENANTS & ALIGNMENT",
          title: "EXECUTIVE SCOPE & VERIFIED RECITALS",
          subtitle: counterparty,
          right-tag: doc-ref,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-text-alignment-block(
          variant: text-alignment-variant,
          org-name: org-name,
          counterparty: counterparty,
          profession: profession,
          citation: citation,
          demand-signal: demand-signal,
          metrics: metrics,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-section-header(
          variant: sec-hdr-2,
          sec-num: "02",
          kicker: "SECTION 02 · ITEMIZED SCHEDULE",
          title: "LINE-ITEM SCHEDULE & ALLOCATION",
          subtitle: org-name,
          right-tag: metrics.at(3).at(1),
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-schedule-table-variation(
          variant: table-variant,
          rows: rows,
          summary-label: metrics.at(3).at(0),
          summary-value: metrics.at(3).at(1),
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-milestone-timeline(
          title: "DELIVERABLE PHASES & SETTLEMENT SCHEDULE",
          steps: milestones,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #jurat-box(
          signer-name: org-name,
          signer-role: profession,
          counterparty-name: counterparty,
          signature-type: sig-type,
          notary-required: notary-required,
          stamp-label: stamp-label,
          stamp-sublabel: stamp-sublabel,
          stamp-color: accent-hex,
          primary-color: primary-color,
        )
      ]
    )

  // ---------------------------------------------------------------------------
  // LAYOUT 2: FORMAL COURT / BOARD PLEADING CAPTION (`pleading_caption`)
  // Left numbered pleading margin ruler (1..22) + Formal Caption Box (`v.`)
  // ---------------------------------------------------------------------------
  } else if page-layout == "pleading_caption" {
    grid(
      columns: (20pt, 1fr),
      gutter: 8pt,
      rect(
        width: 100%,
        stroke: (right: 1.4pt + primary-color),
        inset: (right: 3.5pt, y: 1pt),
        align(right)[
          #set text(font: hde-font-mono, size: 7pt, weight: "bold", fill: rgb("#64748b"))
          #for line-no in range(1, 24) [
            #box(height: 21.5pt, align(right + horizon)[#line-no]) \
          ]
        ]
      ),
      [
        #set text(font: hde-font-serif, size: 8.4pt, fill: rgb("#0f172a"))
        #rect(
          width: 100%,
          stroke: 1.2pt + rgb("#0f172a"),
          fill: rgb("#f8fafc"),
          inset: (x: 9pt, y: 5.5pt),
          grid(
            columns: (1.25fr, 14pt, 0.95fr),
            align: (left + horizon, center + horizon, left + horizon),
            [
              #text(font: hde-font-serif, size: 7.2pt, weight: "bold", tracking: 0.5pt, fill: primary-color)[
                IN THE MATTER OF THE STATUTORY RECORD / TRIBUNAL:
              ] \
              #v(2pt)
              #text(weight: "bold", size: 8.6pt)[#upper(org-name)], \
              #h(8pt)#text(size: 7.2pt, style: "italic")[Petitioner / Claimant / Issuing Party,] \
              #v(1pt)
              #h(20pt)#text(weight: "bold", style: "italic")[v.] \
              #v(1pt)
              #text(weight: "bold", size: 8.4pt)[#upper(counterparty)], \
              #h(8pt)#text(size: 7.2pt, style: "italic")[Respondent / Counterparty of Record.]
            ],
            [
              #for _ in range(1, 7) [ #text(font: hde-font-mono, size: 8pt, weight: "bold")[)] \ ]
            ],
            [
              #if seal-node != none [
                #place(top + right, box(width: 34pt, height: 34pt, image(seal-path, width: 34pt)))
              ]
              #text(font: hde-font-mono, size: 6.8pt, weight: "bold")[DOCKET / INSTRUMENT NO.:] \
              #text(font: hde-font-mono, size: 7.8pt, weight: "bold", fill: primary-color)[#doc-ref] \
              #v(1.5pt)
              #text(font: hde-font-mono, size: 6.6pt)[EFFECTIVE DATE: #date-str] \
              #text(font: hde-font-mono, size: 6.4pt, weight: "bold", fill: accent-color)[STATUTE: #citation]
            ]
          )
        )
        #hde-section-header(
          variant: "centered_double_rule",
          sec-num: "01",
          title: title,
          subtitle: counterparty,
          right-tag: doc-ref,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-comparison-lanes(
          left-label: metrics.at(0).at(0),
          left-val: metrics.at(0).at(1),
          left-sub: "Baseline / Prior Record",
          right-label: metrics.at(3).at(0),
          right-val: metrics.at(3).at(1),
          right-sub: "Contended / Certified Relief",
          delta-badge: stamp-label,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-text-alignment-block(
          variant: if text-alignment-variant == "two_column_justified" { "hanging_indent_clauses" } else { text-alignment-variant },
          org-name: org-name,
          counterparty: counterparty,
          profession: profession,
          citation: citation,
          demand-signal: demand-signal,
          metrics: metrics,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-section-header(
          variant: section-header-variant,
          sec-num: "02",
          kicker: "SECTION II · EVIDENTIARY SCHEDULE",
          title: "CERTIFIED SCHEDULE OF EVIDENCE & STATUTORY BASIS",
          subtitle: org-name,
          right-tag: metrics.at(3).at(1),
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-schedule-table-variation(
          variant: "dark_header",
          rows: rows,
          summary-label: metrics.at(3).at(0),
          summary-value: metrics.at(3).at(1),
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-milestone-timeline(
          title: "III. PROCEDURAL DEADLINES, CURATIVE WINDOW & COMPLIANCE STEPS",
          steps: milestones,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #jurat-box(
          signer-name: org-name,
          signer-role: profession,
          counterparty-name: counterparty,
          signature-type: sig-type,
          notary-required: notary-required,
          stamp-label: stamp-label,
          stamp-sublabel: stamp-sublabel,
          stamp-color: accent-hex,
          primary-color: primary-color,
        )
      ]
    )

  // ---------------------------------------------------------------------------
  // LAYOUT 3: 3-TIER PACKAGE COMPARISON CARDS (`three_tier_packages`)
  // Modeled on `~/.agents/skills/typst/templates/one-pager.typ`
  // ---------------------------------------------------------------------------
  } else if page-layout == "three_tier_packages" {
    hde-masthead-variation(
      variant: masthead-variant,
      title: title,
      subtitle: "Prepared for: " + counterparty + " · " + profession,
      org-name: org-name,
      vertical-badge: vertical,
      doc-ref: doc-ref,
      date-str: date-str,
      citation: citation,
      seal-path: seal-path,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-section-header(
      variant: section-header-variant,
      sec-num: "01",
      kicker: "SECTION 01 · COMMERCIAL TIERS",
      title: "SELECTABLE SERVICE TIERS & PACKAGE COMPARISON",
      subtitle: counterparty,
      right-tag: doc-ref,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    grid(
      columns: (1fr, 1.08fr, 1fr),
      gutter: 7pt,
      // Tier 1
      rect(
        width: 100%,
        stroke: 0.85pt + rgb("#cbd5e1"),
        radius: 5pt,
        fill: rgb("#ffffff"),
        inset: (x: 7.5pt, y: 5pt),
        [
          #text(font: hde-font-mono, size: 6pt, weight: "bold", fill: rgb("#64748b"))[OPTION A · ESSENTIAL TIER] \
          #v(1pt)
          #text(font: hde-font-sans, size: 8.4pt, weight: "bold", fill: rgb("#0f172a"))[#metrics.at(0).at(0)] \
          #v(1.5pt)
          #text(font: hde-font-mono, size: 10.5pt, weight: "black", fill: primary-color)[#metrics.at(0).at(1)] \
          #v(2pt)
          #line(length: 100%, stroke: 0.5pt + rgb("#e2e8f0"))
          #v(2pt)
          #text(font: hde-font-sans, size: 6.7pt, fill: rgb("#334155"))[
            #text(fill: accent-color, weight: "bold")[✓] #rows.at(0).at(1) \
            #v(1pt)
            #text(fill: accent-color, weight: "bold")[✓] Basis: #rows.at(0).at(2) · #milestones.at(0).at(0)
          ]
        ]
      ),
      // Tier 2 (Recommended Hero Card)
      rect(
        width: 100%,
        stroke: 1.8pt + accent-color,
        radius: 5pt,
        fill: rgb("#f0fdf4"),
        inset: (x: 7.5pt, y: 5pt),
        [
          #grid(
            columns: (1fr, auto),
            align: (left + horizon, right + horizon),
            text(font: hde-font-mono, size: 6pt, weight: "bold", fill: accent-color)[OPTION B · RECOMMENDED],
            box(fill: accent-color, radius: 99pt, inset: (x: 5pt, y: 1.2pt))[
              #text(font: hde-font-mono, size: 5.6pt, weight: "bold", fill: white)[★ MOST POPULAR]
            ]
          )
          #v(1pt)
          #text(font: hde-font-sans, size: 8.6pt, weight: "black", fill: rgb("#0f172a"))[#metrics.at(3).at(0)] \
          #v(1.5pt)
          #text(font: hde-font-mono, size: 11pt, weight: "black", fill: accent-color)[#metrics.at(3).at(1)] \
          #v(2pt)
          #line(length: 100%, stroke: 0.75pt + accent-color)
          #v(2pt)
          #text(font: hde-font-sans, size: 6.8pt, fill: rgb("#0f172a"))[
            #text(fill: accent-color, weight: "bold")[✓] *#rows.at(1).at(1)* \
            #v(1pt)
            #text(fill: accent-color, weight: "bold")[✓] *#rows.at(2).at(1)*
          ]
        ]
      ),
      // Tier 3
      rect(
        width: 100%,
        stroke: 1pt + primary-color,
        radius: 5pt,
        fill: rgb("#f8fafc"),
        inset: (x: 7.5pt, y: 5pt),
        [
          #text(font: hde-font-mono, size: 6pt, weight: "bold", fill: primary-color)[OPTION C · PREMIER / FULL] \
          #v(1pt)
          #text(font: hde-font-sans, size: 8.4pt, weight: "bold", fill: rgb("#0f172a"))[#metrics.at(1).at(0)] \
          #v(1.5pt)
          #text(font: hde-font-mono, size: 10.5pt, weight: "black", fill: rgb("#0f172a"))[#metrics.at(1).at(1)] \
          #v(2pt)
          #line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1"))
          #v(2pt)
          #text(font: hde-font-sans, size: 6.7pt, fill: rgb("#334155"))[
            #text(fill: primary-color, weight: "bold")[✓] #rows.at(3).at(1) \
            #v(1pt)
            #text(fill: primary-color, weight: "bold")[✓] #metrics.at(2).at(0): #metrics.at(2).at(1)
          ]
        ]
      )
    )
    hde-text-alignment-block(
      variant: text-alignment-variant,
      org-name: org-name,
      counterparty: counterparty,
      profession: profession,
      citation: citation,
      demand-signal: demand-signal,
      metrics: metrics,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-section-header(
      variant: sec-hdr-2,
      sec-num: "02",
      kicker: "SECTION 02 · DELIVERABLE MATRIX",
      title: "INCLUDED DELIVERABLES, SPECIFICATIONS & SLA SCHEDULE",
      subtitle: org-name,
      right-tag: metrics.at(3).at(1),
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-schedule-table-variation(
      variant: table-variant,
      headers: ("PACKAGE / CODE", "INCLUDED DELIVERABLES, SPECIFICATIONS & SLA", "ALLOCATION / QTY", "FEE / VALUE"),
      rows: rows,
      summary-label: metrics.at(3).at(0),
      summary-value: metrics.at(3).at(1),
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-milestone-timeline(
      title: "ONBOARDING & FULFILLMENT TIMELINE",
      steps: milestones,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    jurat-box(
      signer-name: org-name,
      signer-role: profession,
      counterparty-name: counterparty,
      signature-type: sig-type,
      notary-required: notary-required,
      stamp-label: stamp-label,
      stamp-sublabel: stamp-sublabel,
      stamp-color: accent-hex,
      primary-color: primary-color,
    )

  // ---------------------------------------------------------------------------
  // LAYOUT 4: MULTI-POINT INSPECTION CHECKLIST (`inspection_checklist`)
  // Modeled on `~/.agents/skills/typst/templates/checklist.typ`
  // ---------------------------------------------------------------------------
  } else if page-layout == "inspection_checklist" {
    hde-masthead-variation(
      variant: masthead-variant,
      title: title,
      subtitle: "Asset / Site: " + counterparty + " · Inspector: " + org-name,
      org-name: org-name,
      vertical-badge: vertical,
      doc-ref: doc-ref,
      date-str: date-str,
      citation: citation,
      seal-path: seal-path,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-section-header(
      variant: "dark_command_ribbon",
      sec-num: "01",
      title: "GRADING KEY: [☑ PASS] [☐ ATTN] [☐ FAIL] · DIAGNOSTIC TELEMETRY",
      right-tag: citation,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-kpi-strip-variation(
      variant: kpi-variant,
      metrics: metrics,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    table(
      columns: (54pt, 1.08fr, 0.96fr, 80pt, 74pt),
      inset: (x: 5.5pt, y: 4pt),
      stroke: 0.5pt + rgb("#cbd5e1"),
      align: (left + horizon, left + horizon, left + horizon, center + horizon, right + horizon),
      fill: (x, y) => if y == 0 { primary-color } else if calc.odd(y) { white } else { rgb("#f8fafc") },
      text(font: hde-font-sans, size: 6.6pt, weight: "bold", fill: white)[ITEM CODE],
      text(font: hde-font-sans, size: 6.6pt, weight: "bold", fill: white)[INSPECTION CHECKPOINT & SYSTEM],
      text(font: hde-font-sans, size: 6.6pt, weight: "bold", fill: white)[GRADE CHECKBOXES],
      text(font: hde-font-sans, size: 6.6pt, weight: "bold", fill: white)[OBSERVED SPEC],
      text(font: hde-font-sans, size: 6.6pt, weight: "bold", fill: white)[COST / STATUS],
      ..rows.enumerate().map(((idx, r)) => (
        text(font: hde-font-mono, size: 7pt, weight: "bold", fill: primary-color)[#r.at(0)],
        text(font: hde-font-sans, size: 7.2pt, fill: rgb("#0f172a"))[#r.at(1)],
        text(font: hde-font-mono, size: 6.7pt, weight: "bold", fill: if idx == 1 { rgb("#b45309") } else { rgb("#047857") })[
          #if idx == 1 { "[☐ PASS ☑ ATTN ☐ FAIL]" } else { "[☑ PASS ☐ ATTN ☐ FAIL]" }
        ],
        text(font: hde-font-mono, size: 6.8pt, fill: rgb("#475569"))[#r.at(2)],
        text(font: hde-font-mono, size: 7.2pt, weight: "bold", fill: rgb("#0f172a"))[#r.at(3)],
      )).flatten()
    )
    hde-section-header(
      variant: section-header-variant,
      sec-num: "02",
      kicker: "SECTION 02 · FIELD FINDINGS & SCHEMATIC",
      title: "TECHNICAL TELEMETRY & NARRATIVE FINDINGS",
      subtitle: org-name,
      right-tag: metrics.at(3).at(1),
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-text-alignment-block(
      variant: text-alignment-variant,
      org-name: org-name,
      counterparty: counterparty,
      profession: profession,
      citation: citation,
      demand-signal: demand-signal,
      metrics: metrics,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    svg-architectural-diagram(
      kind: diagram-kind,
      title: "FIELD DIAGNOSTIC TELEMETRY & INSPECTION SCHEMATIC",
      primary-hex: primary-hex,
      accent-hex: accent-hex,
      metric-a: metrics.at(0).at(1),
      metric-b: metrics.at(3).at(1),
      hero-height: 70pt,
    )
    jurat-box(
      signer-name: org-name,
      signer-role: profession,
      counterparty-name: counterparty,
      signature-type: sig-type,
      notary-required: notary-required,
      stamp-label: stamp-label,
      stamp-sublabel: stamp-sublabel,
      stamp-color: accent-hex,
      primary-color: primary-color,
    )

  // ---------------------------------------------------------------------------
  // LAYOUT 5: ENGINEERING BLUEPRINT & CAD SPEC SHEET (`blueprint_schematic`)
  // Technical Title Block + Large Hero CAD Schematic + BOM Table + PE Stamp
  // ---------------------------------------------------------------------------
  } else if page-layout == "blueprint_schematic" {
    rect(
      width: 100%,
      stroke: 1.5pt + rgb("#0f172a"),
      fill: rgb("#ffffff"),
      inset: 8pt,
      [
        #table(
          columns: (if seal-node != none { 50pt } else { 0pt }, 1fr, 112pt, 112pt),
          stroke: 0.75pt + rgb("#0f172a"),
          inset: (x: 7pt, y: 4.5pt),
          fill: (x, y) => if x == 1 { rgb("#f8fafc") } else { white },
          align: (center + horizon, left + horizon, left + horizon, left + horizon),
          [#if seal-node != none { seal-node }],
          [
            #text(font: hde-font-mono, size: 6.3pt, weight: "bold", fill: primary-color)[ENGINEERING / CAD SUBMITTAL · #upper(vertical)] \
            #v(1pt)
            #text(font: hde-font-sans, size: 11pt, weight: "black", fill: rgb("#0f172a"))[#upper(title)] \
            #v(1pt)
            #text(font: hde-font-mono, size: 6.5pt, fill: rgb("#475569"))[SITE / CLIENT: #counterparty]
          ],
          [
            #text(font: hde-font-mono, size: 5.8pt, fill: rgb("#64748b"))[DRAWING / SPEC NO.] \
            #text(font: hde-font-mono, size: 7.5pt, weight: "bold", fill: primary-color)[#doc-ref] \
            #text(font: hde-font-mono, size: 6.2pt, fill: rgb("#334155"))[SCALE: AS NOTED]
          ],
          [
            #text(font: hde-font-mono, size: 5.8pt, fill: rgb("#64748b"))[CODE / STANDARD] \
            #text(font: hde-font-mono, size: 6.5pt, weight: "bold", fill: accent-color)[#citation] \
            #text(font: hde-font-mono, size: 6.2pt, fill: rgb("#334155"))[DATE: #date-str]
          ]
        )
        #svg-architectural-diagram(
          kind: diagram-kind,
          title: "FIGURE 1.0 — CERTIFIED AS-BUILT ENGINEERING & GEOMETRY SCHEMATIC",
          primary-hex: primary-hex,
          accent-hex: accent-hex,
          metric-a: metrics.at(0).at(1),
          metric-b: metrics.at(3).at(1),
          hero-height: 92pt,
        )
        #hde-kpi-strip-variation(
          variant: "dark_terminal",
          metrics: metrics,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-section-header(
          variant: section-header-variant,
          sec-num: "01",
          kicker: "SECTION 01 · ENGINEERING NOTES",
          title: "TECHNICAL SPECIFICATIONS & DESIGN COVENANTS",
          subtitle: counterparty,
          right-tag: doc-ref,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-text-alignment-block(
          variant: text-alignment-variant,
          org-name: org-name,
          counterparty: counterparty,
          profession: profession,
          citation: citation,
          demand-signal: demand-signal,
          metrics: metrics,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-section-header(
          variant: "swiss_index_grid",
          sec-num: "02",
          title: "BILL OF MATERIALS (BOM) & COMPONENT TOLERANCE SCHEDULE",
          right-tag: metrics.at(3).at(1),
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #hde-schedule-table-variation(
          variant: "dark_header",
          headers: ("SPEC ID", "ENGINEERING COMPONENT / MATERIAL SPECIFICATION", "TOLERANCE / QTY", "EXTENDED VALUE"),
          rows: rows,
          summary-label: metrics.at(3).at(0),
          summary-value: metrics.at(3).at(1),
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #jurat-box(
          signer-name: org-name,
          signer-role: profession,
          counterparty-name: counterparty,
          signature-type: sig-type,
          notary-required: notary-required,
          stamp-label: stamp-label,
          stamp-sublabel: stamp-sublabel,
          stamp-color: accent-hex,
          primary-color: primary-color,
        )
      ]
    )

  // ---------------------------------------------------------------------------
  // LAYOUT 6: BILATERAL T-ACCOUNT SPLIT SETTLEMENT (`split_settlement`)
  // Symmetrical Buyer/Seller or Debit/Credit dual-column reconciliation sheet
  // ---------------------------------------------------------------------------
  } else if page-layout == "split_settlement" {
    hde-masthead-variation(
      variant: masthead-variant,
      title: title,
      subtitle: "Bilateral Settlement & Reconciliation Ledger · " + counterparty,
      org-name: org-name,
      vertical-badge: vertical,
      doc-ref: doc-ref,
      date-str: date-str,
      citation: citation,
      seal-path: seal-path,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-kpi-strip-variation(
      variant: kpi-variant,
      metrics: metrics,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-section-header(
      variant: section-header-variant,
      sec-num: "01",
      kicker: "SECTION 01 · BILATERAL T-ACCOUNT",
      title: "DEBIT / CREDIT PRORATION & SETTLEMENT COLUMNS",
      subtitle: counterparty,
      right-tag: doc-ref,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    grid(
      columns: (1fr, 1fr),
      gutter: 8pt,
      // Left T-Account Column
      rect(
        width: 100%,
        stroke: (top: 2.2pt + primary-color, rest: 0.75pt + rgb("#cbd5e1")),
        radius: 4pt,
        fill: rgb("#ffffff"),
        inset: (x: 7.5pt, y: 5pt),
        [
          #text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: primary-color)[
            SCHEDULE A · PRIMARY DEBITS / GROSS BASIS
          ]
          #v(2pt)
          #table(
            columns: (48pt, 1fr, 68pt),
            inset: (x: 4pt, y: 3.5pt),
            stroke: (bottom: 0.5pt + rgb("#e2e8f0")),
            align: (left + horizon, left + horizon, right + horizon),
            text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: rgb("#64748b"))[CODE],
            text(font: hde-font-sans, size: 6.2pt, weight: "bold", fill: rgb("#64748b"))[ITEM DESCRIPTION],
            text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: rgb("#64748b"))[AMOUNT],
            text(font: hde-font-mono, size: 6.6pt, weight: "bold", fill: primary-color)[#rows.at(0).at(0)],
            text(font: hde-font-sans, size: 6.8pt)[#rows.at(0).at(1)],
            text(font: hde-font-mono, size: 6.8pt, weight: "bold")[#rows.at(0).at(3)],
            text(font: hde-font-mono, size: 6.6pt, weight: "bold", fill: primary-color)[#rows.at(1).at(0)],
            text(font: hde-font-sans, size: 6.8pt)[#rows.at(1).at(1)],
            text(font: hde-font-mono, size: 6.8pt, weight: "bold")[#rows.at(1).at(3)],
          )
          #v(2.5pt)
          #rect(width: 100%, fill: rgb("#eff6ff"), radius: 3pt, inset: (x: 6pt, y: 3.5pt))[
            #grid(
              columns: (1fr, auto),
              text(font: hde-font-mono, size: 6.5pt, weight: "bold", fill: primary-color)[#metrics.at(0).at(0)],
              text(font: hde-font-mono, size: 7.6pt, weight: "bold", fill: rgb("#0f172a"))[#metrics.at(0).at(1)]
            )
          ]
        ]
      ),
      // Right T-Account Column
      rect(
        width: 100%,
        stroke: (top: 2.2pt + accent-color, rest: 0.75pt + rgb("#cbd5e1")),
        radius: 4pt,
        fill: rgb("#ffffff"),
        inset: (x: 7.5pt, y: 5pt),
        [
          #text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: accent-color)[
            SCHEDULE B · CREDITS / PRORATIONS & ADJUSTMENTS
          ]
          #v(2pt)
          #table(
            columns: (48pt, 1fr, 68pt),
            inset: (x: 4pt, y: 3.5pt),
            stroke: (bottom: 0.5pt + rgb("#e2e8f0")),
            align: (left + horizon, left + horizon, right + horizon),
            text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: rgb("#64748b"))[CODE],
            text(font: hde-font-sans, size: 6.2pt, weight: "bold", fill: rgb("#64748b"))[CREDIT / OFFSET],
            text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: rgb("#64748b"))[ADJUSTMENT],
            text(font: hde-font-mono, size: 6.6pt, weight: "bold", fill: accent-color)[#rows.at(2).at(0)],
            text(font: hde-font-sans, size: 6.8pt)[#rows.at(2).at(1)],
            text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: accent-color)[#rows.at(2).at(3)],
            text(font: hde-font-mono, size: 6.6pt, weight: "bold", fill: accent-color)[#rows.at(3).at(0)],
            text(font: hde-font-sans, size: 6.8pt)[#rows.at(3).at(1)],
            text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: accent-color)[#rows.at(3).at(3)],
          )
          #v(2.5pt)
          #rect(width: 100%, fill: rgb("#ecfdf5"), radius: 3pt, inset: (x: 6pt, y: 3.5pt))[
            #grid(
              columns: (1fr, auto),
              text(font: hde-font-mono, size: 6.5pt, weight: "bold", fill: accent-color)[#metrics.at(3).at(0)],
              text(font: hde-font-mono, size: 7.6pt, weight: "bold", fill: accent-color)[#metrics.at(3).at(1)]
            )
          ]
        ]
      )
    )
    hde-section-header(
      variant: sec-hdr-2,
      sec-num: "02",
      kicker: "SECTION 02 · DOT-LEADER WATERFALL",
      title: "TABULAR SETTLEMENT LEDGER & DISBURSEMENT SCHEDULE",
      subtitle: org-name,
      right-tag: metrics.at(3).at(1),
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-text-alignment-block(
      variant: if text-alignment-variant == "two_column_justified" { "tabular_dot_leader_ledger" } else { text-alignment-variant },
      org-name: org-name,
      counterparty: counterparty,
      profession: profession,
      citation: citation,
      demand-signal: demand-signal,
      metrics: metrics,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-milestone-timeline(
      title: "SETTLEMENT & DISBURSEMENT AUDIT TRAIL",
      steps: milestones,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    jurat-box(
      signer-name: org-name,
      signer-role: profession,
      counterparty-name: counterparty,
      signature-type: sig-type,
      notary-required: notary-required,
      stamp-label: stamp-label,
      stamp-sublabel: stamp-sublabel,
      stamp-color: accent-hex,
      primary-color: primary-color,
    )

  // ---------------------------------------------------------------------------
  // LAYOUT 7: CLINICAL / REGULATORY BOXED FORM GRID (`clinical_box_grid`)
  // CMS-1500 / BOL style sharp bureaucratic numbered form boxes + coded schedule
  // ---------------------------------------------------------------------------
  } else if page-layout == "clinical_box_grid" {
    rect(
      width: 100%,
      fill: primary-color,
      inset: (x: 10pt, y: 4.5pt),
      grid(
        columns: (1.5fr, 1fr),
        gutter: 8pt,
        align: (left + horizon, right + horizon),
        text(font: hde-font-mono, size: 8pt, weight: "bold", fill: white)[
          OFFICIAL CODED FORM • #upper(title)
        ],
        text(font: hde-font-mono, size: 6.6pt, weight: "bold", fill: rgb("#ffe500"))[
          FORM ID: #doc-ref · #citation
        ]
      )
    )
    table(
      columns: (1.2fr, 1.2fr, 1fr, 0.9fr),
      stroke: 0.6pt + rgb("#0f172a"),
      inset: (x: 6.5pt, y: 4pt),
      fill: (x, y) => if y == 0 or y == 2 { rgb("#f8fafc") } else { white },
      [
        #text(font: hde-font-mono, size: 5.6pt, weight: "bold", fill: primary-color)[BOX 01 · ISSUING PROVIDER / CARRIER / LAB] \
        #v(1pt)
        #text(font: hde-font-sans, size: 7.5pt, weight: "bold", fill: rgb("#0f172a"))[#org-name]
      ],
      [
        #text(font: hde-font-mono, size: 5.6pt, weight: "bold", fill: primary-color)[BOX 02 · PATIENT / CONSIGNEE / SUBJECT] \
        #v(1pt)
        #text(font: hde-font-sans, size: 7.5pt, weight: "bold", fill: rgb("#0f172a"))[#counterparty]
      ],
      [
        #text(font: hde-font-mono, size: 5.6pt, weight: "bold", fill: primary-color)[BOX 03 · CLASSIFICATION] \
        #v(1pt)
        #text(font: hde-font-mono, size: 7.2pt, weight: "bold", fill: accent-color)[#vertical]
      ],
      [
        #text(font: hde-font-mono, size: 5.6pt, weight: "bold", fill: primary-color)[BOX 04 · FILING DATE] \
        #v(1pt)
        #text(font: hde-font-mono, size: 7.4pt, weight: "bold", fill: rgb("#0f172a"))[#date-str]
      ],
      ..metrics.enumerate().map(((idx, m)) => [
        #text(font: hde-font-mono, size: 5.6pt, weight: "bold", fill: rgb("#475569"))[BOX 0#(idx + 5) · #upper(m.at(0))] \
        #v(1pt)
        #text(font: hde-font-mono, size: 8.2pt, weight: "bold", fill: if idx == 3 { accent-color } else { rgb("#0f172a") })[#m.at(1)]
      ])
    )
    hde-section-header(
      variant: section-header-variant,
      sec-num: "09",
      kicker: "BOX 09 · CODED SCHEDULE",
      title: "CODED PROCEDURE, TARIFF OR CLINICAL LINE SCHEDULE",
      subtitle: counterparty,
      right-tag: doc-ref,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-schedule-table-variation(
      variant: "dark_header",
      headers: ("LINE / CODE", "CODED PROCEDURE, TARIFF OR CLINICAL DESCRIPTION", "UNITS / POINTER", "CHARGE / VALUE"),
      rows: rows,
      summary-label: metrics.at(3).at(0),
      summary-value: metrics.at(3).at(1),
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-text-alignment-block(
      variant: text-alignment-variant,
      org-name: org-name,
      counterparty: counterparty,
      profession: profession,
      citation: citation,
      demand-signal: demand-signal,
      metrics: metrics,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    svg-architectural-diagram(
      kind: diagram-kind,
      title: "BOX 10 · VERIFIED TELEMETRY & WORKFLOW DIAGRAM",
      primary-hex: primary-hex,
      accent-hex: accent-hex,
      metric-a: metrics.at(0).at(1),
      metric-b: metrics.at(3).at(1),
      hero-height: 62pt,
    )
    hde-milestone-timeline(
      title: "BOX 11 · CHAIN OF CUSTODY, ROUTING & SETTLEMENT STEPS",
      steps: milestones,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    jurat-box(
      signer-name: org-name,
      signer-role: profession,
      counterparty-name: counterparty,
      signature-type: sig-type,
      notary-required: notary-required,
      stamp-label: stamp-label,
      stamp-sublabel: stamp-sublabel,
      stamp-color: accent-hex,
      primary-color: primary-color,
    )

  // ---------------------------------------------------------------------------
  // LAYOUT 8: CEREMONIAL CERTIFICATE & DEED BROADSHEET (`certificate_broadsheet`)
  // Double-rule ornamental frame + Centered Seal + Formal Recital + Dual Witness
  // ---------------------------------------------------------------------------
  } else if page-layout == "certificate_broadsheet" {
    rect(
      width: 100%,
      stroke: 2.2pt + primary-color,
      inset: 3pt,
      rect(
        width: 100%,
        stroke: 0.75pt + accent-color,
        fill: rgb("#fffdf9"),
        inset: (x: 13pt, y: 9pt),
        [
          #grid(
            columns: (1fr, auto, 1fr),
            align: (left + top, center + horizon, right + top),
            [
              #text(font: hde-font-mono, size: 6.3pt, weight: "bold", fill: rgb("#475569"))[
                INSTRUMENT NO: #doc-ref \
                RECORDED DATE: #date-str
              ]
            ],
            [
              #if seal-node != none { seal-node }
            ],
            [
              #rect(
                stroke: (dash: "dashed", paint: rgb("#94a3b8"), thickness: 0.6pt),
                inset: (x: 6pt, y: 3.5pt),
                align(right)[
                  #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: primary-color)[OFFICIAL RECORDING STAMP ZONE] \
                  #text(font: hde-font-mono, size: 5.8pt, fill: rgb("#64748b"))[#citation]
                ]
              )
            ]
          )
          #v(4pt)
          #align(center)[
            #text(font: hde-font-mono, size: 6.6pt, weight: "bold", tracking: 1.4pt, fill: accent-color)[
              #upper(vertical) • #upper(org-name)
            ] \
            #v(2pt)
            #text(font: hde-font-serif, size: 13.8pt, weight: "bold", fill: rgb("#0f172a"))[#upper(title)]
          ]
          #hde-section-header(
            variant: "centered_double_rule",
            sec-num: "01",
            title: "CEREMONIAL GRANT, RECITAL & STATUTORY AUTHORITY",
            right-tag: doc-ref,
            primary-color: primary-color,
            accent-color: accent-color,
          )
          #hde-text-alignment-block(
            variant: if text-alignment-variant == "two_column_justified" { "centered_recital_quote" } else { text-alignment-variant },
            org-name: org-name,
            counterparty: counterparty,
            profession: profession,
            citation: citation,
            demand-signal: demand-signal,
            metrics: metrics,
            primary-color: primary-color,
            accent-color: accent-color,
          )
          #hde-kpi-strip-variation(
            variant: "metric_cards",
            metrics: metrics,
            primary-color: primary-color,
            accent-color: accent-color,
          )
          #hde-section-header(
            variant: section-header-variant,
            sec-num: "02",
            kicker: "SECTION 02 · CERTIFIED SCHEDULE",
            title: "SCHEDULE OF COVENANTS, GRANTS & VERIFIED AMOUNTS",
            subtitle: counterparty,
            right-tag: metrics.at(3).at(1),
            primary-color: primary-color,
            accent-color: accent-color,
          )
          #hde-schedule-table-variation(
            variant: "minimal_lines",
            headers: ("CLAUSE / REF", "CERTIFIED COVENANT, GRANT OR SCHEDULE ITEM", "STATUTORY BASIS", "VERIFIED AMOUNT"),
            rows: rows,
            summary-label: metrics.at(3).at(0),
            summary-value: metrics.at(3).at(1),
            primary-color: primary-color,
            accent-color: accent-color,
          )
          #v(2pt)
          #jurat-box(
            signer-name: org-name,
            signer-role: profession,
            counterparty-name: counterparty,
            signature-type: sig-type,
            notary-required: notary-required,
            stamp-label: stamp-label,
            stamp-sublabel: stamp-sublabel,
            stamp-color: accent-hex,
            primary-color: primary-color,
          )
        ]
      )
    )

  // ---------------------------------------------------------------------------
  // LAYOUT 9: DARK TACTILE 3D DEPTH BRIEF (`dark_tactile_brief`)
  // Modeled on `~/.agents/skills/typst/templates/social-visual-elements.typ`
  // ---------------------------------------------------------------------------
  } else if page-layout == "dark_tactile_brief" {
    rect(
      width: 100%,
      fill: rgb("#091b34"),
      stroke: 1.5pt + rgb("#254b7c"),
      radius: 6pt,
      inset: (x: 10pt, y: 4.5pt),
      [
        #grid(
          columns: (if seal-node != none { 46pt } else { 0pt }, 1fr, auto),
          gutter: 8pt,
          align: (center + horizon, left + horizon, right + horizon),
          [#if seal-node != none { box(fill: white, radius: 5pt, inset: 2pt, seal-node) }],
          [
            #hde-sticker-badge(label: upper(vertical), angle: -2deg, bg: rgb("#ffd000"), fg: rgb("#0f172a"))
            #h(8pt)
            #text(font: hde-font-mono, size: 6.6pt, weight: "bold", fill: rgb("#38bdf8"))[#doc-ref · #citation]
            #v(2pt)
            #text(font: hde-font-sans, size: 11.4pt, weight: "black", fill: white)[#title] \
            #v(1pt)
            #text(font: hde-font-sans, size: 7pt, fill: rgb("#cbd5e1"))[#org-name → #counterparty]
          ],
          svg-rubber-stamp(label: stamp-label, sublabel: stamp-sublabel, color-hex: "#38bdf8", angle: -4deg, width-in: 1.24in)
        )
      ]
    )
    v(2pt)
    grid(
      columns: (1fr,) * metrics.len(),
      gutter: 6pt,
      ..metrics.enumerate().map(((idx, m)) => hde-depth-card(
        accent-border: if idx == 3 { rgb("#10b981") } else { rgb("#254b7c") },
        fill-color: rgb("#0f172a"),
        shadow-color: if idx == 3 { rgb("#10b981") } else { primary-color },
        inset-val: (x: 6pt, y: 3.5pt),
        [
          #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: rgb("#94a3b8"))[#upper(m.at(0))] \
          #v(1pt)
          #text(font: hde-font-mono, size: 8.8pt, weight: "black", fill: if idx == 3 { rgb("#34d399") } else { rgb("#ffe500") })[#m.at(1)]
        ]
      ))
    )
    hde-section-header(
      variant: "neo_brutalist_tab",
      sec-num: "01",
      title: "EXECUTIVE BRIEF, SCOPE & FOLDED METRIC CALLOUT",
      right-tag: doc-ref,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-text-alignment-block(
      variant: if text-alignment-variant == "two_column_justified" { "asymmetric_callout_split" } else { text-alignment-variant },
      org-name: org-name,
      counterparty: counterparty,
      profession: profession,
      citation: citation,
      demand-signal: demand-signal,
      metrics: metrics,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-section-header(
      variant: section-header-variant,
      sec-num: "02",
      kicker: "SECTION 02 · DELIVERABLE SCHEDULE",
      title: "VERIFIED SCHEDULE OF DELIVERABLES & TELEMETRY",
      subtitle: counterparty,
      right-tag: metrics.at(3).at(1),
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-schedule-table-variation(
      variant: "dark_header",
      rows: rows,
      summary-label: metrics.at(3).at(0),
      summary-value: metrics.at(3).at(1),
      primary-color: primary-color,
      accent-color: accent-color,
    )
    grid(
      columns: (1.2fr, 1fr),
      gutter: 8pt,
      svg-architectural-diagram(
        kind: diagram-kind,
        title: "SYSTEM TOPOLOGY & TELEMETRY",
        primary-hex: primary-hex,
        accent-hex: accent-hex,
        metric-a: metrics.at(0).at(1),
        metric-b: metrics.at(3).at(1),
        hero-height: 48pt,
      ),
      hde-comparative-bar-chart(
        title: "PERFORMANCE & ALLOCATION BENCHMARK",
        rows: chart-rows,
        primary-color: primary-color,
        accent-color: accent-color,
      )
    )
    jurat-box(
      signer-name: org-name,
      signer-role: profession,
      counterparty-name: counterparty,
      signature-type: sig-type,
      notary-required: notary-required,
      stamp-label: stamp-label,
      stamp-sublabel: stamp-sublabel,
      stamp-color: accent-hex,
      primary-color: primary-color,
    )

  // ---------------------------------------------------------------------------
  // LAYOUT 10: EXECUTIVE DRAW & SCHEDULE OF VALUES LEDGER (`executive_draw_ledger`)
  // Wide trade command header + Retainage/Progress bar + SOV Table + Milestones
  // ---------------------------------------------------------------------------
  } else {
    hde-masthead-variation(
      variant: masthead-variant,
      title: title,
      subtitle: "Prepared for: " + counterparty + " · Target Role: " + profession,
      org-name: org-name,
      vertical-badge: vertical,
      doc-ref: doc-ref,
      date-str: date-str,
      citation: citation,
      seal-path: seal-path,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-kpi-strip-variation(
      variant: kpi-variant,
      metrics: metrics,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-section-header(
      variant: section-header-variant,
      sec-num: "01",
      kicker: "SECTION 01 · SCHEDULE OF VALUES",
      title: "CERTIFIED SCHEDULE OF VALUES & PROGRESS ALLOCATION",
      subtitle: counterparty,
      right-tag: doc-ref,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-schedule-table-variation(
      variant: table-variant,
      rows: rows,
      summary-label: metrics.at(3).at(0),
      summary-value: metrics.at(3).at(1),
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-section-header(
      variant: sec-hdr-2,
      sec-num: "02",
      kicker: "SECTION 02 · COVENANTS & TELEMETRY",
      title: "ALIGNED COVENANTS, SCHEMATIC & MILESTONE GATES",
      subtitle: org-name,
      right-tag: metrics.at(3).at(1),
      primary-color: primary-color,
      accent-color: accent-color,
    )
    hde-text-alignment-block(
      variant: text-alignment-variant,
      org-name: org-name,
      counterparty: counterparty,
      profession: profession,
      citation: citation,
      demand-signal: demand-signal,
      metrics: metrics,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    grid(
      columns: (1.25fr, 1fr),
      gutter: 8pt,
      svg-architectural-diagram(
        kind: diagram-kind,
        title: "TECHNICAL SCHEMATIC · " + vertical,
        primary-hex: primary-hex,
        accent-hex: accent-hex,
        metric-a: metrics.at(0).at(1),
        metric-b: metrics.at(3).at(1),
        hero-height: 54pt,
      ),
      hde-comparative-bar-chart(
        title: "FINANCIAL & VALUATION BREAKDOWN",
        rows: chart-rows,
        primary-color: primary-color,
        accent-color: accent-color,
      ),
    )
    jurat-box(
      signer-name: org-name,
      signer-role: profession,
      counterparty-name: counterparty,
      signature-type: sig-type,
      notary-required: notary-required,
      stamp-label: stamp-label,
      stamp-sublabel: stamp-sublabel,
      stamp-color: accent-hex,
      primary-color: primary-color,
    )
  }
}
"##;
