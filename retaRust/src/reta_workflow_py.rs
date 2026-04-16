use crate::reta_begin_py::normalize_request;
use crate::reta_output_py::{derive_exit_code, render_diagnostics, render_output};
use crate::reta_program_types::{RetaError, RetaMetadata, RetaRequest, RetaResponse};
use crate::reta_resulting_table_py::build_resulting_table;
use crate::reta_spalten_py::resolve_column_plan;

pub fn run_reta(request: RetaRequest) -> Result<RetaResponse, RetaError> {
    let mut diagnostics = Vec::new();

    let normalized = normalize_request(&request)?;
    diagnostics.extend(normalized.diagnostics.clone());

    let (column_plan, column_diags) = resolve_column_plan(&normalized)?;
    diagnostics.extend(column_diags);

    let (table, table_diags) = build_resulting_table(&normalized, &column_plan)?;
    diagnostics.extend(table_diags);

    let (rendered_text, output_diags) = render_output(&table, &normalized)?;
    diagnostics.extend(output_diags);

    let stderr_text = render_diagnostics(&diagnostics);
    let exit_code = derive_exit_code(&diagnostics);

    Ok(RetaResponse {
        rendered_text,
        stderr_text,
        exit_code,
        metadata: RetaMetadata {
            effective_width: normalized.effective_width,
            selected_columns: column_plan.selected_columns,
            rows_emitted: table.rows.len(),
        },
        diagnostics,
    })
}
