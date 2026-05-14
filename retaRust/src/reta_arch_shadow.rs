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
    let view_output_report = Some(pipeline.shadow_table_view_output(
        argv,
        &program.finallyDisplayLines,
        &switch_config,
    ));
    let view_output_commit = view_output_report
        .as_ref()
        .map(|report| pipeline.table_view_output_commit_decision(report, &switch_config));
    Some(ShadowTableRuntimeReport {
        report,
        commit,
        view_output_report,
        view_output_commit,
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
