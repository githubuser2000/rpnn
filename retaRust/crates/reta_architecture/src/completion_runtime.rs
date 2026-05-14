//! Completion runtime vocabulary skeleton transcompiled from
//! `python_arch_reference/reta_architecture/completion_runtime.py`.
//!
//! The Python builder still extracts the full i18n vocabulary.  This Rust
//! module owns the typed completion bundle shape and deterministic sort key so
//! prompt frontends can move onto Rust one section at a time.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct CompletionSortKey {
    pub group: u8,
    pub key: String,
}

pub fn sort_completion_key(key: &str) -> CompletionSortKey {
    let trimmed = key.trim().to_string();
    let group = if trimmed.len() == 1 {
        7
    } else if trimmed == "15" || trimmed.starts_with("15_") {
        8
    } else if trimmed == "16" || trimmed.starts_with("16_") {
        9
    } else if trimmed.starts_with('1') {
        11
    } else if matches!(trimmed.as_str(), "hilfe" | "help" | "kurzbefehle" | "absicht") {
        0
    } else if matches!(trimmed.as_str(), "universum" | "thomas" | "befehle" | "groesse") {
        1
    } else if matches!(trimmed.as_str(), "reta" | "bewusstsein" | "geist" | "emotion" | "impulse") {
        2
    } else if matches!(trimmed.as_str(), "loggen" | "nichtloggen" | "exit" | "quit") {
        3
    } else {
        6
    };
    CompletionSortKey { group, key: trimmed }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompletionRuntimeBundle {
    pub befehle: Vec<String>,
    pub befehle2: BTreeSet<String>,
    pub befehle2_list: Vec<String>,
    pub haupt_for_neben: Vec<String>,
    pub haupt_for_neben_set: BTreeSet<String>,
    pub ausgabe_art: Vec<String>,
    pub ausgabe_paras: Vec<String>,
    pub kombi_main_paras: Vec<String>,
    pub main_parameters: Vec<String>,
    pub spalten: Vec<String>,
    pub spalten_dict: BTreeMap<String, Vec<String>>,
    pub zeilen_paras: Vec<String>,
    pub zeilen_typen: Vec<String>,
    pub zeilen_zeit: Vec<String>,
    pub zeilen_typen_b: Vec<String>,
    pub kombi_value_options: BTreeMap<String, Vec<String>>,
}

impl Default for CompletionRuntimeBundle {
    fn default() -> Self {
        let mut befehle = vec![
            "hilfe".to_string(),
            "reta".to_string(),
            "exit".to_string(),
            "quit".to_string(),
            "15_".to_string(),
            "16_".to_string(),
        ];
        befehle.sort_by_key(|item| sort_completion_key(item));
        let befehle2_list = befehle.clone();
        Self {
            befehle: befehle.clone(),
            befehle2: befehle.iter().cloned().collect(),
            befehle2_list,
            haupt_for_neben: vec!["zeilen".to_string(), "spalten".to_string(), "ausgabe".to_string()],
            haupt_for_neben_set: BTreeSet::from(["zeilen".to_string(), "spalten".to_string(), "ausgabe".to_string()]),
            ausgabe_art: vec!["bbcode".to_string(), "html".to_string(), "csv".to_string(), "shell".to_string(), "markdown".to_string(), "emacs".to_string(), "nichts".to_string()],
            ausgabe_paras: vec!["art".to_string(), "breite".to_string(), "breiten".to_string()],
            kombi_main_paras: vec!["galaxie".to_string(), "universum".to_string()],
            main_parameters: vec!["zeilen".to_string(), "spalten".to_string(), "kombination".to_string(), "ausgabe".to_string()],
            spalten: Vec::new(),
            spalten_dict: BTreeMap::new(),
            zeilen_paras: vec!["alles".to_string(), "vorhervonausschnitt".to_string(), "oberesmaximum".to_string()],
            zeilen_typen: vec!["sonne".to_string(), "planet".to_string(), "mond".to_string()],
            zeilen_zeit: vec!["heute".to_string(), "gestern".to_string(), "morgen".to_string()],
            zeilen_typen_b: Vec::new(),
            kombi_value_options: BTreeMap::new(),
        }
    }
}

impl CompletionRuntimeBundle {
    pub fn start_commands(&self, include_numeric_shortcuts: bool) -> Vec<String> {
        let mut commands = self.befehle.clone();
        if include_numeric_shortcuts {
            for item in ["15_", "16_"] {
                if !commands.iter().any(|existing| existing == item) {
                    commands.push(item.to_string());
                }
            }
        }
        commands.sort_by_key(|item| sort_completion_key(item));
        commands
    }

    pub fn snapshot(&self) -> CompletionRuntimeSnapshot {
        CompletionRuntimeSnapshot {
            befehle_len: self.befehle.len(),
            befehle2_len: self.befehle2.len(),
            haupt_for_neben_len: self.haupt_for_neben.len(),
            spalten_dict_keys: self.spalten_dict.len(),
            kombi_option_keys: self.kombi_value_options.keys().cloned().collect(),
            start_commands_with_numeric_shortcuts: self
                .start_commands(true)
                .into_iter()
                .take(10)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompletionRuntimeSnapshot {
    pub befehle_len: usize,
    pub befehle2_len: usize,
    pub haupt_for_neben_len: usize,
    pub spalten_dict_keys: usize,
    pub kombi_option_keys: Vec<String>,
    pub start_commands_with_numeric_shortcuts: Vec<String>,
}

pub fn bootstrap_completion_runtime() -> CompletionRuntimeBundle {
    CompletionRuntimeBundle::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_key_keeps_numeric_shortcuts_late() {
        assert!(sort_completion_key("hilfe") < sort_completion_key("15_"));
        assert_eq!(sort_completion_key("x").group, 7);
    }

    #[test]
    fn bundle_starts_with_prompt_commands() {
        let bundle = bootstrap_completion_runtime();
        assert!(bundle.start_commands(true).contains(&"15_".to_string()));
        assert!(bundle.snapshot().befehle_len > 0);
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "CompletionRuntimeBuilder",
    "__init__",
    "build",
    "program",
    "vocabulary",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
