//! Row-filter and range morphisms transcompiled from
//! `python_arch_reference/reta_architecture/row_filtering.py`.
//!
//! This is the typed Rust owner for the row-selection surface used by table
//! preparation.  The full Python module contains many legacy side-effect hooks;
//! the Rust port below captures the deterministic set morphisms that can be
//! used without a Python `Prepare` instance.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::number_theory::{is_prime_multiple, moon_number};
use crate::row_ranges::{bootstrap_row_range_morphisms, RowRangeMorphismBundle};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RowFilterContext {
    pub original_lines_range: Vec<i64>,
    pub highest_row_1024: i64,
    pub highest_row_114: i64,
    pub if_zeilen_setted: bool,
    pub remove_suns_above_114: bool,
}

impl Default for RowFilterContext {
    fn default() -> Self {
        Self {
            original_lines_range: (0..=1024).collect(),
            highest_row_1024: 1024,
            highest_row_114: 114,
            if_zeilen_setted: false,
            remove_suns_above_114: false,
        }
    }
}

impl RowFilterContext {
    pub fn full_positive_range(&self) -> BTreeSet<i64> {
        (1..=self.highest_row_1024).collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RowFilteringSnapshot {
    pub class: String,
    pub legacy_owner: String,
    pub range_command_morphism: String,
    pub row_filter_morphism: String,
    pub counting_morphism: String,
    pub condition_families: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RowFilteringBundle {
    pub row_ranges: RowRangeMorphismBundle,
}

impl RowFilteringBundle {
    pub fn parameters_cmd_with_some_bereich(
        &self,
        mehrere_bereiche: &str,
        symbol: &str,
        neg: &str,
        keine_neg_beruecksichtigung: bool,
    ) -> BTreeSet<String> {
        parameters_cmd_with_some_bereich(
            &self.row_ranges,
            mehrere_bereiche,
            symbol,
            neg,
            keine_neg_beruecksichtigung,
        )
    }

    pub fn filter_original_lines(
        &self,
        context: &RowFilterContext,
        num_range: BTreeSet<i64>,
        param_lines: &BTreeSet<String>,
    ) -> BTreeSet<i64> {
        filter_original_lines(&self.row_ranges, context, num_range, param_lines)
    }

    pub fn delete_doubles_in_sets<T: Ord + Clone>(
        &self,
        set1: &BTreeSet<T>,
        set2: &BTreeSet<T>,
    ) -> (BTreeSet<T>, BTreeSet<T>) {
        delete_doubles_in_sets(set1, set2)
    }

    pub fn snapshot(&self) -> RowFilteringSnapshot {
        RowFilteringSnapshot {
            class: "RowFilteringBundle".to_string(),
            legacy_owner: "libs.lib4tables_prepare.Prepare".to_string(),
            range_command_morphism: "parameters_cmd_with_some_bereich".to_string(),
            row_filter_morphism: "filter_original_lines".to_string(),
            counting_morphism: "set_zaehlungen".to_string(),
            condition_families: vec![
                "absolute_ranges".to_string(),
                "relative_ranges".to_string(),
                "moon_sun_planet".to_string(),
                "prime_multiples".to_string(),
                "powers".to_string(),
                "ordinary_multiples".to_string(),
                "neighbour_inversion".to_string(),
                "z_y_position_filters".to_string(),
            ],
        }
    }
}

pub fn bootstrap_row_filtering() -> RowFilteringBundle {
    RowFilteringBundle {
        row_ranges: bootstrap_row_range_morphisms(None),
    }
}

pub fn parameters_cmd_with_some_bereich(
    row_ranges: &RowRangeMorphismBundle,
    mehrere_bereiche: &str,
    symbol: &str,
    neg: &str,
    keine_neg_beruecksichtigung: bool,
) -> BTreeSet<String> {
    let mut results = BTreeSet::new();
    if keine_neg_beruecksichtigung {
        if row_ranges.is_row_range(mehrere_bereiche) {
            results.insert(format!("_{symbol}_{mehrere_bereiche}"));
        }
        return results;
    }

    for raw in mehrere_bereiche.split(',') {
        if raw.is_empty() {
            continue;
        }
        let (include, cleaned) = if neg.is_empty() {
            (!raw.starts_with('-'), raw)
        } else if let Some(stripped) = raw.strip_prefix(neg) {
            (true, stripped)
        } else {
            (false, raw)
        };
        if include && row_ranges.is_row_range(cleaned) {
            results.insert(format!("_{symbol}_{cleaned}"));
        }
    }
    results
}

pub fn delete_doubles_in_sets<T: Ord + Clone>(
    set1: &BTreeSet<T>,
    set2: &BTreeSet<T>,
) -> (BTreeSet<T>, BTreeSet<T>) {
    let intersection = set1.intersection(set2).cloned().collect::<BTreeSet<_>>();
    (
        set1.difference(&intersection).cloned().collect(),
        set2.difference(&intersection).cloned().collect(),
    )
}

pub fn from_until(values: &[String]) -> (i64, i64) {
    if values.is_empty() || !values[0].chars().all(|ch| ch.is_ascii_digit()) {
        return (1, 1);
    }
    let first = values[0].parse::<i64>().unwrap_or(1);
    if values.len() == 1 {
        return (1, first);
    }
    if values.len() == 2 && values[1].chars().all(|ch| ch.is_ascii_digit()) {
        return (first, values[1].parse::<i64>().unwrap_or(first));
    }
    (1, 1)
}

pub fn moon_sun_filter(
    moon_not_sun: bool,
    numbers: impl IntoIterator<Item = i64>,
) -> BTreeSet<i64> {
    numbers
        .into_iter()
        .filter(|number| !moon_number(*number).0.is_empty() == moon_not_sun)
        .collect()
}

pub fn filter_original_lines(
    row_ranges: &RowRangeMorphismBundle,
    context: &RowFilterContext,
    mut num_range: BTreeSet<i64>,
    param_lines: &BTreeSet<String>,
) -> BTreeSet<i64> {
    num_range.remove(&0);

    if param_lines.contains("all")
        || param_lines
            .difference(&BTreeSet::from(["ka".to_string(), "ka2".to_string()]))
            .next()
            .is_none()
        || !context.if_zeilen_setted
    {
        num_range = context.full_positive_range();
    } else {
        num_range.clear();
    }

    let mut absolute_ranges = Vec::new();
    let mut relative_ranges = Vec::new();
    let mut position_z = Vec::new();
    let mut position_y = Vec::new();
    let mut powers = Vec::new();
    let mut prime_multiples = Vec::new();
    let mut ordinary_multiples = Vec::new();
    let mut want_moon = false;
    let mut want_sun = false;
    let mut invert_neighbours = false;

    for condition in param_lines {
        if let Some(rest) = condition.strip_prefix("_a_") {
            absolute_ranges.push(rest.to_string());
        } else if let Some(rest) = condition.strip_prefix('a') {
            if row_ranges.is_row_range(rest) {
                absolute_ranges.push(rest.to_string());
            }
        } else if let Some(rest) = condition.strip_prefix("_b_") {
            relative_ranges.push(rest.to_string());
        } else if let Some(rest) = condition.strip_prefix("_z_") {
            position_z.push(rest.to_string());
        } else if let Some(rest) = condition.strip_prefix("_y_") {
            position_y.push(rest.to_string());
        } else if let Some(rest) = condition.strip_prefix("_^_") {
            powers.push(rest.to_string());
        } else if condition == "mond" {
            want_moon = true;
        } else if condition == "sonne" || condition == "schwarzesonne" {
            want_sun = true;
        } else if condition == "_i_" {
            invert_neighbours = true;
        } else if let Some(raw) = condition.strip_suffix('p') {
            if let Ok(value) = raw.parse::<i64>() {
                prime_multiples.push(value);
            }
        } else if let Some(raw) = condition.strip_suffix('v') {
            if let Ok(value) = raw.parse::<i64>() {
                ordinary_multiples.push(value);
            }
        }
    }

    if !absolute_ranges.is_empty() {
        let expression = absolute_ranges.join(",");
        num_range.extend(row_ranges.range_to_numbers(
            &expression,
            false,
            context.highest_row_1024 + 1,
            false,
        ));
        for part in expression.split(',') {
            if let Some(negative) = part.strip_prefix('-') {
                for number in row_ranges.range_to_numbers(
                    negative,
                    false,
                    context.highest_row_1024 + 1,
                    false,
                ) {
                    num_range.remove(&number);
                }
            }
        }
    }

    if !relative_ranges.is_empty() {
        if num_range.is_empty() && !param_lines.contains("all") {
            num_range = (1..=context.highest_row_114).collect();
        }
        let wanted = row_ranges
            .range_to_numbers(
                &relative_ranges.join(","),
                true,
                context.highest_row_114 + 1,
                false,
            )
            .into_iter()
            .collect::<BTreeSet<_>>();
        if !wanted.is_empty() {
            num_range = num_range.intersection(&wanted).copied().collect();
        }
    }

    if want_moon || want_sun {
        if num_range.is_empty() && !param_lines.contains("all") {
            num_range = context.full_positive_range();
        }
        let mut selected = BTreeSet::new();
        if want_moon {
            selected.extend(moon_sun_filter(true, num_range.iter().copied()));
        }
        if want_sun {
            selected.extend(moon_sun_filter(false, num_range.iter().copied()));
        }
        num_range = selected;
    }

    if !prime_multiples.is_empty() {
        if num_range.is_empty() && !param_lines.contains("all") {
            num_range = context.full_positive_range();
        }
        num_range = num_range
            .into_iter()
            .filter(|number| is_prime_multiple(*number, &prime_multiples))
            .collect();
    }

    if !powers.is_empty() {
        if num_range.is_empty() && !param_lines.contains("all") {
            num_range = context.full_positive_range();
        }
        let bases = row_ranges.range_to_numbers(
            &powers.join(","),
            false,
            context.highest_row_1024 + 1,
            false,
        );
        let last = num_range
            .iter()
            .next_back()
            .copied()
            .unwrap_or(context.highest_row_1024);
        let mut power_set = BTreeSet::new();
        for base in bases {
            if base <= 1 {
                continue;
            }
            let mut value = 1i64;
            while value <= last {
                power_set.insert(value);
                match value.checked_mul(base) {
                    Some(next) => value = next,
                    None => break,
                }
            }
        }
        num_range = num_range.intersection(&power_set).copied().collect();
        num_range.remove(&1);
    }

    if !ordinary_multiples.is_empty() {
        num_range = num_range
            .into_iter()
            .filter(|number| {
                ordinary_multiples
                    .iter()
                    .any(|divisor| *divisor != 0 && *number % *divisor == 0)
            })
            .collect();
    }

    if context.remove_suns_above_114 {
        num_range = num_range
            .into_iter()
            .filter(|number| {
                *number <= context.highest_row_114 || !moon_number(*number).0.is_empty()
            })
            .collect();
    }

    if invert_neighbours {
        let h = context.highest_row_1024;
        let original = num_range.clone();
        num_range = (1..=h)
            .filter(|i| {
                (original.contains(&(i + 1)) || original.contains(&(i - 1)))
                    && !original.contains(i)
            })
            .collect();
    }

    if !position_z.is_empty() || !position_y.is_empty() {
        let ordered = num_range.iter().copied().collect::<Vec<_>>();
        let mut mapped = BTreeSet::new();
        let position_expression = if !position_z.is_empty() {
            position_z.join(",")
        } else {
            position_y.join(",")
        };
        let one_based_positions = row_ranges.range_to_numbers(
            &position_expression,
            !position_y.is_empty(),
            context.highest_row_1024 + 1,
            false,
        );
        for pos in one_based_positions {
            if pos > 0 {
                if let Some(number) = ordered.get((pos - 1) as usize) {
                    mapped.insert(*number);
                }
            }
        }
        num_range = num_range.intersection(&mapped).copied().collect();
    }

    num_range.remove(&0);
    num_range
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_delete_matches_python_shape() {
        let (a, b) = delete_doubles_in_sets(&BTreeSet::from([1, 2, 3]), &BTreeSet::from([3, 4]));
        assert_eq!(a, BTreeSet::from([1, 2]));
        assert_eq!(b, BTreeSet::from([4]));
    }

    #[test]
    fn absolute_a_ranges_are_expanded() {
        let bundle = bootstrap_row_filtering();
        let context = RowFilterContext {
            if_zeilen_setted: true,
            ..RowFilterContext::default()
        };
        let lines = BTreeSet::from(["_a_2-4".to_string()]);
        let result = bundle.filter_original_lines(&context, BTreeSet::new(), &lines);
        assert_eq!(result, BTreeSet::from([2, 3, 4]));
    }

    #[test]
    fn command_builder_respects_negation_prefix() {
        let bundle = bootstrap_row_filtering();
        let result = bundle.parameters_cmd_with_some_bereich("1,-2,3", "a", "", false);
        assert_eq!(
            result,
            BTreeSet::from(["_a_1".to_string(), "_a_3".to_string()])
        );
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "cutset",
    "moonsun",
    "zeile_which_zaehlung",
    "set_zaehlungen",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
