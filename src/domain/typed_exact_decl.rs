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
        28 => Some(decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Liebe"), Some("Liebe_(7)"), None],
            &[0, 5],
        )),
        31 => Some(decl(&["Menschliches"], &[Some("Lösungen"), None], &[3, 0])),
        32 => Some(decl(
            &["Menschliches"],
            &[Some("evolutionär_erwerben_und_Intelligenz_Kreativität"), None],
            &[3, 0],
        )),
        33 => Some(decl(
            &["Grundstrukturen", "Menschliches"],
            &[Some("Stimmungen_Kombinationen_(14)"), Some("Musik"), None],
            &[3, 0],
        )),
        34 => Some(decl(
            &["Universum", "Bedeutung"],
            &[Some("universelles_Recht"), Some("Jura"), None],
            &[3, 0],
        )),
        35 => Some(decl(
            &["Bedeutung"],
            &[Some("Vollkommenheit_des_Geistes"), None],
            &[3, 0],
        )),
        36 => Some(decl(
            &["Wichtigstes_zum_gedanklich_einordnen", "Religionen", "Religionen", "Symbole"],
            &[Some("Wichtigste"), Some("Sternpolygon"), Some("der_Tierkreiszeichen"), Some("Religionen"), None],
            &[3, 4, 0, 5],
        )),
        37 => Some(decl(
            &["Wichtigstes_zum_gedanklich_einordnen", "Religionen", "Symbole"],
            &[Some("Wichtigste"), Some("gleichförmiges_Polygon"), Some("Religionen"), None],
            &[3, 5, 1, 4],
        )),
        42 => Some(decl(
            &["Grundstrukturen", "Grundstrukturen", "Menschliches"],
            &[Some("Reziprokes"), Some("Paradigmen_sind_Absichten_(13)"), Some("Motive"), None],
            &[3, 1],
        )),
        43 => Some(decl(&["Inkrementieren"], &[None, None], &[3, 0])),
        45 => Some(decl(&["Bedeutung"], &[Some("Zählungen"), None], &[3, 0])),
        46 => Some(decl(&["Menschliches"], &[Some("alpha_beta"), None], &[3, 0])),
        47 => Some(decl(
            &["Menschliches"],
            &[Some("evolutionär_erwerben_und_Intelligenz_Kreativität"), None],
            &[3, 0],
        )),
        48 => Some(decl(&["Pro_Contra"], &[Some("Pro"), None], &[3, 4, 0, 5])),
        51 => Some(decl(
            &["Menschliches"],
            &[Some("dominierendes_Geschlecht"), None],
            &[3, 0],
        )),
        52 => Some(decl(
            &["Eigenschaften_n"],
            &[Some("gut_böse_lieb_schlecht"), None],
            &[3, 1, 0],
        )),
        53 => Some(decl(
            &["Eigenschaften_n"],
            &[Some("gut_böse_lieb_schlecht"), None],
            &[3, 1, 0],
        )),
        54 => Some(decl(
            &["Grundstrukturen", "Größenordnung", "Universum", "Universum", "Inkrementieren", "Inkrementieren"],
            &[
                Some("Strukturgrösse"),
                Some("Strukturgrösse"),
                Some("Transzendentalien"),
                Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"),
                None,
                Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"),
                None,
            ],
            &[3, 4, 0],
        )),
        55 => Some(decl(
            &["Universum", "Universum", "Kontinuum"],
            &[Some("Transzendentalien"), Some("Netzwerk"), Some("X"), None],
            &[4, 0],
        )),
        57 => Some(decl(&["Menschliches"], &[Some("Angreifbarkeit"), None], &[3, 0])),
        58 => Some(decl(&["Menschliches"], &[Some("Angreifbarkeit"), None], &[3, 1])),
        59 => Some(decl(
            &["Menschliches"],
            &[Some("Glaube_Erkenntnis"), None],
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
    [
        7u32, 8, 9, 28, 31, 32, 33, 34, 35, 36, 37, 42, 43, 45, 46, 47, 48, 51, 52, 53, 54,
        55, 57, 58, 59, 466,
    ]
    .into_iter()
    .filter_map(|col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
    .collect()
}
