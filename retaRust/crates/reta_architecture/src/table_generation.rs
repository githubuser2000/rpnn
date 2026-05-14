//! Table-generation orchestration layer transcompiled from
//! `python_arch_reference/reta_architecture/table_generation.py`.
//!
//! Stage 4 keeps the legacy generators as behaviour owners but gives Rust a
//! typed generation plan/result.  This is the point where selected columns,
//! generated columns, CSV concat sections and Kombi joins are glued into one
//! table-generation morphism.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::column_selection::{ColumnBucketKey, ColumnSelectionBundle};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableGenerationResult {
    pub animals_professions_table_len: usize,
    pub rows_of_combi: BTreeSet<i64>,
    pub kombi_table_kombis_len: usize,
    pub maintable2subtable_relation_len: usize,
    pub prim_spalten_present: bool,
    pub gebr: BTreeMap<String, BTreeSet<i64>>,
    pub animals_professions_table2_len: usize,
    pub kombi_table_kombis2_len: usize,
    pub maintable2subtable_relation2_len: usize,
}

impl TableGenerationResult {
    pub fn snapshot(&self) -> TableGenerationResultSnapshot {
        TableGenerationResultSnapshot {
            class: "TableGenerationResult".to_string(),
            has_prim_spalten: self.prim_spalten_present,
            gebr_keys: self.gebr.keys().cloned().collect(),
            kombi_rows_len: self.rows_of_combi.len(),
            animals_professions_table_len: self.animals_professions_table_len,
            animals_professions_table2_len: self.animals_professions_table2_len,
            csv_asset_count: crate::csv_catalog::csv_asset_count(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableGenerationResultSnapshot {
    pub class: String,
    pub has_prim_spalten: bool,
    pub gebr_keys: Vec<String>,
    pub kombi_rows_len: usize,
    pub animals_professions_table_len: usize,
    pub animals_professions_table2_len: usize,
    pub csv_asset_count: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableGenerationPlan {
    pub selected_rows: BTreeSet<i64>,
    pub selected_columns: BTreeSet<i64>,
    pub generated_rows: BTreeSet<i64>,
    pub rows_of_combi: BTreeSet<i64>,
    pub rows_of_combi2: BTreeSet<i64>,
    pub concat_columns: BTreeSet<i64>,
    pub symbolic_column_buckets: BTreeMap<ColumnBucketKey, BTreeSet<String>>,
    /// Explicit legacy output order from `--spaltenreihenfolgeundnurdiese`.
    /// Python treats this as both order and restriction for the rendered image;
    /// Stage 26 carries it through generation/materialization without changing
    /// the default path when the option is absent.
    pub column_order_override: Vec<i64>,
    pub csv_asset_names: Vec<String>,
}

impl TableGenerationPlan {
    pub fn from_column_buckets(
        buckets: &BTreeMap<ColumnBucketKey, BTreeSet<i64>>,
        rows: impl IntoIterator<Item = i64>,
    ) -> Self {
        let get = |bucket: u8| -> BTreeSet<i64> {
            buckets
                .get(&ColumnBucketKey::positive(bucket))
                .cloned()
                .unwrap_or_default()
        };
        Self {
            selected_rows: rows.into_iter().collect(),
            selected_columns: get(0),
            generated_rows: get(1),
            concat_columns: get(2),
            rows_of_combi: get(3),
            rows_of_combi2: get(8),
            symbolic_column_buckets: BTreeMap::new(),
            column_order_override: Vec::new(),
            csv_asset_names: csv_asset_names_for_bucket_state(
                &get(0),
                &get(2),
                &get(3),
                &get(8),
                &BTreeMap::new(),
            ),
        }
    }

    pub fn requires_kombi(&self) -> bool {
        !self.rows_of_combi.is_empty() || !self.rows_of_combi2.is_empty()
    }

    pub fn requires_concat_csv(&self) -> bool {
        !self.concat_columns.is_empty()
    }

    pub fn requires_symbolic_generated_sections(&self) -> bool {
        !self.symbolic_column_buckets.is_empty()
    }

    /// Effective legacy column order for materialization/rendering.
    ///
    /// Without an explicit override, the stable sorted `BTreeSet` order is used.
    /// With `--spaltenreihenfolgeundnurdiese`, Python's option name says exactly
    /// what should happen: use this order and only these columns.  If no `-spalten`
    /// selection was parsed, the override itself becomes the selected ordinary
    /// column set so the materialization probe can still render the requested
    /// columns.
    pub fn ordered_selected_columns(&self) -> Vec<i64> {
        if self.column_order_override.is_empty() {
            return self.selected_columns.iter().copied().collect();
        }
        let mut out = Vec::new();
        let selected_is_empty = self.selected_columns.is_empty();
        for column in &self.column_order_override {
            if *column <= 0 {
                continue;
            }
            if selected_is_empty || self.selected_columns.contains(column) {
                if !out.contains(column) {
                    out.push(*column);
                }
            }
        }
        out
    }

    pub fn column_order_override_applies(&self) -> bool {
        !self.column_order_override.is_empty()
    }

    pub fn from_parameter_command_sets(
        sets: &crate::parameter_runtime::ParameterCommandSets,
    ) -> Self {
        let normalized = crate::column_selection::bootstrap_column_selection()
            .normalize_bucket_map(&sets.column_buckets);
        let mut plan = Self::from_column_buckets(&normalized, sets.rows_as_numbers.iter().copied());
        plan.symbolic_column_buckets = sets.symbolic_column_buckets.clone();
        plan.column_order_override = sets.spaltenreihenfolgeundnurdiese.clone();
        if plan.selected_columns.is_empty() && !plan.column_order_override.is_empty() {
            plan.selected_columns = plan
                .column_order_override
                .iter()
                .copied()
                .filter(|column| *column > 0)
                .collect();
        }
        plan.csv_asset_names = csv_asset_names_for_bucket_state(
            &plan.selected_columns,
            &plan.concat_columns,
            &plan.rows_of_combi,
            &plan.rows_of_combi2,
            &plan.symbolic_column_buckets,
        );
        plan
    }
}

pub fn csv_asset_names_for_bucket_state(
    selected_columns: &BTreeSet<i64>,
    concat_columns: &BTreeSet<i64>,
    rows_of_combi: &BTreeSet<i64>,
    rows_of_combi2: &BTreeSet<i64>,
    symbolic_column_buckets: &BTreeMap<ColumnBucketKey, BTreeSet<String>>,
) -> Vec<String> {
    let mut names = BTreeSet::new();
    if !selected_columns.is_empty() {
        names.insert("religion.csv".to_string());
    }
    if !concat_columns.is_empty() {
        names.insert("primenumbers.csv".to_string());
    }
    if !rows_of_combi.is_empty() {
        names.insert("kombi.csv".to_string());
    }
    if !rows_of_combi2.is_empty() {
        names.insert("kombi-meta.csv".to_string());
    }
    for (bucket, symbols) in symbolic_column_buckets {
        if symbols.is_empty() {
            continue;
        }
        match bucket.bucket {
            2 | 7 => {
                names.insert("primenumbers.csv".to_string());
            }
            5 => {
                names.insert("gebrochen-rational-universum.csv".to_string());
            }
            6 => {
                names.insert("gebrochen-rational-galaxie.csv".to_string());
            }
            9 => {
                names.insert("gebrochen-rational-emotionen.csv".to_string());
            }
            10 => {
                names.insert("gebrochen-rational-strukturgroesse.csv".to_string());
            }
            3 => {
                names.insert("kombi.csv".to_string());
            }
            8 => {
                names.insert("kombi-meta.csv".to_string());
            }
            _ => {}
        }
    }
    names
        .into_iter()
        .filter(|name| crate::csv_catalog::csv_asset_by_name(name).is_some())
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableGenerationBundle {
    pub component_morphisms: Vec<String>,
    pub universal_property: String,
    pub column_selection: ColumnSelectionBundle,
}

impl TableGenerationBundle {
    pub fn build_plan_from_buckets(
        &self,
        buckets: &BTreeMap<ColumnBucketKey, BTreeSet<i64>>,
        rows: impl IntoIterator<Item = i64>,
    ) -> TableGenerationPlan {
        TableGenerationPlan::from_column_buckets(buckets, rows)
    }

    pub fn empty_result_from_plan(&self, plan: &TableGenerationPlan) -> TableGenerationResult {
        let mut gebr = BTreeMap::new();
        for key in ["Gal", "Gal2", "Uni", "Uni2", "Emo", "Emo2", "Groe", "Groe2"] {
            gebr.insert(key.to_string(), BTreeSet::new());
        }
        TableGenerationResult {
            rows_of_combi: plan.rows_of_combi.clone(),
            prim_spalten_present: plan.requires_concat_csv()
                || plan.requires_symbolic_generated_sections(),
            gebr,
            ..TableGenerationResult::default()
        }
    }

    pub fn snapshot(&self) -> TableGenerationBundleSnapshot {
        TableGenerationBundleSnapshot {
            class: "TableGenerationBundle".to_string(),
            component_morphisms: self.component_morphisms.clone(),
            universal_property: self.universal_property.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableGenerationBundleSnapshot {
    pub class: String,
    pub component_morphisms: Vec<String>,
    pub universal_property: String,
}

pub fn bootstrap_table_generation() -> TableGenerationBundle {
    TableGenerationBundle {
        component_morphisms: vec![
            "GeneratedColumns".to_string(),
            "ConcatCsv".to_string(),
            "KombiJoin".to_string(),
            "TablePreparation".to_string(),
            "TableWrapping".to_string(),
            "TableOutput".to_string(),
            "RowFiltering".to_string(),
            "ColumnOrderOverride".to_string(),
        ],
        universal_property:
            "compatible local CSV/generated/Kombi sections glue to one deterministic result table"
                .to_string(),
        column_selection: crate::column_selection::bootstrap_column_selection(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_plan_reads_python_bucket_coordinates() {
        let selection = crate::column_selection::bootstrap_column_selection();
        let mut buckets = selection.new_bucket_map();
        buckets.insert(ColumnBucketKey::positive(0), BTreeSet::from([1, 2]));
        buckets.insert(ColumnBucketKey::positive(3), BTreeSet::from([5]));
        let plan = TableGenerationPlan::from_column_buckets(&buckets, [1, 2, 3]);
        assert_eq!(plan.selected_columns, BTreeSet::from([1, 2]));
        assert!(plan.requires_kombi());
    }

    #[test]
    fn generation_plan_carries_symbolic_parameter_buckets() {
        let runtime = crate::parameter_runtime::bootstrap_parameter_runtime();
        let parsed = runtime.parse_cli_args(&[
            "reta",
            "-spalten",
            "--multiplikationen=motivstern",
            "--gebrochenuniversum=2",
        ]);
        let plan = TableGenerationPlan::from_parameter_command_sets(&parsed.command_sets);
        assert!(plan.requires_symbolic_generated_sections());
        assert!(
            plan.symbolic_column_buckets[&ColumnBucketKey::positive(7)].contains("primMotivStern")
        );
        assert!(plan.symbolic_column_buckets[&ColumnBucketKey::positive(5)].contains("2"));
        assert!(
            plan.csv_asset_names
                .iter()
                .any(|name| name == "primenumbers.csv")
        );
        assert!(
            plan.csv_asset_names
                .iter()
                .any(|name| name == "gebrochen-rational-universum.csv")
        );
    }

    #[test]
    fn generation_plan_honors_spaltenreihenfolgeundnurdiese_order() {
        let runtime = crate::parameter_runtime::bootstrap_parameter_runtime();
        let parsed = runtime.parse_cli_args(&[
            "reta",
            "-spalten",
            "--kontinuum=m",
            "-ausgabe",
            "--spaltenreihenfolgeundnurdiese=744,493",
        ]);
        let plan = TableGenerationPlan::from_parameter_command_sets(&parsed.command_sets);
        assert_eq!(plan.column_order_override, vec![744, 493]);
        assert_eq!(plan.ordered_selected_columns(), vec![744, 493]);
    }

    #[test]
    fn generation_plan_exposes_csv_presheaf_assets() {
        let selection = crate::column_selection::bootstrap_column_selection();
        let mut buckets = selection.new_bucket_map();
        buckets.insert(ColumnBucketKey::positive(0), BTreeSet::from([493, 744]));
        buckets.insert(ColumnBucketKey::positive(3), BTreeSet::from([13]));
        let plan = TableGenerationPlan::from_column_buckets(&buckets, [1]);
        assert!(
            plan.csv_asset_names
                .iter()
                .any(|name| name == "religion.csv")
        );
        assert!(plan.csv_asset_names.iter().any(|name| name == "kombi.csv"));
    }
}

// Stage 16 continued: concrete table_generation.py compatibility wrappers.
pub fn _set_last_line_number(rows: &[Vec<String>]) -> usize {
    rows.len().saturating_sub(1)
}
pub fn _concat_csv_inputs(columns: &[i64]) -> Vec<i64> {
    columns.iter().copied().filter(|value| *value > 0).collect()
}
pub fn _read_kombi_tables(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .map(|line| line.split(';').map(str::to_string).collect())
        .collect()
}
pub fn _apply_generated_column_morphisms(row_number: i64) -> Vec<String> {
    crate::generated_columns::bootstrap_generated_columns()
        .registry
        .names()
        .into_iter()
        .map(|name| format!("{name}:{row_number}"))
        .collect()
}
pub fn build_for_program(columns: &[i64]) -> TableGenerationPlan {
    let mut buckets: BTreeMap<ColumnBucketKey, BTreeSet<i64>> = BTreeMap::new();
    buckets.insert(
        ColumnBucketKey::positive(0),
        columns.iter().copied().collect(),
    );
    TableGenerationPlan::from_column_buckets(&buckets, std::iter::empty::<i64>())
}

// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "_apply_generated_column_morphisms",
    "_concat_csv_inputs",
    "_read_kombi_tables",
    "_set_last_line_number",
    "build_for_program",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
