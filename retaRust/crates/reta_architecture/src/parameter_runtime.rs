//! CLI parameter runtime transcompiled from
//! `python_arch_reference/reta_architecture/parameter_runtime.py`.
//!
//! The full Python module still contains large legacy side-effect blocks.  This
//! Rust layer owns the typed parse surface used by `rreta` and future table
//! ports: main-command context, sub-parameter tokenization, output-mode
//! extraction and upper-limit inference from row ranges.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::column_selection::ColumnBucketKey;
use crate::output_syntax::OutputMode;
use crate::parameter_matrix::{
    bucket_projections_for_alias_pair, canonical_pair_for_aliases, columns_for_alias_pair,
    nonempty_bucket_projection_count, parameter_matrix_seed_count,
    symbolic_bucket_projection_count,
};
use crate::row_ranges::{RowRangeMorphismBundle, bootstrap_row_range_morphisms};

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
    /// Explicit row order from row selectors such as `--vorhervonausschnitt`.
    /// The legacy visible table can be order-sensitive, so Stage 27 keeps this
    /// sequence separate from the sorted `BTreeSet` membership witness.
    pub rows_as_ordered: Vec<i64>,
    pub rows_of_combi: BTreeSet<i64>,
    pub selected_columns: BTreeSet<i64>,
    pub excluded_columns: BTreeSet<i64>,
    pub column_buckets: BTreeMap<ColumnBucketKey, BTreeSet<i64>>,
    pub symbolic_column_buckets: BTreeMap<ColumnBucketKey, BTreeSet<String>>,
    pub excluded_symbolic_column_buckets: BTreeMap<ColumnBucketKey, BTreeSet<String>>,
    pub resolved_alias_pairs: Vec<(String, String)>,
    pub unresolved_column_pairs: Vec<(String, String)>,
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
            parse_function: format!(
                "parameters_to_commands_and_numbers/parameter_matrix_entries={}/bucket_projections={}/symbols={}",
                parameter_matrix_seed_count(),
                nonempty_bucket_projection_count(),
                symbolic_bucket_projection_count()
            ),
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
                let value_items = value.as_deref().map(split_comma_values).unwrap_or_default();
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
                let main = MainParameter::from_cli(body)
                    .unwrap_or(MainParameter::Unknown(body.to_string()));
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

fn apply_row_token(
    bundle: &ParameterRuntimeBundle,
    token: &ParameterToken,
    sets: &mut ParameterCommandSets,
) {
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
                if matches!(
                    cleaned,
                    "sonne" | "schwarzesonne" | "planet" | "mond" | "SonneMitMondanteil"
                ) {
                    sets.param_lines.insert(cleaned.to_string());
                }
            }
        }
        (Some("vorhervonausschnitt"), Some(value)) => {
            for number in ordered_range_numbers(bundle, value) {
                sets.param_lines.insert(format!("a{number}"));
                sets.rows_as_numbers.insert(number);
                push_ordered_number(&mut sets.rows_as_ordered, number);
            }
        }
        (Some("zaehlung"), Some(value)) => {
            for number in ordered_range_numbers(bundle, value) {
                sets.param_lines.insert(format!("n{number}"));
                sets.rows_as_numbers.insert(number);
                push_ordered_number(&mut sets.rows_as_ordered, number);
            }
        }
        _ => {}
    }
}

fn apply_output_token(
    bundle: &ParameterRuntimeBundle,
    token: &ParameterToken,
    sets: &mut ParameterCommandSets,
) {
    match (token.key.as_deref(), token.value.as_deref()) {
        (Some("spaltenreihenfolgeundnurdiese"), Some(value)) => {
            sets.spaltenreihenfolgeundnurdiese = ordered_range_numbers(bundle, value);
        }
        _ => {}
    }
}

fn ordered_range_numbers(bundle: &ParameterRuntimeBundle, value: &str) -> Vec<i64> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for segment in bundle.row_ranges.syntax.split_comma_list(value) {
        let trimmed = segment.trim();
        if trimmed.is_empty() {
            continue;
        }
        let expanded = bundle.row_ranges.range_to_numbers(trimmed, false, 0, false);
        for number in expanded {
            if seen.insert(number) {
                out.push(number);
            }
        }
    }
    out
}


fn push_ordered_number(values: &mut Vec<i64>, number: i64) {
    if !values.contains(&number) {
        values.push(number);
    }
}

fn apply_column_token(
    bundle: &ParameterRuntimeBundle,
    token: &ParameterToken,
    sets: &mut ParameterCommandSets,
) {
    let Some(key) = token.key.as_deref() else {
        return;
    };
    if let Some(value) = token.value.as_deref() {
        let mut resolved_any = false;
        for raw_item in split_comma_values(value) {
            let trimmed = raw_item.trim();
            let negated = trimmed.starts_with('-');
            let item = trimmed.trim_start_matches('-');
            let projections = bucket_projections_for_alias_pair(key, item);
            let columns = columns_for_alias_pair(key, item);
            if !projections.is_empty() || !columns.is_empty() {
                resolved_any = true;
                if let Some((canonical_main, canonical_parameter)) =
                    canonical_pair_for_aliases(key, item)
                {
                    sets.resolved_alias_pairs
                        .push((canonical_main, canonical_parameter));
                }
                apply_bucket_projections(sets, &projections, negated);
                for column in columns {
                    if negated {
                        sets.excluded_columns.insert(column);
                    } else {
                        sets.selected_columns.insert(column);
                    }
                }
            } else if bundle.row_ranges.is_row_range(item) {
                resolved_any = true;
                for number in bundle.row_ranges.range_to_numbers(item, false, 0, false) {
                    if negated {
                        sets.excluded_columns.insert(number);
                        sets.column_buckets
                            .entry(ColumnBucketKey::negative(0))
                            .or_default()
                            .insert(number);
                    } else {
                        sets.selected_columns.insert(number);
                        // A numeric token inside the `-spalten` context is a
                        // column selector, not a row selector.  Keep row order
                        // owned by `-zeilen` so table materialization does not
                        // accidentally treat column IDs as source row indices.
                        sets.column_buckets
                            .entry(ColumnBucketKey::positive(0))
                            .or_default()
                            .insert(number);
                    }
                }
            }
        }
        if !resolved_any {
            sets.unresolved_column_pairs
                .push((key.to_string(), value.to_string()));
        }
    } else {
        let projections = bucket_projections_for_alias_pair(key, "");
        let columns = columns_for_alias_pair(key, "");
        if projections.is_empty() && columns.is_empty() {
            sets.unresolved_column_pairs
                .push((key.to_string(), String::new()));
        } else {
            apply_bucket_projections(sets, &projections, false);
            for column in columns {
                sets.selected_columns.insert(column);
            }
        }
    }
}

fn apply_bucket_projections(
    sets: &mut ParameterCommandSets,
    projections: &[crate::parameter_matrix::OwnedParameterBucketProjection],
    negated: bool,
) {
    for projection in projections {
        let key = if negated {
            ColumnBucketKey::negative(projection.bucket)
        } else {
            ColumnBucketKey::positive(projection.bucket)
        };
        if !projection.integers.is_empty() {
            sets.column_buckets
                .entry(key)
                .or_default()
                .extend(projection.integers.iter().copied());
        }
        if !projection.symbols.is_empty() {
            let target = if negated {
                &mut sets.excluded_symbolic_column_buckets
            } else {
                &mut sets.symbolic_column_buckets
            };
            target
                .entry(key)
                .or_default()
                .extend(projection.symbols.iter().cloned());
        }
    }
}

fn apply_kombi_token(
    bundle: &ParameterRuntimeBundle,
    token: &ParameterToken,
    sets: &mut ParameterCommandSets,
) {
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
    fn column_alias_matrix_resolves_kontinuum_m_744() {
        let runtime = bootstrap_parameter_runtime();
        let parsed = runtime.parse_cli_args(&["reta", "-spalten", "--kontinuum=m"]);
        assert!(parsed.command_sets.selected_columns.contains(&493));
        assert!(parsed.command_sets.selected_columns.contains(&744));
        assert!(
            parsed
                .command_sets
                .resolved_alias_pairs
                .contains(&("Kontinuum".to_string(), "M".to_string()))
        );
        let args = vec![
            "reta".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
        ];
        assert_eq!(produce_all_spalten_numbers(&args), vec![493, 744]);
    }

    #[test]
    fn column_alias_matrix_supports_negation() {
        let runtime = bootstrap_parameter_runtime();
        let parsed = runtime.parse_cli_args(&["reta", "-spalten", "--kontinuum=m,-m"]);
        assert!(parsed.command_sets.selected_columns.contains(&744));
        assert!(parsed.command_sets.excluded_columns.contains(&744));
        let args = vec![
            "reta".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m,-m".to_string(),
        ];
        assert!(produce_all_spalten_numbers(&args).is_empty());
    }

    #[test]
    fn output_spaltenreihenfolgeundnurdiese_preserves_explicit_order() {
        let runtime = bootstrap_parameter_runtime();
        let parsed = runtime.parse_cli_args(&[
            "reta",
            "-ausgabe",
            "--spaltenreihenfolgeundnurdiese=744,493",
        ]);
        assert_eq!(
            parsed.command_sets.spaltenreihenfolgeundnurdiese,
            vec![744, 493]
        );
    }

    #[test]
    fn column_alias_matrix_preserves_legacy_bucket_coordinates() {
        let runtime = bootstrap_parameter_runtime();
        let parsed = runtime.parse_cli_args(&[
            "reta",
            "-spalten",
            "--kontinuum=m",
            "--multiplikationen=motivstern",
            "--gebrochenuniversum=2",
        ]);
        assert_eq!(
            parsed.command_sets.column_buckets[&ColumnBucketKey::positive(0)],
            BTreeSet::from([493, 744])
        );
        assert!(
            parsed.command_sets.symbolic_column_buckets[&ColumnBucketKey::positive(7)]
                .contains("primMotivStern")
        );
        assert!(
            parsed.command_sets.symbolic_column_buckets[&ColumnBucketKey::positive(5)]
                .contains("2")
        );
    }

    #[test]
    fn symbolic_bucket_negation_removes_matching_local_sections() {
        let args = vec![
            "reta".to_string(),
            "-spalten".to_string(),
            "--gebrochenuniversum=2,-2".to_string(),
        ];
        let symbolic = produce_all_symbolic_column_buckets(&args);
        assert!(
            !symbolic
                .get(&ColumnBucketKey::positive(5))
                .is_some_and(|values| values.contains("2"))
        );
    }

    #[test]
    fn upper_limit_matches_python_cases() {
        let runtime = bootstrap_parameter_runtime();
        assert_eq!(
            runtime.upper_limit_values_for_argument("--oberesmaximum=55"),
            vec![55]
        );
        assert_eq!(
            runtime.upper_limit_from_arguments(&["--vorhervonausschnitt=3"], Some(10)),
            Some(1024)
        );
    }

    #[test]
    fn vorhervonausschnitt_preserves_explicit_row_order() {
        let runtime = bootstrap_parameter_runtime();
        let parsed = runtime.parse_cli_args(&[
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=3,1-2",
        ]);
        assert_eq!(parsed.command_sets.rows_as_ordered, vec![3, 1, 2]);
        assert_eq!(
            parsed.command_sets.rows_as_numbers.iter().copied().collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn numeric_spalten_range_does_not_select_rows() {
        let runtime = bootstrap_parameter_runtime();
        let parsed = runtime.parse_cli_args(&[
            "reta",
            "-spalten",
            "--religion=1-3",
        ]);
        assert!(parsed.command_sets.rows_as_numbers.is_empty());
        assert!(parsed.command_sets.rows_as_ordered.is_empty());
        assert_eq!(
            parsed.command_sets.selected_columns.iter().copied().collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }

}

// Stage 16: concrete Python-name runtime wrappers.
pub fn _ensure_runtime_imports() -> ParameterRuntimeBundle {
    bootstrap_parameter_runtime()
}

pub fn apply_upper_limit_argument(args: &[String]) -> Option<i64> {
    bootstrap_parameter_runtime().upper_limit_from_arguments(args, None)
}

pub fn apply_width_parameter(args: &[String]) -> Option<usize> {
    args.iter().find_map(|arg| {
        arg.strip_prefix("--breite=")
            .or_else(|| arg.strip_prefix("--breiten="))
            .and_then(|value| value.parse::<usize>().ok())
    })
}

pub fn resultingSpaltenFromTuple(columns: &[i64]) -> Vec<i64> {
    let mut out = columns.to_vec();
    out.sort_unstable();
    out.dedup();
    out
}

#[allow(non_snake_case)]
pub fn spalten_removeDoublesNthenRemoveOneFromAnother(
    positive: &[i64],
    negative: &[i64],
) -> Vec<i64> {
    let mut out = resultingSpaltenFromTuple(positive);
    let remove = negative
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    out.retain(|value| !remove.contains(value));
    out
}

pub fn produce_all_spalten_numbers(args: &[String]) -> Vec<i64> {
    let bundle = bootstrap_parameter_runtime();
    let parsed = bundle.parse_cli_args(args);
    let mut out = parsed
        .command_sets
        .selected_columns
        .iter()
        .copied()
        .collect::<Vec<_>>();
    let excluded = parsed.command_sets.excluded_columns;
    out.retain(|column| !excluded.contains(column));
    resultingSpaltenFromTuple(&out)
}

/// Return the legacy `(negation, bucket)` integer column sections produced by
/// the Stage-18 parameter matrix.  Negative bucket sections are subtracted from
/// their positive counterpart just like `column_selection::normalize_bucket_map`.
pub fn produce_all_column_bucket_numbers(
    args: &[String],
) -> BTreeMap<ColumnBucketKey, BTreeSet<i64>> {
    let bundle = bootstrap_parameter_runtime();
    let parsed = bundle.parse_cli_args(args);
    crate::column_selection::bootstrap_column_selection()
        .normalize_bucket_map(&parsed.command_sets.column_buckets)
}

/// Return symbolic generated/fraction/Kombi bucket payloads from the matrix.
/// These values are intentionally kept as strings because Python used them as
/// generated-column and gebrochen-rational selectors, not only as integers.
pub fn produce_all_symbolic_column_buckets(
    args: &[String],
) -> BTreeMap<ColumnBucketKey, BTreeSet<String>> {
    let bundle = bootstrap_parameter_runtime();
    let parsed = bundle.parse_cli_args(args);
    let mut out = parsed.command_sets.symbolic_column_buckets;
    for bucket in 0..12u8 {
        let positive = ColumnBucketKey::positive(bucket);
        let negative = ColumnBucketKey::negative(bucket);
        if let Some(negative_values) = parsed
            .command_sets
            .excluded_symbolic_column_buckets
            .get(&negative)
        {
            if let Some(positive_values) = out.get_mut(&positive) {
                for value in negative_values {
                    positive_values.remove(value);
                }
            }
        }
    }
    out.retain(|_, values| !values.is_empty());
    out
}

// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "_ensure_runtime_imports",
    "resultingSpaltenFromTuple",
    "spalten_removeDoublesNthenRemoveOneFromAnother",
    "apply_upper_limit_argument",
    "apply_width_parameter",
    "produce_all_spalten_numbers",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
