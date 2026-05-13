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
