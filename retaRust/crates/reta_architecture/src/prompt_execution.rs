//! Prompt execution planning transcompiled from
//! `python_arch_reference/reta_architecture/prompt_execution.py`.
//!
//! The Python module still performs the side-effectful CLI execution.  This
//! Rust layer turns prepared prompt state into deterministic execution plans:
//! fraction/range management, helper-command expansion, `reta` argv shaping
//! and row-range maximum calculation.  Frontends can inspect this plan before
//! handing rendering to the existing parity path.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::completion_runtime::{bootstrap_completion_runtime, CompletionRuntimeBundle};
use crate::prompt_language::{bootstrap_prompt_language, is_reta_parameter, PromptLanguageBundle};
use crate::prompt_preparation::{vorher_von_ausschnitt_or_zaehlung, PreparedPromptOutput};
use crate::prompt_session::PromptTextState;
use crate::row_ranges::{bootstrap_row_range_morphisms, RowRangeMorphismBundle};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptExecutionBundle {
    pub row_ranges: RowRangeMorphismBundle,
    pub prompt_language: PromptLanguageBundle,
    pub completion_runtime: CompletionRuntimeBundle,
    pub gebrochen_erlaubte_zahlen: BTreeSet<i64>,
    pub wahl15: BTreeMap<String, String>,
    pub wahl16: BTreeMap<String, String>,
    pub befehle: Vec<String>,
}

impl PromptExecutionBundle {
    pub fn snapshot(&self) -> PromptExecutionSnapshot {
        PromptExecutionSnapshot {
            class: "PromptExecutionBundle".to_string(),
            command_runner: "plan_prompt_execution".to_string(),
            fraction_manager: "fraction_range_management".to_string(),
            reta_executor: "reta_execute_plan".to_string(),
            prompt_language_class: "PromptLanguageBundle".to_string(),
            known_commands_len: self.befehle.len(),
        }
    }

    pub fn plan_prompt_execution(
        &self,
        prepared: &PreparedPromptOutput,
        text_state: &PromptTextState,
    ) -> PromptExecutionPlan {
        plan_prompt_execution(self, prepared, text_state)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptExecutionSnapshot {
    pub class: String,
    pub command_runner: String,
    pub fraction_manager: String,
    pub reta_executor: String,
    pub prompt_language_class: String,
    pub known_commands_len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FractionRangeManagementResult {
    pub reciprocal_fractions: Vec<String>,
    pub zahlen_bereich_c: String,
    pub zahlen_reihe_keine_wteiler: String,
    pub full_block_is_number_range_and_fraction: bool,
    pub ranges_brueche: BTreeMap<String, Vec<i64>>,
    pub ranges_brueche_reverse: BTreeMap<String, Vec<i64>>,
    pub saw_number_specs: bool,
    pub tokens: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptExecutionPlan {
    pub is_pure_reta: bool,
    pub cmd_gave_output: bool,
    pub logging_switch: bool,
    pub reta_argv: Vec<String>,
    pub helper_outputs: Vec<String>,
    pub side_effects: Vec<String>,
    pub max_argument: Option<String>,
    pub number_range_argument: Option<String>,
    pub print_command_again: bool,
}

impl PromptExecutionPlan {
    pub fn empty() -> Self {
        Self {
            is_pure_reta: false,
            cmd_gave_output: false,
            logging_switch: false,
            reta_argv: Vec::new(),
            helper_outputs: Vec::new(),
            side_effects: Vec::new(),
            max_argument: None,
            number_range_argument: None,
            print_command_again: false,
        }
    }
}

pub fn bootstrap_prompt_execution() -> PromptExecutionBundle {
    let completion_runtime = bootstrap_completion_runtime();
    PromptExecutionBundle {
        row_ranges: bootstrap_row_range_morphisms(None),
        prompt_language: bootstrap_prompt_language(),
        gebrochen_erlaubte_zahlen: BTreeSet::new(),
        wahl15: BTreeMap::new(),
        wahl16: BTreeMap::new(),
        befehle: completion_runtime.befehle.clone(),
        completion_runtime,
    }
}

pub fn another_oberes_maximum(
    row_ranges: &RowRangeMorphismBundle,
    zahlen_bereich_c: &str,
    max_num: i64,
    max1024: i64,
) -> String {
    let parsed = row_ranges.range_to_numbers(zahlen_bereich_c, false, 0, false);
    let max_num2 = parsed.iter().next_back().copied().unwrap_or(max_num);
    format!("--oberesmaximum={}", max_num.max(max_num2).max(max1024) + 1)
}

pub fn return_only_paras_as_list(tokens: &[String]) -> Vec<String> {
    tokens
        .iter()
        .filter(|token| is_reta_parameter(token))
        .cloned()
        .collect()
}

pub fn greater_and_less_than_anchor(a: &BTreeSet<i64>, b: &BTreeSet<i64>) -> (BTreeSet<i64>, BTreeSet<i64>) {
    if b.is_empty() {
        return (a.clone(), a.clone());
    }
    let max_b = *b.iter().next_back().unwrap_or(&0);
    let min_b = *b.iter().next().unwrap_or(&0);
    let greater = a.iter().copied().filter(|value| *value > max_b).collect();
    let lower = a.iter().copied().filter(|value| *value < min_b).collect();
    (greater, lower)
}

pub fn get_dict_limited_by_key_list<K: Ord + Clone, V: Clone>(
    dict: &BTreeMap<K, V>,
    keys: &BTreeSet<K>,
) -> BTreeMap<K, V> {
    keys.iter()
        .filter_map(|key| dict.get(key).map(|value| (key.clone(), value.clone())))
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BruchPart {
    Fraction(String, String),
    Text(String),
}

pub fn bruch_spalt(text: &str) -> Vec<BruchPart> {
    if !text.contains('/') {
        return Vec::new();
    }
    let mut out = Vec::new();
    let chars = text.chars().collect::<Vec<_>>();
    let mut index = 0usize;
    while index < chars.len() {
        if chars[index].is_ascii_digit() {
            let start = index;
            while index < chars.len() && chars[index].is_ascii_digit() {
                index += 1;
            }
            if index < chars.len() && chars[index] == '/' {
                let numerator = chars[start..index].iter().collect::<String>();
                index += 1;
                let denominator_start = index;
                while index < chars.len() && chars[index].is_ascii_digit() {
                    index += 1;
                }
                if denominator_start < index {
                    out.push(BruchPart::Fraction(
                        numerator,
                        chars[denominator_start..index].iter().collect(),
                    ));
                    continue;
                }
            }
            out.push(BruchPart::Text(chars[start..index].iter().collect()));
        } else {
            let start = index;
            while index < chars.len() && !chars[index].is_ascii_digit() {
                index += 1;
            }
            out.push(BruchPart::Text(chars[start..index].iter().collect()));
        }
    }
    if out.iter().any(|part| matches!(part, BruchPart::Fraction(_, _))) { out } else { Vec::new() }
}

pub fn create_ranges_for_bruch_parts(parts: &[BruchPart]) -> (Vec<i64>, String) {
    let mut numbers = Vec::new();
    let mut text = String::new();
    for part in parts {
        match part {
            BruchPart::Fraction(numerator, denominator) => {
                if let Ok(number) = numerator.parse::<i64>() {
                    numbers.push(number);
                }
                text.push_str(denominator);
            }
            BruchPart::Text(value) => text.push_str(value),
        }
    }
    (numbers, text)
}

pub fn vorher_von_ausschnitt_oder_zaehlung(text_state: &PromptTextState, bereichs_angabe: &str) -> String {
    vorher_von_ausschnitt_or_zaehlung(text_state, bereichs_angabe)
}

pub fn fraction_range_management(
    row_ranges: &RowRangeMorphismBundle,
    zahlen_bereich_c: &str,
    tokens: &[String],
    zahlen_angaben: &[String],
) -> FractionRangeManagementResult {
    let mut reciprocal_fractions = Vec::new();
    let mut ranges_brueche = BTreeMap::new();
    let mut ranges_brueche_reverse = BTreeMap::new();
    for token in tokens {
        let parts = bruch_spalt(token);
        if parts.is_empty() {
            continue;
        }
        let (numbers, suffix) = create_ranges_for_bruch_parts(&parts);
        if !numbers.is_empty() {
            reciprocal_fractions.push(token.clone());
            ranges_brueche.insert(token.clone(), numbers.clone());
            ranges_brueche_reverse.insert(suffix, numbers);
        }
    }
    let parsed_numbers = if zahlen_bereich_c.is_empty() {
        BTreeSet::new()
    } else {
        row_ranges.range_to_numbers(zahlen_bereich_c, false, 0, false)
    };
    FractionRangeManagementResult {
        reciprocal_fractions,
        zahlen_bereich_c: zahlen_bereich_c.to_string(),
        zahlen_reihe_keine_wteiler: parsed_numbers.iter().map(i64::to_string).collect::<Vec<_>>().join(","),
        full_block_is_number_range_and_fraction: !parsed_numbers.is_empty() && !ranges_brueche.is_empty(),
        ranges_brueche,
        ranges_brueche_reverse,
        saw_number_specs: !zahlen_angaben.is_empty() || !parsed_numbers.is_empty(),
        tokens: tokens.to_vec(),
    }
}

pub fn if_print_cmd_again(text_state: &PromptTextState) -> bool {
    text_state.menge.contains("loggen") || text_state.menge.contains("print") || text_state.menge.contains("cmd")
}

pub fn plan_prompt_execution(
    bundle: &PromptExecutionBundle,
    prepared: &PreparedPromptOutput,
    text_state: &PromptTextState,
) -> PromptExecutionPlan {
    let fraction_result = fraction_range_management(
        &bundle.row_ranges,
        &prepared.zahlen_angaben_c,
        &prepared.tokens,
        &prepared.zahlen_angaben,
    );
    let mut plan = PromptExecutionPlan::empty();
    plan.is_pure_reta = prepared.is_pure_only_reta_cmd;
    plan.print_command_again = if_print_cmd_again(text_state);
    plan.number_range_argument = if prepared.zahlen_angaben_c.is_empty() {
        None
    } else {
        Some(vorher_von_ausschnitt_oder_zaehlung(text_state, &prepared.zahlen_angaben_c))
    };
    plan.max_argument = Some(another_oberes_maximum(
        &bundle.row_ranges,
        &prepared.zahlen_angaben_c,
        prepared.max_num,
        1024,
    ));

    if prepared.is_pure_only_reta_cmd {
        plan.cmd_gave_output = true;
        plan.reta_argv = split_reta_argv_like_python(&prepared.tokens);
        return plan;
    }

    if text_state.menge.contains("kurzbefehle") {
        plan.cmd_gave_output = true;
        plan.helper_outputs.push(format!("Kurzbefehle: {}", bundle.befehle.iter().filter(|item| item.chars().count() == 1).cloned().collect::<Vec<_>>().join(" ")));
    }
    if text_state.menge.contains("befehle") {
        plan.cmd_gave_output = true;
        plan.helper_outputs.push(format!("Befehle: {}", bundle.befehle.join(", ")));
    }
    if text_state.menge.contains("hilfe") || text_state.menge.contains("help") || text_state.menge.contains("h") {
        plan.cmd_gave_output = true;
        plan.side_effects.push("show_reta_prompt_help".to_string());
    }
    if fraction_result.saw_number_specs {
        let mut argv = vec!["reta".to_string(), "-zeilen".to_string()];
        if let Some(range_arg) = &plan.number_range_argument {
            argv.push(range_arg.clone());
        }
        if let Some(max_arg) = &plan.max_argument {
            argv.push(max_arg.clone());
        }
        plan.reta_argv = argv;
    }
    plan
}

pub fn split_reta_argv_like_python(tokens: &[String]) -> Vec<String> {
    if tokens.is_empty() {
        return Vec::new();
    }
    let joined = tokens.join(" ");
    let mut out = Vec::new();
    for (index, part) in joined.split(" -").enumerate() {
        if index == 0 {
            out.push(part.to_string());
        } else {
            out.push(format!("-{part}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt_preparation::bootstrap_prompt_preparation;
    use crate::prompt_language::PromptModus;

    #[test]
    fn fraction_scanner_finds_simple_fraction() {
        let parts = bruch_spalt("a 3/4 b");
        assert!(parts.iter().any(|part| matches!(part, BruchPart::Fraction(a, b) if a == "3" && b == "4")));
    }

    #[test]
    fn pure_reta_plan_keeps_python_split_shape() {
        let prep = bootstrap_prompt_preparation();
        let prepared = prep.prepare_large_output("", PromptModus::Normal, PromptModus::Normal, PromptModus::Normal, "reta -zeilen --alles", &[]);
        let state = PromptTextState::new("reta -zeilen --alles");
        let plan = bootstrap_prompt_execution().plan_prompt_execution(&prepared, &state);
        assert!(plan.is_pure_reta);
        assert_eq!(plan.reta_argv.first().map(String::as_str), Some("reta"));
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "PromptGrosseAusgabe",
    "PromptVonGrosserAusgabeSonderBefehlAusgaben",
    "addMoreVals",
    "addMoreVals2",
    "bruchBereichsManagementAndWbefehl",
    "configure_prompt_execution",
    "createRangesForBruchLists",
    "dictToList",
    "findEqualNennerZaehler",
    "findNennerZaehlerMakesWholeNum",
    "getDictLimtedByKeyList",
    "grKl",
    "maxMenge",
    "retaCmdAbstraction_n_and_1pron",
    "retaExecuteNprint",
    "run_grosse_ausgabe",
    "zeiln1234create",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
