use crate::domain::decl_model::HtmlDeclMeta;

/// Zentrale Liste der Spalten, die bereits über den typisierten Pfad laufen.
pub const TYPED_EXACT_DECL_COLUMNS: &[u32] = &[
    6, 7, 8, 9, 28, 241, 242, 243, 249, 303, 314, 324, 466,
];

pub fn is_typed_exact_decl_column(col: u32) -> bool {
    TYPED_EXACT_DECL_COLUMNS.contains(&col)
}

fn decl(p1: &[&str], p2: &[Option<&str>], p4: &[u8]) -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: p1.iter().map(|s| (*s).to_string()).collect(),
        p2_slots: p2.iter().map(|opt| opt.map(|s| s.to_string())).collect(),
        p4_tags: p4.to_vec(),
    }
}

pub fn typed_exact_decl_for_column(col: u32) -> Option<HtmlDeclMeta> {
    Some(match col {
        6 => decl(&["Religionen"], &[Some("Sternpolygon"), None], &[3, 0]),
        7 => decl(&["Religionen"], &[Some("Messias"), None], &[3, 0]),
        8 => decl(
            &["Wichtigstes_zum_verstehen", "Menschliches", "Grundstrukturen"],
            &[Some("Wichtigste"), Some("Liebe"), Some("Liebe_(7)"), None],
            &[0, 5],
        ),
        9 => decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Liebe"), Some("Liebe_(7)"), None],
            &[0, 5],
        ),
        28 => decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Liebe"), Some("Liebe_(7)"), None],
            &[0, 5],
        ),
        241 => decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Gesellschaftsschicht"), Some("Klassen_(20)"), None],
            &[3, 0, 5],
        ),
        242 => decl(
            &["Universum", "Grundstrukturen", "Grundstrukturen", "Multiversum"],
            &[
                Some("Geist__(15)"),
                Some("nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)"),
                Some("Geist_(15)"),
                Some("Geist_(15)"),
                None,
            ],
            &[4, 0],
        ),
        243 => decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Gefühle"), Some("Gefühle_(7)"), None],
            &[0, 5],
        ),
        249 => decl(
            &["Grundstrukturen", "Menschliches"],
            &[
                Some("Gedanken_sind_Positionen_(17)"),
                Some("Gedanken_sind_Positionen_(17)"),
                None,
            ],
            &[0, 5],
        ),
        303 => decl(&["Galaxie"], &[Some("Thomasevangelium"), None], &[3, 0]),
        314 => decl(&["Menschliches"], &[Some("Mensch-zu-Tier"), None], &[4, 0, 5]),
        324 => decl(
            &["Planet_(10_und_oder_12)"],
            &[Some("Gleichheit_Freiheit_Ordnung"), None],
            &[0, 5],
        ),
        466 => decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Gewalt"), Some("Gewalt"), None],
            &[3, 4, 0],
        ),
        _ => return None,
    })
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    TYPED_EXACT_DECL_COLUMNS
        .iter()
        .filter_map(|&col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
