use reta_architecture::{
    language_coverage_for_cli_args, language_parity_for_cli_args, language_sync_for_cli_args,
    materialize_cli_args, TableMaterializationConfig, TableViewLanguageCoveragePolicy,
    TableViewLanguageParityPolicy, TableViewLanguageSyncPolicy,
};

fn continuum_m_english_args() -> Vec<&'static str> {
    vec![
        "reta",
        "-language=english",
        "-zeilen",
        "--vorhervonausschnitt=1-1",
        "-spalten",
        "--kontinuum=m",
        "--breite=0",
    ]
}

#[test]
fn synced_language_assets_materialize_493_and_744_directly() {
    let args = continuum_m_english_args();
    let config = TableMaterializationConfig::from_cli_args(&args);
    let report = materialize_cli_args(&args, &config);

    let section = report.ordinary_sections.first().expect("expected an ordinary religion section");
    assert_eq!(section.language, "en", "expected English materialization: {report:?}");
    assert_eq!(section.asset_name, "en-religion.csv", "English asset should no longer need base fallback after Stage 62: {report:?}");
    assert!(report.materialized_column_order_legacy.contains(&493), "missing direct 493: {report:?}");
    assert!(report.materialized_column_order_legacy.contains(&744), "missing direct 744: {report:?}");
    assert!(
        report.virtual_columns.iter().all(|column| column.column_legacy != 744),
        "744 must not be virtual after synced language CSVs: {report:?}"
    );
}

#[test]
fn language_coverage_reports_no_stale_religion_variants_for_continuum_m() {
    let args = continuum_m_english_args();
    let report = language_coverage_for_cli_args(&args, &TableViewLanguageCoveragePolicy::default());

    assert!(report.ready(), "language coverage should be ready: {report:?}");
    assert_eq!(report.stale_language_count, 0, "no synced language should be stale: {report:?}");
    assert!(report.languages_missing_744.is_empty(), "no language should miss 744: {report:?}");
    assert!(report.all_language_assets_support_required_columns, "all language assets should support 493/744: {report:?}");
}

#[test]
fn language_sync_has_no_pending_744_actions_after_csv_sync() {
    let args = continuum_m_english_args();
    let report = language_sync_for_cli_args(&args, &TableViewLanguageSyncPolicy::strict());

    assert!(report.ready(), "language sync should be ready: {report:?}");
    assert_eq!(report.pending_action_count, 0, "no pending sync action expected: {report:?}");
    assert!(report.pending_languages.is_empty(), "no pending language expected: {report:?}");
    assert!(report.pending_columns.is_empty(), "no pending columns expected: {report:?}");
}

#[test]
fn language_parity_keeps_744_direct_even_with_english_language_flag() {
    let args = continuum_m_english_args();
    let report = language_parity_for_cli_args(&args, &TableViewLanguageParityPolicy::default());

    assert!(report.ready(), "language parity should be ready: {report:?}");
    assert_eq!(report.requested_language, "en", "expected requested English language: {report:?}");
    assert_eq!(report.effective_asset_name, "en-religion.csv", "expected direct English asset: {report:?}");
    assert!(report.direct_493_materialized, "493 must be direct: {report:?}");
    assert!(report.direct_744_materialized, "744 must be direct: {report:?}");
    assert!(report.missing_columns_legacy.is_empty(), "no missing selected columns expected: {report:?}");
}
