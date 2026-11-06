//! 50 Nano Banana Profession & Archetype Concepts + Multi-Style Element Showcase Engine
//!
//! Author: Daniel Hallman
//!
//! Spans 10 core profession verticals (5 concepts per vertical = 50 total concepts)
//! backed by `last30days` community discovery (`r/smallbusiness`, `r/sweatystartup`,
//! `r/Contractors`, `r/Landlord`, `r/CommercialRealEstate`, `r/typst`, Hacker News,
//! and Typst Universe packages).

use serde::Deserialize;
use serde::Serialize;

use crate::escape_typst_string_literal;
use crate::escape_typst_text;
use crate::graphics::HALLMAN_EMBEDDED_GRAPHICS_TYP;
use crate::ReadabilityPreset;

/// A single profession/job document concept in the 50-Concept Hallman Document Engine catalog.
#[derive(Debug, Clone, Serialize)]
pub struct ArchetypeConceptSpec {
    /// 1-indexed concept number (`1..=50`).
    pub id: u8,
    /// URL-safe slug for dedicated SEO landing pages (`/documents/:slug` and `/concepts/:slug`).
    pub slug: &'static str,
    /// Full display title of the document archetype.
    pub title: &'static str,
    /// Profession vertical category (1 of 10 verticals).
    pub vertical: &'static str,
    /// Vertical slug (`construction-trades`, `real-estate-leasing`, etc.).
    pub vertical_slug: &'static str,
    /// Target job role / profession persona.
    pub target_profession: &'static str,
    /// High-intent SEO search keyword targeted by this concept page.
    pub primary_keyword: &'static str,
    /// Secondary long-tail search keywords.
    pub secondary_keywords: &'static [&'static str],
    /// Real community pain point / search demand signal discovered via `last30days`.
    pub demand_signal: &'static str,
    /// Statutory citation, industry form code, or regulatory standard.
    pub statutory_citation: &'static str,
    /// Default masthead variation (`executive_split`, `neo_brutalist`, `swiss_grid`, `judicial_caption`, `trade_banner`, `editorial_serif`).
    pub default_masthead: &'static str,
    /// Default KPI strip variation (`metric_cards`, `dark_terminal`, `neo_brutalist`).
    pub default_kpi_style: &'static str,
    /// Default schedule table variation (`striped`, `dark_header`, `minimal_lines`).
    pub default_table_style: &'static str,
    /// Default embedded SVG diagram kind (`parcel_lot`, `roof_pitch`, `hvac_diagnostic`, `solar_array`, `vehicle_inspection`, `floorplan_spec`).
    pub default_diagram_kind: &'static str,
    /// Default readability preset (`standard`, `high_legibility`, `executive_compact`, `judicial_formal`).
    pub default_readability: ReadabilityPreset,
    /// Default execution mode (`wet-ink`, `esign`, `notary_jurat`).
    pub default_execution_mode: &'static str,
    /// Whether notary jurat block is enabled by default.
    pub notary_required: bool,
    /// Embedded Nano Banana certification seal filename (`seal_trades_construction.png`, etc.).
    pub seal_filename: &'static str,
    /// Primary brand hex color (`#1e3a8a`, etc.).
    pub primary_hex: &'static str,
    /// Accent hex color (`#d97706`, `#059669`, etc.).
    pub accent_hex: &'static str,
    /// Rubber stamp main text (`PAID IN FULL`, `LIEN WAIVER`, `FILED PRO SE`, etc.).
    pub stamp_label: &'static str,
    /// Rubber stamp sublabel (`VERIFIED RECORD`, `STATUTORY RELEASE`, etc.).
    pub stamp_sublabel: &'static str,
    /// Sample issuing organization name.
    pub sample_org: &'static str,
    /// Sample counterparty / client / jurisdiction name.
    pub sample_counterparty: &'static str,
    /// 4 KPI metric cards `(label, value)`.
    pub kpi_metrics: [(&'static str, &'static str); 4],
    /// 4 schedule line items `(code, description, basis_qty, amount)`.
    pub line_items: [(&'static str, &'static str, &'static str, &'static str); 4],
    /// 3 comparative bar chart rows `(label, formatted_val, percentage_0_to_100, style_tag)`.
    pub chart_rows: [(&'static str, &'static str, u8, &'static str); 3],
    /// 4 execution milestone steps `(phase_title, detail, date_or_trigger)`.
    pub milestones: [(&'static str, &'static str, &'static str); 4],
}

/// Live interactive override parameters for customizing any of the 50 concepts in the browser.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomRenderOptions {
    pub masthead: Option<String>,
    pub kpi: Option<String>,
    pub table: Option<String>,
    pub diagram: Option<String>,
    pub preset: Option<String>,
    pub execution: Option<String>,
    pub stamp: Option<String>,
    pub primary_hex: Option<String>,
    pub accent_hex: Option<String>,
    pub custom_title: Option<String>,
    pub custom_org: Option<String>,
    pub custom_counterparty: Option<String>,
}

/// Complete catalog of 50 Nano Banana Profession & Archetype Concepts across 10 verticals.
pub const ARCHETYPE_CONCEPTS_50: &[ArchetypeConceptSpec] = &[
    // =========================================================================
    // VERTICAL 1: CONSTRUCTION, GENERAL CONTRACTING & SPECIALTY TRADES (1..=5)
    // =========================================================================
    ArchetypeConceptSpec {
        id: 1,
        slug: "aia-g702-g703-progress-billing-draw-schedule",
        title: "AIA G702/G703 Style Progress Billing & Schedule of Values",
        vertical: "Construction & Specialty Trades",
        vertical_slug: "construction-trades",
        target_profession: "General Contractors, Commercial Builders & Construction Managers",
        primary_keyword: "aia g702 g703 progress billing schedule of values template pdf",
        secondary_keywords: &["construction draw request template", "retainage billing invoice", "schedule of values pdf"],
        demand_signal: "r/Contractors & search demand for progress draw schedules with automatic 10% retainage tracking and stored materials verification.",
        statutory_citation: "AIA Document G702/G703 Standard · T.C.A. § 66-34-104 (Prompt Pay & Retainage Escrow)",
        default_masthead: "trade_banner",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: true,
        seal_filename: "seal_trades_construction.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#d97706",
        stamp_label: "DRAW #04 APPROVED",
        stamp_sublabel: "RETAINAGE HELD: 5%",
        sample_org: "Lookout Valley Commercial Builders LLC",
        sample_counterparty: "Riverfront Mixed-Use DevelopmentSPE LLC",
        kpi_metrics: [
            ("ORIGINAL CONTRACT SUM", "$1,480,000.00"),
            ("COMPLETED & STORED TO DATE", "$962,000.00 (65%)"),
            ("RETAINAGE HELD (5.0%)", "$48,100.00"),
            ("CURRENT PAYMENT DUE", "$184,350.00"),
        ],
        line_items: [
            ("DIV-03", "Structural Concrete Foundations, Footings & Slab-on-Grade", "100% Complete", "$245,000.00"),
            ("DIV-05", "Structural Steel Framing, Decking & Moment Connections", "85% Complete", "$318,750.00"),
            ("DIV-23", "Commercial Rooftop VRF HVAC & Sheet Metal Ductwork", "60% Complete", "$234,000.00"),
            ("DIV-26", "480V 3-Phase Switchgear, Conduit & LED Lighting Package", "45% Complete", "$164,250.00"),
        ],
        chart_rows: [
            ("Prior Draws Certified (#1–#3)", "$729,550.00", 76, "primary"),
            ("Current Draw #4 Work in Place", "$184,350.00", 24, "accent"),
            ("Escrowed Retainage Balance (5%)", "$48,100.00", 12, "primary"),
        ],
        milestones: [
            ("Draw #1: Site & Subgrade", "Mobilization, excavation, utilities", "PAID 08/15"),
            ("Draw #2: Core & Shell", "Slab, structural steel erection", "PAID 09/15"),
            ("Draw #3: Dry-In & Rough", "Roofing membrane, MEP rough-in", "PAID 10/15"),
            ("Draw #4: Interior Build", "Drywall, switchgear, VRF trim", "DUE 11/15"),
        ],
    },
    ArchetypeConceptSpec {
        id: 2,
        slug: "construction-change-order-cost-impact-directive",
        title: "Construction Change Order & Schedule Impact Directive",
        vertical: "Construction & Specialty Trades",
        vertical_slug: "construction-trades",
        target_profession: "Remodelers, Custom Home Builders & Trade Subcontractors",
        primary_keyword: "construction change order template pdf with schedule impact",
        secondary_keywords: &["contractor scope change form", "signed change order form", "time and materials markup sheet"],
        demand_signal: "r/Contractors & r/sweatystartup top pain point: preventing unpaid scope creep by requiring signed cost + calendar day impact approval before work.",
        statutory_citation: "AIA G701 Standard · Statutory Written Change Directive · OH&P Markup Disclosure",
        default_masthead: "neo_brutalist",
        default_kpi_style: "neo_brutalist",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_trades_construction.png",
        primary_hex: "#0f172a",
        accent_hex: "#d97706",
        stamp_label: "SCOPE AUTHORIZED",
        stamp_sublabel: "+6 CALENDAR DAYS",
        sample_org: "Ridgecrest Structural Remodeling Inc.",
        sample_counterparty: "Marcus & Elena Vance (Owner-Occupants)",
        kpi_metrics: [
            ("PRIOR CONTRACT TOTAL", "$142,500.00"),
            ("NET CHANGE ORDER #03", "+$14,820.00"),
            ("SCHEDULE EXTENSION", "+6 Working Days"),
            ("NEW REVISED CONTRACT SUM", "$157,320.00"),
        ],
        line_items: [
            ("CO-03.1", "Engineered LVL Flush Beam Upgrade (2x 1.75\" x 14\" Microllam + Steel Flitch)", "24 LF Installed", "$5,840.00"),
            ("CO-03.2", "Relocate 200A Sub-Panel & 4 Home-Run AFCI Circuits in Load-Bearing Wall", "16 Hrs Master Elec", "$3,920.00"),
            ("CO-03.3", "Custom White Oak Waterfall Island Countertop & Concealed Steel Brackets", "42 Sq Ft Fab", "$3,600.00"),
            ("CO-03.4", "General Conditions, Permit Amendment & 10% Overhead / 10% Profit", "Statutory OH&P", "$1,460.00"),
        ],
        chart_rows: [
            ("Original Base Contract Scope", "$142,500.00", 86, "primary"),
            ("Prior Approved Changes (#1–#2)", "$6,400.00", 14, "primary"),
            ("Change Order #03 Net Addition", "$14,820.00", 22, "accent"),
        ],
        milestones: [
            ("Owner Written Sign-Off", "50% deposit due upon execution", "IMMEDIATE"),
            ("Structural Engineer Stamp", "LVL beam calc submittal to City", "DAY 2"),
            ("Rough Framing & Elec", "Beam pocket & panel relocation", "DAY 4"),
            ("Municipal Framing Pass", "City inspector sign-off", "DAY 6"),
        ],
    },
    ArchetypeConceptSpec {
        id: 3,
        slug: "statutory-mechanics-lien-waiver-release-4-mode",
        title: "4-Mode Statutory Mechanics' Lien Waiver & Release",
        vertical: "Construction & Specialty Trades",
        vertical_slug: "construction-trades",
        target_profession: "Subcontractors, Material Suppliers, General Contractors & Title Agents",
        primary_keyword: "conditional and unconditional mechanics lien waiver form pdf",
        secondary_keywords: &["progress payment lien waiver", "final payment lien release form", "subcontractor lien waiver template"],
        demand_signal: "r/legal viral thread (5,702 pts, 1,108 comments): homeowners & GCs demanding clear statutory separation between unsigned estimates and conditional/unconditional lien waivers.",
        statutory_citation: "T.C.A. § 66-11-101 et seq. · Cal. Civ. Code § 8132–8138 · Tex. Prop. Code § 53.284",
        default_masthead: "judicial_caption",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "parcel_lot",
        default_readability: ReadabilityPreset::JudicialFormal,
        default_execution_mode: "wet-ink",
        notary_required: true,
        seal_filename: "seal_trades_construction.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "CONDITIONAL WAIVER",
        stamp_sublabel: "UPON GOOD FUNDS",
        sample_org: "Chattanooga Ready-Mix & Structural Supply Co.",
        sample_counterparty: "First American Title Insurance Co. / Owner",
        kpi_metrics: [
            ("WAIVER CLASSIFICATION", "CONDITIONAL PROGRESS"),
            ("THROUGH BILLING DATE", "October 31, 2026"),
            ("CHECK / ACH AMOUNT", "$46,850.00"),
            ("DISPUTED / RETENTION CARVEOUT", "$5,205.55"),
        ],
        line_items: [
            ("INV-8841", "4,000 PSI Fiber-Reinforced Concrete Mix (320 Cubic Yards Delivered)", "Tickets #401–419", "$41,600.00"),
            ("INV-8892", " #5 Epoxy-Coated Reinforcing Steel Bar & Anchor Bolt Package", "14.2 Tons", "$5,250.00"),
            ("RET-HOLD", "Statutory Contractual Retainage Expressly Reserved from Release", "10% Retainage", "$5,205.55"),
            ("EX-CLAIM", "Pending Written Change Directive #02 (Unresolved Extra Work)", "Carveout", "$0.00"),
        ],
        chart_rows: [
            ("Total Cumulative Billed to Date", "$148,500.00", 100, "primary"),
            ("Previously Released via Paid Draws", "$96,444.45", 65, "primary"),
            ("Current Conditional Release Amount", "$46,850.00", 32, "accent"),
        ],
        milestones: [
            ("Waiver Executed by Sub", "Delivered with Draw #3 Invoice", "STEP 1"),
            ("GC & Title Audit", "Matches Schedule of Values", "STEP 2"),
            ("ACH / Wire Clearing", "Good funds verified in bank", "STEP 3"),
            ("Statutory Lien Extinguished", "Effective through 10/31/2026", "STEP 4"),
        ],
    },
    ArchetypeConceptSpec {
        id: 4,
        slug: "hvac-plumbing-electrical-field-diagnostic-work-order",
        title: "HVAC, Plumbing & Electrical Field Service Diagnostic Work Order",
        vertical: "Construction & Specialty Trades",
        vertical_slug: "construction-trades",
        target_profession: "HVAC Technicians, Master Plumbers & Electrical Service Contractors",
        primary_keyword: "hvac field service work order and flat rate invoice template",
        secondary_keywords: &["plumbing diagnostic work order pdf", "good better best repair estimate", "epa 608 refrigerant log"],
        demand_signal: "r/sweatystartup & ServiceTitan/Housecall Pro search demand for Good-Better-Best flat-rate repair options paired with EPA 608 refrigerant superheat/subcooling telemetry.",
        statutory_citation: "EPA Clean Air Act § 608 Refrigerant Log · ACCA Manual S/J · Pre-Work Authorization",
        default_masthead: "trade_banner",
        default_kpi_style: "dark_terminal",
        default_table_style: "striped",
        default_diagram_kind: "hvac_diagnostic",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_trades_construction.png",
        primary_hex: "#0f172a",
        accent_hex: "#059669",
        stamp_label: "SYSTEM CERTIFIED",
        stamp_sublabel: "DELTA-T: 19.6°F PASS",
        sample_org: "Signal Mountain Climate & Mechanical Services",
        sample_counterparty: "Dr. Robert & Sarah Lindqvist (Residence)",
        kpi_metrics: [
            ("EQUIPMENT SERIAL", "TRANE 4TWX8048A1000"),
            ("SUPPLY / RETURN DELTA-T", "19.6°F (54.6° / 74.2°)"),
            ("SUBCOOLING / STATIC", "11.4°F / 0.36 IWC"),
            ("APPROVED REPAIR TOTAL", "$1,185.00 (PLATINUM)"),
        ],
        line_items: [
            ("DIAG-01", "Comprehensive 21-Point Heat Pump Telemetry, Megohmmeter & Static Pressure Test", "Flat Diagnostic", "$129.00"),
            ("REP-204", "OEM Dual Run Capacitor (45/5 MFD 440V) + Hard-Start Potential Relay Kit", "Parts + 3Yr Warr", "$386.00"),
            ("REF-410", "EPA § 608 Leak Search, Schrader Core Replacement & 2.5 lbs Virgin R-410A", "Subcool to 11.4°F", "$495.00"),
            ("MEM-01", "Annual Bi-Season Precision Tune-Up Membership (15% Repair Discount Applied)", "2 Visits / Yr", "$175.00"),
        ],
        chart_rows: [
            ("Option A: Minimum Essential Repair", "$515.00", 43, "primary"),
            ("Option B: Recommended System Restore", "$1,185.00", 82, "accent"),
            ("Option C: Full Condenser Coil Overhaul", "$2,490.00", 100, "primary"),
        ],
        milestones: [
            ("Pre-Work Sign-Off", "Homeowner selects Option B", "09:15 AM"),
            ("Lockout / Tagout", "240V disconnect verified 0V", "09:25 AM"),
            ("Component & Charge", "Capacitor + R-410A subcool", "10:10 AM"),
            ("Final Telemetry Pass", "Delta-T 19.6°F verified", "10:45 AM"),
        ],
    },
    ArchetypeConceptSpec {
        id: 5,
        slug: "roofing-storm-damage-insurance-supplement-xactimate",
        title: "Roofing Storm Damage Inspection & Insurance Supplement Packet",
        vertical: "Construction & Specialty Trades",
        vertical_slug: "construction-trades",
        target_profession: "Roofing Contractors, Public Adjusters & Storm Restoration Specialists",
        primary_keyword: "roofing insurance supplement estimate template xactimate style",
        secondary_keywords: &["hail damage roof inspection report pdf", "irc r905.2.8.5 drip edge code supplement", "actual cash value vs rcv worksheet"],
        demand_signal: "High search volume among storm restoration roofers for code-upgrade supplement sheets citing IRC R905.2.8.2 (ice barrier) and R905.2.8.5 (drip edge) to recover omitted carrier line items.",
        statutory_citation: "2021 IRC § R905.2.8.2 (Ice Barrier) · § R905.2.8.5 (Drip Edge) · NOAA Hail Swath Verification",
        default_masthead: "executive_split",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "roof_pitch",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_trades_construction.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#dc2626",
        stamp_label: "CODE SUPPLEMENT",
        stamp_sublabel: "IRC R905 MANDATORY",
        sample_org: "Tennessee Valley Storm Restoration & Roofing LLC",
        sample_counterparty: "State Farm Fire & Casualty — Claim #42-8819-X4",
        kpi_metrics: [
            ("CARRIER INITIAL RCV", "$16,420.00"),
            ("CODE SUPPLEMENT ADDITION", "+$6,890.00"),
            ("REVISED REPLACEMENT RCV", "$23,310.00"),
            ("RECOVERABLE DEPRECIATION", "$5,840.00 RELEASE"),
        ],
        line_items: [
            ("RFG-300S", "Remove & Replace Architectural Laminate Shingles (34.2 Squares incl. 12% Steep Waste)", "38.33 SQ @ $395", "$15,140.35"),
            ("RFG-DRIP", "IRC § R905.2.8.5 Code Upgrade: Pre-Painted Aluminum Drip Edge at Eaves & Rakes", "318 LF @ $4.85", "$1,542.30"),
            ("RFG-IWS", "IRC § R905.2.8.2 Code Upgrade: Self-Adhering Ice & Water Barrier in Valleys/Eaves", "840 SF @ $3.10", "$2,604.00"),
            ("O&P-20", "General Contractor Overhead (10%) & Profit (10%) on Multi-Trade Restoration", "10% + 10% Standard", "$4,023.35"),
        ],
        chart_rows: [
            ("Original Carrier Adjuster Scope", "$16,420.00", 70, "primary"),
            ("IRC Building Code Omissions Added", "$4,146.30", 28, "accent"),
            ("Revised Total Certified Claim RCV", "$23,310.00", 100, "accent"),
        ],
        milestones: [
            ("HAAG Hail Inspection", "11+ hits/square on S/W slopes", "VERIFIED"),
            ("Code Supplement Filed", "IRC R905 + EagleView attached", "SUBMITTED"),
            ("Desk Adjuster Review", "Updated Xactimate loss sheet", "APPROVED"),
            ("Certificate of Completion", "Releases $5,840 depreciation", "FINAL DRAW"),
        ],
    },

    // =========================================================================
    // VERTICAL 2: REAL ESTATE, LEASING & PROPERTY MANAGEMENT (6..=10)
    // =========================================================================
    ArchetypeConceptSpec {
        id: 6,
        slug: "residential-lease-agreement-state-statutory-addenda",
        title: "50-State Statutory Residential Lease Agreement & Deposit Schedule",
        vertical: "Real Estate & Property Management",
        vertical_slug: "real-estate-leasing",
        target_profession: "Independent Landlords, Property Managers & Real Estate Investors",
        primary_keyword: "state compliant residential lease agreement template pdf",
        secondary_keywords: &["urlta residential lease pdf", "security deposit escrow disclosure", "late fee grace period clause"],
        demand_signal: "r/Landlord & r/PropertyManagement constant demand for state-specific URLTA leases with statutory security deposit bank disclosure and clear maintenance responsibility matrices.",
        statutory_citation: "URLTA · T.C.A. § 66-28-301 (Security Deposit Escrow) · 42 U.S.C. § 4852d (Lead-Based Paint)",
        default_masthead: "editorial_serif",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::HighLegibility,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_real_estate_title.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "URLTA COMPLIANT",
        stamp_sublabel: "ESCROW VERIFIED",
        sample_org: "Northshore Residential Holdings LLC",
        sample_counterparty: "Jordan T. Miller & Claire E. Miller (Tenants)",
        kpi_metrics: [
            ("MONTHLY BASE RENT", "$2,350.00 / MO"),
            ("SECURITY DEPOSIT ESCROW", "$2,350.00 (FIRST HORIZON)"),
            ("LEASE TERM DURATION", "12 Months (12/01–11/30)"),
            ("STATUTORY GRACE PERIOD", "5 Days (T.C.A. § 66-28-201)"),
        ],
        line_items: [
            ("SEC-01", "First Month Base Rent (Prorated 0 Days — Full Calendar Month Possession)", "Due at Signing", "$2,350.00"),
            ("SEC-02", "Refundable Security Deposit Held in Separate Trust Account #XXXXXX4819", "Statutory Escrow", "$2,350.00"),
            ("ADD-PET", "Non-Refundable Pet Sanitation & Carpet Remediation Fee (1 Dog < 45 lbs)", "One-Time Addendum", "$300.00"),
            ("LATE-10", "Statutory Late Charge Cap: Maximum 10% of Past-Due Rent After 5th Day", "Contingent Cap", "$235.00 MAX"),
        ],
        chart_rows: [
            ("Total Move-In Funds Required", "$5,000.00", 100, "primary"),
            ("Refundable Tenant Trust Deposit", "$2,350.00", 47, "accent"),
            ("Annualized Gross Lease Value", "$28,200.00", 92, "primary"),
        ],
        milestones: [
            ("Lease & Lead Execution", "Signed by all adult occupants", "11/15/2026"),
            ("Move-In Funds Cleared", "Certified funds / ACH settled", "11/28/2026"),
            ("Joint Walk-Through", "Condition checklist signed", "12/01/2026"),
            ("Renewal Notice Window", "60 days prior to expiration", "09/30/2027"),
        ],
    },
    ArchetypeConceptSpec {
        id: 7,
        slug: "commercial-triple-net-nnn-lease-cam-reconciliation",
        title: "Commercial Triple-Net (NNN) Lease & Annual CAM Reconciliation Statement",
        vertical: "Real Estate & Property Management",
        vertical_slug: "real-estate-leasing",
        target_profession: "Commercial Property Managers, Retail Strip Owners & CRE Asset Managers",
        primary_keyword: "commercial nnn lease cam reconciliation statement template",
        secondary_keywords: &["common area maintenance reconciliation spreadsheet pdf", "pro rata share cam audit", "controllable cam cap calculation"],
        demand_signal: "r/CommercialRealEstate discussions on transparent pro-rata RSF share calculations, controllable CAM expense caps (5% cumulative), and property tax pass-through audits.",
        statutory_citation: "BOMA Z65.1 Retail/Office RSF Standard · 5% Controllable CAM Cap · Pro-Rata Pass-Through",
        default_masthead: "swiss_grid",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::ExecutiveCompact,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_real_estate_title.png",
        primary_hex: "#0f172a",
        accent_hex: "#0284c7",
        stamp_label: "CAM RECONCILED",
        stamp_sublabel: "BOMA RSF VERIFIED",
        sample_org: "Hamilton Place Retail Partners LP",
        sample_counterparty: "Apex Physical Therapy & Sports Medicine PLLC (Suite 104)",
        kpi_metrics: [
            ("PREMISES RENTABLE AREA", "3,610 RSF (14.44% SHARE)"),
            ("ACTUAL 2026 NNN / RSF", "$9.42 / RSF ($34,006)"),
            ("ESTIMATED NNN PAID", "$31,768.00 ($8.80/RSF)"),
            ("NET TRUE-UP BALANCE DUE", "$2,238.20"),
        ],
        line_items: [
            ("CAM-TAX", "Ad Valorem Real Estate Taxes (Hamilton County + City of Chattanooga Certified Bill)", "14.44% of $118,400", "$17,096.96"),
            ("CAM-INS", "Commercial Property All-Risk & General Liability Insurance Blanket Policy", "14.44% of $38,200", "$5,516.08"),
            ("CAM-OPS", "Parking Lot Sweeping, LED Lighting, Landscaping, Snow & Exterior Maintenance", "14.44% of $54,900", "$7,927.56"),
            ("CAM-MGT", "Administrative Property Management Fee (Capped at 4.0% of Gross Revenues)", "Capped Per Lease § 7.4", "$3,465.60"),
        ],
        chart_rows: [
            ("Ad Valorem Property Tax Pass-Through", "$17,096.96", 50, "primary"),
            ("Common Area Maintenance & Mgmt", "$11,393.16", 34, "accent"),
            ("Property & Liability Insurance Share", "$5,516.08", 16, "primary"),
        ],
        milestones: [
            ("General Ledger Close", "CPA audit of 2026 invoices", "01/31/2027"),
            ("CAM Statement Issued", "90-day tenant audit window", "02/15/2027"),
            ("True-Up Settlement", "Added to March rent invoice", "03/01/2027"),
            ("New Monthly NNN Escrow", "Adjusted to $2,833.85/mo", "03/01/2027"),
        ],
    },
    ArchetypeConceptSpec {
        id: 8,
        slug: "move-in-move-out-property-condition-security-deposit-itemization",
        title: "Move-In / Move-Out Condition Checklist & Security Deposit Itemization",
        vertical: "Real Estate & Property Management",
        vertical_slug: "real-estate-leasing",
        target_profession: "Residential Property Managers, Multifamily Operators & Student Housing",
        primary_keyword: "security deposit deduction itemization letter and move out checklist pdf",
        secondary_keywords: &["normal wear and tear vs damage schedule", "hud carpet depreciation life expectancy", "move in inspection form"],
        demand_signal: "r/Landlord & r/Tenant high-volume disputes over security deposit deductions, requiring HUD useful-life depreciation schedules (5-yr carpet, 3-yr paint) and side-by-side condition ratings.",
        statutory_citation: "T.C.A. § 66-28-301(b) · HUD Useful-Life Depreciation Standard · 30-Day Refund Accounting",
        default_masthead: "executive_split",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_real_estate_title.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "REFUND ISSUED",
        stamp_sublabel: "WITHIN 30 DAYS",
        sample_org: "Southside Lofts Property Management LLC",
        sample_counterparty: "Devon K. Brooks (Former Tenant — Unit 308)",
        kpi_metrics: [
            ("ESCROWED DEPOSIT HELD", "$1,950.00"),
            ("WEAR & TEAR DEDUCTION", "$0.00 (STATUTORY)"),
            ("DEPRECIATED DAMAGE CHARGE", "-$340.00"),
            ("NET REFUND CHECK ISSUED", "$1,610.00"),
        ],
        line_items: [
            ("INSP-LVG", "Living Room & Hallway Walls: Minor Nail Holes & Scuffs (36 Months Occupancy)", "Normal Wear & Tear", "$0.00"),
            ("INSP-BR1", "Primary Bedroom Plush Carpet: Pet Urine Stain Requiring Subfloor Seal (3/5 Yr Remaining)", "60% of $350 Invoice", "$210.00"),
            ("INSP-KIT", "Kitchen Quartz Countertop & Disposer: Foreign Metal Object Jam Extraction", "Vendor Inv #9104", "$130.00"),
            ("REF-CHK", "Net Security Deposit Refund Disbursed via Check #10842 to Forwarding Address", "Mailed Day 14", "$1,610.00"),
        ],
        chart_rows: [
            ("Original Tenant Security Deposit", "$1,950.00", 100, "primary"),
            ("Net Refund Returned to Tenant", "$1,610.00", 83, "accent"),
            ("Prorated Repair Deductions (17%)", "$340.00", 17, "primary"),
        ],
        milestones: [
            ("Keys Surrendered", "Possession returned to office", "10/31/2026"),
            ("Joint Inspection", "34 time-stamped photos logged", "11/01/2026"),
            ("Vendor Invoices Paid", "HUD depreciation applied", "11/08/2026"),
            ("Refund & Accounting", "Mailed via Certified Mail", "11/12/2026"),
        ],
    },
    ArchetypeConceptSpec {
        id: 9,
        slug: "short-term-rental-airbnb-vrbo-management-owner-payout-statement",
        title: "Short-Term Rental (Airbnb/VRBO) Co-Host Agreement & Owner Payout Statement",
        vertical: "Real Estate & Property Management",
        vertical_slug: "real-estate-leasing",
        target_profession: "STR Property Managers, Airbnb Co-Hosts & Vacation Rental Operators",
        primary_keyword: "short term rental property management owner statement template",
        secondary_keywords: &["airbnb co host agreement pdf", "vacation rental monthly owner payout report", "occupancy tax pass through statement"],
        demand_signal: "Growing demand from boutique STR managers for unified owner statements separating gross nightly revenue, cleaning pass-throughs, OTA channel fees, and local occupancy taxes.",
        statutory_citation: "Municipal Hotel-Motel Occupancy Tax Remittance · Trust Accounting Reconciliation",
        default_masthead: "executive_split",
        default_kpi_style: "dark_terminal",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "esign",
        notary_required: false,
        seal_filename: "seal_real_estate_title.png",
        primary_hex: "#0f172a",
        accent_hex: "#059669",
        stamp_label: "ACH DISBURSED",
        stamp_sublabel: "OWNER TRUST PAYOUT",
        sample_org: "Scenic City Hospitality & Cabin Management LLC",
        sample_counterparty: "Highland Bluff Cabin Holdings LLC (Owner)",
        kpi_metrics: [
            ("OCTOBER OCCUPANCY / ADR", "83.9% (26 Nights @ $315)"),
            ("GROSS NIGHTLY REVENUE", "$8,190.00"),
            ("CO-HOST COMMISSION (18%)", "-$1,474.20"),
            ("NET OWNER ACH PAYOUT", "$6,420.80"),
        ],
        line_items: [
            ("REV-01", "Gross Nightly Accommodation Revenue (18 Nights Airbnb + 8 Nights Direct Booking)", "26 Booked Nights", "$8,190.00"),
            ("CLN-02", "Guest-Paid Turnover Cleaning Pass-Through (9 Turnovers @ $165 Paid to Crew)", "Pass-Through Net $0", "$0.00"),
            ("MGT-18", "Full-Service Co-Host Management Fee (18.0% of Net Nightly Accommodation Revenue)", "18% Contractual", "-$1,474.20"),
            ("MNT-04", "Hot Tub Bromine Shock, Filter Cartridge Replacement & Smart Lock Lithium Pack", "At-Cost Receipts", "-$295.00"),
        ],
        chart_rows: [
            ("Gross Nightly Rental Revenue", "$8,190.00", 100, "primary"),
            ("Net Owner Trust Distribution", "$6,420.80", 78, "accent"),
            ("Management Fee & Supplies (22%)", "$1,769.20", 22, "primary"),
        ],
        milestones: [
            ("Month-End Close", "All October checkouts settled", "10/31/2026"),
            ("Occupancy Tax Filed", "County & City transient tax", "11/05/2026"),
            ("Statement Delivered", "Owner portal + PDF receipt", "11/08/2026"),
            ("ACH Trust Transfer", "Settled to Owner Chase acct", "11/10/2026"),
        ],
    },
    ArchetypeConceptSpec {
        id: 10,
        slug: "alta-settlement-statement-real-estate-closing-sheet",
        title: "ALTA Combined Settlement Statement & Real Estate Closing Balance Sheet",
        vertical: "Real Estate & Property Management",
        vertical_slug: "real-estate-leasing",
        target_profession: "Title Companies, Closing Attorneys, Real Estate Brokers & Wholesalers",
        primary_keyword: "alta combined settlement statement template fillable pdf",
        secondary_keywords: &["real estate closing statement worksheet", "seller net sheet and buyer cash to close", "county property tax proration calculator"],
        demand_signal: "High search demand from real estate investors, cash buyers, and closing attorneys for clean side-by-side Seller/Buyer debit-credit settlement sheets with exact 365-day tax prorations.",
        statutory_citation: "ALTA Combined Settlement Standard · 3.0-Inch Recorder Top Margin · RESPA / TRID Proration",
        default_masthead: "judicial_caption",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "parcel_lot",
        default_readability: ReadabilityPreset::ExecutiveCompact,
        default_execution_mode: "wet-ink",
        notary_required: true,
        seal_filename: "seal_real_estate_title.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#b45309",
        stamp_label: "FUNDS BALANCED",
        stamp_sublabel: "GOOD FUNDS ESCROW",
        sample_org: "Lookout Title & Escrow Services PLLC",
        sample_counterparty: "Seller: H. Vance / Buyer: Blue Ridge Capital LLC",
        kpi_metrics: [
            ("CONTRACT SALES PRICE", "$540,000.00"),
            ("TAX PRORATION (309/365 DAYS)", "$3,184.11 CREDIT"),
            ("BUYER CASH TO CLOSE", "$114,820.40 WIRE"),
            ("NET PROCEEDS TO SELLER", "$218,450.89 WIRE"),
        ],
        line_items: [
            ("101-PRC", "Contract Sales Price — 1428 Lookout Crest Rd (Parcel #145H-B-019.00)", "Warranty Deed", "$540,000.00"),
            ("106-TAX", "2026 County & City Ad Valorem Tax Proration (01/01/2026–11/06/2026 = 309 Days)", "Seller Debit / Buyer Cr", "$3,184.11"),
            ("504-PAY", "First Mortgage Payoff to Regions Bank (Loan #XXXXXX9012 incl. 4 Days Per Diem)", "Seller Payoff", "$286,140.00"),
            ("700-COM", "Listing (2.5%) & Buyer-Side (2.5%) Brokerage Commissions + Title Policy ($2,425)", "Settlement Costs", "$29,425.00"),
        ],
        chart_rows: [
            ("Gross Contract Purchase Price", "$540,000.00", 100, "primary"),
            ("Existing First Mortgage Payoff", "$286,140.00", 53, "primary"),
            ("Net Seller Wire Disbursement", "$218,450.89", 40, "accent"),
        ],
        milestones: [
            ("Title Commitment", "24-month chain & lien clear", "VERIFIED"),
            ("Closing Execution", "Wet-ink deed & settlement", "11/06/2026"),
            ("Fedwire Disbursement", "Payoff + Seller net wired", "11/06 2:15PM"),
            ("Register of Deeds", "e-Recorded Book 13412 Pg 88", "RECORDED"),
        ],
    },

    // =========================================================================
    // VERTICAL 3: PROPERTY TAX APPEALS, CAD AUDITS & STATUTORY RELIEF (11..=15)
    // =========================================================================
    ArchetypeConceptSpec {
        id: 11,
        slug: "board-of-equalization-property-tax-appeal-petition-comp-grid",
        title: "Board of Equalization Property Tax Appeal Petition & Comparable Equity Grid",
        vertical: "Property Tax Appeals & CAD Audits",
        vertical_slug: "property-tax-appeals",
        target_profession: "Homeowners, Real Estate Investors, Tax Consultants & Pro Se Petitioners",
        primary_keyword: "property tax appeal petition and comparable sales grid template pdf",
        secondary_keywords: &["board of equalization appeal form", "uniformity assessment ratio grid", "pro se property tax protest packet"],
        demand_signal: "Core Untax homeowner demand: certified neighborhood $/sqft equity comparison grids paired with physical condition deductions and fillable pro se sworn petitions.",
        statutory_citation: "Tenn. Code Ann. § 67-5-1407 · Tenn. Const. art. II, § 28 · IAAO Uniformity Standard",
        default_masthead: "judicial_caption",
        default_kpi_style: "neo_brutalist",
        default_table_style: "dark_header",
        default_diagram_kind: "parcel_lot",
        default_readability: ReadabilityPreset::JudicialFormal,
        default_execution_mode: "wet-ink",
        notary_required: true,
        seal_filename: "seal_property_tax_cad.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "PRO SE PETITION",
        stamp_sublabel: "EQUITY VARIANCE",
        sample_org: "Untax Property Tax Audit Engine (untax.io)",
        sample_counterparty: "Hamilton County Board of Equalization",
        kpi_metrics: [
            ("CURRENT COUNTY APPRAISAL", "$482,500 ($219.32/SF)"),
            ("NEIGHBORHOOD MEDIAN $/SF", "$184.10 / SF (5 COMPS)"),
            ("CONTENDED FAIR MARKET VALUE", "$394,200.00"),
            ("ESTIMATED ANNUAL TAX DROP", "$493.80 / YR SAVINGS"),
        ],
        line_items: [
            ("SUBJ-00", "Subject Parcel #127N-A-014.00 — 2,200 GLA Sq Ft, 1984 Frame, Grade C+ (Over-Appraised)", "$219.32 / Sq Ft", "$482,500.00"),
            ("COMP-01", "Comp #1 (0.14 mi, Same NBHD Code): 2,180 Sq Ft, 1986 Built, Certified Roll Value", "$182.57 / Sq Ft", "$398,000.00"),
            ("COMP-02", "Comp #2 (0.22 mi, Same NBHD Code): 2,240 Sq Ft, 1983 Built, Certified Roll Value", "$184.10 / Sq Ft", "$412,384.00"),
            ("EXH-B01", "Exhibit B Deferred Maintenance Deduction: Documented Foundation & Roof Estimate", "Contractor Quote", "-$10,820.00"),
        ],
        chart_rows: [
            ("County Notice Appraised Value", "$482,500.00", 100, "primary"),
            ("Peer Equity Median Valuation", "$405,020.00", 84, "primary"),
            ("Petitioner Contended Fair Market", "$394,200.00", 81, "accent"),
        ],
        milestones: [
            ("Roll Audit & Comps", "5 uniform neighborhood comps", "COMPLETED"),
            ("Petitioner Execution", "Interactive AcroForm signed", "READY"),
            ("BOE Docket Filing", "Submitted prior to June 30", "DOCKETED"),
            ("Certified Roll Drop", "Trustee bill recalculated", "FINAL"),
        ],
    },
    ArchetypeConceptSpec {
        id: 12,
        slug: "disabled-veteran-100-pt-statutory-property-tax-relief-packet",
        title: "100% P&T Disabled Veteran Statutory Property Tax Relief Packet (Form F-16 / PT-50DV)",
        vertical: "Property Tax Appeals & CAD Audits",
        vertical_slug: "property-tax-appeals",
        target_profession: "100% P&T Disabled Veterans, Surviving Spouses (DIC) & County VSOs",
        primary_keyword: "100 disabled veteran property tax exemption application packet pdf",
        secondary_keywords: &["tennessee form f-16 disabled veteran tax relief", "georgia pt-50dv homestead exemption", "va summary of benefits property tax"],
        demand_signal: "Untax Veteran Shield ($0.00 Pro-Bono Forever): veterans and VSOs needing high-legibility 10.4pt walk-in packets combining state statutory relief forms with mortgage escrow refunds.",
        statutory_citation: "T.C.A. § 67-5-704 · O.C.G.A. § 48-5-48 · Va. Code § 58.1-3219.5 · 38 U.S.C. § 1311 (DIC)",
        default_masthead: "judicial_caption",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "parcel_lot",
        default_readability: ReadabilityPreset::HighLegibility,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_veteran_shield.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#b45309",
        stamp_label: "$0.00 PRO-BONO",
        stamp_sublabel: "100% P&T VETERAN",
        sample_org: "Untax Veteran Shield — Founder Pro-Bono Charter",
        sample_counterparty: "County Trustee & State Division of Property Assessments",
        kpi_metrics: [
            ("VA DISABILITY RATING", "100% P&T (SERVICE-CONNECTED)"),
            ("STATUTORY EXEMPTION BASIS", "$175,000 MKT / 100% VA"),
            ("ANNUAL TAX RELIEF ESTIMATE", "$2,480.00 / YR EXEMPT"),
            ("UNTAX PREPARATION FEE", "$0.00 FOREVER (PRO-BONO)"),
        ],
        line_items: [
            ("VET-01", "VA Benefits Summary Letter Verification (100% Permanent & Total Service-Connected)", "VA Lighthouse / Letter", "VERIFIED"),
            ("VET-02", "Primary Residence Homestead Occupancy & Deed Ownership Attestation", "Owner-Occupied", "QUALIFIED"),
            ("VET-03", "State & County Statutory Tax Relief Application (TN Form F-16 / GA PT-50DV / VA)", "Walk-In Ready", "$2,480.00/YR"),
            ("VET-04", "RESPA Section 10 Mortgage Servicer Escrow Surplus & Monthly Payment Reduction Demand", "12 C.F.R. § 1024.17", "INCLUDED"),
        ],
        chart_rows: [
            ("Pre-Exemption Annual Property Tax", "$2,840.00", 100, "primary"),
            ("Statutory Disabled Veteran Relief", "$2,480.00", 87, "accent"),
            ("Remaining Net Tax Obligation", "$360.00", 13, "primary"),
        ],
        milestones: [
            ("VA Rating Verified", "100% P&T Summary Letter", "STEP 1"),
            ("Veteran Signs Packet", "10.4pt High-Legibility PDF", "STEP 2"),
            ("Trustee Filing", "Filed with County Trustee", "STEP 3"),
            ("RESPA Escrow Refund", "Servicer drops monthly PITI", "STEP 4"),
        ],
    },
    ArchetypeConceptSpec {
        id: 13,
        slug: "commercial-income-capitalization-dark-store-tax-appeal-dossier",
        title: "Commercial Income Capitalization & Fee-Simple Tax Appeal Dossier",
        vertical: "Property Tax Appeals & CAD Audits",
        vertical_slug: "property-tax-appeals",
        target_profession: "Commercial Property Owners, Hospitality Operators, REITs & Tax Counsel",
        primary_keyword: "commercial property tax appeal income approach capitalization worksheet",
        secondary_keywords: &["stabilized noi tax appeal template", "loaded cap rate tax protest", "intangible business value exclusion"],
        demand_signal: "Commercial owners contesting inflated cost-approach assessments by presenting stabilized Direct Capitalization NOI worksheets with tax-loaded cap rates.",
        statutory_citation: "IAAO Commercial Income Approach Standard · Loaded Cap Rate = Base Cap + Effective Tax Rate",
        default_masthead: "swiss_grid",
        default_kpi_style: "dark_terminal",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::ExecutiveCompact,
        default_execution_mode: "wet-ink",
        notary_required: true,
        seal_filename: "seal_property_tax_cad.png",
        primary_hex: "#0f172a",
        accent_hex: "#059669",
        stamp_label: "INCOME APPROACH",
        stamp_sublabel: "CAP RATE: 9.15%",
        sample_org: "Untax Commercial & Industrial Valuation Division",
        sample_counterparty: "County Assessor Commercial Review Board",
        kpi_metrics: [
            ("COUNTY COST APPRAISAL", "$4,850,000.00"),
            ("STABILIZED NET OPERATING INC", "$334,890.00 / YR"),
            ("TAX-LOADED CAP RATE", "9.15% (7.75% + 1.40%)"),
            ("INDICATED INCOME VALUE", "$3,660,000.00 (-24.5%)"),
        ],
        line_items: [
            ("INC-PGI", "Potential Gross Rental Income (42,000 RSF @ $14.50/RSF Market Rent)", "100% Occupancy", "$609,000.00"),
            ("INC-VAC", "Market Vacancy & Collection Loss Allowance (12.0% Submarket Retail Vacancy)", "-12.0% Market", "-$73,080.00"),
            ("INC-EXP", "Operating Expenses Excl. Property Taxes (Insurance, CAM, Mgmt, CapEx Reserves)", "Audited P&L", "-$201,030.00"),
            ("VAL-CAP", "Capitalized Fee-Simple Real Estate Value ($334,890 NOI ÷ 9.15% Loaded Cap Rate)", "Direct Cap", "$3,660,000.00"),
        ],
        chart_rows: [
            ("Assessor Cost-Approach Valuation", "$4,850,000.00", 100, "primary"),
            ("Indicated Income Capitalization Value", "$3,660,000.00", 75, "accent"),
            ("Contended Over-Assessment Variance", "$1,190,000.00", 25, "primary"),
        ],
        milestones: [
            ("3-Year P&L & Rent Roll", "Certified by Managing Member", "EXHIBIT A"),
            ("Loaded Cap Rate Proof", "Removes circular tax bias", "EXHIBIT B"),
            ("Informal Review", "Assessor commercial analyst", "STAGE 1"),
            ("BOE Formal Hearing", "40% commercial ratio reduction", "STAGE 2"),
        ],
    },
    ArchetypeConceptSpec {
        id: 14,
        slug: "greenbelt-agricultural-forestry-present-use-valuation-application",
        title: "Greenbelt Agricultural & Forestry Present-Use Valuation Schedule (Exhibit G)",
        vertical: "Property Tax Appeals & CAD Audits",
        vertical_slug: "property-tax-appeals",
        target_profession: "Farm Owners, Timberland Investors, Rural Landowners & Estate Planners",
        primary_keyword: "agricultural greenbelt present use value property tax application pdf",
        secondary_keywords: &["tennessee greenbelt application form", "usda nrcs soil capability class tax protest", "rollback tax liability calculator"],
        demand_signal: "Rural acreage owners seeking statutory Greenbelt present-use valuation and USDA NRCS SSURGO soil capability class (Classes IV–VIII slope/karst) adjustments.",
        statutory_citation: "T.C.A. § 67-5-1001 et seq. (Greenbelt Act) · USDA NRCS SSURGO Soil Capability Classes",
        default_masthead: "editorial_serif",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "parcel_lot",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: true,
        seal_filename: "seal_property_tax_cad.png",
        primary_hex: "#14532d",
        accent_hex: "#b45309",
        stamp_label: "GREENBELT PUV",
        stamp_sublabel: "15+ ACRES QUALIFIED",
        sample_org: "Sequatchie Valley Timber & Agricultural Trust",
        sample_counterparty: "County Assessor of Property & Register of Deeds",
        kpi_metrics: [
            ("TOTAL QUALIFYING ACREAGE", "68.40 ACRES (TRACT #04)"),
            ("HIGHEST & BEST MARKET VAL", "$889,200.00 ($13k/AC)"),
            ("STATUTORY PRESENT-USE VAL", "$164,160.00 ($2.4k/AC)"),
            ("ANNUAL FARM TAX SAVINGS", "$4,042.00 / YR"),
        ],
        line_items: [
            ("SOIL-01", "22.0 Acres Class II Silt Loam Cropland & Hay Production ($1,850/Acre Present Use)", "Cropland Tier", "$40,700.00"),
            ("SOIL-02", "31.4 Acres Class VI Steep Slope Hardwood Timberland (>25% Grade, Forestry Plan)", "Forestry Tier", "$53,380.00"),
            ("SOIL-03", "14.0 Acres Class VII Karst Limestone Outcrop & Creek Buffer (Non-Arable Grazing)", "Impaired Pasture", "$15,080.00"),
            ("HOME-01", "1.0 Acre Statutory Homesite Curtilage Carveout (Assessed at Standard 25% Residential)", "Homesite Base", "$55,000.00"),
        ],
        chart_rows: [
            ("Speculative Development Market Value", "$889,200.00", 100, "primary"),
            ("Statutory Present-Use Farm Value", "$164,160.00", 18, "accent"),
            ("Exempt Speculative Land Value (82%)", "$725,040.00", 82, "accent"),
        ],
        milestones: [
            ("NRCS Soil & Timber Map", "68.40 acres mapped & verified", "ATTACHED"),
            ("Schedule F / Farm Proof", "$1,500+ annual gross ag receipts", "VERIFIED"),
            ("Notarized Application", "Signed for Register recording", "READY"),
            ("March 1 Statutory Filing", "Locks present-use tax basis", "FILED"),
        ],
    },
    ArchetypeConceptSpec {
        id: 15,
        slug: "respa-section-10-mortgage-escrow-surplus-refund-demand-letter",
        title: "RESPA Section 10 Mortgage Escrow Recalculation & Surplus Refund Demand",
        vertical: "Property Tax Appeals & CAD Audits",
        vertical_slug: "property-tax-appeals",
        target_profession: "Homeowners, Disabled Veterans, Real Estate Attorneys & Loan Officers",
        primary_keyword: "respa section 10 escrow recalculation and surplus refund demand letter pdf",
        secondary_keywords: &["12 cfr 1024.17 escrow surplus refund", "qualified written request mortgage tax drop", "lower monthly mortgage payment after tax appeal"],
        demand_signal: "Homeowners who win a tax appeal or veteran exemption often wait 12 months for their lender to lower their monthly payment unless they serve a formal 12 C.F.R. § 1024.17 demand.",
        statutory_citation: "12 U.S.C. § 2609 (RESPA § 10) · 12 C.F.R. § 1024.17(f) ($50+ Surplus 30-Day Refund Rule)",
        default_masthead: "judicial_caption",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "parcel_lot",
        default_readability: ReadabilityPreset::HighLegibility,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_property_tax_cad.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "RESPA § 10 DEMAND",
        stamp_sublabel: "30-DAY SURPLUS RULE",
        sample_org: "Untax Statutory Escrow Watchdog Engine",
        sample_counterparty: "Mr. Cooper / Nationstar Mortgage LLC — Escrow Dept",
        kpi_metrics: [
            ("PRIOR ANNUAL TAX ESCROW", "$4,680.00 ($390/MO)"),
            ("CERTIFIED REDUCED TAX BILL", "$2,820.00 ($235/MO)"),
            ("IMMEDIATE ESCROW SURPLUS", "$1,860.00 REFUND DUE"),
            ("MONTHLY PAYMENT DROP", "-$155.00 / MONTH"),
        ],
        line_items: [
            ("ESC-OLD", "Prior Servicer Projected Tax Disbursement Based on Unappealed Roll Value", "$390.00 / Month", "$4,680.00"),
            ("ESC-NEW", "Certified Revised County Trustee Tax Bill (Attached Official Receipt)", "$235.00 / Month", "$2,820.00"),
            ("ESC-CUSH", "Maximum Permissible RESPA 1/6th (2-Month) Cushion Adjustment", "12 C.F.R. § 1024.17(c)", "-$310.00"),
            ("ESC-REF", "Mandatory Cash Surplus Refund Check Required Within 30 Calendar Days (> $50 Threshold)", "12 C.F.R. § 1024.17(f)", "$1,860.00"),
        ],
        chart_rows: [
            ("Old Monthly Tax Escrow Withholding", "$390.00 / mo", 100, "primary"),
            ("New Certified Monthly Tax Escrow", "$235.00 / mo", 60, "accent"),
            ("Monthly Cash-Flow Savings Unlocked", "$155.00 / mo", 40, "accent"),
        ],
        milestones: [
            ("Certified Tax Receipt", "Issued by County Trustee", "ATTACHED"),
            ("RESPA Demand Served", "Sent to QWR Designated Addr", "DAY 0"),
            ("Short-Year Recalc", "Servicer updates PITI payment", "BY DAY 15"),
            ("Surplus Check Mailed", "$1,860.00 refunded to owner", "BY DAY 30"),
        ],
    },

    // =========================================================================
    // VERTICAL 4: SOFTWARE, SAAS, AI & CLOUD ENGINEERING (16..=20)
    // =========================================================================
    ArchetypeConceptSpec {
        id: 16,
        slug: "enterprise-saas-subscription-order-form-annual-invoice",
        title: "Enterprise SaaS Subscription Order Form, Seat Tier & Usage Invoice",
        vertical: "Software, SaaS & Cloud Engineering",
        vertical_slug: "software-saas-ai",
        target_profession: "B2B SaaS Founders, RevOps Leaders, Account Executives & Billing Teams",
        primary_keyword: "enterprise saas order form and annual subscription invoice template",
        secondary_keywords: &["b2b saas order form pdf", "annual recurring revenue arr invoice", "soc2 enterprise software order schedule"],
        demand_signal: "Hacker News (`Show HN: XSDHub` & `TemplateBox`) and SaaS founders seeking clean enterprise order forms combining committed seat tiers, API token overages, and net-30 wire instructions.",
        statutory_citation: "ASC 606 Revenue Recognition Schedule · MSA Incorporation · UETA / ESIGN Execution",
        default_masthead: "swiss_grid",
        default_kpi_style: "dark_terminal",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "esign",
        notary_required: false,
        seal_filename: "seal_saas_cybersecurity.png",
        primary_hex: "#0f172a",
        accent_hex: "#0284c7",
        stamp_label: "NET-30 ENTERPRISE",
        stamp_sublabel: "SOC2 TYPE II VERIFIED",
        sample_org: "VectorPulse Cloud Systems Inc.",
        sample_counterparty: "Meridian Logistics Group Inc. (Enterprise Account)",
        kpi_metrics: [
            ("ANNUAL CONTRACT VALUE (ACV)", "$48,600.00 / YR"),
            ("COMMITTED PLATFORM SEATS", "75 Enterprise Seats"),
            ("INCLUDED API VOLUME", "25,000,000 Req / Mo"),
            ("BILLING CADENCE & TERMS", "Annual Upfront · Net 30"),
        ],
        line_items: [
            ("SKU-ENT", "VectorPulse Enterprise Platform Subscription (SSO/SAML, RBAC, 99.95% SLA)", "75 Seats @ $45/mo", "$40,500.00"),
            ("SKU-API", "Dedicated High-Throughput Ingestion Cluster & 25M Monthly API Calls", "1 Cluster / 12 Mo", "$6,000.00"),
            ("SKU-IMP", "White-Glove Solutions Engineering Onboarding & Custom Webhook Migration", "Fixed Package", "$4,500.00"),
            ("DSC-ANN", "Annual Pre-Payment Commitment Discount (Applied to Base Platform Tier)", "-$2,400.00 Credit", "-$2,400.00"),
        ],
        chart_rows: [
            ("Core Enterprise Seat Licensing", "$40,500.00", 83, "primary"),
            ("Dedicated API Cluster & Onboarding", "$10,500.00", 22, "accent"),
            ("Net Annual Invoice Total (After Disc)", "$48,600.00", 100, "accent"),
        ],
        milestones: [
            ("Order Form Counter-Sign", "ESIGN execution by VP Eng", "11/06/2026"),
            ("SSO & Tenant Provision", "Okta SAML + prod API keys", "11/08/2026"),
            ("Solutions Go-Live", "Webhook cutover verified", "11/20/2026"),
            ("Net-30 Wire Settlement", "ACH to Silicon Valley / Chase", "12/06/2026"),
        ],
    },
    ArchetypeConceptSpec {
        id: 17,
        slug: "software-engineering-statement-of-work-sow-sprint-milestone-accept",
        title: "Software Engineering Statement of Work (SOW) & Sprint Acceptance Certificate",
        vertical: "Software, SaaS & Cloud Engineering",
        vertical_slug: "software-saas-ai",
        target_profession: "Software Agencies, Fractional CTOs, Rust/TypeScript Consultants & Dev Shops",
        primary_keyword: "software development statement of work sow template with sprint milestones",
        secondary_keywords: &["agile sprint acceptance certificate pdf", "custom software deliverables schedule", "ip assignment upon payment clause"],
        demand_signal: "r/freelance & dev agencies needing concrete deliverable acceptance criteria, 5-business-day deemed-acceptance windows, and IP assignment conditioned on final invoice settlement.",
        statutory_citation: "17 U.S.C. § 101 Work-for-Hire / Conditional IP Assignment · 5-Day UAT Acceptance Rule",
        default_masthead: "swiss_grid",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_saas_cybersecurity.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "UAT ACCEPTED",
        stamp_sublabel: "IP TRANSFER READY",
        sample_org: "Ironclad Systems Architecture LLC",
        sample_counterparty: "FinStream Treasury Corp.",
        kpi_metrics: [
            ("TOTAL FIXED SOW BUDGET", "$78,000.00 (8 WEEKS)"),
            ("CURRENT MILESTONE (#03)", "$23,400.00 (30% DRAW)"),
            ("TEST SUITE COVERAGE", "96.4% (142 Nextest Pass)"),
            ("P99 LATENCY BENCHMARK", "4.8 ms (< 15 ms SLA)"),
        ],
        line_items: [
            ("M1-ARCH", "Milestone 1: Rust Tokio/Axum Core Service Architecture, OpenAPI Spec & SQLite WAL Schema", "Paid 10/01 (30%)", "$23,400.00"),
            ("M2-INTG", "Milestone 2: Stripe Webhook Idempotency Engine, 1Password Vault Wiring & OAuth2 PKCE", "Paid 10/22 (30%)", "$23,400.00"),
            ("M3-UAT", "Milestone 3 (Current): Production Load Testing (10k RPS), Security Audit & UAT Sign-Off", "Due Upon Sign-Off", "$23,400.00"),
            ("M4-CUT", "Milestone 4: Zero-Downtime Production Cutover, Runbook & 14-Day Hypercare Window", "Final 10% Holdback", "$7,800.00"),
        ],
        chart_rows: [
            ("Previously Paid Milestones (M1 + M2)", "$46,800.00", 60, "primary"),
            ("Current Milestone #3 UAT Release", "$23,400.00", 30, "accent"),
            ("Final Cutover Holdback (M4)", "$7,800.00", 10, "primary"),
        ],
        milestones: [
            ("Staging Deployment", "Deployed with CI artifacts", "11/02/2026"),
            ("Automated QA Gate", "142/142 tests passing", "11/04/2026"),
            ("Client UAT Sign-Off", "5-day review completed", "11/06/2026"),
            ("IP Assignment Vesting", "Vests upon M3/M4 wire clear", "AUTOMATIC"),
        ],
    },
    ArchetypeConceptSpec {
        id: 18,
        slug: "cybersecurity-penetration-test-soc2-vulnerability-remediation-report",
        title: "Cybersecurity Penetration Test Executive Attestation & CVSS Remediation Matrix",
        vertical: "Software, SaaS & Cloud Engineering",
        vertical_slug: "software-saas-ai",
        target_profession: "Penetration Testers, vCISOs, AppSec Engineers & SOC2 Compliance Auditors",
        primary_keyword: "penetration test executive summary and vulnerability remediation report template",
        secondary_keywords: &["soc2 pentest attestation letter pdf", "cvss v3.1 vulnerability schedule", "owasp top 10 remediation sign off"],
        demand_signal: "Enterprise SaaS vendors needing clean external-facing Penetration Test Letters of Attestation (with 0 Critical / 0 High verified after re-test) to unblock enterprise procurement.",
        statutory_citation: "SOC 2 Trust Services Criteria CC6.1/CC7.1 · NIST SP 800-115 · OWASP ASVS v4.0.3",
        default_masthead: "trade_banner",
        default_kpi_style: "dark_terminal",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_saas_cybersecurity.png",
        primary_hex: "#0f172a",
        accent_hex: "#059669",
        stamp_label: "RE-TEST PASSED",
        stamp_sublabel: "0 CRITICAL / 0 HIGH",
        sample_org: "Aegis Zero-Day Security Labs LLC (OSCP / CREST)",
        sample_counterparty: "CloudLedger Enterprise Inc. (Audit Scope: v4.2)",
        kpi_metrics: [
            ("OPEN CRITICAL / HIGH", "0 CRITICAL · 0 HIGH"),
            ("REMEDIATED FINDINGS", "6 OF 6 VERIFIED CLOSED"),
            ("ASSESSMENT METHODOLOGY", "OWASP ASVS L2 + API"),
            ("ATTESTATION VALIDITY", "Nov 2026 – Nov 2027"),
        ],
        line_items: [
            ("VULN-01", "[HIGH 7.5 → CLOSED] BOLA/IDOR in Multi-Tenant Export Endpoint (/api/v2/exports/:id)", "Patched in commit 9a4f1c", "REMEDIATED"),
            ("VULN-02", "[MED 5.4 → CLOSED] Missing Strict-Transport-Security & CSP frame-ancestors Header", "Verified on Edge CDN", "REMEDIATED"),
            ("VULN-03", "[MED 4.8 → CLOSED] OAuth2 Refresh Token Rotation Grace Window Exceeded 30 Seconds", "Reduced to Atomic Use", "REMEDIATED"),
            ("VULN-04", "[LOW 3.1 → CLOSED] Verbose Stack Trace Disclosure on Malformed Multipart Upload", "Sanitized JSON Error", "REMEDIATED"),
        ],
        chart_rows: [
            ("Initial Discovery Findings (6 Total)", "100% Scoped", 100, "primary"),
            ("Verified Patched & Re-Tested Closed", "6 / 6 Closed (100%)", 100, "accent"),
            ("Residual Open Critical/High Risk", "0 Open (0%)", 4, "primary"),
        ],
        milestones: [
            ("Grey-Box Exploitation", "80 hrs manual + burp/nuclei", "10/14–10/24"),
            ("Initial Findings Issued", "1 High, 2 Med, 3 Low logged", "10/26/2026"),
            ("Engineering Hotfixes", "PRs merged & deployed", "11/01/2026"),
            ("Clean Attestation Issued", "Ready for Enterprise Customers", "11/06/2026"),
        ],
    },
    ArchetypeConceptSpec {
        id: 19,
        slug: "ai-automation-agency-llm-integration-retainer-roi-proposal",
        title: "AI Automation & LLM Workflow Engineering Proposal & Token Cost Schedule",
        vertical: "Software, SaaS & Cloud Engineering",
        vertical_slug: "software-saas-ai",
        target_profession: "AI Automation Agencies, ML Engineers, Prompt/RAG Architects & RevOps Builders",
        primary_keyword: "ai automation agency proposal and retainer agreement template pdf",
        secondary_keywords: &["llm integration statement of work", "rag pipeline architecture proposal", "ai agent roi and token cost worksheet"],
        demand_signal: "Rapidly growing search demand from AI implementation consultants for transparent proposals separating build fees, client-owned API key pass-throughs, and measurable hours saved.",
        statutory_citation: "Zero Data Retention (ZDR) API Governance · Client-Owned Model Weights & Prompts",
        default_masthead: "neo_brutalist",
        default_kpi_style: "dark_terminal",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_saas_cybersecurity.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "ZDR PRIVACY SAFE",
        stamp_sublabel: "4.2X YEAR-1 ROI",
        sample_org: "Synthetix AI Systems & Workflow Engineering",
        sample_counterparty: "Southeastern Freight Brokerage LLC",
        kpi_metrics: [
            ("IMPLEMENTATION BUILD FEE", "$14,500.00 ONE-TIME"),
            ("MONTHLY OPS & EVAL RETAINER", "$2,200.00 / MONTH"),
            ("ESTIMATED API PASS-THROUGH", "~$185.00 / MO (DIRECT)"),
            ("PROJECTED LABOR SAVINGS", "160 Hrs/Mo ($7,200/Mo)"),
        ],
        line_items: [
            ("AI-ING", "Automated Carrier Rate-Confirmation PDF & Bill of Lading Extraction Pipeline", "99.2% Field Accuracy", "$5,500.00"),
            ("AI-ERP", "Two-Way TMS & HighLevel CRM Webhook Dispatcher with Human-in-the-Loop Confidence Gate", "< 0.92 Escalates", "$5,200.00"),
            ("AI-EVL", "Golden-Dataset Regression Suite (250 Real Waybills) & Hallucination Guardrails", "Deterministic Schema", "$3,800.00"),
            ("RET-MNT", "Monthly Prompt Calibration, Model Version Pinning & Latency Monitoring Retainer", "Optional Post-Launch", "$2,200.00/MO"),
        ],
        chart_rows: [
            ("Annualized Manual Data-Entry Cost", "$86,400.00 / yr", 100, "primary"),
            ("Year-1 AI Build + Retainer Investment", "$40,900.00 / yr", 47, "primary"),
            ("Net Year-1 Operational Savings", "$45,500.00 / yr", 53, "accent"),
        ],
        milestones: [
            ("Golden Dataset Audit", "250 historical PDFs benchmarked", "WEEK 1"),
            ("Sandbox Webhook Pipe", "TMS staging integration", "WEEK 2"),
            ("Shadow-Mode Parallel", "5 days side-by-side verification", "WEEK 3"),
            ("Production Cutover", "Live dispatch + team training", "WEEK 4"),
        ],
    },
    ArchetypeConceptSpec {
        id: 20,
        slug: "cloud-finops-sla-incident-postmortem-service-credit-memo",
        title: "Cloud Infrastructure SLA Post-Mortem (RCA) & Contractual Service Credit Memo",
        vertical: "Software, SaaS & Cloud Engineering",
        vertical_slug: "software-saas-ai",
        target_profession: "DevOps/SRE Teams, Cloud Providers, Managed Service Providers (MSPs) & FinOps",
        primary_keyword: "sla incident postmortem report and service credit memo template pdf",
        secondary_keywords: &["root cause analysis rca customer report", "uptime sla credit calculation", "msp incident response summary"],
        demand_signal: "B2B infrastructure and MSP teams needing a formal, customer-ready Root Cause Analysis (RCA) paired with an exact SLA credit calculation to preserve enterprise trust after an incident.",
        statutory_citation: "Enterprise SLA Exhibit B · 99.95% Monthly Uptime Commitment · 10% Service Credit Tier",
        default_masthead: "swiss_grid",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "esign",
        notary_required: false,
        seal_filename: "seal_saas_cybersecurity.png",
        primary_hex: "#0f172a",
        accent_hex: "#d97706",
        stamp_label: "SLA CREDIT ISSUED",
        stamp_sublabel: "RCA PREVENTATIVE FIX",
        sample_org: "NimbusEdge Managed Cloud Infrastructure Inc.",
        sample_counterparty: "omniPay Financial Services Corp.",
        kpi_metrics: [
            ("OCTOBER ACTUAL UPTIME", "99.84% (SLA: 99.95%)"),
            ("TOTAL DEGRADED WINDOW", "68 Minutes (10/19 UTC)"),
            ("APPLICABLE SLA CREDIT TIER", "10.0% Monthly Credit"),
            ("CREDIT APPLIED TO NEXT INV", "-$1,240.00 CREDIT"),
        ],
        line_items: [
            ("RCA-01", "Root Cause: Upstream BGP Route Flap During Regional Firewall Kernel Live-Patch", "02:14–03:22 UTC", "IDENTIFIED"),
            ("FIX-01", "Immediate Remediation: Automated Failover Threshold Lowered from 90s to 8s Health Check", "Deployed 10/20", "VERIFIED"),
            ("FIX-02", "Structural Guardrail: Dual-Region Active-Active Anycast Ingress Enabled for Tenant", "Zero Extra Cost", "ENABLED"),
            ("CRD-10", "Contractual 10% SLA Uptime Service Credit Applied to November Invoice #INV-2026-1104", "10% of $12,400/mo", "-$1,240.00"),
        ],
        chart_rows: [
            ("Monthly Base Infrastructure Fee", "$12,400.00", 100, "primary"),
            ("10% Contractual SLA Service Credit", "-$1,240.00", 18, "accent"),
            ("Net Adjusted November Invoice", "$11,160.00", 82, "primary"),
        ],
        milestones: [
            ("02:14 UTC Alert Fired", "PagerDuty SRE on-call engaged", "T+0 MIN"),
            ("02:41 UTC Traffic Shifted", "Secondary region absorbed load", "T+27 MIN"),
            ("03:22 UTC Full Recovery", "Error rate returned to 0.00%", "T+68 MIN"),
            ("Preventative Patch Live", "8-second failover active", "CLOSED"),
        ],
    },

    // =========================================================================
    // VERTICAL 5: CREATIVE STUDIOS, DESIGN, PHOTOGRAPHY & EVENTS (21..=25)
    // =========================================================================
    ArchetypeConceptSpec {
        id: 21,
        slug: "creative-agency-brand-identity-web-design-proposal-retainer",
        title: "Creative Studio Brand Identity & Web Design Proposal + IP Licensing Schedule",
        vertical: "Creative Studios, Media & Events",
        vertical_slug: "creative-events",
        target_profession: "Brand Designers, Creative Directors, Webflow/Framer Studios & Design Agencies",
        primary_keyword: "brand identity and web design proposal and contract template pdf",
        secondary_keywords: &["creative agency scope of work pdf", "design revision cap clause", "font and vector source file licensing"],
        demand_signal: "r/freelance & creative studios emphasizing revision-round caps (2 rounds included, hourly thereafter), kill-fee protection, and vector source file IP transfer upon final payment.",
        statutory_citation: "AIGA Standard Form of Agreement for Design Services · 17 U.S.C. § 204 IP Transfer",
        default_masthead: "editorial_serif",
        default_kpi_style: "metric_cards",
        default_table_style: "minimal_lines",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_creative_events.png",
        primary_hex: "#311042",
        accent_hex: "#d97706",
        stamp_label: "2 REVISION ROUNDS",
        stamp_sublabel: "AIGA IP STANDARD",
        sample_org: "Atelier Vanguard Brand & Digital Studio",
        sample_counterparty: "Solstice Botanical Beverage Co.",
        kpi_metrics: [
            ("TOTAL PROJECT INVESTMENT", "$24,000.00 FIXED"),
            ("BOOKING DEPOSIT (50%)", "$12,000.00 DUE NOW"),
            ("INCLUDED REVISION ROUNDS", "2 Consolidated Rounds"),
            ("TIMELINE TO LAUNCH", "6 Weeks (3 Phases)"),
        ],
        line_items: [
            ("DES-01", "Phase 1: Brand Strategy, Primary Logomark, Custom Typography System & 32-Page Brand Book", "Weeks 1–2", "$8,500.00"),
            ("DES-02", "Phase 2: 3D Beverage Can Packaging Die-Lines, Print Pre-Press & Retail Display Kit", "Weeks 3–4", "$6,500.00"),
            ("DES-03", "Phase 3: Responsive 12-Page Marketing & Wholesale E-Commerce Website + SEO Schema", "Weeks 5–6", "$9,000.00"),
            ("REV-CAP", "Additional Out-of-Scope Revision Rounds Beyond Round 2 (Billed Only Upon Written Approval)", "$175 / Hr Rate", "CONTINGENT"),
        ],
        chart_rows: [
            ("50% Commencement Booking Deposit", "$12,000.00", 50, "accent"),
            ("25% Brand Book & Packaging Approval", "$6,000.00", 25, "primary"),
            ("25% Final Web Launch & Vector Hand-Off", "$6,000.00", 25, "primary"),
        ],
        milestones: [
            ("Deposit & Kickoff", "50% deposit locks studio slot", "WEEK 1"),
            ("Brand Identity Sign-Off", "Logomark & palette approved", "WEEK 2"),
            ("Packaging Pre-Press", "CMYK/Pantone print-ready", "WEEK 4"),
            ("Site Launch & IP Vest", "Source files transferred", "WEEK 6"),
        ],
    },
    ArchetypeConceptSpec {
        id: 22,
        slug: "commercial-photography-videography-production-call-sheet-usage-license",
        title: "Commercial Photography & Video Production Estimate + Usage Licensing Agreement",
        vertical: "Creative Studios, Media & Events",
        vertical_slug: "creative-events",
        target_profession: "Commercial Photographers, DP/Videographers, Production Houses & Drone Pilots",
        primary_keyword: "commercial photography estimate and usage rights license template pdf",
        secondary_keywords: &["video production day rate invoice", "commercial image licensing buyout vs term", "faa part 107 drone shoot agreement"],
        demand_signal: "Photographers and video producers needing AICP/ASMP-style separation between creative day rates, crew/gear hard costs, and commercial paid-media usage rights.",
        statutory_citation: "ASMP / AICP Production Billing Standard · 17 U.S.C. § 106 Exclusive Usage License",
        default_masthead: "editorial_serif",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_creative_events.png",
        primary_hex: "#0f172a",
        accent_hex: "#e11d48",
        stamp_label: "2-YR PAID MEDIA",
        stamp_sublabel: "ASMP USAGE LICENSE",
        sample_org: "Kinetix Cinema & Stills Production LLC",
        sample_counterparty: "Veloce Carbon Cycling Inc. (Q1 Campaign)",
        kpi_metrics: [
            ("PRODUCTION SHOOT DAYS", "2 Days On-Location"),
            ("DELIVERABLE ASSETS", "40 Retouched Stills + 2 Spots"),
            ("USAGE LICENSE TERRITORY", "North America · 24 Months"),
            ("TOTAL PRODUCTION + LICENSE", "$16,450.00"),
        ],
        line_items: [
            ("CREW-01", "Director / Photographer Creative Fee + FAA Part 107 Cinema Drone Operator (2 Shoot Days)", "2 Days @ $3,200", "$6,400.00"),
            ("GEAR-02", "RED V-Raptor 8K Cinema Package, Medium Format Stills, Aputure Lighting & Grip Van", "2-Day Kit Rental", "$3,150.00"),
            ("POST-03", "Post-Production: 40 High-End 16-Bit Retouched Hero Stills + 60s & 15s DaVinci Color Grade", "Deliverables Pkg", "$3,400.00"),
            ("LIC-04", "Commercial Usage License: Paid Social, Web, OOH Billboard & Retail POS (24 Months NA)", " excludes RAW files", "$3,500.00"),
        ],
        chart_rows: [
            ("Principal Shoot Crew & Equipment", "$9,550.00", 58, "primary"),
            ("Post-Production Grading & Retouching", "$3,400.00", 21, "primary"),
            ("24-Month Commercial Usage License", "$3,500.00", 21, "accent"),
        ],
        milestones: [
            ("50% Retainer Paid", "Holds crew & location permits", "11/10/2026"),
            ("Principal Photography", "2-day shoot on Cherohala Skyway", "11/18–11/19"),
            ("Rough Cut & Selects", "Frame.io review link delivered", "11/25/2026"),
            ("Final Master & License", "Released upon balance payment", "12/02/2026"),
        ],
    },
    ArchetypeConceptSpec {
        id: 23,
        slug: "wedding-event-venue-catering-banquet-event-order-beo-contract",
        title: "Event Venue & Catering Banquet Event Order (BEO) + Guaranteed Headcount Contract",
        vertical: "Creative Studios, Media & Events",
        vertical_slug: "creative-events",
        target_profession: "Wedding Venues, Executive Caterers, Event Planners & Hospitality Directors",
        primary_keyword: "banquet event order beo and catering contract template pdf",
        secondary_keywords: &["wedding venue rental contract pdf", "final guaranteed guest count clause", "catering service charge and gratuity breakdown"],
        demand_signal: "Event venues and caterers requiring clear 14-day guaranteed headcount lock dates, non-refundable date-hold retainers, and explicit separation of service charges vs. tax.",
        statutory_citation: "Hospitality BEO Standard · 14-Day Final Guarantee Lock · Liquor Liability & Force Majeure",
        default_masthead: "editorial_serif",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::HighLegibility,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_creative_events.png",
        primary_hex: "#1e293b",
        accent_hex: "#b45309",
        stamp_label: "DATE RESERVED",
        stamp_sublabel: "160 GUEST GUARANTEE",
        sample_org: "The Foundry Conservatory & Culinary Events",
        sample_counterparty: "Harrison & Montgomery Wedding Reception (Event #BEO-409)",
        kpi_metrics: [
            ("GUARANTEED GUEST COUNT", "160 Guests (Locked 14d Prior)"),
            ("FOOD & BEVERAGE MINIMUM", "$18,500.00"),
            ("DEPOSITS PAID TO DATE", "$11,200.00"),
            ("FINAL BALANCE DUE (14D OUT)", "$14,684.00"),
        ],
        line_items: [
            ("VEN-01", "Exclusive Saturday Grand Conservatory & Courtyard Buyout (10:00 AM – 11:30 PM, Tables/Chivari)", "13.5 Hr Buyout", "$6,500.00"),
            ("CAT-02", "Plated Dual-Entree Dinner Service + 3 Passed Hors d'Oeuvres (160 Guaranteed Guests @ $88)", "160 Guests @ $88", "$14,080.00"),
            ("BAR-03", "4-Hour Craft Beer, Sommelier Wine & Signature Cocktail Package (Licensed ABC Bartenders)", "160 Guests @ $38", "$6,080.00"),
            ("SVC-22", "Production Staffing, Linen/China Charge & Applicable State/Local Sales Tax", "Consolidated", "-$876.00 Disc"),
        ],
        chart_rows: [
            ("Culinary & Bar Hospitality Package", "$20,160.00", 78, "primary"),
            ("Exclusive Venue & Courtyard Buyout", "$6,500.00", 25, "primary"),
            ("Remaining Balance Due 14 Days Out", "$14,684.00", 57, "accent"),
        ],
        milestones: [
            ("Booking Retainer (25%)", "Locks Saturday venue date", "PAID"),
            ("Tasting & Floorplan", "Seating chart & menu locked", "PAID (50%)"),
            ("14-Day Final Headcount", "160 guest count & balance", "DUE 14D OUT"),
            ("Event Day Execution", "10:00 AM load-in to 11:30 PM", "EVENT DAY"),
        ],
    },
    ArchetypeConceptSpec {
        id: 24,
        slug: "creator-influencer-brand-sponsorship-ugc-deliverables-contract",
        title: "Creator / Influencer Brand Sponsorship & Paid Whitelisting Usage Deal Memo",
        vertical: "Creative Studios, Media & Events",
        vertical_slug: "creative-events",
        target_profession: "YouTube/TikTok Creators, Talent Managers, UGC Studios & Brand Marketers",
        primary_keyword: "influencer brand sponsorship contract and ugc usage rights template pdf",
        secondary_keywords: &["creator deal memo template", "paid social whitelisting usage fee", "ftc 16 cfr 255 endorsement disclosure"],
        demand_signal: "Creators and talent agencies demanding clear guardrails on paid ad whitelisting duration (30/60/90 days), prohibition on unauthorized AI voice/likeness cloning, and Net-15 payment.",
        statutory_citation: "FTC 16 C.F.R. Part 255 Endorsement Guides · Right of Publicity & Anti-AI Cloning Clause",
        default_masthead: "neo_brutalist",
        default_kpi_style: "neo_brutalist",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_creative_events.png",
        primary_hex: "#0f172a",
        accent_hex: "#059669",
        stamp_label: "NO AI CLONING",
        stamp_sublabel: "60-DAY SPARK ADS",
        sample_org: "Apex Creator Talent Management LLC",
        sample_counterparty: "AuraSleep Performance Tech Inc. (Brand Sponsor)",
        kpi_metrics: [
            ("TOTAL CAMPAIGN FEE", "$9,500.00 (NET-15)"),
            ("ORGANIC DELIVERABLES", "1 YouTube Integration + 2 Reels"),
            ("PAID AD WHITELISTING", "60 Days (Meta + TikTok)"),
            ("CATEGORY EXCLUSIVITY", "30 Days (Sleep Trackers)"),
        ],
        line_items: [
            ("DEL-YT1", "90-Second Mid-Roll Sponsored Integration on Main YouTube Channel (Pinned Link + 30d Exclusivity)", "1 Video (Est 120k+)", "$5,500.00"),
            ("DEL-SF2", "2x 45-Second Vertical Short-Form Videos (Instagram Reels + TikTok Cross-Post)", "2 Vertical Assets", "$2,500.00"),
            ("LIC-WL", "60-Day Paid Media Whitelisting / Spark Ads Authorization (Organic Handle Boost)", "60 Calendar Days", "$1,500.00"),
            ("AI-PROH", "Express Prohibition on Generative AI Voice, Face, or Biometric Likeness Training/Synthesis", "Mandatory Protection", "INCLUDED"),
        ],
        chart_rows: [
            ("YouTube 90s Deep-Dive Integration", "$5,500.00", 58, "primary"),
            ("Vertical Reels/TikTok Deliverables", "$2,500.00", 26, "primary"),
            ("60-Day Paid Media Whitelisting Rights", "$1,500.00", 16, "accent"),
        ],
        milestones: [
            ("Brief & Talking Points", "Approved by Brand & Creator", "DAY 1"),
            ("Draft Video Review", "1 factual accuracy check", "DAY 7"),
            ("Live Publication", "Published with #ad disclosure", "DAY 10"),
            ("Net-15 ACH Payout", "Full $9,500 settled", "DAY 25"),
        ],
    },
    ArchetypeConceptSpec {
        id: 25,
        slug: "architectural-interior-design-ffe-procurement-schedule-invoice",
        title: "Interior Design & Architectural FF&E Procurement Schedule + Trade Markup Invoice",
        vertical: "Creative Studios, Media & Events",
        vertical_slug: "creative-events",
        target_profession: "Interior Designers, Hospitality Specifiers, Residential Architects & Stagers",
        primary_keyword: "interior design ffe procurement schedule and trade markup invoice template",
        secondary_keywords: &["furniture fixtures and equipment specification sheet pdf", "designer trade discount split invoice", "custom millwork deposit schedule"],
        demand_signal: "Interior design studios needing clear client sign-off on custom non-returnable FF&E orders, freight/white-glove receiving reserves, and transparent trade-discount purchasing fees.",
        statutory_citation: "ASID / AIA B152 Interior Design Procurement Standard · Non-Cancellable Custom Goods Notice",
        default_masthead: "editorial_serif",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_creative_events.png",
        primary_hex: "#1e293b",
        accent_hex: "#b45309",
        stamp_label: "FF&E APPROVED",
        stamp_sublabel: "100% PROCUREMENT",
        sample_org: "Studio Meriwether Architecture & Interiors",
        sample_counterparty: "The Caldwell Residence — Primary Living & Library",
        kpi_metrics: [
            ("RETAIL MSRP VALUE", "$48,200.00"),
            ("DESIGNER NET TRADE COST", "$31,330.00 (-35% MSRP)"),
            ("PROCUREMENT FEE (20%)", "$6,266.00"),
            ("TOTAL CLIENT FF&E INVOICE", "$41,446.00 (SAVES $6,754)"),
        ],
        line_items: [
            ("FFE-101", "Custom 118\" Belgian Performance Linen Sectional Sofa (COM Yardage + Kiln-Dried Alder)", "Lead Time: 8 Wks", "$11,400.00"),
            ("FFE-104", "Pair of Hand-Rubbed Antiqued Brass Library Sconces & Custom Alabaster Chandelier (UL Listed)", "3 Fixtures", "$6,850.00"),
            ("FFE-109", "Bespoke Rift-Cut White Oak Dining Table (108\" x 44\") + 8 Saddle Leather Dining Chairs", "Artisan Millwork", "$13,080.00"),
            ("LOG-20", "Studio Procurement Management (20%) + Climate-Controlled Receiver Warehouse & White-Glove Install", "Turnkey Delivery", "$10,116.00"),
        ],
        chart_rows: [
            ("Showroom Retail List Price (MSRP)", "$48,200.00", 100, "primary"),
            ("Total Client Turnkey Invoice", "$41,446.00", 86, "accent"),
            ("Shared Trade Discount Client Savings", "$6,754.00", 14, "accent"),
        ],
        milestones: [
            ("Finish Samples Signed", "Swatches & CAD elevations OK", "STEP 1"),
            ("100% FF&E Funds Cleared", "Required prior to factory POs", "STEP 2"),
            ("Warehouse Inspection", "Unboxed & inspected for damage", "WEEK 7"),
            ("White-Glove Install Day", "Placed, steamed & styled", "WEEK 8"),
        ],
    },

    // =========================================================================
    // VERTICAL 6: LEGAL, CPA, TAX ADVISORY & FINANCIAL SERVICES (26..=30)
    // =========================================================================
    ArchetypeConceptSpec {
        id: 26,
        slug: "law-firm-evergreen-iolta-trust-retainer-engagement-agreement",
        title: "Law Firm Engagement Agreement, Evergreen IOLTA Trust Retainer & LEDES Billing Statement",
        vertical: "Legal, CPA & Financial Advisory",
        vertical_slug: "legal-cpa-advisory",
        target_profession: "Litigation Attorneys, Real Estate Counsel, Boutique Law Firms & General Counsel",
        primary_keyword: "law firm engagement letter and evergreen iolta trust retainer statement pdf",
        secondary_keywords: &["legal retainer Replenishment invoice template", "aba utbms ledes billing statement", "attorney trust account accounting"],
        demand_signal: "Boutique law firms requiring clear ABA Rule 1.15 IOLTA trust account ledgers showing beginning trust balance, itemized UTBMS task deductions, and evergreen replenishment triggers.",
        statutory_citation: "ABA Model Rules 1.5 & 1.15 (IOLTA Trust Safekeeping) · UTBMS / LEDES Task Codes",
        default_masthead: "judicial_caption",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "parcel_lot",
        default_readability: ReadabilityPreset::JudicialFormal,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_legal_cpa_advisory.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#b45309",
        stamp_label: "IOLTA VERIFIED",
        stamp_sublabel: "ABA RULE 1.15 TRUST",
        sample_org: "Vance, Sterling & Thorne PLLC — Attorneys at Law",
        sample_counterparty: "Highland Commercial Development LLC (Matter #2026-CV-419)",
        kpi_metrics: [
            ("OPENING IOLTA TRUST BAL", "$10,000.00"),
            ("OCTOBER FEES & COSTS", "$6,425.00 (16.2 HRS)"),
            ("REMAINING IOLTA BALANCE", "$3,575.00"),
            ("EVERGREEN REPLENISHMENT", "$6,425.00 DUE TO TRUST"),
        ],
        line_items: [
            ("L120-A", "Analyze Municipal Zoning Variance Record & Draft Verified Petition for Writ of Certiorari", "6.4 Hrs @ $425/hr", "$2,720.00"),
            ("L210-B", "Prepare & Conduct Rule 30(b)(6) Deposition of Planning Commission Staff Engineer", "5.8 Hrs @ $425/hr", "$2,465.00"),
            ("L140-C", "Associate Legal Research & Table of Authorities on Vested Property Rights Doctrine", "4.0 Hrs @ $240/hr", "$960.00"),
            ("E112-D", "Chancery Court Filing Fee & Certified Court Reporter Deposition Transcript (At Cost)", "Disbursements", "$280.00"),
        ],
        chart_rows: [
            ("Evergreen Minimum IOLTA Threshold", "$10,000.00", 100, "primary"),
            ("Earned October Fees Transferred", "$6,425.00", 64, "primary"),
            ("Remaining Client IOLTA Trust Funds", "$3,575.00", 36, "accent"),
        ],
        milestones: [
            ("Retainer Deposited", "$10,000 in IOLTA Trust", "09/15/2026"),
            ("October Statement Sent", "5-day review prior to draw", "11/01/2026"),
            ("IOLTA Fee Transfer", "$6,425 earned fee disbursed", "11/06/2026"),
            ("Trust Replenishment", "Restore trust to $10,000", "DUE 11/15"),
        ],
    },
    ArchetypeConceptSpec {
        id: 27,
        slug: "cpa-tax-advisory-circular-230-engagement-letter-organizer",
        title: "CPA Tax Advisory Engagement Letter (Circular 230), S-Corp Reasonable Comp & Fee Schedule",
        vertical: "Legal, CPA & Financial Advisory",
        vertical_slug: "legal-cpa-advisory",
        target_profession: "CPA Firms, Enrolled Agents (EAs), Fractional CFOs & Tax Strategists",
        primary_keyword: "cpa tax preparation and advisory engagement letter template circular 230",
        secondary_keywords: &["s corp reasonable compensation tax engagement", "aicpa ssts no 1 engagement letter pdf", "quarterly tax projection retainer"],
        demand_signal: "CPAs and Enrolled Agents standardizing fixed-value packaging (Form 1120-S + Form 1040 + S-Corp Reasonable Compensation study + PTET election) under AICPA SSTS standards.",
        statutory_citation: "Treasury Dept. Circular 230 · AICPA SSTS No. 1 · IRC § 7216 Taxpayer Consent Standard",
        default_masthead: "editorial_serif",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_legal_cpa_advisory.png",
        primary_hex: "#0f172a",
        accent_hex: "#059669",
        stamp_label: "AICPA SSTS NO. 1",
        stamp_sublabel: "FIXED-FEE ADVISORY",
        sample_org: "Cumberland Advisory & Certified Public Accountants PLLC",
        sample_counterparty: "Apex Dental Implant Studio PLLC & Dr. K. Patel",
        kpi_metrics: [
            ("2026 ANNUAL ADVISORY FEE", "$6,800.00 FIXED"),
            ("PROJECTED S-CORP SE SAVINGS", "$18,420.00 / YR"),
            ("QBI § 199A DEDUCTION", "$64,500.00 OPTIMIZED"),
            ("RETAINER DEPOSIT DUE", "$3,400.00 (50%)"),
        ],
        line_items: [
            ("TAX-1120S", "Federal Form 1120-S & State Franchise/Excise Return + Schedule K-1 Preparation", "Entity Return", "$2,600.00"),
            ("TAX-1040", "Individual Form 1040, Schedule E Pass-Through & Multi-State Apportionment", "Personal Return", "$1,400.00"),
            ("ADV-COMP", "IRC § 162 S-Corp Reasonable Compensation Market Study & Accountable Plan Setup", "Audit Defense Doc", "$1,600.00"),
            ("ADV-Q4", "Q3/Q4 Proactive Tax Projection, Solo 401(k) Profit-Sharing & Augusta Rule (§ 280A(g))", "Year-End Strategy", "$1,200.00"),
        ],
        chart_rows: [
            ("Estimated Unoptimized Sole-Prop Tax", "$94,200.00", 100, "primary"),
            ("Optimized S-Corp + § 199A Tax Liability", "$75,780.00", 80, "accent"),
            ("Net Annual Tax Reduction (2.7x Fee)", "$18,420.00", 20, "accent"),
        ],
        milestones: [
            ("Engagement Signed", "Circular 230 terms accepted", "11/06/2026"),
            ("Year-End Tax Blueprint", "Payroll & 401(k) adjustments", "12/10/2026"),
            ("Books Closed & Trial Bal", "QBO ledger reconciled", "01/31/2027"),
            ("Form 1120-S e-Filed", "Form 8879 e-sign prior to 3/15", "03/10/2027"),
        ],
    },
    ArchetypeConceptSpec {
        id: 28,
        slug: "small-business-acquisition-loi-quality-of-earnings-sde-addback-schedule",
        title: "Main-Street Business Acquisition LOI & Seller's Discretionary Earnings (SDE) Add-Back Audit",
        vertical: "Legal, CPA & Financial Advisory",
        vertical_slug: "legal-cpa-advisory",
        target_profession: "Search Fund Buyers, SBA 7(a) Acquisition Entrepreneurs, Business Brokers & M&A CPAs",
        primary_keyword: "business acquisition letter of intent loi and sde add back worksheet pdf",
        secondary_keywords: &["quality of earnings qoe sde calculator", "sba 7a business purchase loi template", "cleaning or hvac business valuation worksheet"],
        demand_signal: "r/smallbusiness top viral threads (`BUSINESS SOLD!`, `How much would you pay for this cleaning business?`, `wellness business with 1–2 years left on lease`): buyers verifying SDE add-backs and lease assignment contingencies.",
        statutory_citation: "SBA SOP 50 10 7.1 Cash Flow Verification · Exclusive 45-Day Due Diligence Lockout",
        default_masthead: "swiss_grid",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_legal_cpa_advisory.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "3.1X VERIFIED SDE",
        stamp_sublabel: "SBA 7(a) DSCR: 1.68X",
        sample_org: "Appalachian Main-Street Search Partners LLC",
        sample_counterparty: "Commercial Janitorial Services Inc. (Target Asset Purchase)",
        kpi_metrics: [
            ("VERIFIED TTM SDE", "$284,000.00 / YR"),
            ("PROPOSED ASSET PURCHASE", "$880,000.00 (3.10x SDE)"),
            ("SBA 7(a) DEBT + SELLER NOTE", "80% SBA / 10% Seller / 10% Eq"),
            ("POST-DEBT DSCR COVERAGE", "1.68x (SBA Min: 1.15x)"),
        ],
        line_items: [
            ("SDE-NET", "Reported Tax Return Ordinary Business Income (Verified via IRS Form 4506-C Transcripts)", "2025 Form 1120-S", "$162,400.00"),
            ("ADD-OWN", "Verified Owner W-2 Officer Compensation, Health Insurance & Payroll Taxes (1 Full-Time Owner)", "W-2 Reconciled", "+$98,600.00"),
            ("ADD-DEP", "Non-Cash Depreciation, Section 179 Expense & Interest Add-Backs", "Form 4562", "+$23,000.00"),
            ("DIS-PER", "Disallowed Unverified Personal Travel/Meals Add-Backs Excluded from Quality of Earnings", "Buyer Haircut", "$0.00 ALLOWED"),
        ],
        chart_rows: [
            ("SBA 7(a) Senior Acquisition Loan (80%)", "$704,000.00", 80, "primary"),
            ("Seller Standby Promissory Note (10%)", "$88,000.00", 10, "accent"),
            ("Buyer Cash Equity Injection (10%)", "$88,000.00", 10, "primary"),
        ],
        milestones: [
            ("LOI Counter-Signed", "45-day exclusivity begins", "DAY 0"),
            ("QoE & 4506-C Audit", "Bank statements match tax returns", "DAY 20"),
            ("Lease & Key Customer", "5-yr landlord extension signed", "DAY 32"),
            ("SBA 7(a) Closing", "Asset Purchase Agreement funded", "DAY 45"),
        ],
    },
    ArchetypeConceptSpec {
        id: 29,
        slug: "expert-witness-forensic-valuation-retention-fee-schedule",
        title: "Forensic Accounting & Expert Witness Retention Agreement (FRE 702 / Daubert)",
        vertical: "Legal, CPA & Financial Advisory",
        vertical_slug: "legal-cpa-advisory",
        target_profession: "Forensic CPAs, Real Estate Appraisers (MAI), Structural Engineers & Medical Experts",
        primary_keyword: "expert witness retainer agreement and fee schedule template pdf",
        secondary_keywords: &["forensic valuation expert engagement letter", "deposition and trial testimony minimum fee", "fre 702 daubert independence clause"],
        demand_signal: "Expert witnesses and forensic appraisers requiring non-contingent fee attestations (preserving FRE 702 independence) and half-day/full-day deposition minimums paid in advance.",
        statutory_citation: "Fed. R. Evid. 702 · Fed. R. Civ. P. 26(a)(2)(B) · Non-Contingent Independence Attestation",
        default_masthead: "judicial_caption",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "parcel_lot",
        default_readability: ReadabilityPreset::JudicialFormal,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_legal_cpa_advisory.png",
        primary_hex: "#0f172a",
        accent_hex: "#b45309",
        stamp_label: "FRE 702 INDEPENDENT",
        stamp_sublabel: "NON-CONTINGENT FEE",
        sample_org: "Hargrove Forensic Valuation & MAI Appraisal Group",
        sample_counterparty: "Retaining Counsel: Baker & Miller LLP (Client: Apex Holdings)",
        kpi_metrics: [
            ("INITIAL SECURITY RETAINER", "$7,500.00 (ADVANCE)"),
            ("FORENSIC ANALYSIS RATE", "$375.00 / HOUR"),
            ("DEPOSITION / TRIAL RATE", "$475.00 / HR (4-HR MIN)"),
            ("FEE CONTINGENCY STATUS", "0% (STRICTLY INDEPENDENT)"),
        ],
        line_items: [
            ("EXP-01", "Forensic File Review, Financial Modeling & Rule 26(a)(2)(B) Written Expert Report", "12.0 Hrs @ $375", "$4,500.00"),
            ("EXP-02", "Rebuttal Analysis of Opposing Expert's Discounted Cash Flow & Cap Rate Assumptions", "4.0 Hrs @ $375", "$1,500.00"),
            ("EXP-03", "Half-Day Deposition Testimony Minimum Reserve (Payable 7 Days Prior to Deposition)", "4-Hr Minimum @ $475", "$1,900.00"),
            ("EXP-IND", "Statutory Attestation: Compensation is Strictly Time-Based and Never Contingent on Outcome", "Ethics Rule", "VERIFIED"),
        ],
        chart_rows: [
            ("Initial Retainer Deposit Received", "$7,500.00", 95, "primary"),
            ("Rule 26 Written Report & Rebuttal", "$6,000.00", 76, "primary"),
            ("Deposition Minimum Reserve Balance", "$1,900.00", 24, "accent"),
        ],
        milestones: [
            ("Retainer Executed", "Conflict check cleared", "COMPLETED"),
            ("Rule 26 Report Served", "Signed expert disclosure", "11/15/2026"),
            ("Deposition Prep & Exam", "4-hour minimum reserved", "12/04/2026"),
            ("Chancery Trial Testimony", "Subpoena & trial calendar", "01/18/2027"),
        ],
    },
    ArchetypeConceptSpec {
        id: 30,
        slug: "ria-wealth-management-fiduciary-advisory-fee-schedule-invoice",
        title: "Registered Investment Advisor (RIA) Fiduciary AUM Fee Statement & Q4 Invoice",
        vertical: "Legal, CPA & Financial Advisory",
        vertical_slug: "legal-cpa-advisory",
        target_profession: "Registered Investment Advisors (RIAs), CFP® Planners & Family Offices",
        primary_keyword: "ria wealth management advisory fee invoice and tiered aum schedule pdf",
        secondary_keywords: &["fiduciary investment advisory fee statement", "custodian direct fee deduction notice", "form adv part 2a fee calculation"],
        demand_signal: "Independent RIAs required by state/SEC custody rules to send itemized quarterly fee invoices showing exact average daily AUM, tiered basis-point breakpoints, and custodian deduction authorization.",
        statutory_citation: "Investment Advisers Act of 1940 · SEC Rule 206(4)-2 Custody Fee Itemization · Fiduciary Oath",
        default_masthead: "editorial_serif",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::HighLegibility,
        default_execution_mode: "esign",
        notary_required: false,
        seal_filename: "seal_legal_cpa_advisory.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "100% FIDUCIARY",
        stamp_sublabel: "BLENDED: 0.69% AUM",
        sample_org: "Lookout Fiduciary Wealth Partners LLC (SEC RIA)",
        sample_counterparty: "The Arthur & Evelyn Sterling Revocable Living Trust",
        kpi_metrics: [
            ("BILLABLE Q4 PORTFOLIO AUM", "$2,450,000.00"),
            ("BLENDED ANNUALIZED RATE", "0.694% (69.4 BPS)"),
            ("DAYS IN BILLING QUARTER", "92 / 365 Days (Q4)"),
            ("QUARTERLY FEE DEDUCTION", "$4,284.25 (SCHWAB)"),
        ],
        line_items: [
            ("TIER-01", "First $1,000,000 of Managed Household Assets @ 0.85% Annualized (92/365 Days Prorated)", "$1,000,000 Tier", "$2,142.47"),
            ("TIER-02", "Next $1,450,000 of Managed Household Assets ($1M–$3M Tier) @ 0.60% Annualized", "$1,450,000 Tier", "$2,192.88"),
            ("TAX-LOSS", "Q4 Automated Tax-Loss Harvesting & Municipal Bond Ladder Rebalancing (Included $0)", "$14,200 Tax Alpha", "INCLUDED"),
            ("LOY-CRD", "Founding Client Relationship Fee Credit Applied to Custodian Debited Total", "Fee Adjustment", "-$51.10"),
        ],
        chart_rows: [
            ("Tier 1 ($0 – $1.0M @ 85 bps)", "$2,142.47", 50, "primary"),
            ("Tier 2 ($1.0M – $2.45M @ 60 bps)", "$2,141.78", 50, "accent"),
            ("Total Q4 Fee (0.175% of Portfolio)", "$4,284.25", 18, "accent"),
        ],
        milestones: [
            ("Quarter-End Valuation", "Charles Schwab custodian feed", "09/30/2026"),
            ("Itemized Notice Sent", "Delivered to Client 10d prior", "10/05/2026"),
            ("Custodian Fee Debit", "Deducted from IRA/Trust pro-rata", "10/16/2026"),
            ("Year-End Tax Review", "Roth conversion & RMD audit", "11/18/2026"),
        ],
    },

    // =========================================================================
    // VERTICAL 7: MEDICAL, DENTAL, CLINICAL & WELLNESS PRACTICES (31..=35)
    // =========================================================================
    ArchetypeConceptSpec {
        id: 31,
        slug: "out-of-network-medical-superbill-cpt-icd10-reimbursement-statement",
        title: "Out-of-Network Medical & Physical Therapy Superbill (CPT / ICD-10 / NPI)",
        vertical: "Medical, Dental & Clinical Practices",
        vertical_slug: "medical-clinical",
        target_profession: "Concierge Physicians, Physical Therapists, Psychologists & Out-of-Network Clinics",
        primary_keyword: "medical superbill template pdf with cpt and icd 10 codes",
        secondary_keywords: &["out of network insurance reimbursement superbill", "physical therapy superbill receipt", "hsa fsa qualified medical receipt"],
        demand_signal: "Cash-pay and concierge clinics needing clean, insurer-accepted superbills with NPI Type 1/2, Tax ID, Place of Service (POS 11/02), ICD-10-CM pointers, and CPT modifiers.",
        statutory_citation: "CMS-1500 Data Element Alignment · NPI Registry Type 1/2 · HSA/FSA IRC § 213(d) Receipt",
        default_masthead: "executive_split",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::HighLegibility,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_medical_clinical.png",
        primary_hex: "#0f766e",
        accent_hex: "#0284c7",
        stamp_label: "PAID BY PATIENT",
        stamp_sublabel: "HSA / OON ELIGIBLE",
        sample_org: "Lookout Performance & Orthopedic Physical Therapy PLLC",
        sample_counterparty: "Patient: Eleanor M. Vance (DOB: 04/18/1979 · POS: 11)",
        kpi_metrics: [
            ("RENDERING NPI / TAX ID", "NPI 1487902145 · 84-3910422"),
            ("PRIMARY ICD-10 DIAGNOSIS", "M54.50 · M25.511"),
            ("TOTAL CHARGES PAID", "$265.00 (PAID IN FULL)"),
            ("PATIENT REIMBURSEMENT DUE", "$265.00 DIRECT TO INSURED"),
        ],
        line_items: [
            ("CPT-97162", "Physical Therapy Evaluation: Moderate Complexity (45 Min One-on-One Clinical Eval)", "1 Unit · Dx: A,B", "$145.00"),
            ("CPT-97110", "Therapeutic Exercise to Develop Strength, Endurance & Range of Motion (GP Modifier)", "2 Units (30m) · Dx: A", "$80.00"),
            ("CPT-97140", "Manual Therapy Techniques (Joint Mobilization & Myofascial Release — 59/GP Mod)", "1 Unit (15m) · Dx: A", "$40.00"),
            ("PAY-HSA", "Patient Out-of-Pocket Payment Settled via HSA Visa Card #4812 (Assign Benefits to Patient)", "Paid at Visit", "-$265.00"),
        ],
        chart_rows: [
            ("Total Billed Clinical Procedure Charges", "$265.00", 100, "primary"),
            ("Patient Upfront HSA/Card Payment", "$265.00", 100, "accent"),
            ("Remaining Clinic Balance Due", "$0.00", 4, "primary"),
        ],
        milestones: [
            ("Clinical Visit Completed", "POS 11 In-Office Encounter", "11/06/2026"),
            ("Patient Paid in Full", "HSA receipt & superbill issued", "11/06/2026"),
            ("Member Portal Upload", "Submitted to BCBS OON Claims", "PATIENT"),
            ("OON Deductible Credit", "Applied to out-of-network max", "INSURER"),
        ],
    },
    ArchetypeConceptSpec {
        id: 32,
        slug: "no-surprises-act-good-faith-estimate-gfe-self-pay-medical",
        title: "No Surprises Act Good Faith Estimate (GFE) for Uninsured / Self-Pay Patients",
        vertical: "Medical, Dental & Clinical Practices",
        vertical_slug: "medical-clinical",
        target_profession: "Outpatient Surgery Centers, Behavioral Health Providers, Specialists & Imaging Clinics",
        primary_keyword: "no surprises act good faith estimate template pdf 45 cfr 149.610",
        secondary_keywords: &["self pay patient good faith estimate form", "400 dollar patient provider dispute resolution notice", "uninsured medical cost estimate"],
        demand_signal: "Federal compliance requirement under 45 C.F.R. § 149.610 for all healthcare providers seeing self-pay/uninsured patients, including the mandatory $400 dispute-resolution notice.",
        statutory_citation: "Public Health Service Act § 2799B-6 · 45 C.F.R. § 149.610 ($400 PPDR Threshold Notice)",
        default_masthead: "executive_split",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::HighLegibility,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_medical_clinical.png",
        primary_hex: "#0f766e",
        accent_hex: "#059669",
        stamp_label: "45 CFR § 149.610",
        stamp_sublabel: "$400 PPDR PROTECTED",
        sample_org: "Chattanooga Outpatient Orthopaedic & Imaging Center",
        sample_counterparty: "Self-Pay Patient: Thomas R. Gentry (Scheduled: 11/18/2026)",
        kpi_metrics: [
            ("TOTAL GOOD FAITH ESTIMATE", "$1,485.00"),
            ("PPDR DISPUTE THRESHOLD", "+$400.00 OVER ESTIMATE"),
            ("ESTIMATE DELIVERY WINDOW", "Within 1 Business Day"),
            ("VALIDITY PERIOD", "12 Months From Issue"),
        ],
        line_items: [
            ("CPT-73721", "MRI Lower Extremity Joint (Right Knee) Without Contrast — 3.0 Tesla High-Field Scanner", "Technical Component", "$690.00"),
            ("CPT-73721-26", "Board-Certified Musculoskeletal Radiologist Diagnostic Interpretation & Written Report", "Professional Fee", "$245.00"),
            ("CPT-20611", "Ultrasound-Guided Major Joint Aspiration / Visco-Supplementation Injection (Right Knee)", "Procedure + Guidance", "$365.00"),
            ("HCPCS-J7325", "Hyaluronan Derivative Intra-Articular Injectable Supply (Single-Dose Prefilled Syringe)", "Medication Supply", "$185.00"),
        ],
        chart_rows: [
            ("Hospital Chargemaster List Price", "$4,820.00", 100, "primary"),
            ("Prompt Self-Pay Good Faith Estimate", "$1,485.00", 31, "accent"),
            ("Patient Self-Pay Discount Savings", "$3,335.00", 69, "accent"),
        ],
        milestones: [
            ("Appointment Scheduled", "Booked >= 3 business days out", "11/06/2026"),
            ("Written GFE Delivered", "Provided via portal & PDF", "11/06/2026"),
            ("Patient Consent Signed", "Self-pay election confirmed", "11/18/2026"),
            ("Price Lock Guarantee", "Protected within $400 window", "GUARANTEED"),
        ],
    },
    ArchetypeConceptSpec {
        id: 33,
        slug: "dental-implant-prosthodontic-treatment-plan-insurance-copay-estimate",
        title: "Dental Implant & Prosthodontic Treatment Plan + Insurance Copay Breakdown",
        vertical: "Medical, Dental & Clinical Practices",
        vertical_slug: "medical-clinical",
        target_profession: "General Dentists, Prosthodontists, Oral Surgeons & Orthodontic Practices",
        primary_keyword: "dental treatment plan and patient financial agreement template pdf",
        secondary_keywords: &["cdt code dental implant estimate", "dental insurance maximum and copay worksheet", "phased restorative treatment plan"],
        demand_signal: "Dental treatment coordinators needing clear visual separation between gross CDT fees, PPO contracted write-offs, estimated insurance coverage, and phased patient copays.",
        statutory_citation: "ADA CDT-2026 Procedure Codes · PPO Contracted Fee Schedule · Informed Financial Consent",
        default_masthead: "executive_split",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::HighLegibility,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_medical_clinical.png",
        primary_hex: "#0284c7",
        accent_hex: "#059669",
        stamp_label: "TREATMENT PLAN",
        stamp_sublabel: "CDT-2026 VERIFIED",
        sample_org: "Riverview Prosthodontics & Implant Dentistry DDS",
        sample_counterparty: "Patient: Margaret L. Dawson (Tooth #19 Implant Restoration)",
        kpi_metrics: [
            ("UCR STANDARD FEE TOTAL", "$5,480.00"),
            ("PPO CONTRACTED WRITE-OFF", "-$1,090.00"),
            ("ESTIMATED DELTA PPO BENEFIT", "-$1,750.00"),
            ("NET PATIENT PORTION (3 VISITS)", "$2,640.00"),
        ],
        line_items: [
            ("D6010", "Phase 1 (Visit 1): Surgical Placement of Endosteal Titanium Implant Body — Tooth #19", "PPO Allowed: $2,150", "$1,075.00 Copay"),
            ("D7953", "Phase 1 (Visit 1): Bone Replacement Graft for Ridge Preservation at Implant Site", "PPO Allowed: $540", "$270.00 Copay"),
            ("D6057", "Phase 2 (Month 4): Custom CAD/CAM Titanium Milled Implant Abutment (Includes Torque)", "PPO Allowed: $680", "$445.00 Copay"),
            ("D6058", "Phase 2 (Month 4): Abutment-Supported Monolithic Zirconia Crown — Tooth #19", "PPO Allowed: $1,020", "$850.00 Copay"),
        ],
        chart_rows: [
            ("PPO Allowed Contracted Total", "$4,390.00", 100, "primary"),
            ("Estimated Insurance Benefit Paid", "$1,750.00", 40, "primary"),
            ("Net Patient Copay (Split Over 4 Mos)", "$2,640.00", 60, "accent"),
        ],
        milestones: [
            ("CBCT 3D Scan & Plan", "Bone density & guide verified", "COMPLETED"),
            ("Visit 1: Implant & Graft", "$1,345 due on surgical date", "MONTH 1"),
            ("Osseointegration Check", "12-week torque stability test", "MONTH 3"),
            ("Visit 2: Zirconia Crown", "$1,295 due at final seat", "MONTH 4"),
        ],
    },
    ArchetypeConceptSpec {
        id: 34,
        slug: "concierge-direct-primary-care-dpc-membership-enrollment-agreement",
        title: "Direct Primary Care (DPC) & Longevity Clinic Membership Agreement",
        vertical: "Medical, Dental & Clinical Practices",
        vertical_slug: "medical-clinical",
        target_profession: "Direct Primary Care (DPC) Physicians, Longevity Clinics & Functional Medicine",
        primary_keyword: "direct primary care dpc patient membership agreement template pdf",
        secondary_keywords: &["concierge medicine retainer contract", "dpc non-insurance statutory disclosure", "wholesale lab and imaging pricing schedule"],
        demand_signal: "DPC and longevity clinics requiring state-mandated disclosures that the monthly membership is not health insurance, paired with transparent wholesale lab/DEXA pricing.",
        statutory_citation: "State Direct Primary Care Medical Home Act · IRC § 223(c)(1)(B) HSA Eligibility Disclosure",
        default_masthead: "editorial_serif",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::HighLegibility,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_medical_clinical.png",
        primary_hex: "#0f766e",
        accent_hex: "#b45309",
        stamp_label: "ZERO COPAYS",
        stamp_sublabel: "DIRECT PRIMARY CARE",
        sample_org: "Lookout Longevity & Direct Primary Care MD PLLC",
        sample_counterparty: "Member: Jonathan H. Vance (Adult Individual Tier)",
        kpi_metrics: [
            ("MONTHLY DPC MEMBERSHIP", "$150.00 / MONTH"),
            ("OFFICE & TELEHEALTH COPAY", "$0.00 UNLIMITED"),
            ("PANEL CAPACITY CAP", "Max 400 Patients / MD"),
            ("WHOLESALE LAB SAVINGS", "85%–92% Below Retail"),
        ],
        line_items: [
            ("DPC-MEM", "Monthly Direct Primary Care Membership: Same/Next-Day 60-Min Visits, Direct Physician Text/Call", "$0 Visit Copay", "$150.00/MO"),
            ("LAB-PNL", "Included Annual Executive Biomarker Panel (CBC, CMP, ApoB, Lp(a), HbA1c, hs-CRP, Thyroid, Vit D)", "1x Annual Included", "$0.00"),
            ("DEXA-01", "Dual-Energy X-Ray Absorptiometry (DEXA) Body Composition + Visceral Fat + VO2 Max Baseline", "Member Wholesale", "$95.00"),
            ("DIS-INS", "Statutory Notice: This Agreement is a Direct Retainer and is NOT Health Insurance Coverage", "State DPC Statute", "DISCLOSED"),
        ],
        chart_rows: [
            ("Retail Hospital Lab & Biomarker Cost", "$1,420.00", 100, "primary"),
            ("DPC Member Wholesale Lab Cost", "$0.00 (Included)", 8, "accent"),
            ("Annual Membership Investment (12 Mo)", "$1,800.00", 75, "primary"),
        ],
        milestones: [
            ("Enrollment Executed", "ACH card on file authorized", "DAY 1"),
            ("Fasting Blood Draw", "In-clinic phlebotomy (15m)", "DAY 5"),
            ("90-Min Deep Dive MD", "Full longevity & metabolic plan", "DAY 12"),
            ("Quarterly Check-Ins", "Direct MD text & follow-ups", "ONGOING"),
        ],
    },
    ArchetypeConceptSpec {
        id: 35,
        slug: "hipaa-business-associate-agreement-baa-clinical-data-security-addendum",
        title: "HIPAA Business Associate Agreement (BAA) & ePHI Encryption Security Schedule",
        vertical: "Medical, Dental & Clinical Practices",
        vertical_slug: "medical-clinical",
        target_profession: "Healthcare SaaS Vendors, Medical Billing Companies, EHR Consultants & Covered Entities",
        primary_keyword: "hipaa business associate agreement baa template pdf 45 cfr 164.504",
        secondary_keywords: &["ephi security rule safeguard schedule", "hitech breach notification timeline clause", "subcontractor baa flow down agreement"],
        demand_signal: "Healthcare practices and health-tech vendors needing a clear, non-bloated HIPAA/HITECH BAA specifying breach notification timelines, AES-256/TLS 1.3 encryption, and PHI return/destruction.",
        statutory_citation: "45 C.F.R. §§ 164.308, 164.312 & 164.504(e) · HITECH Act 42 U.S.C. § 17932",
        default_masthead: "judicial_caption",
        default_kpi_style: "dark_terminal",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_medical_clinical.png",
        primary_hex: "#0f172a",
        accent_hex: "#0f766e",
        stamp_label: "HIPAA / HITECH",
        stamp_sublabel: "45 CFR § 164.504(e)",
        sample_org: "Covered Entity: Tennessee Valley Orthopaedic Group PLLC",
        sample_counterparty: "Business Associate: MedVault Cloud Imaging Inc.",
        kpi_metrics: [
            ("ENCRYPTION AT REST / TRANSIT", "AES-256-GCM / TLS 1.3"),
            ("BREACH NOTICE SLA", "Within 5 Business Days"),
            ("SUBCONTRACTOR FLOW-DOWN", "100% Written BAA Req"),
            ("PHI AI TRAINING POLICY", "STRICTLY PROHIBITED"),
        ],
        line_items: [
            ("SEC-312A", "45 C.F.R. § 164.312(a) Access Control: Unique User IDs, Hardware MFA & Automatic 15-Min Logoff", "Technical Safeguard", "MANDATORY"),
            ("SEC-312B", "45 C.F.R. § 164.312(b) Audit Controls: Immutable 6-Year Access Logs of All ePHI Reads/Exports", "6-Yr Retention", "MANDATORY"),
            ("NTC-410", "45 C.F.R. § 164.410 Breach Notification: Immediate Notice (Max 5 Days) of Any Unsecured PHI Incident", "Strict Notice Window", "MANDATORY"),
            ("TRM-504", "45 C.F.R. § 164.504(e)(2)(ii)(J) Termination: Cryptographic Erasure (NIST 800-88) Within 30 Days", "Post-Termination", "MANDATORY"),
        ],
        chart_rows: [
            ("Statutory HIPAA Max Notice Window (60d)", "60 Calendar Days", 100, "primary"),
            ("Contractual BAA Rapid Notice SLA (5d)", "5 Business Days", 12, "accent"),
            ("ePHI Encryption & Audit Log Coverage", "100% Enforced", 100, "accent"),
        ],
        milestones: [
            ("BAA Counter-Signed", "Prior to any ePHI transmission", "PREREQUISITE"),
            ("SOC2 + HIPAA Audit", "Annual third-party attestation", "ANNUAL"),
            ("Access Log Review", "Automated anomaly alerting", "CONTINUOUS"),
            ("NIST 800-88 Wipe", "Upon contract termination", "EXIT GATE"),
        ],
    },

    // =========================================================================
    // VERTICAL 8: LOGISTICS, FREIGHT, AUTOMOTIVE & MANUFACTURING (36..=40)
    // =========================================================================
    ArchetypeConceptSpec {
        id: 36,
        slug: "uniform-straight-bill-of-lading-bol-freight-broker-rate-confirmation",
        title: "Uniform Straight Bill of Lading (BOL) & Freight Broker Load Rate Confirmation",
        vertical: "Logistics, Fleet & Manufacturing",
        vertical_slug: "logistics-freight",
        target_profession: "Freight Brokers, 3PLs, Motor Carriers, Owner-Operators & Warehouse Shippers",
        primary_keyword: "uniform straight bill of lading bol and carrier rate confirmation template pdf",
        secondary_keywords: &["freight broker rate confirmation sheet", "nmfc freight class bill of lading", "detention and layover accessorial schedule"],
        demand_signal: "Freight brokers and carriers demanding combined Rate Confirmations + Uniform BOLs with clear detention rates ($75/hr after 2 hours free), Section 7 non-recourse clauses, and seal verification.",
        statutory_citation: "49 U.S.C. § 14706 (Carmack Amendment) · Uniform Straight BOL · Section 7 Non-Recourse",
        default_masthead: "trade_banner",
        default_kpi_style: "dark_terminal",
        default_table_style: "dark_header",
        default_diagram_kind: "vehicle_inspection",
        default_readability: ReadabilityPreset::ExecutiveCompact,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_logistics_freight.png",
        primary_hex: "#0f172a",
        accent_hex: "#d97706",
        stamp_label: "SEAL INTACT",
        stamp_sublabel: "BOL #CH-884920",
        sample_org: "Volunteer State 3PL & Freight Brokerage LLC (MC #948102)",
        sample_counterparty: "Carrier: Blue Ridge Dedicated Transport LLC (DOT #3104892)",
        kpi_metrics: [
            ("LANE / LOAD DISTANCE", "Chattanooga, TN → Dallas, TX (812 MI)"),
            ("EQUIPMENT / TRAILER SEAL", "53' Dry Van · Seal #SL-904812"),
            ("TOTAL PALLETS / WEIGHT", "24 Pallets · 41,800 LBS"),
            ("ALL-IN CARRIER LINEHAUL", "$2,480.00 ($3.05 / MILE)"),
        ],
        line_items: [
            ("LH-812", "Linehaul Rate: Chattanooga, TN (37410) to Dallas, TX (75247) — Exclusive Use 53' Air-Ride Van", "812 Loaded Miles", "$2,150.00"),
            ("FSC-01", "DOE National Diesel Fuel Surcharge Pass-Through ($0.34 / Mile Indexed)", "812 Miles @ $0.34", "$276.08"),
            ("ACC-DET", "Shipper Loading Detention (1.0 Billable Hour After 2-Hour Free Window @ $53.92)", "Time-Stamped ELD", "$53.92"),
            ("CMK-100", "Carmack Cargo Liability Coverage Verified ($100,000 Primary Cargo Certificate on File)", "COI Verified", "ACTIVE"),
        ],
        chart_rows: [
            ("Base Linehaul Compensation", "$2,150.00", 87, "primary"),
            ("Indexed Fuel Surcharge + Detention", "$330.00", 18, "accent"),
            ("Total Carrier Settlement (QuickPay 48h)", "$2,480.00", 100, "accent"),
        ],
        milestones: [
            ("Rate Con Signed", "MacroPoint GPS tracking active", "11/05 08:00"),
            ("Shipper Loaded & Sealed", "24 skids · Seal #SL-904812", "11/05 11:30"),
            ("Consignee Delivery", "Seal broken by dock supervisor", "11/06 16:15"),
            ("Clean POD Uploaded", "Settled via 48-hr ACH", "11/08 10:00"),
        ],
    },
    ArchetypeConceptSpec {
        id: 37,
        slug: "automotive-diesel-fleet-repair-order-multi-point-inspection-invoice",
        title: "Automotive & Heavy-Duty Fleet Repair Order (RO) + Multi-Point Diagnostic Invoice",
        vertical: "Logistics, Fleet & Manufacturing",
        vertical_slug: "logistics-freight",
        target_profession: "Independent Auto Repair Shops, Diesel Fleet Maintenance Centers & Mobile Mechanics",
        primary_keyword: "automotive repair order and vehicle inspection invoice template pdf",
        secondary_keywords: &["mechanic work order with parts and labor breakdown", "dot annual vehicle inspection repair sheet", "statutory mechanic storage lien notice"],
        demand_signal: "Auto and diesel shops needing clean separation of OEM/aftermarket part numbers, Mitchell1/ALLDATA flat-rate labor hours, core charges, and state statutory mechanic's lien notices.",
        statutory_citation: "T.C.A. § 66-19-101 Garagekeeper's Lien Notice · 49 C.F.R. § 396.17 DOT Inspection Standard",
        default_masthead: "trade_banner",
        default_kpi_style: "dark_terminal",
        default_table_style: "striped",
        default_diagram_kind: "vehicle_inspection",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_logistics_freight.png",
        primary_hex: "#0f172a",
        accent_hex: "#059669",
        stamp_label: "ROAD TEST PASS",
        stamp_sublabel: "12MO / 12K WARRANTY",
        sample_org: "Precision Fleet & European Automotive Works LLC",
        sample_counterparty: "Fleet Account: Summit Beverage Distribution (Unit #F-204)",
        kpi_metrics: [
            ("VIN / ODOMETER IN-OUT", "1FTYR2CM8KKA48192 · 118,412 MI"),
            ("TOTAL FLAT-RATE LABOR", "5.5 Hrs @ $145/hr ($797.50)"),
            ("OEM PARTS & FLUIDS", "$684.50 (WARRANTIED)"),
            ("TOTAL REPAIR ORDER DUE", "$1,548.40"),
        ],
        line_items: [
            ("BRK-FRT", "Replace Front & Rear Ceramic Brake Pads, High-Carbon Coated Rotors & Brake Fluid Flush", "3.0 Hrs + OEM Kit", "$842.00"),
            ("ALT-220", "Replace 220-Amp Heavy-Duty Alternator + Serpentine Belt & Tensioner Assembly (Reman Core Ret.)", "1.8 Hrs + Denso OEM", "$518.00"),
            ("DIAG-OBD", "Bi-Directional CAN-Bus Charging Ripple Test & Dynamic ABS Road Calibration", "0.7 Hrs Master Tech", "$122.00"),
            ("ENV-TAX", "EPA Hazardous Fluid Disposal Fee ($28.40) + Applicable State Shop Supply Tax ($38.00)", "Statutory Fees", "$66.40"),
        ],
        chart_rows: [
            ("ASE Master Technician Labor (5.5 Hrs)", "$797.50", 52, "primary"),
            ("OEM Parts, Rotors & Alternator Assembly", "$684.50", 44, "accent"),
            ("EPA Disposal & Tax ($66.40)", "$66.40", 8, "primary"),
        ],
        milestones: [
            ("Digital Vehicle Inspect", "Brake pad photos texted", "08:30 AM"),
            ("Customer Approval", "$1,548.40 estimate authorized", "09:05 AM"),
            ("Torque & Road Test", "Lug nuts torqued to 148 ft-lbs", "02:15 PM"),
            ("Keys Released", "12-mo/12k-mi nationwide warr", "03:00 PM"),
        ],
    },
    ArchetypeConceptSpec {
        id: 38,
        slug: "precision-cnc-machining-manufacturing-rfq-quote-first-article-inspection",
        title: "Precision CNC Machining Quote, BOM Tier Matrix & AS9102 First Article Summary",
        vertical: "Logistics, Fleet & Manufacturing",
        vertical_slug: "logistics-freight",
        target_profession: "CNC Machine Shops, Sheet Metal Fabricators, Contract Manufacturers & Aerospace Suppliers",
        primary_keyword: "cnc machining quote and first article inspection report template pdf",
        secondary_keywords: &["manufacturing rfq volume break quote", "as9102 fair inspection summary", "dfars material heat lot traceability"],
        demand_signal: "Machine shops and contract fabricators needing quantity price-break tables (10 / 50 / 250 / 1,000 pcs), NRE fixture amortization, and mill test report (MTR) heat-lot traceability.",
        statutory_citation: "ISO 9001:2015 / AS9102 Rev C First Article · ASME Y14.5 GD&T · DFARS 252.225-7009",
        default_masthead: "swiss_grid",
        default_kpi_style: "dark_terminal",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::ExecutiveCompact,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_logistics_freight.png",
        primary_hex: "#0f172a",
        accent_hex: "#0284c7",
        stamp_label: "ISO 9001 / AS9100",
        stamp_sublabel: "CMM TOLERANCE ±.0005\"",
        sample_org: "Tennessee Valley 5-Axis Precision Machining Corp.",
        sample_counterparty: "AeroDynamics Propulsion Systems Inc. (RFQ #AP-2026-88)",
        kpi_metrics: [
            ("PART NO. / REVISION", "PN-4409-B (REV D · STEP/PDF)"),
            ("ALLOY & FINISH SPEC", "7075-T651 Al · Type III Hard Anodize"),
            ("CRITICAL GD&T TOLERANCE", "±0.0005\" True Position (CMM)"),
            ("PRODUCTION LOT (250 PCS)", "$14,850.00 ($59.40 / EA)"),
        ],
        line_items: [
            ("NRE-CAM", "One-Time 5-Axis Mastercam Programming, Custom Soft-Jaws & Tombstone Fixture Tooling", "1 Lot NRE Setup", "$1,600.00"),
            ("QTY-250", "5-Axis CNC Milled Actuator Housing (PN-4409-B Rev D, Certified Domestic 7075-T651 Billet)", "250 Pcs @ $48.20", "$12,050.00"),
            ("FIN-ANO", "MIL-A-8625 Type III Class 2 Black Hardcoat Anodize + Thread Masking (NADCAP Approved)", "250 Pcs @ $4.80", "$1,200.00"),
            ("QA-FAIR", "Zeiss Contura CMM Inspection Report, AS9102 Form 1–3 FAIR & Certified Mill Test Reports", "Included QA Pack", "$0.00"),
        ],
        chart_rows: [
            ("Prototype Run (25 Pcs @ $118.00/ea)", "$2,950.00", 45, "primary"),
            ("Selected Production Lot (250 Pcs @ $59.40/ea)", "$14,850.00", 88, "accent"),
            ("Volume Blanket Order (1,000 Pcs @ $41.10/ea)", "$41,100.00", 100, "primary"),
        ],
        milestones: [
            ("PO & STEP Model Lock", "Rev D CAD hash pinned", "DAY 1"),
            ("Domestic Billet Cert", "Kaiser 7075 Heat Lot #K9104", "DAY 4"),
            ("First Article CMM Pass", "AS9102 42/42 dims in spec", "DAY 11"),
            ("Lot Shipped + MTRs", "250 pcs vacuum-packed", "DAY 18"),
        ],
    },
    ArchetypeConceptSpec {
        id: 39,
        slug: "commercial-equipment-rental-heavy-machinery-dry-lease-agreement",
        title: "Heavy Equipment & Construction Machinery Rental Agreement + Hour-Meter Log",
        vertical: "Logistics, Fleet & Manufacturing",
        vertical_slug: "logistics-freight",
        target_profession: "Equipment Rental Yards, Excavation Contractors, Crane/Lift Fleets & Ag Dealers",
        primary_keyword: "heavy equipment rental agreement and condition checklist template pdf",
        secondary_keywords: &["excavator skid steer rental contract", "equipment hour meter overage log", "rental equipment damage waiver cdw"],
        demand_signal: "Equipment rental yards and site contractors needing ironclad hour-meter logs (8 hrs/day, 40 hrs/week, 160 hrs/month), diesel fuel refill rates, and COI loss-payee verification.",
        statutory_citation: "UCC Article 2A Commercial Equipment Lease · 160-Hr Monthly Shift Cap · Loss Payee COI",
        default_masthead: "trade_banner",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "vehicle_inspection",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_logistics_freight.png",
        primary_hex: "#0f172a",
        accent_hex: "#d97706",
        stamp_label: "COI ON FILE",
        stamp_sublabel: "HOUR METER: 1,412.4",
        sample_org: "Cumberland Heavy Equipment & Trench Rental LLC",
        sample_counterparty: "Lessee: Grading & Utilities Southeast Inc. (Job: Hwy 58)",
        kpi_metrics: [
            ("MACHINE / SERIAL NO.", "CAT 315 GC Excavator · #CAT0315G8912"),
            ("HOUR METER OUT / CAP", "1,412.4 Hrs (Max 160 Hrs/28d)"),
            ("REPLACEMENT INSURED VAL", "$195,000.00 COI VERIFIED"),
            ("28-DAY RENTAL TOTAL", "$5,495.00"),
        ],
        line_items: [
            ("RNT-315", "28-Day Monthly Rental: Caterpillar 315 GC Hydraulic Excavator w/ 36\" Heavy-Duty Trench Bucket", "160 Engine Hrs Included", "$4,450.00"),
            ("ATT-THM", "Hydraulic Pin-Grabber Coupler & Progressive Link Thumb Attachment", "28-Day Attachment", "$420.00"),
            ("DEL-LOW", "Round-Trip Lowboy Heavy Haul Mobilization & Pickup (Within 35-Mile Radius)", "2 Ways @ $225", "$450.00"),
            ("ENV-FEE", "Environmental Fluid & Telematics GPS Tracking Fee (Customer COI Waives 14% Damage Fee)", "COI Waived CDW", "$175.00"),
        ],
        chart_rows: [
            ("Daily Rate Equivalent (20 Days @ $495/d)", "$9,900.00", 100, "primary"),
            ("Weekly Rate Equivalent (4 Wks @ $1,680/wk)", "$6,720.00", 68, "primary"),
            ("Contracted 28-Day Monthly Rate (Saves 44%)", "$5,495.00", 55, "accent"),
        ],
        milestones: [
            ("COI & Rental Signed", "$195k Inland Marine verified", "11/06 07:00"),
            ("Lowboy Drop & Inspect", "100% Diesel · 1,412.4 Hrs", "11/06 08:30"),
            ("28-Day Cycle End", "Off-rent call required", "12/04/2026"),
            ("Check-In Inspection", "Tracks, glass, hours & fuel", "RETURN"),
        ],
    },
    ArchetypeConceptSpec {
        id: 40,
        slug: "international-commercial-invoice-packing-list-hs-tariff-origin-certificate",
        title: "International Commercial Invoice, Packing List & USMCA / HS Tariff Schedule",
        vertical: "Logistics, Fleet & Manufacturing",
        vertical_slug: "logistics-freight",
        target_profession: "Exporters, Importers, Customs Brokers, Freight Forwarders & Industrial Distributors",
        primary_keyword: "international commercial invoice and packing list with hs code template pdf",
        secondary_keywords: &["incoterms 2020 commercial export invoice", "usmca certificate of origin form", "harmonized tariff schedule customs invoice"],
        demand_signal: "Manufacturers and wholesale exporters requiring CBP/customs-compliant commercial invoices with 10-digit Schedule B / HS tariff codes, Incoterms® 2020, net/gross weights, and country of origin.",
        statutory_citation: "19 C.F.R. § 141.86 Customs Invoice Contents · Incoterms® 2020 · USMCA Origin Certification",
        default_masthead: "swiss_grid",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::ExecutiveCompact,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_logistics_freight.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "USMCA ORIGIN: US",
        stamp_sublabel: "INCOTERMS:FCA CHATT",
        sample_org: "Exporter: Southern Flow Control Valves Inc. (EIN 62-1849201)",
        sample_counterparty: "Consignee: Monterrey Industrial Piping S.A. de C.V. (Mexico)",
        kpi_metrics: [
            ("INCOTERMS® 2020 RULE", "FCA Chattanooga, TN USA"),
            ("AES / ITN FILING NO.", "X20261106849201"),
            ("GROSS / NET WEIGHT", "2,840 KG Gross / 2,610 KG Net"),
            ("TOTAL COMMERCIAL VALUE", "$68,450.00 USD"),
        ],
        line_items: [
            ("HS-8481.80", "6\" ANSI Class 300 Stainless Steel Automated Ball Valves (Schedule B: 8481.80.3090 · Origin: US)", "40 Units @ $1,120", "$44,800.00"),
            ("HS-8537.10", "NEMA 4X Pneumatic Valve Actuators w/ 24VDC Solenoid Positioners (Schedule B: 8537.10.9170 · US)", "40 Units @ $485", "$19,400.00"),
            ("HS-4016.93", "PTFE/Viton High-Temperature Replacement Seat & Stem Seal Kits (Schedule B: 4016.93.5050 · US)", "40 Kits @ $85", "$3,400.00"),
            ("CRT-ISPM", "ISPM-15 Heat-Treated Export Crating, VCI Corrosion Wrap & Inland Drayage to Forwarder", "6 Export Crates", "$850.00"),
        ],
        chart_rows: [
            ("HS 8481.80 Stainless Valves (Origin: US)", "$44,800.00", 65, "primary"),
            ("HS 8537.10 Pneumatic Actuators (Origin: US)", "$19,400.00", 28, "accent"),
            ("Seal Kits & ISPM-15 Export Packaging", "$4,250.00", 12, "primary"),
        ],
        milestones: [
            ("AES EEI Filed", "ITN #X20261106849201 issued", "CLEARED"),
            ("ISPM-15 Stamped", "6 crates weighed & sealed", "VERIFIED"),
            ("Laredo Border Cross", "USMCA 0% duty qualification", "IN TRANSIT"),
            ("Consignee Delivery", "Monterrey plant receipt", "ETA 11/10"),
        ],
    },

    // =========================================================================
    // VERTICAL 9: HOME INSPECTION, APPRAISAL, ENVIRONMENTAL & SOLAR (41..=45)
    // =========================================================================
    ArchetypeConceptSpec {
        id: 41,
        slug: "residential-home-inspection-executive-defect-repair-request-summary",
        title: "Residential Home Inspection Defect Summary & Buyer Repair Request Addendum",
        vertical: "Inspection, Appraisal & Clean Energy",
        vertical_slug: "solar-inspection",
        target_profession: "Licensed Home Inspectors (ASHI/InterNACHI), Buyer's Agents & Structural Inspectors",
        primary_keyword: "home inspection summary report and repair request addendum template pdf",
        secondary_keywords: &["ashi internachi inspection defect summary", "real estate repair credit vs seller repair worksheet", "safety hazard inspection report"],
        demand_signal: "Homebuyers, Realtors, and inspectors needing a concise 1-page Executive Defect & Cost-to-Cure Matrix extracted from 70-page inspection PDFs to negotiate seller closing credits.",
        statutory_citation: "ASHI / InterNACHI Standards of Practice · Buyer Inspection Contingency Resolution",
        default_masthead: "executive_split",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "roof_pitch",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_solar_inspection.png",
        primary_hex: "#0f172a",
        accent_hex: "#dc2626",
        stamp_label: "4 SAFETY DEFECTS",
        stamp_sublabel: "TN LIC #HI-4819",
        sample_org: "Lookout Home & Structural Inspection Services LLC",
        sample_counterparty: "Buyers: Nathan & Maya Brooks — 814 Crestwood Dr",
        kpi_metrics: [
            ("SYSTEMS INSPECTED", "10 Core Systems (2,840 SF)"),
            ("MAJOR / SAFETY DEFECTS", "4 Priority Action Items"),
            ("ESTIMATED COST TO CURE", "$7,450.00 – $8,900.00"),
            ("REQUESTED SELLER CREDIT", "$7,800.00 AT CLOSING"),
        ],
        line_items: [
            ("DEF-ELEC", "[SAFETY] Federal Pacific (FPE) Stab-Lok Sub-Panel in Garage: Known Breaker Trip Failure Hazard", "Master Elec Quote", "$2,650.00"),
            ("DEF-ROOF", "[MAJOR] Active Flashing Leak at Chimney Cricket & 6 Cracked Pipe Boots on Rear Slope", "Licensed Roofer", "$1,850.00"),
            ("DEF-CRAWL", "[STRUCT] Elevated Crawlspace Moisture (22% Wood MC) + Torn Vapor Barrier & Rim Joist Fungus", "Crawl Encapsulation", "$2,450.00"),
            ("DEF-HVAC", "[MAINT] Primary Water Heater (2011 Date Code) Missing TPR Discharge Pipe & Expansion Tank", "Licensed Plumber", "$850.00"),
        ],
        chart_rows: [
            ("Electrical Safety Panel Replacement", "$2,650.00", 34, "accent"),
            ("Crawlspace Moisture & Vapor Remediation", "$2,450.00", 31, "primary"),
            ("Roof Chimney Flashing & Plumbing Cure", "$2,700.00", 35, "primary"),
        ],
        milestones: [
            ("Full 3.5-Hr Inspection", "Thermal IR + drone roof scan", "11/04/2026"),
            ("Contractor Quotes", "Verified local trade pricing", "11/05/2026"),
            ("Repair Addendum Sent", "Served prior to contingency end", "11/06/2026"),
            ("Closing Credit Agreed", "$7,800 Seller credit on ALTA", "RESOLVED"),
        ],
    },
    ArchetypeConceptSpec {
        id: 42,
        slug: "residential-commercial-solar-pv-battery-storage-roi-engineering-proposal",
        title: "Solar PV + LFP Battery Storage Engineering Proposal & 25-Year Cash-Flow Schedule",
        vertical: "Inspection, Appraisal & Clean Energy",
        vertical_slug: "solar-inspection",
        target_profession: "NABCEP Solar EPC Installers, Clean Energy Consultants & Commercial Electrical Contractors",
        primary_keyword: "solar pv and battery storage engineering proposal and roi template pdf",
        secondary_keywords: &["nabcep solar installation contract pdf", "30 percent federal itc solar tax credit worksheet", "nrel pvwatts production guarantee"],
        demand_signal: "Solar EPCs needing transparent, non-deceptive proposals separating gross system cost, 30% federal tax credit (IRC § 25D / § 48E), NREL PVWatts kWh yield, and backup load circuits.",
        statutory_citation: "NEC Article 690 / 706 · NREL PVWatts® v8 Production Model · IRC § 25D / § 48E Credit",
        default_masthead: "trade_banner",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "solar_array",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_solar_inspection.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "NABCEP CERTIFIED",
        stamp_sublabel: "10.56 kW DC + 13.5 kWh",
        sample_org: "SunRidge Clean Energy & Storage Engineering LLC",
        sample_counterparty: "Property Owner: David & Kristen Alvarez",
        kpi_metrics: [
            ("ARRAY / STORAGE CAPACITY", "10.56 kW DC + 13.5 kWh LFP"),
            ("YEAR-1 PRODUCTION EST", "15,840 kWh (94% Offset)"),
            ("GROSS TURNKEY CONTRACT", "$34,800.00"),
            ("NET COST AFTER 30% CREDIT", "$24,360.00 (6.4 YR PAYBACK)"),
        ],
        line_items: [
            ("PV-MOD", "24x REC Alpha Pure-R 440W N-Type Heterojunction All-Black Modules (25-Yr Product/Output Warr)", "10.56 kW DC Array", "$14,880.00"),
            ("BATT-LFP", "1x Tesla Powerwall 3 / Hybrid LFP Energy Storage System (13.5 kWh Usable + Whole-Home Backup)", "11.5 kW Continuous", "$12,400.00"),
            ("BOS-ENG", "PE Structural Stamp, IronRidge FlashFoot2 Racking, NEC 690 Rapid Shutdown & Utility PTO", "Turnkey EPC", "$7,520.00"),
            ("ITC-30", "Estimated 30% Federal Clean Energy Tax Credit (Consult CPA for Personal Tax Liability)", "Illustrative Credit", "-$10,440.00"),
        ],
        chart_rows: [
            ("25-Year Projected Utility Bill Without Solar", "$78,400.00", 100, "primary"),
            ("Net Turnkey Solar + Storage Investment", "$24,360.00", 31, "primary"),
            ("25-Year Cumulative Net Utility Savings", "$54,040.00", 69, "accent"),
        ],
        milestones: [
            ("PE Structural & CAD", "Site survey & permit packet", "WEEK 1"),
            ("Municipal Permit & HOA", "Building + electrical permit", "WEEK 3"),
            ("2-Day Turnkey Install", "Array + LFP battery commissioned", "WEEK 5"),
            ("Inspection & Utility PTO", "Bi-directional meter active", "WEEK 7"),
        ],
    },
    ArchetypeConceptSpec {
        id: 43,
        slug: "uspap-restricted-real-estate-appraisal-desktop-valuation-report",
        title: "USPAP Restricted Real Estate Appraisal & Sales Comparison Adjustment Grid",
        vertical: "Inspection, Appraisal & Clean Energy",
        vertical_slug: "solar-inspection",
        target_profession: "Certified Residential & General Real Estate Appraisers (MAI / SRA), Estate Counsel & Private Lenders",
        primary_keyword: "uspap restricted appraisal report and sales comparison grid template pdf",
        secondary_keywords: &["paired sales adjustment grid pdf", "date of death step up basis appraisal", "uspap standard 2-3 certification"],
        demand_signal: "Independent appraisers handling non-lender private assignments (estate step-up in basis, divorce buyouts, tax appeals, private lending) who want a modern vector alternative to legacy forms.",
        statutory_citation: "2024–2026 USPAP Standards Rule 2-2(b) · Mandatory SR 2-3 Appraiser Certification",
        default_masthead: "judicial_caption",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "parcel_lot",
        default_readability: ReadabilityPreset::JudicialFormal,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_solar_inspection.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "USPAP SR 2-2(b)",
        stamp_sublabel: "CERT RES #CG-3910",
        sample_org: "Cumberland Valuation & Advisory Services Inc. (SRA)",
        sample_counterparty: "Intended User: Estate of Walter H. Jennings (IRC § 1014 Basis)",
        kpi_metrics: [
            ("EFFECTIVE VALUATION DATE", "October 14, 2026 (Retro)"),
            ("PROPERTY RIGHTS APPRAISED", "Fee Simple Estate"),
            ("GROSS LIVING AREA (ANSI)", "2,640 Sq Ft (ANSI Z765)"),
            ("INDICATED MARKET VALUE", "$515,000.00"),
        ],
        line_items: [
            ("SALE-01", "Comp Sale #1 (412 Crestline Rd · 0.28 mi): Sold $525,000 on 09/12/2026 · 2,710 SF · Net Adj -$8,500", "3.2% Gross Adj", "$516,500.00"),
            ("SALE-02", "Comp Sale #2 (509 Oakmont Ln · 0.41 mi): Sold $498,000 on 08/28/2026 · 2,520 SF · Net Adj +$14,400", "4.1% Gross Adj", "$512,400.00"),
            ("SALE-03", "Comp Sale #3 (388 Fairway Dr · 0.55 mi): Sold $530,000 on 07/19/2026 · 2,680 SF · Net Adj -$13,000", "3.8% Gross Adj", "$517,000.00"),
            ("RECON-V", "Reconciled Fee-Simple Market Value Opinion (Weighted 40% Sale #1, 35% Sale #2, 25% Sale #3)", "USPAP SR 1-6", "$515,000.00"),
        ],
        chart_rows: [
            ("Adjusted Sale #1 Indicated Value", "$516,500.00", 98, "primary"),
            ("Adjusted Sale #2 Indicated Value", "$512,400.00", 96, "primary"),
            ("Final Reconciled Market Value Opinion", "$515,000.00", 97, "accent"),
        ],
        milestones: [
            ("ANSI Z765 Laser Measure", "2,640 SF GLA verified", "COMPLETED"),
            ("MLS & Deed Verification", "3 arms-length sales confirmed", "VERIFIED"),
            ("SR 2-3 Certification", "Signed by Certified Appraiser", "11/06/2026"),
            ("Delivered to Executor", "Ready for IRS Form 706/8971", "DELIVERED"),
        ],
    },
    ArchetypeConceptSpec {
        id: 44,
        slug: "phase-i-environmental-site-assessment-astm-e1527-21-executive-summary",
        title: "Phase I Environmental Site Assessment (ASTM E1527-21) Executive REC Summary",
        vertical: "Inspection, Appraisal & Clean Energy",
        vertical_slug: "solar-inspection",
        target_profession: "Environmental Engineers (EP), Commercial Lenders, Brownfield Developers & CRE Counsel",
        primary_keyword: "phase i environmental site assessment astm e1527-21 executive summary template",
        secondary_keywords: &["recognized environmental condition rec matrix", "cercla all appropriate inquiries aai report", "vapor encroachment screening report"],
        demand_signal: "Commercial real estate buyers and SBA/CMBS lenders needing clear ASTM E1527-21 tables distinguishing RECs, Controlled RECs (CRECs), and Historical RECs (HRECs) within the 180-day AAI shelf life.",
        statutory_citation: "ASTM E1527-21 Standard · 40 C.F.R. Part 312 (CERCLA Innocent Landowner AAI Defense)",
        default_masthead: "swiss_grid",
        default_kpi_style: "dark_terminal",
        default_table_style: "dark_header",
        default_diagram_kind: "parcel_lot",
        default_readability: ReadabilityPreset::ExecutiveCompact,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_solar_inspection.png",
        primary_hex: "#0f172a",
        accent_hex: "#059669",
        stamp_label: "0 ACTIVE RECs",
        stamp_sublabel: "ASTM E1527-21 AAI",
        sample_org: "Appalachian GeoEnvironmental & Engineering PC",
        sample_counterparty: "User: First Horizon Commercial Real Estate Lending",
        kpi_metrics: [
            ("RECOGNIZED ENV COND (REC)", "0 ACTIVE RECs FOUND"),
            ("HISTORICAL / CONTROLLED", "1 HREC (CLOSED 2014 NFA)"),
            ("AAI 180-DAY VIABILITY", "Valid Through 04/28/2027"),
            ("FURTHER ACTION (PHASE II)", "NOT REQUIRED"),
        ],
        line_items: [
            ("REC-00", "Recognized Environmental Conditions (RECs) on Subject 4.2-Acre Industrial Parcel", "Site Recon + EDR", "NONE (0)"),
            ("HREC-01", "Historical REC: Former 2,000-Gal Diesel UST Removed in 2014 w/ State TDEC No Further Action Letter", "TDEC File #48-0912", "CLOSED HREC"),
            ("VAP-2600", "ASTM E2600 Tier 1 Vapor Encroachment Screening (Adjoining Upgradient Commercial Properties)", "1/3 Mile Radius", "SCREENED OUT"),
            ("EP-CERT", "40 C.F.R. § 312.21 Environmental Professional Declaration & Lender Reliance Authorization", "PE #29401 Signed", "CERTIFIED"),
        ],
        chart_rows: [
            ("Historical Database Radius Records Reviewed", "142 Records", 100, "primary"),
            ("Sanborn Fire Insurance & Aerial Intervals (1938–2026)", "11 Decades", 85, "primary"),
            ("Open Unremediated RECs Requiring Phase II", "0 Open (Clean)", 5, "accent"),
        ],
        milestones: [
            ("EDR Radius & Sanborn", "1938–2026 chain reviewed", "10/28/2026"),
            ("Site Reconnaissance", "Interior/exterior walk-through", "11/01/2026"),
            ("TDEC File Verification", "2014 NFA closure confirmed", "11/03/2026"),
            ("EP Signed & Delivered", "Qualifies for CERCLA AAI", "11/06/2026"),
        ],
    },
    ArchetypeConceptSpec {
        id: 45,
        slug: "alta-nsps-land-title-boundary-survey-certification-table-a-schedule",
        title: "ALTA/NSPS Land Title Boundary Survey Certification, Zoning & Table A Schedule",
        vertical: "Inspection, Appraisal & Clean Energy",
        vertical_slug: "solar-inspection",
        target_profession: "Professional Land Surveyors (PLS), Civil Engineers, Title Underwriters & Developers",
        primary_keyword: "alta nsps land title survey certification and table a checklist pdf",
        secondary_keywords: &["boundary survey metes and bounds report", "fema flood zone surveyor certification", "easement and setback encroachment schedule"],
        demand_signal: "Land surveyors, title companies, and GIS/CAD buyers needing a structured companion sheet for ALTA/NSPS surveys detailing Table A optional items, recorded easements, and precision closure.",
        statutory_citation: "2021 ALTA/NSPS Minimum Standard Detail Requirements · Relative Positional Precision < 0.07 ft",
        default_masthead: "judicial_caption",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "parcel_lot",
        default_readability: ReadabilityPreset::ExecutiveCompact,
        default_execution_mode: "wet-ink",
        notary_required: true,
        seal_filename: "seal_solar_inspection.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#0284c7",
        stamp_label: "CLOSURE: 1:48,200",
        stamp_sublabel: "ALTA/NSPS TABLE A",
        sample_org: "Meridian Geomatics & Professional Land Surveying PLLC",
        sample_counterparty: "Chicago Title Insurance Co. & Buyer SPE LLC",
        kpi_metrics: [
            ("COMPUTED LAND AREA", "2.418 Acres (105,328 SF)"),
            ("POSITIONAL PRECISION", "0.02 ft (1:48,200 Closure)"),
            ("FEMA NFHL FLOOD ZONE", "Zone X (Unshaded · 0.2% Ann)"),
            ("ENCROACHMENTS OBSERVED", "NONE (0 BOUNDARY)"),
        ],
        line_items: [
            ("TBL-A1", "Table A Items 1, 2, 3, 4, 6(b), 7(a), 8, 9, 11, 13, 16 & 19 Field-Verified and Platted", "12 Table A Items", "CERTIFIED"),
            ("ESM-12", "Schedule B-II Exception #12: 20' Public Sanitary Sewer Easement Along Southern Boundary (Plotted)", "Book 4102 Pg 118", "NO CONFLICT"),
            ("ZON-C2", "Municipal C-2 Commercial Zoning Setbacks: 25' Front, 10' Side, 20' Rear · Building Height 28.4'", "118 Paved Spaces", "CONFORMING"),
            ("PLS-SEAL", "Mandatory ALTA/NSPS Section 7 Surveyor Certification to Title Insurer, Lender & Purchaser", "TN RLS #2814", "SEALED"),
        ],
        chart_rows: [
            ("Impervious Building & Parking Coverage", "61,090 SF (58%)", 58, "primary"),
            ("Pervious Landscaped & Detention Area", "44,238 SF (42%)", 42, "accent"),
            ("Maximum Permitted C-2 Lot Coverage", "78,996 SF (75%)", 75, "primary"),
        ],
        milestones: [
            ("Title Commitment In", "Schedule B-II exceptions mapped", "10/20/2026"),
            ("RTK GNSS & Robotic", "Iron pins & monuments shot", "10/27/2026"),
            ("811 Utility Locate", "Subsurface markings tied", "10/30/2026"),
            ("Sealed DXF + PDF Plat", "Delivered to Title & Counsel", "11/06/2026"),
        ],
    },

    // =========================================================================
    // VERTICAL 10: CONSULTING, HR, EDUCATION & GOV GRANTS (46..=50)
    // =========================================================================
    ArchetypeConceptSpec {
        id: 46,
        slug: "executive-employment-offer-letter-equity-vesting-confidentiality",
        title: "Executive Employment Offer Letter, 4-Year Equity Vesting & PIIA Summary",
        vertical: "Consulting, HR & GovCon Grants",
        vertical_slug: "consulting-govcon",
        target_profession: "Startup Founders, VP of People/HR, Executive Recruiters & Compensation Committees",
        primary_keyword: "executive employment offer letter with equity vesting schedule template pdf",
        secondary_keywords: &["1 year cliff 48 month vesting schedule", "irc 83b election notice offer letter", "at will employment and piia agreement"],
        demand_signal: "High-growth companies and recruiters needing a crisp, high-converting offer letter that visually illustrates base salary, target bonus, and 4-year equity vesting with 1-year cliff.",
        statutory_citation: "FLSA Exempt Executive Classification · IRC § 409A Strike Price · IRC § 83(b) 30-Day Window",
        default_masthead: "editorial_serif",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_consulting_govcon.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "BOARD APPROVED",
        stamp_sublabel: "409A STRIKE: $1.85",
        sample_org: "Hyperion Autonomous Systems Inc.",
        sample_counterparty: "Candidate: Samantha J. Chen (VP of Platform Engineering)",
        kpi_metrics: [
            ("ANNUAL BASE SALARY", "$215,000.00 / YR"),
            ("ANNUAL PERFORMANCE BONUS", "20% Target ($43,000)"),
            ("INITIAL ISO EQUITY GRANT", "60,000 Shares (0.60% FD)"),
            ("TOTAL TARGET COMP (TTC)", "$258,000.00 + EQUITY"),
        ],
        line_items: [
            ("COMP-01", "Exempt Annual Base Salary Paid Semi-Monthly via Direct Deposit ($8,958.33 per Pay Period)", "FLSA Exempt", "$215,000.00"),
            ("COMP-02", "Annual Executive KPI Bonus (Tied to Platform Uptime >= 99.95% & Net Dollar Retention)", "20% Target", "$43,000.00"),
            ("EQT-ISO", "60,000 Incentive Stock Options (ISOs) @ $1.85/sh 409A FMV (25% 1-Yr Cliff, 1/48th Monthly)", "4-Year Vesting", "60,000 ISOs"),
            ("BEN-04", "100% Company-Paid Platinum Medical/Dental/Vision + 4% Safe-Harbor 401(k) Match + $5k Relo", "Day-1 Benefits", "INCLUDED"),
        ],
        chart_rows: [
            ("Year 1 Cliff Vest (Month 12: 15,000 Shares)", "25% Vested", 25, "primary"),
            ("Year 2 Cumulative Vest (Month 24: 30,000 Sh)", "50% Vested", 50, "primary"),
            ("Year 4 Full Vesting (Month 48: 60,000 Shares)", "100% Vested", 100, "accent"),
        ],
        milestones: [
            ("Offer Counter-Sign", "Valid through Nov 13, 2026", "STEP 1"),
            ("PIIA & Background", "IP assignment executed", "STEP 2"),
            ("Start Date Onboarding", "December 1, 2026", "DAY 1"),
            ("Board Option Grant", "30-day § 83(b) window opens", "BOARD MTG"),
        ],
    },
    ArchetypeConceptSpec {
        id: 47,
        slug: "independent-contractor-consulting-agreement-1099-safe-harbor-sow",
        title: "Independent Contractor Consulting Agreement (1099 Safe Harbor) & W-9 Cert",
        vertical: "Consulting, HR & GovCon Grants",
        vertical_slug: "consulting-govcon",
        target_profession: "Management Consultants, Fractional Executives, Operations Leaders & SMB Owners",
        primary_keyword: "independent contractor consulting agreement template pdf 1099 compliance",
        secondary_keywords: &["fractional cfo cmo consulting retainer contract", "irs common law 1099 safe harbor clause", "mutual nda and work product ownership"],
        demand_signal: "Businesses and fractional executives structuring 1099 consulting engagements with explicit IRS three-factor independence recitals (behavioral, financial, non-exclusive relationship).",
        statutory_citation: "26 U.S.C. § 3508 / IRS Common-Law 3-Prong Independence Test · 17 U.S.C. § 204 Assignment",
        default_masthead: "executive_split",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_consulting_govcon.png",
        primary_hex: "#0f172a",
        accent_hex: "#0284c7",
        stamp_label: "1099-NEC ENTITY",
        stamp_sublabel: "W-9 EIN ON FILE",
        sample_org: "NorthStar Fractional Operations & Strategy LLC",
        sample_counterparty: "Client: Precision Manufacturing Group Inc.",
        kpi_metrics: [
            ("MONTHLY RETAINER FEE", "$6,500.00 / MONTH"),
            ("TAX CLASSIFICATION", "1099-NEC (LLC S-Corp)"),
            ("TERM & CANCELLATION", "6 Months · 30-Day Notice"),
            ("NON-EXCLUSIVE STATUS", "Verified Independent"),
        ],
        line_items: [
            ("RET-01", "Fractional COO Advisory: Weekly Executive Leadership Cadence, KPI Scorecard & Margin Audit", "Deliverable Based", "$4,500.00/MO"),
            ("RET-02", "ERP Inventory Turnover Optimization & Vendor Lead-Time Renegotiation Playbook", "Q4 Milestone", "$2,000.00/MO"),
            ("EXP-03", "Pre-Approved Out-of-Town Plant Travel Reimbursed at Actual Receipt Cost (Zero Markup)", "Accountable Receipts", "AT COST"),
            ("IRS-SAFE", "Contractor Furnishes Own Equipment, Maintains General Liability COI & Serves Multiple Clients", "Safe Harbor", "VERIFIED"),
        ],
        chart_rows: [
            ("Full-Time W-2 COO Salary + Burden ($265k)", "$22,083 / mo", 100, "primary"),
            ("Fractional COO 1099 Retainer ($78k/yr)", "$6,500 / mo", 29, "accent"),
            ("Net Monthly Payroll & Overhead Savings", "$15,583 / mo", 71, "accent"),
        ],
        milestones: [
            ("Agreement & Form W-9", "EIN 88-4102914 verified", "11/06/2026"),
            ("30-Day Margin Audit", "SKU profitability matrix", "12/06/2026"),
            ("90-Day ERP Rollout", "Safety-stock reorder rules", "02/06/2027"),
            ("Annual Form 1099-NEC", "Issued if non-corporate entity", "01/31/2027"),
        ],
    },
    ArchetypeConceptSpec {
        id: 48,
        slug: "federal-nonprofit-grant-budget-justification-2-cfr-200-uniform-guidance",
        title: "Federal & Foundation Grant Budget Narrative & 2 C.F.R. § 200 Indirect Cost Schedule",
        vertical: "Consulting, HR & GovCon Grants",
        vertical_slug: "consulting-govcon",
        target_profession: "Non-Profit Executive Directors, Grant Writers, University PIs & Municipal Agencies",
        primary_keyword: "federal grant budget narrative and justification template 2 cfr 200 pdf",
        secondary_keywords: &["sf 424a budget justification worksheet", "15 percent de minimis indirect cost rate 2024", "in kind match contribution schedule"],
        demand_signal: "Grant writers and non-profits adopting the updated OMB 2 C.F.R. § 200.414(f) 15% de minimis Modified Total Direct Cost (MTDC) indirect cost rate and matching-fund schedules.",
        statutory_citation: "OMB Uniform Guidance 2 C.F.R. Part 200 · § 200.414(f) 15.0% De Minimis MTDC Rate",
        default_masthead: "swiss_grid",
        default_kpi_style: "metric_cards",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::Standard,
        default_execution_mode: "wet-ink",
        notary_required: false,
        seal_filename: "seal_consulting_govcon.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "2 CFR § 200.414",
        stamp_sublabel: "15% MTDC VERIFIED",
        sample_org: "Appalachian Workforce & Veteran STEM Initiative (501c3)",
        sample_counterparty: "Grantor: U.S. Dept. of Labor / Regional Foundation (SF-424A)",
        kpi_metrics: [
            ("TOTAL PROJECT BUDGET", "$287,500.00 (12 MOS)"),
            ("FEDERAL GRANT REQUEST", "$250,000.00 (87.0%)"),
            ("INDIRECT COST (15% MTDC)", "$32,608.70"),
            ("COMMITTED LOCAL MATCH", "$37,500.00 (15% MATCH)"),
        ],
        line_items: [
            ("CAT-A/B", "Personnel & Fringe Benefits: Program Director (1.0 FTE) + 2 Veteran Career Coaches (1.5 FTE)", "Salary + 24% Fringe", "$154,800.00"),
            ("CAT-E", "Participant Training Stipends, CompTIA/AWS Exam Vouchers & Lab Workstations (120 Veterans)", "120 Trainees", "$62,591.30"),
            ("CAT-J", "Indirect Costs: 15.0% De Minimis Rate on $217,391.30 Modified Total Direct Costs (MTDC)", "2 C.F.R. § 200.414(f)", "$32,608.70"),
            ("MAT-LOC", "Committed Non-Federal Cost Share / In-Kind Match (Employer Host Lab Space & Mentorship)", "Audited Match", "$37,500.00"),
        ],
        chart_rows: [
            ("Direct Program Personnel & Veteran Training", "$217,391.30", 76, "primary"),
            ("15% De Minimis Indirect Admin (MTDC)", "$32,608.70", 14, "primary"),
            ("Committed Non-Federal Matching Funds", "$37,500.00", 16, "accent"),
        ],
        milestones: [
            ("SAM.gov UEI Verified", "UEI #J8K4L9M2N1P4 active", "VERIFIED"),
            ("SF-424A Submitted", "Filed via Grants.gov portal", "11/06/2026"),
            ("Notice of Award (NOA)", "Quarterly PMS drawdown", "01/15/2027"),
            ("Single Audit / FFR", "SF-425 financial report", "12/31/2027"),
        ],
    },
    ArchetypeConceptSpec {
        id: 49,
        slug: "government-contractor-govcon-capability-statement-sam-uei-naics-one-pager",
        title: "Federal Government Contractor (GovCon) Capability Statement (UEI / CAGE / NAICS)",
        vertical: "Consulting, HR & GovCon Grants",
        vertical_slug: "consulting-govcon",
        target_profession: "SDVOSB / 8(a) / WOSB / HUBZone Federal Contractors, Defense Suppliers & GovCon Consultants",
        primary_keyword: "federal government contractor capability statement template pdf sam uei cage",
        secondary_keywords: &["sdvosb capability statement one pager", "naics psc code past performance sheet", "far 13 micro purchase simplified acquisition"],
        demand_signal: "GovGazette & federal contractors needing a high-impact 1-page Capability Statement displaying SAM.gov UEI, CAGE Code, SDVOSB/Small Business socio-economic badges, NAICS codes, and quantified past performance.",
        statutory_citation: "FAR Part 13 & Part 19 Small Business Set-Aside · SAM.gov Active Registration · SDVOSB",
        default_masthead: "trade_banner",
        default_kpi_style: "dark_terminal",
        default_table_style: "dark_header",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::ExecutiveCompact,
        default_execution_mode: "esign",
        notary_required: false,
        seal_filename: "seal_consulting_govcon.png",
        primary_hex: "#0f172a",
        accent_hex: "#b45309",
        stamp_label: "SAM.GOV ACTIVE",
        stamp_sublabel: "SDVOSB CERTIFIED",
        sample_org: "Vanguard Tactical & Geospatial Systems LLC",
        sample_counterparty: "Federal Contracting Officers (DoD / USACE / VA / GSA)",
        kpi_metrics: [
            ("SAM.GOV UEI / CAGE CODE", "UEI: K9M4N8P2Q1R7 · CAGE: 9X4K2"),
            ("SOCIO-ECONOMIC STATUS", "SBA Certified SDVOSB · SB"),
            ("PRIMARY NAICS CODES", "541511 · 541370 · 541611"),
            ("GSA MAS / GPC CARDS", "GSA 47QTCA26D0081 · GPC OK"),
        ],
        line_items: [
            ("PP-USACE", "U.S. Army Corps of Engineers (Contract #W912P5-25-C-0014): LiDAR Bathymetric & Parcel GIS Pipeline", "CPARS: Exceptional", "$1,240,000"),
            ("PP-DVA", "Dept. of Veterans Affairs VISN 9 (Order #36C249-26-P-0182): Automated Claims Document OCR Engine", "100% On-Time SLA", "$485,000"),
            ("PP-TVA", "Tennessee Valley Authority (Subcontract #TVA-88412): Transmission Corridor Right-of-Way CAMA Audit", "Zero Defects", "$310,000"),
            ("DIFF-01", "Core Differentiator: FedRAMP-Ready Rust/Tokio Ingestion Engine (100x Lower Memory Footprint)", "Zero COTS Bloat", "VERIFIED"),
        ],
        chart_rows: [
            ("On-Time Contract Deliverable Rate (CPARS)", "100% On-Time", 100, "accent"),
            ("Direct Prime Federal Past Performance", "$2,035,000 Awarded", 85, "primary"),
            ("Sole-Source SDVOSB Threshold Capacity", "Up to $4.5M (FAR 19.14)", 92, "primary"),
        ],
        milestones: [
            ("Micro-Purchase (GPC)", "Instant award <= $10,000", "24 HOURS"),
            ("Simplified Acq (SAP)", "FAR Part 13 <= $250,000", "5–10 DAYS"),
            ("SDVOSB Sole Source", "FAR 19.1406 up to $4.5M", "15–30 DAYS"),
            ("GSA MAS Task Order", "Direct RFQ via eBuy", "ACTIVE"),
        ],
    },
    ArchetypeConceptSpec {
        id: 50,
        slug: "charitable-501c3-endowment-gift-receipt-irc-170f8-substantiation",
        title: "501(c)(3) Charitable Contribution Receipt & IRC § 170(f)(8) Contemporaneous Acknowledgment",
        vertical: "Consulting, HR & GovCon Grants",
        vertical_slug: "consulting-govcon",
        target_profession: "501(c)(3) Non-Profits, Private Foundations, Churches, Universities & Major Gift Officers",
        primary_keyword: "501c3 charitable donation receipt template pdf irc 170f8",
        secondary_keywords: &["contemporaneous written acknowledgment 250 dollar donation", "quid pro quo contribution disclosure statement", "appreciated stock and daf gift receipt"],
        demand_signal: "Non-profits and CPA auditors emphasizing strict IRS § 170(f)(8) compliance: explicitly stating whether any goods or services were provided in exchange for gifts >= $250 so donors do not lose their tax deduction.",
        statutory_citation: "26 U.S.C. § 170(f)(8) Contemporaneous Written Acknowledgment · IRC § 6115 Quid Pro Quo Rule",
        default_masthead: "editorial_serif",
        default_kpi_style: "metric_cards",
        default_table_style: "striped",
        default_diagram_kind: "floorplan_spec",
        default_readability: ReadabilityPreset::HighLegibility,
        default_execution_mode: "esign",
        notary_required: false,
        seal_filename: "seal_consulting_govcon.png",
        primary_hex: "#1e3a8a",
        accent_hex: "#059669",
        stamp_label: "IRC § 170(f)(8)",
        stamp_sublabel: "501(c)(3) TAX EXEMPT",
        sample_org: "Chattanooga Veteran Housing & Legal Aid Foundation (EIN 62-4910284)",
        sample_counterparty: "Donor: The Harrison Family Charitable Fund",
        kpi_metrics: [
            ("GROSS CONTRIBUTION RECEIVED", "$10,000.00"),
            ("GOODS / SERVICES PROVIDED", "$0.00 (NONE PROVIDED)"),
            ("NET TAX-DEDUCTIBLE AMOUNT", "$10,000.00 (100%)"),
            ("IRS EXEMPTION STATUS", "501(c)(3) Public Charity"),
        ],
        line_items: [
            ("GFT-01", "Unrestricted Cash Wire Contribution to the Disabled Veteran Property Tax Defense Endowment", "Received 11/06/2026", "$10,000.00"),
            ("QPQ-00", "IRC § 170(f)(8)(B)(ii) Mandatory Statement: No Goods or Services Were Provided in Exchange", "100% Deductible", "$0.00"),
            ("EIN-501", "IRS Determination Letter On File: Tax-Exempt Public Charity Under IRC § 501(c)(3) & § 170(b)(1)(A)(vi)", "EIN 62-4910284", "VERIFIED"),
            ("ACK-DT", "Contemporaneous Written Acknowledgment Issued Prior to Donor's Tax Return Due Date", "Statutory Timely", "ISSUED"),
        ],
        chart_rows: [
            ("Gross Charitable Gift Received", "$10,000.00", 100, "primary"),
            ("Direct Program Allocation (91%)", "$9,100.00", 91, "accent"),
            ("Tax-Deductible Receipt Basis (100%)", "$10,000.00", 100, "accent"),
        ],
        milestones: [
            ("Gift Wire Received", "Settled in Endowment Account", "11/06/2026"),
            ("§ 170(f)(8) Receipt", "Generated & signed instantly", "11/06/2026"),
            ("Donor CPA Archive", "Retained for Schedule A / Corp", "TAX FILE"),
            ("Annual Impact Report", "Endowment outcomes published", "01/15/2027"),
        ],
    },
];

/// Look up a concept specification by its URL-safe slug or numeric ID string (`"1"`..`"50"`).
pub fn find_concept_by_slug(slug_or_id: &str) -> Option<&'static ArchetypeConceptSpec> {
    let trimmed = slug_or_id.trim();
    if let Ok(num) = trimmed.parse::<u8>() {
        return ARCHETYPE_CONCEPTS_50.iter().find(|c| c.id == num);
    }
    ARCHETYPE_CONCEPTS_50
        .iter()
        .find(|c| c.slug.eq_ignore_ascii_case(trimmed))
}

fn sanitize_hex_color<'a>(candidate: Option<&'a str>, fallback: &'a str) -> &'a str {
    if let Some(raw) = candidate {
        let s = raw.trim();
        if s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit()) {
            return s;
        }
    }
    fallback
}

/// Render a complete 1-page vector Typst document for any of the 50 concepts, applying optional
/// live interactive overrides (`CustomRenderOptions`) and embedding Nano Banana certification seals
/// plus inline SVG architectural/technical schematics, bar charts, timelines, rubber stamps, and
/// `/AcroForm`-compatible execution blocks.
pub fn render_concept_typst_markup(
    spec: &ArchetypeConceptSpec,
    opts: Option<&CustomRenderOptions>,
    seal_dir_prefix: &str,
) -> String {
    let masthead = opts
        .and_then(|o| o.masthead.as_deref())
        .filter(|s| !s.is_empty())
        .unwrap_or(spec.default_masthead);
    let kpi_style = opts
        .and_then(|o| o.kpi.as_deref())
        .filter(|s| !s.is_empty())
        .unwrap_or(spec.default_kpi_style);
    let table_style = opts
        .and_then(|o| o.table.as_deref())
        .filter(|s| !s.is_empty())
        .unwrap_or(spec.default_table_style);
    let diagram_kind = opts
        .and_then(|o| o.diagram.as_deref())
        .filter(|s| !s.is_empty())
        .unwrap_or(spec.default_diagram_kind);
    let execution_mode = opts
        .and_then(|o| o.execution.as_deref())
        .filter(|s| !s.is_empty())
        .unwrap_or(spec.default_execution_mode);
    let readability = match opts.and_then(|o| o.preset.as_deref()) {
        Some("high_legibility") => ReadabilityPreset::HighLegibility,
        Some("executive_compact") => ReadabilityPreset::ExecutiveCompact,
        Some("judicial_formal") => ReadabilityPreset::JudicialFormal,
        Some("standard") => ReadabilityPreset::Standard,
        _ => spec.default_readability,
    };

    let primary_hex = sanitize_hex_color(
        opts.and_then(|o| o.primary_hex.as_deref()),
        spec.primary_hex,
    );
    let accent_hex =
        sanitize_hex_color(opts.and_then(|o| o.accent_hex.as_deref()), spec.accent_hex);

    let title_str = opts
        .and_then(|o| o.custom_title.as_deref())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(spec.title);
    let org_str = opts
        .and_then(|o| o.custom_org.as_deref())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(spec.sample_org);
    let counterparty_str = opts
        .and_then(|o| o.custom_counterparty.as_deref())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(spec.sample_counterparty);
    let stamp_str = opts
        .and_then(|o| o.stamp.as_deref())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(spec.stamp_label);

    let seal_path = if seal_dir_prefix.is_empty() {
        String::new()
    } else {
        format!(
            "{}/{}",
            seal_dir_prefix.trim_end_matches('/'),
            spec.seal_filename
        )
    };

    let kpi_tuple_str = spec
        .kpi_metrics
        .iter()
        .map(|(k, v)| {
            format!(
                "(\"{}\", \"{}\")",
                escape_typst_string_literal(k),
                escape_typst_string_literal(v)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    let rows_tuple_str = spec
        .line_items
        .iter()
        .map(|(code, desc, qty, amt)| {
            format!(
                "(\"{}\", \"{}\", \"{}\", \"{}\")",
                escape_typst_string_literal(code),
                escape_typst_string_literal(desc),
                escape_typst_string_literal(qty),
                escape_typst_string_literal(amt)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    let chart_tuple_str = spec
        .chart_rows
        .iter()
        .map(|(lbl, val, pct, tag)| {
            format!(
                "(\"{}\", \"{}\", {}, \"{}\")",
                escape_typst_string_literal(lbl),
                escape_typst_string_literal(val),
                pct,
                escape_typst_string_literal(tag)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    let milestones_tuple_str = spec
        .milestones
        .iter()
        .map(|(phase, detail, dt)| {
            format!(
                "(\"{}\", \"{}\", \"{}\")",
                escape_typst_string_literal(phase),
                escape_typst_string_literal(detail),
                escape_typst_string_literal(dt)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    let notary_flag = if execution_mode == "notary_jurat" {
        true
    } else if execution_mode == "esign" {
        false
    } else {
        spec.notary_required
    };

    let sig_type_str = if execution_mode == "esign" {
        "esign"
    } else {
        "wet-ink"
    };

    format!(
        r##"{graphics_lib}

#set page(
  paper: "us-letter",
  margin: (top: 0.34in, bottom: 0.40in, left: 0.42in, right: 0.42in),
  footer: context [
    #line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1"))
    #v(2.5pt)
    #grid(
      columns: (1fr, auto, 1fr),
      align: (left + horizon, center + horizon, right + horizon),
      text(font: hde-font-mono, size: 6.5pt, fill: rgb("#475569"))[
        CONCEPT #{id_num:02} · {slug}
      ],
      text(font: hde-font-sans, size: 6.5pt, weight: "bold", fill: rgb("{primary_hex}"))[
        {citation_text}
      ],
      text(font: hde-font-mono, size: 6.5pt, weight: "bold", fill: rgb("#0f172a"))[
        Page #counter(page).display("1 of 1", both: true) · {preset_id}
      ]
    )
  ]
)
#set text(font: hde-font-sans, size: {font_size}, fill: rgb("#0f172a"))
#set par(leading: 0.48em)

#hde-masthead-variation(
  variant: "{masthead}",
  title: "{title}",
  subtitle: "Prepared for: {counterparty} · Target Role: {profession}",
  org-name: "{org}",
  vertical-badge: "{vertical}",
  doc-ref: "HDE-2026-C{id_num:02}",
  date-str: "2026-11-06",
  citation: "{citation}",
  seal-path: "{seal_path}",
  primary-color: rgb("{primary_hex}"),
  accent-color: rgb("{accent_hex}"),
)

#hde-kpi-strip-variation(
  variant: "{kpi_style}",
  metrics: ({kpi_tuple_str},),
  primary-color: rgb("{primary_hex}"),
  accent-color: rgb("{accent_hex}"),
)

#svg-architectural-diagram(
  kind: "{diagram_kind}",
  title: "EMBEDDED TECHNICAL SCHEMATIC · {vertical}",
  primary-hex: "{primary_hex}",
  accent-hex: "{accent_hex}",
  metric-a: "{metric_a}",
  metric-b: "{metric_b}",
)

#hde-schedule-table-variation(
  variant: "{table_style}",
  rows: ({rows_tuple_str},),
  primary-color: rgb("{primary_hex}"),
  accent-color: rgb("{accent_hex}"),
)

#hde-comparative-bar-chart(
  title: "FINANCIAL & VALUATION ALLOCATION BREAKDOWN",
  rows: ({chart_tuple_str},),
  primary-color: rgb("{primary_hex}"),
  accent-color: rgb("{accent_hex}"),
)

#hde-milestone-timeline(
  title: "EXECUTION MILESTONES, STATUTORY GATES & VERIFICATION TIMELINE",
  steps: ({milestones_tuple_str},),
  primary-color: rgb("{primary_hex}"),
  accent-color: rgb("{accent_hex}"),
)

#jurat-box(
  signer-name: "{org}",
  signer-role: "{profession}",
  counterparty-name: "{counterparty}",
  signature-type: "{sig_type_str}",
  notary-required: {notary_flag},
  stamp-label: "{stamp_label}",
  stamp-sublabel: "{stamp_sublabel}",
  stamp-color: "{accent_hex}",
  primary-color: rgb("{primary_hex}"),
)
"##,
        graphics_lib = HALLMAN_EMBEDDED_GRAPHICS_TYP,
        id_num = spec.id,
        slug = escape_typst_text(spec.slug),
        primary_hex = primary_hex,
        accent_hex = accent_hex,
        citation_text = escape_typst_text(spec.statutory_citation),
        preset_id = readability.as_typst_id(),
        font_size = readability.base_font_size(),
        masthead = escape_typst_string_literal(masthead),
        title = escape_typst_string_literal(title_str),
        counterparty = escape_typst_string_literal(counterparty_str),
        profession = escape_typst_string_literal(spec.target_profession),
        org = escape_typst_string_literal(org_str),
        vertical = escape_typst_string_literal(spec.vertical),
        citation = escape_typst_string_literal(spec.statutory_citation),
        seal_path = escape_typst_string_literal(&seal_path),
        kpi_style = escape_typst_string_literal(kpi_style),
        kpi_tuple_str = kpi_tuple_str,
        diagram_kind = escape_typst_string_literal(diagram_kind),
        metric_a = escape_typst_string_literal(spec.kpi_metrics[0].1),
        metric_b = escape_typst_string_literal(spec.kpi_metrics[3].1),
        table_style = escape_typst_string_literal(table_style),
        rows_tuple_str = rows_tuple_str,
        chart_tuple_str = chart_tuple_str,
        milestones_tuple_str = milestones_tuple_str,
        sig_type_str = sig_type_str,
        notary_flag = if notary_flag { "true" } else { "false" },
        stamp_label = escape_typst_string_literal(stamp_str),
        stamp_sublabel = escape_typst_string_literal(spec.stamp_sublabel),
    )
}

/// Render a 1-page multi-variation visual comparison showcase sheet (`"mastheads"`, `"tables_and_kpis"`,
/// `"diagrams_and_stamps"`, or `"signatures_and_seals"`) for screenshot inspection and live site galleries.
pub fn render_element_showcase_sheet_typst(sheet_kind: &str, seal_dir_prefix: &str) -> String {
    let sp = |name: &str| -> String {
        if seal_dir_prefix.is_empty() {
            String::new()
        } else {
            format!("{}/{}", seal_dir_prefix.trim_end_matches('/'), name)
        }
    };

    match sheet_kind {
        "mastheads" => format!(
            r##"{graphics_lib}
#set page(paper: "us-letter", margin: 0.4in)
#set text(font: hde-font-sans, size: 9pt)

#rect(width: 100%, fill: rgb("#0f172a"), inset: (x: 12pt, y: 8pt), radius: 4pt)[
  #text(font: hde-font-sans, size: 12pt, weight: "black", fill: rgb("#ffe500"))[
    HALLMAN DOC ENGINE — MASTHEAD & HEADER STYLE VARIATIONS (6 ARCHITECTURES)
  ] \
  #text(font: hde-font-mono, size: 7.5pt, fill: white)[
    Shared across Untax (untax.io) & Pressmark (pressmark.io) · Embedded Nano Banana Certification Seals
  ]
]
#v(6pt)

#hde-masthead-variation(
  variant: "neo_brutalist",
  title: "1. Neo-Brutalist Rebel Banner (#FFE500 / 2.5pt Black)",
  subtitle: "High-impact construction change orders, creator deal memos & taxpayer audit summaries",
  org-name: "Ridgecrest Structural Remodeling",
  vertical-badge: "VARIANT 01 · NEO-BRUTALIST",
  doc-ref: "CO-2026-03",
  date-str: "2026-11-06",
  citation: "AIA G701 Standard",
  seal-path: "{seal_trades}",
  primary-color: rgb("#0f172a"),
  accent-color: rgb("#d97706"),
)
#v(5pt)

#hde-masthead-variation(
  variant: "judicial_caption",
  title: "2. Statutory & Quasi-Judicial Caption Header",
  subtitle: "Board of Equalization petitions, mechanics' lien waivers, ALTA closing sheets & IOLTA retainers",
  org-name: "Untax Property Tax Audit Engine",
  vertical-badge: "VARIANT 02 · JUDICIAL CAPTION",
  doc-ref: "BOE-2026-114",
  date-str: "2026-11-06",
  citation: "Tenn. Code Ann. § 67-5-1407",
  seal-path: "{seal_tax}",
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
)
#v(5pt)

#hde-masthead-variation(
  variant: "trade_banner",
  title: "3. Industrial Dark Trade & GovCon Command Header",
  subtitle: "AIA G702 progress draws, HVAC field diagnostics, freight BOLs & SAM.gov capability statements",
  org-name: "Vanguard Geospatial & Tactical LLC",
  vertical-badge: "VARIANT 03 · TRADE COMMAND",
  doc-ref: "SAM-UEI-9X4K2",
  date-str: "2026-11-06",
  citation: "FAR Part 13 / SDVOSB",
  seal-path: "{seal_gov}",
  primary-color: rgb("#0f172a"),
  accent-color: rgb("#d97706"),
)
#v(5pt)

#hde-masthead-variation(
  variant: "swiss_grid",
  title: "4. Swiss Architectural & Enterprise SaaS Precision Grid",
  subtitle: "Enterprise SaaS order forms, NNN CAM reconciliations, CNC quotes & Phase I ESA reports",
  org-name: "VectorPulse Cloud Systems Inc.",
  vertical-badge: "VARIANT 04 · SWISS GRID",
  doc-ref: "ORD-2026-884",
  date-str: "2026-11-06",
  citation: "ASC 606 / SOC2 Type II",
  seal-path: "{seal_saas}",
  primary-color: rgb("#0f172a"),
  accent-color: rgb("#0284c7"),
)
#v(5pt)

#hde-masthead-variation(
  variant: "editorial_serif",
  title: "5. Editorial Serif & Fiduciary Wealth Masthead",
  subtitle: "Residential URLTA leases, CPA Circular 230 engagements, RIA AUM invoices & 501(c)(3) receipts",
  org-name: "Lookout Fiduciary Wealth Partners",
  vertical-badge: "VARIANT 05 · EDITORIAL SERIF",
  doc-ref: "RIA-2026-Q4",
  date-str: "2026-11-06",
  citation: "SEC Rule 206(4)-2",
  seal-path: "{seal_legal}",
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#b45309"),
)
#v(5pt)

#hde-masthead-variation(
  variant: "executive_split",
  title: "6. Clinical Clean & Executive Split Accent Bar",
  subtitle: "Medical CPT/ICD-10 superbills, No Surprises Act GFEs, dental treatment plans & home inspections",
  org-name: "Lookout Orthopedic Physical Therapy",
  vertical-badge: "VARIANT 06 · CLINICAL / EXEC",
  doc-ref: "NPI-1487902145",
  date-str: "2026-11-06",
  citation: "45 C.F.R. § 149.610",
  seal-path: "{seal_med}",
  primary-color: rgb("#0f766e"),
  accent-color: rgb("#059669"),
)
"##,
            graphics_lib = HALLMAN_EMBEDDED_GRAPHICS_TYP,
            seal_trades = escape_typst_string_literal(&sp("seal_trades_construction.png")),
            seal_tax = escape_typst_string_literal(&sp("seal_property_tax_cad.png")),
            seal_gov = escape_typst_string_literal(&sp("seal_consulting_govcon.png")),
            seal_saas = escape_typst_string_literal(&sp("seal_saas_cybersecurity.png")),
            seal_legal = escape_typst_string_literal(&sp("seal_legal_cpa_advisory.png")),
            seal_med = escape_typst_string_literal(&sp("seal_medical_clinical.png")),
        ),

        "tables_and_kpis" => format!(
            r##"{graphics_lib}
#set page(paper: "us-letter", margin: 0.42in)
#set text(font: hde-font-sans, size: 9pt)

#rect(width: 100%, fill: rgb("#0f172a"), inset: (x: 12pt, y: 8pt), radius: 4pt)[
  #text(font: hde-font-sans, size: 12pt, weight: "black", fill: rgb("#00f0ff"))[
    HALLMAN DOC ENGINE — KPI STRIPS, TABLES, CHARTS & TIMELINE VARIATIONS
  ] \
  #text(font: hde-font-mono, size: 7.5pt, fill: white)[
    Multi-Style Financial Strips · Schedule Tables · Comparative Bar Charts · Phase Timelines
  ]
]
#v(4pt)

#text(font: hde-font-mono, size: 7.5pt, weight: "bold", fill: rgb("#1e3a8a"))[A. KPI STRIP VARIATION 1: EXECUTIVE METRIC CARDS]
#hde-kpi-strip-variation(
  variant: "metric_cards",
  metrics: (("CONTRACT SUM", "$1,480,000"), ("BILLED TO DATE", "$962,000 (65%)"), ("RETAINAGE (5%)", "$48,100"), ("CURRENT DRAW DUE", "$184,350")),
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
)

#text(font: hde-font-mono, size: 7.5pt, weight: "bold", fill: rgb("#0f172a"))[B. KPI STRIP VARIATION 2: DARK TERMINAL TELEMETRY STRIP]
#hde-kpi-strip-variation(
  variant: "dark_terminal",
  metrics: (("EQUIP SERIAL", "TRANE 4TWX80"), ("DELTA-T SPLIT", "19.6°F PASS"), ("R-410A SUBCOOL", "11.4°F SPEC"), ("PLATINUM REPAIR", "$1,185.00")),
  primary-color: rgb("#0f172a"),
  accent-color: rgb("#059669"),
)

#text(font: hde-font-mono, size: 7.5pt, weight: "bold", fill: rgb("#000000"))[C. KPI STRIP VARIATION 3: NEO-BRUTALIST HARD-BORDER STRIP]
#hde-kpi-strip-variation(
  variant: "neo_brutalist",
  metrics: (("COUNTY APPRAISAL", "$482,500"), ("PEER MEDIAN $/SF", "$184.10/SF"), ("CONTENDED FMV", "$394,200"), ("ANNUAL SAVINGS", "$493.80 / YR")),
  primary-color: rgb("#000000"),
  accent-color: rgb("#00f0ff"),
)

#text(font: hde-font-mono, size: 7.5pt, weight: "bold", fill: rgb("#1e3a8a"))[D. SCHEDULE TABLE VARIATION 1: DARK INSTITUTIONAL HEADER + ZEBRA STRIPES]
#hde-schedule-table-variation(
  variant: "dark_header",
  rows: (
    ("DIV-03", "Structural Concrete Foundations, Footings & Post-Tensioned Slab", "100% Complete", "$245,000.00"),
    ("DIV-05", "Structural Steel Framing, Composite Decking & Moment Welds", "85% Complete", "$318,750.00"),
    ("DIV-23", "Commercial Rooftop VRF HVAC & Insulated Sheet Metal Ductwork", "60% Complete", "$234,000.00"),
  ),
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
)

#text(font: hde-font-mono, size: 7.5pt, weight: "bold", fill: rgb("#0f766e"))[E. SCHEDULE TABLE VARIATION 2: MINIMAL EDITORIAL HORIZONTAL RULES]
#hde-schedule-table-variation(
  variant: "minimal_lines",
  rows: (
    ("CPT-97162", "Physical Therapy Evaluation: Moderate Complexity (45 Min 1-on-1)", "1 Unit · POS 11", "$145.00"),
    ("CPT-97110", "Therapeutic Exercise for Strength, Endurance & ROM (GP Mod)", "2 Units (30m)", "$80.00"),
  ),
  primary-color: rgb("#0f766e"),
  accent-color: rgb("#059669"),
)

#hde-comparative-bar-chart(
  title: "F. EMBEDDED VECTOR COMPARATIVE BAR CHART",
  rows: (
    ("County Notice Appraised Value", "$482,500.00", 100, "primary"),
    ("Neighborhood Equity Median (5 Comps)", "$405,020.00", 84, "primary"),
    ("Contended Fair Market Valuation", "$394,200.00", 81, "accent"),
  ),
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
)

#hde-milestone-timeline(
  title: "G. 4-PHASE EXECUTION & STATUTORY MILESTONE STRIP",
  steps: (
    ("Phase 1: Kickoff", "50% deposit & CAD lock", "WEEK 1"),
    ("Phase 2: Rough-In", "Municipal framing pass", "WEEK 3"),
    ("Phase 3: Trim & QA", "Punchlist walkthrough", "WEEK 5"),
    ("Phase 4: Final Lien", "Unconditional release", "WEEK 6"),
  ),
  primary-color: rgb("#1e3a8a"),
  accent-color: rgb("#059669"),
)
"##,
            graphics_lib = HALLMAN_EMBEDDED_GRAPHICS_TYP,
        ),

        "diagrams_and_stamps" => format!(
            r##"{graphics_lib}
#set page(paper: "us-letter", margin: 0.38in)
#set text(font: hde-font-sans, size: 8.5pt)

#rect(width: 100%, fill: rgb("#0f172a"), inset: (x: 12pt, y: 7pt), radius: 4pt)[
  #text(font: hde-font-sans, size: 11.5pt, weight: "black", fill: rgb("#ffe500"))[
    HALLMAN DOC ENGINE — 6 EMBEDDED SVG TECHNICAL DIAGRAMS & VECTOR RUBBER STAMPS
  ]
]
#v(3pt)

#grid(
  columns: (1fr, 1fr),
  gutter: 8pt,
  svg-architectural-diagram(
    kind: "parcel_lot",
    title: "1. CADASTRAL PARCEL & SETBACK PLAT",
    primary-hex: "#1e3a8a",
    accent-hex: "#d97706",
    metric-a: "2,200 SF GLA",
    metric-b: "NAD83 PLAT",
  ),
  svg-architectural-diagram(
    kind: "roof_pitch",
    title: "2. ROOF PITCH & HAIL IMPACT SCHEMATIC",
    primary-hex: "#1e3a8a",
    accent-hex: "#dc2626",
    metric-a: "34.2 SQUARES",
    metric-b: "IRC R905 DRIP",
  ),
  svg-architectural-diagram(
    kind: "hvac_diagnostic",
    title: "3. HVAC AIRFLOW & SUPERHEAT TELEMETRY",
    primary-hex: "#0f172a",
    accent-hex: "#059669",
    metric-a: "DELTA-T 19.6°F",
    metric-b: "R-410A PASS",
  ),
  svg-architectural-diagram(
    kind: "solar_array",
    title: "4. SOLAR PV ARRAY & LFP STORAGE TOPOLOGY",
    primary-hex: "#1e3a8a",
    accent-hex: "#059669",
    metric-a: "10.56 kW DC",
    metric-b: "15,840 kWh/YR",
  ),
  svg-architectural-diagram(
    kind: "vehicle_inspection",
    title: "5. AUTOMOTIVE & FLEET CHASSIS DIAGNOSTIC",
    primary-hex: "#0f172a",
    accent-hex: "#d97706",
    metric-a: "DOT 49 CFR",
    metric-b: "740 CCA PASS",
  ),
  svg-architectural-diagram(
    kind: "floorplan_spec",
    title: "6. COMMERCIAL BOMA RSF SUITE ZONING",
    primary-hex: "#1e3a8a",
    accent-hex: "#b45309",
    metric-a: "3,610 RSF",
    metric-b: "14% LOAD FAC",
  ),
)

#v(6pt)
#rect(width: 100%, stroke: 0.75pt + rgb("#cbd5e1"), radius: 4pt, fill: rgb("#f8fafc"), inset: 10pt)[
  #text(font: hde-font-sans, size: 8pt, weight: "bold", fill: rgb("#0f172a"))[
    EMBEDDED VECTOR SVG RUBBER STAMPS (ZERO RASTER BLUR AT ANY ZOOM)
  ]
  #v(8pt)
  #grid(
    columns: (1fr, 1fr, 1fr),
    gutter: 12pt,
    align: center + horizon,
    svg-rubber-stamp(label: "PAID IN FULL", sublabel: "ACH SETTLED · $0 DUE", color-hex: "#059669", angle: -5deg),
    svg-rubber-stamp(label: "PRO SE PETITION", sublabel: "T.C.A. § 67-5-1407", color-hex: "#1e3a8a", angle: -4deg),
    svg-rubber-stamp(label: "$0.00 PRO-BONO", sublabel: "100% P&T VETERAN", color-hex: "#b45309", angle: -6deg),
    svg-rubber-stamp(label: "CONDITIONAL WAIVER", sublabel: "UPON GOOD FUNDS", color-hex: "#d97706", angle: -4deg),
    svg-rubber-stamp(label: "IRC R905 CODE", sublabel: "SUPPLEMENT APPROVED", color-hex: "#dc2626", angle: -5deg),
    svg-rubber-stamp(label: "SOC2 / HIPAA", sublabel: "0 CRITICAL / 0 HIGH", color-hex: "#0f766e", angle: -5deg),
  )
]
"##,
            graphics_lib = HALLMAN_EMBEDDED_GRAPHICS_TYP,
        ),

        _ => format!(
            r##"{graphics_lib}
#set page(paper: "us-letter", margin: 0.34in)
#set text(font: hde-font-sans, size: 8pt)

#rect(width: 100%, fill: rgb("#0f172a"), inset: (x: 10pt, y: 6pt), radius: 4pt)[
  #text(font: hde-font-sans, size: 11pt, weight: "black", fill: rgb("#00f0ff"))[
    HALLMAN DOC ENGINE — 10 NANO BANANA VERTICAL SEALS & 3 EXECUTION MODES
  ]
]
#v(4pt)

#rect(width: 100%, stroke: 0.75pt + rgb("#cbd5e1"), radius: 4pt, fill: white, inset: 6pt)[
  #text(font: hde-font-sans, size: 7.4pt, weight: "bold", fill: rgb("#1e3a8a"))[
    NANO BANANA DIE-CUT VECTOR-STYLE CERTIFICATION SEALS ACROSS ALL 10 PROFESSION VERTICALS
  ]
  #v(4pt)
  #grid(
    columns: (1fr, 1fr, 1fr, 1fr, 1fr),
    gutter: 6pt,
    align: center + top,
    [
      #image("{s1}", width: 42pt, height: 42pt) \
      #text(size: 6.2pt, weight: "bold")[1. Construction & Trades]
    ],
    [
      #image("{s2}", width: 42pt, height: 42pt) \
      #text(size: 6.2pt, weight: "bold")[2. Real Estate & Title]
    ],
    [
      #image("{s3}", width: 42pt, height: 42pt) \
      #text(size: 6.2pt, weight: "bold")[3. Property Tax & CAD]
    ],
    [
      #image("{s4}", width: 42pt, height: 42pt) \
      #text(size: 6.2pt, weight: "bold")[4. SaaS & Cybersecurity]
    ],
    [
      #image("{s5}", width: 42pt, height: 42pt) \
      #text(size: 6.2pt, weight: "bold")[5. Creative & Events]
    ],
    [
      #image("{s6}", width: 42pt, height: 42pt) \
      #text(size: 6.2pt, weight: "bold")[6. Legal, CPA & Advisory]
    ],
    [
      #image("{s7}", width: 42pt, height: 42pt) \
      #text(size: 6.2pt, weight: "bold")[7. Medical & Clinical]
    ],
    [
      #image("{s8}", width: 42pt, height: 42pt) \
      #text(size: 6.2pt, weight: "bold")[8. Logistics & Freight]
    ],
    [
      #image("{s9}", width: 42pt, height: 42pt) \
      #text(size: 6.2pt, weight: "bold")[9. Solar & Inspection]
    ],
    [
      #image("{s10}", width: 42pt, height: 42pt) \
      #text(size: 6.2pt, weight: "bold")[10. Consulting & GovCon]
    ],
  )
]

#v(3pt)
#text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: rgb("#1e3a8a"))[MODE 1: INTERACTIVE /ACROFORM FILLABLE WET-INK EXECUTION BLOCK]
#jurat-box(
  signer-name: "Lookout Valley Commercial Builders LLC",
  signer-role: "Licensed General Contractor",
  counterparty-name: "Riverfront Development SPE LLC",
  signature-type: "wet-ink",
  notary-required: false,
  stamp-label: "ACROFORM READY",
  stamp-sublabel: "TYPE OR SIGN IN PDF",
  stamp-color: "#059669",
  primary-color: rgb("#1e3a8a"),
)

#text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: rgb("#0f766e"))[MODE 2: TIER-1 ESIGN / UETA CRYPTOGRAPHIC COUNTER-SIGNATURE]
#jurat-box(
  signer-name: "Samantha J. Chen, VP of Engineering",
  signer-role: "Enterprise SaaS Subscriber",
  counterparty-name: "VectorPulse Cloud Systems Inc.",
  signature-type: "esign",
  notary-required: false,
  stamp-label: "ESIGN VERIFIED",
  stamp-sublabel: "SHA-256 AUDIT TRAIL",
  stamp-color: "#0284c7",
  primary-color: rgb("#0f766e"),
)

#text(font: hde-font-mono, size: 6.8pt, weight: "bold", fill: rgb("#b45309"))[MODE 3: TIER-2 STATUTORY SWORN NOTARY PUBLIC JURAT + SEAL BLOCK]
#jurat-box(
  signer-name: "Chattanooga Ready-Mix & Structural Supply Co.",
  signer-role: "Lien Claimant / Material Supplier",
  counterparty-name: "First American Title Insurance Co.",
  signature-type: "wet-ink",
  notary-required: true,
  stamp-label: "NOTARY JURAT",
  stamp-sublabel: "T.C.A. § 66-11-101",
  stamp-color: "#b45309",
  primary-color: rgb("#1e3a8a"),
)
"##,
            graphics_lib = HALLMAN_EMBEDDED_GRAPHICS_TYP,
            s1 = escape_typst_string_literal(&sp("seal_trades_construction.png")),
            s2 = escape_typst_string_literal(&sp("seal_real_estate_title.png")),
            s3 = escape_typst_string_literal(&sp("seal_property_tax_cad.png")),
            s4 = escape_typst_string_literal(&sp("seal_saas_cybersecurity.png")),
            s5 = escape_typst_string_literal(&sp("seal_creative_events.png")),
            s6 = escape_typst_string_literal(&sp("seal_legal_cpa_advisory.png")),
            s7 = escape_typst_string_literal(&sp("seal_medical_clinical.png")),
            s8 = escape_typst_string_literal(&sp("seal_logistics_freight.png")),
            s9 = escape_typst_string_literal(&sp("seal_solar_inspection.png")),
            s10 = escape_typst_string_literal(&sp("seal_consulting_govcon.png")),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_catalog_contains_50_unique_concepts_across_10_verticals() {
        assert_eq!(ARCHETYPE_CONCEPTS_50.len(), 50);
        let mut ids = HashSet::new();
        let mut slugs = HashSet::new();
        let mut verticals = HashSet::new();

        for (idx, c) in ARCHETYPE_CONCEPTS_50.iter().enumerate() {
            assert_eq!(c.id as usize, idx + 1, "Concept IDs must be sequential 1..=50");
            assert!(ids.insert(c.id), "Duplicate concept id {}", c.id);
            assert!(slugs.insert(c.slug), "Duplicate concept slug {}", c.slug);
            verticals.insert(c.vertical_slug);
            assert!(!c.title.is_empty());
            assert!(!c.primary_keyword.is_empty());
            assert!(!c.demand_signal.is_empty());
            assert!(!c.statutory_citation.is_empty());
        }
        assert_eq!(verticals.len(), 10, "Must span 10 distinct profession verticals");
    }

    #[test]
    fn test_find_concept_by_slug_or_id() {
        let c1 = find_concept_by_slug("1").expect("concept 1");
        assert_eq!(c1.id, 1);
        let c11 = find_concept_by_slug("board-of-equalization-property-tax-appeal-petition-comp-grid")
            .expect("concept 11");
        assert_eq!(c11.id, 11);
    }

    #[test]
    fn test_render_all_50_concepts_markup_valid() {
        for spec in ARCHETYPE_CONCEPTS_50 {
            let markup = render_concept_typst_markup(spec, None, "/templates/assets/seals");
            assert!(markup.contains("#hde-masthead-variation("));
            assert!(markup.contains("#svg-architectural-diagram("));
            assert!(markup.contains("#jurat-box("));
        }
    }
}



