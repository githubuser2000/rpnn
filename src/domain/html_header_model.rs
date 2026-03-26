use crate::domain::{python_html_meta, python_source_of_truth};

#[derive(Debug, Clone)]
pub struct ResolvedHeaderMeta {
    pub visible_text: String,
    pub class_meta: Option<String>,
}

fn strip_control_suffix<'a>(text: &'a str, marker: &str) -> (&'a str, Option<&'a str>) {
    if let Some(pos) = text.find(marker) {
        let before = &text[..pos];
        let after = &text[pos + marker.len()..];
        (before, Some(after.trim()))
    } else {
        (text, None)
    }
}

fn parse_u32(raw: Option<&str>) -> Option<u32> {
    raw.and_then(|s| s.trim().parse::<u32>().ok())
}

fn strip_all_markers(text: &str) -> (&str, Option<u32>, Option<u32>) {
    let (without_col, col_raw) = strip_control_suffix(text, "\u{1f}COL:");
    let (without_idx, idx_raw) = strip_control_suffix(without_col, "\u{1f}IDX:");
    (without_idx, parse_u32(col_raw), parse_u32(idx_raw))
}

fn strip_trailing_id(mut text: &str) -> (&str, Option<u32>) {
    let mut found = None;
    loop {
        let trimmed = text.trim_end();
        if let Some(pos) = trimmed.rfind("(ID_") {
            if trimmed.ends_with(')') {
                let inside = &trimmed[pos + 4..trimmed.len() - 1];
                if let Ok(id) = inside.parse::<u32>() {
                    found = Some(id);
                    text = trimmed[..pos].trim_end();
                    continue;
                }
            }
        }
        return (trimmed, found);
    }
}

fn canonical_visible_text(text: &str) -> String {
    let (without_markers, _, _) = strip_all_markers(text);
    let (without_id, _) = strip_trailing_id(without_markers);
    without_id.trim().trim_matches('"').to_string()
}

fn first_two_meta(col_idx: usize) -> Option<String> {
    match col_idx {
        0 => Some("p1_✗Zählung,, p2_p3_0_, p4_".to_string()),
        1 => Some("p1_✗Nummerierung,, p2_p3_0_, p4_".to_string()),
        _ => None,
    }
}

fn meta_from_python_truth(col_0: Option<u32>, id_1: Option<u32>, visible: &str) -> Option<String> {
    if let Some(col) = col_0 {
        if let Some(meta) = python_source_of_truth::exact_meta_for_column(col) {
            return Some(meta);
        }
    }

    if let Some(id1) = id_1 {
        if let Some(col0) = id1.checked_sub(1) {
            if let Some(meta) = python_source_of_truth::exact_meta_for_column(col0) {
                return Some(meta);
            }
        }
    }

    match python_html_meta::lookup_header_meta(visible) {
        Some(meta) => Some(meta.to_string()),
        None => None,
    }
}

pub fn resolve_header_meta(full: &str, col_idx: usize, is_header_row: bool) -> ResolvedHeaderMeta {
    let visible = canonical_visible_text(full);
    if !is_header_row {
        return ResolvedHeaderMeta {
            visible_text: visible,
            class_meta: None,
        };
    }

    let (_, col_raw, _) = strip_all_markers(full);
    let (_, id_suffix) = strip_trailing_id(full);

    let class_meta = first_two_meta(col_idx)
        .or_else(|| meta_from_python_truth(col_raw, id_suffix, &visible));

    ResolvedHeaderMeta {
        visible_text: visible,
        class_meta,
    }
}
