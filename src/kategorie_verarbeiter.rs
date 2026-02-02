use crate::cli::{TextBereich, parser::SpaltenNamen};
use crate::columnCategories_complete::KategorieMap;

// 4. Funktion: Verarbeitung der Kategorie-Map
pub fn verarbeite_kategorien(
    kategorie_map: &KategorieMap,
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
