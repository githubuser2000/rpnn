
use crate::domain::html_meta_builder::resolve_html_header_class;

pub struct ResolvedHeaderMeta {
    pub visible_text: String,
    pub class_attr: Option<String>,
}

fn strip_meta_markers(mut s: String) -> String {
    loop {
        let Some(start) = s.find("[[") else { break; };
        let Some(rel_end) = s[start..].find("]]") else { break; };
        let end = start + rel_end + 2;
        s.replace_range(start..end, "");
    }

    if let Some(pos) = s.find('\u{1f}') {
        s.truncate(pos);
    }

    s.trim().to_string()
}

fn strip_id_suffix(s: &str) -> String {
    let cleaned = strip_meta_markers(s.to_string());
    if let Some(pos) = cleaned.rfind("(ID_") {
        cleaned[..pos].trim().to_string()
    } else {
        cleaned.trim().to_string()
    }
}

pub fn resolve_header_meta(raw: &str, col_idx: usize, is_header_row: bool) -> ResolvedHeaderMeta {
    let visible_text = strip_meta_markers(raw.trim().trim_matches('"').to_string());

    if !is_header_row {
        return ResolvedHeaderMeta {
            visible_text,
            class_attr: None,
        };
    }

    if let Some(resolved_class) = resolve_html_header_class(raw, col_idx, is_header_row) {
        return ResolvedHeaderMeta {
            visible_text: strip_id_suffix(&visible_text),
            class_attr: Some(resolved_class.class_attr.to_string()),
        };
    }

    ResolvedHeaderMeta {
        visible_text,
        class_attr: None,
    }
}
