//! Pressmark Document Models & Calculation Helpers
//!
//! Author: Daniel Hallman

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
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
    // Real Estate & Property Management Suite (Initial 11)
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
    // Avail & Stessa Expansion (11)
    RoomRentalLease,
    LeaseRenewalAgreement,
    LeaseAmendment,
    RentalApplication,
    AdverseActionNotice,
    NoticeOfEntry,
    RentIncreaseNotice,
    MutualLeaseTermination,
    TenantRentLedger,
    ScheduleOfRealEstateOwned,
    ScheduleETaxWorksheet,
    // BiggerPockets & Rocket Lawyer Expansion (7)
    RealEstateDealAnalysis,
    SubleaseAgreement,
    RentToOwnAgreement,
    PropertyManagementAgreement,
    RealEstatePurchaseAgreement,
    PropertyDeed,
    MechanicsLienWaiver,
}

impl Default for DocumentType {
    fn default() -> Self {
        Self::Invoice
    }
}

impl<'de> Deserialize<'de> for DocumentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.trim().is_empty() {
            return Ok(Self::Invoice);
        }
        Self::from_slug(&s)
            .or_else(|| Self::parse_state_prefixed_slug(&s).map(|(_, dt)| dt))
            .ok_or_else(|| {
                serde::de::Error::custom(format!(
                    "Unknown document type '{}'. Expected one of the 57 Pressmark archetypes.",
                    s
                ))
            })
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
            Self::RoomRentalLease => "ROOM RENTAL & CO-LIVING LEASE AGREEMENT",
            Self::LeaseRenewalAgreement => "LEASE RENEWAL & RENT ADJUSTMENT AGREEMENT",
            Self::LeaseAmendment => "LEASE AMENDMENT & STATUTORY ADDENDUM",
            Self::RentalApplication => "RESIDENTIAL RENTAL APPLICATION & FCRA CONSENT",
            Self::AdverseActionNotice => "FCRA ADVERSE ACTION & SCREENING DENIAL NOTICE",
            Self::NoticeOfEntry => "STATUTORY NOTICE OF INTENT TO ENTER PREMISES",
            Self::RentIncreaseNotice => "STATUTORY NOTICE OF RENT ADJUSTMENT",
            Self::MutualLeaseTermination => "MUTUAL LEASE TERMINATION & SURRENDER AGREEMENT",
            Self::TenantRentLedger => "CERTIFIED TENANT RENT & CHARGE LEDGER",
            Self::ScheduleOfRealEstateOwned => "SCHEDULE OF REAL ESTATE OWNED (SREO)",
            Self::ScheduleETaxWorksheet => "IRS SCHEDULE E RENTAL TAX & T12 OPERATING WORKSHEET",
            Self::RealEstateDealAnalysis => "REAL ESTATE INVESTMENT & DEAL UNDERWRITING REPORT",
            Self::SubleaseAgreement => "RESIDENTIAL SUBLEASE & LANDLORD CONSENT AGREEMENT",
            Self::RentToOwnAgreement => "RENT-TO-OWN (LEASE-OPTION) AGREEMENT",
            Self::PropertyManagementAgreement => "EXCLUSIVE PROPERTY MANAGEMENT AGREEMENT",
            Self::RealEstatePurchaseAgreement => "REAL ESTATE PURCHASE & SALE AGREEMENT (PSA)",
            Self::PropertyDeed => "STATUTORY RECORDABLE PROPERTY DEED",
            Self::MechanicsLienWaiver => "STATUTORY MECHANIC'S LIEN WAIVER & RELEASE",
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
            Self::Receipt => "REC-",
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
            Self::RoomRentalLease => "RLC-",
            Self::LeaseRenewalAgreement => "REN-",
            Self::LeaseAmendment => "AMD-",
            Self::RentalApplication => "APP-",
            Self::AdverseActionNotice => "FCRA-",
            Self::NoticeOfEntry => "ENT-",
            Self::RentIncreaseNotice => "RIN-",
            Self::MutualLeaseTermination => "MLT-",
            Self::TenantRentLedger => "LDG-",
            Self::ScheduleOfRealEstateOwned => "SREO-",
            Self::ScheduleETaxWorksheet => "TXE-",
            Self::RealEstateDealAnalysis => "DEAL-",
            Self::SubleaseAgreement => "SUBL-",
            Self::RentToOwnAgreement => "RTO-",
            Self::PropertyManagementAgreement => "PMA-",
            Self::RealEstatePurchaseAgreement => "PSA-",
            Self::PropertyDeed => "DEED-",
            Self::MechanicsLienWaiver => "LIEN-",
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
            Self::RoomRentalLease => "room_rental_lease",
            Self::LeaseRenewalAgreement => "lease_renewal_agreement",
            Self::LeaseAmendment => "lease_amendment",
            Self::RentalApplication => "rental_application",
            Self::AdverseActionNotice => "adverse_action_notice",
            Self::NoticeOfEntry => "notice_of_entry",
            Self::RentIncreaseNotice => "rent_increase_notice",
            Self::MutualLeaseTermination => "mutual_lease_termination",
            Self::TenantRentLedger => "tenant_rent_ledger",
            Self::ScheduleOfRealEstateOwned => "schedule_of_real_estate_owned",
            Self::ScheduleETaxWorksheet => "schedule_e_tax_worksheet",
            Self::RealEstateDealAnalysis => "real_estate_deal_analysis",
            Self::SubleaseAgreement => "sublease_agreement",
            Self::RentToOwnAgreement => "rent_to_own_agreement",
            Self::PropertyManagementAgreement => "property_management_agreement",
            Self::RealEstatePurchaseAgreement => "real_estate_purchase_agreement",
            Self::PropertyDeed => "property_deed",
            Self::MechanicsLienWaiver => "mechanics_lien_waiver",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Invoice,
            Self::InvoiceStandard,
            Self::InvoiceFreelance,
            Self::InvoiceConstruction,
            Self::InvoiceMedical,
            Self::InvoiceSaas,
            Self::InvoiceMinimal,
            Self::ProformaInvoice,
            Self::Estimate,
            Self::Receipt,
            Self::PurchaseOrder,
            Self::PackingSlip,
            Self::CreditNote,
            Self::WorkOrder,
            Self::ExpenseReport,
            Self::DeliveryReceipt,
            Self::CommercialLease,
            Self::QuotationTender,
            Self::Agreement,
            Self::AgreementSow,
            Self::ContractorAgreement,
            Self::RetainerAgreement,
            Self::Nda,
            Self::BillOfSale,
            Self::Certificate,
            Self::CertificateInsurance,
            Self::CertificateCompletion,
            Self::InspectionReport,
            Self::ResidentialLease,
            Self::CommercialNnnLease,
            Self::PropertyManagementStatement,
            Self::RentRoll,
            Self::ClosingSettlementStatement,
            Self::NoticeToVacate,
            Self::SecurityDepositDisposition,
            Self::RealEstateCommission,
            Self::PropertyInspectionChecklist,
            Self::LetterOfIntentCre,
            Self::TenantEstoppel,
            Self::RoomRentalLease,
            Self::LeaseRenewalAgreement,
            Self::LeaseAmendment,
            Self::RentalApplication,
            Self::AdverseActionNotice,
            Self::NoticeOfEntry,
            Self::RentIncreaseNotice,
            Self::MutualLeaseTermination,
            Self::TenantRentLedger,
            Self::ScheduleOfRealEstateOwned,
            Self::ScheduleETaxWorksheet,
            Self::RealEstateDealAnalysis,
            Self::SubleaseAgreement,
            Self::RentToOwnAgreement,
            Self::PropertyManagementAgreement,
            Self::RealEstatePurchaseAgreement,
            Self::PropertyDeed,
            Self::MechanicsLienWaiver,
        ]
    }

    pub fn url_slug(&self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
            Self::InvoiceStandard => "invoice-standard",
            Self::InvoiceFreelance => "invoice-freelance",
            Self::InvoiceConstruction => "invoice-construction",
            Self::InvoiceMedical => "invoice-medical",
            Self::InvoiceSaas => "invoice-saas",
            Self::InvoiceMinimal => "invoice-minimal",
            Self::ProformaInvoice => "proforma-invoice",
            Self::Estimate => "estimate",
            Self::Receipt => "receipt",
            Self::PurchaseOrder => "purchase-order",
            Self::PackingSlip => "packing-slip",
            Self::CreditNote => "credit-note",
            Self::WorkOrder => "work-order",
            Self::ExpenseReport => "expense-report",
            Self::DeliveryReceipt => "delivery-receipt",
            Self::CommercialLease => "commercial-lease",
            Self::QuotationTender => "quotation-tender",
            Self::Agreement => "agreement",
            Self::AgreementSow => "agreement-sow",
            Self::ContractorAgreement => "contractor-agreement",
            Self::RetainerAgreement => "retainer-agreement",
            Self::Nda => "nda",
            Self::BillOfSale => "bill-of-sale",
            Self::Certificate => "certificate",
            Self::CertificateInsurance => "certificate-insurance",
            Self::CertificateCompletion => "certificate-completion",
            Self::InspectionReport => "inspection-report",
            Self::ResidentialLease => "residential-lease",
            Self::CommercialNnnLease => "commercial-nnn-lease",
            Self::PropertyManagementStatement => "property-management-statement",
            Self::RentRoll => "rent-roll",
            Self::ClosingSettlementStatement => "closing-settlement-statement",
            Self::NoticeToVacate => "notice-to-vacate",
            Self::SecurityDepositDisposition => "security-deposit-disposition",
            Self::RealEstateCommission => "real-estate-commission",
            Self::PropertyInspectionChecklist => "property-inspection-checklist",
            Self::LetterOfIntentCre => "letter-of-intent-cre",
            Self::TenantEstoppel => "tenant-estoppel",
            Self::RoomRentalLease => "room-rental-lease",
            Self::LeaseRenewalAgreement => "lease-renewal-agreement",
            Self::LeaseAmendment => "lease-amendment",
            Self::RentalApplication => "rental-application",
            Self::AdverseActionNotice => "adverse-action-notice",
            Self::NoticeOfEntry => "notice-of-entry",
            Self::RentIncreaseNotice => "rent-increase-notice",
            Self::MutualLeaseTermination => "mutual-lease-termination",
            Self::TenantRentLedger => "tenant-rent-ledger",
            Self::ScheduleOfRealEstateOwned => "schedule-of-real-estate-owned",
            Self::ScheduleETaxWorksheet => "schedule-e-tax-worksheet",
            Self::RealEstateDealAnalysis => "real-estate-deal-analysis",
            Self::SubleaseAgreement => "sublease-agreement",
            Self::RentToOwnAgreement => "rent-to-own-agreement",
            Self::PropertyManagementAgreement => "property-management-agreement",
            Self::RealEstatePurchaseAgreement => "real-estate-purchase-agreement",
            Self::PropertyDeed => "property-deed",
            Self::MechanicsLienWaiver => "mechanics-lien-waiver",
        }
    }

    pub fn from_slug(slug: &str) -> Option<Self> {
        let normalized = slug
            .trim()
            .to_lowercase()
            .replace(['-', ' '], "_")
            .split('_')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("_");
        let compact = normalized.replace('_', "");
        for item in Self::all() {
            if item.as_str() == normalized || item.as_str().replace('_', "") == compact {
                return Some(item.clone());
            }
        }
        match normalized.as_str() {
            "tax_invoice" | "standard_invoice" => Some(Self::InvoiceStandard),
            "freelance_invoice" | "consulting_invoice" => Some(Self::InvoiceFreelance),
            "construction_invoice" | "progress_billing" => Some(Self::InvoiceConstruction),
            "medical_invoice" | "superbill" => Some(Self::InvoiceMedical),
            "saas_invoice" | "subscription_invoice" => Some(Self::InvoiceSaas),
            "minimal_invoice" | "commercial_invoice" => Some(Self::InvoiceMinimal),
            "quote" | "proposal" => Some(Self::Estimate),
            "payment_receipt" | "paid_receipt" => Some(Self::Receipt),
            "po" => Some(Self::PurchaseOrder),
            "bol" | "bill_of_lading" => Some(Self::PackingSlip),
            "credit" | "refund" | "credit_memo" => Some(Self::CreditNote),
            "msa" | "master_services_agreement" => Some(Self::Agreement),
            "sow" | "statement_of_work" => Some(Self::AgreementSow),
            "mutual_nda" | "non_disclosure_agreement" => Some(Self::Nda),
            "coi" | "certificate_of_insurance" => Some(Self::CertificateInsurance),
            "certificate_of_completion" => Some(Self::CertificateCompletion),
            "lease" | "standard_lease" => Some(Self::ResidentialLease),
            "nnn_lease" | "triple_net_lease" => Some(Self::CommercialNnnLease),
            "owner_statement" | "property_statement" => Some(Self::PropertyManagementStatement),
            "closing_statement" | "settlement_statement" => Some(Self::ClosingSettlementStatement),
            "eviction_notice"
            | "pay_or_quit"
            | "pay_or_quit_notice"
            | "cure_or_quit"
            | "14_day_rent_demand"
            | "3_day_notice_to_pay_or_quit"
            | "5_day_notice_to_pay_or_quit"
            | "scra_military_termination"
            | "vawa_termination" => Some(Self::NoticeToVacate),
            "security_deposit" | "deposit_disposition" => Some(Self::SecurityDepositDisposition),
            "commission_statement" | "brokerage_commission" => Some(Self::RealEstateCommission),
            "move_in_checklist" | "move_out_checklist" => Some(Self::PropertyInspectionChecklist),
            "loi" | "letter_of_intent" => Some(Self::LetterOfIntentCre),
            "estoppel" | "estoppel_certificate" => Some(Self::TenantEstoppel),
            "room_lease" | "coliving_lease" | "co_living_lease" | "roommate_agreement" => Some(Self::RoomRentalLease),
            "lease_renewal" | "lease_extension" => Some(Self::LeaseRenewalAgreement),
            "lease_addendum" | "pet_addendum" | "cosigner_guaranty" | "lease_guaranty" => Some(Self::LeaseAmendment),
            "tenant_application" | "screening_application" => Some(Self::RentalApplication),
            "fcra_adverse_action" | "tenant_denial_notice" => Some(Self::AdverseActionNotice),
            "intent_to_enter" | "entry_notice" => Some(Self::NoticeOfEntry),
            "rent_increase" | "rent_adjustment_notice" => Some(Self::RentIncreaseNotice),
            "cash_for_keys" | "lease_surrender" | "early_termination_agreement" => Some(Self::MutualLeaseTermination),
            "rent_ledger" | "tenant_ledger" => Some(Self::TenantRentLedger),
            "sreo" | "reo_schedule" | "real_estate_portfolio" => Some(Self::ScheduleOfRealEstateOwned),
            "schedule_e" | "t12_income_statement" | "rental_tax_worksheet" => Some(Self::ScheduleETaxWorksheet),
            "deal_analysis"
            | "brrrr_analysis"
            | "brrrr_deal_analysis"
            | "fix_and_flip_analysis"
            | "rental_property_analysis"
            | "rehab_estimator"
            | "str_analysis" => Some(Self::RealEstateDealAnalysis),
            "sublease" | "sublet_agreement" => Some(Self::SubleaseAgreement),
            "rent_to_own" | "lease_option" | "lease_purchase" => Some(Self::RentToOwnAgreement),
            "property_management_contract" => Some(Self::PropertyManagementAgreement),
            "purchase_agreement" | "psa" | "wholesale_assignment" | "contract_for_deed" | "land_contract" => Some(Self::RealEstatePurchaseAgreement),
            "deed"
            | "quitclaim_deed"
            | "warranty_deed"
            | "grant_deed"
            | "special_warranty_deed"
            | "lady_bird_deed"
            | "tod_deed"
            | "beneficiary_deed" => Some(Self::PropertyDeed),
            "lien_waiver" | "conditional_lien_waiver" | "unconditional_lien_waiver" => Some(Self::MechanicsLienWaiver),
            _ => None,
        }
    }

    pub fn allows_negative_items(&self) -> bool {
        matches!(
            self,
            Self::CreditNote
                | Self::PropertyManagementStatement
                | Self::SecurityDepositDisposition
                | Self::RealEstateCommission
                | Self::ClosingSettlementStatement
                | Self::TenantRentLedger
                | Self::ScheduleETaxWorksheet
                | Self::RealEstateDealAnalysis
                | Self::LeaseAmendment
                | Self::RentIncreaseNotice
                | Self::MutualLeaseTermination
                | Self::RealEstatePurchaseAgreement
        )
    }

    pub fn supports_jurisdiction_inference(&self) -> bool {
        self.is_state_lease_instrument()
            || matches!(
                self,
                Self::PropertyDeed
                    | Self::MechanicsLienWaiver
                    | Self::TenantEstoppel
                    | Self::SubleaseAgreement
                    | Self::RentToOwnAgreement
                    | Self::RealEstatePurchaseAgreement
                    | Self::PropertyManagementAgreement
                    | Self::PropertyManagementStatement
                    | Self::ClosingSettlementStatement
                    | Self::RealEstateCommission
                    | Self::PropertyInspectionChecklist
                    | Self::LetterOfIntentCre
                    | Self::RentalApplication
                    | Self::AdverseActionNotice
                    | Self::TenantRentLedger
                    | Self::ScheduleOfRealEstateOwned
                    | Self::ScheduleETaxWorksheet
                    | Self::RealEstateDealAnalysis
                    | Self::RentRoll
                    | Self::CommercialNnnLease
                    | Self::CommercialLease
            )
    }

    /// Resolves state-prefixed programmatic SEO slugs such as:
    /// - `"tennessee-residential-lease"` -> `(TN, ResidentialLease)`
    /// - `"california-eviction-notice"` -> `(CA, NoticeToVacate)`
    /// - `"texas-quitclaim-deed"` -> `(TX, PropertyDeed)`
    /// - `"new-york-room-rental-lease"` -> `(NY, RoomRentalLease)`
    pub fn parse_state_prefixed_slug(
        slug: &str,
    ) -> Option<(&'static crate::state_laws::StateLeaseProfile, Self)> {
        let normalized = slug.trim().to_ascii_lowercase().replace('_', "-");
        // Check full state slugs first (longest match wins, e.g. "west-virginia-" before "virginia-")
        let mut profiles: Vec<&'static crate::state_laws::StateLeaseProfile> =
            crate::state_laws::all_state_lease_profiles().iter().collect();
        profiles.sort_by(|a, b| b.slug.len().cmp(&a.slug.len()));

        for profile in &profiles {
            let prefix = format!("{}-", profile.slug);
            if let Some(rest) = normalized.strip_prefix(&prefix) {
                if let Some(doc_type) = Self::from_slug(rest) {
                    return Some((*profile, doc_type));
                }
            }
        }
        // Check Washington DC alias
        if let Some(rest) = normalized.strip_prefix("washington-dc-") {
            if let Some(profile) = crate::state_laws::get_state_lease_profile("DC") {
                if let Some(doc_type) = Self::from_slug(rest) {
                    return Some((profile, doc_type));
                }
            }
        }
        // Check 2-letter state code prefix (e.g. "tn-residential-lease")
        for profile in &profiles {
            let code_prefix = format!("{}-", profile.code.to_ascii_lowercase());
            if let Some(rest) = normalized.strip_prefix(&code_prefix) {
                if let Some(doc_type) = Self::from_slug(rest) {
                    return Some((*profile, doc_type));
                }
            }
        }
        None
    }

    pub fn is_contract_or_lease(&self) -> bool {
        matches!(
            self,
            Self::Agreement
                | Self::AgreementSow
                | Self::ContractorAgreement
                | Self::RetainerAgreement
                | Self::Nda
                | Self::BillOfSale
                | Self::ResidentialLease
                | Self::RoomRentalLease
                | Self::SubleaseAgreement
                | Self::RentToOwnAgreement
                | Self::CommercialNnnLease
                | Self::CommercialLease
                | Self::LeaseRenewalAgreement
                | Self::LeaseAmendment
                | Self::MutualLeaseTermination
                | Self::PropertyManagementAgreement
                | Self::RealEstatePurchaseAgreement
                | Self::PropertyDeed
                | Self::MechanicsLienWaiver
                | Self::LetterOfIntentCre
                | Self::TenantEstoppel
                | Self::NoticeToVacate
                | Self::NoticeOfEntry
                | Self::RentIncreaseNotice
                | Self::AdverseActionNotice
                | Self::RentalApplication
                | Self::SecurityDepositDisposition
                | Self::RealEstateCommission
                | Self::PropertyInspectionChecklist
                | Self::ClosingSettlementStatement
                | Self::PropertyManagementStatement
                | Self::RentRoll
                | Self::TenantRentLedger
                | Self::ScheduleOfRealEstateOwned
                | Self::ScheduleETaxWorksheet
                | Self::RealEstateDealAnalysis
        )
    }

    pub fn is_state_lease_instrument(&self) -> bool {
        matches!(
            self,
            Self::ResidentialLease
                | Self::RoomRentalLease
                | Self::SubleaseAgreement
                | Self::RentToOwnAgreement
                | Self::LeaseRenewalAgreement
                | Self::LeaseAmendment
                | Self::NoticeToVacate
                | Self::NoticeOfEntry
                | Self::RentIncreaseNotice
                | Self::SecurityDepositDisposition
                | Self::MutualLeaseTermination
        )
    }

    pub fn is_lease_agreement(&self) -> bool {
        matches!(
            self,
            Self::ResidentialLease
                | Self::RoomRentalLease
                | Self::LeaseRenewalAgreement
                | Self::SubleaseAgreement
                | Self::RentToOwnAgreement
        )
    }

    pub fn allows_payment_qr(&self) -> bool {
        !matches!(
            self,
            Self::CreditNote
                | Self::Receipt
                | Self::PropertyDeed
                | Self::RealEstatePurchaseAgreement
                | Self::ScheduleETaxWorksheet
                | Self::ScheduleOfRealEstateOwned
                | Self::RealEstateDealAnalysis
                | Self::SecurityDepositDisposition
                | Self::MutualLeaseTermination
                | Self::PropertyInspectionChecklist
                | Self::RentalApplication
                | Self::AdverseActionNotice
                | Self::NoticeOfEntry
                | Self::RentIncreaseNotice
                | Self::TenantEstoppel
                | Self::LetterOfIntentCre
                | Self::MechanicsLienWaiver
                | Self::Nda
                | Self::Certificate
                | Self::CertificateCompletion
                | Self::CertificateInsurance
        )
    }

    pub fn seo_metadata(&self) -> (String, String) {
        let title_case = self
            .display_title()
            .split_whitespace()
            .map(|w| {
                if matches!(
                    w,
                    "NNN" | "(NNN)" | "LOI" | "(LOI)" | "NDA" | "MSA" | "SOW" | "COI" | "FCRA" | "IRS" | "T12" | "SREO" | "(SREO)" | "PSA" | "(PSA)"
                ) {
                    w.to_string()
                } else {
                    let mut chars = w.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        let page_title = format!("{} Template & PDF Generator · Pressmark Studio", title_case);
        let description = format!(
            "Create and download a print-ready {} PDF in milliseconds. Customize line items, clauses, tax rates, and QR payment links with live preview in Pressmark Studio.",
            title_case
        );
        (page_title, description)
    }

    pub fn state_seo_metadata(
        &self,
        state: &crate::state_laws::StateLeaseProfile,
    ) -> (String, String) {
        let (base_title, _) = self.seo_metadata();
        let clean_doc_name = base_title
            .trim_end_matches(" Template & PDF Generator · Pressmark Studio");
        let page_title = format!(
            "{} {} ({}) · Statutory PDF Generator · Pressmark Studio",
            state.name, clean_doc_name, state.code
        );
        let description = format!(
            "Generate a compliant {} {} under {}. Automatically enforces {} security deposit caps ({}), {} return deadlines, {} Pay-or-Quit notices, and mandatory {} disclosures.",
            state.name,
            clean_doc_name,
            state.governing_statute,
            state.code,
            state.deposit_cap,
            state.return_deadline,
            state.pay_or_quit_notice,
            state.name
        );
        (page_title, description)
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

fn default_quantity() -> f64 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    #[serde(default, alias = "name", alias = "item", alias = "title")]
    pub description: String,
    #[serde(default = "default_quantity", alias = "qty")]
    pub quantity: f64,
    #[serde(default, alias = "rate", alias = "price", alias = "amount")]
    pub unit_price: f64,
    #[serde(default)]
    pub tax_rate: Option<f64>,
    #[serde(default)]
    pub discount: Option<f64>,
}

impl Default for LineItem {
    fn default() -> Self {
        Self {
            description: String::new(),
            quantity: 0.0,
            unit_price: 0.0,
            tax_rate: None,
            discount: None,
        }
    }
}

impl LineItem {
    pub fn amount(&self) -> f64 {
        let gross = self.quantity * self.unit_price;
        if self.unit_price < 0.0 {
            gross
        } else {
            gross - self.discount.unwrap_or(0.0)
        }
    }

    pub fn is_ratio_metric(&self) -> bool {
        let d = self.description.to_ascii_lowercase();
        let amt = self.amount().abs();
        if amt > 100.0
            || d.contains("noi")
            || (d.contains("rent") && !d.contains("multiplier"))
            || d.contains("income")
            || d.contains("expense")
        {
            return false;
        }
        d.contains("dscr")
            || d.contains("coverage ratio")
            || d.contains("cap rate")
            || d.contains("cash-on-cash")
            || d.contains("ltv")
            || d.contains("loan-to-value")
            || d.contains("occupancy rate")
            || d.contains("vacancy rate")
            || d.contains("interest rate")
            || d.contains("irr")
            || d.contains("internal rate of return")
            || d.contains("yield")
            || d.contains("grm")
            || d.contains("rent multiplier")
            || d.contains("roi")
    }

    pub fn is_capital_or_valuation_item(&self) -> bool {
        let d = self.description.to_ascii_lowercase();
        if d.contains("noi")
            || d.contains("net operating income")
            || (d.contains("rent") && !d.contains("multiplier"))
            || d.contains("cash flow")
            || d.contains("vacancy")
            || d.contains("expense")
            || d.contains("debt service")
        {
            return false;
        }
        d.contains("purchase price")
            || d.contains("acquisition price")
            || d.contains("market value")
            || d.contains("property value")
            || d.contains("appraised value")
            || d.contains("after-repair value")
            || d.contains("arv")
            || d.contains("rehab budget")
            || d.contains("down payment")
            || d.contains("closing cost")
            || d.contains("senior debt")
            || d.contains("total debt")
            || d.contains("mortgage balance")
            || d.contains("loan balance")
            || d.contains("loan amount")
            || d.contains("principal balance")
            || d.contains("net equity")
            || d.contains("total equity")
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
    #[serde(default, alias = "type", alias = "archetype", alias = "template", alias = "document_type")]
    pub doc_type: DocumentType,
    pub title: Option<String>,
    #[serde(default, alias = "document_number", alias = "number")]
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
    #[serde(default, alias = "issuer", alias = "seller", alias = "landlord", alias = "sender")]
    pub from: PartyDetails,
    #[serde(default, alias = "recipient", alias = "buyer", alias = "client", alias = "tenant")]
    pub to: PartyDetails,
    #[serde(default, alias = "line_items")]
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
    #[serde(default, alias = "state", alias = "jurisdiction")]
    pub state_code: Option<String>,
    pub municipality: Option<String>,
    pub county: Option<String>,
    pub recordable: Option<bool>,
    pub notary_required: Option<bool>,
    pub year_built: Option<u32>,
    pub lease_package_mode: Option<bool>,
}

impl DocumentRequest {
    pub fn normalized_currency_code(&self) -> &str {
        let raw = self.currency.as_deref().map(str::trim).unwrap_or("");
        match raw.to_ascii_uppercase().as_str() {
            "" | "$" | "USD" | "US$" | "US DOLLAR" | "US DOLLARS" => "USD",
            "€" | "EUR" | "EURO" | "EUROS" => "EUR",
            "£" | "GBP" | "UK£" => "GBP",
            "¥" | "JPY" | "JP¥" => "JPY",
            "CNY" | "RMB" | "CN¥" => "CNY",
            "CA$" | "CAD" | "C$" => "CAD",
            "A$" | "AUD" | "AU$" => "AUD",
            "NZ$" | "NZD" => "NZD",
            "S$" | "SGD" => "SGD",
            "CHF" | "FR." | "FR" | "SFR" => "CHF",
            "₹" | "INR" | "RS" | "RS." => "INR",
            "R$" | "BRL" => "BRL",
            "KR" | "SEK" => "SEK",
            "NOK" => "NOK",
            "DKK" => "DKK",
            "AED" | "DHS" => "AED",
            _ => raw,
        }
    }

    pub fn currency_symbol(&self) -> &str {
        match self.normalized_currency_code() {
            "USD" => "$",
            "EUR" => "€",
            "GBP" => "£",
            "JPY" | "CNY" => "¥",
            "CAD" => "CA$",
            "AUD" => "A$",
            "NZD" => "NZ$",
            "SGD" => "S$",
            "CHF" => "CHF",
            "INR" => "₹",
            "BRL" => "R$",
            "SEK" | "NOK" | "DKK" => "kr",
            "AED" => "AED",
            s if !s.is_empty() => s,
            _ => "$",
        }
    }

    pub fn normalized_execution_mode(&self) -> &'static str {
        match self
            .execution_mode
            .as_deref()
            .unwrap_or("esign")
            .trim()
            .to_ascii_lowercase()
            .as_str()
        {
            "wet_ink" | "wet-ink" | "notarized_wet" | "notarized-wet" | "wet" | "physical" => "wet-ink",
            "ron" | "notarized_ron" | "notarized-ron" | "remote_online_notary" => "ron",
            _ => "esign",
        }
    }

    pub fn normalized_compliance_mode(&self) -> &str {
        let raw = self.compliance_mode.as_deref().map(str::trim).unwrap_or("none");
        match raw.to_ascii_lowercase().as_str() {
            "" | "none" | "standard" => "none",
            "peppol" | "peppol_bis" | "peppol-bis" => "peppol_bis",
            "factur-x" | "factur_x" | "eu_vat" | "eu-vat" => "eu_vat",
            "hipaa" | "hipaa_baa" | "hipaa-baa" => "hipaa_baa",
            "zatca" | "zatca_ksa" => "zatca",
            "irs_w9" | "irs-w9" | "w9" => "irs_w9",
            "pci_dss" | "pci-dss" | "pci" => "pci_dss",
            "soc2" | "soc2_badge" | "soc2-badge" | "iso27001" => "soc2_badge",
            "statutory_sworn" | "statutory-sworn" | "sworn" => "statutory_sworn",
            _ => raw,
        }
    }

    pub fn allows_negative_items(&self) -> bool {
        self.doc_type.allows_negative_items()
    }

    pub fn rent_increase_breakdown(&self) -> Option<(f64, f64, f64)> {
        if self.doc_type != DocumentType::RentIncreaseNotice || self.items.len() < 2 {
            return None;
        }
        let first_amt = self.items[0].amount();
        let second_amt = self.items[1].amount();
        if first_amt <= 0.0 || second_amt <= 0.0 {
            return None;
        }
        let second_desc = self.items[1].description.to_ascii_lowercase();
        let is_new_base_row = second_amt >= first_amt
            || second_desc.contains("new")
            || second_desc.contains("proposed")
            || second_desc.contains("revised")
            || second_desc.contains("updated")
            || second_desc.contains("effective");
        let extra_fees: f64 = self.items.get(2..).unwrap_or(&[]).iter().map(|i| i.amount()).sum();
        if is_new_base_row {
            let new_total = second_amt + extra_fees;
            let net_increase = new_total - first_amt;
            Some((first_amt, net_increase, new_total))
        } else {
            let net_increase = second_amt + extra_fees;
            let new_total = first_amt + net_increase;
            Some((first_amt, net_increase, new_total))
        }
    }

    pub fn subtotal(&self) -> f64 {
        match self.doc_type {
            DocumentType::RentIncreaseNotice => {
                if let Some((_prior, _increase, new_total)) = self.rent_increase_breakdown() {
                    new_total
                } else {
                    self.items.iter().map(|item| item.amount()).sum()
                }
            }
            DocumentType::RealEstateDealAnalysis | DocumentType::ScheduleOfRealEstateOwned => {
                let non_ratio: Vec<&LineItem> =
                    self.items.iter().filter(|i| !i.is_ratio_metric()).collect();
                let has_operating = non_ratio.iter().any(|i| !i.is_capital_or_valuation_item());
                let has_capital = non_ratio.iter().any(|i| i.is_capital_or_valuation_item());
                if has_operating && has_capital {
                    non_ratio
                        .into_iter()
                        .filter(|i| !i.is_capital_or_valuation_item())
                        .map(|i| i.amount())
                        .sum()
                } else {
                    non_ratio.into_iter().map(|i| i.amount()).sum()
                }
            }
            _ => self.items.iter().map(|item| item.amount()).sum(),
        }
    }

    pub fn compute_audit_sha256(&self) -> String {
        if let Some(ref h) = self.audit_hash {
            let trimmed = h.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(self.doc_type.url_slug().as_bytes());
        hasher.update(b"|");
        hasher.update(self.effective_doc_number().as_bytes());
        hasher.update(b"|");
        hasher.update(self.issue_date.as_deref().unwrap_or("2026-10-01").as_bytes());
        hasher.update(b"|");
        hasher.update(self.from.name.as_bytes());
        hasher.update(b"|");
        hasher.update(self.to.name.as_bytes());
        hasher.update(b"|");
        hasher.update(format!("{:.2}", self.grand_total()).as_bytes());
        hasher.update(b"|");
        hasher.update(self.items.len().to_string().as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn tax_total(&self) -> f64 {
        if self.items.iter().any(|i| i.tax_rate.is_some()) {
            self.items
                .iter()
                .map(|i| {
                    let rate = i.tax_rate.unwrap_or_else(|| {
                        if i.amount() < 0.0 {
                            0.0
                        } else {
                            self.tax_rate.unwrap_or(0.0)
                        }
                    });
                    i.amount() * (rate / 100.0)
                })
                .sum()
        } else if let Some(rate) = self.tax_rate {
            self.subtotal() * (rate / 100.0)
        } else {
            0.0
        }
    }

    pub fn grand_total(&self) -> f64 {
        let sub = self.subtotal();
        let tax = self.tax_total();
        let discount = self.discount_amount.unwrap_or(0.0);
        let raw = sub + tax - discount;
        if self.doc_type.allows_negative_items() {
            raw
        } else {
            raw.max(0.0)
        }
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
            self.normalized_currency_code()
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
        let trimmed = date_str.trim();
        let date_part = trimmed
            .split('T')
            .next()
            .unwrap_or(trimmed)
            .split_whitespace()
            .next()
            .unwrap_or(trimmed);

        let parsed_ymd: Option<(String, String, String)> = if date_part.contains('-') {
            let parts: Vec<&str> = date_part.split('-').collect();
            if parts.len() == 3 && parts[0].len() == 4 && parts[0].chars().all(|c| c.is_ascii_digit()) {
                let m = parts[1].parse::<u32>().ok().filter(|&v| (1..=12).contains(&v));
                let d = parts[2].parse::<u32>().ok().filter(|&v| (1..=31).contains(&v));
                if let (Some(mv), Some(dv)) = (m, d) {
                    Some((parts[0].to_string(), format!("{:02}", mv), format!("{:02}", dv)))
                } else {
                    None
                }
            } else {
                None
            }
        } else if date_part.contains('/') {
            let parts: Vec<&str> = date_part.split('/').collect();
            if parts.len() == 3 && parts[0].len() == 4 && parts[0].chars().all(|c| c.is_ascii_digit()) {
                // YYYY/MM/DD
                let m = parts[1].parse::<u32>().ok().filter(|&v| (1..=12).contains(&v));
                let d = parts[2].parse::<u32>().ok().filter(|&v| (1..=31).contains(&v));
                if let (Some(mv), Some(dv)) = (m, d) {
                    Some((parts[0].to_string(), format!("{:02}", mv), format!("{:02}", dv)))
                } else {
                    None
                }
            } else if parts.len() == 3 && parts[2].len() == 4 && parts[2].chars().all(|c| c.is_ascii_digit()) {
                // MM/DD/YYYY
                let m = parts[0].parse::<u32>().ok().filter(|&v| (1..=12).contains(&v));
                let d = parts[1].parse::<u32>().ok().filter(|&v| (1..=31).contains(&v));
                if let (Some(mv), Some(dv)) = (m, d) {
                    Some((parts[2].to_string(), format!("{:02}", mv), format!("{:02}", dv)))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some((y, m, d)) = parsed_ymd {
            let month_name = match m.as_str() {
                "01" => "January", "02" => "February", "03" => "March",
                "04" => "April", "05" => "May", "06" => "June",
                "07" => "July", "08" => "August", "09" => "September",
                "10" => "October", "11" => "November", "12" => "December",
                _ => m.as_str(),
            };
            let short_month = match m.as_str() {
                "01" => "Jan", "02" => "Feb", "03" => "Mar",
                "04" => "Apr", "05" => "May", "06" => "Jun",
                "07" => "Jul", "08" => "Aug", "09" => "Sep",
                "10" => "Oct", "11" => "Nov", "12" => "Dec",
                _ => m.as_str(),
            };

            match self.date_format.as_deref() {
                Some("us_slash") | Some("us") => format!("{}/{}/{}", m, d, y),
                Some("euro") | Some("eu") => format!("{}/{}/{}", d, m, y),
                Some("formal_prose") | Some("long") | Some("formal") | Some("us_long") => {
                    format!("{} {}, {}", month_name, d, y)
                }
                Some("uk_long") => format!("{} {} {}", d, month_name, y),
                Some("short_abbrev") | Some("short") => format!("{}-{}-{}", d, short_month, y),
                Some("short_dot") => format!("{}.{}.{}", d, m, y),
                _ => format!("{}-{}-{}", y, m, d), // default ISO
            }
        } else {
            date_str.to_string()
        }
    }

    pub fn effective_state_profile(&self) -> Option<&'static crate::state_laws::StateLeaseProfile> {
        if let Some(ref code) = self.state_code {
            if let Some(profile) = crate::state_laws::get_state_lease_profile(code) {
                return Some(profile);
            }
        }
        if let Some(muni) = self.effective_municipal_profile() {
            if let Some(profile) = crate::state_laws::get_state_lease_profile(muni.state_code) {
                return Some(profile);
            }
        }
        for cf in &self.custom_fields {
            let lbl = cf.label.trim().to_ascii_lowercase();
            if lbl == "state"
                || lbl == "state code"
                || lbl == "governing state"
                || lbl == "jurisdiction"
                || lbl == "governing law"
            {
                if let Some(profile) = crate::state_laws::get_state_lease_profile(&cf.value) {
                    return Some(profile);
                }
                if let Some(profile) = crate::state_laws::infer_state_from_address(&cf.value) {
                    return Some(profile);
                }
            }
            if lbl.contains("premises")
                || lbl.contains("property")
                || lbl.contains("address")
                || lbl.contains("target unit")
                || lbl.contains("managed portfolio")
                || lbl.contains("governing")
                || lbl.contains("location")
            {
                if let Some(profile) = crate::state_laws::infer_state_from_address(&cf.value) {
                    return Some(profile);
                }
            }
        }
        if self.doc_type.supports_jurisdiction_inference() {
            if let Some(ref addr) = self.to.address {
                if let Some(profile) = crate::state_laws::infer_state_from_address(addr) {
                    return Some(profile);
                }
            }
            if let Some(ref addr) = self.from.address {
                if let Some(profile) = crate::state_laws::infer_state_from_address(addr) {
                    return Some(profile);
                }
            }
        }
        None
    }

    pub fn effective_municipal_profile(
        &self,
    ) -> Option<&'static crate::state_laws::MunicipalLeaseSubProfile> {
        let explicit_state = self
            .state_code
            .as_deref()
            .and_then(crate::state_laws::get_state_lease_profile);

        let candidate = if let Some(ref muni) = self.municipality {
            crate::state_laws::get_municipal_sub_profile(muni)
        } else {
            None
        }
        .or_else(|| {
            for cf in &self.custom_fields {
                let lbl = cf.label.trim().to_ascii_lowercase();
                if lbl == "municipality" || lbl == "city" || lbl == "local ordinance" {
                    if let Some(profile) = crate::state_laws::get_municipal_sub_profile(&cf.value) {
                        return Some(profile);
                    }
                }
            }
            None
        });

        if let Some(muni_prof) = candidate {
            if let Some(st) = explicit_state {
                if muni_prof.state_code != st.code {
                    return None;
                }
            }
            return Some(muni_prof);
        }
        None
    }

    pub fn effective_county(&self) -> String {
        if let Some(ref c) = self.county {
            let trimmed = c.trim().trim_end_matches(" County").trim_end_matches(" county");
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
        for cf in &self.custom_fields {
            let lbl = cf.label.trim().to_ascii_lowercase();
            if lbl.contains("county") {
                let trimmed = cf
                    .value
                    .trim()
                    .trim_end_matches(" County")
                    .trim_end_matches(" county");
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }
        "Hamilton".to_string()
    }

    pub fn effective_year_built(&self) -> Option<u32> {
        if let Some(y) = self.year_built {
            return Some(y);
        }
        for cf in &self.custom_fields {
            let lbl = cf.label.trim().to_ascii_lowercase();
            if lbl.contains("year built") || lbl.contains("construction year") {
                for token in cf.value.split(|c: char| !c.is_ascii_digit()) {
                    if token.len() == 4 {
                        if let Ok(y) = token.parse::<u32>() {
                            if (1700..=2100).contains(&y) {
                                return Some(y);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn is_recordable_instrument(&self) -> bool {
        self.recordable
            .unwrap_or(self.doc_type == DocumentType::PropertyDeed)
    }

    pub fn is_notary_required(&self) -> bool {
        let mode_lower = self
            .execution_mode
            .as_deref()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        self.notary_required.unwrap_or(
            matches!(
                self.doc_type,
                DocumentType::PropertyDeed | DocumentType::MechanicsLienWaiver
            ) || matches!(
                mode_lower.as_str(),
                "notarized_wet" | "notarized-wet" | "notarized_ron" | "notarized-ron" | "ron" | "remote_online_notary"
            ),
        )
    }

    /// Normalizes currency, execution mode, and compliance mode, and validates finite numbers and negative line item rules.
    pub fn normalize_and_validate(&mut self) -> Result<(), String> {
        if let Some(tr) = self.tax_rate {
            if !tr.is_finite() || !(0.0..=100.0).contains(&tr) {
                return Err("Invalid tax_rate: must be a finite percentage between 0 and 100.".to_string());
            }
        }
        if let Some(disc) = self.discount_amount {
            if !disc.is_finite() || disc < 0.0 {
                return Err("Invalid discount_amount: must be a finite non-negative number.".to_string());
            }
        }
        if let Some(paid) = self.amount_paid {
            if !paid.is_finite() || paid < 0.0 {
                return Err("Invalid amount_paid: must be a finite non-negative number.".to_string());
            }
        }
        let allow_neg = self.doc_type.allows_negative_items();
        for (idx, item) in self.items.iter_mut().enumerate() {
            if !item.quantity.is_finite() || !item.unit_price.is_finite() {
                return Err(format!("Line item #{} has non-finite quantity or unit_price.", idx + 1));
            }
            if !allow_neg && (item.quantity < 0.0 || item.unit_price < 0.0) {
                return Err(format!(
                    "Line item #{} ('{}') cannot have negative quantity or unit_price on a '{}' document.",
                    idx + 1,
                    item.description,
                    self.doc_type.as_str()
                ));
            }
            if item.unit_price < 0.0 {
                item.discount = None;
            }
            if let Some(d) = item.discount {
                if !d.is_finite() || d < 0.0 {
                    return Err(format!("Line item #{} has invalid discount.", idx + 1));
                }
            }
            if let Some(tr) = item.tax_rate {
                if !tr.is_finite() || !(0.0..=100.0).contains(&tr) {
                    return Err(format!("Line item #{} has invalid tax_rate.", idx + 1));
                }
            }
        }
        if let Some(dec) = self.number_decimals.as_mut() {
            *dec = (*dec).min(4);
        }
        if self.currency.is_some() {
            let norm = self.normalized_currency_code().to_string();
            self.currency = Some(norm);
        }
        if self.compliance_mode.is_some() {
            let norm = self.normalized_compliance_mode().to_string();
            self.compliance_mode = Some(norm);
        }
        if self.execution_mode.is_some() {
            if self.notary_required.is_none() && self.is_notary_required() {
                self.notary_required = Some(true);
            }
            let norm = self.normalized_execution_mode().to_string();
            self.execution_mode = Some(norm);
        }
        Ok(())
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unlock_checkout_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pro_checkout_url: Option<String>,
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
    #[serde(default)]
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
/// Strips any leading 2-to-5 letter alphabetic archetype prefix followed by a hyphen.
fn extract_doc_base_id(doc_num: &str) -> &str {
    if let Some(dash_idx) = doc_num.find('-') {
        let prefix = &doc_num[..dash_idx];
        if (2..=5).contains(&prefix.len()) && prefix.chars().all(|c| c.is_ascii_alphabetic()) {
            let rest = &doc_num[dash_idx + 1..];
            if !rest.is_empty() {
                return rest;
            }
        }
    }
    doc_num
}



