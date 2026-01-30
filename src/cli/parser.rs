// parser.rs - überarbeitete Version

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
    kategorie_map: Option<&crate::columnCategories_complete::KategorieMap> // NEU: KategorieMap als Parameter
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
            
            // NEUE VERARBEITUNG: --spalten für automatische Spaltensuche
            "--spalten" => {
                if let Some((_, ober)) = iter.next() {
                    if let Some((_, unter)) = iter.next() {
                        println!("🔍 Parameter --spalten erkannt: '{}' '{}'", ober, unter);
                        
                        // Speichere die gesuchten Kategorien
                        gesuchte_oberkategorie = ober.clone();
                        gesuchte_unterkategorie = unter.clone();
                        automatische_spalten_suche = true;
                        
                        // Setze auch die Spaltennamen
                        spalten_namen.oberkategorie = ober.clone();
                        spalten_namen.unterkategorie = unter.clone();
                        
                        println!("✅ Automatische Spaltensuche aktiviert für: '{}' → '{}'", 
                                 ober, unter);
                    } else {
                        println!("❌ Fehler: --spalten benötigt zwei Parameter (Oberkategorie Unterkategorie)");
                    }
                } else {
                    println!("❌ Fehler: --spalten benötigt zwei Parameter (Oberkategorie Unterkategorie)");
                }
            }
            
            // Alternative Option: --spaltenname (bleibt für SQL-Namen)
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
            
            // Weitere Optionen...
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
                        // Hier müsste der letzte Bereich aktualisiert werden
                        if let Some(last) = bereich.zeilen_bereiche.last_mut() {
                            last.1 = zahl;
                        }
                        println!("📍 Zeile bis gesetzt: {}", zahl);
                    }
                }
            }
            
            "--spaltevon" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.von_spalte = zahl;
                        bereich.spalten_bereiche.push((zahl, zahl));
                        println!("📍 Spalte von gesetzt: {}", zahl);
                    }
                }
            }
            
            "--spaltebis" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.bis_spalte = zahl;
                        // Hier müsste der letzte Bereich aktualisiert werden
                        if let Some(last) = bereich.spalten_bereiche.last_mut() {
                            last.1 = zahl;
                        }
                        println!("📍 Spalte bis gesetzt: {}", zahl);
                    }
                }
            }
            
            "--help" | "-h" => {
                println!("📖 Hilfe: Verfügbare Optionen:");
                println!("  --spalten OBER UNTER      Suche automatisch Spaltennummern für Kategorien");
                println!("  --spaltenname OBER UNTER  Setze Spaltennamen (für SQL)");
                println!("  --vorhervonausschnitt ZEILEN    Zeilenbereiche auswählen");
                println!("  --spalten SPALTEN               Spaltenbereiche auswählen");
                println!("  --zeilevon ZAHL                 Startzeile setzen");
                println!("  --zeilebis ZAHL                 Endzeile setzen");
                println!("  --spaltevon ZAHL                Startspalte setzen");
                println!("  --spaltebis ZAHL                Endspalte setzen");
                println!("  --help, -h                      Diese Hilfe anzeigen");
                println!();
                println!("🔍 BEISPIEL für automatische Spaltensuche:");
                println!("  mein-rpnn --spalten Menschliches Motive");
                println!("  # Sucht automatisch alle Spaltennummern für 'Menschliches' → 'Motive'");
                println!();
                println!("📊 BEISPIEL mit konkretem Bereich:");
                println!("  mein-rpnn --spalten Universum Transzendentalien --zeilevon 1 --zeilebis 10");
                println!();
                println!("📝 BEISPIEL für Zeilenangaben:");
                println!("  5               Einzelzeile");
                println!("  1-10            Bereich");
                println!("  1,3,5,7         Einzelne Zeilen");
                println!("  1-3,5,7-9       Kombination");
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

        if !arg.starts_with("--") {  // Nur für nicht-optionale Argumente
            println!("🔍 Argument {}: '{}' → {} Minuszeichen → '{}'",
                    i + 1, arg, dash_count, param);
        }

        minuses.push(dash_count);
        params.push(param);
    }

    // NACH DER OPTIONEN-VERARBEITUNG: Automatische Spaltensuche durchführen
    if automatische_spalten_suche && kategorie_map.is_some() {
        let kategorie_map = kategorie_map.unwrap();
        
        println!("\n🔍 Führe automatische Spaltensuche durch...");
        println!("   Oberkategorie: '{}'", gesuchte_oberkategorie);
        println!("   Unterkategorie: '{}'", gesuchte_unterkategorie);
        
        let gefundene_spalten = kategorie_map.finde_spaltennummern_fuer_kategorien(
            &gesuchte_oberkategorie,
            &gesuchte_unterkategorie
        );
        
        if gefundene_spalten.is_empty() {
            println!("❌ FEHLER: Keine Spaltennummern gefunden für '{}' → '{}'", 
                     gesuchte_oberkategorie, gesuchte_unterkategorie);
            println!("ℹ️  Mögliche Gründe:");
            println!("   - Die Kategorie-Kombination existiert nicht in der Datenbank");
            println!("   - Tippfehler in den Kategorienamen");
            println!("   - Die Kategorie hat keine zugeordneten Spaltennummern");
            println!("   - Die Kategoriedaten wurden nicht korrekt geladen");
            
            // Vorschläge für ähnliche Kategorien
            println!("\n🔍 Ähnliche Oberkategorien:");
            let mut ähnliche_ober = Vec::new();
            for eintrag in &kategorie_map.alle_eintraege {
                if eintrag.oberkategorie.to_lowercase().contains(&gesuchte_oberkategorie.to_lowercase()) {
                    ähnliche_ober.push(eintrag.oberkategorie.clone());
                }
            }
            
            let ähnliche_ober_set: std::collections::HashSet<_> = ähnliche_ober.into_iter().collect();
            for kategorie in ähnliche_ober_set.iter().take(5) {
                println!("   - {}", kategorie);
            }
            
            println!("\n⚠️  KEINE SPALTEN gefunden. Das Programm wird wahrscheinlich fehlschlagen.");
            println!("ℹ️  Versuche es mit: --spalten Menschliches Motive");
        } else {
            println!("✅ Gefundene Spaltennummern: {:?}", gefundene_spalten);
            println!("📊 Anzahl: {} Spaltennummern", gefundene_spalten.len());
            
            // Spaltenbereiche in der TextBereich-Struktur setzen
            let mut bereich_fuer_spalten = TextBereich::default();
            let mut sorted: Vec<usize> = gefundene_spalten.iter().map(|&n| n as usize).collect();
            sorted.sort();
            
            for &num in &sorted {
                bereich_fuer_spalten.spalten_bereiche.push((num, num));
            }
            
            bereich.spalten_bereiche = bereich_fuer_spalten.spalten_bereiche;
            
            if !bereich.spalten_bereiche.is_empty() {
                bereich.von_spalte = bereich.spalten_bereiche[0].0;
                bereich.bis_spalte = bereich.spalten_bereiche.last().unwrap().1;
                println!("📊 Automatisch erzeugte Spaltenbereiche: {:?}", bereich.spalten_bereiche);
                println!("📍 von_spalte: {}, bis_spalte: {}", bereich.von_spalte, bereich.bis_spalte);
                
                // Generiere SQL für die gefundenen Spalten
                let sql = kategorie_map.generiere_sql_selects(
                    &spalten_namen.oberkategorie,
                    &spalten_namen.unterkategorie,
                    Some(&sorted)
                );
                println!("\n📝 SQL für gefundene Spalten generiert ({} Zeilen)", sql.lines().count());
            }
        }
    } else if automatische_spalten_suche && kategorie_map.is_none() {
        println!("⚠️  WARNUNG: Automatische Spaltensuche angefordert, aber keine KategorieMap verfügbar");
        println!("ℹ️  Die Kategoriedaten wurden nicht geladen. Verwende Standard-Spalte 1.");
        bereich.spalten_bereiche.push((1, 1));
        bereich.von_spalte = 1;
        bereich.bis_spalte = 1;
    }

    (minuses, params, bereich, spalten_namen)
}

// Hilfsfunktionen bleiben gleich...
pub(crate) fn parse_zeilenangabe_zu_bereichen(text: &str) -> Option<Vec<(usize, usize)>> {
    println!("🔧 Parse zu Bereichen: '{}'", text);
    
    let mut bereiche = Vec::new();
    
    // 1. Versuche Generator-Notation
    if let Some(zahlen) = str_as_generator_to_vec_i64(text) {
        println!("🎯 Generator-Notation erkannt: {:?}", zahlen);
        
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
            bereiche.sort_by(|a, b| a.0.cmp(&b.0));
            return Some(bereiche);
        }
    }
    
    // 2. MANUELLE Parsing für Komma-getrennte Zahlen
    let teile: Vec<&str> = text.split(',').collect();
    println!("🔪 Zerlegt nach Kommas in {} Teile: {:?}", teile.len(), teile);
    
    // Wenn nur ein Teil vorhanden ist
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
            
            // Prüfe ob es ein Bereich ist
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
