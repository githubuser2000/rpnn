#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortingStatus {
    Ported,
    PartiallyPorted,
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratorGap {
    pub python_name: &'static str,
    pub rust_target: &'static str,
    pub status: PortingStatus,
    pub note: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EigenschaftGap {
    pub canonical_name: &'static str,
    pub aliases: &'static [&'static str],
    pub has_fixed_columns_in_rust: bool,
    pub requires_generator_port: bool,
    pub note: &'static str,
}

pub const MISSING_PYTHON_GENERATORS_IN_RUST: &[GeneratorGap] = &[
    GeneratorGap {
        python_name: "concat1RowPrimUniverse2",
        rust_target: "concat1_row_prim_universe2",
        status: PortingStatus::Missing,
        note: "Prim-Universum-Generator fehlt als echte Rust-Funktion.",
    },
    GeneratorGap {
        python_name: "spalteMetaKontretTheorieAbstrakt_etc_1",
        rust_target: "spalte_meta_konkret_theorie_abstrakt_etc_1",
        status: PortingStatus::Missing,
        note: "Meta/Konkret/Theorie/Abstrakt-Dispatcher fehlt.",
    },
    GeneratorGap {
        python_name: "spalteMetaKonkretAbstrakt_isGanzZahlig",
        rust_target: "spalte_meta_konkret_abstrakt_is_ganzzahlig",
        status: PortingStatus::Missing,
        note: "Hilfsfunktion fuer Meta/Konkret/Theorie/Abstrakt fehlt.",
    },
    GeneratorGap {
        python_name: "spalteMetaKontretTheorieAbstrakt_etc",
        rust_target: "spalte_meta_konkret_theorie_abstrakt_etc",
        status: PortingStatus::Missing,
        note: "Zentrale Generatorlogik der Meta-Familie fehlt.",
    },
    GeneratorGap {
        python_name: "spalteMetaKonkretTheorieAbstrakt_SetHtmlParameters",
        rust_target: "spalte_meta_konkret_theorie_abstrakt_set_html_parameters",
        status: PortingStatus::Missing,
        note: "HTML/Formatierungsparameter fuer Meta-Generator fehlen.",
    },
    GeneratorGap {
        python_name: "spalteMetaKonkretTheorieAbstrakt_mainPart",
        rust_target: "spalte_meta_konkret_theorie_abstrakt_main_part",
        status: PortingStatus::Missing,
        note: "Hauptlogik der Meta/Konkret/Theorie/Abstrakt-Erzeugung fehlt.",
    },
    GeneratorGap {
        python_name: "spalteMetaKonkretTheorieAbstrakt_VorwortBehandlungWieVorwortMeta",
        rust_target: "spalte_meta_konkret_theorie_abstrakt_vorwort_behandlung_wie_vorwort_meta",
        status: PortingStatus::Missing,
        note: "Vorwort-/Meta-Behandlung fehlt.",
    },
    GeneratorGap {
        python_name: "spalteMetaKonkretTheorieAbstrakt_mainPart_InsertingText",
        rust_target: "spalte_meta_konkret_theorie_abstrakt_main_part_inserting_text",
        status: PortingStatus::Missing,
        note: "Texteinsetzungslogik der Meta-Familie fehlt.",
    },
    GeneratorGap {
        python_name: "spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie",
        rust_target: "spalte_meta_konkret_theorie_abstrakt_get_gebr_rat_univ_strukturalie",
        status: PortingStatus::Missing,
        note: "Lookup-/Textaufbereitung fuer Gebr/Rat/Univ/Strukturalie fehlt.",
    },
    GeneratorGap {
        python_name: "spalteMetaKonkretAbstrakt_UeberschriftenUndTags",
        rust_target: "spalte_meta_konkret_abstrakt_ueberschriften_und_tags",
        status: PortingStatus::Missing,
        note: "Ueberschriften-/Tag-Initialisierung der Meta-Familie fehlt.",
    },
    GeneratorGap {
        python_name: "spalteFuerGegenInnenAussenSeitlichPrim",
        rust_target: "spalte_fuer_gegen_innen_aussen_seitlich_prim",
        status: PortingStatus::Missing,
        note: "Gegen/Innen/Aussen/Seitlich/Prim-Generator fehlt.",
    },
    GeneratorGap {
        python_name: "createSpalteGestirn",
        rust_target: "create_spalte_gestirn",
        status: PortingStatus::Missing,
        note: "Gestirn-Spalte aus tableHandling.py fehlt als Rust-Port.",
    },
];

pub const PARTIALLY_PORTED_FOUNDATION_FUNCTIONS: &[GeneratorGap] = &[
    GeneratorGap {
        python_name: "could_be_prime_number_primzahlkreuz",
        rust_target: "could_be_prime_number_primzahlkreuz",
        status: PortingStatus::PartiallyPorted,
        note: "Im aktiven Registry-File implementiert, in lib4tables_concat.rs aber als unimplemented!() vorhanden.",
    },
    GeneratorGap {
        python_name: "could_be_prime_number_primzahlkreuz_fuer_innen",
        rust_target: "could_be_prime_number_primzahlkreuz_fuer_innen",
        status: PortingStatus::PartiallyPorted,
        note: "Inkonsistenter Portierungsstand zwischen Dateien.",
    },
    GeneratorGap {
        python_name: "could_be_prime_number_primzahlkreuz_fuer_aussen",
        rust_target: "could_be_prime_number_primzahlkreuz_fuer_aussen",
        status: PortingStatus::PartiallyPorted,
        note: "Inkonsistenter Portierungsstand zwischen Dateien.",
    },
    GeneratorGap {
        python_name: "prim_creativity",
        rust_target: "prim_creativity",
        status: PortingStatus::PartiallyPorted,
        note: "Im Registry-File implementiert, im Concat-File noch unimplemented!().",
    },
    GeneratorGap {
        python_name: "moon_number",
        rust_target: "moon_number",
        status: PortingStatus::PartiallyPorted,
        note: "Im Registry-File implementiert, im Concat-File noch unimplemented!().",
    },
    GeneratorGap {
        python_name: "prim_multiple",
        rust_target: "prim_multiple",
        status: PortingStatus::PartiallyPorted,
        note: "Im Registry-File implementiert, im Concat-File noch unimplemented!().",
    },
];

pub const EIGENSCHAFTEN_GENERATOR_GAPS: &[EigenschaftGap] = &[
    EigenschaftGap { canonical_name: "Dein_Recht_bekommen", aliases: &["rechte", "recht", "selbstgerecht"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Alias vorhanden, aber keine feste Spaltenzuordnung und keine Generatorlogik." },
    EigenschaftGap { canonical_name: "unterlegen_ueberlegen", aliases: &["unterlegen", "ueberlegen"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Nur Kategorieebene sichtbar." },
    EigenschaftGap { canonical_name: "Ehrlichkeit_und_Streit", aliases: &["streit", "ehrlichkeit"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Dynamische Erzeugung fehlt." },
    EigenschaftGap { canonical_name: "Wuerdig", aliases: &["wuerdig", "wuerde"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Nur Aliasfamilie vorhanden." },
    EigenschaftGap { canonical_name: "Regel_vs_Ausnahme", aliases: &["regel", "ausnahme"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Generator fehlt." },
    EigenschaftGap { canonical_name: "Werte", aliases: &["werte"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Nur Wortebene." },
    EigenschaftGap { canonical_name: "Gutartigkeits_Egoismus", aliases: &["position", "gutesreziprok", "egozentrik"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Keine feste oder generierte Rust-Spalte." },
    EigenschaftGap { canonical_name: "Reflektieren_Erkenntnis_Erkennen", aliases: &["reflektieren", "erkenntnis"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Nur Klassifikation portiert." },
    EigenschaftGap { canonical_name: "Vertrauen_wollen", aliases: &["vertrauenwollen"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Generator fehlt." },
    EigenschaftGap { canonical_name: "einklinken_vertrauen_anprangern", aliases: &["einklinken", "vertrauenerhalten", "anprangern"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Nur Aliasfamilie portiert." },
    EigenschaftGap { canonical_name: "Ausrichten_Einrichten", aliases: &["einrichten", "ausrichten"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Keine effektive Rust-Erzeugung." },
    EigenschaftGap { canonical_name: "Toleranz_Respekt_Akzeptanz_Willkommen", aliases: &["toleranz", "respekt", "akzeptanz", "willkommen"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Nur Kategorien vorhanden." },
    EigenschaftGap { canonical_name: "familiebrauchen", aliases: &["familiebrauchen"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Offene Eigenschaft ohne Spaltenhinterlegung." },
    EigenschaftGap { canonical_name: "ego", aliases: &["ego", "bescheiden"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Offene Eigenschaft ohne Generator." },
    EigenschaftGap { canonical_name: "Selbstsucht_Ichsucht_etc", aliases: &["selbstsucht", "ichsucht"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Eigenschaftsfamilie nicht portiert." },
    EigenschaftGap { canonical_name: "Forschen_Erfinden_Einklinken", aliases: &["wissenschaft", "forschen", "einklinken", "erfinden"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Nur Alias- und Kategorienebene." },
    EigenschaftGap { canonical_name: "Kooperation_vs_Arsch", aliases: &["arschloch", "kooperation", "arsch"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Keine feste Spalte in Rust." },
    EigenschaftGap { canonical_name: "Liebe_usw", aliases: &["liebe", "zuneigung"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Wichtig: trotz anderer Liebesgeneratoren fehlt diese Eigenschaften-Familie als eigene Generierung." },
    EigenschaftGap { canonical_name: "Selbstlosigkeit_Ichlosigkeit_etc", aliases: &["selbstlos", "ichlos"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Nur Oberflaeche portiert." },
    EigenschaftGap { canonical_name: "variationsreich_eintoenig", aliases: &["eintoenig", "variationsreich"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Keine feste Zuordnung, kein Dispatcher." },
    EigenschaftGap { canonical_name: "Zuneigung_Abneigung", aliases: &["abgeneigt", "zugewandt", "reserviert", "zugeneigt"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Erzeugungslogik fehlt." },
    EigenschaftGap { canonical_name: "ehrlich_vs_hoeflich", aliases: &["ehrlich", "hoeflich"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Fehlende Generatorik." },
    EigenschaftGap { canonical_name: "Tragweite", aliases: &["tragweite"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Noch offen." },
    EigenschaftGap { canonical_name: "wertvoll", aliases: &["wertlos"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Explizit vom Nutzer genanntes Loch: Eigenschaften-Generierung fehlt." },
    EigenschaftGap { canonical_name: "Goetter_Propheten_Familien_Freunde", aliases: &["familiaer", "goettlich", "freunde", "propheten"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Nur Aliasfamilie vorhanden." },
    EigenschaftGap { canonical_name: "sanft_vs_hart", aliases: &["sanft", "hart"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Keine Rust-Generierung." },
    EigenschaftGap { canonical_name: "vereinen_vs_verbinden", aliases: &["vereinenverbinden", "vereinen", "verbinden", "einheit", "verbindung"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Keine feste oder generierte Spaltenlogik." },
    EigenschaftGap { canonical_name: "Zeit_vs_Raum", aliases: &["zeit", "raum", "zeitlich", "raeumlich"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Wort vorhanden, Logik fehlt." },
    EigenschaftGap { canonical_name: "egalitaer_vs_autoritaer", aliases: &["egalitaerautoritaer", "egalitaer", "autoritaer"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Keine Generatorik." },
    EigenschaftGap { canonical_name: "Meinungen_und_Ruf", aliases: &["meinungen", "anderemenschen", "ruf"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Nur Aliasfamilie, keine dynamische Erzeugung." },
    EigenschaftGap { canonical_name: "Meinungsintelligenz", aliases: &["meinungsintelligenz", "ursprungsintelligenz"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Noch offen." },
    EigenschaftGap { canonical_name: "Sittlichkeit", aliases: &["sittlichkeit", "annaehrerung"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Noch offen." },
    EigenschaftGap { canonical_name: "Fuehrung", aliases: &["fuehrung"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Noch offen." },
    EigenschaftGap { canonical_name: "Durchleuchten", aliases: &["durchleuchten", "erleuchten"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Noch offen." },
    EigenschaftGap { canonical_name: "Foerdern_Sensiblisieren_und_Gedeihen", aliases: &["foerdern", "sensibilisieren", "gedeihen", "verderben"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Noch offen." },
    EigenschaftGap { canonical_name: "Ueberheblichkeit", aliases: &["ueberheblichkeit", "ueberheblich"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Noch offen." },
    EigenschaftGap { canonical_name: "Polung_der_Liebe", aliases: &["liebepolung"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Noch offen." },
    EigenschaftGap { canonical_name: "kausal", aliases: &["geltung", "genese"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Noch offen." },
    EigenschaftGap { canonical_name: "Gleichheit", aliases: &["gleich"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Als Eigenschaftsfamilie offen, auch wenn es einen separaten Gleichheitsgenerator gibt." },
    EigenschaftGap { canonical_name: "Ueberleben", aliases: &["ueberleben"], has_fixed_columns_in_rust: false, requires_generator_port: true, note: "Noch offen." },
];

pub const EIGENSCHAFTEN_WITH_FIXED_COLUMNS_IN_RUST: &[EigenschaftGap] = &[
    EigenschaftGap { canonical_name: "Weisheit_etc", aliases: &["weisheit", "metaweisheit", "meta-weisheit", "idiot", "weise", "optimal", "optimum"], has_fixed_columns_in_rust: true, requires_generator_port: false, note: "Feste Spaltenzuordnung vec![112]." },
    EigenschaftGap { canonical_name: "Filterart_Widrigkeit", aliases: &["filterartwidrigkeit", "filter", "widrigkeit"], has_fixed_columns_in_rust: true, requires_generator_port: false, note: "Feste Spaltenzuordnung vec![331, 335]." },
    EigenschaftGap { canonical_name: "aehnlich", aliases: &["aehnlich"], has_fixed_columns_in_rust: true, requires_generator_port: false, note: "Feste Spaltenzuordnung vec![220]." },
    EigenschaftGap { canonical_name: "gut_boese_lieb_schlecht", aliases: &["gut", "boese", "lieb", "schlecht"], has_fixed_columns_in_rust: true, requires_generator_port: false, note: "Feste Spaltenzuordnung vec![52, 53]." },
    EigenschaftGap { canonical_name: "Sinn_und_Zweck_des_Lebens", aliases: &["sinn", "zweck", "bedeutung"], has_fixed_columns_in_rust: true, requires_generator_port: false, note: "Feste Spaltenzuordnung vec![88, 189]." },
    EigenschaftGap { canonical_name: "Egoismus_vs_Altruismus", aliases: &["egoismus", "altruismus", "egoist", "altruist"], has_fixed_columns_in_rust: true, requires_generator_port: false, note: "Feste Spaltenzuordnung vec![136]." },
];

pub fn missing_python_generator_names() -> Vec<&'static str> {
    MISSING_PYTHON_GENERATORS_IN_RUST
        .iter()
        .map(|gap| gap.python_name)
        .collect()
}

pub fn missing_eigenschaften_names() -> Vec<&'static str> {
    EIGENSCHAFTEN_GENERATOR_GAPS
        .iter()
        .map(|gap| gap.canonical_name)
        .collect()
}

pub fn consolidated_generator_gap_report() -> String {
    let mut out = String::new();

    out.push_str("Fehlende Python-Generatoren in Rust:\n");
    for gap in MISSING_PYTHON_GENERATORS_IN_RUST {
        out.push_str("- ");
        out.push_str(gap.python_name);
        out.push_str(" -> ");
        out.push_str(gap.rust_target);
        out.push_str(" :: ");
        out.push_str(gap.note);
        out.push('\n');
    }

    out.push_str("\nTeilportierte Basisfunktionen:\n");
    for gap in PARTIALLY_PORTED_FOUNDATION_FUNCTIONS {
        out.push_str("- ");
        out.push_str(gap.python_name);
        out.push_str(" :: ");
        out.push_str(gap.note);
        out.push('\n');
    }

    out.push_str("\nOffene Eigenschaften-Generatoren:\n");
    for gap in EIGENSCHAFTEN_GENERATOR_GAPS {
        out.push_str("- ");
        out.push_str(gap.canonical_name);
        out.push_str(" :: ");
        out.push_str(gap.note);
        out.push('\n');
    }

    out
}
