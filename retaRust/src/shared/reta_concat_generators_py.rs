#![allow(dead_code)]
#![allow(non_snake_case)]

use std::collections::{BTreeMap, BTreeSet};

use crate::shared::lib4tables_enum_py::ST;
use crate::shared::reta_program_types::{Generated2Selection, GeneratorPairSelection, PairStr, Program};
use crate::shared::reta_generators_inventory_py::{GENERATED1_SPECS, GENERATED2_SPECS, METAKONKRET_SPECS};


#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct PyFrac {
    numerator: i64,
    denominator: i64,
}

fn gcd_i64_py(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    if a == 0 { 1 } else { a }
}

impl PyFrac {
    fn new(numerator: i64, denominator: i64) -> Option<Self> {
        if denominator == 0 || numerator == 0 {
            return None;
        }
        let mut n = numerator;
        let mut d = denominator;
        if d < 0 {
            n = -n;
            d = -d;
        }
        let g = gcd_i64_py(n, d);
        Some(Self { numerator: n / g, denominator: d / g })
    }

    fn mul(self, other: Self) -> Option<Self> {
        Self::new(self.numerator * other.numerator, self.denominator * other.denominator)
    }

    fn div(self, other: Self) -> Option<Self> {
        Self::new(self.numerator * other.denominator, self.denominator * other.numerator)
    }

    fn recip(self) -> Option<Self> {
        Self::new(self.denominator, self.numerator)
    }

    fn is_integer(self) -> bool {
        self.denominator == 1
    }
}

impl Program {
    fn push_unique_i64_py(target: &mut Vec<i64>, value: i64) {
        if !target.contains(&value) {
            target.push(value);
        }
    }

    fn zellenwert_py(&self, zeile: usize, spalte: usize) -> String {
        self.relitable
            .get(zeile)
            .and_then(|row| row.get(spalte))
            .cloned()
            .unwrap_or_default()
    }

    fn setze_zellenwert_py(&mut self, zeile: usize, spalte: usize, wert: String) {
        if zeile >= self.relitable.len() {
            return;
        }
        if spalte >= self.relitable[zeile].len() {
            self.relitable[zeile].resize(spalte + 1, String::new());
        }
        self.relitable[zeile][spalte] = wert;
    }



    fn set_generated_spalten_parameter_exact_from_data_dict_py(&mut self, spalte: i64, data_dict_idx: usize, key: &str) {
        if let Some(entries) = self.dataDict.get(data_dict_idx).and_then(|dict| dict.get(key)) {
            if !entries.is_empty() {
                self.generatedSpaltenParameter_Exact.insert(spalte, entries.clone());
            }
        }
    }

    fn set_generated_spalten_tags_exact_py(&mut self, spalte: i64, tags: &[ST]) {
        let mut collected = BTreeSet::new();
        collected.extend(tags.iter().copied());
        if !collected.is_empty() {
            self.generatedSpaltenParameter_Tags.insert(spalte, collected);
        }
    }

    fn set_generated_spalten_parameter_exact_py(&mut self, spalte: i64, entries: Vec<Vec<PairStr>>) {
        if !entries.is_empty() {
            self.generatedSpaltenParameter_Exact.insert(spalte, entries);
        }
    }

    fn pairstr_group_exact_py(main_name: impl Into<String>, parameter_name: impl Into<String>) -> Vec<PairStr> {
        vec![PairStr(main_name.into(), parameter_name.into())]
    }

    fn generated1_parameter_groups_exact_py(
        &self,
        selection: &GeneratorPairSelection,
        fallback_spalte: i64,
    ) -> Vec<Vec<PairStr>> {
        let main_name = selection.parameter_main_name.trim();
        let parameter_name = selection.parameter_name.trim();
        if !main_name.is_empty() && !parameter_name.is_empty() {
            return vec![Self::pairstr_group_exact_py(main_name, parameter_name)];
        }

        let groups: Vec<Vec<PairStr>> = GENERATED1_SPECS
            .iter()
            .filter(|spec| {
                (spec.col_a == selection.left && spec.col_b == selection.right)
                    || (spec.col_a == selection.right && spec.col_b == selection.left)
            })
            .map(|spec| Self::pairstr_group_exact_py(spec.main_name, spec.parameter_name))
            .collect();
        if !groups.is_empty() {
            return groups;
        }

        let fallback = self.generator_pair_selection_meta_name_exact_py(selection, fallback_spalte);
        if fallback.trim().is_empty() {
            return vec![];
        }
        let main = if main_name.is_empty() {
            "Generiert"
        } else {
            main_name
        };
        let parameter = if parameter_name.is_empty() {
            fallback.as_str()
        } else {
            parameter_name
        };
        vec![Self::pairstr_group_exact_py(main, parameter)]
    }

    fn generated2_raw_names_exact_py(&self, selection: &Generated2Selection) -> (String, String) {
        let mut main_name = selection.parameter_main_name.trim().to_string();
        let mut parameter_name = selection.parameter_name.trim().to_string();
        if main_name.is_empty() || parameter_name.is_empty() {
            if let Some(spec) = GENERATED2_SPECS.iter().find(|spec| spec.code == selection.code) {
                if main_name.is_empty() {
                    main_name = spec.main_name.to_string();
                }
                if parameter_name.is_empty() {
                    parameter_name = spec.parameter_name.to_string();
                }
            }
        }
        (main_name, parameter_name)
    }

    fn metakonkret_parameter_groups_exact_py(&self, selection: &GeneratorPairSelection) -> Vec<Vec<PairStr>> {
        let mut main_name = selection.parameter_main_name.trim().to_string();
        let mut parameter_name = selection.parameter_name.trim().to_string();
        if main_name.is_empty() || parameter_name.is_empty() {
            if let Some(spec) = METAKONKRET_SPECS
                .iter()
                .find(|spec| spec.col_a == selection.left && spec.col_b == selection.right)
            {
                if main_name.is_empty() {
                    main_name = spec.main_name.to_string();
                }
                if parameter_name.is_empty() {
                    parameter_name = spec.parameter_name.to_string();
                }
            }
        }
        if main_name.is_empty() || parameter_name.is_empty() {
            return vec![];
        }
        vec![Self::pairstr_group_exact_py(main_name, parameter_name)]
    }

    fn generated2_spalte_parameter_groups_exact_py(&self, code: &str) -> Vec<Vec<PairStr>> {
        match code {
            "PrimCSV" => vec![Self::pairstr_group_exact_py("Multiplikationen", "Nicht_generiert")],
            "primzahlkreuzprocontra" => vec![
                Self::pairstr_group_exact_py("Bedeutung", "Primzahlkreuz_pro_contra"),
                Self::pairstr_group_exact_py("Pro_Contra", "Primzahlkreuz_pro_contra"),
                Self::pairstr_group_exact_py(
                    "Grundstrukturen",
                    "nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)",
                ),
            ],
            _ => vec![],
        }
    }

    fn generated2_spalte_tags_exact_py(&self, code: &str) -> Vec<ST> {
        match code {
            "PrimCSV" => vec![ST::sternPolygon, ST::universum, ST::galaxie],
            "primzahlkreuzprocontra" => vec![ST::sternPolygon, ST::universum],
            _ => vec![],
        }
    }

    fn generated2_coord_parameter_groups_exact_py(
        &self,
        coord: (usize, usize, bool),
        koord2parameter: &BTreeMap<(usize, usize, bool), Vec<Generated2Selection>>,
    ) -> Vec<Vec<PairStr>> {
        let Some(selections) = koord2parameter.get(&coord) else {
            return vec![];
        };
        let mut groups: Vec<Vec<PairStr>> = Vec::new();
        let mut add_viertwichtigste = false;
        for selection in selections {
            let (_, parameter_name) = self.generated2_raw_names_exact_py(selection);
            if !parameter_name.trim().is_empty() {
                groups.push(Self::pairstr_group_exact_py("Multiplikationen", parameter_name));
            }
            if selection.code == "primMotivStern" {
                add_viertwichtigste = true;
            }
        }
        if add_viertwichtigste {
            groups.push(Self::pairstr_group_exact_py(
                "Wichtigstes_zum_verstehen",
                "Viertwichtigste",
            ));
        }
        groups
    }

    fn wrap_items_exact_py(&self, items: &[String], wrap_empty_lists: bool) -> String {
        if self.outType == "html" {
            if items.is_empty() && !wrap_empty_lists {
                return String::new();
            }
            let mut out = String::from("<ul>");
            for item in items {
                out.push_str("<li>");
                out.push_str(item);
                out.push_str("</li>");
            }
            out.push_str("</ul>");
            out
        } else if self.outType == "bbcode" {
            if items.is_empty() && !wrap_empty_lists {
                return String::new();
            }
            let mut out = String::from("[list]");
            for item in items {
                out.push_str("[*]");
                out.push_str(item);
            }
            out.push_str("[/list]");
            out
        } else {
            items.join(" | ")
        }
    }

    fn primzahlkreuz_heading_exact_py(&self) -> &'static str {
        "Gegen / pro: Nach Rechenregeln auf Primzahlkreuz und Vielfachern von Primzahlen"
    }

    fn concat_table_generated_tags_exact_py(&self, concatTable: i64) -> Vec<ST> {
        match concatTable {
            1 => vec![ST::sternPolygon, ST::universum, ST::galaxie],
            2 | 3 => vec![ST::sternPolygon, ST::galaxie, ST::gleichfoermigesPolygon, ST::gebrRat],
            4 | 5 => vec![ST::sternPolygon, ST::universum, ST::gleichfoermigesPolygon, ST::gebrRat],
            6 | 7 => vec![ST::sternPolygon, ST::keinParaOdMetaP, ST::gleichfoermigesPolygon, ST::gebrRat],
            8 | 9 => vec![ST::sternPolygon, ST::gleichfoermigesPolygon, ST::gebrRat, ST::keinParaOdMetaP],
            _ => vec![],
        }
    }
    fn fuege_spalte_hinzu_py(&mut self, zeilenInhalte: Vec<String>, meta_name: &str) -> i64 {
        let spaltenNummer = self.relitable.first().map(|row| row.len()).unwrap_or(0) as i64;
        let zielZeilen = std::cmp::max(self.relitable.len(), zeilenInhalte.len());
        if self.relitable.is_empty() && zielZeilen > 0 {
            self.relitable = vec![vec![]; zielZeilen];
        }
        while self.relitable.len() < zielZeilen {
            let breite = self.relitable.first().map(|row| row.len()).unwrap_or(0);
            self.relitable.push(vec![String::new(); breite]);
        }
        for row in self.relitable.iter_mut() {
            row.push(String::new());
        }
        for (i, wert) in zeilenInhalte.into_iter().enumerate() {
            if i < self.relitable.len() {
                if let Some(last) = self.relitable[i].last_mut() {
                    *last = wert;
                }
            }
        }
        if !meta_name.is_empty() {
            self.generatedSpaltenParameter.push(meta_name.to_string());
        }
        spaltenNummer
    }

    fn hat_generated2_code_py(&self, code: &str) -> bool {
        self.generated2Codes.iter().any(|v| v == code)
            || self.generated2Selections.iter().any(|selection| selection.code == code)
    }

    fn remove_concat1_trigger_columns_py(&self, rowsAsNumbers: &mut Vec<i64>) {
        if self.puniverseprims.is_empty() {
            return;
        }
        rowsAsNumbers.retain(|n| !self.puniverseprims.contains(n));
    }

    fn should_show_concat1_non_generated_column_py(&self) -> bool {
        self.generated2_selections_exact_py().is_empty()
    }

    fn boolAndTupleSet1Options_exact_py(&self) -> Vec<Option<usize>> {
        self.boolAndTupleSet1Options.iter().map(|v| v.map(|x| x as usize)).collect()
    }

    
    fn meta_or_what_exact_py(&self, metavariable: i64) -> ((&'static str, &'static str), (&'static str, &'static str)) {
        match metavariable {
            2 => (("Meta-Thema: ", "Konkretes: "), ("Meta-", "Konkret-")),
            3 => (("Theorie-Thema: ", "Praxis: "), ("Theorie-", "Praxis-")),
            4 => (("Planungs-Thema: ", "Umsetzungs-Thema: "), ("Planung-", "Umsetzung-")),
            5 => (("Anlass-Thema: ", "Wirkungs-Thema: "), ("Anlass-", "wirkung-")),
            6 => (("Kraft-Gebung: ", "Verstärkungs-Thema: "), ("Kraft-geben-", "Verstärkung-")),
            7 => (("Beherrschung: ", "Richtung-Thema: "), ("beherrschend-", "Richtung-")),
            _ => (("Meta-Thema: ", "Konkretes: "), ("Meta-", "Konkret-")),
        }
    }


    fn make_vorwort_exact_py(&self, wiederholungen: usize, vorworte2: (&str, &str), less1ormore2: usize) -> String {
        let basis = if less1ormore2 == 1 { vorworte2.0 } else { vorworte2.1 };
        if wiederholungen > 1 {
            basis.repeat(wiederholungen)
        } else {
            basis.to_string()
        }
    }


    fn py_frac_to_f64_exact(&self, value: PyFrac) -> f64 {
        value.numerator as f64 / value.denominator as f64
    }


    fn py_frac_display_exact(&self, value: PyFrac) -> String {
        if value.denominator == 1 {
            value.numerator.to_string()
        } else {
            format!("{}/{}", value.numerator, value.denominator)
        }
    }


    fn spalteMetaKonkret_switching_exact_py(
        &self,
        transzendentalienSpalten: (usize, usize),
        ifInvers: usize,
        metavariable: i64,
        newCol: usize,
        moreAndLess: (Option<i64>, Option<PyFrac>),
        gebrRatEtwaSchonMalDabeiGewesen: &mut BTreeSet<PyFrac>,
    ) -> (usize, (Option<i64>, Option<PyFrac>)) {
        let next_col = if newCol == transzendentalienSpalten.1 {
            transzendentalienSpalten.0
        } else {
            transzendentalienSpalten.1
        };

        let a = moreAndLess.0.and_then(|left| {
            let mulresult = left.saturating_mul(metavariable);
            if mulresult > 0 && (mulresult as usize) < self.relitable.len() {
                Some(mulresult)
            } else {
                None
            }
        });

        let mut right_current = moreAndLess.1;
        let b = if let Some(mut right) = right_current {
            let right_f = self.py_frac_to_f64_exact(right);
            if right_f < 100.0 && right_f > 0.01 {
                if next_col == (if ifInvers == 0 { transzendentalienSpalten.0 } else { transzendentalienSpalten.1 }) && right.denominator == 1 {
                    if let Some(rec) = right.recip() {
                        right = rec;
                        right_current = Some(rec);
                    }
                }
                let candidate = if self.spalteMetaKonkretAbstrakt_isGanzZahlig_py(self.py_frac_to_f64_exact(right), false) {
                    PyFrac::new(metavariable, right.numerator)
                } else {
                    right.recip().and_then(|rec| rec.div(PyFrac::new(metavariable, 1)?))
                };
                if let Some(frac) = candidate {
                    if gebrRatEtwaSchonMalDabeiGewesen.contains(&frac) {
                        None
                    } else {
                        gebrRatEtwaSchonMalDabeiGewesen.insert(frac);
                        Some(frac)
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let _ = right_current;
        (next_col, (a, b))
    }


    fn spalteMetaKonkret_vorwort_behandlung_exact_py(
        &self,
        metavariable: i64,
        ifInvers: usize,
        transzendentalienSpalten: (usize, usize),
        start_row: i64,
        metaOrWhat: ((&str, &str), (&str, &str)),
    ) -> Vec<((Option<i64>, Option<PyFrac>), usize, String, String)> {
        let mut gebrRatEtwaSchonMalDabeiGewesen: BTreeSet<PyFrac> = BTreeSet::new();
        let mut moreAndLess = (Some(start_row), PyFrac::new(start_row, 1));
        let mut newCol = transzendentalienSpalten.0;
        let mut neue2KoordNeue2Vorwoerter: Vec<((Option<i64>, Option<PyFrac>), usize, String, String)> = vec![];
        while !(moreAndLess.0.is_none() && moreAndLess.1.is_none()) {
            let switched = self.spalteMetaKonkret_switching_exact_py(
                transzendentalienSpalten,
                ifInvers,
                metavariable,
                newCol,
                moreAndLess,
                &mut gebrRatEtwaSchonMalDabeiGewesen,
            );
            newCol = switched.0;
            moreAndLess = switched.1;
            let vorworte2 = if neue2KoordNeue2Vorwoerter.is_empty() {
                metaOrWhat.0
            } else {
                metaOrWhat.1
            };
            let vorwort1 = self.make_vorwort_exact_py(neue2KoordNeue2Vorwoerter.len() + 1, vorworte2, 1);
            let vorwort2 = self.make_vorwort_exact_py(neue2KoordNeue2Vorwoerter.len() + 1, vorworte2, 2);
            neue2KoordNeue2Vorwoerter.push((moreAndLess, newCol, vorwort1, vorwort2));
        }
        neue2KoordNeue2Vorwoerter
    }


    fn spalteMetaKonkret_main_inserting_text_exact_py(
        &self,
        bothRows: i64,
        _i: usize,
        ifInvers: usize,
        neue2KoordNeue2Vorwoerter: &Vec<((Option<i64>, Option<PyFrac>), usize, String, String)>,
        transzendentalienSpalten: (usize, usize),
        gebr_table: &Vec<Vec<String>>,
    ) -> String {
        let mut items: Vec<String> = vec![];
        let mut thema = String::new();
        for vier in neue2KoordNeue2Vorwoerter
            .iter()
            .take(neue2KoordNeue2Vorwoerter.len().saturating_sub(1))
        {
            if bothRows == 0 {
                if let Some(row_idx) = vier.0.0 {
                    let text = self.zellenwert_py(row_idx as usize, vier.1);
                    if text.trim().len() > 3 {
                        let prefix = if vier.1
                            != (if ifInvers == 0 {
                                transzendentalienSpalten.0
                            } else {
                                transzendentalienSpalten.1
                            })
                            && row_idx != 1
                        {
                            "1/"
                        } else {
                            ""
                        };
                        items.push(format!("{}{}{} ({prefix}{row_idx})", vier.2, thema, text));
                    }
                }
            } else if let Some(frac) = vier.0.1 {
                if frac.denominator == 1 {
                    let row_idx = frac.numerator;
                    let text = self.zellenwert_py(row_idx as usize, vier.1);
                    if text.trim().len() > 3 {
                        let prefix = if vier.1
                            != (if ifInvers == 0 {
                                transzendentalienSpalten.0
                            } else {
                                transzendentalienSpalten.1
                            })
                            && row_idx != 1
                        {
                            "1/"
                        } else {
                            ""
                        };
                        items.push(format!("{}{}{} ({prefix}{row_idx})", vier.3, thema, text));
                    }
                } else if let Some(gebrStrukWort) = self
                    .spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie_py(
                        frac,
                        transzendentalienSpalten,
                        gebr_table,
                        false,
                    )
                {
                    if gebrStrukWort.trim().len() > 3 {
                        let frac_display = if frac.denominator > 1 {
                            format!("{}/{}", frac.numerator, frac.denominator)
                        } else {
                            frac.numerator.to_string()
                        };
                        items.push(format!("{}{}{}({})", vier.3, thema, gebrStrukWort, frac_display));
                    }
                }
            }
            thema = "thema: ".to_string();
        }
        self.wrap_items_exact_py(&items, false)
    }

fn metakonkret_pairs_exact_py(&self) -> Vec<(i64, i64)> {
        self.metakonkretPairs.clone()
    }

    fn generator_row_end_py(&self) -> usize {
        // Python-näher: Generatoren rechnen über die bereits aufgebaute Gesamttabelle
        // und die spätere Ausgabe filtert erst danach. Kein Kappen auf aktuell sichtbare
        // Zeilenwünsche, solange die Tabelle die Zeilen bereits trägt.
        self.relitable.len().saturating_sub(1)
    }

    fn spalteMetaKonkretAbstrakt_isGanzZahlig_py(&self, zahl: f64, spaltenWahl: bool) -> bool {
        let mut zahl = if spaltenWahl { 1.0 / zahl } else { zahl };
        zahl = zahl.fract().abs();
        zahl < 0.00001 || zahl > 0.99999
    }

    fn generated1_selections_exact_py(&self) -> Vec<GeneratorPairSelection> {
        if !self.generated1Selections.is_empty() {
            return self.generated1Selections.clone();
        }
        self.generated1Pairs
            .iter()
            .map(|pair| GeneratorPairSelection {
                parameter_main_name: String::new(),
                parameter_name: String::new(),
                left: pair.0,
                right: pair.1,
            })
            .collect()
    }

    fn metakonkret_selections_exact_py(&self) -> Vec<GeneratorPairSelection> {
        if !self.metakonkretSelections.is_empty() {
            return self.metakonkretSelections.clone();
        }
        self.metakonkretPairs
            .iter()
            .map(|pair| GeneratorPairSelection {
                parameter_main_name: String::new(),
                parameter_name: String::new(),
                left: pair.0,
                right: pair.1,
            })
            .collect()
    }

    fn generator_pair_selection_meta_name_exact_py(&self, selection: &GeneratorPairSelection, fallback_spalte: i64) -> String {
        let mut teile: Vec<String> = Vec::new();
        if !selection.parameter_main_name.trim().is_empty() {
            teile.push(selection.parameter_main_name.trim().to_string());
        }
        if !selection.parameter_name.trim().is_empty() {
            teile.push(selection.parameter_name.trim().replace('_', " "));
        }
        if !teile.is_empty() {
            return teile.join(" ");
        }
        self.generierte_spalte_meta_name_py(fallback_spalte)
    }

    fn generated1_pairs_exact_py(&self) -> Vec<(i64, i64)> {
        if !self.generated1Pairs.is_empty() {
            return self.generated1Pairs.clone();
        }
        let gener_rows: BTreeSet<i64> = self.generRows.iter().copied().collect();
        GENERATED1_SPECS
            .iter()
            .filter(|spec| gener_rows.contains(&spec.col_a) || gener_rows.contains(&spec.col_b))
            .map(|spec| (spec.col_a, spec.col_b))
            .collect()
    }


    fn nicht_leere_teile_join_py(&self, teile: Vec<String>, sep: &str) -> String {
        let mut neu: Vec<String> = vec![];
        for teil in teile {
            if !teil.trim().is_empty() {
                neu.push(teil);
            }
        }
        neu.join(sep)
    }

    fn modalTextByDistance_py(&self, distanceFromLine: i64) -> String {
        match distanceFromLine.abs() {
            0 => "sehr: ".to_string(),
            1 => "überdurchschnittlich: ".to_string(),
            2 => "mittelstark überdurchschnittlich: ".to_string(),
            3 => "mittelleicht überdurchschnittlich: ".to_string(),
            _ => "sehr leicht überdurchschnittlich: ".to_string(),
        }
    }

    fn generated2_code_heading_py(&self, code: &str) -> String {
        if code == "primzahlkreuzprocontra" {
            return self.primzahlkreuz_heading_exact_py().to_string();
        }
        for spec in GENERATED2_SPECS {
            if spec.code == code {
                return format!("{} {}", spec.main_name, spec.parameter_name);
            }
        }
        code.to_string()
    }

    fn dedup_preserve_order_strings_py(input: Vec<String>) -> Vec<String> {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut out: Vec<String> = Vec::new();
        for item in input {
            if seen.insert(item.clone()) {
                out.push(item);
            }
        }
        out
    }

    fn push_unique_i64_vec_py(target: &mut Vec<i64>, value: i64) {
        if !target.contains(&value) {
            target.push(value);
        }
    }

    fn primzahlkreuz_pairs_exact_py(&self, num: i64) -> Vec<(i64, i64)> {
        let mut out: Vec<(i64, i64)> = Vec::new();
        let mut seen: BTreeSet<(i64, i64)> = BTreeSet::new();
        for couple in self.primMultiple_pairs_py(num) {
            let mut a = couple.0;
            let mut b = couple.1;
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            let ordered = (a, b);
            if seen.insert(ordered) {
                out.push(ordered);
            }
        }
        out
    }

    fn concat1_main_cell_exact_py(&self, num: i64, into: Vec<String>, into1: Vec<String>, into2: Vec<String>) -> String {
        if num == 0 {
            return into.join(" | ");
        }
        let empathie = format!(" Darin kann sich die {} am Besten hineinversetzen.", num);
        let into_b: Vec<String> = if self.outType == "html" {
            vec![
                "<ul>".to_string(),
                if !into1.is_empty() { "<li>".to_string() } else { String::new() },
                into1.join(", "),
                if !into1.is_empty() { empathie.clone() } else { String::new() },
                if !into1.is_empty() { "</li>".to_string() } else { String::new() },
                if !into2.is_empty() { "<li>".to_string() } else { String::new() },
                into2.join(", "),
                if !into2.is_empty() { empathie.clone() } else { String::new() },
                if !into2.is_empty() { "</li>".to_string() } else { String::new() },
                if !into.is_empty() { "<li>".to_string() } else { String::new() },
                into.join(", "),
                if !into.is_empty() { "</li>".to_string() } else { String::new() },
                "</ul>".to_string(),
            ]
        } else if self.outType == "bbcode" {
            vec![
                "[list]".to_string(),
                if !into1.is_empty() { "[*]".to_string() } else { String::new() },
                into1.join(", "),
                if !into1.is_empty() { empathie.clone() } else { String::new() },
                if !into2.is_empty() { "[*]".to_string() } else { String::new() },
                into2.join(", "),
                if !into2.is_empty() { empathie.clone() } else { String::new() },
                if !into.is_empty() { "[*]".to_string() } else { String::new() },
                into.join(", "),
                "[/list]".to_string(),
            ]
        } else {
            vec![
                into1.join(", "),
                if !into1.is_empty() { empathie.clone() } else { String::new() },
                into2.join(", "),
                if !into2.is_empty() { empathie } else { String::new() },
                into.join(", "),
            ]
        };
        let mut filtered: Vec<String> = Vec::new();
        for item in into_b {
            if !item.is_empty() {
                filtered.push(item);
            }
        }
        if self.outType == "html" || self.outType == "bbcode" {
            filtered.join("")
        } else {
            filtered.join(" | ")
        }
    }

    fn concat1_reverse_hints_exact_py(&self, dreli: &Vec<Vec<String>>, num: i64, values: &Vec<i64>, sep: &str) -> String {
        let mut hints: Vec<String> = Vec::new();
        for c in values {
            let cu = *c as usize;
            if let Some(row) = dreli.get(cu) {
                if let Some(cell) = row.get(206) {
                    let parts: Vec<&str> = cell.split('|').collect();
                    if parts.len() == 2 {
                        if parts[0].trim().parse::<i64>().ok() == Some(num) {
                            let rhs = parts[1].trim();
                            if !rhs.is_empty() {
                                hints.push(rhs.to_string());
                            }
                        }
                    }
                }
            }
        }
        hints.join(sep)
    }

    fn concat1_reverse_cell_exact_py(&self, num: i64, pro2: Vec<i64>, contra2: Vec<i64>, dreli: &Vec<Vec<String>>) -> String {
        if num == 0 {
            return self.generated2_code_heading_py("primzahlkreuzprocontra");
        }
        if pro2.is_empty() && contra2.is_empty() {
            return "-".to_string();
        }
        let dahinter1 = self.concat1_reverse_hints_exact_py(dreli, num, &pro2, " , ");
        let dahinter2 = self.concat1_reverse_hints_exact_py(dreli, num, &contra2, ", ");
        let pro_text = if pro2.len() > 1 {
            format!("pro dieser Zahl sind: {}", pro2.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "))
        } else if pro2.len() == 1 {
            format!("pro dieser Zahl ist {}", pro2[0])
        } else {
            String::new()
        };
        let contra_text = if contra2.len() > 1 {
            format!(" contra dieser Zahl sind: {}", contra2.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "))
        } else if contra2.len() == 1 {
            format!(" contra dieser Zahl ist {}", contra2[0])
        } else {
            String::new()
        };
        let parts: Vec<String> = if self.outType == "bbcode" {
            vec![
                "[list]".to_string(),
                if !pro2.is_empty() { "[*]".to_string() } else { String::new() },
                pro_text,
                if !dahinter1.is_empty() { "[*]".to_string() } else { String::new() },
                dahinter1,
                if !contra2.is_empty() { "[*]".to_string() } else { String::new() },
                contra_text,
                if !dahinter2.is_empty() { "[*]".to_string() } else { String::new() },
                dahinter2,
                "[/list]".to_string(),
                "hineinversetzen/empathisch dazu sein".to_string(),
            ]
        } else if self.outType == "html" {
            vec![
                "<ul>".to_string(),
                if !pro2.is_empty() { "<li>".to_string() } else { String::new() },
                pro_text,
                if !pro2.is_empty() { "</li>".to_string() } else { String::new() },
                if !dahinter1.is_empty() { "<li>".to_string() } else { String::new() },
                dahinter1.clone(),
                if !dahinter1.is_empty() { "</li>".to_string() } else { String::new() },
                if !contra2.is_empty() { "<li>".to_string() } else { String::new() },
                contra_text,
                if !contra2.is_empty() { "</li>".to_string() } else { String::new() },
                if !dahinter2.is_empty() { "<li>".to_string() } else { String::new() },
                dahinter2.clone(),
                if !dahinter2.is_empty() { "</li>".to_string() } else { String::new() },
                "</ul>".to_string(),
                "hineinversetzen/empathisch dazu sein".to_string(),
            ]
        } else {
            let mut shell: Vec<String> = Vec::new();
            if !pro_text.is_empty() { shell.push(pro_text); }
            if !dahinter1.is_empty() { shell.push(format!("({})", dahinter1)); }
            if !contra_text.is_empty() { shell.push(contra_text); }
            if !dahinter2.is_empty() { shell.push(format!("({})", dahinter2)); }
            shell.push("hineinversetzen/empathisch dazu sein".to_string());
            return shell.join(" | ");
        };
        let mut filtered: Vec<String> = Vec::new();
        for item in parts {
            if !item.is_empty() {
                filtered.push(item);
            }
        }
        filtered.join("")
    }

    fn register_generated2_spalte_exact_py(&mut self, code: &str, spalte: i64) {
        self.spaltenArtenKey_SpaltennummernValue
            .entry(self.spaltenTypeNaming.generated2)
            .or_default()
            .insert(spalte);
        for spec in GENERATED2_SPECS {
            if spec.code == code {
                let key = (spec.main_name.to_string(), spec.parameter_name.to_string());
                self.paraDictGenerated.insert(key.clone(), spalte);
                self.paraDictGenerated4htmlTags.insert(key, spalte);
            }
        }
        let groups = self.generated2_spalte_parameter_groups_exact_py(code);
        self.set_generated_spalten_parameter_exact_py(spalte, groups);
        let tags = self.generated2_spalte_tags_exact_py(code);
        self.set_generated_spalten_tags_exact_py(spalte, &tags);
    }

    fn generated2_code_source_columns_py(&self, code: &str) -> (usize, usize) {
        match code {
            "primMotivStern" => (10, 5),
            "primStrukStern" => (5, 10),
            "primMotivGleichf" => (42, 131),
            "primStrukGleichf" => (131, 42),
            "primMotivSternGebr" => (138, 202),
            "primStrukSternGebr" => (202, 138),
            "primMotivGleichfGebr" => (44, 56),
            "primStrukGleichfGebr" => (56, 44),
            "PrimCSV" => {
                let first = self.CsvTheirsSpalten.get(&1).and_then(|v| v.first()).copied().unwrap_or(19);
                (first as usize, first as usize)
            }
            _ => (10, 5),
        }
    }


    fn generated2_exact_coords_py(&self, code: &str) -> Option<(usize, &'static str, Vec<usize>, bool)> {
        match code {
            "primMotivStern" => Some((0, "Sternpolygone", vec![0, 1, 2], false)),
            "primStrukStern" => Some((0, "Sternpolygone", vec![1, 2, 3], false)),
            "primMotivGleichf" => Some((1, "gleichförmige Polygone", vec![0, 1, 2], false)),
            "primStrukGleichf" => Some((1, "gleichförmige Polygone", vec![1, 2, 3], false)),
            "primMotivSternGebr" => Some((0, "Sternpolygone", vec![0, 1, 2], true)),
            "primStrukSternGebr" => Some((0, "Sternpolygone", vec![1, 2, 3], true)),
            "primMotivGleichfGebr" => Some((1, "gleichförmige Polygone", vec![0, 1, 2], true)),
            "primStrukGleichfGebr" => Some((1, "gleichförmige Polygone", vec![1, 2, 3], true)),
            _ => None,
        }
    }


    fn generated2_coord_tag_values_exact_py(&self, code: &str) -> Vec<((usize, usize, bool), Vec<ST>)> {
        let mk = |poly: ST, gal: bool, uni: bool, gebr: bool| -> Vec<ST> {
            let mut tags: Vec<ST> = vec![poly];
            if gal {
                tags.push(ST::galaxie);
            }
            if uni {
                tags.push(ST::universum);
            }
            if gebr {
                tags.push(ST::gebrRat);
            }
            tags
        };
        match code {
            "primMotivSternGebr" => vec![
                ((0, 0, true), mk(ST::sternPolygon, true, false, true)),
                ((0, 1, true), mk(ST::sternPolygon, true, true, true)),
                ((0, 2, true), mk(ST::sternPolygon, true, true, true)),
            ],
            "primStrukSternGebr" => vec![
                ((0, 1, true), mk(ST::sternPolygon, true, true, true)),
                ((0, 2, true), mk(ST::sternPolygon, true, true, true)),
                ((0, 3, true), mk(ST::sternPolygon, false, true, true)),
            ],
            "primMotivGleichfGebr" => vec![
                ((1, 0, true), mk(ST::gleichfoermigesPolygon, true, false, true)),
                ((1, 1, true), mk(ST::gleichfoermigesPolygon, true, true, true)),
                ((1, 2, true), mk(ST::gleichfoermigesPolygon, true, true, true)),
            ],
            "primStrukGleichfGebr" => vec![
                ((1, 1, true), mk(ST::gleichfoermigesPolygon, true, true, true)),
                ((1, 2, true), mk(ST::gleichfoermigesPolygon, true, true, true)),
                ((1, 3, true), mk(ST::gleichfoermigesPolygon, false, true, true)),
            ],
            "primMotivStern" => vec![
                ((0, 0, false), mk(ST::sternPolygon, true, false, false)),
                ((0, 1, false), mk(ST::sternPolygon, true, true, false)),
                ((0, 2, false), mk(ST::sternPolygon, true, true, false)),
            ],
            "primStrukStern" => vec![
                ((0, 1, false), mk(ST::sternPolygon, true, true, false)),
                ((0, 2, false), mk(ST::sternPolygon, true, true, false)),
                ((0, 3, false), mk(ST::sternPolygon, false, true, false)),
            ],
            "primMotivGleichf" => vec![
                ((1, 0, false), mk(ST::gleichfoermigesPolygon, true, false, false)),
                ((1, 1, false), mk(ST::gleichfoermigesPolygon, true, true, false)),
                ((1, 2, false), mk(ST::gleichfoermigesPolygon, true, true, false)),
            ],
            "primStrukGleichf" => vec![
                ((1, 1, false), mk(ST::gleichfoermigesPolygon, true, true, false)),
                ((1, 2, false), mk(ST::gleichfoermigesPolygon, true, true, false)),
                ((1, 3, false), mk(ST::gleichfoermigesPolygon, false, true, false)),
            ],
            _ => vec![],
        }
    }

    fn generated2_coord_parameters_exact_py(
        &self,
        generatedSelections: &Vec<Generated2Selection>,
    ) -> BTreeMap<(usize, usize, bool), Vec<Generated2Selection>> {
        let mut koord2parameter: BTreeMap<(usize, usize, bool), Vec<Generated2Selection>> = BTreeMap::new();
        for selection in generatedSelections {
            if let Some((poly_idx, _poly_name, kombis, is_gebr)) = self.generated2_exact_coords_py(&selection.code) {
                for kombi_idx in kombis {
                    let entry = koord2parameter.entry((poly_idx, kombi_idx, is_gebr)).or_default();
                    if !entry.iter().any(|existing| existing.code == selection.code) {
                        entry.push(selection.clone());
                    }
                }
            }
        }
        koord2parameter
    }

    fn generated2_coord_tags_exact_py(
        &self,
        generatedSelections: &Vec<Generated2Selection>,
    ) -> BTreeMap<(usize, usize, bool), Vec<ST>> {
        let mut koord2tag: BTreeMap<(usize, usize, bool), Vec<ST>> = BTreeMap::new();
        for selection in generatedSelections {
            for (coord, tags) in self.generated2_coord_tag_values_exact_py(&selection.code) {
                koord2tag.entry(coord).or_insert(tags);
            }
        }
        koord2tag
    }

    fn register_generated2_coord_metadata_exact_py(
        &mut self,
        coord: (usize, usize, bool),
        spalte: i64,
        koord2parameter: &BTreeMap<(usize, usize, bool), Vec<Generated2Selection>>,
        koord2tag: &BTreeMap<(usize, usize, bool), Vec<ST>>,
    ) {
        self.spaltenArtenKey_SpaltennummernValue
            .entry(self.spaltenTypeNaming.generated2)
            .or_default()
            .insert(spalte);

        if let Some(selections) = koord2parameter.get(&coord) {
            for selection in selections {
                let mut key = (
                    self.normalize_generated2_display_main_name_py(&selection.parameter_main_name),
                    self.normalize_generated2_display_parameter_name_py(&selection.parameter_name),
                );
                if key.0.is_empty() || key.1.is_empty() {
                    for spec in GENERATED2_SPECS {
                        if spec.code == selection.code {
                            key = (spec.main_name.to_string(), spec.parameter_name.to_string());
                            break;
                        }
                    }
                }
                self.paraDictGenerated.insert(key.clone(), spalte);
                self.paraDictGenerated4htmlTags.insert(key, spalte);
            }
        }

        let groups = self.generated2_coord_parameter_groups_exact_py(coord, koord2parameter);
        self.set_generated_spalten_parameter_exact_py(spalte, groups);

        if let Some(tags) = koord2tag.get(&coord) {
            self.set_generated_spalten_tags_exact_py(spalte, tags);
            let tag_label = tags
                .iter()
                .map(|tag| tag.py_name())
                .collect::<Vec<_>>()
                .join("|");
            self.paraDictGenerated4htmlTags
                .insert(("generated2_tags".to_string(), tag_label), spalte);
        }
    }

    fn py_frac_from_f64_key_exact(value: f64) -> Option<PyFrac> {
        let rounded = value.round();
        if (value - rounded).abs() < 0.00001 {
            return PyFrac::new(rounded as i64, 1);
        }
        None
    }

    fn csv_fraction_table_name_py(&self, concatTable: i64) -> Option<&'static str> {
        match concatTable {
            1 => Some("primenumbers.csv"),
            2 => Some("gebrochen-rational-universum.csv"),
            3 => Some("gebrochen-rational-galaxie.csv"),
            4 => Some("gebrochen-rational-universum.csv"),
            5 => Some("gebrochen-rational-galaxie.csv"),
            _ => None,
        }
    }

    fn get_all_brueche_py(&self, table: &Vec<Vec<String>>) -> BTreeSet<PyFrac> {
        let mut menge = BTreeSet::new();
        for (i, row) in table.iter().enumerate().skip(1) {
            for (k, cell) in row.iter().enumerate().skip(1) {
                if cell.trim().len() > 3 {
                    if let Some(frac) = PyFrac::new((i + 1) as i64, (k + 1) as i64) {
                        if frac.denominator != 1 && frac.numerator != 1 {
                            menge.insert(frac);
                        }
                    }
                }
            }
        }
        menge
    }

    fn convert_set_of_paaren_to_dict_mul_py(
        &self,
        paare_set: &BTreeSet<(PyFrac, PyFrac)>,
        gleichf: bool,
        limit: usize,
    ) -> BTreeMap<usize, Vec<(PyFrac, PyFrac)>> {
        let mut result: BTreeMap<usize, BTreeSet<(PyFrac, PyFrac)>> = BTreeMap::new();
        for paar in paare_set.iter().copied() {
            let Some(prod) = paar.0.mul(paar.1) else { continue; };
            let key = if gleichf {
                let Some(inv) = prod.recip() else { continue; };
                if !inv.is_integer() { continue; }
                inv.numerator as usize
            } else {
                if !prod.is_integer() { continue; }
                prod.numerator as usize
            };
            if key <= limit {
                result.entry(key).or_default().insert(paar);
            }
        }
        result.into_iter().map(|(k, v)| (k, v.into_iter().collect())).collect()
    }

    fn convert_fractions_to_dict_mul_py(
        &self,
        fracs: &BTreeSet<PyFrac>,
        fracs2: &BTreeSet<PyFrac>,
        gleichf: bool,
        limit: usize,
    ) -> BTreeMap<usize, Vec<(PyFrac, PyFrac)>> {
        let mut result: BTreeMap<usize, BTreeSet<(PyFrac, PyFrac)>> = BTreeMap::new();
        if !gleichf {
            for frac in fracs.iter().copied() {
                for zusatz_mul in 1..=limit {
                    let Some(f2) = PyFrac::new(frac.denominator * zusatz_mul as i64, 1) else { continue; };
                    let paar = (frac, f2);
                    let Some(prod) = paar.0.mul(paar.1) else { continue; };
                    if !prod.is_integer() { continue; }
                    let key = prod.numerator as usize;
                    if key > limit { break; }
                    result.entry(key).or_default().insert(paar);
                }
            }
            for frac in fracs.iter().copied() {
                for zusatz_mul in (1..=limit).rev() {
                    let Some(faktor) = PyFrac::new(frac.denominator, zusatz_mul as i64) else { continue; };
                    if fracs2.contains(&faktor) || faktor.numerator == 1 {
                        let paar = (frac, faktor);
                        let Some(prod) = paar.0.mul(paar.1) else { continue; };
                        if !prod.is_integer() { continue; }
                        let key = prod.numerator as usize;
                        if key > limit { break; }
                        result.entry(key).or_default().insert(paar);
                    }
                }
            }
        } else {
            for frac in fracs.iter().copied() {
                for zusatz_div in 1..=limit {
                    let Some(f2) = PyFrac::new(1, frac.numerator * zusatz_div as i64) else { continue; };
                    let paar = (frac, f2);
                    let Some(prod) = paar.0.mul(paar.1) else { continue; };
                    let Some(inv) = prod.recip() else { continue; };
                    if !inv.is_integer() { continue; }
                    let key = inv.numerator as usize;
                    if key > limit { break; }
                    result.entry(key).or_default().insert(paar);
                }
            }
            for frac in fracs.iter().copied() {
                for zusatz_div in 1..=limit {
                    let Some(recip) = frac.recip() else { continue; };
                    let Some(faktor) = recip.div(PyFrac::new(zusatz_div as i64, 1).unwrap()) else { continue; };
                    if fracs2.contains(&faktor) || faktor.numerator == 1 {
                        let paar = (frac, faktor);
                        let Some(prod) = paar.0.mul(paar.1) else { continue; };
                        let Some(inv) = prod.recip() else { continue; };
                        if !inv.is_integer() { continue; }
                        let key = inv.numerator as usize;
                        if key > limit { break; }
                        result.entry(key).or_default().insert(paar);
                    }
                }
            }
        }
        result.into_iter().map(|(k, v)| (k, v.into_iter().collect())).collect()
    }

    fn combine_dicts_pairs_py(
        &self,
        a: BTreeMap<usize, Vec<(PyFrac, PyFrac)>>,
        b: BTreeMap<usize, Vec<(PyFrac, PyFrac)>>,
    ) -> BTreeMap<usize, Vec<(PyFrac, PyFrac)>> {
        let mut e: BTreeMap<usize, BTreeSet<(PyFrac, PyFrac)>> = BTreeMap::new();
        for (k, vals) in a.into_iter().chain(b.into_iter()) {
            for mut v in vals {
                if v.1 < v.0 {
                    v = (v.1, v.0);
                }
                e.entry(k).or_default().insert(v);
            }
        }
        e.into_iter().map(|(k, v)| (k, v.into_iter().collect())).collect()
    }

    fn find_all_brueche_and_their_combinations_py(
        &self,
        limit: usize,
    ) -> BTreeMap<String, BTreeMap<String, BTreeMap<String, BTreeMap<usize, Vec<(PyFrac, PyFrac)>>>>> {
        let uni_name = self.csv_fraction_table_name_py(2).unwrap();
        let gal_name = self.csv_fraction_table_name_py(3).unwrap();
        let uni_table = self.load_csv_rows_semicolon_exact_path(uni_name).unwrap_or_default();
        let gal_table = self.load_csv_rows_semicolon_exact_path(gal_name).unwrap_or_default();
        let brueche_uni = self.get_all_brueche_py(&uni_table);
        let brueche_gal = self.get_all_brueche_py(&gal_table);
        let mut gebr_rat_all_combis: BTreeMap<String, BTreeMap<String, BTreeMap<String, BTreeSet<(PyFrac, PyFrac)>>>> = BTreeMap::new();
        for k in ["UniUni", "UniGal", "GalUni", "GalGal"] {
            let mut poly = BTreeMap::new();
            for p in ["stern", "gleichf"] {
                let mut md = BTreeMap::new();
                md.insert("mul".to_string(), BTreeSet::new());
                md.insert("div".to_string(), BTreeSet::new());
                poly.insert(p.to_string(), md);
            }
            gebr_rat_all_combis.insert(k.to_string(), poly);
        }
        let combos = [
            (&brueche_gal, &brueche_gal, "GalGal"),
            (&brueche_gal, &brueche_uni, "GalUni"),
            (&brueche_uni, &brueche_gal, "UniGal"),
            (&brueche_uni, &brueche_uni, "UniUni"),
        ];
        for (br1, br2, key) in combos {
            for &f1 in br1 {
                for &f2 in br2 {
                    if f1 == f2 { continue; }
                    if let Some(prod) = f1.mul(f2) {
                        if prod.is_integer() {
                            gebr_rat_all_combis.get_mut(key).unwrap().get_mut("stern").unwrap().get_mut("mul").unwrap().insert((f1, f2));
                        }
                        if let Some(inv) = prod.recip() {
                            if inv.is_integer() {
                                gebr_rat_all_combis.get_mut(key).unwrap().get_mut("gleichf").unwrap().get_mut("mul").unwrap().insert((f1, f2));
                            }
                        }
                    }
                    if let Some(div) = f1.div(f2) {
                        if div.is_integer() {
                            gebr_rat_all_combis.get_mut(key).unwrap().get_mut("stern").unwrap().get_mut("div").unwrap().insert((f1, f2));
                        }
                        if let Some(inv_div) = div.recip() {
                            if inv_div.is_integer() {
                                gebr_rat_all_combis.get_mut(key).unwrap().get_mut("gleichf").unwrap().get_mut("div").unwrap().insert((f1, f2));
                            }
                        }
                    }
                }
            }
        }
        let mut alle: BTreeMap<String, BTreeMap<String, BTreeMap<String, BTreeMap<usize, Vec<(PyFrac, PyFrac)>>>>> = BTreeMap::new();
        for key in ["UniUni", "UniGal", "GalUni", "GalGal"] {
            let mut poly_map = BTreeMap::new();
            for poly in ["stern", "gleichf"] {
                let gleichf = poly == "gleichf";
                let set_mul = gebr_rat_all_combis[key][poly]["mul"].clone();
                let mut md = BTreeMap::new();
                let (fr1, fr2) = match key {
                    "UniUni" => (&brueche_uni, &brueche_uni),
                    "UniGal" => (&brueche_uni, &brueche_gal),
                    "GalUni" => (&brueche_gal, &brueche_uni),
                    _ => (&brueche_gal, &brueche_gal),
                };
                md.insert("mul".to_string(), self.combine_dicts_pairs_py(
                    self.convert_set_of_paaren_to_dict_mul_py(&set_mul, gleichf, limit),
                    self.convert_fractions_to_dict_mul_py(fr1, fr2, gleichf, limit),
                ));
                md.insert("div".to_string(), BTreeMap::new());
                poly_map.insert(poly.to_string(), md);
            }
            alle.insert(key.to_string(), poly_map);
        }
        alle
    }

    fn spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie_py(
        &self,
        koord: PyFrac,
        n_and_invers_spalten: (usize, usize),
        gebr_table: &Vec<Vec<String>>,
        is_not_universe: bool,
    ) -> Option<String> {
        let is_universe = !is_not_universe;
        if koord.denominator == 0 || koord.numerator == 0 {
            return Some(String::new());
        } else if koord.denominator > 100 || koord.numerator > 100 {
            return None;
        } else if koord.numerator == 1 {
            let idx = koord.denominator as usize;
            let base = self.zellenwert_py(idx, n_and_invers_spalten.1);
            if base.trim().len() > 3 {
                if is_universe {
                    let extra = self.zellenwert_py(idx, 201);
                    let sep = if extra.trim().len() > 2 {
                        if self.outType == "html" { "<br>" } else { "; " }
                    } else { "" };
                    return Some(format!("{} (1/{}){}{}", base, koord.denominator, sep, extra));
                }
                return Some(base);
            }
            return Some(String::new());
        } else if koord.denominator == 1 {
            let idx = koord.numerator as usize;
            let base = self.zellenwert_py(idx, n_and_invers_spalten.0);
            if base.trim().len() > 3 {
                if is_universe {
                    let extra = self.zellenwert_py(idx, 198);
                    let sep = if extra.trim().len() > 2 {
                        if self.outType == "html" { "<br>" } else { "; " }
                    } else { "" };
                    return Some(format!("{} ({}){}{}", base, koord.numerator, sep, extra));
                }
                return Some(base);
            }
            return Some(String::new());
        } else {
            let r = (koord.numerator - 1) as usize;
            let c = (koord.denominator - 1) as usize;
            return gebr_table.get(r).and_then(|row| row.get(c)).cloned().or(Some(String::new()));
        }
    }

    fn generated2_kombi_pair_text_py(
        &self,
        pair: (i64, i64),
        kombi_idx: usize,
        motivation: &Vec<String>,
        transzendentalien: &Vec<String>,
    ) -> Option<String> {
        let a = pair.0 as usize;
        let b = pair.1 as usize;
        let read = |values: &Vec<String>, idx: usize| -> String {
            values.get(idx).cloned().unwrap_or_default()
        };
        let kombi_a = (
            (read(motivation, a), read(motivation, a)),
            (read(motivation, a), read(transzendentalien, a)),
            (read(transzendentalien, a), read(motivation, a)),
            (read(transzendentalien, a), read(transzendentalien, a)),
        );
        let kombi_b = (
            (read(motivation, b), read(motivation, b)),
            (read(motivation, b), read(transzendentalien, b)),
            (read(transzendentalien, b), read(motivation, b)),
            (read(transzendentalien, b), read(transzendentalien, b)),
        );
        let (lhs_raw, rhs_raw) = match kombi_idx {
            0 => (&kombi_a.0.0, &kombi_b.0.1),
            1 => (&kombi_a.1.0, &kombi_b.1.1),
            2 => (&kombi_a.2.0, &kombi_b.2.1),
            3 => (&kombi_a.3.0, &kombi_b.3.1),
            _ => return None,
        };
        let lhs_trimmed = lhs_raw.trim();
        let rhs_trimmed = rhs_raw.trim();
        let lhs = if lhs_trimmed.len() > 3 { lhs_trimmed } else { "..." };
        let rhs = if rhs_trimmed.len() > 3 { rhs_trimmed } else { "..." };
        Some(format!("({}) * ({})", lhs, rhs))
    }

    fn meta_prefixes_py(&self, metavariable: i64) -> (&'static str, &'static str) {
        match metavariable {
            2 => ("Meta", "Konkretes"),
            3 => ("Theorie", "Praxis"),
            4 => ("Planung", "Umsetzung"),
            5 => ("Anlass", "Wirkung"),
            6 => ("Kraft-Gebung", "Verstärkung"),
            7 => ("Beherrschung", "Richtung"),
            _ => ("Meta", "Konkretes"),
        }
    }

    fn meta_or_what_labels_py(&self, metavariable: i64) -> ((&'static str, &'static str), (&'static str, &'static str)) {
        match metavariable {
            2 => (("Meta-Thema: ", "Konkretes: "), ("Meta-", "Konkret-")),
            3 => (("Theorie-Thema: ", "Praxis: "), ("Theorie-", "Praxis-")),
            4 => (("Planungs-Thema: ", "Umsetzungs-Thema: "), ("Planung-", "Umsetzung-")),
            5 => (("Anlass-Thema: ", "Wirkungs-Thema: "), ("Anlass-", "wirkung-")),
            6 => (("Kraft-Gebung: ", "Verstärkungs-Thema: "), ("Kraft-geben-", "Verstärkung-")),
            7 => (("Beherrschung: ", "Richtung-Thema: "), ("beherrschend-", "Richtung-")),
            _ => (("Meta-Thema: ", "Konkretes: "), ("Meta-", "Konkret-")),
        }
    }

    fn meta_heading_py(&self, metavariable: i64, bothRows: i64, ifInvers: usize) -> String {
        let grund = match (metavariable, bothRows) {
            (2, 0) => "Meta",
            (2, 1) => "Konkretes",
            (3, 0) => "Theorie",
            (3, 1) => "Praxis",
            (4, 0) => "Management",
            (4, 1) => "verändernd",
            (5, 0) => "ganzheitlich",
            (5, 1) => "darüber hinaus gehend",
            (6, 0) => "Verwertung, Unternehmung, Geschäft",
            (6, 1) => "wertvoll",
            (7, 0) => "regieren, beherrschen",
            (7, 1) => "Richtung",
            (_, 0) => "Meta",
            _ => "Konkretes",
        };
        if ifInvers == 1 {
            format!("{} für 1/n statt n", grund)
        } else {
            format!("{} für n", grund)
        }
    }

    fn meta_make_vorwort_py(&self, wiederholungen: usize, vorworte2: (&str, &str), less1ormore2: usize) -> String {
        let basis = if less1ormore2 <= 1 { vorworte2.0 } else { vorworte2.1 };
        if wiederholungen > 1 {
            basis.repeat(wiederholungen)
        } else {
            basis.to_string()
        }
    }

    fn pyfrac_to_f64_py(&self, frac: PyFrac) -> f64 {
        frac.numerator as f64 / frac.denominator as f64
    }

    fn meta_switching_py(
        &self,
        newCol: usize,
        moreAndLess: (Option<i64>, Option<PyFrac>),
        metavariable: i64,
        ifInvers: usize,
        transzendentalienSpalten: (usize, usize),
        gebr_seen: &mut BTreeSet<(i64, i64)>,
    ) -> (usize, (Option<i64>, Option<PyFrac>)) {
        let (new_col2, _spalten_wahl) = if newCol == transzendentalienSpalten.1 {
            (transzendentalienSpalten.0, 0usize)
        } else {
            (transzendentalienSpalten.1, 1usize)
        };

        let a = moreAndLess.0.and_then(|value| {
            let mulresult = value.saturating_mul(metavariable);
            if mulresult >= 0 && (mulresult as usize) < self.relitable.len() {
                Some(mulresult)
            } else {
                None
            }
        });

        let mut b: Option<PyFrac> = None;
        if let Some(current_raw) = moreAndLess.1 {
            let current_f = self.pyfrac_to_f64_py(current_raw);
            if current_f < 100.0 && current_f > 0.01 {
                let mut current = current_raw;
                if new_col2 == if ifInvers == 0 { transzendentalienSpalten.0 } else { transzendentalienSpalten.1 } && current.denominator == 1 {
                    if let Some(inv) = PyFrac::new(1, current.numerator) {
                        current = inv;
                    }
                }
                b = if self.spalteMetaKonkretAbstrakt_isGanzZahlig_py(self.pyfrac_to_f64_py(current), false) {
                    if current.denominator == 1 {
                        PyFrac::new(metavariable, current.numerator)
                    } else {
                        None
                    }
                } else {
                    current.recip().and_then(|v| v.div(PyFrac::new(metavariable, 1).unwrap()))
                };
            }
        }

        if let Some(fr) = b {
            let key = (fr.numerator, fr.denominator);
            if gebr_seen.contains(&key) {
                b = None;
            } else {
                gebr_seen.insert(key);
            }
        }

        (new_col2, (a, b))
    }

    fn meta_collect_cell_text_py(
        &self,
        bothRows: i64,
        i: usize,
        ifInvers: usize,
        metavariable: i64,
        transzendentalienSpalten: (usize, usize),
        gebr_univ_table: &Vec<Vec<String>>,
    ) -> String {
        let mut gebr_seen: BTreeSet<(i64, i64)> = BTreeSet::new();
        let mut moreAndLess: (Option<i64>, Option<PyFrac>) = (Some(i as i64), PyFrac::new(i as i64, 1));
        let mut neue2KoordNeue2Vorwoerter: Vec<((Option<i64>, Option<PyFrac>), usize, String, String)> = vec![];
        let mut newCol = transzendentalienSpalten.0;
        let metaOrWhat = self.meta_or_what_labels_py(metavariable);

        while !(moreAndLess.0.is_none() && moreAndLess.1.is_none()) {
            let switched = self.meta_switching_py(
                newCol,
                moreAndLess,
                metavariable,
                ifInvers,
                transzendentalienSpalten,
                &mut gebr_seen,
            );
            newCol = switched.0;
            moreAndLess = switched.1;

            let vorworte2 = if neue2KoordNeue2Vorwoerter.is_empty() { metaOrWhat.0 } else { metaOrWhat.1 };
            let vorwort1 = self.meta_make_vorwort_py(neue2KoordNeue2Vorwoerter.len() + 1, vorworte2, 1);
            let vorwort2 = self.meta_make_vorwort_py(neue2KoordNeue2Vorwoerter.len() + 1, vorworte2, 2);
            neue2KoordNeue2Vorwoerter.push((moreAndLess, newCol, vorwort1, vorwort2));
        }

        let mut intoList: Vec<String> = vec![];
        let mut thema = String::new();

        for vier in neue2KoordNeue2Vorwoerter.iter().take(neue2KoordNeue2Vorwoerter.len().saturating_sub(1)) {
            let ((mehr, weniger), col, vor1, vor2) = vier;
            if bothRows == 0 {
                if let Some(mehr_i64) = *mehr {
                    let text = self.zellenwert_py(mehr_i64 as usize, *col);
                    if text.trim().len() > 3 {
                        let reciprocal = if *col != if ifInvers == 0 { transzendentalienSpalten.0 } else { transzendentalienSpalten.1 } && weniger.map(|v| !(v.denominator == 1 && v.numerator == 1)).unwrap_or(false) {
                            "1/"
                        } else {
                            ""
                        };
                        let item = format!("{}{}{} ({})", vor1, thema, text, format!("{}{}", reciprocal, mehr_i64));
                        if self.outType == "html" {
                            intoList.push(format!("<li>{}</li>", item));
                        } else if self.outType == "bbcode" {
                            intoList.push(format!("[*]{}", item));
                        } else {
                            intoList.push(item);
                        }
                    }
                }
            } else if let Some(weniger_frac) = *weniger {
                if weniger_frac.denominator == 1 {
                    let idx = weniger_frac.numerator as usize;
                    let text = self.zellenwert_py(idx, *col);
                    if text.trim().len() > 3 {
                        let reciprocal = if *col != if ifInvers == 0 { transzendentalienSpalten.0 } else { transzendentalienSpalten.1 } && !(weniger_frac.denominator == 1 && weniger_frac.numerator == 1) {
                            "1/"
                        } else {
                            ""
                        };
                        let item = format!("{}{}{} ({})", vor2, thema, text, format!("{}{}", reciprocal, weniger_frac.numerator));
                        if self.outType == "html" {
                            intoList.push(format!("<li>{}</li>", item));
                        } else if self.outType == "bbcode" {
                            intoList.push(format!("[*]{}", item));
                        } else {
                            intoList.push(item);
                        }
                    }
                } else {
                    let gebrStrukWort = self.spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie_py(
                        weniger_frac,
                        (5usize, 131usize),
                        gebr_univ_table,
                        false,
                    );
                    if let Some(gebr) = gebrStrukWort {
                        if gebr.trim().len() > 3 {
                            let frac_txt = if weniger_frac.denominator > 1 {
                                format!("{}/{}", weniger_frac.numerator, weniger_frac.denominator)
                            } else {
                                weniger_frac.numerator.to_string()
                            };
                            let item = format!("{}{}{}({})", vor2, thema, gebr, frac_txt);
                            if self.outType == "html" {
                                intoList.push(format!("<li>{}</li>", item));
                            } else if self.outType == "bbcode" {
                                intoList.push(format!("[*]{}", item));
                            } else {
                                intoList.push(item);
                            }
                        }
                    }
                }
            }
            thema = "Thema: ".to_string();
        }

        if self.outType == "html" {
            format!("<ul>{}</ul>", intoList.join(""))
        } else if self.outType == "bbcode" {
            format!("[list]{}[/list]", intoList.join(""))
        } else {
            intoList.join(" | ")
        }
    }

    fn generierte_spalte_meta_name_py(&self, spaltenNummer: i64) -> String {
        if let Some(meta) = self.dataDict.get(0) {
            if let Some(eintrag) = meta.get(&spaltenNummer.to_string()) {
                let mut teile: Vec<String> = vec![];
                for gruppe in eintrag {
                    for paar in gruppe {
                        if !paar.1.is_empty() {
                            teile.push(paar.1.clone());
                        } else if !paar.0.is_empty() {
                            teile.push(paar.0.clone());
                        }
                    }
                }
                if !teile.is_empty() {
                    return teile.join(" / ");
                }
            }
        }
        format!("Generator-Spalte {}", spaltenNummer)
    }

    fn moonNumber(&self, num: i64) -> (Vec<i64>, Vec<i64>) {
        let mut results: Vec<i64> = Vec::new();
        let mut exponent: Vec<i64> = Vec::new();
        if num <= 1 {
            return (results, exponent);
        }
        for i in 2..num {
            let oneResult: f64 = (num as f64).powf(1.0 / i as f64);
            if (oneResult.round() * 100000.0 - (oneResult * 100000.0).round()).abs() < 1e-9 {
                results.push(oneResult.round() as i64);
                exponent.push(i - 2);
            }
        }
        (results, exponent)
    }

    fn primFak(&self, n: i64) -> Vec<i64> {
        let mut faktoren: Vec<i64> = Vec::new();
        let mut z = n;
        while z > 1 {
            let mut i = 2i64;
            let mut gefunden = false;
            let mut p = z;
            while i * i <= n && !gefunden {
                if z % i == 0 {
                    gefunden = true;
                    p = i;
                } else {
                    i += 1;
                }
            }
            if !gefunden {
                p = z;
            }
            faktoren.push(p);
            z /= p;
        }
        faktoren
    }

    fn divisorGenerator(&self, n: i64) -> Vec<i64> {
        let mut divisors = vec![];
        if n <= 0 {
            return divisors;
        }
        let mut i = 1i64;
        while i * i <= n {
            if n % i == 0 {
                divisors.push(i);
                if i * i != n {
                    divisors.push(n / i);
                }
            }
            i += 1;
        }
        divisors.sort_unstable();
        divisors
    }

    fn primRepeat(&self, n: Vec<i64>) -> Vec<(i64, i64)> {
        let mut counts: BTreeMap<i64, i64> = BTreeMap::new();
        for v in n {
            *counts.entry(v).or_insert(0) += 1;
        }
        counts.into_iter().collect()
    }

    fn primCreativity_exact_py(&self, num: i64) -> i64 {
        if num == 0 {
            return 0;
        }
        let fak = self.primRepeat(self.primFak(num));
        if fak.len() == 1 && fak[0].1 == 1 {
            return 1;
        }
        if fak.len() == 1 {
            return 3;
        }
        if fak.is_empty() {
            return 0;
        }
        let mut schnittmenge: Option<BTreeSet<i64>> = None;
        for (_, primAmount) in fak.iter() {
            let divisors: BTreeSet<i64> = self.divisorGenerator(*primAmount).into_iter().filter(|v| *v != 1).collect();
            if divisors.is_empty() {
                schnittmenge = None;
                break;
            }
            schnittmenge = Some(match schnittmenge {
                Some(ref old) => old.intersection(&divisors).cloned().collect(),
                None => divisors,
            });
        }
        match schnittmenge {
            Some(s) if !s.is_empty() => 3,
            Some(_) => 2,
            None => 2,
        }
    }

    fn couldBePrimeNumberPrimzahlkreuz(&self, num: i64) -> bool {
        matches!(num.rem_euclid(24), 1 | 5 | 7 | 11 | 13 | 17 | 19 | 23)
    }

    fn couldBePrimeNumberPrimzahlkreuz_fuer_innen(&self, num: i64) -> bool {
        matches!(num.rem_euclid(24), 5 | 11 | 17 | 23)
    }

    fn couldBePrimeNumberPrimzahlkreuz_fuer_aussen(&self, num: i64) -> bool {
        matches!(num.rem_euclid(24), 1 | 7 | 13 | 19)
    }

    fn primMultiple_pairs_py(&self, num: i64) -> Vec<(i64, i64)> {
        let mut out: Vec<(i64, i64)> = vec![];
        if num <= 0 {
            return out;
        }
        out.push((1, num));
        if num == 1 {
            return out;
        }
        let mut a = 2i64;
        while a * a <= num {
            if num % a == 0 {
                let b = num / a;
                let pair = if a <= b { (a, b) } else { (b, a) };
                if !out.contains(&pair) {
                    out.push(pair);
                }
            }
            a += 1;
        }
        out
    }

    fn gleichheitFreiheitVergleich(&self, zahl: i64) -> String {
        let mut ausgabeStringList: Vec<String> = vec![];
        if zahl % 4 == 0 {
            ausgabeStringList.push("Dominieren, Unterordnen".to_string());
        }
        if zahl % 4 == 1 {
            ausgabeStringList.push("Freiheit".to_string());
        }
        if zahl % 4 == 3 {
            ausgabeStringList.push("Einschränkung der Freiheit".to_string());
        }
        if zahl % 4 == 2 {
            if (zahl - 2) % 8 == 0 {
                ausgabeStringList.push("Gleichheit".to_string());
            }
            if (zahl - 6) % 16 == 0 {
                ausgabeStringList.push("den anderen überbieten wollen".to_string());
            }
            if (zahl - 14) % 16 == 0 {
                ausgabeStringList.push("den anderen unterbieten wollen".to_string());
            }
        }
        ausgabeStringList.join("; ")
    }

    fn geistEmotionEnergieMaterieTopologie(&self, zahl: i64) -> String {
        let prFa = self.primFak(zahl);
        let auss: Vec<bool> = prFa.iter().map(|a| self.couldBePrimeNumberPrimzahlkreuz_fuer_aussen(*a)).collect();
        let innen: Vec<bool> = prFa.iter().map(|a| self.couldBePrimeNumberPrimzahlkreuz_fuer_innen(*a)).collect();
        let zwei = prFa.iter().filter(|a| **a == 2).count() as i64;
        let gefuehl = auss.iter().any(|a| *a);
        let denken = innen.iter().any(|a| *a);
        let totalTopologie = zwei > 1 && gefuehl;
        let etwasTopologie = (zwei > 1 || (zwei > 0 && gefuehl)) && !totalTopologie;
        let totalMaterie = zwei > 4;
        let etwasMaterie = zwei == 4;
        let wenigMaterie = zwei == 3;
        let kaumMaterie = zwei == 2;
        let x = denken;
        let y = prFa.contains(&2);
        let z = prFa.contains(&3);
        let totalEnerge = x && y && z;
        let einermassenEnergie = ((x && y) || (y && z) || (y && z)) && !totalEnerge;
        let kaumEnergie = !einermassenEnergie && !totalEnerge && (x || y || z);
        let mut ausgabeStringList: Vec<String> = vec![];
        if denken { ausgabeStringList.push("eine Denkart".to_string()); }
        if gefuehl { ausgabeStringList.push("eine Gefühlsart".to_string()); }
        if totalMaterie { ausgabeStringList.push("total eine Art, etwas geistig zu erzeugen".to_string()); }
        if totalTopologie { ausgabeStringList.push("total eine Art zu erleben".to_string()); }
        if totalEnerge { ausgabeStringList.push("total eine Energie-Art".to_string()); }
        if etwasTopologie { ausgabeStringList.push("etwas eine Art zu erleben".to_string()); }
        if etwasMaterie { ausgabeStringList.push("etwas eine Art, etwas geistig zu erzeugen".to_string()); }
        if wenigMaterie { ausgabeStringList.push("wenig eine Art, etwas geistig zu erzeugen".to_string()); }
        if einermassenEnergie { ausgabeStringList.push("einigermaßen eine Energie-Art".to_string()); }
        if kaumEnergie { ausgabeStringList.push("kaum eine Energie-Art".to_string()); }
        if kaumMaterie { ausgabeStringList.push("kaum eine Art, etwas geistig zu erzeugen".to_string()); }
        ausgabeStringList.join("; ")
    }

    pub fn concatLovePolygon(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        if !rowsAsNumbers.contains(&9) { return; }
        let mut zeilenInhalte: Vec<String> = vec![];
        for i in 0..self.relitable.len() {
            let a = self.zellenwert_py(i, 8);
            if !a.trim().is_empty() {
                zeilenInhalte.push(format!("{} der eigenen Strukturgröße ({}) auf dich bei gleichförmigen Polygonen", a, self.zellenwert_py(i, 4)));
            } else {
                zeilenInhalte.push(String::new());
            }
        }
        let spalte = self.fuege_spalte_hinzu_py(zeilenInhalte, &self.generierte_spalte_meta_name_py(9));
        self.set_generated_spalten_tags_exact_py(spalte, &[ST::sternPolygon, ST::galaxie, ST::gleichfoermigesPolygon]);
        Self::push_unique_i64_py(rowsAsNumbers, spalte);
    }

    pub fn concatGleichheitFreiheitDominieren(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        if !rowsAsNumbers.contains(&132) { return; }
        let mut zeilenInhalte: Vec<String> = vec![];
        let row_end = self.generator_row_end_py();
        for i in 0..=row_end {
            if i == 0 {
                zeilenInhalte.push("Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert".to_string());
            } else {
                zeilenInhalte.push(self.gleichheitFreiheitVergleich(i as i64));
            }
        }
        let spalte = self.fuege_spalte_hinzu_py(zeilenInhalte, &self.generierte_spalte_meta_name_py(132));
        self.set_generated_spalten_tags_exact_py(spalte, &[ST::sternPolygon, ST::universum]);
        Self::push_unique_i64_py(rowsAsNumbers, spalte);
    }

    pub fn concatGeistEmotionEnergieMaterieTopologie(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        if !rowsAsNumbers.contains(&242) { return; }
        let mut zeilenInhalte: Vec<String> = vec![];
        let row_end = self.generator_row_end_py();
        for i in 0..=row_end {
            if i == 0 {
                zeilenInhalte.push("Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art".to_string());
            } else {
                zeilenInhalte.push(self.geistEmotionEnergieMaterieTopologie(i as i64));
            }
        }
        let spalte = self.fuege_spalte_hinzu_py(zeilenInhalte, &self.generierte_spalte_meta_name_py(242));
        self.set_generated_spalten_tags_exact_py(spalte, &[ST::sternPolygon, ST::universum]);
        Self::push_unique_i64_py(rowsAsNumbers, spalte);
    }

    pub fn concatPrimCreativityType(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        if !rowsAsNumbers.contains(&64) { return; }
        let mut zeilenInhalte: Vec<String> = vec![];
        for i in 0..self.relitable.len() {
            let primCreativityType = self.primCreativity_exact_py(i as i64);
            let wert = if i == 0 {
                "Evolutions-Züchtungs-Kreativität".to_string()
            } else if primCreativityType == 0 {
                "0. Primzahl 1".to_string()
            } else if primCreativityType == 1 {
                "1. Primzahl und Sonnenzahl".to_string()
            } else if primCreativityType == 2 {
                "2. Sonnenzahl, aber keine Primzahl".to_string()
            } else {
                "3. Mondzahl".to_string()
            };
            zeilenInhalte.push(wert);
        }
        let spalte = self.fuege_spalte_hinzu_py(zeilenInhalte, &self.generierte_spalte_meta_name_py(64));
        self.set_generated_spalten_tags_exact_py(spalte, &[ST::sternPolygon, ST::galaxie]);
        Self::push_unique_i64_py(rowsAsNumbers, spalte);
    }

    pub fn concatMondExponzierenLogarithmusTyp(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        if !rowsAsNumbers.contains(&64) { return; }
        let hardcodedCouple = [(44usize, "Mond-Typ eines Sternpolygons"), (56usize, "Mond-Typ eines gleichförmigen Polygons")];
        for (rownum, rowheading) in hardcodedCouple {
            let mut zeilenInhalte: Vec<String> = vec![];
            let row_end = self.generator_row_end_py();
            for i in 0..=row_end {
                let moonTypesOf1Num = self.moonNumber(i as i64);
                if i == 0 {
                    zeilenInhalte.push(rowheading.to_string());
                } else {
                    let mut into: Vec<String> = vec![];
                    if moonTypesOf1Num.0.is_empty() {
                        into.push("kein Mond".to_string());
                    }
                    for k in 0..moonTypesOf1Num.0.len() {
                        if k > 0 {
                            into.push(" | ".to_string());
                        }
                        let basis = moonTypesOf1Num.0[k] as usize;
                        let exponentMinus2 = moonTypesOf1Num.1[k] as usize;
                        let insert = self.zellenwert_py(basis, rownum)
                            .replace("<SG>", &self.zellenwert_py(i, 4))
                            .replace("&lt;SG&gt;", &self.zellenwert_py(i, 4));
                        into.push(insert);
                        into.push(" - ".to_string());
                        into.push(self.zellenwert_py(exponentMinus2 + 2, 10));
                        into.push(" | ".to_string());
                        into.push(self.zellenwert_py(i, 10));
                        into.push(" + ".to_string());
                        into.push(self.zellenwert_py(i, 11));
                        into.push(", ".to_string());
                        into.push(self.zellenwert_py(exponentMinus2 + 2, 85));
                    }
                    zeilenInhalte.push(into.join(""));
                }
            }
            let spalte = self.fuege_spalte_hinzu_py(zeilenInhalte, &self.generierte_spalte_meta_name_py(64));
            if rownum == 44 {
                self.set_generated_spalten_tags_exact_py(spalte, &[ST::sternPolygon, ST::universum, ST::galaxie]);
            } else {
                self.set_generated_spalten_tags_exact_py(spalte, &[ST::gleichfoermigesPolygon, ST::universum, ST::galaxie]);
            }
            Self::push_unique_i64_py(rowsAsNumbers, spalte);
        }
    }

    pub fn concatVervielfacheZeile(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        let spaltenToVervielfache: Vec<usize> = rowsAsNumbers
            .iter()
            .copied()
            .filter(|n| *n == 90 || *n == 19)
            .map(|n| n as usize)
            .collect();
        let row_end = self.generator_row_end_py();
        for s in spaltenToVervielfache {
            let mut store: BTreeMap<(usize, usize), String> = BTreeMap::new();
            for z in 2..=row_end {
                let content = self.zellenwert_py(z, s);
                if !content.trim().is_empty() {
                    store.insert((z, s), content);
                }
            }
            let mut multis: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
            for ((ursprungsZeile, _), _content) in store.iter() {
                let mut vielfacher = 1usize;
                let mut ergebnis = vielfacher * *ursprungsZeile;
                multis.entry(ergebnis).or_default().push(*ursprungsZeile);
                while ergebnis < self.relitable.len() {
                    vielfacher += 1;
                    ergebnis = vielfacher * *ursprungsZeile;
                    multis.entry(ergebnis).or_default().push(*ursprungsZeile);
                }
            }
            for z in 2..=row_end {
                let mut xx = false;
                let mut teile: Vec<String> = if !self.zellenwert_py(z, s).trim().is_empty() {
                    if self.outType == "html" {
                        vec!["<li>".to_string(), self.zellenwert_py(z, s), "</li>".to_string()]
                    } else if self.outType == "bbcode" {
                        vec!["[*]".to_string(), self.zellenwert_py(z, s)]
                    } else {
                        vec![self.zellenwert_py(z, s), " | ".to_string()]
                    }
                } else {
                    vec![self.zellenwert_py(z, s)]
                };

                if let Some(ursZeilen) = multis.get(&z) {
                    for UrZeile in ursZeilen {
                        let basis = store.get(&(*UrZeile, s)).cloned().unwrap_or_default();
                        let aktuell = teile.join("");
                        if *UrZeile != z
                            && aktuell != basis
                            && format!("{} | ", aktuell) != basis
                            && format!("<li>{}</li>", aktuell) != basis
                            && format!("[*]{}", aktuell) != basis
                            && !basis.is_empty()
                        {
                            if self.outType == "html" {
                                teile.push("<li>".to_string());
                                teile.push(basis);
                                teile.push("</li>".to_string());
                            } else if self.outType == "bbcode" {
                                teile.push("[*]".to_string());
                                teile.push(basis);
                            } else {
                                xx = true;
                                teile.push(basis);
                                teile.push(" | ".to_string());
                            }
                        }
                    }
                }
                if self.outType == "html" {
                    teile.insert(0, "<ul>".to_string());
                    teile.push("</ul>".to_string());
                } else if self.outType == "bbcode" {
                    teile.insert(0, "[list]".to_string());
                    teile.push("[/list]".to_string());
                }
                let endwert = if xx && !teile.is_empty() {
                    let mut x = teile.join("");
                    if x.ends_with(" | ") {
                        x.truncate(x.len() - 3);
                    }
                    x
                } else {
                    teile.join("")
                };
                self.setze_zellenwert_py(z, s, endwert);
            }
        }
    }

    pub fn spalteFuerGegenInnenAussenSeitlichPrim(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        if self.boolAndTupleSet1Options_exact_py().is_empty() {
            return;
        }
        fn prim_answer(oldPrimAmounts: i64, primAmounts: i64, i: i64) -> String {
            if i > 3 {
                if primAmounts != oldPrimAmounts {
                    if primAmounts % 2 == 0 {
                        "für innen".to_string()
                    } else {
                        "für außen".to_string()
                    }
                } else {
                    String::new()
                }
            } else if i == 2 {
                "für seitlich und gegen Schwächlinge innen".to_string()
            } else if i == 3 {
                "gegen seitlich und für Schwächlinge innen".to_string()
            } else if i == 1 {
                "für außen".to_string()
            } else {
                String::new()
            }
        }

        let extraSpalten: Vec<Option<usize>> = self.boolAndTupleSet1Options_exact_py();
        let mut vergangenheit: Vec<String> = vec![];
        for kk in extraSpalten {
            let mut zeilenInhalte: Vec<String> = vec![];
            let mut primAmounts = 0i64;
            let mut lastPrimAnswers: BTreeMap<i64, String> = BTreeMap::new();
            let row_end = self.generator_row_end_py();
            for i in 0..=row_end {
                let mut into = if i != 0 {
                    vec![String::new()]
                } else {
                    vec!["Primzahlwirkung (7, Richtung) ".to_string(), match kk { Some(k) => format!("{}", self.zellenwert_py(0, k)), None => "Richtung-Richtung".to_string() }]
                };
                let oldPrimAmounts = primAmounts;
                if self.couldBePrimeNumberPrimzahlkreuz(i as i64) {
                    primAmounts += 1;
                }
                if self.primCreativity_exact_py(i as i64) == 1 {
                    into = vec![prim_answer(oldPrimAmounts, primAmounts, i as i64)];
                    lastPrimAnswers.insert(i as i64, into.join(""));
                } else if i > 1 {
                    for couple in self.primRepeat(self.primFak(i as i64)) {
                        let basisantwort = lastPrimAnswers.get(&couple.0).cloned().unwrap_or_default();
                        if couple.1 == 1 {
                            into.push(basisantwort);
                            into.push(" + ".to_string());
                        } else if let Some(sp) = kk {
                            into.push(self.zellenwert_py(couple.1 as usize, sp));
                            into.push(" * ".to_string());
                            into.push(basisantwort);
                            into.push(" + ".to_string());
                        } else {
                            into.push("[".to_string());
                            into.push(vergangenheit.get(couple.1 as usize).cloned().unwrap_or_default());
                            into.push("] * letztendlich: ".to_string());
                            into.push(basisantwort);
                            into.push(" + ".to_string());
                        }
                    }
                    if into.last().map(|s| s == " + ").unwrap_or(false) {
                        into.pop();
                    }
                } else if i == 1 {
                    into = vec![prim_answer(oldPrimAmounts, primAmounts, 1)];
                }
                let joined = into.join("");
                if kk.is_none() {
                    vergangenheit.push(joined.clone());
                }
                zeilenInhalte.push(joined);
            }
            let spalte = self.fuege_spalte_hinzu_py(zeilenInhalte, "Primzahlwirkung (7, Richtung)");
            self.set_generated_spalten_tags_exact_py(spalte, &[ST::sternPolygon, ST::universum]);
            Self::push_unique_i64_py(rowsAsNumbers, spalte);
        }
    }

    fn concat_csv_name_py(&self, concatTable: i64) -> Option<&'static str> {
        match concatTable {
            1 => Some("primenumbers.csv"),
            2 | 3 => Some("gebrochen-rational-galaxie.csv"),
            4 | 5 => Some("gebrochen-rational-universum.csv"),
            6 | 7 => Some("gebrochen-rational-emotionen.csv"),
            8 | 9 => Some("gebrochen-rational-strukturgroesse.csv"),
            _ => None,
        }
    }

    fn transpose_py(&self, matrix: Vec<Vec<String>>) -> Vec<Vec<String>> {
        if matrix.is_empty() { return matrix; }
        let max_cols = matrix.iter().map(|row| row.len()).max().unwrap_or(0);
        let mut t = vec![vec![String::new(); matrix.len()]; max_cols];
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                t[x][y] = matrix[y][x].clone();
            }
        }
        t
    }

    fn readConcatCsv_ChangeTableToAddToTable(&self, concatTable: i64, mut tableToAdd: Vec<Vec<String>>) -> Vec<Vec<String>> {
        if matches!(concatTable, 3 | 5 | 7 | 9) {
            tableToAdd = self.transpose_py(tableToAdd);
        }
        if (2..=9).contains(&concatTable) && !tableToAdd.is_empty() {
            let prefix = match concatTable {
                2 | 3 => "Galaxie",
                4 | 5 => "Universum",
                6 | 7 => "Emotion",
                8 | 9 => "Strukturgroesse",
                _ => "Fehler",
            };
            let nOderEinsDurchN = if matches!(concatTable, 2 | 4 | 6 | 8) { "n/" } else { "" };
            let einsDurchNOderN = if matches!(concatTable, 3 | 5 | 7 | 9) { "/n" } else { "" };
            let mut kopf = vec![];
            for n in 0..tableToAdd[0].len() {
                kopf.push(format!("{}{}{} {}", nOderEinsDurchN, n + 1, einsDurchNOderN, prefix).trim().to_string());
            }
            tableToAdd.insert(0, kopf);
        }
        tableToAdd
    }

    fn readConcatCsv_set_generated_metadata_exact_py(
        &mut self,
        concatTable: i64,
        heading: &str,
        u: usize,
        spalte: i64,
    ) {
        if concatTable == 1 {
            let groups = vec![Self::pairstr_group_exact_py("Multiplikationen", "Nicht_generiert")];
            self.set_generated_spalten_parameter_exact_py(spalte, groups);
            self.generatedSpaltenParameter.push("Multiplikationen Nicht generiert".to_string());
            return;
        }
        let rangeToDataDict: std::collections::BTreeMap<i64, i64> =
            [(2, 6), (3, 6), (4, 5), (5, 5), (6, 9), (7, 9), (8, 10), (9, 10)]
                .into_iter()
                .collect();
        if let Some(dict_idx) = rangeToDataDict.get(&concatTable) {
            let key = (u as i64 + 2).to_string();
            let eintrag = self
                .dataDict
                .get(*dict_idx as usize)
                .and_then(|dict| dict.get(&key))
                .cloned();
            if let Some(eintrag) = eintrag {
                self.set_generated_spalten_parameter_exact_py(spalte, eintrag.clone());
                let mut teile: Vec<String> = Vec::new();
                for gruppe in &eintrag {
                    for paar in gruppe {
                        if !paar.0.is_empty() {
                            teile.push(paar.0.clone());
                        } else if !paar.1.is_empty() {
                            teile.push(paar.1.clone());
                        }
                    }
                }
                if !teile.is_empty() {
                    self.generatedSpaltenParameter.push(teile.join(" / "));
                    return;
                }
            }
        }
        self.generatedSpaltenParameter.push(heading.to_string());
    }

    pub fn readConcatCsv(&mut self, rowsAsNumbers: &mut Vec<i64>, concatTableSelection: Vec<i64>, concatTable: i64) -> Vec<i64> {
        let mut concatCSVspalten: Vec<i64> = vec![];
        if concatTableSelection.is_empty() { return concatCSVspalten; }
        let Some(csvFileName) = self.concat_csv_name_py(concatTable) else { return concatCSVspalten; };
        let Ok(mut tableToAdd) = self.load_csv_rows_semicolon_exact_path(csvFileName) else { return concatCSVspalten; };
        tableToAdd = self.readConcatCsv_ChangeTableToAddToTable(concatTable, tableToAdd);
        let show_concat1_non_generated = self.should_show_concat1_non_generated_column_py();
        if concatTable == 1 {
            let mut tableToAdd2 = vec![vec!["Primzahlvielfache, nicht generiert".to_string()]];
            for zeile in tableToAdd.into_iter().skip(1) {
                let mut items: Vec<String> = vec![];
                for zelle in zeile {
                    if zelle.trim().len() > 3 {
                        items.push(zelle);
                    }
                }
                tableToAdd2.push(vec![self.wrap_items_exact_py(&items, true)]);
            }
            tableToAdd = tableToAdd2;
        }

        let target_rows = std::cmp::max(self.relitable.len(), tableToAdd.len());
        while self.relitable.len() < target_rows {
            let width = self.relitable.first().map(|r| r.len()).unwrap_or(0);
            self.relitable.push(vec![String::new(); width]);
        }
        while tableToAdd.len() < target_rows {
            let width = tableToAdd.first().map(|r| r.len()).unwrap_or(0);
            tableToAdd.push(vec![String::new(); width]);
        }
        let maxlen = tableToAdd.iter().map(|r| r.len()).max().unwrap_or(0);
        for i in 0..target_rows {
            if tableToAdd[i].len() < maxlen { tableToAdd[i].resize(maxlen, String::new()); }
            let start = self.relitable[i].len() as i64;
            self.relitable[i].extend(tableToAdd[i].clone());
            if i == 0 {
                for u in 0..maxlen {
                    if ((u as i64 + 2).checked_sub(0).unwrap_or(0) != 0 && concatTableSelection.contains(&(u as i64 + 2)) && (2..=9).contains(&concatTable)) || (concatTable == 1 && show_concat1_non_generated) {
                        let selectedSpalten = start + u as i64 + if (2..=9).contains(&concatTable) { 1 } else { 0 };
                        Self::push_unique_i64_py(rowsAsNumbers, selectedSpalten);
                        concatCSVspalten.push(selectedSpalten);
                        let concat_tags = self.concat_table_generated_tags_exact_py(concatTable);
                        self.set_generated_spalten_tags_exact_py(selectedSpalten, &concat_tags);
                        let heading = tableToAdd.get(0).and_then(|row| row.get(u)).cloned().unwrap_or_default();
                        self.readConcatCsv_set_generated_metadata_exact_py(concatTable, &heading, u, selectedSpalten);
                    }
                }
            }
        }
        concatCSVspalten
    }

    fn getModaloperatorsPerLineCoordinates_py(&self, lineWeAreAt: usize) -> (usize, usize, usize) {
        let modalMainOperatorZeile = lineWeAreAt;
        let amountModaloperators = lineWeAreAt.saturating_sub(1);
        let modalOpElseOperatorsZeilenBegin = lineWeAreAt + 1;
        let modalOpElseOperatorsZeilenEnd = lineWeAreAt + amountModaloperators + 1;
        (modalMainOperatorZeile, modalOpElseOperatorsZeilenBegin, modalOpElseOperatorsZeilenEnd)
    }

    fn getModaloperatorsPerLineCells_py(&self, relitable: &Vec<Vec<String>>, lineWeAreAt: usize) -> Vec<String> {
        let coords = self.getModaloperatorsPerLineCoordinates_py(lineWeAreAt);
        let mut modaloperators: Vec<String> = vec![];
        if let Some(row) = relitable.get(coords.0) {
            if let Some(v) = row.get(97) { modaloperators.push(v.clone()); }
            if let Some(v) = row.get(98) { modaloperators.push(v.clone()); }
        }
        for coord in coords.1..coords.2 {
            if let Some(v) = relitable.get(coord).and_then(|row| row.get(42)) {
                modaloperators.push(v.clone());
            }
        }
        modaloperators
    }

    fn modal_text_by_distance_exact_py(&self, distanceFromLine: i64) -> &'static str {
        match distanceFromLine.abs() {
            2 => "mittelstark überdurchschnittlich: ",
            1 => "überdurchschnittlich: ",
            3 => "mittelleicht überdurchschnittlich: ",
            0 => "sehr: ",
            _ => "sehr leicht überdurchschnittlich: ",
        }
    }

    fn modal_replace_zuerst_zweites_py(&self, txt: String) -> String {
        txt.replace("intrinsisch", "zuerst").replace("extrinsisch", "als zweites")
    }

    pub fn concatModallogik(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        let conceptSelections = self.generated1_selections_exact_py();
        if conceptSelections.is_empty() {
            return;
        }
        let reliTableCopy = self.relitable.clone();
        let distances: [i64; 9] = [-4, -3, -2, -1, 0, 1, 2, 3, 4];

        #[derive(Clone, Default)]
        struct ModalEntryPy {
            i_origS: Vec<usize>,
            modalS: Vec<Vec<String>>,
            vervielfachter: Vec<usize>,
        }

        for selection in conceptSelections {
            let concept = (selection.left, selection.right);
            let concept0 = concept.0 as usize;
            let concept1 = concept.1 as usize;
            let mut into_items: Vec<Vec<String>> = vec![vec![]; reliTableCopy.len()];
            let mut cells: Vec<String> = vec![String::new(); reliTableCopy.len()];
            let mut einMalVorkommen: Vec<usize> = vec![];

            for (i, cols) in reliTableCopy.iter().enumerate() {
                if i == 0 {
                    cells[i] = format!("Generiert: {}", cols.get(concept0).cloned().unwrap_or_default());
                } else if cols.get(concept0).map(|s| !s.trim().is_empty()).unwrap_or(false) {
                    if !einMalVorkommen.contains(&i) {
                        einMalVorkommen.push(i);
                    }
                }
            }

            let mut vorkommenVielfacher: BTreeMap<usize, Vec<(usize, usize)>> = BTreeMap::new();
            for &einVorkommen in &einMalVorkommen {
                let mut vielfacher = 1usize;
                let mut ergebnis = vielfacher * einVorkommen;
                vorkommenVielfacher.entry(ergebnis).or_default().push((einVorkommen, vielfacher));
                while ergebnis < reliTableCopy.len() {
                    vielfacher += 1;
                    ergebnis = vielfacher * einVorkommen;
                    vorkommenVielfacher.entry(ergebnis).or_default().push((einVorkommen, vielfacher));
                }
            }

            let mut vorkommenVielfacher_B: BTreeMap<usize, BTreeMap<i64, ModalEntryPy>> = BTreeMap::new();
            let row_end = self.generator_row_end_py();
            for i in 1..=row_end {
                for &distanceFromLine in &distances {
                    let i_with_a_distance_i64 = i as i64 + distanceFromLine;
                    if i_with_a_distance_i64 < 0 {
                        continue;
                    }
                    let i_with_a_distance = i_with_a_distance_i64 as usize;
                    let Some(couples) = vorkommenVielfacher.get(&i_with_a_distance) else {
                        continue;
                    };
                    let mut modalOperatorEnEn: Vec<Vec<String>> = vec![];
                    let mut Orginal_i_mehrere: Vec<usize> = vec![];
                    let mut vervielFachter: Vec<usize> = vec![];
                    for &(vorkommen, vielfacher) in couples {
                        modalOperatorEnEn.push(self.getModaloperatorsPerLineCells_py(&reliTableCopy, vielfacher));
                        vervielFachter.push(vorkommen);
                        Orginal_i_mehrere.push(i_with_a_distance);
                    }
                    let entry = vorkommenVielfacher_B.entry(i).or_default().entry(distanceFromLine).or_default();
                    let mut new_i_origS = Orginal_i_mehrere;
                    new_i_origS.extend(entry.i_origS.clone());
                    let mut new_modalS = modalOperatorEnEn;
                    new_modalS.extend(entry.modalS.clone());
                    let mut new_vervielfachter = vervielFachter;
                    new_vervielfachter.extend(entry.vervielfachter.clone());
                    entry.i_origS = new_i_origS;
                    entry.modalS = new_modalS;
                    entry.vervielfachter = new_vervielfachter;
                }
            }

            for i in 1..=row_end {
                for &distanceFromLine in &distances {
                    let Some(entry_by_dist) = vorkommenVielfacher_B.get(&i).and_then(|m| m.get(&distanceFromLine)) else {
                        continue;
                    };
                    for (modalOperatoren, &vervielfachter) in entry_by_dist
                        .modalS
                        .iter()
                        .zip(entry_by_dist.vervielfachter.iter())
                    {
                        let intoItsContent = if distanceFromLine.abs() % 2 == 0 {
                            reliTableCopy
                                .get(vervielfachter)
                                .and_then(|r| r.get(concept0))
                                .cloned()
                                .unwrap_or_default()
                        } else {
                            reliTableCopy
                                .get(vervielfachter)
                                .and_then(|r| r.get(concept1))
                                .cloned()
                                .unwrap_or_default()
                        };
                        if intoItsContent.is_empty() || modalOperatoren.len() < 2 {
                            continue;
                        }
                        let basis_content = if reliTableCopy
                            .get(1)
                            .and_then(|r| r.get(97))
                            .map(|s| s == &modalOperatoren[0])
                            .unwrap_or(false)
                        {
                            intoItsContent.clone()
                        } else {
                            self.modal_replace_zuerst_zweites_py(intoItsContent.clone())
                        };
                        let mut item = format!(
                            "{}{} {} {}",
                            self.modal_text_by_distance_exact_py(distanceFromLine),
                            modalOperatoren[0],
                            basis_content,
                            modalOperatoren[1]
                        );
                        if distanceFromLine.abs() % 2 == 1 && modalOperatoren.len() > 2 {
                            item.push_str(", nicht: ");
                            item.push_str(&modalOperatoren[2..].join(", "));
                            item.push_str(" (das alles nicht): ");
                            let c0 = reliTableCopy
                                .get(vervielfachter)
                                .and_then(|r| r.get(concept0))
                                .cloned()
                                .unwrap_or_default();
                            item.push_str(&self.modal_replace_zuerst_zweites_py(c0));
                        }
                        into_items[i].push(item);
                    }
                }
                let conditionNvs1perN = matches!(concept.0, 62 | 63 | 358..=367 | 371..=374);
                let fill_ = if conditionNvs1perN {
                    reliTableCopy.get(i).and_then(|r| r.get(197)).cloned().unwrap_or_default()
                } else {
                    reliTableCopy.get(i).and_then(|r| r.get(4)).cloned().unwrap_or_default()
                };
                if !into_items[i].is_empty() {
                    into_items[i].push(format!(
                        "Alles nur bezogen auf die selbe Strukturgröße einer {}",
                        fill_
                    ));
                }
            }

            for i in 1..=self.generator_row_end_py() {
                cells[i] = self.wrap_items_exact_py(&into_items[i], false);
            }

            let meta_name = self.generator_pair_selection_meta_name_exact_py(&selection, concept.0);
            let spalte = self.fuege_spalte_hinzu_py(cells, &meta_name);
            self.set_generated_spalten_parameter_exact_py(
                spalte,
                self.generated1_parameter_groups_exact_py(&selection, concept.0),
            );
            let conditionNvs1perN = matches!(concept.0, 62 | 63 | 358..=367 | 371..=374);
            if conditionNvs1perN {
                self.set_generated_spalten_tags_exact_py(spalte, &[ST::gleichfoermigesPolygon, ST::galaxie]);
            } else {
                self.set_generated_spalten_tags_exact_py(spalte, &[ST::sternPolygon, ST::galaxie]);
            }
            Self::push_unique_i64_py(rowsAsNumbers, spalte);
        }
    }

    fn generated2_selections_exact_py(&self) -> Vec<Generated2Selection> {
        if !self.generated2Selections.is_empty() {
            return self.generated2Selections.clone();
        }
        self.generated2Codes
            .iter()
            .cloned()
            .map(|code| Generated2Selection {
                parameter_main_name: String::new(),
                parameter_name: self.generated2_code_heading_py(&code),
                code,
            })
            .collect()
    }

    fn normalize_generated2_display_main_name_py(&self, main_name: &str) -> String {
        match main_name.to_ascii_lowercase().as_str() {
            "primvielfache" => "generierte Multiplikationen".to_string(),
            _ => main_name.to_string(),
        }
    }

    fn normalize_generated2_display_parameter_name_py(&self, parameter_name: &str) -> String {
        parameter_name
            .replace("Motive_", "")
            .replace("Struktur_", "")
            .replace("_", " ")
    }

    fn generated2_selection_heading_exact_py(&self, selection: &Generated2Selection) -> String {
        let main_name = self.normalize_generated2_display_main_name_py(&selection.parameter_main_name);
        let parameter_name = self.normalize_generated2_display_parameter_name_py(&selection.parameter_name);
        if !main_name.is_empty() && !parameter_name.is_empty() {
            return format!("{} {}", main_name, parameter_name);
        }
        if !parameter_name.is_empty() {
            return parameter_name;
        }
        self.generated2_code_heading_py(&selection.code)
    }

    pub fn concat1RowPrimUniverse2(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        let generatedSelections: Vec<Generated2Selection> = self.generated2_selections_exact_py();
        if generatedSelections.is_empty() {
            return;
        }

        let relitableCopy = self.relitable.clone();
        let row_end = self.generator_row_end_py();
        let kombi_namen = ["Motiv -> Motiv", "Motiv -> Strukur", "Struktur -> Motiv", "Struktur -> Strukur"];
        let kombi_namen2 = ["GalGal", "GalUni", "UniGal", "UniUni"];
        let poly_namen = ["Sternpolygone", "gleichförmige Polygone"];
        let poly_keys = ["stern", "gleichf"];
        let koord2parameter = self.generated2_coord_parameters_exact_py(&generatedSelections);
        let koord2tag = self.generated2_coord_tags_exact_py(&generatedSelections);

        let mut requested_coords: BTreeSet<(usize, usize, bool)> = BTreeSet::new();
        let mut wants_primcsv = false;
        for selection in &generatedSelections {
            let code = selection.code.as_str();
            if code == "primzahlkreuzprocontra" {
                continue;
            }
            if code == "PrimCSV" {
                wants_primcsv = true;
                continue;
            }
            if let Some((poly_idx, _poly_name, kombis, is_gebr)) = self.generated2_exact_coords_py(code) {
                for kombi_idx in kombis {
                    requested_coords.insert((poly_idx, kombi_idx, is_gebr));
                }
            }
        }

        if wants_primcsv {
            if let Some(csv_name) = self.concat_csv_name_py(1) {
                if let Ok(mut tableToAdd) = self.load_csv_rows_semicolon_exact_path(csv_name) {
                    tableToAdd = self.readConcatCsv_ChangeTableToAddToTable(1, tableToAdd);
                    let mut into: Vec<String> = vec!["Primzahlvielfache, nicht generiert".to_string()];
                    for zeile in tableToAdd.into_iter().skip(1).take(row_end) {
                        let mut items: Vec<String> = vec![];
                        for zelle in zeile {
                            if zelle.trim().len() > 3 {
                                items.push(zelle);
                            }
                        }
                        into.push(self.wrap_items_exact_py(&items, true));
                    }
                    while into.len() <= row_end {
                        into.push(String::new());
                    }
                    let spalte = self.fuege_spalte_hinzu_py(into, "Primzahlvielfache, nicht generiert");
                    self.register_generated2_spalte_exact_py("PrimCSV", spalte);
                    Self::push_unique_i64_py(rowsAsNumbers, spalte);
                }
            }
        }

        let hard_coded_couple = [10usize, 42usize];
        let transzendentalien_nrezi = [5usize, 131usize];
        let mut motivation: [Vec<String>; 2] = [vec![], vec![]];
        let mut transzendentalien: [Vec<String>; 2] = [vec![], vec![]];
        for cols in &relitableCopy {
            for zwei in 0..=1usize {
                motivation[zwei].push(cols.get(hard_coded_couple[zwei]).cloned().unwrap_or_default());
                transzendentalien[zwei].push(cols.get(transzendentalien_nrezi[zwei]).cloned().unwrap_or_default());
            }
        }

        let alle_fraction_ergebnisse2 = self.find_all_brueche_and_their_combinations_py(row_end);
        let gal_or_uni_n_or_invers = [
            (hard_coded_couple, hard_coded_couple),
            (hard_coded_couple, transzendentalien_nrezi),
            (transzendentalien_nrezi, hard_coded_couple),
            (transzendentalien_nrezi, transzendentalien_nrezi),
        ];
        let uni_csv = self
            .csv_fraction_table_name_py(2)
            .and_then(|n| self.load_csv_rows_semicolon_exact_path(n).ok())
            .unwrap_or_default();
        let gal_csv = self
            .csv_fraction_table_name_py(3)
            .and_then(|n| self.load_csv_rows_semicolon_exact_path(n).ok())
            .unwrap_or_default();

        let mut kombis_all: [Vec<((String, String), (String, String), (String, String), (String, String))>; 2] = [vec![], vec![]];
        for zwei in 0..=1usize {
            for i in 0..self.relitable.len() {
                kombis_all[zwei].push((
                    (motivation[zwei][i].clone(), motivation[zwei][i].clone()),
                    (motivation[zwei][i].clone(), transzendentalien[zwei][i].clone()),
                    (transzendentalien[zwei][i].clone(), motivation[zwei][i].clone()),
                    (transzendentalien[zwei][i].clone(), transzendentalien[zwei][i].clone()),
                ));
            }
        }

        for brr in 0..=1usize {
            for zwei in 0..=1usize {
                for null_bis_drei in 0..=3usize {
                    let coord = (zwei, null_bis_drei, brr == 1);
                    if !requested_coords.contains(&coord) {
                        continue;
                    }
                    let ganz_oder_gebr = if brr == 0 { "" } else { ", mit Faktoren aus gebrochen-rationalen Zahlen" };
                    let heading = format!(
                        "generierte Multiplikationen {} {}{}",
                        poly_namen[zwei],
                        kombi_namen[null_bis_drei],
                        ganz_oder_gebr,
                    );
                    let mut into: Vec<String> = vec![heading.clone()];
                    for i in 1..=row_end {
                        let mut teile: Vec<String> = vec![];
                        if self.outType == "html" {
                            teile.push("<ul>".to_string());
                        } else if self.outType == "bbcode" {
                            teile.push("[list]".to_string());
                        }
                        if brr == 0 {
                            let mut multipless = self.primMultiple_pairs_py(i as i64);
                            multipless.sort();
                            for (k, multi) in multipless.iter().enumerate() {
                                if k > 0 && self.outType != "html" && self.outType != "bbcode" {
                                    teile.push(", außerdem: ".to_string());
                                }
                                let lhsrhs = match null_bis_drei {
    0 => (
        &kombis_all[zwei][multi.0 as usize].0.0,
        &kombis_all[zwei][multi.1 as usize].0.1,
    ),
    1 => (
        &kombis_all[zwei][multi.0 as usize].1.0,
        &kombis_all[zwei][multi.1 as usize].1.1,
    ),
    2 => (
        &kombis_all[zwei][multi.0 as usize].2.0,
        &kombis_all[zwei][multi.1 as usize].2.1,
    ),
    _ => (
        &kombis_all[zwei][multi.0 as usize].3.0,
        &kombis_all[zwei][multi.1 as usize].3.1,
    ),
};
                               let lhs = lhsrhs.0.trim();
                                let rhs = lhsrhs.1.trim();
                                let lhs_display = if lhs.len() > 3 { lhs.to_string() } else { "...".to_string() };
                                let rhs_display = if rhs.len() > 3 { rhs.to_string() } else { "...".to_string() };
                                if self.outType == "html" {
                                    teile.push(format!("<li>({}) * ({})</li>", lhs_display, rhs_display));
                                } else if self.outType == "bbcode" {
                                    teile.push(format!("[*]({}) * ({})", lhs_display, rhs_display));
                                } else {
                                    teile.push(format!("({}) * ({})", lhs_display, rhs_display));
                                }
                            }
                        } else {
                            let multipless = alle_fraction_ergebnisse2
                                .get(kombi_namen2[null_bis_drei])
                                .and_then(|a| a.get(poly_keys[zwei]))
                                .and_then(|a| a.get("mul"))
                                .and_then(|a| a.get(&i))
                                .cloned()
                                .unwrap_or_default();
                            for (k, multi) in multipless.iter().enumerate() {
                                let csv_von = if null_bis_drei >= 2 { &uni_csv } else { &gal_csv };
                                let csv_bis = if null_bis_drei == 1 || null_bis_drei == 3 { &uni_csv } else { &gal_csv };
                                let gal_or_uni_tuple = if zwei == 0 {
    gal_or_uni_n_or_invers[null_bis_drei].0
} else {
    gal_or_uni_n_or_invers[null_bis_drei].1
};

let von = self.spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie_py(
    multi.0,
    gal_or_uni_tuple.into(),
    csv_von,
    !(null_bis_drei >= 2),
);
let bis = self.spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie_py(
    multi.1,
    gal_or_uni_tuple.into(),
    csv_bis,
    !(null_bis_drei == 1 || null_bis_drei == 3),
);
                               let Some(von) = von else { continue; };
                                let Some(bis) = bis else { continue; };
                                let von = von.trim();
                                let bis = bis.trim();
                                if von.len() <= 3 || bis.len() <= 3 {
                                    continue;
                                }
                                if k > 0 && self.outType != "html" && self.outType != "bbcode" && !teile.is_empty() {
                                    teile.push("| außerdem: ".to_string());
                                }
                                let frac1 = format!("{}/{}", multi.0.numerator, multi.0.denominator);
                                let frac2 = format!("{}/{}", multi.1.numerator, multi.1.denominator);
                                let br = if self.outType == "html" && (von.len() > 30 || bis.len() > 30) { "<br>" } else { " " };
                                if self.outType == "html" {
                                    teile.push(format!("<li>\"{}\"{}({})*({}){}\"{}\"</li>", von, br, frac1, frac2, br, bis));
                                } else if self.outType == "bbcode" {
                                    teile.push(format!("[*]\"{}\" ({})*({}) \"{}\"", von, frac1, frac2, bis));
                                } else {
                                    teile.push(format!("\"{}\" ({})*({}) \"{}\"", von, frac1, frac2, bis));
                                }
                            }
                        }
                        if self.outType == "html" {
                            teile.push("</ul>".to_string());
                        } else if self.outType == "bbcode" {
                            teile.push("[/list]".to_string());
                        }
                        into.push(teile.join(""));
                    }
                    let spalte = self.fuege_spalte_hinzu_py(into, &heading);
                    self.register_generated2_coord_metadata_exact_py(coord, spalte, &koord2parameter, &koord2tag);
                    Self::push_unique_i64_py(rowsAsNumbers, spalte);
                }
            }
        }
    }

    pub fn concat1PrimzahlkreuzProContra(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        if !self.hat_generated2_code_py("primzahlkreuzprocontra") {
            return;
        }
        let row_end = self.generator_row_end_py();
        let dreli = self.relitable.clone();
        let mut pro_pro: BTreeMap<i64, i64> = BTreeMap::new();
        let mut contra_contra: BTreeMap<i64, i64> = BTreeMap::new();
        let mut pro_pro2: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
        let mut contra_contra2: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
        let mut list1: Vec<i64> = vec![];
        let mut list2: Vec<i64> = vec![];
        let mut keine_primzahl1 = true;
        let mut keine_primzahl2 = true;
        let mut weiter1a = 0usize;
        let mut weiter1b = 0usize;
        let mut weiter2a = 0usize;
        let mut weiter2b = 0usize;
        let mut col_main: Vec<String> = vec![];

        for num in 0..=row_end as i64 {
            contra_contra2.entry(num).or_default();
            pro_pro2.entry(num).or_default();
            let mut into: Vec<String> = if num == 0 {
                vec![self.generated2_code_heading_py("primzahlkreuzprocontra")]
            } else {
                vec![]
            };
            let mut into1: Vec<String> = vec![];
            let mut into2: Vec<String> = vec![];

            if self.primCreativity_exact_py(num) == 1 || num == 1 {
                if self.couldBePrimeNumberPrimzahlkreuz_fuer_innen(num) {
                    list1.push(num);
                    if num > 16 {
                        let maybe_gegen = if keine_primzahl1 {
                            list2.get(weiter1b + 1).copied().map(|v| {
                                weiter1b += 1;
                                v
                            })
                        } else {
                            list1.get(weiter1a).copied().map(|v| {
                                weiter1a += 1;
                                v
                            })
                        };
                        if let Some(gegen) = maybe_gegen {
                            contra_contra.insert(num, gegen);
                            Self::push_unique_i64_vec_py(contra_contra2.entry(num).or_default(), gegen);
                            into1.push(format!("gegen {}", gegen));
                        }
                    } else if matches!(num, 5 | 11) {
                        let gegen = 2;
                        contra_contra.insert(num, gegen);
                        Self::push_unique_i64_vec_py(contra_contra2.entry(num).or_default(), gegen);
                        into1.push(format!("gegen {}", gegen));
                    }
                    keine_primzahl1 = false;
                }
                if num == 2 {
                    let gegen = 1;
                    contra_contra.insert(num, gegen);
                    Self::push_unique_i64_vec_py(contra_contra2.entry(num).or_default(), gegen);
                    into1.push(format!("gegen {}", gegen));
                } else if num == 3 {
                    let pro = 1;
                    pro_pro.insert(num, pro);
                    Self::push_unique_i64_vec_py(pro_pro2.entry(num).or_default(), pro);
                    into2.push(format!("pro {}", pro));
                }
                if self.couldBePrimeNumberPrimzahlkreuz_fuer_aussen(num) {
                    list2.push(num);
                    if num > 16 {
                        let maybe_pro = if keine_primzahl2 {
                            list1.get(weiter2b + 1).copied().map(|v| {
                                weiter2b += 1;
                                v
                            })
                        } else {
                            list2.get(weiter2a).copied().map(|v| {
                                weiter2a += 1;
                                v
                            })
                        };
                        if let Some(pro) = maybe_pro {
                            pro_pro.insert(num, pro);
                            Self::push_unique_i64_vec_py(pro_pro2.entry(num).or_default(), pro);
                            into2.push(format!("pro {}", pro));
                        }
                    } else if matches!(num, 7 | 13) {
                        let pro = 3;
                        pro_pro.insert(num, pro);
                        Self::push_unique_i64_vec_py(pro_pro2.entry(num).or_default(), pro);
                        into2.push(format!("pro {}", pro));
                    }
                    keine_primzahl2 = false;
                }
            } else {
                if self.couldBePrimeNumberPrimzahlkreuz_fuer_innen(num) {
                    keine_primzahl1 = true;
                } else if self.couldBePrimeNumberPrimzahlkreuz_fuer_aussen(num) {
                    keine_primzahl2 = true;
                }
                let mut paare: Vec<(i64, i64)> = Vec::new();
let mut menge: Vec<(i64, i64)> = Vec::new();

for couple in self.primMultiple_pairs_py(num) {
    let mut pair = [couple.0, couple.1];
    pair.sort();
    let ordered = (pair[0], pair[1]);
    if !menge.contains(&ordered) {
        menge.push(ordered);
    }
}
paare.extend(menge);

for couple_a in paare {
    if couple_a.1 != 1 && couple_a.0 != 1 {
        let pair_variants: Vec<(i64, i64)> = if couple_a.0 == couple_a.1 {
            vec![couple_a]
        } else {
            vec![couple_a, (couple_a.1, couple_a.0)]
        };

        for couple in pair_variants {
            let positions: Vec<usize> = if couple.0 != couple.1 {
                vec![1usize, 0usize]
            } else {
                vec![1usize]
            };

            for first_or_second in positions {
                let chosen = if first_or_second == 1 { couple.1 } else { couple.0 };
                let other = if first_or_second == 1 { couple.0 } else { couple.1 };

                if self.couldBePrimeNumberPrimzahlkreuz_fuer_innen(chosen)
                    || couple.0 % 2 == 0
                    || couple.1 % 2 == 0
                {
                    if let Some(base) = contra_contra.get(&chosen).copied() {
                        let gegen3 = other * base;
                        contra_contra.insert(num, gegen3);
                        Self::push_unique_i64_vec_py(
                            contra_contra2.entry(num).or_default(),
                            gegen3,
                        );
                        into1.push(format!("gegen {}", gegen3));
                    }
                }

                if self.couldBePrimeNumberPrimzahlkreuz_fuer_aussen(couple.1)
                    || couple.1 % 3 == 0
                    || couple.0 % 3 == 0
                {
                    if let Some(base) = pro_pro.get(&chosen).copied() {
                        let pro3 = other * base;
                        pro_pro.insert(num, pro3);
                        Self::push_unique_i64_vec_py(
                            pro_pro2.entry(num).or_default(),
                            pro3,
                        );
                        into2.push(format!("pro {}", pro3));
                    }
                }
            }
        }
    }
}
}
                
            let text206 = dreli
                .get(num as usize)
                .and_then(|row| row.get(206))
                .cloned()
                .unwrap_or_default();
            if let Some((_, rhs)) = text206.split_once('|') {
                if !rhs.trim().is_empty() {
                    into.push(rhs.trim().to_string());
                }
            }
            into1 = Self::dedup_preserve_order_strings_py(into1);
            into2 = Self::dedup_preserve_order_strings_py(into2);
            col_main.push(self.concat1_main_cell_exact_py(num, into, into1, into2));
        }

        let mut reverse_pro: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
        let mut reverse_contra: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
        for (key, values) in &pro_pro2 {
            for value in values {
                Self::push_unique_i64_vec_py(reverse_pro.entry(*value).or_default(), *key);
            }
        }
        for (key, values) in &contra_contra2 {
            for value in values {
                Self::push_unique_i64_vec_py(reverse_contra.entry(*value).or_default(), *key);
            }
        }
        let mut col_reverse: Vec<String> = vec![];
        for num in 0..=row_end as i64 {
            let pro2: Vec<i64> = reverse_pro.get(&num).cloned().unwrap_or_default();
            let contra2: Vec<i64> = reverse_contra.get(&num).cloned().unwrap_or_default();
            col_reverse.push(self.concat1_reverse_cell_exact_py(num, pro2, contra2, &dreli));
        }

        let heading = self.primzahlkreuz_heading_exact_py().to_string();
        let spalte_main = self.fuege_spalte_hinzu_py(col_main, &heading);
        let spalte_reverse = self.fuege_spalte_hinzu_py(col_reverse, &heading);
        self.register_generated2_spalte_exact_py("primzahlkreuzprocontra", spalte_main);
        self.register_generated2_spalte_exact_py("primzahlkreuzprocontra", spalte_reverse);
        Self::push_unique_i64_py(rowsAsNumbers, spalte_main);
        Self::push_unique_i64_py(rowsAsNumbers, spalte_reverse);
    }

    pub fn spalteMetaKontretTheorieAbstrakt_etc_1(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        let geordneteSelections = self.metakonkret_selections_exact_py();
        for selection in geordneteSelections {
            self.spalteMetaKontretTheorieAbstrakt_etc(rowsAsNumbers, selection);
        }
    }

    pub fn spalteMetaKontretTheorieAbstrakt_etc(
        &mut self,
        rowsAsNumbers: &mut Vec<i64>,
        selection: GeneratorPairSelection,
    ) {
        let metavariable = selection.left;
        let lower1greater2both3 = if selection.right == 0 {
            1
        } else if selection.right == 1 {
            2
        } else {
            3
        };
        let metaOrWhat = self.meta_or_what_exact_py(metavariable);
        let bothRowsListe: Vec<i64> = if lower1greater2both3 == 3 {
            vec![0, 1]
        } else if lower1greater2both3 == 1 {
            vec![0]
        } else if lower1greater2both3 == 2 {
            vec![1]
        } else {
            vec![]
        };
        let struktAndInversSpalten = (5usize, 131usize);
        let gebr_univ_table = self
            .csv_fraction_table_name_py(4)
            .and_then(|name| self.load_csv_rows_semicolon_exact_path(name).ok())
            .unwrap_or_default();
        for (ifInvers, transzendentalienSpalten) in [
            struktAndInversSpalten,
            (struktAndInversSpalten.1, struktAndInversSpalten.0),
        ]
        .into_iter()
        .enumerate()
        {
            for bothRows in bothRowsListe.iter().copied() {
                let heading = self.meta_heading_py(metavariable, bothRows, ifInvers);
                let mut into: Vec<String> = vec![heading.clone()];
                if self.generator_row_end_py() >= 1 {
                    into.push(String::new());
                }
                for i in 2..=self.generator_row_end_py() {
                    let neue2KoordNeue2Vorwoerter = self.spalteMetaKonkret_vorwort_behandlung_exact_py(
                        metavariable,
                        ifInvers,
                        transzendentalienSpalten,
                        i as i64,
                        metaOrWhat,
                    );
                    let cell = self.spalteMetaKonkret_main_inserting_text_exact_py(
                        bothRows,
                        i,
                        ifInvers,
                        &neue2KoordNeue2Vorwoerter,
                        transzendentalienSpalten,
                        &gebr_univ_table,
                    );
                    into.push(cell);
                }
                let spalte = self.fuege_spalte_hinzu_py(into, &heading);
                let polygon_tag = if ifInvers == 0 {
                    ST::sternPolygon
                } else {
                    ST::gleichfoermigesPolygon
                };
                if bothRows == 0 {
                    self.set_generated_spalten_tags_exact_py(spalte, &[polygon_tag, ST::universum]);
                } else {
                    self.set_generated_spalten_tags_exact_py(
                        spalte,
                        &[polygon_tag, ST::universum, ST::gebrRat],
                    );
                }
                self.set_generated_spalten_parameter_exact_py(
                    spalte,
                    self.metakonkret_parameter_groups_exact_py(&selection),
                );
                Self::push_unique_i64_py(rowsAsNumbers, spalte);
            }
        }
    }

    pub fn createSpalteGestirn(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        if !rowsAsNumbers.contains(&64) { return; }
        let mut zeilenInhalte: Vec<String> = vec![];
        let row_end = self.generator_row_end_py();
        zeilenInhalte.push("Gestirn".to_string());
        let mut line1: Vec<String> = vec![];
        line1.push("Sonne (keine Potenzen)".to_string());
        zeilenInhalte.push(line1.join(""));
        for i in 2..=row_end {
            let mut line1: Vec<String> = vec![];
            if !self.moonNumber(i as i64).1.is_empty() {
                line1.push("Mond (Potenzen)".to_string());
            } else {
                line1.push("Sonne (keine Potenzen)".to_string());
            }
            if i % 2 == 0 {
                line1.push("Planet (2*n)".to_string());
            }
            if i % 3 == 0 {
                line1.push("wäre eine schwarze Sonne (-3*n), wenn ins Negative durch eine Typ 13 verdreht".to_string());
            }
            zeilenInhalte.push(line1.join(", und außerdem "));
        }
        let spalte = self.fuege_spalte_hinzu_py(zeilenInhalte, &self.generierte_spalte_meta_name_py(64));
        self.set_generated_spalten_parameter_exact_from_data_dict_py(spalte, 0, "64");
        self.set_generated_spalten_tags_exact_py(spalte, &[ST::sternPolygon, ST::universum, ST::galaxie]);
        Self::push_unique_i64_py(rowsAsNumbers, spalte);
    }

    pub fn apply_concat_generators_py(&mut self) {
        let concat1_selection = self.CsvTheirsSpalten.get(&1).cloned().unwrap_or_default();
        let gebr_gal_n = self.CsvTheirsSpalten.get(&2).cloned().unwrap_or_default();
        let gebr_gal_1n = self.CsvTheirsSpalten.get(&3).cloned().unwrap_or_default();
        let gebr_uni_n = self.CsvTheirsSpalten.get(&4).cloned().unwrap_or_default();
        let gebr_uni_1n = self.CsvTheirsSpalten.get(&5).cloned().unwrap_or_default();
        let gebr_emo_n = self.CsvTheirsSpalten.get(&6).cloned().unwrap_or_default();
        let gebr_emo_1n = self.CsvTheirsSpalten.get(&7).cloned().unwrap_or_default();
        let gebr_groe_n = self.CsvTheirsSpalten.get(&8).cloned().unwrap_or_default();
        let gebr_groe_1n = self.CsvTheirsSpalten.get(&9).cloned().unwrap_or_default();

        let mut rowsAsNumbers = std::mem::take(&mut self.rowsAsNumbers);

        let _ = self.readConcatCsv(&mut rowsAsNumbers, concat1_selection, 1);
        let _ = self.readConcatCsv(&mut rowsAsNumbers, gebr_gal_n, 2);
        let _ = self.readConcatCsv(&mut rowsAsNumbers, gebr_gal_1n, 3);
        let _ = self.readConcatCsv(&mut rowsAsNumbers, gebr_uni_n, 4);
        let _ = self.readConcatCsv(&mut rowsAsNumbers, gebr_uni_1n, 5);
        let _ = self.readConcatCsv(&mut rowsAsNumbers, gebr_emo_n, 6);
        let _ = self.readConcatCsv(&mut rowsAsNumbers, gebr_emo_1n, 7);
        let _ = self.readConcatCsv(&mut rowsAsNumbers, gebr_groe_n, 8);
        let _ = self.readConcatCsv(&mut rowsAsNumbers, gebr_groe_1n, 9);

        self.concatVervielfacheZeile(&mut rowsAsNumbers);
        self.concatModallogik(&mut rowsAsNumbers);
        self.concatPrimCreativityType(&mut rowsAsNumbers);
        self.concatGleichheitFreiheitDominieren(&mut rowsAsNumbers);
        self.concatGeistEmotionEnergieMaterieTopologie(&mut rowsAsNumbers);
        self.concatMondExponzierenLogarithmusTyp(&mut rowsAsNumbers);
        self.concat1RowPrimUniverse2(&mut rowsAsNumbers);
        self.concat1PrimzahlkreuzProContra(&mut rowsAsNumbers);
        self.concatLovePolygon(&mut rowsAsNumbers);
        self.spalteFuerGegenInnenAussenSeitlichPrim(&mut rowsAsNumbers);
        self.spalteMetaKontretTheorieAbstrakt_etc_1(&mut rowsAsNumbers);
        self.createSpalteGestirn(&mut rowsAsNumbers);

        self.remove_concat1_trigger_columns_py(&mut rowsAsNumbers);
        self.rowsAsNumbers = rowsAsNumbers;
    }
}
