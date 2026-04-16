use std::fs;
use std::path::PathBuf;
use super::super::generator_registry::{Table, SimpleFraction};

pub fn find_header_index_casefold(headers: &[String], wanted: &str) -> Option<usize> {
    let w = wanted.to_lowercase();
    headers.iter().position(|h| h.to_lowercase() == w)
}

pub fn cell_by_header(table: &Table, row: usize, wanted: &str) -> String {
    if table.is_empty() || row >= table.len() { return String::new(); }
    let Some(col) = find_header_index_casefold(&table[0], wanted) else { return String::new(); };
    table.get(row).and_then(|r| r.get(col)).cloned().unwrap_or_default()
}

pub fn csv_path(name: &str) -> PathBuf { PathBuf::from("csv").join(name) }

pub fn read_semicolon_csv(name: &str) -> Vec<Vec<String>> {
    let content = fs::read_to_string(csv_path(name)).unwrap_or_default();
    content.lines().map(|line| line.split(';').map(|s| s.to_string()).collect()).collect()
}

pub fn transpose_csv(table: &[Vec<String>]) -> Vec<Vec<String>> {
    let width = table.iter().map(|row| row.len()).max().unwrap_or(0);
    (0..width)
        .map(|col| table.iter().map(|row| row.get(col).cloned().unwrap_or_default()).collect())
        .collect()
}

pub fn get_all_brueche(table: &[Vec<String>]) -> Vec<SimpleFraction> {
    let mut out = Vec::new();
    for row in table {
        for cell in row {
            let parts: Vec<&str> = cell.trim().split('/').collect();
            if parts.len() == 2 {
                if let (Ok(a), Ok(b)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                    if let Some(f) = SimpleFraction::new(a, b) { out.push(f); }
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}
