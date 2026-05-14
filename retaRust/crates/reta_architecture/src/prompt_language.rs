//! Prompt-language helpers transcompiled from
//! `python_arch_reference/reta_architecture/prompt_language.py`.
//!
//! This module gives `rretaPrompt` a typed Rust owner for the bracket-aware
//! token splitters and command classifiers.  Completion and execution can now
//! depend on Rust prompt semantics instead of re-implementing the Python helper
//! functions in each frontend.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::row_ranges::{bootstrap_row_range_morphisms, str_as_generator_to_set, RowRangeMorphismBundle};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PromptModus {
    Normal,
    Speichern,
    LoeschenStart,
    SpeicherungAusgaben,
    LoeschenSelect,
    SpeicherungAusgabenMitZusatz,
    AusgabeSelektiv,
}

impl PromptModus {
    pub const fn py_value(self) -> i64 {
        match self {
            Self::Normal => 0,
            Self::Speichern => 1,
            Self::LoeschenStart => 2,
            Self::SpeicherungAusgaben => 3,
            Self::LoeschenSelect => 4,
            Self::SpeicherungAusgabenMitZusatz => 5,
            Self::AusgabeSelektiv => 6,
        }
    }
}

pub fn custom_split(text: &str) -> Vec<String> {
    let mut stack = Vec::<char>::new();
    let mut result = Vec::new();
    let mut start = 0usize;
    for (index, ch) in text.char_indices() {
        match ch {
            '(' | '{' | '[' => stack.push(ch),
            ')' | '}' | ']' => {
                stack.pop();
            }
            _ if ch.is_whitespace() && stack.is_empty() => {
                result.push(text[start..index].to_string());
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    if start < text.len() {
        result.push(text[start..].to_string());
    }
    result
}

pub fn custom_split2(input: &str, delimiter: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut temp = String::new();
    let mut stack = Vec::<char>::new();
    for ch in input.chars() {
        match ch {
            '(' | '{' | '[' => {
                stack.push(ch);
                temp.push(ch);
            }
            ')' | '}' | ']' => {
                if !stack.is_empty() {
                    stack.pop();
                }
                temp.push(ch);
            }
            _ if ch == delimiter && stack.is_empty() => {
                result.push(temp);
                temp = String::new();
            }
            _ => temp.push(ch),
        }
    }
    if !temp.is_empty() {
        result.push(temp);
    }
    result
}

pub fn verkuerze_dict(dictionary: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let mut seen_values = BTreeSet::new();
    for (key, value) in dictionary {
        if seen_values.insert(value.clone()) {
            out.insert(key.clone(), value.clone());
        }
    }
    out
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLanguageBundle {
    pub row_ranges: RowRangeMorphismBundle,
    pub not_parameter_values: Vec<String>,
    pub gebrochen_erlaubte_zahlen: BTreeSet<i64>,
    pub wahl15: BTreeMap<String, String>,
    pub wahl16: BTreeMap<String, String>,
    pub short_command_letters: BTreeSet<String>,
}

impl Default for PromptLanguageBundle {
    fn default() -> Self {
        Self {
            row_ranges: bootstrap_row_range_morphisms(None),
            not_parameter_values: vec![
                "-zeilen".to_string(),
                "-spalten".to_string(),
                "-ausgabe".to_string(),
                "-kombination".to_string(),
                "--art".to_string(),
                "--alles".to_string(),
                "--vorhervonausschnitt".to_string(),
            ],
            gebrochen_erlaubte_zahlen: BTreeSet::new(),
            wahl15: BTreeMap::new(),
            wahl16: BTreeMap::new(),
            short_command_letters: BTreeSet::new(),
        }
    }
}

impl PromptLanguageBundle {
    pub fn snapshot(&self) -> PromptLanguageSnapshot {
        PromptLanguageSnapshot {
            class: "PromptLanguageBundle".to_string(),
            not_parameter_values_len: self.not_parameter_values.len(),
            gebrochen_erlaubte_zahlen_len: self.gebrochen_erlaubte_zahlen.len(),
            wahl15_len: self.wahl15.len(),
            wahl16_len: self.wahl16.len(),
            short_command_letters: self.short_command_letters.iter().cloned().collect(),
        }
    }

    pub fn str_as_generator_to_numset(&self, text: &str) -> Option<BTreeSet<i64>> {
        str_as_generator_to_set(text)
    }

    pub fn is_zeilenangabe_between_commas(&self, text: &str) -> bool {
        self.row_ranges.is_integer_token(text)
            || self.str_as_generator_to_numset(text).is_some()
            || text.get(1..).and_then(|tail| self.str_as_generator_to_numset(tail)).is_some()
    }

    pub fn is_zeilenbruch_between_commas(&self, text: &str) -> bool {
        self.row_ranges.is_fraction_token(text)
    }

    pub fn is_reta_parameter(&self, text: &str) -> bool {
        !text.is_empty()
            && text.starts_with('-')
            && !self.is_fraction_or_integer_comma_list(text)
            && self
                .not_parameter_values
                .iter()
                .any(|candidate| candidate.split('=').next() == text.split('=').next())
    }

    pub fn is_15_or_16_command(&self, text: &str) -> bool {
        if let Some(rest) = text.strip_prefix("15_") {
            return rest.is_empty() || self.wahl15.contains_key(rest);
        }
        if let Some(rest) = text.strip_prefix("16_") {
            if rest.is_empty() || self.wahl16.contains_key(rest) {
                return true;
            }
            if let Some(after_15) = rest.strip_prefix("15") {
                return after_15.is_empty()
                    || after_15
                        .strip_prefix('_')
                        .is_some_and(|wahl| self.wahl15.contains_key(wahl));
            }
        }
        false
    }

    pub fn verify_bruch_nganz_zahl_between_commas(&self, text: &str) -> FractionOrIntegerCheck {
        FractionOrIntegerCheck {
            text: text.to_string(),
            is_fraction_range: self.is_zeilenbruch_between_commas(text),
            is_integer_or_set_range: self.is_zeilenangabe_between_commas(text),
        }
    }

    pub fn verify_bruch_nganz_zahl_comma_list(&self, text: &str) -> Vec<FractionOrIntegerCheck> {
        self.row_ranges
            .syntax
            .split_comma_list(text)
            .into_iter()
            .map(|part| self.verify_bruch_nganz_zahl_between_commas(&part))
            .collect()
    }

    fn is_fraction_or_integer_comma_list(&self, text: &str) -> bool {
        let parts = self.row_ranges.syntax.split_comma_list(text);
        !parts.is_empty()
            && parts.iter().all(|part| {
                self.is_zeilenbruch_between_commas(part)
                    || self.is_zeilenangabe_between_commas(part)
            })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLanguageSnapshot {
    pub class: String,
    pub not_parameter_values_len: usize,
    pub gebrochen_erlaubte_zahlen_len: usize,
    pub wahl15_len: usize,
    pub wahl16_len: usize,
    pub short_command_letters: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FractionOrIntegerCheck {
    pub text: String,
    pub is_fraction_range: bool,
    pub is_integer_or_set_range: bool,
}

impl FractionOrIntegerCheck {
    pub fn accepted(&self) -> bool {
        self.is_fraction_range || self.is_integer_or_set_range
    }
}

pub fn bootstrap_prompt_language() -> PromptLanguageBundle {
    PromptLanguageBundle::default()
}

pub fn is_reta_parameter(text: &str) -> bool {
    bootstrap_prompt_language().is_reta_parameter(text)
}

pub fn is_15_or_16_command(text: &str) -> bool {
    bootstrap_prompt_language().is_15_or_16_command(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_split_respects_brackets() {
        assert_eq!(custom_split("reta a {1 2} b"), vec!["reta", "a", "{1 2}", "b"]);
        assert_eq!(custom_split2("1,{2,3},4", ','), vec!["1", "{2,3}", "4"]);
    }

    #[test]
    fn prompt_parameter_checks_use_row_ranges() {
        let language = bootstrap_prompt_language();
        assert!(language.is_zeilenangabe_between_commas("1-3"));
        assert!(language.is_zeilenbruch_between_commas("1/2-3/4"));
        assert!(language.is_reta_parameter("-zeilen"));
        assert!(!language.is_reta_parameter("-1-3"));
    }
}

// Stage 16: Python-name prompt language wrappers.
pub fn __post_init__() -> PromptLanguageBundle {
    bootstrap_prompt_language()
}

pub fn _default_prompt_language() -> PromptLanguageBundle {
    bootstrap_prompt_language()
}

pub fn _is_zeilenbruch_or_ganzzahl_angabe(text: &str) -> bool {
    let bundle = bootstrap_prompt_language();
    bundle.verify_bruch_nganz_zahl_between_commas(text).accepted()
}

pub fn befehle() -> Vec<String> {
    vec!["reta".to_string(), "shell".to_string(), "python".to_string(), "math".to_string()]
}

pub fn befehle2() -> Vec<String> {
    { let bundle = bootstrap_prompt_language(); bundle.wahl15.keys().chain(bundle.wahl16.keys()).cloned().collect() }
}

#[allow(non_snake_case)]
pub fn is15or16command(text: &str) -> bool {
    is_15_or_16_command(text)
}

#[allow(non_snake_case)]
pub fn isReTaParameter(text: &str) -> bool {
    is_reta_parameter(text)
}

pub fn is_zeilenangabe_between_kommas(text: &str) -> bool {
    bootstrap_prompt_language().is_zeilenangabe_between_commas(text)
}

pub fn is_zeilenbruch_between_kommas(text: &str) -> bool {
    bootstrap_prompt_language().is_zeilenbruch_between_commas(text)
}

#[allow(non_snake_case)]
pub fn stextFromKleinKleinKleinBefehl(text: &str) -> String {
    custom_split(text).join(" ")
}

// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "__post_init__",
    "_default_prompt_language",
    "_is_zeilenbruch_or_ganzzahl_angabe",
    "befehle",
    "befehle2",
    "is15or16command",
    "isReTaParameter",
    "is_zeilenangabe_between_kommas",
    "is_zeilenbruch_between_kommas",
    "stextFromKleinKleinKleinBefehl",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
