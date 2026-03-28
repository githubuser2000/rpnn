use std::fmt;

use crate::domain::eigenschaften::{EigenschaftKeyId, EigenschaftStandardFamilie};
use crate::domain::errors::ParseSpaltenAnfrageError;
use crate::domain::parser::legacy_cli_typed::{matches_any_alias, LegacyOberToken};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StandardOberkategorie {
    Menschliches,
    Universum,
    Religion,
    Planet,
    Galaxie,
    Multiversum,
    Grundstrukturen,
    Bedeutung,
    ProContra,
    WichtigstesZumVerstehen,
    Eigenschaften,
    EigenschaftenN,
    Eigenschaften1ProN,
    UniversumMetaKonkret,
    Sonstige(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MenschlichesUnter {
    Liebe,
    Gleichheit,
    Hoelle,
    Klasse,
    Gewalt,
    Politische,
    Richtungen,
    Formationen,
    Motive,
    Sonstige(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UniversumUnter {
    Geist,
    Primzahlkreuz,
    Sonstige(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReligionUnter {
    Religion,
    Ethik,
    Sonstige(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StandardAnfrage {
    Menschliches(MenschlichesUnter),
    Universum(UniversumUnter),
    Religion(ReligionUnter),
    Sonstige {
        ober: StandardOberkategorie,
        unter: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpaltenAnfrage {
    Standard(StandardAnfrage),
    KombinationGalaxie { unter: String },
    KombinationUniversum { unter: String },
    GebrochenRationalGalaxie { unter: String },
    GebrochenRationalUniversum { unter: String },
    GebrochenRationalGefuehle { unter: String },
    GebrochenRationalStrukturgroesse { unter: String },
    Primvielfache { unter: String },
    Multiplikationen { unter: String },
    Unknown { ober: String, unter: String },
}

impl StandardOberkategorie {
    pub fn parse(input: &str) -> Self {
        match LegacyOberToken::parse(input) {
            LegacyOberToken::Menschliches => Self::Menschliches,
            LegacyOberToken::Universum => Self::Universum,
            LegacyOberToken::Religion => Self::Religion,
            LegacyOberToken::Planet => Self::Planet,
            LegacyOberToken::Galaxie => Self::Galaxie,
            LegacyOberToken::Multiversum => Self::Multiversum,
            LegacyOberToken::Grundstrukturen => Self::Grundstrukturen,
            LegacyOberToken::Bedeutung => Self::Bedeutung,
            LegacyOberToken::ProContra => Self::ProContra,
            LegacyOberToken::WichtigstesZumVerstehen => Self::WichtigstesZumVerstehen,
            LegacyOberToken::Eigenschaften => Self::Eigenschaften,
            LegacyOberToken::EigenschaftenN => Self::EigenschaftenN,
            LegacyOberToken::Eigenschaften1ProN => Self::Eigenschaften1ProN,
            LegacyOberToken::UniversumMetaKonkret => Self::UniversumMetaKonkret,
            LegacyOberToken::Unknown(value) => Self::Sonstige(value),
            _ => Self::Sonstige(input.trim().to_string()),
        }
    }

    pub fn as_cli_str(&self) -> &str {
        match self {
            Self::Menschliches => "Menschliches",
            Self::Universum => "Universum",
            Self::Religion => "Religion",
            Self::Planet => "Planet",
            Self::Galaxie => "Galaxie",
            Self::Multiversum => "Multiversum",
            Self::Grundstrukturen => "Grundstrukturen",
            Self::Bedeutung => "Bedeutung",
            Self::ProContra => "Pro_Contra",
            Self::WichtigstesZumVerstehen => "Wichtigstes_zum_verstehen",
            Self::Eigenschaften => "Eigenschaften",
            Self::EigenschaftenN => "Eigenschaften_n",
            Self::Eigenschaften1ProN => "Eigenschaften_1/n",
            Self::UniversumMetaKonkret => "universummetakonkret",
            Self::Sonstige(s) => s.as_str(),
        }
    }
}

impl MenschlichesUnter {
    pub fn parse(input: &str) -> Self {
        if matches_any_alias(input, &["Liebe", "Ethik"]) {
            Self::Liebe
        } else if matches_any_alias(input, &["Gleichheit"]) {
            Self::Gleichheit
        } else if matches_any_alias(input, &["Hölle", "Hoelle"]) {
            Self::Hoelle
        } else if matches_any_alias(input, &["Klasse"]) {
            Self::Klasse
        } else if matches_any_alias(input, &["Gewalt"]) {
            Self::Gewalt
        } else if matches_any_alias(input, &["politische"]) {
            Self::Politische
        } else if matches_any_alias(input, &["Richtungen"]) {
            Self::Richtungen
        } else if matches_any_alias(input, &["Formationen"]) {
            Self::Formationen
        } else if matches_any_alias(input, &["Motive"]) {
            Self::Motive
        } else {
            Self::Sonstige(input.trim().to_string())
        }
    }

    pub fn as_cli_str(&self) -> &str {
        match self {
            Self::Liebe => "Liebe",
            Self::Gleichheit => "Gleichheit",
            Self::Hoelle => "Hölle",
            Self::Klasse => "Klasse",
            Self::Gewalt => "Gewalt",
            Self::Politische => "politische",
            Self::Richtungen => "Richtungen",
            Self::Formationen => "Formationen",
            Self::Motive => "Motive",
            Self::Sonstige(s) => s.as_str(),
        }
    }
}

impl UniversumUnter {
    pub fn parse(input: &str) -> Self {
        if matches_any_alias(input, &["Geist"]) {
            Self::Geist
        } else if matches_any_alias(input, &["Primzahlkreuz", "Primzahlkreuz pro contra"]) {
            Self::Primzahlkreuz
        } else {
            Self::Sonstige(input.trim().to_string())
        }
    }

    pub fn as_cli_str(&self) -> &str {
        match self {
            Self::Geist => "Geist",
            Self::Primzahlkreuz => "Primzahlkreuz",
            Self::Sonstige(s) => s.as_str(),
        }
    }
}

impl ReligionUnter {
    pub fn parse(input: &str) -> Self {
        if matches_any_alias(input, &["Religion", "Religionen"]) {
            Self::Religion
        } else if matches_any_alias(input, &["Ethik"]) {
            Self::Ethik
        } else {
            Self::Sonstige(input.trim().to_string())
        }
    }

    pub fn as_cli_str(&self) -> &str {
        match self {
            Self::Religion => "Religion",
            Self::Ethik => "Ethik",
            Self::Sonstige(s) => s.as_str(),
        }
    }
}

fn ober_erlaubt_eigenschaft(ober: &StandardOberkategorie, key: EigenschaftKeyId) -> bool {
    match ober {
        StandardOberkategorie::EigenschaftenN => {
            matches!(key.standard_familie(), EigenschaftStandardFamilie::N)
        }
        StandardOberkategorie::Eigenschaften1ProN => {
            matches!(key.standard_familie(), EigenschaftStandardFamilie::EinsDurchN)
        }
        _ => true,
    }
}

impl SpaltenAnfrage {
    pub fn parse(ober: &str, unter: &str) -> Result<Self, ParseSpaltenAnfrageError> {
        let unter = unter.trim();
        if unter.is_empty() {
            return Err(ParseSpaltenAnfrageError::EmptyUnterkategorie);
        }

        let unter = unter.to_string();
        let ober_token = LegacyOberToken::parse(ober);

        let parsed = match ober_token {
            LegacyOberToken::KombinationGalaxie => Self::KombinationGalaxie { unter },
            LegacyOberToken::KombinationUniversum => Self::KombinationUniversum { unter },
            LegacyOberToken::GebrochenRationalGalaxie => Self::GebrochenRationalGalaxie { unter },
            LegacyOberToken::GebrochenRationalUniversum => Self::GebrochenRationalUniversum { unter },
            LegacyOberToken::GebrochenRationalGefuehle => Self::GebrochenRationalGefuehle { unter },
            LegacyOberToken::GebrochenRationalStrukturgroesse => Self::GebrochenRationalStrukturgroesse { unter },
            LegacyOberToken::Primvielfache => Self::Primvielfache { unter },
            LegacyOberToken::Multiplikationen => Self::Multiplikationen { unter },
            LegacyOberToken::Unknown(_) => Self::Unknown {
                ober: ober.to_string(),
                unter,
            },
            _ => {
                let standard = StandardOberkategorie::parse(ober);
                match standard {
                    StandardOberkategorie::Menschliches => {
                        Self::Standard(StandardAnfrage::Menschliches(MenschlichesUnter::parse(&unter)))
                    }
                    StandardOberkategorie::Universum => {
                        Self::Standard(StandardAnfrage::Universum(UniversumUnter::parse(&unter)))
                    }
                    StandardOberkategorie::Religion => {
                        Self::Standard(StandardAnfrage::Religion(ReligionUnter::parse(&unter)))
                    }
                    StandardOberkategorie::Sonstige(_) => Self::Unknown {
                        ober: ober.to_string(),
                        unter,
                    },
                    known => {
                        if matches!(known, StandardOberkategorie::Eigenschaften | StandardOberkategorie::EigenschaftenN | StandardOberkategorie::Eigenschaften1ProN) {
                            if let Some(key) = EigenschaftKeyId::from_alias(&unter) {
                                if !ober_erlaubt_eigenschaft(&known, key) {
                                    return Err(ParseSpaltenAnfrageError::InvalidUnterkategorieForOberkategorie {
                                        ober: known.as_cli_str().to_string(),
                                        unter,
                                    });
                                }
                            }
                        }
                        Self::Standard(StandardAnfrage::Sonstige { ober: known, unter })
                    },
                }
            }
        };

        Ok(parsed)
    }

    pub fn to_cli(&self) -> String {
        let (ober, unter) = self.ober_unter_cli_pair();
        format!("--spaltenname {} {}", ober, unter)
    }

    pub fn ober_unter_cli_pair(&self) -> (String, String) {
        match self {
            Self::Standard(StandardAnfrage::Menschliches(unter)) => {
                ("Menschliches".to_string(), unter.as_cli_str().to_string())
            }
            Self::Standard(StandardAnfrage::Universum(unter)) => {
                ("Universum".to_string(), unter.as_cli_str().to_string())
            }
            Self::Standard(StandardAnfrage::Religion(unter)) => {
                ("Religion".to_string(), unter.as_cli_str().to_string())
            }
            Self::Standard(StandardAnfrage::Sonstige { ober, unter }) => {
                (ober.as_cli_str().to_string(), unter.clone())
            }
            Self::KombinationGalaxie { unter } => ("KombinationGalaxie".to_string(), unter.clone()),
            Self::KombinationUniversum { unter } => ("KombinationUniversum".to_string(), unter.clone()),
            Self::GebrochenRationalGalaxie { unter } => ("gebrochen-rational_Galaxie_n/m".to_string(), unter.clone()),
            Self::GebrochenRationalUniversum { unter } => ("gebrochen-rational_Universum_n/m".to_string(), unter.clone()),
            Self::GebrochenRationalGefuehle { unter } => ("gebrochen-rational_Gefuehle_n/m".to_string(), unter.clone()),
            Self::GebrochenRationalStrukturgroesse { unter } => ("gebrochen-rational_Strukturgroesse_n/m".to_string(), unter.clone()),
            Self::Primvielfache { unter } => ("Primvielfache".to_string(), unter.clone()),
            Self::Multiplikationen { unter } => ("Multiplikationen".to_string(), unter.clone()),
            Self::Unknown { ober, unter } => (ober.clone(), unter.clone()),
        }
    }

    pub fn ober_normalized(&self) -> String {
        let (ober, _) = self.ober_unter_cli_pair();
        ober.to_lowercase()
    }

    pub fn unter_normalized(&self) -> String {
        let (_, unter) = self.ober_unter_cli_pair();
        unter.to_lowercase()
    }

    pub fn generated_befehle_hint(&self) -> Vec<String> {
        match self {
            Self::Standard(StandardAnfrage::Universum(UniversumUnter::Primzahlkreuz)) => {
                vec!["primzahlkreuz".to_string()]
            }
            Self::Primvielfache { .. } => vec!["primvielfache".to_string()],
            Self::Multiplikationen { .. } => vec!["multiplikationen".to_string()],
            _ => Vec::new(),
        }
    }

    pub fn parameters_main_hint(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
        let (ober, unter) = self.ober_unter_cli_pair();
        (Some(ober.to_lowercase()), None, None, Some(unter.to_lowercase()))
    }
}

impl fmt::Display for SpaltenAnfrage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_cli())
    }
}
