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

    fn markiere_spaltenstatus(bereich: &mut TextBereich, gefunden: bool) {
        bereich.spalten_gefunden = gefunden;
        bereich.spalten_gesucht = gefunden;
        bereich.spalten_gesucht2 = false;
    }

    fn aktualisiere_spaltengrenzen(bereich: &mut TextBereich) {
        bereich.spalten_bereiche.sort_unstable();
        bereich.spalten_bereiche.dedup();

        if let Some((erste, _)) = bereich.spalten_bereiche.first().copied() {
            bereich.von_spalte = erste;
            bereich.bis_spalte = bereich.spalten_bereiche.last().map(|(_, bis)| *bis).unwrap_or(erste);
        } else {
            bereich.von_spalte = usize::MAX;
            bereich.bis_spalte = usize::MAX;
        }
    }

    fn fuege_spalten_hinzu(&self, bereich: &mut TextBereich, spalten: impl IntoIterator<Item = usize>) {
        for spalte in spalten {
            bereich.spalten_bereiche.push((spalte, spalte));

            if !bereich.exact_visible_columns.contains(&spalte) {
                bereich.exact_visible_columns.push(spalte);
            }
            if !bereich.spaltenreihenfolgeundnurdiese.contains(&spalte) {
                bereich.spaltenreihenfolgeundnurdiese.push(spalte);
            }
        }

        bereich.exact_visible_columns.sort_unstable();
        bereich.exact_visible_columns.dedup();
        bereich.spaltenreihenfolgeundnurdiese.sort_unstable();
        bereich.spaltenreihenfolgeundnurdiese.dedup();
        Self::aktualisiere_spaltengrenzen(bereich);
    }

    fn wende_inferenz_an(
        &self,
        bereich: &mut TextBereich,
        ober: &str,
        unter: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let Some(inference) = self.kategorie_map.infer_generated_pair(ober, unter) else {
            return Ok(false);
        };

        if !inference.required_columns.is_empty() {
            self.setze_gefundene_spalten(bereich, inference.required_columns.clone())?;
        }
        Self::markiere_spaltenstatus(bereich, true);
        Ok(true)
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

        let direkte_spalten = exact.direct_columns;

        if exact_is_meta {
            for col in direkte_spalten {
                bereich.spalten_bereiche.push((col, col));
            }
            Self::aktualisiere_spaltengrenzen(bereich);
        } else {
            self.fuege_spalten_hinzu(bereich, direkte_spalten);
        }

        Self::markiere_spaltenstatus(bereich, true);
        true
    }

    fn verarbeite_automatische_spalten(
        &self,
        bereich: &mut TextBereich,
        spalten_namen: &SpaltenNamen,
        spalten_namen_liste: &SpaltenNamenListe,
    ) -> Result<(), Box<dyn Error>> {
        if !bereich.spalten_bereiche.is_empty() {
            return Ok(());
        }

        if spalten_namen_liste.eintraege.len() > 1 {
            return self.suche_und_setze_spalten(bereich, spalten_namen_liste);
        }

        if (spalten_namen.oberkategorie.is_empty() && spalten_namen.unterkategorie.is_empty())
            || (spalten_namen.oberkategorie == "oberkategorie"
                && spalten_namen.unterkategorie == "unterkategorie")
        {
            return Ok(());
        }

        if self.merge_exact(bereich, &spalten_namen.oberkategorie, &spalten_namen.unterkategorie) {
            return Ok(());
        }

        if is_primzahlkreuz_pro_contra_request(
            &spalten_namen.oberkategorie,
            &spalten_namen.unterkategorie,
        ) {
            Self::markiere_spaltenstatus(bereich, true);
            return Ok(());
        }

        let direkte_spalten = self.kategorie_map.finde_spaltennummern_fuer_kategorien(
            &spalten_namen.oberkategorie,
            &spalten_namen.unterkategorie,
        );

        if !direkte_spalten.is_empty() {
            self.setze_gefundene_spalten(bereich, direkte_spalten)?;
            Self::markiere_spaltenstatus(bereich, true);
            return Ok(());
        }

        if self.wende_inferenz_an(
            bereich,
            &spalten_namen.oberkategorie,
            &spalten_namen.unterkategorie,
        )? {
            return Ok(());
        }

        self.suche_und_setze_spalten(bereich, spalten_namen_liste)
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
            self.fuege_spalten_hinzu(
                bereich,
                alle_gefundene_spalten.into_iter().map(|col| col as usize),
            );
            Self::markiere_spaltenstatus(bereich, true);
            return Ok(());
        }

        if !alle_gefundene_spalten.is_empty() {
            alle_gefundene_spalten.sort_unstable();
            alle_gefundene_spalten.dedup();
            self.setze_gefundene_spalten(bereich, alle_gefundene_spalten)?;
            return Ok(());
        }

        let Some(letzte_spalten_namen) = spalten_namen_liste.eintraege.last() else {
            return Err("SpaltenNamenListe ist leer".into());
        };

        if self.wende_inferenz_an(
            bereich,
            &letzte_spalten_namen.oberkategorie,
            &letzte_spalten_namen.unterkategorie,
        )? {
            return Ok(());
        }

        self.fallback_zu_standards(bereich);
        Ok(())
    }

    fn setze_gefundene_spalten(
        &self,
        bereich: &mut TextBereich,
        gefundene_spalten: Vec<u32>,
    ) -> Result<(), Box<dyn Error>> {
        let mut sortiert: Vec<usize> = gefundene_spalten.into_iter().map(|n| n as usize).collect();
        sortiert.sort_unstable();
        sortiert.dedup();

        bereich.spalten_bereiche.clear();
        bereich.exact_visible_columns.clear();
        bereich.spaltenreihenfolgeundnurdiese.clear();

        self.fuege_spalten_hinzu(bereich, sortiert.iter().copied());
        Self::markiere_spaltenstatus(bereich, !sortiert.is_empty());
        Ok(())
    }

    fn fallback_zu_standards(&self, bereich: &mut TextBereich) {
        bereich.spalten_bereiche.clear();
        bereich.spaltenreihenfolgeundnurdiese.clear();
        bereich.exact_visible_columns.clear();
        Self::aktualisiere_spaltengrenzen(bereich);
        Self::markiere_spaltenstatus(bereich, false);
    }
}
