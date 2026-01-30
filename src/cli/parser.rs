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
pub fn parse_cli_args(args: &[String]) -> (Vec<usize>, Vec<String>, TextBereich, SpaltenNamen) {
    let mut minuses = Vec::with_capacity(args.len());
    let mut params = Vec::with_capacity(args.len());

    let mut bereich = TextBereich::default();
    let mut spalten_namen = SpaltenNamen::default();

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
            // NEUE OPTION: Spaltennamen für SQL
            "--spaltenname" => {
                if let Some((_, name1)) = iter.next() {
                    if let Some((_, name2)) = iter.next() {
                        println!("📝 Setze Spaltennamen: Oberkategorie='{}', Unterkategorie='{}'", 
                                 name1, name2);
                        spalten_namen.oberkategorie = name1.clone();
                        spalten_namen.unterkategorie = name2.clone();
                        println!("✅ Spaltennamen gespeichert: {:?}", spalten_namen);
                    } else {
                        println!("❌ Fehler: --spaltenname benötigt zwei Namen");
                    }
                } else {
                    println!("❌ Fehler: --spaltenname benötigt zwei Namen");
                }
            }
            // Hilfe-Option
            "--help" | "-h" => {
                println!("📖 Hilfe: Verfügbare Optionen:");
                println!("  --vorhervonausschnitt ZEILEN    Zeilenbereiche auswählen");
                println!("  --spalten SPALTEN               Spaltenbereiche auswählen");
                println!("  --zeilevon ZAHL                 Startzeile setzen");
                println!("  --zeilebis ZAHL                 Endzeile setzen");
                println!("  --spaltevon ZAHL                Startspalte setzen");
                println!("  --spaltebis ZAHL                Endspalte setzen");
                println!("  --spaltenname NAME1 NAME2       Spaltennamen für SQL setzen");
                println!("  --help, -h                      Diese Hilfe anzeigen");
                println!();
                println!("📝 Beispiele für Zeilenangaben:");
                println!("  5               Einzelzeile");
                println!("  1-10            Bereich");
                println!("  1,3,5,7         Einzelne Zeilen");
                println!("  1-3,5,7-9       Kombination");
                println!("  1..10           Generator-Notation");
                println!("  1..=10          Inklusive Generator");
                println!("  1..10:2         Generator mit Schrittweite");
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
    }

    (minuses, params, bereich, spalten_namen)
}

pub(crate) fn parse_zeilenangabe_zu_bereichen(text: &str) -> Option<Vec<(usize, usize)>> {
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
            // Sortiere aber fasse NICHT zusammen!
            bereiche.sort_by(|a, b| a.0.cmp(&b.0));
            return Some(bereiche);
        }
    }
    
    // 2. MANUELLE Parsing für Komma-getrennte Zahlen
    // Teile zuerst nach Kommas auf
    let teile: Vec<&str> = text.split(',').collect();
    println!("🔪 Zerlegt nach Kommas in {} Teile: {:?}", teile.len(), teile);
    
    // Wenn nur ein Teil vorhanden ist, versuche es mit der originalen Logik
    if teile.len() == 1 {
        // Original-Logik für Bereichs-Notation (1-5) oder einzelne Zahl
        let teil = teile[0].trim();
        
        if teil.contains('-') {
            let bereichs_teile: Vec<&str> = teil.split('-').collect();
            if bereichs_teile.len() == 2 {
                if let (Ok(von), Ok(bis)) = (
                    bereichs_teile[0].trim().parse::<usize>(),
                    bereichs_teile[1].trim().parse::<usize>()
                ) {
                    bereiche.push((von, bis));
                    println!("📈 Bereich {}-{} hinzugefügt", von, bis);
                }
            }
        } else if let Ok(num) = teil.parse::<usize>() {
            bereiche.push((num, num));
            println!("➕ Einzelzahl {} hinzugefügt", num);
        }
    } else {
        // Mehrere Teile = Komma-getrennte Zahlen
        for teil in teile {
            let teil_trimmed = teil.trim();
            
            if teil_trimmed.is_empty() {
                continue;
            }
            
            // Prüfe ob es ein Bereich ist (kann in Komma-Liste vorkommen: "1-3,5")
            if teil_trimmed.contains('-') {
                let bereichs_teile: Vec<&str> = teil_trimmed.split('-').collect();
                if bereichs_teile.len() == 2 {
                    if let (Ok(von), Ok(bis)) = (
                        bereichs_teile[0].trim().parse::<usize>(),
                        bereichs_teile[1].trim().parse::<usize>()
                    ) {
                        bereiche.push((von, bis));
                        println!("📈 Bereich {}-{} hinzugefügt", von, bis);
                        continue;
                    }
                }
            }
            
            // Einzelne Zahl
            if let Ok(num) = teil_trimmed.parse::<usize>() {
                bereiche.push((num, num));
                println!("➕ Einzelzahl {} hinzugefügt", num);
            } else {
                println!("⚠  Konnte '{}' nicht als Zahl parsen", teil_trimmed);
            }
        }
    }
    
    if !bereiche.is_empty() {
        // WICHTIG: Nur sortieren, NICHT zusammenfassen für einzelne Zahlen!
        bereiche.sort_by(|a, b| a.0.cmp(&b.0));
        println!("✅ Insgesamt {} Bereichspaare (sortiert): {:?}", bereiche.len(), bereiche);
        Some(bereiche)
    } else {
        println!("❌ Keine gültigen Bereiche gefunden");
        None
    }
}

// Hilfsfunktion zur Ausgabe der SQL-SELECT Anweisungen
pub fn generate_sql_select(spalten_namen: &SpaltenNamen) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("-- SQL SELECT Anweisungen mit Spaltennamen: '{}', '{}'\n", 
                            spalten_namen.oberkategorie, spalten_namen.unterkategorie));
    output.push_str("\n");
    
    // 1. Einfache SELECTs
    output.push_str("-- 1. Alle Einträge mit Ober- und Unterkategorie\n");
    output.push_str(&format!("SELECT {}, {}, spaltennummer\n", 
                           spalten_namen.oberkategorie, spalten_namen.unterkategorie));
    output.push_str(&format!("FROM kategorie_tabelle\n"));
    output.push_str(&format!("WHERE {} IS NOT NULL AND {} IS NOT NULL\n", 
                           spalten_namen.oberkategorie, spalten_namen.unterkategorie));
    output.push_str("ORDER BY spaltennummer;\n\n");
    
    // 2. Einzigartige Kategorien
    output.push_str("-- 2. Einzigartige Oberkategorien\n");
    output.push_str(&format!("SELECT DISTINCT {}\n", spalten_namen.oberkategorie));
    output.push_str(&format!("FROM kategorie_tabelle\n"));
    output.push_str(&format!("ORDER BY {};\n\n", spalten_namen.oberkategorie));
    
    // 3. Unterkategorien für bestimmte Oberkategorie
    output.push_str("-- 3. Unterkategorien für eine bestimmte Oberkategorie\n");
    output.push_str(&format!("SELECT DISTINCT {}\n", spalten_namen.unterkategorie));
    output.push_str(&format!("FROM kategorie_tabelle\n"));
    output.push_str(&format!("WHERE {} = 'Menschliches'\n", spalten_namen.oberkategorie));
    output.push_str(&format!("ORDER BY {};\n\n", spalten_namen.unterkategorie));
    
    // 4. Komplexe Abfrage mit JOIN
    output.push_str("-- 4. Komplexe Abfrage mit JOIN auf Datentabelle\n");
    output.push_str(&format!("SELECT k.{} as oberkategorie, k.{} as unterkategorie,\n", 
                           spalten_namen.oberkategorie, spalten_namen.unterkategorie));
    output.push_str("       k.spaltennummer, d.*\n");
    output.push_str("FROM kategorie_tabelle k\n");
    output.push_str("JOIN daten_tabelle d ON k.spaltennummer = d.id\n");
    output.push_str(&format!("WHERE k.{} = 'Universum'\n", spalten_namen.oberkategorie));
    output.push_str("ORDER BY k.spaltennummer;\n\n");
    
    // 5. CREATE TABLE Statement
    output.push_str("-- 5. CREATE TABLE Statement\n");
    output.push_str("CREATE TABLE kategorie_tabelle (\n");
    output.push_str("  id INTEGER PRIMARY KEY AUTOINCREMENT,\n");
    output.push_str(&format!("  {} VARCHAR(255) NOT NULL,\n", spalten_namen.oberkategorie));
    output.push_str(&format!("  {} VARCHAR(255) NOT NULL,\n", spalten_namen.unterkategorie));
    output.push_str("  spaltennummer INTEGER NOT NULL\n");
    output.push_str(");\n");
    
    output
}
