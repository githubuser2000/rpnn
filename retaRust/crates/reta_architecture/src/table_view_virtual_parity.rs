//! Parity diagnostics for policy-rendered virtual/non-direct table columns.
//!
//! Stage 37 made virtual columns such as the continuum `744` witness explicitly
//! renderable by policy.  Stage 38 adds the missing safety check: changing the
//! virtual-column policy must not mutate the already materialized direct CSV
//! cells.  This module compares a reference policy, normally `Suppress`, with a
//! rendered policy such as `TagSummary`, `Placeholder` or `Witness` and records
//! whether only virtual cells were added.

use serde::{Deserialize, Serialize};

use crate::output_syntax::OutputMode;
use crate::parameter_runtime::bootstrap_parameter_runtime;
use crate::table_generation::TableGenerationPlan;
use crate::table_materialization::{TableMaterializationConfig, bootstrap_table_materialization};
use crate::table_view::{
    MaterializedTableCellSource, MaterializedTableView, MaterializedTableViewCell,
    MaterializedTableViewConfig, VirtualColumnDisplayPolicy, bootstrap_table_view,
};
use crate::table_view_output::{
    TableViewOutputConfig, parse_table_view_output_cli_options, render_materialized_table_view,
};
use crate::table_view_virtual_columns::parse_table_view_virtual_column_cli_options;
use crate::table_view_output_parity::{
    TableViewOutputParityConfig, TableViewOutputParityReport, compare_output_lines,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DirectCellSignature {
    pub row_zero_based: usize,
    pub column_legacy: usize,
    pub asset_name: String,
    pub source_column_index: usize,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VirtualCellSignature {
    pub row_zero_based: usize,
    pub column_legacy: usize,
    pub asset_name: String,
    pub reason: String,
    pub tag_names: Vec<String>,
    pub html_class_text: Option<String>,
    pub predecessor_source_column_index: Option<usize>,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewVirtualParityConfig {
    pub reference_policy: VirtualColumnDisplayPolicy,
    pub rendered_policy: VirtualColumnDisplayPolicy,
    pub reference_suppress_question_marks: bool,
    pub rendered_suppress_question_marks: bool,
    pub mode_override: Option<OutputMode>,
    pub compare_rendered_lines: bool,
    pub compare_semantic_rows: bool,
}

impl Default for TableViewVirtualParityConfig {
    fn default() -> Self {
        Self {
            reference_policy: VirtualColumnDisplayPolicy::Suppress,
            rendered_policy: VirtualColumnDisplayPolicy::TagSummary,
            reference_suppress_question_marks: true,
            rendered_suppress_question_marks: true,
            mode_override: None,
            compare_rendered_lines: true,
            compare_semantic_rows: true,
        }
    }
}

impl TableViewVirtualParityConfig {
    pub fn with_rendered_policy(mut self, policy: VirtualColumnDisplayPolicy) -> Self {
        self.rendered_policy = policy;
        if matches!(policy, VirtualColumnDisplayPolicy::Placeholder | VirtualColumnDisplayPolicy::Witness) {
            self.rendered_suppress_question_marks = false;
        }
        self
    }

    pub fn with_mode(mut self, mode: OutputMode) -> Self {
        self.mode_override = Some(mode);
        self
    }

    pub fn with_cli_virtual_options<S: AsRef<str>>(mut self, args: &[S]) -> Self {
        let options = parse_table_view_virtual_column_cli_options(args);
        if let Some(policy) = options.policy {
            self = self.with_rendered_policy(policy);
        }
        if let Some(suppress) = options.suppress_question_mark_virtuals {
            self.rendered_suppress_question_marks = suppress;
        }
        self
    }

    pub fn from_cli_args<S: AsRef<str>>(args: &[S], mode: OutputMode) -> Self {
        Self::default()
            .with_mode(mode)
            .with_cli_virtual_options(args)
    }

    pub fn reference_view_config(&self) -> MaterializedTableViewConfig {
        MaterializedTableViewConfig {
            virtual_column_policy: self.reference_policy,
            suppress_question_mark_virtuals: self.reference_suppress_question_marks,
            ..MaterializedTableViewConfig::default()
        }
    }

    pub fn rendered_view_config(&self) -> MaterializedTableViewConfig {
        MaterializedTableViewConfig {
            virtual_column_policy: self.rendered_policy,
            suppress_question_mark_virtuals: self.rendered_suppress_question_marks,
            ..MaterializedTableViewConfig::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewVirtualParityReport {
    pub class: String,
    pub mode: String,
    pub reference_policy: String,
    pub rendered_policy: String,
    pub rendered_policy_source: String,
    pub cli_virtual_option_count: usize,
    pub rendered_policy_matches_cli: bool,
    pub direct_cells_equal: bool,
    pub raw_lines_equal: bool,
    pub semantic_rows_equal: bool,
    pub first_direct_mismatch_index: Option<usize>,
    pub first_raw_line_mismatch_index: Option<usize>,
    pub direct_cell_count: usize,
    pub reference_virtual_cell_count: usize,
    pub rendered_virtual_cell_count: usize,
    pub added_virtual_cell_count: usize,
    pub added_virtual_column_count: usize,
    pub added_virtual_columns: Vec<usize>,
    pub continuum_m_virtual_744_added_only: bool,
    pub continuum_m_direct_493_preserved: bool,
    pub reference_line_count: usize,
    pub rendered_line_count: usize,
    pub reference_preview: Vec<String>,
    pub rendered_preview: Vec<String>,
    pub direct_reference_preview: Vec<DirectCellSignature>,
    pub added_virtual_preview: Vec<VirtualCellSignature>,
    pub semantic_diff: Option<TableViewOutputParityReport>,
    pub warnings: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewVirtualParitySnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub default_reference_policy: String,
    pub default_rendered_policy: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewVirtualParityBundle;

impl TableViewVirtualParityBundle {
    pub fn snapshot(&self) -> TableViewVirtualParitySnapshot {
        TableViewVirtualParitySnapshot {
            class: "TableViewVirtualParitySnapshot".to_string(),
            morphisms: vec![
                "compare_virtual_column_policies_for_cli_args".to_string(),
                "direct_cell_signatures".to_string(),
                "virtual_cell_signatures".to_string(),
                "first_signature_mismatch".to_string(),
                "continuum_m_virtual_parity_smoke".to_string(),
            ],
            default_reference_policy: VirtualColumnDisplayPolicy::Suppress.canonical().to_string(),
            default_rendered_policy: VirtualColumnDisplayPolicy::TagSummary.canonical().to_string(),
            universal_property:
                "rendering virtual columns may add witness cells but must preserve every direct CSV cell"
                    .to_string(),
        }
    }

    pub fn compare_cli_args<S: AsRef<str>>(
        &self,
        args: &[S],
        config: &TableViewVirtualParityConfig,
    ) -> TableViewVirtualParityReport {
        compare_virtual_column_policies_for_cli_args(args, config)
    }
}

pub fn bootstrap_table_view_virtual_parity() -> TableViewVirtualParityBundle {
    TableViewVirtualParityBundle
}

pub fn direct_cell_signatures(view: &MaterializedTableView) -> Vec<DirectCellSignature> {
    view.rows
        .iter()
        .flat_map(|row| {
            row.cells.iter().filter_map(move |cell| match &cell.source {
                MaterializedTableCellSource::Csv {
                    asset_name,
                    source_column_index,
                } => Some(DirectCellSignature {
                    row_zero_based: row.source_row_zero_based,
                    column_legacy: cell.column_legacy,
                    asset_name: asset_name.clone(),
                    source_column_index: *source_column_index,
                    value: cell.value.clone(),
                }),
                MaterializedTableCellSource::Virtual { .. } => None,
            })
        })
        .collect()
}

pub fn virtual_cell_signatures(view: &MaterializedTableView) -> Vec<VirtualCellSignature> {
    view.rows
        .iter()
        .flat_map(|row| {
            row.cells.iter().filter_map(move |cell| virtual_cell_signature(row.source_row_zero_based, cell))
        })
        .collect()
}

pub fn virtual_cell_signature(
    row_zero_based: usize,
    cell: &MaterializedTableViewCell,
) -> Option<VirtualCellSignature> {
    match &cell.source {
        MaterializedTableCellSource::Virtual {
            asset_name,
            reason,
            tag_names,
            html_class_text,
            predecessor_source_column_index,
        } => Some(VirtualCellSignature {
            row_zero_based,
            column_legacy: cell.column_legacy,
            asset_name: asset_name.clone(),
            reason: reason.clone(),
            tag_names: tag_names.clone(),
            html_class_text: html_class_text.clone(),
            predecessor_source_column_index: *predecessor_source_column_index,
            value: cell.value.clone(),
        }),
        MaterializedTableCellSource::Csv { .. } => None,
    }
}

pub fn first_signature_mismatch<T: Eq>(left: &[T], right: &[T]) -> Option<usize> {
    let common = left.len().min(right.len());
    for index in 0..common {
        if left[index] != right[index] {
            return Some(index);
        }
    }
    (left.len() != right.len()).then_some(common)
}

pub fn first_line_mismatch(left: &[String], right: &[String]) -> Option<usize> {
    let common = left.len().min(right.len());
    for index in 0..common {
        if left[index] != right[index] {
            return Some(index);
        }
    }
    (left.len() != right.len()).then_some(common)
}

pub fn compare_virtual_column_policies_for_cli_args<S: AsRef<str>>(
    args: &[S],
    config: &TableViewVirtualParityConfig,
) -> TableViewVirtualParityReport {
    let args_owned = args.iter().map(|arg| arg.as_ref().to_string()).collect::<Vec<_>>();
    let cli_virtual_options = parse_table_view_virtual_column_cli_options(&args_owned);
    let rendered_policy_matches_cli = cli_virtual_options
        .policy
        .map(|policy| policy == config.rendered_policy)
        .unwrap_or(false);
    let rendered_policy_source = if cli_virtual_options.policy.is_some()
        || cli_virtual_options.suppress_question_mark_virtuals.is_some()
    {
        "cli"
    } else {
        "config"
    }
    .to_string();
    let parsed = bootstrap_parameter_runtime().parse_cli_args(&args_owned);
    let mode = config
        .mode_override
        .or(parsed.selected_output_mode)
        .unwrap_or(OutputMode::Shell);
    let plan = TableGenerationPlan::from_parameter_command_sets(&parsed.command_sets);
    let materialization = bootstrap_table_materialization()
        .materialize_plan(&plan, &TableMaterializationConfig::default());

    let reference_view = bootstrap_table_view().view_from_report(
        &materialization,
        &config.reference_view_config(),
    );
    let rendered_view = bootstrap_table_view().view_from_report(
        &materialization,
        &config.rendered_view_config(),
    );

    let cli_options = parse_table_view_output_cli_options(&args_owned);
    let mut reference_output_config = TableViewOutputConfig::default()
        .with_mode(mode)
        .with_cli_options(cli_options.clone());
    reference_output_config.virtual_column_policy = config.reference_policy;
    reference_output_config.suppress_question_mark_virtuals = config.reference_suppress_question_marks;

    let mut rendered_output_config = TableViewOutputConfig::default()
        .with_mode(mode)
        .with_cli_options(cli_options);
    rendered_output_config.virtual_column_policy = config.rendered_policy;
    rendered_output_config.suppress_question_mark_virtuals = config.rendered_suppress_question_marks;

    let reference_output = render_materialized_table_view(&reference_view, &reference_output_config);
    let rendered_output = render_materialized_table_view(&rendered_view, &rendered_output_config);

    let reference_direct = direct_cell_signatures(&reference_view);
    let rendered_direct = direct_cell_signatures(&rendered_view);
    let added_virtual = virtual_cell_signatures(&rendered_view);
    let added_virtual_columns = added_virtual
        .iter()
        .map(|sig| sig.column_legacy)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let direct_cells_equal = reference_direct == rendered_direct;
    let raw_lines_equal = reference_output.rendered_lines == rendered_output.rendered_lines;
    let semantic_diff = if config.compare_semantic_rows {
        Some(compare_output_lines(
            &reference_output.rendered_lines,
            &rendered_output.rendered_lines,
            &TableViewOutputParityConfig::default().with_mode(mode),
        ))
    } else {
        None
    };
    let semantic_rows_equal = semantic_diff
        .as_ref()
        .map(|diff| diff.semantic_equal)
        .unwrap_or(raw_lines_equal);
    let first_direct_mismatch_index = first_signature_mismatch(&reference_direct, &rendered_direct);
    let first_raw_line_mismatch_index = first_line_mismatch(
        &reference_output.rendered_lines,
        &rendered_output.rendered_lines,
    );
    let direct_cell_count = reference_direct.len();
    let reference_virtual_cell_count = virtual_cell_signatures(&reference_view).len();
    let rendered_virtual_cell_count = added_virtual.len();
    let direct_reference_preview = reference_direct.iter().take(8).cloned().collect::<Vec<_>>();
    let added_virtual_preview = added_virtual.iter().take(8).cloned().collect::<Vec<_>>();
    let mut warnings = Vec::new();
    if !direct_cells_equal {
        warnings.push("direct_csv_cells_changed_under_virtual_column_policy".to_string());
    }
    if !raw_lines_equal && direct_cells_equal {
        warnings.push("rendered_lines_changed_only_after_virtual_policy_projection".to_string());
    }
    let continuum_m_direct_493_preserved = reference_direct.iter().any(|sig| {
        sig.column_legacy == 493 && sig.value.contains("M Kontinuum")
    }) && rendered_direct.iter().any(|sig| {
        sig.column_legacy == 493 && sig.value.contains("M Kontinuum")
    });
    let continuum_m_virtual_744_added_only = direct_cells_equal
        && added_virtual.iter().any(|sig| {
            sig.column_legacy == 744
                && sig.tag_names.iter().any(|tag| tag == "sternPolygon")
                && sig.tag_names.iter().any(|tag| tag == "keinParaOdMetaP")
        });

    TableViewVirtualParityReport {
        class: "TableViewVirtualParityReport".to_string(),
        mode: mode.canonical_name().to_string(),
        reference_policy: config.reference_policy.canonical().to_string(),
        rendered_policy: config.rendered_policy.canonical().to_string(),
        rendered_policy_source,
        cli_virtual_option_count: cli_virtual_options.recognized_option_count,
        rendered_policy_matches_cli,
        direct_cells_equal,
        raw_lines_equal,
        semantic_rows_equal,
        first_direct_mismatch_index,
        first_raw_line_mismatch_index,
        direct_cell_count,
        reference_virtual_cell_count,
        rendered_virtual_cell_count,
        added_virtual_cell_count: rendered_virtual_cell_count,
        added_virtual_column_count: added_virtual_columns.len(),
        added_virtual_columns,
        continuum_m_virtual_744_added_only,
        continuum_m_direct_493_preserved,
        reference_line_count: reference_output.rendered_line_count,
        rendered_line_count: rendered_output.rendered_line_count,
        reference_preview: reference_output.rendered_lines.iter().take(8).cloned().collect(),
        rendered_preview: rendered_output.rendered_lines.iter().take(8).cloned().collect(),
        direct_reference_preview,
        added_virtual_preview,
        semantic_diff,
        warnings,
        universal_property:
            "virtual policy morphisms may add virtual witnesses but must be identity on direct CSV cells"
                .to_string(),
    }
}

pub fn continuum_m_virtual_parity_smoke() -> TableViewVirtualParityReport {
    let args = vec![
        "reta".to_string(),
        "-zeilen".to_string(),
        "--vorhervonausschnitt=1-1".to_string(),
        "-spalten".to_string(),
        "--kontinuum=m".to_string(),
        "-ausgabe".to_string(),
        "--spaltenreihenfolgeundnurdiese=744,493".to_string(),
        "--breite=0".to_string(),
    ];
    compare_virtual_column_policies_for_cli_args(
        &args,
        &TableViewVirtualParityConfig::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continuum_m_virtual_policy_adds_744_without_touching_493() {
        let report = continuum_m_virtual_parity_smoke();
        assert!(report.direct_cells_equal);
        assert!(report.continuum_m_direct_493_preserved);
        assert!(report.continuum_m_virtual_744_added_only);
        assert!(report.added_virtual_columns.contains(&744));
        assert!(!report.raw_lines_equal);
    }

    #[test]
    fn suppress_vs_suppress_is_raw_equal() {
        let args = [
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--kontinuum=m",
        ];
        let config = TableViewVirtualParityConfig {
            reference_policy: VirtualColumnDisplayPolicy::Suppress,
            rendered_policy: VirtualColumnDisplayPolicy::Suppress,
            ..TableViewVirtualParityConfig::default()
        };
        let report = compare_virtual_column_policies_for_cli_args(&args, &config);
        assert!(report.direct_cells_equal);
        assert!(report.raw_lines_equal);
        assert_eq!(report.added_virtual_cell_count, 0);
    }

    #[test]
    fn placeholder_policy_reports_virtual_question_marks_without_direct_drift() {
        let report = compare_virtual_column_policies_for_cli_args(
            &[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=1-1",
                "-spalten",
                "--kontinuum=m",
                "-ausgabe",
                "--spaltenreihenfolgeundnurdiese=744,493",
            ],
            &TableViewVirtualParityConfig::default()
                .with_rendered_policy(VirtualColumnDisplayPolicy::Placeholder),
        );
        assert!(report.direct_cells_equal);
        assert!(report.rendered_preview.iter().any(|line| line.contains('?')));
    }

    #[test]
    fn cli_virtual_policy_is_used_when_requested() {
        let args = [
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--kontinuum=m",
            "-ausgabe",
            "--spaltenreihenfolgeundnurdiese=744,493",
            "--virtualwitness",
        ];
        let config = TableViewVirtualParityConfig::default().with_cli_virtual_options(&args);
        let report = compare_virtual_column_policies_for_cli_args(&args, &config);
        assert_eq!(report.rendered_policy, "witness");
        assert_eq!(report.rendered_policy_source, "cli");
        assert!(report.rendered_policy_matches_cli);
        assert!(report.direct_cells_equal);
    }
}
