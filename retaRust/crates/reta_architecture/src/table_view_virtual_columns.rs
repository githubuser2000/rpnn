//! Policy-controlled rendering of virtual/non-direct table-view columns.
//!
//! Stage 37 makes the existing `VirtualColumnDisplayPolicy` visible as an
//! explicit architecture surface. Stage 55 updates the base `religion.csv` so
//! the former `744` witness is now a direct CSV-backed column. This module
//! still owns virtual/non-direct column policy for other out-of-range or
//! generated columns; direct columns are intentionally unaffected by it.

use serde::{Deserialize, Serialize};

use crate::table_materialization::{TableMaterializationConfig, bootstrap_table_materialization};
use crate::table_view::{
    MaterializedTableView, MaterializedTableViewConfig, VirtualColumnDisplayPolicy,
    bootstrap_table_view,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewVirtualColumnCliOptions {
    pub policy: Option<VirtualColumnDisplayPolicy>,
    pub suppress_question_mark_virtuals: Option<bool>,
    pub recognized_option_count: usize,
    pub requested_flags: Vec<String>,
}

impl Default for TableViewVirtualColumnCliOptions {
    fn default() -> Self {
        Self {
            policy: None,
            suppress_question_mark_virtuals: None,
            recognized_option_count: 0,
            requested_flags: Vec::new(),
        }
    }
}

impl TableViewVirtualColumnCliOptions {
    pub fn from_args<S: AsRef<str>>(args: &[S]) -> Self {
        parse_table_view_virtual_column_cli_options(args)
    }

    pub fn has_policy(&self) -> bool {
        self.policy.is_some() || self.suppress_question_mark_virtuals.is_some()
    }

    pub fn resolved_policy(&self, fallback: VirtualColumnDisplayPolicy) -> VirtualColumnDisplayPolicy {
        self.policy.unwrap_or(fallback)
    }

    pub fn resolved_suppress_question_marks(&self, fallback: bool) -> bool {
        self.suppress_question_mark_virtuals.unwrap_or(fallback)
    }

    pub fn apply_to_view_config(
        &self,
        base: &MaterializedTableViewConfig,
    ) -> MaterializedTableViewConfig {
        let mut config = base.clone();
        if let Some(policy) = self.policy {
            config.virtual_column_policy = policy;
        }
        if let Some(suppress) = self.suppress_question_mark_virtuals {
            config.suppress_question_mark_virtuals = suppress;
        }
        config
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewVirtualColumnConfig {
    pub policy: VirtualColumnDisplayPolicy,
    pub suppress_question_mark_virtuals: bool,
}

impl Default for TableViewVirtualColumnConfig {
    fn default() -> Self {
        Self {
            policy: VirtualColumnDisplayPolicy::Suppress,
            suppress_question_mark_virtuals: true,
        }
    }
}

impl TableViewVirtualColumnConfig {
    pub fn suppress() -> Self {
        Self::default()
    }

    pub fn tag_summary() -> Self {
        Self {
            policy: VirtualColumnDisplayPolicy::TagSummary,
            suppress_question_mark_virtuals: true,
        }
    }

    pub fn placeholder() -> Self {
        Self {
            policy: VirtualColumnDisplayPolicy::Placeholder,
            suppress_question_mark_virtuals: false,
        }
    }

    pub fn witness() -> Self {
        Self {
            policy: VirtualColumnDisplayPolicy::Witness,
            suppress_question_mark_virtuals: false,
        }
    }

    pub fn from_cli_options(options: &TableViewVirtualColumnCliOptions) -> Self {
        let base = Self::default();
        Self {
            policy: options.resolved_policy(base.policy),
            suppress_question_mark_virtuals: options
                .resolved_suppress_question_marks(base.suppress_question_mark_virtuals),
        }
    }

    pub fn to_view_config(&self) -> MaterializedTableViewConfig {
        MaterializedTableViewConfig {
            virtual_column_policy: self.policy,
            suppress_question_mark_virtuals: self.suppress_question_mark_virtuals,
            ..MaterializedTableViewConfig::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewVirtualColumnReport {
    pub class: String,
    pub policy: String,
    pub suppress_question_mark_virtuals: bool,
    pub selected_column_count: usize,
    pub virtual_column_count: usize,
    pub rendered_virtual_cell_count: usize,
    pub continuum_m_virtual_744_kept_as_witness: bool,
    pub contains_744_tag_summary: bool,
    pub contains_virtual_witness: bool,
    pub contains_question_mark_placeholder: bool,
    pub rendered_lines: Vec<String>,
    pub warnings: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewVirtualColumnSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub policies: Vec<String>,
    pub default_policy: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewVirtualColumnBundle;

impl TableViewVirtualColumnBundle {
    pub fn snapshot(&self) -> TableViewVirtualColumnSnapshot {
        TableViewVirtualColumnSnapshot {
            class: "TableViewVirtualColumnBundle".to_string(),
            morphisms: vec![
                "parse_table_view_virtual_column_cli_options".to_string(),
                "virtual_column_report_for_cli_args".to_string(),
                "virtual_column_report_from_view".to_string(),
                "continuum_m_virtual_column_policy_smoke".to_string(),
            ],
            policies: vec![
                VirtualColumnDisplayPolicy::Suppress.canonical().to_string(),
                VirtualColumnDisplayPolicy::Placeholder.canonical().to_string(),
                VirtualColumnDisplayPolicy::TagSummary.canonical().to_string(),
                VirtualColumnDisplayPolicy::Witness.canonical().to_string(),
            ],
            default_policy: VirtualColumnDisplayPolicy::Suppress.canonical().to_string(),
            universal_property:
                "virtual columns remain witnesses unless an explicit local policy renders them"
                    .to_string(),
        }
    }

    pub fn report_for_cli_args<S: AsRef<str>>(
        &self,
        args: &[S],
        config: &TableViewVirtualColumnConfig,
    ) -> TableViewVirtualColumnReport {
        virtual_column_report_for_cli_args(args, config)
    }
}

pub fn bootstrap_table_view_virtual_columns() -> TableViewVirtualColumnBundle {
    TableViewVirtualColumnBundle
}

pub fn parse_table_view_virtual_column_cli_options<S: AsRef<str>>(
    args: &[S],
) -> TableViewVirtualColumnCliOptions {
    let mut options = TableViewVirtualColumnCliOptions::default();
    for arg in args {
        let raw = arg.as_ref();
        let Some(body) = raw.strip_prefix("--") else {
            continue;
        };
        let key = body
            .split_once('=')
            .map(|(key, _)| key)
            .unwrap_or(body)
            .trim()
            .to_ascii_lowercase();
        let recognized = match key.as_str() {
            "virtualcolumns"
            | "virtualcolumnsummary"
            | "virtualsummary"
            | "virtualtags"
            | "showvirtualcolumns"
            | "virtuellespalten"
            | "virtuellespaltenzusammenfassung"
            | "virtuellenspalten" => {
                options.policy = Some(VirtualColumnDisplayPolicy::TagSummary);
                true
            }
            "virtualplaceholder"
            | "virtualplaceholders"
            | "virtualquestionmarks"
            | "virtualquestionmark"
            | "virtuelleplatzhalter"
            | "virtuellefragezeichen" => {
                options.policy = Some(VirtualColumnDisplayPolicy::Placeholder);
                options.suppress_question_mark_virtuals = Some(false);
                true
            }
            "novirtualquestionmarks" | "suppressvirtualquestionmarks" => {
                options.suppress_question_mark_virtuals = Some(true);
                true
            }
            "virtualwitness"
            | "virtualcolumnwitness"
            | "virtualwitnesses"
            | "virtuellenspaltenwitness"
            | "virtuellenspaltenzeugen" => {
                options.policy = Some(VirtualColumnDisplayPolicy::Witness);
                options.suppress_question_mark_virtuals = Some(false);
                true
            }
            "suppressvirtualcolumns"
            | "hidevirtualcolumns"
            | "novirtualcolumns"
            | "keinevirtuellenspalten" => {
                options.policy = Some(VirtualColumnDisplayPolicy::Suppress);
                options.suppress_question_mark_virtuals = Some(true);
                true
            }
            _ => false,
        };
        if recognized {
            options.recognized_option_count += 1;
            options.requested_flags.push(raw.to_string());
        }
    }
    options
}

pub fn virtual_column_report_for_cli_args<S: AsRef<str>>(
    args: &[S],
    config: &TableViewVirtualColumnConfig,
) -> TableViewVirtualColumnReport {
    let materialization = bootstrap_table_materialization().materialize_cli_args(
        args,
        &TableMaterializationConfig::default(),
    );
    let view = bootstrap_table_view().view_from_report(&materialization, &config.to_view_config());
    virtual_column_report_from_view(&view, config)
}

pub fn virtual_column_report_from_view(
    view: &MaterializedTableView,
    config: &TableViewVirtualColumnConfig,
) -> TableViewVirtualColumnReport {
    let rendered_text = view.rendered_text();
    TableViewVirtualColumnReport {
        class: "TableViewVirtualColumnReport".to_string(),
        policy: config.policy.canonical().to_string(),
        suppress_question_mark_virtuals: config.suppress_question_mark_virtuals,
        selected_column_count: view.selected_column_count,
        virtual_column_count: view.virtual_column_count,
        rendered_virtual_cell_count: view.rendered_virtual_cell_count,
        continuum_m_virtual_744_kept_as_witness: view.continuum_m_virtual_744_kept_as_witness,
        contains_744_tag_summary: rendered_text.contains("744:sternPolygon,keinParaOdMetaP"),
        contains_virtual_witness: rendered_text.contains("virtual:744:"),
        contains_question_mark_placeholder: rendered_text.contains('?'),
        rendered_lines: view.rendered_lines.clone(),
        warnings: view.warnings.clone(),
        universal_property:
            "same materialized virtual witness has suppress/tag/placeholder/witness projections"
                .to_string(),
    }
}

pub fn continuum_m_virtual_column_policy_smoke(
    policy: VirtualColumnDisplayPolicy,
) -> TableViewVirtualColumnReport {
    let config = TableViewVirtualColumnConfig {
        policy,
        suppress_question_mark_virtuals: policy != VirtualColumnDisplayPolicy::Placeholder,
    };
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
    virtual_column_report_for_cli_args(&args, &config)
}

pub fn non_direct_999_virtual_column_policy_smoke(
    policy: VirtualColumnDisplayPolicy,
) -> TableViewVirtualColumnReport {
    let config = TableViewVirtualColumnConfig {
        policy,
        suppress_question_mark_virtuals: policy != VirtualColumnDisplayPolicy::Placeholder,
    };
    let args = vec![
        "reta".to_string(),
        "-zeilen".to_string(),
        "--vorhervonausschnitt=1-1".to_string(),
        "-spalten".to_string(),
        "--religion=999".to_string(),
        "--breite=0".to_string(),
    ];
    virtual_column_report_for_cli_args(&args, &config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continuum_m_744_is_direct_after_religion_csv_update() {
        let report = continuum_m_virtual_column_policy_smoke(VirtualColumnDisplayPolicy::Suppress);
        assert!(!report.continuum_m_virtual_744_kept_as_witness);
        assert_eq!(report.virtual_column_count, 0);
        assert_eq!(report.rendered_virtual_cell_count, 0);
        assert!(!report.contains_744_tag_summary);
        assert!(report.rendered_lines.iter().any(|line| line.contains("Neues M")));
    }

    #[test]
    fn tag_summary_policy_renders_non_direct_999_when_requested() {
        let report = non_direct_999_virtual_column_policy_smoke(VirtualColumnDisplayPolicy::TagSummary);
        assert!(report.rendered_virtual_cell_count > 0);
        assert!(report.rendered_lines.iter().any(|line| line.contains("999:untagged")));
    }

    #[test]
    fn placeholder_policy_can_emit_question_mark_witnesses_for_non_direct_columns() {
        let report = non_direct_999_virtual_column_policy_smoke(VirtualColumnDisplayPolicy::Placeholder);
        assert!(report.contains_question_mark_placeholder);
        assert!(report.rendered_virtual_cell_count > 0);
    }

    #[test]
    fn cli_virtual_flags_resolve_to_expected_policies() {
        let options = parse_table_view_virtual_column_cli_options(&[
            "reta",
            "-ausgabe",
            "--virtualcolumns",
            "--virtualquestionmarks",
        ]);
        assert_eq!(options.policy, Some(VirtualColumnDisplayPolicy::Placeholder));
        assert_eq!(options.suppress_question_mark_virtuals, Some(false));
        assert_eq!(options.recognized_option_count, 2);
    }
}
