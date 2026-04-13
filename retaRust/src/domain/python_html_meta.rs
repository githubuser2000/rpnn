use crate::domain::python_source_of_truth::{exact_meta_for_column, ExactPythonColumnMeta};
use crate::shared::words_py::Words;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HtmlDeclMeta {
    pub column_number: i64,
    pub classes: Vec<String>,
    pub data_attributes: Vec<(String, String)>,
}

fn slugify_piece(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        if ch.is_alphanumeric() {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    while out.contains("__") {
        out = out.replace("__", "_");
    }
    out.trim_matches('_').to_string()
}

fn push_unique_string(out: &mut Vec<String>, value: String) {
    if !value.is_empty() && !out.contains(&value) {
        out.push(value);
    }
}

fn push_unique_attribute(out: &mut Vec<(String, String)>, key: String, value: String) {
    if !key.is_empty() && !value.is_empty() && !out.contains(&(key.clone(), value.clone())) {
        out.push((key, value));
    }
}

fn classes_from_meta(meta: &ExactPythonColumnMeta) -> Vec<String> {
    let mut classes = vec![format!("p1_{}", meta.column_number)];
    for direct in &meta.direct_matches {
        push_unique_string(&mut classes, format!("p2_{}", slugify_piece(&direct.parameter_main_name)));
        push_unique_string(&mut classes, format!("p3_{}", slugify_piece(&direct.parameter_name)));
        push_unique_string(
            &mut classes,
            format!(
                "p4_{}",
                direct
                    .column_numbers
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join("_")
            ),
        );
        for alias in &direct.parameter_main_aliases {
            push_unique_string(&mut classes, format!("p2alias_{}", slugify_piece(alias)));
        }
        for alias in &direct.parameter_aliases {
            push_unique_string(&mut classes, format!("p3alias_{}", slugify_piece(alias)));
        }
    }
    classes
}

pub fn html_decl_meta_for_column(words: &Words, column_number: i64) -> Option<HtmlDeclMeta> {
    let meta = exact_meta_for_column(words, column_number)?;
    let mut data_attributes = vec![("data-column".to_string(), column_number.to_string())];
    for direct in &meta.direct_matches {
        push_unique_attribute(
            &mut data_attributes,
            "data-main".to_string(),
            direct.parameter_main_name.clone(),
        );
        push_unique_attribute(
            &mut data_attributes,
            "data-parameter".to_string(),
            direct.parameter_name.clone(),
        );
        push_unique_attribute(
            &mut data_attributes,
            "data-main-aliases".to_string(),
            direct.parameter_main_aliases.join(","),
        );
        push_unique_attribute(
            &mut data_attributes,
            "data-parameter-aliases".to_string(),
            direct.parameter_aliases.join(","),
        );
        push_unique_attribute(
            &mut data_attributes,
            "data-column-group".to_string(),
            direct
                .column_numbers
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(","),
        );
    }
    Some(HtmlDeclMeta {
        column_number,
        classes: classes_from_meta(&meta),
        data_attributes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::words_py::Words;

    #[test]
    fn html_meta_has_p_classes() {
        let words = Words::new();
        let meta = html_decl_meta_for_column(&words, 5).expect("known column");
        assert!(meta.classes.iter().any(|value| value.starts_with("p1_")));
        assert!(meta.classes.iter().any(|value| value.starts_with("p2_")));
        assert!(meta.classes.iter().any(|value| value.starts_with("p3_")));
        assert!(meta.classes.iter().any(|value| value.starts_with("p4_")));
    }

    #[test]
    fn html_meta_keeps_alias_attributes() {
        let words = Words::new();
        let meta = html_decl_meta_for_column(&words, 5).expect("known column");
        assert!(meta.data_attributes.iter().any(|(key, _)| key == "data-main-aliases"));
        assert!(meta.data_attributes.iter().any(|(key, _)| key == "data-parameter-aliases"));
    }

    #[test]
    fn html_meta_has_alias_classes() {
        let words = Words::new();
        let meta = html_decl_meta_for_column(&words, 5).expect("known column");
        assert!(meta.classes.iter().any(|value| value.starts_with("p2alias_")));
        assert!(meta.classes.iter().any(|value| value.starts_with("p3alias_")));
    }
}
