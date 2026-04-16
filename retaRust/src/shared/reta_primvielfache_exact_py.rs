#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::shared::reta_program_types::Program;
use crate::shared::words_py::{PyValue, StoreParameterEntry, Words};

#[derive(Clone, Debug, Default)]
pub struct PrimvielfacheExactEntry {
    pub parameter_main_name: String,
    pub parameter_name: String,
    pub generated2_codes: Vec<String>,
}

fn push_unique_string(target: &mut Vec<String>, value: String) {
    if !target.iter().any(|existing| existing == &value) {
        target.push(value);
    }
}

fn is_primvielfache_main_name_exact_py(candidate: &str) -> bool {
    Program::parameter_main_name_matches_local_py(candidate, "primvielfache")
}

fn absorb_entry_generated2_exact_py(out: &mut PrimvielfacheExactEntry, entry: &StoreParameterEntry, generated2_index: usize) {
    for value in entry.datas.get(generated2_index).into_iter().flatten() {
        if let PyValue::Str(code) = value {
            push_unique_string(&mut out.generated2_codes, code.clone());
        }
    }
}

pub fn build_primvielfache_generated2_lookup_exact_py(words: &Words, generated2_index: usize) -> HashMap<String, PrimvielfacheExactEntry> {
    let mut out: HashMap<String, PrimvielfacheExactEntry> = HashMap::new();

    for entry in &words.paraNdataMatrix {
        let is_primvielfache_entry = entry
            .parameterMainNames
            .iter()
            .any(|candidate| is_primvielfache_main_name_exact_py(candidate));
        if !is_primvielfache_entry {
            continue;
        }

        for parameter_name in &entry.parameterNames {
            let normalized_sub = parameter_name.trim().to_ascii_lowercase();
            if normalized_sub.is_empty() {
                continue;
            }
            let slot = out.entry(normalized_sub).or_insert_with(|| PrimvielfacheExactEntry {
                parameter_main_name: entry.parameterMainNames.first().cloned().unwrap_or_else(|| "primvielfache".to_string()),
                parameter_name: entry.parameterNames.first().cloned().unwrap_or_default(),
                generated2_codes: Vec::new(),
            });
            absorb_entry_generated2_exact_py(slot, entry, generated2_index);
        }
    }

    out
}
