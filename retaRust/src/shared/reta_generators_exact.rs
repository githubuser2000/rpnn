#![allow(non_snake_case)]
// Automatisch aus Python- und Rust-Quellen des hochgeladenen reta-Projekts erzeugt.
// Ziel: exakte Referenz aller Generatorspalten in reta-Architektur.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratorFamily { Generated1, Concat1, BoolAndTupleSet1, GebroUni1, GebrGal1, Generated2, Kombi2, GebrEmo1, GebrGroe1, Metakonkret }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratorCallSpec { pub name: &'static str, pub py_source: &'static str, pub description: &'static str }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratorColumnSpec { pub family: GeneratorFamily, pub parameter_main: &'static str, pub parameter_name: &'static str, pub raw_python_data: &'static str }

pub const PYTHON_GENERATOR_CALL_ORDER: &[GeneratorCallSpec] = &[
    GeneratorCallSpec { name: "readConcatCsv", py_source: "reta.py:1460-1497", description: "CSV-basierte Generator-/Concat-Erweiterungen 1..9" },
    GeneratorCallSpec { name: "concatVervielfacheZeile", py_source: "libs/lib4tables_concat.py:410-496", description: "Vielfache-Spalte" },
    GeneratorCallSpec { name: "concatModallogik", py_source: "libs/lib4tables_concat.py:497-853", description: "Modallogik aus generated1" },
    GeneratorCallSpec { name: "concatPrimCreativityType", py_source: "libs/lib4tables_concat.py:282-324", description: "Prim-Kreativität/Typ" },
    GeneratorCallSpec { name: "concatGleichheitFreiheitDominieren", py_source: "libs/lib4tables_concat.py:214-247", description: "Gleichheit/Freiheit/Dominieren" },
    GeneratorCallSpec { name: "concatGeistEmotionEnergieMaterieTopologie", py_source: "libs/lib4tables_concat.py:248-281", description: "Geist/Emotion/Energie/Materie/Topologie" },
    GeneratorCallSpec { name: "concatMondExponzierenLogarithmusTyp", py_source: "libs/lib4tables_concat.py:325-409", description: "Mond/Exponenz/Logarithmus/Typ" },
    GeneratorCallSpec { name: "concat1RowPrimUniverse2", py_source: "libs/lib4tables_concat.py:1421-2009", description: "generated2-Textgeneratoren" },
    GeneratorCallSpec { name: "concat1PrimzahlkreuzProContra", py_source: "libs/lib4tables_concat.py:975-1420", description: "Primzahlkreuz pro/contra" },
    GeneratorCallSpec { name: "concatLovePolygon", py_source: "libs/lib4tables_concat.py:97-131", description: "Liebe/Sternpolygon" },
    GeneratorCallSpec { name: "spalteFuerGegenInnenAussenSeitlichPrim", py_source: "libs/lib4tables_concat.py:2743-2863", description: "boolAndTupleSet1" },
    GeneratorCallSpec { name: "spalteMetaKontretTheorieAbstrakt_etc_1", py_source: "libs/lib4tables_concat.py:2010-2025", description: "metakonkret" },
    GeneratorCallSpec { name: "createSpalteGestirn", py_source: "libs/tableHandling.py:1456-1517", description: "Gestirn-Spalte" },
];

pub const PYTHON_GENERATED1_SPECS: &[GeneratorColumnSpec] = &[
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Menschliches", parameter_name: "Moral", raw_python_data: "vec![Tuple(vec![Int(216), Int(221)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Menschliches", parameter_name: "Sinn_des_Lebens", raw_python_data: "vec![Tuple(vec![Int(181), Int(182)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Menschliches", parameter_name: "Egoismus", raw_python_data: "vec![Tuple(vec![Int(66), Int(67)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Menschliches", parameter_name: "Liebe", raw_python_data: "vec![Tuple(vec![Int(121), Int(122)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Grundstrukturen", parameter_name: "Liebe_(7)", raw_python_data: "vec![Tuple(vec![Int(121), Int(122)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Weisheit_etc", raw_python_data: "vec![Tuple(vec![Int(40), Int(41)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Dein_Recht_bekommen", raw_python_data: "vec![Tuple(vec![Int(291), Int(292)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "unterlegen_überlegen", raw_python_data: "vec![Tuple(vec![Int(380), Int(381)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Ehrlichkeit_und_Streit", raw_python_data: "vec![Tuple(vec![Int(375), Int(376)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_1/n", parameter_name: "Würdig", raw_python_data: "vec![Tuple(vec![Int(373), Int(374)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_1/n", parameter_name: "Regel_vs_Ausnahme", raw_python_data: "vec![Tuple(vec![Int(371), Int(372)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_1/n", parameter_name: "Werte", raw_python_data: "vec![Tuple(vec![Int(360), Int(361)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_1/n", parameter_name: "Gutartigkeits-Egoismus", raw_python_data: "vec![Tuple(vec![Int(362), Int(363)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_1/n", parameter_name: "Reflektieren_Erkenntnis-Erkennen", raw_python_data: "vec![Tuple(vec![Int(364), Int(365)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_1/n", parameter_name: "Vertrauen_wollen", raw_python_data: "vec![Tuple(vec![Int(366), Int(367)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "einklinken_vertrauen_anprangern", raw_python_data: "vec![Tuple(vec![Int(368), Int(369)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_1/n", parameter_name: "Ausrichten_Einrichten", raw_python_data: "vec![Tuple(vec![Int(358), Int(359)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_1/n", parameter_name: "Toleranz_Respekt_Akzeptanz_Willkommen", raw_python_data: "vec![Tuple(vec![Int(62), Int(63)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "familiebrauchen", raw_python_data: "vec![Tuple(vec![Int(279), Int(280)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "ego", raw_python_data: "vec![Tuple(vec![Int(277), Int(278)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Selbstsucht_Ichsucht_etc", raw_python_data: "vec![Tuple(vec![Int(274), Int(275)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Forschen_Erfinden_Einklinken", raw_python_data: "vec![Tuple(vec![Int(258), Int(259)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Kooperation_vs_Arsch", raw_python_data: "vec![Tuple(vec![Int(245), Int(246)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Liebe_usw", raw_python_data: "vec![Tuple(vec![Int(247), Int(248)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Selbstlosigkeit_Ichlosigkeit_etc", raw_python_data: "vec![Tuple(vec![Int(238), Int(239)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "variationsreich_eintönig", raw_python_data: "vec![Tuple(vec![Int(236), Int(237)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Zuneigung_Abneigung", raw_python_data: "vec![Tuple(vec![Int(199), Int(200)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Menschliches", parameter_name: "ehrlich_vs_höflich", raw_python_data: "vec![Tuple(vec![Int(224), Int(225)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "ehrlich_vs_höflich", raw_python_data: "vec![Tuple(vec![Int(224), Int(225)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Tragweite", raw_python_data: "vec![Tuple(vec![Int(211), Int(212)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "wertvoll", raw_python_data: "vec![Tuple(vec![Int(186), Int(187)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Götter_Propheten_Familien_Freunde", raw_python_data: "vec![Tuple(vec![Int(184), Int(185)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "sanft_vs_hart", raw_python_data: "vec![Tuple(vec![Int(161), Int(162)]), Tuple(vec![Int(159), Int(160)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "vereinen_vs_verbinden", raw_python_data: "vec![Tuple(vec![Int(133), Int(134)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "gut_böse_lieb_schlecht", raw_python_data: "vec![Tuple(vec![Int(38), Int(39)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Sinn_und_Zweck_des_Lebens", raw_python_data: "vec![Tuple(vec![Int(181), Int(182)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Zeit_vs_Raum", raw_python_data: "vec![Tuple(vec![Int(49), Int(50)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "egalitär_vs_autoritär", raw_python_data: "vec![Tuple(vec![Int(163), Int(164)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Meinungen_und_Ruf", raw_python_data: "vec![Tuple(vec![Int(60), Int(61)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Meinungsintelligenz", raw_python_data: "vec![Tuple(vec![Int(151), Int(152)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Sittlichkeit", raw_python_data: "vec![Tuple(vec![Int(179), Int(180)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Führung", raw_python_data: "vec![Tuple(vec![Int(173), Int(174)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Durchleuchten", raw_python_data: "vec![Tuple(vec![Int(177), Int(178)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Fördern_Sensiblisieren_und_Gedeihen", raw_python_data: "vec![Tuple(vec![Int(175), Int(176)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Überheblichkeit", raw_python_data: "vec![Tuple(vec![Int(171), Int(172)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Polung_der_Liebe", raw_python_data: "vec![Tuple(vec![Int(121), Int(122)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Egoismus_vs_Altruismus", raw_python_data: "vec![Tuple(vec![Int(66), Int(67)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "kausal", raw_python_data: "vec![Tuple(vec![Int(110), Int(111)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Gleichheit", raw_python_data: "vec![Tuple(vec![Int(192), Int(193)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated1, parameter_main: "Eigenschaften_n", parameter_name: "Überleben", raw_python_data: "vec![Tuple(vec![Int(194), Int(195)])]" },
];

pub const PYTHON_BOOL_AND_TUPLESET1_SPECS: &[GeneratorColumnSpec] = &[
    GeneratorColumnSpec { family: GeneratorFamily::BoolAndTupleSet1, parameter_main: "Wichtigstes_zum_verstehen", parameter_name: "Zweitwichtigste", raw_python_data: "vec![Tuple(vec![Int(10)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::BoolAndTupleSet1, parameter_main: "Primzahlwirkung", parameter_name: "Universum_Strukturalien_Transzendentalien", raw_python_data: "vec![Tuple(vec![Int(5)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::BoolAndTupleSet1, parameter_main: "Primzahlwirkung", parameter_name: "Richtung_als_Richtung", raw_python_data: "vec![Tuple(vec![NoneValue])]" },
    GeneratorColumnSpec { family: GeneratorFamily::BoolAndTupleSet1, parameter_main: "Primzahlwirkung", parameter_name: "Galaxieabsicht", raw_python_data: "vec![Tuple(vec![Int(10)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::BoolAndTupleSet1, parameter_main: "Primzahlwirkung", parameter_name: "Absicht_Reziproke_Galaxie", raw_python_data: "vec![Tuple(vec![Int(42)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::BoolAndTupleSet1, parameter_main: "Primzahlwirkung", parameter_name: "Universum_Reziproke", raw_python_data: "vec![Tuple(vec![Int(131)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::BoolAndTupleSet1, parameter_main: "Primzahlwirkung", parameter_name: "Dagegen-Gegentranszendentalie", raw_python_data: "vec![Tuple(vec![Int(138)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::BoolAndTupleSet1, parameter_main: "Primzahlwirkung", parameter_name: "neutrale_Gegentranszendentalie", raw_python_data: "vec![Tuple(vec![Int(202)])]" },
];

pub const PYTHON_GENERATED2_SPECS: &[GeneratorColumnSpec] = &[
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Wichtigstes_zum_verstehen", parameter_name: "Motive_Sternpolygone", raw_python_data: "vec![Str(\"primMotivStern\")]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Grundstrukturen", parameter_name: "nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)", raw_python_data: "vec![Str(\"primzahlkreuzprocontra\")]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Pro_Contra", parameter_name: "Primzahlkreuz_pro_contra", raw_python_data: "vec![Str(\"primzahlkreuzprocontra\")]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Bedeutung", parameter_name: "Primzahlkreuz_pro_contra", raw_python_data: "vec![Str(\"primzahlkreuzprocontra\")]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Primvielfache", parameter_name: "Motive_gleichförmige_Polygone", raw_python_data: "vec![Str(\"primMotivGleichf\")]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Primvielfache", parameter_name: "Struktur_gleichförmige_Polygone", raw_python_data: "vec![Str(\"primStrukGleichf\")]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Primvielfache", parameter_name: "Motive_Sternpolygone", raw_python_data: "vec![Str(\"primMotivStern\")]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Primvielfache", parameter_name: "Struktur_Sternpolygone", raw_python_data: "vec![Str(\"primStrukStern\")]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Primvielfache", parameter_name: "Motiv_Sternpolygon_gebrochen-rational", raw_python_data: "vec![Str(\"primMotivSternGebr\")]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Primvielfache", parameter_name: "Struktur_Sternpolyon_gebrochen-rational", raw_python_data: "vec![Str(\"primStrukSternGebr\")]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Primvielfache", parameter_name: "Motiv_gleichförmige_Polygone_gebrochen-rational", raw_python_data: "vec![Str(\"primMotivGleichfGebr\")]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Primvielfache", parameter_name: "Struktur_gleichförmige_Polygone_gebrochen-rational", raw_python_data: "vec![Str(\"primStrukGleichfGebr\")]" },
    GeneratorColumnSpec { family: GeneratorFamily::Generated2, parameter_main: "Primvielfache", parameter_name: "beschrieben", raw_python_data: "vec![Str(\"PrimCSV\")]" },
];

pub const PYTHON_METAKONKRET_SPECS: &[GeneratorColumnSpec] = &[
    GeneratorColumnSpec { family: GeneratorFamily::Metakonkret, parameter_main: "Meta_vs_Konkret_(Universum)", parameter_name: "meta", raw_python_data: "vec![Tuple(vec![Int(2), Int(0)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Metakonkret, parameter_main: "Meta_vs_Konkret_(Universum)", parameter_name: "konkret", raw_python_data: "vec![Tuple(vec![Int(2), Int(1)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Metakonkret, parameter_main: "Meta_vs_Konkret_(Universum)", parameter_name: "Theorie", raw_python_data: "vec![Tuple(vec![Int(3), Int(0)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Metakonkret, parameter_main: "Meta_vs_Konkret_(Universum)", parameter_name: "Praxis", raw_python_data: "vec![Tuple(vec![Int(3), Int(1)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Metakonkret, parameter_main: "Meta_vs_Konkret_(Universum)", parameter_name: "Management", raw_python_data: "vec![Tuple(vec![Int(4), Int(0)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Metakonkret, parameter_main: "Meta_vs_Konkret_(Universum)", parameter_name: "verändernd", raw_python_data: "vec![Tuple(vec![Int(4), Int(1)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Metakonkret, parameter_main: "Meta_vs_Konkret_(Universum)", parameter_name: "ganzheitlich", raw_python_data: "vec![Tuple(vec![Int(5), Int(0)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Metakonkret, parameter_main: "Meta_vs_Konkret_(Universum)", parameter_name: "darüber_hinausgehend", raw_python_data: "vec![Tuple(vec![Int(5), Int(1)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Metakonkret, parameter_main: "Meta_vs_Konkret_(Universum)", parameter_name: "Unternehmung_Geschäft", raw_python_data: "vec![Tuple(vec![Int(6), Int(0)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Metakonkret, parameter_main: "Meta_vs_Konkret_(Universum)", parameter_name: "wertvoll", raw_python_data: "vec![Tuple(vec![Int(6), Int(1)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Metakonkret, parameter_main: "Meta_vs_Konkret_(Universum)", parameter_name: "Beherrschen", raw_python_data: "vec![Tuple(vec![Int(7), Int(0)])]" },
    GeneratorColumnSpec { family: GeneratorFamily::Metakonkret, parameter_main: "Meta_vs_Konkret_(Universum)", parameter_name: "Richtung", raw_python_data: "vec![Tuple(vec![Int(7), Int(1)])]" },
];

pub const PYTHON_GEBRO_UNI1_SPECS: &[GeneratorColumnSpec] = &[
    GeneratorColumnSpec { family: GeneratorFamily::GebroUni1, parameter_main: "gebrochen-rational_Universum_n/m", parameter_name: "14", raw_python_data: "vec![Str(\"14\"), Str(\"22\"), Str(\"16\"), Str(\"9\"), Str(\"4\"), Str(\"8\"), Str(\"6\"), Str(\"10\"), Str(\"5\"), Str(\"3\"), Str(\"15\"), Str(\"20\"), Str(\"2\"), Str(\"12\"), Str(\"7\")…" },
];

pub const PYTHON_GEBR_GAL1_SPECS: &[GeneratorColumnSpec] = &[
    GeneratorColumnSpec { family: GeneratorFamily::GebrGal1, parameter_main: "gebrochen-rational_Galaxie_n/m", parameter_name: "14", raw_python_data: "vec![Str(\"14\"), Str(\"22\"), Str(\"16\"), Str(\"9\"), Str(\"4\"), Str(\"8\"), Str(\"6\"), Str(\"10\"), Str(\"5\"), Str(\"3\"), Str(\"15\"), Str(\"20\"), Str(\"2\"), Str(\"12\"), Str(\"7\")…" },
];

pub const PYTHON_GEBR_EMO1_SPECS: &[GeneratorColumnSpec] = &[
    GeneratorColumnSpec { family: GeneratorFamily::GebrEmo1, parameter_main: "gebrochen-rational_Gefuehle_n/m", parameter_name: "14", raw_python_data: "vec![Str(\"14\"), Str(\"22\"), Str(\"16\"), Str(\"9\"), Str(\"4\"), Str(\"8\"), Str(\"6\"), Str(\"10\"), Str(\"5\"), Str(\"3\"), Str(\"15\"), Str(\"20\"), Str(\"2\"), Str(\"12\"), Str(\"7\")…" },
];

pub const PYTHON_GEBR_GROE1_SPECS: &[GeneratorColumnSpec] = &[
    GeneratorColumnSpec { family: GeneratorFamily::GebrGroe1, parameter_main: "gebrochen-rational_Strukturgroesse_n/m", parameter_name: "14", raw_python_data: "vec![Str(\"14\"), Str(\"22\"), Str(\"16\"), Str(\"9\"), Str(\"4\"), Str(\"8\"), Str(\"6\"), Str(\"10\"), Str(\"5\"), Str(\"3\"), Str(\"15\"), Str(\"20\"), Str(\"2\"), Str(\"12\"), Str(\"7\")…" },
];

pub fn all_python_generator_specs() -> Vec<&'static GeneratorColumnSpec> {
    let mut out = Vec::new();
    out.extend(PYTHON_GENERATED1_SPECS.iter());
    out.extend(PYTHON_BOOL_AND_TUPLESET1_SPECS.iter());
    out.extend(PYTHON_GENERATED2_SPECS.iter());
    out.extend(PYTHON_METAKONKRET_SPECS.iter());
    out.extend(PYTHON_GEBRO_UNI1_SPECS.iter());
    out.extend(PYTHON_GEBR_GAL1_SPECS.iter());
    out.extend(PYTHON_GEBR_EMO1_SPECS.iter());
    out.extend(PYTHON_GEBR_GROE1_SPECS.iter());
    out
}

pub fn is_known_python_generator_parameter(parameter_main: &str, parameter_name: &str) -> bool {
    all_python_generator_specs().into_iter().any(|spec| spec.parameter_main == parameter_main && spec.parameter_name == parameter_name)
}

pub fn generator_family_counts() -> &'static [(GeneratorFamily, usize)] {
    &[
        (GeneratorFamily::Generated1, 50),
        (GeneratorFamily::BoolAndTupleSet1, 8),
        (GeneratorFamily::Generated2, 13),
        (GeneratorFamily::Metakonkret, 12),
        (GeneratorFamily::GebroUni1, 1),
        (GeneratorFamily::GebrGal1, 1),
        (GeneratorFamily::GebrEmo1, 1),
        (GeneratorFamily::GebrGroe1, 1),
    ]
}