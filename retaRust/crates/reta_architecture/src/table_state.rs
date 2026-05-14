//! Mutable table-state sections transcompiled from
//! `python_arch_reference/reta_architecture/table_state.py`.
//!
//! The Python architecture keeps these sections mutable because the historical
//! runtime mutates table state in-place.  The Rust version names the same
//! sections explicitly and gives later table/output ports a single owner for
//! highest-row bounds, generated-column metadata and display flags.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub fn highest_rows(highest_row: Option<i64>) -> BTreeMap<i64, i64> {
    match highest_row {
        Some(value) => BTreeMap::from([(1024, value), (114, value)]),
        None => BTreeMap::from([(1024, 1024), (114, 163)]),
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratedColumnSection {
    pub parameters: BTreeMap<i64, Vec<String>>,
    pub tags: BTreeMap<i64, Vec<String>>,
}

impl GeneratedColumnSection {
    pub fn snapshot(&self) -> GeneratedColumnSectionSnapshot {
        GeneratedColumnSectionSnapshot {
            class: "GeneratedColumnSection".to_string(),
            parameters_len: self.parameters.len(),
            tags_len: self.tags.len(),
            parameters_type: "BTreeMap".to_string(),
            tags_type: "BTreeMap".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratedColumnSectionSnapshot {
    pub class: String,
    pub parameters_len: usize,
    pub tags_len: usize,
    pub parameters_type: String,
    pub tags_type: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableDisplayState {
    pub keine_ueberschriften: bool,
    pub keine_leeren_inhalte: bool,
    pub spalte_gestirn: bool,
    pub religion_numbers: Vec<i64>,
}

impl TableDisplayState {
    pub fn snapshot(&self) -> TableDisplayStateSnapshot {
        TableDisplayStateSnapshot {
            class: "TableDisplayState".to_string(),
            keine_ueberschriften: self.keine_ueberschriften,
            keine_leeren_inhalte: self.keine_leeren_inhalte,
            spalte_gestirn: self.spalte_gestirn,
            religion_numbers_len: self.religion_numbers.len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableDisplayStateSnapshot {
    pub class: String,
    pub keine_ueberschriften: bool,
    pub keine_leeren_inhalte: bool,
    pub spalte_gestirn: bool,
    pub religion_numbers_len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableStateSections {
    pub highest_rows: BTreeMap<i64, i64>,
    pub display: TableDisplayState,
    pub generated_columns: GeneratedColumnSection,
    pub row_display_to_original: BTreeMap<i64, i64>,
    pub generated_rows: BTreeSet<i64>,
}

impl TableStateSections {
    pub fn new(highest_row: Option<i64>) -> Self {
        Self {
            highest_rows: highest_rows(highest_row),
            display: TableDisplayState::default(),
            generated_columns: GeneratedColumnSection::default(),
            row_display_to_original: BTreeMap::new(),
            generated_rows: BTreeSet::new(),
        }
    }

    pub fn snapshot(&self) -> TableStateSectionsSnapshot {
        TableStateSectionsSnapshot {
            class: "TableStateSections".to_string(),
            highest_rows: self.highest_rows.clone(),
            display: self.display.snapshot(),
            generated_columns: self.generated_columns.snapshot(),
            row_display_to_original_len: self.row_display_to_original.len(),
            generated_rows_len: self.generated_rows.len(),
            generated_rows_factory: "BTreeSet".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableStateSectionsSnapshot {
    pub class: String,
    pub highest_rows: BTreeMap<i64, i64>,
    pub display: TableDisplayStateSnapshot,
    pub generated_columns: GeneratedColumnSectionSnapshot,
    pub row_display_to_original_len: usize,
    pub generated_rows_len: usize,
    pub generated_rows_factory: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableStateBundle;

impl TableStateBundle {
    pub fn create_sections(&self, highest_row: Option<i64>) -> TableStateSections {
        TableStateSections::new(highest_row)
    }

    pub fn snapshot(&self) -> TableStateBundleSnapshot {
        TableStateBundleSnapshot {
            class: "TableStateBundle".to_string(),
            sections: vec![
                "highest_rows".to_string(),
                "display".to_string(),
                "generated_columns".to_string(),
                "row_display_to_original".to_string(),
                "generated_rows".to_string(),
            ],
            architecture_owner: "reta_architecture.table_state".to_string(),
            legacy_owner: "reta_architecture.table_runtime.Tables".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableStateBundleSnapshot {
    pub class: String,
    pub sections: Vec<String>,
    pub architecture_owner: String,
    pub legacy_owner: String,
}

pub fn bootstrap_table_state() -> TableStateBundle {
    TableStateBundle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_highest_rows_match_python() {
        assert_eq!(highest_rows(None), BTreeMap::from([(1024, 1024), (114, 163)]));
        assert_eq!(highest_rows(Some(42)), BTreeMap::from([(1024, 42), (114, 42)]));
    }

    #[test]
    fn created_sections_share_named_state() {
        let sections = bootstrap_table_state().create_sections(None);
        let snapshot = sections.snapshot();
        assert_eq!(snapshot.highest_rows[&1024], 1024);
        assert_eq!(snapshot.generated_columns.parameters_len, 0);
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "_highest_rows",
    "new_generated_rows",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
