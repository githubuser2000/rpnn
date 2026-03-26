use std::fmt;

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
    UniversumMetaKonkret,
    Sonstige(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpaltenAnfrage {
    Standard { ober: StandardOberkategorie, unter: String },
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
        match normalize_key(input).as_str() {
            "menschliches" => Self::Menschliches,
            "universum" => Self::Universum,
            "religion" | "religionen" => Self::Religion,
            "planet" => Self::Planet,
            "galaxie" => Self::Galaxie,
            "multiversum" => Self::Multiversum,
            "grundstrukturen" => Self::Grundstrukturen,
            "bedeutung" => Self::Bedeutung,
            "procontra" => Self::ProContra,
            "wichtigsteszumverstehen" => Self::WichtigstesZumVerstehen,
            "eigenschaftenn" => Self::EigenschaftenN,
            "universummetakonkret" => Self::UniversumMetaKonkret,
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
            Self::UniversumMetaKonkret => "universummetakonkret",
            Self::Sonstige(s) => s.as_str(),
        }
    }
}

impl SpaltenAnfrage {
    pub fn parse(ober: &str, unter: &str) -> Result<Self, ParseSpaltenAnfrageError> {
        let unter = unter.trim();
        if unter.is_empty() {
            return Err(ParseSpaltenAnfrageError::EmptyUnterkategorie);
        }
        let ober_norm = normalize_key(ober);
        let unter = unter.to_string();
        let parsed = match ober_norm.as_str() {
            "kombinationgalaxie" => Self::KombinationGalaxie { unter },
            "kombinationuniversum" => Self::KombinationUniversum { unter },
            "gebrochenrationalgalaxienm" => Self::GebrochenRationalGalaxie { unter },
            "gebrochenrationaluniversumnm" => Self::GebrochenRationalUniversum { unter },
            "gebrochenrationalgefuehlenm" | "gebrochenrationalgefuhlenm" => Self::GebrochenRationalGefuehle { unter },
            "gebrochenrationalstrukturgroessenm" => Self::GebrochenRationalStrukturgroesse { unter },
            "primvielfache" => Self::Primvielfache { unter },
            "multiplikationen" => Self::Multiplikationen { unter },
            _ => {
                let standard = StandardOberkategorie::parse(ober);
                match standard {
                    StandardOberkategorie::Sonstige(_) => Self::Unknown { ober: ober.to_string(), unter },
                    known => Self::Standard { ober: known, unter },
                }
            }
        };
        Ok(parsed)
    }

    pub fn to_cli(&self) -> String {
        match self {
            Self::Standard { ober, unter } => format!("--spaltenname {} {}", ober.as_cli_str(), unter),
            Self::KombinationGalaxie { unter } => format!("--spaltenname KombinationGalaxie {}", unter),
            Self::KombinationUniversum { unter } => format!("--spaltenname KombinationUniversum {}", unter),
            Self::GebrochenRationalGalaxie { unter } => format!("--spaltenname gebrochen-rational_Galaxie_n/m {}", unter),
            Self::GebrochenRationalUniversum { unter } => format!("--spaltenname gebrochen-rational_Universum_n/m {}", unter),
            Self::GebrochenRationalGefuehle { unter } => format!("--spaltenname gebrochen-rational_Gefühle_n/m {}", unter),
            Self::GebrochenRationalStrukturgroesse { unter } => format!("--spaltenname gebrochen-rational_Strukturgroesse_n/m {}", unter),
            Self::Primvielfache { unter } => format!("--spaltenname primvielfache {}", unter),
            Self::Multiplikationen { unter } => format!("--spaltenname multiplikationen {}", unter),
            Self::Unknown { ober, unter } => format!("--spaltenname {} {}", ober, unter),
        }
    }
}

impl fmt::Display for SpaltenAnfrage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_cli())
    }
}

fn normalize_key(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
}
