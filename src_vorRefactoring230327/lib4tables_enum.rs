// Automatisch aus libs/lib4tables_Enum.py portiert.
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ST {
    SternPolygon,
    GleichfoermigesPolygon,
    KeinPolygon,
    Galaxie,
    Universum,
    KeinParaOdMetaP,
    GebrRat,
}

fn st_set(items: &[ST]) -> BTreeSet<ST> {
    items.iter().copied().collect()
}

fn usize_set(items: &[usize]) -> BTreeSet<usize> {
    items.iter().copied().collect()
}

pub fn table_tags() -> BTreeMap<BTreeSet<ST>, BTreeSet<usize>> {
    BTreeMap::from([
        (
            st_set(&[ST::SternPolygon, ST::Galaxie, ST::KeinParaOdMetaP]),
            usize_set(&[
                241, 370, 394, 395, 411, 424, 492, 493,
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::GleichfoermigesPolygon, ST::Universum, ST::KeinParaOdMetaP]),
            usize_set(&[
                14,
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::Galaxie, ST::Universum, ST::KeinParaOdMetaP]),
            usize_set(&[
                4, 15, 17, 20, 21, 26, 36, 48, 100, 101, 102, 103,
                114, 115, 116, 117, 120, 123, 124, 125, 126, 127, 128, 129,
                130, 137, 140, 141, 142, 143, 144, 222, 318, 422, 495,
            ]),
        ),
        (
            st_set(&[ST::GleichfoermigesPolygon, ST::Galaxie, ST::Universum, ST::KeinParaOdMetaP]),
            usize_set(&[
                37, 197, 313, 319, 328, 331, 335,
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::GleichfoermigesPolygon, ST::KeinParaOdMetaP]),
            usize_set(&[
            ]),
        ),
        (
            st_set(&[ST::GleichfoermigesPolygon, ST::Galaxie, ST::KeinParaOdMetaP]),
            usize_set(&[
                272, 379,
            ]),
        ),
        (
            st_set(&[ST::GleichfoermigesPolygon, ST::KeinParaOdMetaP]),
            usize_set(&[
                257, 284, 285, 326, 327, 330, 332, 334, 342, 352, 378, 392,
                400, 401, 416, 428,
            ]),
        ),
        (
            st_set(&[ST::GleichfoermigesPolygon, ST::Universum, ST::KeinParaOdMetaP]),
            usize_set(&[
                205, 346, 484, 499,
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::Universum, ST::KeinParaOdMetaP]),
            usize_set(&[
                107, 132, 204, 213, 214, 230, 235, 240, 264, 314, 351, 385,
                387, 473, 489, 490, 497, 498, 509, 512, 513,
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::KeinParaOdMetaP]),
            usize_set(&[
                8, 9, 28, 208, 232, 233, 234, 243, 249, 250, 251, 252,
                253, 254, 255, 256, 260, 261, 262, 263, 265, 266, 267, 268,
                269, 270, 271, 272, 276, 281, 282, 283, 286, 287, 288, 289,
                290, 293, 294, 295, 296, 298, 299, 300, 301, 302, 305, 306,
                309, 310, 311, 312, 317, 321, 322, 323, 324, 325, 333, 336,
                337, 338, 339, 340, 341, 343, 344, 345, 347, 348, 349, 350,
                353, 354, 356, 357, 377, 384, 388, 389, 391, 393, 396, 397,
                398, 399, 402, 403, 404, 405, 406, 407, 408, 410, 412, 413,
                414, 415, 417, 418, 419, 420, 421, 423, 425, 427, 431, 432,
                433, 434, 435, 436, 437, 438, 439, 441, 442, 443, 445, 446,
                447, 448, 449, 450, 451, 452, 453, 454, 456, 457, 458, 459,
                460, 461, 467, 468, 469, 482, 485, 486, 487, 488, 491, 494,
                496, 501, 502, 503, 504, 505, 507, 508, 510, 511, 514, 515,
                516, 517, 518, 519,
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::Galaxie]),
            usize_set(&[
                0, 1, 2, 3, 6, 7, 10, 11, 12, 13, 14, 18,
                19, 22, 23, 24, 29, 30, 31, 32, 33, 34, 35, 38,
                39, 40, 41, 43, 44, 45, 46, 47, 49, 50, 51, 56,
                57, 59, 60, 61, 62, 63, 64, 66, 67, 68, 71, 72,
                73, 74, 78, 79, 82, 83, 85, 86, 88, 89, 90, 91,
                92, 95, 96, 97, 98, 99, 105, 106, 108, 109, 110, 111,
                112, 113, 118, 119, 121, 122, 133, 134, 136, 139, 146, 147,
                151, 152, 153, 159, 160, 163, 164, 170, 171, 172, 173, 174,
                175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186,
                187, 189, 192, 193, 194, 195, 199, 200, 207, 211, 212, 215,
                217, 274, 275, 303, 307, 315, 316, 429, 455, 470, 471, 472,
                476, 477, 478,
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::Universum]),
            usize_set(&[
                5, 25, 27, 55, 65, 69, 70, 77, 80, 81, 84, 93,
                94, 104, 138, 158, 169, 190, 191, 196, 198, 202, 206, 209,
                210, 219, 223, 229, 230, 242, 244, 297, 304, 308, 320, 382,
                386, 390, 409, 426, 444, 462, 474, 475, 479, 480, 481, 482,
                500,
            ]),
        ),
        (
            st_set(&[ST::GleichfoermigesPolygon, ST::Galaxie]),
            usize_set(&[
                16, 42, 58, 148, 161, 162, 237, 440, 455,
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::GleichfoermigesPolygon, ST::Universum]),
            usize_set(&[
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::GleichfoermigesPolygon, ST::Galaxie]),
            usize_set(&[
                52, 53, 87, 154, 167, 168,
            ]),
        ),
        (
            st_set(&[ST::GleichfoermigesPolygon, ST::Galaxie, ST::Universum]),
            usize_set(&[
                329,
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::Galaxie, ST::Universum]),
            usize_set(&[
                54, 75, 76, 135, 145, 149, 150, 155, 156, 157, 165, 166,
                188, 218, 220, 226, 463, 464, 465, 466,
            ]),
        ),
        (
            st_set(&[ST::GleichfoermigesPolygon, ST::Universum]),
            usize_set(&[
                131, 201, 203, 231, 273, 383,
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::GleichfoermigesPolygon, ST::Galaxie, ST::Universum]),
            usize_set(&[
                216,
            ]),
        ),
    ])
}

pub fn table_tags_kombi_table() -> BTreeMap<BTreeSet<ST>, BTreeSet<usize>> {
    BTreeMap::from([
        (
            st_set(&[ST::SternPolygon, ST::GleichfoermigesPolygon, ST::Galaxie]),
            usize_set(&[
                1, 2, 3, 7, 8, 9, 10, 12, 13, 16, 17,
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::GleichfoermigesPolygon, ST::Galaxie, ST::Universum]),
            usize_set(&[
                5, 6, 11, 15,
            ]),
        ),
    ])
}

pub fn table_tags_kombi_table2() -> BTreeMap<BTreeSet<ST>, BTreeSet<usize>> {
    BTreeMap::from([
        (
            st_set(&[ST::SternPolygon, ST::GleichfoermigesPolygon, ST::Galaxie, ST::Universum]),
            usize_set(&[
                5,
            ]),
        ),
        (
            st_set(&[ST::SternPolygon, ST::GleichfoermigesPolygon, ST::Universum]),
            usize_set(&[
                1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13,
                15, 16, 17, 18,
            ]),
        ),
    ])
}

pub fn dict_viceversa(dic: &BTreeMap<BTreeSet<ST>, BTreeSet<usize>>) -> BTreeMap<usize, BTreeSet<ST>> {
    let mut new_dict = BTreeMap::new();
    for (key, values) in dic {
        for number in values {
            new_dict.insert(*number, key.clone());
        }
    }
    new_dict
}

pub fn table_tags2() -> BTreeMap<usize, BTreeSet<ST>> {
    dict_viceversa(&table_tags())
}

pub fn table_tags2_kombi_table() -> BTreeMap<usize, BTreeSet<ST>> {
    dict_viceversa(&table_tags_kombi_table())
}

pub fn table_tags2_kombi_table2() -> BTreeMap<usize, BTreeSet<ST>> {
    dict_viceversa(&table_tags_kombi_table2())
}
fn st_to_tag(st: ST) -> u8 {
    match st {
        ST::SternPolygon => 0,
        ST::GleichfoermigesPolygon => 1,
        ST::KeinPolygon => 2,
        ST::Galaxie => 3,
        ST::Universum => 4,
        ST::KeinParaOdMetaP => 5,
        ST::GebrRat => 6,
    }
}

pub fn p4_fragment_for_column(col: u32) -> String {
    let tags_map = table_tags2();
    let Some(tags) = tags_map.get(&(col as usize)) else {
        return String::new();
    };

    let mut nums: Vec<u8> = tags.iter().copied().map(st_to_tag).collect();
    nums.sort_unstable();

    nums.into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
