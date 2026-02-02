// reta_ausgabe-tables.rs
use std::collections::HashMap;

#[derive(Debug)]
pub struct Tables {
    pub hoechste_zeile: HashMap<u32, i32>,
    pub keine_ueberschriften: bool,
    pub keine_leeren_inhalte: bool,
    pub spalten_vanilla_amount: usize,
    pub generated_spalten_parameter: HashMap<String, String>,
    pub religion_numbers: Vec<i32>,
}

impl Tables {
    pub fn new(hoechste_zeile: Option<i32>) -> Self {
        let default_hoechste_zeile = match hoechste_zeile {
            Some(value) => {
                let mut map = HashMap::new();
                map.insert(1024, value);
                map.insert(114, value);
                map
            }
            None => {
                let mut map = HashMap::new();
                map.insert(1024, 1024);
                map.insert(114, 163);
                map
            }
        };
        
        Tables {
            hoechste_zeile: default_hoechste_zeile,
            keine_ueberschriften: false,
            keine_leeren_inhalte: false,
            spalten_vanilla_amount: 0,
            generated_spalten_parameter: HashMap::new(),
            religion_numbers: Vec::new(),
        }
    }
}
