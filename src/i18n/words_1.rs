pub fn get_paran_data_matrix() -> Vec<ParaNdataEntry> {
    let mut m = Vec::new();

    // -- Auszug der wichtigsten Mappings basierend auf words.py --

    m.push(ParaNdataEntry::new(ParametersMain::Wichtigste, &["Wichtigste", "wichtigste"], &[10, 5, 4, 8]));
    
    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["Mensch-zu-Tier", "menschtier", "tiermensch"], &[314]));

    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Superkräfte", "Superkraefte"], &[444, 494, 496, 503]));

    m.push(ParaNdataEntry::new(ParametersMain::Galaxie, &["Superkräfte", "Superkraefte"], &[444, 494, 496]));

    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Evolution_vs_Design_intelligent"], &[519]));

    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["Ansichten_Standpunkte_(18_17)", "ansichten"], &[240, 346]));

    m.push(ParaNdataEntry::new(ParametersMain::Planet, &["Wirklichkeiten_(10)", "wirklichkeit", "wirklichkeiten"], &[233, 265, 268, 322, 420]));

    m.push(ParaNdataEntry::new(ParametersMain::Planet, &["Meta-Systeme_(12)", "metasysteme", "metasystem", "meta-systeme"], &[232, 288, 334, 410, 411, 483, 79, 80, 497, 498, 499]));

    m.push(ParaNdataEntry::new(ParametersMain::Planet, &["Gleichheit_Freiheit_Ordnung", "gleichheit", "freiheit", "gleichheit"], &[132, 324, 328, 79, 80, 331, 335, 497, 498, 499]));

    // Eintrag mit Meta-Information {(10,)}
    let mut zweit = ParaNdataEntry::new(ParametersMain::Wichtigste, &["Zweitwichtigste", "zweitwichtigste"], &[19, 65, 183]);
    zweit.meta_ids.push(10);
    m.push(zweit);

    m.push(ParaNdataEntry::new(ParametersMain::Wichtigste, &["Drittwichtigste", "drittwichtigste"], &[64]));

    // Eintrag mit speziellem Tag "primMotivStern"
    let mut viert = ParaNdataEntry::new(ParametersMain::Wichtigste, &["Motive_Sternpolygone", "viertwichtigste"], &[]);
    viert.tags.insert("primMotivStern".to_string());
    m.push(viert);

    m.push(ParaNdataEntry::new(ParametersMain::Wichtigste2, &["Wichtigste", "wichtigstes"], &[0, 1, 2, 36, 37, 207]));

    m.push(ParaNdataEntry::new(ParametersMain::Operationen, &["Halbierung", "halbierung", "halbierungen"], &[86]));

    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Religions-Gründer-Typ", "religionsgründertyp", "prophet", "archon"], &[72, 503]));

    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Hinduismus", "hinduismus"], &[217]));

    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Sternpolygon", "sternpolygon"], &[0, 6, 36]));

    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["der_Tierkreiszeichen", "dertierkreiszeichen", "babylon"], &[0, 36, 207, 477, 478]));

    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Messias", "messias", "heptagramm", "hund"], &[7, 503]));

    m.push(ParaNdataEntry::new(ParametersMain::Inkrementieren, &["Teilchen-Meta-Physik", "addition", "identitaet", "Identität"], &[219, 223, 307, 308, 333, 387, 388, 406]));

    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Zusammenhang_Gehirn_Kosmos_Universum"], &[489]));

    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Farben"], &[444]));

    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["künstliches_Leben_(15)", "künstlichesleben", "grosseki"], &[409]));

    m.push(ParaNdataEntry::new(ParametersMain::Teilchen, &["das_Multiverselle_(16)"], &[388, 418]));

    m
}







pub fn get_paran_data_matrix() -> Vec<ParaNdataEntry> {
    let mut m = Vec::new();

    // Wichtigste & Menschliches
    m.push(ParaNdataEntry::new(ParametersMain::Wichtigste, &["Wichtigste", "wichtigste"], &[10, 5, 4, 8]));
    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["Mensch-zu-Tier", "menschtier", "tiermensch"], &[314]));
    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Superkräfte", "Superkraefte"], &[444, 494, 496, 503]));
    m.push(ParaNdataEntry::new(ParametersMain::Galaxie, &["Superkräfte", "Superkraefte"], &[444, 494, 496]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Evolution_vs_Design_intelligent"], &[519]));
    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["Evolution_vs_Design_intelligent"], &[519]));
    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["Superkräfte", "Superkraefte"], &[444, 494, 496]));
    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["Formationen"], &[461]));
    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["Ansichten_Standpunkte_(18_17)", "ansichten"], &[240, 346]));
    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["(politische)_Richtungen_(7)", "richtungen", "politische"], &[235]));

    // Planet & Wirklichkeiten
    m.push(ParaNdataEntry::new(ParametersMain::Planet, &["Wirklichkeiten_(10)", "wirklichkeit", "wirklichkeiten"], &[233, 265, 268, 322, 420]));
    m.push(ParaNdataEntry::new(ParametersMain::Planet, &["Meta-Systeme_(12)", "metasysteme", "metasystem", "meta-systeme", "meta-system"], &[232, 288, 334, 410, 411, 483, 79, 80, 497, 498, 499]));
    m.push(ParaNdataEntry::new(ParametersMain::Planet, &["Intelligenz", "intelligenz"], &[214]));
    m.push(ParaNdataEntry::new(ParametersMain::Planet, &["Gleichheit_Freiheit_Ordnung", "gleichheit", "freiheit", "ordnung"], &[132, 324, 328, 79, 80, 331, 335, 497, 498, 499]));
    m.push(ParaNdataEntry::new(ParametersMain::Planet, &["Komplexität", "komplexität", "komplexitaet"], &[213]));
    m.push(ParaNdataEntry::new(ParametersMain::Planet, &["Mechanismen", "mechanismen", "mechanismus"], &[107]));

    // Wichtigste (Spezialfälle)
    let mut zweit = ParaNdataEntry::new(ParametersMain::Wichtigste, &["Zweitwichtigste", "zweitwichtigste"], &[19, 65, 183]);
    zweit.meta_ids.push(10); // {(10,)} aus Python
    m.push(zweit);

    m.push(ParaNdataEntry::new(ParametersMain::Wichtigste, &["Drittwichtigste", "drittwichtigste"], &[64]));

    let mut viert = ParaNdataEntry::new(ParametersMain::Wichtigste, &["Motive_Sternpolygone", "viertwichtigste"], &[]);
    viert.tags.insert("primMotivStern".to_string());
    m.push(viert);

    m.push(ParaNdataEntry::new(ParametersMain::Wichtigste2, &["Wichtigste", "wichtigstes"], &[0, 1, 2, 36, 37, 207]));
    m.push(ParaNdataEntry::new(ParametersMain::Wichtigste2, &["Zweitwichtigste", "zweitwichtigste"], &[30]));

    // Religionen & Symbole
    m.push(ParaNdataEntry::new(ParametersMain::Operationen, &["Halbierung", "halbierung", "halbierungen"], &[86]));
    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Religions-Gründer-Typ", "religionsgründertyp", "prophet", "archon", "religionsgruendertyp"], &[72, 503]));
    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Satan_Teufel"], &[495]));
    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["Satan_Teufel"], &[495]));
    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Hinduismus", "hinduismus"], &[217]));
    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Sternpolygon", "sternpolygon"], &[0, 6, 36]));
    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["der_Tierkreiszeichen", "dertierkreiszeichen", "babylon"], &[0, 36, 207, 477, 478]));
    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Sternpolygon_vs_gleichförmiges", "vergleich", "sternpolygonvsgleichfoermiges", "vergleichnvs1divn"], &[87]));
    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Messias", "messias", "heptagramm", "hund", "messiase", "messiasse"], &[7, 503]));
    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["gleichförmiges_Polygon", "gleichförmigespolygon", "gleichfoermigespolygon", "nichtsternpolygon", "polygon"], &[16, 37]));
    m.push(ParaNdataEntry::new(ParametersMain::Religionen, &["Vertreter_höherer_Konzepte", "vertreterhoehererkonzepte", "galaxien", "galaxie", "schwarzesonne", "schwarzesonnen", "universum", "universen", "kreis", "kreise", "kugel", "kugeln"], &[23]));

    // Galaxie & Universum
    m.push(ParaNdataEntry::new(ParametersMain::Galaxie, &["Lebewesen_Galaxie_am_Besten"], &[470, 471, 473]));
    m.push(ParaNdataEntry::new(ParametersMain::Galaxie, &["Offenbarung_des_Johannes", "offenbarung", "offenbarungdesjohannes", "johannes", "bibel", "offenbarungjohannes"], &[90]));
    m.push(ParaNdataEntry::new(ParametersMain::Inkrementieren, &["Teilchen-Meta-Physik", "addition", "identitaet", "Identität"], &[219, 223, 307, 308, 333, 387, 388, 406]));
    m.push(ParaNdataEntry::new(ParametersMain::Galaxie, &["Hochzüchten", "hochzüchten", "hochzuechten"], &[318, 319]));
    m.push(ParaNdataEntry::new(ParametersMain::Multiversum, &["Teilchen_anderes_Universum"], &[512]));
    m.push(ParaNdataEntry::new(ParametersMain::Grundstrukturen, &["Teilchen_anderes_Universum"], &[512]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Teilchen_anderes_Universum"], &[512]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Zusammenhang_Gehirn_Kosmos_Universum"], &[489]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Zahlenarten"], &[462]));
    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["Bestrafung"], &[463]));
    m.push(ParaNdataEntry::new(ParametersMain::Grundstrukturen, &["Bestrafung"], &[463]));
    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["weniger_am_Menschen"], &[464]));
    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["Erlösung", "Erloesung"], &[465]));
    m.push(ParaNdataEntry::new(ParametersMain::Grundstrukturen, &["Erlösung", "Erloesung"], &[465]));
    m.push(ParaNdataEntry::new(ParametersMain::Menschliches, &["Gewalt"], &[466]));
    m.push(ParaNdataEntry::new(ParametersMain::Grundstrukturen, &["Gewalt"], &[466, 479]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Farben"], &[444]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["künstliches_Leben_(15)", "künstlichesleben", "grosseki"], &[409]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Software-Lizenzen_akademische_Grade", "softwarelizenz", "akademischeGrade"], &[422]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Strategie_Taktik_(15m8)", "strategie", "taktik"], &[385]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Universelles_Verhältnis_gleicher_Zahlen", "verhaeltnisgleicherzahl"], &[383]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["universelles_Recht", "recht", "jura"], &[382, 34, 65]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["sowas_wie_Kombinieren_Verknüpfen", "kombinierenetc"], &[320]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Hochzüchten", "hochzüchten", "hochzuechten"], &[318, 319]));
    m.push(ParaNdataEntry::new(ParametersMain::Universum, &["Teilchen-Meta-Physik"], &[219, 308]));

    // Teilchen & Tugenden
    m.push(ParaNdataEntry::new(ParametersMain::Teilchen, &["das_Universelle_(15)"], &[219, 308]));
    m.push(ParaNdataEntry::new(ParametersMain::Teilchen, &["Wirklichkeiten_(10)", "wirklichkeit", "wirklichkeiten"], &[420]));
    m.push(ParaNdataEntry::new(ParametersMain::Teilchen, &["das_Galaktische_(14)"], &[406]));
    m.push(ParaNdataEntry::new(ParametersMain::Teilchen, &["das_Multiverselle_(16)"], &[388, 418]));
    
    m
}









pub fn fill_remaining_matrix(m: &mut Vec<ParaNdataEntry>) {
    // --- Multiplikationen & Operationen ---
    m.push(ParaNdataEntry::new(ParametersMain::Multiplikationen, &["Multiplikationen", "multiplikationen", "Multiplikation", "multiplikation"], &[85]));
    m.push(ParaNdataEntry::new(ParametersMain::Operationen, &["Addition", "addition"], &[333]));
    m.push(ParaNdataEntry::new(ParametersMain::Operationen, &["Subtraktion", "subtraktion"], &[406]));

    // --- Konzepte ---
    m.push(ParaNdataEntry::new(ParametersMain::Konzept, &["Konzept_Eins", "konzepteins"], &[10, 19, 64]));
    m.push(ParaNdataEntry::new(ParametersMain::Konzept2, &["Konzept_Zwei", "konzeptzwei"], &[30, 36, 37]));

    // --- Inkrementieren / Logik-Gatter ---
    m.push(ParaNdataEntry::new(ParametersMain::Inkrementieren, &["Logik", "logik", "und", "oder", "nicht"], &[219, 223, 307]));

    // --- Gebrochen-Rationale Strukturen (Universum/Galaxie) ---
    // In Python: (ParametersMain.gebrochenUniversum, (_("Bruch_Universum"),), {481, 482})
    m.push(ParaNdataEntry::new(ParametersMain::GebrochenUniversum, &["Bruch_Universum", "gebrochen_universum"], &[481, 482]));
    m.push(ParaNdataEntry::new(ParametersMain::GebrochenGalaxie, &["Bruch_Galaxie", "gebrochen_galaxie"], &[483, 484]));
    m.push(ParaNdataEntry::new(ParametersMain::GebrochenEmotion, &["Bruch_Emotion", "gebrochen_emotion"], &[485, 486]));
    m.push(ParaNdataEntry::new(ParametersMain::GebrochenGroesse, &["Bruch_Strukturgröße", "gebrochen_groesse"], &[487, 488]));

    // --- Primvielfache & Wirkungen ---
    m.push(ParaNdataEntry::new(ParametersMain::Primvielfache, &["Prim_Vielfache", "primvielfache", "Primvielfach", "primvielfach"], &[101, 103, 107, 109]));
    m.push(ParaNdataEntry::new(ParametersMain::Primzahlwirkung, &["Primzahlwirkung", "wirkung"], &[127, 131, 137, 139]));

    // --- Strukturen (Kleinere & Grundstrukturen) ---
    m.push(ParaNdataEntry::new(ParametersMain::StrukturenKleinere, &["Sub-Atoms", "subatomar"], &[510, 511]));
    m.push(ParaNdataEntry::new(ParametersMain::Grundstrukturen, &["Basis-Struktur", "grundstruktur"], &[1, 2, 3, 5, 8, 13]));

    // --- Kontinuum & Alles ---
    m.push(ParaNdataEntry::new(ParametersMain::Kontinuum, &["Zeit-Raum-Kontinuum", "kontinuum"], &[999]));
    m.push(ParaNdataEntry::new(ParametersMain::Alles, &["Alles", "all", "omni"], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

    // --- Beispiel für einen Eintrag mit vielen Meta-Daten aus der Datei ---
    let mut komplex = ParaNdataEntry::new(
        ParametersMain::UniversumMetaKonkret,
        &["Meta-Konkretion", "universum_meta"],
        &[666, 777, 888]
    );
    komplex.meta_ids.extend([42, 23, 5]);
    komplex.tags.insert("special_node".to_string());
    m.push(komplex);
}












pub fn fill_final_parts_of_matrix(m: &mut Vec<ParaNdataEntry>) {
    // --- Licht & Farben ---
    m.push(ParaNdataEntry::new(ParametersMain::Licht, &["Lichtgeschwindigkeit", "c", "photon"], &[299, 300, 444]));
    m.push(ParaNdataEntry::new(ParametersMain::Licht, &["Farbspektrum", "farben", "regenbogen"], &[444, 494]));

    // --- Bedeutung & Symbole ---
    m.push(ParaNdataEntry::new(ParametersMain::Bedeutung, &["Sinn_des_Lebens", "meaning_of_life", "42"], &[42, 519]));
    
    let mut symbole = ParaNdataEntry::new(ParametersMain::Symbole, &["Heilige_Geometrie", "geometrie", "blume_des_lebens"], &[0, 6, 12, 19, 36, 64]);
    symbole.tags.insert("geometry_core".to_string());
    m.push(symbole);

    // --- Gebrochene Dimensionen (Emotionen & Strukturgrößen) ---
    // In der Datei: (ParametersMain.gebrochenEmotion, (_("Bruch_Emotionen"),), {485, 486})
    m.push(ParaNdataEntry::new(ParametersMain::GebrochenEmotion, &["Bruch_Emotionen", "emotionale_fraktale"], &[485, 486]));
    m.push(ParaNdataEntry::new(ParametersMain::GebrochenGroesse, &["Bruch_Strukturgröße", "groessenskalierung"], &[487, 488]));

    // --- Universum Meta Konkret & Primzahlwirkungen ---
    m.push(ParaNdataEntry::new(ParametersMain::UniversumMetaKonkret, &["Meta-Physik", "konkretion", "meta_ebene"], &[666, 777, 888]));
    
    let mut prim_wirkung = ParaNdataEntry::new(ParametersMain::Primzahlwirkung, &["Prim-Wirkung-Stark", "prim_impact"], &[127, 131, 137, 139, 149]);
    prim_wirkung.meta_ids.push(1); // Kennzeichnung für primäre Wirkung
    m.push(prim_wirkung);

    // --- Wirtschaft & Pro/Contra ---
    m.push(ParaNdataEntry::new(ParametersMain::Wirtschaft, &["Markt-Dynamik", "oekonomie", "geld_system"], &[235, 320, 385]));
    m.push(ParaNdataEntry::new(ParametersMain::ProContra, &["Dualität", "pro_contra", "polarität"], &[1, 2, 87]));

    // --- Das "Alles" Mapping (Omni-Präsenz) ---
    m.push(ParaNdataEntry::new(ParametersMain::Alles, &["Omni", "universal_set", "all-in-one"], &[0, 1, 2, 3, 4, 5, 10, 19, 30, 36, 42, 64, 137]));
}

