use crate::domain::html_meta_builder::{build_html_class, resolve_header_decl};

fn extract_col_marker(raw: &str) -> Option<u32> {
    let marker = "\u{1f}COL:";
    let start = raw.find(marker)?;
    raw[start + marker.len()..].trim().parse::<u32>().ok()
}

fn extract_id_suffix_1_based(raw: &str) -> Option<u32> {
    let id_pos = raw.rfind("(ID_")?;
    let rest = &raw[id_pos + 4..];
    let end = rest.find(')')?;
    rest[..end].parse::<u32>().ok()
}

fn strip_transport_markers(raw: &str) -> String {
    let mut s = raw.to_string();

    if let Some(pos) = s.find("\u{1f}COL:") {
        s.truncate(pos);
    }
    if let Some(pos) = s.find("\u{1f}IDX:") {
        s.truncate(pos);
    }

    s.trim().trim_matches('"').to_string()
}

fn strip_id_suffix(s: &str) -> String {
    if let Some(pos) = s.rfind("(ID_") {
        s[..pos].trim().to_string()
    } else {
        s.trim().to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedHeaderMeta {
    pub visible_text: String,
    pub class_attr: Option<String>,
}

pub fn resolve_header_meta(raw: &str, col_idx: usize, is_header_row: bool) -> ResolvedHeaderMeta {
    let visible_text = strip_transport_markers(raw);

    if !is_header_row {
        return ResolvedHeaderMeta {
            visible_text,
            class_attr: None,
        };
    }

    if col_idx == 0 {
        return ResolvedHeaderMeta {
            visible_text: String::new(),
            class_attr: Some("z_0 r_0 p1_✗Zählung,, p2_p3_0_, p4_".to_string()),
        };
    }

    if col_idx == 1 {
        return ResolvedHeaderMeta {
            visible_text: String::new(),
            class_attr: Some("z_0 r_1 p1_✗Nummerierung,, p2_p3_0_, p4_".to_string()),
        };
    }

    if let Some(col0) = extract_col_marker(raw) {
        if let Some(meta) = crate::domain::python_source_of_truth::exact_meta_for_column(col0) {
            return ResolvedHeaderMeta {
                visible_text,
                class_attr: Some(format!("z_0 r_{} {}", col_idx, meta)),
            };
        }
    }

    if let Some(id1) = extract_id_suffix_1_based(raw) {
        if let Some(col0) = id1.checked_sub(1) {
            if let Some(meta) = crate::domain::python_source_of_truth::exact_meta_for_column(col0) {
                let visible_without_id = strip_id_suffix(&visible_text);
                return ResolvedHeaderMeta {
                    visible_text: visible_without_id,
                    class_attr: Some(format!("z_0 r_{} {}", col_idx, meta)),
                };
            }
        }
    }

    ResolvedHeaderMeta {
        visible_text,
        class_attr: None,
    }
}
