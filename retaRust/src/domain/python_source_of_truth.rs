use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::shared::words_py::{PyValue, Words};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PythonAliasGroup {
    pub canonical: String,
    pub aliases: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactPythonColumn {
    pub column_number: i64,
    pub parameter_main: String,
    pub parameter_main_aliases: Vec<String>,
    pub parameter: String,
    pub parameter_aliases: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnAliasSummary {
    pub column_number: i64,
    pub canonical_pairs: Vec<(String, String)>,
    pub parameter_main_aliases: Vec<String>,
    pub parameter_aliases: Vec<String>,
}

fn normalize_alias_like_python(txt: &str) -> String {
    txt.trim().replace('ß', "ss").to_lowercase()
}

fn extract_ints(datas: &[Vec<PyValue>]) -> Vec<i64> {
    let mut out = Vec::new();
    for data_block in datas {
        for value in data_block {
            if let PyValue::Int(n) = value {
                out.push(*n);
            }
        }
    }
    out
}

fn canonical_and_aliases(names: &[String]) -> Option<(String, Vec<String>)> {
    let canonical = names.first()?.clone();
    let mut aliases = Vec::new();
    let mut seen = BTreeSet::new();
    for name in names {
        let key = normalize_alias_like_python(name);
        if seen.insert(key) {
            aliases.push(name.clone());
        }
    }
    Some((canonical, aliases))
}

pub fn all_main_alias_groups(words: &Words) -> Vec<PythonAliasGroup> {
    let mut by_canonical: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for entry in &words.paraNdataMatrix {
        if let Some((canonical, aliases)) = canonical_and_aliases(&entry.parameterMainNames) {
            let set = by_canonical.entry(canonical.clone()).or_default();
            for alias in aliases {
                set.insert(alias);
            }
        }
    }
    by_canonical
        .into_iter()
        .map(|(canonical, aliases)| PythonAliasGroup {
            canonical,
            aliases: aliases.into_iter().collect(),
        })
        .collect()
}

pub fn resolve_parameter_main_alias(words: &Words, main_alias: &str) -> Option<String> {
    let needle = normalize_alias_like_python(main_alias);
    for group in all_main_alias_groups(words) {
        if group
            .aliases
            .iter()
            .any(|alias| normalize_alias_like_python(alias) == needle)
        {
            return Some(group.canonical);
        }
    }
    None
}

pub fn parameter_alias_groups_for_main(words: &Words, canonical_main: &str) -> Vec<PythonAliasGroup> {
    let wanted = normalize_alias_like_python(canonical_main);
    let mut by_canonical: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for entry in &words.paraNdataMatrix {
        let Some((entry_main, _)) = canonical_and_aliases(&entry.parameterMainNames) else {
            continue;
        };
        if normalize_alias_like_python(&entry_main) != wanted {
            continue;
        }
        let Some((canonical_param, aliases)) = canonical_and_aliases(&entry.parameterNames) else {
            continue;
        };
        let set = by_canonical.entry(canonical_param.clone()).or_default();
        for alias in aliases {
            set.insert(alias);
        }
    }
    by_canonical
        .into_iter()
        .map(|(canonical, aliases)| PythonAliasGroup {
            canonical,
            aliases: aliases.into_iter().collect(),
        })
        .collect()
}

pub fn resolve_parameter_alias(words: &Words, canonical_main: &str, parameter_alias: &str) -> Option<String> {
    let needle = normalize_alias_like_python(parameter_alias);
    for group in parameter_alias_groups_for_main(words, canonical_main) {
        if group
            .aliases
            .iter()
            .any(|alias| normalize_alias_like_python(alias) == needle)
        {
            return Some(group.canonical);
        }
    }
    None
}

pub fn canonicalize_pair(words: &Words, parameter_main: &str, parameter: &str) -> Option<(String, String)> {
    let canonical_main = resolve_parameter_main_alias(words, parameter_main)?;
    let canonical_param = resolve_parameter_alias(words, &canonical_main, parameter)?;
    Some((canonical_main, canonical_param))
}

pub fn exact_all_direct_columns(words: &Words) -> Vec<ExactPythonColumn> {
    let mut out = Vec::new();
    for entry in &words.paraNdataMatrix {
        let Some((canonical_main, main_aliases)) = canonical_and_aliases(&entry.parameterMainNames) else {
            continue;
        };
        let Some((canonical_param, param_aliases)) = canonical_and_aliases(&entry.parameterNames) else {
            continue;
        };
        for n in extract_ints(&entry.datas) {
            out.push(ExactPythonColumn {
                column_number: n,
                parameter_main: canonical_main.clone(),
                parameter_main_aliases: main_aliases.clone(),
                parameter: canonical_param.clone(),
                parameter_aliases: param_aliases.clone(),
            });
        }
    }
    out.sort_by(|a, b| {
        a.parameter_main
            .cmp(&b.parameter_main)
            .then(a.parameter.cmp(&b.parameter))
            .then(a.column_number.cmp(&b.column_number))
    });
    out
}

pub fn exact_all_direct_columns_for_pair(words: &Words, parameter_main: &str, parameter: &str) -> Vec<ExactPythonColumn> {
    let Some((canonical_main, canonical_param)) = canonicalize_pair(words, parameter_main, parameter) else {
        return Vec::new();
    };
    exact_all_direct_columns(words)
        .into_iter()
        .filter(|column| column.parameter_main == canonical_main && column.parameter == canonical_param)
        .collect()
}

pub fn column_numbers_for_pair(words: &Words, parameter_main: &str, parameter: &str) -> Vec<i64> {
    exact_all_direct_columns_for_pair(words, parameter_main, parameter)
        .into_iter()
        .map(|c| c.column_number)
        .collect()
}

pub fn exact_meta_for_column(words: &Words, column_number: i64) -> Vec<ExactPythonColumn> {
    exact_all_direct_columns(words)
        .into_iter()
        .filter(|column| column.column_number == column_number)
        .collect()
}

pub fn reverse_map_all_direct_columns(words: &Words) -> BTreeMap<i64, Vec<ExactPythonColumn>> {
    let mut out: BTreeMap<i64, Vec<ExactPythonColumn>> = BTreeMap::new();
    for column in exact_all_direct_columns(words) {
        out.entry(column.column_number).or_default().push(column);
    }
    out
}

pub fn reverse_map_canonical_pairs(words: &Words) -> BTreeMap<i64, Vec<(String, String)>> {
    let mut out = BTreeMap::new();
    for (column, metas) in reverse_map_all_direct_columns(words) {
        let mut pairs = Vec::new();
        let mut seen = BTreeSet::new();
        for meta in metas {
            let pair = (meta.parameter_main, meta.parameter);
            let key = format!("{}\u{1f}{}", pair.0, pair.1);
            if seen.insert(key) {
                pairs.push(pair);
            }
        }
        out.insert(column, pairs);
    }
    out
}

pub fn all_main_alias_names(words: &Words) -> Vec<String> {
    let mut out = BTreeSet::new();
    for group in all_main_alias_groups(words) {
        for alias in group.aliases {
            out.insert(alias);
        }
    }
    out.into_iter().collect()
}

pub fn parameter_alias_names(words: &Words, canonical_main: &str) -> Vec<String> {
    let mut out = BTreeSet::new();
    for group in parameter_alias_groups_for_main(words, canonical_main) {
        for alias in group.aliases {
            out.insert(alias);
        }
    }
    out.into_iter().collect()
}

pub fn alias_summary_for_column(words: &Words, column_number: i64) -> Option<ColumnAliasSummary> {
    let metas = exact_meta_for_column(words, column_number);
    if metas.is_empty() {
        return None;
    }
    let mut canonical_pairs = Vec::new();
    let mut pair_seen = BTreeSet::new();
    let mut main_aliases = BTreeSet::new();
    let mut param_aliases = BTreeSet::new();
    for meta in metas {
        let pair_key = format!("{}\u{1f}{}", meta.parameter_main, meta.parameter);
        if pair_seen.insert(pair_key) {
            canonical_pairs.push((meta.parameter_main.clone(), meta.parameter.clone()));
        }
        for alias in meta.parameter_main_aliases {
            main_aliases.insert(alias);
        }
        for alias in meta.parameter_aliases {
            param_aliases.insert(alias);
        }
    }
    Some(ColumnAliasSummary {
        column_number,
        canonical_pairs,
        parameter_main_aliases: main_aliases.into_iter().collect(),
        parameter_aliases: param_aliases.into_iter().collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::words_py::Words;

    #[test]
    fn direct_pair_lookup_finds_menschliches_motive() {
        let words = Words::new();
        let cols = exact_all_direct_columns_for_pair(&words, "Menschliches", "Motive");
        assert!(!cols.is_empty());
    }

    #[test]
    fn reverse_map_contains_known_column() {
        let words = Words::new();
        let rev = reverse_map_all_direct_columns(&words);
        assert!(rev.contains_key(&240));
    }

    #[test]
    fn all_main_alias_groups_contains_menschliches_alias() {
        let words = Words::new();
        let groups = all_main_alias_groups(&words);
        assert!(groups.iter().any(|g| g.aliases.iter().any(|a| a == "menschliches")));
    }

    #[test]
    fn parameter_alias_groups_follow_main_alias() {
        let words = Words::new();
        let main = resolve_parameter_main_alias(&words, "menschliches").unwrap();
        let groups = parameter_alias_groups_for_main(&words, &main);
        assert!(groups.iter().any(|g| g.aliases.iter().any(|a| normalize_alias_like_python(a) == "motive")));
    }

    #[test]
    fn exact_meta_for_known_column_returns_match() {
        let words = Words::new();
        let meta = exact_meta_for_column(&words, 240);
        assert!(!meta.is_empty());
    }

    #[test]
    fn canonicalize_pair_accepts_aliases() {
        let words = Words::new();
        let pair = canonicalize_pair(&words, "menschliches", "motive").unwrap();
        assert_eq!(normalize_alias_like_python(&pair.0), "menschliches");
        assert_eq!(normalize_alias_like_python(&pair.1), "motive");
    }

    #[test]
    fn parameter_alias_names_follow_canonical_pair() {
        let words = Words::new();
        let aliases = parameter_alias_names(&words, "Menschliches");
        assert!(aliases.iter().any(|a| normalize_alias_like_python(a) == "motive"));
    }

    #[test]
    fn alias_summary_for_known_column_is_not_empty() {
        let words = Words::new();
        let summary = alias_summary_for_column(&words, 240).unwrap();
        assert!(!summary.canonical_pairs.is_empty());
        assert!(!summary.parameter_main_aliases.is_empty());
    }
}
