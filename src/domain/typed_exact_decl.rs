use crate::domain::decl_model::HtmlDeclMeta;

/// Übergangspfad für die Migration weg von rohen Meta-Strings.
/// Aktuell noch leer, damit die Quelle typisiert angebunden ist,
/// auch wenn die eigentlichen Einträge schrittweise nachgezogen werden.
pub fn typed_exact_decl_for_column(_col: u32) -> Option<HtmlDeclMeta> {
    None
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    Vec::new()
}
