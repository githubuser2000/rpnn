// src/cli/bereich.rs
#[derive(Debug, Clone)]
pub struct TextBereich {
    pub von_zeile: usize,
    pub bis_zeile: usize,
    pub von_spalte: usize,
    pub bis_spalte: usize,
    pub keineleereninhalte: bool,
    pub zeilen_bereiche: Vec<(usize, usize)>,
    pub spalten_bereiche: Vec<(usize, usize)>,
    pub spaltenreihenfolgeundnurdiese: Vec<usize>,
    pub breiten: Vec<usize>,
    pub spalten_gefunden: bool,  // NEU: Wurden Spalten explizit gefunden?
    pub spalten_gesucht: bool,  // NEU: Wurden Spalten explizit gefunden?
    pub spalten_gesucht2: bool,  // NEU: Wurden Spalten explizit gefunden?
}

impl Default for TextBereich {
    fn default() -> Self {
        Self {
            keineleereninhalte: false,
            von_zeile: 0,
            bis_zeile: 0,
            von_spalte: usize::MAX,
            bis_spalte: usize::MAX,
            zeilen_bereiche: Vec::new(),
            spalten_bereiche: Vec::new(),
            breiten: Vec::new(),
            spaltenreihenfolgeundnurdiese: Vec::new(),
            spalten_gefunden: false,  // Standard: nicht gefunden
            spalten_gesucht: false,  // Standard: nicht gefunden
            spalten_gesucht2: false,  // Standard: nicht gefunden
        }
    }
}


