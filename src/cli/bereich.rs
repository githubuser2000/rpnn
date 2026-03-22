
// src/cli/bereich.rs
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct TextBereich {
    pub von_zeile: usize,
    pub bis_zeile: usize,
    pub von_spalte: usize,
    pub bis_spalte: usize,
    pub keineleereninhalte: bool,
    pub vorher_vielfache: bool,
    pub vorher_primfaktoren: bool,
    pub zeilen_bereiche: Vec<(usize, usize)>,
    pub spalten_bereiche: Vec<(usize, usize)>,
    pub spaltenreihenfolgeundnurdiese: Vec<usize>,
    pub breiten: Vec<usize>,
    pub spalten_gefunden: bool,
    pub spalten_gesucht: bool,
    pub spalten_gesucht2: bool,
    pub exact_generated_befehle: BTreeSet<String>,
    pub exact_modal_pairs: Vec<(usize, usize)>, // 0-basiert für concat_modallogik
    pub exact_meta_konkret_specs: Vec<(usize, usize)>, // (metavariable, side0or1)
    pub exact_visible_columns: Vec<usize>, // 1-basiert sichtbare Spalten aus exaktem Resolver
}

impl Default for TextBereich {
    fn default() -> Self {
        Self {
            keineleereninhalte: false,
            vorher_vielfache: false,
            vorher_primfaktoren: false,
            von_zeile: 0,
            bis_zeile: 0,
            von_spalte: usize::MAX,
            bis_spalte: usize::MAX,
            zeilen_bereiche: Vec::new(),
            spalten_bereiche: Vec::new(),
            breiten: Vec::new(),
            spaltenreihenfolgeundnurdiese: Vec::new(),
            spalten_gefunden: false,
            spalten_gesucht: false,
            spalten_gesucht2: false,
            exact_generated_befehle: BTreeSet::new(),
            exact_modal_pairs: Vec::new(),
            exact_meta_konkret_specs: Vec::new(),
            exact_visible_columns: Vec::new(),
        }
    }
}
