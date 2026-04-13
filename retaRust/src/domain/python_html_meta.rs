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

fn classes_from_meta(meta: &ExactPythonColumnMeta) -> Vec<String> {
    let mut classes = vec![format!("p1_{}", meta.column_number)];
    for direct in &meta.direct_matches {
        let p2 = format!("p2_{}", slugify_piece(&direct.parameter_main_name));
        let p3 = format!("p3_{}", slugify_piece(&direct.parameter_name));
        let p4 = format!(
            "p4_{}",
            direct
                .column_numbers
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join("_")
        );
        for value in [p2, p3, p4] {
            if !classes.contains(&value) {
                classes.push(value);
            }
        }
    }
    classes
}

pub fn html_decl_meta_for_column(words: &Words, column_number: i64) -> Option<HtmlDeclMeta> {
    let meta = exact_meta_for_column(words, column_number)?;
    let mut data_attributes = vec![("data-column".to_string(), column_number.to_string())];
    if let Some(first) = meta.direct_matches.first() {
        data_attributes.push((
            "data-main".to_string(),
            first.parameter_main_name.clone(),
        ));
        data_attributes.push((
            "data-parameter".to_string(),
            first.parameter_name.clone(),
        ));
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
}
