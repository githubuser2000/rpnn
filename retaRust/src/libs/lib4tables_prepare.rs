#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::collections::{BTreeMap, BTreeSet};

use crate::shared::lib4tables_enum_py::ST;

pub use crate::libs::tableHandling::{getShellRowsAmount, setShellRowsAmount, TablesPrepare as Prepare};

pub const PYTHON_SOURCE__LIB4TABLES_PREPARE: &str = include_str!("../../python_reference/lib4tables_prepare.py");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Wraptype {
    pyphen = 1,
    pyhyphen = 2,
    nohyphen = 3,
}

pub fn chunks<T: Clone>(lst: &[T], n: usize) -> Vec<Vec<T>> {
    if n == 0 {
        panic!("range() arg 3 must not be zero");
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lst.len() {
        out.push(lst[i..(i + n).min(lst.len())].to_vec());
        i += n;
    }
    out
}

fn string_chunks(text: &str, n: usize) -> Vec<String> {
    if n == 0 {
        panic!("range() arg 3 must not be zero");
    }
    let chars = text.chars().collect::<Vec<_>>();
    chunks(&chars, n)
        .into_iter()
        .map(|part| part.into_iter().collect::<String>())
        .collect()
}

pub fn splitMoreIfNotSmall(textList: Vec<String>, lenToBe: usize) -> Vec<String> {
    let neededToBeDoneAtAll = textList.iter().any(|text| text.chars().count() > lenToBe);
    if !neededToBeDoneAtAll {
        return textList;
    }

    let mut newList = Vec::new();
    for text in textList {
        if text.chars().count() > lenToBe {
            newList.extend(string_chunks(&text, lenToBe));
        } else {
            newList.push(text);
        }
    }
    newList
}

pub fn alxwrap(text: &str, len_: usize) -> Vec<String> {
    if len_ == 0 || text.chars().count() <= len_ {
        return vec![text.to_string()];
    }
    // Python versucht zuerst pyhyphen/pyphen. Die Rust-Schicht nutzt hier die
    // deterministische harte Fallback-Aufteilung, damit lange Zellen nie wieder
    // ungeprüft als ein einzelnes Element durchrutschen.
    splitMoreIfNotSmall(vec![text.to_string()], len_)
}

pub fn wrapping(text: &str, length: usize) -> Option<Vec<String>> {
    if text.chars().count() > length && length != 0 {
        Some(alxwrap(text, length))
    } else {
        None
    }
}

pub fn fromUntil(mut a: Vec<String>) -> (i64, i64) {
    if a.first().map(|v| !v.is_empty() && v.chars().all(|ch| ch.is_ascii_digit())).unwrap_or(false) {
        let first = a[0].parse::<i64>().unwrap_or(1);
        if a.len() == 2 && !a[1].is_empty() && a[1].chars().all(|ch| ch.is_ascii_digit()) {
            return (first, a[1].parse::<i64>().unwrap_or(1));
        }
        if a.len() == 1 {
            a.push(first.to_string());
            return (1, a[1].parse::<i64>().unwrap_or(first));
        }
        return (1, 1);
    }
    (1, 1)
}

pub fn cellWork(cell: &str, certaintextwidth: usize) -> Vec<String> {
    let cell = cell.trim().to_string();
    if certaintextwidth == 0 {
        return vec![cell];
    }

    let mut isItNone = wrapping(&cell, certaintextwidth);
    let mut cell2: Vec<String> = Vec::new();
    let mut rest = cell;

    while matches!(&isItNone, Some(parts) if !parts.is_empty()) {
        let parts = isItNone.take().unwrap_or_default();
        cell2.extend(parts);
        if let Some(last) = cell2.pop() {
            rest = last;
        } else {
            rest.clear();
        }
        isItNone = wrapping(&rest, certaintextwidth);
        if rest.chars().count() > certaintextwidth && isItNone.is_none() {
            let head = rest.chars().take(certaintextwidth).collect::<String>();
            let tail = rest.chars().skip(certaintextwidth).collect::<String>();
            cell2.push(head);
            isItNone = Some(vec![tail]);
        }
    }

    cell2.push(rest.chars().take(certaintextwidth).collect::<String>());
    cell2
}


/// Python `Prepare.prepare4out_beforeForLoop_SpaltenZeilenBestimmen` as a
/// module-level facade for callers that imported `lib4tables_prepare` directly
/// instead of going through `Tables.getPrepare`.
#[allow(non_snake_case)]
pub fn prepare4out_beforeForLoop_SpaltenZeilenBestimmen(
    prepare: &Prepare,
    relitable: Vec<Vec<String>>,
    paramLines: Vec<String>,
    paramLinesNot: Vec<String>,
) -> (Vec<String>, i64, Vec<Vec<String>>, i64, Vec<i64>) {
    prepare.prepare4out_beforeForLoop_SpaltenZeilenBestimmen(relitable, paramLines, paramLinesNot)
}

/// Python `Prepare.prepare4out_LoopBody` facade.
#[allow(non_snake_case)]
pub fn prepare4out_LoopBody(
    prepare: &Prepare,
    combiRows: i64,
    headingsAmount: i64,
    line: Vec<String>,
    rowsAsNumbers: Vec<i64>,
    u: i64,
) -> Vec<Vec<String>> {
    prepare.prepare4out_LoopBody(combiRows, headingsAmount, line, rowsAsNumbers, u)
}

/// Python `Prepare.prepare4out_Tagging` facade.
#[allow(non_snake_case)]
pub fn prepare4out_Tagging(
    prepare: &Prepare,
    rowsRange: Vec<i64>,
) -> BTreeMap<i64, BTreeSet<ST>> {
    prepare.prepare4out_Tagging(rowsRange)
}

/// Python `Prepare.prepare4out` facade.
#[allow(non_snake_case)]
pub fn prepare4out(
    prepare: &Prepare,
    paramLines: Vec<String>,
    paramLinesNot: Vec<String>,
    relitable: Vec<Vec<String>>,
    rowsAsNumbers: Vec<i64>,
) -> (Vec<String>, Vec<Vec<String>>, i64, Vec<i64>, Vec<i64>) {
    prepare.prepare4out(paramLines, paramLinesNot, relitable, rowsAsNumbers, None, None, None, None, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::libs::tableHandling::Tables;

    #[test]
    fn split_more_only_splits_when_any_entry_is_too_long() {
        assert_eq!(splitMoreIfNotSmall(vec!["ab".into(), "cd".into()], 2), vec!["ab", "cd"]);
        assert_eq!(splitMoreIfNotSmall(vec!["abc".into(), "de".into()], 2), vec!["ab", "c", "de"]);
    }

    #[test]
    fn from_until_matches_python_single_number_expansion() {
        assert_eq!(fromUntil(vec!["7".into()]), (1, 7));
        assert_eq!(fromUntil(vec!["7".into(), "9".into()]), (7, 9));
        assert_eq!(fromUntil(vec!["x".into()]), (1, 1));
    }

    #[test]
    fn prepare_module_facades_call_table_prepare_methods() {
        let tables = Tables::new(Some(20), None);
        tables.getPrepare.set_shellRowsAmount(80);
        tables.getPrepare.set_textWidth(3);
        let row = vec!["zero".to_string(), "abcdef".to_string()];
        assert_eq!(
            prepare4out_LoopBody(&tables.getPrepare, 0, 2, row, vec![1], 1),
            vec![vec!["abc".to_string(), "def".to_string()]]
        );

        let relitable = vec![
            vec!["h0".to_string(), "h1".to_string()],
            vec!["r1c0".to_string(), "r1c1".to_string()],
        ];
        let (_display, headings, _newer, _numlen, rows_range) =
            prepare4out_beforeForLoop_SpaltenZeilenBestimmen(
                &tables.getPrepare,
                relitable,
                vec!["all".to_string()],
                vec![],
            );
        assert_eq!(headings, 2);
        assert_eq!(rows_range, vec![0, 1]);
    }
}
