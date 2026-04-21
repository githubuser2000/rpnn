#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::collections::BTreeSet;

use indexmap::{IndexMap, IndexSet};

pub use crate::libs::tableHandling::TablesConcat as Concat;


pub type Pair = (i64, i64);
pub type PairsByNumber = IndexMap<i64, IndexSet<Pair>>;

/// Minimal, normalized stand-in for Python `fractions.Fraction` used by
/// `libs/lib4tables_concat.py`.
///
/// The direct lib4tables-concat facade had only the integer pair helpers, while
/// the Python architecture also routes a large part of the meta/concrete
/// fraction machinery through `convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction`.
/// Keeping the type public lets tests and future facade methods pass the same
/// already-normalized values around without reintroducing floats.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PyFraction {
    pub numerator: i64,
    pub denominator: i64,
}

impl PyFraction {
    pub fn new(numerator: i64, denominator: i64) -> Option<Self> {
        if denominator == 0 {
            return None;
        }
        let mut n = numerator;
        let mut d = denominator;
        if d < 0 {
            n = -n;
            d = -d;
        }
        let gcd = gcd_i64(n, d);
        Some(Self {
            numerator: n / gcd,
            denominator: d / gcd,
        })
    }

    pub fn from_int(value: i64) -> Self {
        Self { numerator: value, denominator: 1 }
    }

    pub fn mul(self, other: Self) -> Option<Self> {
        Self::new(
            self.numerator.checked_mul(other.numerator)?,
            self.denominator.checked_mul(other.denominator)?,
        )
    }

    pub fn div(self, other: Self) -> Option<Self> {
        Self::new(
            self.numerator.checked_mul(other.denominator)?,
            self.denominator.checked_mul(other.numerator)?,
        )
    }

    pub fn recip(self) -> Option<Self> {
        Self::new(self.denominator, self.numerator)
    }

    pub fn is_integer(self) -> bool {
        self.denominator == 1
    }

    pub fn as_f64(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

pub type FractionPair = (PyFraction, PyFraction);
pub type FractionPairsByNumber = IndexMap<i64, IndexSet<FractionPair>>;
pub type RawFractionCombinationMap = IndexMap<String, IndexMap<String, IndexMap<String, IndexSet<FractionPair>>>>;

fn py_round_float(value: f64) -> f64 {
    let floor = value.floor();
    let frac = value - floor;
    if frac < 0.5 {
        floor
    } else if frac > 0.5 {
        floor + 1.0
    } else if (floor as i64).rem_euclid(2) == 0 {
        floor
    } else {
        floor + 1.0
    }
}

fn round_to_thousand_py(value: f64) -> f64 {
    py_round_float(value * 1000.0) / 1000.0
}

fn insert_fraction_pair(result: &mut FractionPairsByNumber, key: i64, pair: FractionPair) {
    result.entry(key).or_default().insert(pair);
}

fn fraction_set(values: &[PyFraction]) -> IndexSet<PyFraction> {
    values.iter().copied().collect()
}

fn int_key_from_python_float(value: f64) -> i64 {
    // Python's `int(float)` truncates toward zero.  All active values in this
    // path are non-negative, but truncation keeps the same contract for tests.
    value.trunc() as i64
}

/// Python `Concat.convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction`.
///
/// This intentionally keeps the two-phase insertion order of the original:
/// first the direct integer multiplier/divider scan, then the secondary scan
/// using `fracs2` and `faktor.numerator == 1`.  The result therefore behaves
/// like Python's `DefaultOrderedDict(OrderedSet)` instead of the previously
/// available sorted helpers.
pub fn convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction(
    fracs: &[PyFraction],
    fracs2: &[PyFraction],
    gleichf: bool,
    hoechsteZeile1024: i64,
) -> FractionPairsByNumber {
    let limit = hoechsteZeile1024.max(0);
    let fracs2_set = fraction_set(fracs2);
    let mut result: FractionPairsByNumber = IndexMap::new();

    if !gleichf {
        for frac in fracs.iter().copied() {
            for zusatz_mul in 1..=limit {
                let Some(faktor) = PyFraction::new(frac.denominator.saturating_mul(zusatz_mul), 1) else {
                    continue;
                };
                let pair = (frac, faktor);
                let Some(product) = pair.0.mul(pair.1) else { continue; };
                let product_float = product.as_f64();
                let mulr = py_round_float(product_float);
                let mul = round_to_thousand_py(product_float);
                assert_eq!(mulr, mul);
                if mul > limit as f64 {
                    break;
                }
                insert_fraction_pair(&mut result, int_key_from_python_float(mul), pair);
            }
        }

        for frac in fracs.iter().copied() {
            for zusatz_mul in (1..=limit).rev() {
                let Some(faktor) = PyFraction::new(frac.denominator, zusatz_mul) else { continue; };
                if fracs2_set.contains(&faktor) || faktor.numerator == 1 {
                    let pair = (frac, faktor);
                    let Some(product) = pair.0.mul(pair.1) else { continue; };
                    let product_float = product.as_f64();
                    let mulr = py_round_float(product_float);
                    if product_float > limit as f64 {
                        break;
                    }
                    if (mulr - product_float).abs() == 0.0 {
                        insert_fraction_pair(&mut result, mulr as i64, pair);
                    }
                }
            }
        }
    } else {
        for frac in fracs.iter().copied() {
            for zusatz_div in 1..=limit {
                let Some(faktor) = PyFraction::new(1, frac.numerator.saturating_mul(zusatz_div)) else {
                    continue;
                };
                let pair = (frac, faktor);
                let Some(product) = pair.0.mul(pair.1) else { continue; };
                let Some(div) = product.recip() else { continue; };
                let div_float = div.as_f64();
                let divr = py_round_float(div_float);
                let div = round_to_thousand_py(div_float);
                assert_eq!(divr, div);
                if div > limit as f64 {
                    break;
                }
                insert_fraction_pair(&mut result, divr as i64, pair);
            }
        }

        for frac in fracs.iter().copied() {
            for zusatz_div in 1..=limit {
                let Some(recip) = frac.recip() else { continue; };
                let Some(divisor) = PyFraction::new(zusatz_div, 1) else { continue; };
                let Some(faktor) = recip.div(divisor) else { continue; };
                if fracs2_set.contains(&faktor) || faktor.numerator == 1 {
                    let pair = (frac, faktor);
                    let Some(product) = pair.0.mul(pair.1) else { continue; };
                    let Some(inv_product) = product.recip() else { continue; };
                    let inv_float = inv_product.as_f64();
                    let mulr = py_round_float(inv_float);
                    let mul = round_to_thousand_py(inv_float);
                    assert_eq!(mulr, mul);
                    if mul != 0.0 && 1.0 / mul > limit as f64 {
                        break;
                    }
                    insert_fraction_pair(&mut result, mulr as i64, pair);
                }
            }
        }
    }

    result
}

fn empty_fraction_poly_map() -> IndexMap<String, IndexMap<String, IndexSet<FractionPair>>> {
    let mut poly = IndexMap::new();
    for poly_key in ["stern", "gleichf"] {
        let mut md = IndexMap::new();
        md.insert("mul".to_string(), IndexSet::new());
        md.insert("div".to_string(), IndexSet::new());
        poly.insert(poly_key.to_string(), md);
    }
    poly
}

fn insert_combo(
    target: &mut IndexMap<String, IndexMap<String, IndexMap<String, IndexSet<FractionPair>>>>,
    outer: &str,
    poly: &str,
    op: &str,
    pair: FractionPair,
) {
    target
        .get_mut(outer)
        .and_then(|inner| inner.get_mut(poly))
        .and_then(|ops| ops.get_mut(op))
        .expect("fraction combination bucket must exist")
        .insert(pair);
}

fn python_is_rounded_integer(value: f64) -> bool {
    py_round_float(value) == round_to_thousand_py(value)
}

fn python_div_condition_bug_compatible(value: f64) -> bool {
    // `lib4tables_concat.py` intentionally/accidentally compares
    //     round(x) == round(x * 1000)
    // without dividing by 1000.  Keep that observable condition here rather
    // than replacing it with the mathematically likely variant.
    py_round_float(value) == py_round_float(value * 1000.0)
}

/// Python `Concat.findAllBruecheAndTheirCombinations` core, parameterized with
/// already loaded Galaxie/Universum fraction sets.
///
/// The return shape mirrors Python's nested `OrderedDict`:
/// `UniUni|UniGal|GalUni|GalGal -> stern|gleichf -> mul|div -> OrderedSet[pair]`.
pub fn findAllBruecheAndTheirCombinations_from_fraction_sets(
    brueche_gal: &[PyFraction],
    brueche_uni: &[PyFraction],
) -> RawFractionCombinationMap {
    let mut all = IndexMap::new();
    for key in ["UniUni", "UniGal", "GalUni", "GalGal"] {
        all.insert(key.to_string(), empty_fraction_poly_map());
    }

    let mut gal = brueche_gal.to_vec();
    let mut uni = brueche_uni.to_vec();
    gal.sort();
    uni.sort();

    let combos: [(&[PyFraction], &[PyFraction], &str, &str); 4] = [
        (gal.as_slice(), gal.as_slice(), "Gal", "Gal"),
        (gal.as_slice(), uni.as_slice(), "Gal", "Uni"),
        (uni.as_slice(), gal.as_slice(), "Uni", "Gal"),
        (uni.as_slice(), uni.as_slice(), "Uni", "Uni"),
    ];

    for (lefts, rights, left_name, right_name) in combos {
        let outer = format!("{}{}", left_name, right_name);
        for left in lefts.iter().copied() {
            for right in rights.iter().copied() {
                if left == right {
                    continue;
                }
                let pair = (left, right);
                if let Some(product) = left.mul(right) {
                    let product_float = product.as_f64();
                    if python_is_rounded_integer(product_float) {
                        insert_combo(&mut all, &outer, "stern", "mul", pair);
                    }
                    if product_float != 0.0 && python_is_rounded_integer(1.0 / product_float) {
                        insert_combo(&mut all, &outer, "gleichf", "mul", pair);
                    }
                }
                if let Some(div) = left.div(right) {
                    let div_float = div.as_f64();
                    if python_div_condition_bug_compatible(div_float) {
                        insert_combo(&mut all, &outer, "stern", "div", pair);
                    }
                    if div_float != 0.0 && python_is_rounded_integer(1.0 / div_float) {
                        insert_combo(&mut all, &outer, "gleichf", "div", pair);
                    }
                }
            }
        }
    }

    all
}

fn round_to_thousand(value: f64) -> f64 {
    round_to_thousand_py(value)
}

pub fn convertSetOfPaarenToDictOfNumToPaareDiv(paareSet: BTreeSet<Pair>, gleichf: bool) -> PairsByNumber {
    let mut result: PairsByNumber = IndexMap::new();
    for paar in paareSet {
        let div = if !gleichf {
            paar.0 as f64 / paar.1 as f64
        } else {
            paar.1 as f64 / paar.0 as f64
        };
        let rounded = round_to_thousand(div);
        assert_eq!(rounded, rounded.round());
        result.entry(rounded as i64).or_default().insert(paar);
    }
    result
}

pub fn convertSetOfPaarenToDictOfNumToPaareMul(paareSet: BTreeSet<Pair>, gleichf: bool) -> PairsByNumber {
    let mut result: PairsByNumber = IndexMap::new();
    for paar in paareSet {
        let mut mul = paar.0 as f64 * paar.1 as f64;
        if gleichf {
            mul = 1.0 / mul;
        }
        let mulr = mul.round();
        let rounded = round_to_thousand(mul);
        assert_eq!(rounded, mulr);
        result.entry(mulr as i64).or_default().insert(paar);
    }
    result
}

pub fn combineDicts(a: PairsByNumber, b: PairsByNumber) -> PairsByNumber {
    let mut e: PairsByNumber = IndexMap::new();
    for (key, value) in a.into_iter().chain(b.into_iter()) {
        e.entry(key).or_default().extend(value.into_iter().map(|v| (v.0, v.1)));
    }
    e
}

pub fn spalteMetaKonkretAbstrakt_isGanzZahlig(mut zahl: f64, spaltenWahl: bool) -> bool {
    if spaltenWahl {
        zahl = 1.0 / zahl;
    }
    let rest = zahl.rem_euclid(1.0);
    rest < 0.00001 || rest > 0.99999
}

pub fn getAllBrueche(gebrUnivTable4metaKonkret: &[Vec<String>]) -> BTreeSet<(i64, i64)> {
    let mut menge = BTreeSet::new();
    for (i, row) in gebrUnivTable4metaKonkret.iter().skip(1).enumerate() {
        for (k, cell) in row.iter().skip(1).enumerate() {
            if cell.trim().chars().count() > 3 {
                let numerator = i as i64 + 2;
                let denominator = k as i64 + 2;
                let gcd = gcd_i64(numerator, denominator);
                let reduced_n = numerator / gcd;
                let reduced_d = denominator / gcd;
                if reduced_d != 1 && reduced_n != 1 {
                    menge.insert((reduced_n, reduced_d));
                }
            }
        }
    }
    menge
}

fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs().max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_division_groups_by_exact_integer_ratio() {
        let input = BTreeSet::from([(6, 3), (9, 3)]);
        let grouped = convertSetOfPaarenToDictOfNumToPaareDiv(input, false);
        assert_eq!(
            grouped.get(&2).unwrap().iter().copied().collect::<Vec<_>>(),
            vec![(6, 3)]
        );
        assert_eq!(
            grouped.get(&3).unwrap().iter().copied().collect::<Vec<_>>(),
            vec![(9, 3)]
        );
    }

    #[test]
    fn combine_dicts_keeps_python_ordered_dict_and_ordered_set_semantics() {
        let mut a: PairsByNumber = IndexMap::new();
        a.entry(3).or_default().insert((9, 3));
        a.entry(2).or_default().insert((6, 3));

        let mut b: PairsByNumber = IndexMap::new();
        b.entry(4).or_default().insert((8, 2));
        b.entry(2).or_default().insert((10, 5));

        let combined = combineDicts(a, b);
        assert_eq!(combined.keys().copied().collect::<Vec<_>>(), vec![3, 2, 4]);
        assert_eq!(
            combined.get(&2).unwrap().iter().copied().collect::<Vec<_>>(),
            vec![(6, 3), (10, 5)]
        );
    }

    #[test]
    fn brueche_skip_integer_and_unit_numerator_like_python() {
        let table = vec![
            vec!["h".into(), "h".into(), "h".into()],
            vec!["r".into(), "xxxx".into(), "xxxx".into()],
            vec!["r".into(), "xxxx".into(), "xxxx".into()],
        ];
        let fracs = getAllBrueche(&table);
        assert!(fracs.contains(&(2, 3)));
        assert!(!fracs.contains(&(1, 1)));
    }

    fn frac(n: i64, d: i64) -> PyFraction {
        PyFraction::new(n, d).unwrap()
    }

    fn rendered_pairs(map: &FractionPairsByNumber, key: i64) -> Vec<String> {
        map.get(&key)
            .map(|pairs| {
                pairs
                    .iter()
                    .map(|(left, right)| {
                        format!(
                            "{}/{}*{}/{}",
                            left.numerator, left.denominator, right.numerator, right.denominator
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    #[test]
    fn fraction_multiplier_dict_keeps_python_order_and_secondary_pass() {
        let fracs = vec![frac(2, 3), frac(3, 4)];
        let fracs2 = vec![frac(3, 2), frac(4, 3), frac(1, 2)];
        let out = convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction(
            &fracs,
            &fracs2,
            false,
            8,
        );

        assert_eq!(out.keys().copied().collect::<Vec<_>>(), vec![2, 4, 6, 8, 3, 1]);
        assert_eq!(rendered_pairs(&out, 2), vec!["2/3*3/1"]);
        assert_eq!(rendered_pairs(&out, 6), vec!["2/3*9/1", "3/4*8/1"]);
        assert_eq!(rendered_pairs(&out, 1), vec!["2/3*3/2", "3/4*4/3"]);
    }

    #[test]
    fn fraction_equalform_dict_keeps_python_reciprocal_shape() {
        let fracs = vec![frac(2, 3), frac(3, 4)];
        let fracs2 = vec![frac(3, 2), frac(4, 3), frac(1, 2)];
        let out = convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction(
            &fracs,
            &fracs2,
            true,
            8,
        );

        assert_eq!(out.keys().copied().collect::<Vec<_>>(), vec![3, 6, 4, 8, 1]);
        assert_eq!(rendered_pairs(&out, 3), vec!["2/3*1/2"]);
        assert_eq!(rendered_pairs(&out, 4), vec!["3/4*1/3"]);
        assert_eq!(rendered_pairs(&out, 1), vec!["2/3*3/2", "3/4*4/3"]);
    }

    #[test]
    fn raw_fraction_combinations_keep_python_outer_order_and_div_quirk() {
        let gal = vec![frac(2, 3), frac(3, 2)];
        let uni = vec![frac(3, 4), frac(4, 3)];
        let out = findAllBruecheAndTheirCombinations_from_fraction_sets(&gal, &uni);

        assert_eq!(
            out.keys().map(|key| key.as_str()).collect::<Vec<_>>(),
            vec!["UniUni", "UniGal", "GalUni", "GalGal"]
        );
        let galgal = out.get("GalGal").unwrap();
        let stern = galgal.get("stern").unwrap();
        assert!(stern.get("mul").unwrap().contains(&(frac(2, 3), frac(3, 2))));
        assert!(stern.get("div").unwrap().is_empty());
    }
}
