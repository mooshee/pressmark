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
// 7. LINE-ITEM & EVIDENCE SCHEDULE TABLE
// -----------------------------------------------------------------------------
#let hde-schedule-table-variation(
  variant: "striped",
  headers: ("CODE", "DESCRIPTION & TECHNICAL SPECIFICATION", "BASIS / QTY", "AMOUNT / DELTA"),
  rows: (),
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
) = {
  block(breakable: false, width: 100%, above: 3pt, below: 3pt)[
    #if variant == "dark_header" or variant == "striped" {
      table(
        columns: (58pt, 1fr, 84pt, 76pt),
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
          text(font: hde-font-sans, size: 7.5pt, fill: rgb("#1e293b"))[#r.at(1)],
          text(font: hde-font-mono, size: 7.2pt, fill: rgb("#475569"))[#r.at(2)],
          text(font: hde-font-mono, size: 7.6pt, weight: "bold", fill: rgb("#0f172a"))[#r.at(3)],
        )).flatten()
      )
    } else {
      table(
        columns: (58pt, 1fr, 84pt, 76pt),
        inset: (x: 6pt, y: 4.5pt),
        stroke: (x, y) => if y == 0 { (bottom: 1.5pt + primary-color, top: 1.5pt + primary-color) } else { (bottom: 0.5pt + rgb("#cbd5e1")) },
        align: (left + horizon, left + horizon, right + horizon, right + horizon),
        ..headers.map(h => text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: primary-color)[#upper(h)]),
        ..rows.map(r => (
          text(font: hde-font-mono, size: 7.2pt, weight: "bold", fill: rgb("#334155"))[#r.at(0)],
          text(font: hde-font-sans, size: 7.5pt, fill: rgb("#0f172a"))[#r.at(1)],
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
    image(seal-path, width: 54pt, height: 54pt, fit: "contain")
  } else {
    none
  }

  // ---------------------------------------------------------------------------
  // LAYOUT 1: ASYMMETRIC SIDEBAR LEDGER (`asymmetric_sidebar`)
  // Left full-height dark slate/accent anchor rail + Right open executive ledger
  // ---------------------------------------------------------------------------
  if page-layout == "asymmetric_sidebar" {
    grid(
      columns: (154pt, 1fr),
      gutter: 11pt,
      rect(
        width: 100%,
        fill: rgb("#0f172a"),
        stroke: (right: 3pt + accent-color),
        radius: 5pt,
        inset: (x: 9pt, y: 9pt),
        [
          #if seal-node != none [
            #align(center, box(fill: white, radius: 6pt, inset: 3pt, seal-node))
            #v(5pt)
          ]
          #box(fill: accent-color, radius: 3pt, inset: (x: 6pt, y: 2pt))[
            #text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: white)[#upper(vertical)]
          ]
          #v(4pt)
          #text(font: hde-font-sans, size: 9.5pt, weight: "black", fill: white)[#org-name]
          #v(2pt)
          #text(font: hde-font-sans, size: 7pt, fill: rgb("#94a3b8"))[#profession]
          #v(6pt)
          #line(length: 100%, stroke: 0.5pt + rgb("#334155"))
          #v(6pt)
          #text(font: hde-font-mono, size: 6pt, weight: "bold", fill: rgb("#38bdf8"))[DOCUMENT REFERENCE] \
          #text(font: hde-font-mono, size: 7.4pt, weight: "bold", fill: white)[#doc-ref · #date-str]
          #v(5pt)
          #text(font: hde-font-mono, size: 6pt, weight: "bold", fill: rgb("#38bdf8"))[COUNTERPARTY / CLIENT] \
          #text(font: hde-font-sans, size: 7.2pt, weight: "bold", fill: rgb("#f8fafc"))[#counterparty]
          #v(5pt)
          #text(font: hde-font-mono, size: 6pt, weight: "bold", fill: rgb("#38bdf8"))[GOVERNANCE STANDARD] \
          #text(font: hde-font-mono, size: 6.5pt, fill: rgb("#cbd5e1"))[#citation]
          #v(6pt)
          #line(length: 100%, stroke: 0.5pt + rgb("#334155"))
          #v(6pt)
          #for (idx, m) in metrics.enumerate() [
            #rect(
              width: 100%,
              fill: if idx == metrics.len() - 1 { rgb("#064e3b") } else { rgb("#1e293b") },
              stroke: 0.6pt + if idx == metrics.len() - 1 { rgb("#10b981") } else { rgb("#334155") },
              radius: 3.5pt,
              inset: (x: 6pt, y: 4pt),
              [
                #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: rgb("#94a3b8"))[#upper(m.at(0))] \
                #v(1pt)
                #text(font: hde-font-mono, size: 8.4pt, weight: "bold", fill: if idx == metrics.len() - 1 { rgb("#34d399") } else { white })[#m.at(1)]
              ]
            )
            #v(3pt)
          ]
          #v(4pt)
          #align(center, svg-rubber-stamp(label: stamp-label, sublabel: stamp-sublabel, color-hex: accent-hex, angle: -4deg, width-in: 1.32in))
        ]
      ),
      [
        #rect(
          width: 100%,
          fill: rgb("#f8fafc"),
          stroke: (bottom: 2pt + primary-color),
          inset: (x: 9pt, y: 6pt),
          [
            #text(font: hde-font-mono, size: 6.5pt, weight: "bold", tracking: 0.8pt, fill: primary-color)[
              EXECUTIVE ENGAGEMENT & COMMERCIAL LEDGER · #doc-ref
            ] \
            #v(2pt)
            #text(font: hde-font-sans, size: 12.5pt, weight: "black", fill: rgb("#0f172a"))[#title] \
            #v(2pt)
            #text(font: hde-font-sans, size: 7.3pt, fill: rgb("#475569"))[#demand-signal]
          ]
        )
        #v(2.5pt)
        #hde-schedule-table-variation(
          variant: table-variant,
          rows: rows,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #v(2pt)
        #hde-comparative-bar-chart(
          title: "COMMERCIAL VALUE & COST ALLOCATION SUMMARY",
          rows: chart-rows,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #v(2pt)
        #hde-milestone-timeline(
          title: "DELIVERABLE PHASES & SETTLEMENT SCHEDULE",
          steps: milestones,
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

  // ---------------------------------------------------------------------------
  // LAYOUT 2: FORMAL COURT / BOARD PLEADING CAPTION (`pleading_caption`)
  // Left numbered pleading margin ruler (1..22) + Formal Caption Box (`v.`)
  // ---------------------------------------------------------------------------
  } else if page-layout == "pleading_caption" {
    grid(
      columns: (22pt, 1fr),
      gutter: 9pt,
      rect(
        width: 100%,
        stroke: (right: 1.4pt + primary-color),
        inset: (right: 4pt, y: 1pt),
        align(right)[
          #set text(font: hde-font-mono, size: 7.2pt, weight: "bold", fill: rgb("#64748b"))
          #for line-no in range(1, 24) [
            #box(height: 22pt, align(right + horizon)[#line-no]) \
          ]
        ]
      ),
      [
        #set text(font: hde-font-serif, size: 8.8pt, fill: rgb("#0f172a"))
        #rect(
          width: 100%,
          stroke: 1.2pt + rgb("#0f172a"),
          fill: rgb("#f8fafc"),
          inset: (x: 10pt, y: 7pt),
          grid(
            columns: (1.25fr, 16pt, 0.95fr),
            align: (left + horizon, center + horizon, left + horizon),
            [
              #text(font: hde-font-serif, size: 7.6pt, weight: "bold", tracking: 0.5pt, fill: primary-color)[
                IN THE MATTER OF THE STATUTORY RECORD / TRIBUNAL:
              ] \
              #v(3pt)
              #text(weight: "bold", size: 9pt)[#upper(org-name)], \
              #h(8pt)#text(size: 7.6pt, style: "italic")[Petitioner / Claimant / Issuing Party,] \
              #v(1.5pt)
              #h(20pt)#text(weight: "bold", style: "italic")[v.] \
              #v(1.5pt)
              #text(weight: "bold", size: 8.8pt)[#upper(counterparty)], \
              #h(8pt)#text(size: 7.6pt, style: "italic")[Respondent / Counterparty of Record.]
            ],
            [
              #for _ in range(1, 7) [ #text(font: hde-font-mono, size: 8.5pt, weight: "bold")[)] \ ]
            ],
            [
              #if seal-node != none [
                #place(top + right, box(width: 36pt, height: 36pt, image(seal-path, width: 36pt)))
              ]
              #text(font: hde-font-mono, size: 7.2pt, weight: "bold")[DOCKET / INSTRUMENT NO.:] \
              #text(font: hde-font-mono, size: 8.2pt, weight: "bold", fill: primary-color)[#doc-ref] \
              #v(2pt)
              #text(font: hde-font-mono, size: 7pt)[EFFECTIVE DATE: #date-str] \
              #text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: accent-color)[STATUTE: #citation]
            ]
          )
        )
        #v(5pt)
        #align(center)[
          #text(font: hde-font-serif, size: 11pt, weight: "bold", tracking: 0.4pt)[#upper(title)]
          #v(1.5pt)
          #line(length: 65%, stroke: 0.75pt + primary-color)
        ]
        #v(4pt)
        #text(font: hde-font-serif, size: 8.2pt)[
          *COMES NOW* the undersigned party of record (*#org-name*) and respectfully submits this verified statutory instrument and evidentiary schedule pursuant to *#citation*:
        ]
        #v(4pt)
        #rect(
          width: 100%,
          stroke: (left: 2.5pt + primary-color, rest: 0.5pt + rgb("#cbd5e1")),
          fill: white,
          inset: (x: 9pt, y: 6pt),
          [
            #text(font: hde-font-sans, size: 7pt, weight: "bold", fill: primary-color)[
              I. VERIFIED STATUTORY FINDINGS & JURISDICTIONAL METRICS
            ]
            #v(3pt)
            #grid(
              columns: (1fr, 1fr),
              gutter: 7pt,
              ..metrics.enumerate().map(((idx, m)) => [
                #text(font: hde-font-serif, size: 7.8pt)[
                  *¶ #(idx + 1).* *#m.at(0):* #text(font: hde-font-mono, weight: "bold", fill: if idx == 3 { accent-color } else { rgb("#0f172a") })[#m.at(1)]
                ]
              ])
            )
          ]
        )
        #v(4pt)
        #text(font: hde-font-sans, size: 7.2pt, weight: "bold", fill: primary-color)[
          II. CERTIFIED SCHEDULE OF EVIDENCE, LINE ITEMS & STATUTORY BASIS
        ]
        #hde-schedule-table-variation(
          variant: "dark_header",
          rows: rows,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #v(2pt)
        #hde-milestone-timeline(
          title: "III. PROCEDURAL DEADLINES, CURATIVE WINDOW & COMPLIANCE STEPS",
          steps: milestones,
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
    v(3pt)
    text(font: hde-font-sans, size: 7.4pt, weight: "bold", tracking: 0.5pt, fill: primary-color)[
      SELECTABLE SERVICE TIERS & COMMERCIAL PACKAGE COMPARISON
    ]
    v(2.5pt)
    grid(
      columns: (1fr, 1.08fr, 1fr),
      gutter: 8pt,
      // Tier 1
      rect(
        width: 100%,
        stroke: 0.85pt + rgb("#cbd5e1"),
        radius: 5pt,
        fill: rgb("#ffffff"),
        inset: (x: 8pt, y: 6pt),
        [
          #text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: rgb("#64748b"))[OPTION A · ESSENTIAL TIER] \
          #v(1.5pt)
          #text(font: hde-font-sans, size: 9pt, weight: "bold", fill: rgb("#0f172a"))[#metrics.at(0).at(0)] \
          #v(2pt)
          #text(font: hde-font-mono, size: 11.5pt, weight: "black", fill: primary-color)[#metrics.at(0).at(1)] \
          #v(3pt)
          #line(length: 100%, stroke: 0.5pt + rgb("#e2e8f0"))
          #v(3pt)
          #text(font: hde-font-sans, size: 7.1pt, fill: rgb("#334155"))[
            #text(fill: accent-color, weight: "bold")[✓] #rows.at(0).at(1) \
            #v(1.5pt)
            #text(fill: accent-color, weight: "bold")[✓] Basis: #rows.at(0).at(2) \
            #v(1.5pt)
            #text(fill: accent-color, weight: "bold")[✓] Phase 1: #milestones.at(0).at(0)
          ]
        ]
      ),
      // Tier 2 (Recommended Hero Card)
      rect(
        width: 100%,
        stroke: 2pt + accent-color,
        radius: 5pt,
        fill: rgb("#f0fdf4"),
        inset: (x: 8pt, y: 6pt),
        [
          #grid(
            columns: (1fr, auto),
            align: (left + horizon, right + horizon),
            text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: accent-color)[OPTION B · RECOMMENDED],
            box(fill: accent-color, radius: 99pt, inset: (x: 5pt, y: 1.5pt))[
              #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: white)[★ MOST POPULAR]
            ]
          )
          #v(1.5pt)
          #text(font: hde-font-sans, size: 9.2pt, weight: "black", fill: rgb("#0f172a"))[#metrics.at(3).at(0)] \
          #v(2pt)
          #text(font: hde-font-mono, size: 12pt, weight: "black", fill: accent-color)[#metrics.at(3).at(1)] \
          #v(3pt)
          #line(length: 100%, stroke: 0.75pt + accent-color)
          #v(3pt)
          #text(font: hde-font-sans, size: 7.2pt, fill: rgb("#0f172a"))[
            #text(fill: accent-color, weight: "bold")[✓] *#rows.at(1).at(1)* \
            #v(1.5pt)
            #text(fill: accent-color, weight: "bold")[✓] *#rows.at(2).at(1)* \
            #v(1.5pt)
            #text(fill: accent-color, weight: "bold")[✓] Statutory Compliance: #citation
          ]
        ]
      ),
      // Tier 3
      rect(
        width: 100%,
        stroke: 1pt + primary-color,
        radius: 5pt,
        fill: rgb("#f8fafc"),
        inset: (x: 8pt, y: 6pt),
        [
          #text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: primary-color)[OPTION C · PREMIER / FULL] \
          #v(1.5pt)
          #text(font: hde-font-sans, size: 9pt, weight: "bold", fill: rgb("#0f172a"))[#metrics.at(1).at(0)] \
          #v(2pt)
          #text(font: hde-font-mono, size: 11.5pt, weight: "black", fill: rgb("#0f172a"))[#metrics.at(1).at(1)] \
          #v(3pt)
          #line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1"))
          #v(3pt)
          #text(font: hde-font-sans, size: 7.1pt, fill: rgb("#334155"))[
            #text(fill: primary-color, weight: "bold")[✓] #rows.at(3).at(1) \
            #v(1.5pt)
            #text(fill: primary-color, weight: "bold")[✓] Metric: #metrics.at(2).at(0) (#metrics.at(2).at(1)) \
            #v(1.5pt)
            #text(fill: primary-color, weight: "bold")[✓] Priority SLA: #milestones.at(3).at(2)
          ]
        ]
      )
    )
    v(2.5pt)
    hde-schedule-table-variation(
      variant: table-variant,
      headers: ("PACKAGE / CODE", "INCLUDED DELIVERABLES, SPECIFICATIONS & SLA", "ALLOCATION / QTY", "FEE / VALUE"),
      rows: rows,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    v(2pt)
    hde-milestone-timeline(
      title: "ONBOARDING & FULFILLMENT TIMELINE",
      steps: milestones,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    v(2pt)
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
    v(4pt)
    rect(
      width: 100%,
      fill: rgb("#0f172a"),
      radius: 4pt,
      inset: (x: 10pt, y: 5pt),
      grid(
        columns: (auto, 1fr),
        gutter: 10pt,
        align: (left + horizon, right + horizon),
        text(font: hde-font-mono, size: 7pt, weight: "bold", fill: rgb("#ffe500"))[
          GRADING KEY: #h(4pt) #text(fill: white, weight: "regular")[[☑ PASS]  [☐ ATTN]  [☐ FAIL]  [☐ N/A]]
        ],
        text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: rgb("#34d399"))[
          STANDARD: #citation
        ]
      )
    )
    v(4pt)
    hde-kpi-strip-variation(
      variant: kpi-variant,
      metrics: metrics,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    v(4pt)
    table(
      columns: (58pt, 1.05fr, 1fr, 82pt, 78pt),
      inset: (x: 6pt, y: 5.5pt),
      stroke: 0.5pt + rgb("#cbd5e1"),
      align: (left + horizon, left + horizon, left + horizon, center + horizon, right + horizon),
      fill: (x, y) => if y == 0 { primary-color } else if calc.odd(y) { white } else { rgb("#f8fafc") },
      text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: white)[ITEM CODE],
      text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: white)[INSPECTION CHECKPOINT & SYSTEM],
      text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: white)[GRADE CHECKBOXES],
      text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: white)[OBSERVED SPEC],
      text(font: hde-font-sans, size: 6.8pt, weight: "bold", fill: white)[COST / STATUS],
      ..rows.enumerate().map(((idx, r)) => (
        text(font: hde-font-mono, size: 7.2pt, weight: "bold", fill: primary-color)[#r.at(0)],
        text(font: hde-font-sans, size: 7.5pt, fill: rgb("#0f172a"))[#r.at(1)],
        text(font: hde-font-mono, size: 7pt, weight: "bold", fill: if idx == 1 { rgb("#b45309") } else { rgb("#047857") })[
          #if idx == 1 { "[☐ PASS  ☑ ATTN  ☐ FAIL]" } else { "[☑ PASS  ☐ ATTN  ☐ FAIL]" }
        ],
        text(font: hde-font-mono, size: 7pt, fill: rgb("#475569"))[#r.at(2)],
        text(font: hde-font-mono, size: 7.4pt, weight: "bold", fill: rgb("#0f172a"))[#r.at(3)],
      )).flatten()
    )
    v(4pt)
    svg-architectural-diagram(
      kind: diagram-kind,
      title: "FIELD DIAGNOSTIC TELEMETRY & INSPECTION SCHEMATIC",
      primary-hex: primary-hex,
      accent-hex: accent-hex,
      metric-a: metrics.at(0).at(1),
      metric-b: metrics.at(3).at(1),
    )
    v(4pt)
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
      inset: 9pt,
      [
        #table(
          columns: (if seal-node != none { 52pt } else { 0pt }, 1fr, 115pt, 115pt),
          stroke: 0.75pt + rgb("#0f172a"),
          inset: (x: 8pt, y: 5pt),
          fill: (x, y) => if x == 1 { rgb("#f8fafc") } else { white },
          align: (center + horizon, left + horizon, left + horizon, left + horizon),
          [#if seal-node != none { seal-node }],
          [
            #text(font: hde-font-mono, size: 6.5pt, weight: "bold", fill: primary-color)[ENGINEERING / CAD SUBMITTAL · #upper(vertical)] \
            #v(1.5pt)
            #text(font: hde-font-sans, size: 11.5pt, weight: "black", fill: rgb("#0f172a"))[#upper(title)] \
            #v(1pt)
            #text(font: hde-font-mono, size: 6.8pt, fill: rgb("#475569"))[SITE / CLIENT: #counterparty]
          ],
          [
            #text(font: hde-font-mono, size: 6pt, fill: rgb("#64748b"))[DRAWING / SPEC NO.] \
            #text(font: hde-font-mono, size: 7.8pt, weight: "bold", fill: primary-color)[#doc-ref] \
            #text(font: hde-font-mono, size: 6.4pt, fill: rgb("#334155"))[SCALE: AS NOTED]
          ],
          [
            #text(font: hde-font-mono, size: 6pt, fill: rgb("#64748b"))[CODE / STANDARD] \
            #text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: accent-color)[#citation] \
            #text(font: hde-font-mono, size: 6.4pt, fill: rgb("#334155"))[DATE: #date-str]
          ]
        )
        #v(6pt)
        #svg-architectural-diagram(
          kind: diagram-kind,
          title: "FIGURE 1.0 — CERTIFIED AS-BUILT ENGINEERING & GEOMETRY SCHEMATIC",
          primary-hex: primary-hex,
          accent-hex: accent-hex,
          metric-a: metrics.at(0).at(1),
          metric-b: metrics.at(3).at(1),
          hero-height: 122pt,
        )
        #v(4pt)
        #hde-kpi-strip-variation(
          variant: "dark_terminal",
          metrics: metrics,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #v(4pt)
        #text(font: hde-font-mono, size: 7.2pt, weight: "bold", fill: primary-color)[
          BILL OF MATERIALS (BOM), COMPONENT TOLERANCES & SCHEDULE OF QUANTITIES
        ]
        #hde-schedule-table-variation(
          variant: "dark_header",
          headers: ("SPEC ID", "ENGINEERING COMPONENT / MATERIAL SPECIFICATION", "TOLERANCE / QTY", "EXTENDED VALUE"),
          rows: rows,
          primary-color: primary-color,
          accent-color: accent-color,
        )
        #v(4pt)
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
    v(3pt)
    hde-kpi-strip-variation(
      variant: kpi-variant,
      metrics: metrics,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    v(3pt)
    grid(
      columns: (1fr, 1fr),
      gutter: 9pt,
      // Left T-Account Column
      rect(
        width: 100%,
        stroke: (top: 2.5pt + primary-color, rest: 0.75pt + rgb("#cbd5e1")),
        radius: 4pt,
        fill: rgb("#ffffff"),
        inset: (x: 8pt, y: 6pt),
        [
          #text(font: hde-font-mono, size: 7pt, weight: "bold", fill: primary-color)[
            SCHEDULE A · PRIMARY DEBITS / GROSS BASIS
          ]
          #v(3pt)
          #table(
            columns: (50pt, 1fr, 70pt),
            inset: (x: 4.5pt, y: 4pt),
            stroke: (bottom: 0.5pt + rgb("#e2e8f0")),
            align: (left + horizon, left + horizon, right + horizon),
            text(font: hde-font-mono, size: 6.4pt, weight: "bold", fill: rgb("#64748b"))[CODE],
            text(font: hde-font-sans, size: 6.4pt, weight: "bold", fill: rgb("#64748b"))[ITEM DESCRIPTION],
            text(font: hde-font-mono, size: 6.4pt, weight: "bold", fill: rgb("#64748b"))[AMOUNT],
            text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: primary-color)[#rows.at(0).at(0)],
            text(font: hde-font-sans, size: 7pt)[#rows.at(0).at(1)],
            text(font: hde-font-mono, size: 7pt, weight: "bold")[#rows.at(0).at(3)],
            text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: primary-color)[#rows.at(1).at(0)],
            text(font: hde-font-sans, size: 7pt)[#rows.at(1).at(1)],
            text(font: hde-font-mono, size: 7pt, weight: "bold")[#rows.at(1).at(3)],
          )
          #v(3.5pt)
          #rect(width: 100%, fill: rgb("#eff6ff"), radius: 3pt, inset: (x: 6pt, y: 4pt))[
            #grid(
              columns: (1fr, auto),
              text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: primary-color)[#metrics.at(0).at(0)],
              text(font: hde-font-mono, size: 8pt, weight: "bold", fill: rgb("#0f172a"))[#metrics.at(0).at(1)]
            )
          ]
        ]
      ),
      // Right T-Account Column
      rect(
        width: 100%,
        stroke: (top: 2.5pt + accent-color, rest: 0.75pt + rgb("#cbd5e1")),
        radius: 4pt,
        fill: rgb("#ffffff"),
        inset: (x: 8pt, y: 6pt),
        [
          #text(font: hde-font-mono, size: 7pt, weight: "bold", fill: accent-color)[
            SCHEDULE B · CREDITS / PRORATIONS & ADJUSTMENTS
          ]
          #v(3pt)
          #table(
            columns: (50pt, 1fr, 70pt),
            inset: (x: 4.5pt, y: 4pt),
            stroke: (bottom: 0.5pt + rgb("#e2e8f0")),
            align: (left + horizon, left + horizon, right + horizon),
            text(font: hde-font-mono, size: 6.4pt, weight: "bold", fill: rgb("#64748b"))[CODE],
            text(font: hde-font-sans, size: 6.4pt, weight: "bold", fill: rgb("#64748b"))[CREDIT / OFFSET],
            text(font: hde-font-mono, size: 6.4pt, weight: "bold", fill: rgb("#64748b"))[ADJUSTMENT],
            text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: accent-color)[#rows.at(2).at(0)],
            text(font: hde-font-sans, size: 7pt)[#rows.at(2).at(1)],
            text(font: hde-font-mono, size: 7pt, weight: "bold", fill: accent-color)[#rows.at(2).at(3)],
            text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: accent-color)[#rows.at(3).at(0)],
            text(font: hde-font-sans, size: 7pt)[#rows.at(3).at(1)],
            text(font: hde-font-mono, size: 7pt, weight: "bold", fill: accent-color)[#rows.at(3).at(3)],
          )
          #v(3.5pt)
          #rect(width: 100%, fill: rgb("#ecfdf5"), radius: 3pt, inset: (x: 6pt, y: 4pt))[
            #grid(
              columns: (1fr, auto),
              text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: accent-color)[#metrics.at(3).at(0)],
              text(font: hde-font-mono, size: 8pt, weight: "bold", fill: accent-color)[#metrics.at(3).at(1)]
            )
          ]
        ]
      )
    )
    v(2.5pt)
    hde-comparative-bar-chart(
      title: "BILATERAL RECONCILIATION & NET SETTLEMENT WATERFALL",
      rows: chart-rows,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    v(2pt)
    hde-milestone-timeline(
      title: "SETTLEMENT & DISBURSEMENT AUDIT TRAIL",
      steps: milestones,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    v(2pt)
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
      inset: (x: 10pt, y: 5pt),
      grid(
        columns: (1.5fr, 1fr),
        gutter: 8pt,
        align: (left + horizon, right + horizon),
        text(font: hde-font-mono, size: 8.2pt, weight: "bold", fill: white)[
          OFFICIAL CODED FORM • #upper(title)
        ],
        text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: rgb("#ffe500"))[
          FORM ID: #doc-ref · #citation
        ]
      )
    )
    table(
      columns: (1.2fr, 1.2fr, 1fr, 0.9fr),
      stroke: 0.6pt + rgb("#0f172a"),
      inset: (x: 7pt, y: 4.5pt),
      fill: (x, y) => if y == 0 or y == 2 { rgb("#f8fafc") } else { white },
      [
        #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: primary-color)[BOX 01 · ISSUING PROVIDER / CARRIER / LAB] \
        #v(1pt)
        #text(font: hde-font-sans, size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[#org-name]
      ],
      [
        #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: primary-color)[BOX 02 · PATIENT / CONSIGNEE / SUBJECT] \
        #v(1pt)
        #text(font: hde-font-sans, size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[#counterparty]
      ],
      [
        #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: primary-color)[BOX 03 · CLASSIFICATION] \
        #v(1pt)
        #text(font: hde-font-mono, size: 7.4pt, weight: "bold", fill: accent-color)[#vertical]
      ],
      [
        #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: primary-color)[BOX 04 · FILING DATE] \
        #v(1pt)
        #text(font: hde-font-mono, size: 7.6pt, weight: "bold", fill: rgb("#0f172a"))[#date-str]
      ],
      ..metrics.enumerate().map(((idx, m)) => [
        #text(font: hde-font-mono, size: 5.8pt, weight: "bold", fill: rgb("#475569"))[BOX 0#(idx + 5) · #upper(m.at(0))] \
        #v(1pt)
        #text(font: hde-font-mono, size: 8.6pt, weight: "bold", fill: if idx == 3 { accent-color } else { rgb("#0f172a") })[#m.at(1)]
      ])
    )
    v(2.5pt)
    hde-schedule-table-variation(
      variant: "dark_header",
      headers: ("LINE / CODE", "CODED PROCEDURE, TARIFF OR CLINICAL DESCRIPTION", "UNITS / POINTER", "CHARGE / VALUE"),
      rows: rows,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    v(2pt)
    svg-architectural-diagram(
      kind: diagram-kind,
      title: "BOX 09 · VERIFIED TELEMETRY & WORKFLOW DIAGRAM",
      primary-hex: primary-hex,
      accent-hex: accent-hex,
      metric-a: metrics.at(0).at(1),
      metric-b: metrics.at(3).at(1),
      hero-height: 76pt,
    )
    v(2pt)
    hde-milestone-timeline(
      title: "BOX 10 · CHAIN OF CUSTODY, ROUTING & SETTLEMENT STEPS",
      steps: milestones,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    v(2pt)
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
      inset: 3.5pt,
      rect(
        width: 100%,
        stroke: 0.75pt + accent-color,
        fill: rgb("#fffdf9"),
        inset: (x: 16pt, y: 12pt),
        [
          #grid(
            columns: (1fr, auto, 1fr),
            align: (left + top, center + horizon, right + top),
            [
              #text(font: hde-font-mono, size: 6.5pt, weight: "bold", fill: rgb("#475569"))[
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
                inset: (x: 7pt, y: 4pt),
                align(right)[
                  #text(font: hde-font-mono, size: 6pt, weight: "bold", fill: primary-color)[OFFICIAL RECORDING STAMP ZONE] \
                  #text(font: hde-font-mono, size: 6pt, fill: rgb("#64748b"))[#citation]
                ]
              )
            ]
          )
          #v(6pt)
          #align(center)[
            #text(font: hde-font-mono, size: 7pt, weight: "bold", tracking: 1.5pt, fill: accent-color)[
              #upper(vertical) • #upper(org-name)
            ] \
            #v(3pt)
            #text(font: hde-font-serif, size: 15pt, weight: "bold", fill: rgb("#0f172a"))[#upper(title)] \
            #v(3pt)
            #line(length: 55%, stroke: 1pt + accent-color)
          ]
          #v(8pt)
          #text(font: hde-font-serif, size: 8.6pt, fill: rgb("#1e293b"))[
            *KNOW ALL PERSONS BY THESE PRESENTS*, that *#org-name* (#profession), acting pursuant to statutory and regulatory authority under *#citation*, does hereby execute, certify, and deliver this official instrument unto *#counterparty*, subject to the verified schedules and covenants set forth below:
          ]
          #v(6pt)
          #hde-kpi-strip-variation(
            variant: "metric_cards",
            metrics: metrics,
            primary-color: primary-color,
            accent-color: accent-color,
          )
          #v(4pt)
          #hde-schedule-table-variation(
            variant: "minimal_lines",
            headers: ("CLAUSE / REF", "CERTIFIED COVENANT, GRANT OR SCHEDULE ITEM", "STATUTORY BASIS", "VERIFIED AMOUNT"),
            rows: rows,
            primary-color: primary-color,
            accent-color: accent-color,
          )
          #v(6pt)
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
      inset: (x: 12pt, y: 9pt),
      [
        #grid(
          columns: (if seal-node != none { 52pt } else { 0pt }, 1fr, auto),
          gutter: 10pt,
          align: (center + horizon, left + horizon, right + horizon),
          [#if seal-node != none { box(fill: white, radius: 5pt, inset: 2pt, seal-node) }],
          [
            #hde-sticker-badge(label: upper(vertical), angle: -2deg, bg: rgb("#ffd000"), fg: rgb("#0f172a"))
            #h(8pt)
            #text(font: hde-font-mono, size: 7pt, weight: "bold", fill: rgb("#38bdf8"))[#doc-ref · #citation]
            #v(4pt)
            #text(font: hde-font-sans, size: 13pt, weight: "black", fill: white)[#title] \
            #v(1.5pt)
            #text(font: hde-font-sans, size: 7.5pt, fill: rgb("#cbd5e1"))[#org-name → #counterparty]
          ],
          svg-rubber-stamp(label: stamp-label, sublabel: stamp-sublabel, color-hex: "#38bdf8", angle: -4deg, width-in: 1.38in)
        )
      ]
    )
    v(7pt)
    grid(
      columns: (1fr,) * metrics.len(),
      gutter: 8pt,
      ..metrics.enumerate().map(((idx, m)) => hde-depth-card(
        accent-border: if idx == 3 { rgb("#10b981") } else { rgb("#254b7c") },
        fill-color: rgb("#0f172a"),
        shadow-color: if idx == 3 { rgb("#10b981") } else { primary-color },
        inset-val: (x: 8pt, y: 6pt),
        [
          #text(font: hde-font-mono, size: 6.2pt, weight: "bold", fill: rgb("#94a3b8"))[#upper(m.at(0))] \
          #v(2pt)
          #text(font: hde-font-mono, size: 9.6pt, weight: "black", fill: if idx == 3 { rgb("#34d399") } else { rgb("#ffe500") })[#m.at(1)]
        ]
      ))
    )
    v(7pt)
    hde-schedule-table-variation(
      variant: "dark_header",
      rows: rows,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    v(3pt)
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
        hero-height: 76pt,
      ),
      hde-comparative-bar-chart(
        title: "PERFORMANCE & ALLOCATION BENCHMARK",
        rows: chart-rows,
        primary-color: primary-color,
        accent-color: accent-color,
      )
    )
    v(3pt)
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
    v(2.5pt)
    hde-kpi-strip-variation(
      variant: kpi-variant,
      metrics: metrics,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    v(2pt)
    hde-schedule-table-variation(
      variant: table-variant,
      rows: rows,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    v(2pt)
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
        hero-height: 76pt,
      ),
      hde-comparative-bar-chart(
        title: "FINANCIAL & VALUATION BREAKDOWN",
        rows: chart-rows,
        primary-color: primary-color,
        accent-color: accent-color,
      ),
    )
    v(2pt)
    hde-milestone-timeline(
      title: "EXECUTION MILESTONES, STATUTORY GATES & VERIFICATION TIMELINE",
      steps: milestones,
      primary-color: primary-color,
      accent-color: accent-color,
    )
    v(2pt)
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
