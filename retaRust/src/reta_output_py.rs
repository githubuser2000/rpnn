use crate::reta_program_types::{
    DiagnosticLevel, NormalizedRequest, ResultingTable, RetaDiagnostic, RetaError,
};

pub fn render_output(
    table: &ResultingTable,
    normalized: &NormalizedRequest,
) -> Result<(String, Vec<RetaDiagnostic>), RetaError> {
    let mut diagnostics = Vec::new();
    let mut lines = if normalized.onetable {
        diagnostics.push(RetaDiagnostic::info(
            "ONETABLE_RENDER",
            "onetable-Rendering wurde in der Library ausgeführt.",
        ));
        render_as_single_table_lines(table)
    } else {
        render_as_plain_lines(table)
    };

    if let Some(width) = normalized.effective_width {
        let mut truncated_lines = 0usize;
        for line in &mut lines {
            let shortened = truncate_to_width(line, width);
            if shortened.len() != line.len() {
                truncated_lines += 1;
                *line = shortened;
            }
        }

        if truncated_lines > 0 {
            diagnostics.push(RetaDiagnostic::info(
                "WIDTH_APPLIED",
                format!(
                    "{} Ausgabezeilen wurden auf die effektive Breite {} begrenzt.",
                    truncated_lines, width
                ),
            ));
        }
    }

    Ok((lines.join("\n"), diagnostics))
}

pub fn render_diagnostics(diagnostics: &[RetaDiagnostic]) -> String {
    let mut stderr = String::new();

    for diagnostic in diagnostics {
        let level = match diagnostic.level {
            DiagnosticLevel::Info => "INFO",
            DiagnosticLevel::Warning => "WARN",
            DiagnosticLevel::Error => "ERROR",
        };
        stderr.push_str(&format!(
            "[{level}] {}: {}\n",
            diagnostic.code, diagnostic.message
        ));
    }

    stderr
}

pub fn derive_exit_code(diagnostics: &[RetaDiagnostic]) -> i32 {
    if diagnostics
        .iter()
        .any(|diagnostic| matches!(diagnostic.level, DiagnosticLevel::Error))
    {
        1
    } else {
        0
    }
}

fn render_as_single_table_lines(table: &ResultingTable) -> Vec<String> {
    let mut lines = Vec::new();

    if !table.headers.is_empty() {
        lines.push(table.headers.join("\t"));
    }

    for row in &table.rows {
        lines.push(row.join("\t"));
    }

    lines
}

fn render_as_plain_lines(table: &ResultingTable) -> Vec<String> {
    table.rows.iter().map(|row| row.join("\t")).collect()
}

fn truncate_to_width(line: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    let mut result = String::new();
    for ch in line.chars().take(width) {
        result.push(ch);
    }
    result
}
