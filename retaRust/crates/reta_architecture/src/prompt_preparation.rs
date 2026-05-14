//! Prompt preparation morphisms transcompiled from
//! `python_arch_reference/reta_architecture/prompt_preparation.py`.
//!
//! This module normalizes prompt text into Reta command tokens.  It keeps the
//! same responsibilities as the Python layer: rotate misplaced `reta` commands,
//! resolve wildcard/regex-like prompt tokens against known domains, and attach
//! row-range shortcuts to `-zeilen` arguments before execution.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::completion_runtime::{bootstrap_completion_runtime, CompletionRuntimeBundle};
use crate::prompt_language::{custom_split, PromptModus};
use crate::prompt_session::{bootstrap_prompt_session, PromptSessionBundle, PromptTextState};
use crate::row_ranges::{bootstrap_row_range_morphisms, RowRangeMorphismBundle};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptPreparationBundle {
    pub completion_runtime: CompletionRuntimeBundle,
    pub prompt_session: PromptSessionBundle,
    pub row_ranges: RowRangeMorphismBundle,
    pub replacements: BTreeMap<String, String>,
    pub befehle_beenden: BTreeSet<String>,
}

impl PromptPreparationBundle {
    pub fn snapshot(&self) -> PromptPreparationSnapshot {
        PromptPreparationSnapshot {
            class: "PromptPreparationBundle".to_string(),
            command_rotator: "rotate_where_reta_command".to_string(),
            regex_rewriter: "regex_replace".to_string(),
            output_preparer: "prepare_large_output".to_string(),
            cached_parameter_value_domains: self.parameter_value_domains_snapshot(),
            beenden_commands_len: self.befehle_beenden.len(),
        }
    }

    pub fn rotate_where_reta_command(
        &self,
        text1: &str,
        text2: &str,
        text3: &[String],
        prompt_mode: PromptModus,
    ) -> (String, String, Vec<String>) {
        rotate_where_reta_command(text1, text2, text3, prompt_mode)
    }

    pub fn regex_replace(&self, text_state: &PromptTextState) -> Vec<String> {
        regex_replace(
            text_state,
            &self.parameter_value_domains(),
            &self.completion_runtime,
        )
    }

    pub fn prepare_large_output(
        &self,
        placeholder: &str,
        prompt_mode: PromptModus,
        prompt_mode2: PromptModus,
        prompt_mode_last: PromptModus,
        text: &str,
        text_dazu0: &[String],
    ) -> PreparedPromptOutput {
        prepare_large_output(
            self,
            placeholder,
            prompt_mode,
            prompt_mode2,
            prompt_mode_last,
            text,
            text_dazu0,
        )
    }

    pub fn parameter_value_domains(&self) -> BTreeMap<String, BTreeMap<String, BTreeSet<String>>> {
        let mut domains = BTreeMap::new();
        domains.insert(
            "zeilen".to_string(),
            BTreeMap::from([
                ("alles".to_string(), BTreeSet::from([String::new()])),
                (
                    "vorhervonausschnitt".to_string(),
                    HUNDERT_STRINGS
                        .iter()
                        .map(|item| item.to_string())
                        .collect(),
                ),
                (
                    "oberesmaximum".to_string(),
                    HUNDERT_STRINGS
                        .iter()
                        .map(|item| item.to_string())
                        .collect(),
                ),
                (
                    "typ".to_string(),
                    self.completion_runtime
                        .zeilen_typen
                        .iter()
                        .cloned()
                        .collect(),
                ),
                (
                    "zeit".to_string(),
                    self.completion_runtime
                        .zeilen_zeit
                        .iter()
                        .cloned()
                        .collect(),
                ),
            ]),
        );
        domains.insert(
            "spalten".to_string(),
            self.completion_runtime
                .spalten_dict
                .iter()
                .map(|(key, values)| (key.clone(), values.iter().cloned().collect()))
                .collect(),
        );
        domains.insert(
            "kombination".to_string(),
            self.completion_runtime
                .kombi_value_options
                .iter()
                .map(|(key, values)| (key.clone(), values.iter().cloned().collect()))
                .collect(),
        );
        domains.insert(
            "ausgabe".to_string(),
            BTreeMap::from([
                (
                    "art".to_string(),
                    self.completion_runtime
                        .ausgabe_art
                        .iter()
                        .cloned()
                        .collect(),
                ),
                (
                    "breite".to_string(),
                    HUNDERT_STRINGS
                        .iter()
                        .map(|item| item.to_string())
                        .collect(),
                ),
                (
                    "breiten".to_string(),
                    HUNDERT_STRINGS
                        .iter()
                        .map(|item| item.to_string())
                        .collect(),
                ),
            ]),
        );
        domains
    }

    fn parameter_value_domains_snapshot(&self) -> BTreeMap<String, usize> {
        self.parameter_value_domains()
            .into_iter()
            .map(|(key, values)| (key, values.len()))
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptPreparationSnapshot {
    pub class: String,
    pub command_rotator: String,
    pub regex_rewriter: String,
    pub output_preparer: String,
    pub cached_parameter_value_domains: BTreeMap<String, usize>,
    pub beenden_commands_len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PreparedPromptOutput {
    pub is_pure_only_reta_cmd: bool,
    pub brueche: Vec<String>,
    pub zahlen_angaben_c: String,
    pub chains: Vec<String>,
    pub max_num: i64,
    pub tokens: Vec<String>,
    pub zahlen_angaben: Vec<String>,
    pub if_kurz_kurz: bool,
}

impl PreparedPromptOutput {
    pub fn as_legacy_tuple_shape(
        &self,
    ) -> (
        &bool,
        &Vec<String>,
        &String,
        &Vec<String>,
        &i64,
        &Vec<String>,
        &Vec<String>,
        &bool,
    ) {
        (
            &self.is_pure_only_reta_cmd,
            &self.brueche,
            &self.zahlen_angaben_c,
            &self.chains,
            &self.max_num,
            &self.tokens,
            &self.zahlen_angaben,
            &self.if_kurz_kurz,
        )
    }
}

const HUNDERT_STRINGS: [&str; 100] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
    "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32",
    "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48",
    "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63", "64",
    "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80",
    "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96",
    "97", "98", "99",
];

pub fn bootstrap_prompt_preparation() -> PromptPreparationBundle {
    let prompt_session = bootstrap_prompt_session();
    PromptPreparationBundle {
        completion_runtime: bootstrap_completion_runtime(),
        prompt_session,
        row_ranges: bootstrap_row_range_morphisms(None),
        replacements: BTreeMap::new(),
        befehle_beenden: BTreeSet::from(["exit".to_string(), "quit".to_string()]),
    }
}

pub fn rotate_where_reta_command(
    text1: &str,
    text2: &str,
    text3: &[String],
    _prompt_mode: PromptModus,
) -> (String, String, Vec<String>) {
    if text2.starts_with("reta") && !text1.starts_with("reta") && !text3.is_empty() {
        (text2.to_string(), text1.to_string(), custom_split(text2))
    } else {
        (text1.to_string(), text2.to_string(), text3.to_vec())
    }
}

pub fn regex_replace(
    text_state: &PromptTextState,
    domains: &BTreeMap<String, BTreeMap<String, BTreeSet<String>>>,
    completion_runtime: &CompletionRuntimeBundle,
) -> Vec<String> {
    if !text_state
        .liste
        .iter()
        .any(|item| item.contains("r\"") || item.contains('*'))
    {
        return text_state.liste.clone();
    }
    let if_reta = text_state.liste.first().is_some_and(|item| item == "reta");
    let mut out = Vec::new();
    let mut last_main_parameter: Option<String> = None;
    for token in &text_state.liste {
        if token == "reta" {
            out.push(token.clone());
            continue;
        }
        if let Some(main) = token
            .strip_prefix('-')
            .filter(|rest| !rest.starts_with('-'))
        {
            last_main_parameter = Some(main.to_string());
            if token.contains('*') || token.starts_with("r\"") {
                out.extend(match_main_parameters(token, completion_runtime));
            } else {
                out.push(token.clone());
            }
            continue;
        }
        if let Some((parameter, value_probe)) = token
            .strip_prefix("--")
            .and_then(|tail| tail.split_once('='))
        {
            if token.contains('*') || token.contains("r\"") {
                if let Some(main) = &last_main_parameter {
                    out.extend(match_parameter_values(
                        main,
                        parameter,
                        value_probe,
                        domains,
                    ));
                }
            } else {
                out.push(token.clone());
            }
            continue;
        }
        if token.contains('*') || token.starts_with("r\"") {
            if if_reta {
                if let Some(main) = &last_main_parameter {
                    out.extend(match_parameter_names(main, token, domains));
                } else {
                    out.extend(match_main_parameters(token, completion_runtime));
                }
            } else {
                out.extend(match_prompt_commands(token, completion_runtime));
            }
        } else {
            out.push(token.clone());
        }
    }
    if out.is_empty() {
        text_state.liste.clone()
    } else {
        out
    }
}

fn match_prompt_commands(probe: &str, completion_runtime: &CompletionRuntimeBundle) -> Vec<String> {
    completion_runtime
        .befehle2_list
        .iter()
        .filter(|candidate| simple_pattern_match(probe, candidate))
        .cloned()
        .collect()
}

fn match_main_parameters(probe: &str, completion_runtime: &CompletionRuntimeBundle) -> Vec<String> {
    let normalized = probe.trim_start_matches('-');
    completion_runtime
        .main_parameters
        .iter()
        .filter(|candidate| simple_pattern_match(normalized, candidate))
        .map(|candidate| format!("-{candidate}"))
        .collect()
}

fn match_parameter_names(
    main: &str,
    probe: &str,
    domains: &BTreeMap<String, BTreeMap<String, BTreeSet<String>>>,
) -> Vec<String> {
    let normalized = probe.trim_start_matches('-');
    domains
        .get(main)
        .into_iter()
        .flat_map(|domain| domain.keys())
        .filter(|candidate| simple_pattern_match(normalized, candidate))
        .map(|candidate| format!("--{candidate}"))
        .collect()
}

fn match_parameter_values(
    main: &str,
    parameter: &str,
    value_probe: &str,
    domains: &BTreeMap<String, BTreeMap<String, BTreeSet<String>>>,
) -> Vec<String> {
    domains
        .get(main)
        .and_then(|domain| domain.get(parameter))
        .into_iter()
        .flat_map(|values| values.iter())
        .filter(|candidate| simple_pattern_match(value_probe, candidate))
        .map(|candidate| format!("--{parameter}={candidate}"))
        .collect()
}

pub fn simple_pattern_match(pattern: &str, candidate: &str) -> bool {
    let pattern = pattern.trim();
    if pattern == "*" || pattern == "--*" || pattern == "r\"(.*)\"" {
        return true;
    }
    let stripped = pattern
        .strip_prefix("r\"")
        .and_then(|item| item.strip_suffix('"'))
        .unwrap_or(pattern)
        .replace(".*", "")
        .replace('^', "")
        .replace('$', "");
    if stripped.is_empty() {
        return true;
    }
    let wildcard_parts = stripped
        .split('*')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if wildcard_parts.is_empty() {
        return true;
    }
    let mut remainder = candidate;
    for part in wildcard_parts {
        if let Some(index) = remainder.find(part) {
            remainder = &remainder[index + part.len()..];
        } else {
            return false;
        }
    }
    true
}

pub fn prepare_large_output(
    bundle: &PromptPreparationBundle,
    placeholder: &str,
    prompt_mode: PromptModus,
    prompt_mode2: PromptModus,
    prompt_mode_last: PromptModus,
    text: &str,
    text_dazu0: &[String],
) -> PreparedPromptOutput {
    let mut txt = bundle.prompt_session.new_text_state(text);
    txt.set_platzhalter(placeholder);
    if prompt_mode2 == PromptModus::AusgabeSelektiv && prompt_mode_last == PromptModus::Normal {
        let mut combined = text_dazu0.to_vec();
        combined.extend(txt.liste.clone());
        txt.set_liste(combined);
    }

    let numeric_tokens = txt
        .liste
        .iter()
        .filter(|token| bundle.row_ranges.is_row_range(token))
        .cloned()
        .collect::<Vec<_>>();
    let max_num = txt
        .liste
        .iter()
        .filter_map(|token| token.parse::<i64>().ok())
        .max()
        .unwrap_or(1024);

    if prompt_mode == PromptModus::Normal
        && txt.platzhalter.starts_with("reta")
        && !numeric_tokens.is_empty()
        && !txt.liste.iter().any(|token| token == "-zeilen")
    {
        txt.liste.push("-zeilen".to_string());
        txt.liste.push(vorher_von_ausschnitt_or_zaehlung(
            &txt,
            &numeric_tokens.join(","),
        ));
        txt.set_liste(txt.liste.clone());
    }

    if !txt.liste.first().is_some_and(|token| token == "reta") {
        let mut dedup = txt.menge.iter().cloned().collect::<Vec<_>>();
        dedup.sort();
        txt.set_liste(dedup);
    }
    let replaced = bundle.regex_replace(&txt);
    txt.set_liste(replaced);

    PreparedPromptOutput {
        is_pure_only_reta_cmd: txt.liste.first().is_some_and(|token| token == "reta"),
        brueche: Vec::new(),
        zahlen_angaben_c: numeric_tokens.join(","),
        chains: Vec::new(),
        max_num,
        tokens: txt.liste,
        zahlen_angaben: numeric_tokens,
        if_kurz_kurz: false,
    }
}

pub fn vorher_von_ausschnitt_or_zaehlung(txt: &PromptTextState, range_spec: &str) -> String {
    if txt.menge.contains("range") || txt.menge.contains("R") {
        format!("--zaehlung={range_spec}")
    } else {
        format!("--vorhervonausschnitt={range_spec}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotates_reta_command_like_python_helper() {
        let result = rotate_where_reta_command(
            "abc",
            "reta -zeilen",
            &["x".to_string()],
            PromptModus::Normal,
        );
        assert_eq!(result.0, "reta -zeilen");
        assert_eq!(result.1, "abc");
    }

    #[test]
    fn wildcard_matches_known_main_parameter() {
        let bundle = bootstrap_prompt_preparation();
        let state = PromptTextState::new("reta -*");
        let replaced = bundle.regex_replace(&state);
        assert!(replaced.iter().any(|item| item == "-zeilen"));
    }

    #[test]
    fn preparation_attaches_row_range_to_reta_placeholder() {
        let bundle = bootstrap_prompt_preparation();
        let prepared = bundle.prepare_large_output(
            "reta -spalten --religion=1",
            PromptModus::Normal,
            PromptModus::Normal,
            PromptModus::Normal,
            "1-3",
            &[],
        );
        assert!(prepared
            .tokens
            .iter()
            .any(|item| item.starts_with("--vorhervonausschnitt=")));
    }
}
