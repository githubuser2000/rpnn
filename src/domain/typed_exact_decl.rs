use crate::domain::decl_model::HtmlDeclMeta;

fn decl(p1: &[&str], p2: &[Option<&str>], p4: &[u8]) -> HtmlDeclMeta {
    HtmlDeclMeta::from_slices(p1, p2, p4)
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
            &[Some("Wichtigste"), Some("Strukturgrösse"), Some("Strukturgrösse"), Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"), Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"), None],
            &[3, 4, 0, 5],
        )),
        5 => Some(decl(
            &["Wichtigstes_zum_verstehen", "Universum", "Universum", "Universum", "Grundstrukturen", "Multiversum", "Inkrementieren", "Inkrementieren", "Kontinuum"],
            &[Some("Wichtigste"), Some("Transzendentalien"), Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"), Some("warum_Transzendentalie_=_Komplexität_von_Michael_Commons"), Some("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"), Some("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"), Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"), Some("warum_Transzendentalie_=_Komplexität_von_Michael_Commons"), Some("O"), None],
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
        _ => None,
    }
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 28, 466]
        .into_iter()
        .filter_map(|col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
