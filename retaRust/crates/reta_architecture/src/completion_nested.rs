//! Nested prompt-completion morphisms transcompiled from
//! `python_arch_reference/reta_architecture/completion_nested.py`.
//!
//! The Python module owns the historic `nestedAlx.NestedCompleter` state
//! machine.  This Rust module keeps the same architectural cut: a prompt text
//! is a local section, a cursor prefix selects an open set in the prompt command
//! topology, and completion candidates are glued from runtime vocabularies.
//! It is deliberately prompt-toolkit-free so `rretaPrompt` can use it from all
//! frontends.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::completion_runtime::{bootstrap_completion_runtime, CompletionRuntimeBundle};
use crate::completion_word::{word_completion_matches, CompletionCandidate, PromptDocument, WordCompletionOptions};
use crate::prompt_language::custom_split;

pub const HUNDERT: [&str; 100] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19",
    "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39",
    "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
    "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79",
    "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96", "97", "98", "99",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComplSitua {
    HauptPara,
    ZeilenPara,
    Value,
    NeitherNor,
    RetaAnfang,
    Unbekannt,
    SpaltenPara,
    KomiPara,
    KombiMetaPara,
    AusgabePara,
    SpaltenValPara,
    ZeilenValPara,
    KombiValPara,
    AusgabeValPara,
    BefehleNichtReta,
}

impl ComplSitua {
    pub const fn py_value(self) -> i64 {
        match self {
            Self::HauptPara => 0,
            Self::ZeilenPara => 1,
            Self::Value => 3,
            Self::NeitherNor => 4,
            Self::RetaAnfang => 5,
            Self::Unbekannt => 6,
            Self::SpaltenPara => 7,
            Self::KomiPara => 8,
            Self::KombiMetaPara => 9,
            Self::AusgabePara => 10,
            Self::SpaltenValPara => 11,
            Self::ZeilenValPara => 12,
            Self::KombiValPara => 13,
            Self::AusgabeValPara => 14,
            Self::BefehleNichtReta => 15,
        }
    }

    pub const fn is_value_context(self) -> bool {
        matches!(
            self,
            Self::SpaltenValPara | Self::ZeilenValPara | Self::KombiValPara | Self::AusgabeValPara | Self::Value
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NestedCompletionRuntimeView {
    pub ausgabe_art: Vec<String>,
    pub ausgabe_paras: Vec<String>,
    pub befehle: Vec<String>,
    pub befehle2: BTreeSet<String>,
    pub befehle2_list: Vec<String>,
    pub haupt_for_neben: Vec<String>,
    pub haupt_for_neben_set: BTreeSet<String>,
    pub kombi_main_paras: Vec<String>,
    pub main_parameters: Vec<String>,
    pub spalten: Vec<String>,
    pub spalten_dict: BTreeMap<String, Vec<String>>,
    pub zeilen_paras: Vec<String>,
    pub zeilen_typen: Vec<String>,
    pub zeilen_typen_b: Vec<String>,
    pub zeilen_zeit: Vec<String>,
    pub kombi_value_options: BTreeMap<String, Vec<String>>,
}

impl NestedCompletionRuntimeView {
    pub fn from_runtime(runtime: &CompletionRuntimeBundle) -> Self {
        Self {
            ausgabe_art: runtime.ausgabe_art.clone(),
            ausgabe_paras: runtime.ausgabe_paras.clone(),
            befehle: runtime.befehle.clone(),
            befehle2: runtime.befehle2.clone(),
            befehle2_list: runtime.befehle2_list.clone(),
            haupt_for_neben: runtime.haupt_for_neben.clone(),
            haupt_for_neben_set: runtime.haupt_for_neben_set.clone(),
            kombi_main_paras: runtime.kombi_main_paras.clone(),
            main_parameters: runtime.main_parameters.clone(),
            spalten: runtime.spalten.clone(),
            spalten_dict: runtime.spalten_dict.clone(),
            zeilen_paras: runtime.zeilen_paras.clone(),
            zeilen_typen: runtime.zeilen_typen.clone(),
            zeilen_typen_b: runtime.zeilen_typen_b.clone(),
            zeilen_zeit: runtime.zeilen_zeit.clone(),
            kombi_value_options: runtime.kombi_value_options.clone(),
        }
    }

    pub fn snapshot(&self) -> NestedCompletionRuntimeSnapshot {
        NestedCompletionRuntimeSnapshot {
            ausgabe_art_len: self.ausgabe_art.len(),
            befehle_len: self.befehle.len(),
            befehle2_len: self.befehle2.len(),
            haupt_for_neben_len: self.haupt_for_neben.len(),
            spalten_len: self.spalten.len(),
            zeilen_paras_len: self.zeilen_paras.len(),
            kombi_option_keys: self.kombi_value_options.keys().cloned().collect(),
        }
    }
}

impl Default for NestedCompletionRuntimeView {
    fn default() -> Self {
        Self::from_runtime(&bootstrap_completion_runtime())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NestedCompletionRuntimeSnapshot {
    pub ausgabe_art_len: usize,
    pub befehle_len: usize,
    pub befehle2_len: usize,
    pub haupt_for_neben_len: usize,
    pub spalten_len: usize,
    pub zeilen_paras_len: usize,
    pub kombi_option_keys: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NestedCompletionOptions {
    pub options: BTreeMap<String, Option<ComplSitua>>,
    pub options_standard: BTreeMap<String, Option<ComplSitua>>,
    pub situation: ComplSitua,
    pub last_string: String,
    pub para_nach_para: BTreeMap<String, ComplSitua>,
    pub fuzzy: bool,
}

impl NestedCompletionOptions {
    pub fn reta_start(runtime: &CompletionRuntimeBundle) -> Self {
        let mut options = BTreeMap::new();
        options.insert("reta".to_string(), Some(ComplSitua::RetaAnfang));
        for command in runtime.start_commands(true) {
            options.insert(command, Some(ComplSitua::BefehleNichtReta));
        }
        Self {
            options,
            options_standard: BTreeMap::new(),
            situation: ComplSitua::RetaAnfang,
            last_string: String::new(),
            para_nach_para: BTreeMap::new(),
            fuzzy: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NestedCompletionContext {
    pub text: String,
    pub cursor_position: usize,
    pub tokens: Vec<String>,
    pub situation: ComplSitua,
    pub prefix: String,
    pub current_main_parameter: Option<String>,
    pub current_value_parameter: Option<String>,
    pub equality_value_prefix: Option<String>,
}

impl NestedCompletionContext {
    pub fn from_text(text: &str, runtime: &NestedCompletionRuntimeView) -> Self {
        let cursor_position = text.chars().count();
        let tokens = custom_split(text.trim());
        classify_nested_completion_context(text, cursor_position, tokens, runtime)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NestedCompletionCandidate {
    pub text: String,
    pub start_position: isize,
    pub situation: ComplSitua,
    pub source: String,
}

impl From<NestedCompletionCandidate> for CompletionCandidate {
    fn from(value: NestedCompletionCandidate) -> Self {
        Self {
            text: value.text.clone(),
            start_position: value.start_position,
            display: value.text,
            display_meta: value.source,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NestedCompletionMorphismBundle {
    pub runtime_view: NestedCompletionRuntimeView,
    pub activated_stage: u32,
    pub compatibility_names: Vec<String>,
}

impl NestedCompletionMorphismBundle {
    pub fn classify(&self, text: &str) -> NestedCompletionContext {
        NestedCompletionContext::from_text(text, &self.runtime_view)
    }

    pub fn complete(&self, text: &str) -> Vec<NestedCompletionCandidate> {
        let context = self.classify(text);
        nested_completion_candidates(&context, &self.runtime_view, true)
    }

    pub fn match_text_alx(&self, candidate: &str, prefix: &str, fuzzy: bool) -> bool {
        match_text_alx(candidate, prefix, fuzzy)
    }

    pub fn snapshot(&self) -> NestedCompletionSnapshot {
        NestedCompletionSnapshot {
            class: "NestedCompletionMorphismBundle".to_string(),
            stage: self.activated_stage,
            runtime: self.runtime_view.snapshot(),
            compatibility_names: self.compatibility_names.clone(),
            morphisms: vec![
                "classify_nested_completion_context".to_string(),
                "match_text_alx".to_string(),
                "nested_completion_candidates".to_string(),
                "para_transition".to_string(),
                "gleich_komma_transition".to_string(),
            ],
            category: "NestedPromptCompletionCategory".to_string(),
            natural_transformation: "NestedCompleterToArchitectureTransformation".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NestedCompletionSnapshot {
    pub class: String,
    pub stage: u32,
    pub runtime: NestedCompletionRuntimeSnapshot,
    pub compatibility_names: Vec<String>,
    pub morphisms: Vec<String>,
    pub category: String,
    pub natural_transformation: String,
}

pub fn bootstrap_nested_completion_morphisms() -> NestedCompletionMorphismBundle {
    let runtime = bootstrap_completion_runtime();
    NestedCompletionMorphismBundle {
        runtime_view: NestedCompletionRuntimeView::from_runtime(&runtime),
        activated_stage: 41,
        compatibility_names: vec![
            "NestedCompleter".to_string(),
            "nestedAlx.NestedCompleter".to_string(),
            "completion_nested.ArchitectureNestedCompleter".to_string(),
        ],
    }
}

pub fn match_text_alx(candidate: &str, prefix: &str, fuzzy: bool) -> bool {
    if prefix.is_empty() {
        return true;
    }
    if fuzzy {
        word_completion_matches(candidate, prefix, true, true)
    } else {
        word_completion_matches(candidate, prefix, true, false)
    }
}

pub fn classify_nested_completion_context(
    text: &str,
    cursor_position: usize,
    tokens: Vec<String>,
    runtime: &NestedCompletionRuntimeView,
) -> NestedCompletionContext {
    let before_cursor = text.chars().take(cursor_position).collect::<String>();
    let prefix = before_cursor
        .split_whitespace()
        .last()
        .unwrap_or_default()
        .to_string();
    let first = tokens.first().map(String::as_str).unwrap_or_default();
    let mut current_main_parameter = None;
    let mut current_value_parameter = None;

    for token in &tokens {
        if let Some(main) = token.strip_prefix('-').filter(|rest| !rest.starts_with('-')) {
            current_main_parameter = Some(main.to_string());
        }
        if let Some(secondary) = token.strip_prefix("--") {
            current_value_parameter = secondary.split('=').next().map(str::to_string);
        }
    }

    let mut situation = if tokens.is_empty() || (tokens.len() == 1 && "reta".starts_with(first)) {
        ComplSitua::RetaAnfang
    } else if first != "reta" {
        ComplSitua::BefehleNichtReta
    } else {
        ComplSitua::HauptPara
    };

    let mut equality_value_prefix = None;
    if first == "reta" {
        if let Some(last) = tokens.last() {
            if let Some((parameter, value_prefix)) = last.strip_prefix("--").and_then(|tail| tail.split_once('=')) {
                current_value_parameter = Some(parameter.to_string());
                equality_value_prefix = Some(value_prefix.rsplit(',').next().unwrap_or_default().to_string());
                situation = match current_main_parameter.as_deref() {
                    Some("spalten") => ComplSitua::SpaltenValPara,
                    Some("zeilen") => ComplSitua::ZeilenValPara,
                    Some("kombination") => ComplSitua::KombiValPara,
                    Some("ausgabe") => ComplSitua::AusgabeValPara,
                    _ => ComplSitua::Value,
                };
            } else if last.starts_with("--") {
                situation = match current_main_parameter.as_deref() {
                    Some("spalten") => ComplSitua::SpaltenPara,
                    Some("zeilen") => ComplSitua::ZeilenPara,
                    Some("kombination") => ComplSitua::KomiPara,
                    Some("ausgabe") => ComplSitua::AusgabePara,
                    _ => ComplSitua::HauptPara,
                };
            } else if last.starts_with('-') && !last.starts_with("--") {
                situation = ComplSitua::HauptPara;
            } else if runtime.haupt_for_neben_set.contains(last) {
                current_main_parameter = Some(last.clone());
                situation = ComplSitua::HauptPara;
            }
        }
    }

    NestedCompletionContext {
        text: text.to_string(),
        cursor_position,
        tokens,
        situation,
        prefix,
        current_main_parameter,
        current_value_parameter,
        equality_value_prefix,
    }
}

pub fn nested_completion_candidates(
    context: &NestedCompletionContext,
    runtime: &NestedCompletionRuntimeView,
    fuzzy: bool,
) -> Vec<NestedCompletionCandidate> {
    let prefix = context
        .equality_value_prefix
        .as_deref()
        .unwrap_or(context.prefix.as_str());
    let mut values = candidates_for_situation(context, runtime);
    values.sort();
    values.dedup();
    values
        .into_iter()
        .filter(|candidate| match_text_alx(candidate, prefix, fuzzy))
        .map(|candidate| NestedCompletionCandidate {
            text: candidate,
            start_position: -(prefix.chars().count() as isize),
            situation: context.situation,
            source: format!("{:?}", context.situation),
        })
        .collect()
}

pub fn candidates_for_situation(
    context: &NestedCompletionContext,
    runtime: &NestedCompletionRuntimeView,
) -> Vec<String> {
    match context.situation {
        ComplSitua::RetaAnfang => {
            let mut commands = vec!["reta".to_string()];
            commands.extend(runtime.befehle2_list.clone());
            commands
        }
        ComplSitua::BefehleNichtReta => runtime.befehle2_list.clone(),
        ComplSitua::HauptPara => runtime
            .main_parameters
            .iter()
            .map(|item| format!("-{item}"))
            .collect(),
        ComplSitua::SpaltenPara => runtime
            .spalten
            .iter()
            .chain(runtime.spalten_dict.keys())
            .map(|item| format!("--{item}"))
            .collect(),
        ComplSitua::ZeilenPara => runtime
            .zeilen_paras
            .iter()
            .map(|item| format!("--{item}"))
            .collect(),
        ComplSitua::KomiPara | ComplSitua::KombiMetaPara => runtime
            .kombi_main_paras
            .iter()
            .map(|item| format!("--{item}"))
            .collect(),
        ComplSitua::AusgabePara => runtime
            .ausgabe_paras
            .iter()
            .map(|item| format!("--{item}"))
            .collect(),
        ComplSitua::SpaltenValPara => context
            .current_value_parameter
            .as_ref()
            .and_then(|key| runtime.spalten_dict.get(key))
            .cloned()
            .unwrap_or_else(|| runtime.spalten.clone()),
        ComplSitua::ZeilenValPara => {
            let mut out = runtime.zeilen_typen.clone();
            out.extend(runtime.zeilen_zeit.clone());
            out.extend(runtime.zeilen_typen_b.clone());
            out.extend(HUNDERT.iter().map(|item| item.to_string()));
            out
        }
        ComplSitua::KombiValPara => context
            .current_value_parameter
            .as_ref()
            .and_then(|key| runtime.kombi_value_options.get(key))
            .cloned()
            .unwrap_or_else(|| runtime.kombi_value_options.values().flatten().cloned().collect()),
        ComplSitua::AusgabeValPara => runtime.ausgabe_art.clone(),
        ComplSitua::Value | ComplSitua::NeitherNor | ComplSitua::Unbekannt => Vec::new(),
    }
}

pub fn prompt_document_for_nested_text(text: &str) -> PromptDocument {
    PromptDocument::new(text)
}

pub fn word_options_for_nested(fuzzy: bool) -> WordCompletionOptions {
    WordCompletionOptions {
        ignore_case: true,
        match_middle: fuzzy,
        ..WordCompletionOptions::default()
    }
}



#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompleteEvent {
    pub completion_requested: bool,
}

impl Default for CompleteEvent {
    fn default() -> Self {
        Self { completion_requested: true }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Completion {
    pub text: String,
    pub start_position: isize,
    pub display_meta: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub text: String,
    pub cursor_position: usize,
}

impl Document {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let cursor_position = text.chars().count();
        Self { text, cursor_position }
    }

    pub fn text_before_cursor(&self) -> String {
        self.text.chars().take(self.cursor_position).collect()
    }
}

pub trait Completer {
    fn get_completions(&self, document: &Document, event: &CompleteEvent) -> Vec<Completion>;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FuzzyWordCompleter {
    pub words: Vec<String>,
    pub fuzzy: bool,
}

impl FuzzyWordCompleter {
    pub fn new(words: Vec<String>, fuzzy: bool) -> Self {
        Self { words, fuzzy }
    }
}

impl Completer for FuzzyWordCompleter {
    fn get_completions(&self, document: &Document, _event: &CompleteEvent) -> Vec<Completion> {
        let prefix = document.text_before_cursor().split_whitespace().last().unwrap_or_default().to_string();
        self.words
            .iter()
            .filter(|word| match_text_alx(word, &prefix, self.fuzzy))
            .map(|word| Completion { text: word.clone(), start_position: -(prefix.chars().count() as isize), display_meta: "FuzzyWordCompleter".to_string() })
            .collect()
    }
}

pub fn __init__() -> NestedCompletionMorphismBundle {
    bootstrap_nested_completion_morphisms()
}

pub fn __post_init__() -> NestedCompletionMorphismBundle {
    bootstrap_nested_completion_morphisms()
}

pub fn __repr__(context: &NestedCompletionContext) -> String {
    format!("NestedCompletionContext({:?}, prefix={})", context.situation, context.prefix)
}

pub fn __eq__(left: &NestedCompletionContext, right: &NestedCompletionContext) -> bool {
    left == right
}

pub fn __hash__(context: &NestedCompletionContext) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    context.text.hash(&mut hasher);
    context.cursor_position.hash(&mut hasher);
    context.situation.hash(&mut hasher);
    hasher.finish()
}

pub fn _default_i18n() -> Vec<String> {
    vec!["de".to_string(), "en".to_string(), "meta".to_string()]
}

pub fn _default_row_range_morphisms() -> Vec<String> {
    vec!["is_row_range".to_string(), "range_to_numbers".to_string(), "str_as_generator_to_set".to_string()]
}

pub fn _default_prompt_language_refs() -> Vec<String> {
    vec!["prompt_language".to_string(), "custom_split".to_string(), "is_reta_parameter".to_string()]
}

pub fn _default_completion_runtime() -> CompletionRuntimeBundle {
    bootstrap_completion_runtime()
}

pub fn _child(name: &str, situation: ComplSitua) -> (String, ComplSitua) {
    (name.to_string(), situation)
}

pub fn options_sync(runtime: &NestedCompletionRuntimeView) -> BTreeMap<String, Option<ComplSitua>> {
    let mut out = BTreeMap::new();
    for item in &runtime.main_parameters {
        out.insert(format!("-{item}"), Some(ComplSitua::HauptPara));
    }
    out
}

pub fn __set_options(options: &mut NestedCompletionOptions, values: BTreeMap<String, Option<ComplSitua>>) {
    options.options = values;
}

pub fn gleich_komma_kombi(prefix: &str, runtime: &NestedCompletionRuntimeView) -> Vec<String> {
    runtime.kombi_value_options.values().flatten().filter(|value| match_text_alx(value, prefix, true)).cloned().collect()
}

pub fn gleich_komma_zeilen(prefix: &str, runtime: &NestedCompletionRuntimeView) -> Vec<String> {
    runtime.zeilen_typen.iter().chain(runtime.zeilen_zeit.iter()).filter(|value| match_text_alx(value, prefix, true)).cloned().collect()
}

pub fn gleich_komma_ausg(prefix: &str, runtime: &NestedCompletionRuntimeView) -> Vec<String> {
    runtime.ausgabe_art.iter().filter(|value| match_text_alx(value, prefix, true)).cloned().collect()
}

pub fn gleich_komma_spalten(prefix: &str, runtime: &NestedCompletionRuntimeView) -> Vec<String> {
    runtime.spalten.iter().filter(|value| match_text_alx(value, prefix, true)).cloned().collect()
}

pub fn para_kombination(runtime: &NestedCompletionRuntimeView) -> Vec<String> {
    runtime.kombi_main_paras.iter().map(|item| format!("--{item}")).collect()
}

pub fn para_ausgabe(runtime: &NestedCompletionRuntimeView) -> Vec<String> {
    runtime.ausgabe_paras.iter().map(|item| format!("--{item}")).collect()
}

pub fn para_spalten(runtime: &NestedCompletionRuntimeView) -> Vec<String> {
    runtime.spalten.iter().map(|item| format!("--{item}")).collect()
}

pub fn para_zeilen(runtime: &NestedCompletionRuntimeView) -> Vec<String> {
    runtime.zeilen_paras.iter().map(|item| format!("--{item}")).collect()
}

pub fn create_completer(words: Vec<String>, fuzzy: bool) -> FuzzyWordCompleter {
    FuzzyWordCompleter::new(words, fuzzy)
}

pub fn get_completions<C: Completer>(completer: &C, document: &Document, event: &CompleteEvent) -> Vec<Completion> {
    completer.get_completions(document, event)
}

pub fn text_before_cursor(document: &Document) -> String {
    document.text_before_cursor()
}

pub fn sample_options() -> Vec<String> {
    let bundle = bootstrap_nested_completion_morphisms();
    bundle.complete("reta -").into_iter().take(8).map(|candidate| candidate.text).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_reta_main_parameter_context() {
        let bundle = bootstrap_nested_completion_morphisms();
        let context = bundle.classify("reta -s");
        assert_eq!(context.situation, ComplSitua::HauptPara);
        let completions = bundle.complete("reta -s");
        assert!(completions.iter().any(|item| item.text == "-spalten"));
    }

    #[test]
    fn classifies_ausgabe_value_context() {
        let bundle = bootstrap_nested_completion_morphisms();
        let context = bundle.classify("reta -ausgabe --art=h");
        assert_eq!(context.situation, ComplSitua::AusgabeValPara);
        assert_eq!(context.equality_value_prefix.as_deref(), Some("h"));
        assert!(bundle.complete("reta -ausgabe --art=h").iter().any(|item| item.text == "html"));
    }

    #[test]
    fn fuzzy_match_uses_word_completion_semantics() {
        assert!(match_text_alx("religion", "lig", true));
        assert!(!match_text_alx("religion", "lig", false));
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// Marker-only names still need semantic Rust implementation before activation.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "ArchitectureNestedCompleter",
    "runtime_view",
    "text",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}

// Stage 16 small-surface concrete wrappers.
pub type ArchitectureNestedCompleter = NestedCompletionMorphismBundle;

pub fn runtime_view(bundle: &NestedCompletionMorphismBundle) -> &NestedCompletionRuntimeView {
    &bundle.runtime_view
}

pub fn text(document: &Document) -> &str {
    &document.text
}
