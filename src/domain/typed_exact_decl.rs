use crate::domain::decl_model::HtmlDeclMeta;

fn decl(p1: &[&str], p2: &[Option<&str>], p4: &[u8]) -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: p1.iter().map(|s| (*s).to_string()).collect(),
        p2_slots: p2.iter().map(|opt| opt.map(|s| s.to_string())).collect(),
        p4_tags: p4.to_vec(),
    }
}

pub const TYPED_EXACT_DECL_COLUMNS: &[u32] = &[
    6, 7, 8, 9,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35, 36, 37,
    241, 242, 243, 249, 303, 314, 324,
    466,
];

pub fn is_typed_exact_decl_column(col: u32) -> bool {
    TYPED_EXACT_DECL_COLUMNS.contains(&col)
}

pub fn typed_exact_decl_for_column(col: u32) -> Option<HtmlDeclMeta> {
    Some(match col {
        6 => decl(&["Wichtigstes_zum_verstehen"], &[Some("Wichtigste")], &[0]),
        7 => decl(&["Menschliches", "Grundstrukturen"], &[Some("Gesellschaftsschicht"), Some("Klassen_(20)")], &[3, 5, 0]),
        8 => decl(&["Menschliches", "Grundstrukturen", "Eigenschaften_n"], &[Some("Liebe"), Some("Liebe_(7)")], &[3, 0]),
        9 => decl(&["Planet_(10_und_oder_12)", "Menschliches", "Grundstrukturen"], &[Some("Gleichheit_Freiheit_Ordnung"), Some("Gleichheit_Freiheit"), Some("Ordnung_und_Filterung_12_und_1pro12")], &[4, 5, 0]),
        21 => decl(&["Grundstrukturen", "Größenordnung"], &[Some("Strukturgrösse"), Some("Strukturgrösse"), None], &[3, 4, 0, 5]),
        22 => decl(&["Bedeutung"], &[Some("Anwendung_der_Sonnen_und_Monde"), None], &[3, 0]),
        23 => decl(&["Religionen"], &[Some("Vertreter_höherer_Konzepte"), None], &[3, 0]),
        24 => decl(&["Grundstrukturen", "Menschliches"], &[Some("gegen_5"), Some("Krankheit"), None], &[3, 0]),
        25 => decl(&["Universum", "Bedeutung", "Kontinuum"], &[Some("Netzwerk"), Some("Zählungen"), Some("X"), None], &[4, 0]),
        26 => decl(&["Pro_Contra"], &[Some("contra"), None], &[3, 4, 0, 5]),
        27 => decl(&["Größenordnung", "Menschliches", "Licht"], &[Some("Licht"), Some("evolutionär_erwerben_und_Intelligenz_Kreativität"), None, None], &[4, 0]),
        28 => decl(&["Religion"], &[Some("Religion")], &[3, 0]),
        29 => decl(&["Grundstrukturen", "Menschliches"], &[Some("Gefühle_(7)"), Some("Anführer"), None], &[3, 0]),
        30 => decl(&["Wichtigstes_zum_gedanklich_einordnen", "Größenordnung", "Menschliches"], &[Some("Zweitwichtigste"), Some("Organisationen"), Some("Berufe"), None], &[3, 0]),
        31 => decl(&["Menschliches"], &[Some("Lösungen"), None], &[3, 0]),
        32 => decl(&["Menschliches"], &[Some("evolutionär_erwerben_und_Intelligenz_Kreativität"), None], &[3, 0]),
        33 => decl(&["Grundstrukturen", "Menschliches"], &[Some("Stimmungen_Kombinationen_(14)"), Some("Musik"), None], &[3, 0]),
        34 => decl(&["Universum", "Bedeutung"], &[Some("universelles_Recht"), Some("Jura"), None], &[3, 0]),
        35 => decl(&["Bedeutung"], &[Some("Vollkommenheit_des_Geistes"), None], &[3, 0]),
        36 => decl(&["Wichtigstes_zum_gedanklich_einordnen", "Religionen", "Religionen", "Symbole"], &[Some("Wichtigste"), Some("Sternpolygon"), Some("der_Tierkreiszeichen"), Some("Religionen"), None], &[3, 4, 0, 5]),
        37 => decl(&["Wichtigstes_zum_gedanklich_einordnen", "Religionen", "Symbole"], &[Some("Wichtigste"), Some("gleichförmiges_Polygon"), Some("Religionen"), None], &[3, 5, 1, 4]),
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
