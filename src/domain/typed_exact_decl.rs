use crate::domain::decl_model::HtmlDeclMeta;

fn parse_exact(raw: &str) -> HtmlDeclMeta {
    HtmlDeclMeta::parse(raw).expect("typed exact decl literal must be valid")
}

pub fn typed_exact_decl_for_column(col: u32) -> Option<HtmlDeclMeta> {
    match col {
        7 => Some(parse_exact("p1_✗Religionen,, p2_p3_0_Messias,p3_1_, p4_3,0")),
        8 => Some(parse_exact("p1_✗Wichtigstes_zum_verstehen,✗Menschliches,✗Grundstrukturen,, p2_p3_0_Wichtigste,p3_1_Liebe,p3_2_Liebe_(7),p3_3_, p4_0,5")),
        9 => Some(parse_exact("p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Liebe,p3_1_Liebe_(7),p3_2_, p4_0,5")),
        28 => Some(parse_exact("p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Liebe,p3_1_Liebe_(7),p3_2_, p4_0,5")),
        466 => Some(parse_exact("p1_✗Eigenschaften_1/n,, p2_p3_0_Toleranz_Respekt_Akzeptanz_Willkommen, p4_3,0")),
        _ => None,
    }
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    [7u32, 8, 9, 28, 466]
        .into_iter()
        .filter_map(|col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
