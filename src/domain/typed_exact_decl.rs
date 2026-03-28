use crate::domain::decl_model::HtmlDeclMeta;

fn decl(p1: &[&str], p2: &[Option<&str>], p4: &[u8]) -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: p1.iter().map(|s| (*s).to_string()).collect(),
        p2_slots: p2.iter().map(|opt| opt.map(|s| s.to_string())).collect(),
        p4_tags: p4.to_vec(),
    }
}

pub const TYPED_EXACT_DECL_COLUMNS: &[u32] = &[6, 7, 8, 9, 28, 466];

pub fn is_typed_exact_decl_column(col: u32) -> bool {
    TYPED_EXACT_DECL_COLUMNS.contains(&col)
}

pub fn typed_exact_decl_for_column(col: u32) -> Option<HtmlDeclMeta> {
    Some(match col {
        6 => decl(&["Wichtigstes_zum_verstehen"], &[Some("Wichtigste")], &[0]),
        7 => decl(&["Menschliches", "Grundstrukturen"], &[Some("Gesellschaftsschicht"), Some("Klassen_(20)")], &[3,5,0]),
        8 => decl(&["Menschliches", "Grundstrukturen", "Eigenschaften_n"], &[Some("Liebe"), Some("Liebe_(7)")], &[3,0]),
        9 => decl(&["Planet_(10_und_oder_12)", "Menschliches", "Grundstrukturen"], &[Some("Gleichheit_Freiheit_Ordnung"), Some("Gleichheit_Freiheit"), Some("Ordnung_und_Filterung_12_und_1pro12")], &[4,5,0]),
        28 => decl(&["Religion"], &[Some("Religion")], &[3,0]),
        466 => decl(&["Universum"], &[Some("Geist__(15)")], &[3,0]),
        _ => return None,
    })
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    TYPED_EXACT_DECL_COLUMNS
        .iter()
        .filter_map(|&col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
