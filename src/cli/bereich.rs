// src/cli/bereich.rs
#[derive(Debug, Clone)]
pub struct TextBereich {
    pub von_zeile: usize,
    pub bis_zeile: usize,
    pub von_spalte: usize,
    pub bis_spalte: usize,
    pub zeilen_bereiche: Vec<(usize, usize)>,
    pub spalten_bereiche: Vec<(usize, usize)>,
}

impl Default for TextBereich {
    fn default() -> Self {
        Self {
            von_zeile: 0,
            bis_zeile: 0,
            von_spalte: 0,
            bis_spalte: 0,
            zeilen_bereiche: Vec::new(),
            spalten_bereiche: Vec::new(),
        }
    }
}

// Optional: Hilfsmethoden für TextBereich
impl TextBereich {
    pub fn ist_leer(&self) -> bool {
        self.zeilen_bereiche.is_empty() && self.spalten_bereiche.is_empty()
    }
    
    pub fn gesamt_zeilen(&self) -> usize {
        self.zeilen_bereiche.iter()
            .map(|(von, bis)| bis - von + 1)
            .sum()
    }
    
    pub fn gesamt_spalten(&self) -> usize {
        self.spalten_bereiche.iter()
            .map(|(von, bis)| bis - von + 1)
            .sum()
    }
    
    pub fn enthaelt_zeile(&self, zeile: usize) -> bool {
        self.zeilen_bereiche.iter()
            .any(|(von, bis)| zeile >= *von && zeile <= *bis)
    }
    
    pub fn enthaelt_spalte(&self, spalte: usize) -> bool {
        self.spalten_bereiche.iter()
            .any(|(von, bis)| spalte >= *von && spalte <= *bis)
    }
}
