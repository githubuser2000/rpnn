use crate::reta_program_types::{
    ColumnPlan, NormalizedRequest, ResultingTable, RetaDiagnostic, RetaError,
};

pub fn build_resulting_table(
    normalized: &NormalizedRequest,
    column_plan: &ColumnPlan,
) -> Result<(ResultingTable, Vec<RetaDiagnostic>), RetaError> {
    let mut diagnostics = Vec::new();
    let headers = column_plan.selected_columns.clone();
    let mut rows = Vec::new();
    let mut source_rows = 0usize;

    for (index, line) in normalized.stdin_text.as_deref().unwrap_or("").lines().enumerate() {
        let line_number = index + 1;
        source_rows += 1;

        if !normalized.row_selection.contains(line_number) {
            continue;
        }

        rows.push(build_row(line, &headers));
    }

    if let Some(selected_count) = normalized.row_selection.selected_count() {
        if selected_count > 0 && rows.is_empty() {
            diagnostics.push(RetaDiagnostic::warning(
                "ROW_SELECTION_EMPTY_RESULT",
                "die Zeilenauswahl wurde ausgewertet, aber keine Zeilen haben die Auswahl getroffen.",
            ));
        }
    }

    diagnostics.push(RetaDiagnostic::info(
        "RESULTING_TABLE_BUILT",
        format!(
            "ResultingTable wurde in der Library aufgebaut: {}/{} Zeilen emittiert.",
            rows.len(),
            source_rows
        ),
    ));

    Ok((ResultingTable { headers, rows }, diagnostics))
}

fn build_row(line: &str, headers: &[String]) -> Vec<String> {
    if headers.len() == 1 && headers.first().map(String::as_str) == Some("line") {
        return vec![line.to_string()];
    }

    let cells = split_input_line(line);
    (0..headers.len())
        .map(|index| cells.get(index).cloned().unwrap_or_default())
        .collect()
}

fn split_input_line(line: &str) -> Vec<String> {
    let tab_cells = line.split('\t').map(ToOwned::to_owned).collect::<Vec<_>>();
    if tab_cells.len() > 1 {
        return tab_cells;
    }

    let whitespace_cells = line
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    if whitespace_cells.is_empty() {
        vec![String::new()]
    } else {
        whitespace_cells
    }
}
