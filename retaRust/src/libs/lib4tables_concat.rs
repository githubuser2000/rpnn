#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::collections::{BTreeMap, BTreeSet};

use indexmap::{IndexMap, IndexSet};

use crate::shared::lib4tables_enum_py::ST;

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


fn cell_at(relitable: &[Vec<String>], row: usize, col: usize) -> String {
    relitable
        .get(row)
        .and_then(|line| line.get(col))
        .cloned()
        .unwrap_or_default()
}

fn cell_len_gt(relitable: &[Vec<String>], row: usize, col: usize, min_chars: usize) -> bool {
    cell_at(relitable, row, col).trim().chars().count() > min_chars
}

fn list_open(html_output_yes: bool, bbcode_output_yes: bool) -> &'static str {
    if html_output_yes {
        "<ul>"
    } else if bbcode_output_yes {
        "[list]"
    } else {
        ""
    }
}

fn list_close(html_output_yes: bool, bbcode_output_yes: bool) -> &'static str {
    if html_output_yes {
        "</ul>"
    } else if bbcode_output_yes {
        "[/list]"
    } else {
        ""
    }
}

fn list_item_prefix(html_output_yes: bool, bbcode_output_yes: bool) -> &'static str {
    if html_output_yes {
        "<li>"
    } else if bbcode_output_yes {
        "[*]"
    } else {
        ""
    }
}

fn list_item_suffix(html_output_yes: bool, bbcode_output_yes: bool) -> &'static str {
    if html_output_yes {
        "</li>"
    } else if bbcode_output_yes {
        ""
    } else {
        " | "
    }
}

fn modal_text_by_distance_concat(distance_from_line: i64) -> &'static str {
    match distance_from_line.abs() {
        2 => "mittelstark überdurchschnittlich: ",
        1 => "überdurchschnittlich: ",
        3 => "mittelleicht überdurchschnittlich: ",
        0 => "sehr: ",
        _ => "sehr leicht überdurchschnittlich: ",
    }
}

fn modal_replace_zuerst_zweites(text: &str) -> String {
    text.replace("intrinsisch", "zuerst")
        .replace("extrinsisch", "als zweites")
}

/// Storage shape of Python `vorkommenVielfacher_B[i][distance]`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModalEntry {
    pub i_origS: Vec<usize>,
    pub modalS: Vec<Vec<String>>,
    pub vervielfachter: Vec<usize>,
}

/// Python `getModaloperatorsPerLineCells` from `Concat.concatModallogik`.
#[allow(non_snake_case)]
pub fn getModaloperatorsPerLineCells(relitable: &[Vec<String>], lineWeAreAt: usize) -> Vec<String> {
    let modal_main_operator_line = lineWeAreAt;
    let amount_modaloperators = lineWeAreAt.saturating_sub(1);
    let modal_else_begin = lineWeAreAt.saturating_add(1);
    let modal_else_end = lineWeAreAt
        .saturating_add(amount_modaloperators)
        .saturating_add(1);

    let mut modaloperators = Vec::new();
    if let Some(row) = relitable.get(modal_main_operator_line) {
        if let Some(value) = row.get(97) {
            modaloperators.push(value.clone());
        }
        if let Some(value) = row.get(98) {
            modaloperators.push(value.clone());
        }
    }
    for coord in modal_else_begin..modal_else_end {
        if let Some(value) = relitable.get(coord).and_then(|row| row.get(42)) {
            modaloperators.push(value.clone());
        }
    }
    modaloperators
}

/// Python `prepareModalIntoTable`: collect modal operators for all multiples at a
/// relative distance and prepend them to the already known lists.
#[allow(non_snake_case)]
pub fn prepareModalIntoTable(
    distanceFromLine: i64,
    i: usize,
    relitable: &[Vec<String>],
    vorkommenVielfacher: &BTreeMap<usize, Vec<(usize, usize)>>,
    vorkommenVielfacher_B: &mut BTreeMap<usize, BTreeMap<i64, ModalEntry>>,
) {
    let Some(i_with_distance) = (i as i64).checked_add(distanceFromLine) else {
        return;
    };
    if i_with_distance < 0 {
        return;
    }
    let Some(couples) = vorkommenVielfacher.get(&(i_with_distance as usize)) else {
        return;
    };

    let mut modal_en = Vec::new();
    let mut original_i = Vec::new();
    let mut vervielfachter = Vec::new();
    for (vorkommen, vielfacher) in couples.iter().copied() {
        modal_en.push(getModaloperatorsPerLineCells(relitable, vielfacher));
        vervielfachter.push(vorkommen);
        original_i.push(i_with_distance as usize);
    }

    let by_distance = vorkommenVielfacher_B.entry(i).or_default();
    let existing = by_distance.remove(&distanceFromLine).unwrap_or_default();
    modal_en.extend(existing.modalS);
    original_i.extend(existing.i_origS);
    vervielfachter.extend(existing.vervielfachter);
    by_distance.insert(
        distanceFromLine,
        ModalEntry {
            i_origS: original_i,
            modalS: modal_en,
            vervielfachter,
        },
    );
}

/// Python `ModalLogikIntoTable`: append the modal text fragments generated for
/// one concept column pair into the target row buffers.
#[allow(non_snake_case)]
pub fn ModalLogikIntoTable(
    concept: (usize, usize),
    distanceFromLine: i64,
    i: usize,
    relitable: &[Vec<String>],
    into: &mut BTreeMap<usize, Vec<String>>,
    vorkommenVielfacher_B: &BTreeMap<usize, BTreeMap<i64, ModalEntry>>,
    htmlOutputYes: bool,
    bbcodeOutputYes: bool,
) {
    let Some(entry) = vorkommenVielfacher_B
        .get(&i)
        .and_then(|by_distance| by_distance.get(&distanceFromLine))
    else {
        return;
    };

    for (modalOperatoren, vervielfachter) in entry.modalS.iter().zip(entry.vervielfachter.iter().copied()) {
        if modalOperatoren.len() < 2 {
            continue;
        }
        let content_col = if distanceFromLine.abs() % 2 == 0 { concept.0 } else { concept.1 };
        let into_its_content = cell_at(relitable, vervielfachter, content_col);
        if into_its_content.is_empty() {
            continue;
        }
        let first_modal = modalOperatoren.get(0).cloned().unwrap_or_default();
        let base_modal = cell_at(relitable, 1, 97);
        let displayed_content = if first_modal == base_modal {
            into_its_content.clone()
        } else {
            modal_replace_zuerst_zweites(&into_its_content)
        };

        let mut fragment = String::new();
        fragment.push_str(list_item_prefix(htmlOutputYes, bbcodeOutputYes));
        fragment.push_str(modal_text_by_distance_concat(distanceFromLine));
        fragment.push_str(&first_modal);
        fragment.push(' ');
        fragment.push_str(&displayed_content);
        fragment.push(' ');
        fragment.push_str(&modalOperatoren[1]);

        if distanceFromLine.abs() % 2 == 1 && modalOperatoren.len() > 2 {
            fragment.push_str(", nicht: ");
            fragment.push_str(&modalOperatoren[2..].join(", "));
            fragment.push_str(" (das alles nicht): ");
            fragment.push_str(&modal_replace_zuerst_zweites(&cell_at(relitable, vervielfachter, concept.0)));
        }
        if !htmlOutputYes && !bbcodeOutputYes {
            fragment.push_str(" | ");
        }
        if htmlOutputYes {
            fragment.push_str("</li>");
        }
        into.entry(i).or_default().push(fragment);
    }
}

/// Coordinate variant used by the Python meta/concrete table walkers: either a
/// direct row index or a `fractions.Fraction` pointing into the Gebr/Universum
/// helper matrix.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MetaCoordinate {
    Int(i64),
    Fraction(PyFraction),
}

impl MetaCoordinate {
    fn as_fraction(self) -> Option<PyFraction> {
        match self {
            Self::Int(value) => PyFraction::new(value, 1),
            Self::Fraction(value) => Some(value),
        }
    }

    fn as_index(self) -> Option<usize> {
        match self {
            Self::Int(value) if value >= 0 => Some(value as usize),
            _ => None,
        }
    }

    pub fn display(self) -> String {
        match self {
            Self::Int(value) => value.to_string(),
            Self::Fraction(value) if value.denominator == 1 => value.numerator.to_string(),
            Self::Fraction(value) => format!("{}/{}", value.numerator, value.denominator),
        }
    }
}

pub type MetaMoreAndLess = (Option<MetaCoordinate>, Option<MetaCoordinate>);

fn spalteMetaKonkretAbstrakt_isGanzZahlig_fraction(value: PyFraction, spaltenWahl: bool) -> bool {
    spalteMetaKonkretAbstrakt_isGanzZahlig(value.as_f64(), spaltenWahl)
}

/// Python `switching` nested in `spalteMetaKonkretTheorieAbstrakt_etc_1`.
#[allow(non_snake_case)]
pub fn switching(
    newCol: usize,
    moreAndLess: MetaMoreAndLess,
    metavariable: i64,
    ifInvers: usize,
    transzendentalienSpalten: (usize, usize),
    relitable_len: usize,
    gebrRatEtwaSchonMalDabeiGewesen: &mut BTreeSet<PyFraction>,
) -> (usize, MetaMoreAndLess) {
    let (new_col, _spalten_wahl) = if newCol == transzendentalienSpalten.1 {
        (transzendentalienSpalten.0, 0usize)
    } else {
        (transzendentalienSpalten.1, 1usize)
    };

    let a = moreAndLess.0.and_then(|coord| match coord {
        MetaCoordinate::Int(value) => value.checked_mul(metavariable).and_then(|mul| {
            if mul >= 0 && (mul as usize) < relitable_len {
                Some(MetaCoordinate::Int(mul))
            } else {
                None
            }
        }),
        MetaCoordinate::Fraction(value) => value.mul(PyFraction::from_int(metavariable)).and_then(|mul| {
            if mul.is_integer() && mul.numerator >= 0 && (mul.numerator as usize) < relitable_len {
                Some(MetaCoordinate::Int(mul.numerator))
            } else {
                None
            }
        }),
    });

    let b = moreAndLess.1.and_then(|coord| {
        let mut right = coord.as_fraction()?;
        let right_f = right.as_f64();
        if !(right_f < 100.0 && right_f > 0.01) {
            return None;
        }
        let inverse_col = if ifInvers == 0 {
            transzendentalienSpalten.0
        } else {
            transzendentalienSpalten.1
        };
        if new_col == inverse_col {
            if let MetaCoordinate::Int(value) = coord {
                right = PyFraction::new(1, value)?;
            }
        }
        let candidate = if spalteMetaKonkretAbstrakt_isGanzZahlig_fraction(right, false) {
            PyFraction::new(metavariable, 1)?.div(right)?
        } else {
            right.recip()?.div(PyFraction::new(metavariable, 1)?)?
        };
        if gebrRatEtwaSchonMalDabeiGewesen.contains(&candidate) {
            None
        } else {
            gebrRatEtwaSchonMalDabeiGewesen.insert(candidate);
            Some(MetaCoordinate::Fraction(candidate))
        }
    });

    (new_col, (a, b))
}

/// Python `spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie`.
#[allow(non_snake_case)]
pub fn spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(
    koord: PyFraction,
    n_and_invers_spalten: (usize, usize),
    relitable: &[Vec<String>],
    gebrTable4metaKonkretAndMore: &[Vec<String>],
    isNotUniverse: bool,
    htmlOutputYes: bool,
) -> Option<String> {
    let is_universe = !isNotUniverse;
    if koord.denominator == 0 || koord.numerator == 0 {
        return Some(String::new());
    }
    if koord.denominator > 100 || koord.numerator > 100 || koord.denominator < 0 || koord.numerator < 0 {
        return None;
    }
    if koord.numerator == 1 {
        let idx = koord.denominator as usize;
        if cell_len_gt(relitable, idx, n_and_invers_spalten.1, 3) {
            let base = cell_at(relitable, idx, n_and_invers_spalten.1);
            if is_universe {
                let extra = cell_at(relitable, idx, 201);
                let sep = if extra.chars().count() > 2 {
                    if htmlOutputYes { "<br>" } else { "; " }
                } else {
                    ""
                };
                Some(format!("{} (1/{}){}{}", base, koord.denominator, sep, extra))
            } else {
                Some(base)
            }
        } else {
            Some(String::new())
        }
    } else if koord.denominator == 1 {
        let idx = koord.numerator as usize;
        if cell_len_gt(relitable, idx, n_and_invers_spalten.0, 3) {
            let base = cell_at(relitable, idx, n_and_invers_spalten.0);
            if is_universe {
                let extra = cell_at(relitable, idx, 198);
                let sep = if extra.chars().count() > 2 {
                    if htmlOutputYes { "<br>" } else { "; " }
                } else {
                    ""
                };
                Some(format!("{} ({}){}{}", base, koord.numerator, sep, extra))
            } else {
                Some(base)
            }
        } else {
            Some(String::new())
        }
    } else {
        let row = koord.numerator.checked_sub(1)? as usize;
        let col = koord.denominator.checked_sub(1)? as usize;
        Some(
            gebrTable4metaKonkretAndMore
                .get(row)
                .and_then(|line| line.get(col))
                .cloned()
                .unwrap_or_default(),
        )
    }
}

/// Python `spalteMetaKonkretAbstrakt_UeberschriftenUndTags` as a pure header
/// and tag computation.
#[allow(non_snake_case)]
pub fn spalteMetaKonkretAbstrakt_UeberschriftenUndTags(
    bothRows: i64,
    ifInvers: usize,
    metavariable: i64,
    rowsAsNumbers: &BTreeSet<i64>,
    current_header_len: i64,
) -> (BTreeSet<i64>, String, BTreeSet<ST>) {
    let mut rows = rowsAsNumbers.clone();
    rows.insert(current_header_len);
    let star_tag = if ifInvers == 0 { ST::sternPolygon } else { ST::gleichfoermigesPolygon };
    let mut tags = BTreeSet::from([star_tag, ST::universum]);
    if bothRows == 1 {
        tags.insert(ST::gebrRat);
    }
    let mut heading = match (bothRows, metavariable) {
        (0, 2) => "Meta",
        (0, 3) => "Theorie",
        (0, 4) => "Management",
        (0, 5) => "ganzheitlich",
        (0, 6) => "Verwertung, Unternehmung, Geschäft",
        (0, 7) => "regieren, beherrschen",
        (1, 2) => "Konkretes",
        (1, 3) => "Praxis",
        (1, 4) => "verändernd",
        (1, 5) => "darüber hinaus gehend",
        (1, 6) => "wertvoll",
        (1, 7) => "Richtung",
        _ => "",
    }
    .to_string();
    heading.push_str(if ifInvers == 1 { " für 1/n statt n" } else { " für n" });
    (rows, heading, tags)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaVorwortEntry {
    pub moreAndLess: MetaMoreAndLess,
    pub column: usize,
    pub vorwort1: String,
    pub vorwort2: String,
}

/// Python `spalteMetaKonkretTheorieAbstrakt_VorwortBehandlungWieVorwortMeta`.
#[allow(non_snake_case)]
pub fn spalteMetaKonkretTheorieAbstrakt_VorwortBehandlungWieVorwortMeta(
    metavariable: i64,
    ifInvers: usize,
    transzendentalienSpalten: (usize, usize),
    relitable_len: usize,
    make_vorwort_prefixes: (&str, &str),
    mut newCol: usize,
    mut moreAndLess: MetaMoreAndLess,
    seen: &mut BTreeSet<PyFraction>,
) -> Vec<MetaVorwortEntry> {
    let mut entries = Vec::new();
    while !(moreAndLess.0.is_none() && moreAndLess.1.is_none()) {
        let switched = switching(
            newCol,
            moreAndLess,
            metavariable,
            ifInvers,
            transzendentalienSpalten,
            relitable_len,
            seen,
        );
        newCol = switched.0;
        moreAndLess = switched.1;
        if moreAndLess.0.is_none() && moreAndLess.1.is_none() {
            break;
        }
        let repetitions = entries.len() + 1;
        let vorwort1 = make_vorwort_prefixes.0.repeat(repetitions.max(1));
        let vorwort2 = make_vorwort_prefixes.1.repeat(repetitions.max(1));
        entries.push(MetaVorwortEntry {
            moreAndLess,
            column: newCol,
            vorwort1,
            vorwort2,
        });
    }
    entries
}

/// Python `spalteMetaKonkretTheorieAbstrakt_mainPart_InsertingText`, returned as
/// the rendered side-column text instead of mutating `self.relitable[i]`.
#[allow(non_snake_case)]
pub fn spalteMetaKonkretTheorieAbstrakt_mainPart_InsertingText(
    bothRows: i64,
    _i: usize,
    ifInvers: usize,
    neue2KoordNeue2Vorwoerter: &[MetaVorwortEntry],
    relitable: &[Vec<String>],
    transzendentalienSpalten: (usize, usize),
    gebrUnivTable4metaKonkret: &[Vec<String>],
    htmlOutputYes: bool,
    bbcodeOutputYes: bool,
) -> String {
    let mut into_list = String::new();
    let mut thema = String::new();
    for entry in neue2KoordNeue2Vorwoerter.iter() {
        if bothRows == 0 {
            if let Some(row) = entry.moreAndLess.0.and_then(|coord| coord.as_index()) {
                let cell = cell_at(relitable, row, entry.column);
                if cell.trim().chars().count() > 3 {
                    let inverse_col = if ifInvers == 0 {
                        transzendentalienSpalten.0
                    } else {
                        transzendentalienSpalten.1
                    };
                    let inverse_prefix = if entry.column != inverse_col && row != 1 { "1/" } else { "" };
                    into_list.push_str(list_item_prefix(htmlOutputYes, bbcodeOutputYes));
                    into_list.push_str(&entry.vorwort1);
                    into_list.push_str(&thema);
                    into_list.push_str(&cell);
                    into_list.push_str(" (");
                    into_list.push_str(inverse_prefix);
                    into_list.push_str(&row.to_string());
                    into_list.push(')');
                    into_list.push_str(list_item_suffix(htmlOutputYes, bbcodeOutputYes));
                }
            }
        } else if bothRows == 1 {
            match entry.moreAndLess.1 {
                Some(MetaCoordinate::Int(row_i64)) if row_i64 >= 0 => {
                    let row = row_i64 as usize;
                    let cell = cell_at(relitable, row, entry.column);
                    if cell.trim().chars().count() > 3 {
                        let inverse_col = if ifInvers == 0 {
                            transzendentalienSpalten.0
                        } else {
                            transzendentalienSpalten.1
                        };
                        let inverse_prefix = if entry.column != inverse_col && row != 1 { "1/" } else { "" };
                        into_list.push_str(list_item_prefix(htmlOutputYes, bbcodeOutputYes));
                        into_list.push_str(&entry.vorwort2);
                        into_list.push_str(&thema);
                        into_list.push_str(&cell);
                        into_list.push_str(" (");
                        into_list.push_str(inverse_prefix);
                        into_list.push_str(&row.to_string());
                        into_list.push(')');
                        into_list.push_str(list_item_suffix(htmlOutputYes, bbcodeOutputYes));
                    }
                }
                Some(MetaCoordinate::Fraction(frac)) => {
                    if let Some(gebr) = spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(
                        frac,
                        transzendentalienSpalten,
                        relitable,
                        gebrUnivTable4metaKonkret,
                        false,
                        htmlOutputYes,
                    ) {
                        if gebr.trim().chars().count() > 3 {
                            into_list.push_str(list_item_prefix(htmlOutputYes, bbcodeOutputYes));
                            into_list.push_str(&entry.vorwort2);
                            into_list.push_str(&thema);
                            into_list.push_str(&gebr);
                            into_list.push('(');
                            into_list.push_str(&frac.numerator.to_string());
                            if frac.denominator > 1 {
                                into_list.push('/');
                                into_list.push_str(&frac.denominator.to_string());
                            }
                            into_list.push(')');
                            into_list.push_str(list_item_suffix(htmlOutputYes, bbcodeOutputYes));
                        }
                    }
                }
                _ => {}
            }
        }
        thema = "Thema: ".to_string();
    }

    if into_list.is_empty() {
        String::new()
    } else {
        format!("{}{}{}", list_open(htmlOutputYes, bbcodeOutputYes), into_list, list_close(htmlOutputYes, bbcodeOutputYes))
    }
}

/// Python `spalteMetaKonkretTheorieAbstrakt_mainPart` as a deterministic,
/// side-effect-light facade over the generated meta/concrete side columns.
#[allow(non_snake_case)]
pub fn spalteMetaKonkretTheorieAbstrakt_mainPart(
    bothRows: i64,
    ifInvers: usize,
    metavariable: i64,
    relitable: &[Vec<String>],
    transzendentalienSpalten: (usize, usize),
    gebrUnivTable4metaKonkret: &[Vec<String>],
    rowsAsNumbers: &BTreeSet<i64>,
    htmlOutputYes: bool,
    bbcodeOutputYes: bool,
) -> (Vec<Vec<String>>, BTreeSet<i64>) {
    let (rows, heading, _tags) = spalteMetaKonkretAbstrakt_UeberschriftenUndTags(
        bothRows,
        ifInvers,
        metavariable,
        rowsAsNumbers,
        relitable.first().map(|row| row.len()).unwrap_or(0) as i64,
    );
    let mut out = relitable.to_vec();
    if out.is_empty() {
        return (out, rows);
    }
    if let Some(header) = out.get_mut(0) {
        header.push(heading);
    }
    if let Some(second) = out.get_mut(1) {
        second.push(String::new());
    }

    let mut seen = BTreeSet::new();
    let prefixes = match metavariable {
        2 => ("Meta-Thema: ", "Konkretes: "),
        3 => ("Theorie-Thema: ", "Praxis: "),
        4 => ("Planungs-Thema: ", "Umsetzungs-Thema: "),
        5 => ("Anlass-Thema: ", "Wirkungs-Thema: "),
        6 => ("Kraft-Gebung: ", "Verstärkungs-Thema: "),
        7 => ("Beherrschung: ", "Richtung-Thema: "),
        _ => ("", ""),
    };

    for i in 2..out.len() {
        let start_coord = MetaCoordinate::Int(i as i64);
        let entries = spalteMetaKonkretTheorieAbstrakt_VorwortBehandlungWieVorwortMeta(
            metavariable,
            ifInvers,
            transzendentalienSpalten,
            out.len(),
            prefixes,
            transzendentalienSpalten.0,
            (Some(start_coord), Some(start_coord)),
            &mut seen,
        );
        let rendered = spalteMetaKonkretTheorieAbstrakt_mainPart_InsertingText(
            bothRows,
            i,
            ifInvers,
            &entries,
            &out,
            transzendentalienSpalten,
            gebrUnivTable4metaKonkret,
            htmlOutputYes,
            bbcodeOutputYes,
        );
        if let Some(row) = out.get_mut(i) {
            row.push(rendered);
        }
    }
    (out, rows)
}

/// Python `readConcatCsv_LoopBody`: decide whether one concat CSV side-column is
/// selected and return the column number that Python adds to both result sets.
#[allow(non_snake_case)]
pub fn readConcatCsv_LoopBody(
    concatTableSelection: &BTreeSet<i64>,
    concatTable: i64,
    dazu_len: usize,
    relitable_header_len: usize,
    _generated_spalten_parameter_len: i64,
    _spalten_vanilla_amount: i64,
    u: usize,
) -> Option<i64> {
    let selected = (concatTableSelection.contains(&((u + 2) as i64)) && (2..10).contains(&concatTable))
        || concatTable == 1;
    if !selected {
        return None;
    }
    if (2..10).contains(&concatTable) && u + 1 == dazu_len {
        return None;
    }
    let delta = if (2..10).contains(&concatTable) { 1usize } else { 0usize };
    Some((u + relitable_header_len).saturating_sub(dazu_len).saturating_add(delta) as i64)
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
