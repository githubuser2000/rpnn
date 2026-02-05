use crate::cli::{TextBereich, parse_cli_args};
use crate::cli::parser::SpaltenNamen;
use crate::column_categories_complete::KategorieMap;
use std::error::Error;

// 3. Funktion: Verarbeitung der Spaltennamen und Kategorien

// Struct für die Spaltenverarbeitung
pub struct SpaltenVerarbeiter<'a> {
    args: &'a [String],
    kategorie_map: &'a KategorieMap,
}

// Struct für das Ergebnis
pub struct VerarbeitungsErgebnis {
    pub bereich: TextBereich,
    pub spalten_namen: SpaltenNamen,
}

impl<'a> SpaltenVerarbeiter<'a> {
    // Konstruktor
    pub fn new(args: &'a [String], kategorie_map: &'a KategorieMap) -> Self {
        Self { args, kategorie_map }
    }
    // Konvenienz-Methode für direkten Tupel-Rückgabe
    pub fn verarbeite_zu_tupel(&self) -> Result<(TextBereich, SpaltenNamen), Box<dyn Error>> {
        let ergebnis = self.verarbeite()?;
        Ok((ergebnis.bereich, ergebnis.spalten_namen))
    } 
    // Hauptverarbeitungsmethode
    pub fn verarbeite(&self) -> Result<VerarbeitungsErgebnis, Box<dyn Error>> {
        println!("🔍 CLI Argumente: {:?}", self.args);
        
        // Parse CLI-Argumente
        let (_dashes, _params, mut bereich, spalten_namen, spalten_namen_liste) = 
            parse_cli_args(self.args, Some(self.kategorie_map));
        
        println!("📊 Bereich nach Parser: {:?}", bereich);
        println!("📝 Spaltennamen: {:?}", spalten_namen);
        
        self.verarbeite_automatische_spalten(&mut bereich, &spalten_namen)?;
        
        Ok(VerarbeitungsErgebnis {
            bereich,
            spalten_namen,
        })
    }
    
    // Private Methode für automatische Spaltensuche
    fn verarbeite_automatische_spalten(
        &self,
        bereich: &mut TextBereich,
        spalten_namen: &SpaltenNamen
    ) -> Result<(), Box<dyn Error>> {
        let hat_manuelle_spalten = !bereich.spalten_bereiche.is_empty();
        
        if !hat_manuelle_spalten &&
           (spalten_namen.oberkategorie != "oberkategorie" ||
            spalten_namen.unterkategorie != "unterkategorie") {
            
            println!("\n🔍 Automatische Spaltensuche für: '{}' → '{}'",
                     spalten_namen.oberkategorie, spalten_namen.unterkategorie);
            
            self.suche_und_setze_spalten(bereich, spalten_namen)?;
        }
        
        Ok(())
    }
    
    // Methode für die Suche nach Spaltennummern
    fn suche_und_setze_spalten(
        &self,
        bereich: &mut TextBereich,
        spalten_namen: &SpaltenNamen
    ) -> Result<(), Box<dyn Error>> {
        let gefundene_spalten = self.kategorie_map.finde_spaltennummern_fuer_kategorien(
            &spalten_namen.oberkategorie,
            &spalten_namen.unterkategorie
        );
        
        if !gefundene_spalten.is_empty() {
            self.setze_gefundene_spalten(bereich, gefundene_spalten)?;
        } else {
            self.fallback_zu_standards(bereich, spalten_namen)?;
        }
        
        Ok(())
    }
    
    // Methode zum Setzen gefundener Spalten
    fn setze_gefundene_spalten(
        &self,
        bereich: &mut TextBereich,
        gefundene_spalten: Vec<u32>
    ) -> Result<(), Box<dyn Error>> {
        println!("✅ Gefundene Spaltennummern: {:?}", gefundene_spalten);
        
        let mut sorted: Vec<usize> = gefundene_spalten.iter()
            .map(|&n| n as usize)
            .collect();
        sorted.sort();
        
        // Erstelle temporären Bereich für die gefundenen Spalten
        let mut bereich_fuer_spalten = TextBereich::default();
        for &num in &sorted {
            bereich_fuer_spalten.spalten_bereiche.push((num, num));
        }
        
        bereich.spalten_bereiche = bereich_fuer_spalten.spalten_bereiche;
        
        if !bereich.spalten_bereiche.is_empty() {
            bereich.von_spalte = bereich.spalten_bereiche[0].0;
            bereich.bis_spalte = bereich.spalten_bereiche.last().unwrap().1;
            println!("📊 Automatisch erzeugte Spaltenbereiche: {:?}", 
                     bereich.spalten_bereiche);
        }
        
        Ok(())
    }
    
    // Fallback-Methode wenn keine Spalten gefunden wurden
    fn fallback_zu_standards(
        &self,
        bereich: &mut TextBereich,
        spalten_namen: &SpaltenNamen
    ) -> Result<(), Box<dyn Error>> {
        println!("❌ Keine Spaltennummern gefunden für: '{}' → '{}'",
                 spalten_namen.oberkategorie, spalten_namen.unterkategorie);
        
        self.zeige_alternative_kombinationen();
        
        println!("⚠️  Verwende Standard-Spalte 1 als Fallback");
        bereich.von_spalte = 1;
        bereich.bis_spalte = 1;
        
        Ok(())
    }
    
    // Methode zum Anzeigen alternativer Kombinationen
    fn zeige_alternative_kombinationen(&self) {
        println!("ℹ️  Versuche es mit diesen Kombinationen:");
        println!("  --spaltenname 'Menschliches' 'Motive'");
        println!("  --spaltenname 'Universum' 'Transzendentalien'");
        println!("  --spaltenname 'Religionen' 'Superkräfte'");
    }
}

// Beispiel für die Verwendung:
// let verarbeiter = SpaltenVerarbeiter::new(&args, &kategorie_map);
// let ergebnis = verarbeiter.verarbeite()?;
// let bereich = ergebnis.bereich;
// let spalten_namen = ergebnis.spalten_namen;
