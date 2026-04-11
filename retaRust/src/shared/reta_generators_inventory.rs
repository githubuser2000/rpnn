#![allow(non_snake_case)]
// Aus Python reta automatisch extrahiertes Generator-Inventar.
// Noch keine Vollimplementierung aller Algorithmen; dies ist die bitgenaue Referenz fuer die Architektur.

#[derive(Debug, Clone, Copy)]
pub struct GeneratorPairSpec {
    pub main_name: &'static str,
    pub parameter_name: &'static str,
    pub col_a: i64,
    pub col_b: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct GeneratorTextSpec {
    pub main_name: &'static str,
    pub parameter_name: &'static str,
    pub code: &'static str,
}

pub const CONCAT_VERVIELFACHE_ZEILE_SOURCE_COLUMNS: &[i64] = &[19, 90];
pub const CONCAT_PRIM_CREATIVITY_TRIGGER: i64 = 64;
pub const CONCAT_GLEICHHEIT_FREIHEIT_TRIGGER: i64 = 132;
pub const CONCAT_GEIST_EMOTION_TRIGGER: i64 = 242;
pub const CONCAT_MOND_EXP_LOG_TRIGGER: i64 = 64;
pub const CONCAT_MOND_EXP_LOG_SOURCE_ROWS: &[i64] = &[44, 56];
pub const CONCAT_LOVE_POLYGON_TRIGGER: i64 = 9;
pub const CREATE_GESTIRN_TRIGGER: i64 = 64;

pub const GENERATED1_SPECS: &[GeneratorPairSpec] = &[

    GeneratorPairSpec { main_name: "Menschliches", parameter_name: "Moral", col_a: 216, col_b: 221 },
    GeneratorPairSpec { main_name: "Menschliches", parameter_name: "Sinn_des_Lebens", col_a: 181, col_b: 182 },
    GeneratorPairSpec { main_name: "Menschliches", parameter_name: "Egoismus", col_a: 66, col_b: 67 },
    GeneratorPairSpec { main_name: "Menschliches", parameter_name: "Liebe", col_a: 121, col_b: 122 },
    GeneratorPairSpec { main_name: "Grundstrukturen", parameter_name: "Liebe_(7)", col_a: 121, col_b: 122 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Weisheit_etc", col_a: 40, col_b: 41 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Dein_Recht_bekommen", col_a: 291, col_b: 292 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "unterlegen_überlegen", col_a: 380, col_b: 381 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Ehrlichkeit_und_Streit", col_a: 375, col_b: 376 },
    GeneratorPairSpec { main_name: "Eigenschaften_1/n", parameter_name: "Würdig", col_a: 373, col_b: 374 },
    GeneratorPairSpec { main_name: "Eigenschaften_1/n", parameter_name: "Regel_vs_Ausnahme", col_a: 371, col_b: 372 },
    GeneratorPairSpec { main_name: "Eigenschaften_1/n", parameter_name: "Werte", col_a: 360, col_b: 361 },
    GeneratorPairSpec { main_name: "Eigenschaften_1/n", parameter_name: "Gutartigkeits-Egoismus", col_a: 362, col_b: 363 },
    GeneratorPairSpec { main_name: "Eigenschaften_1/n", parameter_name: "Reflektieren_Erkenntnis-Erkennen", col_a: 364, col_b: 365 },
    GeneratorPairSpec { main_name: "Eigenschaften_1/n", parameter_name: "Vertrauen_wollen", col_a: 366, col_b: 367 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "einklinken_vertrauen_anprangern", col_a: 368, col_b: 369 },
    GeneratorPairSpec { main_name: "Eigenschaften_1/n", parameter_name: "Ausrichten_Einrichten", col_a: 358, col_b: 359 },
    GeneratorPairSpec { main_name: "Eigenschaften_1/n", parameter_name: "Toleranz_Respekt_Akzeptanz_Willkommen", col_a: 62, col_b: 63 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "familiebrauchen", col_a: 279, col_b: 280 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "ego", col_a: 277, col_b: 278 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Selbstsucht_Ichsucht_etc", col_a: 274, col_b: 275 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Forschen_Erfinden_Einklinken", col_a: 258, col_b: 259 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Kooperation_vs_Arsch", col_a: 245, col_b: 246 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Liebe_usw", col_a: 247, col_b: 248 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Selbstlosigkeit_Ichlosigkeit_etc", col_a: 238, col_b: 239 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "variationsreich_eintönig", col_a: 236, col_b: 237 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Zuneigung_Abneigung", col_a: 199, col_b: 200 },
    GeneratorPairSpec { main_name: "Menschliches", parameter_name: "ehrlich_vs_höflich", col_a: 224, col_b: 225 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "ehrlich_vs_höflich", col_a: 224, col_b: 225 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Tragweite", col_a: 211, col_b: 212 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "wertvoll", col_a: 186, col_b: 187 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Götter_Propheten_Familien_Freunde", col_a: 184, col_b: 185 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "sanft_vs_hart", col_a: 161, col_b: 162 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "sanft_vs_hart", col_a: 159, col_b: 160 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "vereinen_vs_verbinden", col_a: 133, col_b: 134 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "gut_böse_lieb_schlecht", col_a: 38, col_b: 39 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Sinn_und_Zweck_des_Lebens", col_a: 181, col_b: 182 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Zeit_vs_Raum", col_a: 49, col_b: 50 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "egalitär_vs_autoritär", col_a: 163, col_b: 164 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Meinungen_und_Ruf", col_a: 60, col_b: 61 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Meinungsintelligenz", col_a: 151, col_b: 152 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Sittlichkeit", col_a: 179, col_b: 180 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Führung", col_a: 173, col_b: 174 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Durchleuchten", col_a: 177, col_b: 178 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Fördern_Sensiblisieren_und_Gedeihen", col_a: 175, col_b: 176 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Überheblichkeit", col_a: 171, col_b: 172 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Polung_der_Liebe", col_a: 121, col_b: 122 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Egoismus_vs_Altruismus", col_a: 66, col_b: 67 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "kausal", col_a: 110, col_b: 111 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Gleichheit", col_a: 192, col_b: 193 },
    GeneratorPairSpec { main_name: "Eigenschaften_n", parameter_name: "Überleben", col_a: 194, col_b: 195 },
];

pub const GENERATED2_SPECS: &[GeneratorTextSpec] = &[
    GeneratorTextSpec { main_name: "Wichtigstes_zum_verstehen", parameter_name: "Motive_Sternpolygone", code: "primMotivStern" },
    GeneratorTextSpec { main_name: "Grundstrukturen", parameter_name: "nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)", code: "primzahlkreuzprocontra" },
    GeneratorTextSpec { main_name: "Pro_Contra", parameter_name: "Primzahlkreuz_pro_contra", code: "primzahlkreuzprocontra" },
    GeneratorTextSpec { main_name: "Bedeutung", parameter_name: "Primzahlkreuz_pro_contra", code: "primzahlkreuzprocontra" },
    GeneratorTextSpec { main_name: "Primvielfache", parameter_name: "Motive_gleichförmige_Polygone", code: "primMotivGleichf" },
    GeneratorTextSpec { main_name: "Primvielfache", parameter_name: "Struktur_gleichförmige_Polygone", code: "primStrukGleichf" },
    GeneratorTextSpec { main_name: "Primvielfache", parameter_name: "Motive_Sternpolygone", code: "primMotivStern" },
    GeneratorTextSpec { main_name: "Primvielfache", parameter_name: "Struktur_Sternpolygone", code: "primStrukStern" },
    GeneratorTextSpec { main_name: "Primvielfache", parameter_name: "Motiv_Sternpolygon_gebrochen-rational", code: "primMotivSternGebr" },
    GeneratorTextSpec { main_name: "Primvielfache", parameter_name: "Struktur_Sternpolyon_gebrochen-rational", code: "primStrukSternGebr" },
    GeneratorTextSpec { main_name: "Primvielfache", parameter_name: "Motiv_gleichförmige_Polygone_gebrochen-rational", code: "primMotivGleichfGebr" },
    GeneratorTextSpec { main_name: "Primvielfache", parameter_name: "Struktur_gleichförmige_Polygone_gebrochen-rational", code: "primStrukGleichfGebr" },
    GeneratorTextSpec { main_name: "Primvielfache", parameter_name: "beschrieben", code: "PrimCSV" },
];

pub const BOOL_AND_TUPLE_SET1_SPECS: &[GeneratorPairSpec] = &[
    GeneratorPairSpec { main_name: "Wichtigstes_zum_verstehen", parameter_name: "Zweitwichtigste", col_a: 10, col_b: -1 },
    GeneratorPairSpec { main_name: "Primzahlwirkung", parameter_name: "Universum_Strukturalien_Transzendentalien", col_a: 5, col_b: -1 },
    GeneratorPairSpec { main_name: "Primzahlwirkung", parameter_name: "Richtung_als_Richtung", col_a: -1, col_b: -1 },
    GeneratorPairSpec { main_name: "Primzahlwirkung", parameter_name: "Galaxieabsicht", col_a: 10, col_b: -1 },
    GeneratorPairSpec { main_name: "Primzahlwirkung", parameter_name: "Absicht_Reziproke_Galaxie", col_a: 42, col_b: -1 },
    GeneratorPairSpec { main_name: "Primzahlwirkung", parameter_name: "Universum_Reziproke", col_a: 131, col_b: -1 },
    GeneratorPairSpec { main_name: "Primzahlwirkung", parameter_name: "Dagegen-Gegentranszendentalie", col_a: 138, col_b: -1 },
    GeneratorPairSpec { main_name: "Primzahlwirkung", parameter_name: "neutrale_Gegentranszendentalie", col_a: 202, col_b: -1 },
];

pub const METAKONKRET_SPECS: &[GeneratorPairSpec] = &[
    GeneratorPairSpec { main_name: "Meta_vs_Konkret_(Universum)", parameter_name: "meta", col_a: 2, col_b: 0 },
    GeneratorPairSpec { main_name: "Meta_vs_Konkret_(Universum)", parameter_name: "konkret", col_a: 2, col_b: 1 },
    GeneratorPairSpec { main_name: "Meta_vs_Konkret_(Universum)", parameter_name: "Theorie", col_a: 3, col_b: 0 },
    GeneratorPairSpec { main_name: "Meta_vs_Konkret_(Universum)", parameter_name: "Praxis", col_a: 3, col_b: 1 },
    GeneratorPairSpec { main_name: "Meta_vs_Konkret_(Universum)", parameter_name: "Management", col_a: 4, col_b: 0 },
    GeneratorPairSpec { main_name: "Meta_vs_Konkret_(Universum)", parameter_name: "verändernd", col_a: 4, col_b: 1 },
    GeneratorPairSpec { main_name: "Meta_vs_Konkret_(Universum)", parameter_name: "ganzheitlich", col_a: 5, col_b: 0 },
    GeneratorPairSpec { main_name: "Meta_vs_Konkret_(Universum)", parameter_name: "darüber_hinausgehend", col_a: 5, col_b: 1 },
    GeneratorPairSpec { main_name: "Meta_vs_Konkret_(Universum)", parameter_name: "Unternehmung_Geschäft", col_a: 6, col_b: 0 },
    GeneratorPairSpec { main_name: "Meta_vs_Konkret_(Universum)", parameter_name: "wertvoll", col_a: 6, col_b: 1 },
    GeneratorPairSpec { main_name: "Meta_vs_Konkret_(Universum)", parameter_name: "Beherrschen", col_a: 7, col_b: 0 },
    GeneratorPairSpec { main_name: "Meta_vs_Konkret_(Universum)", parameter_name: "Richtung", col_a: 7, col_b: 1 },
];

