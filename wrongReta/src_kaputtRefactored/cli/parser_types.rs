use crate::domain::model::spalten_anfrage::SpaltenAnfrage as CanonicalSpaltenAnfrage;

#[derive(Debug, Clone)]
pub struct SpaltenNamen {
    pub oberkategorie: String,
    pub unterkategorie: String,
    pub typed_request: Option<CanonicalSpaltenAnfrage>,
}

#[derive(Debug, Clone, Default)]
pub struct SpaltenNamenListe {
    pub eintraege: Vec<SpaltenNamen>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpaltenAuswahlModus {
    #[default]
    Explizit,
    Alle,
}

impl Default for SpaltenNamen {
    fn default() -> Self {
        Self {
            oberkategorie: String::new(),
            unterkategorie: String::new(),
            typed_request: None,
        }
    }
}
