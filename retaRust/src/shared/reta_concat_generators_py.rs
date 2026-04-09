
use std::collections::{BTreeMap, BTreeSet};

use crate::shared::reta_program_types::Program;
use crate::shared::reta_generators_inventory_py::{BOOL_AND_TUPLE_SET1_SPECS, GENERATED1_SPECS, GENERATED2_SPECS, METAKONKRET_SPECS};

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
    }

    fn boolAndTupleSet1Options_exact_py(&self) -> Vec<Option<usize>> {
        self.boolAndTupleSet1Options.iter().map(|v| v.map(|x| x as usize)).collect()
    }

    fn metakonkret_pairs_exact_py(&self) -> Vec<(i64, i64)> {
        self.metakonkretPairs.clone()
    }

    fn generator_row_end_py(&self) -> usize {
        if self.lastLineNumber > 0 {
            let max_idx = self.relitable.len().saturating_sub(1) as i64;
            return self.lastLineNumber.min(max_idx).max(0) as usize;
        }
        self.relitable.len().saturating_sub(1)
    }

    fn spalteMetaKonkretAbstrakt_isGanzZahlig_py(&self, zahl: f64, spaltenWahl: bool) -> bool {
        let mut zahl = if spaltenWahl { 1.0 / zahl } else { zahl };
        zahl = zahl.fract().abs();
        zahl < 0.00001 || zahl > 0.99999
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
        for spec in GENERATED2_SPECS {
            if spec.code == code {
                return format!("{} {}", spec.main_name, spec.parameter_name);
            }
        }
        code.to_string()
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

    fn factor_pairs_py(&self, n: i64) -> Vec<(i64, i64)> {
        let mut out: Vec<(i64, i64)> = vec![];
        if n <= 1 {
            return out;
        }
        let mut a = 2i64;
        while a * a <= n {
            if n % a == 0 {
                let b = n / a;
                out.push((a, b));
            }
            a += 1;
        }
        out
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
            let mut oldPrimAmounts = 0i64;
            let mut lastPrimAnswers: BTreeMap<i64, String> = BTreeMap::new();
            let row_end = self.generator_row_end_py();
            for i in 0..=row_end {
                let mut into = if i != 0 {
                    vec![String::new()]
                } else {
                    vec!["Primzahlwirkung (7, Richtung) ".to_string(), match kk { Some(k) => format!("{}", self.zellenwert_py(0, k)), None => "Richtung-Richtung".to_string() }]
                };
                oldPrimAmounts = primAmounts;
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

    pub fn readConcatCsv(&mut self, rowsAsNumbers: &mut Vec<i64>, concatTableSelection: Vec<i64>, concatTable: i64) -> Vec<i64> {
        let mut concatCSVspalten: Vec<i64> = vec![];
        if concatTableSelection.is_empty() { return concatCSVspalten; }
        let Some(csvFileName) = self.concat_csv_name_py(concatTable) else { return concatCSVspalten; };
        let Ok(mut tableToAdd) = self.load_csv_rows_semicolon_exact_path(csvFileName) else { return concatCSVspalten; };
        tableToAdd = self.readConcatCsv_ChangeTableToAddToTable(concatTable, tableToAdd);
        if concatTable == 1 {
            let mut tableToAdd2 = vec![vec!["Primzahlvielfache, nicht generiert".to_string()]];
            for zeile in tableToAdd.into_iter().skip(1) {
                let mut teile: Vec<String> = vec![];
                for zelle in zeile {
                    if zelle.trim().len() > 3 {
                        teile.push(zelle);
                    }
                }
                tableToAdd2.push(vec![teile.join(" | ")]);
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
                    if ((u as i64 + 2).checked_sub(0).unwrap_or(0) != 0 && concatTableSelection.contains(&(u as i64 + 2)) && (2..=9).contains(&concatTable)) || concatTable == 1 {
                        let selectedSpalten = start + u as i64 + if (2..=9).contains(&concatTable) { 1 } else { 0 };
                        Self::push_unique_i64_py(rowsAsNumbers, selectedSpalten);
                        concatCSVspalten.push(selectedSpalten);
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
        let conceptsRowsSetOfTuple2 = self.generated1_pairs_exact_py();
        if conceptsRowsSetOfTuple2.is_empty() {
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

        for concept in conceptsRowsSetOfTuple2 {
            let concept0 = concept.0 as usize;
            let concept1 = concept.1 as usize;
            let mut into: Vec<String> = vec![String::new(); reliTableCopy.len()];
            let mut einMalVorkommen: Vec<usize> = vec![];

            for (i, cols) in reliTableCopy.iter().enumerate() {
                if i == 0 {
                    into[i] = format!("Generiert: {}", cols.get(concept0).cloned().unwrap_or_default());
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
                    for (modalOperatoren, &vervielfachter) in entry_by_dist.modalS.iter().zip(entry_by_dist.vervielfachter.iter()) {
                        let intoItsContent = if distanceFromLine.abs() % 2 == 0 {
                            reliTableCopy.get(vervielfachter).and_then(|r| r.get(concept0)).cloned().unwrap_or_default()
                        } else {
                            reliTableCopy.get(vervielfachter).and_then(|r| r.get(concept1)).cloned().unwrap_or_default()
                        };
                        if intoItsContent.is_empty() || modalOperatoren.len() < 2 {
                            continue;
                        }
                        let basis_content = if reliTableCopy.get(1).and_then(|r| r.get(97)).map(|s| s == &modalOperatoren[0]).unwrap_or(false) {
                            intoItsContent.clone()
                        } else {
                            self.modal_replace_zuerst_zweites_py(intoItsContent.clone())
                        };
                        into[i].push_str(self.modal_text_by_distance_exact_py(distanceFromLine));
                        into[i].push_str(&modalOperatoren[0]);
                        into[i].push(' ');
                        into[i].push_str(&basis_content);
                        into[i].push(' ');
                        into[i].push_str(&modalOperatoren[1]);
                        if distanceFromLine.abs() % 2 == 1 && modalOperatoren.len() > 2 {
                            into[i].push_str(", nicht: ");
                            into[i].push_str(&modalOperatoren[2..].join(", "));
                            into[i].push_str(" (das alles nicht): ");
                            let c0 = reliTableCopy.get(vervielfachter).and_then(|r| r.get(concept0)).cloned().unwrap_or_default();
                            into[i].push_str(&self.modal_replace_zuerst_zweites_py(c0));
                        }
                        into[i].push_str(" | ");
                    }
                }
                let conditionNvs1perN = matches!(concept.0, 62 | 63 | 358..=367 | 371..=374);
                let fill_ = if conditionNvs1perN {
                    reliTableCopy.get(i).and_then(|r| r.get(197)).cloned().unwrap_or_default()
                } else {
                    reliTableCopy.get(i).and_then(|r| r.get(4)).cloned().unwrap_or_default()
                };
                if !into[i].is_empty() {
                    if into[i].ends_with(" | ") {
                        let new_len = into[i].len().saturating_sub(3);
                        into[i].truncate(new_len);
                    }
                    into[i].push_str(" | Alles nur bezogen auf die selbe Strukturgröße einer ");
                    into[i].push_str(&fill_);
                }
            }

            let spalte = self.fuege_spalte_hinzu_py(into, &self.generierte_spalte_meta_name_py(concept.0));
            Self::push_unique_i64_py(rowsAsNumbers, spalte);
        }
    }

    pub fn concat1RowPrimUniverse2(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        let generatedBefehle: Vec<String> = self.generated2Codes.clone();
        if generatedBefehle.is_empty() {
            return;
        }
        for code in generatedBefehle {
            if code == "primzahlkreuzprocontra" {
                continue;
            }
            let (col_a, col_b) = self.generated2_code_source_columns_py(&code);
            let heading = self.generated2_code_heading_py(&code);
            let mut into: Vec<String> = vec![];
            let row_end = self.generator_row_end_py();
            for i in 0..=row_end {
                if i == 0 {
                    into.push(heading.clone());
                    continue;
                }
                let mut teile: Vec<String> = vec![];
                for (prim, primAmount) in self.primRepeat(self.primFak(i as i64)) {
                    let basis = if primAmount % 2 == 0 {
                        self.zellenwert_py(prim as usize, col_b)
                    } else {
                        self.zellenwert_py(prim as usize, col_a)
                    };
                    if basis.trim().is_empty() {
                        continue;
                    }
                    if primAmount > 1 {
                        teile.push(format!("{} * {}", primAmount, basis));
                    } else {
                        teile.push(basis);
                    }
                }
                if teile.is_empty() {
                    into.push(String::new());
                } else {
                    into.push(self.nicht_leere_teile_join_py(teile, " + "));
                }
            }
            let spalte = self.fuege_spalte_hinzu_py(into, &heading);
            Self::push_unique_i64_py(rowsAsNumbers, spalte);
        }
    }

    pub fn concat1PrimzahlkreuzProContra(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        if !self.hat_generated2_code_py("primzahlkreuzprocontra") {
            return;
        }

        let row_end = self.generator_row_end_py();
        let mut detail_col: Vec<String> = vec![];
        let mut reverse_col: Vec<String> = vec![];
        let mut reverse_pro: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
        let mut reverse_contra: BTreeMap<i64, Vec<i64>> = BTreeMap::new();

        detail_col.push("Primzahlkreuz pro/contra".to_string());
        reverse_col.push("Primzahlkreuz pro/contra - invers".to_string());

        for num in 1..=row_end {
            let n = num as i64;
            let mut pro_texte: Vec<String> = vec![];
            let mut contra_texte: Vec<String> = vec![];
            let mut pro_nums: Vec<i64> = vec![];
            let mut contra_nums: Vec<i64> = vec![];

            for (prim, prim_amount) in self.primRepeat(self.primFak(n)) {
                let basis_pro = if self.couldBePrimeNumberPrimzahlkreuz_fuer_aussen(prim) {
                    prim
                } else if self.couldBePrimeNumberPrimzahlkreuz_fuer_innen(prim) {
                    prim
                } else {
                    0
                };
                if basis_pro != 0 {
                    if self.couldBePrimeNumberPrimzahlkreuz_fuer_aussen(prim) {
                        pro_nums.push(basis_pro);
                        reverse_pro.entry(basis_pro).or_default().push(n);
                        if prim_amount > 1 {
                            pro_texte.push(format!("pro {}*{}", prim_amount, basis_pro));
                        } else {
                            pro_texte.push(format!("pro {}", basis_pro));
                        }
                    }
                    if self.couldBePrimeNumberPrimzahlkreuz_fuer_innen(prim) {
                        contra_nums.push(basis_pro);
                        reverse_contra.entry(basis_pro).or_default().push(n);
                        if prim_amount > 1 {
                            contra_texte.push(format!("gegen {}*{}", prim_amount, basis_pro));
                        } else {
                            contra_texte.push(format!("gegen {}", basis_pro));
                        }
                    }
                }
            }

            for (a, b) in self.factor_pairs_py(n) {
                if self.couldBePrimeNumberPrimzahlkreuz_fuer_aussen(a) {
                    reverse_pro.entry(a).or_default().push(n);
                    pro_nums.push(a);
                }
                if self.couldBePrimeNumberPrimzahlkreuz_fuer_aussen(b) {
                    reverse_pro.entry(b).or_default().push(n);
                    pro_nums.push(b);
                }
                if self.couldBePrimeNumberPrimzahlkreuz_fuer_innen(a) {
                    reverse_contra.entry(a).or_default().push(n);
                    contra_nums.push(a);
                }
                if self.couldBePrimeNumberPrimzahlkreuz_fuer_innen(b) {
                    reverse_contra.entry(b).or_default().push(n);
                    contra_nums.push(b);
                }
            }

            pro_nums.sort_unstable();
            pro_nums.dedup();
            contra_nums.sort_unstable();
            contra_nums.dedup();

            let mut teile: Vec<String> = vec![];
            if !pro_texte.is_empty() {
                teile.push(pro_texte.join(", "));
            }
            if !contra_texte.is_empty() {
                teile.push(contra_texte.join(", "));
            }
            if teile.is_empty() {
                detail_col.push(String::new());
            } else {
                detail_col.push(teile.join(" | "));
            }
        }

        for num in 1..=row_end {
            let n = num as i64;
            let mut teile: Vec<String> = vec![];
            let mut pro2 = reverse_pro.get(&n).cloned().unwrap_or_default();
            let mut contra2 = reverse_contra.get(&n).cloned().unwrap_or_default();
            pro2.sort_unstable();
            pro2.dedup();
            contra2.sort_unstable();
            contra2.dedup();
            if !pro2.is_empty() {
                teile.push(format!("pro dieser Zahl sind: {}", pro2.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")));
            }
            if !contra2.is_empty() {
                teile.push(format!("contra dieser Zahl sind: {}", contra2.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")));
            }
            reverse_col.push(teile.join(" | "));
        }

        let spalte_detail = self.fuege_spalte_hinzu_py(detail_col, "Primzahlkreuz pro contra");
        let spalte_reverse = self.fuege_spalte_hinzu_py(reverse_col, "Primzahlkreuz pro contra invers");
        Self::push_unique_i64_py(rowsAsNumbers, spalte_detail);
        Self::push_unique_i64_py(rowsAsNumbers, spalte_reverse);
    }

    pub fn spalteMetaKontretTheorieAbstrakt_etc_1(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        let geordnetePaare = self.metakonkret_pairs_exact_py();
        for paar in geordnetePaare {
            let metavariable = paar.0;
            let lower1greater2both3 = if paar.1 == 0 { 1 } else if paar.1 == 1 { 2 } else { 3 };
            self.spalteMetaKontretTheorieAbstrakt_etc(rowsAsNumbers, metavariable, lower1greater2both3);
        }
    }

    pub fn spalteMetaKontretTheorieAbstrakt_etc(&mut self, rowsAsNumbers: &mut Vec<i64>, metavariable: i64, lower1greater2both3: i64) {
        let (meta_name, konkret_name) = self.meta_prefixes_py(metavariable);
        let bothRowsListe: Vec<i64> = if lower1greater2both3 == 3 { vec![0, 1] } else if lower1greater2both3 == 1 { vec![0] } else if lower1greater2both3 == 2 { vec![1] } else { vec![] };
        for ifInvers in 0..=1 {
            let transzendentalienSpalten = if ifInvers == 0 { (5usize, 131usize) } else { (131usize, 5usize) };
            for bothRows in bothRowsListe.iter() {
                let mut into: Vec<String> = vec![];
                let row_end = self.generator_row_end_py();
            for i in 0..=row_end {
                    if i == 0 {
                        let praefix = if *bothRows == 0 { meta_name } else { konkret_name };
                        let suffix = if ifInvers == 0 { "für n" } else { "für 1/n statt n" };
                        into.push(format!("{} {}", praefix, suffix));
                        continue;
                    }
                    if i < 2 {
                        into.push(String::new());
                        continue;
                    }
                    let mut neue2KoordNeue2Vorwoerter: Vec<String> = vec![];
                    let mut moreAndLess = (i as i64, i as i64);
                    let mut newCol = transzendentalienSpalten.0;
                    let mut zaehler = 0usize;
                    while zaehler < 6 {
                        zaehler += 1;
                        if moreAndLess.0 <= 0 && moreAndLess.1 <= 0 {
                            break;
                        }
                        let praefix = if *bothRows == 0 { meta_name } else { konkret_name };
                        let text = self.zellenwert_py(moreAndLess.0.max(1) as usize, newCol);
                        if !text.trim().is_empty() {
                            neue2KoordNeue2Vorwoerter.push(format!("{}-{} ({})", praefix, text, moreAndLess.0.max(1)));
                        }
                        if newCol == transzendentalienSpalten.0 {
                            newCol = transzendentalienSpalten.1;
                            moreAndLess.0 /= metavariable.max(1);
                        } else {
                            newCol = transzendentalienSpalten.0;
                            moreAndLess.0 *= metavariable.max(1);
                            if moreAndLess.0 as usize >= self.relitable.len() {
                                break;
                            }
                        }
                    }
                    into.push(self.nicht_leere_teile_join_py(neue2KoordNeue2Vorwoerter, " | "));
                }
                let spalte = self.fuege_spalte_hinzu_py(into, &format!("{} {}", meta_name, konkret_name));
                Self::push_unique_i64_py(rowsAsNumbers, spalte);
            }
        }
    }

    pub fn createSpalteGestirn(&mut self, rowsAsNumbers: &mut Vec<i64>) {
        if !rowsAsNumbers.contains(&64) { return; }
        let mut zeilenInhalte: Vec<String> = vec![];
        let row_end = self.generator_row_end_py();
        for i in 0..=row_end {
            if i == 0 {
                zeilenInhalte.push("Gestirn".to_string());
                continue;
            }
            let mut line1: Vec<String> = vec![];
            if (i - 1) % 3 == 1 {
                line1.push("wäre eine schwarze Sonne (-3*n), wenn ins Negative durch eine Typ 13 verdreht".to_string());
            }
            if !self.moonNumber((i - 1) as i64).1.is_empty() {
                line1.push("Mond (Potenzen)".to_string());
            } else {
                line1.push("Sonne (keine Potenzen)".to_string());
            }
            if (i - 1) % 2 == 0 {
                line1.push("Planet (2*n)".to_string());
            }
            zeilenInhalte.push(line1.join(", und außerdem "));
        }
        let spalte = self.fuege_spalte_hinzu_py(zeilenInhalte, &self.generierte_spalte_meta_name_py(64));
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

        self.rowsAsNumbers = rowsAsNumbers;
    }
}
