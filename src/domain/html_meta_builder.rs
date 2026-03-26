use crate::domain::decl_model::HtmlDeclMeta;
use crate::domain::{python_html_meta, python_source_of_truth};

#[derive(Debug, Clone)]
pub struct HeaderDecl {
    pub visible_text: String,
    pub col0: Option<u32>,
    pub id1: Option<u32>,
    pub meta: Option<HtmlDeclMeta>,
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

fn first_two_decl(col_idx: usize) -> Option<HeaderDecl> {
    let visible_text = String::new();
    let meta = match col_idx {
        0 => HtmlDeclMeta::parse("p1_✗Zählung,, p2_p3_0_, p4_"),
        1 => HtmlDeclMeta::parse("p1_✗Nummerierung,, p2_p3_0_, p4_"),
        _ => None,
    };
    meta.map(|meta| HeaderDecl {
        visible_text,
        col0: None,
        id1: None,
        meta: Some(meta),
    })
}

fn lookup_exact_meta(col0: Option<u32>, id1: Option<u32>) -> Option<HtmlDeclMeta> {
    if let Some(col) = col0 {
        if let Some(meta) = python_source_of_truth::exact_meta_for_column(col) {
            if let Some(parsed) = HtmlDeclMeta::parse(&meta) {
                return Some(parsed);
            }
        }
    }

    if let Some(id1) = id1 {
        if let Some(col0) = id1.checked_sub(1) {
            if let Some(meta) = python_source_of_truth::exact_meta_for_column(col0) {
                if let Some(parsed) = HtmlDeclMeta::parse(&meta) {
                    return Some(parsed);
                }
            }
        }
    }

    None
}

fn lookup_full_header_meta(visible: &str) -> Option<HtmlDeclMeta> {
    python_html_meta::lookup_header_meta(visible).and_then(HtmlDeclMeta::parse)
}

fn choose_meta(exact: Option<HtmlDeclMeta>, full: Option<HtmlDeclMeta>) -> Option<HtmlDeclMeta> {
    match (exact, full) {
        (None, None) => None,
        (Some(exact), None) => Some(exact),
        (None, Some(full)) => Some(full),
        (Some(exact), Some(full)) => {
            // Prefer the richer declaration: more p2 slots wins. On ties, prefer the full-header map,
            // because it preserves the exact visible-header layout seen in the Python HTML output.
            if full.p2_slots.len() >= exact.p2_slots.len() {
                Some(full)
            } else {
                Some(exact)
            }
        }
    }
}

pub fn resolve_header_decl(full: &str, col_idx: usize, is_header_row: bool) -> HeaderDecl {
    let visible = canonical_visible_text(full);
    let (_, col0, _) = strip_all_markers(full);
    let (_, id1) = strip_trailing_id(full);

    if !is_header_row {
        return HeaderDecl {
            visible_text: visible,
            col0,
            id1,
            meta: None,
        };
    }

    if let Some(first) = first_two_decl(col_idx) {
        return HeaderDecl {
            visible_text: first.visible_text,
            col0,
            id1,
            meta: first.meta,
        };
    }

    let exact = lookup_exact_meta(col0, id1);
    let full_meta = lookup_full_header_meta(&visible);
    let meta = choose_meta(exact, full_meta);

    HeaderDecl {
        visible_text: visible,
        col0,
        id1,
        meta,
    }
}

pub fn build_html_class(col_idx: usize, decl: &HeaderDecl) -> Option<String> {
    decl.meta
        .as_ref()
        .map(|meta| format!("z_0 r_{} {}", col_idx, meta.render()))
}
