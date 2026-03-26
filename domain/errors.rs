use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseSpaltenAnfrageError {
    UnknownOberkategorie(String),
    UnknownUnterkategorie { ober: String, unter: String },
    InvalidGebrochenRationalValue(String),
    InvalidGeneratorArgument { generator: String, argument: String },
}

impl fmt::Display for ParseSpaltenAnfrageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownOberkategorie(ober) => write!(f, "Unbekannte Oberkategorie: {}", ober),
            Self::UnknownUnterkategorie { ober, unter } => write!(f, "Unbekannte Unterkategorie '{}' für Oberkategorie '{}'", unter, ober),
            Self::InvalidGebrochenRationalValue(value) => write!(f, "Ungültiger gebrochen-rational Wert: {}", value),
            Self::InvalidGeneratorArgument { generator, argument } => write!(f, "Ungültiges Argument '{}' für Generator '{}'", argument, generator),
        }
    }
}

impl std::error::Error for ParseSpaltenAnfrageError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratorError {
    RuleFailed(&'static str),
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RuleFailed(rule) => write!(f, "Generator-Regel fehlgeschlagen: {}", rule),
        }
    }
}

impl std::error::Error for GeneratorError {}
