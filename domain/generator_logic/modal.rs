use std::collections::BTreeSet;
use super::super::generator_registry::{Table, Tables};

pub fn get_modaloperators_per_line_cells(table: &Table, line: usize) -> Vec<String> {
    if line >= table.len() { return Vec::new(); }
    table[line]
        .iter()
        .filter(|s| !s.trim().is_empty())
        .cloned()
        .collect()
}

pub fn prepare_modal_into_table(
    _table: &mut Table,
    _concepts_rows_set_of_tuple: &BTreeSet<(usize, usize)>,
    _tables: &mut Tables,
) {
    // Stage 5 extraction point: orchestration remains in generator_registry.rs.
}
