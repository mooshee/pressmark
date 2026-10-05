// Pressmark Vector Document Engine: Strongly Typed Fluent Document Builder
// Author: Daniel Hallman

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::branding::DocumentBrandingConfig;
use crate::engine::{compile_pdf_bytes, compile_png_bytes};

/// Safely escape text for inclusion inside string literals `"..."`.
pub fn escape_typst_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => {},
            _ => out.push(c),
        }
    }
    out
}

/// Safely escape plain text for inclusion inside content blocks `[...]`.
pub fn escape_typst_content(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '[' => out.push_str("\\["),
            ']' => out.push_str("\\]"),
            '#' => out.push_str("\\#"),
            '$' => out.push_str("\\$"),
            '@' => out.push_str("\\@"),
            _ => out.push(c),
        }
    }
    out
}

/// Three-way execution mode state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionMode {
    /// Traditional physical wet-ink execution with guidelines and clerk date-stamp box
    WetInk,
    /// Digital electronic signature with 15 U.S.C. § 7001 (ESIGN Act) audit compliance badge
    Esign,
    /// Remote Online Notary (RON) with statutory recital and 1.0-inch seal clearance box
    NotarizedRon,
}

impl Default for ExecutionMode {
    fn default() -> Self {
        Self::Esign
    }
}

/// Masthead configuration
#[derive(Debug, Clone)]
pub struct MastheadConfig {
    pub jurisdiction: String,
    pub document_title: String,
    pub subtitle: String,
    pub docket: Option<String>,
    pub reference_id: String,
    pub date_str: String,
}

impl Default for MastheadConfig {
    fn default() -> Self {
        Self {
            jurisdiction: "OFFICIAL COMMERCIAL & STATUTORY JURISDICTION".to_string(),
            document_title: "COMMERCIAL TRANSACTION PETITION".to_string(),
            subtitle: "Certified Vector Document · Sovereign Commercial Record".to_string(),
            docket: None,
            reference_id: "REF-2026-001".to_string(),
            date_str: "2026-10-01".to_string(),
        }
    }
}

/// Statutory / Regulatory authority callout box
#[derive(Debug, Clone)]
pub struct StatutoryBoxConfig {
    pub citation: String,
    pub burden: String,
    pub body: String,
}

/// Financial line delta or tax breakdown
#[derive(Debug, Clone)]
pub struct TaxLevyDelta {
    pub jurisdiction: String,
    pub millage_rate: String,
    pub billed_tax: String,
    pub contended_tax: String,
    pub net_savings: String,
}

/// Financial valuation and tax liability delta grid
#[derive(Debug, Clone)]
pub struct FinancialDeltaGridConfig {
    pub current_appraised: String,
    pub proposed_appraised: String,
    pub current_assessed: String,
    pub proposed_assessed: String,
    pub net_appraised_reduction: String,
    pub net_assessed_reduction: String,
    pub annual_savings: String,
    pub ratio: String,
    pub levy_breakdown: Vec<TaxLevyDelta>,
}

/// Jurat and execution block configuration
#[derive(Debug, Clone)]
pub struct JuratConfig {
    pub heading: String,
    pub attestation: String,
    pub notary_required: bool,
    pub pro_se: bool,
    pub signer_title: String,
    pub signer_capacity: String,
    pub seal_box: bool,
    pub cross_reference: String,
    pub signature_type: ExecutionMode,
    pub signer_name: String,
    pub signature_img: Option<String>,
    pub execution_date: String,
    pub esign_audit_id: String,
    pub esign_timestamp: String,
    pub esign_ip: String,
    pub ron_statute_citation: String,
    pub ron_session_id: String,
    pub include_clerk_stamp: bool,
    pub state_name: String,
    pub county_name: String,
}

impl Default for JuratConfig {
    fn default() -> Self {
        Self {
            heading: "EXECUTION & SWORN ATTESTATION".to_string(),
            attestation: "I hereby attest under penalty of perjury that the statements, figures, and attached schedules herein are true, correct, and complete to the best of my knowledge and belief.".to_string(),
            notary_required: false,
            pro_se: true,
            signer_title: "Authorized Representative / Managing Principal".to_string(),
            signer_capacity: "Managing Principal".to_string(),
            seal_box: false,
            cross_reference: String::new(),
            signature_type: ExecutionMode::Esign,
            signer_name: "Daniel Hallman".to_string(),
            signature_img: None,
            execution_date: "10/01/2026".to_string(),
            esign_audit_id: "PRESSMARK-ESIGN-8492-7A1B".to_string(),
            esign_timestamp: "2026-10-01T02:00:00Z".to_string(),
            esign_ip: "192.0.2.14".to_string(),
            ron_statute_citation: "T.C.A. § 8-16-301 et seq.".to_string(),
            ron_session_id: String::new(),
            include_clerk_stamp: false,
            state_name: "Tennessee".to_string(),
            county_name: "Hamilton".to_string(),
        }
    }
}

/// Cryptographic verification slug configuration
#[derive(Debug, Clone)]
pub struct AuditSlugConfig {
    pub reference_id: String,
    pub hash: String,
    pub engine_version: String,
    pub timestamp: String,
}

impl Default for AuditSlugConfig {
    fn default() -> Self {
        Self {
            reference_id: "DOC-2026-0042".to_string(),
            hash: "c4ca4238a0b923820dcc509a6f75849b84927a1b8c8d9e0f1a2b3c4d5e6f7a8b".to_string(),
            engine_version: "Pressmark Vector Engine v1.0.0".to_string(),
            timestamp: "2026-10-01T02:00:00Z".to_string(),
        }
    }
}

/// Walk-in filing reception box configuration
#[derive(Debug, Clone)]
pub struct WalkInBoxConfig {
    pub facility_title: String,
    pub room_window: String,
    pub physical_address: String,
    pub hours: String,
    pub clerk_phone: String,
}

impl Default for WalkInBoxConfig {
    fn default() -> Self {
        Self {
            facility_title: "COUNTY ASSESSOR / RECORDING DIVISION".to_string(),
            room_window: "Customer Counter · Service Window #3".to_string(),
            physical_address: "6135 Heritage Park Dr, Chattanooga, TN 37416".to_string(),
            hours: "Monday – Friday · 8:00 AM – 4:00 PM".to_string(),
            clerk_phone: "(423) 209-7300".to_string(),
        }
    }
}

/// Submission checklist for physical filing
#[derive(Debug, Clone)]
pub struct ClerkChecklistConfig {
    pub title: String,
    pub items: Vec<String>,
}

/// Built document product
#[derive(Debug, Clone)]
pub struct BuiltDocument {
    pub markup: String,
}

/// Fluent type-safe builder for Pressmark vector documents
#[derive(Debug, Clone)]
pub struct PressmarkDocumentBuilder {
    title: String,
    subtitle: String,
    jurisdiction: String,
    docket: Option<String>,
    reference_id: String,
    tax_year: String,
    branding: DocumentBrandingConfig,
    recordable: bool,
    elements: Vec<String>,
}

impl Default for PressmarkDocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PressmarkDocumentBuilder {
    pub fn new() -> Self {
        Self {
            title: "COMMERCIAL INSTRUMENT".to_string(),
            subtitle: String::new(),
            jurisdiction: "OFFICIAL COMMERCIAL & STATUTORY JURISDICTION".to_string(),
            docket: None,
            reference_id: String::new(),
            tax_year: "2026".to_string(),
            branding: DocumentBrandingConfig::default(),
            recordable: false,
            elements: Vec::new(),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = subtitle.to_string();
        self
    }

    pub fn jurisdiction(mut self, jurisdiction: &str) -> Self {
        self.jurisdiction = jurisdiction.to_string();
        self
    }

    pub fn docket(mut self, docket: &str) -> Self {
        self.docket = Some(docket.to_string());
        self
    }

    pub fn reference_id(mut self, reference_id: &str) -> Self {
        self.reference_id = reference_id.to_string();
        self
    }

    pub fn tax_year(mut self, tax_year: &str) -> Self {
        self.tax_year = tax_year.to_string();
        self
    }

    pub fn branding(mut self, branding: DocumentBrandingConfig) -> Self {
        self.branding = branding;
        self
    }

    pub fn recordable(mut self, recordable: bool) -> Self {
        self.recordable = recordable;
        self
    }

    pub fn add_raw(mut self, markup_snippet: &str) -> Self {
        self.elements.push(markup_snippet.to_string());
        self
    }

    pub fn add_page_break(mut self) -> Self {
        self.elements.push("#pagebreak()\n".to_string());
        self
    }

    pub fn add_masthead(mut self, config: MastheadConfig) -> Self {
        let docket_val = match config.docket {
            Some(d) => format!("\"{}\"", escape_typst_string(&d)),
            None => "none".to_string(),
        };
        let snippet = format!(
            "#masthead(\n  jurisdiction: \"{}\",\n  document-title: \"{}\",\n  subtitle: \"{}\",\n  docket: {},\n  parcel-id: \"{}\",\n  tax-year: \"{}\",\n  branding: (\n    org-name: \"{}\",\n    accent-primary: rgb(\"{}\")\n  )\n)\n",
            escape_typst_string(&config.jurisdiction),
            escape_typst_string(&config.document_title),
            escape_typst_string(&config.subtitle),
            docket_val,
            escape_typst_string(&config.reference_id),
            escape_typst_string(&config.date_str),
            escape_typst_string(&self.branding.org_name),
            self.branding.accent_primary
        );
        self.elements.push(snippet);
        self
    }

    pub fn add_statutory_box(mut self, citation: &str, burden: &str, body: &str) -> Self {
        let snippet = format!(
            "#statutory-box(\n  citation: \"{}\",\n  burden: \"{}\",\n  [\n    #text(font: font-sans, size: 8pt, fill: ink-secondary)[\n      {}\n    ]\n  ]\n)\n",
            escape_typst_string(citation),
            escape_typst_string(burden),
            escape_typst_content(body)
        );
        self.elements.push(snippet);
        self
    }

    pub fn add_financial_delta_grid(mut self, config: FinancialDeltaGridConfig) -> Self {
        let mut breakdown_code = String::new();
        if !config.levy_breakdown.is_empty() {
            breakdown_code.push_str("  levy-breakdown: (\n");
            for item in config.levy_breakdown {
                breakdown_code.push_str(&format!(
                    "    (jurisdiction: \"{}\", millage-rate: \"{}\", billed-tax: \"{}\", contended-tax: \"{}\", net-savings: \"{}\"),\n",
                    escape_typst_string(&item.jurisdiction),
                    escape_typst_string(&item.millage_rate),
                    escape_typst_string(&item.billed_tax),
                    escape_typst_string(&item.contended_tax),
                    escape_typst_string(&item.net_savings)
                ));
            }
            breakdown_code.push_str("  ),\n");
        }

        let snippet = format!(
            "#financial-delta-grid(\n  current-appraised: \"{}\",\n  proposed-appraised: \"{}\",\n  current-assessed: \"{}\",\n  proposed-assessed: \"{}\",\n  net-appraised-reduction: \"{}\",\n  net-assessed-reduction: \"{}\",\n  annual-savings: \"{}\",\n  ratio: \"{}\",\n{}\n)\n",
            escape_typst_string(&config.current_appraised),
            escape_typst_string(&config.proposed_appraised),
            escape_typst_string(&config.current_assessed),
            escape_typst_string(&config.proposed_assessed),
            escape_typst_string(&config.net_appraised_reduction),
            escape_typst_string(&config.net_assessed_reduction),
            escape_typst_string(&config.annual_savings),
            escape_typst_string(&config.ratio),
            breakdown_code
        );
        self.elements.push(snippet);
        self
    }

    pub fn add_walk_in_box(mut self, config: WalkInBoxConfig) -> Self {
        let snippet = format!(
            "#walk-in-box(\n  facility-title: \"{}\",\n  room-window: \"{}\",\n  physical-address: \"{}\",\n  hours: \"{}\",\n  clerk-phone: \"{}\"\n)\n",
            escape_typst_string(&config.facility_title),
            escape_typst_string(&config.room_window),
            escape_typst_string(&config.physical_address),
            escape_typst_string(&config.hours),
            escape_typst_string(&config.clerk_phone)
        );
        self.elements.push(snippet);
        self
    }

    pub fn add_clerk_checklist(mut self, config: ClerkChecklistConfig) -> Self {
        let mut items_code = String::new();
        for item in config.items {
            items_code.push_str(&format!("  \"{}\",\n", escape_typst_string(&item)));
        }
        let snippet = format!(
            "#clerk-checklist(\n  title: \"{}\",\n  items: (\n{}\n  )\n)\n",
            escape_typst_string(&config.title),
            items_code
        );
        self.elements.push(snippet);
        self
    }

    pub fn add_jurat(mut self, config: JuratConfig) -> Self {
        let sig_type_str = match config.signature_type {
            ExecutionMode::WetInk => "wet-ink",
            ExecutionMode::Esign => "esign",
            ExecutionMode::NotarizedRon => "ron",
        };
        let sig_img_str = match config.signature_img {
            Some(ref img) => format!("\"{}\"", escape_typst_string(img)),
            None => "none".to_string(),
        };

        let snippet = format!(
            "#jurat-box(\n  heading: \"{}\",\n  attestation: \"{}\",\n  notary-required: {},\n  pro-se: {},\n  signer-title: \"{}\",\n  signer-capacity: \"{}\",\n  seal-box: {},\n  cross-reference: \"{}\",\n  signature-type: \"{}\",\n  signer-name: \"{}\",\n  signature-img: {},\n  execution-date: \"{}\",\n  esign-audit-id: \"{}\",\n  esign-timestamp: \"{}\",\n  esign-ip: \"{}\",\n  ron-statute-citation: \"{}\",\n  ron-session-id: \"{}\",\n  include-clerk-stamp: {},\n  state: \"{}\",\n  county: \"{}\"\n)\n",
            escape_typst_string(&config.heading),
            escape_typst_string(&config.attestation),
            config.notary_required,
            config.pro_se,
            escape_typst_string(&config.signer_title),
            escape_typst_string(&config.signer_capacity),
            config.seal_box,
            escape_typst_string(&config.cross_reference),
            sig_type_str,
            escape_typst_string(&config.signer_name),
            sig_img_str,
            escape_typst_string(&config.execution_date),
            escape_typst_string(&config.esign_audit_id),
            escape_typst_string(&config.esign_timestamp),
            escape_typst_string(&config.esign_ip),
            escape_typst_string(&config.ron_statute_citation),
            escape_typst_string(&config.ron_session_id),
            config.include_clerk_stamp,
            escape_typst_string(&config.state_name),
            escape_typst_string(&config.county_name)
        );
        self.elements.push(snippet);
        self
    }

    pub fn add_audit_slug(mut self, config: AuditSlugConfig) -> Self {
        let snippet = format!(
            "#audit-slug(\n  parcel-id: \"{}\",\n  hash: \"{}\",\n  engine-version: \"{}\",\n  timestamp: \"{}\"\n)\n",
            escape_typst_string(&config.reference_id),
            escape_typst_string(&config.hash),
            escape_typst_string(&config.engine_version),
            escape_typst_string(&config.timestamp)
        );
        self.elements.push(snippet);
        self
    }

    /// Assemble full vector markup
    pub fn build(&self) -> BuiltDocument {
        let mut markup = String::new();

        // 1. Embed theme primitives
        markup.push_str(crate::templates::TYPST_THEME);
        markup.push_str("\n\n");

        // 2. Wrap inside document-layout
        let docket_val = match self.docket {
            Some(ref d) => format!("\"{}\"", escape_typst_string(d)),
            None => "\"\"".to_string(),
        };

        markup.push_str(&format!(
            "#show: doc => document-layout(\n  title: \"{}\",\n  subtitle: \"{}\",\n  jurisdiction: \"{}\",\n  docket: {},\n  parcel-id: \"{}\",\n  tax-year: \"{}\",\n  recordable: {},\n  branding: (\n    org-name: \"{}\",\n    header-left: \"{}\",\n    footer-left: \"{}\",\n    footer-center: \"{}\",\n    disclaimer: \"{}\",\n    accent-primary: rgb(\"{}\")\n  ),\n  doc\n)\n\n",
            escape_typst_string(&self.title),
            escape_typst_string(&self.subtitle),
            escape_typst_string(&self.jurisdiction),
            docket_val,
            escape_typst_string(&self.reference_id),
            escape_typst_string(&self.tax_year),
            self.recordable,
            escape_typst_string(&self.branding.org_name),
            escape_typst_string(self.branding.header_left.as_deref().unwrap_or("")),
            escape_typst_string(self.branding.footer_left.as_deref().unwrap_or("")),
            escape_typst_string(self.branding.footer_center.as_deref().unwrap_or("")),
            escape_typst_string(self.branding.disclaimer.as_deref().unwrap_or("")),
            self.branding.accent_primary
        ));

        // 3. Append elements
        for el in &self.elements {
            markup.push_str(el);
            markup.push_str("\n");
        }

        BuiltDocument { markup }
    }

    /// Compile document directly to PDF bytes in-memory
    pub fn compile_pdf_bytes(&self) -> Result<Vec<u8>, String> {
        let doc = self.build();
        compile_pdf_bytes(&doc.markup)
    }

    /// Compile document to a PDF file on disk
    pub fn compile_pdf(&self, target_path: &Path) -> Result<PathBuf, String> {
        let bytes = self.compile_pdf_bytes()?;
        if let Some(parent) = target_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(target_path, bytes)
            .map_err(|e| format!("Failed to write PDF file to {:?}: {}", target_path, e))?;
        Ok(target_path.to_path_buf())
    }

    /// Compile document to high-resolution PNG raster preview
    pub fn compile_png_bytes(&self, ppi: u32) -> Result<Vec<u8>, String> {
        let doc = self.build();
        compile_png_bytes(&doc.markup, ppi)
    }
}
