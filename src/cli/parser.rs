// parser.rs - korrigierte Version
use crate::ifIsZeilenAngabe::{is_zeilen_angabe, str_as_generator_to_vec_i64};
use super::bereich::TextBereich;

// Neuer Datentyp für Spaltennamen-Konfiguration
#[derive(Debug, Clone)]
pub struct SpaltenNamen {
    pub oberkategorie: String,
    pub unterkategorie: String,
}

impl Default for SpaltenNamen {
    fn default() -> Self {
        Self {
            oberkategorie: "oberkategorie".to_string(),
            unterkategorie: "unterkategorie".to_string(),
        }
    }
}

// Rückgabetyp erweitert um Spaltennamen
pub fn parse_cli_args(
    args: &[String], 
    kategorie_map: Option<&crate::columnCategories_complete::KategorieMap>
) -> (Vec<usize>, Vec<String>, TextBereich, SpaltenNamen) {
    let mut minuses = Vec::with_capacity(args.len());
    let mut params = Vec::with_capacity(args.len());

    let mut bereich = TextBereich::default();
    let mut spalten_namen = SpaltenNamen::default();
    
    let mut automatische_spalten_suche = false;
    let mut gesuchte_oberkategorie = String::new();
    let mut gesuchte_unterkategorie = String::new();

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
                    println!("📋 Verarbeite --vorhervonausschnitt: {}", nachfolger);
                    
                    if is_zeilen_angabe(nachfolger) {
                        if let Some(bereichspaare) = parse_zeilenangabe_zu_bereichen(nachfolger) {
                            if !bereichspaare.is_empty() {
                                bereich.zeilen_bereiche = bereichspaare.clone();
                                bereich.von_zeile = bereichspaare[0].0;
                                
                                if let Some(last_bereich) = bereichspaare.last() {
                                    bereich.bis_zeile = last_bereich.1;
                                }
                            }
                        } else if let Ok(zahl) = nachfolger.parse::<usize>() {
                            bereich.zeilen_bereiche.push((zahl, zahl));
                            bereich.von_zeile = zahl;
                            bereich.bis_zeile = zahl;
                        }
                    }
                }
            }
            
            "--spalten" => {
                if let Some((_, ober)) = iter.next() {
                    if let Some((_, unter)) = iter.next() {
                        println!("🔍 Parameter --spalten: '{}' '{}'", ober, unter);
                        bereich.spalten_gesucht = true; 
                        gesuchte_oberkategorie = ober.clone();
                        gesuchte_unterkategorie = unter.clone();
                        automatische_spalten_suche = true;
                        
                        spalten_namen.oberkategorie = ober.clone();
                        spalten_namen.unterkategorie = unter.clone();
                        
                        println!("✅ Suche: '{}' → '{}'", ober, unter);
                    }
                }
            }
            
            "--spaltenname" => {
                if let Some((_, name1)) = iter.next() {
                    if let Some((_, name2)) = iter.next() {
                        spalten_namen.oberkategorie = name1.clone();
                        spalten_namen.unterkategorie = name2.clone();
                        bereich.spalten_gesucht = true; 
                    }
                }
            }
            
            "--zeilevon" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.von_zeile = zahl;
                        if bereich.zeilen_bereiche.is_empty() {
                            bereich.zeilen_bereiche.push((zahl, zahl));
                        } else if let Some(last) = bereich.zeilen_bereiche.last_mut() {
                            last.0 = zahl;
                        }
                    }
                }
            }
            
            "--zeilebis" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.bis_zeile = zahl;
                        if let Some(last) = bereich.zeilen_bereiche.last_mut() {
                            last.1 = zahl;
                        }
                    }
                }
            }
            
            "--spaltevon" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.von_spalte = zahl;
                        bereich.spalten_bereiche.push((zahl, zahl));
                        bereich.spalten_gesucht = true; 
                    }
                }
            }
            
            "--spaltebis" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.bis_spalte = zahl;
                        if let Some(last) = bereich.spalten_bereiche.last_mut() {
                            last.1 = zahl;
                            bereich.spalten_gesucht = true; 
                        }
                    }
                }
            }
            
            "--help" | "-h" => {
                println!("📖 Hilfe:");
                println!("  --spalten OBER UNTER      Suche Spaltennummern für Kategorien");
                println!("  --spaltenname OBER UNTER  Setze Spaltennamen (für SQL)");
                println!("  --vorhervonausschnitt Z   Zeilenbereiche (z.B. 1-10, 1,3,5)");
                println!("  --zeilevon ZAHL           Startzeile");
                println!("  --zeilebis ZAHL           Endzeile");
                println!("  --spaltevon ZAHL          Startspalte");
                println!("  --spaltebis ZAHL          Endspalte");
                println!("  --help, -h                Diese Hilfe");
                println!();
                println!("Beispiele:");
                println!("  mein-rpnn --spalten Menschliches Motive");
                println!("  mein-rpnn --spalten Universum Transzendentalien --zeilevon 1 --zeilebis 10");
            }
            
            _ => {
                // Keine Ausgabe für unbekannte Parameter
            }
        }

        let param = if dash_count > 0 {
            arg.chars().skip(dash_count).collect()
        } else {
            arg.clone()
        };

        minuses.push(dash_count);
        params.push(param);
    }

    // Automatische Spaltensuche durchführen
    if automatische_spalten_suche {
        if let Some(kategorie_map) = kategorie_map {
            // SUCHFUNKTION AUFRUFEN - KORREKTE SYNTAX
            // ERSETZEN durch exakte Suche:
            let gefundene_spalten = kategorie_map.finde_spaltennummern_exakt(
                &gesuchte_oberkategorie,
                &gesuchte_unterkategorie
            );
                       
            if !gefundene_spalten.is_empty() {
                /*
                println!("❌ Keine Spaltennummern gefunden für '{}' → '{}'", 
                         gesuchte_oberkategorie, gesuchte_unterkategorie);
                
                // Zeige Vorschläge
                let mut vorschlaege = Vec::new();
                
                // Finde ähnliche Oberkategorien
                for eintrag in &kategorie_map.alle_eintraege {
                    if eintrag.oberkategorie.to_lowercase().contains(
                        &gesuchte_oberkategorie.to_lowercase()
                    ) {
                        vorschlaege.push(eintrag.oberkategorie.clone());
                    }
                }
                
                if !vorschlaege.is_empty() {
                    println!("ℹ️  Ähnliche Oberkategorien:");
                    let unique: std::collections::HashSet<_> = vorschlaege.into_iter().collect();
                    for kat in unique.iter().take(5) {
                        println!("   - {}", kat);
                    }
                }
                
                println!("⚠️  Verwende Standard-Spalte 1");
                bereich.spalten_bereiche.push((1, 1));
                bereich.von_spalte = 1;
                bereich.bis_spalte = 1;
                panic!("Keine Spalten gefunden für: '{}' → '{}'",
           gesuchte_oberkategorie, gesuchte_unterkategorie);
            } else {*/
                println!("✅ {} Spaltennummern gefunden", gefundene_spalten.len());
                
                // Spaltenbereiche setzen
                let mut sorted: Vec<usize> = gefundene_spalten.iter().map(|&n| n as usize).collect();
                sorted.sort();
                sorted.dedup();
                
                bereich.spalten_bereiche.clear();
                for &num in &sorted {
                    bereich.spalten_bereiche.push((num, num));
                }
                
                if !bereich.spalten_bereiche.is_empty() {
                    bereich.von_spalte = bereich.spalten_bereiche[0].0;
                    bereich.bis_spalte = bereich.spalten_bereiche.last().unwrap().1;
                }
            }
        } else {
            println!("⚠️  Kategoriedaten nicht verfügbar");
            bereich.spalten_bereiche.push((1, 1));
            bereich.von_spalte = 1;
            bereich.bis_spalte = 1;
        }
    }

    (minuses, params, bereich, spalten_namen)
}

// Hilfsfunktion zum Parsen von Zeilenangaben (bereinigt)
pub(crate) fn parse_zeilenangabe_zu_bereichen(text: &str) -> Option<Vec<(usize, usize)>> {
    let mut bereiche = Vec::new();
    
    // 1. Versuche Generator-Notation
    if let Some(zahlen) = str_as_generator_to_vec_i64(text) {
        for &zahl in &zahlen {
            if zahl >= 0 {
                bereiche.push((zahl as usize, zahl as usize));
            }
        }
        
        if !bereiche.is_empty() {
            bereiche.sort_by(|a, b| a.0.cmp(&b.0));
            return Some(bereiche);
        }
    }
    
    // 2. Manuelles Parsing
    let teile: Vec<&str> = text.split(',').collect();
    
    if teile.len() == 1 {
        let teil = teile[0].trim();
        
        if teil.contains('-') {
            let bereichs_teile: Vec<&str> = teil.split('-').collect();
            if bereichs_teile.len() == 2 {
                if let (Ok(von), Ok(bis)) = (
                    bereichs_teile[0].trim().parse::<usize>(),
                    bereichs_teile[1].trim().parse::<usize>()
                ) {
                    bereiche.push((von, bis));
                }
            }
        } else if let Ok(num) = teil.parse::<usize>() {
            bereiche.push((num, num));
        }
    } else {
        for teil in teile {
            let teil_trimmed = teil.trim();
            
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
                        bereiche.push((von, bis));
                        continue;
                    }
                }
            }
            
            if let Ok(num) = teil_trimmed.parse::<usize>() {
                bereiche.push((num, num));
            }
        }
    }
    
    if !bereiche.is_empty() {
        bereiche.sort_by(|a, b| a.0.cmp(&b.0));
        Some(bereiche)
    } else {
        None
    }
}
