use crate::domain::decl_model::HtmlDeclMeta;

pub const TYPED_EXACT_DECL_COLUMNS: &[u32] = &[0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32, 13u32, 14u32, 15u32, 16u32, 17u32, 18u32, 19u32, 20u32, 28u32, 241u32, 242u32, 243u32, 249u32, 303u32, 314u32, 324u32, 466u32];

pub fn is_typed_exact_decl_column(col: u32) -> bool {
    TYPED_EXACT_DECL_COLUMNS.contains(&col)
}

pub fn typed_exact_decl_for_column(col: u32) -> Option<HtmlDeclMeta> {
    Some(match col {
        0 => HtmlDeclMeta { p1_groups: vec!["Wichtigstes_zum_gedanklich_einordnen".to_string(), "Religionen".to_string(), "Religionen".to_string(), "Galaxie".to_string()], p2_slots: vec![Some("Wichtigste".to_string()), Some("Sternpolygon".to_string()), Some("der_Tierkreiszeichen".to_string()), Some("Thomasevangelium".to_string()), None], p4_tags: vec![3, 0] },
        1 => HtmlDeclMeta { p1_groups: vec!["Wichtigstes_zum_gedanklich_einordnen".to_string(), "Galaxie".to_string()], p2_slots: vec![Some("Wichtigste".to_string()), Some("babylonische_Tierkreiszeichen".to_string()), None], p4_tags: vec![3, 0] },
        2 => HtmlDeclMeta { p1_groups: vec!["Wichtigstes_zum_gedanklich_einordnen".to_string(), "Galaxie".to_string()], p2_slots: vec![Some("Wichtigste".to_string()), Some("babylonische_Tierkreiszeichen".to_string()), None], p4_tags: vec![3, 0] },
        3 => HtmlDeclMeta { p1_groups: vec!["Galaxie".to_string()], p2_slots: vec![Some("Thomasevangelium".to_string()), None], p4_tags: vec![3, 0] },
        4 => HtmlDeclMeta { p1_groups: vec!["Wichtigstes_zum_verstehen".to_string(), "Grundstrukturen".to_string(), "Größenordnung".to_string(), "Universum".to_string(), "Inkrementieren".to_string()], p2_slots: vec![Some("Wichtigste".to_string()), Some("Strukturgrösse".to_string()), Some("Strukturgrösse".to_string()), Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter".to_string()), Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter".to_string()), None], p4_tags: vec![3, 4, 0, 5] },
        5 => HtmlDeclMeta { p1_groups: vec!["Wichtigstes_zum_verstehen".to_string(), "Universum".to_string(), "Universum".to_string(), "Universum".to_string(), "Grundstrukturen".to_string(), "Multiversum".to_string(), "Inkrementieren".to_string(), "Inkrementieren".to_string(), "Kontinuum".to_string()], p2_slots: vec![Some("Wichtigste".to_string()), Some("Transzendentalien".to_string()), Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter".to_string()), Some("warum_Transzendentalie_=_Komplexität_von_Michael_Commons".to_string()), Some("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)".to_string()), Some("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)".to_string()), Some("warum_Transzendentalie_=_Strukturgroesse_=_Charakter".to_string()), Some("warum_Transzendentalie_=_Komplexität_von_Michael_Commons".to_string()), Some("O".to_string()), None], p4_tags: vec![4, 0] },
        6 => HtmlDeclMeta { p1_groups: vec!["Religionen".to_string()], p2_slots: vec![Some("Sternpolygon".to_string()), None], p4_tags: vec![3, 0] },
        7 => HtmlDeclMeta { p1_groups: vec!["Religionen".to_string()], p2_slots: vec![Some("Messias".to_string()), None], p4_tags: vec![3, 0] },
        8 => HtmlDeclMeta { p1_groups: vec!["Wichtigstes_zum_verstehen".to_string(), "Menschliches".to_string(), "Grundstrukturen".to_string()], p2_slots: vec![Some("Wichtigste".to_string()), Some("Liebe".to_string()), Some("Liebe_(7)".to_string()), None], p4_tags: vec![0, 5] },
        9 => HtmlDeclMeta { p1_groups: vec!["Menschliches".to_string(), "Grundstrukturen".to_string()], p2_slots: vec![Some("Liebe".to_string()), Some("Liebe_(7)".to_string()), None], p4_tags: vec![0, 5] },
        10 => HtmlDeclMeta { p1_groups: vec!["Wichtigstes_zum_verstehen".to_string(), "Grundstrukturen".to_string(), "Menschliches".to_string()], p2_slots: vec![Some("Wichtigste".to_string()), Some("Paradigmen_sind_Absichten_(13)".to_string()), Some("Motive".to_string()), None], p4_tags: vec![3, 0] },
        11 => HtmlDeclMeta { p1_groups: vec!["Menschliches".to_string()], p2_slots: vec![Some("Errungenschaften".to_string()), None], p4_tags: vec![3, 0] },
        12 => HtmlDeclMeta { p1_groups: vec!["Menschliches".to_string()], p2_slots: vec![Some("evolutionär_erwerben_und_Intelligenz_Kreativität".to_string()), None], p4_tags: vec![3, 0] },
        13 => HtmlDeclMeta { p1_groups: vec!["Menschliches".to_string(), "Menschliches".to_string()], p2_slots: vec![Some("evolutionär_erwerben_und_Intelligenz_Kreativität".to_string()), Some("brauchen".to_string()), None], p4_tags: vec![3, 0] },
        14 => HtmlDeclMeta { p1_groups: vec!["Menschliches".to_string()], p2_slots: vec![Some("brauchen".to_string()), None], p4_tags: vec![3, 0] },
        15 => HtmlDeclMeta { p1_groups: vec!["Pro_Contra".to_string()], p2_slots: vec![Some("contra".to_string()), None], p4_tags: vec![3, 4, 0, 5] },
        16 => HtmlDeclMeta { p1_groups: vec!["Religionen".to_string()], p2_slots: vec![Some("gleichförmiges_Polygon".to_string()), None], p4_tags: vec![3, 1] },
        17 => HtmlDeclMeta { p1_groups: vec!["Pro_Contra".to_string()], p2_slots: vec![Some("Pro".to_string()), None], p4_tags: vec![3, 4, 0, 5] },
        18 => HtmlDeclMeta { p1_groups: vec!["Menschliches".to_string()], p2_slots: vec![Some("Motive".to_string()), None], p4_tags: vec![3, 0] },
        19 => HtmlDeclMeta { p1_groups: vec!["Wichtigstes_zum_verstehen".to_string(), "Bedeutung".to_string()], p2_slots: vec![Some("Zweitwichtigste".to_string()), Some("Primzahlen".to_string()), None], p4_tags: vec![3, 0] },
        20 => HtmlDeclMeta { p1_groups: vec!["Größenordnung".to_string(), "Licht".to_string()], p2_slots: vec![Some("Licht".to_string()), None, None], p4_tags: vec![3, 4, 0, 5] },
        28 => HtmlDeclMeta { p1_groups: vec!["Menschliches".to_string(), "Grundstrukturen".to_string()], p2_slots: vec![Some("Liebe".to_string()), Some("Liebe_(7)".to_string()), None], p4_tags: vec![0, 5] },
        241 => HtmlDeclMeta { p1_groups: vec!["Menschliches".to_string(), "Grundstrukturen".to_string()], p2_slots: vec![Some("Gesellschaftsschicht".to_string()), Some("Klassen_(20)".to_string()), None], p4_tags: vec![3, 0, 5] },
        242 => HtmlDeclMeta { p1_groups: vec!["Universum".to_string(), "Grundstrukturen".to_string(), "Grundstrukturen".to_string(), "Multiversum".to_string()], p2_slots: vec![Some("Geist__(15)".to_string()), Some("nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)".to_string()), Some("Geist_(15)".to_string()), Some("Geist_(15)".to_string()), None], p4_tags: vec![4, 0] },
        243 => HtmlDeclMeta { p1_groups: vec!["Menschliches".to_string(), "Grundstrukturen".to_string()], p2_slots: vec![Some("Gefühle".to_string()), Some("Gefühle_(7)".to_string()), None], p4_tags: vec![0, 5] },
        249 => HtmlDeclMeta { p1_groups: vec!["Grundstrukturen".to_string(), "Menschliches".to_string()], p2_slots: vec![Some("Gedanken_sind_Positionen_(17)".to_string()), Some("Gedanken_sind_Positionen_(17)".to_string()), None], p4_tags: vec![0, 5] },
        303 => HtmlDeclMeta { p1_groups: vec!["Galaxie".to_string()], p2_slots: vec![Some("Thomasevangelium".to_string()), None], p4_tags: vec![3, 0] },
        314 => HtmlDeclMeta { p1_groups: vec!["Menschliches".to_string()], p2_slots: vec![Some("Mensch-zu-Tier".to_string()), None], p4_tags: vec![4, 0, 5] },
        324 => HtmlDeclMeta { p1_groups: vec!["Planet_(10_und_oder_12)".to_string()], p2_slots: vec![Some("Gleichheit_Freiheit_Ordnung".to_string()), None], p4_tags: vec![0, 5] },
        466 => HtmlDeclMeta { p1_groups: vec!["Menschliches".to_string(), "Grundstrukturen".to_string()], p2_slots: vec![Some("Gewalt".to_string()), Some("Gewalt".to_string()), None], p4_tags: vec![3, 4, 0] },
        _ => return None,
    })
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    TYPED_EXACT_DECL_COLUMNS
        .iter()
        .filter_map(|&col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
