//! Prompt session state transcompiled from
//! `python_arch_reference/reta_architecture/prompt_session.py`.
//!
//! This is the non-interactive, prompt-toolkit-free Rust owner for mutable
//! prompt text state, one-shot setup, storage placeholder handling and deletion
//! of stored command sections.  The real TUI/frontends can layer their input
//! mechanism on top of this deterministic state bundle.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::completion_runtime::{bootstrap_completion_runtime, CompletionRuntimeBundle};
use crate::prompt_language::{bootstrap_prompt_language, custom_split, PromptLanguageBundle, PromptModus};
use crate::prompt_runtime::{bootstrap_prompt_runtime, PromptRuntimeBundle};
use crate::row_ranges::{bootstrap_row_range_morphisms, RowRangeMorphismBundle};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptTextState {
    pub text: String,
    pub platzhalter: String,
    pub liste: Vec<String>,
    pub liste_s: Vec<String>,
    pub liste_e: Vec<String>,
    pub e: Vec<String>,
    pub menge: BTreeSet<String>,
    pub menge_e: BTreeSet<String>,
    pub befehl_davor: String,
}

impl PromptTextState {
    pub fn new(text: impl Into<String>) -> Self {
        let mut state = Self {
            text: String::new(),
            platzhalter: String::new(),
            liste: Vec::new(),
            liste_s: Vec::new(),
            liste_e: Vec::new(),
            e: Vec::new(),
            menge: BTreeSet::new(),
            menge_e: BTreeSet::new(),
            befehl_davor: String::new(),
        };
        state.set_text(text);
        state
    }

    pub fn set_text(&mut self, value: impl Into<String>) {
        let value = value.into().trim().to_string();
        self.text = value.clone();
        if value.starts_with("reta") {
            self.liste = value
                .split_whitespace()
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(str::to_string)
                .collect();
            self.liste_s = self.liste.clone();
        } else {
            self.liste = split_prompt_text(&value);
            self.liste_s = split_command_words(&value);
        }
        self.recompute_sets();
    }

    pub fn set_liste(&mut self, value: Vec<String>) {
        self.liste = value
            .into_iter()
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect();
        self.liste_s = self
            .liste
            .iter()
            .flat_map(|item| split_prompt_text(item))
            .collect();
        self.text = self.liste.join(" ");
        self.recompute_sets();
    }

    pub fn set_e(&mut self, value: Vec<String>) {
        self.e = value;
        self.recompute_sets();
    }

    pub fn set_platzhalter(&mut self, value: impl Into<String>) {
        self.platzhalter = value.into().trim().to_string();
    }

    pub fn has(&self, has_set: &BTreeSet<String>) -> bool {
        !self.menge.intersection(has_set).next().is_none()
    }

    pub fn has_without_abc(&self, has_set: &BTreeSet<String>) -> bool {
        let abc = BTreeSet::from(["abc".to_string(), "abcd".to_string()]);
        self.has(has_set) && self.menge.intersection(&abc).next().is_none()
    }

    fn recompute_sets(&mut self) {
        self.menge = self.liste.iter().cloned().collect();
        self.liste_e = self.liste.iter().cloned().chain(self.e.iter().cloned()).collect();
        self.menge_e = self.liste_e.iter().cloned().collect();
    }
}

pub fn split_prompt_text(text: &str) -> Vec<String> {
    custom_split(text)
        .into_iter()
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .collect()
}

pub fn split_command_words(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLoopSetup {
    pub befehle_beenden: BTreeSet<String>,
    pub logging_switch: bool,
    pub prompt_mode: PromptModus,
    pub prompt_mode2: PromptModus,
    pub prompt_prefixes: BTreeMap<PromptModus, String>,
    pub start_commands: Vec<String>,
    pub only_one_command: Vec<String>,
    pub force_e_command: bool,
    pub text_dazu0: Vec<String>,
}

impl PromptLoopSetup {
    pub fn snapshot(&self) -> PromptLoopSetupSnapshot {
        PromptLoopSetupSnapshot {
            logging_switch: self.logging_switch,
            prompt_mode: self.prompt_mode,
            prompt_mode2: self.prompt_mode2,
            only_one_command: self.only_one_command.clone(),
            force_e_command: self.force_e_command,
            befehle_beenden_len: self.befehle_beenden.len(),
            prompt_prefixes: self.prompt_prefixes.clone(),
            start_commands_len: self.start_commands.len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLoopSetupSnapshot {
    pub logging_switch: bool,
    pub prompt_mode: PromptModus,
    pub prompt_mode2: PromptModus,
    pub only_one_command: Vec<String>,
    pub force_e_command: bool,
    pub befehle_beenden_len: usize,
    pub prompt_prefixes: BTreeMap<PromptModus, String>,
    pub start_commands_len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptStoreResult {
    pub chains: Vec<String>,
    pub text_state: PromptTextState,
    pub prompt_mode2: PromptModus,
    pub text_dazu0: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptSessionBundle {
    pub prompt_runtime: PromptRuntimeBundle,
    pub completion_runtime: CompletionRuntimeBundle,
    pub prompt_language: PromptLanguageBundle,
    pub row_ranges: RowRangeMorphismBundle,
    pub history_file: String,
    pub befehle_beenden: BTreeSet<String>,
}

impl PromptSessionBundle {
    pub fn snapshot(&self) -> PromptSessionSnapshot {
        PromptSessionSnapshot {
            class: "PromptSessionBundle".to_string(),
            history_file: self.history_file.clone(),
            prompt_runtime_class: "PromptRuntimeBundle".to_string(),
            completion_runtime_class: "CompletionRuntimeBundle".to_string(),
            prompt_language_class: "PromptLanguageBundle".to_string(),
            befehle_beenden_len: self.befehle_beenden.len(),
        }
    }

    pub fn new_text_state(&self, text: impl Into<String>) -> PromptTextState {
        PromptTextState::new(text)
    }

    pub fn build_loop_setup(&self, argv: &[String]) -> PromptLoopSetup {
        let mut logging_switch = false;
        let mut only_one_command = Vec::new();
        let mut force_e_command = false;
        let mut index = 0usize;
        while index < argv.len() {
            match argv[index].as_str() {
                "-log" => logging_switch = true,
                "-e" => force_e_command = true,
                "-befehl" | "-command" => {
                    only_one_command = argv[index + 1..].to_vec();
                    break;
                }
                _ => {}
            }
            index += 1;
        }
        let prompt_prefixes = BTreeMap::from([
            (PromptModus::Normal, ">".to_string()),
            (PromptModus::Speichern, "was speichern>".to_string()),
            (PromptModus::LoeschenSelect, "was löschen>".to_string()),
            (PromptModus::SpeicherungAusgaben, "o>".to_string()),
        ]);
        PromptLoopSetup {
            befehle_beenden: self.befehle_beenden.clone(),
            logging_switch,
            prompt_mode: PromptModus::Normal,
            prompt_mode2: PromptModus::Normal,
            prompt_prefixes,
            start_commands: self.completion_runtime.start_commands(true),
            only_one_command,
            force_e_command,
            text_dazu0: Vec::new(),
        }
    }

    pub fn store_prompt(
        &self,
        chains: Vec<String>,
        placeholder: &str,
        text: &str,
        prompt_mode2: PromptModus,
    ) -> PromptStoreResult {
        let mut text_state = self.new_text_state(text);
        text_state.set_platzhalter(placeholder);
        let has_placeholder = !text_state.platzhalter.is_empty();
        let has_chains = !chains.is_empty();
        if has_placeholder || has_chains {
            let mut combined_tokens = split_prompt_text(&text_state.platzhalter);
            combined_tokens.extend(text_state.liste.iter().cloned());
            let mut seen = BTreeSet::new();
            let normalized = combined_tokens
                .into_iter()
                .filter(|item| seen.insert(item.clone()))
                .collect::<Vec<_>>();
            text_state.set_platzhalter(normalized.join(" "));
        } else {
            text_state.set_platzhalter(text_state.text.clone());
        }
        text_state.set_text("");
        let prompt_mode2 = if !text_state.platzhalter.is_empty() || !(has_placeholder || has_chains) {
            PromptModus::AusgabeSelektiv
        } else {
            prompt_mode2
        };
        let text_dazu0 = split_prompt_text(&text_state.platzhalter);
        PromptStoreResult {
            chains,
            text_state,
            prompt_mode2,
            text_dazu0,
        }
    }

    pub fn delete_before_storage_commands(
        &self,
        placeholder: &str,
        text: &str,
    ) -> (String, PromptModus, String) {
        let text_to_delete = self.new_text_state(text);
        let existing = split_prompt_text(placeholder);
        let mut keep_by_index = existing
            .iter()
            .enumerate()
            .map(|(index, value)| (index + 1, value.clone()))
            .collect::<BTreeMap<_, _>>();
        let delete_text_is_decimal = !text_to_delete.text.is_empty()
            && text_to_delete.text.chars().all(|ch| ch.is_ascii_digit());
        let delete_text_exists_as_stored_token = existing
            .iter()
            .any(|item| item == &text_to_delete.text);
        let use_range = self.row_ranges.is_row_range(text_to_delete.text.as_str())
            && (!delete_text_exists_as_stored_token || !delete_text_is_decimal);
        if use_range {
            for token in self.row_ranges.range_to_numbers(&text_to_delete.text, false, 0, false) {
                if token > 0 {
                    keep_by_index.remove(&(token as usize));
                }
            }
            let placeholder = keep_by_index.into_values().collect::<Vec<_>>().join(" ");
            (placeholder, PromptModus::Normal, text_to_delete.text)
        } else {
            let delete_set: BTreeSet<String> = text_to_delete.liste.iter().cloned().collect();
            let placeholder = existing
                .into_iter()
                .filter(|item| !delete_set.contains(item))
                .collect::<Vec<_>>()
                .join(" ");
            (placeholder, PromptModus::Normal, String::new())
        }
    }

    pub fn apply_storage_output(
        &self,
        pending_output: &[String],
        prompt_mode: PromptModus,
        mut text_state: PromptTextState,
    ) -> PromptTextState {
        match prompt_mode {
            PromptModus::SpeicherungAusgaben => text_state.set_text(text_state.platzhalter.clone()),
            PromptModus::SpeicherungAusgabenMitZusatz => {
                let mut text = text_state.platzhalter.clone();
                if !pending_output.is_empty() {
                    if !text.is_empty() {
                        text.push(' ');
                    }
                    text.push_str(&pending_output.join(" "));
                }
                text_state.set_text(text);
            }
            _ => {}
        }
        text_state
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptSessionSnapshot {
    pub class: String,
    pub history_file: String,
    pub prompt_runtime_class: String,
    pub completion_runtime_class: String,
    pub prompt_language_class: String,
    pub befehle_beenden_len: usize,
}

pub fn bootstrap_prompt_session() -> PromptSessionBundle {
    PromptSessionBundle {
        prompt_runtime: bootstrap_prompt_runtime(),
        completion_runtime: bootstrap_completion_runtime(),
        prompt_language: bootstrap_prompt_language(),
        row_ranges: bootstrap_row_range_morphisms(None),
        history_file: "~/.ReTaPromptHistory".to_string(),
        befehle_beenden: BTreeSet::from(["exit".to_string(), "quit".to_string()]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_state_splits_reta_plainly() {
        let state = PromptTextState::new("reta -zeilen --alles");
        assert_eq!(state.liste, vec!["reta", "-zeilen", "--alles"]);
        assert!(state.menge.contains("--alles"));
    }

    #[test]
    fn deletion_uses_row_ranges_for_placeholder_indices() {
        let session = bootstrap_prompt_session();
        let (placeholder, mode, _) = session.delete_before_storage_commands("a b c d", "2-3");
        assert_eq!(mode, PromptModus::Normal);
        assert_eq!(placeholder, "a d");
    }

    #[test]
    fn deletion_prefers_indices_for_non_decimal_range_even_when_literal_exists() {
        let session = bootstrap_prompt_session();
        let (placeholder, mode, text_out) =
            session.delete_before_storage_commands("1-2 x y", "1-2");
        assert_eq!(mode, PromptModus::Normal);
        assert_eq!(placeholder, "y");
        assert_eq!(text_out, "1-2");
    }

    #[test]
    fn deletion_keeps_decimal_literal_collision_as_word_deletion() {
        let session = bootstrap_prompt_session();
        let (placeholder, mode, text_out) =
            session.delete_before_storage_commands("1 2 3", "2");
        assert_eq!(mode, PromptModus::Normal);
        assert_eq!(placeholder, "1 3");
        assert_eq!(text_out, "");
    }
}

// Stage 16: concrete PromptSession compatibility surface.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileHistory {
    pub filename: String,
    pub strings: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Style {
    pub class_names: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToggleHistory {
    pub enabled: bool,
    pub history: FileHistory,
}

impl Default for ToggleHistory {
    fn default() -> Self {
        Self { enabled: true, history: FileHistory::default() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptSession {
    pub state: PromptTextState,
    pub history: ToggleHistory,
    pub logging_enabled: bool,
}

pub fn new_session(text: impl Into<String>) -> PromptSession {
    PromptSession { state: PromptTextState::new(text), history: ToggleHistory::default(), logging_enabled: true }
}

pub fn __init__(text: impl Into<String>) -> PromptSession {
    new_session(text)
}

pub fn from_dict(values: &BTreeMap<String, String>) -> PromptSession {
    let mut session = new_session(values.get("text").cloned().unwrap_or_default());
    if let Some(platzhalter) = values.get("platzhalter") {
        session.state.set_platzhalter(platzhalter.clone());
    }
    session
}

pub fn add_to_history(session: &mut PromptSession, value: impl Into<String>) {
    session.history.history.strings.push(value.into());
}

pub fn get_strings(history: &FileHistory) -> Vec<String> {
    history.strings.clone()
}

pub fn append_string(history: &mut FileHistory, value: impl Into<String>) {
    history.strings.push(value.into());
}

pub fn enable_logging(session: &mut PromptSession) { session.logging_enabled = true; }
pub fn disable_logging(session: &mut PromptSession) { session.logging_enabled = false; }

#[allow(non_snake_case)]
pub fn hasWithoutABC(state: &PromptTextState, has_set: &BTreeSet<String>) -> bool {
    state.has_without_abc(has_set)
}

pub fn prompt_input(session: &PromptSession) -> String { session.state.text.clone() }
pub fn prompt(session: &PromptSession) -> String { session.state.text.clone() }
pub fn text(session: &PromptSession) -> String { session.state.text.clone() }
pub fn platzhalter(session: &PromptSession) -> String { session.state.platzhalter.clone() }
pub fn liste(session: &PromptSession) -> Vec<String> { session.state.liste.clone() }
#[allow(non_snake_case)]
pub fn listeS(session: &PromptSession) -> Vec<String> { session.state.liste_s.clone() }
#[allow(non_snake_case)]
pub fn listeE(session: &PromptSession) -> Vec<String> { session.state.liste_e.clone() }
pub fn e(session: &PromptSession) -> Vec<String> { session.state.e.clone() }
pub fn menge(session: &PromptSession) -> BTreeSet<String> { session.state.menge.clone() }
#[allow(non_snake_case)]
pub fn mengeE(session: &PromptSession) -> BTreeSet<String> { session.state.menge_e.clone() }
#[allow(non_snake_case)]
pub fn befehlDavor(session: &PromptSession) -> String { session.state.befehl_davor.clone() }

// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "FileHistory",
    "Style",
    "ToggleHistory",
    "__init__",
    "add_to_history",
    "append_string",
    "disable_logging",
    "enable_logging",
    "from_dict",
    "get_strings",
    "hasWithoutABC",
    "new_session",
    "prompt_input",
    "PromptSession",
    "befehlDavor",
    "e",
    "liste",
    "listeE",
    "listeS",
    "menge",
    "mengeE",
    "platzhalter",
    "prompt",
    "text",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
