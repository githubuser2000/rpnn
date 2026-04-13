use std::collections::{BTreeMap, BTreeSet};

use crate::domain::python_html_meta_fallback::fallback_html_meta_for_column;
use crate::domain::python_source_of_truth::{alias_summary_for_column, exact_meta_for_column, reverse_map_canonical_pairs};
use crate::shared::words_py::Words;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HtmlDeclMeta {
    pub column_number: i64,
    pub classes: Vec<String>,
    pub class_string: String,
    pub data_attributes: BTreeMap<String, String>,
}

fn slug(txt: &str) -> String {
    txt.trim()
        .replace(' ', "_")
        .replace('(', "")
        .replace(')', "")
        .replace('/', "_")
        .replace(',', "_")
        .replace('ß', "ss")
        .to_lowercase()
}

pub fn html_meta_for_column(words: &Words, column_number: i64) -> Option<HtmlDeclMeta> {
    let exact = exact_meta_for_column(words, column_number);
    if exact.is_empty() {
        return fallback_html_meta_for_column(column_number);
    }
    let mut classes = vec![format!("p1_col_{}", column_number)];
    let first = &exact[0];
    classes.push(format!("p2_{}", slug(&first.parameter_main)));
    classes.push(format!("p3_{}", slug(&first.parameter)));

    let summary = alias_summary_for_column(words, column_number)?;
    for alias in &summary.parameter_main_aliases {
        classes.push(format!("p2alias_{}", slug(alias)));
    }
    for alias in &summary.parameter_aliases {
        classes.push(format!("p3alias_{}", slug(alias)));
    }
    classes.sort();
    classes.dedup();

    let mut data_attributes = BTreeMap::new();
    data_attributes.insert("data-column-number".to_string(), column_number.to_string());
    data_attributes.insert(
        "data-column-group".to_string(),
        summary
            .canonical_pairs
            .iter()
            .map(|(a, b)| format!("{}::{}", a, b))
            .collect::<Vec<_>>()
            .join("|")
    );
    data_attributes.insert(
        "data-main-aliases".to_string(),
        summary.parameter_main_aliases.join("|")
    );
    data_attributes.insert(
        "data-parameter-aliases".to_string(),
        summary.parameter_aliases.join("|")
    );

    let class_string = classes.join(" ");
    Some(HtmlDeclMeta {
        column_number,
        classes,
        class_string,
        data_attributes,
    })
}

pub fn all_known_html_columns(words: &Words) -> Vec<i64> {
    let mut all = BTreeSet::new();
    for column in reverse_map_canonical_pairs(words).keys() {
        all.insert(*column);
    }
    for column in 1..=728 {
        if html_meta_for_column(words, column).is_some() {
            all.insert(column);
        }
    }
    all.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::words_py::Words;

    #[test]
    fn html_meta_has_p_classes() {
        let words = Words::new();
        let meta = html_meta_for_column(&words, 240).unwrap();
        assert!(meta.classes.iter().any(|c| c.starts_with("p1_")));
        assert!(meta.classes.iter().any(|c| c.starts_with("p2_")));
        assert!(meta.classes.iter().any(|c| c.starts_with("p3_")));
    }

    #[test]
    fn html_meta_keeps_alias_attributes() {
        let words = Words::new();
        let meta = html_meta_for_column(&words, 240).unwrap();
        assert!(meta.data_attributes.contains_key("data-main-aliases"));
        assert!(meta.data_attributes.contains_key("data-parameter-aliases"));
    }

    #[test]
    fn html_meta_has_alias_classes() {
        let words = Words::new();
        let meta = html_meta_for_column(&words, 240).unwrap();
        assert!(meta.classes.iter().any(|c| c.starts_with("p2alias_")));
        assert!(meta.classes.iter().any(|c| c.starts_with("p3alias_")));
    }

    #[test]
    fn fallback_html_meta_is_available_for_missing_column() {
        let words = Words::new();
        let meta = html_meta_for_column(&words, 728).unwrap();
        assert!(meta.classes.iter().any(|c| c == "p1_col_728"));
        assert!(meta.data_attributes.contains_key("data-column-group"));
    }
}
