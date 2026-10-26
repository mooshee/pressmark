// Pressmark Studio - High-Performance Interactive Controller
// Author: Daniel Hallman
// Sub-Millisecond Native Document Engine & Real-Time Studio

(function() {
  'use strict';

  // Application State
  const state = {
    doc_type: 'invoice',
    title: 'TAX INVOICE',
    header_subtitle: 'Enterprise Infrastructure & Systems Engineering',
    doc_number: 'INV-2026-0042',
    issue_date: new Date().toISOString().split('T')[0],
    due_date: new Date(Date.now() + 14 * 86400000).toISOString().split('T')[0],
    theme: 'modern_slate',
    primary_color: '#4f46e5',
    secondary_color: '#475569',
    accent_color: '#06b6d4',
    surface_color: '#f8fafc',
    layout_style: 'standard',
    font_family: 'sans',
    font_scale: 'standard',
    spacing_density: 'standard',
    paper_size: 'us-letter',
    background_style: 'white',
    footer_text: 'CONFIDENTIAL & PROPRIETARY · PREPARED FOR VANCE DYNAMICS',
    show_page_numbers: true,
    show_footer_divider: true,
    logo_data: null,
    from: {
      name: 'Pressmark Studio Inc.',
      company: 'Pressmark Technologies LLC',
      address: '742 Evergreen Terrace, Suite 400\nSan Francisco, CA 94107',
      email: 'billing@pressmark.dev',
      phone: '+1 (415) 890-4421',
      tax_id: 'US-94-3829104',
      website: 'https://pressmark.dev',
      registration_number: 'REG-84920'
    },
    to: {
      name: 'Alexandria Vance',
      company: 'Vance Dynamics Corp',
      address: '100 Innovation Parkway, Floor 12\nAustin, TX 78701',
      email: 'finance@vancedynamics.com',
      phone: '+1 (512) 555-0199',
      tax_id: 'US-74-8891024',
      website: 'https://vancedynamics.com',
      registration_number: 'CRN-09281'
    },
    items: [
      { description: 'High-Performance Rust Backend Architecture', quantity: 32.0, unit_price: 185.0, discount: 0.0 },
      { description: 'Pressmark Vector Document Synthesis Engine Integration', quantity: 24.0, unit_price: 185.0, discount: 0.0 },
      { description: 'ChatGPT Actions & MCP Protocol Orchestration', quantity: 16.0, unit_price: 185.0, discount: 0.0 }
    ],
    currency: 'USD',
    currency_symbol_mode: 'symbol',
    currency_placement: 'prefix',
    currency_separators: 'standard',
    date_format: 'iso',
    phone_format: 'us_standard',
    number_decimals: 2,
    unit_measure: 'units',
    tax_rate: 8.25,
    discount_amount: 250.0,
    notes: 'Thank you for your business. Payment is due within 14 calendar days via ACH, Wire, or Credit Card.',
    terms: 'Late payments are subject to a 1.5% compounding monthly administrative surcharge.',
    custom_fields: [
      { label: 'Project Code', value: 'PRJ-NEO-2026' },
      { label: 'PO Reference', value: 'PO-88291' }
    ],
    sections: [
      {
        title: '1. SCOPE OF ENGAGEMENT & ACCEPTANCE',
        content: 'Contractor delivers high-throughput vector synthesis pipelines, real-time client studio instrumentation, and automated regression test harnesses.'
      }
    ],
    signatures: [
      {
        signer_name: 'Daniel Hallman',
        signer_title: 'Managing Principal',
        date: new Date().toISOString().split('T')[0],
        signature_data: null
      }
    ],
    is_paid: false,
    payment_link: 'https://buy.stripe.com/4gM3cu8jk4Psd095197wA04',
    payment_qr_code: true,
    payment_status: 'unpaid',
    payment_method: 'Stripe / Card / Apple Pay',
    paid_at: null,
    transaction_reference: null,
    amount_paid: null,
    watermark: '',
    compliance_mode: 'none',
    show_qr_seal: true,
    show_signature: true,
    show_remittance_stub: false,
    show_line_numbers: false
  };

  let debounceTimer = null;
  let currentZoom = 100;

  // Undo & Redo History State
  const undoStack = [];
  const redoStack = [];
  const MAX_UNDO = 60;

  // Interactive Canvas & Selection State
  let interactiveMode = true;
  let selectedElement = null; // { type, index, card }
  let dragSource = null; // { type, index, fromPreview }

  // Multi-Page Preview State
  let currentPages = [];
  let currentPageIndex = 0;
  let currentTotalPages = 1;
  let pageViewMode = 'stack'; // 'stack' | 'single'

  // Persistent Storage Keys
  const STORAGE_KEY_ISSUER = 'pressmark_issuer_profile';
  const STORAGE_KEY_COUNTERPARTIES = 'pressmark_counterparties';
  const STORAGE_KEY_USER_DOC = 'pressmark_user_active_doc';

  // Data Persistence Mode: 'my_data' (stable & protected) vs 'sample_data' (exploring demo data)
  let dataMode = 'my_data';
  let cachedUserDoc = null;
  let hasUserCustomizedData = false;
  let hasUserCustomizedItems = false;

  // DOM Elements
  const previewImg = document.getElementById('previewImg');
  const previewDocWrapper = document.getElementById('previewDocWrapper');
  const previewPagesContainer = document.getElementById('previewPagesContainer');
  const previewInteractiveOverlay = document.getElementById('previewInteractiveOverlay');
  const compileTelemetry = document.getElementById('compileTelemetry');
  const itemsTbody = document.getElementById('itemsTbody');
  const subtotalDisplay = document.getElementById('subtotalDisplay');
  const taxDisplay = document.getElementById('taxDisplay');
  const discountDisplay = document.getElementById('discountDisplay');
  const totalDisplay = document.getElementById('totalDisplay');
  const pageNavControls = document.getElementById('pageNavControls');
  const btnPrevPage = document.getElementById('btnPrevPage');
  const btnNextPage = document.getElementById('btnNextPage');
  const pageDisplay = document.getElementById('pageDisplay');
  const btnViewStack = document.getElementById('btnViewStack');
  const btnViewSingle = document.getElementById('btnViewSingle');
  const cardLineItems = document.getElementById('cardLineItems');
  const lineItemsCardTitle = document.getElementById('lineItemsCardTitle');
  const btnOpenArchetypeModal = document.getElementById('btnOpenArchetypeModal');
  const btnModeMyData = document.getElementById('btnModeMyData');
  const btnModeSampleData = document.getElementById('btnModeSampleData');
  const btnSaveIssuerProfile = document.getElementById('btnSaveIssuerProfile');
  const btnResetIssuerProfile = document.getElementById('btnResetIssuerProfile');
  const counterpartyQuickSelect = document.getElementById('counterpartyQuickSelect');
  const btnSaveCounterparty = document.getElementById('btnSaveCounterparty');
  const btnClearCounterparty = document.getElementById('btnClearCounterparty');

  // Multi-Color Theme Palettes (Tokens matching Pressmark Vector Engine)
  const THEME_PALETTES = {
    modern_slate: {
      name: 'Modern Slate',
      primary: '#4f46e5',
      secondary: '#475569',
      accent: '#06b6d4',
      surface: '#f8fafc',
    },
    executive_navy: {
      name: 'Executive Navy',
      primary: '#1e3a8a',
      secondary: '#d4af37',
      accent: '#c59b27',
      surface: '#fdfbf7',
    },
    monaco_gold: {
      name: 'Monaco Sovereign',
      primary: '#ca8a04',
      secondary: '#18181b',
      accent: '#eab308',
      surface: '#fffdf5',
    },
    crimson_legal: {
      name: 'Crimson Legal',
      primary: '#b91c1c',
      secondary: '#b45309',
      accent: '#dc2626',
      surface: '#fffbeb',
    },
    bordeaux: {
      name: 'Bordeaux Luxury',
      primary: '#831843',
      secondary: '#be185d',
      accent: '#db2777',
      surface: '#fdf2f8',
    },
    warm_sand: {
      name: 'Warm Amber',
      primary: '#b45309',
      secondary: '#78350f',
      accent: '#f59e0b',
      surface: '#fefce8',
    },
    desert_sand: {
      name: 'Mojave Desert',
      primary: '#d97706',
      secondary: '#78350f',
      accent: '#f59e0b',
      surface: '#fefce8',
    },
    cyber_neon: {
      name: 'Cyber Cyan',
      primary: '#06b6d4',
      secondary: '#7c3aed',
      accent: '#ec4899',
      surface: '#f8fafc',
    },
    emerald: {
      name: 'Emerald Growth',
      primary: '#059669',
      secondary: '#10b981',
      accent: '#34d399',
      surface: '#f0fdf4',
    },
    nordic_teal: {
      name: 'Nordic Teal',
      primary: '#0d9488',
      secondary: '#14b8a6',
      accent: '#2dd4bf',
      surface: '#f0fdfa',
    },
    forest_pine: {
      name: 'Alpine Forest',
      primary: '#166534',
      secondary: '#4d7c0f',
      accent: '#15803d',
      surface: '#f7fee7',
    },
    royal_violet: {
      name: 'Royal Violet',
      primary: '#7c3aed',
      secondary: '#a855f7',
      accent: '#ec4899',
      surface: '#faf5ff',
    },
    sunset_coral: {
      name: 'Sunset Coral',
      primary: '#e11d48',
      secondary: '#d97706',
      accent: '#e11d48',
      surface: '#fff7ed',
    },
    electric_indigo: {
      name: 'Electric Indigo',
      primary: '#4338ca',
      secondary: '#6366f1',
      accent: '#818cf8',
      surface: '#eef2ff',
    },
    neobrutalist: {
      name: 'Neobrutalist',
      primary: '#000000',
      secondary: '#facc15',
      accent: '#2563eb',
      surface: '#ffffff',
    },
    bauhaus_primary: {
      name: 'Bauhaus Triadic',
      primary: '#2563eb',
      secondary: '#b91c1c',
      accent: '#eab308',
      surface: '#f8fafc',
    },
    obsidian: {
      name: 'Dark Obsidian',
      primary: '#18181b',
      secondary: '#3f3f46',
      accent: '#71717a',
      surface: '#fafafa',
    },
    swiss_minimal: {
      name: 'Swiss Minimal',
      primary: '#334155',
      secondary: '#dc2626',
      accent: '#71717a',
      surface: '#fafafa',
    },
  };

  // Pluggable Component System & Archetype Layout Variants
  const COMPONENT_SYSTEM = {
    header: {
      name: 'Document Header & Identity',
      universal: true,
      variants: [
        { id: 'standard', label: 'Standard Clean Header' },
        { id: 'minimal', label: 'Minimalist Direct' },
        { id: 'modern_banner', label: 'Modern Accent Banner' },
        { id: 'classic_letterhead', label: 'Corporate Letterhead' },
        { id: 'editorial_centered', label: 'Editorial Centered' },
        { id: 'split_sidebar', label: 'Split Sidebar' },
        { id: 'swiss_grid', label: 'Swiss Grid' },
        { id: 'compact_ledger', label: 'Compact Ledger' },
        { id: 'framed', label: 'Executive Border Frame' },
      ],
      current: () => state.layout_style || 'standard',
      apply: (variant) => {
        state.layout_style = variant;
        const sel = document.getElementById('layoutStyleSelect');
        if (sel) sel.value = variant;
      }
    },
    from: {
      name: 'Issuer Party Card',
      universal: true,
      variants: [
        { id: 'standard', label: 'Standard Identity Card' },
        { id: 'badged', label: 'Badged Primary Bar' },
      ],
      current: () => 'standard',
      apply: () => {}
    },
    to: {
      name: 'Recipient Party Card',
      universal: true,
      variants: [
        { id: 'standard', label: 'Standard Recipient Card' },
        { id: 'badged', label: 'Badged Secondary Bar' },
      ],
      current: () => 'standard',
      apply: () => {}
    },
    item: {
      name: 'Line Items Ledger Row',
      universal: false,
      canDelete: true,
      canDuplicate: true,
    },
    totals: {
      name: 'Financial Summary & Totals',
      universal: false,
      variants: [
        { id: 'standard', label: 'Standard Balanced Totals' },
        { id: 'compact', label: 'Compact Totals Bar' },
      ],
      current: () => 'standard',
      apply: () => {}
    },
    section: {
      name: 'Custom Clause & Section',
      universal: true,
      canDelete: true,
      canDuplicate: true,
    },
    signature: {
      name: 'Signatory Attestation Block',
      universal: true,
      canDelete: true,
    },
    custom_field: {
      name: 'Custom Metadata Attribute',
      universal: true,
      canDelete: true,
    },
    notes: {
      name: 'Payment Instructions & Notes',
      universal: true,
      canDelete: true,
    },
    terms: {
      name: 'Statutory Terms & Covenants',
      universal: true,
      canDelete: true,
    },
  };

  // Curated Industry Scenes for Randomization
  const RANDOM_SCENES = [
    {
      doc_type: 'invoice',
      title: 'TAX INVOICE',
      header_subtitle: 'Avionics Firmware & Edge Telemetry Infrastructure',
      prefix: 'INV-',
      theme: 'modern_slate',
      primary_color: '#4f46e5',
      layout_style: 'standard',
      background_style: 'white',
      footer_text: 'PROPRIETARY FLIGHT SYSTEMS TELEMETRY · AUDITED RECORD',
      currency: 'USD',
      from: { name: 'Kensho AI Lab', company: 'Kensho Robotics LLC', address: '500 Howard St, Floor 8\nSan Francisco, CA 94105', email: 'billing@kensho.ai', phone: '+1 415-555-0182', tax_id: 'US-88-291039' },
      to: { name: 'Elena Rostova', company: 'Apex Autonomous Flight', address: '2200 Space Park Way\nSeattle, WA 98101', email: 'ap@apexflight.io', phone: '+1 206-555-4920', tax_id: 'US-91-884910' },
      items: [
        { description: 'Autonomous Flight Computer Firmware & Avionics Bus', quantity: 45.0, unit_price: 220.0, discount: 0 },
        { description: 'Real-Time Edge Sensor Fusion & Telemetry Pipeline', quantity: 30.0, unit_price: 220.0, discount: 0 }
      ],
      custom_fields: [
        { label: 'AVIONICS BUS', value: 'ARINC-429-QUAD' },
        { label: 'FAA AIRWORTHINESS', value: 'PART-23-CERT' }
      ],
      sections: [
        {
          title: '1. AVIONICS HARDENING SPECIFICATIONS',
          content: 'Firmware images are cryptographically signed with ECDSA-P256 keys and pre-verified against DO-178C Level A verification criteria.'
        }
      ],
      signatures: [
        { signer_name: 'Dr. Hiroshi Tanaka', signer_title: 'Chief Avionics Engineer', date: new Date().toISOString().split('T')[0], signature_data: null },
        { signer_name: 'Elena Rostova', signer_title: 'VP Flight Operations', date: new Date().toISOString().split('T')[0], signature_data: null }
      ],
      tax_rate: 9.5,
      discount_amount: 500,
      compliance_mode: 'pci_dss',
      show_qr_seal: true,
      show_signature: true,
      show_remittance_stub: true
    },
    {
      doc_type: 'agreement',
      title: 'MASTER SERVICES AGREEMENT',
      header_subtitle: 'Admiralty, Maritime Cabotage & Trade Sanctions Retainer',
      prefix: 'MSA-',
      theme: 'executive_navy',
      primary_color: '#1e3a8a',
      layout_style: 'framed',
      background_style: 'parchment',
      footer_text: 'PRIVILEGED & CONFIDENTIAL · SOLICITOR-CLIENT RETENTION',
      currency: 'GBP',
      from: { name: 'Kingsley & Thorne LLP', company: 'Kingsley Maritime Counsel', address: '14 Chancery Lane\nLondon, WC2A 1PL, UK', email: 'accounts@kingsleythorne.co.uk', phone: '+44 20 7946 0912', tax_id: 'GB-992-1849-01' },
      to: { name: 'Lord Marcus Sterling', company: 'Borealis Global Logistics Plc', address: '88 St Mary Axe, Level 32\nLondon, EC3A 8EP', email: 'procurement@borealislogistics.com', phone: '+44 20 7946 8820', tax_id: 'GB-812-4029-44' },
      items: [
        { description: 'Admiralty & Cabotage Retainer Services (Q4)', quantity: 1.0, unit_price: 18500.0, discount: 0 },
        { description: 'Statutory Maritime Sanctions Regulatory Review', quantity: 12.0, unit_price: 650.0, discount: 0 }
      ],
      custom_fields: [
        { label: 'LLOYDS REGISTER', value: 'LR-LON-8891' },
        { label: 'JURISDICTION', value: 'ENGLAND & WALES' }
      ],
      sections: [
        {
          title: '1. MARITIME LEGAL RETAINER & COVENANTS',
          content: 'Counsel shall represent Client before the High Court of Justice, Queen\'s Bench Division (Admiralty Court) in all matters relating to salvage and charterparty disputes.'
        },
        {
          title: '2. STATUTORY SANCTIONS COMPLIANCE',
          content: 'All cargo manifests shall undergo automated screening against OFSI and EU consolidated asset-freeze lists prior to vessel departure.'
        }
      ],
      signatures: [
        { signer_name: 'Sir Arthur Kingsley, KC', signer_title: 'Senior Partner', date: new Date().toISOString().split('T')[0], signature_data: null },
        { signer_name: 'Lord Marcus Sterling', signer_title: 'Executive Chairman', date: new Date().toISOString().split('T')[0], signature_data: null },
        { signer_name: 'Victoria Vance', signer_title: 'Notary Public of London', date: new Date().toISOString().split('T')[0], signature_data: null }
      ],
      tax_rate: 20.0,
      discount_amount: 0,
      compliance_mode: 'statutory_sworn',
      show_qr_seal: true,
      show_signature: true,
      show_remittance_stub: false
    },
    {
      doc_type: 'estimate',
      title: 'COMMERCIAL CONSTRUCTION ESTIMATE',
      header_subtitle: 'Structural Steel Framing & Grade Beam Fabrication Scope',
      prefix: 'EST-',
      theme: 'warm_sand',
      primary_color: '#b45309',
      layout_style: 'standard',
      background_style: 'slate_tint',
      footer_text: 'CANYON PEAK BUILDERS · LICENSED & BONDED GENERAL CONTRACTOR',
      currency: 'USD',
      from: { name: 'Canyon Peak Builders', company: 'Canyon Peak Construction Co.', address: '4400 Industrial Blvd\nDenver, CO 80216', email: 'estimating@canyonpeak.com', phone: '+1 303-555-0914', tax_id: 'US-84-1920394' },
      to: { name: 'David Mercer', company: 'Summit Creek Commercial Properties', address: '1200 17th St, Suite 500\nDenver, CO 80202', email: 'dmercer@summitcreek.com', phone: '+1 303-555-8831', tax_id: 'US-84-9910294' },
      items: [
        { description: 'Structural Steel Framing & Grade Beam Fabrication', quantity: 1.0, unit_price: 64500.0, discount: 0 },
        { description: 'Reinforced Post-Tension Foundation Pour & Cure', quantity: 1.0, unit_price: 42000.0, discount: 2500 }
      ],
      custom_fields: [
        { label: 'PERMIT ID', value: 'DEN-BLD-2026-908' },
        { label: 'OSHA COMPLIANCE', value: 'CERT-1926-PASS' }
      ],
      sections: [
        {
          title: '1. MATERIAL CONTINGENCIES & SITE LOGISTICS',
          content: 'Price reflects mill certifications for ASTM A992 structural steel with all testing performed by third-party geotechnical inspectors.'
        }
      ],
      signatures: [
        { signer_name: 'Frank Galli', signer_title: 'Lead Estimator & PM', date: new Date().toISOString().split('T')[0], signature_data: null },
        { signer_name: 'David Mercer', signer_title: 'Property Principal', date: new Date().toISOString().split('T')[0], signature_data: null }
      ],
      tax_rate: 4.81,
      discount_amount: 2500,
      compliance_mode: 'none',
      show_qr_seal: true,
      show_signature: true,
      show_remittance_stub: true
    },
    {
      doc_type: 'retainer_agreement',
      title: 'QUANTITATIVE ASSET MANAGEMENT MANDATE',
      header_subtitle: 'Algorithmic Execution & Sovereign Risk Advisory',
      prefix: 'QAM-',
      theme: 'royal_violet',
      primary_color: '#7c3aed',
      layout_style: 'swiss_grid',
      background_style: 'marble_vellum',
      font_family: 'grotesk',
      font_scale: 'standard',
      spacing_density: 'compact',
      paper_size: 'a4',
      footer_text: 'CONFIDENTIAL PORTFOLIO MANDATE · REGULATED BY FINMA',
      currency: 'CHF',
      currency_symbol_mode: 'symbol',
      currency_placement: 'prefix',
      currency_separators: 'space',
      date_format: 'euro',
      number_decimals: 2,
      unit_measure: 'mo',
      from: { name: 'Dr. Beatrix von Berg', company: 'Helvetia Quantitative AG', address: 'Bahnhofstrasse 45\n8001 Zürich, Switzerland', email: 'mandates@helvetiaquant.ch', phone: '+41 44 215 5000', tax_id: 'CHE-109.876.543 MWST', website: 'https://helvetiaquant.ch', registration_number: 'CH-020.3.031.904-1' },
      to: { name: 'Henrietta Dupont', company: 'Geneva Private Wealth S.A.', address: 'Rue du Rhône 12\n1204 Genève', email: 'hdupont@genevawealth.ch', phone: '+41 22 819 9000', tax_id: 'CHE-412.098.112 MWST', website: 'https://genevawealth.ch', registration_number: 'CH-660.0.124.992-0' },
      items: [
        { description: 'Statistical Arbitrage Alpha Stream Mandate (Q4)', quantity: 3.0, unit_price: 35000.0, discount: 0 },
        { description: 'High-Frequency Risk Attribution Engine (Real-Time)', quantity: 3.0, unit_price: 12500.0, discount: 5000 }
      ],
      custom_fields: [
        { label: 'FINMA REGISTRATION', value: 'CHE-908.12' },
        { label: 'CUSTODIAN CLEARING', value: 'SIX-SIS-CH' }
      ],
      sections: [
        {
          title: '1. RISK CONSTRAINTS & MAXIMUM DRAWDOWN LIMITS',
          content: 'Value-at-Risk (99% 1-day horizon) shall not exceed 1.85% of Gross Portfolio Value under stressed Monte Carlo simulations.'
        }
      ],
      signatures: [
        { signer_name: 'Dr. Beatrix von Berg', signer_title: 'Chief Investment Officer', date: new Date().toISOString().split('T')[0], signature_data: null },
        { signer_name: 'Henrietta Dupont', signer_title: 'Managing Director', date: new Date().toISOString().split('T')[0], signature_data: null }
      ],
      tax_rate: 7.7,
      discount_amount: 5000,
      compliance_mode: 'statutory_sworn',
      show_qr_seal: true,
      show_signature: true,
      show_remittance_stub: false
    },
    {
      doc_type: 'work_order',
      title: 'QUANTUM SENSOR CALIBRATION WORK ORDER',
      header_subtitle: 'Sub-Kelvin Cryogenic Dilution Refrigerator Servicing',
      prefix: 'WO-',
      theme: 'cyber_neon',
      primary_color: '#06b6d4',
      layout_style: 'compact_ledger',
      background_style: 'blueprint',
      font_family: 'mono',
      font_scale: 'compact',
      spacing_density: 'compact',
      paper_size: 'a4',
      footer_text: 'QUANTUM SENSING LABS · ISO 17025 ACCREDITED METROLOGY',
      currency: 'JPY',
      currency_symbol_mode: 'symbol',
      currency_placement: 'prefix',
      currency_separators: 'standard',
      date_format: 'iso',
      number_decimals: 0,
      unit_measure: 'hrs',
      from: { name: 'Kenji Takahashi', company: 'Tokyo Quantum Instruments Inc.', address: '4-1-28 Toranomon, Minato-ku\nTokyo 105-0001, Japan', email: 'service@tokyoquantum.jp', phone: '+81 3 5555 0143', tax_id: 'JP-T1010001012345', website: 'https://tokyoquantum.jp', registration_number: 'T1010001012345' },
      to: { name: 'Dr. Ren Ishikawa', company: 'RIKEN Quantum Metrology Division', address: '2-1 Hirosawa, Wako\nSaitama 351-0198', email: 'lab-ops@riken.jp', phone: '+81 48 462 1111', tax_id: 'JP-T7030005007111', website: 'https://riken.jp', registration_number: 'T7030005007111' },
      items: [
        { description: 'Dilution Refrigerator 10mK Base Temperature Thermalization', quantity: 24.0, unit_price: 65000.0, discount: 0 },
        { description: 'Superconducting Qubit Flux Bias Coaxial Line S-Parameter Profiling', quantity: 16.0, unit_price: 85000.0, discount: 100000 }
      ],
      custom_fields: [
        { label: 'LAB CALIBRATION ID', value: 'NMIJ-QNT-8821' },
        { label: 'CRYOGENIC CYCLE', value: 'HE-3/HE-4-CLOSED' }
      ],
      sections: [
        {
          title: '1. ACCEPTANCE PROTOCOLS',
          content: 'Phase-coherent microwave transmission metrics must demonstrate resonator Q-factors exceeding 2.4 million at 4.2 Kelvin.'
        }
      ],
      signatures: [
        { signer_name: 'Kenji Takahashi', signer_title: 'Lead Cryogenic Metrologist', date: new Date().toISOString().split('T')[0], signature_data: null },
        { signer_name: 'Dr. Ren Ishikawa', signer_title: 'Division Chief', date: new Date().toISOString().split('T')[0], signature_data: null }
      ],
      tax_rate: 10.0,
      discount_amount: 100000,
      compliance_mode: 'soc2',
      show_qr_seal: true,
      show_signature: true,
      show_remittance_stub: false
    },
    {
      doc_type: 'proforma_invoice',
      title: 'PROFORMA EXPORT INVOICE',
      header_subtitle: 'Robotic Industrial Automation Cells & CNC Laser Cutters',
      prefix: 'PRO-',
      theme: 'nordic_teal',
      primary_color: '#0d9488',
      layout_style: 'classic_letterhead',
      background_style: 'linen_subtle',
      font_family: 'editorial',
      font_scale: 'standard',
      spacing_density: 'standard',
      paper_size: 'a4',
      footer_text: 'EXPORT CONTROL ECCN 2B001 · DDP INCOTERMS 2026',
      currency: 'EUR',
      currency_symbol_mode: 'symbol',
      currency_placement: 'suffix',
      currency_separators: 'euro',
      date_format: 'short_dot',
      number_decimals: 2,
      unit_measure: 'units',
      from: { name: 'Klaus Weber', company: 'Bavaria Mechatronik GmbH', address: 'Industriestraße 18\n80339 München, Deutschland', email: 'export@bavariamechatronik.de', phone: '+49 89 2018 7700', tax_id: 'DE 312 849 201', website: 'https://bavariamechatronik.de', registration_number: 'HRB 241908' },
      to: { name: 'Marco Vanezzi', company: 'Officine Meccaniche Milano S.p.A.', address: 'Via Tortona 54\n20144 Milano, Italia', email: 'ordini@vanezzi.it', phone: '+39 02 8940 1200', tax_id: 'IT 08921040962', website: 'https://vanezzi.it', registration_number: 'MI-190248' },
      items: [
        { description: '6-Axis High-Precision Articulated Robotics Cell (IP67)', quantity: 2.0, unit_price: 58500.0, discount: 0 },
        { description: 'Fiber Laser Cutting Head with Adaptive Optical Focusing', quantity: 2.0, unit_price: 19200.0, discount: 3000 }
      ],
      custom_fields: [
        { label: 'CUSTOMS TARIF', value: 'TARIC 8479.50.00' },
        { label: 'EUR-1 CERTIFICATE', value: 'DE-BY-94021' }
      ],
      sections: [
        {
          title: '1. EXPORT CONTROL & COMPLIANCE UNDERTAKING',
          content: 'Goods are subject to Council Regulation (EU) 2021/821 and certified not for diversion to unsanctioned jurisdictions.'
        }
      ],
      signatures: [
        { signer_name: 'Klaus Weber', signer_title: 'Export Operations Director', date: new Date().toISOString().split('T')[0], signature_data: null },
        { signer_name: 'Marco Vanezzi', signer_title: 'Direttore Acquisti', date: new Date().toISOString().split('T')[0], signature_data: null }
      ],
      tax_rate: 19.0,
      discount_amount: 3000,
      compliance_mode: 'eu_vat',
      show_qr_seal: true,
      show_signature: true,
      show_remittance_stub: true
    },
    {
      doc_type: 'certificate_insurance',
      title: 'CERTIFICATE OF COMMERCIAL LIABILITY',
      header_subtitle: 'Comprehensive General & Maritime Umbrella Coverage',
      prefix: 'COI-',
      theme: 'monaco_gold',
      primary_color: '#ca8a04',
      layout_style: 'modern_banner',
      background_style: 'security_frame',
      font_family: 'display',
      font_scale: 'standard',
      spacing_density: 'generous',
      paper_size: 'us-letter',
      footer_text: 'AUTHORIZED UNDERWRITER BINDING MEMORANDUM · NAIC #24190',
      currency: 'USD',
      currency_symbol_mode: 'code',
      currency_placement: 'prefix',
      currency_separators: 'standard',
      date_format: 'formal_prose',
      number_decimals: 2,
      unit_measure: 'units',
      from: { name: 'Underwriting Syndicate 402', company: 'Sovereign Marine & Energy Assurance', address: '100 Wall Street, 28th Floor\nNew York, NY 10005', email: 'underwriting@sovereignmarine.com', phone: '+1 (212) 555-8900', tax_id: 'US-13-4920194', website: 'https://sovereignmarine.com', registration_number: 'NY-INS-88402' },
      to: { name: 'Pacific Horizon Energy LLC', company: 'Pacific Horizon Offshore Exploration', address: '1100 Louisiana St, Suite 4400\nHouston, TX 77002', email: 'risk@pacifichorizon.com', phone: '+1 (713) 555-4019', tax_id: 'US-76-9029148', website: 'https://pacifichorizon.com', registration_number: 'TX-CORP-49201' },
      items: [
        { description: 'Commercial General Liability Each Occurrence Policy Limit', quantity: 1.0, unit_price: 25000000.0, discount: 0 },
        { description: 'Environmental Pollution & Spill Remediation Sublimit', quantity: 1.0, unit_price: 15000000.0, discount: 0 }
      ],
      custom_fields: [
        { label: 'POLICY NUMBER', value: 'POL-GL-2026-992' },
        { label: 'NAIC COMPANY CODE', value: 'NAIC-24190' }
      ],
      sections: [
        {
          title: '1. ADDITIONAL INSURED ENDORSEMENT',
          content: 'Certificate holder is named as additional insured with respect to operations performed by named insured under Master Drilling Services Agreement.'
        }
      ],
      signatures: [
        { signer_name: 'Genevieve St. Claire', signer_title: 'Senior Managing Underwriter', date: new Date().toISOString().split('T')[0], signature_data: null },
        { signer_name: 'Captain Donald Vance', signer_title: 'Marine Surveyor', date: new Date().toISOString().split('T')[0], signature_data: null }
      ],
      tax_rate: 0.0,
      discount_amount: 0,
      compliance_mode: 'statutory_sworn',
      show_qr_seal: true,
      show_signature: true,
      show_remittance_stub: false
    }
  ];

  // ==========================================================================
  // UNDO & REDO HISTORY SYSTEM
  // ==========================================================================

  function pushUndo(actionName) {
    undoStack.push({
      action: actionName,
      state: JSON.parse(JSON.stringify(state))
    });
    if (undoStack.length > MAX_UNDO) {
      undoStack.shift();
    }
    redoStack.length = 0;
    updateUndoRedoButtons();
  }

  function updateUndoRedoButtons() {
    const btnUndo = document.getElementById('btnUndo');
    const btnRedo = document.getElementById('btnRedo');
    if (btnUndo) {
      btnUndo.disabled = undoStack.length === 0;
      btnUndo.title = undoStack.length > 0
        ? `Undo: ${undoStack[undoStack.length - 1].action} (⌘Z / Ctrl+Z)`
        : 'Undo (⌘Z / Ctrl+Z)';
    }
    if (btnRedo) {
      btnRedo.disabled = redoStack.length === 0;
      btnRedo.title = redoStack.length > 0
        ? `Redo: ${redoStack[redoStack.length - 1].action} (⌘⇧Z / Ctrl+Y)`
        : 'Redo (⌘⇧Z / Ctrl+Y)';
    }
  }

  function undo() {
    if (undoStack.length === 0) return;

    // Animate Undo Button in Navbar (counter-clockwise spring spin)
    const btnUndo = document.getElementById('btnUndo');
    if (btnUndo) {
      btnUndo.classList.remove('btn-undo-spin');
      void btnUndo.offsetWidth;
      btnUndo.classList.add('btn-undo-spin');
      setTimeout(() => btnUndo.classList.remove('btn-undo-spin'), 450);
    }

    // Animate Preview Canvas Rewind Aura (time-travel cyan/indigo glow)
    const previewWrap = document.getElementById('previewDocWrapper');
    if (previewWrap) {
      previewWrap.classList.remove('anim-undo-rewind', 'anim-redo-forward');
      void previewWrap.offsetWidth;
      previewWrap.classList.add('anim-undo-rewind');
      setTimeout(() => previewWrap.classList.remove('anim-undo-rewind'), 550);
    }

    const previous = undoStack.pop();
    redoStack.push({
      action: previous.action,
      state: JSON.parse(JSON.stringify(state))
    });
    Object.assign(state, previous.state);
    deselectElement();
    syncFormFromState();
    renderLineItemsTable();
    renderSectionsList();
    renderSignaturesList();
    renderCustomFieldsList();
    updateCalculations();
    renderInteractiveOverlay();
    schedulePreview();
    applyRestoredHighlights();
    showUndoToast(`↩ Undone: ${previous.action}`);
    updateUndoRedoButtons();
  }

  function redo() {
    if (redoStack.length === 0) return;

    // Animate Redo Button in Navbar (clockwise spring spin)
    const btnRedo = document.getElementById('btnRedo');
    if (btnRedo) {
      btnRedo.classList.remove('btn-redo-spin');
      void btnRedo.offsetWidth;
      btnRedo.classList.add('btn-redo-spin');
      setTimeout(() => btnRedo.classList.remove('btn-redo-spin'), 450);
    }

    // Animate Preview Canvas Forward Aura (energetic emerald/indigo surge)
    const previewWrap = document.getElementById('previewDocWrapper');
    if (previewWrap) {
      previewWrap.classList.remove('anim-undo-rewind', 'anim-redo-forward');
      void previewWrap.offsetWidth;
      previewWrap.classList.add('anim-redo-forward');
      setTimeout(() => previewWrap.classList.remove('anim-redo-forward'), 550);
    }

    const next = redoStack.pop();
    undoStack.push({
      action: next.action,
      state: JSON.parse(JSON.stringify(state))
    });
    Object.assign(state, next.state);
    deselectElement();
    syncFormFromState();
    renderLineItemsTable();
    renderSectionsList();
    renderSignaturesList();
    renderCustomFieldsList();
    updateCalculations();
    renderInteractiveOverlay();
    schedulePreview();
    applyRestoredHighlights();
    showUndoToast(`↪ Redone: ${next.action}`);
    updateUndoRedoButtons();
  }

  function applyRestoredHighlights() {
    // Pulse restored table rows
    const rows = itemsTbody.querySelectorAll('tr');
    rows.forEach(r => {
      r.classList.remove('anim-restored');
      void r.offsetWidth;
      r.classList.add('anim-restored');
      setTimeout(() => r.classList.remove('anim-restored'), 800);
    });

    // Pulse restored overlay cards
    const cards = document.querySelectorAll('.interactive-card');
    cards.forEach(c => {
      c.classList.remove('anim-restored');
      void c.offsetWidth;
      c.classList.add('anim-restored');
      setTimeout(() => c.classList.remove('anim-restored'), 800);
    });
  }

  function showUndoToast(msg) {
    const toast = document.getElementById('undoToast');
    if (!toast) return;
    const isUndone = msg.startsWith('↩ Undone');

    let btnHtml = '';
    if (isUndone) {
      btnHtml = `<button class="undo-toast-btn" id="btnToastRedo">↪ Redo</button>`;
    } else {
      btnHtml = `<button class="undo-toast-btn" id="btnToastUndo">↩ Undo</button>`;
    }

    toast.innerHTML = `
      <span>${msg}</span>
      ${btnHtml}
    `;
    toast.classList.remove('show');
    void toast.offsetWidth;
    toast.classList.add('show');

    const undoBtn = document.getElementById('btnToastUndo');
    if (undoBtn) {
      undoBtn.onclick = () => {
        undo();
        toast.classList.remove('show');
      };
    }
    const redoBtn = document.getElementById('btnToastRedo');
    if (redoBtn) {
      redoBtn.onclick = () => {
        redo();
        toast.classList.remove('show');
      };
    }
    clearTimeout(toast._timer);
    toast._timer = setTimeout(() => {
      toast.classList.remove('show');
    }, 4500);
  }

  function bindUndoRedo() {
    const btnUndo = document.getElementById('btnUndo');
    const btnRedo = document.getElementById('btnRedo');
    if (btnUndo) btnUndo.addEventListener('click', undo);
    if (btnRedo) btnRedo.addEventListener('click', redo);

    window.addEventListener('keydown', (e) => {
      const isInput = ['INPUT', 'TEXTAREA'].includes(document.activeElement?.tagName);

      // Undo: Cmd+Z or Ctrl+Z (without Shift)
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'z' && !e.shiftKey) {
        if (!isInput) {
          e.preventDefault();
          undo();
        }
      }
      // Redo: Cmd+Shift+Z or Ctrl+Shift+Z or Cmd+Y / Ctrl+Y
      else if ((e.metaKey || e.ctrlKey) && ((e.key.toLowerCase() === 'z' && e.shiftKey) || e.key.toLowerCase() === 'y')) {
        if (!isInput) {
          e.preventDefault();
          redo();
        }
      }
      // Delete / Backspace when an interactive element is selected
      else if ((e.key === 'Delete' || e.key === 'Backspace') && selectedElement && !isInput) {
        e.preventDefault();
        deleteSelectedElement();
      }
      // Escape to deselect
      else if (e.key === 'Escape' && selectedElement) {
        deselectElement();
      }
    });
  }

  // ==========================================================================
  // INTERACTIVE CANVAS SELECTION, DELETION & DRAG-AND-DROP
  // ==========================================================================

  function escapeHtml(str) {
    if (!str) return '';
    return String(str)
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#039;');
  }

  function selectElement(type, index, cardEl, labelText) {
    deselectElement();
    selectedElement = { type, index, card: cardEl };
    cardEl.classList.add('is-selected');

    // Add 4 corner pips
    ['pip-tl', 'pip-tr', 'pip-bl', 'pip-br'].forEach(pos => {
      const pip = document.createElement('div');
      pip.className = `selection-pip ${pos}`;
      cardEl.appendChild(pip);
    });

    // Add attached floating toolbar
    const toolbar = document.createElement('div');
    toolbar.className = 'interactive-toolbar';

    let buttonsHtml = '';
    if (type === 'item') {
      buttonsHtml = `
        <span class="itb-title">📦 Item #${index + 1}</span>
        <span class="itb-btn" data-action="duplicate" title="Duplicate item">📋 Duplicate</span>
        <span class="itb-btn" data-action="edit" title="Scroll to form">✏️ Edit</span>
        <span class="itb-btn itb-btn-danger" data-action="delete" title="Delete instance (Del / Backspace)">🗑️ Delete</span>
      `;
    } else if (type === 'section') {
      buttonsHtml = `
        <span class="itb-title">📑 Section #${index + 1}</span>
        <span class="itb-btn" data-action="duplicate" title="Duplicate section">📋 Duplicate</span>
        <span class="itb-btn" data-action="edit" title="Scroll to form">✏️ Edit</span>
        <span class="itb-btn itb-btn-danger" data-action="delete" title="Delete instance (Del / Backspace)">🗑️ Delete</span>
      `;
    } else if (type === 'signature') {
      buttonsHtml = `
        <span class="itb-title">✍️ Signatory #${index + 1}</span>
        <span class="itb-btn" data-action="edit" title="Scroll to form">✏️ Edit</span>
        <span class="itb-btn itb-btn-danger" data-action="delete" title="Delete instance (Del / Backspace)">🗑️ Delete</span>
      `;
    } else if (['notes', 'terms', 'remittance', 'compliance'].includes(type)) {
      buttonsHtml = `
        <span class="itb-title">⚙️ ${type.toUpperCase()}</span>
        <span class="itb-btn itb-btn-danger" data-action="delete" title="Delete instance (Del / Backspace)">🗑️ Remove</span>
      `;
    } else {
      buttonsHtml = `
        <span class="itb-title">📝 ${labelText || type}</span>
        <span class="itb-btn" data-action="edit">✏️ Edit in Form</span>
      `;
    }
    toolbar.innerHTML = buttonsHtml;

    toolbar.querySelectorAll('.itb-btn').forEach(btn => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const action = btn.dataset.action;
        if (action === 'delete') {
          deleteSelectedElement();
        } else if (action === 'duplicate') {
          duplicateSelectedElement();
        } else if (action === 'edit') {
          scrollToFormField(type, index);
        }
      });
    });

    // Flip toolbar below if card is near the top edge of canvas
    if (cardEl.offsetTop < 42) {
      toolbar.classList.add('flip-bottom');
    }

    cardEl.appendChild(toolbar);

    // Update floating selection action dock
    const selBar = document.getElementById('selectionActionBar');
    const selLabel = document.getElementById('selectionLabel');
    const dockWrap = document.getElementById('dockVariantWrap');
    const dockSelect = document.getElementById('dockVariantSelect');
    const dockBtnDup = document.getElementById('dockBtnDuplicate');
    const btnDel = document.getElementById('btnDeleteSelected');

    if (selBar && selLabel) {
      selBar.style.display = 'flex';
      selLabel.innerText = labelText || `${type} #${index !== undefined ? index + 1 : ''}`;

      const comp = COMPONENT_SYSTEM[type];
      if (dockWrap && dockSelect) {
        if (comp && comp.variants && comp.variants.length > 0) {
          dockWrap.style.display = 'flex';
          dockSelect.innerHTML = comp.variants.map(v => `<option value="${v.id}">${v.label}</option>`).join('');
          dockSelect.value = comp.current ? comp.current() : comp.variants[0].id;
          dockSelect.onchange = (e) => {
            if (comp.apply) {
              pushUndo(`Switched layout for ${comp.name} to ${e.target.value}`);
              comp.apply(e.target.value);
              schedulePreview();
              showUndoToast(`Switched layout: ${comp.variants.find(v => v.id === e.target.value)?.label || e.target.value}`);
            }
          };
        } else {
          dockWrap.style.display = 'none';
        }
      }

      if (dockBtnDup) {
        dockBtnDup.style.display = (comp && comp.canDuplicate) ? 'inline-flex' : 'none';
      }
      if (btnDel) {
        const canDel = (comp && comp.canDelete) || ['item', 'section', 'signature', 'custom_field', 'notes', 'terms', 'remittance', 'compliance'].includes(type);
        btnDel.style.display = canDel ? 'inline-flex' : 'none';
      }
    }

    highlightSidebarElement(type, index);
  }

  function deselectElement() {
    if (selectedElement && selectedElement.card) {
      selectedElement.card.classList.remove('is-selected');
      selectedElement.card.querySelectorAll('.selection-pip, .interactive-toolbar').forEach(el => el.remove());
    }
    selectedElement = null;
    const selBar = document.getElementById('selectionActionBar');
    if (selBar) selBar.style.display = 'none';
  }

  function deleteSelectedElement() {
    if (!selectedElement) return;
    const target = { ...selectedElement };
    const { type, index, card } = target;

    // Recoil animation on Delete button in toolbar
    const btnDel = document.getElementById('btnDeleteSelected');
    if (btnDel) {
      btnDel.classList.remove('btn-delete-recoil');
      void btnDel.offsetWidth;
      btnDel.classList.add('btn-delete-recoil');
      setTimeout(() => btnDel.classList.remove('btn-delete-recoil'), 250);
    }

    // Animate target card in overlay with crimson ghost dissolve
    if (card) {
      card.classList.remove('hotspot-add-emerald');
      card.classList.add('hotspot-delete-crimson');
    }

    // Clean up selection pips & toolbar right away so only the card animates out
    deselectElement();

    if (type === 'item') {
      if (state.items.length === 0) return;
      const rows = itemsTbody.querySelectorAll('tr');
      if (rows[index]) rows[index].classList.add('anim-row-delete-crimson');

      setTimeout(() => {
        const removed = state.items[index];
        pushUndo(`Deleted Item "${(removed && removed.description) || index + 1}"`);
        state.items.splice(index, 1);
        renderLineItemsTable();
        updateCalculations();
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast(`🗑️ Deleted Item · [↩ Undo]`);
      }, 320);
    } else if (type === 'section') {
      if (state.sections.length === 0) return;
      const secCards = document.querySelectorAll('#sectionsContainer .section-card');
      if (secCards[index]) secCards[index].classList.add('anim-card-delete-crimson');

      setTimeout(() => {
        const removed = state.sections[index];
        pushUndo(`Deleted Section "${(removed && removed.title) || index + 1}"`);
        state.sections.splice(index, 1);
        renderSectionsList();
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast(`🗑️ Deleted Section · [↩ Undo]`);
      }, 320);
    } else if (type === 'signature') {
      if (state.signatures.length === 0) return;
      const sigCards = document.querySelectorAll('#signaturesContainer .sig-block-card');
      if (sigCards[index]) sigCards[index].classList.add('anim-card-delete-crimson');

      setTimeout(() => {
        const removed = state.signatures[index];
        pushUndo(`Deleted Signatory "${(removed && removed.signer_name) || index + 1}"`);
        state.signatures.splice(index, 1);
        renderSignaturesList();
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast(`🗑️ Deleted Signatory · [↩ Undo]`);
      }, 320);
    } else if (type === 'custom_field') {
      if (state.custom_fields.length === 0) return;
      const fieldRows = document.querySelectorAll('#customFieldsContainer .custom-field-row');
      if (fieldRows[index]) fieldRows[index].classList.add('anim-card-delete-crimson');

      setTimeout(() => {
        const removed = state.custom_fields[index];
        pushUndo(`Deleted Field "${(removed && removed.label) || index + 1}"`);
        state.custom_fields.splice(index, 1);
        renderCustomFieldsList();
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast(`🗑️ Deleted Field · [↩ Undo]`);
      }, 320);
    } else if (type === 'notes') {
      setTimeout(() => {
        pushUndo('Cleared Payment Notes');
        state.notes = '';
        const el = document.getElementById('notesInput');
        if (el) el.value = '';
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast('🗑️ Cleared Notes · [↩ Undo]');
      }, 320);
    } else if (type === 'terms') {
      setTimeout(() => {
        pushUndo('Cleared Terms & Conditions');
        state.terms = '';
        const el = document.getElementById('termsInput');
        if (el) el.value = '';
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast('🗑️ Cleared Terms · [↩ Undo]');
      }, 320);
    } else if (type === 'remittance') {
      setTimeout(() => {
        pushUndo('Removed Remittance Stub');
        state.show_remittance_stub = false;
        const el = document.getElementById('toggleRemittance');
        if (el) el.checked = false;
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast('🗑️ Removed Remittance Stub · [↩ Undo]');
      }, 320);
    } else if (type === 'compliance') {
      setTimeout(() => {
        pushUndo('Removed Compliance Regime');
        state.compliance_mode = 'none';
        const el = document.getElementById('complianceSelect');
        if (el) el.value = 'none';
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast('🗑️ Removed Compliance · [↩ Undo]');
      }, 320);
    }
  }

  function duplicateSelectedElement() {
    if (!selectedElement) return;
    const { type, index } = selectedElement;

    if (type === 'item') {
      const orig = state.items[index];
      if (!orig) return;
      pushUndo(`Duplicated Item #${index + 1}`);
      const clone = JSON.parse(JSON.stringify(orig));
      clone.description = `${clone.description} (Copy)`;
      state.items.splice(index + 1, 0, clone);
      deselectElement();
      renderLineItemsTable();
      updateCalculations();
      renderInteractiveOverlay();
      schedulePreview();

      // Highlight the cloned item in table and preview
      const newRow = itemsTbody.querySelectorAll('tr')[index + 1];
      if (newRow) {
        newRow.classList.add('anim-restored');
        setTimeout(() => newRow.classList.remove('anim-restored'), 800);
      }
      const newCard = document.querySelector(`.interactive-card[data-type="item"][data-index="${index + 1}"]`);
      if (newCard) {
        newCard.classList.add('anim-restored');
        setTimeout(() => newCard.classList.remove('anim-restored'), 800);
      }

      showUndoToast(`📋 Duplicated Item · [↩ Undo]`);
    } else if (type === 'section') {
      const orig = state.sections[index];
      if (!orig) return;
      pushUndo(`Duplicated Section #${index + 1}`);
      const clone = JSON.parse(JSON.stringify(orig));
      clone.title = `${clone.title} (Copy)`;
      state.sections.splice(index + 1, 0, clone);
      deselectElement();
      renderSectionsList();
      renderInteractiveOverlay();
      schedulePreview();

      // Highlight the cloned section
      const newSec = document.querySelectorAll('#sectionsContainer .section-card')[index + 1];
      if (newSec) {
        newSec.classList.add('anim-restored');
        setTimeout(() => newSec.classList.remove('anim-restored'), 800);
      }
      const newCard = document.querySelector(`.interactive-card[data-type="section"][data-index="${index + 1}"]`);
      if (newCard) {
        newCard.classList.add('anim-restored');
        setTimeout(() => newCard.classList.remove('anim-restored'), 800);
      }

      showUndoToast(`📋 Duplicated Section · [↩ Undo]`);
    }
  }

  function scrollToFormField(type, index) {
    // If in Preview-Only mode, switch to split view so the editor is visible
    const mainContainer = document.querySelector('.main-container');
    if (mainContainer && mainContainer.classList.contains('view-mode-preview')) {
      setViewMode('split');
    }

    // Activate the appropriate tab
    if (['item', 'header', 'from', 'to', 'metadata'].includes(type)) {
      switchSidebarTab('content');
    } else if (['section', 'signature', 'notes', 'terms', 'compliance', 'remittance', 'watermark'].includes(type)) {
      switchSidebarTab('legal');
    } else if (['archetype', 'theme', 'layout', 'typography', 'format', 'currency'].includes(type)) {
      switchSidebarTab('design');
    } else if (['ai', 'copilot', 'scan'].includes(type)) {
      switchSidebarTab('ai');
    }

    if (type === 'item') {
      const card = document.querySelector('.items-table');
      if (card) {
        card.closest('.control-card').classList.remove('collapsed');
        card.scrollIntoView({ behavior: 'smooth', block: 'center' });
        const row = card.querySelectorAll('tbody tr')[index];
        if (row) {
          row.classList.add('pulse');
          setTimeout(() => row.classList.remove('pulse'), 1200);
          const input = row.querySelector('.item-desc');
          if (input) input.focus();
        }
      }
    } else if (type === 'section') {
      const cont = document.getElementById('sectionsContainer');
      if (cont) {
        cont.closest('.control-card').classList.remove('collapsed');
        cont.scrollIntoView({ behavior: 'smooth', block: 'center' });
        const secCard = cont.querySelectorAll('.section-card')[index];
        if (secCard) {
          secCard.classList.add('pulse');
          setTimeout(() => secCard.classList.remove('pulse'), 1200);
        }
      }
    } else if (type === 'signature') {
      const cont = document.getElementById('signaturesContainer');
      if (cont) {
        cont.closest('.control-card').classList.remove('collapsed');
        cont.scrollIntoView({ behavior: 'smooth', block: 'center' });
      }
    } else if (type === 'from' || type === 'to') {
      const el = document.getElementById(type === 'from' ? 'fromName' : 'toName');
      if (el) {
        el.closest('.control-card').classList.remove('collapsed');
        el.scrollIntoView({ behavior: 'smooth', block: 'center' });
        el.focus();
      }
    } else if (type === 'header') {
      const el = document.getElementById('docTitle');
      if (el) {
        el.closest('.control-card').classList.remove('collapsed');
        el.scrollIntoView({ behavior: 'smooth', block: 'center' });
        el.focus();
      }
    }
  }

  function highlightSidebarElement(type, index) {
    if (type === 'item') {
      const rows = document.querySelectorAll('#itemsTbody tr');
      rows.forEach((r, idx) => {
        if (idx === index) r.classList.add('field-highlight');
        else r.classList.remove('field-highlight');
      });
    }
  }

  function setupDraggableCard(cardEl, type, i) {
    cardEl.addEventListener('dragstart', (e) => {
      dragSource = { type, index: i, fromPreview: true };
      cardEl.classList.add('is-dragging');
      e.dataTransfer.effectAllowed = 'move';
      e.dataTransfer.setData('text/plain', i);
    });

    cardEl.addEventListener('dragend', () => {
      cardEl.classList.remove('is-dragging');
      document.querySelectorAll('.drop-target-before, .drop-target-after').forEach(el => {
        el.classList.remove('drop-target-before', 'drop-target-after');
      });
      dragSource = null;
    });

    cardEl.addEventListener('dragover', (e) => {
      if (!dragSource || dragSource.type !== type || dragSource.index === i) return;
      e.preventDefault();
      e.dataTransfer.dropEffect = 'move';
      const rect = cardEl.getBoundingClientRect();
      const relY = e.clientY - rect.top;
      if (relY < rect.height / 2) {
        cardEl.classList.add('drop-target-before');
        cardEl.classList.remove('drop-target-after');
      } else {
        cardEl.classList.add('drop-target-after');
        cardEl.classList.remove('drop-target-before');
      }
    });

    cardEl.addEventListener('dragleave', () => {
      cardEl.classList.remove('drop-target-before', 'drop-target-after');
    });

    cardEl.addEventListener('drop', (e) => {
      e.preventDefault();
      if (!dragSource || dragSource.type !== type) return;
      const fromIdx = dragSource.index;
      const isAfter = cardEl.classList.contains('drop-target-after');
      cardEl.classList.remove('drop-target-before', 'drop-target-after');
      if (fromIdx === i) return;

      if (type === 'item') {
        pushUndo('Reorder Line Items');
        const moved = state.items.splice(fromIdx, 1)[0];
        const targetIdx = isAfter ? (fromIdx < i ? i : i + 1) : (fromIdx < i ? i - 1 : i);
        state.items.splice(targetIdx, 0, moved);
        renderLineItemsTable();
        updateCalculations();
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast('↕️ Reordered line items · [↩ Undo]');
      } else if (type === 'section') {
        pushUndo('Reorder Sections');
        const moved = state.sections.splice(fromIdx, 1)[0];
        const targetIdx = isAfter ? (fromIdx < i ? i : i + 1) : (fromIdx < i ? i - 1 : i);
        state.sections.splice(targetIdx, 0, moved);
        renderSectionsList();
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast('↕️ Reordered sections · [↩ Undo]');
      } else if (type === 'signature') {
        pushUndo('Reorder Signatories');
        const moved = state.signatures.splice(fromIdx, 1)[0];
        const targetIdx = isAfter ? (fromIdx < i ? i : i + 1) : (fromIdx < i ? i - 1 : i);
        state.signatures.splice(targetIdx, 0, moved);
        renderSignaturesList();
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast('↕️ Reordered signatories · [↩ Undo]');
      }
    });
  }

  function renderInteractiveOverlay() {
    const overlay = document.getElementById('previewInteractiveOverlay');
    if (!overlay) return;
    overlay.innerHTML = '';

    if (!interactiveMode) {
      overlay.classList.add('disabled');
      return;
    }
    overlay.classList.remove('disabled');

    function createCard(type, index, label, extraClass = '') {
      const card = document.createElement('div');
      card.className = `interactive-card ${extraClass}`;
      card.dataset.type = type;
      if (index !== undefined) card.dataset.index = index;
      card.title = `Click to select ${label} · Del to remove`;
      return card;
    }

    function createPill(text, grip = false) {
      const pill = document.createElement('span');
      pill.className = 'hotspot-pill';
      pill.innerHTML = `${grip ? '<span class="drag-grip">⠿</span> ' : ''}${escapeHtml(text)}`;
      return pill;
    }

    // 1. Header Zone
    const headerZone = document.createElement('div');
    headerZone.className = 'interactive-zone zone-header';
    const headerCard = createCard('header', undefined, 'Header', 'card-header');
    headerCard.appendChild(createPill(`Header · ${state.title || 'Document'}`));
    headerCard.addEventListener('click', (e) => {
      e.stopPropagation();
      selectElement('header', undefined, headerCard, `Header: ${state.title}`);
    });
    headerZone.appendChild(headerCard);
    overlay.appendChild(headerZone);

    // 2. Parties Zone
    const partiesZone = document.createElement('div');
    partiesZone.className = 'interactive-zone zone-parties';
    partiesZone.style.display = 'grid';
    partiesZone.style.gridTemplateColumns = '1fr 1fr';
    partiesZone.style.gap = '12px';

    const fromCard = createCard('from', undefined, 'Issuer (From)', 'card-party');
    fromCard.appendChild(createPill(`From: ${state.from.name || state.from.company || 'Issuer'}`));
    fromCard.addEventListener('click', (e) => {
      e.stopPropagation();
      selectElement('from', undefined, fromCard, `From: ${state.from.name || 'Issuer'}`);
    });

    const toCard = createCard('to', undefined, 'Recipient (To)', 'card-party');
    toCard.appendChild(createPill(`To: ${state.to.name || state.to.company || 'Recipient'}`));
    toCard.addEventListener('click', (e) => {
      e.stopPropagation();
      selectElement('to', undefined, toCard, `To: ${state.to.name || 'Recipient'}`);
    });

    partiesZone.appendChild(fromCard);
    partiesZone.appendChild(toCard);
    overlay.appendChild(partiesZone);

    // 2.5 Custom Fields Zone
    if (state.custom_fields && state.custom_fields.length > 0) {
      const cfZone = document.createElement('div');
      cfZone.className = 'interactive-zone zone-custom-fields';
      cfZone.style.display = 'grid';
      cfZone.style.gridTemplateColumns = `repeat(${Math.min(state.custom_fields.length, 3)}, 1fr)`;
      cfZone.style.gap = '10px';

      state.custom_fields.forEach((cf, i) => {
        const cfCard = createCard('custom_field', i, `Field: ${cf.label}`, 'card-custom-field');
        cfCard.appendChild(createPill(`${cf.label}: ${cf.value}`));
        cfCard.addEventListener('click', (e) => {
          e.stopPropagation();
          selectElement('custom_field', i, cfCard, `Field: ${cf.label}`);
        });
        cfZone.appendChild(cfCard);
      });
      overlay.appendChild(cfZone);
    }

    // 3. Line Items Zone (Prescriptive: omit for certificates, pure agreements with no items)
    const isPureCert = state.doc_type === 'certificate' || state.doc_type === 'certificate_completion';
    const isInspection = state.doc_type === 'property_inspection_checklist';
    const isNdaWithoutItems = (state.doc_type === 'nda' || state.doc_type === 'agreement') && (!state.items || state.items.length === 0);

    if (!isPureCert && !isNdaWithoutItems && state.items && state.items.length > 0) {
      const itemsZone = document.createElement('div');
      itemsZone.className = 'interactive-zone zone-items';

      state.items.forEach((item, i) => {
        const itemCard = createCard('item', i, `Item #${i + 1}`, 'card-item');
        itemCard.draggable = true;
        const lineTotal = (item.quantity * item.unit_price) - (item.discount || 0);

        const pill = createPill(`#${i + 1} ${(item.description || 'Item').slice(0, 26)} · ${formatAmountUi(lineTotal)}`, true);
        itemCard.appendChild(pill);

        itemCard.addEventListener('click', (e) => {
          e.stopPropagation();
          selectElement('item', i, itemCard, `Item #${i + 1}: ${(item.description || '').slice(0, 20)}`);
        });

        setupDraggableCard(itemCard, 'item', i);
        itemsZone.appendChild(itemCard);
      });

      overlay.appendChild(itemsZone);
    }

    // 4. Financials / Totals Zone (Prescriptive: omit for certificates, checklists, and empty NDAs)
    if (!isPureCert && !isInspection && !isNdaWithoutItems) {
      const totalsZone = document.createElement('div');
      totalsZone.className = 'interactive-zone zone-totals';
      const totalsCard = createCard('totals', undefined, 'Financials', 'card-totals');

      let subtotal = computeSubtotalUi();
      let tax = subtotal * (state.tax_rate / 100);
      let rawTotal = subtotal + tax - (state.discount_amount || 0);
      let grandTotal = allowsNegativeItems(state.doc_type) ? rawTotal : Math.max(0, rawTotal);

      totalsCard.appendChild(createPill(`Totals · Due ${formatAmountUi(grandTotal)}`));
      totalsCard.addEventListener('click', (e) => {
        e.stopPropagation();
        selectElement('totals', undefined, totalsCard, `Total Due: ${formatAmountUi(grandTotal)}`);
      });
      totalsZone.appendChild(totalsCard);
      overlay.appendChild(totalsZone);
    }

    // 5. Custom Sections Zone
    if (state.sections && state.sections.length > 0) {
      const secZone = document.createElement('div');
      secZone.className = 'interactive-zone zone-sections';

      state.sections.forEach((sec, i) => {
        const secCard = createCard('section', i, `Section #${i + 1}`, 'card-section');
        secCard.draggable = true;
        secCard.appendChild(createPill(`Section #${i + 1}: ${(sec.title || 'Untitled').slice(0, 24)}`, true));
        secCard.addEventListener('click', (e) => {
          e.stopPropagation();
          selectElement('section', i, secCard, `Section: ${(sec.title || '').slice(0, 20)}`);
        });
        setupDraggableCard(secCard, 'section', i);
        secZone.appendChild(secCard);
      });
      overlay.appendChild(secZone);
    }

    // 6. Signatories Zone
    if (state.signatures && state.signatures.length > 0) {
      const sigZone = document.createElement('div');
      sigZone.className = 'interactive-zone zone-signatures';

      const sigGrid = document.createElement('div');
      sigGrid.style.display = 'grid';
      sigGrid.style.gridTemplateColumns = `repeat(${Math.min(state.signatures.length, 3)}, 1fr)`;
      sigGrid.style.gap = '8px';

      state.signatures.forEach((sig, i) => {
        const sigCard = createCard('signature', i, `Signatory #${i + 1}`, 'card-signature');
        sigCard.draggable = true;
        sigCard.appendChild(createPill(`Signatory: ${sig.signer_name || i + 1}`, true));
        sigCard.addEventListener('click', (e) => {
          e.stopPropagation();
          selectElement('signature', i, sigCard, `Signatory: ${sig.signer_name || i + 1}`);
        });
        setupDraggableCard(sigCard, 'signature', i);
        sigGrid.appendChild(sigCard);
      });
      sigZone.appendChild(sigGrid);
      overlay.appendChild(sigZone);
    }

    // 7. Notes & Terms Zone
    if (state.notes || state.terms || state.show_remittance_stub) {
      const bottomZone = document.createElement('div');
      bottomZone.className = 'interactive-zone zone-bottom';

      if (state.notes) {
        const notesCard = createCard('notes', undefined, 'Payment Notes', 'card-notes');
        notesCard.appendChild(createPill('Payment Notes'));
        notesCard.addEventListener('click', (e) => {
          e.stopPropagation();
          selectElement('notes', undefined, notesCard, 'Payment Notes');
        });
        bottomZone.appendChild(notesCard);
      }

      if (state.terms) {
        const termsCard = createCard('terms', undefined, 'Terms & Conditions', 'card-terms');
        termsCard.appendChild(createPill('Terms & Conditions'));
        termsCard.addEventListener('click', (e) => {
          e.stopPropagation();
          selectElement('terms', undefined, termsCard, 'Terms & Conditions');
        });
        bottomZone.appendChild(termsCard);
      }

      if (state.show_remittance_stub) {
        const remCard = createCard('remittance', undefined, 'Remittance Stub', 'card-remittance');
        remCard.appendChild(createPill('✂ Remittance Stub'));
        remCard.addEventListener('click', (e) => {
          e.stopPropagation();
          selectElement('remittance', undefined, remCard, 'Remittance Stub');
        });
        bottomZone.appendChild(remCard);
      }

      overlay.appendChild(bottomZone);
    }

    overlay.onclick = (e) => {
      if (e.target === overlay) {
        deselectElement();
      }
    };
  }

  function bindInteractiveControls() {
    const btnToggle = document.getElementById('btnToggleInteractive');
    if (btnToggle) {
      btnToggle.addEventListener('click', () => {
        interactiveMode = !interactiveMode;
        if (interactiveMode) {
          btnToggle.classList.add('active');
          btnToggle.innerHTML = '🎯 Interactive: ON';
          showUndoToast('🎯 Interactive Mode ON');
        } else {
          btnToggle.classList.remove('active');
          btnToggle.innerHTML = '👁️ Clean View';
          deselectElement();
          showUndoToast('👁️ Clean View Enabled');
        }
        renderInteractiveOverlay();
      });
    }

    const btnDelSelected = document.getElementById('btnDeleteSelected');
    if (btnDelSelected) {
      btnDelSelected.addEventListener('click', () => {
        deleteSelectedElement();
      });
    }
  }

  // ===========================================================================
  // CHATGPT AUTHENTICATION & SUBSCRIPTION PASS-THROUGH
  // ===========================================================================
  // ===========================================================================
  // UNIVERSAL AUTHENTICATION & WORKSPACE SESSION MANAGEMENT
  // ===========================================================================
  let currentChatGptSession = null;

  async function checkChatGptSession() {
    try {
      const res = await fetch('/api/v1/auth/session');
      if (res.ok) {
        const data = await res.json();
        if (data.authenticated && data.user) {
          currentChatGptSession = data;
          renderLoggedInChatGptUI(data);
        } else {
          currentChatGptSession = null;
          renderLoggedOutChatGptUI();
        }
      }
    } catch (e) {
      console.warn('Session query error:', e);
    }
  }

  function renderLoggedInChatGptUI(data) {
    const user = data.user;
    const loginBtn = document.getElementById('btnChatGptLogin');
    const authTrigger = document.getElementById('btnOpenAuthModal');
    const userDropdown = document.getElementById('chatgptUserDropdown');
    const planLabel = document.getElementById('chatgptPlanLabel');
    const userEmail = document.getElementById('chatgptUserEmail');
    const userName = document.getElementById('dropdownUserName');
    const planStatus = document.getElementById('dropdownPlanStatus');
    const watermarkStatus = document.getElementById('dropdownWatermarkStatus');

    if (loginBtn) loginBtn.style.display = 'none';
    if (authTrigger) authTrigger.style.display = 'none';
    if (userDropdown) userDropdown.style.display = 'inline-block';
    if (planLabel) planLabel.innerText = user.plan_display;
    if (userEmail) userEmail.innerText = user.email;
    if (userName) userName.innerText = user.name;
    if (planStatus) planStatus.innerText = `${user.plan_display} (Active)`;
    if (watermarkStatus) {
      watermarkStatus.innerText = user.watermark_free ? 'None (Unlocked)' : 'Standard Watermark';
    }

    // Update AI Copilot sidebar banner
    const badge = document.getElementById('bannerSubscriberBadge');
    const title = document.getElementById('bannerComputeTitle');
    const desc = document.getElementById('bannerComputeDesc');
    const actionContainer = document.getElementById('bannerActionContainer');

    if (badge) {
      badge.innerText = `${user.plan_display} Active`;
      badge.style.backgroundColor = user.is_subscriber ? 'rgba(16, 163, 127, 0.25)' : 'rgba(100, 116, 139, 0.2)';
      badge.style.color = user.is_subscriber ? '#34d399' : '#94a3b8';
    }
    if (title) title.innerText = `⚡ ${user.plan_display}`;
    if (desc) {
      if (user.provider === 'chatgpt') {
        desc.innerText = `Connected as ${user.email}. All Copilot actions, layout changes, and OCR extractions run directly on your personal ${user.plan_display} compute quota with zero API bills.`;
      } else {
        desc.innerText = `Active workspace for ${user.email}. Previews compile in sub-15ms with full state persistence.`;
      }
    }
    if (actionContainer) {
      if (user.watermark_free) {
        actionContainer.innerHTML = `<span style="font-size: 11px; font-weight: 600; color: #34d399; display: flex; align-items: center; gap: 4px;"><span>🟢</span> Unlimited Unwatermarked Exports Active</span>`;
      } else {
        actionContainer.innerHTML = `<button type="button" class="btn btn-secondary btn-sm" id="btnBannerUpgrade"><span>Upgrade to Pro</span></button>`;
        const btnUpg = document.getElementById('btnBannerUpgrade');
        if (btnUpg) {
          btnUpg.addEventListener('click', () => {
            const pricingModal = document.getElementById('pricingModal');
            if (pricingModal) pricingModal.style.display = 'flex';
          });
        }
      }
    }
  }

  function renderLoggedOutChatGptUI() {
    const loginBtn = document.getElementById('btnChatGptLogin');
    const authTrigger = document.getElementById('btnOpenAuthModal');
    const userDropdown = document.getElementById('chatgptUserDropdown');
    const triggerLabel = document.getElementById('authTriggerLabel');

    if (loginBtn) loginBtn.style.display = 'none';
    if (authTrigger) authTrigger.style.display = 'inline-flex';
    if (userDropdown) userDropdown.style.display = 'none';
    if (triggerLabel) triggerLabel.innerText = 'Sign In';

    const badge = document.getElementById('bannerSubscriberBadge');
    const title = document.getElementById('bannerComputeTitle');
    const desc = document.getElementById('bannerComputeDesc');
    const actionContainer = document.getElementById('bannerActionContainer');

    if (badge) {
      badge.innerText = 'Free Workspace';
      badge.style.backgroundColor = 'rgba(100, 116, 139, 0.2)';
      badge.style.color = '#94a3b8';
    }
    if (title) title.innerText = 'Sign In or Connect ChatGPT';
    if (desc) {
      desc.innerText = 'Open a guest workspace, sign in with your email, or connect ChatGPT Plus/Pro to power AI Copilot and watermark-free exports.';
    }
    if (actionContainer) {
      actionContainer.innerHTML = `<button type="button" class="btn btn-secondary btn-sm" id="btnBannerOpenAuth"><span>Sign In / Workspace</span></button>`;
      const btnAuth = document.getElementById('btnBannerOpenAuth');
      if (btnAuth) {
        btnAuth.addEventListener('click', () => {
          const modalAuth = document.getElementById('modalAuth');
          if (modalAuth) modalAuth.classList.add('active');
        });
      }
    }
  }

  function bindChatGptAuth() {
    const modalAuth = document.getElementById('modalAuth');
    const btnOpenAuth = document.getElementById('btnOpenAuthModal');
    const btnCloseAuth = document.getElementById('btnCloseAuth');
    const btnSwitchAccount = document.getElementById('btnSwitchAccount');

    if (btnOpenAuth && modalAuth) {
      btnOpenAuth.addEventListener('click', () => modalAuth.classList.add('active'));
    }
    if (btnSwitchAccount && modalAuth) {
      btnSwitchAccount.addEventListener('click', () => {
        const dd = document.getElementById('chatgptUserDropdown');
        if (dd) dd.classList.remove('open');
        modalAuth.classList.add('active');
      });
    }
    if (btnCloseAuth && modalAuth) {
      btnCloseAuth.addEventListener('click', () => modalAuth.classList.remove('active'));
    }
    if (modalAuth) {
      modalAuth.addEventListener('click', (e) => {
        if (e.target === modalAuth) modalAuth.classList.remove('active');
      });
    }

    // Guest login button
    const btnAuthGuest = document.getElementById('btnAuthGuest');
    if (btnAuthGuest) {
      btnAuthGuest.addEventListener('click', async () => {
        try {
          const res = await fetch('/api/v1/auth/guest', { method: 'POST' });
          if (res.ok) {
            const data = await res.json();
            currentChatGptSession = data;
            renderLoggedInChatGptUI(data);
            if (modalAuth) modalAuth.classList.remove('active');
            showAiToast('🛡️ Guest workspace active. Documents saved to local storage.');
          }
        } catch (e) {
          console.error(e);
        }
      });
    }

    // Email login form
    const formEmailAuth = document.getElementById('formEmailAuth');
    const inputAuthEmail = document.getElementById('inputAuthEmail');
    if (formEmailAuth && inputAuthEmail) {
      formEmailAuth.addEventListener('submit', async (e) => {
        e.preventDefault();
        const email = inputAuthEmail.value.trim();
        if (!email || !email.includes('@')) {
          alert('Please enter a valid email address');
          return;
        }
        try {
          const res = await fetch('/api/v1/auth/email/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email: email })
          });
          if (res.ok) {
            const data = await res.json();
            currentChatGptSession = data;
            renderLoggedInChatGptUI(data);
            if (modalAuth) modalAuth.classList.remove('active');
            showAiToast(`✉️ Signed in as ${email}. Full studio access unlocked.`);
          }
        } catch (err) {
          console.error(err);
        }
      });
    }

    // API Key login form
    const formApiKeyAuth = document.getElementById('formApiKeyAuth');
    const inputAuthApiKey = document.getElementById('inputAuthApiKey');
    if (formApiKeyAuth && inputAuthApiKey) {
      formApiKeyAuth.addEventListener('submit', async (e) => {
        e.preventDefault();
        const key = inputAuthApiKey.value.trim();
        if (!key) {
          alert('Please enter an API or Agent key');
          return;
        }
        try {
          const res = await fetch('/api/v1/auth/api-key', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ api_key: key })
          });
          if (res.ok) {
            const data = await res.json();
            currentChatGptSession = data;
            renderLoggedInChatGptUI(data);
            if (modalAuth) modalAuth.classList.remove('active');
            showAiToast('🔑 Developer API key authenticated. Watermark-free exports active.');
          }
        } catch (err) {
          console.error(err);
        }
      });
    }

    // Logout button
    const btnLogout = document.getElementById('btnChatGptLogout');
    if (btnLogout) {
      btnLogout.addEventListener('click', async (e) => {
        e.preventDefault();
        try {
          await fetch('/api/v1/auth/logout', { method: 'POST' });
          currentChatGptSession = null;
          renderLoggedOutChatGptUI();
          showAiToast('Signed out of workspace.');
        } catch (err) {
          console.error(err);
        }
      });
    }

    // Mobile nav toggle
    const btnMobileNavToggle = document.getElementById('btnMobileNavToggle');
    const topNavbar = document.getElementById('topNavbar');
    if (btnMobileNavToggle && topNavbar) {
      btnMobileNavToggle.addEventListener('click', () => {
        topNavbar.classList.toggle('nav-mobile-open');
      });
    }

    // Check URL parameters for return from OAuth
    const urlParams = new URLSearchParams(window.location.search);
    if (urlParams.get('auth') === 'chatgpt_connected' || urlParams.get('auth') === 'success') {
      showAiToast('⚡ Connected to ChatGPT! Subscription compute sharing is now active.');
      const cleanUrl = window.location.pathname;
      window.history.replaceState({}, document.title, cleanUrl);
    }
  }

  let currentTaxonomyCat = 'Commercial Invoicing';
  let currentTaxonomyName = 'Tax Invoice';

  function allowsNegativeItems(docType) {
    return [
      'credit_note',
      'property_management_statement',
      'closing_settlement_statement',
      'security_deposit_disposition',
      'real_estate_commission',
      'lease_amendment',
      'mutual_lease_termination',
      'rent_increase_notice',
      'tenant_rent_ledger',
      'schedule_e_tax_worksheet',
      'real_estate_deal_analysis',
      'real_estate_purchase_agreement'
    ].includes(docType);
  }

  function lineItemAmount(item) {
    const qty = Number(item.quantity) || 0;
    const rate = Number(item.unit_price) || 0;
    const gross = qty * rate;
    if (rate < 0) return gross;
    return gross - (Number(item.discount) || 0);
  }

  function isRatioMetricItem(item) {
    const d = String(item.description || '').toLowerCase();
    const amt = Math.abs(lineItemAmount(item));
    if (amt > 100.0 || d.includes('noi') || (d.includes('rent') && !d.includes('multiplier')) || d.includes('income') || d.includes('expense')) {
      return false;
    }
    return d.includes('dscr')
      || d.includes('coverage ratio')
      || d.includes('cap rate')
      || d.includes('cash-on-cash')
      || d.includes('ltv')
      || d.includes('loan-to-value')
      || d.includes('occupancy rate')
      || d.includes('vacancy rate')
      || d.includes('interest rate')
      || d.includes('irr')
      || d.includes('internal rate of return')
      || d.includes('yield')
      || d.includes('grm')
      || d.includes('rent multiplier')
      || d.includes('roi');
  }

  function isCapitalOrValuationItem(item) {
    const d = String(item.description || '').toLowerCase();
    if (d.includes('noi') || d.includes('net operating income') || (d.includes('rent') && !d.includes('multiplier')) || d.includes('cash flow') || d.includes('vacancy') || d.includes('expense') || d.includes('debt service')) {
      return false;
    }
    return d.includes('purchase price')
      || d.includes('acquisition price')
      || d.includes('market value')
      || d.includes('property value')
      || d.includes('appraised value')
      || d.includes('after-repair value')
      || d.includes('arv')
      || d.includes('rehab budget')
      || d.includes('down payment')
      || d.includes('closing cost')
      || d.includes('senior debt')
      || d.includes('total debt')
      || d.includes('mortgage balance')
      || d.includes('loan balance')
      || d.includes('loan amount')
      || d.includes('principal balance')
      || d.includes('net equity')
      || d.includes('total equity');
  }

  function computeSubtotalUi() {
    const items = Array.isArray(state.items) ? state.items : [];
    if (state.doc_type === 'rent_increase_notice' && items.length >= 2) {
      const firstAmt = lineItemAmount(items[0]);
      const secondAmt = lineItemAmount(items[1]);
      if (firstAmt > 0 && secondAmt > 0) {
        const secondDesc = String(items[1].description || '').toLowerCase();
        const isNewBaseRow = secondAmt >= firstAmt
          || secondDesc.includes('new')
          || secondDesc.includes('proposed')
          || secondDesc.includes('revised')
          || secondDesc.includes('updated')
          || secondDesc.includes('effective');
        const extraFees = items.slice(2).reduce((s, it) => s + lineItemAmount(it), 0);
        return isNewBaseRow ? (secondAmt + extraFees) : (firstAmt + secondAmt + extraFees);
      }
    }
    if (state.doc_type === 'real_estate_deal_analysis' || state.doc_type === 'schedule_of_real_estate_owned') {
      const nonRatio = items.filter(it => !isRatioMetricItem(it));
      const hasOperating = nonRatio.some(it => !isCapitalOrValuationItem(it));
      const hasCapital = nonRatio.some(it => isCapitalOrValuationItem(it));
      if (hasOperating && hasCapital) {
        return nonRatio.filter(it => !isCapitalOrValuationItem(it)).reduce((s, it) => s + lineItemAmount(it), 0);
      }
      return nonRatio.reduce((s, it) => s + lineItemAmount(it), 0);
    }
    return items.reduce((s, it) => s + lineItemAmount(it), 0);
  }

  function normalizeComplianceMode(raw) {
    if (!raw) return 'none';
    const s = String(raw).trim().toLowerCase().replace(/[\s-]+/g, '_');
    const map = {
      none: 'none',
      standard: 'none',
      eu_vat: 'eu_vat',
      vat_reverse_charge: 'eu_vat',
      reverse_charge: 'eu_vat',
      peppol: 'peppol',
      peppol_bis: 'peppol',
      peppol_bis_3: 'peppol',
      factur_x: 'factur-x',
      zugferd: 'factur-x',
      en16931: 'factur-x',
      en_16931: 'factur-x',
      irs_w9: 'irs_w9',
      w9: 'irs_w9',
      w_9: 'irs_w9',
      pci_dss: 'pci_dss',
      pci: 'pci_dss',
      soc2: 'soc2',
      soc_2: 'soc2',
      soc2_badge: 'soc2',
      soc2_type2: 'soc2',
      hipaa: 'hipaa',
      hipaa_baa: 'hipaa',
      baa: 'hipaa',
      zatca: 'zatca',
      fatoora: 'zatca',
      zatca_fatoora: 'zatca',
      statutory_sworn: 'statutory_sworn',
      sworn: 'statutory_sworn',
      affidavit: 'statutory_sworn'
    };
    return map[s] || 'none';
  }

  function normalizeDateIso(raw) {
    if (!raw) return '';
    let s = String(raw).trim();
    if (!s) return '';
    if (s.includes('T') && s.indexOf('T') >= 8) {
      s = s.split('T')[0].trim();
    } else if (s.includes(' ') && s.indexOf(' ') >= 8 && /^\d/.test(s.split(' ')[1] || '')) {
      s = s.split(' ')[0].trim();
    }
    let m = s.match(/^(\d{4})-(\d{1,2})-(\d{1,2})$/);
    if (m) {
      return `${m[1]}-${m[2].padStart(2, '0')}-${m[3].padStart(2, '0')}`;
    }
    m = s.match(/^(\d{1,2})\/(\d{1,2})\/(\d{4})$/);
    if (m) {
      return `${m[3]}-${m[1].padStart(2, '0')}-${m[2].padStart(2, '0')}`;
    }
    m = s.match(/^(\d{4})\/(\d{1,2})\/(\d{1,2})$/);
    if (m) {
      return `${m[1]}-${m[2].padStart(2, '0')}-${m[3].padStart(2, '0')}`;
    }
    return s;
  }

  function resolveArchetypeSlug(raw) {
    if (!raw) return 'invoice';
    let s = String(raw)
      .trim()
      .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
      .toLowerCase()
      .replace(/[\s-]+/g, '_');
    const aliasMap = {
      quote: 'estimate',
      proforma: 'proforma_invoice',
      credit: 'credit_note',
      credit_memo: 'credit_note',
      po: 'purchase_order',
      bol: 'packing_slip',
      bill_of_lading: 'packing_slip',
      tender: 'quotation_tender',
      rfp: 'quotation_tender',
      msa: 'agreement',
      master_services_agreement: 'agreement',
      sow: 'agreement_sow',
      statement_of_work: 'agreement_sow',
      ica: 'contractor_agreement',
      independent_contractor_agreement: 'contractor_agreement',
      retainer: 'retainer_agreement',
      mutual_nda: 'nda',
      non_disclosure_agreement: 'nda',
      lease: 'residential_lease',
      residential_lease_agreement: 'residential_lease',
      nnn_lease: 'commercial_nnn_lease',
      triple_net_lease: 'commercial_nnn_lease',
      owner_statement: 'property_management_statement',
      settlement_statement: 'closing_settlement_statement',
      hud1: 'closing_settlement_statement',
      alta_settlement: 'closing_settlement_statement',
      eviction_notice: 'notice_to_vacate',
      pay_or_quit: 'notice_to_vacate',
      deposit_disposition: 'security_deposit_disposition',
      commission_statement: 'real_estate_commission',
      move_in_checklist: 'property_inspection_checklist',
      loi: 'letter_of_intent_cre',
      letter_of_intent: 'letter_of_intent_cre',
      estoppel: 'tenant_estoppel',
      estoppel_certificate: 'tenant_estoppel',
      room_lease: 'room_rental_lease',
      roommate_agreement: 'room_rental_lease',
      lease_renewal: 'lease_renewal_agreement',
      addendum: 'lease_amendment',
      fcra_adverse_action: 'adverse_action_notice',
      entry_notice: 'notice_of_entry',
      rent_increase: 'rent_increase_notice',
      lease_termination: 'mutual_lease_termination',
      rent_ledger: 'tenant_rent_ledger',
      sreo: 'schedule_of_real_estate_owned',
      schedule_e: 'schedule_e_tax_worksheet',
      deal_analysis: 'real_estate_deal_analysis',
      sublease: 'sublease_agreement',
      rent_to_own: 'rent_to_own_agreement',
      lease_option: 'rent_to_own_agreement',
      pma: 'property_management_agreement',
      psa: 'real_estate_purchase_agreement',
      purchase_agreement: 'real_estate_purchase_agreement',
      deed: 'property_deed',
      quitclaim_deed: 'property_deed',
      warranty_deed: 'property_deed',
      lien_waiver: 'mechanics_lien_waiver',
      coi: 'certificate_insurance',
      pod: 'delivery_receipt'
    };
    if (aliasMap[s]) return aliasMap[s];
    // Strip leading state prefix if a state-prefixed slug was passed (e.g. tennessee_residential_lease)
    const stateSuffixes = [
      'residential_lease', 'commercial_nnn_lease', 'commercial_lease',
      'room_rental_lease', 'sublease_agreement', 'rent_to_own_agreement',
      'lease_renewal_agreement', 'lease_amendment', 'notice_to_vacate',
      'security_deposit_disposition', 'notice_of_entry', 'rent_increase_notice',
      'mutual_lease_termination', 'real_estate_purchase_agreement',
      'property_deed', 'mechanics_lien_waiver', 'bill_of_sale'
    ];
    for (const suffix of stateSuffixes) {
      if (s.endsWith('_' + suffix)) {
        return suffix;
      }
    }
    return s;
  }

  // Initialization
  function init() {
    bindUndoRedo();
    bindInteractiveControls();
    bindFormInputs();
    bindThemeSelectors();
    bindBackgroundSelectors();
    bindCategoryTabs();
    bindTemplateGrid();
    bindPrimaryArchetypeHeader();
    bindLifecycleStepper();
    bindMultiPageControls();
    bindAccordions();
    bindCopilotBar();
    bindZoomControls();
    bindDropdowns();
    bindViewModes();
    bindSidebarTabs();
    bindGlobalAccordionToggles();
    bindArchetypeSearch();
    bindSidebarAiCopilot();
    bindSidebarScanner();
    bindFloatingDockActions();
    bindExportActions();
    bindChatGptAuth();
    bindDataModeControls();
    bindPartyProfileActions();
    populateCounterpartyDropdown();
    const hasSavedDoc = loadSavedUserDocument();
    if (!hasSavedDoc) {
      loadIssuerProfile();
    }
    const initUrlParams = new URLSearchParams(window.location.search);
    const requestedState =
      window.__PRESSMARK_INITIAL_STATE__ ||
      initUrlParams.get('state') ||
      initUrlParams.get('state_code');
    if (requestedState) {
      state.state_code = String(requestedState).trim().toUpperCase();
    }
    const requestedMuni = initUrlParams.get('municipality') || initUrlParams.get('city');
    if (requestedMuni) {
      state.municipality = String(requestedMuni).trim();
    }
    const requestedTemplate =
      window.__PRESSMARK_INITIAL_ARCHETYPE__ ||
      initUrlParams.get('template') ||
      initUrlParams.get('archetype') ||
      initUrlParams.get('doc_type');
    if (requestedTemplate) {
      const normalizedType = resolveArchetypeSlug(requestedTemplate);
      const targetPill = document.querySelector(`.template-pill[data-type="${normalizedType}"]`);
      if (targetPill) {
        document.querySelectorAll('.template-pill').forEach(p => p.classList.remove('active'));
        targetPill.classList.add('active');
      }
      if (!hasSavedDoc || state.doc_type !== normalizedType) {
        loadSampleDataForArchetype(normalizedType);
      }
      applyDocumentTypeDefaults(normalizedType);
    }
    setupDynamicInteractions();
    adaptFormToArchetype(state.doc_type);
    syncFormFromState();
    renderSignaturesList();
    renderSectionsList();
    renderCustomFieldsList();
    renderLineItemsTable();
    updateCalculations();
    renderInteractiveOverlay();
    requestPreview();
    checkChatGptSession();

    // Testing & Automated UI Verification Hooks
    window.__PRESSMARK_STUDIO__ = {
      getState: () => state,
      selectItem: (idx) => {
        const cards = document.querySelectorAll('.interactive-card[data-type="item"]');
        if (cards[idx]) {
          selectElement('item', idx, cards[idx], `Item #${idx + 1}: ${(state.items[idx] ? state.items[idx].description : '').slice(0, 20)}`);
        }
      },
      addSampleItems: () => {
        pushUndo('Add Enterprise Line Items');
        state.items.push(
          { description: 'DevOps CI/CD Automation Pipeline', quantity: 16.0, unit_price: 185.0, discount: 0.0 },
          { description: 'SOC-2 Type II Penetration Testing & Cryptographic Verification', quantity: 24.0, unit_price: 220.0, discount: 500.0 }
        );
        state.sections.push({
          title: 'SERVICE LEVEL AGREEMENT & ESCALATION MATRIX',
          content: 'Pressmark guarantees 99.999% uptime with sub-15ms vector synthesis latency across all distributed compilation nodes.'
        });
        state.signatures.push({
          signer_name: 'Marcus Sterling, Esq.',
          signer_title: 'General Counsel & Corporate Secretary',
          date: '2026-10-01'
        });
        renderLineItemsTable();
        renderSectionsList();
        renderSignaturesList();
        updateCalculations();
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast('Added enterprise items & SLA section · [↩ Undo]');
      },
      deleteItem: (idx) => {
        const cards = document.querySelectorAll('.interactive-card[data-type="item"]');
        if (cards[idx]) {
          selectElement('item', idx, cards[idx], `Item #${idx + 1}`);
          setTimeout(() => deleteSelectedElement(), 100);
        }
      },
      undo: () => undo(),
      redo: () => redo(),
      getChatGptSession: () => currentChatGptSession,
      setDemoChatGptSession: (sessionData) => {
        currentChatGptSession = sessionData;
        renderLoggedInChatGptUI(sessionData);
      },
      getDataMode: () => dataMode,
      setDataMode: (mode) => {
        if (mode === 'my_data' && btnModeMyData) btnModeMyData.click();
        else if (mode === 'sample_data' && btnModeSampleData) btnModeSampleData.click();
      },
      saveIssuerProfile,
      loadIssuerProfile,
      resetIssuerProfile,
      saveCounterparty,
      clearCounterparty,
      applyDocumentTypeDefaults,
      transitionLifecycle: (target) => executeLifecycleTransition(target),
      updateLifecycleUI
    };

    const urlParams = new URLSearchParams(window.location.search);
    const testAction = urlParams.get('test_action');
    if (testAction === 'select_item') {
      setTimeout(() => {
        const idx = parseInt(urlParams.get('index') || '0', 10);
        window.__PRESSMARK_STUDIO__.selectItem(idx);
      }, 700);
    } else if (testAction === 'add_elements') {
      setTimeout(() => {
        window.__PRESSMARK_STUDIO__.addSampleItems();
      }, 500);
    } else if (testAction === 'delete_and_toast') {
      setTimeout(() => {
        window.__PRESSMARK_STUDIO__.deleteItem(1);
      }, 700);
    } else if (testAction === 'undo_glow') {
      setTimeout(() => {
        window.__PRESSMARK_STUDIO__.deleteItem(1);
        setTimeout(() => {
          undo();
        }, 400);
      }, 700);
    } else if (testAction === 'chatgpt_connected') {
      setTimeout(() => {
        window.__PRESSMARK_STUDIO__.setDemoChatGptSession({
          authenticated: true,
          session_id: 'cpt_sess_demo_plus_49210',
          user: {
            id: 'usr_chatgpt_plus_98231',
            name: 'Daniel Hallman',
            email: 'daniel@mooshworks.com',
            plan: 'plus',
            plan_display: 'ChatGPT Plus',
            badge_color: '#10a37f',
            is_subscriber: true,
            compute_allowance: 'unlimited_delegated_compute',
            watermark_free: true,
            priority_compilation: true
          }
        });
        switchSidebarTab('ai');
      }, 500);
    }
  }

  // Setup Global Highlight & Animation Listeners
  function setupDynamicInteractions() {
    document.addEventListener('input', (e) => {
      if (e.target.matches('input, textarea, select')) {
        highlightField(e.target);
        triggerPreviewCompilingAnimation();
      }
    });

    document.addEventListener('change', (e) => {
      if (e.target.matches('input, select')) {
        highlightField(e.target);
        triggerPreviewCompilingAnimation();
      }
    });
  }

  function highlightField(el) {
    el.classList.add('field-highlight');
    clearTimeout(el._highlightTimer);
    el._highlightTimer = setTimeout(() => {
      el.classList.remove('field-highlight');
    }, 700);
  }

  function triggerPreviewCompilingAnimation() {
    previewDocWrapper.classList.add('compiling-pulse');
    previewDocWrapper.classList.add('is-compiling');
    compileTelemetry.classList.add('pulse');
    compileTelemetry.innerHTML = '⚡ Compiling vector document...';
  }

  // Bind Form Inputs to State
  function bindFormInputs() {
    const bind = (id, prop, targetObj = state) => {
      const el = document.getElementById(id);
      if (!el) return;
      el.value = targetObj[prop] !== undefined ? targetObj[prop] : '';
      let initialVal = el.value;
      el.addEventListener('focus', () => {
        initialVal = el.value;
      });
      el.addEventListener('input', (e) => {
        if (targetObj === state.from || targetObj === state.to || ['title', 'header_subtitle', 'notes', 'terms'].includes(prop)) {
          hasUserCustomizedData = true;
        }
        targetObj[prop] = e.target.type === 'number' ? parseFloat(e.target.value) || 0 : e.target.value;
        updateCalculations();
        schedulePreview();
        renderInteractiveOverlay();
      });
      el.addEventListener('change', () => {
        if (el.value !== initialVal) {
          pushUndo(`Edit ${prop}`);
          initialVal = el.value;
        }
      });
    };

    bind('docTitle', 'title');
    bind('headerSubtitle', 'header_subtitle');
    bind('docNumber', 'doc_number');
    bind('issueDate', 'issue_date');
    bind('dueDate', 'due_date');
    bind('primaryColorInput', 'primary_color');
    bind('fontFamilySelect', 'font_family');
    bind('fontScaleSelect', 'font_scale');
    bind('spacingDensitySelect', 'spacing_density');
    bind('paperSizeSelect', 'paper_size');
    bind('layoutStyleSelect', 'layout_style');
    bind('notesInput', 'notes');
    bind('termsInput', 'terms');
    bind('taxRateInput', 'tax_rate');
    bind('discountInput', 'discount_amount');
    bind('complianceSelect', 'compliance_mode');
    bind('stateCodeSelect', 'state_code');
    bind('municipalitySelect', 'municipality');
    bind('footerTextInput', 'footer_text');

    // Data Formatters & Localization
    bind('currencySelect', 'currency');
    bind('currencySymbolModeSelect', 'currency_symbol_mode');
    bind('currencyPlacementSelect', 'currency_placement');
    bind('separatorsSelect', 'currency_separators');
    bind('dateFormatSelect', 'date_format');
    bind('unitMeasureSelect', 'unit_measure');

    const numDecSelect = document.getElementById('numberDecimalsSelect');
    if (numDecSelect) {
      numDecSelect.value = state.number_decimals !== undefined ? String(state.number_decimals) : '2';
      numDecSelect.addEventListener('change', (e) => {
        state.number_decimals = parseInt(e.target.value, 10) || 0;
        updateCalculations();
        schedulePreview();
      });
    }

    const taxPresetSelect = document.getElementById('taxPresetSelect');
    if (taxPresetSelect) {
      taxPresetSelect.addEventListener('change', (e) => {
        const val = e.target.value;
        if (val !== 'custom') {
          const rate = parseFloat(val);
          if (!isNaN(rate)) {
            state.tax_rate = rate;
            const taxInput = document.getElementById('taxRateInput');
            if (taxInput) taxInput.value = rate;
            updateCalculations();
            schedulePreview();
          }
        }
      });
    }

    // From Party Contact & Entity Details
    bind('fromName', 'name', state.from);
    bind('fromCompany', 'company', state.from);
    bind('fromAddress', 'address', state.from);
    bind('fromEmail', 'email', state.from);
    bind('fromPhone', 'phone', state.from);
    bind('fromTaxId', 'tax_id', state.from);
    bind('fromWebsite', 'website', state.from);
    bind('fromRegNumber', 'registration_number', state.from);

    // To Party Contact & Entity Details
    bind('toName', 'name', state.to);
    bind('toCompany', 'company', state.to);
    bind('toAddress', 'address', state.to);
    bind('toEmail', 'email', state.to);
    bind('toPhone', 'phone', state.to);
    bind('toTaxId', 'tax_id', state.to);
    bind('toWebsite', 'website', state.to);
    bind('toRegNumber', 'registration_number', state.to);

    // Checkboxes / Toggles
    const bindToggle = (id, prop) => {
      const el = document.getElementById(id);
      if (!el) return;
      el.checked = !!state[prop];
      el.addEventListener('change', (e) => {
        pushUndo(`Toggle ${prop}`);
        state[prop] = e.target.checked;
        schedulePreview();
        renderInteractiveOverlay();
      });
    };

    bindToggle('isPaidCheck', 'is_paid');
    bindToggle('toggleQrSeal', 'show_qr_seal');
    bindToggle('toggleSignature', 'show_signature');
    bindToggle('toggleRemittance', 'show_remittance_stub');
    bindToggle('toggleLineNumbers', 'show_line_numbers');
    bindToggle('togglePageNumbers', 'show_page_numbers');
    bindToggle('toggleFooterDivider', 'show_footer_divider');

    // Payment & Stripe Checkout Controls
    bind('paymentLinkInput', 'payment_link');
    bind('paymentMethodInput', 'payment_method');
    bind('transactionRefInput', 'transaction_reference');
    bindToggle('togglePaymentQr', 'payment_qr_code');

    const btnCopyPayment = document.getElementById('btnCopyPaymentLink');
    if (btnCopyPayment) {
      btnCopyPayment.addEventListener('click', async () => {
        const link = state.payment_link || document.getElementById('paymentLinkInput')?.value;
        if (link) {
          try {
            await navigator.clipboard.writeText(link);
            showAiToast('Payment link copied to clipboard! 📋', 'success');
          } catch (err) {
            showAiToast('Payment link: ' + link);
          }
        } else {
          showAiToast('No payment link to copy.');
        }
      });
    }

    const amountPaidGroup = document.getElementById('amountPaidGroup');
    const amountPaidInput = document.getElementById('amountPaidInput');
    if (amountPaidInput) {
      amountPaidInput.addEventListener('input', (e) => {
        const val = parseFloat(e.target.value);
        state.amount_paid = isNaN(val) ? undefined : val;
        updateCalculations();
        schedulePreview();
      });
    }

    const payStatusSelect = document.getElementById('paymentStatusSelect');
    if (payStatusSelect) {
      payStatusSelect.addEventListener('change', (e) => {
        pushUndo(`Change payment status to ${e.target.value}`);
        state.payment_status = e.target.value;
        if (state.payment_status === 'paid') {
          state.is_paid = true;
          if (amountPaidGroup) amountPaidGroup.style.display = 'none';
          if (!state.paid_at) state.paid_at = new Date().toISOString().split('T')[0];
          if (!state.transaction_reference) {
            state.transaction_reference = 'STRIPE-REC-' + Math.floor(100000 + Math.random() * 900000);
            const refInput = document.getElementById('transactionRefInput');
            if (refInput) refInput.value = state.transaction_reference;
          }
        } else if (state.payment_status === 'partial') {
          state.is_paid = false;
          if (amountPaidGroup) amountPaidGroup.style.display = 'block';
          if (!state.amount_paid) {
            const subtotal = state.items.reduce((sum, item) => sum + lineItemAmount(item), 0);
            const tax = state.tax_rate ? subtotal * (state.tax_rate / 100.0) : 0.0;
            const rawTotal = subtotal + tax - (state.discount_amount || 0);
            const grandTotal = allowsNegativeItems(state.doc_type) ? rawTotal : Math.max(0, rawTotal);
            state.amount_paid = Math.round((grandTotal / 2) * 100) / 100;
            if (amountPaidInput) amountPaidInput.value = state.amount_paid;
          }
        } else {
          state.is_paid = false;
          state.amount_paid = undefined;
          if (amountPaidGroup) amountPaidGroup.style.display = 'none';
        }
        updateCalculations();
        updateLifecycleUI();
        schedulePreview();
      });
    }

    const btnQuickRecord = document.getElementById('btnQuickRecordPayment');
    if (btnQuickRecord) {
      btnQuickRecord.addEventListener('click', () => {
        executeLifecycleTransition('receipt');
      });
    }

    const btnTestCheckout = document.getElementById('btnTestStripeCheckout');
    if (btnTestCheckout) {
      btnTestCheckout.addEventListener('click', () => {
        const link = state.payment_link || document.getElementById('paymentLinkInput')?.value;
        if (link) {
          window.open(link, '_blank');
        } else {
          showAiToast('Please enter a Stripe checkout link first.');
        }
      });
    }

    // Watermark input
    const wmInput = document.getElementById('watermarkInput');
    if (wmInput) {
      wmInput.addEventListener('input', (e) => {
        state.watermark = e.target.value;
        schedulePreview();
      });
    }

    // Logo upload
    const logoFile = document.getElementById('logoFileInput');
    if (logoFile) {
      logoFile.addEventListener('change', (e) => {
        const file = e.target.files[0];
        if (file) {
          const reader = new FileReader();
          reader.onload = function(evt) {
            state.logo_data = evt.target.result;
            schedulePreview();
          };
          reader.readAsDataURL(file);
        }
      });
    }

    // Top action buttons
    document.getElementById('btnDownloadPdf').addEventListener('click', downloadPdf);
    document.getElementById('btnPrintDoc').addEventListener('click', printDocument);
    document.getElementById('btnRandomize').addEventListener('click', randomizeScene);
    document.getElementById('btnClone').addEventListener('click', cloneDocument);
    document.getElementById('btnAddItem').addEventListener('click', () => {
      pushUndo('Add Line Item');
      hasUserCustomizedData = true;
      hasUserCustomizedItems = true;
      state.items.push({ description: 'New Professional Service Item', quantity: 1.0, unit_price: 150.0, discount: 0.0 });
      const newIdx = state.items.length - 1;
      renderLineItemsTable(newIdx);
      updateCalculations();
      renderInteractiveOverlay();
      const newCard = document.querySelector(`.interactive-card[data-type="item"][data-index="${newIdx}"]`);
      if (newCard) {
        newCard.classList.remove('hotspot-delete-crimson');
        newCard.classList.add('hotspot-add-emerald');
      }
      schedulePreview();
      showUndoToast('✨ Added new line item · [↩ Undo]');
    });

    // Add Section button
    document.getElementById('btnAddSection').addEventListener('click', () => {
      pushUndo('Add Custom Section');
      state.sections.push({
        title: `${state.sections.length + 1}. OPERATIVE PROVISIONS`,
        content: 'Enter technical specifications, contractual recitals, or inspection observations here...'
      });
      const newIdx = state.sections.length - 1;
      renderSectionsList(newIdx);
      renderInteractiveOverlay();
      const newCard = document.querySelector(`.interactive-card[data-type="section"][data-index="${newIdx}"]`);
      if (newCard) {
        newCard.classList.remove('hotspot-delete-crimson');
        newCard.classList.add('hotspot-add-emerald');
      }
      schedulePreview();
      showUndoToast('✨ Added new section · [↩ Undo]');
    });

    // Add Signatory button
    document.getElementById('btnAddSignature').addEventListener('click', () => {
      pushUndo('Add Signatory');
      state.signatures.push({
        signer_name: '',
        signer_title: 'Authorized Signatory',
        date: new Date().toISOString().split('T')[0],
        signature_data: null
      });
      const newIdx = state.signatures.length - 1;
      renderSignaturesList(newIdx);
      renderInteractiveOverlay();
      const newCard = document.querySelector(`.interactive-card[data-type="signature"][data-index="${newIdx}"]`);
      if (newCard) {
        newCard.classList.remove('hotspot-delete-crimson');
        newCard.classList.add('hotspot-add-emerald');
      }
      schedulePreview();
      showUndoToast('✨ Added new signatory · [↩ Undo]');
    });

    // Add Field button
    document.getElementById('btnAddField').addEventListener('click', () => {
      pushUndo('Add Custom Field');
      state.custom_fields.push({ label: 'REFERENCE', value: 'VAL-001' });
      const newIdx = state.custom_fields.length - 1;
      renderCustomFieldsList(newIdx);
      renderInteractiveOverlay();
      schedulePreview();
      showUndoToast('✨ Added custom field · [↩ Undo]');
    });

    setupModals();
  }

  // Accordion Toggles
  function bindAccordions() {
    document.querySelectorAll('.card-header-toggle').forEach(header => {
      header.addEventListener('click', () => {
        const card = header.closest('.control-card');
        card.classList.toggle('collapsed');
      });
    });
  }

  // Category Tabs & Template Grid
  function bindCategoryTabs() {
    const tabs = document.querySelectorAll('.category-tab');
    tabs.forEach(tab => {
      tab.addEventListener('click', () => {
        tabs.forEach(t => t.classList.remove('active'));
        tab.classList.add('active');
        const cat = tab.dataset.category;
        document.querySelectorAll('.template-pill').forEach(pill => {
          if (cat === 'all' || pill.dataset.category === cat) {
            pill.style.display = 'flex';
          } else {
            pill.style.display = 'none';
          }
        });
      });
    });
  }

  function bindTemplateGrid() {
    const pills = document.querySelectorAll('.template-pill');
    pills.forEach(pill => {
      pill.addEventListener('click', () => {
        pushUndo(`Change Archetype to ${pill.innerText.trim()}`);
        pills.forEach(p => p.classList.remove('active'));
        pill.classList.add('active');
        const type = pill.dataset.type;
        state.doc_type = type;
        applyDocumentTypeDefaults(type);
        syncFormFromState();
        renderLineItemsTable();
        renderSectionsList();
        renderSignaturesList();
        renderCustomFieldsList();
        updateCalculations();
        renderInteractiveOverlay();
        document.querySelectorAll('.preview-page-frame').forEach(f => {
          f.classList.remove('layout-morph-enter');
          f.classList.add('layout-morph-exit');
        });
        setTimeout(() => {
          document.querySelectorAll('.preview-page-frame').forEach(f => {
            f.classList.remove('layout-morph-exit');
            f.classList.add('layout-morph-enter');
          });
          setTimeout(() => {
            document.querySelectorAll('.preview-page-frame').forEach(f => f.classList.remove('layout-morph-enter'));
          }, 380);
        }, 120);
        schedulePreview();
        showUndoToast(`Switched archetype to ${pill.innerText.trim()} · [↩ Undo]`);
      });
    });
  }

  function applyDocumentTypeDefaults(type) {
    const map = {
      invoice: { title: 'TAX INVOICE', prefix: 'INV-' },
      invoice_standard: { title: 'TAX INVOICE', prefix: 'INV-' },
      invoice_freelance: { title: 'FREELANCE / CONSULTING INVOICE', prefix: 'FRL-' },
      invoice_construction: { title: 'PROGRESS BILLING APPLICATION', prefix: 'CST-' },
      invoice_medical: { title: 'CLINICAL SERVICES SUPERBILL', prefix: 'MED-' },
      invoice_saas: { title: 'SUBSCRIPTION & CLOUD BILLING', prefix: 'SUB-' },
      invoice_minimal: { title: 'COMMERCIAL INVOICE', prefix: 'DOC-' },
      proforma_invoice: { title: 'PROFORMA INVOICE', prefix: 'PRO-' },
      estimate: { title: 'FORMAL ESTIMATE & QUOTE', prefix: 'EST-' },
      receipt: { title: 'OFFICIAL PAYMENT RECEIPT', prefix: 'REC-', is_paid: true },
      purchase_order: { title: 'ENTERPRISE PURCHASE ORDER', prefix: 'PO-' },
      packing_slip: { title: 'PACKING SLIP & BILL OF LADING', prefix: 'PKG-' },
      credit_note: { title: 'CREDIT NOTE', prefix: 'CRD-' },
      quotation_tender: { title: 'TENDER & BID QUOTATION', prefix: 'BID-' },
      work_order: { title: 'WORK ORDER & JOB TICKET', prefix: 'WO-' },
      agreement: { title: 'MASTER SERVICES AGREEMENT', prefix: 'MSA-' },
      agreement_sow: { title: 'STATEMENT OF WORK', prefix: 'SOW-' },
      contractor_agreement: { title: 'INDEPENDENT CONTRACTOR AGREEMENT', prefix: 'ICA-' },
      retainer_agreement: { title: 'STRATEGIC RETAINER AGREEMENT', prefix: 'RET-' },
      nda: { title: 'MUTUAL NON-DISCLOSURE AGREEMENT', prefix: 'NDA-' },
      commercial_lease: { title: 'COMMERCIAL LEASE AGREEMENT', prefix: 'COM-' },
      bill_of_sale: { title: 'STATUTORY BILL OF SALE', prefix: 'BOS-' },
      certificate: { title: 'CERTIFICATE OF ACHIEVEMENT', prefix: 'CERT-' },
      certificate_completion: { title: 'CERTIFICATE OF COMPLETION', prefix: 'CRT-' },
      certificate_insurance: { title: 'CERTIFICATE OF INSURANCE (COI)', prefix: 'COI-' },
      expense_report: { title: 'EXPENSE REIMBURSEMENT REPORT', prefix: 'EXP-' },
      delivery_receipt: { title: 'PROOF OF DELIVERY RECEIPT', prefix: 'POD-' },
      inspection_report: { title: 'CONDITION & INSPECTION REPORT', prefix: 'INSP-' },
      // Real Estate & Property Management Suite
      residential_lease: { title: 'RESIDENTIAL LEASE AGREEMENT', prefix: 'RLA-' },
      commercial_nnn_lease: { title: 'TRIPLE NET (NNN) COMMERCIAL LEASE', prefix: 'NNN-' },
      property_management_statement: { title: 'MONTHLY PROPERTY MANAGEMENT STATEMENT', prefix: 'PMS-' },
      rent_roll: { title: 'MULTI-UNIT RENT ROLL & LEASE SCHEDULE', prefix: 'RRL-' },
      closing_settlement_statement: { title: 'REAL ESTATE CLOSING SETTLEMENT STATEMENT', prefix: 'ESC-' },
      notice_to_vacate: { title: 'NOTICE TO VACATE & CURE LEASE VIOLATION', prefix: 'NTV-' },
      security_deposit_disposition: { title: 'SECURITY DEPOSIT ITEMIZED DISPOSITION', prefix: 'SDD-' },
      real_estate_commission: { title: 'REAL ESTATE COMMISSION & BROKERAGE FEE STATEMENT', prefix: 'COM-' },
      property_inspection_checklist: { title: 'PROPERTY CONDITION & INSPECTION CHECKLIST', prefix: 'CHK-' },
      letter_of_intent_cre: { title: 'COMMERCIAL REAL ESTATE LETTER OF INTENT (LOI)', prefix: 'LOI-' },
      tenant_estoppel: { title: 'TENANT ESTOPPEL CERTIFICATE', prefix: 'TEC-' },
      room_rental_lease: { title: 'ROOM RENTAL & CO-LIVING LEASE AGREEMENT', prefix: 'RLC-' },
      lease_renewal_agreement: { title: 'LEASE RENEWAL & EXTENSION AGREEMENT', prefix: 'REN-' },
      lease_amendment: { title: 'LEASE AMENDMENT & ADDENDUM', prefix: 'AMD-' },
      rental_application: { title: 'RESIDENTIAL RENTAL APPLICATION', prefix: 'APP-' },
      adverse_action_notice: { title: 'FCRA ADVERSE ACTION NOTICE', prefix: 'FCRA-' },
      notice_of_entry: { title: 'LANDLORD NOTICE OF ENTRY', prefix: 'ENT-' },
      rent_increase_notice: { title: 'STATUTORY NOTICE OF RENT INCREASE', prefix: 'RIN-' },
      mutual_lease_termination: { title: 'MUTUAL LEASE TERMINATION AGREEMENT', prefix: 'MLT-' },
      tenant_rent_ledger: { title: 'TENANT RENT & PAYMENT LEDGER', prefix: 'LDG-' },
      schedule_of_real_estate_owned: { title: 'SCHEDULE OF REAL ESTATE OWNED (SREO)', prefix: 'SREO-' },
      schedule_e_tax_worksheet: { title: 'IRS SCHEDULE E TAX PREP WORKSHEET', prefix: 'TXE-' },
      real_estate_deal_analysis: { title: 'REAL ESTATE INVESTMENT DEAL ANALYSIS', prefix: 'DEAL-' },
      sublease_agreement: { title: 'RESIDENTIAL SUBLEASE AGREEMENT', prefix: 'SUBL-' },
      rent_to_own_agreement: { title: 'RENT-TO-OWN (LEASE-OPTION) AGREEMENT', prefix: 'RTO-' },
      property_management_agreement: { title: 'PROPERTY MANAGEMENT AGREEMENT', prefix: 'PMA-' },
      real_estate_purchase_agreement: { title: 'REAL ESTATE PURCHASE & SALE AGREEMENT', prefix: 'PSA-' },
      property_deed: { title: 'QUITCLAIM / WARRANTY DEED', prefix: 'DEED-' },
      mechanics_lien_waiver: { title: 'MECHANIC\'S LIEN WAIVER & RELEASE', prefix: 'LIEN-' }
    };

    const conf = map[type] || map.invoice;
    state.doc_type = type;
    state.title = conf.title;
    state.doc_number = `${conf.prefix}2026-${Math.floor(1000 + Math.random() * 9000)}`;
    state.recordable = (type === 'property_deed');
    state.notary_required = ['property_deed', 'mechanics_lien_waiver', 'tenant_estoppel'].includes(type);
    state.is_paid = Boolean(conf.is_paid);
    state.payment_status = state.is_paid ? 'paid' : 'unpaid';
    const isPaidCheck = document.getElementById('isPaidCheck');
    if (isPaidCheck) isPaidCheck.checked = state.is_paid;

    const isPayableArchetype = type.startsWith('invoice') || [
      'proforma_invoice',
      'retainer_agreement',
      'notice_to_vacate',
      'tenant_rent_ledger'
    ].includes(type);
    const isTaxableCommercial = type.startsWith('invoice') || [
      'estimate',
      'quote',
      'proforma_invoice',
      'purchase_order',
      'receipt',
      'credit_note'
    ].includes(type);

    if (dataMode === 'sample_data' || !hasUserCustomizedItems) {
      const customFromSnapshot = (dataMode === 'my_data' && hasUserCustomizedData && state.from)
        ? Object.assign({}, state.from)
        : null;
      loadSampleDataForArchetype(type);
      if (dataMode === 'my_data' && localStorage.getItem(STORAGE_KEY_ISSUER)) {
        loadIssuerProfile();
      } else if (customFromSnapshot) {
        state.from = customFromSnapshot;
      }
    } else {
      // In My Data mode after explicit line-item customization, preserve typed party/items
      // while sanitizing incompatible financial/payment/legal fields for the target archetype.
      state.amount_paid = null;
      state.include_clerk_stamp = false;
      state.execution_mode = null;
      if (!isPayableArchetype) {
        state.payment_qr_code = false;
        state.payment_link = '';
      }
      if (!isTaxableCommercial) {
        state.tax_rate = 0;
        state.discount_amount = 0;
        state.notes = '';
        state.terms = '';
        state.footer_text = '';
      }
      if (!allowsNegativeItems(type) && Array.isArray(state.items)) {
        state.items = state.items.map(it => ({
          ...it,
          quantity: Math.abs(Number(it.quantity) || 1),
          unit_price: Math.abs(Number(it.unit_price) || 0)
        }));
      }
      attachDefaultSectionsForArchetype(type, true);
    }

    const docTitleEl = document.getElementById('docTitle');
    if (docTitleEl) docTitleEl.value = state.title;
    const docNumberEl = document.getElementById('docNumber');
    if (docNumberEl) docNumberEl.value = state.doc_number;
    const headerSubtitleEl = document.getElementById('headerSubtitle');
    if (headerSubtitleEl) headerSubtitleEl.value = state.header_subtitle || '';

    adaptFormToArchetype(type);
    updateLifecycleUI();
    if (dataMode === 'my_data' && hasUserCustomizedData) {
      autoSaveUserDocument();
    }
  }

  // =============================================================================
  // DOCUMENT LIFECYCLE ACCELERATOR (ESTIMATE -> TAX INVOICE -> PAID RECEIPT)
  // =============================================================================
  function bindLifecycleStepper() {
    const stepEstimate = document.getElementById('stepBtnEstimate');
    const stepInvoice = document.getElementById('stepBtnInvoice');
    const stepReceipt = document.getElementById('stepBtnReceipt');
    const stepCredit = document.getElementById('stepBtnCredit');
    const actionBtn = document.getElementById('btnLifecycleConvertAction');

    if (stepEstimate) {
      stepEstimate.addEventListener('click', () => executeLifecycleTransition('estimate'));
    }
    if (stepInvoice) {
      stepInvoice.addEventListener('click', () => executeLifecycleTransition('invoice'));
    }
    if (stepReceipt) {
      stepReceipt.addEventListener('click', () => executeLifecycleTransition('receipt'));
    }
    if (stepCredit) {
      stepCredit.addEventListener('click', () => executeLifecycleTransition('credit_note'));
    }
    if (actionBtn) {
      actionBtn.addEventListener('click', () => {
        const active = getLifecycleActiveState();
        if (active === 'estimate') {
          executeLifecycleTransition('invoice');
        } else if (active === 'invoice') {
          executeLifecycleTransition('receipt');
        } else if (active === 'receipt' || active === 'credit_note') {
          downloadPdf();
        }
      });
    }

    updateLifecycleUI();
  }

  function getLifecycleActiveState() {
    if (state.doc_type === 'estimate' || state.doc_type === 'quote') {
      return 'estimate';
    }
    if (state.doc_type === 'credit_note' || state.doc_type === 'credit') {
      return 'credit_note';
    }
    if (state.doc_type === 'receipt' || state.is_paid || state.payment_status === 'paid') {
      return 'receipt';
    }
    return 'invoice';
  }

  function updateLifecycleUI() {
    const active = getLifecycleActiveState();
    const stepEstimate = document.getElementById('stepBtnEstimate');
    const stepInvoice = document.getElementById('stepBtnInvoice');
    const stepReceipt = document.getElementById('stepBtnReceipt');
    const stepCredit = document.getElementById('stepBtnCredit');
    const badge = document.getElementById('lifecycleStatusBadge');
    const actionIcon = document.getElementById('lifecycleActionIcon');
    const actionLabel = document.getElementById('lifecycleActionLabel');

    if (stepEstimate) {
      stepEstimate.classList.toggle('active', active === 'estimate');
      stepEstimate.classList.toggle('completed', active === 'invoice' || active === 'receipt' || active === 'credit_note');
      stepEstimate.setAttribute('aria-selected', active === 'estimate');
    }
    if (stepInvoice) {
      stepInvoice.classList.toggle('active', active === 'invoice');
      stepInvoice.classList.toggle('completed', active === 'receipt' || active === 'credit_note');
      stepInvoice.setAttribute('aria-selected', active === 'invoice');
    }
    if (stepReceipt) {
      stepReceipt.classList.toggle('active', active === 'receipt');
      stepReceipt.classList.toggle('completed', active === 'credit_note');
      stepReceipt.setAttribute('aria-selected', active === 'receipt');
    }
    if (stepCredit) {
      stepCredit.classList.toggle('active', active === 'credit_note');
      stepCredit.setAttribute('aria-selected', active === 'credit_note');
    }

    if (badge) {
      badge.className = 'lifecycle-badge';
      if (active === 'estimate') {
        badge.innerText = 'Draft Estimate';
        badge.classList.add('badge-estimate');
      } else if (active === 'invoice') {
        badge.innerText = 'Tax Invoice';
      } else if (active === 'receipt') {
        badge.innerText = '✓ Paid in Full';
        badge.classList.add('badge-receipt');
      } else if (active === 'credit_note') {
        badge.innerText = 'Credit Note / Refund';
        badge.classList.add('badge-credit');
      }
    }

    if (actionIcon && actionLabel) {
      if (active === 'estimate') {
        actionIcon.innerText = '⚡';
        actionLabel.innerText = 'Convert to Tax Invoice (1-Click)';
      } else if (active === 'invoice') {
        actionIcon.innerText = '💳';
        actionLabel.innerText = 'Mark as Paid & Generate Receipt';
      } else if (active === 'receipt') {
        actionIcon.innerText = '📥';
        actionLabel.innerText = 'Download Official Receipt (PDF)';
      } else if (active === 'credit_note') {
        actionIcon.innerText = '📥';
        actionLabel.innerText = 'Download Credit Note (PDF)';
      }
    }

    const payStatusSelect = document.getElementById('paymentStatusSelect');
    if (payStatusSelect) {
      payStatusSelect.value = state.payment_status || (state.is_paid ? 'paid' : 'unpaid');
    }
  }

  function extractBaseDocNumber(num) {
    if (!num) return '2026-0042';
    return num.replace(/^[A-Za-z]{2,5}-/, '');
  }

  function executeLifecycleTransition(targetState) {
    pushUndo(`Transition lifecycle to ${targetState}`);

    const baseId = extractBaseDocNumber(state.doc_number);

    if (targetState === 'estimate') {
      state.doc_type = 'estimate';
      state.title = 'FORMAL ESTIMATE & QUOTE';
      state.doc_number = `EST-${baseId}`;
      state.is_paid = false;
      state.payment_status = 'unpaid';
      state.amount_paid = undefined;
    } else if (targetState === 'invoice') {
      state.doc_type = 'invoice';
      state.title = 'TAX INVOICE';
      state.doc_number = `INV-${baseId}`;
      state.is_paid = false;
      state.payment_status = 'unpaid';
    } else if (targetState === 'receipt') {
      state.doc_type = 'receipt';
      state.title = 'OFFICIAL PAYMENT RECEIPT';
      state.doc_number = `REC-${baseId}`;
      state.is_paid = true;
      state.payment_status = 'paid';
      if (!state.paid_at) {
        state.paid_at = new Date().toISOString().split('T')[0];
      }
      if (!state.transaction_reference) {
        state.transaction_reference = 'STRIPE-REC-' + Math.floor(100000 + Math.random() * 900000);
      }
      if (!state.payment_method) {
        state.payment_method = 'Stripe / Card / Apple Pay';
      }
    } else if (targetState === 'credit_note') {
      state.doc_type = 'credit_note';
      state.title = 'CREDIT NOTE';
      state.doc_number = `CRD-${baseId}`;
      state.is_paid = false;
      state.payment_status = 'refunded';
      if (!state.notes) {
        state.notes = 'Credit adjustment issued. Balance refunded or credited to customer account.';
      }
    }

    const docTitle = document.getElementById('docTitle');
    if (docTitle) docTitle.value = state.title;
    const docNum = document.getElementById('docNumber');
    if (docNum) docNum.value = state.doc_number;
    const isPaidCheck = document.getElementById('isPaidCheck');
    if (isPaidCheck) isPaidCheck.checked = state.is_paid;
    const txRef = document.getElementById('transactionRefInput');
    if (txRef) txRef.value = state.transaction_reference || '';
    const payMethod = document.getElementById('paymentMethodInput');
    if (payMethod) payMethod.value = state.payment_method || 'Stripe / Card / Apple Pay';

    syncFormFromState();
    updateLifecycleUI();
    updateCalculations();
    adaptFormToArchetype(state.doc_type);
    schedulePreview();
    showAiToast(`⚡ Document converted to ${state.title}`);
  }

  function attachDefaultSectionsForArchetype(type, replaceIfDefault = false) {
    if (!replaceIfDefault && state.sections && state.sections.length > 0) return;
    state.sections = [];
    if (type.includes('lease') || type === 'notice_to_vacate') {
      state.sections.push({
        title: 'Statutory Lease Provisions',
        content: 'Tenant agrees to occupy the premises in compliance with all municipal, state, and federal codes. Quiet enjoyment and habitability are warranted by Landlord pursuant to statutory requirements.'
      });
    } else if (type.includes('agreement') || type === 'nda') {
      state.sections.push({
        title: 'Governing Law & Severability',
        content: 'This Agreement shall be governed by and construed in accordance with the laws of the jurisdiction of execution. If any provision is deemed unenforceable, the remaining provisions remain in full force.'
      });
    } else if (type.includes('certificate')) {
      state.sections.push({
        title: 'Official Attestation',
        content: 'This credential certifies that the named recipient has satisfied all prescribed standards and statutory competencies required for formal recognition.'
      });
    }
  }

  function loadSampleDataForArchetype(type) {
    hasUserCustomizedItems = false;
    state.tax_rate = 0;
    state.discount_amount = 0;
    state.amount_paid = null;
    state.is_paid = (type === 'receipt');
    state.payment_status = state.is_paid ? 'paid' : 'unpaid';
    const isPayableArchetype = type.startsWith('invoice') || [
      'proforma_invoice',
      'retainer_agreement',
      'notice_to_vacate',
      'tenant_rent_ledger'
    ].includes(type);
    state.payment_qr_code = isPayableArchetype;
    state.payment_link = isPayableArchetype ? 'https://buy.stripe.com/4gM3cu8jk4Psd095197wA04' : '';
    state.recordable = (type === 'property_deed');
    state.notary_required = ['property_deed', 'mechanics_lien_waiver', 'tenant_estoppel'].includes(type);
    state.include_clerk_stamp = false;
    state.execution_mode = null;
    state.notes = '';
    state.terms = '';
    state.footer_text = '';
    state.sections = [];
    state.signatures = [];
    if (type === 'residential_lease') {
      state.header_subtitle = 'Standard Fixed-Term Residential Tenancy Agreement';
      state.from = {
        name: 'Elena Rostova',
        company: 'Pacifica Property Holdings LLC',
        address: '100 Montgomery St, Suite 1400, San Francisco, CA 94104',
        email: 'leasing@pacificaholdings.com',
        phone: '+1 (415) 890-4421',
        tax_id: 'US-94-3829104',
        registration_number: 'DRE Lic #01928472'
      };
      state.to = {
        name: 'Marcus & Eleanor Vance',
        company: '',
        address: '742 Evergreen Terrace, Unit 4B, San Francisco, CA 94107',
        email: 'marcus.vance@example.com',
        phone: '+1 (415) 555-0199'
      };
      state.custom_fields = [
        { label: 'Premises Address', value: '742 Evergreen Terrace, Unit 4B, San Francisco, CA 94107' },
        { label: 'Parcel APN', value: '042-8819-012' },
        { label: 'Lease Term', value: '12 Months (Nov 1, 2026 – Oct 31, 2027)' },
        { label: 'Security Deposit Held', value: '$3,200.00 (Chase Client Escrow Trust)' },
        { label: 'Permitted Occupancy', value: '2 Adults, 1 Dependent' },
        { label: 'Pet Policy', value: '1 Domestic Feline Authorized ($50/mo pet rent)' }
      ];
      state.items = [
        { description: 'Monthly Base Rent (Unit 4B)', quantity: 1.0, unit_price: 3200.00 },
        { description: 'Security Deposit (Statutory Trust Escrow)', quantity: 1.0, unit_price: 3200.00 },
        { description: 'Assigned Subterranean Parking Stall (#14)', quantity: 1.0, unit_price: 175.00 },
        { description: 'Pet Rent (Authorized Domestic Feline)', quantity: 1.0, unit_price: 50.00 }
      ];
      state.sections = [
        { title: '1. Rent Payment & Grace Period', content: 'Rent is due on the 1st calendar day of each month. A late fee of 5% ($160.00) applies if rent is not received by 11:59 PM on the 5th calendar day.' },
        { title: '2. Security Deposit & Trust Account', content: 'The security deposit of $3,200.00 is held in a segregated client trust account at Chase Bank pursuant to state law. The deposit will be itemized and returned within 21 days of tenancy termination.' },
        { title: '3. Landlord Right of Entry', content: 'Landlord shall provide at least 24 hours prior written notice before entering the premises for non-emergency inspection, maintenance, or prospective tenant showings.' },
        { title: '4. Lead-Based Paint & Habitability Disclosure', content: 'Housing built before 1978 may contain lead-based paint. Landlord has provided the federally approved lead hazard information pamphlet "Protect Your Family From Lead In Your Home".' }
      ];
      state.signatures = [
        { signer_name: 'Elena Rostova', signer_title: 'Managing Agent, Pacifica Holdings', date: '2026-10-01' },
        { signer_name: 'Marcus Vance', signer_title: 'Primary Lessee', date: '2026-10-01' },
        { signer_name: 'Eleanor Vance', signer_title: 'Co-Lessee', date: '2026-10-01' }
      ];
    } else if (type === 'commercial_nnn_lease') {
      state.header_subtitle = 'Class-A Office / Tech Flex Facility Lease Abstract';
      state.from = {
        name: 'Arthur Sterling',
        company: 'Apex Commercial Capital REIT',
        address: '555 California St, Suite 4200, San Francisco, CA 94104',
        email: 'leasing@apexcommercial.com',
        phone: '+1 (415) 980-2200',
        tax_id: 'US-94-1182903',
        registration_number: 'CRE Broker Lic #00918274'
      };
      state.to = {
        name: 'Dr. Sarah Lin',
        company: 'NeuralCloud Systems Inc.',
        address: '500 Howard Street, Suite 800, San Francisco, CA 94105',
        email: 'legal@neuralcloud.ai',
        phone: '+1 (415) 555-8821',
        registration_number: 'DE-CORP-774912'
      };
      state.custom_fields = [
        { label: 'Premises Address', value: '500 Howard Street, Suite 800, San Francisco, CA 94105' },
        { label: 'Rentable Area (RSF)', value: '14,500 RSF (Usable: 12,850 USF)' },
        { label: 'Load Factor', value: '1.1284 (12.84%)' },
        { label: 'Initial Lease Term', value: '60 Months (5 Years)' },
        { label: 'Commencement Date', value: 'November 1, 2026' },
        { label: 'Permitted Use', value: 'Executive Tech Offices, R&D, Software Engineering' },
        { label: 'Proportionate Share', value: '18.42% of Building Operating Expenses' }
      ];
      state.items = [
        { description: 'Monthly Base Rent ($68.00 / RSF / Year)', quantity: 1.0, unit_price: 82166.67 },
        { description: 'Common Area Maintenance (CAM / NNN Share)', quantity: 1.0, unit_price: 21025.00 },
        { description: 'Building Property Tax Escrow Reserve', quantity: 1.0, unit_price: 8450.00 },
        { description: 'Casualty & Commercial General Liability Escrow', quantity: 1.0, unit_price: 2180.00 }
      ];
      state.sections = [
        { title: '1. Operating Expense Reconciliation', content: 'Tenant pays 18.42% Proportionate Share of actual Operating Expenses (Taxes, Insurance, Utilities, HVAC upkeep, Janitorial). Annual reconciliation statement delivered within 90 days of calendar year end.' },
        { title: '2. Tenant Improvement (TI) Allowance', content: 'Landlord provides a turnkey TI allowance of $45.00 per RSF ($652,500.00 total) to be disbursed via architect progress certificates.' },
        { title: '3. Assignment & Subletting', content: 'Tenant may not assign or sublet without Landlord\'s prior written consent, which shall not be unreasonably withheld, conditioned, or delayed.' },
        { title: '4. Subordination & SNDA', content: 'This Lease is subject and subordinate to all ground leases and mortgages, conditioned upon Tenant receiving standard SNDA covenant from senior lender.' }
      ];
      state.signatures = [
        { signer_name: 'Arthur Sterling', signer_title: 'Senior Managing Director, Apex Capital REIT', date: '2026-10-01' },
        { signer_name: 'Dr. Sarah Lin', signer_title: 'Chief Executive Officer, NeuralCloud Systems Inc.', date: '2026-10-01' }
      ];
    } else if (type === 'property_management_statement') {
      state.header_subtitle = 'Owner Remittance & Portfolio Operating Summary';
      state.from = {
        name: 'Evelyn Reed, CPM',
        company: 'Beacon Hill Property Management LLC',
        address: '120 Commonwealth Ave, Boston, MA 02116',
        email: 'accounting@beaconhillpm.com',
        phone: '+1 (617) 555-0142',
        tax_id: 'US-04-9928172',
        registration_number: 'Broker Lic #01884920'
      };
      state.to = {
        name: 'Arthur Highland, Managing Partner',
        company: 'Highland Real Estate Investors IV LLC',
        address: '75 State Street, Floor 22, Boston, MA 02109',
        email: 'assets@highlandrei.com',
        phone: '+1 (617) 555-9000'
      };
      state.custom_fields = [
        { label: 'Property Name', value: 'The Beacon Residences (24 Units)' },
        { label: 'Property Address', value: '120 Commonwealth Ave, Boston, MA 02116' },
        { label: 'Statement Period', value: 'September 1, 2026 – September 30, 2026' },
        { label: 'Portfolio Occupancy', value: '95.8% (23 of 24 Units Occupied)' },
        { label: 'Reserve Escrow Account', value: '$25,000.00 (Maintained at Boston Private)' }
      ];
      state.items = [
        { description: 'Gross Actual Rents Collected (23 Units)', quantity: 1.0, unit_price: 65550.00, discount: 0.0 },
        { description: 'Late Fees & Parking Surcharges Collected', quantity: 1.0, unit_price: 1150.00, discount: 0.0 },
        { description: 'Property Management Fee (6.0% of Collections)', quantity: 1.0, unit_price: -4002.00, discount: 0.0 },
        { description: 'HVAC Emergency Compressor Repair (Unit 4B)', quantity: 1.0, unit_price: -1850.00, discount: 0.0 },
        { description: 'Landscaping & Grounds Upkeep Contract', quantity: 1.0, unit_price: -1200.00, discount: 0.0 },
        { description: 'Common Area Electric & Water (Eversource)', quantity: 1.0, unit_price: -2410.00, discount: 0.0 }
      ];
      state.sections = [
        { title: 'Trust Fund & Escrow Certification', content: 'All operating funds and security deposits are segregated in federally insured trust accounts at Boston Private Bank pursuant to Massachusetts General Laws Chapter 186 § 15B.' }
      ];
      state.signatures = [
        { signer_name: 'David Thornton, CPM', signer_title: 'Managing Principal, Beacon Hill PM', date: '2026-10-01' }
      ];
    } else if (type === 'closing_settlement_statement') {
      state.header_subtitle = 'Buyer / Seller Combined Settlement Disclosures (RESPA / ALTA Standard)';
      state.from = {
        name: 'First American Title Company',
        company: 'Escrow Division - Settlement Services',
        address: '1842 Pacific Coast Highway, Suite 100, Malibu, CA 90265',
        email: 'escrow.settlement@firstam.com',
        phone: '+1 (310) 555-0188',
        tax_id: 'US-95-1829012'
      };
      state.to = {
        name: 'Robert Vance (Buyer) & Claire Sterling (Seller)',
        company: 'Malibu Coastal Transaction',
        address: '1842 Pacific Coast Highway, Malibu, CA 90265'
      };
      state.custom_fields = [
        { label: 'Property Address', value: '1842 Pacific Coast Highway, Malibu, CA 90265' },
        { label: 'APN / Parcel ID', value: '4458-012-044' },
        { label: 'Escrow File #', value: 'FE-2026-9921' },
        { label: 'Settlement Date', value: 'October 15, 2026' },
        { label: 'Title / Escrow Officer', value: 'First American Title (Officer: Diane Warren)' },
        { label: 'Primary Loan', value: '$1,450,000.00 (First Republic / JPMorgan)' }
      ];
      state.items = [
        { description: 'Contract Purchase Price', quantity: 1.0, unit_price: 2100000.00 },
        { description: 'Listing Brokerage Commission (2.50%)', quantity: 1.0, unit_price: 52500.00 },
        { description: 'Selling Brokerage Commission (2.50%)', quantity: 1.0, unit_price: 52500.00 },
        { description: 'Owner\'s Title Insurance Policy', quantity: 1.0, unit_price: 4200.00 },
        { description: 'Lender\'s ALTA Title Policy', quantity: 1.0, unit_price: 1850.00 },
        { description: 'County Transfer Tax ($1.10 per $1,000)', quantity: 1.0, unit_price: 2310.00 },
        { description: 'Escrow Settlement Closing Fee', quantity: 1.0, unit_price: 2400.00 }
      ];
      state.sections = [
        { title: 'RESPA Settlement Notice', content: 'This Settlement Statement is prepared pursuant to the Real Estate Settlement Procedures Act (RESPA) 12 U.S.C. § 2601 et seq. All settlement charges and disbursements have been itemized with zero unearned fees or kickbacks.' }
      ];
      state.signatures = [
        { signer_name: 'Diane Warren', signer_title: 'Senior Escrow Officer, First American Title', date: '2026-10-01' },
        { signer_name: 'Robert Vance', signer_title: 'Buyer Signatory', date: '2026-10-01' },
        { signer_name: 'Claire Sterling', signer_title: 'Seller Signatory', date: '2026-10-01' }
      ];
    } else if (type === 'rent_roll') {
      state.header_subtitle = 'Asset Valuation & Tenancy Census Audit';
      state.from = {
        name: 'Sunset Real Estate Partners LP',
        company: 'Asset Management Division',
        address: '450 7th Ave, Suite 1200, Brooklyn, NY 11215',
        email: 'portfolio@sunsetpartners.com',
        phone: '+1 (718) 555-0144'
      };
      state.to = {
        name: 'Commercial Underwriting Team',
        company: 'First Regional Mortgage Corporation',
        address: '100 Wall Street, Floor 14, New York, NY 10005',
        email: 'underwriting@firstregional.com'
      };
      state.custom_fields = [
        { label: 'Property', value: 'Sunset Park Apartments (16 Units)' },
        { label: 'Address', value: '450 7th Ave, Brooklyn, NY 11215' },
        { label: 'As-Of Date', value: 'October 1, 2026' },
        { label: 'Total Units / Occupancy', value: '16 Units · 93.75% Occupied (15 Units)' },
        { label: 'Annual Gross Scheduled Rent', value: '$547,200.00' }
      ];
      state.items = [
        { description: 'Unit 1A · 1BR/1BA · Sarah Miller · Exp: 05/2027', quantity: 1.0, unit_price: 2650.00 },
        { description: 'Unit 1B · 2BR/1.5BA · David Chen · Exp: 08/2027', quantity: 1.0, unit_price: 3400.00 },
        { description: 'Unit 2A · 1BR/1BA · Carlos Rivera · Exp: 11/2026', quantity: 1.0, unit_price: 2700.00 },
        { description: 'Unit 2B · 2BR/2BA · Michael Ross · Exp: 03/2027', quantity: 1.0, unit_price: 3650.00 },
        { description: 'Unit 3A · Studio · VACANT (Turnover / Renovating)', quantity: 1.0, unit_price: 0.00 },
        { description: 'Unit 3B · 2BR/1.5BA · Amanda Vance · Exp: 09/2027', quantity: 1.0, unit_price: 3500.00 },
        { description: 'Unit 4A · Penthouse 3BR/2BA · Robert Sterling · Exp: 01/2028', quantity: 1.0, unit_price: 5200.00 }
      ];
      state.sections = [
        { title: 'Underwriting & Occupancy Attestation', content: 'This rent roll represents an accurate census of all active residential leases, deposits held, and contractual terms as verified against executed lease agreements.' }
      ];
      state.signatures = [
        { signer_name: 'Jessica Reynolds', signer_title: 'Senior Portfolio Analyst', date: '2026-10-01' }
      ];
    } else if (type === 'notice_to_vacate') {
      state.header_subtitle = 'Statutory Notice to Pay Rent or Quit Premises';
      state.from = {
        name: 'Elena Rostova',
        company: 'Pacifica Property Management LLC',
        address: '100 Montgomery St, San Francisco, CA 94104',
        email: 'management@pacificapm.com',
        phone: '+1 (415) 890-4421'
      };
      state.to = {
        name: 'Jordan Blake & All Occupants',
        company: '',
        address: '880 Harrison St, Apt 302, San Francisco, CA 94107',
        phone: '+1 (415) 555-3391'
      };
      state.custom_fields = [
        { label: 'Premises Address', value: '880 Harrison St, Apt 302, San Francisco, CA 94107' },
        { label: 'Tenant Name', value: 'Jordan Blake & All Unknown Occupants' },
        { label: 'Total Delinquent Balance', value: '$4,250.00' },
        { label: 'Statutory Cure Period', value: '3 Calendar Days from Service' }
      ];
      state.items = [
        { description: 'Delinquent Rent - August 2026', quantity: 1.0, unit_price: 2050.00 },
        { description: 'Delinquent Rent - September 2026', quantity: 1.0, unit_price: 2050.00 },
        { description: 'Statutory Late Charge (August)', quantity: 1.0, unit_price: 75.00 },
        { description: 'Statutory Late Charge (September)', quantity: 1.0, unit_price: 75.00 }
      ];
      state.sections = [
        { title: 'DEMAND TO PAY OR QUIT', content: 'WITHIN THREE (3) DAYS after service upon you of this notice, you are required to pay the total amount of $4,250.00, or vacate and deliver surrender of the premises to Landlord. Failure to comply will result in immediate legal proceedings for unlawful detainer.' },
        { title: 'DECLARATION OF SERVICE', content: 'I declare under penalty of perjury under the laws of the State of California that on October 1, 2026, I served this Notice upon Tenant by personal delivery / conspicuous posting at the premises.' }
      ];
      state.signatures = [
        { signer_name: 'Elena Rostova', signer_title: 'Managing Agent for Pacifica PM', date: '2026-10-01' }
      ];
    } else if (type === 'security_deposit_disposition') {
      state.header_subtitle = 'Statutory Accounting & Itemized Repair Deductions';
      state.from = {
        name: 'Marcus Sterling',
        company: 'Oceanview Residential Properties LLC',
        address: '1420 4th Street, Santa Monica, CA 90401',
        email: 'accounting@oceanviewapts.com',
        phone: '+1 (310) 555-8291'
      };
      state.to = {
        name: 'Sophie Laurent',
        company: '',
        address: '224 Ocean Ave, Apt 12, Santa Monica, CA 90402',
        email: 'sophie.laurent@example.com'
      };
      state.custom_fields = [
        { label: 'Property Address', value: '1420 4th Street, Unit 201, Santa Monica, CA 90401' },
        { label: 'Tenant Name', value: 'Sophie Laurent' },
        { label: 'Move-Out Inspection Date', value: 'September 28, 2026' },
        { label: 'Original Deposit Paid', value: '$3,500.00' },
        { label: 'Statutory Interest Accrued', value: '$28.42' }
      ];
      state.items = [
        { description: 'Original Security Deposit Held', quantity: 1.0, unit_price: 3500.00, discount: 0.0 },
        { description: 'Accrued Statutory Trust Interest', quantity: 1.0, unit_price: 28.42, discount: 0.0 },
        { description: 'Professional Deep Clean & Carpet Sanitization (Receipt #441)', quantity: 1.0, unit_price: -320.00, discount: 0.0 },
        { description: 'Drywall Patching & Paint Beyond Normal Wear (Room 2)', quantity: 1.0, unit_price: -450.00, discount: 0.0 },
        { description: 'Replacement of Missing Garage Gate Remote', quantity: 1.0, unit_price: -120.00, discount: 0.0 },
        { description: 'Unpaid Prorated Utility Charges (Water / Trash)', quantity: 1.0, unit_price: -84.20, discount: 0.0 }
      ];
      state.sections = [
        { title: 'Statutory Return Notice', content: 'Enclosed is check #4092 drawn on First Republic Bank for the net refund of $2,554.22. Pursuant to state law, itemized receipts and contractor invoices are attached.' }
      ];
      state.signatures = [
        { signer_name: 'Marcus Sterling', signer_title: 'Property Manager', date: '2026-10-01' }
      ];
    } else if (type === 'real_estate_commission') {
      state.header_subtitle = 'Brokerage Commission Disbursement & Transaction Reconciliation';
      state.from = {
        name: 'Elizabeth Vance',
        company: 'Vanguard Realty Partners LLC',
        address: '240 Mercer Street, New York, NY 10012',
        email: 'disbursements@vanguardrealty.com',
        phone: '+1 (212) 555-0912',
        tax_id: 'US-13-8849201'
      };
      state.to = {
        name: 'Diane Warren, Senior Escrow Officer',
        company: 'First American Title & Escrow Co.',
        address: '666 Fifth Avenue, Floor 14, New York, NY 10103',
        email: 'escrow.ny@firstam.com'
      };
      state.custom_fields = [
        { label: 'Property Address', value: '240 Mercer Street, New York, NY 10012' },
        { label: 'MLS Listing #', value: 'RLS-948102' },
        { label: 'Gross Closing Price', value: '$3,850,000.00' },
        { label: 'Closing Date', value: 'October 2, 2026' },
        { label: 'Listing Broker Lic', value: 'NY-DOS-104928' },
        { label: 'Selling Broker Lic', value: 'NY-DOS-309182' }
      ];
      state.items = [
        { description: 'Total Gross Commission (5.00% of $3,850,000.00)', quantity: 1.0, unit_price: 192500.00, discount: 0.0 },
        { description: 'Listing Brokerage Share (50.00%)', quantity: 1.0, unit_price: -96250.00, discount: 0.0 },
        { description: 'Brokerage House Split (70/30 Tier)', quantity: 1.0, unit_price: -28875.00, discount: 0.0 },
        { description: 'Transaction Coordinator (TC) Fee', quantity: 1.0, unit_price: -450.00, discount: 0.0 },
        { description: 'E&O Insurance Risk Fee', quantity: 1.0, unit_price: -150.00, discount: 0.0 }
      ];
      state.sections = [
        { title: 'Disbursement Authorization', content: 'Escrow Officer is authorized and instructed to disburse the proceeds as itemized above at time of recording pursuant to the cooperative brokerage agreement.' }
      ];
      state.signatures = [
        { signer_name: 'Elizabeth Vance', signer_title: 'Managing Principal Broker', date: '2026-10-01' }
      ];
    } else if (type === 'property_inspection_checklist') {
      state.header_subtitle = 'Comprehensive Room-by-Room Condition & Inventory Verification';
      state.from = {
        name: 'Elena Rostova',
        company: 'Lincoln Park Residential Management LLC',
        address: '772 Lakeview Terrace, Chicago, IL 60614',
        email: 'inspections@lincolnparkpm.com',
        phone: '+1 (312) 555-8910'
      };
      state.to = {
        name: 'Alexander Wright',
        company: '',
        address: '772 Lakeview Terrace, Unit 3, Chicago, IL 60614',
        email: 'alex.wright@example.com'
      };
      state.custom_fields = [
        { label: 'Property Address', value: '772 Lakeview Terrace, Unit 3, Chicago, IL 60614' },
        { label: 'Tenant Name', value: 'Alexander Wright' },
        { label: 'Inspection Type', value: 'Joint Move-In Walkthrough' },
        { label: 'Date of Walkthrough', value: 'October 1, 2026' },
        { label: 'Keys Delivered', value: '2 Front Door, 1 Mailbox, 1 Garage Fob' }
      ];
      state.items = [
        { description: 'Living Room & Entryway (Floors, Baseboards, Outlets) - Pristine', quantity: 1.0, unit_price: 0.00 },
        { description: 'Kitchen (Granite Counters, Bosch Dishwasher, Range) - Clean', quantity: 1.0, unit_price: 0.00 },
        { description: 'Primary Bedroom (Hardwood, Closets, Window Locks) - Satisfactory', quantity: 1.0, unit_price: 0.00 },
        { description: 'Bathroom (Tile, Grout, Fixtures, Exhaust Fan) - Good Condition', quantity: 1.0, unit_price: 0.00 },
        { description: 'Smoke & Carbon Monoxide Detectors - Tested & Operational', quantity: 1.0, unit_price: 0.00 }
      ];
      state.sections = [
        { title: 'Joint Tenant & Landlord Acknowledgment', content: 'The undersigned Tenant and Landlord/Agent have jointly inspected the premises. The condition noted above accurately reflects the physical state of the premises at occupancy.' }
      ];
      state.signatures = [
        { signer_name: 'Elena Rostova', signer_title: 'Property Inspector', date: '2026-10-01' },
        { signer_name: 'Alexander Wright', signer_title: 'Tenant Signatory', date: '2026-10-01' }
      ];
    } else if (type === 'letter_of_intent_cre') {
      state.header_subtitle = 'Non-Binding Proposal to Lease Commercial Retail / Office Premises';
      state.from = {
        name: 'David Vance',
        company: 'Vance Health Technologies Inc.',
        address: '1200 Avenue of the Americas, Suite 1500, New York, NY 10036',
        email: 'leasing@vancehealth.com',
        phone: '+1 (212) 555-8820'
      };
      state.to = {
        name: 'Arthur Sterling, Senior VP of Leasing',
        company: 'Metropolitan Commercial Realty REIT',
        address: '350 Fifth Avenue, Floor 44, New York, NY 10118',
        email: 'leasing@metrorealty.com'
      };
      state.custom_fields = [
        { label: 'Property Name', value: 'Metropolitan Plaza Building' },
        { label: 'Premises Address', value: '1200 Avenue of the Americas, Suite 1500, New York, NY 10036' },
        { label: 'Rentable Area', value: '8,200 RSF' },
        { label: 'Proposed Lease Term', value: '7 Years (84 Months)' },
        { label: 'Proposed Commencement', value: 'January 1, 2027' },
        { label: 'Tenant Improvement Allowance', value: '$60.00 / RSF ($492,000.00)' },
        { label: 'Rent Abatement', value: '4 Months Free Rent' }
      ];
      state.items = [
        { description: 'Year 1 Annual Base Rent ($72.00 / RSF / Year)', quantity: 1.0, unit_price: 590400.00 },
        { description: 'Tenant Improvement Contribution', quantity: 1.0, unit_price: 492000.00 },
        { description: 'Security Deposit (3 Months Letter of Credit)', quantity: 1.0, unit_price: 147600.00 }
      ];
      state.sections = [
        { title: 'Non-Binding Nature', content: 'This Letter of Intent represents an expression of interest and business terms for discussion purposes only. Neither party shall be bound until a formal, definitive lease agreement is executed.' }
      ];
      state.signatures = [
        { signer_name: 'David Vance', signer_title: 'Chief Operating Officer, Vance Health', date: '2026-10-01' }
      ];
    } else if (type === 'tenant_estoppel') {
      state.header_subtitle = 'Definitive Statement of Lease Standing for Lender & Purchaser Reliance';
      state.from = {
        name: 'Dr. Evelyn Reed',
        company: 'Global Health Partners LLC',
        address: '400 Park Place, Suite 400, Chicago, IL 60601',
        email: 'admin@globalhealthpartners.org',
        phone: '+1 (312) 555-8833'
      };
      state.to = {
        name: 'Commercial Closing & Underwriting Desk',
        company: 'Park Place Realty Holdings LLC',
        address: '10 South Wacker Dr, Floor 32, Chicago, IL 60606',
        email: 'closing@parkplacerealty.com'
      };
      state.custom_fields = [
        { label: 'Property Name', value: 'Park Place Commercial Tower, Unit 400' },
        { label: 'Tenant Entity', value: 'Global Health Partners LLC' },
        { label: 'Landlord Entity', value: 'Park Place Realty Holdings LLC' },
        { label: 'Current Monthly Base Rent', value: '$18,500.00' },
        { label: 'Security Deposit Held', value: '$37,000.00' },
        { label: 'Lease Expiration Date', value: 'December 31, 2029' }
      ];
      state.items = [
        { description: 'Current Monthly Base Rent', quantity: 1.0, unit_price: 18500.00 },
        { description: 'Current Monthly CAM Contribution', quantity: 1.0, unit_price: 4200.00 },
        { description: 'Security Deposit Held by Landlord', quantity: 1.0, unit_price: 37000.00 }
      ];
      state.sections = [
        { title: 'Certification & Reliance', content: 'Tenant certifies that the Lease is in full force and effect without modification, Landlord is not in default, and rent is paid through October 31, 2026. Tenant acknowledges that Purchaser and its Lenders are relying on this certification.' }
      ];
      state.signatures = [
        { signer_name: 'Dr. Evelyn Reed', signer_title: 'Managing Member, Global Health Partners LLC', date: '2026-10-01' }
      ];
    } else if (type === 'room_rental_lease') {
      state.header_subtitle = 'Individual Bedroom Lease & Shared Common-Area Utility Split Schedule';
      state.from = { name: 'Vance Residential Group', company: 'Vance Co-Living Properties LLC', address: '842 Valencia St, San Francisco, CA 94110', email: 'leasing@vanceresidential.com', phone: '+1 (415) 555-0190' };
      state.to = { name: 'Liam Chen', company: '', address: '842 Valencia St, Bedroom B, San Francisco, CA 94110', email: 'liam.chen@example.com' };
      state.custom_fields = [
        { label: 'Leased Bedroom', value: 'Private Bedroom B (Upper Floor, Keyed Lock #B2)' },
        { label: 'Shared Common Areas', value: 'Kitchen, Living Room, Laundry & Patio' },
        { label: 'Utility Split Ratio', value: '25% Pro-Rata Share (4 Private Bedrooms)' },
        { label: 'Quiet Hours', value: '10:00 PM – 7:00 AM Sun–Thu; 11:00 PM – 8:00 AM Fri–Sat' }
      ];
      state.items = [
        { description: 'Monthly Base Rent — Private Bedroom B', quantity: 1.0, unit_price: 1350.00 },
        { description: 'Fixed High-Speed Fiber & Common Area Cleaning Share', quantity: 1.0, unit_price: 65.00 },
        { description: 'Refundable Bedroom Security Deposit', quantity: 1.0, unit_price: 1350.00 }
      ];
      state.sections = [
        { title: 'Individual Liability & House Rules', content: 'Co-Tenant is solely liable for Bedroom B base rent and a 25% share of metered electric/gas/water utilities. Overnight guests may not exceed 7 nights per calendar month without Landlord consent.' }
      ];
    } else if (type === 'lease_renewal_agreement') {
      state.header_subtitle = 'Statutory Lease Term Extension & Adjusted Monthly Rent Schedule';
      state.from = { name: 'Vance Residential Group', company: 'Vance Residential Holdings LLC', address: '742 Evergreen Terrace, Suite 200, Austin, TX 78701', email: 'renewals@vanceresidential.com' };
      state.to = { name: 'Marcus & Elena Sterling', company: '', address: '1420 South Congress Ave, Unit 4B, Austin, TX 78704', email: 'm.sterling@example.com' };
      state.custom_fields = [
        { label: 'Original Lease Date', value: 'November 1, 2025' },
        { label: 'Renewal Commencement', value: 'November 1, 2026' },
        { label: 'Renewal Expiration', value: 'October 31, 2027 (12 Months)' },
        { label: 'Prior Monthly Rent', value: '$2,650.00 / month' },
        { label: 'Renewed Monthly Rent', value: '$2,730.00 / month (+3.0%)' }
      ];
      state.items = [
        { description: 'Renewed Monthly Base Rent (Nov 2026 – Oct 2027)', quantity: 1.0, unit_price: 2730.00 },
        { description: 'Reserved Covered Parking Space #18 (Unchanged)', quantity: 1.0, unit_price: 75.00 }
      ];
      state.sections = [
        { title: 'Ratification of Original Lease', content: 'Except as expressly modified by this Renewal Agreement, all covenants, security deposit terms, and statutory disclosures of the Original Lease remain in full force and effect.' }
      ];
    } else if (type === 'lease_amendment') {
      state.header_subtitle = 'Mid-Term Lease Modification · Pet, Occupant & Parking Addendum';
      state.from = { name: 'Vance Residential Group', company: 'Vance Residential Holdings LLC', address: '742 Evergreen Terrace, Suite 200, Austin, TX 78701', email: 'leasing@vanceresidential.com' };
      state.to = { name: 'Marcus & Elena Sterling', company: '', address: '1420 South Congress Ave, Unit 4B, Austin, TX 78704', email: 'm.sterling@example.com' };
      state.custom_fields = [
        { label: 'Amendment Number', value: 'Amendment No. 1 (Pet & Parking Addendum)' },
        { label: 'Effective Date', value: 'November 1, 2026' },
        { label: 'Approved Pet', value: '1 Golden Retriever ("Milo", 55 lbs, Vaccinated)' }
      ];
      state.items = [
        { description: 'Refundable Pet Security Deposit Addendum', quantity: 1.0, unit_price: 300.00 },
        { description: 'Monthly Pet Rent Adjustment', quantity: 1.0, unit_price: 35.00 }
      ];
      state.sections = [
        { title: 'Amended Lease Provisions', content: 'Section 14 (Pets) of the Lease is hereby amended to authorize one (1) domesticated dog described above, subject to current rabies vaccination records and tenant liability coverage.' }
      ];
    } else if (type === 'rental_application') {
      state.header_subtitle = 'Prospective Tenant Residency, Employment & FCRA Screening Application';
      state.from = { name: 'Vance Residential Group', company: 'Vance Property Management LLC', address: '742 Evergreen Terrace, Suite 200, Austin, TX 78701', email: 'screening@vanceresidential.com' };
      state.to = { name: 'Sophia Alvarez', company: '', address: '910 West 6th St, Apt 312, Austin, TX 78703', email: 'sophia.alvarez@example.com', phone: '+1 (512) 555-0144' };
      state.custom_fields = [
        { label: 'Target Unit', value: '1420 South Congress Ave, Unit 6A, Austin, TX 78704' },
        { label: 'Desired Move-In Date', value: 'November 15, 2026' },
        { label: 'Current Employer & Title', value: 'Meridian Cloud Systems · Senior Product Designer' },
        { label: 'Verified Gross Monthly Income', value: '$11,400.00 / month (4.1x Rent-to-Income Ratio)' },
        { label: 'Prior Landlord Reference', value: 'Highland Park Lofts (+1 512-555-0199 · 24 Months On-Time)' }
      ];
      state.items = [
        { description: 'Statutory Tenant Screening & Credit/Eviction Report Fee', quantity: 1.0, unit_price: 45.00 }
      ];
      state.sections = [
        { title: 'FCRA Consumer Report Authorization', content: 'Applicant authorizes Landlord and its consumer reporting agency to obtain consumer credit, eviction, and criminal history reports pursuant to the Fair Credit Reporting Act (15 U.S.C. § 1681b).' }
      ];
    } else if (type === 'adverse_action_notice') {
      state.header_subtitle = 'Statutory Fair Credit Reporting Act (15 U.S.C. § 1681m(a)) Consumer Disclosure';
      state.from = { name: 'Vance Compliance Department', company: 'Vance Property Management LLC', address: '742 Evergreen Terrace, Suite 200, Austin, TX 78701', email: 'compliance@vanceresidential.com' };
      state.to = { name: 'Jordan Blake', company: '', address: '220 Barton Springs Rd, Austin, TX 78704', email: 'j.blake@example.com' };
      state.custom_fields = [
        { label: 'Application Reference', value: 'APP-2026-8841 (Unit 3C)' },
        { label: 'Adverse Action Taken', value: 'Conditional Approval Requiring Co-Signer or Additional Deposit' },
        { label: 'Consumer Reporting Agency', value: 'TransUnion Rental Screening Solutions, Inc. (P.O. Box 1000, Chester, PA 19016 · 1-800-888-4213)' },
        { label: 'Credit Score Disclosed', value: '618 (Score Range: 300–850 · Date: 2026-10-24)' },
        { label: 'Key Score Factors', value: '1. Ratio of revolving balances to credit limits; 2. Length of credit history' }
      ];
      state.items = [];
      state.sections = [
        { title: 'Statutory Consumer Rights Under 15 U.S.C. § 1681m(a)', content: 'The consumer reporting agency did not make this leasing decision and cannot explain why it was made. You have the right to obtain a free copy of your consumer report within 60 days and to dispute the accuracy or completeness of any record.' }
      ];
    } else if (type === 'notice_of_entry') {
      state.header_subtitle = 'Statutory Advance Written Notice of Landlord Entry to Premises';
      state.from = { name: 'Vance Property Operations', company: 'Vance Property Management LLC', address: '742 Evergreen Terrace, Suite 200, Austin, TX 78701', email: 'maintenance@vanceresidential.com', phone: '+1 (512) 555-0188' };
      state.to = { name: 'Marcus & Elena Sterling', company: '', address: '1420 South Congress Ave, Unit 4B, Austin, TX 78704', email: 'm.sterling@example.com' };
      state.custom_fields = [
        { label: 'Scheduled Entry Date', value: 'Thursday, October 29, 2026' },
        { label: 'Entry Time Window', value: '10:00 AM – 1:00 PM Local Time' },
        { label: 'Authorized Personnel', value: 'Apex HVAC Mechanical Services & Property Supervisor' },
        { label: 'Statutory Purpose', value: 'Annual Winter HVAC & Smoke/CO Detector Safety Inspection' }
      ];
      state.items = [];
      state.sections = [
        { title: 'Notice & Right of Quiet Enjoyment', content: 'This notice is served in accordance with state statutory advance notice requirements. Tenant presence is welcomed but not required; personnel will knock, announce entry, and secure the premises upon completion.' }
      ];
    } else if (type === 'rent_increase_notice') {
      state.header_subtitle = 'Statutory Advance Written Notice of Monthly Rent Adjustment';
      state.from = { name: 'Vance Residential Group', company: 'Vance Residential Holdings LLC', address: '742 Evergreen Terrace, Suite 200, Austin, TX 78701', email: 'leasing@vanceresidential.com' };
      state.to = { name: 'Marcus & Elena Sterling', company: '', address: '1420 South Congress Ave, Unit 4B, Austin, TX 78704', email: 'm.sterling@example.com' };
      state.custom_fields = [
        { label: 'Effective Date of New Rent', value: 'January 1, 2027 (65+ Days Advance Notice)' },
        { label: 'Current Monthly Rent', value: '$2,650.00 / month' },
        { label: 'New Monthly Rent', value: '$2,750.00 / month' },
        { label: 'Net Adjustment', value: '+$100.00 / month (+3.77%)' }
      ];
      state.items = [
        { description: 'Current Monthly Base Rent (Through Dec 31, 2026)', quantity: 1.0, unit_price: 2650.00, discount: 0.0 },
        { description: 'Monthly Base Rent Increase Adjustment (Effective Jan 1, 2027 · +3.77%)', quantity: 1.0, unit_price: 100.00, discount: 0.0 }
      ];
      state.sections = [
        { title: 'Statutory Notice & Election', content: 'Please confirm renewal acceptance or intent to vacate at least 30 days prior to the Effective Date. All other lease provisions remain unchanged.' }
      ];
    } else if (type === 'mutual_lease_termination') {
      state.header_subtitle = 'Bilateral Early Lease Surrender, Release of Claims & Move-Out Settlement';
      state.from = { name: 'Vance Residential Group', company: 'Vance Residential Holdings LLC', address: '742 Evergreen Terrace, Suite 200, Austin, TX 78701', email: 'leasing@vanceresidential.com' };
      state.to = { name: 'Marcus & Elena Sterling', company: '', address: '1420 South Congress Ave, Unit 4B, Austin, TX 78704', email: 'm.sterling@example.com' };
      state.custom_fields = [
        { label: 'Original Lease Expiration', value: 'May 31, 2027' },
        { label: 'Agreed Early Surrender Date', value: 'November 30, 2026 at 5:00 PM' },
        { label: 'Key Return & Walkthrough', value: 'On-Site Management Office upon vacant broom-clean delivery' }
      ];
      state.items = [
        { description: 'Agreed Early Lease Buyout & Re-Leasing Settlement Fee', quantity: 1.0, unit_price: 1500.00 }
      ];
      state.sections = [
        { title: 'Mutual Release of Future Rent Obligations', content: 'Upon timely surrender of possession in broom-clean condition and payment of the settlement consideration above, Landlord and Tenant mutually release one another from all future rent obligations accruing after the Surrender Date.' }
      ];
    } else if (type === 'tenant_rent_ledger') {
      state.header_subtitle = 'Chronological Audit Trail of Charges, Payments, Late Fees & Running Balance';
      state.from = { name: 'Vance Accounting & Escrow', company: 'Vance Property Management LLC', address: '742 Evergreen Terrace, Suite 200, Austin, TX 78701', email: 'accounting@vanceresidential.com' };
      state.to = { name: 'Marcus & Elena Sterling', company: '', address: '1420 South Congress Ave, Unit 4B, Austin, TX 78704', email: 'm.sterling@example.com' };
      state.custom_fields = [
        { label: 'Audit Period', value: 'August 1, 2026 – October 25, 2026' },
        { label: 'Monthly Base Rent', value: '$2,650.00 (Due 1st · 5-Day Grace Period)' },
        { label: 'Security Deposit in Escrow', value: '$2,650.00 (First Republic Escrow #8841)' }
      ];
      state.items = [
        { description: '2026-08-01 · August Base Rent Charge', quantity: 1.0, unit_price: 2650.00, discount: 0.0 },
        { description: '2026-08-02 · ACH Payment Received (#ACH-99102)', quantity: 1.0, unit_price: -2650.00, discount: 0.0 },
        { description: '2026-09-01 · September Base Rent Charge', quantity: 1.0, unit_price: 2650.00, discount: 0.0 },
        { description: '2026-09-03 · ACH Payment Received (#ACH-99841)', quantity: 1.0, unit_price: -2650.00, discount: 0.0 },
        { description: '2026-10-01 · October Base Rent Charge', quantity: 1.0, unit_price: 2650.00, discount: 0.0 },
        { description: '2026-10-07 · Statutory Late Fee Assessment (Past Grace Period)', quantity: 1.0, unit_price: 95.00, discount: 0.0 }
      ];
      state.sections = [
        { title: 'Ledger Certification', content: 'I hereby certify that the foregoing Tenant Rent & Payment Ledger is a true, complete, and accurate accounting of all charges and credits posted to the tenancy account.' }
      ];
    } else if (type === 'schedule_of_real_estate_owned') {
      state.header_subtitle = 'Portfolio Balance Sheet · Market Valuation, Senior Debt, LTV, DSCR & Annual NOI';
      state.from = { name: 'Daniel Hallman', company: 'Hallman Capital Properties LLC', address: '1200 Market St, Suite 400, Chattanooga, TN 37402', email: 'capital@hallmanproperties.com' };
      state.to = { name: 'Commercial Real Estate Underwriting Committee', company: 'First Horizon Commercial Bank', address: '701 Market St, Chattanooga, TN 37402', email: 'cre.underwriting@firsthorizon.com' };
      state.custom_fields = [
        { label: 'Total Portfolio Market Value', value: '$4,850,000.00 (4 Properties · 18 Units)' },
        { label: 'Total Senior Mortgage Debt', value: '$2,910,000.00' },
        { label: 'Total Net Portfolio Equity', value: '$1,940,000.00 (60.0% Weighted LTV)' },
        { label: 'Portfolio Weighted DSCR', value: '1.58x Debt Service Coverage Ratio' }
      ];
      state.items = [
        { description: '412 Northshore Dr (6-Unit Multifamily · Value $1.65M · Debt $990K · LTV 60% · DSCR 1.62x) — Annual NOI', quantity: 1.0, unit_price: 118500.00 },
        { description: '808 Georgia Ave (4-Unit Historic Row · Value $1.15M · Debt $690K · LTV 60% · DSCR 1.54x) — Annual NOI', quantity: 1.0, unit_price: 82400.00 },
        { description: '1904 East Main St (6-Unit Mixed-Use · Value $1.40M · Debt $840K · LTV 60% · DSCR 1.61x) — Annual NOI', quantity: 1.0, unit_price: 101200.00 },
        { description: '215 Riverfront Pkwy (Duplex Townhomes · Value $650K · Debt $390K · LTV 60% · DSCR 1.49x) — Annual NOI', quantity: 1.0, unit_price: 46800.00 }
      ];
      state.sections = [
        { title: 'Borrower Representation of Schedule Accuracy', content: 'The undersigned Principal certifies that the Schedule of Real Estate Owned (SREO) accurately reflects current title ownership, unpaid principal mortgage balances, and trailing 12-month Net Operating Income.' }
      ];
    } else if (type === 'schedule_e_tax_worksheet') {
      state.header_subtitle = 'IRS Form 1040 Schedule E (Part I) Rental Real Estate Income & Expense Summary';
      state.from = { name: 'Daniel Hallman', company: 'Hallman Capital Properties LLC', address: '1200 Market St, Suite 400, Chattanooga, TN 37402', email: 'tax@hallmanproperties.com' };
      state.to = { name: 'Robert Vance, CPA', company: 'Vance & Associates CPAs LLP', address: '500 Chestnut St, Suite 900, Chattanooga, TN 37402', email: 'filings@vancecpa.com' };
      state.custom_fields = [
        { label: 'Property 1A Address', value: '412 Northshore Dr, Chattanooga, TN 37405 (Multi-Family)' },
        { label: 'Gross Rents Received (Line 3)', value: '$148,200.00' },
        { label: 'Total Operating Expenses', value: '$54,680.00 (Lines 5–17)' },
        { label: 'MACRS 27.5-Yr Depreciation (Line 18)', value: '$38,900.00' },
        { label: 'Net Schedule E Taxable Income', value: '$54,620.00' }
      ];
      state.items = [
        { description: 'Line 3 · Gross Rents Received (12 Months)', quantity: 1.0, unit_price: 148200.00, discount: 0.0 },
        { description: 'Line 9 · Insurance (Hazard, Liability & Loss of Rent)', quantity: 1.0, unit_price: -6400.00, discount: 0.0 },
        { description: 'Line 11 · Management Fees (8% Gross Collected)', quantity: 1.0, unit_price: -11856.00, discount: 0.0 },
        { description: 'Line 12 · Mortgage Interest Paid (Form 1098)', quantity: 1.0, unit_price: -21400.00, discount: 0.0 },
        { description: 'Line 14 · Repairs & Maintenance', quantity: 1.0, unit_price: -6824.00, discount: 0.0 },
        { description: 'Line 16 · Real Estate Property Taxes', quantity: 1.0, unit_price: -8200.00, discount: 0.0 },
        { description: 'Line 18 · Depreciation Expense (27.5-Yr Straight-Line)', quantity: 1.0, unit_price: -38900.00, discount: 0.0 }
      ];
      state.sections = [
        { title: 'Tax Preparer Reconciliation Notes', content: 'All operating expenses tie directly to general ledger bank statements and Form 1098 mortgage interest statements. Capital improvements placed in service during 2026 are depreciated separately on Form 4562.' }
      ];
      state.signatures = [
        { signer_name: 'Daniel Hallman', signer_title: 'Managing Principal, Hallman Capital Properties LLC', date: '2026-10-01' }
      ];
    } else if (type === 'real_estate_deal_analysis') {
      state.header_subtitle = 'Institutional Investment Underwriting · Cap Rate, Cash-on-Cash, DSCR & BRRRR Pro Forma';
      state.from = { name: 'Acquisitions & Underwriting Team', company: 'Hallman Capital Properties LLC', address: '1200 Market St, Suite 400, Chattanooga, TN 37402', email: 'acquisitions@hallmanproperties.com' };
      state.to = { name: 'Investment Committee', company: 'Northshore Multifamily Partners LP', address: 'Chattanooga, TN 37402', email: 'investors@hallmanproperties.com' };
      state.custom_fields = [
        { label: 'Purchase Price & Rehab Budget', value: '$820,000 Purchase + $115,000 Value-Add Rehab ($935,000 All-In)' },
        { label: 'After-Repair Value (ARV)', value: '$1,225,000.00 (76.3% All-In LTV to ARV)' },
        { label: 'Stabilized Net Operating Income', value: '$94,800.00 / year' },
        { label: 'Going-In Cap Rate / Yield on Cost', value: '10.14% Stabilized Cap Rate on Cost (7.74% Exit Cap)' },
        { label: 'Cash-on-Cash Return (Year 1)', value: '16.8% Net Cash-on-Cash Return' },
        { label: 'Debt Service Coverage Ratio (DSCR)', value: '1.52x DSCR (@ 6.50% 30-Yr Amortizing)' }
      ];
      state.items = [
        { description: 'Stabilized Gross Scheduled Rent (8 Units @ $1,350/mo)', quantity: 12.0, unit_price: 10800.00, discount: 0.0 },
        { description: 'Vacancy & Credit Loss Reserve (5.0%)', quantity: 1.0, unit_price: -6480.00, discount: 0.0 },
        { description: 'Operating Expenses (Taxes, Insurance, Mgmt, CapEx Reserve)', quantity: 1.0, unit_price: -28320.00, discount: 0.0 },
        { description: 'Annual Senior Debt Service ($857,500 Refi @ 6.50%)', quantity: 1.0, unit_price: -62350.00, discount: 0.0 }
      ];
      state.sections = [
        { title: 'Underwriting & Value-Add Execution Thesis', content: 'Interior unit renovations ($14,375/unit across 8 units) bring in-place rents from $1,075/mo to market $1,350/mo within 9 months, supporting a 70% LTV agency refinance at stabilization.' }
      ];
      state.signatures = [
        { signer_name: 'Daniel Hallman', signer_title: 'Managing Principal, Acquisitions', date: '2026-10-01' }
      ];
    } else if (type === 'sublease_agreement') {
      state.header_subtitle = 'Residential Subtenancy Agreement & Master Lease Flow-Through Consent';
      state.from = { name: 'Marcus Sterling', company: '', address: '1420 South Congress Ave, Unit 4B, Austin, TX 78704', email: 'm.sterling@example.com' };
      state.to = { name: 'Noah Takahashi', company: '', address: '1420 South Congress Ave, Unit 4B, Austin, TX 78704', email: 'noah.t@example.com' };
      state.custom_fields = [
        { label: 'Sublease Term', value: 'January 1, 2027 – May 31, 2027 (5 Months)' },
        { label: 'Master Landlord Consent', value: 'Vance Residential Holdings LLC (Written Consent Attached)' },
        { label: 'Sublease Security Deposit', value: '$2,650.00 Held in Escrow' }
      ];
      state.items = [
        { description: 'Monthly Sublease Base Rent', quantity: 1.0, unit_price: 2650.00 },
        { description: 'Refundable Sublease Security Deposit', quantity: 1.0, unit_price: 2650.00 }
      ];
      state.sections = [
        { title: 'Master Lease Incorporation & Flow-Through Covenants', content: 'Sublessee agrees to assume and comply with all tenant obligations under the Master Lease dated November 1, 2025. Sublessor remains secondarily liable to Master Landlord pursuant to the attached Prime Landlord Consent.' }
      ];
      state.signatures = [
        { signer_name: 'Marcus Sterling', signer_title: 'Sublessor (Prime Tenant)', date: '2026-10-01' },
        { signer_name: 'Noah Takahashi', signer_title: 'Sublessee (Subtenant)', date: '2026-10-01' }
      ];
    } else if (type === 'rent_to_own_agreement') {
      state.header_subtitle = 'Residential Lease with Exclusive Irrevocable Option to Purchase Real Estate';
      state.from = { name: 'Elena Rostova, Managing Director', company: 'Vance Residential Holdings LLC', address: '1200 Market St, Suite 400, Chattanooga, TN 37402', email: 'closings@vanceresidential.com' };
      state.to = { name: 'Caleb & Hannah Montgomery', company: '', address: '1842 Lookout Mountain Trl, Chattanooga, TN 37409', email: 'montgomery.family@example.com' };
      state.custom_fields = [
        { label: 'Agreed Option Strike Price', value: '$465,000.00 (Locked for 24 Months)' },
        { label: 'Upfront Non-Refundable Option Fee', value: '$14,000.00 (100% Credited Toward Purchase Price at Closing)' },
        { label: 'Monthly Rent Credit', value: '$350.00 / month Credited to Down Payment ($8,400 Over 24 Mos)' },
        { label: 'Option Expiration Date', value: 'October 31, 2028 at 5:00 PM' }
      ];
      state.items = [
        { description: 'Upfront Option Consideration Fee (Credited at Closing)', quantity: 1.0, unit_price: 14000.00 },
        { description: 'Monthly Base Lease Payment (Includes $350/mo Purchase Credit)', quantity: 1.0, unit_price: 2450.00 }
      ];
      state.sections = [
        { title: 'Exclusive Option to Purchase & Rent Credit Application', content: 'Landlord/Optionor grants Tenant/Optionee the exclusive right and option to purchase the Premises at the Strike Price above. Provided rent is paid on or before the 5th day of each month, $350.00 per month plus 100% of the Upfront Option Fee shall be credited toward the purchase price at closing.' }
      ];
      state.signatures = [
        { signer_name: 'Elena Rostova', signer_title: 'Managing Director, Vance Residential Holdings LLC', date: '2026-10-01' },
        { signer_name: 'Caleb Montgomery', signer_title: 'Tenant / Optionee', date: '2026-10-01' },
        { signer_name: 'Hannah Montgomery', signer_title: 'Co-Tenant / Optionee', date: '2026-10-01' }
      ];
    } else if (type === 'property_management_agreement') {
      state.header_subtitle = 'Exclusive Agency Contract for Leasing, Rent Collection, Escrow & Maintenance';
      state.from = { name: 'Marcus Sterling, Principal Broker', company: 'Vance Property Management LLC', address: '1200 Market St, Suite 400, Chattanooga, TN 37402', email: 'agency@vanceresidential.com', registration_number: 'TN Broker Lic #294810' };
      state.to = { name: 'Arthur Highland, Managing Member', company: 'Highland Crest Holdings LLC', address: '412 Northshore Dr, Chattanooga, TN 37405', email: 'owner@highlandcrest.com' };
      state.custom_fields = [
        { label: 'Managed Portfolio', value: '412 Northshore Dr (6-Unit Multifamily)' },
        { label: 'Monthly Management Fee', value: '8.0% of Gross Collected Rents' },
        { label: 'New Tenant Placement Fee', value: '50% of First Full Month Rent ($1,100 Est.)' },
        { label: 'Repair Authorization Limit', value: '$500.00 per occurrence (Emergency Repairs Exempt)' },
        { label: 'Owner Reserve Balance', value: '$2,500.00 Held in Trust Escrow Account' }
      ];
      state.items = [
        { description: 'Initial Owner Operating Reserve Deposit (Trust Account)', quantity: 1.0, unit_price: 2500.00 },
        { description: 'Estimated Monthly Management Fee (8% on $12,350 Gross Rent)', quantity: 1.0, unit_price: 988.00 }
      ];
      state.sections = [
        { title: 'Exclusive Agency Authority & Trust Accounting', content: 'Owner appoints Broker as exclusive managing agent to advertise, screen tenants, execute leases, collect rents, hold security deposits in a segregated statutory escrow account, and disburse net owner proceeds by the 10th of each month.' }
      ];
      state.signatures = [
        { signer_name: 'Marcus Sterling', signer_title: 'Principal Broker, Vance Property Management LLC', date: '2026-10-01' },
        { signer_name: 'Arthur Highland', signer_title: 'Managing Member, Highland Crest Holdings LLC', date: '2026-10-01' }
      ];
    } else if (type === 'real_estate_purchase_agreement') {
      state.header_subtitle = 'Definitive Contract for the Purchase & Sale of Real Property';
      state.from = { name: 'Arthur Highland, Managing Member', company: 'Highland Crest Holdings LLC', address: '412 Northshore Dr, Chattanooga, TN 37405', email: 'closings@highlandcrest.com' };
      state.to = { name: 'Daniel Hallman, Managing Principal', company: 'Hallman Capital Properties LLC', address: '1200 Market St, Suite 400, Chattanooga, TN 37402', email: 'acquisitions@hallmanproperties.com' };
      state.custom_fields = [
        { label: 'Property Address & APN', value: '412 Northshore Dr, Chattanooga, TN 37405 · APN 135K-A-014.00' },
        { label: 'Contract Purchase Price', value: '$1,650,000.00' },
        { label: 'Earnest Money Deposit', value: '$35,000.00 Held by Pioneer Title & Escrow' },
        { label: 'Due Diligence & Inspection Period', value: '21 Calendar Days from Effective Date' },
        { label: 'Financing Contingency', value: 'Conventional Commercial Loan (65% LTV · 30 Days)' },
        { label: 'Scheduled Closing Date', value: 'December 15, 2026' }
      ];
      state.items = [
        { description: 'Contract Purchase Price', quantity: 1.0, unit_price: 1650000.00, discount: 0.0 },
        { description: 'Earnest Money Deposit Credited at Closing', quantity: 1.0, unit_price: -35000.00, discount: 0.0 }
      ];
      state.sections = [
        { title: 'Title, Prorations & Closing Deliveries', content: 'Seller shall convey marketable fee simple title by General Warranty Deed free and clear of monetary liens. Real estate taxes, collected rents, and tenant security deposits shall be prorated as of 11:59 PM on the day preceding Closing.' }
      ];
      state.signatures = [
        { signer_name: 'Arthur Highland', signer_title: 'Seller · Managing Member, Highland Crest Holdings LLC', date: '2026-10-01' },
        { signer_name: 'Daniel Hallman', signer_title: 'Buyer · Managing Principal, Hallman Capital Properties LLC', date: '2026-10-01' }
      ];
    } else if (type === 'property_deed') {
      state.header_subtitle = 'Recordable Statutory Real Property Conveyance Instrument (3.0in Recorder Stamp Margin)';
      state.recordable = true;
      state.notary_required = true;
      state.county = 'Hamilton';
      state.state_code = state.state_code || 'TN';
      state.from = { name: 'Arthur Highland, Managing Member', company: 'Highland Crest Holdings LLC', address: '412 Northshore Dr, Chattanooga, TN 37405', email: 'legal@highlandcrest.com' };
      state.to = { name: 'Daniel Hallman, Managing Principal', company: 'Hallman Capital Properties LLC', address: '1200 Market St, Suite 400, Chattanooga, TN 37402', email: 'title@hallmanproperties.com' };
      state.custom_fields = [
        { label: 'Prepared By', value: 'Daniel Hallman, Hallman Capital Properties LLC, 1200 Market St, Suite 400, Chattanooga, TN 37402' },
        { label: 'Return To', value: 'Hallman Capital Properties LLC, 1200 Market St, Suite 400, Chattanooga, TN 37402' },
        { label: 'Conveyance Instrument Type', value: 'General Warranty Deed (Statutory Covenants of Title)' },
        { label: 'Assessor Parcel Number (APN)', value: '135K-A-014.00 (Hamilton County, Tennessee)' },
        { label: 'Legal Description', value: 'Lot 14, Block A, Northshore Subdivision, Plat Book 42, Page 118, Register\'s Office of Hamilton County, TN' },
        { label: 'Derivation Clause', value: 'Being the same property conveyed to Grantor by Deed recorded in Book 11420, Page 882' }
      ];
      state.items = [
        { description: 'Actual Consideration Paid for Transfer Tax Valuation', quantity: 1.0, unit_price: 1650000.00 }
      ];
      state.sections = [
        { title: 'Statutory Granting Clause & Covenants of Warranty', content: 'FOR AND IN CONSIDERATION of the sum stated above paid in hand, Grantor does hereby grant, bargain, sell, and convey unto Grantee, its successors and assigns forever, in fee simple, the real property described herein, together with all appurtenances, and Grantor covenants that it is lawfully seized in fee simple and will forever warrant and defend the title against the lawful claims of all persons whomsoever.' }
      ];
      state.signatures = [
        { signer_name: 'Arthur Highland, Managing Member', signer_title: 'Grantor · Highland Crest Holdings LLC', date: '2026-10-01' }
      ];
    } else if (type === 'mechanics_lien_waiver') {
      state.header_subtitle = 'Statutory Contractor & Subcontractor Waiver and Release Upon Progress or Final Payment';
      state.notary_required = true;
      state.from = { name: 'Logan Mercer, Project Executive', company: 'Canyon Peak Construction Co.', address: '4400 Industrial Blvd, Denver, CO 80216', email: 'billing@canyonpeak.com' };
      state.to = { name: 'Daniel Hallman, Managing Principal', company: 'Hallman Capital Properties LLC', address: '1200 Market St, Suite 400, Chattanooga, TN 37402', email: 'draws@hallmanproperties.com' };
      state.custom_fields = [
        { label: 'Waiver Type', value: 'Conditional Waiver and Release Upon Progress Payment (Draw #3)' },
        { label: 'Project Property Address', value: '412 Northshore Dr, Chattanooga, TN 37405' },
        { label: 'Through Date Covered', value: 'October 25, 2026' },
        { label: 'Disputed / Retained Claims', value: '$0.00 (Excludes 10% Statutory Contract Retainage of $11,500.00)' }
      ];
      state.items = [
        { description: 'Progress Draw #3 Payment Subject to Clearance of Funds', quantity: 1.0, unit_price: 38500.00 }
      ];
      state.sections = [
        { title: 'Statutory Waiver & Release Upon Progress Payment', content: 'Upon receipt by the undersigned Claimant of a check or wire transfer in the amount set forth above properly endorsed and paid by the depository institution on which it is drawn, this document shall become effective to release any mechanic\'s lien, stop payment notice, or payment bond right Claimant has on the job of Owner located at the Project Property Address through the Through Date Covered.' }
      ];
      state.signatures = [
        { signer_name: 'Logan Mercer', signer_title: 'Project Executive · Canyon Peak Construction Co.', date: '2026-10-01' }
      ];
    } else if (type.includes('certificate')) {
      state.header_subtitle = 'Conferred for Demonstrated Technical Excellence & Innovation';
      state.from = {
        name: 'Council of Systems Typographers',
        company: 'International Document Typographic Standards Board',
        address: 'World Standards Centre, Zurich, Switzerland',
        email: 'certifications@typographicstandards.org',
        website: 'typographicstandards.org'
      };
      state.to = {
        name: 'Daniel Hallman',
        company: 'Hallman Lab Systems',
        address: 'San Francisco, CA',
        email: 'daniel@example.com'
      };
      state.custom_fields = [
        { label: 'Credential ID', value: 'CST-2026-994812' },
        { label: 'Conferral Date', value: 'October 1, 2026' },
        { label: 'Honors Classification', value: 'Highest Distinction (Summa Cum Laude)' },
        { label: 'Accreditation Status', value: 'Verified & Registered in Global Ledger' }
      ];
      state.items = [];
      state.sections = [
        { title: 'Citation of Achievement', content: 'In recognition of pioneering contributions to vector-grade typesetting architectures, sub-millisecond document compilation engines, and enterprise document design systems.' }
      ];
      state.signatures = [
        { signer_name: 'Prof. Henrik Lindqvist', signer_title: 'Dean of Faculty, International Standards Board', date: '2026-10-01' },
        { signer_name: 'Beatrice Fontaine', signer_title: 'President, Pressmark Engineering Society', date: '2026-10-01' }
      ];
    } else if (type === 'agreement') {
      state.header_subtitle = 'Enterprise Master Services Terms, Deliverables & Intellectual Property Governance';
      state.from = {
        name: 'Alexander Cross',
        company: 'Vanguard Systems Architecture LLC',
        address: '500 Howard Street, Suite 400, San Francisco, CA 94105',
        email: 'legal@vanguardsystems.io',
        phone: '+1 (415) 880-9120',
        tax_id: 'US-94-8829104'
      };
      state.to = {
        name: 'Danielle Brooks',
        company: 'Horizon Logistics Global Inc.',
        address: '1200 Avenue of the Americas, Floor 18, New York, NY 10036',
        email: 'procurement@horizonlogistics.com',
        phone: '+1 (212) 555-0182'
      };
      state.custom_fields = [
        { label: 'Effective Date', value: 'October 1, 2026' },
        { label: 'Initial Term', value: '12 Months (Auto-Renews Annually)' },
        { label: 'Payment Terms', value: 'Net 30 Calendar Days from Invoice' },
        { label: 'Governing Law', value: 'State of Delaware' }
      ];
      state.items = [
        { description: 'Phase 1 · Distributed Platform Architecture & Security Blueprint', quantity: 1.0, unit_price: 18500.00 },
        { description: 'Phase 2 · Core Rust Service Implementation & Load Verification', quantity: 1.0, unit_price: 34000.00 }
      ];
      state.sections = [
        { title: '1. Scope of Services & Acceptance', content: 'Provider shall perform the engineering and advisory services itemized in this Master Service Agreement and any executed Statement of Work. Client shall have ten (10) business days from delivery to review and accept each milestone.' },
        { title: '2. Intellectual Property Assignment', content: 'Upon receipt of full payment for applicable deliverables, Provider assigns to Client all right, title, and interest in custom work product created specifically for Client, excluding Provider pre-existing tooling.' },
        { title: '3. Limitation of Liability', content: 'Neither party\'s aggregate liability under this Agreement shall exceed the total fees paid by Client to Provider during the twelve (12) months preceding the claim.' }
      ];
      state.signatures = [
        { signer_name: 'Alexander Cross', signer_title: 'Managing Principal, Vanguard Systems', date: '2026-10-01' },
        { signer_name: 'Danielle Brooks', signer_title: 'VP of Operations, Horizon Logistics', date: '2026-10-01' }
      ];
    } else if (type === 'agreement_sow') {
      state.header_subtitle = 'Technical Project Milestones, Acceptance Criteria & Fixed-Fee Delivery Schedule';
      state.from = {
        name: 'Alexander Cross',
        company: 'Vanguard Systems Architecture LLC',
        address: '500 Howard Street, Suite 400, San Francisco, CA 94105',
        email: 'delivery@vanguardsystems.io',
        phone: '+1 (415) 880-9120'
      };
      state.to = {
        name: 'Danielle Brooks',
        company: 'Horizon Logistics Global Inc.',
        address: '1200 Avenue of the Americas, Floor 18, New York, NY 10036',
        email: 'engineering@horizonlogistics.com'
      };
      state.custom_fields = [
        { label: 'Parent MSA Reference', value: 'MSA-2026-0881' },
        { label: 'SOW Project Code', value: 'SOW-2026-Q4-VECTOR' },
        { label: 'Target Completion', value: 'December 18, 2026 (10 Weeks)' },
        { label: 'Acceptance Window', value: '5 Business Days per Milestone' }
      ];
      state.items = [
        { description: 'Milestone 1 · Schema & API Contract Specification (Week 2)', quantity: 1.0, unit_price: 9500.00 },
        { description: 'Milestone 2 · High-Throughput Rust Worker & Queue Pipeline (Week 6)', quantity: 1.0, unit_price: 22000.00 },
        { description: 'Milestone 3 · Production Cutover, Observability & Runbook Handover (Week 10)', quantity: 1.0, unit_price: 11500.00 }
      ];
      state.sections = [
        { title: '1. Deliverable Acceptance Criteria', content: 'Each milestone deliverable shall pass automated integration tests, sustain p99 latency under 25ms at 500 RPS, and include complete runbook documentation prior to sign-off.' },
        { title: '2. Change Order Procedure', content: 'Any additions or material modifications to the scope defined above require a written Change Order signed by authorized representatives of both parties.' }
      ];
      state.signatures = [
        { signer_name: 'Alexander Cross', signer_title: 'Principal Solutions Architect', date: '2026-10-01' },
        { signer_name: 'Danielle Brooks', signer_title: 'VP of Engineering & Operations', date: '2026-10-01' }
      ];
    } else if (type === 'contractor_agreement') {
      state.header_subtitle = 'Independent 1099 Engagement, Work-Made-for-Hire IP Assignment & Non-Solicitation';
      state.from = {
        name: 'Marcus Vance',
        company: 'Vance Technical Advisory LLC',
        address: '842 Valencia St, Suite 300, San Francisco, CA 94110',
        email: 'marcus@vanceadvisory.io',
        phone: '+1 (415) 555-0194',
        tax_id: 'EIN 84-3920184'
      };
      state.to = {
        name: 'Dr. Sarah Lin',
        company: 'NeuralCloud Systems Inc.',
        address: '500 Howard Street, Suite 800, San Francisco, CA 94105',
        email: 'legal@neuralcloud.ai'
      };
      state.custom_fields = [
        { label: 'Engagement Classification', value: 'Independent Contractor (IRS Form 1099-NEC)' },
        { label: 'Contract Term', value: 'October 1, 2026 – March 31, 2027' },
        { label: 'Authorized Rate & Cap', value: '$210.00 / hr (Up to 30 hrs/week)' },
        { label: 'Invoicing Cadence', value: 'Bi-Weekly on Net 15 Terms' }
      ];
      state.items = [
        { description: 'Senior Distributed Systems & Rust Engineering Services (Monthly Cap Est.)', quantity: 120.0, unit_price: 210.00 }
      ];
      state.sections = [
        { title: '1. Independent Contractor Status', content: 'Contractor performs all services as an independent business entity and not as an employee, partner, or agent of Company. Contractor is solely responsible for all federal, state, and local self-employment taxes.' },
        { title: '2. Invention & Work-Made-for-Hire Assignment', content: 'All code, designs, documentation, and inventions conceived or reduced to practice by Contractor in connection with the Services shall be the sole and exclusive property of Company.' }
      ];
      state.signatures = [
        { signer_name: 'Marcus Vance', signer_title: 'Managing Member, Vance Technical Advisory LLC', date: '2026-10-01' },
        { signer_name: 'Dr. Sarah Lin', signer_title: 'Chief Executive Officer, NeuralCloud Systems Inc.', date: '2026-10-01' }
      ];
    } else if (type === 'retainer_agreement') {
      state.header_subtitle = 'Dedicated Monthly Advisory Capacity, SLA Response Guarantees & Priority Access';
      state.from = {
        name: 'Alexander Cross',
        company: 'Vanguard Systems Architecture LLC',
        address: '500 Howard Street, Suite 400, San Francisco, CA 94105',
        email: 'retainers@vanguardsystems.io',
        phone: '+1 (415) 880-9120'
      };
      state.to = {
        name: 'Danielle Brooks',
        company: 'Horizon Logistics Global Inc.',
        address: '1200 Avenue of the Americas, Floor 18, New York, NY 10036',
        email: 'accounts.payable@horizonlogistics.com'
      };
      state.custom_fields = [
        { label: 'Retainer Tier', value: 'Enterprise Principal Advisory (25 Dedicated Hours / Month)' },
        { label: 'Response Time SLA', value: 'Under 2 Hours (24/7 P1 Incident Escalation)' },
        { label: 'Monthly Billing Date', value: '1st Business Day of Each Calendar Month' },
        { label: 'Overage Rate', value: '$225.00 / hr (Pre-Approved in Writing)' }
      ];
      state.items = [
        { description: 'Monthly Executive Architecture & Reliability Retainer (25 Hours Included)', quantity: 1.0, unit_price: 5500.00 }
      ];
      state.sections = [
        { title: '1. Reserved Capacity & Priority SLA', content: 'Advisor reserves twenty-five (25) hours per calendar month exclusively for Client architecture reviews, production incident response, and security audits with a two-hour SLA.' },
        { title: '2. Rollover & Renewal Terms', content: 'Up to five (5) unused retainer hours may roll over to the immediately following calendar month. Either party may terminate renewal upon thirty (30) days prior written notice.' }
      ];
      state.signatures = [
        { signer_name: 'Alexander Cross', signer_title: 'Managing Principal, Vanguard Systems', date: '2026-10-01' },
        { signer_name: 'Danielle Brooks', signer_title: 'VP of Operations, Horizon Logistics', date: '2026-10-01' }
      ];
    } else if (type === 'nda') {
      state.header_subtitle = 'Mutual Confidentiality & Proprietary Rights Preservation';
      state.from = {
        name: 'Robert Vance',
        company: 'Vance Biosystems Inc.',
        address: '100 Binney St, Cambridge, MA 02142',
        email: 'legal@vancebio.com',
        phone: '+1 (617) 555-0199',
        registration_number: 'DE-CORP-948102'
      };
      state.to = {
        name: 'Dr. Evelyn Reed',
        company: 'Apex Therapeutics LLC',
        address: '400 Technology Square, Cambridge, MA 02139',
        email: 'evelyn.reed@apexrx.com',
        phone: '+1 (617) 555-4421'
      };
      state.custom_fields = [
        { label: 'Confidentiality Period', value: '3 Years from Disclosure' },
        { label: 'Governing Jurisdiction', value: 'Commonwealth of Massachusetts' },
        { label: 'Authorized Purpose', value: 'Biotechnology Licensing & Strategic Evaluation' }
      ];
      state.items = [];
      state.sections = [
        { title: '1. Confidential Information Defined', content: '"Confidential Information" encompasses all proprietary data, formulas, trade secrets, software code, customer lists, and financial forecasts disclosed in oral, visual, or tangible writing.' },
        { title: '2. Standard of Care', content: 'Receiving Party agrees to exercise at least the same degree of care it uses for its own confidential assets of like importance, but not less than reasonable care.' },
        { title: '3. Exclusions from Confidentiality', content: 'Obligations shall not apply to information that is publicly known prior to disclosure, independently developed without reference to discloser\'s assets, or required by subpoena.' }
      ];
      state.signatures = [
        { signer_name: 'Robert Vance', signer_title: 'Chief Executive Officer, Vance Biosystems', date: '2026-10-01' },
        { signer_name: 'Dr. Evelyn Reed', signer_title: 'Chief Scientific Officer, Apex Therapeutics', date: '2026-10-01' }
      ];
    } else if (type === 'commercial_lease') {
      state.header_subtitle = 'Commercial Property Lease & Facility Agreement';
      state.from = {
        name: 'Arthur Sterling',
        company: 'Apex Commercial Properties REIT',
        address: '555 California St, Suite 4200, San Francisco, CA 94104',
        email: 'leasing@apexcommercial.com',
        phone: '+1 (415) 980-2200',
        tax_id: 'US-94-1182903'
      };
      state.to = {
        name: 'Dr. Sarah Lin',
        company: 'NeuralCloud Systems Inc.',
        address: '500 Howard Street, Suite 800, San Francisco, CA 94105',
        email: 'legal@neuralcloud.ai',
        phone: '+1 (415) 555-8821'
      };
      state.custom_fields = [
        { label: 'Premises Address', value: '500 Howard Street, Suite 800, San Francisco, CA 94105' },
        { label: 'Rentable Square Feet', value: '14,500 RSF' },
        { label: 'Lease Term', value: '36 Months (Nov 1, 2026 – Oct 31, 2029)' },
        { label: 'Security Deposit', value: '$65,000.00 (Letter of Credit)' }
      ];
      state.items = [
        { description: 'Monthly Base Rent ($55.00 / RSF / Year)', quantity: 1.0, unit_price: 66458.33 },
        { description: 'Common Area Operating Expense Contribution', quantity: 1.0, unit_price: 12500.00 },
        { description: 'Reserved Executive Garage Parking (6 stalls)', quantity: 6.0, unit_price: 350.00 }
      ];
      state.sections = [
        { title: '1. Commercial Rent & Operating Costs', content: 'Base rent is payable in advance on the first day of each calendar month. Tenant agrees to pay its proportionate share of building operating expenses.' },
        { title: '2. Permitted Use', content: 'The premises shall be used strictly for executive software development, AI research, and general corporate administrative offices.' }
      ];
      state.signatures = [
        { signer_name: 'Arthur Sterling', signer_title: 'Senior Managing Director, Apex Capital', date: '2026-10-01' },
        { signer_name: 'Dr. Sarah Lin', signer_title: 'Chief Executive Officer, NeuralCloud Systems', date: '2026-10-01' }
      ];
    } else if (type === 'proforma_invoice') {
      state.header_subtitle = 'Commercial Valuation & International Customs Proforma';
      state.from = {
        name: 'Hans Becker',
        company: 'Precision Optics Engineering GmbH',
        address: 'Industriestrasse 42, 70565 Stuttgart, Germany',
        email: 'export@precisionoptics.de',
        phone: '+49 711 490281',
        tax_id: 'DE-814902819'
      };
      state.to = {
        name: 'Elena Rostova',
        company: 'Advanced Robotics Technologies LLC',
        address: '100 Technology Parkway, Suite 300, Boston, MA 02142',
        email: 'procurement@advancedrobotics.com',
        phone: '+1 (617) 555-0144'
      };
      state.custom_fields = [
        { label: 'Incoterms 2020', value: 'DAP - Delivered at Place (Boston Harbor)' },
        { label: 'Country of Origin', value: 'Germany (Federal Republic)' },
        { label: 'Harmonized Tariff (HS)', value: '9001.90.0000 (Optical Elements)' },
        { label: 'Port of Loading', value: 'Frankfurt Air Cargo Terminal (FRA)' }
      ];
      state.items = [
        { description: 'Precision Laser Collimator Assemblies (Grade A)', quantity: 12.0, unit_price: 1850.00 },
        { description: 'Interferometric Beam Splitter Prisms', quantity: 24.0, unit_price: 620.00 },
        { description: 'Export Crated Packaging & Shock Absorption', quantity: 1.0, unit_price: 950.00 }
      ];
      state.sections = [
        { title: 'Customs & Valuation Attestation', content: 'We certify that this proforma invoice indicates the true value of the goods contained, and that all particulars are true and correct for customs clearance purposes.' }
      ];
      state.signatures = [
        { signer_name: 'Hans Becker', signer_title: 'Export Control Director', date: '2026-10-01' }
      ];
    } else if (type === 'quotation_tender') {
      state.header_subtitle = 'Institutional Tender Response & Bid Proposal';
      state.from = {
        name: 'Jonathan Reynolds',
        company: 'Apex Infrastructure Group Inc.',
        address: '1200 Pennsylvania Ave NW, Washington, DC 20004',
        email: 'tenders@apexinfrastructure.com',
        phone: '+1 (202) 555-0188',
        tax_id: 'US-52-9918204'
      };
      state.to = {
        name: 'Procurement Review Board',
        company: 'Department of Public Works & Transportation',
        address: '400 7th Street SW, Washington, DC 20024',
        email: 'bids@dpw.gov'
      };
      state.custom_fields = [
        { label: 'Solicitation RFP #', value: 'RFP-DOT-2026-0441' },
        { label: 'Tender Validity', value: '90 Days from Submission' },
        { label: 'Bonding Capacity', value: '$10,000,000.00 (Surety Travelers)' },
        { label: 'Contractor License', value: 'General Engineering Class A #884920' }
      ];
      state.items = [
        { description: 'Bridge Structural Sensor Telemetry & Installation', quantity: 1.0, unit_price: 145000.00 },
        { description: 'Autonomous IoT Gateway Hardened Hubs (Qty 18)', quantity: 18.0, unit_price: 4200.00 },
        { description: 'Real-Time Stress Analysis Cloud Dashboard & Warranty', quantity: 1.0, unit_price: 48000.00 }
      ];
      state.sections = [
        { title: 'Tender Terms & Firm Price Guarantee', content: 'This formal bid constitutes a firm offer to perform all required scope items detailed in the RFP at the prices specified herein, valid for 90 calendar days.' }
      ];
      state.signatures = [
        { signer_name: 'Jonathan Reynolds', signer_title: 'Executive Vice President, Apex Infrastructure', date: '2026-10-01' }
      ];
    } else if (type === 'work_order') {
      state.header_subtitle = 'Field Operations & Service Dispatch Authorization';
      state.from = {
        name: 'Dispatch Coordinator',
        company: 'Metro Mechanical Systems LLC',
        address: '420 Commercial Ave, Suite 10, Denver, CO 80202',
        email: 'dispatch@metromechanical.com',
        phone: '+1 (303) 555-0192'
      };
      state.to = {
        name: 'Highland Ridge Commercial Center',
        company: 'Highland Ridge Properties LLC',
        address: '8800 E Arapahoe Rd, Centennial, CO 80112',
        phone: '+1 (303) 555-0144'
      };
      state.custom_fields = [
        { label: 'Work Order #', value: 'WO-2026-8819' },
        { label: 'Priority Level', value: 'Urgent / Priority 1' },
        { label: 'Lead Technician', value: 'Tyler Vance (EPA Universal #77491)' },
        { label: 'Equipment Tag', value: 'Trane Roof Top Unit #4 (RTU-04)' }
      ];
      state.items = [
        { description: 'Diagnostic Troubleshooting & Refrigerant Leak Check', quantity: 2.5, unit_price: 165.00 },
        { description: 'Variable Speed ECM Blower Motor Replacement', quantity: 1.0, unit_price: 1250.00 },
        { description: 'R-410A Refrigerant Recovery and Re-charge (12 lbs)', quantity: 12.0, unit_price: 45.00 }
      ];
      state.sections = [
        { title: 'Work Completion Sign-Off', content: 'Work performed in accordance with manufacturer technical specifications and local mechanical codes. All old components properly recycled.' }
      ];
      state.signatures = [
        { signer_name: 'Tyler Vance', signer_title: 'Lead HVAC Certified Tech', date: '2026-10-01' },
        { signer_name: 'Facility Manager Sign-off', signer_title: 'Site Representative', date: '2026-10-01' }
      ];
    } else if (type === 'expense_report') {
      state.header_subtitle = 'Employee Business Expense Claim & Reconciliation';
      state.from = {
        name: 'Claire Sterling',
        company: 'Global Enterprises Inc.',
        address: '100 Montgomery St, San Francisco, CA 94104',
        email: 'claire.sterling@example.com',
        phone: '+1 (415) 555-0177'
      };
      state.to = {
        name: 'Finance & Accounts Payable',
        company: 'Global Enterprises Inc.',
        address: '100 Montgomery St, Floor 14, San Francisco, CA 94104',
        email: 'ap@globalenterprises.com'
      };
      state.custom_fields = [
        { label: 'Employee ID', value: 'EMP-90412' },
        { label: 'Department / Cost Center', value: 'Product Engineering (CC-420)' },
        { label: 'Purpose of Travel', value: 'Client Onsite Deployment & Architecture Review' },
        { label: 'Report Period', value: 'Sep 25, 2026 – Sep 28, 2026' }
      ];
      state.items = [
        { description: 'Flight SFO -> JFK (Delta Airlines Roundtrip)', quantity: 1.0, unit_price: 642.50 },
        { description: 'Lodging: The Beekman Hotel NYC (3 Nights)', quantity: 3.0, unit_price: 345.00 },
        { description: 'Client Dinner & Architecture Working Session', quantity: 1.0, unit_price: 248.80 },
        { description: 'Ground Transportation (JFK Airport Express / Rides)', quantity: 1.0, unit_price: 112.40 }
      ];
      state.sections = [
        { title: 'Employee Attestation', content: 'I certify that all expenses reported were incurred in the necessary conduct of company business, comply with the corporate travel policy, and are supported by itemized receipts.' }
      ];
      state.signatures = [
        { signer_name: 'Claire Sterling', signer_title: 'Lead Software Architect', date: '2026-10-01' },
        { signer_name: 'VP of Engineering', signer_title: 'Approving Officer', date: '2026-10-01' }
      ];
    } else if (type === 'delivery_receipt') {
      state.header_subtitle = 'Consignee Acknowledgment & Proof of Freight Delivery';
      state.from = {
        name: 'Freight Dispatch Operations',
        company: 'Apex Logistics Freightways LLC',
        address: '700 Harbor Blvd, Suite 200, Newark, NJ 07114',
        email: 'dispatch@apexlogistics.com',
        phone: '+1 (973) 555-0199'
      };
      state.to = {
        name: 'Receiving Dock #4',
        company: 'Precision Instruments Manufacturing',
        address: '1200 Industrial Way, Philadelphia, PA 19104',
        email: 'dock@precisioninst.com',
        phone: '+1 (215) 555-0182'
      };
      state.custom_fields = [
        { label: 'Bill of Lading (BOL)', value: 'BOL-APX-8849102' },
        { label: 'Carrier Tractor / Trailer #', value: 'Unit #402 / Trailer #5322' },
        { label: 'Driver Name & CDL', value: 'Marcus Brody (CDL-NJ #94821)' },
        { label: 'Delivery Timestamp', value: 'October 1, 2026 14:35 EST' }
      ];
      state.items = [
        { description: 'Shrink-wrapped Pallets - Electronic Components', quantity: 4.0, unit_price: 0.00 },
        { description: 'Crated Calibration Sensors (Heavy / Fragile)', quantity: 2.0, unit_price: 0.00 },
        { description: 'Master Documentation Enclosure Pouch', quantity: 1.0, unit_price: 0.00 }
      ];
      state.sections = [
        { title: 'Condition Upon Receipt', content: 'Consignee acknowledges receipt of the freight listed above in apparent good condition except as noted. All carton seals intact.' }
      ];
      state.signatures = [
        { signer_name: 'Marcus Brody', signer_title: 'Delivery Driver, Apex Logistics', date: '2026-10-01' },
        { signer_name: 'Receiving Dock Signatory', signer_title: 'Warehouse Receiving Clerk', date: '2026-10-01' }
      ];
    } else if (type === 'packing_slip') {
      state.header_subtitle = 'Order Fulfillment Manifest & Package Contents';
      state.from = {
        name: 'Fulfillment Operations',
        company: 'Vanguard Hardware Systems Inc.',
        address: '500 Howard Street, San Francisco, CA 94105',
        email: 'shipping@vanguardhardware.io',
        phone: '+1 (415) 880-9120'
      };
      state.to = {
        name: 'Receiving Department',
        company: 'Horizon Data Centers LLC',
        address: '400 Enterprise Way, Reston, VA 20190',
        email: 'datacenter@horizondc.com'
      };
      state.custom_fields = [
        { label: 'Order Number', value: 'ORD-2026-9921' },
        { label: 'Shipping Method', value: 'FedEx Priority Overnight Air' },
        { label: 'Tracking Number', value: '7829-4412-8819' },
        { label: 'Total Packages', value: '3 Master Cartons' }
      ];
      state.items = [
        { description: '1U Rackmount Vector Processing Nodes', quantity: 4.0, unit_price: 0.00 },
        { description: '100GbE QSFP28 Direct Attach Copper Cables (2m)', quantity: 8.0, unit_price: 0.00 },
        { description: 'Redundant Titanium 800W Hot-Swap Power Supplies', quantity: 8.0, unit_price: 0.00 }
      ];
      state.sections = [
        { title: 'Inspection Instructions', content: 'Please inspect cartons immediately upon delivery. Verify serial numbers against packing manifest before deploying hardware into server racks.' }
      ];
      state.signatures = [
        { signer_name: 'Warehouse Inspector #42', signer_title: 'Quality Assurance Clerk', date: '2026-10-01' }
      ];
    } else {
      // Default Commercial Invoice / Estimate / Statement Preset
      if (type.startsWith('invoice')) {
        state.tax_rate = 8.25;
        state.discount_amount = 250.0;
        state.notes = 'Thank you for your business. Payment is due within 30 calendar days of invoice date.';
        state.terms = 'Late payments are subject to a 1.5% monthly administrative finance charge. All deliverables remain property of issuer until settled in full.';
      }
      state.header_subtitle = 'Enterprise Systems & Precision Infrastructure Retainer';
      state.from = {
        name: 'Alexander Cross',
        company: 'Vanguard Systems Architecture LLC',
        address: '500 Howard Street, Suite 400, San Francisco, CA 94105',
        email: 'billing@vanguardsystems.io',
        phone: '+1 (415) 880-9120',
        tax_id: 'US-94-8829104',
        website: 'vanguardsystems.io',
        registration_number: 'CA-CORP-488219'
      };
      state.to = {
        name: 'Danielle Brooks',
        company: 'Horizon Logistics Global Inc.',
        address: '1200 Avenue of the Americas, Floor 18, New York, NY 10036',
        email: 'accounts.payable@horizonlogistics.com',
        phone: '+1 (212) 555-0182',
        tax_id: 'US-13-4991823'
      };
      state.custom_fields = [
        { label: 'Master Agreement', value: 'MSA-2026-0881' },
        { label: 'Purchase Order', value: 'PO-HORIZON-4991' },
        { label: 'Billing Period', value: 'September 1, 2026 – September 30, 2026' },
        { label: 'Payment Terms', value: 'Net 30 Days' }
      ];
      state.items = [
        { description: 'Cloud Infrastructure Modernization & High-Throughput Microservices', quantity: 40.0, unit_price: 225.00 },
        { description: 'Pressmark Vector Engine PDF Pipeline Architecture & Tuning', quantity: 24.0, unit_price: 250.00 },
        { description: 'Distributed PostgreSQL Database Clustered Re-indexing', quantity: 16.0, unit_price: 195.00 },
        { description: 'Enterprise SOC-2 Security Compliance Remediation Sprint', quantity: 1.0, unit_price: 4500.00 }
      ];
      state.sections = [
        { title: 'Payment & Remittance Instructions', content: 'Remittance via ACH/Wire Transfer to JPMorgan Chase Bank NA, Routing #021000021, Account #8849201882. Please quote invoice number on all remittances.' }
      ];
      state.signatures = [
        { signer_name: 'Alexander Cross', signer_title: 'Managing Principal Architect', date: '2026-10-01' }
      ];
    }
    if (!Array.isArray(state.signatures) || state.signatures.length === 0) {
      state.signatures = [
        {
          signer_name: (state.from && state.from.name) ? state.from.name : 'Authorized Signatory',
          signer_title: (state.from && state.from.company) ? state.from.company : 'Authorized Representative',
          date: state.issue_date || '2026-10-01'
        }
      ];
    }
  }

  // Entity Profile & Directory Management
  function saveIssuerProfile() {
    if (!state.from || (!state.from.name && !state.from.company)) {
      showUndoToast('⚠️ Please enter a Business Name or Contact Name first');
      return;
    }
    try {
      localStorage.setItem(STORAGE_KEY_ISSUER, JSON.stringify(state.from));
      showUndoToast(`💾 Saved "${state.from.company || state.from.name}" as Default Issuer Profile!`);
    } catch (e) {
      console.error('Failed to save issuer profile:', e);
    }
  }

  function loadIssuerProfile() {
    try {
      const raw = localStorage.getItem(STORAGE_KEY_ISSUER);
      if (!raw) return false;
      const profile = JSON.parse(raw);
      if (profile && typeof profile === 'object') {
        state.from = Object.assign({}, state.from, profile);
        syncFormFromState();
        return true;
      }
    } catch (e) {
      console.error('Failed to load issuer profile:', e);
    }
    return false;
  }

  function resetIssuerProfile() {
    state.from = {
      name: '',
      company: '',
      address: '',
      email: '',
      phone: '',
      tax_id: '',
      website: '',
      registration_number: ''
    };
    try {
      localStorage.removeItem(STORAGE_KEY_ISSUER);
    } catch (e) {}
    syncFormFromState();
    schedulePreview();
    renderInteractiveOverlay();
    showUndoToast('🔄 Issuer profile cleared');
  }

  function saveCounterparty() {
    if (!state.to || (!state.to.name && !state.to.company)) {
      showUndoToast('⚠️ Please enter a Client Name or Company first');
      return;
    }
    try {
      let list = [];
      const raw = localStorage.getItem(STORAGE_KEY_COUNTERPARTIES);
      if (raw) {
        list = JSON.parse(raw) || [];
      }
      const targetLabel = (state.to.company || state.to.name).trim().toLowerCase();
      list = list.filter(item => {
        const itemLabel = (item.company || item.name || '').trim().toLowerCase();
        return itemLabel !== targetLabel;
      });
      list.unshift(Object.assign({}, state.to));
      if (list.length > 50) list = list.slice(0, 50);
      localStorage.setItem(STORAGE_KEY_COUNTERPARTIES, JSON.stringify(list));
      populateCounterpartyDropdown();
      showUndoToast(`⭐ Saved "${state.to.company || state.to.name}" to Counterparty Directory!`);
    } catch (e) {
      console.error('Failed to save counterparty:', e);
    }
  }

  function populateCounterpartyDropdown() {
    const select = document.getElementById('counterpartyQuickSelect');
    if (!select) return;
    try {
      const raw = localStorage.getItem(STORAGE_KEY_COUNTERPARTIES);
      const list = raw ? JSON.parse(raw) : [];
      select.innerHTML = '<option value="">📁 Saved Clients / Counterparties...</option>';
      list.forEach((cp, idx) => {
        const opt = document.createElement('option');
        opt.value = idx;
        const title = cp.company || cp.name || 'Unnamed Contact';
        const detail = cp.email || cp.phone || cp.name || '';
        opt.textContent = detail && detail !== title ? `${title} (${detail})` : title;
        select.appendChild(opt);
      });
    } catch (e) {
      console.error('Failed to populate counterparties:', e);
    }
  }

  function clearCounterparty() {
    state.to = {
      name: '',
      company: '',
      address: '',
      email: '',
      phone: '',
      tax_id: '',
      website: '',
      registration_number: ''
    };
    const select = document.getElementById('counterpartyQuickSelect');
    if (select) select.value = '';
    syncFormFromState();
    schedulePreview();
    renderInteractiveOverlay();
    showUndoToast('🧹 Cleared recipient / client inputs');
  }

  function autoSaveUserDocument() {
    if (dataMode !== 'my_data') return;
    try {
      localStorage.setItem(STORAGE_KEY_USER_DOC, JSON.stringify(state));
    } catch (e) {}
  }

  function loadSavedUserDocument() {
    try {
      const raw = localStorage.getItem(STORAGE_KEY_USER_DOC);
      if (!raw) return false;
      const doc = JSON.parse(raw);
      if (doc && typeof doc === 'object' && doc.title) {
        Object.assign(state, doc);
        return true;
      }
    } catch (e) {}
    return false;
  }

  function bindDataModeControls() {
    const btnMyData = document.getElementById('btnModeMyData');
    const btnSample = document.getElementById('btnModeSampleData');
    if (!btnMyData || !btnSample) return;

    btnMyData.addEventListener('click', () => {
      if (dataMode === 'my_data') return;
      dataMode = 'my_data';
      btnMyData.classList.add('active');
      btnSample.classList.remove('active');

      if (cachedUserDoc) {
        Object.assign(state, cachedUserDoc);
      } else if (loadSavedUserDocument()) {
        // Loaded successfully from storage
      } else {
        loadIssuerProfile();
      }

      adaptFormToArchetype(state.doc_type);
      syncFormFromState();
      renderLineItemsTable();
      renderSectionsList();
      renderSignaturesList();
      renderCustomFieldsList();
      updateCalculations();
      renderInteractiveOverlay();
      schedulePreview();
      showUndoToast('🛡️ My Data mode active: Your inputs are preserved across all archetypes');
    });

    btnSample.addEventListener('click', () => {
      if (dataMode === 'sample_data') return;
      cachedUserDoc = JSON.parse(JSON.stringify(state));
      autoSaveUserDocument();

      dataMode = 'sample_data';
      btnSample.classList.add('active');
      btnMyData.classList.remove('active');

      loadSampleDataForArchetype(state.doc_type);

      adaptFormToArchetype(state.doc_type);
      syncFormFromState();
      renderLineItemsTable();
      renderSectionsList();
      renderSignaturesList();
      renderCustomFieldsList();
      updateCalculations();
      renderInteractiveOverlay();
      schedulePreview();
      showUndoToast('🎲 Sample Data mode active: Freely explore curated industry presets');
    });
  }

  function bindPartyProfileActions() {
    const btnSaveIssuer = document.getElementById('btnSaveIssuerProfile');
    if (btnSaveIssuer) {
      btnSaveIssuer.addEventListener('click', saveIssuerProfile);
    }
    const btnResetIssuer = document.getElementById('btnResetIssuerProfile');
    if (btnResetIssuer) {
      btnResetIssuer.addEventListener('click', resetIssuerProfile);
    }
    const btnSaveCp = document.getElementById('btnSaveCounterparty');
    if (btnSaveCp) {
      btnSaveCp.addEventListener('click', saveCounterparty);
    }
    const btnClearCp = document.getElementById('btnClearCounterparty');
    if (btnClearCp) {
      btnClearCp.addEventListener('click', clearCounterparty);
    }
    const cpSelect = document.getElementById('counterpartyQuickSelect');
    if (cpSelect) {
      cpSelect.addEventListener('change', (e) => {
        const idx = e.target.value;
        if (idx === '') return;
        try {
          const raw = localStorage.getItem(STORAGE_KEY_COUNTERPARTIES);
          if (!raw) return;
          const list = JSON.parse(raw);
          const contact = list[parseInt(idx, 10)];
          if (contact) {
            state.to = Object.assign({}, contact);
            syncFormFromState();
            schedulePreview();
            renderInteractiveOverlay();
            showUndoToast(`👤 Loaded contact: ${contact.company || contact.name}`);
          }
        } catch (err) {
          console.error('Error applying counterparty:', err);
        }
      });
    }
  }

  // Theme & Multi-Color Palette Controller
  function applyThemePalette(themeKey, pushToUndo = true) {
    if (pushToUndo) pushUndo(`Change Theme to ${themeKey}`);
    state.theme = themeKey;

    if (themeKey === 'custom') {
      const customPanel = document.getElementById('customPalettePanel');
      if (customPanel) customPanel.style.display = 'block';
    } else {
      const customPanel = document.getElementById('customPalettePanel');
      if (customPanel) customPanel.style.display = 'none';

      const pal = THEME_PALETTES[themeKey];
      if (pal) {
        state.primary_color = pal.primary;
        state.secondary_color = pal.secondary;
        state.accent_color = pal.accent;
        state.surface_color = pal.surface;
      }
    }

    syncPaletteControlsFromState();
    schedulePreview();
    renderInteractiveOverlay();
    autoSaveUserDocument();
  }

  function syncPaletteControlsFromState() {
    if (state.theme === 'slate') state.theme = 'modern_slate';
    const pal = THEME_PALETTES[state.theme] || {
      primary: state.primary_color || '#4f46e5',
      secondary: state.secondary_color || '#475569',
      accent: state.accent_color || '#06b6d4',
      surface: state.surface_color || '#f8fafc',
    };

    if (state.theme !== 'custom' && THEME_PALETTES[state.theme]) {
      state.primary_color = pal.primary;
      state.secondary_color = pal.secondary;
      state.accent_color = pal.accent;
      state.surface_color = pal.surface;
    }

    const prim = state.primary_color || pal.primary;
    const sec = state.secondary_color || pal.secondary;
    const acc = state.accent_color || pal.accent;
    const surf = state.surface_color || pal.surface;

    // 1. Sync Dropdown
    const select = document.getElementById('themePresetSelect');
    if (select) select.value = state.theme || 'modern_slate';

    // 2. Sync Live Swatch Preview Bar
    const setSwatch = (colorId, hexId, val) => {
      const cEl = document.getElementById(colorId);
      const hEl = document.getElementById(hexId);
      if (cEl) cEl.style.backgroundColor = val;
      if (hEl) hEl.textContent = val;
    };
    setSwatch('swatchPrimary', 'swatchHexPrimary', prim);
    setSwatch('swatchSecondary', 'swatchHexSecondary', sec);
    setSwatch('swatchAccent', 'swatchHexAccent', acc);
    setSwatch('swatchSurface', 'swatchHexSurface', surf);

    // 3. Sync Active Chip
    document.querySelectorAll('.theme-chip').forEach(c => {
      c.classList.toggle('active', c.dataset.theme === state.theme);
    });

    // 4. Sync Custom Palette Inputs
    const setInput = (pickerId, textId, val) => {
      const picker = document.getElementById(pickerId);
      const text = document.getElementById(textId);
      if (picker && val) picker.value = val;
      if (text && val) text.value = val;
    };
    setInput('customPrimaryColor', 'primaryColorInput', prim);
    setInput('customSecondaryColor', 'secondaryColorInput', sec);
    setInput('customAccentColor', 'accentColorInput', acc);
    setInput('customSurfaceColor', 'surfaceColorInput', surf);

    // 5. Toggle Custom Panel
    const customPanel = document.getElementById('customPalettePanel');
    if (customPanel) {
      customPanel.style.display = (state.theme === 'custom') ? 'block' : 'none';
    }
  }

  function bindThemeSelectors() {
    // Dropdown change
    const themeSelect = document.getElementById('themePresetSelect');
    if (themeSelect) {
      themeSelect.addEventListener('change', (e) => {
        applyThemePalette(e.target.value);
      });
    }

    // Theme Chips click
    const chips = document.querySelectorAll('.theme-chip');
    chips.forEach(chip => {
      chip.addEventListener('click', () => {
        applyThemePalette(chip.dataset.theme);
      });
    });

    // Custom Color Inputs Helper
    const bindColorPair = (pickerId, textId, stateKey, swatchColorId, swatchHexId) => {
      const picker = document.getElementById(pickerId);
      const text = document.getElementById(textId);

      const updateColor = (val) => {
        if (!val) return;
        state[stateKey] = val;
        state.theme = 'custom';
        const themeSelect = document.getElementById('themePresetSelect');
        if (themeSelect) themeSelect.value = 'custom';

        document.querySelectorAll('.theme-chip').forEach(c => {
          c.classList.toggle('active', c.dataset.theme === 'custom');
        });

        const customPanel = document.getElementById('customPalettePanel');
        if (customPanel) customPanel.style.display = 'block';

        const swatchColor = document.getElementById(swatchColorId);
        const swatchHex = document.getElementById(swatchHexId);
        if (swatchColor) swatchColor.style.backgroundColor = val;
        if (swatchHex) swatchHex.textContent = val;

        schedulePreview();
        autoSaveUserDocument();
      };

      if (picker) {
        picker.addEventListener('input', (e) => {
          const val = e.target.value;
          if (text) text.value = val;
          updateColor(val);
        });
      }

      if (text) {
        text.addEventListener('input', (e) => {
          let val = e.target.value.trim();
          if (!val.startsWith('#') && val.length === 6) val = '#' + val;
          if (/^#[0-9A-Fa-f]{6}$/.test(val)) {
            if (picker) picker.value = val;
            updateColor(val);
          }
        });
        text.addEventListener('change', (e) => {
          pushUndo(`Change ${stateKey} to ${e.target.value}`);
        });
      }
    };

    bindColorPair('customPrimaryColor', 'primaryColorInput', 'primary_color', 'swatchPrimary', 'swatchHexPrimary');
    bindColorPair('customSecondaryColor', 'secondaryColorInput', 'secondary_color', 'swatchSecondary', 'swatchHexSecondary');
    bindColorPair('customAccentColor', 'accentColorInput', 'accent_color', 'swatchAccent', 'swatchHexAccent');
    bindColorPair('customSurfaceColor', 'surfaceColorInput', 'surface_color', 'swatchSurface', 'swatchHexSurface');

    // Initial sync
    syncPaletteControlsFromState();
  }

  // Background Selector Chips
  function bindBackgroundSelectors() {
    const chips = document.querySelectorAll('.bg-chip');
    chips.forEach(chip => {
      chip.addEventListener('click', () => {
        pushUndo(`Change Background to ${chip.dataset.bg}`);
        chips.forEach(c => c.classList.remove('active'));
        chip.classList.add('active');
        state.background_style = chip.dataset.bg;
        schedulePreview();
        renderInteractiveOverlay();
      });
    });
  }

  // In-Studio AI Command Bar
  function bindCopilotBar() {
    const copilotInput = document.getElementById('copilotInput');
    const copilotBtn = document.getElementById('btnCopilotSubmit');

    const executePrompt = () => {
      const text = copilotInput.value.trim();
      if (!text) return;

      copilotInput.classList.add('loading');
      parseNaturalLanguagePrompt(text);
      copilotInput.value = '';
      copilotInput.classList.remove('loading');
    };

    copilotBtn.addEventListener('click', executePrompt);
    copilotInput.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') {
        e.preventDefault();
        executePrompt();
      }
    });
  }

  // Natural Language Intent Parser
  async function parseNaturalLanguagePrompt(prompt) {
    document.body.classList.add('ai-compute-active');
    try {
      showAiToast("Copilot thinking...");
      const reqHeaders = {'Content-Type': 'application/json'};
      if (currentChatGptSession && currentChatGptSession.session_id) {
        reqHeaders['Authorization'] = 'Bearer ' + currentChatGptSession.session_id;
      }
      const res = await fetch('/api/v1/copilot', {
        method: 'POST',
        headers: reqHeaders,
        body: JSON.stringify({ prompt, current_state: state })
      });
      if (res.ok) {
        const data = await res.json();
        pushUndo(`AI Copilot: "${prompt.slice(0, 20)}..."`);
        Object.assign(state, data.modified_state);
        syncFormFromState();
        renderLineItemsTable();
        renderSectionsList();
        renderSignaturesList();
        renderCustomFieldsList();
        updateCalculations();
        renderInteractiveOverlay();
        schedulePreview();
        showAiToast(data.ai_message || "Done!");
        showUndoToast(`✨ Copilot updated document · [↩ Undo]`);
      }
    } catch (err) {
      console.error(err);
      showAiToast("Copilot Error: " + err);
    } finally {
      document.body.classList.remove('ai-compute-active');
    }
  }

  function showAiToast(msg) {
    const t = document.getElementById('aiToast');
    t.innerText = "✨ " + msg;
    t.classList.add('show');
    setTimeout(() => t.classList.remove('show'), 3000);
  }

  // 🎲 Randomize Scene
  let sceneIndex = 0;
  function randomizeScene() {
    pushUndo('Surprise Me (Random Scene)');
    const scene = RANDOM_SCENES[sceneIndex % RANDOM_SCENES.length];
    sceneIndex++;

    Object.assign(state, JSON.parse(JSON.stringify(scene)));
    state.doc_number = `${scene.prefix}2026-${Math.floor(1000 + Math.random() * 9000)}`;

    syncFormFromState();
    renderLineItemsTable();
    renderSectionsList();
    renderSignaturesList();
    renderCustomFieldsList();
    updateCalculations();
    renderInteractiveOverlay();
    schedulePreview();
    showUndoToast(`🎲 Switched to ${scene.title} · [↩ Undo]`);
  }

  // 📋 Clone Document
  function cloneDocument() {
    pushUndo('Clone Document');
    const cloned = JSON.parse(JSON.stringify(state));
    cloned.doc_number = `${state.doc_number.split('-')[0] || 'DOC'}-2026-${Math.floor(1000 + Math.random() * 9000)}`;
    Object.assign(state, cloned);
    syncFormFromState();
    renderLineItemsTable();
    renderSectionsList();
    renderSignaturesList();
    renderCustomFieldsList();
    updateCalculations();
    renderInteractiveOverlay();
    schedulePreview();
    showUndoToast(`📋 Cloned #${state.doc_number} · [↩ Undo]`);
  }

  // Sync Form UI Elements from State
  function syncFormFromState() {
    const setVal = (id, val) => {
      const el = document.getElementById(id);
      if (el && val !== undefined) el.value = val;
    };
    const setCheck = (id, val) => {
      const el = document.getElementById(id);
      if (el && val !== undefined) el.checked = !!val;
    };

    if (state.issue_date) state.issue_date = normalizeDateIso(state.issue_date);
    if (state.due_date) state.due_date = normalizeDateIso(state.due_date);
    state.compliance_mode = normalizeComplianceMode(state.compliance_mode);

    setVal('docTitle', state.title);
    setVal('headerSubtitle', state.header_subtitle || '');
    setVal('docNumber', state.doc_number);
    setVal('issueDate', state.issue_date);
    setVal('dueDate', state.due_date);
    setVal('primaryColorInput', state.primary_color);
    setVal('layoutStyleSelect', state.layout_style || 'standard');
    setVal('fontFamilySelect', state.font_family || 'sans');
    setVal('fontScaleSelect', state.font_scale || 'standard');
    setVal('spacingDensitySelect', state.spacing_density || 'standard');
    setVal('paperSizeSelect', state.paper_size || 'us-letter');
    setVal('complianceSelect', state.compliance_mode || 'none');
    setVal('stateCodeSelect', state.state_code || '');
    setVal('municipalitySelect', state.municipality || '');
    setVal('taxRateInput', state.tax_rate);
    setVal('discountInput', state.discount_amount);
    setVal('footerTextInput', state.footer_text || '');
    setVal('notesInput', state.notes || '');
    setVal('termsInput', state.terms || '');

    // Sync Formatters & Localization
    setVal('currencySelect', state.currency || 'USD');
    setVal('currencySymbolModeSelect', state.currency_symbol_mode || 'symbol');
    setVal('currencyPlacementSelect', state.currency_placement || 'prefix');
    setVal('separatorsSelect', state.currency_separators || 'standard');
    setVal('dateFormatSelect', state.date_format || 'iso');
    setVal('unitMeasureSelect', state.unit_measure || 'units');
    setVal('numberDecimalsSelect', state.number_decimals !== undefined ? String(state.number_decimals) : '2');

    // Sync Party Details
    setVal('fromName', state.from.name || '');
    setVal('fromCompany', state.from.company || '');
    setVal('fromAddress', state.from.address || '');
    setVal('fromEmail', state.from.email || '');
    setVal('fromPhone', state.from.phone || '');
    setVal('fromTaxId', state.from.tax_id || '');
    setVal('fromWebsite', state.from.website || '');
    setVal('fromRegNumber', state.from.registration_number || '');

    setVal('toName', state.to.name || '');
    setVal('toCompany', state.to.company || '');
    setVal('toAddress', state.to.address || '');
    setVal('toEmail', state.to.email || '');
    setVal('toPhone', state.to.phone || '');
    setVal('toTaxId', state.to.tax_id || '');
    setVal('toWebsite', state.to.website || '');
    setVal('toRegNumber', state.to.registration_number || '');

    setCheck('isPaidCheck', state.is_paid);
    setCheck('toggleQrSeal', state.show_qr_seal);
    setCheck('toggleSignature', state.show_signature);
    setCheck('toggleRemittance', state.show_remittance_stub);
    setCheck('toggleLineNumbers', state.show_line_numbers);
    setCheck('togglePageNumbers', state.show_page_numbers !== false);
    setCheck('toggleFooterDivider', !!state.show_footer_divider);

    // Sync payment and checkout fields
    setVal('paymentLinkInput', state.payment_link || '');
    setVal('paymentStatusSelect', state.payment_status || (state.is_paid ? 'paid' : 'unpaid'));
    setVal('paymentMethodInput', state.payment_method || 'Stripe / Card / Apple Pay');
    setVal('transactionRefInput', state.transaction_reference || '');
    setCheck('togglePaymentQr', state.payment_qr_code !== false);
    updateLifecycleUI();

    // Sync template archetype pills and adapt prescriptive UI taxonomy
    document.querySelectorAll('.template-pill').forEach(c => {
      c.classList.toggle('active', c.dataset.type === state.doc_type);
    });
    adaptFormToArchetype(state.doc_type);

    // Sync theme palette and tokens
    syncPaletteControlsFromState();

    // Sync background chips
    document.querySelectorAll('.bg-chip').forEach(c => {
      c.classList.toggle('active', c.dataset.bg === state.background_style);
    });

    renderSignaturesList();
    renderSectionsList();
    renderCustomFieldsList();
  }

  // Zoom Controls (Multi-Page Scaled)
  function bindZoomControls() {
    const updateZoom = (z) => {
      currentZoom = Math.min(200, Math.max(50, z));
      document.querySelectorAll('.preview-page-frame').forEach(f => {
        f.style.transform = `scale(${currentZoom / 100})`;
      });
      const wrapper = document.getElementById('previewDocWrapper');
      if (wrapper) wrapper.style.transform = `scale(${currentZoom / 100})`;
      const zoomDisplay = document.getElementById('zoomDisplay');
      if (zoomDisplay) zoomDisplay.innerText = `${currentZoom}%`;
    };

    const btnIn = document.getElementById('btnZoomIn');
    if (btnIn) btnIn.addEventListener('click', () => updateZoom(currentZoom + 10));
    const btnOut = document.getElementById('btnZoomOut');
    if (btnOut) btnOut.addEventListener('click', () => updateZoom(currentZoom - 10));
    const btnReset = document.getElementById('btnZoomReset');
    if (btnReset) btnReset.addEventListener('click', () => updateZoom(100));
    const btnFit = document.getElementById('btnZoomFit');
    if (btnFit) {
      btnFit.addEventListener('click', () => {
        const viewport = document.getElementById('previewViewport');
        const firstFrame = document.querySelector('.preview-page-frame');
        if (viewport && firstFrame) {
          const availWidth = viewport.clientWidth - 60;
          const frameWidth = firstFrame.offsetWidth || 595;
          const fitPercent = Math.min(180, Math.max(50, Math.round((availWidth / frameWidth) * 100)));
          updateZoom(fitPercent);
        }
      });
    }
  }

  // Dropdown Menus
  function bindDropdowns() {
    document.querySelectorAll('.dropdown').forEach(dropdown => {
      const trigger = dropdown.querySelector('.btn-dropdown-trigger');
      if (!trigger) return;
      trigger.addEventListener('click', (e) => {
        e.stopPropagation();
        document.querySelectorAll('.dropdown.open').forEach(d => {
          if (d !== dropdown) d.classList.remove('open');
        });
        dropdown.classList.toggle('open');
      });
    });

    document.addEventListener('click', (e) => {
      if (!e.target.closest('.dropdown')) {
        document.querySelectorAll('.dropdown.open').forEach(d => d.classList.remove('open'));
      }
    });

    document.querySelectorAll('.dropdown-item').forEach(item => {
      item.addEventListener('click', () => {
        document.querySelectorAll('.dropdown.open').forEach(d => d.classList.remove('open'));
      });
    });

    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        document.querySelectorAll('.dropdown.open').forEach(d => d.classList.remove('open'));
      }
    });
  }

  // View Modes Switcher
  function setViewMode(mode) {
    const mainContainer = document.querySelector('.main-container');
    if (!mainContainer) return;
    mainContainer.classList.remove('view-mode-split', 'view-mode-editor', 'view-mode-preview');
    mainContainer.classList.add(`view-mode-${mode}`);

    document.querySelectorAll('.segmented-btn').forEach(btn => {
      btn.classList.toggle('active', btn.dataset.view === mode);
    });

    if (mode === 'preview') {
      schedulePreview();
    }
  }

  function bindViewModes() {
    document.querySelectorAll('.segmented-btn').forEach(btn => {
      btn.addEventListener('click', () => {
        const mode = btn.dataset.view;
        if (mode) setViewMode(mode);
      });
    });
  }

  // Multi-Page Navigation & Stack Rendering
  function bindMultiPageControls() {
    if (btnPrevPage) {
      btnPrevPage.addEventListener('click', () => {
        if (currentPageIndex > 0) {
          currentPageIndex--;
          renderMultiPagePreview();
        }
      });
    }

    if (btnNextPage) {
      btnNextPage.addEventListener('click', () => {
        if (currentPageIndex < currentTotalPages - 1) {
          currentPageIndex++;
          renderMultiPagePreview();
        }
      });
    }

    if (btnViewStack) {
      btnViewStack.addEventListener('click', () => {
        pageViewMode = 'stack';
        btnViewStack.classList.add('active');
        if (btnViewSingle) btnViewSingle.classList.remove('active');
        renderMultiPagePreview();
      });
    }

    if (btnViewSingle) {
      btnViewSingle.addEventListener('click', () => {
        pageViewMode = 'single';
        btnViewSingle.classList.add('active');
        if (btnViewStack) btnViewStack.classList.remove('active');
        renderMultiPagePreview();
      });
    }
  }

  function updatePreviewFrame(frame, pageIdx, totalPages, uri, isPrimary) {
    frame.style.transform = `scale(${currentZoom / 100})`;

    let tag = frame.querySelector('.page-number-tag');
    if (!tag) {
      tag = document.createElement('div');
      tag.className = 'page-number-tag';
      frame.prepend(tag);
    }
    tag.innerText = `Page ${pageIdx + 1} of ${totalPages}`;

    let buffer = frame.querySelector('.preview-stage-buffer');
    if (!buffer) {
      buffer = document.createElement('div');
      buffer.className = 'preview-stage-buffer';
      frame.appendChild(buffer);
    }

    const currentImg = buffer.querySelector('.preview-doc-image');
    if (!currentImg) {
      const img = document.createElement('img');
      if (isPrimary) img.id = 'previewImg';
      img.className = 'preview-doc-image preview-layer-current';
      img.src = uri;
      img.alt = `Live Document Preview - Page ${pageIdx + 1}`;
      buffer.appendChild(img);
    } else if (!currentImg.getAttribute('src')) {
      currentImg.src = uri;
    } else if (currentImg.src !== uri && uri) {
      const nextImg = new Image();
      if (isPrimary) nextImg.id = 'previewImg';
      nextImg.className = 'preview-doc-image preview-layer-next preview-layer-fade-out';
      nextImg.alt = `Live Document Preview - Page ${pageIdx + 1}`;
      nextImg.onload = () => {
        buffer.appendChild(nextImg);
        requestAnimationFrame(() => {
          currentImg.classList.add('preview-layer-fade-out');
          nextImg.classList.remove('preview-layer-fade-out');
          nextImg.classList.add('preview-layer-fade-in');
          setTimeout(() => {
            if (currentImg && currentImg.parentElement) currentImg.remove();
            nextImg.classList.remove('preview-layer-fade-in');
            nextImg.classList.add('preview-layer-current');
          }, 240);
        });
      };
      nextImg.onerror = () => {
        currentImg.src = uri;
      };
      nextImg.src = uri;
    }

    if (isPrimary) {
      let overlay = frame.querySelector('#previewInteractiveOverlay');
      if (!overlay) {
        overlay = document.createElement('div');
        overlay.id = 'previewInteractiveOverlay';
        overlay.className = 'preview-interactive-overlay';
        frame.appendChild(overlay);
      }
    }
  }

  function renderMultiPagePreview() {
    const container = document.getElementById('previewPagesContainer');
    if (!container) return;

    if (pageDisplay) {
      if (pageViewMode === 'stack') {
        pageDisplay.innerText = `${currentTotalPages} Page${currentTotalPages > 1 ? 's' : ''}`;
      } else {
        pageDisplay.innerText = `Page ${currentPageIndex + 1} of ${currentTotalPages}`;
      }
    }

    if (btnPrevPage) btnPrevPage.disabled = (pageViewMode === 'stack' || currentPageIndex <= 0);
    if (btnNextPage) btnNextPage.disabled = (pageViewMode === 'stack' || currentPageIndex >= currentTotalPages - 1);

    const pages = (currentPages && currentPages.length > 0) ? currentPages : (previewImg && previewImg.src ? [previewImg.src] : []);
    const targetPages = (pageViewMode === 'stack') ? pages : [pages[Math.min(currentPageIndex, pages.length - 1)] || ''];
    const targetCount = targetPages.length;

    let existingFrames = Array.from(container.querySelectorAll('.preview-page-frame'));
    while (existingFrames.length > targetCount) {
      existingFrames.pop().remove();
    }
    while (existingFrames.length < targetCount) {
      const idx = existingFrames.length;
      const f = document.createElement('div');
      f.className = 'preview-doc-wrapper preview-page-frame';
      f.id = (idx === 0) ? 'previewDocWrapper' : `previewDocWrapper-${idx}`;
      container.appendChild(f);
      existingFrames.push(f);
    }

    existingFrames.forEach((frame, idx) => {
      const actualPageIdx = (pageViewMode === 'stack') ? idx : Math.min(currentPageIndex, pages.length - 1);
      const isPrimary = (actualPageIdx === 0);
      frame.id = isPrimary ? 'previewDocWrapper' : `previewDocWrapper-${actualPageIdx}`;
      frame.dataset.pageIndex = actualPageIdx;
      updatePreviewFrame(frame, actualPageIdx, pages.length, targetPages[idx], isPrimary);
    });

    if (pageViewMode === 'stack' || currentPageIndex === 0) {
      renderInteractiveOverlay();
    }
  }

  // Primary Document Archetype Header & Prescriptive Taxonomies
  function bindPrimaryArchetypeHeader() {
    const btnOpen = document.getElementById('btnOpenArchetypeModal');
    if (btnOpen) {
      btnOpen.addEventListener('click', () => {
        switchSidebarTab('design');
        const archetypeCard = document.querySelector('#tabContent-design .control-card');
        if (archetypeCard) {
          archetypeCard.scrollIntoView({ behavior: 'smooth', block: 'start' });
          archetypeCard.classList.remove('collapsed');
        }
      });
    }

    // Quick Category Chips
    const chips = document.querySelectorAll('.cat-chip');
    chips.forEach(chip => {
      chip.addEventListener('click', () => {
        chips.forEach(c => c.classList.remove('active'));
        chip.classList.add('active');
        const cat = chip.dataset.category;

        switchSidebarTab('design');

        const categoryTabs = document.querySelectorAll('.category-tab');
        categoryTabs.forEach(ct => {
          ct.classList.toggle('active', ct.dataset.category === cat);
        });

        document.querySelectorAll('.template-pill').forEach(pill => {
          if (cat === 'all' || pill.dataset.category === cat) {
            pill.style.display = 'flex';
          } else {
            pill.style.display = 'none';
          }
        });
      });
    });
  }

  function adaptFormToArchetype(type) {
    const heroIcon = document.getElementById('currentArchetypeIcon');
    const heroCategory = document.getElementById('currentCategoryEyebrow');
    const heroHeading = document.getElementById('currentArchetypeHeading');
    const cardLineItems = document.getElementById('cardLineItems');
    const lineItemsTitle = document.getElementById('lineItemsCardTitle');
    const fromBadge = document.getElementById('fromPartyBadge');
    const toBadge = document.getElementById('toPartyBadge');
    const fromRole = document.getElementById('fromPartyRoleHint');
    const toRole = document.getElementById('toPartyRoleHint');
    const grandTotalLabel = document.getElementById('grandTotalLabel');

    const ARCHETYPE_TAXONOMY = {
      // Invoices
      invoice: { icon: '📄', cat: 'Commercial Invoicing', name: 'Tax Invoice', fromBadge: '● ISSUER / MERCHANT', fromRole: 'Originator', toBadge: '○ BILLED TO / CLIENT', toRole: 'Primary Client', items: 'Line Items & Deliverables', totalLabel: 'Total Due' },
      invoice_standard: { icon: '📄', cat: 'Commercial Invoicing', name: 'Standard Tax Invoice', fromBadge: '● ISSUER / MERCHANT', fromRole: 'Originator', toBadge: '○ BILLED TO / CLIENT', toRole: 'Primary Client', items: 'Line Items & Deliverables', totalLabel: 'Total Due' },
      invoice_freelance: { icon: '💻', cat: 'Professional Services', name: 'Freelance & Consulting Invoice', fromBadge: '● CONSULTANT / PRACTITIONER', fromRole: 'Contractor', toBadge: '○ CLIENT / SPONSOR', toRole: 'Principal Client', items: 'Consulting Tasks & Retainer Milestones', totalLabel: 'Total Consulting Fee Due' },
      invoice_construction: { icon: '🏗️', cat: 'Construction & Trades', name: 'Progress Billing Application', fromBadge: '● GENERAL CONTRACTOR', fromRole: 'Prime Contractor', toBadge: '○ PROPERTY OWNER / DEVELOPER', toRole: 'Owner Entity', items: 'Schedule of Values (SOV) Work Items', totalLabel: 'Net Progress Payment Demanded' },
      invoice_medical: { icon: '🩺', cat: 'Healthcare & Clinical', name: 'Clinical Services Superbill', fromBadge: '● HEALTHCARE PROVIDER / CLINIC', fromRole: 'Licensed Clinician', toBadge: '○ PATIENT / SUBSCRIBER', toRole: 'Responsible Party', items: 'CPT Diagnostic & Clinical Procedures', totalLabel: 'Patient Financial Responsibility' },
      invoice_saas: { icon: '☁️', cat: 'Cloud & Software', name: 'SaaS Subscription Statement', fromBadge: '● SOFTWARE OPERATOR', fromRole: 'Platform Provider', toBadge: '○ SUBSCRIBER / ACCOUNT HOLDER', toRole: 'Account Admin', items: 'User Seats, Compute Units & Add-ons', totalLabel: 'Scheduled Recurring Charge' },
      invoice_minimal: { icon: '📑', cat: 'Commercial Invoicing', name: 'Minimal Commercial Invoice', fromBadge: '● ISSUER', fromRole: 'Originator', toBadge: '○ RECIPIENT', toRole: 'Client', items: 'Itemized Schedule', totalLabel: 'Total Due' },
      proforma_invoice: { icon: '📑', cat: 'International Trade', name: 'Proforma Invoice', fromBadge: '● EXPORTER / CONSIGNOR', fromRole: 'Seller Entity', toBadge: '○ IMPORTER / CONSIGNEE', toRole: 'Buyer Entity', items: 'Customs Valuation & Goods Schedule', totalLabel: 'Proforma Estimated Total' },

      // Quotes, Estimates & Procurement
      estimate: { icon: '📊', cat: 'Commercial Estimates', name: 'Formal Quote & Estimate', fromBadge: '● ESTIMATING CONTRACTOR', fromRole: 'Quoting Party', toBadge: '○ PROSPECT / CLIENT', toRole: 'Prospective Client', items: 'Proposed Deliverables & Scope Items', totalLabel: 'Total Estimated Investment' },
      receipt: { icon: '🧾', cat: 'Accounting & Audit', name: 'Official Payment Receipt', fromBadge: '● RECEIVED BY / PAYEE', fromRole: 'Recipient Merchant', toBadge: '○ PAID BY / REMITTER', toRole: 'Paying Customer', items: 'Satisfied Billable Items', totalLabel: 'Total Settled & Paid' },
      purchase_order: { icon: '📦', cat: 'Procurement & Supply Chain', name: 'Enterprise Purchase Order', fromBadge: '● PURCHASING AGENT / BUYER', fromRole: 'Procuring Entity', toBadge: '○ VENDOR / SUPPLIER', toRole: 'Fulfilling Vendor', items: 'Requisition & Bill of Materials (BOM)', totalLabel: 'Authorized Purchase Total' },
      credit_note: { icon: '💳', cat: 'Accounting Adjustments', name: 'Credit Memorandum', fromBadge: '● ISSUING CREDITOR', fromRole: 'Creditor Entity', toBadge: '○ CREDITED CLIENT', toRole: 'Account Holder', items: 'Credit Adjustments & Remittances', totalLabel: 'Total Credit Issued' },
      quotation_tender: { icon: '📑', cat: 'Enterprise Procurement', name: 'Tender & Bid Quotation', fromBadge: '● BIDDING CONTRACTOR', fromRole: 'Prime Bidder', toBadge: '○ PROCURING AUTHORITY', toRole: 'Procurement Officer', items: 'Tender Scope & Bid Itemization', totalLabel: 'Total Bid Valuation' },
      work_order: { icon: '🛠️', cat: 'Field Operations', name: 'Work Order & Job Ticket', fromBadge: '● DISPATCH / CONTRACTOR', fromRole: 'Field Operations', toBadge: '○ CLIENT / WORK SITE', toRole: 'Site Contact', items: 'Labor, Diagnostic & Parts Schedule', totalLabel: 'Total Authorized Work Value' },

      // Agreements & Legal
      agreement: { icon: '⚖️', cat: 'Legal & Contracts', name: 'Master Services Agreement (MSA)', fromBadge: '● SERVICE PROVIDER / CONSULTANT', fromRole: 'Contractor', toBadge: '○ CLIENT / ENTERPRISE', toRole: 'Hiring Principal', items: 'Contract Retainers & Consideration (Optional)', totalLabel: 'Contract Consideration' },
      agreement_sow: { icon: '📝', cat: 'Scope & Deliverables', name: 'Statement of Work (SOW)', fromBadge: '● PERFORMING CONTRACTOR', fromRole: 'Delivery Lead', toBadge: '○ CLIENT PRINCIPAL', toRole: 'Client Sponsor', items: 'Phase Milestones & Deliverables', totalLabel: 'Total Milestone Budget' },
      contractor_agreement: { icon: '🤝', cat: 'Labor & Contracting', name: 'Independent Contractor Agreement', fromBadge: '● INDEPENDENT CONTRACTOR', fromRole: 'Service Professional', toBadge: '○ HIRING ENTITY / PRINCIPAL', toRole: 'Company Principal', items: 'Fee Schedule & Milestones', totalLabel: 'Contractor Compensation' },
      retainer_agreement: { icon: '💎', cat: 'Advisory Services', name: 'Strategic Retainer Agreement', fromBadge: '● STRATEGIC ADVISOR / FIRM', fromRole: 'Lead Counsel', toBadge: '○ CLIENT ENTERPRISE', toRole: 'Client Executive', items: 'Monthly Retainer & Advisory Scope', totalLabel: 'Monthly Retainer Due' },
      nda: { icon: '🔒', cat: 'Confidentiality & IP', name: 'Mutual Non-Disclosure Agreement (NDA)', fromBadge: '● DISCLOSING PARTY', fromRole: 'Information Discloser', toBadge: '○ RECEIVING PARTY', toRole: 'Recipient Party', items: 'None', totalLabel: 'None' },

      // Real Estate & Property Management Suite
      residential_lease: { icon: '🏠', cat: 'Real Estate & Property Management', name: 'Residential Lease Agreement', fromBadge: '● LANDLORD / LESSOR', fromRole: 'Property Owner / Agent', toBadge: '○ TENANT / LESSEE', toRole: 'Primary Occupant', items: 'Rent, Security Deposit & Fee Schedule', totalLabel: 'Total Initial Move-In Commitment' },
      commercial_nnn_lease: { icon: '🏗️', cat: 'Commercial Real Estate (CRE)', name: 'Triple Net (NNN) Commercial Lease', fromBadge: '● LANDLORD / MASTER LESSOR', fromRole: 'Building Owner / REIT', toBadge: '○ COMMERCIAL TENANT / LESSEE', toRole: 'Corporate Tenant', items: 'Base Rent & Operating Expense (CAM) Schedule', totalLabel: 'Total Initial Monthly Commitment' },
      commercial_lease: { icon: '🏢', cat: 'Commercial Real Estate (CRE)', name: 'Commercial Lease Agreement', fromBadge: '● LESSOR / BUILDING OWNER', fromRole: 'Property Authority', toBadge: '○ LESSEE / COMMERCIAL TENANT', toRole: 'Tenant Entity', items: 'Commercial Rent & Security Deposit Schedule', totalLabel: 'Total Move-In Commitment' },
      property_management_statement: { icon: '📊', cat: 'Property Management & Escrow', name: 'Monthly Property Management Statement', fromBadge: '● PROPERTY MANAGER / AGENT', fromRole: 'Management Firm', toBadge: '○ BENEFICIAL PROPERTY OWNER', toRole: 'Asset Owner', items: 'Property Income & Operating Expense Ledger', totalLabel: 'Net Owner Distribution Due' },
      rent_roll: { icon: '📑', cat: 'Real Estate Valuation & Audit', name: 'Multi-Unit Rent Roll & Tenancy Schedule', fromBadge: '● PROPERTY ASSET MANAGER', fromRole: 'Asset Management', toBadge: '○ UNDERWRITER / FINANCIAL LENDER', toRole: 'Auditing Institution', items: 'Multi-Unit Tenancy Schedule & Rent Roll Census', totalLabel: 'Total Scheduled Monthly Rent' },
      closing_settlement_statement: { icon: '📜', cat: 'Real Estate Closing & Escrow', name: 'Real Estate Closing Settlement Statement', fromBadge: '● ESCROW & SETTLEMENT AGENT', fromRole: 'Title Officer', toBadge: '○ BUYER & SELLER PRINCIPALS', toRole: 'Transaction Parties', items: 'Escrow Settlement Disbursements & Adjustments', totalLabel: 'Net Settlement Due' },
      notice_to_vacate: { icon: '⚠️', cat: 'Statutory Landlord Notice', name: 'Notice to Vacate & Pay Delinquent Rent', fromBadge: '● LANDLORD / PROPERTY MANAGER', fromRole: 'Authorized Claimant', toBadge: '○ DELINQUENT TENANT / OCCUPANT', toRole: 'Primary Tenant', items: 'Statutory Delinquency & Past-Due Rental Ledger', totalLabel: 'Total Delinquent Balance Demanded' },
      security_deposit_disposition: { icon: '🛡️', cat: 'Landlord Statutory Compliance', name: 'Security Deposit Itemized Disposition', fromBadge: '● LANDLORD / PROPERTY MANAGER', fromRole: 'Escrow Fiduciary', toBadge: '○ FORMER TENANT / VACATING LESSEE', toRole: 'Beneficial Payee', items: 'Security Deposit Itemization & Deductions', totalLabel: 'Net Deposit Refund Due' },
      real_estate_commission: { icon: '💼', cat: 'Real Estate Brokerage', name: 'Brokerage Commission Disbursement Statement', fromBadge: '● LISTING / SELLING BROKERAGE', fromRole: 'Managing Broker', toBadge: '○ CLOSING TITLE & ESCROW AGENT', toRole: 'Disbursing Fiduciary', items: 'Transaction Commissions & Agent Splits', totalLabel: 'Total Commission Disbursement' },
      property_inspection_checklist: { icon: '📋', cat: 'Property Operations & Auditing', name: 'Property Condition & Inspection Checklist', fromBadge: '● INSPECTING AGENT / MANAGER', fromRole: 'Inspector', toBadge: '○ TENANT / RESIDENTIAL OCCUPANT', toRole: 'Acknowledging Tenant', items: 'Property Condition & Fixture Audit Matrix', totalLabel: 'None' },
      letter_of_intent_cre: { icon: '🤝', cat: 'Commercial Real Estate (CRE)', name: 'CRE Letter of Intent (LOI) to Lease', fromBadge: '● PROSPECTIVE TENANT / LESSEE', fromRole: 'Tenant Principal', toBadge: '○ BUILDING OWNER / LESSOR', toRole: 'Landlord Entity', items: 'Commercial Lease Terms & Proposed Consideration', totalLabel: 'Proposed Initial Deposit Commitment' },
      tenant_estoppel: { icon: '🏛️', cat: 'CRE Diligence & Financing', name: 'Tenant Estoppel Certificate', fromBadge: '● TENANT ENTITY / LESSEE', fromRole: 'Occupying Tenant', toBadge: '○ PURCHASER & LENDING INSTITUTION', toRole: 'Relying Beneficiaries', items: 'Lease Standing & Financial Attestation Schedule', totalLabel: 'Confirmed Monthly Base Rent' },
      room_rental_lease: { icon: '🛏️', cat: 'Real Estate & Co-Living', name: 'Room Rental & Co-Living Lease Agreement', fromBadge: '● LANDLORD / HOUSE MANAGER', fromRole: 'Property Owner / Master Lessor', toBadge: '○ CO-TENANT / ROOM OCCUPANT', toRole: 'Bedroom Lessee', items: 'Bedroom Rent, Utility Split & Deposit Schedule', totalLabel: 'Total Move-In Commitment' },
      lease_renewal_agreement: { icon: '🔄', cat: 'Real Estate & Property Management', name: 'Lease Renewal & Extension Agreement', fromBadge: '● LANDLORD / LESSOR', fromRole: 'Property Owner / Agent', toBadge: '○ RENEWING TENANT / LESSEE', toRole: 'Occupying Tenant', items: 'Renewed Monthly Rent & Fee Schedule', totalLabel: 'Total Renewed Monthly Rent' },
      lease_amendment: { icon: '📝', cat: 'Real Estate & Property Management', name: 'Lease Amendment & Addendum', fromBadge: '● LANDLORD / LESSOR', fromRole: 'Property Owner / Agent', toBadge: '○ TENANT / LESSEE', toRole: 'Occupying Tenant', items: 'Amended Lease Charges & Deposits', totalLabel: 'Total Addendum Adjustment' },
      rental_application: { icon: '🪪', cat: 'Tenant Screening & Leasing', name: 'Residential Rental Application', fromBadge: '● LEASING OFFICE / LANDLORD', fromRole: 'Screening Authority', toBadge: '○ RENTAL APPLICANT', toRole: 'Prospective Tenant', items: 'Application & Screening Fee Schedule', totalLabel: 'Total Application Fee Due' },
      adverse_action_notice: { icon: '⚖️', cat: 'FCRA Leasing Compliance', name: 'FCRA Adverse Action Notice', fromBadge: '● LANDLORD / SCREENING ENTITY', fromRole: 'Compliance Officer', toBadge: '○ RENTAL APPLICANT', toRole: 'Consumer Applicant', items: 'None', totalLabel: 'None' },
      notice_of_entry: { icon: '🚪', cat: 'Statutory Landlord Notice', name: 'Landlord Notice of Entry', fromBadge: '● LANDLORD / PROPERTY MANAGER', fromRole: 'Authorized Agent', toBadge: '○ OCCUPYING TENANT', toRole: 'Resident', items: 'None', totalLabel: 'None' },
      rent_increase_notice: { icon: '📈', cat: 'Statutory Landlord Notice', name: 'Statutory Notice of Rent Increase', fromBadge: '● LANDLORD / PROPERTY MANAGER', fromRole: 'Property Owner / Agent', toBadge: '○ RESIDENTIAL TENANT', toRole: 'Occupying Lessee', items: 'Prior vs. Adjusted Monthly Rent Comparison', totalLabel: 'New Monthly Rent Effective' },
      mutual_lease_termination: { icon: '🤝', cat: 'Lease Surrender & Settlement', name: 'Mutual Lease Termination Agreement', fromBadge: '● LANDLORD / LESSOR', fromRole: 'Property Owner', toBadge: '○ VACATING TENANT / LESSEE', toRole: 'Surrendering Tenant', items: 'Early Surrender Settlement & Deposit Schedule', totalLabel: 'Net Termination Settlement' },
      tenant_rent_ledger: { icon: '📒', cat: 'Property Accounting & Audit', name: 'Tenant Rent & Payment Ledger', fromBadge: '● PROPERTY ACCOUNTING / ESCROW', fromRole: 'Ledger Custodian', toBadge: '○ TENANT ACCOUNT HOLDER', toRole: 'Lessee of Record', items: 'Chronological Charges, Payments & Running Balance', totalLabel: 'Current Account Balance Due' },
      schedule_of_real_estate_owned: { icon: '🏦', cat: 'Real Estate Portfolio & Lending', name: 'Schedule of Real Estate Owned (SREO)', fromBadge: '● PORTFOLIO PRINCIPAL / BORROWER', fromRole: 'Sponsor / Guarantor', toBadge: '○ COMMERCIAL LENDER / UNDERWRITER', toRole: 'Lending Institution', items: 'Portfolio Asset Valuation, Debt, LTV, DSCR & NOI', totalLabel: 'Total Annual Portfolio NOI' },
      schedule_e_tax_worksheet: { icon: '🧾', cat: 'Real Estate Tax & Accounting', name: 'IRS Schedule E Tax Prep Worksheet', fromBadge: '● RENTAL PROPERTY OWNER', fromRole: 'Taxpayer Entity', toBadge: '○ CPA / TAX PREPARER', toRole: 'Accounting Firm', items: 'IRS Form 1040 Schedule E Part I Income & Deductions', totalLabel: 'Net Schedule E Taxable Income' },
      real_estate_deal_analysis: { icon: '📐', cat: 'Real Estate Investment Underwriting', name: 'Real Estate Investment Deal Analysis', fromBadge: '● ACQUISITIONS / SPONSOR', fromRole: 'Underwriting Lead', toBadge: '○ INVESTMENT COMMITTEE / LP', toRole: 'Capital Partner', items: 'Pro Forma Cash Flow, Rehab & Debt Service Schedule', totalLabel: 'Projected Year-1 Net Cash Flow' },
      sublease_agreement: { icon: '🔑', cat: 'Real Estate & Subtenancy', name: 'Residential Sublease Agreement', fromBadge: '● MASTER TENANT / SUBLESSOR', fromRole: 'Original Lessee', toBadge: '○ INCOMING SUBTENANT / SUBLESSEE', toRole: 'Sublessee Occupant', items: 'Sublease Rent & Security Deposit Schedule', totalLabel: 'Total Initial Sublease Commitment' },
      rent_to_own_agreement: { icon: '🏡', cat: 'Lease-Option & Creative Finance', name: 'Rent-to-Own (Lease-Option) Agreement', fromBadge: '● PROPERTY OWNER / OPTIONOR', fromRole: 'Seller / Lessor', toBadge: '○ TENANT-BUYER / OPTIONEE', toRole: 'Optionee / Lessee', items: 'Option Consideration, Base Rent & Purchase Credits', totalLabel: 'Total Initial Option & Move-In Due' },
      property_management_agreement: { icon: '🏢', cat: 'Property Management Agency', name: 'Property Management Agreement', fromBadge: '● LICENSED PROPERTY MANAGER', fromRole: 'Managing Brokerage', toBadge: '○ BENEFICIAL PROPERTY OWNER', toRole: 'Asset Principal', items: 'Management Fee, Leasing Fee & Reserve Schedule', totalLabel: 'Initial Owner Reserve & Setup' },
      real_estate_purchase_agreement: { icon: '✍️', cat: 'Real Estate Acquisition & Closing', name: 'Real Estate Purchase & Sale Agreement', fromBadge: '● SELLER / CURRENT TITLE HOLDER', fromRole: 'Conveying Seller', toBadge: '○ PURCHASER / ACQUIRING ENTITY', toRole: 'Contract Buyer', items: 'Purchase Price, Earnest Money & Closing Credits', totalLabel: 'Net Contract Balance at Closing' },
      property_deed: { icon: '🏛️', cat: 'County Recorder Title Conveyance', name: 'Quitclaim / Warranty Deed', fromBadge: '● GRANTOR (CONVEYING OWNER)', fromRole: 'Title Grantor', toBadge: '○ GRANTEE (ACQUIRING OWNER)', toRole: 'Title Grantee', items: 'Conveyance Consideration & Transfer Tax Valuation', totalLabel: 'Total Conveyance Consideration' },
      mechanics_lien_waiver: { icon: '🔨', cat: 'Construction & Title Clearance', name: 'Mechanic\'s Lien Waiver & Release', fromBadge: '● CONTRACTOR / LIEN CLAIMANT', fromRole: 'Releasing Contractor', toBadge: '○ PROPERTY OWNER & LENDER', toRole: 'Reliance Parties', items: 'Progress / Final Draw Payment Released', totalLabel: 'Total Payment Subject to Waiver' },

      // Operations & Logistics
      packing_slip: { icon: '🚚', cat: 'Logistics & Fulfillment', name: 'Packing Slip & Bill of Lading', fromBadge: '● SHIPPER / LOGISTICS HUB', fromRole: 'Carrier / Dispatch', toBadge: '○ CONSIGNEE / RECEIVER', toRole: 'Receiving Party', items: 'Manifest & Package Contents', totalLabel: 'Total Pieces Shipped' },
      bill_of_sale: { icon: '📜', cat: 'Legal Conveyance & Title', name: 'Statutory Bill of Sale', fromBadge: '● SELLER / TRANSFEROR', fromRole: 'Conveying Party', toBadge: '○ BUYER / TRANSFEREE', toRole: 'Acquiring Party', items: 'Conveyed Personal Property & Assets', totalLabel: 'Total Purchase Consideration' },
      certificate: { icon: '🏆', cat: 'Honors & Recognition', name: 'Certificate of Achievement', fromBadge: '● ISSUING AUTHORITY', fromRole: 'Accreditation Board', toBadge: '○ RECIPIENT / HONOREE', toRole: 'Conferred Individual', items: 'None', totalLabel: 'None' },
      certificate_completion: { icon: '🎖️', cat: 'Professional Credentials', name: 'Certificate of Completion', fromBadge: '● CERTIFYING INSTITUTION', fromRole: 'Authorized Dean', toBadge: '○ CANDIDATE / GRADUATE', toRole: 'Certified Candidate', items: 'None', totalLabel: 'None' },
      certificate_insurance: { icon: '🛡️', cat: 'Risk Management & Insurance', name: 'Certificate of Insurance (COI)', fromBadge: '● AUTHORIZED BROKER / INSURER', fromRole: 'Issuing Producer', toBadge: '○ CERTIFICATE HOLDER / INSURED', toRole: 'Covered Entity', items: 'Coverage Schedule & Policy Limits', totalLabel: 'Total Combined Aggregate Limit' },
      expense_report: { icon: '💵', cat: 'Corporate Finance', name: 'Corporate Expense Reimbursement', fromBadge: '● CLAIMANT / EMPLOYEE', fromRole: 'Staff Member', toBadge: '○ FINANCE DEPARTMENT', toRole: 'Approving Manager', items: 'Itemized Receipts & Business Expenses', totalLabel: 'Total Reimbursement Due' },
      delivery_receipt: { icon: '📦', cat: 'Logistics & Receiving', name: 'Proof of Delivery Receipt', fromBadge: '● COURIER / FREIGHT OPERATOR', fromRole: 'Delivery Carrier', toBadge: '○ RECIPIENT / RECEIVING CLERK', toRole: 'Accepting Signatory', items: 'Delivered Cartons & Tracking Items', totalLabel: 'Total Items Received' },
      inspection_report: { icon: '📋', cat: 'Quality Assurance', name: 'Technical Inspection Report', fromBadge: '● LEAD INSPECTOR', fromRole: 'QA Examiner', toBadge: '○ AUDITING ORGANIZATION', toRole: 'Compliance Lead', items: 'Inspection Findings & Discrepancies', totalLabel: 'None' }
    };

    const tax = ARCHETYPE_TAXONOMY[type] || ARCHETYPE_TAXONOMY.invoice;
    currentTaxonomyCat = tax.cat;
    currentTaxonomyName = tax.name;

    // 1. Update Step 1 Primary Archetype Header Hero
    if (heroIcon) heroIcon.innerText = tax.icon;
    if (heroCategory) heroCategory.innerText = tax.cat.toUpperCase();
    if (heroHeading) heroHeading.innerText = tax.name;

    // Synchronize Top Studio Breadcrumbs
    const crumbCategory = document.getElementById('crumbCategory');
    const crumbDocName = document.getElementById('crumbDocName');
    const crumbDocNumber = document.getElementById('crumbDocNumber');
    if (crumbCategory) crumbCategory.innerText = tax.cat;
    if (crumbDocName) crumbDocName.innerText = state.title || tax.name;
    if (crumbDocNumber) crumbDocNumber.innerText = '#' + (state.doc_number || 'INV-2026-0001');

    // 2. Prescriptive Line Items & Table Card
    if (cardLineItems) {
      if (tax.items === 'None' || type === 'certificate' || type === 'certificate_completion') {
        cardLineItems.style.display = 'none';
      } else {
        cardLineItems.style.display = 'block';
        if (lineItemsTitle) {
          lineItemsTitle.innerText = `💰 ${tax.items}`;
        }
      }
    }

    // 3. Differentiated Party Badges & Roles
    if (fromBadge) fromBadge.innerText = tax.fromBadge;
    if (fromRole) fromRole.innerText = tax.fromRole;
    if (toBadge) toBadge.innerText = tax.toBadge;
    if (toRole) toRole.innerText = tax.toRole;

    // 4. Prescriptive Totals Label
    if (grandTotalLabel) {
      grandTotalLabel.innerText = tax.totalLabel !== 'None' ? tax.totalLabel : 'Summary Valuation';
    }

    // 5. Sync Category Chips
    document.querySelectorAll('.cat-chip').forEach(chip => {
      const chipCat = chip.dataset.category;
      const pill = document.querySelector(`.template-pill[data-type="${type}"]`);
      if (pill) {
        const pillCat = pill.dataset.category;
        chip.classList.toggle('active', chipCat === pillCat || (chipCat === 'all' && !pillCat));
      }
    });
  }

  // Sidebar Workflow Tabs
  function switchSidebarTab(tabName) {
    document.querySelectorAll('.sidebar-tab-btn').forEach(btn => {
      btn.classList.toggle('active', btn.dataset.tab === tabName);
    });
    document.querySelectorAll('.sidebar-tab-content').forEach(content => {
      const isTarget = content.id === `tabContent-${tabName}`;
      content.style.display = isTarget ? 'flex' : 'none';
      if (isTarget) content.classList.add('active');
      else content.classList.remove('active');
    });

    const titles = {
      content: 'Document Content',
      design: 'Design & Formatting',
      legal: 'Legal & Execution',
      ai: 'AI Copilot & OCR'
    };
    const activeLabel = document.getElementById('activeTabName');
    if (activeLabel) activeLabel.innerText = titles[tabName] || 'Document Content';
  }

  function bindSidebarTabs() {
    document.querySelectorAll('.sidebar-tab-btn').forEach(btn => {
      btn.addEventListener('click', () => {
        const tab = btn.dataset.tab;
        if (tab) switchSidebarTab(tab);
      });
    });
  }

  // Global Accordion Expand / Collapse
  function bindGlobalAccordionToggles() {
    const btnExpand = document.getElementById('btnExpandAll');
    const btnCollapse = document.getElementById('btnCollapseAll');

    if (btnExpand) {
      btnExpand.addEventListener('click', () => {
        const activeTab = document.querySelector('.sidebar-tab-content.active');
        if (activeTab) {
          activeTab.querySelectorAll('.control-card').forEach(card => card.classList.remove('collapsed'));
        }
      });
    }

    if (btnCollapse) {
      btnCollapse.addEventListener('click', () => {
        const activeTab = document.querySelector('.sidebar-tab-content.active');
        if (activeTab) {
          activeTab.querySelectorAll('.control-card').forEach(card => card.classList.add('collapsed'));
        }
      });
    }
  }

  // Archetype Live Search Filter
  function bindArchetypeSearch() {
    const searchInput = document.getElementById('archetypeSearchInput');
    if (!searchInput) return;

    searchInput.addEventListener('input', () => {
      const q = searchInput.value.trim().toLowerCase();
      const pills = document.querySelectorAll('.template-pill');
      pills.forEach(pill => {
        const text = pill.innerText.toLowerCase();
        const type = (pill.dataset.type || '').toLowerCase();
        if (!q || text.includes(q) || type.includes(q)) {
          pill.style.display = 'flex';
        } else {
          pill.style.display = 'none';
        }
      });
    });
  }

  // Sidebar AI Copilot Presets
  function bindSidebarAiCopilot() {
    document.querySelectorAll('.copilot-preset-chip').forEach(chip => {
      chip.addEventListener('click', () => {
        const prompt = chip.dataset.prompt;
        const input = document.getElementById('copilotInput');
        if (input && prompt) {
          input.value = prompt;
          input.focus();
          const submitBtn = document.getElementById('btnCopilotSubmit');
          if (submitBtn) submitBtn.click();
        }
      });
    });
  }

  // Sidebar Scanner Dropzone & File Input
  function bindSidebarScanner() {
    const dropzone = document.getElementById('sidebarPhotoDropzone');
    const fileInput = document.getElementById('sidebarPhotoInput');
    if (!dropzone || !fileInput) return;

    dropzone.addEventListener('click', () => fileInput.click());

    fileInput.addEventListener('change', (e) => {
      const file = e.target.files[0];
      if (file) handleScannedFile(file);
    });

    dropzone.addEventListener('dragover', (e) => {
      e.preventDefault();
      dropzone.classList.add('dragover');
    });

    dropzone.addEventListener('dragleave', () => {
      dropzone.classList.remove('dragover');
    });

    dropzone.addEventListener('drop', (e) => {
      e.preventDefault();
      dropzone.classList.remove('dragover');
      if (e.dataTransfer.files && e.dataTransfer.files[0]) {
        handleScannedFile(e.dataTransfer.files[0]);
      }
    });
  }

  // Floating Dock Actions
  function bindFloatingDockActions() {
    const btnEdit = document.getElementById('dockBtnEdit');
    if (btnEdit) {
      btnEdit.addEventListener('click', () => {
        if (selectedElement) {
          scrollToFormField(selectedElement.type, selectedElement.index);
        }
      });
    }

    const btnDup = document.getElementById('dockBtnDuplicate');
    if (btnDup) {
      btnDup.addEventListener('click', () => {
        if (selectedElement) {
          duplicateSelectedElement();
        }
      });
    }

    const btnDeselect = document.getElementById('dockBtnDeselect');
    if (btnDeselect) {
      btnDeselect.addEventListener('click', () => {
        deselectElement();
      });
    }
  }

  // Export Actions & Zoom Fit
  function bindExportActions() {
    const btnExportPng = document.getElementById('btnExportPng');
    if (btnExportPng) {
      btnExportPng.addEventListener('click', () => {
        const pages = (currentPages && currentPages.length > 0) ? currentPages : (previewImg && previewImg.src ? [previewImg.src] : []);
        if (pages.length > 0) {
          pages.forEach((uri, idx) => {
            const a = document.createElement('a');
            a.href = uri;
            const suffix = pages.length > 1 ? `_p${idx + 1}` : '';
            a.download = `${(state.doc_number || 'document').replace(/[^a-zA-Z0-9_-]/g, '_')}${suffix}.png`;
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
          });
          showUndoToast(pages.length > 1 ? `🖼️ Exported all ${pages.length} high-res PNG pages` : '🖼️ High-Res PNG Exported');
        } else {
          showUndoToast('⚠️ Please wait for preview to finish compiling');
        }
      });
    }
  }

  // Render Multi-Signatory Manager
  function renderSignaturesList(highlightNewIdx) {
    const container = document.getElementById('signaturesContainer');
    if (!container) return;
    container.innerHTML = '';

    state.signatures.forEach((sig, index) => {
      const card = document.createElement('div');
      card.className = (highlightNewIdx !== undefined && index === highlightNewIdx)
        ? 'sig-block-card anim-card-add-emerald'
        : 'sig-block-card anim-slide-in';
      card.innerHTML = `
        <div class="sig-block-header">
          <span class="sig-block-title">✍️ Signatory #${index + 1}</span>
          ${state.signatures.length > 1 ? `<button class="btn-delete-node btn-remove-sig" data-idx="${index}" title="Remove Signatory">×</button>` : ''}
        </div>
        <div class="form-row">
          <div class="form-group">
            <label class="form-label">Signer Full Name</label>
            <input type="text" class="form-input sig-name-input" data-idx="${index}" value="${sig.signer_name || ''}" placeholder="Full Name">
          </div>
          <div class="form-group">
            <label class="form-label">Signer Title / Authority</label>
            <input type="text" class="form-input sig-title-input" data-idx="${index}" value="${sig.signer_title || ''}" placeholder="e.g. Managing Director">
          </div>
        </div>
        <div class="form-row" style="align-items: center; margin-bottom: 6px;">
          <div class="form-group">
            <label class="form-label">Execution Date</label>
            <input type="date" class="form-input sig-date-input" data-idx="${index}" value="${sig.date || ''}">
          </div>
          <div style="display: flex; justify-content: flex-end; align-self: flex-end; margin-bottom: 10px;">
            <button class="btn btn-secondary btn-sm btn-clear-pad" data-idx="${index}">Clear Pad</button>
          </div>
        </div>
        <div class="sig-canvas-container">
          <canvas id="sigCanvas_${index}" class="sig-canvas"></canvas>
        </div>
      `;
      container.appendChild(card);

      // Initialize canvas pad for this signature
      setupSignaturePad(index, card.querySelector(`#sigCanvas_${index}`));
    });

    // Bind Name, Title, Date Inputs
    container.querySelectorAll('.sig-name-input').forEach(input => {
      input.addEventListener('input', (e) => {
        state.signatures[e.target.dataset.idx].signer_name = e.target.value;
        schedulePreview();
      });
    });

    container.querySelectorAll('.sig-title-input').forEach(input => {
      input.addEventListener('input', (e) => {
        state.signatures[e.target.dataset.idx].signer_title = e.target.value;
        schedulePreview();
      });
    });

    container.querySelectorAll('.sig-date-input').forEach(input => {
      input.addEventListener('input', (e) => {
        state.signatures[e.target.dataset.idx].date = e.target.value;
        schedulePreview();
      });
    });

    // Bind Remove Button
    container.querySelectorAll('.btn-remove-sig').forEach(btn => {
      btn.addEventListener('click', (e) => {
        const idx = parseInt(e.target.dataset.idx);
        const card = btn.closest('.sig-block-card');
        if (card) card.classList.add('anim-card-delete-crimson');
        const overlayCard = document.querySelector(`.interactive-card[data-type="signature"][data-index="${idx}"]`);
        if (overlayCard) overlayCard.classList.add('hotspot-delete-crimson');

        setTimeout(() => {
          const removed = state.signatures[idx];
          pushUndo(`Deleted Signatory "${(removed && removed.signer_name) || idx + 1}"`);
          state.signatures.splice(idx, 1);
          renderSignaturesList();
          renderInteractiveOverlay();
          schedulePreview();
          showUndoToast('🗑️ Deleted signatory · [↩ Undo]');
        }, 320);
      });
    });
  }

  // Setup Individual Signature Pad
  function setupSignaturePad(index, padCanvas) {
    if (!padCanvas) return;
    const ctx = padCanvas.getContext('2d');
    padCanvas.width = padCanvas.parentElement.clientWidth || 300;
    padCanvas.height = padCanvas.parentElement.clientHeight || 75;

    ctx.strokeStyle = '#0f172a';
    ctx.lineWidth = 2.2;
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';

    let drawing = false;

    const getPos = (e) => {
      const rect = padCanvas.getBoundingClientRect();
      const clientX = e.touches ? e.touches[0].clientX : e.clientX;
      const clientY = e.touches ? e.touches[0].clientY : e.clientY;
      return { x: clientX - rect.left, y: clientY - rect.top };
    };

    const startDraw = (e) => {
      drawing = true;
      const pos = getPos(e);
      ctx.beginPath();
      ctx.moveTo(pos.x, pos.y);
      e.preventDefault();
    };

    const draw = (e) => {
      if (!drawing) return;
      const pos = getPos(e);
      ctx.lineTo(pos.x, pos.y);
      ctx.stroke();
      e.preventDefault();
    };

    const endDraw = () => {
      if (!drawing) return;
      drawing = false;
      state.signatures[index].signature_data = padCanvas.toDataURL('image/png');
      schedulePreview();
    };

    padCanvas.addEventListener('mousedown', startDraw);
    padCanvas.addEventListener('mousemove', draw);
    window.addEventListener('mouseup', endDraw);

    padCanvas.addEventListener('touchstart', startDraw);
    padCanvas.addEventListener('touchmove', draw);
    window.addEventListener('touchend', endDraw);

    // Clear pad button
    const clearBtn = padCanvas.closest('.sig-block-card').querySelector('.btn-clear-pad');
    if (clearBtn) {
      clearBtn.addEventListener('click', () => {
        ctx.clearRect(0, 0, padCanvas.width, padCanvas.height);
        state.signatures[index].signature_data = null;
        schedulePreview();
      });
    }
  }

  // Render Custom Document Sections
  function renderSectionsList(highlightNewIdx) {
    const container = document.getElementById('sectionsContainer');
    if (!container) return;
    container.innerHTML = '';

    state.sections.forEach((sec, index) => {
      const card = document.createElement('div');
      card.className = (highlightNewIdx !== undefined && index === highlightNewIdx)
        ? 'section-card anim-card-add-emerald'
        : 'section-card anim-slide-in';
      card.innerHTML = `
        <div class="section-card-header">
          <span style="font-size: 11px; font-weight: 700; color: #818cf8;">📄 Section #${index + 1}</span>
          <button class="btn-delete-node btn-remove-sec" data-idx="${index}" title="Remove Section">×</button>
        </div>
        <div class="form-group">
          <input type="text" class="form-input sec-title-input" data-idx="${index}" value="${sec.title || ''}" placeholder="Section Title (e.g. SCOPE OF SERVICES)">
        </div>
        <div class="form-group">
          <textarea class="form-textarea sec-content-input" data-idx="${index}" rows="3" placeholder="Contractual terms, operational specifications, or findings...">${sec.content || ''}</textarea>
        </div>
      `;
      container.appendChild(card);
    });

    container.querySelectorAll('.sec-title-input').forEach(input => {
      input.addEventListener('input', (e) => {
        state.sections[e.target.dataset.idx].title = e.target.value;
        schedulePreview();
      });
    });

    container.querySelectorAll('.sec-content-input').forEach(input => {
      input.addEventListener('input', (e) => {
        state.sections[e.target.dataset.idx].content = e.target.value;
        schedulePreview();
      });
    });

    container.querySelectorAll('.btn-remove-sec').forEach(btn => {
      btn.addEventListener('click', (e) => {
        const idx = parseInt(e.target.dataset.idx);
        const card = btn.closest('.section-card');
        if (card) card.classList.add('anim-card-delete-crimson');
        const overlayCard = document.querySelector(`.interactive-card[data-type="section"][data-index="${idx}"]`);
        if (overlayCard) overlayCard.classList.add('hotspot-delete-crimson');

        setTimeout(() => {
          const removed = state.sections[idx];
          pushUndo(`Deleted Section "${(removed && removed.title) || idx + 1}"`);
          state.sections.splice(idx, 1);
          renderSectionsList();
          renderInteractiveOverlay();
          schedulePreview();
          showUndoToast('🗑️ Deleted section · [↩ Undo]');
        }, 320);
      });
    });
  }

  // Render Custom Metadata Fields
  function renderCustomFieldsList(highlightNewIdx) {
    const container = document.getElementById('customFieldsContainer');
    if (!container) return;
    container.innerHTML = '';

    state.custom_fields.forEach((field, index) => {
      const row = document.createElement('div');
      row.className = (highlightNewIdx !== undefined && index === highlightNewIdx)
        ? 'custom-field-row anim-card-add-emerald'
        : 'custom-field-row anim-slide-in';
      row.innerHTML = `
        <input type="text" class="form-input field-label-input" data-idx="${index}" value="${field.label || ''}" placeholder="Label (e.g. PO #)">
        <input type="text" class="form-input field-value-input" data-idx="${index}" value="${field.value || ''}" placeholder="Value">
        <button class="btn-delete-node btn-remove-field" data-idx="${index}" title="Remove Field">×</button>
      `;
      container.appendChild(row);
    });

    container.querySelectorAll('.field-label-input').forEach(input => {
      input.addEventListener('input', (e) => {
        state.custom_fields[e.target.dataset.idx].label = e.target.value;
        schedulePreview();
      });
    });

    container.querySelectorAll('.field-value-input').forEach(input => {
      input.addEventListener('input', (e) => {
        state.custom_fields[e.target.dataset.idx].value = e.target.value;
        schedulePreview();
      });
    });

    container.querySelectorAll('.btn-remove-field').forEach(btn => {
      btn.addEventListener('click', (e) => {
        const idx = parseInt(e.target.dataset.idx);
        const row = btn.closest('.custom-field-row');
        if (row) row.classList.add('anim-card-delete-crimson');

        setTimeout(() => {
          const removed = state.custom_fields[idx];
          pushUndo(`Deleted Field "${(removed && removed.label) || idx + 1}"`);
          state.custom_fields.splice(idx, 1);
          renderCustomFieldsList();
          renderInteractiveOverlay();
          schedulePreview();
          showUndoToast('🗑️ Deleted field · [↩ Undo]');
        }, 320);
      });
    });
  }

  // Render Line Items
  function renderLineItemsTable(highlightNewIdx) {
    itemsTbody.innerHTML = '';
    state.items.forEach((item, index) => {
      const tr = document.createElement('tr');
      tr.className = (highlightNewIdx !== undefined && index === highlightNewIdx)
        ? 'anim-row-add-emerald'
        : 'anim-slide-in';
      tr.draggable = true;
      tr.dataset.index = index;
      tr.innerHTML = `
        <td class="drag-handle-cell" title="Drag to reorder"><span class="drag-grip">⠿</span></td>
        <td><input type="text" value="${escapeHtml(item.description)}" data-idx="${index}" class="item-desc"></td>
        <td><input type="number" step="0.5" value="${item.quantity}" data-idx="${index}" class="item-qty" style="width: 55px;"></td>
        <td><input type="number" step="1" value="${item.unit_price}" data-idx="${index}" class="item-rate" style="width: 70px;"></td>
        <td class="item-amount-cell" style="text-align: right; font-weight: bold;">${formatAmountUi(lineItemAmount(item))}</td>
        <td style="text-align: center;"><button class="btn-remove btn-delete-row" data-idx="${index}" title="Delete row">×</button></td>
      `;

      // Sidebar Table Drag and Drop
      tr.addEventListener('dragstart', (e) => {
        dragSource = { type: 'item', index, fromPreview: false };
        tr.classList.add('is-dragging');
        e.dataTransfer.effectAllowed = 'move';
        e.dataTransfer.setData('text/plain', index);
      });

      tr.addEventListener('dragend', () => {
        tr.classList.remove('is-dragging');
        itemsTbody.querySelectorAll('.drop-target-before, .drop-target-after').forEach(r => {
          r.classList.remove('drop-target-before', 'drop-target-after');
        });
        dragSource = null;
      });

      tr.addEventListener('dragover', (e) => {
        if (!dragSource || dragSource.type !== 'item' || dragSource.index === index) return;
        e.preventDefault();
        e.dataTransfer.dropEffect = 'move';
        const rect = tr.getBoundingClientRect();
        if (e.clientY - rect.top < rect.height / 2) {
          tr.classList.add('drop-target-before');
          tr.classList.remove('drop-target-after');
        } else {
          tr.classList.add('drop-target-after');
          tr.classList.remove('drop-target-before');
        }
      });

      tr.addEventListener('dragleave', () => {
        tr.classList.remove('drop-target-before', 'drop-target-after');
      });

      tr.addEventListener('drop', (e) => {
        e.preventDefault();
        if (!dragSource || dragSource.type !== 'item') return;
        const fromIdx = dragSource.index;
        const isAfter = tr.classList.contains('drop-target-after');
        tr.classList.remove('drop-target-before', 'drop-target-after');
        if (fromIdx === index) return;

        pushUndo('Reorder Line Items');
        const moved = state.items.splice(fromIdx, 1)[0];
        const targetIdx = isAfter ? (fromIdx < index ? index : index + 1) : (fromIdx < index ? index - 1 : index);
        state.items.splice(targetIdx, 0, moved);
        renderLineItemsTable();
        updateCalculations();
        renderInteractiveOverlay();
        schedulePreview();
        showUndoToast('↕️ Reordered line items · [↩ Undo]');
      });

      itemsTbody.appendChild(tr);
    });

    itemsTbody.querySelectorAll('.item-desc').forEach(input => {
      input.addEventListener('input', (e) => {
        hasUserCustomizedData = true;
        hasUserCustomizedItems = true;
        state.items[e.target.dataset.idx].description = e.target.value;
        schedulePreview();
        renderInteractiveOverlay();
      });
    });

    itemsTbody.querySelectorAll('.item-qty').forEach(input => {
      input.addEventListener('input', (e) => {
        hasUserCustomizedData = true;
        hasUserCustomizedItems = true;
        const idx = e.target.dataset.idx;
        state.items[idx].quantity = parseFloat(e.target.value) || 0;
        const row = e.target.closest('tr');
        const amtCell = row && row.querySelector('.item-amount-cell');
        if (amtCell) amtCell.textContent = formatAmountUi(lineItemAmount(state.items[idx]));
        updateCalculations();
        schedulePreview();
        renderInteractiveOverlay();
      });
    });

    itemsTbody.querySelectorAll('.item-rate').forEach(input => {
      input.addEventListener('input', (e) => {
        hasUserCustomizedData = true;
        hasUserCustomizedItems = true;
        const idx = e.target.dataset.idx;
        const val = parseFloat(e.target.value) || 0;
        state.items[idx].unit_price = val;
        if (val < 0) {
          state.items[idx].discount = 0;
        }
        const row = e.target.closest('tr');
        const amtCell = row && row.querySelector('.item-amount-cell');
        if (amtCell) amtCell.textContent = formatAmountUi(lineItemAmount(state.items[idx]));
        updateCalculations();
        schedulePreview();
        renderInteractiveOverlay();
      });
    });

    itemsTbody.querySelectorAll('.btn-delete-row').forEach(btn => {
      btn.addEventListener('click', (e) => {
        hasUserCustomizedData = true;
        hasUserCustomizedItems = true;
        const idx = parseInt(e.target.dataset.idx);
        const tr = e.target.closest('tr');
        if (tr) tr.classList.add('anim-row-delete-crimson');
        const overlayCard = document.querySelector(`.interactive-card[data-type="item"][data-index="${idx}"]`);
        if (overlayCard) overlayCard.classList.add('hotspot-delete-crimson');

        setTimeout(() => {
          const removed = state.items[idx];
          pushUndo(`Deleted Item "${(removed && removed.description) || idx + 1}"`);
          state.items.splice(idx, 1);
          renderLineItemsTable();
          updateCalculations();
          renderInteractiveOverlay();
          schedulePreview();
          showUndoToast('🗑️ Deleted item · [↩ Undo]');
        }, 320);
      });
    });
  }

  // Format financial values according to user localization and currency preferences
  function formatAmountUi(val) {
    const dec = state.number_decimals !== undefined ? state.number_decimals : 2;
    const numericVal = Number(val) || 0;
    const isNeg = numericVal < 0;
    let parts = Math.abs(numericVal).toFixed(dec).split('.');
    let intPart = parts[0];
    let fracPart = parts[1];

    const sepMode = state.currency_separators || 'standard';
    let thousandsSep = ',';
    let decimalSep = '.';

    if (sepMode === 'euro') {
      thousandsSep = '.';
      decimalSep = ',';
    } else if (sepMode === 'space') {
      thousandsSep = ' ';
      decimalSep = ',';
    } else if (sepMode === 'plain') {
      thousandsSep = '';
      decimalSep = '.';
    }

    if (thousandsSep) {
      intPart = intPart.replace(/\B(?=(\d{3})+(?!\d))/g, thousandsSep);
    }

    const numStr = dec > 0 && fracPart !== undefined ? `${intPart}${decimalSep}${fracPart}` : intPart;

    const currMap = {
      'USD': '$', 'EUR': '€', 'GBP': '£', 'CAD': 'CA$', 'AUD': 'A$',
      'NZD': 'NZ$', 'SGD': 'S$', 'CNY': '¥',
      'JPY': '¥', 'CHF': 'Fr. ', 'INR': '₹', 'BRL': 'R$', 'SEK': 'kr ', 'AED': 'AED '
    };
    const symbolToIso = {
      '$': 'USD', 'US$': 'USD', '€': 'EUR', '£': 'GBP', '¥': 'JPY',
      '₹': 'INR', 'R$': 'BRL', 'CA$': 'CAD', 'C$': 'CAD', 'A$': 'AUD',
      'AU$': 'AUD', 'NZ$': 'NZD', 'S$': 'SGD', 'Fr.': 'CHF', 'Fr. ': 'CHF',
      'kr': 'SEK', 'kr ': 'SEK'
    };
    const rawCurr = (state.currency || 'USD').trim();
    const isoCode = symbolToIso[rawCurr] || rawCurr.toUpperCase();
    let sym = currMap[isoCode] || rawCurr || '$';
    if (state.currency_symbol_mode === 'code') {
      sym = isoCode;
    }

    const signPrefix = isNeg ? '-' : '';
    if (state.currency_placement === 'suffix') {
      return `${signPrefix}${numStr} ${sym}`.trim();
    } else if (state.currency_symbol_mode === 'code') {
      return `${signPrefix}${sym} ${numStr}`;
    } else {
      return `${signPrefix}${sym}${numStr}`;
    }
  }

  function animateNumberChange(el, newText) {
    if (!el) return;
    if (el.innerText !== newText) {
      el.innerText = newText;
      el.classList.remove('number-spring-pop');
      void el.offsetWidth;
      el.classList.add('number-spring-pop');
    }
  }

  // Financial Calculations
  function updateCalculations() {
    const subtotal = computeSubtotalUi();
    const tax = state.tax_rate ? subtotal * (state.tax_rate / 100.0) : 0.0;
    const discount = state.discount_amount || 0.0;
    const rawTotal = subtotal + tax - discount;
    const grandTotal = allowsNegativeItems(state.doc_type) ? rawTotal : Math.max(0, rawTotal);

    animateNumberChange(subtotalDisplay, formatAmountUi(subtotal));
    animateNumberChange(taxDisplay, formatAmountUi(tax));
    animateNumberChange(discountDisplay, `-${formatAmountUi(discount)}`);
    animateNumberChange(totalDisplay, formatAmountUi(grandTotal));

    // Dynamic Amount Paid & Balance Due logic
    const amountPaidRow = document.getElementById('amountPaidRow');
    const amountPaidDisplay = document.getElementById('amountPaidDisplay');
    const balanceDueRow = document.getElementById('balanceDueRow');
    const balanceDueDisplay = document.getElementById('balanceDueDisplay');
    const balanceDueLabel = document.getElementById('balanceDueLabel');

    const isPaid = state.is_paid || state.payment_status === 'paid' || state.doc_type === 'receipt';
    const isPartial = state.payment_status === 'partial' || (state.amount_paid !== undefined && state.amount_paid !== null && state.amount_paid > 0 && !isPaid);

    if (isPaid) {
      const paidAmt = state.amount_paid !== undefined && state.amount_paid !== null ? state.amount_paid : grandTotal;
      if (amountPaidRow) amountPaidRow.style.display = 'flex';
      if (amountPaidDisplay) animateNumberChange(amountPaidDisplay, formatAmountUi(paidAmt));
      if (balanceDueRow) balanceDueRow.style.display = 'flex';
      if (balanceDueDisplay) {
        animateNumberChange(balanceDueDisplay, formatAmountUi(0.0));
        balanceDueDisplay.style.color = '#10b981';
      }
      if (balanceDueLabel) balanceDueLabel.innerText = 'Balance Due (PAID)';
    } else if (isPartial) {
      const paidAmt = state.amount_paid || 0.0;
      const remainingBalance = Math.max(0, grandTotal - paidAmt);
      if (amountPaidRow) amountPaidRow.style.display = 'flex';
      if (amountPaidDisplay) animateNumberChange(amountPaidDisplay, formatAmountUi(paidAmt));
      if (balanceDueRow) balanceDueRow.style.display = 'flex';
      if (balanceDueDisplay) {
        animateNumberChange(balanceDueDisplay, formatAmountUi(remainingBalance));
        balanceDueDisplay.style.color = remainingBalance === 0 ? '#10b981' : '#ef4444';
      }
      if (balanceDueLabel) balanceDueLabel.innerText = remainingBalance === 0 ? 'Balance Due (PAID)' : 'Remaining Balance';
    } else {
      if (amountPaidRow) amountPaidRow.style.display = 'none';
      if (balanceDueRow) balanceDueRow.style.display = 'none';
    }

    // Synchronize Studio Breadcrumbs
    const crumbCategory = document.getElementById('crumbCategory');
    const crumbDocName = document.getElementById('crumbDocName');
    const crumbDocNumber = document.getElementById('crumbDocNumber');
    if (crumbCategory) {
      crumbCategory.innerText = (currentTaxonomyCat || state.category || 'Commercial Invoicing').replace(/_/g, ' ');
    }
    if (crumbDocName) {
      crumbDocName.innerText = state.title || currentTaxonomyName || state.document_type || 'Tax Invoice';
    }
    if (crumbDocNumber) {
      crumbDocNumber.innerText = '#' + (state.doc_number || state.document_number || 'INV-2026-0001');
    }
  }

  // Debounced Live Preview
  function schedulePreview() {
    if (dataMode === 'my_data') {
      autoSaveUserDocument();
    }
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(requestPreview, 220);
  }

  async function requestPreview() {
    triggerPreviewCompilingAnimation();

    try {
      const reqHeaders = { 'Content-Type': 'application/json' };
      if (currentChatGptSession && currentChatGptSession.session_id) {
        reqHeaders['Authorization'] = 'Bearer ' + currentChatGptSession.session_id;
      }
      const resp = await fetch('/api/v1/preview', {
        method: 'POST',
        headers: reqHeaders,
        body: JSON.stringify(state)
      });
      const data = await resp.json();
      if (data.success && (data.image_data_uri || (data.pages && data.pages.length > 0))) {
        currentPages = (data.pages && data.pages.length > 0) ? data.pages : [data.image_data_uri];
        currentTotalPages = data.total_pages || currentPages.length || 1;
        if (currentPageIndex >= currentTotalPages) currentPageIndex = 0;

        renderMultiPagePreview();

        compileTelemetry.classList.remove('pulse');
        const computeTag = (currentChatGptSession && currentChatGptSession.user && currentChatGptSession.user.is_subscriber)
          ? ` · <span style="color:#34d399">⚡ ${currentChatGptSession.user.plan_display} Compute</span>`
          : '';
        const pageCountTag = currentTotalPages > 1 ? ` · ${currentTotalPages} Pages` : '';
        compileTelemetry.innerHTML = `⚡ <strong>${data.compile_time_ms}ms</strong> · Pressmark Vector Engine${pageCountTag}${computeTag}`;

        document.querySelectorAll('.preview-page-frame').forEach(f => {
          f.classList.remove('compiling-pulse', 'is-compiling', 'layout-morph-exit');
        });
        const wrapper = document.getElementById('previewDocWrapper');
        if (wrapper) {
          wrapper.classList.add('preview-updated-flash');
          setTimeout(() => wrapper.classList.remove('preview-updated-flash'), 500);
        }
      } else {
        compileTelemetry.classList.remove('pulse');
        compileTelemetry.innerHTML = `<span style="color:#f87171">⚠️ ${data.error || 'Vector synthesis notice'}</span>`;
        document.querySelectorAll('.preview-page-frame').forEach(f => {
          f.classList.remove('compiling-pulse', 'is-compiling', 'layout-morph-exit');
        });
      }
    } catch (err) {
      compileTelemetry.classList.remove('pulse');
      compileTelemetry.innerHTML = `<span style="color:#f87171">⚠️ Engine connection notice</span>`;
      document.querySelectorAll('.preview-page-frame').forEach(f => {
        f.classList.remove('compiling-pulse', 'is-compiling', 'layout-morph-exit');
      });
      console.error(err);
    }
  }

  // Download PDF
  async function downloadPdf() {
    const btn = document.getElementById('btnDownloadPdf');
    const originalText = btn.innerHTML;
    btn.innerHTML = 'Synthesizing...';
    btn.disabled = true;

    try {
      const reqHeaders = { 'Content-Type': 'application/json' };
      if (currentChatGptSession && currentChatGptSession.session_id) {
        reqHeaders['Authorization'] = 'Bearer ' + currentChatGptSession.session_id;
      }
      const resp = await fetch('/api/v1/generate?format=binary', {
        method: 'POST',
        headers: reqHeaders,
        body: JSON.stringify(state)
      });
      if (resp.ok) {
        const isSubscriber = resp.headers.get('X-ChatGPT-Subscriber') === 'true';
        if (isSubscriber) {
          showAiToast('⚡ Exported unwatermarked vector PDF via your ChatGPT subscription!');
        }
        const blob = await resp.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `${state.doc_number || 'document'}.pdf`;
        document.body.appendChild(a);
        a.click();
        window.URL.revokeObjectURL(url);
        a.remove();
        btn.classList.add('export-celebration');
        setTimeout(() => btn.classList.remove('export-celebration'), 800);
      } else {
        alert('Failed to generate PDF document');
      }
    } catch (e) {
      console.error(e);
      alert('Error connecting to Pressmark backend');
    } finally {
      btn.innerHTML = originalText;
      btn.disabled = false;
    }
  }

  // Print Document
  function printDocument() {
    const win = window.open('', '_blank');
    win.document.write(`
      <html>
        <head><title>Print ${state.doc_number}</title></head>
        <body style="margin: 0; display: flex; justify-content: center; align-items: center; background: white;">
          <img src="${previewImg.src}" style="max-width: 100%; height: auto;" onload="window.print(); window.close();" />
        </body>
      </html>
    `);
    win.document.close();
  }

  // Modals & Photo Extractor
  function setupModals() {
    const pricingModal = document.getElementById('pricingModal');
    const gptModal = document.getElementById('gptModal');
    const scanModal = document.getElementById('scanModal');
    const jsonModal = document.getElementById('jsonModal');

    document.getElementById('btnOpenPricing').addEventListener('click', () => pricingModal.style.display = 'flex');
    document.getElementById('btnClosePricing').addEventListener('click', () => pricingModal.style.display = 'none');
    if (window.location.pathname === '/pricing' || window.location.search.includes('pricing=1')) {
      pricingModal.style.display = 'flex';
    }

    document.getElementById('btnOpenGpt').addEventListener('click', () => gptModal.style.display = 'flex');
    document.getElementById('btnCloseGpt').addEventListener('click', () => gptModal.style.display = 'none');

    document.getElementById('btnOpenScan').addEventListener('click', () => scanModal.style.display = 'flex');
    document.getElementById('btnCloseScan').addEventListener('click', () => scanModal.style.display = 'none');

    document.getElementById('btnCopyJson').addEventListener('click', () => {
      document.getElementById('jsonTextarea').value = JSON.stringify(state, null, 2);
      jsonModal.style.display = 'flex';
    });
    document.getElementById('btnCloseJson').addEventListener('click', () => jsonModal.style.display = 'none');

    document.getElementById('btnApplyJson').addEventListener('click', () => {
      try {
        const parsed = JSON.parse(document.getElementById('jsonTextarea').value);
        pushUndo('Apply JSON State');
        Object.assign(state, parsed);
        syncFormFromState();
        renderLineItemsTable();
        renderSectionsList();
        renderSignaturesList();
        renderCustomFieldsList();
        updateCalculations();
        renderInteractiveOverlay();
        schedulePreview();
        jsonModal.style.display = 'none';
        showUndoToast('Applied JSON document configuration · [↩ Undo]');
      } catch (err) {
        alert('Invalid JSON: ' + err.message);
      }
    });

    window.addEventListener('click', (e) => {
      if (e.target === pricingModal) pricingModal.style.display = 'none';
      if (e.target === gptModal) gptModal.style.display = 'none';
      if (e.target === scanModal) scanModal.style.display = 'none';
      if (e.target === jsonModal) jsonModal.style.display = 'none';
    });

    window.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        if (pricingModal && pricingModal.style.display === 'flex') pricingModal.style.display = 'none';
        if (gptModal && gptModal.style.display === 'flex') gptModal.style.display = 'none';
        if (scanModal && scanModal.style.display === 'flex') scanModal.style.display = 'none';
        if (jsonModal && jsonModal.style.display === 'flex') jsonModal.style.display = 'none';
        const authModal = document.getElementById('modalAuth');
        if (authModal && authModal.style.display === 'flex') authModal.style.display = 'none';
      }
    });

    // Copy Prompt Button
    document.getElementById('btnCopyPrompt').addEventListener('click', () => {
      const promptText = `Please generate an invoice for client "${state.to.name}" from "${state.from.name}" with total ${totalDisplay.innerText} using the Pressmark plugin.`;
      navigator.clipboard.writeText(promptText);
      alert('ChatGPT Action prompt copied to clipboard!');
    });

    // Photo Dropzone
    const dropzone = document.getElementById('photoDropzone');
    const photoInput = document.getElementById('photoInput');

    dropzone.addEventListener('click', () => photoInput.click());
    dropzone.addEventListener('dragover', (e) => { e.preventDefault(); dropzone.classList.add('dragover'); });
    dropzone.addEventListener('dragleave', () => dropzone.classList.remove('dragover'));
    dropzone.addEventListener('drop', (e) => {
      e.preventDefault();
      dropzone.classList.remove('dragover');
      if (e.dataTransfer.files.length) handleScannedFile(e.dataTransfer.files[0]);
    });
    photoInput.addEventListener('change', (e) => {
      if (e.target.files.length) handleScannedFile(e.target.files[0]);
    });
  }

  // Handle Scanned Photo / Receipt via /api/v1/vision-extract
  function handleScannedFile(file) {
    const dropzone = document.getElementById('photoDropzone');
    dropzone.innerHTML = `<div style="font-size: 13px; font-weight: bold; color: #818cf8;">🔍 Scanning document structure & extracting items...</div>`;

    const resetDropzone = () => {
      document.getElementById('scanModal').style.display = 'none';
      dropzone.innerHTML = `
        <div style="font-size: 28px; margin-bottom: 8px;">📷</div>
        <div style="font-size: 13px; font-weight: 600;">Drop any receipt, invoice photo, or document image here</div>
        <div style="font-size: 11px; color: var(--text-muted); margin-top: 4px;">Supports PNG, JPG, HEIC, WebP</div>
      `;
    };

    const reader = new FileReader();
    reader.onload = async (ev) => {
      try {
        const res = await fetch('/api/v1/vision-extract', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            image_data: ev.target.result || '',
            filename: file ? file.name : undefined
          })
        });
        const data = await res.json();
        pushUndo('Photo Scan Extraction');
        if (data && data.extracted_from) {
          state.from = Object.assign({}, state.from, data.extracted_from);
        }
        if (data && Array.isArray(data.extracted_items) && data.extracted_items.length > 0) {
          state.items = data.extracted_items.map(it => ({
            description: it.description || 'Extracted Item',
            quantity: Number(it.quantity) || 1,
            unit_price: Number(it.unit_price) || 0,
            discount: Number(it.discount) || 0
          }));
        }
        state.doc_type = 'receipt';
        state.title = 'OFFICIAL ITEMIZED RECEIPT';
        state.doc_number = `REC-2026-${Math.floor(1000 + Math.random() * 9000)}`;
        state.is_paid = true;
        state.show_qr_seal = true;

        syncFormFromState();
        renderLineItemsTable();
        renderSectionsList();
        renderSignaturesList();
        renderCustomFieldsList();
        updateCalculations();
        renderInteractiveOverlay();
        schedulePreview();
        resetDropzone();
        showUndoToast((data && data.ai_message ? data.ai_message : 'Photo extracted into Itemized Receipt') + ' · [↩ Undo]');
      } catch (_err) {
        resetDropzone();
        showAiToast('Could not extract image details. Please try another file.');
      }
    };
    reader.readAsDataURL(file);
  }

  // Run on page ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
