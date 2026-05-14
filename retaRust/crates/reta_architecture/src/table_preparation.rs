//! Table-output preparation morphisms transcompiled from
//! `python_arch_reference/reta_architecture/table_preparation.py`.
//!
//! The legacy Python `Prepare` object still performs many side effects.  This
//! Rust module owns the pure row/cell/tag preparation surface: selecting display
//! lines, wrapping cells, maintaining old/new column maps and deriving generated
//! column tags.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::row_filtering::{RowFilterContext, RowFilteringBundle};
use crate::table_wrapping::{wrap_cell_text, TableWidthContext};
use crate::tag_schema::{
    kombi_table2_tags_for_column, kombi_table_tags_for_column, ordinary_tags_for_column, TableTag,
};

pub type PreparedCell = Vec<String>;
pub type PreparedRow = Vec<PreparedCell>;
pub type PreparedTable = Vec<PreparedRow>;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct OldNewTableMap {
    pub original_to_display: BTreeMap<usize, usize>,
    pub display_to_original: BTreeMap<usize, usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MainTablePreparationResult {
    pub finally_display_lines: BTreeSet<usize>,
    pub new_table: PreparedTable,
    pub numlen: usize,
    pub rows_range: Vec<usize>,
    pub old2new_table: OldNewTableMap,
}

impl MainTablePreparationResult {
    pub fn snapshot(&self) -> MainTablePreparationResultSnapshot {
        MainTablePreparationResultSnapshot {
            class: "MainTablePreparationResult".to_string(),
            finally_display_lines_len: self.finally_display_lines.len(),
            new_table_len: self.new_table.len(),
            numlen: self.numlen,
            rows_range_len: self.rows_range.len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MainTablePreparationResultSnapshot {
    pub class: String,
    pub finally_display_lines_len: usize,
    pub new_table_len: usize,
    pub numlen: usize,
    pub rows_range_len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KombiTablePreparationResult {
    pub finally_display_lines: BTreeSet<usize>,
    pub new_table: PreparedTable,
    pub line_len: usize,
    pub animals_professions_table: Vec<Vec<String>>,
    pub old2new_table_animals_professions: OldNewTableMap,
}

impl KombiTablePreparationResult {
    pub fn snapshot(&self) -> KombiTablePreparationResultSnapshot {
        KombiTablePreparationResultSnapshot {
            class: "KombiTablePreparationResult".to_string(),
            finally_display_lines_len: self.finally_display_lines.len(),
            new_table_len: self.new_table.len(),
            line_len: self.line_len,
            animals_professions_table_len: self.animals_professions_table.len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KombiTablePreparationResultSnapshot {
    pub class: String,
    pub finally_display_lines_len: usize,
    pub new_table_len: usize,
    pub line_len: usize,
    pub animals_professions_table_len: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct GebrSpalten {
    pub gal: BTreeSet<usize>,
    pub gal2: BTreeSet<usize>,
    pub uni: BTreeSet<usize>,
    pub uni2: BTreeSet<usize>,
    pub emo: BTreeSet<usize>,
    pub emo2: BTreeSet<usize>,
    pub groe: BTreeSet<usize>,
    pub groe2: BTreeSet<usize>,
}

impl GebrSpalten {
    fn contains_gal(&self, value: usize) -> bool {
        self.gal.contains(&value) || self.gal2.contains(&value)
    }

    fn contains_uni(&self, value: usize) -> bool {
        self.uni.contains(&value) || self.uni2.contains(&value)
    }

    fn contains_emo(&self, value: usize) -> bool {
        self.emo.contains(&value) || self.emo2.contains(&value)
    }

    fn contains_groe(&self, value: usize) -> bool {
        self.groe.contains(&value) || self.groe2.contains(&value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TablePreparationContext {
    pub row_filter: RowFilterContext,
    pub width: TableWidthContext,
    pub if_zeilen_setted: bool,
}

impl Default for TablePreparationContext {
    fn default() -> Self {
        let row_filter = RowFilterContext::default();
        Self {
            if_zeilen_setted: row_filter.if_zeilen_setted,
            row_filter,
            width: TableWidthContext::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DisplayLineSelection {
    pub finally_display_lines: BTreeSet<usize>,
    pub headings_amount: usize,
    pub numlen: usize,
    pub rows_range: Vec<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PreparedRowWithMap {
    pub row: PreparedRow,
    pub old2new: OldNewTableMap,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TablePreparationBundle {
    pub row_filtering: RowFilteringBundle,
}

impl TablePreparationBundle {
    pub fn select_display_lines(
        &self,
        context: &TablePreparationContext,
        content_table: &[Vec<String>],
        param_lines: &BTreeSet<String>,
        param_lines_not: &BTreeSet<String>,
    ) -> DisplayLineSelection {
        select_display_lines(
            &self.row_filtering,
            context,
            content_table,
            param_lines,
            param_lines_not,
        )
    }

    pub fn prepare_row_cells(
        &self,
        line: &[String],
        rows_as_numbers: &BTreeSet<usize>,
        width_context: &TableWidthContext,
    ) -> PreparedRowWithMap {
        prepare_row_cells(line, rows_as_numbers, width_context)
    }

    pub fn tag_output_column(
        &self,
        combi_rows: usize,
        gebr_spalten: &GebrSpalten,
        prim_spalten: Option<&BTreeSet<usize>>,
        t: usize,
        kombi_csv_number: usize,
    ) -> Option<BTreeSet<TableTag>> {
        tag_output_column(combi_rows, gebr_spalten, prim_spalten, t, kombi_csv_number)
    }

    pub fn cell_work(&self, cell: &str, certaintextwidth: usize) -> PreparedCell {
        cell_work(cell, certaintextwidth)
    }

    pub fn deduplicate_parameter_sections<T: Ord + Clone>(
        &self,
        set1: &BTreeSet<T>,
        set2: &BTreeSet<T>,
    ) -> (BTreeSet<T>, BTreeSet<T>) {
        self.row_filtering.delete_doubles_in_sets(set1, set2)
    }

    pub fn prepare_main_output(
        &self,
        context: &TablePreparationContext,
        param_lines: &BTreeSet<String>,
        param_lines_not: &BTreeSet<String>,
        content_table: &[Vec<String>],
        rows_as_numbers: &BTreeSet<usize>,
    ) -> MainTablePreparationResult {
        prepare_output_table(
            self,
            context,
            param_lines,
            param_lines_not,
            content_table,
            rows_as_numbers,
        )
    }

    pub fn snapshot(&self) -> TablePreparationBundleSnapshot {
        TablePreparationBundleSnapshot {
            class: "TablePreparationBundle".to_string(),
            display_line_morphism: "select_display_lines".to_string(),
            row_morphism: "prepare_row_cells".to_string(),
            tag_gluing_morphism: "tag_output_column".to_string(),
            cell_morphism: "cell_work".to_string(),
            parallel_row_morphism: "prepare_rows_in_processes".to_string(),
            deduplication_morphism: "deduplicate_parameter_sections".to_string(),
            last_line_morphism: "capture_last_line_number".to_string(),
            universal_operations: vec![
                "deduplicate_parameter_sections".to_string(),
                "capture_last_line_number".to_string(),
                "prepare_main_output".to_string(),
                "prepare_kombi_output".to_string(),
                "process_parallel_row_chunks".to_string(),
            ],
            main_table_result: "MainTablePreparationResult".to_string(),
            kombi_table_result: "KombiTablePreparationResult".to_string(),
            legacy_delegate: "libs.lib4tables_prepare.Prepare".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TablePreparationBundleSnapshot {
    pub class: String,
    pub display_line_morphism: String,
    pub row_morphism: String,
    pub tag_gluing_morphism: String,
    pub cell_morphism: String,
    pub parallel_row_morphism: String,
    pub deduplication_morphism: String,
    pub last_line_morphism: String,
    pub universal_operations: Vec<String>,
    pub main_table_result: String,
    pub kombi_table_result: String,
    pub legacy_delegate: String,
}

pub fn bootstrap_table_preparation() -> TablePreparationBundle {
    TablePreparationBundle {
        row_filtering: crate::row_filtering::bootstrap_row_filtering(),
    }
}

pub fn select_display_lines(
    row_filtering: &RowFilteringBundle,
    context: &TablePreparationContext,
    content_table: &[Vec<String>],
    param_lines: &BTreeSet<String>,
    param_lines_not: &BTreeSet<String>,
) -> DisplayLineSelection {
    let headings_amount = content_table.first().map(|row| row.len()).unwrap_or(0);
    let rows_range = (0..headings_amount).collect::<Vec<_>>();
    let original = context
        .row_filter
        .original_lines_range
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let mut finally_display_lines = row_filtering
        .filter_original_lines(&context.row_filter, original, param_lines)
        .into_iter()
        .filter_map(|value| usize::try_from(value).ok())
        .collect::<BTreeSet<_>>();

    if !param_lines_not.is_empty() {
        let excluded = row_filtering
            .filter_original_lines(
                &context.row_filter,
                finally_display_lines
                    .iter()
                    .map(|value| *value as i64)
                    .collect(),
                param_lines_not,
            )
            .into_iter()
            .filter_map(|value| usize::try_from(value).ok())
            .collect::<BTreeSet<_>>();
        let changed = context
            .row_filter
            .original_lines_range
            .iter()
            .filter(|value| **value != 0)
            .any(|value| {
                !excluded.contains(&(*value as usize))
                    && finally_display_lines.contains(&(*value as usize))
            });
        if changed {
            finally_display_lines = finally_display_lines
                .difference(&excluded)
                .copied()
                .collect();
        }
    }

    if finally_display_lines.is_empty() {
        if context.if_zeilen_setted {
            finally_display_lines = BTreeSet::new();
        } else {
            finally_display_lines =
                (0..=context.row_filter.highest_row_1024.max(0) as usize).collect();
        }
    }
    finally_display_lines.insert(0);
    let last = finally_display_lines
        .iter()
        .next_back()
        .copied()
        .unwrap_or(0);
    let numlen = last.to_string().len();
    DisplayLineSelection {
        finally_display_lines,
        headings_amount,
        numlen,
        rows_range,
    }
}

pub fn prepare_output_table(
    bundle: &TablePreparationBundle,
    context: &TablePreparationContext,
    param_lines: &BTreeSet<String>,
    param_lines_not: &BTreeSet<String>,
    content_table: &[Vec<String>],
    rows_as_numbers: &BTreeSet<usize>,
) -> MainTablePreparationResult {
    let selection =
        bundle.select_display_lines(context, content_table, param_lines, param_lines_not);
    let mut new_table = Vec::new();
    let mut old2new_table = OldNewTableMap::default();

    for (row_index, line) in content_table.iter().enumerate() {
        if selection.finally_display_lines.contains(&row_index) {
            let prepared = bundle.prepare_row_cells(line, rows_as_numbers, &context.width);
            if row_index == 0 {
                old2new_table = prepared.old2new.clone();
            }
            if !prepared.row.is_empty() {
                new_table.push(prepared.row);
            }
        }
    }

    MainTablePreparationResult {
        finally_display_lines: selection.finally_display_lines,
        new_table,
        numlen: selection.numlen,
        rows_range: selection.rows_range,
        old2new_table,
    }
}

pub fn prepare_row_cells(
    line: &[String],
    rows_as_numbers: &BTreeSet<usize>,
    width_context: &TableWidthContext,
) -> PreparedRowWithMap {
    let mut row = Vec::new();
    let mut old2new = OldNewTableMap::default();
    let selected_columns = if rows_as_numbers.is_empty() {
        (0..line.len()).collect::<BTreeSet<_>>()
    } else {
        rows_as_numbers.clone()
    };

    let mut row_to_display = 0usize;
    for (t, cell) in line.iter().enumerate() {
        if selected_columns.contains(&t) {
            row_to_display += 1;
            let width =
                crate::table_wrapping::width_for_row_context(width_context, row_to_display, 0);
            let width = usize::try_from(width.max(0)).unwrap_or(0);
            let display_index = row.len();
            row.push(cell_work(cell, width));
            old2new.original_to_display.insert(t, display_index);
            old2new.display_to_original.insert(display_index, t);
        }
    }
    PreparedRowWithMap { row, old2new }
}

pub fn tag_output_column(
    combi_rows: usize,
    gebr_spalten: &GebrSpalten,
    prim_spalten: Option<&BTreeSet<usize>>,
    t: usize,
    kombi_csv_number: usize,
) -> Option<BTreeSet<TableTag>> {
    if combi_rows == 0 {
        if prim_spalten.is_some_and(|set| set.contains(&t)) {
            return Some(BTreeSet::from([
                TableTag::sternPolygon,
                TableTag::universum,
                TableTag::galaxie,
            ]));
        }
        if gebr_spalten.contains_gal(t) {
            return Some(BTreeSet::from([
                TableTag::sternPolygon,
                TableTag::galaxie,
                TableTag::gleichfoermigesPolygon,
                TableTag::gebrRat,
            ]));
        }
        if gebr_spalten.contains_uni(t) {
            return Some(BTreeSet::from([
                TableTag::sternPolygon,
                TableTag::universum,
                TableTag::gleichfoermigesPolygon,
                TableTag::gebrRat,
            ]));
        }
        if gebr_spalten.contains_emo(t) || gebr_spalten.contains_groe(t) {
            return Some(BTreeSet::from([
                TableTag::sternPolygon,
                TableTag::keinParaOdMetaP,
                TableTag::gleichfoermigesPolygon,
                TableTag::gebrRat,
            ]));
        }
        ordinary_tags_for_column(t as i64)
    } else if kombi_csv_number == 0 {
        kombi_table_tags_for_column(t as i64)
    } else if kombi_csv_number == 1 {
        kombi_table2_tags_for_column(t as i64)
    } else {
        None
    }
}

pub fn cell_work(cell: &str, certaintextwidth: usize) -> PreparedCell {
    let cell = cell.trim();
    if certaintextwidth == 0 {
        return vec![cell.to_string()];
    }
    let mut out = Vec::new();
    let mut rest = cell.to_string();
    loop {
        match wrap_cell_text(&rest, certaintextwidth, None) {
            Some(mut wrapped) if !wrapped.is_empty() => {
                if wrapped.len() == 1 && wrapped[0] == rest {
                    out.push(rest.chars().take(certaintextwidth).collect());
                    let remainder = rest.chars().skip(certaintextwidth).collect::<String>();
                    if remainder.is_empty() {
                        break;
                    }
                    rest = remainder;
                } else {
                    let last = wrapped.pop().unwrap_or_default();
                    out.extend(wrapped);
                    rest = last;
                    if rest.chars().count() <= certaintextwidth {
                        out.push(rest);
                        break;
                    }
                }
            }
            _ => {
                out.push(rest.chars().take(certaintextwidth).collect());
                break;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_work_splits_long_cells_like_prepare_fallback() {
        assert_eq!(cell_work("abcdef", 2), vec!["ab", "cd", "ef"]);
        assert_eq!(cell_work("abcdef", 0), vec!["abcdef"]);
    }

    #[test]
    fn header_map_tracks_selected_columns() {
        let width = TableWidthContext {
            shell_rows_amount: Some(80),
            rows_as_numbers_len: 2,
            breiten: vec![10, 10],
            textwidth: 10,
        };
        let prepared = prepare_row_cells(
            &["a".to_string(), "b".to_string(), "c".to_string()],
            &BTreeSet::from([0usize, 2usize]),
            &width,
        );
        assert_eq!(prepared.row.len(), 2);
        assert_eq!(prepared.old2new.original_to_display[&2], 1);
    }

    #[test]
    fn tag_gluing_preserves_current_744_mapping() {
        let tags = tag_output_column(0, &GebrSpalten::default(), None, 744, 0).unwrap();
        assert!(tags.contains(&TableTag::sternPolygon));
        assert!(tags.contains(&TableTag::keinParaOdMetaP));
    }
}
