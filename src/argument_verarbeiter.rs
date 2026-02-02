use crate::cli::{TextBereich, parse_cli_args};
use crate::cli::parser::SpaltenNamen;
use crate::columnCategories_complete::KategorieMap;

// 3. Funktion: Verarbeitung der Spaltennamen und Kategorien
pub fn verarbeite_spaltennamen(
    args: &[String],
    kategorie_map: &KategorieMap
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
