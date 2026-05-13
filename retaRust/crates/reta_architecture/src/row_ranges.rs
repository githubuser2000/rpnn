//! Row-range morphisms transcompiled from
//! `python_arch_reference/reta_architecture/row_ranges.py` and
//! `input_semantics.RowRangeSyntax`.
//!
//! This module intentionally avoids a regex dependency.  It implements the
//! same accepted surface for the integer/fraction range tokens used by Reta:
//! comma lists, subtractive segments, `v`-prefixed multiples, plus-offsets and
//! explicit integer set/list/tuple literals.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RowRangeSyntax {
    pub multiple_prefix: String,
    pub comma_split_pattern: String,
}

impl Default for RowRangeSyntax {
    fn default() -> Self {
        Self {
            multiple_prefix: "v".to_string(),
            comma_split_pattern: r",(?![^\[\]\{\}\(\)]*[\]\}\)])".to_string(),
        }
    }
}

impl RowRangeSyntax {
    pub fn new(multiple_prefix: impl Into<String>) -> Self {
        Self {
            multiple_prefix: multiple_prefix.into(),
            ..Self::default()
        }
    }

    pub fn split_comma_list(&self, text: &str) -> Vec<String> {
        split_commas_outside_brackets(text)
    }

    pub fn compact_comma_list(&self, text: &str) -> String {
        self.split_comma_list(text)
            .into_iter()
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn is_integer_range_token(&self, text: &str) -> bool {
        is_integer_range_token_with_prefix(text, &self.multiple_prefix)
    }

    pub fn is_fraction_range_token(&self, text: &str) -> bool {
        is_fraction_range_token_with_prefix(text, &self.multiple_prefix)
    }
}

fn split_commas_outside_brackets(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut square_depth = 0i64;
    let mut curly_depth = 0i64;
    let mut paren_depth = 0i64;

    for ch in text.chars() {
        match ch {
            '[' => {
                square_depth += 1;
                current.push(ch);
            }
            ']' => {
                square_depth = (square_depth - 1).max(0);
                current.push(ch);
            }
            '{' => {
                curly_depth += 1;
                current.push(ch);
            }
            '}' => {
                curly_depth = (curly_depth - 1).max(0);
                current.push(ch);
            }
            '(' => {
                paren_depth += 1;
                current.push(ch);
            }
            ')' => {
                paren_depth = (paren_depth - 1).max(0);
                current.push(ch);
            }
            ',' if square_depth == 0 && curly_depth == 0 && paren_depth == 0 => {
                out.push(current);
                current = String::new();
            }
            _ => current.push(ch),
        }
    }
    out.push(current);
    out
}

fn strip_prefix_once<'a>(text: &'a str, prefix: &str) -> (&'a str, bool) {
    if !prefix.is_empty() && text.starts_with(prefix) {
        (&text[prefix.len()..], true)
    } else {
        (text, false)
    }
}

fn parse_signed_decimal_prefix(text: &str) -> Option<(usize, bool)> {
    let mut chars = text.char_indices().peekable();
    if let Some((_, '-')) = chars.peek().copied() {
        chars.next();
    }
    let start_digits = chars.peek().map(|(idx, _)| *idx).unwrap_or(text.len());
    let mut end = start_digits;
    let mut saw_digit = false;
    while let Some((idx, ch)) = chars.peek().copied() {
        if ch.is_ascii_digit() {
            saw_digit = true;
            end = idx + ch.len_utf8();
            chars.next();
        } else {
            break;
        }
    }
    saw_digit.then_some((end, text.starts_with('-')))
}

fn parse_unsigned_decimal_prefix(text: &str) -> Option<usize> {
    let mut end = 0usize;
    let mut saw_digit = false;
    for (idx, ch) in text.char_indices() {
        if ch.is_ascii_digit() {
            saw_digit = true;
            end = idx + ch.len_utf8();
        } else {
            break;
        }
    }
    saw_digit.then_some(end)
}

fn is_unsigned_decimal(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|ch| ch.is_ascii_digit())
}

fn parse_fraction_prefix(text: &str) -> Option<usize> {
    let (num_end, _) = parse_signed_decimal_prefix(text)?;
    let rest = &text[num_end..];
    if !rest.starts_with('/') {
        return None;
    }
    let denominator = &rest[1..];
    let den_end = parse_unsigned_decimal_prefix(denominator)?;
    Some(num_end + 1 + den_end)
}

fn is_integer_range_token_with_prefix(text: &str, prefix: &str) -> bool {
    let text = text.trim();
    let (mut rest, _) = strip_prefix_once(text, prefix);
    let Some((first_end, _)) = parse_signed_decimal_prefix(rest) else {
        return false;
    };
    rest = &rest[first_end..];

    if rest.starts_with('-') {
        let after_dash = &rest[1..];
        let Some(second_end) = parse_unsigned_decimal_prefix(after_dash) else {
            return false;
        };
        rest = &after_dash[second_end..];
    }

    while rest.starts_with('+') {
        let after_plus = &rest[1..];
        let Some(plus_end) = parse_unsigned_decimal_prefix(after_plus) else {
            return false;
        };
        rest = &after_plus[plus_end..];
    }

    rest.is_empty()
}

fn is_fraction_range_token_with_prefix(text: &str, prefix: &str) -> bool {
    let text = text.trim();
    let (mut rest, _) = strip_prefix_once(text, prefix);
    let Some(first_end) = parse_fraction_prefix(rest) else {
        return false;
    };
    rest = &rest[first_end..];

    if rest.starts_with('-') {
        let after_dash = &rest[1..];
        let Some(second_end) = parse_fraction_prefix(after_dash) else {
            return false;
        };
        rest = &after_dash[second_end..];
    }

    while rest.starts_with('+') {
        let after_plus = &rest[1..];
        let Some(plus_end) = parse_fraction_prefix(after_plus) else {
            return false;
        };
        rest = &after_plus[plus_end..];
    }

    rest.is_empty()
}

pub fn str_as_generator_to_set(text: &str) -> Option<BTreeSet<i64>> {
    let mut text = text.trim().to_string();
    if text.len() >= 2 && text.starts_with('(') && text.ends_with(')') {
        text = format!("[{}]", &text[1..text.len() - 1]);
    }
    if text.len() < 2 {
        return None;
    }
    let starts_ok = text.starts_with('[') || text.starts_with('{');
    let ends_ok = text.ends_with(']') || text.ends_with('}');
    if !starts_ok || !ends_ok {
        return None;
    }

    let inner = &text[1..text.len() - 1];
    if inner.trim().is_empty() {
        return Some(BTreeSet::new());
    }

    let mut out = BTreeSet::new();
    for raw in split_commas_outside_brackets(inner) {
        let token = raw.trim();
        if token.is_empty() {
            continue;
        }
        let Ok(value) = token.parse::<i64>() else {
            return None;
        };
        out.insert(value);
    }
    Some(out)
}

pub fn is_fraction_range_token(text: &str, syntax: Option<&RowRangeSyntax>) -> bool {
    syntax
        .cloned()
        .unwrap_or_default()
        .is_fraction_range_token(text)
}

pub fn is_integer_range_token(text: &str, syntax: Option<&RowRangeSyntax>) -> bool {
    syntax
        .cloned()
        .unwrap_or_default()
        .is_integer_range_token(text)
}

pub fn is_row_range_token(text: &str, syntax: Option<&RowRangeSyntax>) -> bool {
    let syntax = syntax.cloned().unwrap_or_default();
    syntax.is_integer_range_token(text)
        || str_as_generator_to_set(text).is_some()
        || text
            .get(1..)
            .and_then(str_as_generator_to_set)
            .is_some()
}

pub fn is_fraction_or_integer_range(text: &str, syntax: Option<&RowRangeSyntax>) -> bool {
    let syntax = syntax.cloned().unwrap_or_default();
    syntax
        .split_comma_list(text)
        .iter()
        .all(|token| is_fraction_range_token(token, Some(&syntax)) || is_row_range_token(token, Some(&syntax)))
}

pub fn is_fraction_range(text: &str, syntax: Option<&RowRangeSyntax>) -> bool {
    let syntax = syntax.cloned().unwrap_or_default();
    let tokens = syntax.split_comma_list(text);
    let any_at_all = tokens.iter().any(|token| !token.is_empty());
    tokens
        .iter()
        .all(|token| is_fraction_range_token(token, Some(&syntax)) || (token.is_empty() && any_at_all))
}

pub fn is_row_range(text: &str, syntax: Option<&RowRangeSyntax>) -> bool {
    let syntax = syntax.cloned().unwrap_or_default();
    let tokens = syntax.split_comma_list(text);
    let any_at_all = tokens.iter().any(|token| !token.is_empty());
    tokens
        .iter()
        .all(|token| is_row_range_token(token, Some(&syntax)) || (token.is_empty() && any_at_all))
}

fn add_non_multiple_values(range_couple: &[String], around: &[i64], max_value: i64, target: &mut BTreeSet<i64>) {
    if range_couple.len() < 2 {
        return;
    }
    let Ok(start) = range_couple[0].parse::<i64>() else { return; };
    let Ok(end) = range_couple[1].parse::<i64>() else { return; };
    for number in start..=end {
        for offset in around {
            let plus = number + offset;
            if plus < max_value {
                target.insert(plus);
            }
            let minus = number - offset;
            if minus > 0 && minus < max_value {
                target.insert(minus);
            }
        }
    }
}

fn add_multiple_values(range_couple: &[String], around: &[i64], max_value: i64, target: &mut BTreeSet<i64>) {
    if range_couple.len() < 2 {
        return;
    }
    let Ok(start) = range_couple[0].parse::<i64>() else { return; };
    let Ok(end) = range_couple[1].parse::<i64>() else { return; };
    if start == 0 {
        return;
    }

    let mut i = 0i64;
    let offsets = if around.is_empty() { vec![0] } else { around.to_vec() };
    loop {
        if !offsets.iter().all(|offset| start * i < max_value - offset) {
            break;
        }
        i += 1;
        for number in start..=end {
            if around.is_empty() || around.iter().all(|value| *value == 0) {
                let value = number * i;
                if value <= max_value {
                    target.insert(value);
                }
            } else {
                for offset in around {
                    let plus = (number * i) + offset;
                    if plus <= max_value {
                        target.insert(plus);
                    }
                    let minus = (number * i) - offset;
                    if minus > 0 && minus < max_value {
                        target.insert(minus);
                    }
                }
            }
        }
    }
}

fn add_range_couple_values(
    range_couple: &mut [String],
    max_value: i64,
    target: &mut BTreeSet<i64>,
    multiples: bool,
) {
    if range_couple.len() != 2 || !is_unsigned_decimal(&range_couple[0]) || range_couple[0] == "0" {
        return;
    }

    let plus_tokens = range_couple[1].split('+').map(str::to_string).collect::<Vec<_>>();
    let around = if plus_tokens.len() < 2 {
        vec![0]
    } else {
        let mut numbers = Vec::new();
        for token in &plus_tokens {
            if !is_unsigned_decimal(token) {
                return;
            }
            let Ok(value) = token.parse::<i64>() else { return; };
            numbers.push(value);
        }
        if numbers.is_empty() {
            return;
        }
        range_couple[1] = numbers[0].to_string();
        numbers.into_iter().skip(1).collect::<Vec<_>>()
    };

    if multiples {
        add_multiple_values(range_couple, &around, max_value, target);
    } else {
        add_non_multiple_values(range_couple, &around, max_value, target);
    }
}

fn add_single_range_segment(
    raw_segment: &str,
    include: &mut BTreeSet<i64>,
    exclude: &mut BTreeSet<i64>,
    max_value: i64,
    multiples: bool,
) {
    let mut segment = raw_segment.to_string();
    let target_is_exclude = if segment.len() > 1 && segment.starts_with('-') {
        segment = segment[1..].to_string();
        true
    } else if !segment.is_empty() && !segment.starts_with('-') {
        false
    } else {
        return;
    };

    let plus_tokens = segment.split('+').map(str::to_string).collect::<Vec<_>>();
    if is_unsigned_decimal(&segment) {
        segment = format!("{segment}-{segment}");
    } else if plus_tokens
        .first()
        .is_some_and(|first| is_unsigned_decimal(first))
    {
        segment = format!("{}-{}", plus_tokens[0], plus_tokens[0]);
        if plus_tokens.len() > 1 {
            segment.push('+');
            segment.push_str(&plus_tokens[1..].join("+"));
        }
    }

    let mut range_couple = segment.split('-').map(str::to_string).collect::<Vec<_>>();
    if target_is_exclude {
        add_range_couple_values(&mut range_couple, max_value, exclude, multiples);
    } else {
        add_range_couple_values(&mut range_couple, max_value, include, multiples);
    }
}

pub fn range_to_numbers(
    ranges_text: &str,
    multiples: bool,
    max_value: i64,
    allow_less_equal_zero: bool,
    syntax: Option<&RowRangeSyntax>,
) -> BTreeSet<i64> {
    let syntax = syntax.cloned().unwrap_or_default();
    let ranges_text = syntax.compact_comma_list(ranges_text);
    if !is_row_range(&ranges_text, Some(&syntax)) {
        return BTreeSet::new();
    }

    let unlimited = !multiples && max_value == 0;
    let effective_max = if unlimited { i64::MAX / 4 } else { max_value };
    let mut include = BTreeSet::new();
    let mut exclude = BTreeSet::new();

    for mut segment in syntax.split_comma_list(&ranges_text) {
        if segment.len() > 1 && segment.starts_with('-') {
            if let Some(generated) = str_as_generator_to_set(&segment[1..]) {
                exclude.extend(generated);
                continue;
            }
        } else if !segment.is_empty() && !segment.starts_with('-') {
            if let Some(generated) = str_as_generator_to_set(&segment) {
                include.extend(generated);
                continue;
            }
        }

        let segment_multiples = if !syntax.multiple_prefix.is_empty()
            && segment.starts_with(&syntax.multiple_prefix)
        {
            segment = segment[syntax.multiple_prefix.len()..].to_string();
            true
        } else {
            false
        };
        let local_max = if (multiples || segment_multiples) && unlimited {
            1028
        } else {
            effective_max
        };
        add_single_range_segment(
            &segment,
            &mut include,
            &mut exclude,
            local_max,
            multiples || segment_multiples,
        );
    }

    let mut result = include
        .difference(&exclude)
        .copied()
        .collect::<BTreeSet<_>>();
    if !allow_less_equal_zero {
        result = result.into_iter().filter(|value| *value > 0).collect();
    }
    result
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RowRangeMorphismBundle {
    pub syntax: RowRangeSyntax,
    pub legacy_owner: String,
    pub activated_stage: u32,
}

impl Default for RowRangeMorphismBundle {
    fn default() -> Self {
        Self {
            syntax: RowRangeSyntax::default(),
            legacy_owner: "libs.center".to_string(),
            activated_stage: 37,
        }
    }
}

impl RowRangeMorphismBundle {
    pub fn str_as_generator(&self, text: &str) -> Option<BTreeSet<i64>> {
        str_as_generator_to_set(text)
    }

    pub fn is_fraction_token(&self, text: &str) -> bool {
        is_fraction_range_token(text, Some(&self.syntax))
    }

    pub fn is_integer_token(&self, text: &str) -> bool {
        is_integer_range_token(text, Some(&self.syntax))
    }

    pub fn is_row_token(&self, text: &str) -> bool {
        is_row_range_token(text, Some(&self.syntax))
    }

    pub fn is_fraction_range(&self, text: &str) -> bool {
        is_fraction_range(text, Some(&self.syntax))
    }

    pub fn is_row_range(&self, text: &str) -> bool {
        is_row_range(text, Some(&self.syntax))
    }

    pub fn is_fraction_or_integer_range(&self, text: &str) -> bool {
        is_fraction_or_integer_range(text, Some(&self.syntax))
    }

    pub fn range_to_numbers(
        &self,
        ranges_text: &str,
        multiples: bool,
        max_value: i64,
        allow_less_equal_zero: bool,
    ) -> BTreeSet<i64> {
        range_to_numbers(
            ranges_text,
            multiples,
            max_value,
            allow_less_equal_zero,
            Some(&self.syntax),
        )
    }

    pub fn morphisms(&self) -> Vec<&'static str> {
        vec![
            "str_as_generator_to_set",
            "is_fraction_range_token",
            "is_integer_range_token",
            "is_row_range_token",
            "is_fraction_or_integer_range",
            "is_fraction_range",
            "is_row_range",
            "range_to_numbers",
            "add_single_range_segment",
            "add_range_couple_values",
            "add_non_multiple_values",
            "add_multiple_values",
        ]
    }

    pub fn observable_invariant(&self) -> &'static str {
        "center wrappers and architecture morphisms return identical row sets for accepted row-range expressions"
    }
}

pub fn bootstrap_row_range_morphisms(syntax: Option<RowRangeSyntax>) -> RowRangeMorphismBundle {
    RowRangeMorphismBundle {
        syntax: syntax.unwrap_or_default(),
        ..RowRangeMorphismBundle::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(values: &[i64]) -> BTreeSet<i64> {
        values.iter().copied().collect()
    }

    #[test]
    fn comma_split_ignores_bracketed_commas() {
        let syntax = RowRangeSyntax::default();
        assert_eq!(syntax.split_comma_list("1,{2,3},4"), vec!["1", "{2,3}", "4"]);
        assert_eq!(syntax.compact_comma_list("1,,2"), "1,2");
    }

    #[test]
    fn token_checks_match_python_surface() {
        let syntax = RowRangeSyntax::default();
        assert!(syntax.is_integer_range_token("1-3+2"));
        assert!(syntax.is_integer_range_token("v2"));
        assert!(syntax.is_fraction_range_token("1/2-3/4+5/6"));
        assert!(is_row_range_token("{1,2,3}", Some(&syntax)));
        assert!(is_row_range("1-3,-2", Some(&syntax)));
        assert!(!is_row_range("abc", Some(&syntax)));
    }

    #[test]
    fn range_expansion_handles_include_exclude_and_sets() {
        assert_eq!(range_to_numbers("1-5,-3", false, 10, false, None), set(&[1, 2, 4, 5]));
        assert_eq!(range_to_numbers("{1,3,5},-3", false, 10, false, None), set(&[1, 5]));
        assert_eq!(range_to_numbers("(1,2)", false, 10, false, None), set(&[1, 2]));
    }

    #[test]
    fn multiples_and_plus_offsets_follow_legacy_rules() {
        assert_eq!(range_to_numbers("v2", false, 6, false, None), set(&[2, 4, 6]));
        assert_eq!(range_to_numbers("2+1", false, 10, false, None), set(&[1, 3]));
    }
}
