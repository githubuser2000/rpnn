//! CLI parameter runtime transcompiled from
//! `python_arch_reference/reta_architecture/parameter_runtime.py`.
//!
//! The full Python module still contains large legacy side-effect blocks.  This
//! Rust layer owns the typed parse surface used by `rreta` and future table
//! ports: main-command context, sub-parameter tokenization, output-mode
//! extraction and upper-limit inference from row ranges.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::output_syntax::OutputMode;
use crate::row_ranges::{bootstrap_row_range_morphisms, RowRangeMorphismBundle};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MainParameter {
    Zeilen,
    Spalten,
    Kombination,
    Ausgabe,
    Debug,
    Help,
    Unknown(String),
}

impl MainParameter {
    pub fn from_cli(value: &str) -> Option<Self> {
        match value.trim() {
            "zeilen" | "z" => Some(Self::Zeilen),
            "spalten" | "s" => Some(Self::Spalten),
            "kombination" | "kombi" | "kombinationen" | "k" => Some(Self::Kombination),
            "ausgabe" | "a" => Some(Self::Ausgabe),
            "debug" => Some(Self::Debug),
            "h" | "help" | "hilfe" => Some(Self::Help),
            "nichts" | "nothing" => None,
            other if !other.is_empty() => Some(Self::Unknown(other.to_string())),
            _ => None,
        }
    }

    pub fn canonical_name(&self) -> String {
        match self {
            Self::Zeilen => "zeilen".to_string(),
            Self::Spalten => "spalten".to_string(),
            Self::Kombination => "kombination".to_string(),
            Self::Ausgabe => "ausgabe".to_string(),
            Self::Debug => "debug".to_string(),
            Self::Help => "help".to_string(),
            Self::Unknown(value) => value.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParameterTokenKind {
    ProgramName,
    MainCommand,
    SubParameter,
    LanguageSwitch,
    IgnoredNothing,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterToken {
    pub index: usize,
    pub raw: String,
    pub kind: ParameterTokenKind,
    pub main_context: Option<MainParameter>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub value_items: Vec<String>,
    pub negated_value: bool,
}

impl ParameterToken {
    fn program_name(index: usize, raw: &str) -> Self {
        Self {
            index,
            raw: raw.to_string(),
            kind: ParameterTokenKind::ProgramName,
            main_context: None,
            key: None,
            value: None,
            value_items: Vec::new(),
            negated_value: false,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterCommandSets {
    pub param_lines: BTreeSet<String>,
    pub rows_as_numbers: BTreeSet<i64>,
    pub rows_of_combi: BTreeSet<i64>,
    pub spaltenreihenfolgeundnurdiese: Vec<i64>,
    pub puniverseprims_only: BTreeSet<i64>,
    pub gener_rows: BTreeSet<i64>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterParseResult {
    pub tokens: Vec<ParameterToken>,
    pub main_context_history: Vec<MainParameter>,
    pub selected_output_mode: Option<OutputMode>,
    pub upper_limit: Option<i64>,
    pub command_sets: ParameterCommandSets,
}

impl ParameterParseResult {
    pub fn has_main(&self, main: MainParameter) -> bool {
        self.main_context_history.iter().any(|item| item == &main)
    }

    pub fn subparameters_for(&self, main: MainParameter) -> Vec<&ParameterToken> {
        self.tokens
            .iter()
            .filter(|token| token.kind == ParameterTokenKind::SubParameter)
            .filter(|token| token.main_context.as_ref() == Some(&main))
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterRuntimeSnapshot {
    pub class: String,
    pub column_function: String,
    pub width_function: String,
    pub parse_function: String,
    pub upper_limit_argument_function: String,
    pub upper_limit_aggregate_function: String,
    pub upper_limit_apply_function: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterRuntimeBundle {
    pub row_ranges: RowRangeMorphismBundle,
    pub main_commands: Vec<String>,
}

impl Default for ParameterRuntimeBundle {
    fn default() -> Self {
        Self {
            row_ranges: bootstrap_row_range_morphisms(None),
            main_commands: vec![
                "zeilen".to_string(),
                "spalten".to_string(),
                "kombination".to_string(),
                "ausgabe".to_string(),
                "debug".to_string(),
                "help".to_string(),
            ],
        }
    }
}

impl ParameterRuntimeBundle {
    pub fn snapshot(&self) -> ParameterRuntimeSnapshot {
        ParameterRuntimeSnapshot {
            class: "ParameterRuntimeBundle".to_string(),
            column_function: "produce_all_spalten_numbers".to_string(),
            width_function: "apply_width_parameter".to_string(),
            parse_function: "parameters_to_commands_and_numbers".to_string(),
            upper_limit_argument_function: "upper_limit_values_for_argument".to_string(),
            upper_limit_aggregate_function: "upper_limit_from_arguments".to_string(),
            upper_limit_apply_function: "apply_upper_limit_argument".to_string(),
        }
    }

    pub fn parse_cli_args<S: AsRef<str>>(&self, args: &[S]) -> ParameterParseResult {
        let mut result = ParameterParseResult::default();
        let mut active_main: Option<MainParameter> = None;
        let mut upper_limit_values = Vec::new();

        for (index, arg) in args.iter().enumerate() {
            let raw = arg.as_ref();
            if index == 0 {
                result.tokens.push(ParameterToken::program_name(index, raw));
                continue;
            }

            if raw.starts_with("--") {
                let body = &raw[2..];
                let (key, value) = split_key_value(body);
                let value_items = value
                    .as_deref()
                    .map(split_comma_values)
                    .unwrap_or_default();
                let negated_value = value
                    .as_ref()
                    .is_some_and(|item| item.trim_start().starts_with('-'));
                let token = ParameterToken {
                    index,
                    raw: raw.to_string(),
                    kind: ParameterTokenKind::SubParameter,
                    main_context: active_main.clone(),
                    key: Some(key.clone()),
                    value: value.clone(),
                    value_items,
                    negated_value,
                };

                self.apply_token_to_command_sets(&token, &mut result.command_sets);
                if let Some(mode) = output_mode_from_token(&token) {
                    result.selected_output_mode = Some(mode);
                }
                upper_limit_values.extend(self.upper_limit_values_for_argument(raw));
                result.tokens.push(token);
                continue;
            }

            if raw.starts_with('-') && raw.len() > 1 {
                let body = &raw[1..];
                if body.starts_with("sprachen=") || body.starts_with("sprache=") {
                    result.tokens.push(ParameterToken {
                        index,
                        raw: raw.to_string(),
                        kind: ParameterTokenKind::LanguageSwitch,
                        main_context: active_main.clone(),
                        key: body.split('=').next().map(str::to_string),
                        value: body.split_once('=').map(|(_key, value)| value.to_string()),
                        value_items: Vec::new(),
                        negated_value: false,
                    });
                    continue;
                }
                if matches!(body, "nichts" | "nothing") {
                    result.tokens.push(ParameterToken {
                        index,
                        raw: raw.to_string(),
                        kind: ParameterTokenKind::IgnoredNothing,
                        main_context: active_main.clone(),
                        key: Some(body.to_string()),
                        value: None,
                        value_items: Vec::new(),
                        negated_value: false,
                    });
                    continue;
                }
                let main = MainParameter::from_cli(body).unwrap_or(MainParameter::Unknown(body.to_string()));
                active_main = Some(main.clone());
                result.main_context_history.push(main.clone());
                result.tokens.push(ParameterToken {
                    index,
                    raw: raw.to_string(),
                    kind: ParameterTokenKind::MainCommand,
                    main_context: active_main.clone(),
                    key: Some(main.canonical_name()),
                    value: None,
                    value_items: Vec::new(),
                    negated_value: false,
                });
                continue;
            }

            result.tokens.push(ParameterToken {
                index,
                raw: raw.to_string(),
                kind: ParameterTokenKind::Unknown,
                main_context: active_main.clone(),
                key: None,
                value: None,
                value_items: Vec::new(),
                negated_value: false,
            });
        }

        result.upper_limit = upper_limit_values.into_iter().max();
        result
    }

    pub fn parameters_to_commands_and_numbers<S: AsRef<str>>(
        &self,
        args: &[S],
    ) -> ParameterCommandSets {
        self.parse_cli_args(args).command_sets
    }

    pub fn upper_limit_values_for_argument(&self, arg: &str) -> Vec<i64> {
        let Some(body) = arg.strip_prefix("--") else {
            return Vec::new();
        };
        let (key, value) = split_key_value(body);
        match (key.as_str(), value.as_deref()) {
            ("oberesmaximum", Some(value)) => value
                .trim()
                .parse::<i64>()
                .map(|parsed| vec![parsed])
                .unwrap_or_default(),
            ("vorhervonausschnitt", Some(value)) => self
                .row_ranges
                .range_to_numbers(value, false, 0, false)
                .into_iter()
                .map(|number| (number + 1).max(1024))
                .collect(),
            _ => Vec::new(),
        }
    }

    pub fn upper_limit_from_arguments<S: AsRef<str>>(
        &self,
        args: &[S],
        current_highest: Option<i64>,
    ) -> Option<i64> {
        let mut values = current_highest.into_iter().collect::<Vec<_>>();
        for arg in args {
            values.extend(self.upper_limit_values_for_argument(arg.as_ref()));
        }
        values.into_iter().max()
    }

    fn apply_token_to_command_sets(&self, token: &ParameterToken, sets: &mut ParameterCommandSets) {
        match token.main_context.as_ref() {
            Some(MainParameter::Zeilen) => apply_row_token(self, token, sets),
            Some(MainParameter::Ausgabe) => apply_output_token(self, token, sets),
            Some(MainParameter::Spalten) => apply_column_token(self, token, sets),
            Some(MainParameter::Kombination) => apply_kombi_token(self, token, sets),
            _ => {}
        }
    }
}

pub fn bootstrap_parameter_runtime() -> ParameterRuntimeBundle {
    ParameterRuntimeBundle::default()
}

fn split_key_value(body: &str) -> (String, Option<String>) {
    match body.split_once('=') {
        Some((key, value)) => (key.to_string(), Some(value.to_string())),
        None => (body.to_string(), None),
    }
}

fn split_comma_values(value: &str) -> Vec<String> {
    value
        .split(',')
        .filter(|item| !item.is_empty())
        .map(|item| item.to_string())
        .collect()
}

fn output_mode_from_token(token: &ParameterToken) -> Option<OutputMode> {
    if token.key.as_deref() == Some("art") {
        token.value.as_deref().and_then(OutputMode::from_name)
    } else {
        None
    }
}

fn apply_row_token(bundle: &ParameterRuntimeBundle, token: &ParameterToken, sets: &mut ParameterCommandSets) {
    match (token.key.as_deref(), token.value.as_deref()) {
        (Some("alles"), _) => {
            sets.param_lines.insert("all".to_string());
        }
        (Some("zeit"), Some(value)) => {
            for item in split_comma_values(value) {
                match item.trim_start_matches('-') {
                    "heute" => {
                        sets.param_lines.insert("=".to_string());
                    }
                    "gestern" => {
                        sets.param_lines.insert("<".to_string());
                    }
                    "morgen" => {
                        sets.param_lines.insert(">".to_string());
                    }
                    _ => {}
                }
            }
        }
        (Some("typ"), Some(value)) => {
            for item in split_comma_values(value) {
                let cleaned = item.trim_start_matches('-');
                if matches!(cleaned, "sonne" | "schwarzesonne" | "planet" | "mond" | "SonneMitMondanteil") {
                    sets.param_lines.insert(cleaned.to_string());
                }
            }
        }
        (Some("vorhervonausschnitt"), Some(value)) => {
            for number in bundle.row_ranges.range_to_numbers(value, false, 0, false) {
                sets.param_lines.insert(format!("a{number}"));
                sets.rows_as_numbers.insert(number);
            }
        }
        (Some("zaehlung"), Some(value)) => {
            for number in bundle.row_ranges.range_to_numbers(value, false, 0, false) {
                sets.param_lines.insert(format!("n{number}"));
                sets.rows_as_numbers.insert(number);
            }
        }
        _ => {}
    }
}

fn apply_output_token(bundle: &ParameterRuntimeBundle, token: &ParameterToken, sets: &mut ParameterCommandSets) {
    match (token.key.as_deref(), token.value.as_deref()) {
        (Some("spaltenreihenfolgeundnurdiese"), Some(value)) => {
            sets.spaltenreihenfolgeundnurdiese = bundle
                .row_ranges
                .range_to_numbers(value, false, 0, false)
                .into_iter()
                .collect();
        }
        _ => {}
    }
}

fn apply_column_token(bundle: &ParameterRuntimeBundle, token: &ParameterToken, sets: &mut ParameterCommandSets) {
    if let Some(value) = token.value.as_deref() {
        for number in bundle.row_ranges.range_to_numbers(value, false, 0, false) {
            sets.rows_as_numbers.insert(number);
        }
    }
}

fn apply_kombi_token(bundle: &ParameterRuntimeBundle, token: &ParameterToken, sets: &mut ParameterCommandSets) {
    if let Some(value) = token.value.as_deref() {
        for number in bundle.row_ranges.range_to_numbers(value, false, 0, false) {
            sets.rows_of_combi.insert(number);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_tracks_main_contexts_and_output_mode() {
        let runtime = bootstrap_parameter_runtime();
        let parsed = runtime.parse_cli_args(&[
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=1-2",
            "-ausgabe",
            "--art=html",
        ]);
        assert!(parsed.has_main(MainParameter::Zeilen));
        assert_eq!(parsed.selected_output_mode, Some(OutputMode::Html));
        assert!(parsed.command_sets.param_lines.contains("a1"));
    }

    #[test]
    fn upper_limit_matches_python_cases() {
        let runtime = bootstrap_parameter_runtime();
        assert_eq!(runtime.upper_limit_values_for_argument("--oberesmaximum=55"), vec![55]);
        assert_eq!(runtime.upper_limit_from_arguments(&["--vorhervonausschnitt=3"], Some(10)), Some(1024));
    }
}
