use crate::domain::python_source_of_truth::{exact_meta_for_column, PythonColumnMeta};
use crate::shared::words_py::Words;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HtmlDeclMeta {
    pub column_number: i64,
    pub classes: Vec<String>,
    pub data_attributes: Vec<(String, String)>,
}

fn slugify(value: &str) -> String {
    value.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => c.to_ascii_lowercase(),
            'ä' => 'a',
            'ö' => 'o',
            'ü' => 'u',
            'ß' => 's',
            _ => '-',
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn classes_from_meta(meta: &PythonColumnMeta) -> Vec<String> {
    vec![
        format!("p1-{}", slugify(&meta.ober)),
        format!("p2-{}", slugify(&meta.unter)),
        format!("p3-slot-{}", meta.data_slot),
        format!("p4-col-{}", meta.column_number),
    ]
}

pub fn exact_html_meta_for_column(words: &Words, column_number: i64) -> Vec<HtmlDeclMeta> {
    exact_meta_for_column(words, column_number)
        .into_iter()
        .map(|meta| HtmlDeclMeta {
            column_number,
            classes: classes_from_meta(&meta),
            data_attributes: vec![
                ("data-ober".to_string(), meta.ober),
                ("data-unter".to_string(), meta.unter),
                ("data-slot".to_string(), meta.data_slot.to_string()),
                ("data-column".to_string(), meta.column_number.to_string()),
            ],
        })
        .collect()
}
