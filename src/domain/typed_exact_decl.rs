use crate::domain::decl_model::HtmlDeclMeta;

/// Zentrale Liste der Spalten, die bereits über den typisierten Pfad laufen.
pub const TYPED_EXACT_DECL_COLUMNS: &[u32] = &[];

pub fn is_typed_exact_decl_column(col: u32) -> bool {
    TYPED_EXACT_DECL_COLUMNS.contains(&col)
}

/// Übergangspfad für die Migration weg von rohen Meta-Strings.
/// Solange die Liste oben leer ist, fällt das System noch vollständig auf
/// den Legacy-Pfad zurück.
pub fn typed_exact_decl_for_column(_col: u32) -> Option<HtmlDeclMeta> {
    None
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    TYPED_EXACT_DECL_COLUMNS
        .iter()
        .filter_map(|&col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
