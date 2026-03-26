use crate::domain::html_meta_builder::{build_html_class, resolve_header_decl};

#[derive(Debug, Clone)]
pub struct ResolvedHeaderMeta {
    pub visible_text: String,
    pub class_attr: Option<String>,
}

pub fn resolve_header_meta(full: &str, col_idx: usize, is_header_row: bool) -> ResolvedHeaderMeta {
    let decl = resolve_header_decl(full, col_idx, is_header_row);
    let class_attr = build_html_class(col_idx, &decl);

    ResolvedHeaderMeta {
        visible_text: decl.visible_text,
        class_attr,
    }
}
