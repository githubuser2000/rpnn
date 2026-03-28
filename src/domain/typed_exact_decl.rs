use crate::domain::decl_model::HtmlDeclMeta;

fn decl(p1: &[&str], p2: &[Option<&str>], p4: &[u8]) -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: p1.iter().map(|s| (*s).to_string()).collect(),
        p2_slots: p2.iter().map(|opt| opt.map(|s| s.to_string())).collect(),
        p4_tags: p4.to_vec(),
    }
}

pub const TYPED_EXACT_DECL_COLUMNS: &[u32] = &[
    6, 7, 8, 9, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32,
    33, 34, 35, 36, 37, 42, 43, 45,
    46, 47, 48, 51, 52, 53, 54, 55,
    57, 58, 59, 64, 65, 68, 69, 70,
    71, 72, 73, 74, 75, 76, 77, 78,
    79, 80, 81, 82, 83, 84, 86, 87,
    88, 89, 90, 91, 92, 93, 94, 95,
    96, 99, 100, 241, 242, 243, 249, 303,
    314, 324, 466,
 ];

pub fn is_typed_exact_decl_column(col: u32) -> bool {
    TYPED_EXACT_DECL_COLUMNS.contains(&col)
}

pub fn typed_exact_decl_for_column(col: u32) -> Option<HtmlDeclMeta> {
    Some(match col {
        6 => decl(&["Religionen"], &[Some("Sternpolygon"), None], &[3, 0]),
        7 => decl(&["Religionen"], &[Some("Messias"), None], &[3, 0]),
        8 => decl(&["Wichtigstes_zum_verstehen", "Menschliches", "Grundstrukturen"], &[Some("Wichtigste"), Some("Liebe"), Some("Liebe_(7)"), None], &[0, 5]),
        9 => decl(&["Menschliches", "Grundstrukturen"], &[Some("Liebe"), Some("Liebe_(7)"), None], &[0, 5]),
        21 => decl(&["Grundstrukturen", "Größenordnung"], &[Some("Strukturgrösse"), Some("Strukturgrösse"), None], &[3, 4, 0, 5]),
        22 => decl(&["Bedeutung"], &[Some("Anwendung_der_Sonnen_und_Monde"), None], &[3, 0]),
        23 => decl(&["Religionen"], &[Some("Vertreter_höherer_Konzepte"), None], &[3, 0]),
        24 => decl(&["Grundstrukturen", "Menschliches"], &[Some("gegen_5"), Some("Krankheit"), None], &[3, 0]),
        25 => decl(&["Universum", "Bedeutung", "Kontinuum"], &[Some("Netzwerk"), Some("Zählungen"), Some("X"), None], &[4, 0]),
        26 => decl(&["Pro_Contra"], &[Some("contra"), None], &[3, 4, 0, 5]),
        27 => decl(&["Größenordnung", "Menschliches", "Licht"], &[Some("Licht"), Some("evolutionär_erwerben_und_Intelligenz_Kreativität"), None, None], &[4, 0]),
        28 => decl(&["Menschliches", "Grundstrukturen"], &[Some("Liebe"), Some("Liebe_(7)"), None], &[0, 5]),
        29 => decl(&["Grundstrukturen", "Menschliches"], &[Some("Gefühle_(7)"), Some("Anführer"), None], &[3, 0]),
        30 => decl(&["Wichtigstes_zum_gedanklich_einordnen", "Größenordnung", "Menschliches"], &[Some("Zweitwichtigste"), Some("Organisationen"), Some("Berufe"), None], &[3, 0]),
        31 => decl(&["Menschliches"], &[Some("Lösungen"), None], &[3, 0]),
        32 => decl(&["Menschliches"], &[Some("evolutionär_erwerben_und_Intelligenz_Kreativität"), None], &[3, 0]),
        33 => decl(&["Grundstrukturen", "Menschliches"], &[Some("Stimmungen_Kombinationen_(14)"), Some("Musik"), None], &[3, 0]),
        34 => decl(&["Universum", "Bedeutung"], &[Some("universelles_Recht"), Some("Jura"), None], &[3, 0]),
        35 => decl(&["Bedeutung"], &[Some("Vollkommenheit_des_Geistes"), None], &[3, 0]),
        36 => decl(&["Wichtigstes_zum_gedanklich_einordnen", "Religionen", "Religionen", "Symbole"], &[Some("Wichtigste"), Some("Sternpolygon"), Some("der_Tierkreiszeichen"), Some("Religionen"), None], &[3, 4, 0, 5]),
        37 => decl(&["Wichtigstes_zum_gedanklich_einordnen", "Religionen", "Symbole"], &[Some("Wichtigste"), Some("gleichförmiges_Polygon"), Some("Religionen"), None], &[3, 5, 1, 4]),
        42 => decl(&["Grundstrukturen", "Grundstrukturen", "Menschliches"], &[Some("Reziprokes"), Some("Paradigmen_sind_Absichten_(13)"), Some("Motive"), None], &[3, 1]),
        43 => decl(&["Inkrementieren"], &[None, None], &[3, 0]),
        45 => decl(&["Bedeutung"], &[Some("Zählungen"), None], &[3, 0]),
        46 => decl(&["Menschliches"], &[Some("alpha_beta"), None], &[3, 0]),
        47 => decl(&["Menschliches"], &[Some("evolutionär_erwerben_und_Intelligenz_Kreativität"), None], &[3, 0]),
        48 => decl(&["Pro_Contra"], &[Some("Pro"), None], &[3, 4, 0, 5]),
        51 => decl(&["Menschliches"], &[Some("dominierendes_Geschlecht"), None], &[3, 0]),
        52 => decl(&["Eigenschaften_n"], &[Some("gut_böse_lieb_schlecht"), None], &[3, 1, 0]),
        53 => decl(&["Eigenschaften_n"], &[Some("gut_böse_lieb_schlecht"), None], &[3, 1, 0]),
        54 => decl(&["Grundstrukturen", "Größenordnung", "Universum", "Universum", "Inkrementieren", "Inkrementieren"], &[Some("Strukturgrösse"), Some("Strukturgrösse"), Some("Transzendentalien"), Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"), None, Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"), None], &[3, 4, 0]),
        55 => decl(&["Universum", "Universum", "Kontinuum"], &[Some("Transzendentalien"), Some("Netzwerk"), Some("X"), None], &[4, 0]),
        57 => decl(&["Menschliches"], &[Some("Angreifbarkeit"), None], &[3, 0]),
        58 => decl(&["Menschliches"], &[Some("Angreifbarkeit"), None], &[3, 1]),
        59 => decl(&["Menschliches"], &[Some("Glaube_Erkenntnis"), None], &[3, 0]),
        64 => decl(&["Wichtigstes_zum_verstehen", "Bedeutung"], &[Some("Drittwichtigste"), Some("Gestirn"), None], &[3, 0]),
        65 => decl(&["Wichtigstes_zum_verstehen", "Universum", "Universum", "Grundstrukturen", "Universum", "Multiversum", "Inkrementieren"], &[Some("Zweitwichtigste"), Some("universelles_Recht"), Some("warum_Transzendentalie_=_Komplexität_von_Michael_Commons"), Some("Model_of_Hierarchical_Complexity"), Some("Model_of_Hierarchical_Complexity"), Some("Model_of_Hierarchical_Complexity"), Some("warum_Transzendentalie_=_Komplexität_von_Michael_Commons"), None], &[4, 0]),
        68 => decl(&["Menschliches"], &[Some("INCELs"), None], &[3, 0]),
        69 => decl(&["Grundstrukturen", "Wirtschaft"], &[Some("System"), Some("System"), None], &[4, 0]),
        70 => decl(&["Grundstrukturen", "Wirtschaft"], &[Some("System"), Some("System"), None], &[4, 0]),
        71 => decl(&["Wirtschaft"], &[Some("Erklärung"), None], &[3, 0]),
        72 => decl(&["Religionen"], &[Some("Religions-Gründer-Typ"), None], &[3, 0]),
        73 => decl(&["Menschliches"], &[Some("irrationale_Zahlen_durch_Wurzelbildung"), None], &[3, 0]),
        74 => decl(&["Inkrementieren"], &[None, None], &[3, 0]),
        75 => decl(&["Grundstrukturen", "Universum", "Multiversum"], &[Some("Model_of_Hierarchical_Complexity"), Some("Model_of_Hierarchical_Complexity"), Some("Model_of_Hierarchical_Complexity"), None], &[3, 4, 0]),
        76 => decl(&["Operationen"], &[Some("4"), None], &[3, 4, 0]),
        77 => decl(&["Universum", "Galaxie", "Operationen"], &[Some("Kugeln_Kreise"), Some("Kugeln_Kreise"), Some("4"), None], &[4, 0]),
        78 => decl(&["Operationen"], &[Some("2"), None], &[3, 0]),
        79 => decl(&["Planet_(10_und_oder_12)", "Planet_(10_und_oder_12)", "Operationen", "Grundstrukturen"], &[Some("Meta-Systeme_(12)"), Some("Gleichheit_Freiheit_Ordnung"), Some("2"), Some("Meta-Systeme_(12)"), None], &[3, 0]),
        80 => decl(&["Planet_(10_und_oder_12)", "Planet_(10_und_oder_12)", "Operationen", "Grundstrukturen"], &[Some("Meta-Systeme_(12)"), Some("Gleichheit_Freiheit_Ordnung"), Some("2"), Some("Meta-Systeme_(12)"), None], &[4, 0]),
        81 => decl(&["Operationen"], &[Some("4"), None], &[4, 0]),
        82 => decl(&["Größenordnung"], &[Some("Organisationen"), None], &[3, 0]),
        83 => decl(&["Größenordnung"], &[Some("politische_Systeme"), None], &[3, 0]),
        84 => decl(&["Grundstrukturen", "Galaxie", "Universum"], &[Some("analytische_Ontologie"), Some("analytische_Ontologie"), Some("analytische_Ontologie"), None], &[4, 0]),
        86 => decl(&["Operationen"], &[Some("Halbierung"), None], &[3, 0]),
        87 => decl(&["Religionen"], &[Some("Sternpolygon_vs_gleichförmiges"), None], &[3, 1, 0]),
        88 => decl(&["Menschliches", "Eigenschaften_n"], &[Some("Sinn_des_Lebens"), Some("Sinn_und_Zweck_des_Lebens"), None], &[3, 0]),
        89 => decl(&["Wirtschaft"], &[Some("Maschinen"), None], &[3, 0]),
        90 => decl(&["Galaxie"], &[Some("Offenbarung_des_Johannes"), None], &[3, 0]),
        91 => decl(&["Galaxie"], &[Some("chinesisches_Horoskop"), None], &[3, 0]),
        92 => decl(&["Operationen"], &[Some("3"), None], &[3, 0]),
        93 => decl(&["Operationen"], &[Some("3"), None], &[4, 0]),
        94 => decl(&["Operationen"], &[Some("9"), None], &[4, 0]),
        95 => decl(&["Inkrementieren"], &[None, None], &[3, 0]),
        96 => decl(&["Operationen"], &[Some("5"), None], &[3, 0]),
        99 => decl(&["Wirtschaft"], &[Some("Organisationsform"), None], &[3, 0]),
        100 => decl(&["Pro_Contra"], &[Some("Gegenteil"), None], &[3, 4, 0, 5]),
        241 => decl(&["Menschliches", "Grundstrukturen"], &[Some("Gesellschaftsschicht"), Some("Klassen_(20)"), None], &[3, 0, 5]),
        242 => decl(&["Universum", "Grundstrukturen", "Grundstrukturen", "Multiversum"], &[Some("Geist__(15)"), Some("nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)"), Some("Geist_(15)"), Some("Geist_(15)"), None], &[4, 0]),
        243 => decl(&["Menschliches", "Grundstrukturen"], &[Some("Gefühle"), Some("Gefühle_(7)"), None], &[0, 5]),
        249 => decl(&["Grundstrukturen", "Menschliches"], &[Some("Gedanken_sind_Positionen_(17)"), Some("Gedanken_sind_Positionen_(17)"), None], &[0, 5]),
        303 => decl(&["Galaxie"], &[Some("Thomasevangelium"), None], &[3, 0]),
        314 => decl(&["Menschliches"], &[Some("Mensch-zu-Tier"), None], &[4, 0, 5]),
        324 => decl(&["Planet_(10_und_oder_12)"], &[Some("Gleichheit_Freiheit_Ordnung"), None], &[0, 5]),
        466 => decl(&["Menschliches", "Grundstrukturen"], &[Some("Gewalt"), Some("Gewalt"), None], &[3, 4, 0]),
        _ => return None,
    })
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    TYPED_EXACT_DECL_COLUMNS
        .iter()
        .filter_map(|&col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
