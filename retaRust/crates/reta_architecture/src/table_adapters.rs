//! Architecture-owned compatibility adapters transcompiled from
//! `python_arch_reference/reta_architecture/table_adapters.py`.
//!
//! The historical Python classes `Prepare` and `Concat` were stateful adapter
//! shells over table preparation, wrapping, filtering, generated columns,
//! meta-columns and concat CSV logic.  This Rust module keeps those adapter
//! shells typed while delegating deep semantics to the already ported bundles.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::concat_csv::{bootstrap_concat_csv, ConcatCsvBundle};
use crate::generated_columns::{bootstrap_generated_columns, GeneratedColumnsBundle};
use crate::meta_columns::{bootstrap_meta_columns, MetaColumnsBundle};
use crate::row_filtering::{bootstrap_row_filtering, RowFilterContext, RowFilteringBundle};
use crate::table_preparation::{
    bootstrap_table_preparation, PreparedTable, TablePreparationBundle, TablePreparationContext,
};
use crate::table_wrapping::{
    alxwrap, bootstrap_table_wrapping, split_more_if_not_small, TableWrappingBundle, WrapType,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PrepareAdapter {
    pub highest_rows: BTreeMap<i64, i64>,
    pub original_lines_range_len: usize,
    pub shell_rows_amount: Option<String>,
    pub zaehlungen: Vec<BTreeMap<String, i64>>,
    pub religion_numbers: Vec<i64>,
    pub gezaehlt: bool,
    pub if_zeilen_setted: bool,
    pub breiten: Vec<usize>,
    pub nummerierung: bool,
    pub textwidth: usize,
}

impl Default for PrepareAdapter {
    fn default() -> Self {
        Self {
            highest_rows: BTreeMap::new(),
            original_lines_range_len: 0,
            shell_rows_amount: None,
            zaehlungen: vec![BTreeMap::new(); 5],
            religion_numbers: Vec::new(),
            gezaehlt: false,
            if_zeilen_setted: false,
            breiten: Vec::new(),
            nummerierung: true,
            textwidth: 21,
        }
    }
}

impl PrepareAdapter {
    pub fn new(highest_rows: BTreeMap<i64, i64>) -> Self {
        let original_lines_range_len = highest_rows
            .get(&1024)
            .copied()
            .unwrap_or_default()
            .saturating_add(4) as usize;
        Self {
            highest_rows,
            original_lines_range_len,
            ..Self::default()
        }
    }

    pub fn wrapping(&self, text: &str, length: usize, wrapping_type: WrapType) -> Vec<String> {
        crate::table_wrapping::wrap_cell_text(text, length, Some(wrapping_type))
            .unwrap_or_else(|| vec![text.to_string()])
    }

    pub fn set_width(&self, row_to_display: i64, combi_rows: i64) -> usize {
        let context = crate::table_wrapping::TableWidthContext {
            shell_rows_amount: self
                .shell_rows_amount
                .as_ref()
                .and_then(|value| value.parse::<i64>().ok()),
            rows_as_numbers_len: combi_rows.max(0) as usize,
            breiten: self.breiten.iter().map(|value| *value as i64).collect(),
            textwidth: self.textwidth as i64,
        };
        crate::table_wrapping::width_for_row_context(
            &context,
            row_to_display.max(0) as usize,
            combi_rows.max(0) as usize,
        ) as usize
    }

    pub fn parameters_cmd_with_some_bereich(
        &self,
        filter: &RowFilteringBundle,
        text: &str,
        symbol: &str,
        neg: &str,
        keine_neg_beruecksichtigung: bool,
    ) -> BTreeSet<i64> {
        let _context = RowFilterContext::default();
        filter
            .parameters_cmd_with_some_bereich(text, symbol, neg, keine_neg_beruecksichtigung)
            .into_iter()
            .filter_map(|value| value.parse::<i64>().ok())
            .collect()
    }

    pub fn prepare4out(
        &self,
        preparation: &TablePreparationBundle,
        content_table: Vec<Vec<String>>,
        rows_as_numbers: BTreeSet<i64>,
    ) -> PreparedTable {
        let context = TablePreparationContext::default();
        let rows_as_numbers = rows_as_numbers
            .into_iter()
            .filter_map(|value| usize::try_from(value).ok())
            .collect::<BTreeSet<_>>();
        let empty = BTreeSet::new();
        preparation
            .prepare_main_output(&context, &empty, &empty, &content_table, &rows_as_numbers)
            .new_table
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConcatAdapter {
    pub ones: BTreeSet<i64>,
    pub csvs_already_read: BTreeMap<String, String>,
    pub csvs_same: BTreeMap<i64, Vec<i64>>,
    pub brueche_uni: BTreeSet<String>,
    pub brueche_gal: BTreeSet<String>,
    pub generated_bundle: GeneratedColumnsBundle,
    pub meta_bundle: MetaColumnsBundle,
    pub concat_csv_bundle: ConcatCsvBundle,
}

impl Default for ConcatAdapter {
    fn default() -> Self {
        Self {
            ones: BTreeSet::new(),
            csvs_already_read: BTreeMap::new(),
            csvs_same: BTreeMap::from([
                (1, vec![1]),
                (2, vec![2, 4]),
                (3, vec![3, 5]),
                (4, vec![2, 4]),
                (5, vec![3, 5]),
            ]),
            brueche_uni: BTreeSet::new(),
            brueche_gal: BTreeSet::new(),
            generated_bundle: bootstrap_generated_columns(),
            meta_bundle: bootstrap_meta_columns(),
            concat_csv_bundle: bootstrap_concat_csv(),
        }
    }
}

impl ConcatAdapter {
    pub fn concat_love_polygon(&self, row_number: i64) -> String {
        crate::generated_columns::love_polygon_cell(
            &row_number.to_string(),
            &row_number.to_string(),
        )
        .unwrap_or_default()
    }

    pub fn gleichheit_freiheit_vergleich(&self, value: i64) -> String {
        crate::generated_columns::equality_freedom_domination_type(value).to_string()
    }

    pub fn geist_emotion_energie_materie_topologie(&self, value: i64) -> String {
        crate::generated_columns::mind_emotion_energy_matter_topology_type(value).to_string()
    }

    pub fn concat_prim_creativity_type(&self, value: i64) -> String {
        crate::generated_columns::concat_prim_creativity_type(value)
    }

    pub fn meta_number_signature(&self, value: i64) -> String {
        crate::meta_columns::meta_number_signature(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableAdaptersSnapshot {
    pub class: String,
    pub shell_rows_amount: Option<String>,
    pub wrapping_type: String,
    pub prepare_adapter_fields: usize,
    pub concat_adapter_fields: usize,
    pub morphisms: Vec<String>,
    pub compatibility_classes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableAdaptersBundle {
    pub wrapping: TableWrappingBundle,
    pub filtering: RowFilteringBundle,
    pub preparation: TablePreparationBundle,
    pub prepare: PrepareAdapter,
    pub concat: ConcatAdapter,
}

impl TableAdaptersBundle {
    pub fn set_shell_rows_amount(&mut self, amount: Option<String>) {
        self.prepare.shell_rows_amount = amount;
    }

    pub fn chunks<T: Clone>(&self, values: &[T], size: usize) -> Vec<Vec<T>> {
        values
            .chunks(size.max(1))
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    pub fn split_more_if_not_small(&self, values: &[String], len_to_be: usize) -> Vec<String> {
        split_more_if_not_small(values, len_to_be)
    }

    pub fn alxwrap(&self, text: &str, width: usize) -> Vec<String> {
        alxwrap(text, width, Some(self.wrapping.runtime.wrapping_type))
    }

    pub fn snapshot(&self) -> TableAdaptersSnapshot {
        TableAdaptersSnapshot {
            class: "TableAdaptersBundle".to_string(),
            shell_rows_amount: self.prepare.shell_rows_amount.clone(),
            wrapping_type: self.wrapping.runtime.wrapping_type.py_name().to_string(),
            prepare_adapter_fields: 10,
            concat_adapter_fields: 8,
            morphisms: vec![
                "setShellRowsAmount".to_string(),
                "chunks".to_string(),
                "splitMoreIfNotSmall".to_string(),
                "alxwrap".to_string(),
                "Prepare.prepare4out".to_string(),
                "Prepare.parametersCmdWithSomeBereich".to_string(),
                "Concat.concatLovePolygon".to_string(),
                "Concat.convertSetOfPaarenToDictOfNumToPaareDiv".to_string(),
                "Concat.readConcatCsv".to_string(),
            ],
            compatibility_classes: vec!["Prepare".to_string(), "Concat".to_string()],
        }
    }
}

pub fn bootstrap_table_adapters() -> TableAdaptersBundle {
    TableAdaptersBundle {
        wrapping: bootstrap_table_wrapping(),
        filtering: bootstrap_row_filtering(),
        preparation: bootstrap_table_preparation(),
        prepare: PrepareAdapter::default(),
        concat: ConcatAdapter::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_adapter_uses_generated_morphism() {
        let concat = ConcatAdapter::default();
        assert!(!concat.concat_love_polygon(13).is_empty());
    }
}
