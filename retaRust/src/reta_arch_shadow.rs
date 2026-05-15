//! Root-crate bridge from the legacy-compatible `Program` run to the typed
//! `reta_architecture` shadow pipeline.
//!
//! This module keeps legacy output as the default.  It builds typed table-shadow
//! reports after the legacy path has produced its table and visible lines, and it
//! returns commit decisions for explicit architecture modes.  A visible switch is
//! only possible when the runtime gate and parity policy both allow it.

use crate::shared::reta_program_types::Program;

#[derive(Clone, Debug)]
pub struct ShadowTableRuntimeReport {
    pub report: reta_architecture::ShadowTableReport,
    pub commit: reta_architecture::ShadowCommitDecision,
    pub view_output_report: Option<reta_architecture::ShadowTableViewOutputReport>,
    pub view_output_commit: Option<reta_architecture::ShadowTableViewOutputCommitDecision>,
    pub view_output_audit: Option<reta_architecture::TableViewCommitAuditReport>,
    pub view_output_transaction: Option<reta_architecture::TableViewActivationTransactionReport>,
    pub view_output_journal: Option<reta_architecture::TableViewActivationJournal>,
    pub view_output_replay: Option<reta_architecture::TableViewActivationReplayReport>,
    pub view_output_ledger: Option<reta_architecture::TableViewActivationLedger>,
}

pub fn shadow_table_report_for_program(
    program: &Program,
    argv: &[String],
) -> Option<reta_architecture::ShadowTableReport> {
    shadow_table_runtime_report_for_program(program, argv).map(|value| value.report)
}

pub fn shadow_table_runtime_report_for_program(
    program: &Program,
    argv: &[String],
) -> Option<ShadowTableRuntimeReport> {
    let (_, switch_config) = reta_architecture::extract_architecture_switch_from_argv(argv, None);
    if !switch_config.mode.should_shadow_execute() && !switch_config.trace {
        return None;
    }
    let input = shadow_table_input_from_program(program);
    let pipeline = reta_architecture::bootstrap_shadow_pipeline();
    let report = pipeline.shadow_table(&input, &switch_config);
    let commit = pipeline.table_commit_decision(&report, &switch_config);
    let view_output_report =
        Some(pipeline.shadow_table_view_output(argv, &program.finallyDisplayLines, &switch_config));
    let view_output_commit = view_output_report
        .as_ref()
        .map(|report| pipeline.table_view_output_commit_decision(report, &switch_config));
    let view_output_audit = view_output_report
        .as_ref()
        .zip(view_output_commit.as_ref())
        .map(|(report, commit)| reta_architecture::audit_table_view_output_commit(report, commit));
    let view_output_transaction = view_output_report
        .as_ref()
        .zip(view_output_commit.as_ref())
        .map(|(report, commit)| {
            reta_architecture::table_view_activation_transaction(
                report,
                commit,
                &program.finallyDisplayLines,
                &reta_architecture::TableViewActivationTransactionPolicy::default(),
            )
        });
    let view_output_journal = view_output_transaction.as_ref().map(|transaction| {
        reta_architecture::activation_journal_from_transactions(
            std::slice::from_ref(transaction),
            &reta_architecture::TableViewActivationJournalPolicy::default(),
        )
    });
    let view_output_replay = view_output_journal
        .as_ref()
        .zip(view_output_transaction.as_ref())
        .map(|(journal, transaction)| {
            reta_architecture::activation_replay_from_journal(
                journal,
                &program.finallyDisplayLines,
                Some(transaction.transaction_id.as_str()),
                &reta_architecture::TableViewActivationReplayPolicy::default(),
            )
        });
    let view_output_ledger = view_output_journal
        .as_ref()
        .zip(view_output_transaction.as_ref())
        .map(|(journal, transaction)| {
            reta_architecture::activation_ledger_from_journal(
                journal,
                &program.finallyDisplayLines,
                Some(transaction.transaction_id.as_str()),
                &reta_architecture::TableViewActivationLedgerPolicy::default(),
            )
        });
    Some(ShadowTableRuntimeReport {
        report,
        commit,
        view_output_report,
        view_output_commit,
        view_output_audit,
        view_output_transaction,
        view_output_journal,
        view_output_replay,
        view_output_ledger,
    })
}

pub fn shadow_table_input_from_program(program: &Program) -> reta_architecture::ShadowTableInput {
    let mode = reta_architecture::OutputMode::from_name(&program.outType)
        .unwrap_or(reta_architecture::OutputMode::Shell);
    let selected_columns_zero_based = program
        .spaltenreihenfolgeundnurdiese
        .iter()
        .filter_map(|value| {
            if *value > 0 {
                usize::try_from(value.saturating_sub(1)).ok()
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    reta_architecture::ShadowTableInput {
        content_table: program.__resultingTable.clone(),
        legacy_display_lines: program.finallyDisplayLines.clone(),
        selected_columns_zero_based,
        mode,
        textwidth: non_negative_usize(program.textWidth).unwrap_or(21),
        textheight: non_negative_usize(program.textHeight).unwrap_or(0),
        breiten: program
            .breiten
            .iter()
            .filter_map(|value| non_negative_usize(*value))
            .collect(),
        shell_rows_amount: non_negative_usize(program.shellRowsAmount)
            .or_else(|| non_negative_usize(program.shellWidth))
            .unwrap_or(0),
        numbering: program.nummeriere,
        color: !program.nocolor,
        keine_ueberschriften: program.keineUeberschriften,
        keine_leeren_inhalte: program.keineleereninhalte,
    }
}

fn non_negative_usize(value: i64) -> Option<usize> {
    if value < 0 {
        None
    } else {
        usize::try_from(value).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_negative_usize_rejects_negative_values() {
        assert_eq!(non_negative_usize(-1), None);
        assert_eq!(non_negative_usize(0), Some(0));
        assert_eq!(non_negative_usize(7), Some(7));
    }
}
