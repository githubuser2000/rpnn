use std::collections::BTreeSet;
use std::path::Path;

use crate::reta_program_types::{
    NormalizedRequest, RetaDiagnostic, RetaError, RetaInput, RetaOptions, RetaRequest,
    RetaRuntime, RowSelection,
};

pub fn build_cli_request(
    raw_args: &[String],
    stdin_text: Option<String>,
    runtime: RetaRuntime,
) -> RetaRequest {
    RetaRequest {
        raw_args: raw_args.to_vec(),
        options: parse_cli_options(raw_args),
        input: RetaInput { stdin_text },
        runtime,
    }
}

pub fn parse_cli_options(raw_args: &[String]) -> RetaOptions {
    let mut options = RetaOptions::default();

    for arg in normalize_invocation_args(raw_args) {
        if arg == "--onetable" {
            options.onetable = true;
            continue;
        }

        if let Some(value) = arg.strip_prefix("--breite=") {
            options.breite = value.parse::<usize>().ok();
            continue;
        }

        if let Some(value) = arg.strip_prefix("--spaltenreihenfolgeundnurdiese=") {
            let columns = value
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>();
            options.spaltenreihenfolgeundnurdiese = Some(columns);
            continue;
        }

        if let Some(value) = arg.strip_prefix("--vorhervonausschnitt=") {
            options.vorhervonausschnitt = Some(value.to_string());
            continue;
        }

        options.passthrough_flags.push(arg.clone());
    }

    options
}

fn normalize_invocation_args(raw_args: &[String]) -> &[String] {
    match raw_args.split_first() {
        Some((first, rest)) if looks_like_reta_binary_name(first) => rest,
        _ => raw_args,
    }
}

fn looks_like_reta_binary_name(raw: &str) -> bool {
    if raw.starts_with('-') || raw.is_empty() {
        return false;
    }

    let candidate = Path::new(raw)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(raw);

    candidate == "reta"
}

pub fn normalize_request(request: &RetaRequest) -> Result<NormalizedRequest, RetaError> {
    let mut diagnostics = Vec::new();

    let effective_width = match request.options.breite {
        Some(0) => {
            diagnostics.push(RetaDiagnostic::info(
                "WIDTH_FROM_CALLER",
                "breite=0 wurde auf die vom Aufrufer gelieferte Terminalbreite abgebildet.",
            ));
            request.runtime.terminal_width
        }
        Some(width) => Some(width),
        None => request.runtime.terminal_width,
    };

    if request.options.breite == Some(0) && effective_width.is_none() {
        diagnostics.push(RetaDiagnostic::warning(
            "NO_TERMINAL_WIDTH",
            "breite=0 wurde angefordert, aber der Aufrufer hat keine Terminalbreite geliefert.",
        ));
    }

    if let Some(width) = effective_width {
        if width == 0 {
            return Err(RetaError::InvalidOptions(
                "effective_width darf nicht 0 sein".to_string(),
            ));
        }
    }

    let row_selection = parse_row_selection(
        request
            .options
            .vorhervonausschnitt
            .as_deref()
            .map(str::trim)
            .filter(|expr| !expr.is_empty()),
    )?;

    if let Some(selected_count) = row_selection.selected_count() {
        diagnostics.push(RetaDiagnostic::info(
            "ROW_SELECTION_ACTIVE",
            format!(
                "vorhervonausschnitt wurde auf {} Zeilennummern normalisiert.",
                selected_count
            ),
        ));
    }

    if !request.options.passthrough_flags.is_empty() {
        diagnostics.push(RetaDiagnostic::info(
            "PASSTHROUGH_FLAGS_PRESENT",
            format!(
                "{} unbekannte oder noch nicht transplantierte Flags wurden als passthrough markiert.",
                request.options.passthrough_flags.len()
            ),
        ));
    }

    Ok(NormalizedRequest {
        effective_width,
        onetable: request.options.onetable,
        raw_selection_expr: request.options.vorhervonausschnitt.clone(),
        raw_column_order: request.options.spaltenreihenfolgeundnurdiese.clone(),
        stdin_text: request.input.stdin_text.clone(),
        row_selection,
        diagnostics,
    })
}

fn parse_row_selection(expr: Option<&str>) -> Result<RowSelection, RetaError> {
    let Some(expr) = expr else {
        return Ok(RowSelection::All);
    };

    let mut lines = BTreeSet::new();

    for token in expr.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        if let Some((start, end)) = token.split_once('-') {
            let start = parse_positive_line_number(start)?;
            let end = parse_positive_line_number(end)?;
            if end < start {
                return Err(RetaError::InvalidOptions(format!(
                    "ungültiger Bereich in vorhervonausschnitt: {token}"
                )));
            }
            for value in start..=end {
                lines.insert(value);
            }
            continue;
        }

        lines.insert(parse_positive_line_number(token)?);
    }

    if lines.is_empty() {
        Ok(RowSelection::All)
    } else {
        Ok(RowSelection::Selected(lines))
    }
}

fn parse_positive_line_number(raw: &str) -> Result<usize, RetaError> {
    let value = raw.trim().parse::<usize>().map_err(|_| {
        RetaError::InvalidOptions(format!(
            "ungültige Zeilennummer in vorhervonausschnitt: {}",
            raw.trim()
        ))
    })?;

    if value == 0 {
        return Err(RetaError::InvalidOptions(
            "Zeilennummern in vorhervonausschnitt beginnen bei 1".to_string(),
        ));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::parse_cli_options;

    #[test]
    fn parse_cli_options_accepts_bare_args() {
        let argv = vec!["--onetable".to_string(), "--breite=80".to_string()];
        let parsed = parse_cli_options(&argv);
        assert!(parsed.onetable);
        assert_eq!(parsed.breite, Some(80));
    }

    #[test]
    fn parse_cli_options_ignores_plain_reta_program_name() {
        let argv = vec![
            "reta".to_string(),
            "--onetable".to_string(),
            "--vorhervonausschnitt=1-3".to_string(),
        ];
        let parsed = parse_cli_options(&argv);
        assert!(parsed.onetable);
        assert_eq!(parsed.vorhervonausschnitt.as_deref(), Some("1-3"));
        assert!(parsed.passthrough_flags.is_empty());
    }

    #[test]
    fn parse_cli_options_ignores_path_like_reta_program_name() {
        let argv = vec![
            "/tmp/target/debug/reta".to_string(),
            "--spaltenreihenfolgeundnurdiese=a,b".to_string(),
        ];
        let parsed = parse_cli_options(&argv);
        assert_eq!(
            parsed.spaltenreihenfolgeundnurdiese,
            Some(vec!["a".to_string(), "b".to_string()])
        );
        assert!(parsed.passthrough_flags.is_empty());
    }
}
