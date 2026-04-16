#[derive(Debug, Clone)]
pub struct SpaltenNamen {
    pub oberkategorie: String,
    pub unterkategorie: String,
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
        }
    }
}
