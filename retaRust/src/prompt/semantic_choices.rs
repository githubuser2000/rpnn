//! Python-exakte Prompt-Auswahldaten für `15_...`, `16_...` und `16_15_...`.
//!
//! Diese Konstanten entsprechen `i18n.words.wahl15`/`wahl16` plus der
//! Prompt-spezifischen Mutation aus `retaPrompt.py`:
//! `wahl15[""] = wahl15["15"]` und `wahl16[""] = wahl16["16"]`.
//! Die Regex-/Completion-Inventare stammen aus denselben Python-Wörterbüchern,
//! die `regExReplace` und `LibRetaPrompt.NestedCompleter` verwenden.
//! Ausführung, Regex-Expansion und Completion müssen diese Daten gemeinsam benutzen.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SemanticChoiceEntry {
    pub key: &'static str,
    pub value: &'static str,
}

pub const WAHL15_I18N_ENTRIES: &[SemanticChoiceEntry] = &[
    SemanticChoiceEntry { key: "15", value: "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity,Biologischer_Baum_(15),Teilchen_anderes_Universum,nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)" },
    SemanticChoiceEntry { key: "2", value: "Konkreta_und_Focus_(2)" },
    SemanticChoiceEntry { key: "5", value: "Impulse_(5)" },
    SemanticChoiceEntry { key: "7", value: "Gefühle_(7),Anführer_Arten_(7),Erlösung" },
    SemanticChoiceEntry { key: "8", value: "Modus_und_Sein_(8),Bestrafung,Gewalt" },
    SemanticChoiceEntry { key: "10", value: "Wirklichkeiten_Wahrheit_Wahrnehmung_(10)" },
    SemanticChoiceEntry { key: "1pro30", value: "analytische_Ontologie" },
    SemanticChoiceEntry { key: "12", value: "Meta-Systeme_(12),Ordnung_und_Filterung_12_und_1pro12" },
    SemanticChoiceEntry { key: "13", value: "Paradigmen_sind_Absichten_(13)" },
    SemanticChoiceEntry { key: "17", value: "Gedanken_sind_Positionen_(17)" },
    SemanticChoiceEntry { key: "18", value: "Verbundenheiten_(18)" },
    SemanticChoiceEntry { key: "6", value: "Triebe_und_Bedürfnisse_(6),System" },
    SemanticChoiceEntry { key: "9", value: "Lust_(9)" },
    SemanticChoiceEntry { key: "3", value: "Reflexe_(3),Existenzialien_(3)" },
    SemanticChoiceEntry { key: "13_6", value: "Absicht_6_ist_Vorteilsmaximierung" },
    SemanticChoiceEntry { key: "13_7", value: "Absicht_7_ist_Selbstlosigkeit" },
    SemanticChoiceEntry { key: "13_10", value: "Absicht_10_ist_Wirklichkeit_erkennen" },
    SemanticChoiceEntry { key: "13_17", value: "Absicht_17_ist_zu_meinen" },
    SemanticChoiceEntry { key: "10_4", value: "Zeit_(4)_als_Wirklichkeit" },
    SemanticChoiceEntry { key: "16", value: "Funktionen_Vorstellungen_(16)" },
    SemanticChoiceEntry { key: "4", value: "Achtung_(4)" },
    SemanticChoiceEntry { key: "13_1pro8", value: "Absicht_1/8" },
    SemanticChoiceEntry { key: "13_1pro6", value: "Absicht_1/6_ist_Reinigung_und_Klarheit" },
    SemanticChoiceEntry { key: "1pro15", value: "Reflektion_und_Kategorien_(1/15)" },
    SemanticChoiceEntry { key: "1", value: "Bewusstheit_statt_Bewusstsein_(1)" },
    SemanticChoiceEntry { key: "30", value: "Energie_und_universelle_Eigenschaften_(30)" },
    SemanticChoiceEntry { key: "14", value: "Stimmungen_Kombinationen_(14)" },
    SemanticChoiceEntry { key: "14_6", value: "Rechnen" },
    SemanticChoiceEntry { key: "20", value: "Klassen_(20)" },
    SemanticChoiceEntry { key: "37", value: "Empathie_(37)" },
    SemanticChoiceEntry { key: "31", value: "Garben_und_Verhalten_nachfühlen(31)" },
    SemanticChoiceEntry { key: "11", value: "Verhalten_(11)" },
    SemanticChoiceEntry { key: "5_10", value: "Bedeutung_(10)" },
    SemanticChoiceEntry { key: "17_6", value: "Themen_(6)" },
    SemanticChoiceEntry { key: "17_6_10mit4", value: "Optimierung_(10)" },
    SemanticChoiceEntry { key: "36", value: "Attraktionen_(36)" },
    SemanticChoiceEntry { key: "13_16", value: "Absicht_16_ist_zu_genügen" },
    SemanticChoiceEntry { key: "18_7", value: "Liebe_(7)" },
    SemanticChoiceEntry { key: "18_10", value: "Koalitionen_(10)" },
    SemanticChoiceEntry { key: "18_17", value: "Ansichten_Standpunkte_(18_17)" },
    SemanticChoiceEntry { key: "1pro8", value: "Prinzipien(1/8)" },
    SemanticChoiceEntry { key: "1pro5", value: "Bestrebungen(1/5)" },
    SemanticChoiceEntry { key: "1pro3", value: "Bedingung_und_Auslöser_(1/3)" },
    SemanticChoiceEntry { key: "10_4_18_6", value: "relativer_Zeit-Betrag_(15_10_4_18_6)" },
    SemanticChoiceEntry { key: "18_6", value: "Zahlenvergleich_(15_18_6)" },
    SemanticChoiceEntry { key: "21", value: "Leidenschaften_(21)" },
    SemanticChoiceEntry { key: "26", value: "Erwartungshaltungen_(26)" },
    SemanticChoiceEntry { key: "19", value: "Extremalien_(19),Ziele_(19)" },
    SemanticChoiceEntry { key: "18_15", value: "universeller_Komperativ_(18→15)" },
    SemanticChoiceEntry { key: "18_15_n-vs-1pron", value: "Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)" },
    SemanticChoiceEntry { key: "1pro13", value: "Sollen_Frage_Vorgehensweise_(1/13)" },
    SemanticChoiceEntry { key: "1pro19", value: "Fundament_(1/19)" },
    SemanticChoiceEntry { key: "90", value: "abhängige_Verbundenheit_(90)" },
    SemanticChoiceEntry { key: "13_13", value: "Absicht_13_ist_Helfen" },
    SemanticChoiceEntry { key: "1pro12", value: "Karte_Filter_und_Unterscheidung_(1/12)" },
    SemanticChoiceEntry { key: "39", value: "Maßnahmen_(39)" },
    SemanticChoiceEntry { key: "1pro6", value: "innere_Werte_1/6_der_Reinigung_und_Klarheit" },
    SemanticChoiceEntry { key: "28", value: "Lebensbereiche_Problemklassen_(28)" },
    SemanticChoiceEntry { key: "24", value: "Netzwerk" },
    SemanticChoiceEntry { key: "32", value: "mathematisches_Design_(32)" },
    SemanticChoiceEntry { key: "gegen5", value: "gegen_5" },
    SemanticChoiceEntry { key: "9_6", value: "Größenordnung" },
    SemanticChoiceEntry { key: "51", value: "Kontroverse_(51)" },
    SemanticChoiceEntry { key: "13_4", value: "Taetigkeiten" },
    SemanticChoiceEntry { key: "7mit6", value: "Wohlbefinden_(7mit6)" },
];
pub const WAHL16_I18N_ENTRIES: &[SemanticChoiceEntry] = &[
    SemanticChoiceEntry { key: "1", value: "Meta-Physik-Teilchen_(1)" },
    SemanticChoiceEntry { key: "2", value: "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Model_of_Hierarchical_Complexity" },
    SemanticChoiceEntry { key: "3", value: "Teilchen_anderes_Universum" },
    SemanticChoiceEntry { key: "5", value: "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Model_of_Hierarchical_Complexity,Biologischer_Baum_(16_->_5),P5" },
    SemanticChoiceEntry { key: "6", value: "Geist_(15)" },
    SemanticChoiceEntry { key: "15", value: "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Model_of_Hierarchical_Complexity" },
    SemanticChoiceEntry { key: "10", value: "Struktur-Wissenschaften_(10)" },
    SemanticChoiceEntry { key: "16", value: "Multiversalien_(16),P" },
    SemanticChoiceEntry { key: "20", value: "Muster-Wissenschaften_(20)" },
];
pub const RETAPROMPT_WAHL15_ENTRIES: &[SemanticChoiceEntry] = &[
    SemanticChoiceEntry { key: "15", value: "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity,Biologischer_Baum_(15),Teilchen_anderes_Universum,nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)" },
    SemanticChoiceEntry { key: "2", value: "Konkreta_und_Focus_(2)" },
    SemanticChoiceEntry { key: "5", value: "Impulse_(5)" },
    SemanticChoiceEntry { key: "7", value: "Gefühle_(7),Anführer_Arten_(7),Erlösung" },
    SemanticChoiceEntry { key: "8", value: "Modus_und_Sein_(8),Bestrafung,Gewalt" },
    SemanticChoiceEntry { key: "10", value: "Wirklichkeiten_Wahrheit_Wahrnehmung_(10)" },
    SemanticChoiceEntry { key: "1pro30", value: "analytische_Ontologie" },
    SemanticChoiceEntry { key: "12", value: "Meta-Systeme_(12),Ordnung_und_Filterung_12_und_1pro12" },
    SemanticChoiceEntry { key: "13", value: "Paradigmen_sind_Absichten_(13)" },
    SemanticChoiceEntry { key: "17", value: "Gedanken_sind_Positionen_(17)" },
    SemanticChoiceEntry { key: "18", value: "Verbundenheiten_(18)" },
    SemanticChoiceEntry { key: "6", value: "Triebe_und_Bedürfnisse_(6),System" },
    SemanticChoiceEntry { key: "9", value: "Lust_(9)" },
    SemanticChoiceEntry { key: "3", value: "Reflexe_(3),Existenzialien_(3)" },
    SemanticChoiceEntry { key: "13_6", value: "Absicht_6_ist_Vorteilsmaximierung" },
    SemanticChoiceEntry { key: "13_7", value: "Absicht_7_ist_Selbstlosigkeit" },
    SemanticChoiceEntry { key: "13_10", value: "Absicht_10_ist_Wirklichkeit_erkennen" },
    SemanticChoiceEntry { key: "13_17", value: "Absicht_17_ist_zu_meinen" },
    SemanticChoiceEntry { key: "10_4", value: "Zeit_(4)_als_Wirklichkeit" },
    SemanticChoiceEntry { key: "16", value: "Funktionen_Vorstellungen_(16)" },
    SemanticChoiceEntry { key: "4", value: "Achtung_(4)" },
    SemanticChoiceEntry { key: "13_1pro8", value: "Absicht_1/8" },
    SemanticChoiceEntry { key: "13_1pro6", value: "Absicht_1/6_ist_Reinigung_und_Klarheit" },
    SemanticChoiceEntry { key: "1pro15", value: "Reflektion_und_Kategorien_(1/15)" },
    SemanticChoiceEntry { key: "1", value: "Bewusstheit_statt_Bewusstsein_(1)" },
    SemanticChoiceEntry { key: "30", value: "Energie_und_universelle_Eigenschaften_(30)" },
    SemanticChoiceEntry { key: "14", value: "Stimmungen_Kombinationen_(14)" },
    SemanticChoiceEntry { key: "14_6", value: "Rechnen" },
    SemanticChoiceEntry { key: "20", value: "Klassen_(20)" },
    SemanticChoiceEntry { key: "37", value: "Empathie_(37)" },
    SemanticChoiceEntry { key: "31", value: "Garben_und_Verhalten_nachfühlen(31)" },
    SemanticChoiceEntry { key: "11", value: "Verhalten_(11)" },
    SemanticChoiceEntry { key: "5_10", value: "Bedeutung_(10)" },
    SemanticChoiceEntry { key: "17_6", value: "Themen_(6)" },
    SemanticChoiceEntry { key: "17_6_10mit4", value: "Optimierung_(10)" },
    SemanticChoiceEntry { key: "36", value: "Attraktionen_(36)" },
    SemanticChoiceEntry { key: "13_16", value: "Absicht_16_ist_zu_genügen" },
    SemanticChoiceEntry { key: "18_7", value: "Liebe_(7)" },
    SemanticChoiceEntry { key: "18_10", value: "Koalitionen_(10)" },
    SemanticChoiceEntry { key: "18_17", value: "Ansichten_Standpunkte_(18_17)" },
    SemanticChoiceEntry { key: "1pro8", value: "Prinzipien(1/8)" },
    SemanticChoiceEntry { key: "1pro5", value: "Bestrebungen(1/5)" },
    SemanticChoiceEntry { key: "1pro3", value: "Bedingung_und_Auslöser_(1/3)" },
    SemanticChoiceEntry { key: "10_4_18_6", value: "relativer_Zeit-Betrag_(15_10_4_18_6)" },
    SemanticChoiceEntry { key: "18_6", value: "Zahlenvergleich_(15_18_6)" },
    SemanticChoiceEntry { key: "21", value: "Leidenschaften_(21)" },
    SemanticChoiceEntry { key: "26", value: "Erwartungshaltungen_(26)" },
    SemanticChoiceEntry { key: "19", value: "Extremalien_(19),Ziele_(19)" },
    SemanticChoiceEntry { key: "18_15", value: "universeller_Komperativ_(18→15)" },
    SemanticChoiceEntry { key: "18_15_n-vs-1pron", value: "Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)" },
    SemanticChoiceEntry { key: "1pro13", value: "Sollen_Frage_Vorgehensweise_(1/13)" },
    SemanticChoiceEntry { key: "1pro19", value: "Fundament_(1/19)" },
    SemanticChoiceEntry { key: "90", value: "abhängige_Verbundenheit_(90)" },
    SemanticChoiceEntry { key: "13_13", value: "Absicht_13_ist_Helfen" },
    SemanticChoiceEntry { key: "1pro12", value: "Karte_Filter_und_Unterscheidung_(1/12)" },
    SemanticChoiceEntry { key: "39", value: "Maßnahmen_(39)" },
    SemanticChoiceEntry { key: "1pro6", value: "innere_Werte_1/6_der_Reinigung_und_Klarheit" },
    SemanticChoiceEntry { key: "28", value: "Lebensbereiche_Problemklassen_(28)" },
    SemanticChoiceEntry { key: "24", value: "Netzwerk" },
    SemanticChoiceEntry { key: "32", value: "mathematisches_Design_(32)" },
    SemanticChoiceEntry { key: "gegen5", value: "gegen_5" },
    SemanticChoiceEntry { key: "9_6", value: "Größenordnung" },
    SemanticChoiceEntry { key: "51", value: "Kontroverse_(51)" },
    SemanticChoiceEntry { key: "13_4", value: "Taetigkeiten" },
    SemanticChoiceEntry { key: "7mit6", value: "Wohlbefinden_(7mit6)" },
    SemanticChoiceEntry { key: "", value: "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity,Biologischer_Baum_(15),Teilchen_anderes_Universum,nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)" },
];
pub const RETAPROMPT_WAHL16_ENTRIES: &[SemanticChoiceEntry] = &[
    SemanticChoiceEntry { key: "1", value: "Meta-Physik-Teilchen_(1)" },
    SemanticChoiceEntry { key: "2", value: "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Model_of_Hierarchical_Complexity" },
    SemanticChoiceEntry { key: "3", value: "Teilchen_anderes_Universum" },
    SemanticChoiceEntry { key: "5", value: "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Model_of_Hierarchical_Complexity,Biologischer_Baum_(16_->_5),P5" },
    SemanticChoiceEntry { key: "6", value: "Geist_(15)" },
    SemanticChoiceEntry { key: "15", value: "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Model_of_Hierarchical_Complexity" },
    SemanticChoiceEntry { key: "10", value: "Struktur-Wissenschaften_(10)" },
    SemanticChoiceEntry { key: "16", value: "Multiversalien_(16),P" },
    SemanticChoiceEntry { key: "20", value: "Muster-Wissenschaften_(20)" },
    SemanticChoiceEntry { key: "", value: "Multiversalien_(16),P" },
];
pub const RETAPROMPT_WAHL15_KEYS: &[&str] = &[
    "15",
    "2",
    "5",
    "7",
    "8",
    "10",
    "1pro30",
    "12",
    "13",
    "17",
    "18",
    "6",
    "9",
    "3",
    "13_6",
    "13_7",
    "13_10",
    "13_17",
    "10_4",
    "16",
    "4",
    "13_1pro8",
    "13_1pro6",
    "1pro15",
    "1",
    "30",
    "14",
    "14_6",
    "20",
    "37",
    "31",
    "11",
    "5_10",
    "17_6",
    "17_6_10mit4",
    "36",
    "13_16",
    "18_7",
    "18_10",
    "18_17",
    "1pro8",
    "1pro5",
    "1pro3",
    "10_4_18_6",
    "18_6",
    "21",
    "26",
    "19",
    "18_15",
    "18_15_n-vs-1pron",
    "1pro13",
    "1pro19",
    "90",
    "13_13",
    "1pro12",
    "39",
    "1pro6",
    "28",
    "24",
    "32",
    "gegen5",
    "9_6",
    "51",
    "13_4",
    "7mit6",
    "",
];
pub const RETAPROMPT_WAHL16_KEYS: &[&str] = &[
    "1",
    "2",
    "3",
    "5",
    "6",
    "15",
    "10",
    "16",
    "20",
    "",
];
pub const RETAPROMPT_RETA_MAIN_SWITCHES: &[&str] = &[
    "-zeilen",
    "-spalten",
    "-kombination",
    "-ausgabe",
    "-h",
    "-help",
    "-debug",
    "-nichts",
];
pub const RETAPROMPT_RETA_SECTION_SWITCHES: &[&str] = &[
    "-zeilen",
    "-spalten",
    "-kombination",
    "-ausgabe",
];
pub const RETAPROMPT_ZEILEN_REGEX_PARAMETERS: &[&str] = &[
    "alles",
    "gestern",
    "heute",
    "hoehemaximal",
    "mond",
    "morgen",
    "nachtraeglichneuabzaehlung",
    "nachtraeglichneuabzaehlungvielfache",
    "oberesmaximum",
    "planet",
    "potenzenvonzahlen",
    "primzahlvielfache",
    "schwarzesonne",
    "sonne",
    "typ",
    "vielfachevonzahlen",
    "vorhervonausschnitt",
    "vorhervonausschnittteiler",
    "zaehlung",
    "zeit",
    "primzahlen",
    "aussenerste",
    "innenerste",
    "aussenalle",
    "innenalle",
    "invertieren",
    "SonneMitMondanteil",
];
pub const RETAPROMPT_ZEILEN_PARAMETER_TOKENS: &[&str] = &[
    "--zeit=",
    "--zaehlung=",
    "--vorhervonausschnitt=",
    "--vorhervonausschnittteiler",
    "--primzahlvielfache=",
    "--nachtraeglichneuabzaehlung=",
    "--nachtraeglichneuabzaehlungvielfache=",
    "--alles",
    "--potenzenvonzahlen=",
    "--typ=",
    "--vielfachevonzahlen=",
    "--oberesmaximum=",
    "--primzahlen=",
    "--invertieren",
    "--*=",
];
pub const RETAPROMPT_ZEILEN_TYP_PARAMETER: &str = "typ";
pub const RETAPROMPT_ZEILEN_TYP_VALUES: &[&str] = &[
    "sonne",
    "mond",
    "planet",
    "schwarzesonne",
    "SonneMitMondanteil",
];
pub const RETAPROMPT_ZEILEN_ZEIT_PARAMETER: &str = "zeit";
pub const RETAPROMPT_ZEILEN_ZEIT_VALUES: &[&str] = &[
    "heute",
    "gestern",
    "morgen",
];
pub const RETAPROMPT_ZEILEN_PRIMZAHLEN_PARAMETER: &str = "primzahlen";
pub const RETAPROMPT_ZEILEN_PRIMZAHLEN_VALUES: &[&str] = &[
    "aussenerste",
    "innenerste",
    "aussenalle",
    "innenalle",
];
pub const RETAPROMPT_AUSGABE_REGEX_PARAMETERS: &[&str] = &[
    "nocolor",
    "justtext",
    "art",
    "onetable",
    "spaltenreihenfolgeundnurdiese",
    "endlessscreen",
    "endless",
    "dontwrap",
    "breite",
    "breiten",
    "keineleereninhalte",
    "keinenummerierung",
    "keineueberschriften",
];
pub const RETAPROMPT_AUSGABE_PARAMETER_TOKENS: &[&str] = &[
    "--nocolor",
    "--justtext",
    "--art=",
    "--onetable",
    "--spaltenreihenfolgeundnurdiese=",
    "--endlessscreen",
    "--endless",
    "--dontwrap",
    "--breite=",
    "--breiten=",
    "--keineleereninhalte",
    "--keinenummerierung",
    "--keineueberschriften",
    "--*=",
];
pub const RETAPROMPT_AUSGABE_ART_PARAMETER: &str = "art";
pub const RETAPROMPT_AUSGABE_ART_VALUES: &[&str] = &[
    "bbcode",
    "html",
    "csv",
    "shell",
    "markdown",
    "emacs",
    "nichts",
];
pub const RETAPROMPT_AUSGABE_BREITE_PARAMETER: &str = "breite";
pub const RETAPROMPT_AUSGABE_BREITEN_PARAMETER: &str = "breiten";
pub const RETAPROMPT_KOMBINATION_GALAXIE_PARAMETER: &str = "galaxie";
pub const RETAPROMPT_KOMBINATION_UNIVERSUM_PARAMETER: &str = "universum";
pub const RETAPROMPT_KOMBINATION_PARAMETER_TOKENS: &[&str] = &[
    "--galaxie=",
    "--universum=",
    "--*=",
];
pub fn retaprompt_wahl15_entries() -> &'static [SemanticChoiceEntry] {
    RETAPROMPT_WAHL15_ENTRIES
}

pub fn retaprompt_wahl16_entries() -> &'static [SemanticChoiceEntry] {
    RETAPROMPT_WAHL16_ENTRIES
}

pub fn semantic_wahl15_ordered_keys() -> &'static [&'static str] {
    RETAPROMPT_WAHL15_KEYS
}

pub fn semantic_wahl16_ordered_keys() -> &'static [&'static str] {
    RETAPROMPT_WAHL16_KEYS
}

pub fn semantic_wahl15_value(key: &str) -> Option<&'static str> {
    RETAPROMPT_WAHL15_ENTRIES
        .iter()
        .find(|entry| entry.key == key)
        .map(|entry| entry.value)
}

pub fn semantic_wahl16_value(key: &str) -> Option<&'static str> {
    RETAPROMPT_WAHL16_ENTRIES
        .iter()
        .find(|entry| entry.key == key)
        .map(|entry| entry.value)
}

pub fn is_wahl15_key(key: &str) -> bool {
    semantic_wahl15_value(key).is_some()
}

pub fn is_wahl16_key(key: &str) -> bool {
    semantic_wahl16_value(key).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_unique_choice_keys(entries: &[SemanticChoiceEntry]) {
        let mut seen = std::collections::BTreeSet::new();
        for entry in entries {
            assert!(seen.insert(entry.key), "duplicate semantic choice key {:?}", entry.key);
        }
    }

    #[test]
    fn prompt_mutation_keeps_python_empty_choice_aliases() {
        assert_eq!(semantic_wahl15_value(""), semantic_wahl15_value("15"));
        assert_eq!(semantic_wahl16_value(""), semantic_wahl16_value("16"));
    }

    #[test]
    fn prompt_choice_keys_keep_python_dict_uniqueness_after_prompt_mutation() {
        assert_unique_choice_keys(WAHL15_I18N_ENTRIES);
        assert_unique_choice_keys(WAHL16_I18N_ENTRIES);
        assert_unique_choice_keys(RETAPROMPT_WAHL15_ENTRIES);
        assert_unique_choice_keys(RETAPROMPT_WAHL16_ENTRIES);
    }

    #[test]
    fn wahl15_values_match_python_canonical_strings_for_known_drift_cases() {
        assert_eq!(
            semantic_wahl15_value("15"),
            Some("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity,Biologischer_Baum_(15),Teilchen_anderes_Universum,nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)")
        );
        assert_eq!(semantic_wahl15_value("9_6"), Some("Größenordnung"));
    }

    #[test]
    fn prompt_choice_counts_match_python_words_plus_retaprompt_empty_alias() {
        assert_eq!(WAHL15_I18N_ENTRIES.len(), 65);
        assert_eq!(WAHL16_I18N_ENTRIES.len(), 9);
        assert_eq!(RETAPROMPT_WAHL15_ENTRIES.len(), 66);
        assert_eq!(RETAPROMPT_WAHL16_ENTRIES.len(), 10);
        assert_eq!(RETAPROMPT_WAHL15_KEYS.len(), RETAPROMPT_WAHL15_ENTRIES.len());
        assert_eq!(RETAPROMPT_WAHL16_KEYS.len(), RETAPROMPT_WAHL16_ENTRIES.len());
    }

    #[test]
    fn prompt_regex_and_completion_tables_come_from_python_words() {
        assert!(RETAPROMPT_RETA_MAIN_SWITCHES.contains(&"-debug"));
        assert!(RETAPROMPT_RETA_SECTION_SWITCHES.contains(&"-kombination"));
        assert!(RETAPROMPT_ZEILEN_PARAMETER_TOKENS.contains(&"--typ="));
        assert!(RETAPROMPT_ZEILEN_TYP_VALUES.contains(&"SonneMitMondanteil"));
        assert!(RETAPROMPT_AUSGABE_PARAMETER_TOKENS.contains(&"--keineueberschriften"));
        assert!(RETAPROMPT_AUSGABE_ART_VALUES.contains(&"markdown"));
        assert_eq!(RETAPROMPT_KOMBINATION_GALAXIE_PARAMETER, "galaxie");
        assert!(RETAPROMPT_KOMBINATION_PARAMETER_TOKENS.contains(&"--universum="));
    }
}
