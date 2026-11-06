//! End-to-End Typst Compilation & Screenshot Generation for 50 Concepts + 4 Element Showcase Sheets
//!
//! Author: Daniel Hallman

use std::path::PathBuf;

use hallman_doc_engine::catalog::find_concept_by_slug;
use hallman_doc_engine::catalog::render_concept_typst_markup;
use hallman_doc_engine::catalog::render_element_showcase_sheet_typst;
use hallman_doc_engine::catalog::ARCHETYPE_CONCEPTS_50;
use hallman_doc_engine::compile_typst_to_pdf_bytes;
use hallman_doc_engine::compile_typst_to_png_bytes;
use hallman_doc_engine::find_typst_executable;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
}

fn resolve_seal_prefix(root: &std::path::Path) -> &'static str {
    if root.join("templates/assets/seals").exists() {
        "/templates/assets/seals"
    } else if root.join("web/assets/seals").exists() {
        "/web/assets/seals"
    } else if root.join("static/assets/seals").exists() {
        "/static/assets/seals"
    } else {
        "/assets/seals"
    }
}

#[test]
fn test_compile_all_4_element_showcase_sheets_and_15_flagship_concept_pngs() {
    if find_typst_executable().is_none() {
        return;
    }

    let root = workspace_root();
    let seal_prefix = resolve_seal_prefix(&root);
    let out_dir = std::env::var("HDE_SCREENSHOT_OUT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| root.join("target/hde_screenshots"));
    std::fs::create_dir_all(&out_dir).expect("create screenshot output dir");

    let sheets = [
        ("page_layouts", "showcase_sheet_page_layouts.png"),
        ("mastheads", "showcase_sheet_mastheads.png"),
        ("tables_and_kpis", "showcase_sheet_tables_and_kpis.png"),
        (
            "diagrams_and_stamps",
            "showcase_sheet_diagrams_and_stamps.png",
        ),
        (
            "signatures_and_seals",
            "showcase_sheet_signatures_and_seals.png",
        ),
    ];

    for (kind, filename) in sheets {
        let markup = render_element_showcase_sheet_typst(kind, seal_prefix);
        let pdf = compile_typst_to_pdf_bytes(&markup, Some(&root))
            .unwrap_or_else(|e| panic!("Failed to compile showcase sheet PDF {}: {}", kind, e));
        let parsed = lopdf::Document::load_mem(&pdf).expect("valid lopdf showcase document");
        assert_eq!(
            parsed.get_pages().len(),
            1,
            "Showcase sheet {} must fit on 1 page, got {} pages",
            kind,
            parsed.get_pages().len()
        );
        let png = compile_typst_to_png_bytes(&markup, Some(&root), 144)
            .unwrap_or_else(|e| panic!("Failed to compile showcase sheet {}: {}", kind, e));
        assert!(
            png.starts_with(b"\x89PNG\r\n\x1a\n"),
            "Invalid PNG header for {}",
            filename
        );
        assert!(png.len() > 25_000, "PNG unexpectedly small for {}", filename);
        std::fs::write(out_dir.join(filename), &png).expect("write showcase PNG");
    }

    let flagship_concepts = [
        ("1", "concept_01_aia_progress_draw.png"),
        ("2", "concept_02_construction_change_order.png"),
        ("3", "concept_03_mechanics_lien_waiver.png"),
        ("4", "concept_04_hvac_work_order.png"),
        ("5", "concept_05_roofing_storm_supplement.png"),
        ("6", "concept_06_residential_lease.png"),
        ("10", "concept_10_alta_closing_statement.png"),
        ("11", "concept_11_boe_appeal_petition.png"),
        ("12", "concept_12_veteran_shield_packet.png"),
        ("16", "concept_16_enterprise_saas_invoice.png"),
        ("21", "concept_21_creative_brand_proposal.png"),
        ("26", "concept_26_law_firm_iolta_retainer.png"),
        ("28", "concept_28_business_acquisition_loi.png"),
        ("31", "concept_31_medical_superbill.png"),
        ("36", "concept_36_freight_bill_of_lading.png"),
        ("42", "concept_42_solar_pv_proposal.png"),
        ("49", "concept_49_govcon_capability_statement.png"),
        ("51", "concept_51_hotel_group_room_block.png"),
        ("52", "concept_52_wedding_venue_packages.png"),
        ("54", "concept_54_catering_bar_packages.png"),
        ("57", "concept_57_geotechnical_soil_boring.png"),
        ("61", "concept_61_aircraft_maintenance_337.png"),
        ("66", "concept_66_film_production_call_sheet.png"),
        ("72", "concept_72_timber_cruise_stumpage.png"),
        ("78", "concept_78_analytical_lab_coa.png"),
        ("81", "concept_81_federal_grant_sf424.png"),
        ("88", "concept_88_dealership_buyers_order.png"),
        ("91", "concept_91_yc_post_money_safe.png"),
        ("97", "concept_97_statutory_durable_poa.png"),
    ];

    for (id_str, filename) in flagship_concepts {
        let spec = find_concept_by_slug(id_str).expect("find flagship concept");
        let markup = render_concept_typst_markup(spec, None, seal_prefix);
        let png = compile_typst_to_png_bytes(&markup, Some(&root), 144)
            .unwrap_or_else(|e| panic!("Failed to compile concept {} PNG: {}", spec.slug, e));
        assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));
        std::fs::write(out_dir.join(filename), &png).expect("write concept PNG");

        let pdf = compile_typst_to_pdf_bytes(&markup, Some(&root))
            .unwrap_or_else(|e| panic!("Failed to compile concept {} PDF: {}", spec.slug, e));
        assert!(pdf.starts_with(b"%PDF-1."));
        if spec.default_execution_mode != "esign" {
            let pdf_lossy = String::from_utf8_lossy(&pdf);
            assert!(
                pdf_lossy.contains("/AcroForm") && pdf_lossy.contains("taxpayer_signature"),
                "Concept {} should contain interactive /AcroForm signature widget",
                spec.slug
            );
        }
    }
}

#[test]
fn test_all_50_concepts_compile_to_valid_pdf() {
    if find_typst_executable().is_none() {
        return;
    }

    let root = workspace_root();
    let seal_prefix = resolve_seal_prefix(&root);
    let mut overflows = Vec::new();
    for spec in ARCHETYPE_CONCEPTS_50 {
        let markup = render_concept_typst_markup(spec, None, seal_prefix);
        let pdf = compile_typst_to_pdf_bytes(&markup, Some(&root)).unwrap_or_else(|e| {
            panic!(
                "Concept #{} ({}) failed Typst PDF compilation: {}",
                spec.id, spec.slug, e
            )
        });
        assert!(
            pdf.starts_with(b"%PDF-1."),
            "Concept #{} did not produce valid PDF bytes",
            spec.id
        );
        let parsed = lopdf::Document::load_mem(&pdf).expect("valid lopdf document");
        let pages = parsed.get_pages().len();
        if pages != 1 {
            overflows.push(format!(
                "#{} ({}, layout={}, notary={}) -> {} pages",
                spec.id, spec.slug, spec.default_page_layout, spec.notary_required, pages
            ));
        }
    }
    assert!(
        overflows.is_empty(),
        "All 100 concepts must fit cleanly on 1 page, but found overflows:\n{}",
        overflows.join("\n")
    );

    // Also stress-test all 10 Structural Page Archetypes under worst-case overrides:
    // preset=high_legibility + execution=notary_jurat
    let stress_ids = ["51", "11", "52", "4", "57", "6", "31", "12", "91", "1"];
    let mut stress_overflows = Vec::new();
    for id_str in stress_ids {
        let spec = find_concept_by_slug(id_str).expect("find stress concept");
        let opts = hallman_doc_engine::catalog::CustomRenderOptions {
            preset: Some("high_legibility".to_string()),
            execution: Some("notary_jurat".to_string()),
            ..Default::default()
        };
        let markup = render_concept_typst_markup(spec, Some(&opts), seal_prefix);
        let pdf = compile_typst_to_pdf_bytes(&markup, Some(&root)).expect("compile stress PDF");
        let parsed = lopdf::Document::load_mem(&pdf).expect("valid stress lopdf");
        let pages = parsed.get_pages().len();
        if pages != 1 {
            stress_overflows.push(format!(
                "#{} ({}, layout={}) under high_legibility+notary_jurat -> {} pages",
                spec.id, spec.slug, spec.default_page_layout, pages
            ));
        }
    }
    assert!(
        stress_overflows.is_empty(),
        "All 10 layouts must fit on 1 page under high_legibility+notary_jurat, but found overflows:\n{}",
        stress_overflows.join("\n")
    );
}
