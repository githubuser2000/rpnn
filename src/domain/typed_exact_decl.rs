use crate::domain::decl_model::HtmlDeclMeta;

fn decl(p1: &[&str], p2: &[Option<&str>], p4: &[u8]) -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: p1.iter().map(|s| (*s).to_string()).collect(),
        p2_slots: p2.iter().map(|s| s.map(|v| v.to_string())).collect(),
        p4_tags: p4.to_vec(),
    }
}

pub fn typed_exact_decl_for_column(col: u32) -> Option<HtmlDeclMeta> {
    match col {
        7 => Some(decl(
            &["Religionen"],
            &[Some("Religion"), Some("Religions-Gründer-Typ"), None],
            &[3, 0],
        )),
        8 => Some(decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Gesellschaftsschicht"), Some("Klassen_(20)"), None],
            &[3, 5, 0],
        )),
        9 => Some(decl(
            &["Menschliches", "Grundstrukturen", "Eigenschaften_n"],
            &[Some("Liebe"), Some("Liebe_(7)"), None],
            &[3, 0],
        )),
        28 => Some(decl(
            &["Universum"],
            &[Some("Geist__(15)"), Some("Geist_(15)"), None],
            &[4, 0],
        )),
        466 => Some(decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Gewalt"), Some("Gewalt"), None],
            &[3, 4, 0],
        )),
        _ => None,
    }
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    [7u32, 8, 9, 28, 466]
        .into_iter()
        .filter_map(|col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
