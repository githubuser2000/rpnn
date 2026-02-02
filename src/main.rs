use std::path::PathBuf;
use std::env;
use cli::parse_cli_args;
use csv_importer::import_csvs_to_sqlite;
use table_printer::{query_column_by_index, print_table};
use column_manager::get_column_names;
use utils::print_recursive;
use columnCategories_complete::lade_kategorie_map;
use cli::TextBereich;
// Füge den Import für SpaltenNamen hinzu - je nachdem wo es definiert ist
use cli::parser::SpaltenNamen;  // Direkter Import, falls in cli mod
// ODER: use cli::parser::SpaltenNamen; // Falls in parser submodul
use retaAusgabe::{Tables, CliOutput, OutputSyntax};

mod retaAusgabe;
mod cli;
mod data;
mod utils;
mod csv_importer;
mod column_manager;
mod data_fetcher;
mod table_printer;
mod ifIsZeilenAngabe;
mod columnCategories_complete;

// 1. Funktion: Tabellentest
pub fn test_simple_table() {
    let tables = Tables::new(Some(100));
    let mut output = CliOutput::new(&tables, OutputSyntax::Plain);
    output.cliout2("TEST: Dies sollte ausgegeben werden\n");

    println!("\n=== EINFACHER TABELLEN-TEST ===");

    let headers = vec![
        "Name".to_string(),
        "Alter".to_string(),
        "Stadt".to_string(),
    ];
    
    let data = vec![
        vec!["Hans".to_string(), "25".to_string(), "Berlin".to_string()],
        vec!["Anna".to_string(), "30".to_string(), "München".to_string()],
        vec!["Peter".to_string(), "22".to_string(), "Hamburg".to_string()],
    ];

    let mut max_lengths = vec![0, 0, 0];
    for (i, header) in headers.iter().enumerate() {
        max_lengths[i] = max_lengths[i].max(header.len());
    }
    for row in &data {
        for (i, cell) in row.iter().enumerate() {
            if i < max_lengths.len() {
                max_lengths[i] = max_lengths[i].max(cell.len());
            }
        }
    }

    let zeilen_bereiche: Vec<(usize, usize)> = Vec::new();
    print_table(&headers, data, &max_lengths, &zeilen_bereiche);
}

// 2. Funktion: Anzeige der Nutzungshinweise
pub fn show_usage() {
    println!("Benutzung: mein-rpnn --spalten OBERKATEGORIE UNTERKATEGORIE");
    println!("Beispiel:  mein-rpnn --spalten Menschliches Motive");
    println!("Beispiel:  mein-rpnn --spalten Universum Transzendentalien");
    println!("\nAlternative mit manuellen Bereichen:");
    println!("mein-rpnn --vorhervonausschnitt 7-12 --spaltenname Menschliches Motive");
    println!("mein-rpnn --vorhervonausschnitt 7,9 --spaltenname Menschliches Motive");
}

// 3. Funktion: Verarbeitung der Spaltennamen und Kategorien
pub fn verarbeite_spaltennamen(
    args: &[String],
    kategorie_map: &columnCategories_complete::KategorieMap
) -> Result<(TextBereich, SpaltenNamen), Box<dyn std::error::Error>> {
    println!("🔍 CLI Argumente: {:?}", args);
    
    let (_dashes, _params, mut bereich, spalten_namen) = parse_cli_args(args, Some(kategorie_map));
    println!("📊 Bereich nach Parser: {:?}", bereich);
    println!("📝 Spaltennamen: {:?}", spalten_namen);

    let hat_manuelle_spalten = !bereich.spalten_bereiche.is_empty();
    
    if !hat_manuelle_spalten && 
       (spalten_namen.oberkategorie != "oberkategorie" ||
        spalten_namen.unterkategorie != "unterkategorie") {
        
        println!("\n🔍 Automatische Spaltensuche für: '{}' → '{}'", 
                spalten_namen.oberkategorie, spalten_namen.unterkategorie);
        
        let gefundene_spalten = kategorie_map.finde_spaltennummern_fuer_kategorien(
            &spalten_namen.oberkategorie,
            &spalten_namen.unterkategorie
        );

        if !gefundene_spalten.is_empty() {
            println!("✅ Gefundene Spaltennummern: {:?}", gefundene_spalten);
            
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
            }
        } else {
            println!("❌ Keine Spaltennummern gefunden für: '{}' → '{}'", 
                    spalten_namen.oberkategorie, spalten_namen.unterkategorie);
            println!("ℹ️  Versuche es mit diesen Kombinationen:");
            println!("  --spaltenname 'Menschliches' 'Motive'");
            println!("  --spaltenname 'Universum' 'Transzendentalien'");
            println!("  --spaltenname 'Religionen' 'Superkräfte'");
            
            println!("⚠️  Verwende Standard-Spalte 1 als Fallback");
            bereich.von_spalte = 1;
            bereich.bis_spalte = 1;
        }
    }

    Ok((bereich, spalten_namen))
}

// 4. Funktion: Verarbeitung der Kategorie-Map
pub fn verarbeite_kategorien(
    kategorie_map: &columnCategories_complete::KategorieMap,
    bereich: &TextBereich,
    spalten_namen: &SpaltenNamen
) -> Result<(), Box<dyn std::error::Error>> {
    if !bereich.spalten_bereiche.is_empty() {
        let mut spalten_nummern = Vec::new();
        for (von, bis) in &bereich.spalten_bereiche {
            for nummer in *von..=*bis {
                spalten_nummern.push(nummer);
            }
        }
        
        let gefilterte = kategorie_map.filtere_nach_spaltennummern(&spalten_nummern);
        println!("📈 Gefundene Kategorie-Einträge für Spalten {:?}: {}", 
                bereich.spalten_bereiche, gefilterte.len());
        
        if !gefilterte.is_empty() {
            let sql = kategorie_map.generiere_sql_selects(
                &spalten_namen.oberkategorie,
                &spalten_namen.unterkategorie,
                Some(&spalten_nummern)
            );
            // println!("\n{}", sql); // Optional: SQL ausgeben
        }
    } else {
        println!("⚠️  Keine Spaltennummern verfügbar - überspringe Kategorie-Verarbeitung");
    }
    
    // Zeige verfügbare Kategorien (optional)
    let mut oberkategorien = std::collections::HashSet::new();
    for eintrag in &kategorie_map.alle_eintraege {
        oberkategorien.insert(eintrag.oberkategorie.clone());
    }
    
    println!("ℹ️  Verfügbare Oberkategorien (erste 10):");
    for (i, kategorie) in oberkategorien.iter().take(10).enumerate() {
        println!("  {}. {}", i + 1, kategorie);
    }
    
    Ok(())
}

// 5. Funktion: Haupt-Workflow
pub fn main_workflow() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== START TABELLEN-TEST ===");
    test_simple_table();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        show_usage();
        return Ok(());
    }

    // 1. Lade Kategorie-Daten
    println!("\n📂 Lade Kategorie-Daten...");
    let kategorie_map = lade_kategorie_map();

    // 2. Verarbeite CLI-Argumente und Spaltennamen
    let (bereich, spalten_namen) = verarbeite_spaltennamen(&args, &kategorie_map)?;

    // 3. Verarbeite Kategorien
    verarbeite_kategorien(&kategorie_map, &bereich, &spalten_namen)?;

    // 4. Datenstruktur erstellen und ausgeben
    let meine_liste = data::create_example_structure();
    println!("Struktur wurde erstellt. Hier ist die rekursive Ausgabe:");
    print_recursive(&meine_liste, 0);

    // 5. CSV-Daten importieren
    let projPath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pfad1 = projPath.to_string_lossy().into_owned() + "/csv/religion.csv";
    let pfad2 = projPath.to_string_lossy().into_owned() + "/csv/merged_filtered.csv";
    let dateien = [pfad1, pfad2];
    
    let conn = import_csvs_to_sqlite(&dateien)?;
    
    // 6. Spalten abfragen
    let wurde_spalten_gesucht2 = bereich.spalten_gesucht2;
    query_column_by_index(&conn, bereich, wurde_spalten_gesucht2)?;
    
    // 7. Spaltennamen abrufen
    let column_names = get_column_names(&conn)?;
    println!("Die Tabelle hat {} Spalten.", column_names.len());
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    main_workflow()
}
