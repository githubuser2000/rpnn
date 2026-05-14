//! Kombi join/relation morphisms transcompiled from
//! `python_arch_reference/reta_architecture/combi_join.py`.
//!
//! The Python class `KombiJoin` still performs the complete legacy table join.
//! This Rust layer owns the typed relation shape: chosen lines, reduced
//! subtables, duplicate-number removal and final row gluing.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KombiJoinSpec {
    pub method_name: String,
    pub description: String,
}

impl KombiJoinSpec {
    pub fn new(method_name: &str, description: &str) -> Self {
        Self { method_name: method_name.to_string(), description: description.to_string() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KombiJoinBundle {
    pub parameter_name: String,
    pub specs: Vec<KombiJoinSpec>,
}

impl KombiJoinBundle {
    pub fn snapshot(&self) -> KombiJoinSnapshot {
        KombiJoinSnapshot {
            class: "KombiJoinBundle".to_string(),
            parameter_name: self.parameter_name.clone(),
            count: self.specs.len(),
            morphisms: self.specs.iter().map(|spec| spec.method_name.clone()).collect(),
        }
    }

    pub fn prepare_table_join(
        &self,
        chosen_kombi_lines: &BTreeMap<i64, BTreeSet<i64>>,
        new_table_kombi_1: &[Vec<String>],
    ) -> Vec<KombiSubTable> {
        prepare_table_join(chosen_kombi_lines, new_table_kombi_1)
    }

    pub fn table_join(
        &self,
        main_table: &[Vec<String>],
        many_sub_tables: &[KombiSubTable],
        relation: &BTreeMap<i64, BTreeSet<i64>>,
    ) -> Vec<Vec<String>> {
        table_join(main_table, many_sub_tables, relation)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KombiJoinSnapshot {
    pub class: String,
    pub parameter_name: String,
    pub count: usize,
    pub morphisms: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KombiSubTable {
    pub main_line: i64,
    pub rows: Vec<Vec<String>>,
}

pub fn bootstrap_combi_join() -> KombiJoinBundle {
    KombiJoinBundle {
        parameter_name: "kombination".to_string(),
        specs: vec![
            KombiJoinSpec::new("prepareTableJoin", "Reduce kombi rows into subtables keyed by main table line."),
            KombiJoinSpec::new("removeOneNumber", "Remove a selected number from a textual combination cell."),
            KombiJoinSpec::new("tableJoin", "Glue main table rows with prepared kombi subtables."),
            KombiJoinSpec::new("tableJoinPrep", "Prepare final relation shape for table output."),
        ],
    }
}

pub fn prepare_table_join(
    chosen_kombi_lines: &BTreeMap<i64, BTreeSet<i64>>,
    new_table_kombi_1: &[Vec<String>],
) -> Vec<KombiSubTable> {
    let mut out = Vec::new();
    for (main_line, kombi_lines) in chosen_kombi_lines {
        let mut rows = Vec::new();
        for kombi_line in kombi_lines {
            if *kombi_line <= 0 {
                continue;
            }
            let index = (*kombi_line as usize).saturating_sub(1);
            if let Some(row) = new_table_kombi_1.get(index) {
                rows.push(row.clone());
            }
        }
        if !rows.is_empty() {
            out.push(KombiSubTable { main_line: *main_line, rows });
        }
    }
    out
}

pub fn remove_one_number(input: &[String], col_num: i64) -> Vec<String> {
    let needle = col_num.abs().to_string();
    input
        .iter()
        .map(|cell| remove_number_from_cell(cell, &needle))
        .filter(|cell| !cell.trim().is_empty())
        .collect()
}

pub fn remove_number_from_cell(cell: &str, needle: &str) -> String {
    let mut result = Vec::new();
    for group in cell.split('|') {
        let numbers = group
            .trim_matches(|ch| ch == '(' || ch == ')' || ch == ' ')
            .split('/')
            .filter(|part| !part.is_empty() && *part != needle)
            .collect::<Vec<_>>();
        if !numbers.is_empty() {
            result.push(numbers.join("/"));
        }
    }
    result.join("|")
}

pub fn table_join(
    main_table: &[Vec<String>],
    many_sub_tables: &[KombiSubTable],
    relation: &BTreeMap<i64, BTreeSet<i64>>,
) -> Vec<Vec<String>> {
    let sub_by_main = many_sub_tables
        .iter()
        .map(|sub| (sub.main_line, sub.rows.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut out = Vec::new();
    for (index, row) in main_table.iter().enumerate() {
        let line_number = index as i64 + 1;
        let mut joined = row.clone();
        if let Some(kombi_lines) = relation.get(&line_number) {
            for kombi_line in kombi_lines {
                if let Some(rows) = sub_by_main.get(kombi_line) {
                    for sub_row in rows {
                        joined.extend(sub_row.clone());
                    }
                }
            }
        }
        out.push(joined);
    }
    out
}

pub fn rows_of_combi_from_relation(relation: &BTreeMap<i64, BTreeSet<i64>>) -> BTreeSet<i64> {
    relation.values().flatten().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepares_subtables_by_line_number() {
        let table = vec![vec!["a".to_string()], vec!["b".to_string()]];
        let chosen = BTreeMap::from([(1, BTreeSet::from([2]))]);
        let out = prepare_table_join(&chosen, &table);
        assert_eq!(out[0].rows[0][0], "b");
    }

    #[test]
    fn remove_one_number_preserves_other_numbers() {
        assert_eq!(remove_number_from_cell("(1/2|3)", "2"), "1|3");
    }
}
