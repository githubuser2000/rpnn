use std::collections::BTreeSet;

use crate::reta_program_types::{ColumnPlan, NormalizedRequest, RetaDiagnostic, RetaError};

pub fn resolve_column_plan(
    normalized: &NormalizedRequest,
) -> Result<(ColumnPlan, Vec<RetaDiagnostic>), RetaError> {
    let mut diagnostics = Vec::new();
    let mut selected_columns = Vec::new();
    let mut seen = BTreeSet::new();

    for raw_column in normalized.raw_column_order.clone().unwrap_or_default() {
        let column = raw_column.trim();
        if column.is_empty() {
            continue;
        }

        if !seen.insert(column.to_string()) {
            diagnostics.push(RetaDiagnostic::warning(
                "DUPLICATE_COLUMN_IGNORED",
                format!("doppelte Spalte wurde ignoriert: {column}"),
            ));
            continue;
        }

        selected_columns.push(column.to_string());
    }

    if selected_columns.is_empty() {
        selected_columns.push("line".to_string());
        diagnostics.push(RetaDiagnostic::info(
            "DEFAULT_COLUMN_SELECTION",
            "keine explizite Spaltenauswahl geliefert; Standardspalte 'line' wird verwendet.",
        ));
    } else {
        diagnostics.push(RetaDiagnostic::info(
            "COLUMN_SELECTION_ACTIVE",
            format!(
                "explizite Spaltenauswahl wurde auf den ColumnPlan abgebildet: {}",
                selected_columns.join(", ")
            ),
        ));
    }

    Ok((ColumnPlan { selected_columns }, diagnostics))
}
