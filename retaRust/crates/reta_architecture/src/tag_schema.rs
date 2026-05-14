#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//! Rust owner for the architecture tag schema transcompiled from
//! `python_arch_reference/reta_architecture/tag_schema.py`.
//!
//! The important detail is Python's `dictViceversa` overwrite semantics:
//! when a column occurs in multiple tag groups, the later group in the Python
//! dictionary wins for the reverse map.  `tags_for_column` below scans the
//! generated groups in order and intentionally keeps the last match.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TableTag {
    sternPolygon,
    gleichfoermigesPolygon,
    keinPolygon,
    galaxie,
    universum,
    keinParaOdMetaP,
    gebrRat,
}

impl TableTag {
    pub const fn py_name(self) -> &'static str {
        match self {
            Self::sternPolygon => "sternPolygon",
            Self::gleichfoermigesPolygon => "gleichfoermigesPolygon",
            Self::keinPolygon => "keinPolygon",
            Self::galaxie => "galaxie",
            Self::universum => "universum",
            Self::keinParaOdMetaP => "keinParaOdMetaP",
            Self::gebrRat => "gebrRat",
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

    pub fn from_py_name(value: &str) -> Option<Self> {
        match value.trim() {
            "sternPolygon" => Some(Self::sternPolygon),
            "gleichfoermigesPolygon" => Some(Self::gleichfoermigesPolygon),
            "keinPolygon" => Some(Self::keinPolygon),
            "galaxie" => Some(Self::galaxie),
            "universum" => Some(Self::universum),
            "keinParaOdMetaP" => Some(Self::keinParaOdMetaP),
            "gebrRat" => Some(Self::gebrRat),
            _ => None,
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TagTableSelector {
    Ordinary,
    KombiTable,
    KombiTable2,
}

impl TagTableSelector {
    pub const fn from_python_selector(value: Option<i64>) -> Option<Self> {
        match value {
            None => Some(Self::Ordinary),
            Some(0) => Some(Self::KombiTable),
            Some(1) => Some(Self::KombiTable2),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TagGroup {
    pub tags: &'static [TableTag],
    pub columns: &'static [i64],
}

pub const ORDINARY_TAG_GROUPS: &[TagGroup] = &[
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::galaxie, TableTag::keinParaOdMetaP], columns: &[241, 370, 394, 395, 411, 424, 492, 493] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::gleichfoermigesPolygon, TableTag::universum, TableTag::keinParaOdMetaP], columns: &[14] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::galaxie, TableTag::universum, TableTag::keinParaOdMetaP], columns: &[4, 15, 17, 20, 21, 26, 36, 48, 100, 101, 102, 103, 114, 115, 116, 117, 120, 123, 124, 125, 126, 127, 128, 129, 130, 137, 140, 141, 142, 143, 144, 222, 318, 422, 495] },
    TagGroup { tags: &[TableTag::gleichfoermigesPolygon, TableTag::galaxie, TableTag::universum, TableTag::keinParaOdMetaP], columns: &[37, 197, 313, 319, 328, 331, 335] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::gleichfoermigesPolygon, TableTag::keinParaOdMetaP], columns: &[] },
    TagGroup { tags: &[TableTag::gleichfoermigesPolygon, TableTag::galaxie, TableTag::keinParaOdMetaP], columns: &[272, 379] },
    TagGroup { tags: &[TableTag::gleichfoermigesPolygon, TableTag::keinParaOdMetaP], columns: &[257, 284, 285, 326, 327, 330, 332, 334, 342, 352, 378, 392, 400, 401, 416, 428] },
    TagGroup { tags: &[TableTag::gleichfoermigesPolygon, TableTag::universum, TableTag::keinParaOdMetaP], columns: &[205, 346, 484, 499] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::universum, TableTag::keinParaOdMetaP], columns: &[107, 132, 204, 213, 214, 230, 235, 240, 264, 314, 351, 385, 387, 473, 489, 490, 497, 498, 509, 512, 513] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::keinParaOdMetaP], columns: &[8, 9, 28, 208, 232, 233, 234, 243, 249, 250, 251, 252, 253, 254, 255, 256, 260, 261, 262, 263, 265, 266, 267, 268, 269, 270, 271, 272, 276, 281, 282, 283, 286, 287, 288, 289, 290, 293, 294, 295, 296, 298, 299, 300, 301, 302, 305, 306, 309, 310, 311, 312, 317, 321, 322, 323, 324, 325, 333, 336, 337, 338, 339, 340, 341, 343, 344, 345, 347, 348, 349, 350, 353, 354, 356, 357, 377, 384, 388, 389, 391, 393, 396, 397, 398, 399, 402, 403, 404, 405, 406, 407, 408, 410, 412, 413, 414, 415, 417, 418, 419, 420, 421, 423, 425, 427, 431, 432, 433, 434, 435, 436, 437, 438, 439, 441, 442, 443, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 456, 457, 458, 459, 460, 461, 467, 468, 469, 482, 485, 486, 487, 488, 491, 494, 496, 501, 502, 503, 504, 505, 507, 508, 510, 511, 514, 515, 516, 517, 518, 519, 744] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::galaxie], columns: &[0, 1, 2, 3, 6, 7, 10, 11, 12, 13, 14, 18, 19, 22, 23, 24, 29, 30, 31, 32, 33, 34, 35, 38, 39, 40, 41, 43, 44, 45, 46, 47, 49, 50, 51, 56, 57, 59, 60, 61, 62, 63, 64, 66, 67, 68, 71, 72, 73, 74, 78, 79, 82, 83, 85, 86, 88, 89, 90, 91, 92, 95, 96, 97, 98, 99, 105, 106, 108, 109, 110, 111, 112, 113, 118, 119, 121, 122, 133, 134, 136, 139, 146, 147, 151, 152, 153, 159, 160, 163, 164, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 189, 192, 193, 194, 195, 199, 200, 207, 211, 212, 215, 217, 274, 275, 303, 307, 315, 316, 429, 455, 470, 471, 472, 476, 477, 478] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::universum], columns: &[5, 25, 27, 55, 65, 69, 70, 77, 80, 81, 84, 93, 94, 104, 138, 158, 169, 190, 191, 196, 198, 202, 206, 209, 210, 219, 223, 229, 230, 242, 244, 297, 304, 308, 320, 382, 386, 390, 409, 426, 444, 462, 474, 475, 479, 480, 481, 482, 500] },
    TagGroup { tags: &[TableTag::gleichfoermigesPolygon, TableTag::galaxie], columns: &[16, 42, 58, 148, 161, 162, 237, 440, 455] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::gleichfoermigesPolygon, TableTag::universum], columns: &[] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::gleichfoermigesPolygon, TableTag::galaxie], columns: &[52, 53, 87, 154, 167, 168] },
    TagGroup { tags: &[TableTag::gleichfoermigesPolygon, TableTag::galaxie, TableTag::universum], columns: &[329] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::galaxie, TableTag::universum], columns: &[54, 75, 76, 135, 145, 149, 150, 155, 156, 157, 165, 166, 188, 218, 220, 226, 463, 464, 465, 466] },
    TagGroup { tags: &[TableTag::gleichfoermigesPolygon, TableTag::universum], columns: &[131, 201, 203, 231, 273, 383] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::gleichfoermigesPolygon, TableTag::galaxie, TableTag::universum], columns: &[216] },
];

pub const KOMBI_TABLE_TAG_GROUPS: &[TagGroup] = &[
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::gleichfoermigesPolygon, TableTag::galaxie], columns: &[1, 2, 3, 7, 8, 9, 10, 12, 13, 16, 17] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::gleichfoermigesPolygon, TableTag::galaxie, TableTag::universum], columns: &[5, 6, 11, 15] },
];

pub const KOMBI_TABLE2_TAG_GROUPS: &[TagGroup] = &[
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::gleichfoermigesPolygon, TableTag::galaxie, TableTag::universum], columns: &[5] },
    TagGroup { tags: &[TableTag::sternPolygon, TableTag::gleichfoermigesPolygon, TableTag::universum], columns: &[1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 15, 16, 17, 18] },
];


pub fn groups_for_selector(selector: TagTableSelector) -> &'static [TagGroup] {
    match selector {
        TagTableSelector::Ordinary => ORDINARY_TAG_GROUPS,
        TagTableSelector::KombiTable => KOMBI_TABLE_TAG_GROUPS,
        TagTableSelector::KombiTable2 => KOMBI_TABLE2_TAG_GROUPS,
    }
}

pub fn tags_for_column_in_selector(
    column_number: i64,
    selector: TagTableSelector,
) -> Option<BTreeSet<TableTag>> {
    let mut found: Option<BTreeSet<TableTag>> = None;
    for group in groups_for_selector(selector) {
        if group.columns.contains(&column_number) {
            found = Some(group.tags.iter().copied().collect());
        }
    }
    found
}

pub fn ordinary_tags_for_column(column_number: i64) -> Option<BTreeSet<TableTag>> {
    tags_for_column_in_selector(column_number, TagTableSelector::Ordinary)
}

pub fn kombi_table_tags_for_column(column_number: i64) -> Option<BTreeSet<TableTag>> {
    tags_for_column_in_selector(column_number, TagTableSelector::KombiTable)
}

pub fn kombi_table2_tags_for_column(column_number: i64) -> Option<BTreeSet<TableTag>> {
    tags_for_column_in_selector(column_number, TagTableSelector::KombiTable2)
}

pub fn columns_for_tags_in_selector<I>(
    tags: I,
    selector: TagTableSelector,
) -> BTreeSet<i64>
where
    I: IntoIterator<Item = TableTag>,
{
    let wanted: BTreeSet<TableTag> = tags.into_iter().collect();
    for group in groups_for_selector(selector) {
        let actual: BTreeSet<TableTag> = group.tags.iter().copied().collect();
        if actual == wanted {
            return group.columns.iter().copied().collect();
        }
    }
    BTreeSet::new()
}

pub fn ordinary_columns_for_tags<I>(tags: I) -> BTreeSet<i64>
where
    I: IntoIterator<Item = TableTag>,
{
    columns_for_tags_in_selector(tags, TagTableSelector::Ordinary)
}

pub fn reverse_map_for_selector(selector: TagTableSelector) -> BTreeMap<i64, BTreeSet<TableTag>> {
    let mut out = BTreeMap::new();
    for group in groups_for_selector(selector) {
        let tags = group.tags.iter().copied().collect::<BTreeSet<_>>();
        for column in group.columns {
            out.insert(*column, tags.clone());
        }
    }
    out
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TagSchemaSnapshot {
    pub primary_tag_groups: usize,
    pub primary_reverse_entries: usize,
    pub kombi_tag_groups: usize,
    pub kombi_reverse_entries: usize,
    pub kombi2_tag_groups: usize,
    pub kombi2_reverse_entries: usize,
    pub tag_names: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TagSchemaBundle;

impl TagSchemaBundle {
    pub fn snapshot(&self) -> TagSchemaSnapshot {
        TagSchemaSnapshot {
            primary_tag_groups: ORDINARY_TAG_GROUPS.len(),
            primary_reverse_entries: reverse_map_for_selector(TagTableSelector::Ordinary).len(),
            kombi_tag_groups: KOMBI_TABLE_TAG_GROUPS.len(),
            kombi_reverse_entries: reverse_map_for_selector(TagTableSelector::KombiTable).len(),
            kombi2_tag_groups: KOMBI_TABLE2_TAG_GROUPS.len(),
            kombi2_reverse_entries: reverse_map_for_selector(TagTableSelector::KombiTable2).len(),
            tag_names: vec![
                TableTag::sternPolygon.py_name().to_string(),
                TableTag::gleichfoermigesPolygon.py_name().to_string(),
                TableTag::keinPolygon.py_name().to_string(),
                TableTag::galaxie.py_name().to_string(),
                TableTag::universum.py_name().to_string(),
                TableTag::keinParaOdMetaP.py_name().to_string(),
                TableTag::gebrRat.py_name().to_string(),
            ],
        }
    }

    pub fn tags_for_column(
        &self,
        column_number: i64,
        kombi_table: Option<i64>,
    ) -> Result<BTreeSet<TableTag>, String> {
        let Some(selector) = TagTableSelector::from_python_selector(kombi_table) else {
            return Err(format!("Unknown kombi_table selector: {kombi_table:?}"));
        };
        Ok(tags_for_column_in_selector(column_number, selector).unwrap_or_default())
    }

    pub fn columns_for_tags<I>(
        &self,
        tags: I,
        kombi_table: Option<i64>,
    ) -> Result<BTreeSet<i64>, String>
    where
        I: IntoIterator<Item = TableTag>,
    {
        let Some(selector) = TagTableSelector::from_python_selector(kombi_table) else {
            return Err(format!("Unknown kombi_table selector: {kombi_table:?}"));
        };
        Ok(columns_for_tags_in_selector(tags, selector))
    }
}

pub fn bootstrap_tag_schema() -> TagSchemaBundle {
    TagSchemaBundle
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(tags: &[TableTag]) -> BTreeSet<TableTag> {
        tags.iter().copied().collect()
    }

    #[test]
    fn enum_values_match_python_st() {
        assert_eq!(TableTag::sternPolygon.py_value(), 0);
        assert_eq!(TableTag::gleichfoermigesPolygon.py_value(), 1);
        assert_eq!(TableTag::keinPolygon.py_value(), 2);
        assert_eq!(TableTag::galaxie.py_value(), 3);
        assert_eq!(TableTag::universum.py_value(), 4);
        assert_eq!(TableTag::keinParaOdMetaP.py_value(), 5);
        assert_eq!(TableTag::gebrRat.py_value(), 6);
        assert_eq!(TableTag::from_py_name("sternPolygon"), Some(TableTag::sternPolygon));
    }

    #[test]
    fn reverse_map_keeps_python_last_writer_semantics() {
        assert_eq!(
            ordinary_tags_for_column(14),
            Some(set(&[TableTag::sternPolygon, TableTag::galaxie]))
        );
        assert_eq!(
            ordinary_tags_for_column(744),
            Some(set(&[TableTag::keinParaOdMetaP, TableTag::sternPolygon]))
        );
        assert_eq!(
            ordinary_tags_for_column(370),
            Some(set(&[TableTag::keinParaOdMetaP, TableTag::sternPolygon, TableTag::galaxie]))
        );
    }

    #[test]
    fn columns_for_tags_uses_python_forward_table() {
        let cols = ordinary_columns_for_tags([TableTag::keinParaOdMetaP, TableTag::sternPolygon]);
        assert!(cols.contains(&744));
        assert!(cols.contains(&232));
        let special = ordinary_columns_for_tags([TableTag::universum, TableTag::keinParaOdMetaP, TableTag::sternPolygon, TableTag::gleichfoermigesPolygon]);
        assert!(special.contains(&14));
    }

    #[test]
    fn snapshot_counts_match_python_tag_schema() {
        let snapshot = bootstrap_tag_schema().snapshot();
        assert_eq!(snapshot.primary_tag_groups, 19);
        assert_eq!(snapshot.primary_reverse_entries, 477);
        assert_eq!(snapshot.kombi_tag_groups, 2);
        assert_eq!(snapshot.kombi_reverse_entries, 15);
        assert_eq!(snapshot.kombi2_tag_groups, 2);
        assert_eq!(snapshot.kombi2_reverse_entries, 17);
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "ST",
    "dictViceversa",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
