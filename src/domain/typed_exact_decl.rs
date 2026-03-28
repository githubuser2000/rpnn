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
            &[Some("Religion"), Some("Religions-Gründer-Typ"), None],
            &[3, 0],
        )),
        8 => Some(decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Gesellschaftsschicht"), Some("Klassen_(20)"), None],
            &[3, 5, 0],
        )),
        9 => Some(decl(
            &["Menschliches", "Grundstrukturen", "Eigenschaften_n"],
            &[Some("Liebe"), Some("Liebe_(7)"), None],
            &[3, 0],
        )),
        28 => Some(decl(
            &["Universum"],
            &[Some("Geist__(15)"), Some("Geist_(15)"), None],
            &[4, 0],
        )),
        64 => Some(decl(
            &["Wichtigstes_zum_verstehen", "Bedeutung"],
            &[Some("Drittwichtigste"), Some("Gestirn"), None],
            &[3, 0],
        )),
        65 => Some(decl(
            &[
                "Wichtigstes_zum_verstehen",
                "Universum",
                "Universum",
                "Grundstrukturen",
                "Universum",
                "Multiversum",
                "Inkrementieren",
            ],
            &[
                Some("Zweitwichtigste"),
                Some("universelles_Recht"),
                Some("warum_Transzendentalie_=_Komplexität_von_Michael_Commons"),
                Some("Model_of_Hierarchical_Complexity"),
                Some("Model_of_Hierarchical_Complexity"),
                Some("Model_of_Hierarchical_Complexity"),
                Some("warum_Transzendentalie_=_Komplexität_von_Michael_Commons"),
                None,
            ],
            &[4, 0],
        )),
        68 => Some(decl(
            &["Menschliches"],
            &[Some("INCELs"), None],
            &[3, 0],
        )),
        69 => Some(decl(
            &["Grundstrukturen", "Wirtschaft"],
            &[Some("System"), Some("System"), None],
            &[4, 0],
        )),
        70 => Some(decl(
            &["Grundstrukturen", "Wirtschaft"],
            &[Some("System"), Some("System"), None],
            &[4, 0],
        )),
        71 => Some(decl(
            &["Wirtschaft"],
            &[Some("Erklärung"), None],
            &[3, 0],
        )),
        72 => Some(decl(
            &["Religionen"],
            &[Some("Religions-Gründer-Typ"), None],
            &[3, 0],
        )),
        73 => Some(decl(
            &["Menschliches"],
            &[Some("irrationale_Zahlen_durch_Wurzelbildung"), None],
            &[3, 0],
        )),
        74 => Some(decl(
            &["Inkrementieren"],
            &[None, None],
            &[3, 0],
        )),
        75 => Some(decl(
            &["Grundstrukturen", "Universum", "Multiversum"],
            &[
                Some("Model_of_Hierarchical_Complexity"),
                Some("Model_of_Hierarchical_Complexity"),
                Some("Model_of_Hierarchical_Complexity"),
                None,
            ],
            &[3, 4, 0],
        )),
        76 => Some(decl(
            &["Operationen"],
            &[Some("4"), None],
            &[3, 4, 0],
        )),
        77 => Some(decl(
            &["Universum", "Galaxie", "Operationen"],
            &[Some("Kugeln_Kreise"), Some("Kugeln_Kreise"), Some("4"), None],
            &[4, 0],
        )),
        78 => Some(decl(
            &["Operationen"],
            &[Some("2"), None],
            &[3, 0],
        )),
        79 => Some(decl(
            &[
                "Planet_(10_und_oder_12)",
                "Planet_(10_und_oder_12)",
                "Operationen",
                "Grundstrukturen",
            ],
            &[
                Some("Meta-Systeme_(12)"),
                Some("Gleichheit_Freiheit_Ordnung"),
                Some("2"),
                Some("Meta-Systeme_(12)"),
                None,
            ],
            &[3, 0],
        )),
        80 => Some(decl(
            &[
                "Planet_(10_und_oder_12)",
                "Planet_(10_und_oder_12)",
                "Operationen",
                "Grundstrukturen",
            ],
            &[
                Some("Meta-Systeme_(12)"),
                Some("Gleichheit_Freiheit_Ordnung"),
                Some("2"),
                Some("Meta-Systeme_(12)"),
                None,
            ],
            &[4, 0],
        )),
        81 => Some(decl(
            &["Operationen"],
            &[Some("4"), None],
            &[4, 0],
        )),
        82 => Some(decl(
            &["Größenordnung"],
            &[Some("Organisationen"), None],
            &[3, 0],
        )),
        83 => Some(decl(
            &["Größenordnung"],
            &[Some("politische_Systeme"), None],
            &[3, 0],
        )),
        84 => Some(decl(
            &["Grundstrukturen", "Galaxie", "Universum"],
            &[
                Some("analytische_Ontologie"),
                Some("analytische_Ontologie"),
                Some("analytische_Ontologie"),
                None,
            ],
            &[4, 0],
        )),
        86 => Some(decl(
            &["Operationen"],
            &[Some("Halbierung"), None],
            &[3, 0],
        )),
        87 => Some(decl(
            &["Religionen"],
            &[Some("Sternpolygon_vs_gleichförmiges"), None],
            &[3, 1, 0],
        )),
        88 => Some(decl(
            &["Menschliches", "Eigenschaften_n"],
            &[Some("Sinn_des_Lebens"), Some("Sinn_und_Zweck_des_Lebens"), None],
            &[3, 0],
        )),
        89 => Some(decl(
            &["Wirtschaft"],
            &[Some("Maschinen"), None],
            &[3, 0],
        )),
        90 => Some(decl(
            &["Galaxie"],
            &[Some("Offenbarung_des_Johannes"), None],
            &[3, 0],
        )),
        91 => Some(decl(
            &["Galaxie"],
            &[Some("chinesisches_Horoskop"), None],
            &[3, 0],
        )),
        92 => Some(decl(
            &["Operationen"],
            &[Some("3"), None],
            &[3, 0],
        )),
        93 => Some(decl(
            &["Operationen"],
            &[Some("3"), None],
            &[4, 0],
        )),
        94 => Some(decl(
            &["Operationen"],
            &[Some("9"), None],
            &[4, 0],
        )),
        95 => Some(decl(
            &["Inkrementieren"],
            &[None, None],
            &[3, 0],
        )),
        96 => Some(decl(
            &["Operationen"],
            &[Some("5"), None],
            &[3, 0],
        )),
        99 => Some(decl(
            &["Wirtschaft"],
            &[Some("Organisationsform"), None],
            &[3, 0],
        )),
        100 => Some(decl(
            &["Pro_Contra"],
            &[Some("Gegenteil"), None],
            &[3, 4, 0, 5],
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
        7u32, 8, 9, 28, 64, 65, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82,
        83, 84, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 99, 100, 466,
    ]
    .into_iter()
    .filter_map(|col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
    .collect()
}
