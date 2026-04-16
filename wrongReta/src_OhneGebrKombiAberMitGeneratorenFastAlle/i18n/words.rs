use std::collections::HashMap;
use once_cell::sync::Lazy;

/// === table_handling ===
/// reine Namespace-Struktur
pub mod table_handling {

    /// entspricht:
    /// parameterName: dict = {"kombination": _("kombination")}
    pub static PARAMETER_NAME: Lazy<HashMap<&'static str, &'static str>> =
        Lazy::new(|| {
            HashMap::from([
                ("kombination", "kombination"),
            ])
        });

    /// entspricht: into = { ... }
    pub static INTO: Lazy<HashMap<&'static str, &'static str>> =
        Lazy::new(|| {
            HashMap::from([
                (
                    "Kombination_(Galaxie_und_schwarzes_Loch)_(14_mit_13)",
                    "Kombination_(Galaxie_und_schwarzes_Loch)_(14_mit_13)",
                ),
                (
                    "Wichtigstes_zum_gedanklich_einordnen",
                    "Wichtigstes_zum_gedanklich_einordnen",
                ),
                ("Zweitwichtigste", "Zweitwichtigste"),
                ("berufe", "berufe"),
                ("intelligenz", "intelligenz"),
                ("tiere", "tiere"),
            ])
        });
}
pub mod readme_files {
    pub const RETA: &str = "readme-reta.md";
    pub const RETA_PROMPT: &str = "readme-retaPrompt.md";
    pub const START_FILES: &str = "readme-startFiles.md";
    pub const DEVELOPER: &str = "readme.org";
}
wrongLangSentence = (
    _("für '-languages=' sind die Paramter-Werte erlaubt: ")
    + str(tuple(sprachen.values()))[1:-1]
)
pub fn wrong_lang_sentence(languages: &[&str]) -> String {
    let values = languages.join(", ");
    format!(
        "für '-languages=' sind die Paramter-Werte erlaubt: {}",
        values
    )
}
pub static TOM_DECODED_MOTIVES_LANG: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        HashMap::from([
            (
                "kr",
                "kr-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
            ),
            (
                "cn",
                "cn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
            ),
        ])
    });

}

// ============================================================
// OPTIONALE HILFSFUNKTIONEN (praktisch, nicht magisch)
// ============================================================

pub fn supported_languages() -> Vec<&'static str> {
    TOM_DECODED_MOTIVES_LANG.keys().copied().collect()
}

pub fn csv_for_language(lang: &str) -> Option<&'static str> {
    TOM_DECODED_MOTIVES_LANG.get(lang).copied()
}

// ============================================================
// words.rs — transcompiliert, typsicher, zero-allocation
// ============================================================

use std::fmt;

// ============================================================
// LANGUAGE ENUM (ersetzt freie Strings)
// ============================================================

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Language {
    De,
    En,
    Kr,
    Cn,
}

impl Language {
    pub const ALL: &'static [Language] = &[
        Language::De,
        Language::En,
        Language::Kr,
        Language::Cn,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Language::De => "de",
            Language::En => "en",
            Language::Kr => "kr",
            Language::Cn => "cn",
        }
    }

    pub const fn csv_file(self) -> &'static str {
        match self {
            Language::De =>
                "de-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
            Language::En =>
                "en-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
            Language::Kr =>
                "kr-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
            Language::Cn =>
                "cn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
        }
    }
}

// optional, praktisch
impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
#[derive(Debug)]
pub struct LanguageParseError;

impl std::str::FromStr for Language {
    type Err = LanguageParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "de" => Ok(Language::De),
            "en" => Ok(Language::En),
            "kr" => Ok(Language::Kr),
            "cn" => Ok(Language::Cn),
            _ => Err(LanguageParseError),
        }
    }
}

// ============================================================
// TABLE HANDLING (ehemals Klassen mit Dicts)
// ============================================================

pub mod table_handling {

    pub const PARAMETER_NAME: &[(&str, &str)] = &[
        ("kombination", "kombination"),
    ];

    pub const INTO: &[(&str, &str)] = &[
        (
            "Kombination_(Galaxie_und_schwarzes_Loch)_(14_mit_13)",
            "Kombination_(Galaxie_und_schwarzes_Loch)_(14_mit_13)",
        ),
        (
            "Wichtigstes_zum_gedanklich_einordnen",
            "Wichtigstes_zum_gedanklich_einordnen",
        ),
        ("Zweitwichtigste", "Zweitwichtigste"),
        ("berufe", "berufe"),
        ("intelligenz", "intelligenz"),
        ("tiere", "tiere"),
    ];
}


