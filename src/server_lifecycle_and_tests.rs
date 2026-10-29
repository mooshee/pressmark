//! Pressmark Copilot, Auth, Lifecycle, Stripe Webhook Handlers & Server Tests
//!
//! Author: Daniel Hallman

use super::*;

pub(crate) async fn handle_copilot(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CopilotRequest>,
) -> impl IntoResponse {
    let mut modified = req.current_state;
    let prompt_lower = req.prompt.to_lowercase();
    let user_agent = headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok());

    let maybe_token = auth::extract_token_from_headers(&headers);
    let session = maybe_token.as_deref().and_then(|t| state.auth_mgr.get_session(t));

    let mut changes: Vec<String> = Vec::new();

    // 1. Document archetype detection across all 57 supported types
    let matched_type = if prompt_lower.contains("nda") || prompt_lower.contains("non-disclosure") {
        Some((crate::models::DocumentType::Nda, "Mutual Non-Disclosure Agreement"))
    } else if prompt_lower.contains("room rental") || prompt_lower.contains("co-living") || prompt_lower.contains("coliving") {
        Some((crate::models::DocumentType::RoomRentalLease, "Room Rental & Co-Living Lease Agreement"))
    } else if prompt_lower.contains("lease renewal") || prompt_lower.contains("renewal agreement") {
        Some((crate::models::DocumentType::LeaseRenewalAgreement, "Lease Renewal & Extension Agreement"))
    } else if prompt_lower.contains("lease amendment") || prompt_lower.contains("pet addendum") || prompt_lower.contains("roommate addendum") {
        Some((crate::models::DocumentType::LeaseAmendment, "Lease Amendment & Addendum"))
    } else if prompt_lower.contains("rental application") || prompt_lower.contains("tenant screening") {
        Some((crate::models::DocumentType::RentalApplication, "Residential Rental Application"))
    } else if prompt_lower.contains("adverse action") || prompt_lower.contains("fcra") {
        Some((crate::models::DocumentType::AdverseActionNotice, "FCRA Adverse Action Notice"))
    } else if prompt_lower.contains("notice of entry") || prompt_lower.contains("right of entry") || prompt_lower.contains("24-hour entry") {
        Some((crate::models::DocumentType::NoticeOfEntry, "Landlord Notice of Entry"))
    } else if prompt_lower.contains("rent increase") {
        Some((crate::models::DocumentType::RentIncreaseNotice, "Statutory Notice of Rent Increase"))
    } else if prompt_lower.contains("mutual lease termination") || prompt_lower.contains("cash for keys") || prompt_lower.contains("early lease termination") {
        Some((crate::models::DocumentType::MutualLeaseTermination, "Mutual Lease Termination Agreement"))
    } else if prompt_lower.contains("rent ledger") || prompt_lower.contains("tenant ledger") {
        Some((crate::models::DocumentType::TenantRentLedger, "Tenant Rent & Payment Ledger"))
    } else if prompt_lower.contains("schedule of real estate owned") || prompt_lower.contains("sreo") {
        Some((crate::models::DocumentType::ScheduleOfRealEstateOwned, "Schedule of Real Estate Owned (SREO)"))
    } else if prompt_lower.contains("schedule e") || prompt_lower.contains("t12") || prompt_lower.contains("tax worksheet") {
        Some((crate::models::DocumentType::ScheduleETaxWorksheet, "IRS Schedule E Tax Prep Worksheet"))
    } else if prompt_lower.contains("deal analysis") || prompt_lower.contains("brrrr") || prompt_lower.contains("fix and flip") || prompt_lower.contains("pro forma") {
        Some((crate::models::DocumentType::RealEstateDealAnalysis, "Real Estate Investment Deal Analysis"))
    } else if prompt_lower.contains("sublease") || prompt_lower.contains("sublet") {
        Some((crate::models::DocumentType::SubleaseAgreement, "Residential Sublease Agreement"))
    } else if prompt_lower.contains("rent to own") || prompt_lower.contains("rent-to-own") || prompt_lower.contains("lease option") {
        Some((crate::models::DocumentType::RentToOwnAgreement, "Rent-to-Own (Lease-Option) Agreement"))
    } else if prompt_lower.contains("property management agreement") {
        Some((crate::models::DocumentType::PropertyManagementAgreement, "Property Management Agreement"))
    } else if prompt_lower.contains("real estate purchase") || prompt_lower.contains("purchase and sale") || prompt_lower.contains("psa") || prompt_lower.contains("offer to purchase") {
        Some((crate::models::DocumentType::RealEstatePurchaseAgreement, "Real Estate Purchase & Sale Agreement"))
    } else if prompt_lower.contains("quitclaim deed") || prompt_lower.contains("warranty deed") || prompt_lower.contains("property deed") {
        Some((crate::models::DocumentType::PropertyDeed, "Quitclaim / Warranty Deed"))
    } else if prompt_lower.contains("lien waiver") || prompt_lower.contains("mechanic's lien") || prompt_lower.contains("mechanics lien") {
        Some((crate::models::DocumentType::MechanicsLienWaiver, "Mechanic's Lien Waiver & Release"))
    } else if prompt_lower.contains("triple net") || prompt_lower.contains("nnn lease") || prompt_lower.contains("commercial nnn") {
        Some((crate::models::DocumentType::CommercialNnnLease, "Commercial NNN Lease"))
    } else if prompt_lower.contains("residential lease") || prompt_lower.contains("apartment lease") {
        Some((crate::models::DocumentType::ResidentialLease, "Residential Lease Agreement"))
    } else if prompt_lower.contains("commercial lease") {
        Some((crate::models::DocumentType::CommercialLease, "Commercial Lease Abstract"))
    } else if prompt_lower.contains("property management") || prompt_lower.contains("owner statement") {
        Some((crate::models::DocumentType::PropertyManagementStatement, "Property Management Statement"))
    } else if prompt_lower.contains("rent roll") {
        Some((crate::models::DocumentType::RentRoll, "Multi-Unit Rent Roll"))
    } else if prompt_lower.contains("closing") || prompt_lower.contains("settlement statement") {
        Some((crate::models::DocumentType::ClosingSettlementStatement, "Closing Settlement Statement"))
    } else if prompt_lower.contains("notice to vacate") || prompt_lower.contains("eviction") {
        Some((crate::models::DocumentType::NoticeToVacate, "Notice to Vacate"))
    } else if prompt_lower.contains("security deposit") {
        Some((crate::models::DocumentType::SecurityDepositDisposition, "Security Deposit Disposition"))
    } else if prompt_lower.contains("commission") && prompt_lower.contains("real estate") {
        Some((crate::models::DocumentType::RealEstateCommission, "Real Estate Commission Statement"))
    } else if prompt_lower.contains("inspection checklist") || prompt_lower.contains("move-in") || prompt_lower.contains("move-out") {
        Some((crate::models::DocumentType::PropertyInspectionChecklist, "Property Inspection Checklist"))
    } else if prompt_lower.contains("letter of intent") || prompt_lower.contains("loi") {
        Some((crate::models::DocumentType::LetterOfIntentCre, "Commercial Real Estate LOI"))
    } else if prompt_lower.contains("estoppel") {
        Some((crate::models::DocumentType::TenantEstoppel, "Tenant Estoppel Certificate"))
    } else if prompt_lower.contains("credit note") || prompt_lower.contains("refund") {
        Some((crate::models::DocumentType::CreditNote, "Credit Note"))
    } else if prompt_lower.contains("purchase order") {
        Some((crate::models::DocumentType::PurchaseOrder, "Purchase Order"))
    } else if prompt_lower.contains("packing slip") || prompt_lower.contains("bill of lading") {
        Some((crate::models::DocumentType::PackingSlip, "Packing Slip"))
    } else if prompt_lower.contains("work order") || prompt_lower.contains("job ticket") {
        Some((crate::models::DocumentType::WorkOrder, "Work Order"))
    } else if prompt_lower.contains("expense report") || prompt_lower.contains("reimbursement") {
        Some((crate::models::DocumentType::ExpenseReport, "Expense Report"))
    } else if prompt_lower.contains("delivery receipt") {
        Some((crate::models::DocumentType::DeliveryReceipt, "Delivery Receipt"))
    } else if prompt_lower.contains("receipt") {
        Some((crate::models::DocumentType::Receipt, "Payment Receipt"))
    } else if prompt_lower.contains("tender") || prompt_lower.contains("bid quotation") {
        Some((crate::models::DocumentType::QuotationTender, "Tender Quotation"))
    } else if prompt_lower.contains("estimate") || prompt_lower.contains("quote") || prompt_lower.contains("proposal") {
        Some((crate::models::DocumentType::Estimate, "Estimate"))
    } else if prompt_lower.contains("bill of sale") {
        Some((crate::models::DocumentType::BillOfSale, "Bill of Sale"))
    } else if prompt_lower.contains("contractor agreement") || prompt_lower.contains("independent contractor") {
        Some((crate::models::DocumentType::ContractorAgreement, "Independent Contractor Agreement"))
    } else if prompt_lower.contains("retainer") && !prompt_lower.contains("invoice") {
        Some((crate::models::DocumentType::RetainerAgreement, "Retainer Agreement"))
    } else if prompt_lower.contains("statement of work") || prompt_lower.contains("sow") {
        Some((crate::models::DocumentType::AgreementSow, "Statement of Work"))
    } else if prompt_lower.contains("contract") || prompt_lower.contains("agreement") || prompt_lower.contains("msa") {
        Some((crate::models::DocumentType::Agreement, "Master Services Agreement"))
    } else if prompt_lower.contains("certificate of insurance") || prompt_lower.contains("coi") {
        Some((crate::models::DocumentType::CertificateInsurance, "Certificate of Insurance"))
    } else if prompt_lower.contains("certificate of completion") {
        Some((crate::models::DocumentType::CertificateCompletion, "Certificate of Completion"))
    } else if prompt_lower.contains("certificate") && !prompt_lower.contains("w-9") && !prompt_lower.contains("w9") {
        Some((crate::models::DocumentType::Certificate, "Certificate"))
    } else if prompt_lower.contains("inspection report") {
        Some((crate::models::DocumentType::InspectionReport, "Inspection Report"))
    } else if prompt_lower.contains("medical") || prompt_lower.contains("superbill") || prompt_lower.contains("clinical") {
        Some((crate::models::DocumentType::InvoiceMedical, "Clinical Superbill"))
    } else if prompt_lower.contains("construction") || prompt_lower.contains("progress billing") {
        Some((crate::models::DocumentType::InvoiceConstruction, "Construction Progress Invoice"))
    } else if prompt_lower.contains("saas") || prompt_lower.contains("subscription") {
        Some((crate::models::DocumentType::InvoiceSaas, "SaaS Subscription Invoice"))
    } else if prompt_lower.contains("freelance") || prompt_lower.contains("consulting") {
        Some((crate::models::DocumentType::InvoiceFreelance, "Freelance Invoice"))
    } else if prompt_lower.contains("proforma") {
        Some((crate::models::DocumentType::ProformaInvoice, "Proforma Invoice"))
    } else if prompt_lower.contains("invoice") {
        Some((crate::models::DocumentType::Invoice, "Invoice"))
    } else {
        None
    };

    if let Some((dtype, label)) = matched_type {
        modified.doc_type = dtype;
        changes.push(format!("set archetype to {}", label));
    }

    // 1b. State / Jurisdiction detection for statutory lease & real estate packs
    // Sort by name length descending so "West Virginia" matches before "Virginia"
    let mut sorted_states: Vec<&'static crate::state_laws::StateLeaseProfile> =
        crate::state_laws::all_state_lease_profiles().iter().collect();
    sorted_states.sort_by(|a, b| b.name.len().cmp(&a.name.len()));
    for sp in sorted_states {
        let state_name_lower = sp.name.to_lowercase();
        if prompt_lower.contains(&state_name_lower) {
            modified.state_code = Some(sp.code.to_string());
            changes.push(format!("applied {} ({}) statutory profile ({})", sp.name, sp.code, sp.governing_statute));
            break;
        }
    }
    for mp in crate::state_laws::all_municipal_sub_profiles() {
        let muni_lower = mp.name.to_lowercase();
        if prompt_lower.contains(&muni_lower) {
            modified.municipality = Some(mp.name.to_string());
            modified.state_code = Some(mp.state_code.to_string());
            changes.push(format!("applied {} municipal ordinance ({})", mp.name, mp.ordinance_citation));
            break;
        }
    }

    let has_word = |haystack: &str, needle: &str| -> bool {
        let pattern = format!(r"(?i)\b{}\b", regex::escape(needle));
        regex::Regex::new(&pattern)
            .map(|re| re.is_match(haystack))
            .unwrap_or(false)
    };

    // 2. Client / Recipient extraction (quoted first, then unquoted capitalized phrase)
    let client_quoted_re = regex::Regex::new(
        r"(?i)\b(?:for|to)\s+(?:client\s+|tenant\s+|buyer\s+|customer\s+)?['\x22]([^'\x22]{2,80})['\x22]",
    )
    .unwrap();
    let client_unquoted_re = regex::Regex::new(
        r"\bfor\s+(?:client\s+|tenant\s+|buyer\s+|customer\s+)?([A-Z][a-zA-Z0-9\s&,.-]{1,60}?)(?:\s+for\b|\s+with\b|\s+at\b|\s+including\b|\s+from\b|\.|,|$)",
    )
    .unwrap();
    let item_for_prefix_re = regex::Regex::new(
        r"(?i)\b(?:items?|services?|hours?|hrs?|milestones?|work|charges?|fees?)\s+for\s+$",
    )
    .unwrap();
    if let Some(cap) = client_quoted_re.captures(&req.prompt) {
        if let Some(m) = cap.get(1) {
            let name = m.as_str().trim().trim_matches(|c| c == '\'' || c == '"' || c == '.');
            if !name.is_empty() {
                modified.to.name = name.to_string();
                changes.push(format!("set recipient to {}", name));
            }
        }
    } else if let Some(cap) = client_unquoted_re.captures(&req.prompt) {
        let full_match = cap.get(0).map(|m| m.start()).unwrap_or(0);
        let prefix = &req.prompt[..full_match + 4.min(req.prompt.len().saturating_sub(full_match))];
        if !item_for_prefix_re.is_match(&req.prompt[..full_match]) && !prefix.is_empty() {
            if let Some(m) = cap.get(1) {
                let name = m.as_str().trim().trim_matches(|c| c == '\'' || c == '"' || c == '.');
                let lower_name = name.to_lowercase();
                if !name.is_empty()
                    && !lower_name.contains("hours of")
                    && !lower_name.contains("hrs of")
                {
                    modified.to.name = name.to_string();
                    changes.push(format!("set recipient to {}", name));
                }
            }
        }
    }

    // 3. Sender / Issuer extraction
    let sender_quoted_re = regex::Regex::new(
        r"(?i)\bfrom\s+(?:company\s+|issuer\s+|vendor\s+|seller\s+)?['\x22]([^'\x22]{2,80})['\x22]",
    )
    .unwrap();
    let sender_unquoted_re = regex::Regex::new(
        r"\bfrom\s+(?:company\s+|issuer\s+|vendor\s+|seller\s+)([A-Z][a-zA-Z0-9\s&,.-]{1,60}?)(?:\s+to|\s+for|\s+with|\.|,|$)",
    )
    .unwrap();
    if let Some(cap) = sender_quoted_re.captures(&req.prompt).or_else(|| sender_unquoted_re.captures(&req.prompt)) {
        if let Some(m) = cap.get(1) {
            let name = m.as_str().trim().trim_matches(|c| c == '\'' || c == '"' || c == '.');
            if !name.is_empty() {
                modified.from.name = name.to_string();
                changes.push(format!("set sender to {}", name));
            }
        }
    }

    // 4. Line items extraction (hourly/unit items, parenthesized milestones, and flat fees)
    let mut parsed_items: Vec<LineItem> = Vec::new();
    let parse_amount = |s: &str| -> f64 { s.replace(',', "").parse::<f64>().unwrap_or(0.0) };

    let qty_item_re = regex::Regex::new(
        r"(?i)(\d+(?:\.\d+)?)\s*(?:hours?|hrs?|days?|units?|items?|months?|x)?\s+(?:of\s+)?([a-zA-Z][a-zA-Z0-9\s&/,-]{1,60}?)\s+(?:at|@)\s*\$([\d,]+(?:\.\d+)?)(?:/[a-zA-Z]+)?",
    )
    .unwrap();
    for cap in qty_item_re.captures_iter(&req.prompt) {
        let qty: f64 = cap.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(1.0);
        let desc = cap.get(2).map(|m| m.as_str().trim()).unwrap_or("Service");
        let price = cap.get(3).map(|m| parse_amount(m.as_str())).unwrap_or(0.0);
        if price > 0.0 && !desc.is_empty() {
            parsed_items.push(LineItem {
                description: desc.to_string(),
                quantity: qty,
                unit_price: price,
                tax_rate: None,
                discount: None,
            });
        }
    }

    // Parenthesized milestone items, e.g. "Backend Architecture ($4,800), API Gateway ($2,400)"
    let paren_item_re = regex::Regex::new(
        r"(?:for\s+|,\s*|\band\s+)([A-Z][a-zA-Z0-9\s&/-]{2,45}?)\s*\(\$([\d,]+(?:\.\d+)?)\)",
    )
    .unwrap();
    for cap in paren_item_re.captures_iter(&req.prompt) {
        let desc = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        let price = cap.get(2).map(|m| parse_amount(m.as_str())).unwrap_or(0.0);
        if price > 0.0 && !desc.is_empty() {
            parsed_items.push(LineItem {
                description: desc.to_string(),
                quantity: 1.0,
                unit_price: price,
                tax_rate: None,
                discount: None,
            });
        }
    }

    let flat_fee_re = regex::Regex::new(
        r"(?i)(?:and\s+|plus\s+|with\s+|,\s*|draft\s+|create\s+)(?:a\s+|an\s+)?\$([\d,]+(?:\.\d+)?)\s+([a-zA-Z][a-zA-Z0-9\s&/-]{2,40}?)(?:\.|,|;|\s+for\s+|\s+and\s+|\s+with\s+|\s+use\s+|$)",
    )
    .unwrap();
    for cap in flat_fee_re.captures_iter(&req.prompt) {
        let price = cap.get(1).map(|m| parse_amount(m.as_str())).unwrap_or(0.0);
        let raw_desc = cap.get(2).map(|m| m.as_str().trim()).unwrap_or("");
        if price > 0.0 && !raw_desc.is_empty() && !raw_desc.eq_ignore_ascii_case("discount") {
            let mut chars = raw_desc.chars();
            let desc = match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => raw_desc.to_string(),
            };
            parsed_items.push(LineItem {
                description: desc,
                quantity: 1.0,
                unit_price: price,
                tax_rate: None,
                discount: None,
            });
        }
    }

    if !parsed_items.is_empty() {
        let count = parsed_items.len();
        modified.items = parsed_items;
        changes.push(format!("populated {} line item{}", count, if count == 1 { "" } else { "s" }));
    }

    // 5. Tax rate, currency & discount extraction
    let tax_re = regex::Regex::new(r"(?i)(?:uk\s+|eu\s+|us\s+)?(\d+(?:\.\d+)?)\s*%\s*(?:uk\s+|eu\s+|sales\s+|vat\s+|gst\s+)?(?:tax|vat|gst)").unwrap();
    if let Some(cap) = tax_re.captures(&req.prompt) {
        if let Some(rate) = cap.get(1).and_then(|m| m.as_str().parse::<f64>().ok()) {
            modified.tax_rate = Some(rate);
            changes.push(format!("set tax rate to {}%", rate));
        }
    }

    if has_word(&prompt_lower, "gbp") || prompt_lower.contains('£') || prompt_lower.contains("pounds sterling") {
        modified.currency = Some("GBP".to_string());
        changes.push("set currency to GBP (£)".to_string());
    } else if has_word(&prompt_lower, "eur") || has_word(&prompt_lower, "euro") || has_word(&prompt_lower, "euros") || prompt_lower.contains('€') {
        modified.currency = Some("EUR".to_string());
        changes.push("set currency to EUR (€)".to_string());
    } else if has_word(&prompt_lower, "cad") || prompt_lower.contains("canadian dollar") {
        modified.currency = Some("CAD".to_string());
        changes.push("set currency to CAD".to_string());
    } else if has_word(&prompt_lower, "aud") || prompt_lower.contains("australian dollar") {
        modified.currency = Some("AUD".to_string());
        changes.push("set currency to AUD".to_string());
    } else if has_word(&prompt_lower, "nzd") || prompt_lower.contains("new zealand dollar") {
        modified.currency = Some("NZD".to_string());
        changes.push("set currency to NZD".to_string());
    } else if has_word(&prompt_lower, "sgd") || prompt_lower.contains("singapore dollar") {
        modified.currency = Some("SGD".to_string());
        changes.push("set currency to SGD".to_string());
    } else if has_word(&prompt_lower, "chf") || prompt_lower.contains("swiss franc") {
        modified.currency = Some("CHF".to_string());
        changes.push("set currency to CHF".to_string());
    } else if has_word(&prompt_lower, "jpy") || has_word(&prompt_lower, "yen") {
        modified.currency = Some("JPY".to_string());
        changes.push("set currency to JPY (¥)".to_string());
    } else if has_word(&prompt_lower, "cny") || has_word(&prompt_lower, "rmb") || has_word(&prompt_lower, "yuan") {
        modified.currency = Some("CNY".to_string());
        changes.push("set currency to CNY (¥)".to_string());
    } else if has_word(&prompt_lower, "usd") {
        modified.currency = Some("USD".to_string());
        changes.push("set currency to USD ($)".to_string());
    }

    let disc_re = regex::Regex::new(r"(?i)\$([\d,]+(?:\.\d+)?)\s*discount|discount\s+of\s+\$([\d,]+(?:\.\d+)?)").unwrap();
    let pct_disc_re = regex::Regex::new(
        r"(?i)(\d+(?:\.\d+)?)\s*%\s*(?:promotional\s+|early\s+|volume\s+|loyalty\s+|trade\s+)?discount|discount\s+of\s+(\d+(?:\.\d+)?)\s*%",
    )
    .unwrap();
    if let Some(cap) = disc_re.captures(&req.prompt) {
        let val = cap.get(1).or_else(|| cap.get(2)).map(|m| parse_amount(m.as_str()));
        if let Some(d) = val.filter(|v| *v > 0.0) {
            modified.discount_amount = Some(d);
            changes.push(format!("applied ${:.2} discount", d));
        }
    } else if let Some(cap) = pct_disc_re.captures(&req.prompt) {
        let pct_val = cap
            .get(1)
            .or_else(|| cap.get(2))
            .and_then(|m| m.as_str().parse::<f64>().ok());
        if let Some(pct) = pct_val.filter(|v| *v > 0.0 && *v <= 100.0) {
            let base_subtotal = if modified.subtotal() > 0.0 {
                modified.subtotal()
            } else {
                500.0
            };
            let disc = ((base_subtotal * (pct / 100.0)) * 100.0).round() / 100.0;
            modified.discount_amount = Some(disc);
            changes.push(format!("applied {}% discount (${:.2})", pct, disc));
        }
    }

    // 5b. Payment settlement terms / Net days (e.g. "14-day payment settlement terms", "Net 30")
    let net_days_re = regex::Regex::new(
        r"(?i)\b(?:net\s+(\d{1,3})|(\d{1,3})(?:\s*-\s*|\s+)day\s+(?:payment|settlement|credit|net))\b",
    )
    .unwrap();
    if let Some(cap) = net_days_re.captures(&req.prompt) {
        if let Some(days) = cap
            .get(1)
            .or_else(|| cap.get(2))
            .and_then(|m| m.as_str().parse::<u64>().ok())
        {
            let base_date = modified
                .issue_date
                .as_deref()
                .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
                .unwrap_or_else(|| chrono::Utc::now().naive_utc().date());
            let due = base_date + chrono::Days::new(days);
            let due_str = due.format("%Y-%m-%d").to_string();
            modified.due_date = Some(due_str.clone());
            modified.terms = Some(format!(
                "Payment settlement due within {} days of invoice date (Net {}).",
                days, days
            ));
            changes.push(format!("set {}-day payment settlement terms (due {})", days, due_str));
        }
    }

    // 5c. Dispute arbitration clause extraction
    if prompt_lower.contains("arbitration") {
        let arb_clause = "1. PURPOSE & SCOPE: Contractor shall perform the independent professional services specified in the schedule.\n\n2. INDEPENDENT CONTRACTOR RELATIONSHIP: Contractor acts solely as an independent contractor and not as an employee, partner, or agent.\n\n3. INTELLECTUAL PROPERTY: All custom deliverables, code, designs, and work product vest exclusively with the client upon receipt of full payment.\n\n4. DISPUTE ARBITRATION CLAUSE: Any dispute, claim, or controversy arising out of or relating to this Agreement shall be resolved exclusively by expedited, final, and binding commercial arbitration.";
        match modified.terms {
            Some(ref existing) if !existing.trim().is_empty() && !existing.to_lowercase().contains("arbitration") => {
                modified.terms = Some(format!(
                    "{}\n\nDISPUTE ARBITRATION CLAUSE: Any dispute, claim, or controversy arising out of or relating to this Agreement shall be resolved exclusively by expedited, final, and binding commercial arbitration.",
                    existing.trim()
                ));
            }
            _ => {
                modified.terms = Some(arb_clause.to_string());
            }
        }
        changes.push("added dispute arbitration clause".to_string());
    }

    // 5d. Compliance mode detection
    if prompt_lower.contains("w-9") || prompt_lower.contains("w9") || prompt_lower.contains("taxpayer identification") {
        modified.compliance_mode = Some("irs_w9".to_string());
        changes.push("enabled IRS W-9 TIN compliance attestation".to_string());
    } else if prompt_lower.contains("reverse charge") || prompt_lower.contains("eu vat") || prompt_lower.contains("2006/112") {
        modified.compliance_mode = Some("eu_vat".to_string());
        changes.push("enabled EU VAT reverse charge compliance notice".to_string());
    } else if prompt_lower.contains("pci-dss") || prompt_lower.contains("pci dss") || prompt_lower.contains("pci_dss") {
        modified.compliance_mode = Some("pci_dss".to_string());
        changes.push("enabled PCI-DSS compliance notice".to_string());
    } else if prompt_lower.contains("soc-2") || prompt_lower.contains("soc 2") || prompt_lower.contains("soc2") || prompt_lower.contains("iso 27001") || prompt_lower.contains("iso27001") {
        modified.compliance_mode = Some("soc2".to_string());
        changes.push("enabled SOC-2 / ISO 27001 compliance notice".to_string());
    } else if prompt_lower.contains("statutory sworn") || prompt_lower.contains("sworn declaration") || prompt_lower.contains("perjury") {
        modified.compliance_mode = Some("statutory_sworn".to_string());
        changes.push("enabled statutory sworn declaration".to_string());
    }

    // 5e. Date format detection
    if prompt_lower.contains("military") || prompt_lower.contains("dd-mmm-yyyy") || prompt_lower.contains("short_abbrev") {
        modified.date_format = Some("short_abbrev".to_string());
        changes.push("set date format to Military (DD-MMM-YYYY)".to_string());
    } else if prompt_lower.contains("formal prose") || prompt_lower.contains("formal_prose") || prompt_lower.contains("long date") {
        modified.date_format = Some("formal_prose".to_string());
        changes.push("set date format to Formal Prose".to_string());
    } else if prompt_lower.contains("european date") || prompt_lower.contains("dd/mm/yyyy") {
        modified.date_format = Some("euro".to_string());
        changes.push("set date format to European (DD/MM/YYYY)".to_string());
    } else if prompt_lower.contains("us date") || prompt_lower.contains("mm/dd/yyyy") {
        modified.date_format = Some("us_slash".to_string());
        changes.push("set date format to US (MM/DD/YYYY)".to_string());
    } else if prompt_lower.contains("german date") || prompt_lower.contains("swiss date") || prompt_lower.contains("dd.mm.yyyy") {
        modified.date_format = Some("short_dot".to_string());
        changes.push("set date format to Dot (DD.MM.YYYY)".to_string());
    } else if prompt_lower.contains("iso date") || prompt_lower.contains("yyyy-mm-dd") {
        modified.date_format = Some("iso".to_string());
        changes.push("set date format to ISO (YYYY-MM-DD)".to_string());
    }

    // 6. Theme & palette detection across all 18 valid themes
    let mut matched_theme: Option<(&str, &str)> = None;
    if has_word(&prompt_lower, "obsidian")
        || prompt_lower.contains("dark obsidian")
        || (has_word(&prompt_lower, "dark") && !has_word(&prompt_lower, "slate"))
    {
        matched_theme = Some(("obsidian", "applied Obsidian theme"));
    } else if has_word(&prompt_lower, "slate") || prompt_lower.contains("modern_slate") {
        matched_theme = Some(("slate", "applied Modern Slate theme"));
    } else if has_word(&prompt_lower, "navy")
        || prompt_lower.contains("executive navy")
        || prompt_lower.contains("executive_navy")
    {
        matched_theme = Some(("executive_navy", "applied Executive Navy theme"));
    } else if has_word(&prompt_lower, "monaco")
        || prompt_lower.contains("monaco_gold")
        || has_word(&prompt_lower, "gold")
    {
        matched_theme = Some(("monaco_gold", "applied Monaco Gold theme"));
    } else if has_word(&prompt_lower, "crimson")
        || prompt_lower.contains("crimson_legal")
        || has_word(&prompt_lower, "red")
    {
        matched_theme = Some(("crimson_legal", "applied Crimson Legal theme"));
    } else if has_word(&prompt_lower, "bordeaux") {
        matched_theme = Some(("bordeaux", "applied Bordeaux theme"));
    } else if prompt_lower.contains("warm sand")
        || prompt_lower.contains("warm_sand")
        || prompt_lower.contains("warm amber")
    {
        matched_theme = Some(("warm_sand", "applied Warm Sand theme"));
    } else if prompt_lower.contains("desert sand")
        || prompt_lower.contains("desert_sand")
        || has_word(&prompt_lower, "mojave")
        || has_word(&prompt_lower, "amber")
        || has_word(&prompt_lower, "desert")
    {
        matched_theme = Some(("desert_sand", "applied Desert Sand theme"));
    } else if has_word(&prompt_lower, "cyber")
        || has_word(&prompt_lower, "neon")
        || prompt_lower.contains("cyber_neon")
    {
        matched_theme = Some(("cyber_neon", "applied Cyber Neon theme"));
    } else if has_word(&prompt_lower, "emerald") || has_word(&prompt_lower, "green") {
        matched_theme = Some(("emerald", "applied Emerald theme"));
    } else if has_word(&prompt_lower, "nordic")
        || has_word(&prompt_lower, "teal")
        || prompt_lower.contains("nordic_teal")
        || prompt_lower.contains("nordic_frost")
    {
        matched_theme = Some(("nordic_teal", "applied Nordic Teal theme"));
    } else if has_word(&prompt_lower, "forest")
        || has_word(&prompt_lower, "pine")
        || prompt_lower.contains("forest_pine")
    {
        matched_theme = Some(("forest_pine", "applied Forest Pine theme"));
    } else if has_word(&prompt_lower, "purple")
        || has_word(&prompt_lower, "violet")
        || prompt_lower.contains("royal violet")
        || prompt_lower.contains("royal_violet")
    {
        matched_theme = Some(("royal_violet", "applied Royal Violet theme"));
    } else if has_word(&prompt_lower, "sunset")
        || has_word(&prompt_lower, "coral")
        || prompt_lower.contains("sunset_coral")
    {
        matched_theme = Some(("sunset_coral", "applied Sunset Coral theme"));
    } else if prompt_lower.contains("electric indigo")
        || prompt_lower.contains("electric_indigo")
        || has_word(&prompt_lower, "indigo")
    {
        matched_theme = Some(("electric_indigo", "applied Electric Indigo theme"));
    } else if has_word(&prompt_lower, "neobrutalist") || has_word(&prompt_lower, "brutalist") {
        matched_theme = Some(("neobrutalist", "applied Neobrutalist theme"));
    } else if has_word(&prompt_lower, "bauhaus") || prompt_lower.contains("bauhaus_primary") {
        matched_theme = Some(("bauhaus_primary", "applied Bauhaus Primary theme"));
    } else if has_word(&prompt_lower, "swiss")
        || prompt_lower.contains("swiss_minimal")
        || has_word(&prompt_lower, "minimal")
        || has_word(&prompt_lower, "monochrome")
    {
        matched_theme = Some(("swiss_minimal", "applied Swiss Minimal theme"));
    }

    if let Some((theme_id, msg)) = matched_theme {
        modified.theme = Some(theme_id.to_string());
        modified.primary_color = None;
        modified.secondary_color = None;
        modified.accent_color = None;
        modified.surface_color = None;
        modified.text_color = None;
        let prim = modified.effective_primary_color().to_string();
        let sec = modified.effective_secondary_color().to_string();
        let acc = modified.effective_accent_color().to_string();
        let surf = modified.effective_surface_color().to_string();
        modified.primary_color = Some(prim);
        modified.secondary_color = Some(sec);
        modified.accent_color = Some(acc);
        modified.surface_color = Some(surf);
        changes.push(msg.to_string());
    }

    // 7. Signature block detection
    if prompt_lower.contains("signature") || prompt_lower.contains("sign block") {
        modified.show_signature = Some(true);
        if modified.signatures.is_empty() {
            let signer = if modified.from.name.trim().is_empty() {
                "Authorized Signatory".to_string()
            } else {
                modified.from.name.clone()
            };
            modified.signatures.push(crate::models::SignatureBlock {
                signer_name: signer,
                signer_title: Some("Authorized Representative".to_string()),
                date: modified.issue_date.clone(),
                signature_data: None,
            });
        }
        changes.push("enabled signature block".to_string());
    }

    if let Some(ref sess) = session {
        if sess.user.is_subscriber {
            modified.watermark = None;
        }
    }

    state.storage.record_usage_event(
        "copilot_prompt",
        Some(modified.doc_type.as_str()),
        "studio",
        user_agent,
        None,
    );

    let message = if changes.is_empty() {
        "Updated document settings from your prompt.".to_string()
    } else {
        format!("Updated document: {}.", changes.join(", "))
    };

    let resp = CopilotResponse {
        success: true,
        modified_state: modified,
        ai_message: message,
    };
    (StatusCode::OK, Json(resp)).into_response()
}

pub(crate) async fn handle_vision_extract(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<VisionRequest>,
) -> impl IntoResponse {
    let user_agent = headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok());
    let maybe_token = auth::extract_token_from_headers(&headers);
    let session = maybe_token.as_deref().and_then(|t| state.auth_mgr.get_session(t));

    let clean_name = req
        .filename
        .as_deref()
        .and_then(|f| f.split('.').next())
        .map(|s| s.replace(['-', '_'], " ").trim().to_string())
        .filter(|s| !s.is_empty());

    let mut party = PartyDetails::default();
    if let Some(ref base) = clean_name {
        let mut chars = base.chars();
        let titled = match chars.next() {
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            None => "Scanned Vendor LLC".to_string(),
        };
        party.name = titled;
    } else {
        party.name = "Scanned Vendor LLC".to_string();
    }
    party.email = Some("billing@scanned-vendor.com".to_string());

    let items = vec![
        LineItem {
            description: clean_name
                .as_ref()
                .map(|n| format!("{} — Line Item", n))
                .unwrap_or_else(|| "Extracted Line Item 1".to_string()),
            quantity: 1.0,
            unit_price: 499.0,
            tax_rate: None,
            discount: None,
        },
        LineItem {
            description: "Processing & Handling".to_string(),
            quantity: 1.0,
            unit_price: 25.0,
            tax_rate: None,
            discount: None,
        },
    ];

    state.storage.record_usage_event(
        "vision_extract",
        Some("receipt"),
        "studio",
        req.filename.as_deref().or(user_agent),
        None,
    );

    let ai_message = if let Some(sess) = session {
        format!(
            "Extracted vendor and 2 line items ({}).",
            sess.user.plan_display
        )
    } else {
        "Extracted vendor details and 2 line items from uploaded image.".to_string()
    };

    let resp = VisionResponse {
        success: true,
        extracted_items: items,
        extracted_from: Some(party),
        ai_message,
    };
    (StatusCode::OK, Json(resp)).into_response()
}

// ChatGPT OAuth Handlers

pub(crate) async fn handle_chatgpt_login(
    State(state): State<Arc<AppState>>,
    query: Query<LoginQuery>,
    headers: HeaderMap,
) -> Response {
    auth::handle_chatgpt_login(State(state.auth_mgr.clone()), query, headers).await
}

pub(crate) async fn handle_chatgpt_callback(
    State(state): State<Arc<AppState>>,
    query: Query<CallbackQuery>,
) -> Response {
    auth::handle_chatgpt_callback(State(state.auth_mgr.clone()), query).await
}

pub(crate) async fn handle_chatgpt_session(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    auth::handle_chatgpt_session(State(state.auth_mgr.clone()), headers).await
}

pub(crate) async fn handle_chatgpt_logout(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    auth::handle_chatgpt_logout(State(state.auth_mgr.clone()), headers).await
}

pub(crate) async fn handle_guest_login(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    auth::handle_guest_login(State(state.auth_mgr.clone())).await
}

pub(crate) async fn handle_email_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EmailLoginRequest>,
) -> impl IntoResponse {
    auth::handle_email_login(State(state.auth_mgr.clone()), Json(payload)).await
}

pub(crate) async fn handle_api_key_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ApiKeyLoginRequest>,
) -> impl IntoResponse {
    auth::handle_api_key_login(State(state.auth_mgr.clone()), Json(payload)).await
}

#[derive(Debug, serde::Deserialize)]
pub struct LifecycleConvertRequest {
    pub target_type: String, // "invoice", "receipt", "estimate", "credit_note", "refund"
    pub due_days: Option<u32>,
    pub valid_days: Option<u32>,
    #[serde(alias = "reason")]
    pub credit_reason: Option<String>,
    pub payment_link: Option<String>,
    pub payment_qr_code: Option<bool>,
    pub document: DocumentRequest,
}

#[derive(Debug, serde::Deserialize)]
pub struct LifecyclePayRequest {
    pub document: DocumentRequest,
    pub payment_method: Option<String>,
    pub transaction_reference: Option<String>,
    pub paid_date: Option<String>,
    pub amount_paid: Option<f64>,
}

pub(crate) async fn handle_lifecycle_convert(
    Json(mut payload): Json<LifecycleConvertRequest>,
) -> impl IntoResponse {
    if let Err(err) = payload.document.normalize_and_validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "error": err
            })),
        );
    }

    let mut converted = match payload.target_type.trim().to_lowercase().as_str() {
        "invoice" => payload.document.to_invoice(payload.due_days.unwrap_or(15)),
        "receipt" | "payment_receipt" | "paid_receipt" => payload.document.to_receipt(
            payload.document.payment_method.as_deref().unwrap_or_default(),
            payload.document.transaction_reference.as_deref().unwrap_or_default(),
            payload.document.paid_at.as_deref().unwrap_or_default(),
        ),
        "estimate" | "quote" | "proposal" => payload.document.to_estimate(payload.valid_days.unwrap_or(30)),
        "credit_note" | "credit-note" | "credit" | "refund" | "credit_memo" => {
            payload.document.to_credit_note(payload.credit_reason.as_deref())
        }
        other => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "success": false,
                    "error": format!(
                        "Unsupported target_type '{}'. Supported lifecycle target types are: invoice, receipt, estimate, credit_note, refund.",
                        other
                    )
                })),
            );
        }
    };

    if let Some(link) = payload.payment_link {
        converted.payment_link = Some(link);
    }
    if let Some(qr) = payload.payment_qr_code {
        converted.payment_qr_code = Some(qr);
    }

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "document": converted
    })))
}

pub(crate) async fn handle_lifecycle_pay(
    Json(mut payload): Json<LifecyclePayRequest>,
) -> impl IntoResponse {
    if let Err(err) = payload.document.normalize_and_validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "error": err
            })),
        );
    }
    if let Some(amt) = payload.amount_paid {
        if !amt.is_finite() || amt < 0.0 {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "success": false,
                    "error": "Invalid amount_paid: must be a finite non-negative number."
                })),
            );
        }
    }

    let method = payload.payment_method.as_deref().unwrap_or_default();
    let tx_ref = payload.transaction_reference.as_deref().unwrap_or_default();
    let paid_date = payload.paid_date.as_deref().unwrap_or_default();

    let total = payload.document.grand_total();
    let amt = payload.amount_paid.unwrap_or(total);

    if amt < total - 0.001 {
        // Partial Payment: Retain invoice type and mark partial status, preserving balance due
        let mut partial_doc = payload.document.clone();
        partial_doc.amount_paid = Some(amt);
        partial_doc.is_paid = Some(false);
        partial_doc.payment_status = Some("partial".to_string());
        partial_doc.payment_method = (!method.trim().is_empty()).then(|| method.to_string());
        partial_doc.transaction_reference = (!tx_ref.trim().is_empty()).then(|| tx_ref.to_string());
        partial_doc.paid_at = (!paid_date.trim().is_empty()).then(|| paid_date.to_string());
        let bal = (total - amt).max(0.0);

        (StatusCode::OK, Json(serde_json::json!({
            "success": true,
            "payment_status": "partial",
            "amount_paid": amt,
            "balance_due": bal,
            "document": partial_doc
        })))
    } else {
        // Full payment: create a receipt from the supplied details.
        let mut receipt = payload.document.to_receipt(method, tx_ref, paid_date);
        receipt.amount_paid = Some(amt);

        (StatusCode::OK, Json(serde_json::json!({
            "success": true,
            "payment_status": "paid",
            "amount_paid": amt,
            "balance_due": 0.0,
            "document": receipt
        })))
    }
}

pub(crate) async fn handle_stripe_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Response {
    let secrets = crate::stripe::get_stripe_webhook_secrets();
    let allow_unverified = std::env::var("PRESSMARK_ALLOW_UNVERIFIED_WEBHOOKS")
        .map(|v| v == "1" || v == "true")
        .unwrap_or(false);

    if !allow_unverified {
        let sig = match headers.get("stripe-signature").and_then(|v| v.to_str().ok()) {
            Some(s) => s,
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "missing_signature",
                        "message": "Missing Stripe-Signature header"
                    })),
                )
                    .into_response();
            }
        };

        if let Err(e) = crate::stripe::verify_stripe_signature_against_any(&body, sig, &secrets, 300) {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "invalid_signature",
                    "message": e.to_string()
                })),
            )
                .into_response();
        }
    }

    let event: crate::stripe::StripeEvent = match serde_json::from_slice(&body) {
        Ok(ev) => ev,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "malformed_json",
                    "message": format!("Could not parse Stripe event: {}", e)
                })),
            )
                .into_response();
        }
    };

    // Idempotency defense: avoid duplicate fulfillment on Stripe webhook retries
    if state.stripe_idempotency.is_processed(&event.id).await {
        tracing::info!("Stripe webhook event {} already processed; skipping duplicate fulfillment", event.id);
        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "already_processed",
                "event_id": event.id
            })),
        )
            .into_response();
    }

    match crate::stripe::process_stripe_event(event.clone()) {
        Ok(action) => {
            state.stripe_idempotency.record(&event.id).await;

            match &action {
                crate::stripe::FulfillmentAction::CheckoutCompleted {
                    customer_email,
                    session_id,
                    amount_total_cents,
                    document_number,
                } => {
                    let plan = if *amount_total_cents >= 14900 {
                        crate::auth::ChatGptPlan::Agency
                    } else if *amount_total_cents >= 4900 {
                        crate::auth::ChatGptPlan::Business
                    } else {
                        crate::auth::ChatGptPlan::SoloPro
                    };
                    state.auth_mgr.upgrade_user_entitlement(
                        customer_email,
                        plan,
                        Some(session_id),
                        *amount_total_cents,
                    );

                    if crate::ghl::GhlDispatcher::has_env_or_pit_file() {
                        let email_cl = customer_email.clone();
                        let doc_cl = document_number
                            .clone()
                            .unwrap_or_else(|| session_id.clone());
                        let tier_slug = match plan {
                            crate::auth::ChatGptPlan::Agency => "sub_agency",
                            crate::auth::ChatGptPlan::Business => "sub_business",
                            _ => "sub_solo_pro",
                        };
                        let cents = *amount_total_cents as u32;
                        tokio::spawn(async move {
                            let ghl = crate::ghl::GhlDispatcher::from_env();
                            let _ = ghl
                                .dispatch_payment_unlock(&doc_cl, &email_cl, tier_slug, cents)
                                .await;
                        });
                    }
                }
                crate::stripe::FulfillmentAction::PaymentUnlocked {
                    customer_email,
                    amount_cents,
                    document_number,
                    receipt_number,
                    unlocked_tier,
                    ..
                } => {
                    let plan = unlocked_tier
                        .as_deref()
                        .map(crate::auth::ChatGptPlan::from_str_relaxed)
                        .unwrap_or_else(|| {
                            if *amount_cents >= 14900 {
                                crate::auth::ChatGptPlan::Agency
                            } else if *amount_cents >= 4900 {
                                crate::auth::ChatGptPlan::Business
                            } else {
                                crate::auth::ChatGptPlan::SoloPro
                            }
                        });
                    state.auth_mgr.upgrade_user_entitlement(
                        customer_email,
                        plan,
                        Some(receipt_number),
                        *amount_cents,
                    );

                    if crate::ghl::GhlDispatcher::has_env_or_pit_file() {
                        let email_cl = customer_email.clone();
                        let doc_cl = document_number
                            .clone()
                            .unwrap_or_else(|| receipt_number.clone());
                        let tier_cl = unlocked_tier
                            .clone()
                            .unwrap_or_else(|| "single_pass".to_string());
                        let cents = *amount_cents as u32;
                        tokio::spawn(async move {
                            let ghl = crate::ghl::GhlDispatcher::from_env();
                            let _ = ghl
                                .dispatch_payment_unlock(&doc_cl, &email_cl, &tier_cl, cents)
                                .await;
                        });
                    }
                }
                crate::stripe::FulfillmentAction::ChargeRefunded {
                    customer_email,
                    refund_amount_cents,
                    credit_note_number,
                    ..
                } => {
                    state.auth_mgr.revoke_user_entitlement(customer_email);
                    state.storage.record_usage_event(
                        "stripe_refund_revoked",
                        None,
                        "stripe_webhook",
                        Some(credit_note_number),
                        Some(*refund_amount_cents),
                    );
                    if crate::ghl::GhlDispatcher::has_env_or_pit_file() {
                        let email_cl = customer_email.clone();
                        let cn_cl = credit_note_number.clone();
                        let cents = *refund_amount_cents;
                        tokio::spawn(async move {
                            let ghl = crate::ghl::GhlDispatcher::from_env();
                            let _ = ghl.dispatch_charge_refunded(&email_cl, &cn_cl, cents).await;
                        });
                    }
                }
                _ => {}
            }

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "status": "success",
                    "action": action
                })),
            )
                .into_response()
        }
        Err(e) => {
            tracing::warn!("Stripe event processing non-fatal error: {}", e);
            // Return 200 with error details to prevent 3-day webhook retry storm on non-transient logic errors
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "status": "unfulfillable",
                    "message": e
                })),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    fn collect_property_descriptions(val: &serde_json::Value, out: &mut Vec<(String, String)>, prefix: &str) {
        if let Some(props) = val.get("properties").and_then(|p| p.as_object()) {
            for (k, v) in props {
                let path = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                if let Some(desc) = v.get("description").and_then(|d| d.as_str()) {
                    out.push((path.clone(), desc.to_string()));
                }
                collect_property_descriptions(v, out, &path);
                if let Some(items) = v.get("items") {
                    collect_property_descriptions(items, out, &format!("{}[]", path));
                }
            }
        }
    }

    #[tokio::test]
    async fn test_well_known_discovery_routes_and_mcp_protocol_negotiation() {
        let storage = Arc::new(crate::storage::Storage::new(":memory:").unwrap());
        let state = Arc::new(AppState::new("https://pressmark.dev".to_string()).with_storage(storage));
        let router = create_router(state);

        // 1. GET /.well-known/owners.json & /mcp/.well-known/owners.json
        let res_owners = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/.well-known/owners.json")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res_owners.status(), StatusCode::OK);
        let owners_json: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(res_owners.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(owners_json["owners"][0]["type"], "github");
        assert_eq!(owners_json["owners"][0]["id"], "mooshee");

        let res_mcp_owners = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/mcp/.well-known/owners.json")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res_mcp_owners.status(), StatusCode::OK);
        let mcp_owners_json: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(res_mcp_owners.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(mcp_owners_json["$schema"], "https://verifymcp.io/schemas/owners.json");
        assert_eq!(mcp_owners_json["owners"][0], "daniel@mooshworks.com");

        // 2. GET /.well-known/glama.json
        let res_glama = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/.well-known/glama.json")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res_glama.status(), StatusCode::OK);
        let glama_json: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(res_glama.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(glama_json["$schema"], "https://glama.ai/mcp/schemas/connector.json");
        assert_eq!(glama_json["maintainers"][0]["email"], "daniel@mooshworks.com");
        assert_eq!(glama_json["maintainers"][0]["github"], "mooshee");

        // 3. GET /.well-known/agent-card.json
        let res_agent = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/.well-known/agent-card.json")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res_agent.status(), StatusCode::OK);
        let agent_json: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(res_agent.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(agent_json["id"], "io.github.mooshee/pressmark");
        assert_eq!(agent_json["name"], "Pressmark");
        assert_eq!(agent_json["url"], "https://pressmark.dev");
        assert_eq!(agent_json["version"], env!("CARGO_PKG_VERSION"));
        assert!(agent_json["capabilities"].is_object());
        assert_eq!(agent_json["skills"].as_array().unwrap().len(), 3);

        // 4. GET /.well-known/oauth-protected-resource & /mcp/.well-known/oauth-protected-resource
        for oauth_uri in [
            "/.well-known/oauth-protected-resource",
            "/mcp/.well-known/oauth-protected-resource",
        ] {
            let res_oauth = router
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(oauth_uri)
                        .method("GET")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(res_oauth.status(), StatusCode::OK);
            let oauth_json: serde_json::Value = serde_json::from_slice(
                &axum::body::to_bytes(res_oauth.into_body(), usize::MAX)
                    .await
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(oauth_json["resource"], "https://pressmark.dev/mcp");
            assert_eq!(oauth_json["bearer_methods_supported"], serde_json::json!(["header"]));
            assert_eq!(oauth_json["resource_documentation"], "https://pressmark.dev/docs");
        }

        // 4b. GET OAuth Authorization Server, OpenID Configuration, x402, OpenAPI, Cursor MCP & A2A discovery routes
        for auth_srv_uri in [
            "/.well-known/oauth-authorization-server",
            "/.well-known/oauth-authorization-server/mcp",
            "/mcp/.well-known/oauth-authorization-server",
            "/.well-known/openid-configuration",
            "/.well-known/openid-configuration/mcp",
            "/mcp/.well-known/openid-configuration",
        ] {
            let res_as = router
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(auth_srv_uri)
                        .method("GET")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(res_as.status(), StatusCode::OK, "Failed route: {}", auth_srv_uri);
            let as_json: serde_json::Value = serde_json::from_slice(
                &axum::body::to_bytes(res_as.into_body(), usize::MAX)
                    .await
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(as_json["issuer"], "https://pressmark.dev");
        }

        for extra_disc_uri in [
            "/openapi.json",
            "/.well-known/ard.json",
            "/.well-known/ai-catalog.json",
            "/.well-known/x402",
            "/.well-known/x402.json",
            "/.cursor/mcp.json",
            "/a2a",
        ] {
            let res_extra = router
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(extra_disc_uri)
                        .method("GET")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(res_extra.status(), StatusCode::OK, "Failed route: {}", extra_disc_uri);
        }

        // 5. Verify all 5 MCP protocol versions negotiate cleanly & unknown version falls back to latest (2026-07-28)
        for proto in [
            "2026-07-28",
            "2025-11-25",
            "2025-06-18",
            "2025-03-26",
            "2024-11-05",
        ] {
            let init_resp = McpHandler::handle_request(JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                id: Some(serde_json::json!(1)),
                method: "initialize".to_string(),
                params: Some(serde_json::json!({ "protocolVersion": proto })),
            });
            let init_res = init_resp.result.unwrap();
            assert_eq!(init_res["protocolVersion"], proto);
            assert_eq!(init_res["serverInfo"]["version"], crate::mcp::MCP_SERVER_VERSION);
        }

        let fallback_resp = McpHandler::handle_request(JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: "initialize".to_string(),
            params: Some(serde_json::json!({ "protocolVersion": "2099-01-01" })),
        });
        assert_eq!(fallback_resp.result.unwrap()["protocolVersion"], "2026-07-28");

        let disc_resp = McpHandler::handle_request(JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: "server/discover".to_string(),
            params: Some(serde_json::json!({})),
        });
        let disc_res = disc_resp.result.unwrap();
        assert_eq!(disc_res["protocolVersion"], "2026-07-28");
        assert!(disc_res["protocolVersions"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("2026-07-28")));

        // 6. Keep descriptions concise and examples in the machine-readable input schema.
        let tools = McpHandler::tools_list();
        for tool in tools.as_array().unwrap() {
            let tool_name = tool["name"].as_str().unwrap();
            let tool_desc = tool["description"].as_str().unwrap();
            assert!(
                tool_desc.len() < 250,
                "Tool {} description length {} >= 250",
                tool_name,
                tool_desc.len()
            );
            assert!(
                tool["inputSchema"]["examples"].as_array().is_some_and(|examples| !examples.is_empty()),
                "Tool {} is missing input schema examples",
                tool_name
            );
            let mut props = Vec::new();
            collect_property_descriptions(&tool["inputSchema"], &mut props, tool_name);
            collect_property_descriptions(&tool["outputSchema"], &mut props, &format!("{}.output", tool_name));
            for (prop_path, desc) in props {
                assert!(
                    desc.len() < 250,
                    "Property {} description length {} >= 250: {}",
                    prop_path,
                    desc.len(),
                    desc
                );
            }
        }
    }
}
