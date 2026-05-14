use crate::reta_program_types::{
    DiagnosticLevel, RetaDiagnostic, RetaError, RetaMetadata, RetaRequest, RetaResponse,
};
use crate::reta_runtime_bridge::with_runtime_override;
use crate::{fresh_program_from_template, preload_reta_runtime, shared_words};

pub fn run_reta(request: RetaRequest) -> Result<RetaResponse, RetaError> {
    let argv = normalize_program_argv(&request.raw_args);
    let _architecture_run = reta_architecture::RetaRunArchitecture::from_cli_args(&argv);
    let (arch_clean_argv, _) = reta_architecture::extract_architecture_switch_from_argv(&argv, None);
    let (legacy_argv, _) = reta_architecture::extract_parallel_config_from_argv(&arch_clean_argv, None);

    preload_reta_runtime().map_err(RetaError::Execution)?;

    let runtime = request.runtime.clone();
    let program = with_runtime_override(Some(runtime.clone()), || {
        let mut program = fresh_program_from_template(legacy_argv);
        let words = shared_words();
        program.runAllesLikePythonInit(words);
        program.run(words);
        program.combiTableWorkflow();
        program
    });

    let mut diagnostics = Vec::new();
    if runtime.terminal_width.is_some() {
        diagnostics.push(RetaDiagnostic {
            level: DiagnosticLevel::Info,
            code: "RUNTIME_TERMINAL_WIDTH".to_string(),
            message: "Terminalbreite wurde vom Aufrufer in die Library übernommen.".to_string(),
        });
    }

    if !request.input.stdin_text.as_deref().unwrap_or_default().is_empty() {
        diagnostics.push(RetaDiagnostic {
            level: DiagnosticLevel::Info,
            code: "STDIN_BUFFER_PRESENT".to_string(),
            message: "stdin wurde vom Binary entgegengenommen und an die Library weitergereicht.".to_string(),
        });
    }

    diagnostics.extend(program.cliErrors.iter().cloned().map(|message| RetaDiagnostic {
        level: DiagnosticLevel::Error,
        code: "CLI_ERROR".to_string(),
        message,
    }));

    let rendered_text = if !program.finallyDisplayLines.is_empty() {
        let mut text = program.finallyDisplayLines.join("\n");
        if !text.is_empty() {
            text.push('\n');
        }
        text
    } else {
        String::new()
    };

    let stderr_text = if program.cliErrors.is_empty() {
        String::new()
    } else {
        let mut text = program.cliErrors.join("\n");
        if !text.is_empty() {
            text.push('\n');
        }
        text
    };

    let exit_code = if program.cliErrors.is_empty() { 0 } else { 1 };

    let metadata = RetaMetadata {
        effective_width: runtime.terminal_width.or_else(|| {
            if program.shellWidth > 0 {
                Some(program.shellWidth as usize)
            } else if program.textWidth > 0 {
                Some(program.textWidth as usize)
            } else {
                None
            }
        }),
        selected_columns: program
            .spaltenreihenfolgeundnurdiese
            .iter()
            .map(ToString::to_string)
            .collect(),
        rows_emitted: program.__resultingTable.len().max(program.finallyDisplayLines.len()),
    };

    Ok(RetaResponse {
        rendered_text,
        stderr_text,
        exit_code,
        diagnostics,
        metadata,
    })
}

fn normalize_program_argv(raw_args: &[String]) -> Vec<String> {
    if raw_args.is_empty() {
        return vec!["reta".to_string()];
    }

    let first = &raw_args[0];
    let first_basename = std::path::Path::new(first)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or(first.as_str());

    let looks_like_program_name = matches!(first_basename, "reta" | "reta.exe");
    if looks_like_program_name {
        raw_args.to_vec()
    } else {
        let mut argv = Vec::with_capacity(raw_args.len() + 1);
        argv.push("reta".to_string());
        argv.extend(raw_args.iter().cloned());
        argv
    }
}
