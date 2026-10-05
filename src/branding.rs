// Pressmark Vector Document Engine: Multi-Tenant White-Labeling System
// Author: Daniel Hallman

use serde::{Deserialize, Serialize};

/// Operational branding mode for multi-tenant and white-label document rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrandingMode {
    /// Standard Pressmark vector synthesis branding
    PressmarkDefault,
    /// Corporate Enterprise white-label (removes vendor marks, applies custom tenant palette)
    CorporateWhiteLabel,
    /// Formal legal counsel & sworn statutory filing persona
    LegalCounsel,
    /// High-contrast, minimal government & institutional submission mode
    GovernmentInstitutional,
}

impl Default for BrandingMode {
    fn default() -> Self {
        Self::PressmarkDefault
    }
}

/// Comprehensive document branding configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentBrandingConfig {
    pub mode: BrandingMode,
    pub org_name: String,
    pub subtitle: Option<String>,
    pub accent_primary: String,
    pub accent_secondary: Option<String>,
    pub primary_ink: Option<String>,
    pub header_left: Option<String>,
    pub footer_left: Option<String>,
    pub footer_center: Option<String>,
    pub disclaimer: Option<String>,
    pub show_engine_badge: bool,
}

impl Default for DocumentBrandingConfig {
    fn default() -> Self {
        Self::pressmark_default()
    }
}

impl DocumentBrandingConfig {
    /// Standard Pressmark vector engine configuration
    pub fn pressmark_default() -> Self {
        Self {
            mode: BrandingMode::PressmarkDefault,
            org_name: "PRESSMARK STUDIO".to_string(),
            subtitle: Some("High-Performance Vector Document Synthesis".to_string()),
            accent_primary: "#4f46e5".to_string(),
            accent_secondary: Some("#818cf8".to_string()),
            primary_ink: Some("#0f172a".to_string()),
            header_left: Some("PRESSMARK · OFFICIAL VECTOR INSTRUMENT".to_string()),
            footer_left: Some("Pressmark Vector Engine · Sub-Millisecond Synthesis".to_string()),
            footer_center: Some("Confidential & Certified Record".to_string()),
            disclaimer: Some("Synthesized with Pressmark Vector Engine. Cryptographically verifiable record.".to_string()),
            show_engine_badge: true,
        }
    }

    /// Corporate enterprise white-label persona
    pub fn corporate_white_label(org_name: &str, subtitle: Option<&str>) -> Self {
        Self {
            mode: BrandingMode::CorporateWhiteLabel,
            org_name: org_name.to_string(),
            subtitle: subtitle.map(|s| s.to_string()),
            accent_primary: "#1e3a8a".to_string(),
            accent_secondary: Some("#3b82f6".to_string()),
            primary_ink: Some("#0f172a".to_string()),
            header_left: Some(format!("{} · COMMERCIAL DOCUMENT", org_name.to_uppercase())),
            footer_left: Some(org_name.to_string()),
            footer_center: Some("Authorized Corporate Instrument".to_string()),
            disclaimer: Some("All rights reserved. Proprietary and confidential commercial document.".to_string()),
            show_engine_badge: false,
        }
    }

    /// Formal legal counsel and attorney filing persona
    pub fn legal_counsel(firm_name: &str, practice_group: Option<&str>, bar_number: Option<&str>) -> Self {
        let sub = match (practice_group, bar_number) {
            (Some(pg), Some(bar)) => format!("{} · {}", pg, bar),
            (Some(pg), None) => pg.to_string(),
            (None, Some(bar)) => bar.to_string(),
            (None, None) => "Attorneys at Law".to_string(),
        };
        Self {
            mode: BrandingMode::LegalCounsel,
            org_name: firm_name.to_string(),
            subtitle: Some(sub),
            accent_primary: "#1e3a8a".to_string(),
            accent_secondary: Some("#64748b".to_string()),
            primary_ink: Some("#020617".to_string()),
            header_left: Some(format!("{} · LEGAL INSTRUMENT", firm_name.to_uppercase())),
            footer_left: Some(format!("{} · Counsel of Record", firm_name)),
            footer_center: Some("Privileged & Confidential Work Product".to_string()),
            disclaimer: Some("Privileged & Confidential Legal Instrument. Prepared for submission to official tribunal or registry.".to_string()),
            show_engine_badge: false,
        }
    }

    /// Government and institutional filing persona (monochrome, zero marketing)
    pub fn government_institutional() -> Self {
        Self {
            mode: BrandingMode::GovernmentInstitutional,
            org_name: "OFFICIAL INSTRUMENT".to_string(),
            subtitle: Some("Statutory Administrative Filing".to_string()),
            accent_primary: "#000000".to_string(),
            accent_secondary: Some("#475569".to_string()),
            primary_ink: Some("#000000".to_string()),
            header_left: Some("OFFICIAL STATUTORY SUBMISSION".to_string()),
            footer_left: Some("Official Administrative Record".to_string()),
            footer_center: Some("Statutory Authority".to_string()),
            disclaimer: Some("OFFICIAL DOCUMENT PURSUANT TO APPLICABLE STATUTORY AND ADMINISTRATIVE REGULATIONS.".to_string()),
            show_engine_badge: false,
        }
    }

    pub fn with_accent_color(mut self, hex: &str) -> Self {
        self.accent_primary = hex.to_string();
        self
    }

    pub fn with_primary_ink(mut self, hex: &str) -> Self {
        self.primary_ink = Some(hex.to_string());
        self
    }

    pub fn with_disclaimer(mut self, disclaimer: &str) -> Self {
        self.disclaimer = Some(disclaimer.to_string());
        self
    }
}
