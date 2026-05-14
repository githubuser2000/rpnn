//! Program workflow glue transcompiled from
//! `python_arch_reference/reta_architecture/program_workflow.py`.
//!
//! This Rust layer records the same top-level orchestration shape: load CSV,
//! parse positive and negative parameters, bind column sections, generate table
//! sections and render output.  The byte-exact renderer remains in the legacy
//! path for now; this module is the typed universal glue node for the next
//! replacement stages.

use serde::{Deserialize, Serialize};

use crate::output_syntax::OutputMode;
use crate::parameter_runtime::{MainParameter, ParameterParseResult, ParameterRuntimeBundle};
use crate::table_generation::{TableGenerationBundle, TableGenerationPlan};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub name: String,
    pub input_section: String,
    pub output_section: String,
    pub morphism: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTrace {
    pub args_len: usize,
    pub main_contexts: Vec<String>,
    pub selected_output_mode: Option<OutputMode>,
    pub upper_limit: Option<i64>,
    pub requires_kombi: bool,
    pub requires_concat_csv: bool,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProgramWorkflowBundle {
    pub component_order: Vec<String>,
    pub universal_property: String,
    pub parameter_runtime: ParameterRuntimeBundle,
    pub table_generation: TableGenerationBundle,
}

impl ProgramWorkflowBundle {
    pub fn trace_from_parse(
        &self,
        args_len: usize,
        parsed: &ParameterParseResult,
        generation_plan: &TableGenerationPlan,
    ) -> WorkflowTrace {
        WorkflowTrace {
            args_len,
            main_contexts: parsed
                .main_context_history
                .iter()
                .map(MainParameter::canonical_name)
                .collect(),
            selected_output_mode: parsed.selected_output_mode,
            upper_limit: parsed.upper_limit,
            requires_kombi: generation_plan.requires_kombi(),
            requires_concat_csv: generation_plan.requires_concat_csv(),
            steps: self.default_steps(),
        }
    }

    pub fn trace_from_args<S: AsRef<str>>(&self, args: &[S]) -> WorkflowTrace {
        let parsed = self.parameter_runtime.parse_cli_args(args);
        let plan = TableGenerationPlan::default();
        self.trace_from_parse(args.len(), &parsed, &plan)
    }

    pub fn default_steps(&self) -> Vec<WorkflowStep> {
        vec![
            WorkflowStep {
                name: "load_religion_table".to_string(),
                input_section: "csv/religion.csv".to_string(),
                output_section: "program.relitable".to_string(),
                morphism: "decode_religion_cell".to_string(),
            },
            WorkflowStep {
                name: "read_positive_and_negative_parameters".to_string(),
                input_section: "argv".to_string(),
                output_section: "param_lines + param_lines_not".to_string(),
                morphism: "parameters_to_commands_and_numbers".to_string(),
            },
            WorkflowStep {
                name: "bind_column_sections".to_string(),
                input_section: "column_bucket_presheaf".to_string(),
                output_section: "program selected-column sections".to_string(),
                morphism: "ColumnSelectionBundle::bind_program_sections".to_string(),
            },
            WorkflowStep {
                name: "build_table_sections".to_string(),
                input_section: "CSV/generated/Kombi local sections".to_string(),
                output_section: "global result table section".to_string(),
                morphism: "TableGenerationBundle::build_for_program".to_string(),
            },
            WorkflowStep {
                name: "render_output".to_string(),
                input_section: "prepared result table".to_string(),
                output_section: "stdout text".to_string(),
                morphism: "TableOutput::cliOut".to_string(),
            },
        ]
    }

    pub fn snapshot(&self) -> ProgramWorkflowSnapshot {
        ProgramWorkflowSnapshot {
            class: "ProgramWorkflowBundle".to_string(),
            component_order: self.component_order.clone(),
            universal_property: self.universal_property.clone(),
            steps: self.default_steps(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProgramWorkflowSnapshot {
    pub class: String,
    pub component_order: Vec<String>,
    pub universal_property: String,
    pub steps: Vec<WorkflowStep>,
}

pub fn bootstrap_program_workflow() -> ProgramWorkflowBundle {
    ProgramWorkflowBundle {
        component_order: vec![
            "csv_loader".to_string(),
            "parameter_runtime".to_string(),
            "column_selection".to_string(),
            "table_preparation".to_string(),
            "table_generation".to_string(),
            "table_output".to_string(),
        ],
        universal_property:
            "same argv and CSV sections have a unique deterministic rendered output section"
                .to_string(),
        parameter_runtime: crate::parameter_runtime::bootstrap_parameter_runtime(),
        table_generation: crate::table_generation::bootstrap_table_generation(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_trace_keeps_output_mode_and_steps() {
        let workflow = bootstrap_program_workflow();
        let trace = workflow.trace_from_args(&["reta", "-ausgabe", "--art=html"]);
        assert_eq!(trace.selected_output_mode, Some(OutputMode::Html));
        assert!(trace.steps.iter().any(|step| step.name == "render_output"));
    }
}

// Stage 16 continued: concrete program_workflow.py compatibility wrappers.
pub fn _reset_runtime_flags() -> Vec<String> { vec!["runtime_flags_reset".to_string()] }
pub fn _csv_path(name: &str) -> String { format!("csv/{name}") }
pub fn _decode_religion_cell(cell: &str) -> Vec<i64> { cell.split(|ch: char| !ch.is_ascii_digit() && ch != '-').filter_map(|part| part.parse().ok()).collect() }
pub fn _load_religion_table(text: &str) -> Vec<Vec<String>> { text.lines().map(|line| line.split(';').map(str::to_string).collect()).collect() }
pub fn _read_positive_and_negative_parameters(values: &[i64]) -> (Vec<i64>, Vec<i64>) { values.iter().copied().partition(|value| *value >= 0) }
pub fn _requested_religion_output_kind(args: &[String]) -> Option<String> { args.iter().find_map(|arg| arg.strip_prefix("--religion=").map(str::to_string)) }
pub fn _apply_language_specific_motive_column(language: &str, column: i64) -> i64 { if language == "de" || language == "deutsch" { column } else { column + 1000 } }
pub fn bring_all_important_begin_things(args: &[String]) -> WorkflowTrace { bootstrap_program_workflow().trace_from_args(args) }
pub fn workflow_everything(args: &[String]) -> WorkflowTrace { bootstrap_program_workflow().trace_from_args(args) }
pub fn combi_table_workflow(args: &[String]) -> WorkflowTrace { bootstrap_program_workflow().trace_from_args(args) }

// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "_apply_language_specific_motive_column",
    "_csv_path",
    "_decode_religion_cell",
    "_load_religion_table",
    "_read_positive_and_negative_parameters",
    "_requested_religion_output_kind",
    "_reset_runtime_flags",
    "bring_all_important_begin_things",
    "combi_table_workflow",
    "workflow_everything",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
