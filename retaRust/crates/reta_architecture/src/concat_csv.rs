//! CSV and fractional gluing morphisms transcompiled from
//! `python_arch_reference/reta_architecture/concat_csv.py`.
//!
//! The Python module glues external CSV presheaf sections and fraction-indexed
//! generated columns.  This Rust module owns the typed helper morphisms: pair
//! maps by division/multiplication, fraction-pair expansion and deterministic
//! dictionary union.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::meta_columns::{gcd_i64, Rational};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConcatCsvSpec {
    pub method_name: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl ConcatCsvSpec {
    pub fn new(method_name: &str, description: &str, tags: &[&str]) -> Self {
        Self {
            method_name: method_name.to_string(),
            description: description.to_string(),
            tags: tags.iter().map(|item| item.to_string()).collect(),
        }
    }

    pub fn snapshot(&self) -> ConcatCsvSpecSnapshot {
        ConcatCsvSpecSnapshot {
            method_name: self.method_name.clone(),
            description: self.description.clone(),
            tags: self.tags.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConcatCsvSpecSnapshot {
    pub method_name: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConcatCsvBundle {
    pub specs: Vec<ConcatCsvSpec>,
    pub csv_sources: Vec<String>,
    pub fraction_helpers: Vec<String>,
}

impl ConcatCsvBundle {
    pub fn snapshot(&self) -> ConcatCsvSnapshot {
        ConcatCsvSnapshot {
            class: "ConcatCsvBundle".to_string(),
            count: self.specs.len(),
            morphisms: self.specs.iter().map(ConcatCsvSpec::snapshot).collect(),
            csv_sources: self.csv_sources.clone(),
            fraction_helpers: self.fraction_helpers.clone(),
        }
    }

    pub fn combine_dicts(&self, a: &FractionPairMap, b: &FractionPairMap) -> FractionPairMap {
        combine_dicts(a, b)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConcatCsvSnapshot {
    pub class: String,
    pub count: usize,
    pub morphisms: Vec<ConcatCsvSpecSnapshot>,
    pub csv_sources: Vec<String>,
    pub fraction_helpers: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct FractionPair {
    pub left: Rational,
    pub right: Rational,
}

impl FractionPair {
    pub fn new(left: Rational, right: Rational) -> Self {
        Self { left, right }
    }
}

pub type FractionPairMap = BTreeMap<i64, BTreeSet<FractionPair>>;

pub fn bootstrap_concat_csv() -> ConcatCsvBundle {
    ConcatCsvBundle {
        specs: vec![
            ConcatCsvSpec::new(
                "readConcatCsv",
                "Glues an external CSV presheaf section into the current global table section.",
                &["csv", "presheaf", "gluing"],
            ),
            ConcatCsvSpec::new(
                "readConcatCsv_tabelleDazuColchange",
                "Transforms fraction-indexed CSV columns into meta/concrete cell content.",
                &["fraction", "meta", "morphism"],
            ),
            ConcatCsvSpec::new(
                "readConcatCsv_SetHtmlParamaters",
                "Registers generated CSV columns in the HTML/tag parameter sheaf.",
                &["html", "tags", "generated-column"],
            ),
            ConcatCsvSpec::new(
                "convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction",
                "Builds number-indexed fraction-pair sections used by generated prime-universe columns.",
                &["fraction", "brueche", "relation"],
            ),
        ],
        csv_sources: vec!["prim", "bruch13", "bruch15", "bruch7", "bruchStrukGroesse"].into_iter().map(str::to_string).collect(),
        fraction_helpers: vec![
            "convertSetOfPaarenToDictOfNumToPaareDiv",
            "convertSetOfPaarenToDictOfNumToPaareMul",
            "convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction",
            "combineDicts",
        ].into_iter().map(str::to_string).collect(),
    }
}

pub fn rational_mul(a: Rational, b: Rational) -> Rational {
    Rational::new(a.numerator * b.numerator, a.denominator * b.denominator)
}

pub fn rational_div(a: Rational, b: Rational) -> Rational {
    Rational::new(a.numerator * b.denominator, a.denominator * b.numerator)
}

pub fn convert_set_of_pairs_to_dict_of_num_to_pairs_div(
    pairs: &BTreeSet<FractionPair>,
    gleichf: bool,
) -> FractionPairMap {
    let mut result: FractionPairMap = BTreeMap::new();
    for pair in pairs {
        let div = if gleichf {
            rational_div(pair.right, pair.left)
        } else {
            rational_div(pair.left, pair.right)
        };
        if let Some(key) = div.checked_integer() {
            result.entry(key).or_default().insert(*pair);
        }
    }
    result
}

pub fn convert_set_of_pairs_to_dict_of_num_to_pairs_mul(
    pairs: &BTreeSet<FractionPair>,
    gleichf: bool,
) -> FractionPairMap {
    let mut result: FractionPairMap = BTreeMap::new();
    for pair in pairs {
        let mul = rational_mul(pair.left, pair.right);
        let key_value = if gleichf { mul.reciprocal() } else { mul };
        if let Some(key) = key_value.checked_integer() {
            result.entry(key).or_default().insert(*pair);
        }
    }
    result
}

pub fn convert_fractions_to_dict_of_num_to_pairs_of_mul_of_int_and_fraction(
    fracs: &BTreeSet<Rational>,
    fracs2: &BTreeSet<Rational>,
    max_row: i64,
    gleichf: bool,
) -> FractionPairMap {
    let mut result: FractionPairMap = BTreeMap::new();
    let max_row = max_row.max(1);
    if !gleichf {
        for frac in fracs {
            for zusatz_mul in 1..=max_row {
                let pair = FractionPair::new(*frac, Rational::new(frac.denominator * zusatz_mul, 1));
                let mul = rational_mul(pair.left, pair.right);
                if let Some(key) = mul.checked_integer() {
                    if key > max_row { break; }
                    result.entry(key).or_default().insert(pair);
                }
            }
            for zusatz_mul in (1..=max_row).rev() {
                let faktor = Rational::new(frac.denominator, zusatz_mul);
                if fracs2.contains(&faktor) || faktor.numerator == 1 {
                    let pair = FractionPair::new(*frac, faktor);
                    let mul = rational_mul(pair.left, pair.right);
                    if let Some(key) = mul.checked_integer() {
                        if key > max_row { break; }
                        result.entry(key).or_default().insert(pair);
                    }
                }
            }
        }
    } else {
        for frac in fracs {
            for zusatz_div in 1..=max_row {
                let pair = FractionPair::new(*frac, Rational::new(1, frac.numerator * zusatz_div));
                let div = rational_div(Rational::new(1, 1), rational_mul(pair.right, pair.left));
                if let Some(key) = div.checked_integer() {
                    if key > max_row { break; }
                    result.entry(key).or_default().insert(pair);
                }
            }
            for zusatz_div in 1..=max_row {
                let faktor = Rational::new(frac.denominator, frac.numerator * zusatz_div);
                if fracs2.contains(&faktor) || faktor.numerator == 1 {
                    let pair = FractionPair::new(*frac, faktor);
                    let div = rational_div(Rational::new(1, 1), rational_mul(pair.right, pair.left));
                    if let Some(key) = div.checked_integer() {
                        if key > max_row { break; }
                        result.entry(key).or_default().insert(pair);
                    }
                }
            }
        }
    }
    result
}

pub fn combine_dicts(a: &FractionPairMap, b: &FractionPairMap) -> FractionPairMap {
    let mut out = a.clone();
    for (key, values) in b {
        out.entry(*key).or_default().extend(values.iter().copied());
    }
    out
}

pub fn read_concat_csv_tabelle_dazu_colchange(
    row_number: i64,
    table_column: &[String],
    transposed: bool,
) -> Vec<String> {
    table_column
        .iter()
        .enumerate()
        .map(|(index, cell)| {
            let column_number = index as i64 + 1;
            let ratio = if transposed {
                Rational::new(column_number, row_number.max(1))
            } else {
                Rational::new(row_number, column_number)
            };
            if cell.trim().is_empty() {
                String::new()
            } else if ratio.denominator == 1 {
                format!("{cell} ({})", ratio.numerator)
            } else {
                format!("{cell} ({}/{})", ratio.numerator, ratio.denominator)
            }
        })
        .collect()
}

pub fn normalize_fraction(numerator: i64, denominator: i64) -> Rational {
    let gcd = gcd_i64(numerator.abs(), denominator.abs()).max(1);
    Rational::new(numerator / gcd, denominator / gcd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combines_pair_maps() {
        let pair = FractionPair::new(Rational::new(1, 2), Rational::new(2, 1));
        let mut a = FractionPairMap::new();
        a.entry(1).or_default().insert(pair);
        let b = FractionPairMap::new();
        assert_eq!(combine_dicts(&a, &b).get(&1).unwrap().len(), 1);
    }

    #[test]
    fn division_map_uses_integer_keys() {
        let pair = FractionPair::new(Rational::new(4, 1), Rational::new(2, 1));
        let pairs = BTreeSet::from([pair]);
        let map = convert_set_of_pairs_to_dict_of_num_to_pairs_div(&pairs, false);
        assert!(map.contains_key(&2));
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "_ensure_runtime_dependencies",
    "choose_csv_file",
    "readConcatCSV_choseCsvFile",
    "readConcatCsv_ChangeTableToAddToTable",
    "readConcatCsv_LoopBody",
    "read_concat_csv",
    "transpose",
    "convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction",
    "convertSetOfPaarenToDictOfNumToPaareDiv",
    "convertSetOfPaarenToDictOfNumToPaareMul",
    "readConcatCsv_SetHtmlParamaters",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
