#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::collections::BTreeSet;

pub const PYTHON_SOURCE__LIB4TABLES: &str = include_str!("../../python_reference/lib4tables.py");

pub use crate::libs::tableHandling::{
    bbCodeSyntax, csvSyntax, emacsSyntax, htmlSyntax, markdownSyntax, NichtsSyntax, OutputSyntax,
};
pub use crate::libs::tableHandling::primCreativity;
pub use crate::shared::lib4tables_enum_py::{
    dictViceversa, tableTags, tableTags2, tableTags2_for_column,
    tableTags2_kombiTable, tableTags2_kombiTable2, tableTags2_kombiTable2_for_column,
    tableTags2_kombiTable_for_column, tableTags_columns_for_tags, tableTags_kombiTable,
    tableTags_kombiTable2, tableTags_kombiTable2_columns_for_tags,
    tableTags_kombiTable_columns_for_tags, TableTags, TableTags2,
};

fn py_round_positive(value: f64) -> i64 {
    let floor = value.floor();
    let frac = value - floor;
    if frac < 0.5 {
        floor as i64
    } else if frac > 0.5 {
        floor as i64 + 1
    } else {
        let floor_i = floor as i64;
        if floor_i % 2 == 0 { floor_i } else { floor_i + 1 }
    }
}

pub fn moonNumber(num: i64) -> (Vec<i64>, Vec<i64>) {
    let mut results = Vec::new();
    let mut exponent = Vec::new();
    if num <= 2 {
        return (results, exponent);
    }
    for i in 2..num {
        let one_result = (num as f64).powf(1.0 / i as f64);
        if py_round_positive(one_result) * 100000 == py_round_positive(one_result * 100000.0) {
            results.push(py_round_positive(one_result));
            exponent.push(i - 2);
        }
    }
    (results, exponent)
}

pub fn primFak(n: i64) -> Vec<i64> {
    let mut faktoren = Vec::new();
    let mut z = n;
    while z > 1 {
        let mut i = 2i64;
        let mut gefunden = false;
        let mut p = z;
        while i * i <= n && !gefunden {
            if z % i == 0 {
                gefunden = true;
                p = i;
            } else {
                i += 1;
            }
        }
        if !gefunden {
            p = z;
        }
        faktoren.push(p);
        z /= p;
    }
    faktoren
}

pub fn divisorGenerator(n: i64) -> Vec<i64> {
    let mut out = Vec::new();
    let mut large_divisors = Vec::new();
    if n <= 0 {
        return out;
    }
    let limit = (n as f64).sqrt() as i64;
    for i in 1..=limit {
        if n % i == 0 {
            out.push(i);
            if i * i != n {
                large_divisors.push(n / i);
            }
        }
    }
    large_divisors.reverse();
    out.extend(large_divisors);
    out
}

pub fn primRepeat(n: &[i64]) -> Vec<(i64, i64)> {
    let mut reversed = n.to_vec();
    reversed.reverse();
    let mut c = 1i64;
    let mut b: Option<i64> = None;
    let mut d: Vec<(i64, i64)> = Vec::new();
    for a in reversed {
        if b == Some(a) {
            c += 1;
        } else {
            c = 1;
        }
        d.push((a, c));
        b = Some(a);
    }
    d.reverse();

    let mut f = Vec::new();
    let mut previous: Option<i64> = None;
    for (e, g) in d {
        if previous != Some(e) {
            f.push((e, if g == 1 { 1 } else { g }));
        }
        previous = Some(e);
    }
    f
}

pub fn primMultiple(n: i64) -> Vec<(i64, i64)> {
    let mut multiples = vec![(1, n)];
    for (prim, _) in primRepeat(&primFak(n)) {
        if prim != 0 {
            multiples.push((prim, ((n as f64) / (prim as f64)).round() as i64));
        }
    }
    multiples
}

pub fn isPrimMultiple(isIt: i64, multiples1: &[i64], dontReturnList: bool) -> Result<bool, Vec<bool>> {
    let mut are_they = Vec::new();
    let multiples2 = primMultiple(isIt);
    for multiple1 in multiples1 {
        for (_, multiple2) in &multiples2 {
            let hit = multiple1 == multiple2;
            are_they.push(hit);
            if dontReturnList && hit {
                return Ok(true);
            }
        }
    }
    if dontReturnList {
        Ok(false)
    } else {
        Err(are_they)
    }
}

pub fn isPrimMultiple_bool(isIt: i64, multiples1: &[i64]) -> bool {
    match isPrimMultiple(isIt, multiples1, true) {
        Ok(value) => value,
        Err(_) => false,
    }
}

pub fn couldBePrimeNumberPrimzahlkreuz(num: i64) -> bool {
    matches!(num.rem_euclid(24), 1 | 5 | 7 | 11 | 13 | 17 | 19 | 23)
}

pub fn couldBePrimeNumberPrimzahlkreuz_fuer_innen(num: i64) -> bool {
    matches!(num.rem_euclid(24), 5 | 11 | 17 | 23)
}

pub fn couldBePrimeNumberPrimzahlkreuz_fuer_aussen(num: i64) -> bool {
    matches!(num.rem_euclid(24), 1 | 7 | 13 | 19)
}

pub fn divisors_without_one(n: i64) -> BTreeSet<i64> {
    divisorGenerator(n).into_iter().filter(|v| *v != 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moon_number_keeps_python_exponent_offset() {
        assert_eq!(moonNumber(4), (vec![2], vec![0]));
        assert_eq!(moonNumber(8), (vec![3, 2], vec![0, 1]));
    }

    #[test]
    fn prim_multiple_starts_with_identity_pair_like_python() {
        assert_eq!(primMultiple(12), vec![(1, 12), (2, 6), (3, 4)]);
        assert_eq!(isPrimMultiple_bool(12, &[6]), true);
        assert_eq!(isPrimMultiple_bool(12, &[5]), false);
    }

    #[test]
    fn lib4tables_module_exposes_python_source_and_tag_facade() {
        use crate::shared::lib4tables_enum_py::ST;

        assert!(PYTHON_SOURCE__LIB4TABLES.contains("def primCreativity"));
        assert_eq!(
            tableTags2_for_column(14),
            Some([ST::sternPolygon, ST::galaxie].into_iter().collect())
        );
        assert!(tableTags_columns_for_tags([ST::sternPolygon, ST::galaxie]).contains(&14));
    }
}
