use crate::shared::words_py::{PyValue, StoreParameterEntry, Words};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactPythonColumn {
    pub parameter_main_name: String,
    pub parameter_name: String,
    pub parameter_main_aliases: Vec<String>,
    pub parameter_aliases: Vec<String>,
    pub column_numbers: Vec<i64>,
    pub source_entry_index: usize,
    pub source_data_index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactPythonColumnMeta {
    pub column_number: i64,
    pub direct_matches: Vec<ExactPythonColumn>,
}

fn first_non_empty(names: &[String]) -> String {
    names.iter().find(|value| !value.is_empty()).cloned().unwrap_or_default()
}

fn dedup_strings_preserve_order(values: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for value in values {
        if !value.is_empty() && !out.contains(value) {
            out.push(value.clone());
        }
    }
    out
}

fn dedup_i64_preserve_order(values: &[i64]) -> Vec<i64> {
    let mut out: Vec<i64> = Vec::new();
    for value in values {
        if !out.contains(value) {
            out.push(*value);
        }
    }
    out
}

fn names_match_alias(query: &str, names: &[String]) -> bool {
    names.iter().any(|name| name == query)
}

fn extract_direct_columns(data: &[PyValue]) -> Vec<i64> {
    let mut out = Vec::new();
    for value in data {
        match value {
            PyValue::Int(v) => out.push(*v),
            // direct columns only; tuples are generator/meta shapes and stay out here on purpose
            PyValue::Tuple(values) => {
                if values.iter().all(|item| matches!(item, PyValue::Int(_))) {
                    // keep strict direct-column semantics: tuples are not direct columns
                }
            }
            PyValue::Str(_) | PyValue::Bool(_) | PyValue::NoneValue => {}
        }
    }
    dedup_i64_preserve_order(&out)
}

fn entry_to_direct_columns(entry: &StoreParameterEntry, entry_index: usize) -> Vec<ExactPythonColumn> {
    let parameter_main_aliases = dedup_strings_preserve_order(&entry.parameterMainNames);
    let parameter_aliases = dedup_strings_preserve_order(&entry.parameterNames);
    let parameter_main_name = first_non_empty(&parameter_main_aliases);
    let parameter_name = first_non_empty(&parameter_aliases);

    let mut out = Vec::new();
    for (data_index, data) in entry.datas.iter().enumerate() {
        let columns = extract_direct_columns(data);
        if !columns.is_empty() {
            out.push(ExactPythonColumn {
                parameter_main_name: parameter_main_name.clone(),
                parameter_name: parameter_name.clone(),
                parameter_main_aliases: parameter_main_aliases.clone(),
                parameter_aliases: parameter_aliases.clone(),
                column_numbers: columns,
                source_entry_index: entry_index,
                source_data_index: data_index,
            });
        }
    }
    out
}

pub fn exact_all_direct_columns(words: &Words) -> Vec<ExactPythonColumn> {
    let mut out = Vec::new();
    for (entry_index, entry) in words.paraNdataMatrix.iter().enumerate() {
        out.extend(entry_to_direct_columns(entry, entry_index));
    }
    out
}

pub fn exact_all_direct_columns_for_pair(
    words: &Words,
    parameter_main_name: &str,
    parameter_name: &str,
) -> Vec<ExactPythonColumn> {
    exact_all_direct_columns(words)
        .into_iter()
        .filter(|entry| {
            entry.parameter_main_name == parameter_main_name && entry.parameter_name == parameter_name
        })
        .collect()
}

pub fn exact_all_direct_columns_for_pair_alias(
    words: &Words,
    parameter_main_query: &str,
    parameter_query: &str,
) -> Vec<ExactPythonColumn> {
    exact_all_direct_columns(words)
        .into_iter()
        .filter(|entry| {
            names_match_alias(parameter_main_query, &entry.parameter_main_aliases)
                && names_match_alias(parameter_query, &entry.parameter_aliases)
        })
        .collect()
}

pub fn exact_column_numbers_for_pair_alias(
    words: &Words,
    parameter_main_query: &str,
    parameter_query: &str,
) -> Vec<i64> {
    let mut out = Vec::new();
    for entry in exact_all_direct_columns_for_pair_alias(words, parameter_main_query, parameter_query) {
        out.extend(entry.column_numbers);
    }
    dedup_i64_preserve_order(&out)
}

pub fn exact_meta_for_column(words: &Words, column_number: i64) -> Option<ExactPythonColumnMeta> {
    let direct_matches: Vec<ExactPythonColumn> = exact_all_direct_columns(words)
        .into_iter()
        .filter(|entry| entry.column_numbers.contains(&column_number))
        .collect();

    if direct_matches.is_empty() {
        None
    } else {
        Some(ExactPythonColumnMeta {
            column_number,
            direct_matches,
        })
    }
}

pub fn all_parameter_main_names(words: &Words) -> Vec<String> {
    let mut out = Vec::new();
    for entry in &words.paraNdataMatrix {
        let name = first_non_empty(&entry.parameterMainNames);
        if !name.is_empty() && !out.contains(&name) {
            out.push(name);
        }
    }
    out
}

pub fn all_parameter_main_alias_groups(words: &Words) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    for entry in &words.paraNdataMatrix {
        let aliases = dedup_strings_preserve_order(&entry.parameterMainNames);
        if !aliases.is_empty() && !out.contains(&aliases) {
            out.push(aliases);
        }
    }
    out
}

pub fn parameter_names_for_main(words: &Words, parameter_main_name: &str) -> Vec<String> {
    let mut out = Vec::new();
    for entry in &words.paraNdataMatrix {
        if first_non_empty(&entry.parameterMainNames) == parameter_main_name {
            let name = first_non_empty(&entry.parameterNames);
            if !name.is_empty() && !out.contains(&name) {
                out.push(name);
            }
        }
    }
    out
}

pub fn parameter_alias_groups_for_main_alias(words: &Words, parameter_main_query: &str) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    for entry in &words.paraNdataMatrix {
        let main_aliases = dedup_strings_preserve_order(&entry.parameterMainNames);
        if names_match_alias(parameter_main_query, &main_aliases) {
            let aliases = dedup_strings_preserve_order(&entry.parameterNames);
            if !aliases.is_empty() && !out.contains(&aliases) {
                out.push(aliases);
            }
        }
    }
    out
}

pub fn direct_column_reverse_map(words: &Words) -> BTreeMap<i64, Vec<(String, String)>> {
    let mut out: BTreeMap<i64, Vec<(String, String)>> = BTreeMap::new();
    for entry in exact_all_direct_columns(words) {
        for column in entry.column_numbers {
            let pair = (entry.parameter_main_name.clone(), entry.parameter_name.clone());
            let slot = out.entry(column).or_default();
            if !slot.contains(&pair) {
                slot.push(pair);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::words_py::Words;

    #[test]
    fn direct_pair_lookup_finds_menschliches_motive() {
        let words = Words::new();
        let hits = exact_all_direct_columns_for_pair_alias(&words, "menschliches", "motive")
            .into_iter()
            .flat_map(|entry| entry.column_numbers)
            .collect::<Vec<_>>();
        assert!(!hits.is_empty());
    }

    #[test]
    fn exact_meta_for_known_column_returns_match() {
        let words = Words::new();
        let meta = exact_meta_for_column(&words, 5).expect("known direct column should resolve");
        assert!(!meta.direct_matches.is_empty());
    }

    #[test]
    fn all_main_alias_groups_contains_menschliches_alias() {
        let words = Words::new();
        let groups = all_parameter_main_alias_groups(&words);
        assert!(groups.iter().any(|group| group.iter().any(|value| value == "menschliches")));
    }

    #[test]
    fn parameter_alias_groups_follow_main_alias() {
        let words = Words::new();
        let groups = parameter_alias_groups_for_main_alias(&words, "menschliches");
        assert!(groups.iter().any(|group| group.iter().any(|value| value == "motive")));
    }

    #[test]
    fn reverse_map_contains_known_column() {
        let words = Words::new();
        let reverse = direct_column_reverse_map(&words);
        assert!(reverse.contains_key(&5));
    }
}
