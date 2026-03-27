use crate::domain::parser::cli_alias_parser::parse_spalten_anfrage;
use crate::cli::parser::{SpaltenAuswahlModus, SpaltenNamen, SpaltenNamenListe};
use crate::cli::{parse_cli_args, TextBereich};
use crate::domain::categories::KategorieMap;
use crate::processing::spalten_support::defaults::fallback_zu_standards;
use crate::processing::spalten_support::exact_merge::merge_exact;
use crate::processing::spalten_support::normalize::is_primzahlkreuz_pro_contra_request;
use crate::processing::spalten_support::selection_sync::{
    finalize_found_columns,
    setze_gefundene_spalten,
};
use std::error::Error;

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
        let (_dashes, _params, mut bereich, spalten_namen, spalten_namen_liste, auswahl_modus) =
            parse_cli_args(self.args, Some(self.kategorie_map));

        match auswahl_modus {
            SpaltenAuswahlModus::Alle => {
                self.setze_alle_spalten(&mut bereich);
                bereich.mark_columns_resolved();
            }
            SpaltenAuswahlModus::Explizit => {
                self.verarbeite_automatische_spalten(
                    &mut bereich,
                    &spalten_namen,
                    &spalten_namen_liste,
                )?;
            }
        }

        Ok(VerarbeitungsErgebnis { bereich, spalten_namen })
    }

    fn setze_alle_spalten(&self, bereich: &mut TextBereich) {
        let mut alle_gefundene_spalten: Vec<u32> = self.kategorie_map.alle_spaltennummern();

        if alle_gefundene_spalten.is_empty() {
            fallback_zu_standards(bereich);
            return;
        }

        alle_gefundene_spalten.sort_unstable();
        alle_gefundene_spalten.dedup();
        setze_gefundene_spalten(bereich, alle_gefundene_spalten);
        finalize_found_columns(bereich);
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

        if merge_exact(bereich, &spalten_namen.oberkategorie, &spalten_namen.unterkategorie) {
            finalize_found_columns(bereich);
            return Ok(());
        }

        if is_primzahlkreuz_pro_contra_request(
            &spalten_namen.oberkategorie,
            &spalten_namen.unterkategorie,
        ) {
            bereich.mark_columns_resolved();
            return Ok(());
        }

        if let Some(request) = &spalten_namen.typed_request {
            let direkte_spalten = self.kategorie_map.finde_spaltennummern_fuer_canonical_request(request);

            if !direkte_spalten.is_empty() {
                setze_gefundene_spalten(bereich, direkte_spalten);
                bereich.mark_columns_resolved();
                return Ok(());
            }

            if let Some(inference) = self.kategorie_map.infer_generated_canonical_request(request) {
                if !inference.required_columns.is_empty() {
                    setze_gefundene_spalten(bereich, inference.required_columns.clone());
                }
                bereich.mark_columns_resolved();
                return Ok(());
            }
        } else {
            let direkte_spalten = self.kategorie_map.finde_spaltennummern_fuer_kategorien(
                &spalten_namen.oberkategorie,
                &spalten_namen.unterkategorie,
            );

            if !direkte_spalten.is_empty() {
                setze_gefundene_spalten(bereich, direkte_spalten);
                bereich.mark_columns_resolved();
                return Ok(());
            }

            if let Some(inference) = self.kategorie_map.infer_generated_pair(
                &spalten_namen.oberkategorie,
                &spalten_namen.unterkategorie,
            ) {
                if !inference.required_columns.is_empty() {
                    setze_gefundene_spalten(bereich, inference.required_columns.clone());
                }
                bereich.mark_columns_resolved();
                return Ok(());
            }
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
            if merge_exact(bereich, &spalten_namen.oberkategorie, &spalten_namen.unterkategorie) {
                exact_hit = true;
                continue;
            }
            let gefundene_spalten = if let Some(request) = &spalten_namen.typed_request {
                self.kategorie_map.finde_spaltennummern_fuer_canonical_request(request)
            } else {
                self.kategorie_map.finde_spaltennummern_fuer_kategorien(
                    &spalten_namen.oberkategorie,
                    &spalten_namen.unterkategorie,
                )
            };
            alle_gefundene_spalten.extend(gefundene_spalten);
        }

        if exact_hit {
            for col in alle_gefundene_spalten {
                let c = col as usize;
                bereich.spalten_bereiche.push((c, c));
                if !bereich.exact_visible_columns.contains(&c) {
                    bereich.exact_visible_columns.push(c);
                }
                if !bereich.spaltenreihenfolgeundnurdiese.contains(&c) {
                    bereich.spaltenreihenfolgeundnurdiese.push(c);
                }
            }
            bereich.exact_visible_columns.sort_unstable();
            bereich.exact_visible_columns.dedup();
            bereich.spaltenreihenfolgeundnurdiese.sort_unstable();
            bereich.spaltenreihenfolgeundnurdiese.dedup();
            finalize_found_columns(bereich);
            bereich.mark_columns_resolved();
            return Ok(());
        }

        if !alle_gefundene_spalten.is_empty() {
            alle_gefundene_spalten.sort_unstable();
            alle_gefundene_spalten.dedup();
            setze_gefundene_spalten(bereich, alle_gefundene_spalten);
            return Ok(());
        }

        if let Some(letzte_spalten_namen) = spalten_namen_liste.eintraege.last() {
           let typed_inference = letzte_spalten_namen
    .typed_request
    .as_ref()
    .and_then(|request| self.kategorie_map.infer_generated_canonical_request(request))
    .or_else(|| {
        parse_spalten_anfrage(
            &letzte_spalten_namen.oberkategorie,
            &letzte_spalten_namen.unterkategorie,
        )
        .ok()
        .and_then(|request| self.kategorie_map.infer_generated_canonical_request(&request))
    });
           if let Some(inference) = typed_inference.or_else(|| self.kategorie_map.infer_generated_pair(
                &letzte_spalten_namen.oberkategorie,
                &letzte_spalten_namen.unterkategorie,
            )) {
                if !inference.required_columns.is_empty() {
                    setze_gefundene_spalten(bereich, inference.required_columns.clone());
                }
                bereich.mark_columns_resolved();
            } else {
                fallback_zu_standards(bereich);
            }
        } else {
            return Err("SpaltenNamenListe ist leer".into());
        }

        Ok(())
    }
}
