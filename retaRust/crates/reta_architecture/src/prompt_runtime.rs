//! Prompt runtime program view transcompiled from
//! `python_arch_reference/reta_architecture/prompt_runtime.py`.
//!
//! The Python runtime constructs a lightweight `Program` view for `retaPrompt`:
//! main-parameter commands, table maxima, prompt vocabulary and validation
//! snapshots.  This Rust version keeps the same shape with typed data so later
//! prompt execution can stop depending on Python globals.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::completion_runtime::{bootstrap_completion_runtime, CompletionRuntimeBundle};
use crate::parameter_runtime::{bootstrap_parameter_runtime, ParameterRuntimeBundle};

pub fn prime_command_predicate(num: i64) -> i64 {
    if num <= 1 {
        return 0;
    }
    if num == 2 {
        return 1;
    }
    if num % 2 == 0 {
        return 3;
    }
    let mut divisor = 3i64;
    while divisor * divisor <= num {
        if num % divisor == 0 {
            return 3;
        }
        divisor += 2;
    }
    1
}

pub fn build_main_parameter_commands() -> BTreeMap<String, Option<usize>> {
    BTreeMap::from([
        ("zeilen".to_string(), Some(0)),
        ("spalten".to_string(), Some(1)),
        ("kombination".to_string(), Some(2)),
        ("ausgabe".to_string(), Some(3)),
        ("debug".to_string(), None),
        ("h".to_string(), None),
        ("help".to_string(), None),
    ])
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptTablesView {
    pub hoechste_zeile: BTreeMap<i64, i64>,
}

impl Default for PromptTablesView {
    fn default() -> Self {
        Self {
            hoechste_zeile: BTreeMap::from([(1024, 1024), (114, 163)]),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptVocabulary {
    pub start_commands: Vec<String>,
    pub main_parameters: Vec<String>,
    pub output_modes: Vec<String>,
    pub row_parameters: Vec<String>,
    pub combination_parameters: Vec<String>,
}

impl PromptVocabulary {
    pub fn from_completion_runtime(runtime: &CompletionRuntimeBundle) -> Self {
        Self {
            start_commands: runtime.start_commands(true),
            main_parameters: runtime.main_parameters.clone(),
            output_modes: runtime.ausgabe_art.clone(),
            row_parameters: runtime.zeilen_paras.clone(),
            combination_parameters: runtime.kombi_main_paras.clone(),
        }
    }

    pub fn snapshot(&self) -> PromptVocabularySnapshot {
        PromptVocabularySnapshot {
            start_commands_len: self.start_commands.len(),
            main_parameters_len: self.main_parameters.len(),
            output_modes_len: self.output_modes.len(),
            row_parameters_len: self.row_parameters.len(),
            combination_parameters_len: self.combination_parameters.len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptVocabularySnapshot {
    pub start_commands_len: usize,
    pub main_parameters_len: usize,
    pub output_modes_len: usize,
    pub row_parameters_len: usize,
    pub combination_parameters_len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptProgramView {
    pub main_para_cmds: BTreeMap<String, Option<usize>>,
    pub para_dict_len: usize,
    pub data_dict_sizes: Vec<usize>,
    pub kombi_reverse_dict_len: usize,
    pub kombi_reverse_dict2_len: usize,
    pub all_simple_command_spalten: BTreeSet<i64>,
    pub tables: PromptTablesView,
}

impl PromptProgramView {
    pub fn from_parameter_runtime(parameter_runtime: &ParameterRuntimeBundle) -> Self {
        let command_count = parameter_runtime.main_commands.len();
        Self {
            main_para_cmds: build_main_parameter_commands(),
            para_dict_len: command_count,
            data_dict_sizes: vec![command_count, 0, 0, 0],
            kombi_reverse_dict_len: 0,
            kombi_reverse_dict2_len: 0,
            all_simple_command_spalten: BTreeSet::new(),
            tables: PromptTablesView::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptRuntimeValidation {
    pub wahl15_missing_values: Vec<String>,
    pub wahl15_valid: bool,
    pub prime_predicate_for_2: i64,
    pub prime_predicate_for_4: i64,
}

impl Default for PromptRuntimeValidation {
    fn default() -> Self {
        Self {
            wahl15_missing_values: Vec::new(),
            wahl15_valid: true,
            prime_predicate_for_2: prime_command_predicate(2),
            prime_predicate_for_4: prime_command_predicate(4),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptRuntimeBundle {
    pub program: PromptProgramView,
    pub vocabulary: PromptVocabulary,
    pub validation: PromptRuntimeValidation,
}

impl PromptRuntimeBundle {
    pub fn snapshot(&self) -> PromptRuntimeSnapshot {
        PromptRuntimeSnapshot {
            program_view_class: "PromptProgramView".to_string(),
            main_para_cmds: self.program.main_para_cmds.keys().cloned().collect(),
            para_dict_len: self.program.para_dict_len,
            data_dict_sizes: self.program.data_dict_sizes.clone(),
            kombi_reverse_dict_len: self.program.kombi_reverse_dict_len,
            kombi_reverse_dict2_len: self.program.kombi_reverse_dict2_len,
            all_simple_command_spalten_len: self.program.all_simple_command_spalten.len(),
            max_rows: self.program.tables.hoechste_zeile.clone(),
            vocabulary: self.vocabulary.snapshot(),
            validation: self.validation.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptRuntimeSnapshot {
    pub program_view_class: String,
    pub main_para_cmds: Vec<String>,
    pub para_dict_len: usize,
    pub data_dict_sizes: Vec<usize>,
    pub kombi_reverse_dict_len: usize,
    pub kombi_reverse_dict2_len: usize,
    pub all_simple_command_spalten_len: usize,
    pub max_rows: BTreeMap<i64, i64>,
    pub vocabulary: PromptVocabularySnapshot,
    pub validation: PromptRuntimeValidation,
}

#[derive(Clone, Debug)]
pub struct PromptRuntimeBuilder {
    pub parameter_runtime: ParameterRuntimeBundle,
    pub completion_runtime: CompletionRuntimeBundle,
}

impl Default for PromptRuntimeBuilder {
    fn default() -> Self {
        Self {
            parameter_runtime: bootstrap_parameter_runtime(),
            completion_runtime: bootstrap_completion_runtime(),
        }
    }
}

impl PromptRuntimeBuilder {
    pub fn build(&self) -> PromptRuntimeBundle {
        let program = PromptProgramView::from_parameter_runtime(&self.parameter_runtime);
        let vocabulary = PromptVocabulary::from_completion_runtime(&self.completion_runtime);
        PromptRuntimeBundle {
            program,
            vocabulary,
            validation: PromptRuntimeValidation::default(),
        }
    }
}

pub fn bootstrap_prompt_runtime() -> PromptRuntimeBundle {
    PromptRuntimeBuilder::default().build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_predicate_matches_python_marker_values() {
        assert_eq!(prime_command_predicate(0), 0);
        assert_eq!(prime_command_predicate(2), 1);
        assert_eq!(prime_command_predicate(9), 3);
    }

    #[test]
    fn runtime_snapshot_has_main_commands() {
        let runtime = bootstrap_prompt_runtime();
        let snapshot = runtime.snapshot();
        assert!(snapshot.main_para_cmds.contains(&"zeilen".to_string()));
        assert!(snapshot.vocabulary.start_commands_len > 0);
    }
}
