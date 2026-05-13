#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub const PYTHON_SOURCE__LIB4TABLES_ENUM: &str = include_str!("../../python_reference/lib4tables_Enum.py");

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ST {
    sternPolygon,
    universum,
    galaxie,
    gleichfoermigesPolygon,
    gebrRat,
    keinParaOdMetaP,
    keinPolygon,
}

impl ST {
    pub const fn py_name(self) -> &'static str {
        match self {
            Self::sternPolygon => "sternPolygon",
            Self::universum => "universum",
            Self::galaxie => "galaxie",
            Self::gleichfoermigesPolygon => "gleichfoermigesPolygon",
            Self::gebrRat => "gebrRat",
            Self::keinParaOdMetaP => "keinParaOdMetaP",
            Self::keinPolygon => "keinPolygon",
        }
    }

    pub const fn py_value(self) -> i64 {
        match self {
            Self::sternPolygon => 0,
            Self::gleichfoermigesPolygon => 1,
            Self::keinPolygon => 2,
            Self::galaxie => 3,
            Self::universum => 4,
            Self::keinParaOdMetaP => 5,
            Self::gebrRat => 6,
        }
    }

    pub fn from_py_value(value: i64) -> Option<Self> {
        match value {
            0 => Some(Self::sternPolygon),
            1 => Some(Self::gleichfoermigesPolygon),
            2 => Some(Self::keinPolygon),
            3 => Some(Self::galaxie),
            4 => Some(Self::universum),
            5 => Some(Self::keinParaOdMetaP),
            6 => Some(Self::gebrRat),
            _ => None,
        }
    }

    pub fn from_py_name(value: &str) -> Option<Self> {
        match value.trim() {
            "sternPolygon" => Some(Self::sternPolygon),
            "universum" => Some(Self::universum),
            "galaxie" => Some(Self::galaxie),
            "gleichfoermigesPolygon" => Some(Self::gleichfoermigesPolygon),
            "gebrRat" => Some(Self::gebrRat),
            "keinParaOdMetaP" => Some(Self::keinParaOdMetaP),
            "keinPolygon" => Some(Self::keinPolygon),
            _ => None,
        }
    }

    pub fn html_class(self) -> String {
        format!("p4_{}", self.py_name())
    }
}

impl fmt::Display for ST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.py_name())
    }
}


pub type TableTags = BTreeMap<BTreeSet<ST>, BTreeSet<i64>>;
pub type TableTags2 = BTreeMap<i64, BTreeSet<ST>>;

fn st_from_arch_tag(tag: reta_architecture::TableTag) -> ST {
    match tag {
        reta_architecture::TableTag::sternPolygon => ST::sternPolygon,
        reta_architecture::TableTag::gleichfoermigesPolygon => ST::gleichfoermigesPolygon,
        reta_architecture::TableTag::keinPolygon => ST::keinPolygon,
        reta_architecture::TableTag::galaxie => ST::galaxie,
        reta_architecture::TableTag::universum => ST::universum,
        reta_architecture::TableTag::keinParaOdMetaP => ST::keinParaOdMetaP,
        reta_architecture::TableTag::gebrRat => ST::gebrRat,
    }
}

fn tag_set_from_arch(tags: BTreeSet<reta_architecture::TableTag>) -> BTreeSet<ST> {
    tags.into_iter().map(st_from_arch_tag).collect()
}

fn group_reverse_tags(table_tags2: &TableTags2) -> TableTags {
    let mut grouped: TableTags = BTreeMap::new();
    for (column, tags) in table_tags2 {
        grouped.entry(tags.clone()).or_default().insert(*column);
    }
    grouped
}

/// Python `dictViceversa(dic)`: turn `{frozenset(ST): {columns...}}` into
/// `{column: frozenset(ST)}`.  Python overwrites earlier entries when a column
/// appears in multiple tag groups; the ordered Rust map intentionally mirrors
/// that final effective value.
pub fn dictViceversa(dic: &TableTags) -> TableTags2 {
    let mut new_dict = TableTags2::new();
    for (key, value) in dic {
        for number in value {
            new_dict.insert(*number, key.clone());
        }
    }
    new_dict
}

pub fn tableTags2_for_column(column_number: i64) -> Option<BTreeSet<ST>> {
    if column_number < 0 {
        return None;
    }
    reta_architecture::ordinary_tags_for_column(column_number).map(tag_set_from_arch)
}

pub fn tableTags2() -> TableTags2 {
    let mut out = TableTags2::new();
    // Python's ordinary `tableTags` currently covers columns below 520.  The
    // wider scan keeps this facade stable if the generated exact table grows.
    for column in 0..=2048i64 {
        if let Some(tags) = tableTags2_for_column(column) {
            out.insert(column, tags);
        }
    }
    out
}

/// Effective ordinary `tableTags` grouped back from `tableTags2`.  This is the
/// representation active consumers need after Python's duplicate-column
/// overwrite semantics have been applied by `dictViceversa`.
pub fn tableTags() -> TableTags {
    group_reverse_tags(&tableTags2())
}

pub fn tableTags_columns_for_tags<I>(tags: I) -> BTreeSet<i64>
where
    I: IntoIterator<Item = ST>,
{
    let wanted: BTreeSet<ST> = tags.into_iter().collect();
    tableTags()
        .get(&wanted)
        .cloned()
        .unwrap_or_default()
}

pub fn tableTags2_kombiTable_for_column(column_number: i64) -> Option<BTreeSet<ST>> {
    if column_number < 0 {
        return None;
    }
    reta_architecture::kombi_table_tags_for_column(column_number).map(tag_set_from_arch)
}

pub fn tableTags2_kombiTable() -> TableTags2 {
    let mut out = TableTags2::new();
    for column in 0..=256i64 {
        if let Some(tags) = tableTags2_kombiTable_for_column(column) {
            out.insert(column, tags);
        }
    }
    out
}

pub fn tableTags_kombiTable() -> TableTags {
    group_reverse_tags(&tableTags2_kombiTable())
}

pub fn tableTags_kombiTable_columns_for_tags<I>(tags: I) -> BTreeSet<i64>
where
    I: IntoIterator<Item = ST>,
{
    let wanted: BTreeSet<ST> = tags.into_iter().collect();
    tableTags_kombiTable()
        .get(&wanted)
        .cloned()
        .unwrap_or_default()
}

pub fn tableTags2_kombiTable2_for_column(column_number: i64) -> Option<BTreeSet<ST>> {
    if column_number < 0 {
        return None;
    }
    reta_architecture::kombi_table2_tags_for_column(column_number).map(tag_set_from_arch)
}

pub fn tableTags2_kombiTable2() -> TableTags2 {
    let mut out = TableTags2::new();
    for column in 0..=256i64 {
        if let Some(tags) = tableTags2_kombiTable2_for_column(column) {
            out.insert(column, tags);
        }
    }
    out
}

pub fn tableTags_kombiTable2() -> TableTags {
    group_reverse_tags(&tableTags2_kombiTable2())
}

pub fn tableTags_kombiTable2_columns_for_tags<I>(tags: I) -> BTreeSet<i64>
where
    I: IntoIterator<Item = ST>,
{
    let wanted: BTreeSet<ST> = tags.into_iter().collect();
    tableTags_kombiTable2()
        .get(&wanted)
        .cloned()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn st_python_enum_values_match_lib4tables_enum_py() {
        assert_eq!(ST::sternPolygon.py_value(), 0);
        assert_eq!(ST::gleichfoermigesPolygon.py_value(), 1);
        assert_eq!(ST::keinPolygon.py_value(), 2);
        assert_eq!(ST::galaxie.py_value(), 3);
        assert_eq!(ST::universum.py_value(), 4);
        assert_eq!(ST::keinParaOdMetaP.py_value(), 5);
        assert_eq!(ST::gebrRat.py_value(), 6);
        assert_eq!(ST::from_py_value(2), Some(ST::keinPolygon));
        assert_eq!(ST::from_py_name("keinPolygon"), Some(ST::keinPolygon));
        assert!(PYTHON_SOURCE__LIB4TABLES_ENUM.contains("tableTags2 = dictViceversa(tableTags)"));
    }

    fn set(tags: &[ST]) -> std::collections::BTreeSet<ST> {
        tags.iter().copied().collect()
    }

    #[test]
    fn ordinary_table_tags2_matches_python_effective_duplicate_overwrite() {
        assert_eq!(
            tableTags2_for_column(14),
            Some(set(&[ST::sternPolygon, ST::galaxie]))
        );
        assert_eq!(
            tableTags2_for_column(370),
            Some(set(&[ST::keinParaOdMetaP, ST::sternPolygon, ST::galaxie]))
        );
        assert_eq!(
            tableTags2_for_column(744),
            Some(set(&[ST::keinParaOdMetaP, ST::sternPolygon]))
        );

        let star_galaxy_columns = tableTags_columns_for_tags([ST::sternPolygon, ST::galaxie]);
        assert!(star_galaxy_columns.contains(&14));
        assert!(star_galaxy_columns.contains(&0));
        assert!(tableTags2().len() > 400);
    }

    #[test]
    fn kombi_table_tags_match_python_kombi_reverse_maps() {
        assert_eq!(
            tableTags2_kombiTable_for_column(5),
            Some(set(&[
                ST::universum,
                ST::gleichfoermigesPolygon,
                ST::sternPolygon,
                ST::galaxie,
            ]))
        );
        assert_eq!(
            tableTags2_kombiTable2_for_column(1),
            Some(set(&[
                ST::universum,
                ST::gleichfoermigesPolygon,
                ST::sternPolygon,
            ]))
        );
        assert!(tableTags_kombiTable_columns_for_tags([
            ST::gleichfoermigesPolygon,
            ST::sternPolygon,
            ST::galaxie,
        ])
        .contains(&1));
        assert!(tableTags_kombiTable2_columns_for_tags([
            ST::universum,
            ST::gleichfoermigesPolygon,
            ST::sternPolygon,
        ])
        .contains(&18));
    }

    #[test]
    fn dict_viceversa_keeps_python_last_writer_semantics() {
        let mut grouped = TableTags::new();
        grouped.insert(set(&[ST::universum]), [7].into_iter().collect());
        grouped.insert(set(&[ST::sternPolygon]), [7].into_iter().collect());

        let reversed = dictViceversa(&grouped);
        // BTreeMap iteration is deterministic.  Like Python's loop, the later
        // visited tag group wins for duplicate columns.
        assert_eq!(reversed.get(&7), Some(&set(&[ST::universum])));
    }
}
