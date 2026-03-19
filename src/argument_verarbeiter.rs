use crate::cli::{TextBereich, parse_cli_args};
use crate::cli::parser::{SpaltenNamen, SpaltenNamenListe};
use crate::column_categories_complete::KategorieMap;
use crate::table_printer::query::try_resolve_generated_pair;
use std::collections::BTreeSet;
use std::error::Error;

fn normalize_category_key(s: &str) -> String {
    s.to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
}

fn is_primzahlkreuz_pro_contra_request(ober: &str, unter: &str) -> bool {
    let ober = normalize_category_key(ober);
    let unter = normalize_category_key(unter);

    matches!(ober.as_str(), "bedeutung" | "procontra" | "universum")
        && matches!(unter.as_str(), "primzahlkreuzprocontra" | "primzahlkreuz")
}

pub struct SpaltenVerarbeiter<'a> {
    args: &'a [String],
    kategorie_map: &'a KategorieMap,
}

pub struct VerarbeitungsErgebnis {
    pub bereich: TextBereich,
    pub spalten_namen: SpaltenNamen,
}

impl<'a> SpaltenVerarbeiter<'a> {
    pub fn new(args: &'a [String], kategorie_map: &'a KategorieMap) -> Self {
        Self { args, kategorie_map }
    }

    pub fn verarbeite_zu_tupel(&self) -> Result<(TextBereich, SpaltenNamen), Box<dyn Error>> {
        let ergebnis = self.verarbeite()?;
        Ok((ergebnis.bereich, ergebnis.spalten_namen))
    }


    pub fn verarbeite(&self) -> Result<VerarbeitungsErgebnis, Box<dyn Error>> {
        println!("🔍 CLI Argumente: {:?}", self.args);

        let (_dashes, _params, mut bereich, spalten_namen, spalten_namen_liste) =
            parse_cli_args(self.args, Some(self.kategorie_map));

        println!("📊 Bereich nach Parser: {:?}", bereich);
        println!("📝 Spaltennamen: {:?}", spalten_namen);

        self.verarbeite_automatische_spalten(
            &mut bereich,
            &spalten_namen,
            &spalten_namen_liste,
        )?;

        Ok(VerarbeitungsErgebnis {
            bereich,
            spalten_namen,
        })
    }

fn verarbeite_automatische_spalten(
    &self,
    bereich: &mut TextBereich,
    spalten_namen: &SpaltenNamen,
    spalten_namen_liste: &SpaltenNamenListe
) -> Result<(), Box<dyn Error>> {
    let hat_manuelle_spalten = !bereich.spalten_bereiche.is_empty();

    if !hat_manuelle_spalten &&
       (spalten_namen.oberkategorie != "oberkategorie" ||
        spalten_namen.unterkategorie != "unterkategorie") {

        println!(
            "\n🔍 Automatische Spaltensuche für: '{}' → '{}'",
            spalten_namen.oberkategorie, spalten_namen.unterkategorie
        );

        if is_primzahlkreuz_pro_contra_request(
            &spalten_namen.oberkategorie,
            &spalten_namen.unterkategorie,
        ) {
            println!("ℹ️ Spezialfall Primzahlkreuz erkannt: keine normale Spaltensuche");

            bereich.spalten_gefunden = true;
            bereich.spalten_gesucht = true;
            bereich.spalten_gesucht2 = false;

            return Ok(());
        }

        // 🔥 ERST: normale direkte Spalten suchen
let mut alle_gefundene_spalten: Vec<u32> = Vec::new();

for spalten_namen in &spalten_namen_liste.eintraege {
    let gefundene_spalten: Vec<u32> =
        self.kategorie_map.finde_spaltennummern_fuer_kategorien(
            &spalten_namen.oberkategorie,
            &spalten_namen.unterkategorie,
        );

    alle_gefundene_spalten.extend(gefundene_spalten);
}

if !alle_gefundene_spalten.is_empty() {
    println!("✅ Direkte Spalten gefunden → KEIN Generator nötig");

    alle_gefundene_spalten.sort_unstable();
    alle_gefundene_spalten.dedup();

    self.setze_gefundene_spalten(bereich, alle_gefundene_spalten)?;
    bereich.spalten_gefunden = true;
    bereich.spalten_gesucht = true;
    bereich.spalten_gesucht2 = false;

    return Ok(());
}
}
// 🔥 ERST JETZT: Generator prüfen
let mut generated_befehle = BTreeSet::new();
let mut required_columns = BTreeSet::new();

if try_resolve_generated_pair(
    &spalten_namen.oberkategorie,
    &spalten_namen.unterkategorie,
    &mut generated_befehle,
    &mut required_columns,
) {
    println!(
        "ℹ️ Generierte Wortpaar-Kombination erkannt: {:?} → Basisspalten {:?}",
        generated_befehle, required_columns
    );

    let required: Vec<u32> = required_columns.into_iter().map(|n| n as u32).collect();
    self.setze_gefundene_spalten(bereich, required)?;
    bereich.spalten_gefunden = true;
    bereich.spalten_gesucht = true;
    bereich.spalten_gesucht2 = false;

    return Ok(());
}

// 🔥 FALLBACK wie vorher
self.suche_und_setze_spalten(bereich, spalten_namen_liste)?;
            Ok(())
}

    fn suche_und_setze_spalten(
        &self,
        bereich: &mut TextBereich,
        spalten_namen_liste: &SpaltenNamenListe,
    ) -> Result<(), Box<dyn Error>> {
        let mut alle_gefundene_spalten: Vec<u32> = Vec::new();

        for spalten_namen in &spalten_namen_liste.eintraege {
            let gefundene_spalten: Vec<u32> =
                self.kategorie_map.finde_spaltennummern_fuer_kategorien(
                    &spalten_namen.oberkategorie,
                    &spalten_namen.unterkategorie,
                );

            alle_gefundene_spalten.extend(gefundene_spalten);
        }

        if !alle_gefundene_spalten.is_empty() {
            alle_gefundene_spalten.sort_unstable();
            alle_gefundene_spalten.dedup();

            self.setze_gefundene_spalten(bereich, alle_gefundene_spalten)?;
        } else {
            if let Some(letzte_spalten_namen) = spalten_namen_liste.eintraege.last() {
                let mut generated_befehle = BTreeSet::new();
                let mut required_columns = BTreeSet::new();

                if try_resolve_generated_pair(
                    &letzte_spalten_namen.oberkategorie,
                    &letzte_spalten_namen.unterkategorie,
                    &mut generated_befehle,
                    &mut required_columns,
                ) {
                    println!(
                        "ℹ️ Fallback auf generierte Spaltenauflösung: {:?} → Basisspalten {:?}",
                        generated_befehle, required_columns
                    );

                    let required: Vec<u32> = required_columns.into_iter().map(|n| n as u32).collect();
                    self.setze_gefundene_spalten(bereich, required)?;
                    bereich.spalten_gefunden = true;
                    bereich.spalten_gesucht = true;
                    bereich.spalten_gesucht2 = false;
                } else {
                    self.fallback_zu_standards(bereich, letzte_spalten_namen)?;
                }
            } else {
                return Err("SpaltenNamenListe ist leer".into());
            }
        }

        Ok(())
    }

    fn setze_gefundene_spalten(
        &self,
        bereich: &mut TextBereich,
        gefundene_spalten: Vec<u32>,
    ) -> Result<(), Box<dyn Error>> {
        println!("✅ Gefundene Spaltennummern: {:?}", gefundene_spalten);

        let mut sorted: Vec<usize> = gefundene_spalten.iter().map(|&n| n as usize).collect();
        sorted.sort();

        let mut bereich_fuer_spalten = TextBereich::default();
        for &num in &sorted {
            bereich_fuer_spalten.spalten_bereiche.push((num, num));
        }

        bereich.spalten_bereiche = bereich_fuer_spalten.spalten_bereiche;

        if !bereich.spalten_bereiche.is_empty() {
            bereich.von_spalte = bereich.spalten_bereiche[0].0;
            bereich.bis_spalte = bereich.spalten_bereiche.last().unwrap().1;
            println!(
                "📊 Automatisch erzeugte Spaltenbereiche: {:?}",
                bereich.spalten_bereiche
            );
        }

        Ok(())
    }

    fn fallback_zu_standards(
        &self,
        bereich: &mut TextBereich,
        spalten_namen: &SpaltenNamen,
    ) -> Result<(), Box<dyn Error>> {
        println!(
            "❌ Keine Spaltennummern gefunden für: '{}' → '{}'",
            spalten_namen.oberkategorie, spalten_namen.unterkategorie
        );

        self.zeige_alternative_kombinationen();

        println!("⚠️  Verwende Standard-Spalte 1 als Fallback");
        bereich.von_spalte = 1;
        bereich.bis_spalte = 1;

        Ok(())
    }

    fn zeige_alternative_kombinationen(&self) {
        println!("ℹ️  Versuche es mit diesen Kombinationen:");
        println!("  --spaltenname 'Menschliches' 'Motive'");
        println!("   --spaltenname 'Universum' 'Transzendentalien --spaltenname 'Menschliches' 'Liebe'");
        println!("  --spaltenname 'Religionen' 'Superkräfte'");
    }
}
