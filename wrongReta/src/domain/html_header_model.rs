
use crate::domain::html_meta_builder::build_python_exact_html_class;

pub struct ResolvedHeaderMeta {
    pub visible_text: String,
    pub class_attr: Option<String>,
}

fn strip_id_suffix(s: &str) -> String {
    if let Some(pos) = s.rfind("(ID_") {
        s[..pos].trim().to_string()
    } else {
        s.trim().to_string()
    }
}

pub fn resolve_header_meta(raw: &str, col_idx: usize, is_header_row: bool) -> ResolvedHeaderMeta {
    let visible_text = {
        let mut s = raw.to_string();

        if let Some(pos) = s.find('\u{1f}') {
            s.truncate(pos);
        }

        s.trim().trim_matches('"').to_string()
    };

    if !is_header_row {
        return ResolvedHeaderMeta {
            visible_text,
            class_attr: None,
        };
    }

    if let Some(class_attr) = build_python_exact_html_class(raw, col_idx, is_header_row) {
        return ResolvedHeaderMeta {
            visible_text: strip_id_suffix(&visible_text),
            class_attr: Some(class_attr),
        };
    }

    ResolvedHeaderMeta {
        visible_text,
        class_attr: None,
    }
}
