use std::fmt;

use crate::domain::eigenschaften::{EigenschaftKeyId, EigenschaftStandardFamilie};
use crate::domain::errors::ParseSpaltenAnfrageError;

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
        match input.trim() {
            "Menschliches" | "menschliches" => Self::Menschliches,
            "Universum" | "universum" => Self::Universum,
            "Religion" | "religion" | "Religionen" | "religionen" => Self::Religion,
            "Planet" | "planet" | "Planet_(10_und_oder_12)" | "planet_(10_und_oder_12)" => Self::Planet,
            "Galaxie" | "galaxie" | "Galaxien" | "galaxien" | "AlteSchriften" | "alteschriften" | "Kreis" | "kreis" | "Kreise" | "kreise" => Self::Galaxie,
            "Multiversum" | "multiversum" => Self::Multiversum,
            "Grundstrukturen" | "grundstrukturen" => Self::Grundstrukturen,
            "Bedeutung" | "bedeutung" | "Wirtschaft" | "wirtschaft" => Self::Bedeutung,
            "Pro_Contra" | "pro_contra" | "ProContra" | "procontra" => Self::ProContra,
            "Wichtigstes_zum_verstehen" | "wichtigstes_zum_verstehen" | "wichtigsteverstehen" => Self::WichtigstesZumVerstehen,
            "Eigenschaften_n" | "eigenschaften_n" | "Konzept1" | "konzept1" | "Konzepte1" | "konzepte1" => Self::EigenschaftenN,
            "Eigenschaften_1/n" | "eigenschaften_1/n" | "Konzept2" | "konzept2" | "Konzepte2" | "konzepte2" => Self::Eigenschaften1ProN,
            "universummetakonkret" | "UniversumMetaKonkret" | "universum_metakonkret" | "metakonkret" => Self::UniversumMetaKonkret,
            other => Self::Sonstige(other.to_string()),
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
            Self::EigenschaftenN => "Eigenschaften_n",
            Self::Eigenschaften1ProN => "Eigenschaften_1/n",
            Self::UniversumMetaKonkret => "universummetakonkret",
            Self::Sonstige(s) => s.as_str(),
        }
    }
}

impl MenschlichesUnter {
    pub fn parse(input: &str) -> Self {
        match input.trim() {
            "Liebe" | "liebe" => Self::Liebe,
            "Gleichheit" | "gleichheit" => Self::Gleichheit,
            "Hölle" | "Hölle " | "hölle" | "Hoelle" | "hoelle" => Self::Hoelle,
            "Klasse" | "klasse" => Self::Klasse,
            "Gewalt" | "gewalt" => Self::Gewalt,
            "politische" | "Politische" => Self::Politische,
            "Richtungen" | "richtungen" => Self::Richtungen,
            "Formationen" | "formationen" => Self::Formationen,
            "Motive" | "motive" => Self::Motive,
            other => Self::Sonstige(other.to_string()),
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
        match input.trim() {
            "Geist" | "geist" => Self::Geist,
            "Primzahlkreuz" | "primzahlkreuz" | "Primzahlkreuz_pro_contra" | "primzahlkreuzprocontra" => Self::Primzahlkreuz,
            other => Self::Sonstige(other.to_string()),
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
        match input.trim() {
            "Religion" | "religion" | "Religionen" | "religionen" => Self::Religion,
            "Ethik" | "ethik" => Self::Ethik,
            other => Self::Sonstige(other.to_string()),
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

        let parsed = match ober.trim() {
            "KombinationGalaxie" | "kombinationgalaxie" => Self::KombinationGalaxie { unter },
            "KombinationUniversum" | "kombinationuniversum" => Self::KombinationUniversum { unter },
            "gebrochen-rational_Galaxie_n/m" | "gebrochen-rational_galaxie_n/m" => Self::GebrochenRationalGalaxie { unter },
            "gebrochen-rational_Universum_n/m" | "gebrochen-rational_universum_n/m" => Self::GebrochenRationalUniversum { unter },
            "gebrochen-rational_Gefuehle_n/m" | "gebrochen-rational_gefühle_n/m" | "gebrochen-rational_gefuehle_n/m" => {
                Self::GebrochenRationalGefuehle { unter }
            }
            "gebrochen-rational_Strukturgroesse_n/m" | "gebrochen-rational_strukturgroesse_n/m" => {
                Self::GebrochenRationalStrukturgroesse { unter }
            }
            "Primvielfache" | "primvielfache" => Self::Primvielfache { unter },
            "Multiplikationen" | "multiplikationen" => Self::Multiplikationen { unter },
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
                        if matches!(known, StandardOberkategorie::EigenschaftenN | StandardOberkategorie::Eigenschaften1ProN) {
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
        match self {
            Self::Standard(StandardAnfrage::Menschliches(unter)) => {
                format!("--spaltenname Menschliches {}", unter.as_cli_str())
            }
            Self::Standard(StandardAnfrage::Universum(unter)) => {
                format!("--spaltenname Universum {}", unter.as_cli_str())
            }
            Self::Standard(StandardAnfrage::Religion(unter)) => {
                format!("--spaltenname Religion {}", unter.as_cli_str())
            }
            Self::Standard(StandardAnfrage::Sonstige { ober, unter }) => {
                format!("--spaltenname {} {}", ober.as_cli_str(), unter)
            }
            Self::KombinationGalaxie { unter } => {
                format!("--spaltenname KombinationGalaxie {}", unter)
            }
            Self::KombinationUniversum { unter } => {
                format!("--spaltenname KombinationUniversum {}", unter)
            }
            Self::GebrochenRationalGalaxie { unter } => {
                format!("--spaltenname gebrochen-rational_Galaxie_n/m {}", unter)
            }
            Self::GebrochenRationalUniversum { unter } => {
                format!("--spaltenname gebrochen-rational_Universum_n/m {}", unter)
            }
            Self::GebrochenRationalGefuehle { unter } => {
                format!("--spaltenname gebrochen-rational_Gefuehle_n/m {}", unter)
            }
            Self::GebrochenRationalStrukturgroesse { unter } => {
                format!("--spaltenname gebrochen-rational_Strukturgroesse_n/m {}", unter)
            }
            Self::Primvielfache { unter } => {
                format!("--spaltenname primvielfache {}", unter)
            }
            Self::Multiplikationen { unter } => {
                format!("--spaltenname multiplikationen {}", unter)
            }
            Self::Unknown { ober, unter } => {
                format!("--spaltenname {} {}", ober, unter)
            }
        }
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
        fold_cli_token(&ober)
    }

    pub fn unter_normalized(&self) -> String {
        let (_, unter) = self.ober_unter_cli_pair();
        fold_cli_token(&unter)
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
        let ober = self.ober_normalized();
        let unter = self.unter_normalized();
        (Some(ober), None, None, Some(unter))
    }

}

impl fmt::Display for SpaltenAnfrage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_cli())
    }
}

pub fn fold_cli_token(s: &str) -> String {
    s.trim().to_lowercase()
}
