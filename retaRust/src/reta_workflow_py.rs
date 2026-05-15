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

    let mut committed_shadow_lines: Option<Vec<String>> = None;
    if let Some(shadow_runtime) = crate::reta_arch_shadow::shadow_table_runtime_report_for_program(&program, &argv) {
        let shadow_report = shadow_runtime.report;
        let commit = shadow_runtime.commit;
        let view_output_report = shadow_runtime.view_output_report;
        let view_output_commit = shadow_runtime.view_output_commit;
        let view_output_audit = shadow_runtime.view_output_audit;
        let view_output_transaction = shadow_runtime.view_output_transaction;
        diagnostics.push(RetaDiagnostic {
            level: if shadow_report.diff.equal { DiagnosticLevel::Info } else { DiagnosticLevel::Warning },
            code: "ARCH_SHADOW_TABLE".to_string(),
            message: format!(
                "Rust-Architektur-Shadow-Renderer: mode={} gate={} legacy_rows={} shadow_rows={} equal={} first_diff={:?}",
                shadow_report.switch_mode,
                shadow_report.gate.reason,
                shadow_report.legacy_rows,
                shadow_report.rendered_rows,
                shadow_report.diff.equal,
                shadow_report.diff.first_mismatch_index,
            ),
        });
        diagnostics.push(RetaDiagnostic {
            level: if commit.use_shadow_output || !commit.gate_allowed_to_commit {
                DiagnosticLevel::Info
            } else {
                DiagnosticLevel::Warning
            },
            code: "ARCH_SHADOW_COMMIT".to_string(),
            message: format!(
                "Rust-Architektur-Commit-Gate: mode={} use_shadow={} reason={} gate={} diff_equal={} lines={} rollback={:?}",
                commit.switch_mode,
                commit.use_shadow_output,
                commit.reason,
                commit.gate_reason,
                commit.diff_equal,
                commit.rendered_line_count,
                commit.rollback_anchor,
            ),
        });
        if let Some(report) = view_output_report.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if report.diff.equal { DiagnosticLevel::Info } else { DiagnosticLevel::Warning },
                code: "ARCH_TABLE_VIEW_OUTPUT".to_string(),
                message: format!(
                    "Rust-Architektur-Materialized-View-Output: mode={} gate={} output_mode={} legacy_rows={} rendered_rows={} raw_equal={} semantic_equal={} first_diff={:?} semantic_first_diff={:?}",
                    report.switch_mode,
                    report.gate.reason,
                    report.output_mode,
                    report.legacy_rows,
                    report.rendered_rows,
                    report.diff.equal,
                    report.semantic_diff.semantic_equal,
                    report.diff.first_mismatch_index,
                    report.semantic_diff.first_semantic_mismatch_index,
                ),
            });
        }
        if let Some(commit) = view_output_commit.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if commit.use_view_output || !commit.gate_allowed_to_commit {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_OUTPUT_COMMIT".to_string(),
                message: format!(
                    "Rust-Architektur-Materialized-View-Commit-Gate: mode={} use_view_output={} reason={} gate={} diff_equal={} semantic_equal={} lines={} rollback={:?}",
                    commit.switch_mode,
                    commit.use_view_output,
                    commit.reason,
                    commit.gate_reason,
                    commit.diff_equal,
                    commit.semantic_equal,
                    commit.rendered_line_count,
                    commit.rollback_anchor,
                ),
            });
        }
        if let Some(audit) = view_output_audit.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if audit.safe_to_commit || !audit.use_view_output {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_COMMIT_AUDIT".to_string(),
                message: format!(
                    "Rust-Architektur-Commit-Audit: mode={} safe={} use_view_output={} required={}/{} failed={:?} raw_equal={} semantic_equal={} virtual_direct={} virtual_added={} first_raw_diff={:?} first_semantic_diff={:?}",
                    audit.switch_mode,
                    audit.safe_to_commit,
                    audit.use_view_output,
                    audit.passed_required_check_count,
                    audit.required_check_count,
                    audit.failed_required_checks,
                    audit.raw_equal,
                    audit.semantic_equal,
                    audit.virtual_direct_cells_equal,
                    audit.virtual_added_column_count,
                    audit.first_raw_mismatch_index,
                    audit.first_semantic_mismatch_index,
                ),
            });
        }
        if let Some(transaction) = view_output_transaction.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if transaction.should_replace_visible_output || !transaction.commit_decision_allows_view_output {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_ACTIVATION_TRANSACTION".to_string(),
                message: format!(
                    "Rust-Architektur-Aktivierungs-Transaktion: mode={} source={} replace={} safe={} reason={} selected_lines={} legacy_lines={} view_lines={} checksum={} rollback={:?}",
                    transaction.switch_mode,
                    transaction.selected_source,
                    transaction.should_replace_visible_output,
                    transaction.safe_to_apply,
                    transaction.reason,
                    transaction.selected_line_count,
                    transaction.legacy_line_count,
                    transaction.view_output_line_count,
                    transaction.selected_lines_checksum,
                    transaction.rollback_anchor,
                ),
            });
        }
        if view_output_transaction
            .as_ref()
            .map(|transaction| transaction.should_replace_visible_output)
            .unwrap_or(false)
        {
            if let Some(transaction) = view_output_transaction {
                committed_shadow_lines = Some(transaction.selected_lines.clone());
            }
        } else if commit.use_shadow_output {
            committed_shadow_lines = Some(shadow_report.rendered_lines.clone());
        }
    }

    diagnostics.extend(program.cliErrors.iter().cloned().map(|message| RetaDiagnostic {
        level: DiagnosticLevel::Error,
        code: "CLI_ERROR".to_string(),
        message,
    }));

    let rendered_text = if let Some(lines) = committed_shadow_lines.as_ref() {
        join_output_lines(lines)
    } else if !program.finallyDisplayLines.is_empty() {
        join_output_lines(&program.finallyDisplayLines)
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
        rows_emitted: committed_shadow_lines
            .as_ref()
            .map(Vec::len)
            .unwrap_or_else(|| program.__resultingTable.len().max(program.finallyDisplayLines.len())),
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

fn join_output_lines(lines: &[String]) -> String {
    if lines.is_empty() {
        return String::new();
    }
    let mut text = lines.join("\n");
    if !text.is_empty() {
        text.push('\n');
    }
    text
}
