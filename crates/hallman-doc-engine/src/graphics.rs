//! Shared Typst Embedded Vector Graphics, Diagrams, Charts, Stamps & Multi-Style Element Variations
//!
//! Author: Daniel Hallman
//!
//! Pure-Typst + inline SVG vector graphics and layout variation primitives shared across
//! Untax (`untax.io`) and Pressmark (`pressmark.io`). Zero external runtime dependencies.

/// Shared Typst library of embedded SVG diagrams, charts, rubber stamps, and element style variations.
pub const HALLMAN_EMBEDDED_GRAPHICS_TYP: &str = r##"
// =============================================================================
// HALLMAN DOC ENGINE: EMBEDDED VECTOR GRAPHICS, DIAGRAMS & ELEMENT VARIATIONS
// Author: Daniel Hallman
// =============================================================================

#let hde-font-sans  = ("Helvetica Neue", "Arial")
#let hde-font-serif = ("Georgia", "Libertinus Serif", "Times New Roman")
#let hde-font-mono  = ("Menlo", "Courier New")

// -----------------------------------------------------------------------------
// 1. EMBEDDED VECTOR RUBBER STAMPS (SVG)
// -----------------------------------------------------------------------------
#let svg-rubber-stamp(
  label: "PAID IN FULL",
  sublabel: "VERIFIED RECORD",
  color-hex: "#059669",
  angle: -6deg,
) = {
  let safe-label = label.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
  let safe-sub = sublabel.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
  let main-fs = if safe-label.len() > 15 { "12" } else if safe-label.len() > 12 { "13.5" } else { "15" }
  let svg-str = (
    "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 228 66' fill='none'>"
    + "<rect x='3' y='3' width='222' height='60' rx='6' stroke='" + color-hex + "' stroke-width='3'/>"
    + "<rect x='7' y='7' width='214' height='52' rx='3.5' stroke='" + color-hex + "' stroke-width='1.1' stroke-dasharray='5 2.5'/>"
    + "<text x='114' y='32' text-anchor='middle' font-family='Arial, Helvetica, sans-serif' font-size='" + main-fs + "' font-weight='900' letter-spacing='1.4' fill='" + color-hex + "'>" + safe-label + "</text>"
    + "<line x1='20' y1='40' x2='208' y2='40' stroke='" + color-hex + "' stroke-width='1'/>"
    + "<text x='114' y='52' text-anchor='middle' font-family='Courier New, monospace' font-size='9' font-weight='700' letter-spacing='1.1' fill='" + color-hex + "'>" + safe-sub + "</text>"
    + "</svg>"
  )
  rotate(angle, image(bytes(svg-str), format: "svg", width: 1.58in))
}

// -----------------------------------------------------------------------------
// 2. EMBEDDED ARCHITECTURAL, TECHNICAL & CAD SCHEMATICS (6 DOMAIN DIAGRAMS)
// -----------------------------------------------------------------------------
#let svg-architectural-diagram(
  kind: "parcel_lot",
  title: "TECHNICAL SCHEMATIC & VERIFICATION DIAGRAM",
  primary-hex: "#1e3a8a",
  accent-hex: "#d97706",
  metric-a: "Primary Metric",
  metric-b: "Secondary Metric",
) = {
  let body-svg = if kind == "parcel_lot" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 104' fill='none'>"
      + "<rect x='2' y='2' width='536' height='100' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<g stroke='#e2e8f0' stroke-width='0.8'>"
      + "<line x1='0' y1='26' x2='540' y2='26'/><line x1='0' y1='52' x2='540' y2='52'/><line x1='0' y1='78' x2='540' y2='78'/>"
      + "<line x1='90' y1='0' x2='90' y2='104'/><line x1='180' y1='0' x2='180' y2='104'/><line x1='270' y1='0' x2='270' y2='104'/><line x1='360' y1='0' x2='360' y2='104'/><line x1='450' y1='0' x2='450' y2='104'/>"
      + "</g>"
      + "<polygon points='42,16 335,14 352,88 30,90' fill='#dbeafe' fill-opacity='0.55' stroke='" + primary-hex + "' stroke-width='2.2'/>"
      + "<polygon points='84,30 288,28 294,74 80,76' fill='#ffffff' stroke='" + accent-hex + "' stroke-width='1.8' stroke-dasharray='4 2'/>"
      + "<text x='187' y='49' text-anchor='middle' font-family='Arial, sans-serif' font-size='9' font-weight='bold' fill='#0f172a'>IMPROVED GLA FOOTPRINT</text>"
      + "<text x='187' y='63' text-anchor='middle' font-family='Courier New, monospace' font-size='7.8' font-weight='bold' fill='" + primary-hex + "'>SETBACK: 25 FT FRONT / 10 FT SIDE</text>"
      + "<text x='188' y='11' text-anchor='middle' font-family='Courier New, monospace' font-size='7.8' font-weight='bold' fill='" + primary-hex + "'>N 88°14'20\" E — 142.50 FT</text>"
      + "<text x='190' y='99' text-anchor='middle' font-family='Courier New, monospace' font-size='7.8' font-weight='bold' fill='" + primary-hex + "'>S 88°14'20\" W — 148.00 FT (R.O.W.)</text>"
      + "<circle cx='465' cy='40' r='19' fill='#ffffff' stroke='" + primary-hex + "' stroke-width='1.5'/>"
      + "<polygon points='465,23 470,42 465,38 460,42' fill='" + primary-hex + "'/>"
      + "<text x='465' y='18' text-anchor='middle' font-family='Arial, sans-serif' font-size='7.5' font-weight='bold' fill='" + primary-hex + "'>N</text>"
      + "<text x='465' y='78' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#0f172a'>NAD83 / STATE PLANE</text>"
      + "<text x='465' y='90' text-anchor='middle' font-family='Courier New, monospace' font-size='7.5' fill='#475569'>CADASTRAL LOT PLAT</text>"
      + "</svg>"
    )
  } else if kind == "roof_pitch" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 104' fill='none'>"
      + "<rect x='2' y='2' width='536' height='100' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<polygon points='40,82 175,20 300,82' fill='#e0f2fe' stroke='" + primary-hex + "' stroke-width='2'/>"
      + "<polygon points='175,20 375,20 465,82 300,82' fill='#fef3c7' stroke='" + accent-hex + "' stroke-width='2'/>"
      + "<line x1='175' y1='20' x2='175' y2='82' stroke='#64748b' stroke-width='1.2' stroke-dasharray='4 3'/>"
      + "<circle cx='285' cy='44' r='5.5' fill='#dc2626' fill-opacity='0.25' stroke='#dc2626' stroke-width='1.5'/>"
      + "<circle cx='330' cy='56' r='6.5' fill='#dc2626' fill-opacity='0.25' stroke='#dc2626' stroke-width='1.5'/>"
      + "<circle cx='368' cy='42' r='5' fill='#dc2626' fill-opacity='0.25' stroke='#dc2626' stroke-width='1.5'/>"
      + "<text x='135' y='64' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.5' font-weight='bold' fill='" + primary-hex + "'>FACET A (7/12 PITCH)</text>"
      + "<text x='325' y='73' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.5' font-weight='bold' fill='#92400e'>FACET B — HAIL IMPACT ZONE (12 SQ)</text>"
      + "<text x='275' y='14' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#0f172a'>RIDGE VENT: 48 LF • DRIP EDGE &amp; ICE/WATER SHIELD</text>"
      + "<text x='275' y='96' text-anchor='middle' font-family='Courier New, monospace' font-size='8' fill='#334155'>TOTAL ROOF AREA: 3,420 SQ FT (34.2 SQUARES) + 12% WASTE FACTOR</text>"
      + "</svg>"
    )
  } else if kind == "hvac_diagnostic" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 104' fill='none'>"
      + "<rect x='2' y='2' width='536' height='100' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<rect x='24' y='18' width='126' height='68' rx='5' fill='#ffffff' stroke='" + primary-hex + "' stroke-width='2'/>"
      + "<text x='87' y='38' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.5' font-weight='bold' fill='" + primary-hex + "'>RETURN PLENUM</text>"
      + "<text x='87' y='54' text-anchor='middle' font-family='Courier New, monospace' font-size='8' fill='#0f172a'>TEMP: 74.2°F DB</text>"
      + "<text x='87' y='68' text-anchor='middle' font-family='Courier New, monospace' font-size='7.8' fill='#475569'>STATIC: -0.18 IWC</text>"
      + "<path d='M154 52 L198 52' stroke='" + accent-hex + "' stroke-width='3' stroke-dasharray='5 3'/>"
      + "<rect x='202' y='14' width='136' height='76' rx='6' fill='#eff6ff' stroke='" + primary-hex + "' stroke-width='2.2'/>"
      + "<text x='270' y='33' text-anchor='middle' font-family='Arial, sans-serif' font-size='9' font-weight='bold' fill='#0f172a'>AIR HANDLER / COIL</text>"
      + "<text x='270' y='48' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#059669'>R-410A SUBCOOL: 11.4°F</text>"
      + "<text x='270' y='62' text-anchor='middle' font-family='Courier New, monospace' font-size='8' fill='#0f172a'>BLOWER: 1,620 CFM</text>"
      + "<text x='270' y='76' text-anchor='middle' font-family='Courier New, monospace' font-size='7.5' fill='#dc2626'>CAPACITOR: 45/5 MFD PASS</text>"
      + "<path d='M342 52 L386 52' stroke='" + primary-hex + "' stroke-width='3'/>"
      + "<rect x='390' y='18' width='126' height='68' rx='5' fill='#ffffff' stroke='#059669' stroke-width='2'/>"
      + "<text x='453' y='38' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.5' font-weight='bold' fill='#059669'>SUPPLY REGISTER</text>"
      + "<text x='453' y='54' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#0f172a'>SUPPLY: 54.6°F DB</text>"
      + "<text x='453' y='68' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#059669'>DELTA-T: 19.6°F PASS</text>"
      + "</svg>"
    )
  } else if kind == "solar_array" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 104' fill='none'>"
      + "<rect x='2' y='2' width='536' height='100' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<rect x='24' y='22' width='160' height='68' rx='4' fill='#1e3a8a' stroke='#0f172a' stroke-width='1.5'/>"
      + "<g stroke='#93c5fd' stroke-width='1'>"
      + "<line x1='64' y1='22' x2='64' y2='90'/><line x1='104' y1='22' x2='104' y2='90'/><line x1='144' y1='22' x2='144' y2='90'/>"
      + "<line x1='24' y1='45' x2='184' y2='45'/><line x1='24' y1='68' x2='184' y2='68'/>"
      + "</g>"
      + "<text x='104' y='15' text-anchor='middle' font-family='Courier New, monospace' font-size='7.2' font-weight='bold' fill='#0f172a'>24x 440W BIFACIAL ARRAY (10.56 kW)</text>"
      + "<rect x='214' y='26' width='120' height='58' rx='5' fill='#ffffff' stroke='#059669' stroke-width='2'/>"
      + "<text x='274' y='44' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.2' font-weight='bold' fill='#059669'>HYBRID INVERTER + LFP</text>"
      + "<text x='274' y='58' text-anchor='middle' font-family='Courier New, monospace' font-size='7.8' fill='#0f172a'>97.8% CEC EFFICIENCY</text>"
      + "<text x='274' y='71' text-anchor='middle' font-family='Courier New, monospace' font-size='7.8' fill='#475569'>13.5 kWh STORAGE</text>"
      + "<rect x='362' y='16' width='154' height='74' rx='5' fill='#ecfdf5' stroke='#059669' stroke-width='1.8'/>"
      + "<text x='439' y='34' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.2' font-weight='bold' fill='#047857'>25-YR PRODUCTION &amp; ITC</text>"
      + "<text x='439' y='49' text-anchor='middle' font-family='Courier New, monospace' font-size='9' font-weight='bold' fill='#0f172a'>15,840 kWh / YEAR 1</text>"
      + "<text x='439' y='64' text-anchor='middle' font-family='Courier New, monospace' font-size='8' font-weight='bold' fill='#059669'>30% IRC § 25D CREDIT</text>"
      + "<text x='439' y='78' text-anchor='middle' font-family='Courier New, monospace' font-size='7.5' fill='#334155'>OFFSET: 94% UTILITY LOAD</text>"
      + "</svg>"
    )
  } else if kind == "vehicle_inspection" {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 104' fill='none'>"
      + "<rect x='2' y='2' width='536' height='100' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<rect x='155' y='22' width='230' height='60' rx='12' fill='#ffffff' stroke='" + primary-hex + "' stroke-width='2'/>"
      + "<rect x='185' y='30' width='170' height='44' rx='5' fill='#f1f5f9' stroke='#64748b' stroke-width='1.2'/>"
      + "<rect x='172' y='13' width='38' height='9' rx='2.5' fill='#059669'/><rect x='330' y='13' width='38' height='9' rx='2.5' fill='#059669'/>"
      + "<rect x='172' y='82' width='38' height='9' rx='2.5' fill='#d97706'/><rect x='330' y='82' width='38' height='9' rx='2.5' fill='#059669'/>"
      + "<text x='18' y='36' font-family='Courier New, monospace' font-size='7.8' font-weight='bold' fill='#059669'>[PASS] LF BRAKE: 8mm</text>"
      + "<text x='18' y='78' font-family='Courier New, monospace' font-size='7.8' font-weight='bold' fill='#d97706'>[WARN] LR ROTOR: 3mm</text>"
      + "<text x='396' y='36' font-family='Courier New, monospace' font-size='7.8' font-weight='bold' fill='#059669'>[PASS] RF BRAKE: 8mm</text>"
      + "<text x='396' y='78' font-family='Courier New, monospace' font-size='7.8' font-weight='bold' fill='#059669'>[PASS] BATT CCA: 740A</text>"
      + "<text x='270' y='55' text-anchor='middle' font-family='Arial, sans-serif' font-size='7.8' font-weight='bold' fill='#0f172a'>CHASSIS &amp; POWERTRAIN TELEMETRY</text>"
      + "</svg>"
    )
  } else {
    (
      "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 540 104' fill='none'>"
      + "<rect x='2' y='2' width='536' height='100' rx='4' fill='#f8fafc' stroke='#cbd5e1' stroke-width='1'/>"
      + "<rect x='24' y='16' width='190' height='72' fill='#ffffff' stroke='" + primary-hex + "' stroke-width='2'/>"
      + "<rect x='214' y='16' width='150' height='72' fill='#eff6ff' stroke='" + primary-hex + "' stroke-width='2'/>"
      + "<rect x='364' y='16' width='152' height='72' fill='#fef3c7' stroke='" + accent-hex + "' stroke-width='2'/>"
      + "<text x='119' y='48' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.5' font-weight='bold' fill='#0f172a'>ZONE A — MAIN SUITE</text>"
      + "<text x='119' y='63' text-anchor='middle' font-family='Courier New, monospace' font-size='8' fill='" + primary-hex + "'>1,850 RSF / 1,610 USF</text>"
      + "<text x='289' y='48' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.5' font-weight='bold' fill='#0f172a'>ZONE B — LAB / OPS</text>"
      + "<text x='289' y='63' text-anchor='middle' font-family='Courier New, monospace' font-size='8' fill='" + primary-hex + "'>1,120 RSF • MEP TIER 2</text>"
      + "<text x='440' y='48' text-anchor='middle' font-family='Arial, sans-serif' font-size='8.5' font-weight='bold' fill='#92400e'>ZONE C — COMMON / ADA</text>"
      + "<text x='440' y='63' text-anchor='middle' font-family='Courier New, monospace' font-size='8' fill='#92400e'>640 RSF • LOAD FACTOR 14%</text>"
      + "</svg>"
    )
  }

  block(breakable: false, width: 100%, above: 4pt, below: 4pt)[
    #rect(
      width: 100%,
      stroke: 0.75pt + rgb("#cbd5e1"),
      radius: 4pt,
      fill: rgb("#ffffff"),
      inset: (x: 9pt, y: 5pt),
      [
        #grid(
          columns: (1fr, auto, auto),
          gutter: 6pt,
          align: (left + horizon, right + horizon, right + horizon),
          text(font: hde-font-sans, size: 7.2pt, weight: "bold", tracking: 0.5pt, fill: rgb(primary-hex))[
            SCHEMATIC • #upper(title)
          ],
          box(
            fill: rgb("#f1f5f9"),
            radius: 3pt,
            inset: (x: 5pt, y: 2pt),
            text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: rgb("#0f172a"))[#metric-a]
          ),
          box(
            fill: rgb("#ecfdf5"),
            radius: 3pt,
            inset: (x: 5pt, y: 2pt),
            text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: rgb("#047857"))[#metric-b]
          )
        )
        #v(3pt)
        #image(bytes(body-svg), format: "svg", width: 100%)
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
      inset: (x: 9pt, y: 6pt),
      [
        #text(font: hde-font-sans, size: 7.2pt, weight: "bold", tracking: 0.5pt, fill: primary-color)[
          ALLOCATION • #upper(title)
        ]
        #v(4pt)
        #for row in rows [
          #let pct = row.at(2)
          #let bar-fill = if row.len() > 3 and row.at(3) == "accent" { accent-color } else { primary-color }
          #grid(
            columns: (145pt, 1fr, 82pt),
            gutter: 8pt,
            align: (left + horizon, left + horizon, right + horizon),
            text(font: hde-font-sans, size: 7.6pt, weight: "bold", fill: rgb("#1e293b"))[#row.at(0)],
            box(width: 100%, height: 7.5pt, fill: rgb("#e2e8f0"), radius: 2.5pt)[
              #box(width: pct * 1%, height: 7.5pt, fill: bar-fill, radius: 2.5pt)
            ],
            text(font: hde-font-mono, size: 7.6pt, weight: "bold", fill: rgb("#0f172a"))[#row.at(1)]
          )
          #v(2.5pt)
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
        #text(font: hde-font-sans, size: 7.2pt, weight: "bold", tracking: 0.5pt, fill: primary-color)[
          TIMELINE • #upper(title)
        ]
        #v(4pt)
        #grid(
          columns: (1fr,) * steps.len(),
          gutter: 6pt,
          ..steps.enumerate().map(((idx, step)) => {
            let is-active = (idx == 0)
            rect(
              width: 100%,
              fill: if is-active { rgb("#eff6ff") } else { rgb("#f8fafc") },
              stroke: (top: 2pt + if is-active { primary-color } else { accent-color }, rest: 0.5pt + rgb("#cbd5e1")),
              radius: 3pt,
              inset: (x: 6pt, y: 4pt),
              [
                #text(font: hde-font-mono, size: 6.4pt, weight: "bold", fill: if is-active { primary-color } else { accent-color })[
                  PHASE #(idx + 1) • #step.at(2)
                ] \
                #v(1pt)
                #text(font: hde-font-sans, size: 7.6pt, weight: "bold", fill: rgb("#0f172a"))[#step.at(0)] \
                #v(0.5pt)
                #text(font: hde-font-sans, size: 6.8pt, fill: rgb("#475569"))[#step.at(1)]
              ]
            )
          })
        )
      ]
    )
  ]
}

// -----------------------------------------------------------------------------
// 5. 8-VARIANT MASTHEAD ENGINE
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
    box(width: 44pt, height: 44pt, image(seal-path, width: 44pt, height: 44pt, fit: "contain"))
  } else {
    none
  }

  if variant == "neo_brutalist" {
    rect(
      width: 100%,
      fill: rgb("#ffe500"),
      stroke: 2.2pt + rgb("#000000"),
      inset: (x: 10pt, y: 7pt),
      [
        #grid(
          columns: (if seal-node != none { 48pt } else { 0pt }, 1fr, 145pt),
          gutter: if seal-node != none { 8pt } else { 0pt },
          align: (left + horizon, left + horizon, right + top),
          [#if seal-node != none { seal-node }],
          [
            #box(fill: rgb("#000000"), inset: (x: 5pt, y: 2pt))[
              #text(font: hde-font-mono, size: 6.5pt, weight: "bold", fill: rgb("#ffe500"))[#upper(vertical-badge)]
            ]
            #v(2.5pt)
            #text(font: hde-font-sans, size: 12.5pt, weight: "black", fill: rgb("#000000"))[#upper(title)] \
            #text(font: hde-font-sans, size: 7.5pt, weight: "bold", fill: rgb("#1e293b"))[#subtitle]
          ],
          rect(
            width: 100%,
            fill: rgb("#ffffff"),
            stroke: 1.5pt + rgb("#000000"),
            inset: (x: 7pt, y: 5pt),
            align(right)[
              #text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: rgb("#000000"))[REF: #doc-ref] \
              #text(font: hde-font-mono, size: 6.5pt, fill: rgb("#334155"))[DATE: #date-str] \
              #text(font: hde-font-sans, size: 6.5pt, weight: "bold", fill: primary-color)[#org-name]
            ]
          )
        )
      ]
    )
  } else if variant == "swiss_grid" {
    table(
      columns: (if seal-node != none { 54pt } else { 0pt }, 1fr, 150pt),
      stroke: 0.75pt + rgb("#0f172a"),
      inset: (x: 8pt, y: 6pt),
      align: (center + horizon, left + horizon, left + top),
      [#if seal-node != none { seal-node }],
      [
        #text(font: hde-font-mono, size: 6.5pt, weight: "bold", tracking: 0.8pt, fill: primary-color)[01 :: #upper(vertical-badge)] \
        #v(1.5pt)
        #text(font: hde-font-sans, size: 12.2pt, weight: "bold", fill: rgb("#0f172a"))[#title] \
        #v(1pt)
        #text(font: hde-font-sans, size: 7.4pt, fill: rgb("#475569"))[#subtitle]
      ],
      [
        #text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: rgb("#64748b"))[ISSUING ENTITY] \
        #text(font: hde-font-sans, size: 7.4pt, weight: "bold", fill: rgb("#0f172a"))[#org-name] \
        #v(2pt)
        #text(font: hde-font-mono, size: 6.5pt, fill: primary-color)[ID: #doc-ref · #date-str]
      ]
    )
  } else if variant == "judicial_caption" {
    rect(
      width: 100%,
      stroke: (top: 2.2pt + primary-color, bottom: 1.2pt + primary-color, left: 0.75pt + rgb("#94a3b8"), right: 0.75pt + rgb("#94a3b8")),
      fill: rgb("#f8fafc"),
      inset: (x: 10pt, y: 6pt),
      [
        #grid(
          columns: (if seal-node != none { 48pt } else { 0pt }, 1fr, 155pt),
          gutter: 8pt,
          align: (center + horizon, left + horizon, left + horizon),
          [#if seal-node != none { seal-node }],
          [
            #text(font: hde-font-serif, size: 7.2pt, weight: "bold", tracking: 0.6pt, fill: primary-color)[#upper(vertical-badge)] \
            #v(1.5pt)
            #text(font: hde-font-serif, size: 12pt, weight: "bold", fill: rgb("#0f172a"))[#upper(title)] \
            #v(1pt)
            #text(font: hde-font-sans, size: 7.2pt, style: "italic", fill: rgb("#334155"))[#subtitle]
          ],
          rect(
            width: 100%,
            stroke: (left: 1.5pt + primary-color),
            inset: (left: 7pt, y: 2pt),
            [
              #text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: rgb("#0f172a"))[DOCKET / REF: #doc-ref] \
              #text(font: hde-font-mono, size: 6.5pt, fill: rgb("#475569"))[FILED: #date-str] \
              #text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: accent-color)[#citation]
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
      inset: (x: 10pt, y: 7pt),
      [
        #grid(
          columns: (if seal-node != none { 48pt } else { 0pt }, 1fr, 150pt),
          gutter: 8pt,
          align: (center + horizon, left + horizon, right + horizon),
          [#if seal-node != none { box(fill: white, radius: 4pt, inset: 2pt, seal-node) }],
          [
            #text(font: hde-font-mono, size: 6.5pt, weight: "bold", tracking: 0.6pt, fill: rgb("#fbbf24"))[★ #upper(vertical-badge) • #upper(org-name)] \
            #v(1.5pt)
            #text(font: hde-font-sans, size: 12.2pt, weight: "black", fill: white)[#upper(title)] \
            #v(1pt)
            #text(font: hde-font-sans, size: 7.2pt, fill: rgb("#cbd5e1"))[#subtitle]
          ],
          box(
            width: 100%,
            fill: rgb("#1e293b"),
            stroke: 0.75pt + accent-color,
            radius: 3pt,
            inset: (x: 7pt, y: 4pt),
            align(right)[
              #text(font: hde-font-mono, size: 7pt, weight: "bold", fill: rgb("#fbbf24"))[#doc-ref] \
              #text(font: hde-font-mono, size: 6.5pt, fill: white)[#date-str] \
              #text(font: hde-font-mono, size: 6pt, fill: rgb("#94a3b8"))[#citation]
            ]
          )
        )
      ]
    )
  } else if variant == "editorial_serif" {
    block(width: 100%, below: 2pt)[
      #line(length: 100%, stroke: 1.8pt + primary-color)
      #v(1.5pt)
      #line(length: 100%, stroke: 0.5pt + accent-color)
      #v(4pt)
      #grid(
        columns: (if seal-node != none { 48pt } else { 0pt }, 1fr, 150pt),
        gutter: 8pt,
        align: (left + horizon, left + horizon, right + horizon),
        [#if seal-node != none { seal-node }],
        [
          #text(font: hde-font-sans, size: 6.5pt, weight: "bold", tracking: 1pt, fill: accent-color)[#upper(org-name) • #upper(vertical-badge)] \
          #v(1.5pt)
          #text(font: hde-font-serif, size: 13pt, weight: "bold", fill: rgb("#0f172a"))[#title] \
          #v(1pt)
          #text(font: hde-font-serif, size: 7.8pt, style: "italic", fill: rgb("#475569"))[#subtitle]
        ],
        align(right)[
          #text(font: hde-font-mono, size: 7pt, weight: "bold", fill: primary-color)[#doc-ref] \
          #text(font: hde-font-sans, size: 6.8pt, fill: rgb("#475569"))[Issued: #date-str] \
          #text(font: hde-font-mono, size: 6.2pt, fill: accent-color)[#citation]
        ]
      )
      #v(4pt)
      #line(length: 100%, stroke: 0.75pt + rgb("#cbd5e1"))
    ]
  } else {
    rect(
      width: 100%,
      stroke: (left: 3.5pt + primary-color, rest: 0.75pt + rgb("#cbd5e1")),
      fill: rgb("#f8fafc"),
      radius: (right: 4pt, left: 1pt),
      inset: (x: 10pt, y: 6pt),
      [
        #grid(
          columns: (if seal-node != none { 48pt } else { 0pt }, 1fr, 150pt),
          gutter: 8pt,
          align: (center + horizon, left + horizon, right + horizon),
          [#if seal-node != none { seal-node }],
          [
            #box(fill: rgb("#dbeafe"), radius: 3pt, inset: (x: 5pt, y: 1.5pt))[
              #text(font: hde-font-sans, size: 6.5pt, weight: "bold", fill: primary-color)[#upper(vertical-badge)]
            ]
            #h(5pt)
            #text(font: hde-font-mono, size: 6.5pt, weight: "bold", fill: accent-color)[#citation] \
            #v(2pt)
            #text(font: hde-font-sans, size: 12.2pt, weight: "bold", fill: rgb("#0f172a"))[#title] \
            #v(1pt)
            #text(font: hde-font-sans, size: 7.4pt, fill: rgb("#475569"))[#subtitle]
          ],
          rect(
            width: 100%,
            fill: white,
            stroke: 0.5pt + rgb("#cbd5e1"),
            radius: 3pt,
            inset: (x: 7pt, y: 4pt),
            align(right)[
              #text(font: hde-font-sans, size: 7pt, weight: "bold", fill: rgb("#0f172a"))[#org-name] \
              #text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: primary-color)[#doc-ref] \
              #text(font: hde-font-mono, size: 6.4pt, fill: rgb("#64748b"))[#date-str]
            ]
          )
        )
      ]
    )
  }
}

// -----------------------------------------------------------------------------
// 6. 4-VARIANT KPI & FINANCIAL METRIC STRIP
// -----------------------------------------------------------------------------
#let hde-kpi-strip-variation(
  variant: "metric_cards",
  metrics: (),
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
) = {
  block(breakable: false, width: 100%, above: 4pt, below: 4pt)[
    #if variant == "dark_terminal" {
      rect(
        width: 100%,
        fill: rgb("#0f172a"),
        radius: 4pt,
        inset: (x: 9pt, y: 5.5pt),
        grid(
          columns: (1fr,) * metrics.len(),
          gutter: 7pt,
          ..metrics.enumerate().map(((idx, m)) => [
            #text(font: hde-font-mono, size: 6.4pt, weight: "bold", fill: rgb("#94a3b8"))[#upper(m.at(0))] \
            #v(1.5pt)
            #text(font: hde-font-mono, size: 9.8pt, weight: "bold", fill: if idx == metrics.len() - 1 { rgb("#34d399") } else { rgb("#38bdf8") })[#m.at(1)]
          ])
        )
      )
    } else if variant == "neo_brutalist" {
      grid(
        columns: (1fr,) * metrics.len(),
        gutter: 6pt,
        ..metrics.enumerate().map(((idx, m)) => rect(
          width: 100%,
          fill: if idx == metrics.len() - 1 { rgb("#00f0ff") } else { rgb("#ffffff") },
          stroke: 1.5pt + rgb("#000000"),
          inset: (x: 7pt, y: 5pt),
          [
            #text(font: hde-font-mono, size: 6.4pt, weight: "bold", fill: rgb("#000000"))[#upper(m.at(0))] \
            #v(1.5pt)
            #text(font: hde-font-mono, size: 9.8pt, weight: "black", fill: rgb("#000000"))[#m.at(1)]
          ]
        ))
      )
    } else {
      grid(
        columns: (1fr,) * metrics.len(),
        gutter: 6pt,
        ..metrics.enumerate().map(((idx, m)) => rect(
          width: 100%,
          fill: if idx == metrics.len() - 1 { rgb("#ecfdf5") } else { rgb("#f8fafc") },
          stroke: (top: 2pt + if idx == metrics.len() - 1 { accent-color } else { primary-color }, rest: 0.5pt + rgb("#cbd5e1")),
          radius: 3pt,
          inset: (x: 7pt, y: 5pt),
          [
            #text(font: hde-font-sans, size: 6.4pt, weight: "bold", fill: rgb("#475569"))[#upper(m.at(0))] \
            #v(1.5pt)
            #text(font: hde-font-mono, size: 9.8pt, weight: "bold", fill: if idx == metrics.len() - 1 { accent-color } else { rgb("#0f172a") })[#m.at(1)]
          ]
        ))
      )
    }
  ]
}

// -----------------------------------------------------------------------------
// 7. 4-VARIANT LINE-ITEM & EVIDENCE SCHEDULE TABLE
// -----------------------------------------------------------------------------
#let hde-schedule-table-variation(
  variant: "striped",
  headers: ("CODE", "DESCRIPTION & TECHNICAL SPECIFICATION", "BASIS / QTY", "AMOUNT / DELTA"),
  rows: (),
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
) = {
  block(breakable: false, width: 100%, above: 4pt, below: 4pt)[
    #if variant == "dark_header" or variant == "striped" {
      table(
        columns: (64pt, 1fr, 92pt, 84pt),
        inset: (x: 6pt, y: 4.5pt),
        stroke: 0.5pt + rgb("#cbd5e1"),
        align: (left + horizon, left + horizon, right + horizon, right + horizon),
        fill: (x, y) => if y == 0 {
          if variant == "dark_header" { rgb("#0f172a") } else { primary-color }
        } else if calc.odd(y) {
          rgb("#ffffff")
        } else {
          rgb("#f8fafc")
        },
        ..headers.map(h => text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: white)[#upper(h)]),
        ..rows.map(r => (
          text(font: hde-font-mono, size: 7.2pt, weight: "bold", fill: primary-color)[#r.at(0)],
          text(font: hde-font-sans, size: 7.6pt, fill: rgb("#1e293b"))[#r.at(1)],
          text(font: hde-font-mono, size: 7.2pt, fill: rgb("#475569"))[#r.at(2)],
          text(font: hde-font-mono, size: 7.6pt, weight: "bold", fill: rgb("#0f172a"))[#r.at(3)],
        )).flatten()
      )
    } else {
      table(
        columns: (64pt, 1fr, 92pt, 84pt),
        inset: (x: 6pt, y: 4.5pt),
        stroke: (x, y) => if y == 0 { (bottom: 1.5pt + primary-color, top: 1.5pt + primary-color) } else { (bottom: 0.5pt + rgb("#cbd5e1")) },
        align: (left + horizon, left + horizon, right + horizon, right + horizon),
        ..headers.map(h => text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: primary-color)[#upper(h)]),
        ..rows.map(r => (
          text(font: hde-font-mono, size: 7.2pt, weight: "bold", fill: rgb("#334155"))[#r.at(0)],
          text(font: hde-font-sans, size: 7.6pt, fill: rgb("#0f172a"))[#r.at(1)],
          text(font: hde-font-mono, size: 7.2pt, fill: rgb("#475569"))[#r.at(2)],
          text(font: hde-font-mono, size: 7.6pt, weight: "bold", fill: accent-color)[#r.at(3)],
        )).flatten()
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
  block(breakable: false, width: 100%, above: 4pt)[
    #rect(
      width: 100%,
      stroke: 0.85pt + primary-color,
      radius: 4pt,
      fill: rgb("#f8fafc"),
      inset: (x: 9pt, y: 6pt),
      [
        #grid(
          columns: (1fr, auto),
          gutter: 8pt,
          align: (left + top, right + horizon),
          [
            #text(font: hde-font-sans, size: 7pt, weight: "bold", tracking: 0.5pt, fill: primary-color)[
              EXECUTION • ATTESTATION & INTERACTIVE SIGNATURE BLOCK (#upper(signature-type))
            ]
            #v(2pt)
            #text(font: hde-font-sans, size: 7pt, fill: rgb("#334155"))[
              I hereby certify and affirm that the schedules, technical specifications, and monetary figures stated herein are true, accurate, and complete.
            ]
            #v(5pt)
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
                    height: 17pt,
                    fill: white,
                    stroke: (bottom: 1.2pt + rgb("#0f172a"), rest: 0.5pt + rgb("#94a3b8")),
                    radius: 2pt,
                    inset: (x: 5pt, y: 3pt),
                    align(left + horizon)[
                      #text(font: hde-font-mono, size: 6.5pt, fill: rgb("#64748b"))[SIGN OR TYPE HERE (INTERACTIVE /ACROFORM FIELD)]
                    ]
                  )
                ]
                #v(1.5pt)
                #text(font: hde-font-sans, size: 7pt, weight: "bold", fill: rgb("#0f172a"))[#signer-name]
                #text(font: hde-font-sans, size: 6.5pt, fill: rgb("#475569"))[ — #signer-role]
              ],
              [
                #if signature-type == "esign" [
                  #text(font: hde-font-mono, size: 8.8pt, weight: "bold", fill: rgb("#0f172a"))[2026-11-06 · ESIGN-SHA256]
                  #v(2pt)
                ] else [
                  #rect(
                    width: 100%,
                    height: 17pt,
                    fill: white,
                    stroke: (bottom: 1.2pt + rgb("#0f172a"), rest: 0.5pt + rgb("#94a3b8")),
                    radius: 2pt,
                    inset: (x: 5pt, y: 3pt),
                    align(left + horizon)[
                      #text(font: hde-font-mono, size: 6.5pt, fill: rgb("#64748b"))[DATE (MM/DD/YYYY)]
                    ]
                  )
                ]
                #v(1.5pt)
                #text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: rgb("#334155"))[Counterparty: #counterparty-name]
              ]
            )
            #if notary-required [
              #v(4pt)
              #line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1"))
              #v(3pt)
              #grid(
                columns: (1fr, 102pt),
                gutter: 8pt,
                align: (left + horizon, center + horizon),
                [
                  #text(font: hde-font-serif, size: 6.8pt, weight: "bold", fill: rgb("#0f172a"))[
                    STATE OF TENNESSEE / COMMONWEALTH · COUNTY OF RECORD — NOTARY PUBLIC JURAT:
                  ] \
                  #text(font: hde-font-serif, size: 6.5pt, fill: rgb("#334155"))[
                    Subscribed and sworn to before me on this \_\_\_\_\_ day of \_\_\_\_\_\_\_\_\_\_, 2026. Notary Signature: \_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_ · Exp: \_\_\_\_\_\_\_\_\_\_
                  ]
                ],
                rect(
                  width: 100%,
                  height: 20pt,
                  stroke: (dash: "dashed", paint: rgb("#64748b"), thickness: 0.75pt),
                  radius: 2pt,
                  align(center + horizon)[
                    #text(font: hde-font-mono, size: 6pt, weight: "bold", fill: rgb("#475569"))[OFFICIAL NOTARY SEAL]
                  ]
                )
              )
            ]
          ],
          svg-rubber-stamp(label: stamp-label, sublabel: stamp-sublabel, color-hex: stamp-color, angle: -5deg)
        )
      ]
    )
  ]
}
"##;
