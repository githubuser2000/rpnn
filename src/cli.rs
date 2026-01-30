// src/cli.rs
// Ändere den Import:
use crate::ifIsZeilenAngabe::{
    is_zeilen_angabe, 
    str_as_generator_to_list_of_num_strs, 
    str_as_generator_to_vec_i64,  // NEU hinzufügen
    split_with_bracket_balance
};

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

pub fn parse_cli_args(args: &[String]) -> (Vec<usize>, Vec<String>, TextBereich) {
    let mut minuses = Vec::with_capacity(args.len());
    let mut params = Vec::with_capacity(args.len());
    let mut dash_count_before = 0;

    let mut bereich = TextBereich::default();

    let mut iter = args.iter().enumerate();
    while let Some((i, arg)) = iter.next() {
        let mut dash_count = 0;
        
        for c in arg.chars() {
            if c == '-' {
                dash_count += 1;
            } else {
                break;
            }
        }

        match arg.as_str() {
            "--vorhervonausschnitt" => {
                if let Some((_, nachfolger)) = iter.next() {
                    println!("📋 Verarbeite --vorhervonausschnitt mit Wert: '{}'", nachfolger);
                    
                    if is_zeilen_angabe(nachfolger) {
                        println!("✓ '{}' ist eine gültige Zeilenangabe", nachfolger);
                        
                        if let Some(bereichspaare) = parse_zeilenangabe_zu_bereichen(nachfolger) {
                            if !bereichspaare.is_empty() {
                                bereich.zeilen_bereiche = bereichspaare.clone();
                                bereich.von_zeile = bereichspaare[0].0;
                                
                                if let Some(last_bereich) = bereichspaare.last() {
                                    bereich.bis_zeile = last_bereich.1;
                                }
                                
                                println!("📊 Gespeicherte Zeilenbereiche: {:?}", bereich.zeilen_bereiche);
                                println!("📍 von_zeile: {}, bis_zeile: {}", 
                                         bereich.von_zeile, bereich.bis_zeile);
                                
                                let gesamt_zeilen: usize = bereichspaare.iter()
                                    .map(|(von, bis)| bis - von + 1)
                                    .sum();
                                println!("📈 Gesamtzahl der Zeilen: {}", gesamt_zeilen);
                            }
                        } else {
                            println!("⚠  Konnte Bereiche nicht parsen, obwohl Angabe gültig");
                            if let Ok(zahl) = nachfolger.parse::<usize>() {
                                bereich.zeilen_bereiche.push((zahl, zahl));
                                bereich.von_zeile = zahl;
                                bereich.bis_zeile = zahl;
                                println!("📝 Einzelzeile als Bereich gespeichert: ({},{})", zahl, zahl);
                            }
                        }
                    } else {
                        println!("✗ '{}' ist keine gültige Zeilenangabe", nachfolger);
                    }
                } else {
                    println!("❌ Fehler: --vorhervonausschnitt benötigt einen Wert");
                }
            }
            "--spalten" => {
                if let Some((_, nachfolger)) = iter.next() {
                    println!("📋 Verarbeite --spalten mit Wert: '{}'", nachfolger);
                    
                    if let Some(spalten_bereiche) = parse_zeilenangabe_zu_bereichen(nachfolger) {
                        bereich.spalten_bereiche = spalten_bereiche;
                        
                        if !bereich.spalten_bereiche.is_empty() {
                            bereich.von_spalte = bereich.spalten_bereiche[0].0;
                            if let Some(last_bereich) = bereich.spalten_bereiche.last() {
                                bereich.bis_spalte = last_bereich.1;
                            }
                        }
                        
                        println!("📊 Gespeicherte Spaltenbereiche: {:?}", bereich.spalten_bereiche);
                    }
                }
            }
            "--zeilevon" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.von_zeile = zahl;
                        bereich.zeilen_bereiche.push((zahl, zahl));
                        println!("📍 Zeile von gesetzt und als Bereich gespeichert: ({},{})", zahl, zahl);
                    }
                }
            }
            "--zeilebis" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.bis_zeile = zahl;
                        if bereich.zeilen_bereiche.is_empty() {
                            bereich.zeilen_bereiche.push((zahl, zahl));
                        } else if let Some(last) = bereich.zeilen_bereiche.last_mut() {
                            last.1 = zahl;
                        }
                        println!("📍 Zeile bis gesetzt auf: {}", zahl);
                    }
                }
            }
            "--spaltevon" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.von_spalte = zahl;
                        bereich.spalten_bereiche.push((zahl, zahl));
                        println!("📍 Spalte von gesetzt und als Bereich gespeichert: ({},{})", zahl, zahl);
                    }
                }
            }
            "--spaltebis" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.bis_spalte = zahl;
                        if bereich.spalten_bereiche.is_empty() {
                            bereich.spalten_bereiche.push((zahl, zahl));
                        } else if let Some(last) = bereich.spalten_bereiche.last_mut() {
                            last.1 = zahl;
                        }
                        println!("📍 Spalte bis gesetzt auf: {}", zahl);
                    }
                }
            }
            _ => {
                if dash_count == 0 {
                    println!("📦 Parameter: {}", arg);
                }
            }
        }

        let param = if dash_count > 0 {
            arg.chars().skip(dash_count).collect()
        } else {
            arg.clone()
        };

        println!("🔍 Argument {}: '{}' → {} Minuszeichen → '{}'",
                i + 1, arg, dash_count, param);

        minuses.push(dash_count);
        params.push(param);
        dash_count_before = dash_count;
    }

    (minuses, params, bereich)
}
// In cli.rs, ersetze den Generator-Parsing Teil:
fn parse_zeilenangabe_zu_bereichen(text: &str) -> Option<Vec<(usize, usize)>> {
    println!("🔧 Parse zu Bereichen: '{}'", text);
    
    let mut bereiche = Vec::new();
    
    // 1. Versuche Generator-Notation mit der besseren Funktion
    if let Some(zahlen) = str_as_generator_to_vec_i64(text) {
        println!("🎯 Generator-Notation erkannt: {:?}", zahlen);
        
        // Konvertiere i64 zu usize (mit Überlauf-Prüfung)
        for &zahl in &zahlen {
            if zahl >= 0 {
                let zahl_usize = zahl as usize;
                bereiche.push((zahl_usize, zahl_usize));
            } else {
                println!("⚠  Negative Zahl {} in Generator ignoriert", zahl);
            }
        }
        
        if !bereiche.is_empty() {
            println!("📊 Aus Generator erstellte Bereiche: {:?}", bereiche);
            return Some(bereiche);
        }
    }
    
    // 2. Zerlege in Komponenten
    let teile: Vec<&str> = split_with_bracket_balance(text);
    println!("🔪 Zerlegt in {} Teile: {:?}", teile.len(), teile);
    
    for teil in teile {
        let teil_trimmed: &str = teil;
        let teil_trimmed = teil_trimmed.trim();
        
        if teil_trimmed.is_empty() {
            continue;
        }
        
        if teil_trimmed.contains('-') {
            let bereichs_teile: Vec<&str> = teil_trimmed.split('-').collect();
            if bereichs_teile.len() == 2 {
                if let (Ok(von), Ok(bis)) = (
                    bereichs_teile[0].trim().parse::<usize>(),
                    bereichs_teile[1].trim().parse::<usize>()
                ) {
                    if von <= bis {
                        bereiche.push((von, bis));
                        println!("📈 Bereich {}-{} hinzugefügt", von, bis);
                    } else {
                        bereiche.push((bis, von));
                        println!("🔄 Bereich {}-{} getauscht zu {}-{}", von, bis, bis, von);
                    }
                    continue;
                }
            }
        }
        
        let mut num_str = teil_trimmed;
        if num_str.starts_with('v') {
            num_str = &num_str[1..];
        }
        
        if let Ok(num) = num_str.parse::<usize>() {
            bereiche.push((num, num));
            println!("➕ Einzelzahl {} als Bereich ({},{}) hinzugefügt", num, num, num);
        } else {
            println!("⚠  Konnte '{}' nicht als Zahl parsen", teil_trimmed);
        }
    }
    
    if !bereiche.is_empty() {
        let bereiche_geordnet = sortiere_und_fasse_zusammen(bereiche);
        println!("✅ Insgesamt {} Bereichspaare: {:?}", bereiche_geordnet.len(), bereiche_geordnet);
        Some(bereiche_geordnet)
    } else {
        println!("❌ Keine gültigen Bereiche gefunden");
        None
    }
}

fn sortiere_und_fasse_zusammen(mut bereiche: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if bereiche.is_empty() {
        return Vec::new();
    }
    
    bereiche.sort_by(|a, b| a.0.cmp(&b.0));
    
    let mut result = Vec::new();
    let mut aktuell = bereiche[0];
    
    for &(von, bis) in &bereiche[1..] {
        if von <= aktuell.1 + 1 {
            if bis > aktuell.1 {
                aktuell.1 = bis;
            }
        } else {
            result.push(aktuell);
            aktuell = (von, bis);
        }
    }
    
    result.push(aktuell);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_einzelne_zahlen_zu_bereichen() {
        let args = vec![
            "--vorhervonausschnitt".to_string(),
            "(1,3,5,7,9)".to_string(),
        ];
        
        let (_, _, bereich) = parse_cli_args(&args);
        assert_eq!(bereich.zeilen_bereiche, vec![(1,1), (3,3), (5,5), (7,7), (9,9)]);
        assert_eq!(bereich.von_zeile, 1);
        assert_eq!(bereich.bis_zeile, 9);
    }
    
    #[test]
    fn test_parse_bereich_zu_bereich() {
        let args = vec![
            "--vorhervonausschnitt".to_string(),
            "1-5".to_string(),
        ];
        
        let (_, _, bereich) = parse_cli_args(&args);
        assert_eq!(bereich.zeilen_bereiche, vec![(1, 5)]);
        assert_eq!(bereich.von_zeile, 1);
        assert_eq!(bereich.bis_zeile, 5);
    }
}
