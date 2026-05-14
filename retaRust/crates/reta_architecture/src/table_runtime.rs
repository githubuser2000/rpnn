//! Table runtime/global-section layer transcompiled from
//! `python_arch_reference/reta_architecture/table_runtime.py`.
//!
//! This is intentionally a light Rust owner for the mutable sections.  The
//! historical Python renderer still owns final byte-for-byte output in this
//! transcompilation stage, but Rust now has the same table-state and output-mode
//! sections available for future replacement.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::output_semantics::{bootstrap_output_semantics, OutputConfig, RetaOutputSemantics};
use crate::output_syntax::OutputMode;
use crate::table_state::{bootstrap_table_state, TableStateBundle, TableStateSections};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableRuntimeState {
    pub sections: TableStateSections,
    pub output_config: OutputConfig,
    pub number_rows: bool,
    pub text_height: i64,
    pub widths: Vec<i64>,
}

impl TableRuntimeState {
    pub fn new(highest_row: Option<i64>, state_bundle: &TableStateBundle) -> Self {
        Self {
            sections: state_bundle.create_sections(highest_row),
            output_config: OutputConfig::default(),
            number_rows: true,
            text_height: 0,
            widths: Vec::new(),
        }
    }

    pub fn set_numbering(&mut self, value: bool) {
        self.number_rows = value;
    }

    pub fn set_text_width(&mut self, value: i64, shell_rows_amount: i64) {
        let bounded = if (shell_rows_amount > value + 7 || shell_rows_amount == 0)
            && (value != 0
                || self.output_config.marks_html_or_bbcode
                || self.output_config.one_table)
        {
            value
        } else {
            shell_rows_amount - 7
        };
        self.output_config.text_width = Some(bounded);
    }

    pub fn set_widths(&mut self, values: &[i64], shell_rows_amount: i64) {
        self.widths = values
            .iter()
            .map(|value| {
                if shell_rows_amount > value + 7 || shell_rows_amount == 0 {
                    *value
                } else {
                    shell_rows_amount - 7
                }
            })
            .collect();
    }

    pub fn snapshot(&self) -> TableRuntimeStateSnapshot {
        TableRuntimeStateSnapshot {
            class: "TableRuntimeState".to_string(),
            state_sections: self.sections.snapshot(),
            output_config: self.output_config.clone(),
            number_rows: self.number_rows,
            text_height: self.text_height,
            widths_len: self.widths.len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableRuntimeStateSnapshot {
    pub class: String,
    pub state_sections: crate::table_state::TableStateSectionsSnapshot,
    pub output_config: OutputConfig,
    pub number_rows: bool,
    pub text_height: i64,
    pub widths_len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableRuntimeBundle {
    pub output_semantics: RetaOutputSemantics,
    pub table_state: TableStateBundle,
}

impl TableRuntimeBundle {
    pub fn create_tables_state(&self, highest_row: Option<i64>) -> TableRuntimeState {
        TableRuntimeState::new(highest_row, &self.table_state)
    }

    pub fn fill_both<T: Clone>(&self, mut left: Vec<T>, mut right: Vec<T>, fill: T) -> (Vec<T>, Vec<T>) {
        while left.len() < right.len() {
            left.push(fill.clone());
        }
        while right.len() < left.len() {
            right.push(fill.clone());
        }
        (left, right)
    }

    pub fn table_reduced_in_lines_by_type_set<T: Clone>(
        &self,
        table: &[T],
        lines_allowed: &BTreeSet<usize>,
    ) -> Vec<T> {
        table
            .iter()
            .enumerate()
            .filter_map(|(index, line)| lines_allowed.contains(&index).then_some(line.clone()))
            .collect()
    }

    pub fn snapshot(&self) -> TableRuntimeBundleSnapshot {
        TableRuntimeBundleSnapshot {
            class: "TableRuntimeBundle".to_string(),
            table_class: "TableRuntimeState".to_string(),
            owns_legacy_tables: true,
            legacy_facade: "libs/tableHandling.py".to_string(),
            state_sections: self.table_state.snapshot(),
            component_morphisms: vec![
                "Prepare".to_string(),
                "Concat".to_string(),
                "KombiJoin".to_string(),
                "TableOutput".to_string(),
                "GeneratedColumns".to_string(),
            ],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableRuntimeBundleSnapshot {
    pub class: String,
    pub table_class: String,
    pub owns_legacy_tables: bool,
    pub legacy_facade: String,
    pub state_sections: crate::table_state::TableStateBundleSnapshot,
    pub component_morphisms: Vec<String>,
}

pub fn bootstrap_table_runtime() -> TableRuntimeBundle {
    TableRuntimeBundle {
        output_semantics: bootstrap_output_semantics(),
        table_state: bootstrap_table_state(),
    }
}



#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BreakoutException {
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Maintable {
    pub rows: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tables {
    pub maintable: Maintable,
    pub state: TableRuntimeState,
}

impl Tables {
    pub fn new(rows: Vec<Vec<String>>) -> Self {
        let bundle = bootstrap_table_runtime();
        Self { maintable: Maintable { rows }, state: bundle.create_tables_state(None) }
    }

    pub fn table_state_snapshot(&self) -> TableRuntimeStateSnapshot {
        self.state.snapshot()
    }
}

pub fn __init__() -> Tables {
    Tables::new(Vec::new())
}

pub fn create_tables(rows: Vec<Vec<String>>) -> Tables {
    Tables::new(rows)
}

pub fn create_spalte_gestirn(row_number: i64) -> String {
    crate::generated_columns::create_spalte_gestirn(row_number)
}

pub fn spalteg_gestirn(row_number: i64) -> String {
    create_spalte_gestirn(row_number)
}

pub fn table_state_snapshot(tables: &Tables) -> TableRuntimeStateSnapshot {
    tables.table_state_snapshot()
}

pub fn hoechste_zeile(tables: &Tables) -> Option<usize> {
    tables.maintable.rows.len().checked_sub(1)
}

pub fn gener_rows(tables: &Tables) -> usize {
    tables.maintable.rows.len()
}

pub fn gebr_univ_set(values: &[i64]) -> BTreeSet<i64> {
    values.iter().copied().collect()
}

pub fn if_prim_multis(value: i64, multiples: &[i64]) -> bool {
    crate::number_theory::is_prime_multiple(value, multiples)
}

pub fn if_zeilen_setted(lines: &BTreeSet<usize>) -> bool {
    !lines.is_empty()
}

pub fn nummeriere(state: &TableRuntimeState) -> bool {
    state.number_rows
}

pub fn breitenn(state: &TableRuntimeState) -> Vec<i64> {
    state.widths.clone()
}

pub fn text_width(state: &TableRuntimeState) -> Option<i64> {
    state.output_config.text_width
}

pub fn text_height(state: &TableRuntimeState) -> i64 {
    state.text_height
}

pub fn out_type(state: &TableRuntimeState) -> OutputMode {
    state.output_config.mode
}

pub fn output_mode_name(state: &TableRuntimeState) -> String {
    state.output_config.mode.canonical_name().to_string()
}

pub fn keine_ueberschriften(_state: &TableRuntimeState) -> bool {
    false
}

pub fn keineleereninhalte(_state: &TableRuntimeState) -> bool {
    false
}

pub fn html_output_yes(state: &TableRuntimeState) -> bool {
    state.output_config.mode == OutputMode::Html
}

pub fn bbcode_output_yes(state: &TableRuntimeState) -> bool {
    state.output_config.mode == OutputMode::Bbcode
}

pub fn markdown_output_yes(state: &TableRuntimeState) -> bool {
    state.output_config.mode == OutputMode::Markdown
}

pub fn nichts_output_yes(state: &TableRuntimeState) -> bool {
    state.output_config.mode == OutputMode::Nichts
}

pub fn _concat_class() -> &'static str {
    "ConcatAdapter"
}

pub fn _prepare_class() -> &'static str {
    "PrepareAdapter"
}

pub fn _get_text_wrap_things(text: &str, width: usize) -> Vec<String> {
    crate::table_wrapping::wrap_cell_text(text, width, None).unwrap_or_else(|| vec![text.to_string()])
}

#[allow(non_snake_case)]
pub fn NichtsOutputYes(state: &TableRuntimeState) -> bool {
    nichts_output_yes(state)
}

#[allow(non_snake_case)]
pub fn htmlOutputYes(state: &TableRuntimeState) -> bool {
    html_output_yes(state)
}

#[allow(non_snake_case)]
pub fn bbcodeOutputYes(state: &TableRuntimeState) -> bool {
    bbcode_output_yes(state)
}

#[allow(non_snake_case)]
pub fn markdownOutputYes(state: &TableRuntimeState) -> bool {
    markdown_output_yes(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_state_starts_like_python_tables() {
        let bundle = bootstrap_table_runtime();
        let state = bundle.create_tables_state(None);
        assert_eq!(state.sections.highest_rows[&1024], 1024);
        assert!(state.number_rows);
        assert_eq!(state.output_config.text_width, Some(21));
    }

    #[test]
    fn fill_both_pads_shorter_side() {
        let bundle = bootstrap_table_runtime();
        let (left, right) = bundle.fill_both(vec![1], vec![2, 3], 0);
        assert_eq!(left, vec![1, 0]);
        assert_eq!(right, vec![2, 3]);
    }
}
