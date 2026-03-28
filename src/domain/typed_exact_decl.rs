use crate::domain::decl_model::HtmlDeclMeta;

fn decl(p1: &[&str], p2: &[Option<&str>], p4: &[u8]) -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: p1.iter().map(|s| (*s).to_string()).collect(),
        p2_slots: p2.iter().map(|opt| opt.map(|s| s.to_string())).collect(),
        p4_tags: p4.to_vec(),
    }
}

pub const TYPED_EXACT_DECL_COLUMNS: &[u32] = &[
    0, 1, 2, 3, 4, 5,
    6, 7, 8, 9, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    28,
    241, 242, 243, 249, 303, 314, 324,
    466,
];

pub fn is_typed_exact_decl_column(col: u32) -> bool {
    TYPED_EXACT_DECL_COLUMNS.contains(&col)
}

pub fn typed_exact_decl_for_column(col: u32) -> Option<HtmlDeclMeta> {
    Some(match col {
        0 => decl(
            &["Wichtigstes_zum_gedanklich_einordnen", "Religionen", "Religionen", "Galaxie"],
            &[Some("Wichtigste"), Some("Sternpolygon"), Some("der_Tierkreiszeichen"), Some("Thomasevangelium"), None],
            &[3, 0],
        ),
        1 => decl(
            &["Wichtigstes_zum_gedanklich_einordnen", "Galaxie"],
            &[Some("Wichtigste"), Some("babylonische_Tierkreiszeichen"), None],
            &[3, 0],
        ),
        2 => decl(
            &["Wichtigstes_zum_gedanklich_einordnen", "Galaxie"],
            &[Some("Wichtigste"), Some("babylonische_Tierkreiszeichen"), None],
            &[3, 0],
        ),
        3 => decl(&["Galaxie"], &[Some("Thomasevangelium"), None], &[3, 0]),
        4 => decl(
            &["Wichtigstes_zum_verstehen", "Grundstrukturen", "Größenordnung", "Universum", "Inkrementieren"],
            &[
                Some("Wichtigste"),
                Some("Strukturgrösse"),
                Some("Strukturgrösse"),
                Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"),
                Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"),
                None,
            ],
            &[3, 4, 0, 5],
        ),
        5 => decl(
            &[
                "Wichtigstes_zum_verstehen", "Universum", "Universum", "Universum", "Grundstrukturen",
                "Multiversum", "Inkrementieren", "Inkrementieren", "Kontinuum",
            ],
            &[
                Some("Wichtigste"),
                Some("Transzendentalien"),
                Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"),
                Some("warum_Transzendentalie_=_Komplexität_von_Michael_Commons"),
                Some("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"),
                Some("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"),
                Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"),
                Some("warum_Transzendentalie_=_Komplexität_von_Michael_Commons"),
                Some("O"),
                None,
            ],
            &[4, 0],
        ),
        6 => decl(&["Religionen"], &[Some("Sternpolygon"), None], &[3, 0]),
        7 => decl(&["Religionen"], &[Some("Messias"), None], &[3, 0]),
        8 => decl(
            &["Wichtigstes_zum_verstehen", "Menschliches", "Grundstrukturen"],
            &[Some("Wichtigste"), Some("Liebe"), Some("Liebe_(7)"), None],
            &[0, 5],
        ),
        9 => decl(&["Menschliches", "Grundstrukturen"], &[Some("Liebe"), Some("Liebe_(7)"), None], &[0, 5]),
        10 => decl(
            &["Wichtigstes_zum_verstehen", "Grundstrukturen", "Menschliches"],
            &[Some("Wichtigste"), Some("Paradigmen_sind_Absichten_(13)"), Some("Motive"), None],
            &[3, 0],
        ),
        11 => decl(&["Menschliches"], &[Some("Errungenschaften"), None], &[3, 0]),
        12 => decl(&["Menschliches"], &[Some("evolutionär_erwerben_und_Intelligenz_Kreativität"), None], &[3, 0]),
        13 => decl(
            &["Menschliches", "Menschliches"],
            &[Some("evolutionär_erwerben_und_Intelligenz_Kreativität"), Some("brauchen"), None],
            &[3, 0],
        ),
        14 => decl(&["Menschliches"], &[Some("brauchen"), None], &[3, 0]),
        15 => decl(&["Pro_Contra"], &[Some("contra"), None], &[3, 4, 0, 5]),
        16 => decl(&["Religionen"], &[Some("gleichförmiges_Polygon"), None], &[3, 1]),
        17 => decl(&["Pro_Contra"], &[Some("Pro"), None], &[3, 4, 0, 5]),
        18 => decl(&["Menschliches"], &[Some("Motive"), None], &[3, 0]),
        19 => decl(&["Wichtigstes_zum_verstehen", "Bedeutung"], &[Some("Zweitwichtigste"), Some("Primzahlen"), None], &[3, 0]),
        20 => decl(&["Größenordnung", "Licht"], &[Some("Licht"), None, None], &[3, 4, 0, 5]),
        28 => decl(&["Religion"], &[Some("Religion")], &[3, 0]),
        241 => decl(&["Menschliches", "Grundstrukturen"], &[Some("Gesellschaftsschicht"), Some("Klassen_(20)"), None], &[3, 0, 5]),
        242 => decl(&["Universum", "Grundstrukturen", "Grundstrukturen", "Multiversum"], &[Some("Geist__(15)"), Some("nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)"), Some("Geist_(15)"), Some("Geist_(15)"), None], &[4, 0]),
        243 => decl(&["Menschliches", "Grundstrukturen"], &[Some("Gefühle"), Some("Gefühle_(7)"), None], &[0, 5]),
        249 => decl(&["Grundstrukturen", "Menschliches"], &[Some("Gedanken_sind_Positionen_(17)"), Some("Gedanken_sind_Positionen_(17)"), None], &[0, 5]),
        303 => decl(&["Galaxie"], &[Some("Thomasevangelium"), None], &[3, 0]),
        314 => decl(&["Menschliches"], &[Some("Mensch-zu-Tier"), None], &[4, 0, 5]),
        324 => decl(&["Planet_(10_und_oder_12)"], &[Some("Gleichheit_Freiheit_Ordnung"), None], &[0, 5]),
        466 => decl(&["Universum"], &[Some("Geist__(15)")], &[3, 0]),
        _ => return None,
    })
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    TYPED_EXACT_DECL_COLUMNS
        .iter()
        .filter_map(|&col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
