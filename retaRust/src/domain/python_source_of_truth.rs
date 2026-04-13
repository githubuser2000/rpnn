use crate::shared::words_py::{PyValue, StoreParameterEntry, Words};
use indexmap::IndexMap;

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

fn first_name(names: &[String]) -> String {
    names.first().cloned().unwrap_or_default()
}

fn dedup_preserve_order(values: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for value in values {
        if !value.is_empty() && !out.contains(&value) {
            out.push(value);
        }
    }
    out
}

fn normalize_for_alias_match(input: &str) -> String {
    input
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .flat_map(|ch| ch.to_lowercase())
        .collect()
}

fn entry_main_aliases(entry: &StoreParameterEntry) -> Vec<String> {
    dedup_preserve_order(entry.parameterMainNames.clone())
}

fn entry_parameter_aliases(entry: &StoreParameterEntry) -> Vec<String> {
    dedup_preserve_order(entry.parameterNames.clone())
}

fn extract_direct_columns(data: &[PyValue]) -> Vec<i64> {
    let mut out = Vec::new();
    for value in data {
        if let PyValue::Int(v) = value {
            out.push(*v);
        }
    }
    out
}

fn entry_to_direct_columns(entry: &StoreParameterEntry, entry_index: usize) -> Vec<ExactPythonColumn> {
    let main_aliases = entry_main_aliases(entry);
    let parameter_aliases = entry_parameter_aliases(entry);
    let parameter_main_name = first_name(&main_aliases);
    let parameter_name = first_name(&parameter_aliases);

    entry
        .datas
        .iter()
        .enumerate()
        .filter_map(|(data_index, data)| {
            let column_numbers = extract_direct_columns(data);
            if column_numbers.is_empty() {
                None
            } else {
                Some(ExactPythonColumn {
                    parameter_main_name: parameter_main_name.clone(),
                    parameter_name: parameter_name.clone(),
                    parameter_main_aliases: main_aliases.clone(),
                    parameter_aliases: parameter_aliases.clone(),
                    column_numbers,
                    source_entry_index: entry_index,
                    source_data_index: data_index,
                })
            }
        })
        .collect()
}

pub fn exact_all_direct_columns(words: &Words) -> Vec<ExactPythonColumn> {
    words
        .paraNdataMatrix
        .iter()
        .enumerate()
        .flat_map(|(entry_index, entry)| entry_to_direct_columns(entry, entry_index))
        .collect()
}

pub fn all_main_alias_groups(words: &Words) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    for entry in &words.paraNdataMatrix {
        let group = entry_main_aliases(entry);
        if !group.is_empty() && !out.contains(&group) {
            out.push(group);
        }
    }
    out
}

pub fn resolve_parameter_main_alias(words: &Words, parameter_main_name: &str) -> Option<String> {
    let needle = normalize_for_alias_match(parameter_main_name);
    if needle.is_empty() {
        return None;
    }
    for aliases in all_main_alias_groups(words) {
        if aliases
            .iter()
            .any(|alias| normalize_for_alias_match(alias) == needle)
        {
            return aliases.first().cloned();
        }
    }
    None
}

pub fn parameter_alias_groups_for_main(words: &Words, parameter_main_name: &str) -> Vec<Vec<String>> {
    let Some(canonical_main) = resolve_parameter_main_alias(words, parameter_main_name) else {
        return Vec::new();
    };

    let mut out: Vec<Vec<String>> = Vec::new();
    for entry in &words.paraNdataMatrix {
        if first_name(&entry.parameterMainNames) == canonical_main {
            let group = entry_parameter_aliases(entry);
            if !group.is_empty() && !out.contains(&group) {
                out.push(group);
            }
        }
    }
    out
}

pub fn resolve_parameter_alias(words: &Words, parameter_main_name: &str, parameter_name: &str) -> Option<String> {
    let needle = normalize_for_alias_match(parameter_name);
    if needle.is_empty() {
        return None;
    }
    for aliases in parameter_alias_groups_for_main(words, parameter_main_name) {
        if aliases
            .iter()
            .any(|alias| normalize_for_alias_match(alias) == needle)
        {
            return aliases.first().cloned();
        }
    }
    None
}

pub fn canonicalize_pair(words: &Words, parameter_main_name: &str, parameter_name: &str) -> Option<(String, String)> {
    let canonical_main = resolve_parameter_main_alias(words, parameter_main_name)?;
    let canonical_parameter = resolve_parameter_alias(words, &canonical_main, parameter_name)?;
    Some((canonical_main, canonical_parameter))
}

pub fn exact_all_direct_columns_for_pair(
    words: &Words,
    parameter_main_name: &str,
    parameter_name: &str,
) -> Vec<ExactPythonColumn> {
    let Some((canonical_main, canonical_parameter)) =
        canonicalize_pair(words, parameter_main_name, parameter_name)
    else {
        return Vec::new();
    };

    exact_all_direct_columns(words)
        .into_iter()
        .filter(|entry| {
            entry.parameter_main_name == canonical_main && entry.parameter_name == canonical_parameter
        })
        .collect()
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

pub fn reverse_map_all_direct_columns(words: &Words) -> IndexMap<i64, Vec<ExactPythonColumn>> {
    let mut out: IndexMap<i64, Vec<ExactPythonColumn>> = IndexMap::new();
    for entry in exact_all_direct_columns(words) {
        for column_number in &entry.column_numbers {
            out.entry(*column_number).or_default().push(entry.clone());
        }
    }
    out
}

pub fn all_parameter_main_names(words: &Words) -> Vec<String> {
    all_main_alias_groups(words)
        .into_iter()
        .filter_map(|aliases| aliases.first().cloned())
        .collect()
}

pub fn parameter_names_for_main(words: &Words, parameter_main_name: &str) -> Vec<String> {
    parameter_alias_groups_for_main(words, parameter_main_name)
        .into_iter()
        .filter_map(|aliases| aliases.first().cloned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::words_py::Words;

    #[test]
    fn direct_pair_lookup_finds_menschliches_motive() {
        let words = Words::new();
        let hits = exact_all_direct_columns_for_pair(&words, "Menschliches", "Motive")
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
    fn reverse_map_contains_known_column() {
        let words = Words::new();
        let reverse = reverse_map_all_direct_columns(&words);
        assert!(reverse.contains_key(&5));
    }

    #[test]
    fn all_main_alias_groups_contains_menschliches_alias() {
        let words = Words::new();
        let groups = all_main_alias_groups(&words);
        assert!(groups.iter().any(|group| {
            group.contains(&"Menschliches".to_string()) && group.contains(&"menschliches".to_string())
        }));
    }

    #[test]
    fn parameter_alias_groups_follow_main_alias() {
        let words = Words::new();
        let groups = parameter_alias_groups_for_main(&words, "menschliches");
        assert!(groups.iter().any(|group| group.contains(&"Motive".to_string()) || group.contains(&"motive".to_string())));
    }
}
