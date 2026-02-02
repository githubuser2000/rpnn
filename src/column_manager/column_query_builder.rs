// file: column_manager/column_query_builder.rs
use crate::cli::TextBereich;
use super::column_selector::{collect_spalten_nummern, resolve_spaltennamen};
use super::row_query_builder::build_row_query;
use super::validation::validate_spalten_input;

pub fn build_column_query(
    column_names: &[String],
    bereich: &mut TextBereich,
    wurde_spalten_gesucht: bool,
) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    validate_spalten_input(bereich, wurde_spalten_gesucht)?;

    let spalten_nummern = collect_spalten_nummern(bereich, wurde_spalten_gesucht)?;
    let selected_names = resolve_spaltennamen(column_names, &spalten_nummern)?;
    let columns_clause = selected_names.join(", ");

    let query = build_row_query(&columns_clause, bereich)?;

    Ok((query, selected_names))
}


