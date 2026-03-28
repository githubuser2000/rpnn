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
        466 => Some(decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Gewalt"), Some("Gewalt"), None],
            &[3, 4, 0],
        )),
        101 => Some(decl(&["Pro_Contra"], &[Some("Gegenteil"), None], &[3, 4, 0, 5])),
        102 => Some(decl(&["Pro_Contra"], &[Some("Harmonie"), None], &[3, 4, 0, 5])),
        103 => Some(decl(&["Pro_Contra"], &[Some("Harmonie"), None], &[3, 4, 0, 5])),
        104 => Some(decl(&["Operationen"], &[Some("4"), None], &[4, 0])),
        105 => Some(decl(&["Menschliches"], &[Some("Gefühle"), None], &[3, 0])),
        106 => Some(decl(&["Bedeutung"], &[Some("Konjunktiv_Wurzelbildung"), None], &[3, 0])),
        107 => Some(decl(&["Planet_(10_und_oder_12)", "Bedeutung"], &[Some("Mechanismen"), Some("Mechanismen_der_Züchtung"), None], &[4, 0, 5])),
        108 => Some(decl(&["Bedeutung"], &[Some("Mechanismen_der_Züchtung"), None], &[3, 0])),
        109 => Some(decl(&["Wirtschaft", "Bedeutung"], &[Some("BWL"), Some("Mechanismen_der_Züchtung"), None], &[3, 0])),
        112 => Some(decl(&["Eigenschaften_n"], &[Some("Weisheit_etc"), None], &[3, 0])),
        113 => Some(decl(&["Wirtschaft"], &[Some("Pflanzen"), None], &[3, 0])),
        114 => Some(decl(&["Pro_Contra"], &[Some("Hilfe_erhalten"), None], &[3, 4, 0, 5])),
        115 => Some(decl(&["Pro_Contra"], &[Some("Helfen"), None], &[3, 4, 0, 5])),
        116 => Some(decl(&["Pro_Contra"], &[Some("Gegenposition"), None], &[3, 4, 0, 5])),
        117 => Some(decl(&["Pro_Contra"], &[Some("pro_nutzen"), None], &[3, 4, 0, 5])),
        118 => Some(decl(&["Bedeutung"], &[Some("Vorzeichen"), None], &[3, 0])),
        119 => Some(decl(&["Bedeutung"], &[Some("Vorzeichen"), None], &[3, 0])),
        120 => Some(decl(&["Pro_Contra"], &[Some("nervig"), None], &[3, 4, 0, 5])),
        123 => Some(decl(&["Pro_Contra"], &[Some("nicht_miteinander_auskommen"), None], &[3, 4, 0, 5])),
        124 => Some(decl(&["Pro_Contra"], &[Some("nicht_dagegen"), None], &[3, 4, 0, 5])),
        125 => Some(decl(&["Pro_Contra"], &[Some("kein_Gegenteil"), None], &[3, 4, 0, 5])),
        126 => Some(decl(&["Pro_Contra"], &[Some("nicht_dafür"), None], &[3, 4, 0, 5])),
        127 => Some(decl(&["Pro_Contra"], &[Some("Hilfe_nicht_gebrauchen"), None], &[3, 4, 0, 5])),
        128 => Some(decl(&["Pro_Contra"], &[Some("nicht_helfen_können"), None], &[3, 4, 0, 5])),
        129 => Some(decl(&["Pro_Contra"], &[Some("nicht_abgeneigt"), None], &[3, 4, 0, 5])),
        130 => Some(decl(&["Pro_Contra"], &[Some("unmotivierbar"), None], &[3, 4, 0, 5])),
        131 => Some(decl(
            &["Universum", "Grundstrukturen", "Multiversum", "Grundstrukturen"],
            &[Some("Reziproke_von_Transzendentalien"), Some("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"), Some("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"), Some("Reziprokes"), None],
            &[4, 1],
        )),
        132 => Some(decl(
            &["Planet_(10_und_oder_12)", "Menschliches", "Grundstrukturen"],
            &[Some("Gleichheit_Freiheit_Ordnung"), Some("Gleichheit_Freiheit"), Some("Ordnung_und_Filterung_12_und_1pro12"), None],
            &[4, 0, 5],
        )),
        135 => Some(decl(&["Menschliches"], &[Some("Wirkung"), None], &[3, 4, 0])),
        136 => Some(decl(&["Menschliches", "Eigenschaften_n"], &[Some("Egoismus"), Some("Egoismus_vs_Altruismus"), None], &[3, 0])),
        137 => Some(decl(&["Pro_Contra"], &[Some("Gegenspieler"), None], &[3, 4, 0, 5])),
        138 => Some(decl(&["Universum", "Menschliches"], &[Some("Gegentranszendentalien"), Some("Gegentranszendentalien"), None], &[4, 0])),
        139 => Some(decl(&["Menschliches"], &[Some("Gegentranszendentalien"), None], &[3, 0])),
        140 => Some(decl(&["Pro_Contra"], &[Some("ergibt_Sinn"), None], &[3, 4, 0, 5])),
        _ => None,
    }
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    [
        7u32, 8, 9, 28, 101, 102, 103, 104, 105, 106, 107, 108, 109, 112, 113, 114, 115, 116,
        117, 118, 119, 120, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 135, 136, 137,
        138, 139, 140, 466,
    ]
    .into_iter()
    .filter_map(|col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
    .collect()
}
