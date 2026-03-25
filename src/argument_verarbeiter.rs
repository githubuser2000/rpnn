
use crate::cli::parser::{SpaltenNamen, SpaltenNamenListe};
use crate::cli::{parse_cli_args, TextBereich};
use crate::column_categories_complete::KategorieMap;
use crate::exact_generator_bridge::resolve_exact_generator;
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

    matches!(ober.as_str(), "bedeutung" | "procontra" | "universum" )
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

    fn merge_exact(&self, bereich: &mut TextBereich, ober: &str, unter: &str) -> bool {
        let Some(exact) = resolve_exact_generator(ober, unter) else {
            return false;
        };

        let exact_is_meta = exact.generated_befehle.contains("universummetakonkret");
        for cmd in exact.generated_befehle {
            bereich.exact_generated_befehle.insert(cmd);
        }
        for pair in exact.modal_pairs {
            if !bereich.exact_modal_pairs.contains(&pair) {
                bereich.exact_modal_pairs.push(pair);
            }
        }
        for spec in exact.meta_konkret_specs {
            if !bereich.exact_meta_konkret_specs.contains(&spec) {
                bereich.exact_meta_konkret_specs.push(spec);
            }
        }
        for col in exact.direct_columns {
            if !exact_is_meta && !bereich.exact_visible_columns.contains(&col) {
                bereich.exact_visible_columns.push(col);
            }
            bereich.spalten_bereiche.push((col, col));
        }

        bereich.exact_visible_columns.sort_unstable();
        bereich.exact_visible_columns.dedup();
        bereich.spalten_bereiche.sort_unstable();
        bereich.spalten_bereiche.dedup();
        bereich.spalten_gefunden = true;
        bereich.spalten_gesucht = true;
        bereich.spalten_gesucht2 = false;
        true
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

        if self.merge_exact(bereich, &spalten_namen.oberkategorie, &spalten_namen.unterkategorie) {
            self.finalize_found_columns(bereich);
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
        let mut exact_hit = false;

        for spalten_namen in &spalten_namen_liste.eintraege {
            if self.merge_exact(bereich, &spalten_namen.oberkategorie, &spalten_namen.unterkategorie) {
                exact_hit = true;
                continue;
            }
            let gefundene_spalten = self.kategorie_map.finde_spaltennummern_fuer_kategorien(
                &spalten_namen.oberkategorie,
                &spalten_namen.unterkategorie,
            );
            alle_gefundene_spalten.extend(gefundene_spalten);
        }

        if exact_hit {
            for col in alle_gefundene_spalten {
                bereich.spalten_bereiche.push((col as usize, col as usize));
            }
            self.finalize_found_columns(bereich);
            return Ok(());
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

    fn finalize_found_columns(&self, bereich: &mut TextBereich) {
        bereich.spalten_bereiche.sort_unstable();
        bereich.spalten_bereiche.dedup();
        if !bereich.spalten_bereiche.is_empty() {
            bereich.von_spalte = bereich.spalten_bereiche[0].0;
            bereich.bis_spalte = bereich.spalten_bereiche.last().unwrap().1;
        }
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
    bereich.spalten_bereiche.clear();
    bereich.spaltenreihenfolgeundnurdiese.clear();
    bereich.exact_visible_columns.clear();

    bereich.von_spalte = usize::MAX;
    bereich.bis_spalte = usize::MAX;

    bereich.spalten_gefunden = false;
    bereich.spalten_gesucht = false;
    bereich.spalten_gesucht2 = false;

    Ok(())
}
}
