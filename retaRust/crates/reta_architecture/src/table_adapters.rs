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



pub fn legacy_table_adapter_surface_names() -> Vec<&'static str> {
    vec![
        "FilterOriginalLines",
        "prepare4out_beforeForLoop_SpaltenZeilenBestimmen",
        "prepare4out_LoopBody",
        "prepare4out_Tagging",
        "cellWork",
        "concatVervielfacheZeile",
        "concatModallogik",
        "concat1PrimzahlkreuzProContra",
        "concat1RowPrimUniverse2",
        "convertSetOfPaarenToDictOfNumToPaareDiv",
        "convertSetOfPaarenToDictOfNumToPaareMul",
        "convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction",
        "combineDicts",
        "readConcatCsv_tabelleDazuColchange",
        "readConcatCsv_ChangeTableToAddToTable",
        "readConcatCsv_LoopBody",
        "readConcatCSV_choseCsvFile",
        "readConcatCsv_SetHtmlParamaters",
        "spalteMetaKontretTheorieAbstrakt_etc_1",
        "spalteMetaKonkretAbstrakt_isGanzZahlig",
        "spalteMetaKonkretTheorieAbstrakt_SetHtmlParameters",
        "spalteMetaKonkretTheorieAbstrakt_mainPart",
        "spalteMetaKonkretTheorieAbstrakt_VorwortBehandlungWieVorwortMeta",
        "spalteMetaKonkretTheorieAbstrakt_mainPart_InsertingText",
        "spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie",
        "spalteMetaKonkretAbstrakt_UeberschriftenUndTags",
        "spalteFuerGegenInnenAussenSeitlichPrim",
        "readOneCSVAndReturn",
        "getAllBrueche",
        "findAllBruecheAndTheirCombinations",
        "breitenn",
        "nummeriere",
        "textWidth",
        "zeileWhichZaehlung",
        "moonsun",
    ]
}

pub fn __init__() -> TableAdaptersBundle {
    bootstrap_table_adapters()
}

pub fn set_zaehlungen(adapter: &mut PrepareAdapter, values: Vec<BTreeMap<String, i64>>) {
    adapter.zaehlungen = values;
}

pub fn breitenn(adapter: &PrepareAdapter) -> Vec<usize> {
    adapter.breiten.clone()
}

pub fn nummeriere(adapter: &PrepareAdapter) -> bool {
    adapter.nummerierung
}

pub fn text_width(adapter: &PrepareAdapter) -> usize {
    adapter.textwidth
}

pub fn delete_doubles_in_sets(values: &[i64]) -> BTreeSet<i64> {
    values.iter().copied().collect()
}

pub fn from_until(from: i64, until: i64) -> BTreeSet<i64> {
    if from <= until {
        (from..=until).collect()
    } else {
        (until..=from).collect()
    }
}

pub fn zeile_which_zaehlung(row_number: i64) -> usize {
    row_number.rem_euclid(5) as usize
}

pub fn moonsun(row_number: i64) -> (Vec<i64>, Vec<i64>) {
    crate::number_theory::moon_number(row_number)
}

pub fn filter_original_lines(rows: &[i64], allowed: &BTreeSet<i64>) -> Vec<i64> {
    rows.iter().copied().filter(|row| allowed.contains(row)).collect()
}

pub fn prepare4out_before_for_loop_spalten_zeilen_bestimmen(rows: &[i64]) -> BTreeSet<i64> {
    rows.iter().copied().collect()
}

pub fn prepare4out_loop_body(row: &[String]) -> Vec<String> {
    row.to_vec()
}

pub fn prepare4out_tagging(column_number: i64) -> Vec<String> {
    crate::tag_schema::ordinary_tags_for_column(column_number).unwrap_or_default().into_iter().map(|tag| format!("{:?}", tag)).collect()
}

pub fn cell_work(cell: &str, width: usize) -> Vec<String> {
    crate::table_wrapping::wrap_cell_text(cell, width, None).unwrap_or_else(|| vec![cell.to_string()])
}

pub fn concat_vervielfache_zeile(row_number: i64, source_cell: &str) -> String {
    crate::generated_columns::concat_vervielfache_zeile(row_number, source_cell)
}

pub fn concat_modallogik(row_number: i64) -> String {
    crate::generated_columns::concat_modallogik(row_number)
}

pub fn concat_gleichheit_freiheit_dominieren(row_number: i64) -> String {
    crate::generated_columns::concat_gleichheit_freiheit_dominieren(row_number)
}

pub fn concat_geist_emotion_energie_materie_topologie(row_number: i64) -> String {
    crate::generated_columns::concat_geist_emotion_energie_materie_topologie(row_number)
}

pub fn concat_mond_exponzieren_logarithmus_typ(row_number: i64) -> String {
    crate::generated_columns::concat_mond_exponzieren_logarithmus_typ(row_number)
}

pub fn concat1_primzahlkreuz_pro_contra(row_number: i64) -> String {
    crate::generated_columns::concat_primzahlkreuz_pro_contra(row_number)
}

pub fn concat1_row_prim_universe2(row_number: i64) -> String {
    crate::generated_columns::concat_prim_universe_row(row_number)
}

pub fn convert_set_of_paaren_to_dict_of_num_to_paare_div(pairs: &[(i64, i64)]) -> crate::concat_csv::FractionPairMap {
    let pairs = pairs
        .iter()
        .map(|(left, right)| crate::concat_csv::FractionPair::new(crate::meta_columns::Rational::new(*left, 1), crate::meta_columns::Rational::new(*right, 1)))
        .collect::<BTreeSet<_>>();
    crate::concat_csv::convert_set_of_pairs_to_dict_of_num_to_pairs_div(&pairs, false)
}

pub fn convert_set_of_paaren_to_dict_of_num_to_paare_mul(pairs: &[(i64, i64)]) -> crate::concat_csv::FractionPairMap {
    let pairs = pairs
        .iter()
        .map(|(left, right)| crate::concat_csv::FractionPair::new(crate::meta_columns::Rational::new(*left, 1), crate::meta_columns::Rational::new(*right, 1)))
        .collect::<BTreeSet<_>>();
    crate::concat_csv::convert_set_of_pairs_to_dict_of_num_to_pairs_mul(&pairs, false)
}

pub fn convert_fractions_to_dict_of_num_to_paare_of_mul_of_int_and_fraction(fractions: &[crate::meta_columns::Rational], max_row: i64) -> crate::concat_csv::FractionPairMap {
    let fracs = fractions.iter().copied().collect::<BTreeSet<_>>();
    crate::concat_csv::convert_fractions_to_dict_of_num_to_pairs_of_mul_of_int_and_fraction(&fracs, &fracs, max_row, false)
}

pub fn combine_dicts(left: &crate::concat_csv::FractionPairMap, right: &crate::concat_csv::FractionPairMap) -> crate::concat_csv::FractionPairMap {
    crate::concat_csv::combine_dicts(left, right)
}

pub fn spalte_meta_konkret_abstrakt_is_ganz_zahlig(value: crate::meta_columns::Rational, inverse: bool) -> bool {
    crate::meta_columns::spalte_meta_konkret_abstrakt_is_ganz_zahlig(value, inverse)
}

pub fn spalte_meta_kontret_theorie_abstrakt_etc_1(value: i64) -> String {
    crate::meta_columns::spalte_meta_kontret_theorie_abstrakt_etc_1(value)
}

pub fn spalte_meta_konkret_theorie_abstrakt_set_html_parameters(enabled: bool) -> Vec<(String, String)> {
    crate::meta_columns::spalte_meta_konkret_theorie_abstrakt_set_html_parameters(enabled)
}

pub fn spalte_meta_konkret_theorie_abstrakt_main_part(value: i64) -> String {
    crate::meta_columns::spalte_meta_konkret_theorie_abstrakt_main_part(value)
}

pub fn spalte_meta_konkret_theorie_abstrakt_vorwort_behandlung_wie_vorwort_meta(prefix: &str, repetitions: usize) -> String {
    crate::meta_columns::spalte_meta_konkret_theorie_abstrakt_vorwort_behandlung_wie_vorwort_meta(prefix, repetitions)
}

pub fn spalte_meta_konkret_theorie_abstrakt_main_part_inserting_text(value: i64, text: &str) -> String {
    crate::meta_columns::spalte_meta_konkret_theorie_abstrakt_main_part_inserting_text(value, text)
}

pub fn spalte_meta_konkret_theorie_abstrakt_get_gebr_rat_univ_strukturalie(value: i64) -> String {
    crate::meta_columns::spalte_meta_konkret_theorie_abstrakt_get_gebr_rat_univ_strukturalie(value)
}

pub fn spalte_meta_konkret_abstrakt_ueberschriften_und_tags() -> Vec<String> {
    crate::meta_columns::spalte_meta_konkret_abstrakt_ueberschriften_und_tags()
}

pub fn spalte_fuer_gegen_innen_aussen_seitlich_prim(value: i64) -> crate::meta_columns::PrimeCrossColumnClass {
    crate::meta_columns::spalte_fuer_gegen_innen_aussen_seitlich_prim(value)
}

pub fn get_all_brueche(max_denominator: i64) -> Vec<crate::meta_columns::Rational> {
    crate::meta_columns::get_all_brueche(max_denominator)
}

pub fn read_one_csv_and_return(text: &str) -> Vec<Vec<String>> {
    crate::meta_columns::read_one_csv_and_return(text)
}

pub fn find_all_brueche_and_their_combinations(max_denominator: i64) -> Vec<crate::meta_columns::Rational> {
    crate::meta_columns::find_all_brueche_and_their_combinations(max_denominator)
}

pub fn read_concat_csv_tabelle_dazu_colchange(cells: &[String], row_number: i64) -> Vec<String> {
    crate::concat_csv::read_concat_csv_tabelle_dazu_colchange(row_number, cells, false)
}

pub fn read_concat_csv_chose_csv_file(name: &str) -> String {
    name.to_string()
}

pub fn read_concat_csv_change_table_to_add_to_table(table: &[Vec<String>]) -> Vec<Vec<String>> {
    table.to_vec()
}

pub fn read_concat_csv_loop_body(row: &[String]) -> Vec<String> {
    row.to_vec()
}

pub fn read_concat_csv_set_html_paramaters(enabled: bool) -> Vec<(String, String)> {
    vec![("html".to_string(), enabled.to_string())]
}

pub fn read_one_csv_and_return_adapter(text: &str) -> Vec<Vec<String>> {
    read_one_csv_and_return(text)
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

// Stage 16 continued: Python camel-case adapter wrappers.
pub type Prepare = PrepareAdapter;
pub type Concat = ConcatAdapter;
#[allow(non_snake_case)]
pub fn prepare4out_LoopBody(row: &[String]) -> Vec<String> { prepare4out_loop_body(row) }
#[allow(non_snake_case)]
pub fn prepare4out_Tagging(column_number: i64) -> Vec<String> { prepare4out_tagging(column_number) }
#[allow(non_snake_case)]
pub fn prepare4out_beforeForLoop_SpaltenZeilenBestimmen(rows: &[i64]) -> BTreeSet<i64> { prepare4out_before_for_loop_spalten_zeilen_bestimmen(rows) }
#[allow(non_snake_case)]
pub fn readConcatCSV_choseCsvFile(name: &str) -> String { read_concat_csv_chose_csv_file(name) }
#[allow(non_snake_case)]
pub fn readConcatCsv(cells: &[String], row_number: i64) -> Vec<String> { read_concat_csv_tabelle_dazu_colchange(cells, row_number) }
#[allow(non_snake_case)]
pub fn readConcatCsv_ChangeTableToAddToTable(table: &[Vec<String>]) -> Vec<Vec<String>> { read_concat_csv_change_table_to_add_to_table(table) }
#[allow(non_snake_case)]
pub fn readConcatCsv_LoopBody(row: &[String]) -> Vec<String> { read_concat_csv_loop_body(row) }
#[allow(non_snake_case)]
pub fn readConcatCsv_SetHtmlParamaters(enabled: bool) -> Vec<(String, String)> { read_concat_csv_set_html_paramaters(enabled) }
#[allow(non_snake_case)]
pub fn readOneCSVAndReturn(text: &str) -> Vec<Vec<String>> { read_one_csv_and_return(text) }
#[allow(non_snake_case)]
pub fn spalteMetaKonkretAbstrakt_UeberschriftenUndTags() -> Vec<String> { spalte_meta_konkret_abstrakt_ueberschriften_und_tags() }
#[allow(non_snake_case)]
pub fn spalteMetaKonkretTheorieAbstrakt_SetHtmlParameters(enabled: bool) -> Vec<(String, String)> { spalte_meta_konkret_theorie_abstrakt_set_html_parameters(enabled) }
#[allow(non_snake_case)]
pub fn spalteMetaKonkretTheorieAbstrakt_VorwortBehandlungWieVorwortMeta(prefix: &str, repetitions: usize) -> String { spalte_meta_konkret_theorie_abstrakt_vorwort_behandlung_wie_vorwort_meta(prefix, repetitions) }
#[allow(non_snake_case)]
pub fn spalteMetaKonkretTheorieAbstrakt_mainPart_InsertingText(value: i64, text: &str) -> String { spalte_meta_konkret_theorie_abstrakt_main_part_inserting_text(value, text) }
#[allow(non_snake_case)]
pub fn spalteMetaKontretTheorieAbstrakt_etc(value: i64) -> String { spalte_meta_kontret_theorie_abstrakt_etc_1(value) }

// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "spalteMetaKontretTheorieAbstrakt_etc",
    "Concat",
    "Prepare",
    "prepare4out_LoopBody",
    "prepare4out_Tagging",
    "prepare4out_beforeForLoop_SpaltenZeilenBestimmen",
    "readConcatCSV_choseCsvFile",
    "readConcatCsv",
    "readConcatCsv_ChangeTableToAddToTable",
    "readConcatCsv_LoopBody",
    "readConcatCsv_SetHtmlParamaters",
    "readOneCSVAndReturn",
    "spalteMetaKonkretAbstrakt_UeberschriftenUndTags",
    "spalteMetaKonkretTheorieAbstrakt_SetHtmlParameters",
    "spalteMetaKonkretTheorieAbstrakt_VorwortBehandlungWieVorwortMeta",
    "spalteMetaKonkretTheorieAbstrakt_mainPart_InsertingText",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
