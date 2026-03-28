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
            &["Universum"],
            &[Some("Geist__(15)"), Some("Geist_(15)"), None],
            &[4, 0],
        )),
        201 => Some(decl(
            &["Universum"],
            &[Some("Reziproke_von_Transzendentalien"), None],
            &[4, 1],
        )),
        202 => Some(decl(
            &["Universum", "Menschliches"],
            &[Some("Gegentranszendentalien"), Some("Gegentranszendentalien"), None],
            &[4, 0],
        )),
        203 => Some(decl(
            &["Grundstrukturen", "Universum", "Multiversum"],
            &[
                Some("Model_of_Hierarchical_Complexity"),
                Some("Model_of_Hierarchical_Complexity"),
                Some("Model_of_Hierarchical_Complexity"),
                None,
            ],
            &[4, 1],
        )),
        204 => Some(decl(
            &["Universum", "Grundstrukturen", "Grundstrukturen"],
            &[
                Some("Kategorie"),
                Some("Reziprokes"),
                Some("Reflektion_und_Kategorien_(1/15)"),
                None,
            ],
            &[4, 0, 5],
        )),
        205 => Some(decl(
            &["Universum", "Grundstrukturen", "Grundstrukturen"],
            &[
                Some("Kategorie"),
                Some("Reziprokes"),
                Some("Reflektion_und_Kategorien_(1/15)"),
                None,
            ],
            &[1, 4, 5],
        )),
        207 => Some(decl(
            &["Wichtigstes_zum_gedanklich_einordnen", "Religionen"],
            &[Some("Wichtigste"), Some("der_Tierkreiszeichen"), None],
            &[3, 0],
        )),
        208 => Some(decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Liebe"), Some("Liebe_(7)"), None],
            &[0, 5],
        )),
        209 => Some(decl(
            &["Bedeutung"],
            &[Some("in_ReTa"), None],
            &[4, 0],
        )),
        210 => Some(decl(
            &["Bedeutung"],
            &[Some("in_ReTa"), None],
            &[4, 0],
        )),
        213 => Some(decl(
            &["Planet_(10_und_oder_12)"],
            &[Some("Komplexität"), None],
            &[4, 0, 5],
        )),
        214 => Some(decl(
            &["Planet_(10_und_oder_12)"],
            &[Some("Intelligenz"), None],
            &[4, 0, 5],
        )),
        215 => Some(decl(
            &["Menschliches"],
            &[Some("Moral"), None],
            &[3, 0],
        )),
        216 => Some(decl(
            &["Menschliches"],
            &[Some("Moral"), None],
            &[3, 4, 1, 0],
        )),
        217 => Some(decl(
            &["Religionen"],
            &[Some("Hinduismus"), None],
            &[3, 0],
        )),
        218 => Some(decl(
            &["Universum", "Galaxie"],
            &[Some("Raum-Missionen"), Some("Raum-Missionen"), None],
            &[3, 4, 0],
        )),
        219 => Some(decl(
            &["Inkrementieren", "Universum", "Teilchen-Meta-Physik"],
            &[
                Some("Teilchen-Meta-Physik"),
                Some("Teilchen-Meta-Physik"),
                Some("das_Universelle_(15)"),
                None,
            ],
            &[4, 0],
        )),
        220 => Some(decl(
            &["Eigenschaften_n"],
            &[Some("ähnlich"), None],
            &[3, 4, 0],
        )),
        221 => Some(decl(
            &["Grundstrukturen"],
            &[Some("Liebe_(7)"), None],
            &[],
        )),
        222 => Some(decl(
            &["Pro_Contra"],
            &[Some("Gegenteil"), None],
            &[3, 4, 0, 5],
        )),
        223 => Some(decl(
            &["Inkrementieren", "Teilchen-Meta-Physik"],
            &[Some("Teilchen-Meta-Physik"), Some("die_Galaxie_Unterbereiche_(13)"), None],
            &[4, 0],
        )),
        226 => Some(decl(
            &["Primvielfache"],
            &[Some("Rahmen-Bedingungen"), None],
            &[3, 4, 0],
        )),
        229 => Some(decl(
            &[
                "Grundstrukturen",
                "Multiversum",
                "Grundstrukturen",
                "Multiversum",
                "Menschliches",
                "Menschliches",
            ],
            &[
                Some("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"),
                Some("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"),
                Some("Geist_(15)"),
                Some("Geist_(15)"),
                Some("Motive"),
                Some("Bewusstsein_und_Wahrnehmung"),
                None,
            ],
            &[4, 0],
        )),
        230 => Some(decl(
            &["Menschliches", "Menschliches"],
            &[Some("Gefühle"), Some("Motive"), None],
            &[4, 0],
        )),
        231 => Some(decl(
            &["Grundstrukturen", "Grundstrukturen", "Multiversum", "Menschliches"],
            &[
                Some("Reziprokes"),
                Some("Geist_(15)"),
                Some("Geist_(15)"),
                Some("Bewusstsein_und_Wahrnehmung"),
                None,
            ],
            &[4, 1],
        )),
        232 => Some(decl(
            &["Planet_(10_und_oder_12)", "Grundstrukturen"],
            &[Some("Meta-Systeme_(12)"), Some("Meta-Systeme_(12)"), None],
            &[0, 5],
        )),
        233 => Some(decl(
            &["Planet_(10_und_oder_12)", "Grundstrukturen"],
            &[
                Some("Wirklichkeiten_(10)"),
                Some("Wirklichkeiten_Wahrheit_Wahrnehmung_(10)"),
                None,
            ],
            &[0, 5],
        )),
        234 => Some(decl(
            &["Grundstrukturen"],
            &[Some("Modus_und_Sein_(8)"), None],
            &[0, 5],
        )),
        235 => Some(decl(
            &["Menschliches"],
            &[Some("(politische)_Richtungen_(7)"), None],
            &[4, 0, 5],
        )),
        240 => Some(decl(
            &["Menschliches", "Grundstrukturen"],
            &[
                Some("Ansichten_Standpunkte_(18_17)"),
                Some("Ansichten_Standpunkte_(18_17)"),
                None,
            ],
            &[4, 0, 5],
        )),
        241 => Some(decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Gesellschaftsschicht"), Some("Klassen_(20)"), None],
            &[3, 0, 5],
        )),
        242 => Some(decl(
            &["Universum", "Grundstrukturen", "Grundstrukturen", "Multiversum"],
            &[
                Some("Geist__(15)"),
                Some("nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)"),
                Some("Geist_(15)"),
                Some("Geist_(15)"),
                None,
            ],
            &[4, 0],
        )),
        243 => Some(decl(
            &["Menschliches", "Grundstrukturen"],
            &[Some("Gefühle"), Some("Gefühle_(7)"), None],
            &[0, 5],
        )),
        249 => Some(decl(
            &["Grundstrukturen", "Menschliches"],
            &[Some("Gedanken_sind_Positionen_(17)"), Some("Gedanken_sind_Positionen_(17)"), None],
            &[0, 5],
        )),
        250 => Some(decl(
            &["Grundstrukturen"],
            &[Some("Konkreta_und_Focus_(2)"), None],
            &[0, 5],
        )),
        251 => Some(decl(
            &["Grundstrukturen", "Menschliches"],
            &[Some("Impulse_(5)"), Some("Errungenschaften"), None],
            &[0, 5],
        )),
        252 => Some(decl(
            &["Grundstrukturen"],
            &[Some("Verbundenheiten_(18)"), None],
            &[0, 5],
        )),
        253 => Some(decl(
            &["Grundstrukturen", "Grundstrukturen"],
            &[Some("Konkreta_und_Focus_(2)"), Some("Impulse_(5)"), None],
            &[0, 5],
        )),
        254 => Some(decl(
            &["Grundstrukturen"],
            &[Some("Triebe_und_Bedürfnisse_(6)"), None],
            &[0, 5],
        )),
        255 => Some(decl(
            &["Grundstrukturen"],
            &[Some("Lust_(9)"), None],
            &[0, 5],
        )),
        256 => Some(decl(
            &["Grundstrukturen"],
            &[Some("Reflexe_(3)"), None],
            &[0, 5],
        )),
        257 => Some(decl(
            &["Grundstrukturen", "Grundstrukturen", "Menschliches"],
            &[Some("Reziprokes"), Some("Impulse_(5)"), Some("Errungenschaften"), None],
            &[1, 5],
        )),
        260 => Some(decl(
            &["Grundstrukturen"],
            &[Some("Absicht_10_ist_Wirklichkeit_erkennen"), None],
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
    [
        7u32, 8, 9, 28, 201, 202, 203, 204, 205, 207, 208, 209, 210, 213, 214, 215, 216, 217,
        218, 219, 220, 221, 222, 223, 226, 229, 230, 231, 232, 233, 234, 235, 240, 241, 242,
        243, 249, 250, 251, 252, 253, 254, 255, 256, 257, 260, 466,
    ]
    .into_iter()
    .filter_map(|col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
    .collect()
}
