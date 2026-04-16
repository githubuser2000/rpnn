use std::collections::BTreeSet;

use crate::cli::TextBereich;
use crate::generated_columns_eigenschaften::generate_eigenschaften_columns;
use crate::generated_columns_missing_port::{
    concat1_row_prim_universe2,
    spalte_fuer_gegen_innen_aussen_seitlich_prim,
    spalte_meta_kontret_theorie_abstrakt_etc_1,
};
use crate::generated_columns_words_registry::{ParametersMain, Tables, Table, RowSet};

pub fn apply_missing_generators_and_eigenschaften(
    table: &mut Table,
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
    _bereich: &TextBereich,
    generated_befehle: &BTreeSet<String>,
    _parameters_main: &ParametersMain,
) {
    let tokens = generated_befehle
        .iter()
        .map(|s| s.trim().to_lowercase())
        .collect::<BTreeSet<_>>();

    let wants_prim_universe = tokens.contains("primuniversum")
        || tokens.contains("primmotivstern")
        || tokens.contains("primstrukstern")
        || tokens.contains("primmotivgleichf")
        || tokens.contains("primstrukgleichf");

    let wants_meta = tokens.contains("meta")
        || tokens.contains("konkret")
        || tokens.contains("theorie")
        || tokens.contains("praxis")
        || tokens.contains("management")
        || tokens.contains("wertvoll")
        || tokens.contains("richtung");

    let wants_innen_aussen = tokens.contains("innen")
        || tokens.contains("aussen")
        || tokens.contains("außen")
        || tokens.contains("seitlich")
        || tokens.contains("gegen");

    let wants_eigenschaften = tokens.contains("eigenschaften")
        || tokens.contains("eigenschaft")
        || tokens.contains("konzept")
        || tokens.contains("konzepte")
        || tokens.contains("konzept2")
        || tokens.contains("konzepte2");

    if wants_prim_universe {
        concat1_row_prim_universe2(table, rows_as_numbers, tables);
    }
    if wants_meta {
        spalte_meta_kontret_theorie_abstrakt_etc_1(table, rows_as_numbers, tables);
    }
    if wants_innen_aussen {
        spalte_fuer_gegen_innen_aussen_seitlich_prim(table, rows_as_numbers, tables);
    }
    if wants_eigenschaften {
        // Diese Funktion lebt bewusst separat, damit die Eigenschaften-Familie
        // unabhängig vom restlichen Generator-Port weiterentwickelt werden kann.
        let mut shadow_tables = crate::generated_columns_eigenschaften::Tables::default();
        shadow_tables.spalten_vanilla_amount = tables.spalten_vanilla_amount;
        shadow_tables.last_line_number = tables.last_line_number;
        shadow_tables.data_dict = tables.data_dict.clone();
        let mut shadow_rows = rows_as_numbers.clone();
        generate_eigenschaften_columns(table, &mut shadow_rows, &mut shadow_tables, generated_befehle);
    }
}
