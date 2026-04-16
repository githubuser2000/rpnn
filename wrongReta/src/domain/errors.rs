use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratorError {
    EmptySelection,
    InvalidVisibleColumn { column: usize },
    InvalidParameter(String),
    RuleApplicationFailed { rule: &'static str, detail: String },
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySelection => write!(f, "Es wurden keine sichtbaren oder generierten Spalten ausgewählt."),
            Self::InvalidVisibleColumn { column } => write!(f, "Ungültige sichtbare Spalte: {}", column),
            Self::InvalidParameter(detail) => write!(f, "Ungültiger Generator-Parameter: {}", detail),
            Self::RuleApplicationFailed { rule, detail } => write!(f, "Generator-Regel '{}' fehlgeschlagen: {}", rule, detail),
        }
    }
}

impl std::error::Error for GeneratorError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseSpaltenAnfrageError {
    UnknownOberkategorie(String),
    EmptyUnterkategorie,
}

impl fmt::Display for ParseSpaltenAnfrageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownOberkategorie(v) => write!(f, "Unbekannte Oberkategorie: {}", v),
            Self::EmptyUnterkategorie => write!(f, "Unterkategorie darf nicht leer sein."),
        }
    }
}

impl std::error::Error for ParseSpaltenAnfrageError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestPipelineError {
    ParseSpaltenAnfrage(ParseSpaltenAnfrageError),
}

impl fmt::Display for RequestPipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseSpaltenAnfrage(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for RequestPipelineError {}
