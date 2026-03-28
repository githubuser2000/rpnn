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
        0 => Some(decl(
            &["Wichtigstes_zum_gedanklich_einordnen", "Religionen", "Religionen", "Galaxie"],
            &[Some("Wichtigste"), Some("Sternpolygon"), Some("der_Tierkreiszeichen"), Some("Thomasevangelium"), None],
            &[3, 0],
        )),
        1 => Some(decl(
            &["Wichtigstes_zum_gedanklich_einordnen", "Galaxie"],
            &[Some("Wichtigste"), Some("babylonische_Tierkreiszeichen"), None],
            &[3, 0],
        )),
        2 => Some(decl(
            &["Wichtigstes_zum_gedanklich_einordnen", "Galaxie"],
            &[Some("Wichtigste"), Some("babylonische_Tierkreiszeichen"), None],
            &[3, 0],
        )),
        3 => Some(decl(
            &["Galaxie"],
            &[Some("Thomasevangelium"), None],
            &[3, 0],
        )),
        4 => Some(decl(
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
        )),
        5 => Some(decl(
            &[
                "Wichtigstes_zum_verstehen",
                "Universum",
                "Universum",
                "Universum",
                "Grundstrukturen",
                "Multiversum",
                "Inkrementieren",
                "Inkrementieren",
                "Kontinuum",
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
        )),
        6 => Some(decl(
            &["Religionen"],
            &[Some("Sternpolygon"), None],
            &[3, 0],
        )),
        7 => Some(decl(
            &["Religionen"],
            &[Some("Messias"), None],
            &[3, 0],
        )),
        8 => Some(decl(
            &["Wichtigstes_zum_verstehen", "Menschliches", "Grundstrukturen"],
            &[Some("Wichtigste"), Some("Liebe"), Some("Liebe_(7)"), None],
            &[0, 5],
        )),
        9 => Some(decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Liebe"), Some("Liebe_(7)"), None],
            &[0, 5],
        )),
        10 => Some(decl(
            &["Wichtigstes_zum_verstehen", "Grundstrukturen", "Menschliches"],
            &[Some("Wichtigste"), Some("Paradigmen_sind_Absichten_(13)"), Some("Motive"), None],
            &[3, 0],
        )),
        11 => Some(decl(
            &["Menschliches"],
            &[Some("Errungenschaften"), None],
            &[3, 0],
        )),
        12 => Some(decl(
            &["Menschliches"],
            &[Some("evolutionär_erwerben_und_Intelligenz_Kreativität"), None],
            &[3, 0],
        )),
        13 => Some(decl(
            &["Menschliches", "Menschliches"],
            &[Some("evolutionär_erwerben_und_Intelligenz_Kreativität"), Some("brauchen"), None],
            &[3, 0],
        )),
        14 => Some(decl(
            &["Menschliches"],
            &[Some("brauchen"), None],
            &[3, 0],
        )),
        15 => Some(decl(
            &["Pro_Contra"],
            &[Some("contra"), None],
            &[3, 4, 0, 5],
        )),
        16 => Some(decl(
            &["Religionen"],
            &[Some("gleichförmiges_Polygon"), None],
            &[3, 1],
        )),
        17 => Some(decl(
            &["Pro_Contra"],
            &[Some("Pro"), None],
            &[3, 4, 0, 5],
        )),
        18 => Some(decl(
            &["Menschliches"],
            &[Some("Motive"), None],
            &[3, 0],
        )),
        19 => Some(decl(
            &["Wichtigstes_zum_verstehen", "Bedeutung"],
            &[Some("Zweitwichtigste"), Some("Primzahlen"), None],
            &[3, 0],
        )),
        20 => Some(decl(
            &["Größenordnung", "Licht"],
            &[Some("Licht"), None, None],
            &[3, 4, 0, 5],
        )),
        21 => Some(decl(
            &["Grundstrukturen", "Größenordnung"],
            &[Some("Strukturgrösse"), Some("Strukturgrösse"), None],
            &[3, 4, 0, 5],
        )),
        22 => Some(decl(
            &["Bedeutung"],
            &[Some("Anwendung_der_Sonnen_und_Monde"), None],
            &[3, 0],
        )),
        23 => Some(decl(
            &["Religionen"],
            &[Some("Vertreter_höherer_Konzepte"), None],
            &[3, 0],
        )),
        24 => Some(decl(
            &["Grundstrukturen", "Menschliches"],
            &[Some("gegen_5"), Some("Krankheit"), None],
            &[3, 0],
        )),
        25 => Some(decl(
            &["Universum", "Bedeutung", "Kontinuum"],
            &[Some("Netzwerk"), Some("Zählungen"), Some("X"), None],
            &[4, 0],
        )),
        26 => Some(decl(
            &["Pro_Contra"],
            &[Some("contra"), None],
            &[3, 4, 0, 5],
        )),
        27 => Some(decl(
            &["Größenordnung", "Menschliches", "Licht"],
            &[Some("Licht"), Some("evolutionär_erwerben_und_Intelligenz_Kreativität"), None, None],
            &[4, 0],
        )),
        28 => Some(decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Liebe"), Some("Liebe_(7)"), None],
            &[0, 5],
        )),
        29 => Some(decl(
            &["Grundstrukturen", "Menschliches"],
            &[Some("Gefühle_(7)"), Some("Anführer"), None],
            &[3, 0],
        )),
        30 => Some(decl(
            &["Wichtigstes_zum_gedanklich_einordnen", "Größenordnung", "Menschliches"],
            &[Some("Zweitwichtigste"), Some("Organisationen"), Some("Berufe"), None],
            &[3, 0],
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
    let cols: &[u32] = &[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
        466,
    ];
    cols.iter()
        .copied()
        .filter_map(|col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
