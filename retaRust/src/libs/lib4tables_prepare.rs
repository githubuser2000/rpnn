#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub use crate::libs::tableHandling::{getShellRowsAmount, setShellRowsAmount, TablesPrepare as Prepare};

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
