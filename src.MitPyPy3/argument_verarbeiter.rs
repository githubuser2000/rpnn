use crate::cli::parser::{SpaltenNamen, SpaltenNamenListe};
use crate::cli::{parse_cli_args, TextBereich};
use crate::column_categories_complete::KategorieMap;
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
        let (_dashes, _params, mut bereich, spalten_namen, spalten_namen_liste) =
            parse_cli_args(self.args, Some(self.kategorie_map));

        self.verarbeite_automatische_spalten(&mut bereich, &spalten_namen, &spalten_namen_liste)?;

        Ok(VerarbeitungsErgebnis {
            bereich,
            spalten_namen,
        })
    }

    fn verarbeite_automatische_spalten(
        &self,
        bereich: &mut TextBereich,
        spalten_namen: &SpaltenNamen,
        spalten_namen_liste: &SpaltenNamenListe,
    ) -> Result<(), Box<dyn Error>> {
        let hat_manuelle_spalten = !bereich.spalten_bereiche.is_empty();

        if hat_manuelle_spalten {
            return Ok(());
        }

        // Bei mehreren --spaltenname-Einträgen darf NICHT nur der letzte Eintrag
        // (spalten_namen) ausgewertet werden. Sonst gewinnt der letzte Filter und
        // frühere Kombinationen wie "Menschliches Motive" verschwinden.
        if spalten_namen_liste.eintraege.len() > 1 {
            self.suche_und_setze_spalten(bereich, spalten_namen_liste)?;
            return Ok(());
        }

        if spalten_namen.oberkategorie.is_empty() && spalten_namen.unterkategorie.is_empty() {
            return Ok(());
        }

        if spalten_namen.oberkategorie == "oberkategorie"
            && spalten_namen.unterkategorie == "unterkategorie"
        {
            return Ok(());
        }

        if is_primzahlkreuz_pro_contra_request(
            &spalten_namen.oberkategorie,
            &spalten_namen.unterkategorie,
        ) {
            bereich.spalten_gefunden = true;
            bereich.spalten_gesucht = true;
            bereich.spalten_gesucht2 = false;
            return Ok(());
        }

        let direkte_spalten = self.kategorie_map.finde_spaltennummern_fuer_kategorien(
            &spalten_namen.oberkategorie,
            &spalten_namen.unterkategorie,
        );

        if !direkte_spalten.is_empty() {
            self.setze_gefundene_spalten(bereich, direkte_spalten)?;
            bereich.spalten_gefunden = true;
            bereich.spalten_gesucht = true;
            bereich.spalten_gesucht2 = false;
            return Ok(());
        }

        if let Some(inference) = self.kategorie_map.infer_generated_pair(
            &spalten_namen.oberkategorie,
            &spalten_namen.unterkategorie,
        ) {
            if !inference.required_columns.is_empty() {
                self.setze_gefundene_spalten(bereich, inference.required_columns.clone())?;
            }
            bereich.spalten_gefunden = true;
            bereich.spalten_gesucht = true;
            bereich.spalten_gesucht2 = false;
            return Ok(());
        }

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
            let gefundene_spalten = self.kategorie_map.finde_spaltennummern_fuer_kategorien(
                &spalten_namen.oberkategorie,
                &spalten_namen.unterkategorie,
            );
            alle_gefundene_spalten.extend(gefundene_spalten);
        }

        if !alle_gefundene_spalten.is_empty() {
            alle_gefundene_spalten.sort_unstable();
            alle_gefundene_spalten.dedup();
            self.setze_gefundene_spalten(bereich, alle_gefundene_spalten)?;
            return Ok(());
        }

        if let Some(letzte_spalten_namen) = spalten_namen_liste.eintraege.last() {
            if let Some(inference) = self.kategorie_map.infer_generated_pair(
                &letzte_spalten_namen.oberkategorie,
                &letzte_spalten_namen.unterkategorie,
            ) {
                if !inference.required_columns.is_empty() {
                    self.setze_gefundene_spalten(bereich, inference.required_columns.clone())?;
                }
                bereich.spalten_gefunden = true;
                bereich.spalten_gesucht = true;
                bereich.spalten_gesucht2 = false;
            } else {
                self.fallback_zu_standards(bereich, letzte_spalten_namen)?;
            }
        } else {
            return Err("SpaltenNamenListe ist leer".into());
        }

        Ok(())
    }

    fn setze_gefundene_spalten(
        &self,
        bereich: &mut TextBereich,
        gefundene_spalten: Vec<u32>,
    ) -> Result<(), Box<dyn Error>> {
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
        }

        Ok(())
    }

    fn fallback_zu_standards(
        &self,
        bereich: &mut TextBereich,
        _spalten_namen: &SpaltenNamen,
    ) -> Result<(), Box<dyn Error>> {
        bereich.von_spalte = 1;
        bereich.bis_spalte = 1;
        Ok(())
    }

    fn zeige_alternative_kombinationen(&self) {
        println!("ℹ️  Versuche es mit diesen Kombinationen:");
        println!("  --spaltenname 'Menschliches' 'Motive'");
        println!(
            "   --spaltenname 'Universum' 'Transzendentalien --spaltenname 'Menschliches' 'Liebe'"
        );
        println!("  --spaltenname 'Religionen' 'Superkräfte'");
    }
}

