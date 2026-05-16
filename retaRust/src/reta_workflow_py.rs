use crate::reta_program_types::{
    DiagnosticLevel, RetaDiagnostic, RetaError, RetaMetadata, RetaRequest, RetaResponse,
};
use crate::reta_runtime_bridge::with_runtime_override;
use crate::{fresh_program_from_template, preload_reta_runtime, shared_words};

pub fn run_reta(request: RetaRequest) -> Result<RetaResponse, RetaError> {
    let argv = normalize_program_argv(&request.raw_args);
    let _architecture_run = reta_architecture::RetaRunArchitecture::from_cli_args(&argv);
    let (arch_clean_argv, _) =
        reta_architecture::extract_architecture_switch_from_argv(&argv, None);
    let (legacy_argv, _) =
        reta_architecture::extract_parallel_config_from_argv(&arch_clean_argv, None);

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

    if !request
        .input
        .stdin_text
        .as_deref()
        .unwrap_or_default()
        .is_empty()
    {
        diagnostics.push(RetaDiagnostic {
            level: DiagnosticLevel::Info,
            code: "STDIN_BUFFER_PRESENT".to_string(),
            message: "stdin wurde vom Binary entgegengenommen und an die Library weitergereicht."
                .to_string(),
        });
    }

    let mut committed_shadow_lines: Option<Vec<String>> = None;
    if let Some(shadow_runtime) =
        crate::reta_arch_shadow::shadow_table_runtime_report_for_program(&program, &argv)
    {
        let shadow_report = shadow_runtime.report;
        let commit = shadow_runtime.commit;
        let view_output_report = shadow_runtime.view_output_report;
        let view_output_commit = shadow_runtime.view_output_commit;
        let view_output_audit = shadow_runtime.view_output_audit;
        let view_output_transaction = shadow_runtime.view_output_transaction;
        let view_output_journal = shadow_runtime.view_output_journal;
        let view_output_replay = shadow_runtime.view_output_replay;
        let view_output_ledger = shadow_runtime.view_output_ledger;
        let view_output_store = shadow_runtime.view_output_store;
        let view_output_persistence = shadow_runtime.view_output_persistence;
        let view_output_file = shadow_runtime.view_output_file;
        let view_output_recovery = shadow_runtime.view_output_recovery;
        let view_output_readiness = shadow_runtime.view_output_readiness;
        let view_output_promotion = shadow_runtime.view_output_promotion;
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
        if let Some(report) = view_output_report.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if report.language_parity.ready() {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_LANGUAGE_PARITY".to_string(),
                message: format!(
                    "Rust-Architektur-Sprachparität: requested_language={} effective_language={} requested_asset={} effective_asset={} fallback_required={} fallback_applied={} direct_744={} status={} failed={:?}",
                    report.language_parity.requested_language,
                    report.language_parity.effective_language,
                    report.language_parity.requested_asset_name,
                    report.language_parity.effective_asset_name,
                    report.language_parity.fallback_required,
                    report.language_parity.fallback_applied,
                    report.language_parity.direct_744_materialized,
                    report.language_parity.status,
                    report.language_parity.failed_guards,
                ),
            });
        }
        if let Some(report) = view_output_report.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if report.language_coverage.ready() {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_LANGUAGE_COVERAGE".to_string(),
                message: format!(
                    "Rust-Architektur-Sprachabdeckung: requested_language={} requested_asset={} effective_asset={} fallback_required={} fallback_applied={} stale_languages={} missing_744={:?} status={} failed={:?}",
                    report.language_coverage.requested_language,
                    report.language_coverage.requested_asset_name,
                    report.language_coverage.effective_asset_name,
                    report.language_coverage.fallback_required,
                    report.language_coverage.fallback_applied,
                    report.language_coverage.stale_language_count,
                    report.language_coverage.languages_missing_744,
                    report.language_coverage.status,
                    report.language_coverage.failed_guards,
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
                    "Rust-Architektur-Materialized-View-Commit-Gate: mode={} use_view_output={} reason={} gate={} diff_equal={} semantic_equal={} language_ready={} language_asset={} lines={} rollback={:?}",
                    commit.switch_mode,
                    commit.use_view_output,
                    commit.reason,
                    commit.gate_reason,
                    commit.diff_equal,
                    commit.semantic_equal,
                    commit.language_parity_ready,
                    commit.language_effective_asset_name,
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
                    "Rust-Architektur-Commit-Audit: mode={} safe={} use_view_output={} required={}/{} failed={:?} raw_equal={} semantic_equal={} virtual_direct={} virtual_added={} language_ready={} language_asset={} first_raw_diff={:?} first_semantic_diff={:?}",
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
                    audit.language_parity_ready,
                    audit.language_effective_asset_name,
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
        if let Some(journal) = view_output_journal.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if journal.replayable || journal.rejected_record_count > 0 {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_ACTIVATION_JOURNAL".to_string(),
                message: format!(
                    "Rust-Architektur-Aktivierungs-Journal: records={} safe={} rejected={} replayable={} latest_source={:?} checksum={:?} rollback={:?}",
                    journal.record_count,
                    journal.safe_record_count,
                    journal.rejected_record_count,
                    journal.replayable,
                    journal.latest_selected_source,
                    journal.latest_selected_checksum,
                    journal.latest_rollback_anchor,
                ),
            });
        }
        if let Some(replay) = view_output_replay.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if replay.replay_visible_output || replay.selected_source == "legacy_output" {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_ACTIVATION_REPLAY".to_string(),
                message: format!(
                    "Rust-Architektur-Aktivierungs-Replay: replay={} safe={} source={} reason={} selected_lines={} checksum={} current_legacy_checksum={} tx_match={} legacy_match={}",
                    replay.replay_visible_output,
                    replay.replay_safe,
                    replay.selected_source,
                    replay.reason,
                    replay.selected_line_count,
                    replay.selected_lines_checksum,
                    replay.current_legacy_checksum,
                    replay.latest_transaction_matches_current,
                    replay.latest_legacy_checksum_matches_current,
                ),
            });
        }
        if let Some(ledger) = view_output_ledger.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if ledger.validation.is_ready() {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_ACTIVATION_LEDGER".to_string(),
                message: format!(
                    "Rust-Architektur-Aktivierungs-Ledger: entries={} safe={} rejected={} status={} chain_valid={} replay={} source={} chain={:?} failed={:?}",
                    ledger.entry_count,
                    ledger.safe_entry_count,
                    ledger.rejected_entry_count,
                    ledger.validation.status,
                    ledger.validation.hash_chain_valid,
                    ledger.replay_visible_output,
                    ledger.replay_selected_source,
                    ledger.latest_chain_hash,
                    ledger.validation.failed_guards,
                ),
            });
        }
        if let Some(store) = view_output_store.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if store.validation.is_ready() {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_ACTIVATION_STORE".to_string(),
                message: format!(
                    "Rust-Architektur-Aktivierungs-Store: records={} lines={} selected_lines={} ledger_entries={} status={} checksum={} chain={:?} failed={:?}",
                    store.record_count,
                    store.line_count,
                    store.selected_line_count,
                    store.ledger_entry_count,
                    store.validation.status,
                    store.text_checksum,
                    store.latest_chain_hash,
                    store.validation.failed_guards,
                ),
            });
        }
        if let Some(persistence) = view_output_persistence.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if persistence.is_ready() {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_ACTIVATION_PERSISTENCE".to_string(),
                message: format!(
                    "Rust-Architektur-Aktivierungs-Persistenz: status={} kind={} name={} loaded_matches={} parse_ready={} source_digest={} parse_failed={:?}",
                    persistence.status,
                    persistence.store_kind,
                    persistence.store_name,
                    persistence.loaded_matches_source,
                    persistence.parse_ready,
                    persistence.source_text_digest,
                    persistence.parse_failed_guards,
                ),
            });
        }
        if let Some(file) = view_output_file.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if file.is_ready() {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_ACTIVATION_FILE".to_string(),
                message: format!(
                    "Rust-Architektur-Aktivierungs-Datei: status={} path={} wrote={} read={} matches={} parse_ready={} source_digest={} failed={:?}",
                    file.status,
                    file.path,
                    file.wrote_file,
                    file.read_file,
                    file.read_matches_source,
                    file.parse_ready,
                    file.source_text_digest,
                    file.failed_guards,
                ),
            });
        }
        if let Some(recovery) = view_output_recovery.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if recovery.is_ready() || recovery.selected_source == "legacy_output" {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_ACTIVATION_RECOVERY".to_string(),
                message: format!(
                    "Rust-Architektur-Aktivierungs-Recovery: status={} path={:?} read={} parsed={} parse_ready={} replay_safe={} recover={} source={} selected_lines={} checksum={} failed={:?}",
                    recovery.status,
                    recovery.path,
                    recovery.read_file,
                    recovery.parsed,
                    recovery.parse_ready,
                    recovery.replay_safe,
                    recovery.recover_visible_output,
                    recovery.selected_source,
                    recovery.selected_line_count,
                    recovery.selected_lines_checksum,
                    recovery.failed_guards,
                ),
            });
        }
        if let Some(readiness) = view_output_readiness.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if readiness.ready_for_visible_activation {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_ACTIVATION_READINESS".to_string(),
                message: format!(
                    "Rust-Architektur-Aktivierungs-Readiness: status={} ready={} level={} source={} required={}/{} raw_equal={} semantic_equal={} virtual_direct={} language_ready={} language_asset={} failed={:?}",
                    readiness.status,
                    readiness.ready_for_visible_activation,
                    readiness.promotion_level,
                    readiness.selected_source,
                    readiness.passed_required_check_count,
                    readiness.required_check_count,
                    readiness.raw_equal,
                    readiness.semantic_equal,
                    readiness.virtual_direct_cells_equal,
                    readiness.language_parity_ready,
                    readiness.language_effective_asset_name,
                    readiness.failed_required_checks,
                ),
            });
        }
        if let Some(promotion) = view_output_promotion.as_ref() {
            diagnostics.push(RetaDiagnostic {
                level: if promotion.ready_for_default_promotion {
                    DiagnosticLevel::Info
                } else {
                    DiagnosticLevel::Warning
                },
                code: "ARCH_TABLE_VIEW_ACTIVATION_PROMOTION".to_string(),
                message: format!(
                    "Rust-Architektur-Aktivierungs-Promotion: status={} ready={} level={} action={} visible_source={} required={}/{} raw_equal={} semantic_equal={} virtual_direct={} language_ready={} language_asset={} gate={} failed={:?}",
                    promotion.status,
                    promotion.ready_for_default_promotion,
                    promotion.promotion_level,
                    promotion.promotion_action,
                    promotion.visible_output_source,
                    promotion.passed_required_check_count,
                    promotion.required_check_count,
                    promotion.raw_equal,
                    promotion.semantic_equal,
                    promotion.virtual_direct_cells_equal,
                    promotion.language_parity_ready,
                    promotion.language_effective_asset_name,
                    promotion.gate.reason,
                    promotion.failed_required_checks,
                ),
            });
        }
        if committed_shadow_lines.is_none()
            && view_output_recovery
                .as_ref()
                .map(|recovery| recovery.recover_visible_output)
                .unwrap_or(false)
        {
            if let Some(recovery) = view_output_recovery.as_ref() {
                committed_shadow_lines = Some(recovery.selected_lines.clone());
            }
        }
        if let Some(ledger) = view_output_ledger.as_ref() {
            if ledger.validation.is_ready() && ledger.replay_visible_output {
                if let Some(replay) = ledger.replay.as_ref() {
                    committed_shadow_lines = Some(replay.selected_lines.clone());
                }
            }
        }
        if committed_shadow_lines.is_none()
            && view_output_replay
                .as_ref()
                .map(|replay| replay.replay_visible_output)
                .unwrap_or(false)
        {
            if let Some(replay) = view_output_replay {
                committed_shadow_lines = Some(replay.selected_lines.clone());
            }
        } else if committed_shadow_lines.is_none()
            && view_output_transaction
                .as_ref()
                .map(|transaction| transaction.should_replace_visible_output)
                .unwrap_or(false)
        {
            if let Some(transaction) = view_output_transaction {
                committed_shadow_lines = Some(transaction.selected_lines.clone());
            }
        } else if committed_shadow_lines.is_none() && commit.use_shadow_output {
            committed_shadow_lines = Some(shadow_report.rendered_lines.clone());
        }
    }

    diagnostics.extend(
        program
            .cliErrors
            .iter()
            .cloned()
            .map(|message| RetaDiagnostic {
                level: DiagnosticLevel::Error,
                code: "CLI_ERROR".to_string(),
                message,
            }),
    );

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
            .unwrap_or_else(|| {
                program
                    .__resultingTable
                    .len()
                    .max(program.finallyDisplayLines.len())
            }),
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
