//! Pressmark DocumentRequest Theme, Formatting, Statutory & Validation Helpers
//!
//! Author: Daniel Hallman

use super::*;

impl DocumentRequest {
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
            Some("serif") => "(\"Georgia\", \"Libertinus Serif\", \"DejaVu Serif\", \"Times New Roman\")",
            Some("mono") => "(\"Menlo\", \"DejaVu Sans Mono\", \"Liberation Mono\", \"Courier New\")",
            Some("display") => "(\"Playfair Display\", \"Georgia\", \"Libertinus Serif\", \"DejaVu Serif\", \"Times New Roman\")",
            Some("grotesk") => "(\"Space Grotesk\", \"Helvetica Neue\", \"Arial\", \"Liberation Sans\", \"DejaVu Sans\")",
            Some("editorial") => "(\"Palatino\", \"Garamond\", \"Georgia\", \"Libertinus Serif\", \"DejaVu Serif\")",
            _ => "(\"Helvetica Neue\", \"Arial\", \"Liberation Sans\", \"DejaVu Sans\")",
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
