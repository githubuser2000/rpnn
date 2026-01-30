use crate::ifIsZeilenAngabe::{is_zeilen_angabe, str_as_generator_to_vec_i64};
use super::bereich::TextBereich;

// Import für Kategorie-Funktionen (falls benötigt)
// use crate::columnCategories_complete::{lade_kategorie_map, KategorieMap};

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

// Funktion zum Suchen von Spaltennummern basierend auf Kategorienamen
// Diese Funktion kann von main.rs aufgerufen werden
pub fn finde_spaltennummern_fuer_kategorien(
    oberkategorie: &str,
    unterkategorie: &str,
    kategorie_map: &crate::columnCategories_complete::KategorieMap
) -> Vec<u32> {
    println!("🔍 Suche Spaltennummern für: '{}' → '{}'", oberkategorie, unterkategorie);
    
    let mut gefundene_nummern = Vec::new();
    
    // Durchsuche alle Einträge
    for eintrag in &kategorie_map.alle_eintraege {
        if eintrag.oberkategorie.eq_ignore_ascii_case(oberkategorie) ||
           eintrag.oberkategorie.replace("_", "").eq_ignore_ascii_case(oberkategorie) {
            
            if eintrag.unterkategorie.eq_ignore_ascii_case(unterkategorie) ||
               eintrag.unterkategorie.replace("_", "").eq_ignore_ascii_case(unterkategorie) {
                
                gefundene_nummern.extend_from_slice(&eintrag.spaltennummern);
                println!("✅ Gefunden: {} Spaltennummern", eintrag.spaltennummern.len());
            }
        }
    }
    
    // Entferne Duplikate und sortiere
    let mut unique: Vec<u32> = gefundene_nummern.into_iter().collect();
    unique.sort();
    unique.dedup();
    
    println!("📊 Insgesamt {} eindeutige Spaltennummern gefunden", unique.len());
    unique
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
            // NEUE OPTION: Spaltennamen für SQL
            "--spaltenname" => {
                if let Some((_, name1)) = iter.next() {
                    if let Some((_, name2)) = iter.next() {
                        println!("📝 Setze Spaltennamen: Oberkategorie='{}', Unterkategorie='{}'", 
                                 name1, name2);
                        spalten_namen.oberkategorie = name1.clone();
                        spalten_namen.unterkategorie = name2.clone();
                        println!("✅ Spaltennamen gespeichert: {:?}", spalten_namen);
                        
                        // JETZT: Spaltennummern automatisch suchen
                        println!("\n🔍 Suche automatisch nach Spaltennummern für diese Kategorien...");
                        
                        // Hier können wir die Kategoriedaten laden und suchen
                        // Das müsste aber in main.rs passieren, da wir hier 
                        // keine KategorieMap haben
                        
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
            // ... restliche Optionen
            "--help" | "-h" => {
                println!("📖 Hilfe: Verfügbare Optionen:");
                println!("  --vorhervonausschnitt ZEILEN    Zeilenbereiche auswählen");
                println!("  --spalten SPALTEN               Spaltenbereiche auswählen");
                println!("  --spaltenname OBER UNTER        Spaltennamen setzen + automatisch Spalten finden");
                println!("  --zeilevon ZAHL                 Startzeile setzen");
                println!("  --zeilebis ZAHL                 Endzeile setzen");
                println!("  --spaltevon ZAHL                Startspalte setzen");
                println!("  --spaltebis ZAHL                Endspalte setzen");
                println!("  --help, -h                      Diese Hilfe anzeigen");
                println!();
                println!("📝 Beispiel für automatische Spaltensuche:");
                println!("  mein-rpnn --spaltenname Menschliches Motive");
                println!("  # Sucht automatisch alle Spaltennummern für 'Menschliches' → 'Motive'");
                println!();
                println!("📝 Beispiele für Zeilenangaben:");
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
