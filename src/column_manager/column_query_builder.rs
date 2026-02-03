// file: column_manager/column_query_builder.rs
use crate::cli::TextBereich;
use super::column_selector::{collect_spalten_nummern, resolve_spaltennamen};
use super::row_query_builder::build_row_query;
use super::validation::validate_spalten_input;

pub fn build_column_query(
    column_names: &[String],
    bereich: &mut TextBereich,
) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    validate_spalten_input(bereich)?;

    let spalten_nummern = collect_spalten_nummern(bereich)?;
    let selected_names = resolve_spaltennamen(column_names, &spalten_nummern)?;
    let columns_clause_satz_string = selected_names.join(", ");

    let query = build_row_query(&columns_clause_satz_string, bereich)?;

    Ok((query, selected_names))
}


