//! Root-crate bridge from the legacy-compatible `Program` run to the typed
//! `reta_architecture` shadow pipeline.
//!
//! This module deliberately does not change visible output.  It builds a typed
//! table-shadow report after the legacy path has produced its table and visible
//! lines, so adapter activation can be audited before a commit gate is allowed.

use crate::shared::reta_program_types::Program;

pub fn shadow_table_report_for_program(
    program: &Program,
    argv: &[String],
) -> Option<reta_architecture::ShadowTableReport> {
    let (_, switch_config) = reta_architecture::extract_architecture_switch_from_argv(argv, None);
    if !switch_config.mode.should_shadow_execute() && !switch_config.trace {
        return None;
    }
    let input = shadow_table_input_from_program(program);
    Some(reta_architecture::bootstrap_shadow_pipeline().shadow_table(&input, &switch_config))
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
