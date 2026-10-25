//! Pressmark Document Models & Calculation Helpers
//!
//! Author: Daniel Hallman

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    Invoice,
    InvoiceStandard,
    InvoiceFreelance,
    InvoiceConstruction,
    InvoiceMedical,
    InvoiceSaas,
    InvoiceMinimal,
    ProformaInvoice,
    Estimate,
    Receipt,
    PurchaseOrder,
    PackingSlip,
    CreditNote,
    WorkOrder,
    ExpenseReport,
    DeliveryReceipt,
    CommercialLease,
    QuotationTender,
    Agreement,
    AgreementSow,
    ContractorAgreement,
    RetainerAgreement,
    Nda,
    BillOfSale,
    Certificate,
    CertificateInsurance,
    CertificateCompletion,
    InspectionReport,
    // Real Estate & Property Management Suite
    ResidentialLease,
    CommercialNnnLease,
    PropertyManagementStatement,
    RentRoll,
    ClosingSettlementStatement,
    NoticeToVacate,
    SecurityDepositDisposition,
    RealEstateCommission,
    PropertyInspectionChecklist,
    LetterOfIntentCre,
    TenantEstoppel,
}

impl Default for DocumentType {
    fn default() -> Self {
        Self::Invoice
    }
}

impl DocumentType {
    pub fn display_title(&self) -> &'static str {
        match self {
            Self::Invoice | Self::InvoiceStandard => "TAX INVOICE",
            Self::InvoiceFreelance => "FREELANCE / CONSULTING INVOICE",
            Self::InvoiceConstruction => "PROGRESS / CONSTRUCTION BILLING",
            Self::InvoiceMedical => "CLINICAL SERVICES SUPERBILL",
            Self::InvoiceSaas => "SUBSCRIPTION & CLOUD BILLING",
            Self::InvoiceMinimal => "COMMERCIAL INVOICE",
            Self::ProformaInvoice => "PROFORMA INVOICE",
            Self::Estimate => "FORMAL ESTIMATE & QUOTE",
            Self::Receipt => "OFFICIAL PAYMENT RECEIPT",
            Self::PurchaseOrder => "PURCHASE ORDER",
            Self::PackingSlip => "PACKING SLIP & BILL OF LADING",
            Self::CreditNote => "CREDIT NOTE",
            Self::WorkOrder => "WORK ORDER & JOB TICKET",
            Self::ExpenseReport => "EXPENSE & DISBURSEMENT REPORT",
            Self::DeliveryReceipt => "DELIVERY & ACCEPTANCE RECEIPT",
            Self::CommercialLease => "COMMERCIAL LEASE ABSTRACT",
            Self::QuotationTender => "FORMAL TENDER & BID QUOTATION",
            Self::Agreement | Self::AgreementSow => "MASTER SERVICES AGREEMENT",
            Self::ContractorAgreement => "INDEPENDENT CONTRACTOR AGREEMENT",
            Self::RetainerAgreement => "STRATEGIC RETAINER AGREEMENT",
            Self::Nda => "MUTUAL NON-DISCLOSURE AGREEMENT",
            Self::BillOfSale => "STATUTORY BILL OF SALE",
            Self::Certificate => "CERTIFICATE OF ACHIEVEMENT",
            Self::CertificateInsurance => "CERTIFICATE OF LIABILITY INSURANCE",
            Self::CertificateCompletion => "CERTIFICATE OF COMPLETION & ACCEPTANCE",
            Self::InspectionReport => "CONDITION & INSPECTION REPORT",
            Self::ResidentialLease => "RESIDENTIAL LEASE AGREEMENT",
            Self::CommercialNnnLease => "TRIPLE NET (NNN) COMMERCIAL LEASE",
            Self::PropertyManagementStatement => "MONTHLY PROPERTY MANAGEMENT STATEMENT",
            Self::RentRoll => "MULTI-UNIT RENT ROLL & LEASE SCHEDULE",
            Self::ClosingSettlementStatement => "REAL ESTATE CLOSING SETTLEMENT STATEMENT",
            Self::NoticeToVacate => "NOTICE TO VACATE & CURE LEASE VIOLATION",
            Self::SecurityDepositDisposition => "SECURITY DEPOSIT ITEMIZED DISPOSITION",
            Self::RealEstateCommission => "REAL ESTATE COMMISSION & BROKERAGE FEE STATEMENT",
            Self::PropertyInspectionChecklist => "PROPERTY CONDITION & INSPECTION CHECKLIST",
            Self::LetterOfIntentCre => "COMMERCIAL REAL ESTATE LETTER OF INTENT (LOI)",
            Self::TenantEstoppel => "TENANT ESTOPPEL CERTIFICATE",
        }
    }

    pub fn default_number_prefix(&self) -> &'static str {
        match self {
            Self::Invoice | Self::InvoiceStandard => "INV-",
            Self::InvoiceFreelance => "FRL-",
            Self::InvoiceConstruction => "CST-",
            Self::InvoiceMedical => "MED-",
            Self::InvoiceSaas => "SUB-",
            Self::InvoiceMinimal => "DOC-",
            Self::ProformaInvoice => "PI-",
            Self::Estimate => "EST-",
            Self::Receipt => "RCT-",
            Self::PurchaseOrder => "PO-",
            Self::PackingSlip => "PKG-",
            Self::CreditNote => "CRD-",
            Self::WorkOrder => "WO-",
            Self::ExpenseReport => "EXP-",
            Self::DeliveryReceipt => "DEL-",
            Self::CommercialLease => "LSE-",
            Self::QuotationTender => "TND-",
            Self::Agreement | Self::AgreementSow => "MSA-",
            Self::ContractorAgreement => "ICA-",
            Self::RetainerAgreement => "RET-",
            Self::Nda => "NDA-",
            Self::BillOfSale => "BOS-",
            Self::Certificate => "CERT-",
            Self::CertificateInsurance => "COI-",
            Self::CertificateCompletion => "COC-",
            Self::InspectionReport => "INSP-",
            Self::ResidentialLease => "RLA-",
            Self::CommercialNnnLease => "NNN-",
            Self::PropertyManagementStatement => "PMS-",
            Self::RentRoll => "RRL-",
            Self::ClosingSettlementStatement => "ESC-",
            Self::NoticeToVacate => "NTV-",
            Self::SecurityDepositDisposition => "SDD-",
            Self::RealEstateCommission => "COM-",
            Self::PropertyInspectionChecklist => "CHK-",
            Self::LetterOfIntentCre => "LOI-",
            Self::TenantEstoppel => "TEC-",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
            Self::InvoiceStandard => "invoice_standard",
            Self::InvoiceFreelance => "invoice_freelance",
            Self::InvoiceConstruction => "invoice_construction",
            Self::InvoiceMedical => "invoice_medical",
            Self::InvoiceSaas => "invoice_saas",
            Self::InvoiceMinimal => "invoice_minimal",
            Self::ProformaInvoice => "proforma_invoice",
            Self::Estimate => "estimate",
            Self::Receipt => "receipt",
            Self::PurchaseOrder => "purchase_order",
            Self::PackingSlip => "packing_slip",
            Self::CreditNote => "credit_note",
            Self::WorkOrder => "work_order",
            Self::ExpenseReport => "expense_report",
            Self::DeliveryReceipt => "delivery_receipt",
            Self::CommercialLease => "commercial_lease",
            Self::QuotationTender => "quotation_tender",
            Self::Agreement => "agreement",
            Self::AgreementSow => "agreement_sow",
            Self::ContractorAgreement => "contractor_agreement",
            Self::RetainerAgreement => "retainer_agreement",
            Self::Nda => "nda",
            Self::BillOfSale => "bill_of_sale",
            Self::Certificate => "certificate",
            Self::CertificateInsurance => "certificate_insurance",
            Self::CertificateCompletion => "certificate_completion",
            Self::InspectionReport => "inspection_report",
            Self::ResidentialLease => "residential_lease",
            Self::CommercialNnnLease => "commercial_nnn_lease",
            Self::PropertyManagementStatement => "property_management_statement",
            Self::RentRoll => "rent_roll",
            Self::ClosingSettlementStatement => "closing_settlement_statement",
            Self::NoticeToVacate => "notice_to_vacate",
            Self::SecurityDepositDisposition => "security_deposit_disposition",
            Self::RealEstateCommission => "real_estate_commission",
            Self::PropertyInspectionChecklist => "property_inspection_checklist",
            Self::LetterOfIntentCre => "letter_of_intent_cre",
            Self::TenantEstoppel => "tenant_estoppel",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyDetails {
    pub name: String,
    pub company: Option<String>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tax_id: Option<String>,
    pub website: Option<String>,
    pub registration_number: Option<String>,
}

impl PartyDetails {
    pub fn format_multiline(&self) -> String {
        let mut lines = Vec::new();
        if let Some(ref c) = self.company {
            if !c.is_empty() {
                lines.push(c.clone());
            }
        }
        if let Some(ref a) = self.address {
            if !a.is_empty() {
                lines.push(a.clone());
            }
        }
        if let Some(ref e) = self.email {
            if !e.is_empty() {
                lines.push(format!("Email: {}", e));
            }
        }
        if let Some(ref p) = self.phone {
            if !p.is_empty() {
                lines.push(format!("Tel: {}", p));
            }
        }
        if let Some(ref t) = self.tax_id {
            if !t.is_empty() {
                lines.push(format!("Tax / TIN: {}", t));
            }
        }
        if let Some(ref w) = self.website {
            if !w.is_empty() {
                lines.push(format!("Web: {}", w));
            }
        }
        if let Some(ref r) = self.registration_number {
            if !r.is_empty() {
                lines.push(format!("Reg / Lic: {}", r));
            }
        }
        lines.join("\n")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LineItem {
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub tax_rate: Option<f64>,
    pub discount: Option<f64>,
}

impl LineItem {
    pub fn amount(&self) -> f64 {
        let gross = self.quantity * self.unit_price;
        let discount = self.discount.unwrap_or(0.0);
        (gross - discount).max(0.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomField {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignatureBlock {
    pub signer_name: String,
    pub signer_title: Option<String>,
    pub date: Option<String>,
    pub signature_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentSection {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentRequest {
    #[serde(default)]
    pub doc_type: DocumentType,
    pub title: Option<String>,
    pub doc_number: Option<String>,
    pub issue_date: Option<String>,
    pub due_date: Option<String>,
    pub theme: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub accent_color: Option<String>,
    pub surface_color: Option<String>,
    pub text_color: Option<String>,
    pub font_family: Option<String>,
    pub font_scale: Option<String>,
    pub paper_size: Option<String>,
    pub logo_data: Option<String>,
    pub signature_data: Option<String>,
    pub signer_name: Option<String>,
    pub signer_title: Option<String>,
    #[serde(default)]
    pub from: PartyDetails,
    #[serde(default)]
    pub to: PartyDetails,
    #[serde(default)]
    pub items: Vec<LineItem>,
    pub currency: Option<String>,
    pub tax_rate: Option<f64>,
    pub discount_amount: Option<f64>,
    pub notes: Option<String>,
    pub terms: Option<String>,
    #[serde(default)]
    pub custom_fields: Vec<CustomField>,
    #[serde(default)]
    pub signatures: Vec<SignatureBlock>,
    #[serde(default)]
    pub sections: Vec<DocumentSection>,
    pub header_subtitle: Option<String>,
    pub footer_text: Option<String>,
    pub background_style: Option<String>,
    pub show_page_numbers: Option<bool>,
    pub show_footer_divider: Option<bool>,
    pub is_paid: Option<bool>,
    pub status: Option<String>,
    pub watermark: Option<String>,
    pub compliance_mode: Option<String>,
    pub show_qr_seal: Option<bool>,
    pub show_signature: Option<bool>,
    pub show_remittance_stub: Option<bool>,
    pub show_line_numbers: Option<bool>,
    pub layout_style: Option<String>,
    pub spacing_density: Option<String>,
    pub execution_mode: Option<String>,
    pub branding_mode: Option<String>,
    pub audit_hash: Option<String>,
    pub include_clerk_stamp: Option<bool>,
    pub currency_symbol_mode: Option<String>,
    pub currency_placement: Option<String>,
    pub currency_separators: Option<String>,
    pub date_format: Option<String>,
    pub phone_format: Option<String>,
    pub number_decimals: Option<u32>,
    pub unit_measure: Option<String>,
    pub payment_link: Option<String>,
    pub payment_qr_code: Option<bool>,
    pub payment_status: Option<String>,
    pub payment_method: Option<String>,
    pub paid_at: Option<String>,
    pub transaction_reference: Option<String>,
    pub amount_paid: Option<f64>,
}

impl DocumentRequest {
    pub fn currency_symbol(&self) -> &str {
        match self.currency.as_deref() {
            Some("EUR") | Some("€") => "€",
            Some("GBP") | Some("£") => "£",
            Some("JPY") | Some("CNY") | Some("¥") => "¥",
            Some("CAD") => "CA$",
            Some("AUD") => "A$",
            Some("NZD") => "NZ$",
            Some("SGD") => "S$",
            Some("CHF") => "CHF",
            Some("INR") | Some("₹") => "₹",
            Some("BRL") | Some("R$") => "R$",
            Some("SEK") | Some("NOK") | Some("DKK") => "kr",
            Some("AED") => "AED",
            Some(s) if !s.is_empty() => s,
            _ => "$",
        }
    }

    pub fn subtotal(&self) -> f64 {
        self.items.iter().map(|item| item.amount()).sum()
    }

    pub fn tax_total(&self) -> f64 {
        let sub = self.subtotal();
        if let Some(rate) = self.tax_rate {
            sub * (rate / 100.0)
        } else {
            0.0
        }
    }

    pub fn grand_total(&self) -> f64 {
        let sub = self.subtotal();
        let tax = self.tax_total();
        let discount = self.discount_amount.unwrap_or(0.0);
        (sub + tax - discount).max(0.0)
    }

    pub fn extract_base_id(doc_num: &str) -> &str {
        extract_doc_base_id(doc_num)
    }

    pub fn balance_due(&self) -> f64 {
        let total = self.grand_total();
        if self.is_paid == Some(true) || self.payment_status.as_deref() == Some("paid") {
            0.0
        } else if let Some(paid) = self.amount_paid {
            (total - paid).max(0.0)
        } else {
            total
        }
    }

    pub fn is_paid_state(&self) -> bool {
        self.is_paid == Some(true)
            || self.payment_status.as_deref() == Some("paid")
            || self.doc_type == DocumentType::Receipt
            || (self.amount_paid.is_some() && self.balance_due() <= 0.001)
    }

    pub fn effective_payment_status(&self) -> &str {
        if self.is_paid_state() {
            "paid"
        } else if let Some(ref st) = self.payment_status {
            st.as_str()
        } else {
            "unpaid"
        }
    }

    pub fn to_invoice(&self, due_days: u32) -> Self {
        let mut cloned = self.clone();
        cloned.doc_type = DocumentType::Invoice;
        cloned.title = Some("TAX INVOICE".to_string());
        let today = chrono::Utc::now().naive_utc().date();
        cloned.issue_date = Some(today.format("%Y-%m-%d").to_string());
        let due = today + chrono::Days::new(due_days as u64);
        cloned.due_date = Some(due.format("%Y-%m-%d").to_string());
        if let Some(ref num) = cloned.doc_number {
            let base = extract_doc_base_id(num);
            cloned.doc_number = Some(format!("INV-{}", base));
        } else {
            cloned.doc_number = Some(format!("INV-{}", today.format("%Y%m%d")));
        }
        cloned.payment_status = Some("pending".to_string());
        cloned.is_paid = Some(false);
        cloned
    }

    pub fn to_receipt(&self, payment_method: &str, tx_ref: &str, paid_date: &str) -> Self {
        let mut cloned = self.clone();
        cloned.doc_type = DocumentType::Receipt;
        cloned.title = Some("OFFICIAL PAYMENT RECEIPT".to_string());
        if let Some(ref num) = cloned.doc_number {
            let base = extract_doc_base_id(num);
            cloned.doc_number = Some(format!("REC-{}", base));
        } else {
            let today = chrono::Utc::now().naive_utc().date();
            cloned.doc_number = Some(format!("REC-{}", today.format("%Y%m%d")));
        }
        cloned.is_paid = Some(true);
        cloned.payment_status = Some("paid".to_string());
        cloned.payment_method = Some(payment_method.to_string());
        cloned.transaction_reference = Some(tx_ref.to_string());
        
        let valid_paid_date = if let Some(ref issue) = cloned.issue_date {
            if paid_date < issue.as_str() {
                issue.clone()
            } else {
                paid_date.to_string()
            }
        } else {
            paid_date.to_string()
        };
        cloned.paid_at = Some(valid_paid_date);
        cloned.amount_paid = Some(self.grand_total());
        cloned
    }

    pub fn to_estimate(&self, valid_days: u32) -> Self {
        let mut cloned = self.clone();
        cloned.doc_type = DocumentType::Estimate;
        cloned.title = Some("COST ESTIMATE & PROPOSAL".to_string());
        let today = chrono::Utc::now().naive_utc().date();
        cloned.issue_date = Some(today.format("%Y-%m-%d").to_string());
        let valid_until = today + chrono::Days::new(valid_days as u64);
        cloned.due_date = Some(valid_until.format("%Y-%m-%d").to_string());
        if let Some(ref num) = cloned.doc_number {
            let base = extract_doc_base_id(num);
            cloned.doc_number = Some(format!("EST-{}", base));
        } else {
            cloned.doc_number = Some(format!("EST-{}", today.format("%Y%m%d")));
        }
        cloned.is_paid = Some(false);
        cloned.payment_status = Some("unpaid".to_string());
        cloned
    }

    pub fn to_credit_note(&self, reason: Option<&str>) -> Self {
        let mut cloned = self.clone();
        cloned.doc_type = DocumentType::CreditNote;
        cloned.title = Some("CREDIT NOTE".to_string());
        let today = chrono::Utc::now().naive_utc().date();
        cloned.issue_date = Some(today.format("%Y-%m-%d").to_string());
        if let Some(ref num) = cloned.doc_number {
            let base = extract_doc_base_id(num);
            cloned.doc_number = Some(format!("CRD-{}", base));
        } else {
            cloned.doc_number = Some(format!("CRD-{}", today.format("%Y%m%d")));
        }
        let note_text = if let Some(r) = reason {
            format!("Reason: {}", r)
        } else {
            "Credit adjustment and balance reversal.".to_string()
        };
        if let Some(ref existing) = cloned.notes {
            cloned.notes = Some(format!("{}\n[CREDIT NOTE]: {}", existing, note_text));
        } else {
            cloned.notes = Some(format!("[CREDIT NOTE]: {}", note_text));
        }
        cloned.due_date = None;
        cloned.payment_qr_code = Some(false);
        cloned.payment_link = None;
        cloned.payment_status = Some("refunded".to_string());
        cloned.is_paid = Some(false);
        cloned
    }

    pub fn effective_title(&self) -> String {
        self.title
            .clone()
            .unwrap_or_else(|| self.doc_type.display_title().to_string())
    }

    pub fn effective_doc_number(&self) -> String {
        self.doc_number.clone().unwrap_or_else(|| {
            format!("{}2026-0042", self.doc_type.default_number_prefix())
        })
    }

    pub fn effective_paper_size(&self) -> &str {
        match self.paper_size.as_deref() {
            Some("a4") => "a4",
            Some("a5") => "a5",
            Some("a3") => "a3",
            Some("us-legal") | Some("legal") => "us-legal",
            Some("us-executive") | Some("executive") => "us-executive",
            Some("tabloid") | Some("us-tabloid") | Some("ledger") | Some("us-ledger") => "us-tabloid",
            _ => "us-letter",
        }
    }

    pub fn effective_primary_color(&self) -> &str {
        if let Some(ref hex) = self.primary_color {
            if hex.starts_with('#') && hex.len() == 7 && hex[1..].chars().all(|c| c.is_ascii_hexdigit()) {
                return hex.as_str();
            }
        }
        match self.theme.as_deref() {
            Some("executive_navy") => "#1e3a8a",
            Some("emerald") => "#059669",
            Some("crimson_legal") => "#b91c1c",
            Some("warm_sand") => "#b45309",
            Some("obsidian") => "#18181b",
            Some("neobrutalist") => "#000000",
            Some("nordic_frost") | Some("nordic_teal") => "#0d9488",
            Some("bordeaux") => "#831843",
            Some("royal_violet") => "#7c3aed",
            Some("sunset_coral") => "#e11d48",
            Some("cyber_neon") => "#06b6d4",
            Some("forest_pine") => "#166534",
            Some("desert_sand") => "#d97706",
            Some("monaco_gold") => "#ca8a04",
            Some("bauhaus_primary") => "#2563eb",
            Some("electric_indigo") => "#4338ca",
            Some("swiss_minimal") => "#334155",
            _ => "#4f46e5", // Modern Slate / Electric Indigo
        }
    }

    pub fn effective_secondary_color(&self) -> &str {
        if let Some(ref hex) = self.secondary_color {
            if hex.starts_with('#') && hex.len() == 7 && hex[1..].chars().all(|c| c.is_ascii_hexdigit()) {
                return hex.as_str();
            }
        }
        match self.theme.as_deref() {
            Some("executive_navy") => "#d4af37", // Champagne Gold
            Some("emerald") => "#10b981",        // Mint Sage
            Some("crimson_legal") => "#b45309",  // Burnished Bronze
            Some("warm_sand") => "#78350f",      // Dark Amber
            Some("obsidian") => "#3f3f46",       // Zinc Slate
            Some("neobrutalist") => "#facc15",   // Canary Yellow
            Some("nordic_frost") | Some("nordic_teal") => "#14b8a6", // Frosted Teal
            Some("bordeaux") => "#be185d",       // Rose Gold
            Some("royal_violet") => "#a855f7",   // Lavender Accent
            Some("sunset_coral") => "#d97706",   // Warm Ochre
            Some("cyber_neon") => "#06b6d4",     // Electric Cyan
            Some("forest_pine") => "#4d7c0f",    // Moss Olive
            Some("desert_sand") => "#78350f",    // Dark Amber
            Some("monaco_gold") => "#18181b",    // Onyx Black
            Some("bauhaus_primary") => "#b91c1c", // Bauhaus Red
            Some("electric_indigo") => "#6366f1", // Soft Indigo
            Some("swiss_minimal") => "#dc2626",  // Signal Red
            _ => "#475569",                      // Slate 600
        }
    }

    pub fn effective_accent_color(&self) -> &str {
        if let Some(ref hex) = self.accent_color {
            if hex.starts_with('#') && hex.len() == 7 && hex[1..].chars().all(|c| c.is_ascii_hexdigit()) {
                return hex.as_str();
            }
        }
        match self.theme.as_deref() {
            Some("executive_navy") => "#c59b27",
            Some("emerald") => "#34d399",
            Some("crimson_legal") => "#dc2626",
            Some("warm_sand") => "#f59e0b",
            Some("obsidian") => "#71717a",
            Some("neobrutalist") => "#2563eb",
            Some("nordic_frost") | Some("nordic_teal") => "#2dd4bf",
            Some("bordeaux") => "#db2777",
            Some("royal_violet") => "#ec4899",
            Some("sunset_coral") => "#e11d48",
            Some("cyber_neon") => "#ec4899",
            Some("forest_pine") => "#15803d",
            Some("desert_sand") => "#f59e0b",
            Some("monaco_gold") => "#eab308",
            Some("bauhaus_primary") => "#eab308",
            Some("electric_indigo") => "#818cf8",
            Some("swiss_minimal") => "#71717a",
            _ => "#06b6d4",
        }
    }

    pub fn effective_surface_color(&self) -> &str {
        if let Some(ref hex) = self.surface_color {
            if hex.starts_with('#') && hex.len() == 7 && hex[1..].chars().all(|c| c.is_ascii_hexdigit()) {
                return hex.as_str();
            }
        }
        match self.theme.as_deref() {
            Some("executive_navy") => "#fdfbf7", // Warm Ivory Cream
            Some("emerald") => "#f0fdf4",        // Frosted Mint
            Some("crimson_legal") => "#fffbeb",  // Legal Parchment
            Some("warm_sand") => "#fefce8",      // Light Sand
            Some("obsidian") => "#fafafa",       // Clean Studio
            Some("neobrutalist") => "#ffffff",   // Stark White
            Some("nordic_frost") | Some("nordic_teal") => "#f0fdfa", // Frosted Ice
            Some("bordeaux") => "#fdf2f8",       // Silk Rose Tint
            Some("royal_violet") => "#faf5ff",   // Lavender Mist
            Some("sunset_coral") => "#fff7ed",   // Warm Peach Tint
            Some("cyber_neon") => "#f8fafc",     // Silicon Mist
            Some("forest_pine") => "#f7fee7",    // Pale Lime Glow
            Some("desert_sand") => "#fefce8",    // Calico Sand
            Some("monaco_gold") => "#fffdf5",    // Ivory Pearl
            Some("bauhaus_primary") => "#f8fafc", // White Vellum
            Some("electric_indigo") => "#eef2ff", // Soft Indigo Tint
            Some("swiss_minimal") => "#fafafa",  // Pure Milk White
            _ => "#f8fafc",                      // Slate 50
        }
    }

    pub fn effective_text_color(&self) -> &str {
        if let Some(ref hex) = self.text_color {
            if hex.starts_with('#') && hex.len() == 7 && hex[1..].chars().all(|c| c.is_ascii_hexdigit()) {
                return hex.as_str();
            }
        }
        match self.theme.as_deref() {
            Some("executive_navy") => "#0f172a",
            Some("emerald") => "#06281f",
            Some("crimson_legal") => "#292524",
            Some("neobrutalist") => "#000000",
            Some("swiss_minimal") => "#09090b",
            _ => "#0f172a",
        }
    }

    pub fn effective_font_family(&self) -> &str {
        match self.font_family.as_deref() {
            Some("serif") => "(\"Georgia\", \"Libertinus Serif\", \"Times New Roman\")",
            Some("mono") => "(\"Menlo\", \"Courier New\")",
            Some("display") => "(\"Playfair Display\", \"Georgia\", \"Times New Roman\")",
            Some("grotesk") => "(\"Space Grotesk\", \"Helvetica Neue\", \"Arial\")",
            Some("editorial") => "(\"Palatino\", \"Garamond\", \"Georgia\")",
            _ => "(\"Helvetica Neue\", \"Arial\")",
        }
    }

    pub fn effective_font_size(&self) -> &str {
        match self.font_scale.as_deref() {
            Some("compact") => "8.5pt",
            Some("large") => "10.5pt",
            _ => "9.5pt",
        }
    }

    pub fn effective_margins(&self) -> &'static str {
        match self.spacing_density.as_deref() {
            Some("compact") => "(top: 24pt, bottom: 24pt, left: 28pt, right: 28pt)",
            Some("generous") => "(top: 48pt, bottom: 48pt, left: 54pt, right: 54pt)",
            Some("executive") => "(top: 60pt, bottom: 60pt, left: 68pt, right: 68pt)",
            _ => "(top: 36pt, bottom: 36pt, left: 40pt, right: 40pt)",
        }
    }

    pub fn effective_line_height(&self) -> &'static str {
        match self.spacing_density.as_deref() {
            Some("compact") => "1.15",
            Some("generous") => "1.45",
            Some("executive") => "1.6",
            _ => "1.3",
        }
    }

    pub fn effective_signatures(&self) -> Vec<SignatureBlock> {
        if !self.signatures.is_empty() {
            self.signatures.clone()
        } else if let Some(ref name) = self.signer_name {
            if !name.trim().is_empty() {
                vec![SignatureBlock {
                    signer_name: name.clone(),
                    signer_title: self.signer_title.clone(),
                    date: self.issue_date.clone(),
                    signature_data: self.signature_data.clone(),
                }]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    pub fn effective_background_fill(&self) -> &str {
        match self.background_style.as_deref() {
            Some("parchment") => "#fcfbf7",
            Some("slate_tint") => "#f8fafc",
            Some("blueprint") => "#f0fdfa",
            Some("linen_subtle") => "#faf9f5",
            Some("dot_matrix") => "#f8fafb",
            Some("marble_vellum") => "#fbfaf8",
            _ => "#ffffff",
        }
    }

    pub fn format_amount(&self, val: f64) -> String {
        let decimals = self.number_decimals.unwrap_or(2);
        let sep_mode = self.currency_separators.as_deref();
        let thousands_sep: Option<char> = match sep_mode {
            Some("dot_comma") | Some("euro") => Some('.'),
            Some("space_comma") | Some("space") => Some(' '),
            Some("apostrophe_dot") => Some('\''),
            Some("plain") => None,
            _ => Some(','),
        };
        let decimal_sep = match sep_mode {
            Some("dot_comma") | Some("euro") | Some("space_comma") | Some("space") => ',',
            _ => '.',
        };

        let is_negative = val < 0.0;
        let abs_val = val.abs();
        let int_part = abs_val.trunc() as u64;
        let frac_str = if decimals > 0 {
            let mult = 10f64.powi(decimals as i32);
            let frac_val = (abs_val.fract() * mult).round() as u64;
            format!("{}{:0width$}", decimal_sep, frac_val, width = decimals as usize)
        } else {
            String::new()
        };

        let int_str = int_part.to_string();
        let mut formatted_int = String::new();
        let num_digits = int_str.len();
        for (i, ch) in int_str.chars().enumerate() {
            if i > 0 && (num_digits - i) % 3 == 0 {
                if let Some(sep) = thousands_sep {
                    formatted_int.push(sep);
                }
            }
            formatted_int.push(ch);
        }

        let num_formatted = format!("{}{}", formatted_int, frac_str);
        let symbol_or_code = if self.currency_symbol_mode.as_deref() == Some("code") {
            self.currency.as_deref().unwrap_or("USD")
        } else {
            self.currency_symbol()
        };

        let is_suffix = self.currency_placement.as_deref() == Some("suffix");
        let result = if is_suffix {
            format!("{} {}", num_formatted, symbol_or_code)
        } else if symbol_or_code.len() > 1 && symbol_or_code.chars().all(|c| c.is_ascii_alphabetic()) {
            format!("{} {}", symbol_or_code, num_formatted)
        } else {
            format!("{}{}", symbol_or_code, num_formatted)
        };

        if is_negative {
            format!("-{}", result)
        } else {
            result
        }
    }

    pub fn format_amount_typst(&self, val: f64) -> String {
        crate::engine::escape_typst_text(&self.format_amount(val))
    }

    pub fn format_date(&self, date_str: &str) -> String {
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() == 3 && parts[0].len() == 4 {
            let y = parts[0];
            let m = parts[1];
            let d = parts[2];
            let month_name = match m {
                "01" => "January", "02" => "February", "03" => "March",
                "04" => "April", "05" => "May", "06" => "June",
                "07" => "July", "08" => "August", "09" => "September",
                "10" => "October", "11" => "November", "12" => "December",
                _ => m,
            };
            let short_month = match m {
                "01" => "Jan", "02" => "Feb", "03" => "Mar",
                "04" => "Apr", "05" => "May", "06" => "Jun",
                "07" => "Jul", "08" => "Aug", "09" => "Sep",
                "10" => "Oct", "11" => "Nov", "12" => "Dec",
                _ => m,
            };

            match self.date_format.as_deref() {
                Some("us_slash") | Some("us") => format!("{}/{}/{}", m, d, y),
                Some("euro") | Some("eu") => format!("{}/{}/{}", d, m, y),
                Some("formal_prose") | Some("long") | Some("formal") => format!("{} {}, {}", month_name, d, y),
                Some("short_abbrev") | Some("short") => format!("{}-{}-{}", d, short_month, y),
                Some("short_dot") => format!("{}.{}.{}", d, m, y),
                _ => date_str.to_string(), // default ISO
            }
        } else {
            date_str.to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResponse {
    pub success: bool,
    pub pdf_base64: Option<String>,
    pub preview_data_uri: Option<String>,
    pub doc_number: String,
    pub grand_total: f64,
    pub compile_time_ms: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewResponse {
    pub success: bool,
    pub image_data_uri: String,
    #[serde(default)]
    pub pages: Vec<String>,
    #[serde(default)]
    pub total_pages: usize,
    pub compile_time_ms: u128,
}

#[derive(Debug, Deserialize)]
pub struct CopilotRequest {
    pub prompt: String,
    pub current_state: DocumentRequest,
}

#[derive(Debug, Serialize)]
pub struct CopilotResponse {
    pub success: bool,
    pub modified_state: DocumentRequest,
    pub ai_message: String,
}

#[derive(Debug, Deserialize)]
pub struct VisionRequest {
    pub image_data: String,
    #[serde(default)]
    pub filename: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VisionResponse {
    pub success: bool,
    pub extracted_items: Vec<LineItem>,
    pub extracted_from: Option<PartyDetails>,
    pub ai_message: String,
}

/// Extract the numeric/base sequence from a prefixed document number (e.g. "EST-2026-0042" -> "2026-0042").
fn extract_doc_base_id(doc_num: &str) -> &str {
    for prefix in &[
        "INV-", "EST-", "QT-", "REC-", "CRD-", "DOC-", "PO-", "PKG-", "MSA-", "RLA-", "NNN-",
        "PMS-", "RRL-", "ESC-", "NTV-", "SDD-", "COM-", "CHK-", "LOI-", "TEC-", "CERT-", "MED-",
        "CST-", "SUB-", "BOS-", "ICA-", "SOW-", "NDA-", "INSP-",
    ] {
        if doc_num.starts_with(prefix) {
            return &doc_num[prefix.len()..];
        }
    }
    doc_num
}

