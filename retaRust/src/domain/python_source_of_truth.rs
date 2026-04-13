use crate::shared::words_py::{PyValue, StoreParameterEntry, Words};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PythonAliasGroup {
    pub canonical_name: String,
    pub aliases: Vec<String>,
}

fn primary_name(names: &[String]) -> String {
    names.first().cloned().unwrap_or_default()
}

fn normalize_lookup_name(input: &str) -> String {
    input
        .trim()
        .to_lowercase()
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('ß', "ss")
        .chars()
        .map(|ch| {
            if ch.is_alphanumeric() {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('_')
        .filter(|piece| !piece.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn extract_direct_columns(data: &[PyValue]) -> Vec<i64> {
    let mut out = Vec::new();
    for value in data {
        match value {
            PyValue::Int(v) => out.push(*v),
            // strict direct-column semantics: tuples are generator/meta shapes.
            PyValue::Tuple(_) | PyValue::Str(_) | PyValue::Bool(_) | PyValue::NoneValue => {}
        }
    }
    out
}

fn entry_to_direct_columns(entry: &StoreParameterEntry, entry_index: usize) -> Vec<ExactPythonColumn> {
    let main_name = primary_name(&entry.parameterMainNames);
    let parameter_name = primary_name(&entry.parameterNames);
    let parameter_main_aliases = entry.parameterMainNames.clone();
    let parameter_aliases = entry.parameterNames.clone();

    let mut out = Vec::new();
    for (data_index, data) in entry.datas.iter().enumerate() {
        let columns = extract_direct_columns(data);
        if !columns.is_empty() {
            out.push(ExactPythonColumn {
                parameter_main_name: main_name.clone(),
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

pub fn all_main_alias_groups(words: &Words) -> Vec<PythonAliasGroup> {
    let mut groups = Vec::new();
    for entry in &words.paraNdataMatrix {
        let canonical_name = primary_name(&entry.parameterMainNames);
        if canonical_name.is_empty() || groups.iter().any(|group: &PythonAliasGroup| group.canonical_name == canonical_name) {
            continue;
        }
        groups.push(PythonAliasGroup {
            canonical_name,
            aliases: entry.parameterMainNames.clone(),
        });
    }
    groups
}

pub fn parameter_alias_groups_for_main(words: &Words, parameter_main_name: &str) -> Vec<PythonAliasGroup> {
    let canonical_main = match resolve_parameter_main_alias(words, parameter_main_name) {
        Some(value) => value,
        None => return Vec::new(),
    };

    let mut groups = Vec::new();
    for entry in &words.paraNdataMatrix {
        if primary_name(&entry.parameterMainNames) != canonical_main {
            continue;
        }
        let canonical_name = primary_name(&entry.parameterNames);
        if canonical_name.is_empty() || groups.iter().any(|group: &PythonAliasGroup| group.canonical_name == canonical_name) {
            continue;
        }
        groups.push(PythonAliasGroup {
            canonical_name,
            aliases: entry.parameterNames.clone(),
        });
    }
    groups
}

pub fn resolve_parameter_main_alias(words: &Words, parameter_main_name: &str) -> Option<String> {
    let needle = normalize_lookup_name(parameter_main_name);
    for group in all_main_alias_groups(words) {
        if group.aliases.iter().any(|alias| normalize_lookup_name(alias) == needle) {
            return Some(group.canonical_name);
        }
    }
    None
}

pub fn resolve_parameter_alias(words: &Words, parameter_main_name: &str, parameter_name: &str) -> Option<String> {
    let canonical_main = resolve_parameter_main_alias(words, parameter_main_name)?;
    let needle = normalize_lookup_name(parameter_name);
    for group in parameter_alias_groups_for_main(words, &canonical_main) {
        if group.aliases.iter().any(|alias| normalize_lookup_name(alias) == needle) {
            return Some(group.canonical_name);
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
    let (canonical_main, canonical_parameter) = match canonicalize_pair(words, parameter_main_name, parameter_name) {
        Some(value) => value,
        None => return Vec::new(),
    };

    exact_all_direct_columns(words)
        .into_iter()
        .filter(|entry| {
            entry.parameter_main_name == canonical_main && entry.parameter_name == canonical_parameter
        })
        .collect()
}

pub fn reverse_map_all_direct_columns(words: &Words, column_number: i64) -> Vec<ExactPythonColumn> {
    exact_all_direct_columns(words)
        .into_iter()
        .filter(|entry| entry.column_numbers.contains(&column_number))
        .collect()
}

pub fn exact_meta_for_column(words: &Words, column_number: i64) -> Option<ExactPythonColumnMeta> {
    let direct_matches = reverse_map_all_direct_columns(words, column_number);
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
    all_main_alias_groups(words)
        .into_iter()
        .map(|group| group.canonical_name)
        .collect()
}

pub fn parameter_names_for_main(words: &Words, parameter_main_name: &str) -> Vec<String> {
    parameter_alias_groups_for_main(words, parameter_main_name)
        .into_iter()
        .map(|group| group.canonical_name)
        .collect()
}

pub fn all_main_alias_names(words: &Words, parameter_main_name: &str) -> Vec<String> {
    let canonical_main = match resolve_parameter_main_alias(words, parameter_main_name) {
        Some(value) => value,
        None => return Vec::new(),
    };

    for group in all_main_alias_groups(words) {
        if group.canonical_name == canonical_main {
            return group.aliases;
        }
    }
    Vec::new()
}

pub fn parameter_alias_names(words: &Words, parameter_main_name: &str, parameter_name: &str) -> Vec<String> {
    let canonical_main = match resolve_parameter_main_alias(words, parameter_main_name) {
        Some(value) => value,
        None => return Vec::new(),
    };
    let canonical_parameter = match resolve_parameter_alias(words, &canonical_main, parameter_name) {
        Some(value) => value,
        None => return Vec::new(),
    };

    for group in parameter_alias_groups_for_main(words, &canonical_main) {
        if group.canonical_name == canonical_parameter {
            return group.aliases;
        }
    }
    Vec::new()
}

pub fn column_numbers_for_pair(words: &Words, parameter_main_name: &str, parameter_name: &str) -> Vec<i64> {
    let mut out = Vec::new();
    for entry in exact_all_direct_columns_for_pair(words, parameter_main_name, parameter_name) {
        for column_number in entry.column_numbers {
            if !out.contains(&column_number) {
                out.push(column_number);
            }
        }
    }
    out
}

pub fn reverse_map_canonical_pairs(words: &Words, column_number: i64) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for entry in reverse_map_all_direct_columns(words, column_number) {
        let pair = (entry.parameter_main_name, entry.parameter_name);
        if !out.contains(&pair) {
            out.push(pair);
        }
    }
    out
}

pub fn alias_summary_for_column(words: &Words, column_number: i64) -> Vec<(Vec<String>, Vec<String>)> {
    let mut out = Vec::new();
    for entry in reverse_map_all_direct_columns(words, column_number) {
        let pair = (entry.parameter_main_aliases, entry.parameter_aliases);
        if !out.contains(&pair) {
            out.push(pair);
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
    fn all_main_alias_groups_contains_menschliches_alias() {
        let words = Words::new();
        let groups = all_main_alias_groups(&words);
        let menschliches = groups
            .iter()
            .find(|group| group.canonical_name == "Menschliches")
            .expect("expected Menschliches alias group");
        assert!(menschliches.aliases.iter().any(|alias| alias == "menschliches"));
    }

    #[test]
    fn parameter_alias_groups_follow_main_alias() {
        let words = Words::new();
        let groups = parameter_alias_groups_for_main(&words, "menschliches");
        assert!(groups.iter().any(|group| group.canonical_name == "Motive"));
    }

    #[test]
    fn reverse_map_contains_known_column() {
        let words = Words::new();
        let reverse = reverse_map_canonical_pairs(&words, 5);
        assert!(!reverse.is_empty());
    }

    #[test]
    fn canonicalize_pair_accepts_aliases() {
        let words = Words::new();
        let pair = canonicalize_pair(&words, "menschliches", "motive").expect("alias pair should resolve");
        assert_eq!(pair, ("Menschliches".to_string(), "Motive".to_string()));
    }

    #[test]
    fn parameter_alias_names_follow_canonical_pair() {
        let words = Words::new();
        let aliases = parameter_alias_names(&words, "Menschliches", "Motive");
        assert!(aliases.iter().any(|alias| normalize_lookup_name(alias) == "motive"));
    }

    #[test]
    fn alias_summary_for_known_column_is_not_empty() {
        let words = Words::new();
        assert!(!alias_summary_for_column(&words, 5).is_empty());
    }
}
