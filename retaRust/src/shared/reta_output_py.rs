#![allow(non_snake_case)]

use std::collections::{BTreeMap, BTreeSet};

use hypher::{hyphenate, Lang};

use crate::shared::reta_program_types::{dedup_preserve_order_i64, Program};

impl Program {
    pub(crate) fn prepare4out_py(
        &mut self,
        paramLines: Vec<String>,
        paramLinesNot: Vec<String>,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
    ) -> (Vec<String>, Vec<Vec<String>>, i64, Vec<i64>, Vec<i64>) {
        let mut newTable: Vec<Vec<String>> = vec![];
        let mut finallyDisplayLines: Vec<String> = vec![];
        let mut old2newTable: Vec<i64> = vec![];

        if relitable.is_empty() {
            return (finallyDisplayLines, newTable, 0, vec![], old2newTable);
        }

        let max_row = relitable.len().saturating_sub(1) as i64;
        let mut selected_rows: Vec<i64> = self.selected_rows_from_param_lines_py(
            &paramLines,
            &paramLinesNot,
            max_row,
        );
        selected_rows = dedup_preserve_order_i64(selected_rows);
        if !self.keineUeberschriften && !selected_rows.contains(&0) {
            selected_rows.insert(0, 0);
        }

        let mut selected_cols: Vec<i64> = if rowsAsNumbers.is_empty() {
            if relitable[0].is_empty() {
                vec![]
            } else {
                (0..(relitable[0].len() as i64)).collect()
            }
        } else {
            rowsAsNumbers.clone()
        };
        selected_cols = dedup_preserve_order_i64(selected_cols);
        let selected_cols_set: BTreeSet<i64> = selected_cols.iter().cloned().collect();

        for row_no in selected_rows.iter() {
            let idx = *row_no as usize;
            if idx >= relitable.len() {
                continue;
            }
            let mut new2Lines: Vec<String> = vec![];
            for (t, cell) in relitable[idx].iter().enumerate() {
                if selected_cols_set.contains(&(t as i64)) {
                    new2Lines.push(cell.clone());
                }
            }
            newTable.push(new2Lines);
            old2newTable.push(*row_no);
        }

        if newTable.is_empty() {
            newTable = relitable.clone();
            old2newTable = (0..(relitable.len() as i64)).collect();
        }

        finallyDisplayLines = old2newTable.iter().map(|n| n.to_string()).collect();
        if !finallyDisplayLines.is_empty() && !self.keineUeberschriften {
            finallyDisplayLines[0] = "".to_string();
        }

        let rowsRange: Vec<i64> = if newTable.is_empty() {
            vec![]
        } else {
            (0..(newTable[0].len() as i64)).collect()
        };
        let numlen = old2newTable.last().map(|v| v.to_string().len() as i64).unwrap_or(0);
        (finallyDisplayLines, newTable, numlen, rowsRange, old2newTable)
    }


    fn selected_rows_from_param_lines_py(
        &self,
        param_lines: &[String],
        param_lines_not: &[String],
        max_row: i64,
    ) -> Vec<i64> {
        let mut selected_rows: Vec<i64> = self
            .filter_original_lines_py(BTreeSet::new(), param_lines, max_row)
            .into_iter()
            .collect();
        if selected_rows.is_empty() && !self.rowRange.is_empty() {
            selected_rows = self
                .rowRange
                .iter()
                .copied()
                .filter(|row| *row > 0 && *row <= max_row)
                .collect();
        }
        if !param_lines_not.is_empty() {
            let exclude_rows = self.filter_original_lines_py(BTreeSet::new(), param_lines_not, max_row);
            selected_rows.retain(|row| !exclude_rows.contains(row));
        }
        selected_rows.sort_unstable();
        selected_rows
    }

    fn cutset_py(wether: bool, a: &BTreeSet<i64>, b: &BTreeSet<i64>) -> BTreeSet<i64> {
        if wether {
            a.intersection(b).copied().collect()
        } else {
            a.clone()
        }
    }

    fn split_top_level_commas_filter_py(txt: &str) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut current = String::new();
        let mut depth_round = 0i32;
        let mut depth_square = 0i32;
        let mut depth_curly = 0i32;

        for ch in txt.chars() {
            match ch {
                '(' => { depth_round += 1; current.push(ch); }
                ')' => { depth_round -= 1; current.push(ch); }
                '[' => { depth_square += 1; current.push(ch); }
                ']' => { depth_square -= 1; current.push(ch); }
                '{' => { depth_curly += 1; current.push(ch); }
                '}' => { depth_curly -= 1; current.push(ch); }
                ',' if depth_round == 0 && depth_square == 0 && depth_curly == 0 => {
                    if !current.is_empty() {
                        out.push(current.clone());
                    }
                    current.clear();
                }
                _ => current.push(ch),
            }
        }
        if !current.is_empty() {
            out.push(current);
        }
        out
    }

    fn is_zeilen_angabe_between_kommas_filter_py(txt: &str) -> bool {
        let txt = txt.trim();
        if txt.is_empty() {
            return false;
        }
        let txt = txt.strip_prefix('v').unwrap_or(txt);
        let txt = txt.strip_prefix('-').unwrap_or(txt);
        if txt.is_empty() {
            return false;
        }
        let mut parts = txt.split('+');
        let first = parts.next().unwrap_or_default();
        let first_ok = if let Some((a, b)) = first.split_once('-') {
            !a.is_empty() && !b.is_empty() && a.chars().all(|c| c.is_ascii_digit()) && b.chars().all(|c| c.is_ascii_digit())
        } else {
            first.chars().all(|c| c.is_ascii_digit())
        };
        first_ok && parts.all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
    }

    fn is_zeilen_angabe_filter_py(txt: &str) -> bool {
        let parts = Self::split_top_level_commas_filter_py(txt);
        let any_at_all = parts.iter().any(|part| !part.is_empty());
        any_at_all && parts.iter().all(|part| part.is_empty() || Self::is_zeilen_angabe_between_kommas_filter_py(part))
    }

    fn bereich_to_numbers2_py(txt: &str, vielfache: bool, max_zahl: i64, allow_less_eq_zero: bool) -> BTreeSet<i64> {
        let cleaned_parts = Self::split_top_level_commas_filter_py(txt);
        let cleaned = cleaned_parts.iter().filter(|s| !s.is_empty()).cloned().collect::<Vec<_>>().join(",");
        if cleaned.is_empty() || !Self::is_zeilen_angabe_filter_py(&cleaned) {
            return BTreeSet::new();
        }

        let effective_max = if !vielfache && max_zahl == 0 { i64::MAX / 4 } else if max_zahl <= 0 { 1028 } else { max_zahl };
        let mut dazu: BTreeSet<i64> = BTreeSet::new();
        let mut hinfort: BTreeSet<i64> = BTreeSet::new();

        for mut ein_bereich in Self::split_top_level_commas_filter_py(&cleaned) {
            if ein_bereich.is_empty() {
                continue;
            }
            let mut vielfache2 = vielfache;
            if let Some(rest) = ein_bereich.strip_prefix('v') {
                ein_bereich = rest.to_string();
                vielfache2 = true;
            }

            let mut remove = false;
            if let Some(rest) = ein_bereich.strip_prefix('-') {
                remove = true;
                ein_bereich = rest.to_string();
            }

            let menge = if remove { &mut hinfort } else { &mut dazu };
            let mut around: Vec<i64> = Vec::new();
            let plus_parts: Vec<&str> = ein_bereich.split('+').collect();
            let normalized = if ein_bereich.chars().all(|c| c.is_ascii_digit()) {
                format!("{0}-{0}", ein_bereich)
            } else if !plus_parts.is_empty() && plus_parts[0].chars().all(|c| c.is_ascii_digit()) {
                let mut rebuilt = format!("{0}-{0}", plus_parts[0]);
                if plus_parts.len() > 1 {
                    rebuilt.push('+');
                    rebuilt.push_str(&plus_parts[1..].join("+"));
                }
                rebuilt
            } else {
                ein_bereich.clone()
            };

            let Some((left, right_all)) = normalized.split_once('-') else {
                continue;
            };
            if !left.chars().all(|c| c.is_ascii_digit()) || left == "0" {
                continue;
            }
            let right_parts: Vec<&str> = right_all.split('+').collect();
            if right_parts.is_empty() || !right_parts[0].chars().all(|c| c.is_ascii_digit()) {
                continue;
            }
            let start = left.parse::<i64>().unwrap_or(0);
            let end = right_parts[0].parse::<i64>().unwrap_or(0);
            if start <= 0 || end <= 0 || end < start {
                continue;
            }
            if right_parts.len() < 2 {
                around.push(0);
            } else {
                let mut ok = true;
                for part in &right_parts[1..] {
                    if part.chars().all(|c| c.is_ascii_digit()) {
                        around.push(part.parse::<i64>().unwrap_or(0));
                    } else {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }
            }

            if !vielfache2 {
                for number in start..=end {
                    for a in &around {
                        let c = number + *a;
                        if c < effective_max {
                            menge.insert(c);
                        }
                        let d = number - *a;
                        if d > 0 && d < effective_max {
                            menge.insert(d);
                        }
                    }
                }
            } else {
                let around_only_zero = around.is_empty() || around.iter().all(|a| *a == 0);
                let mut i = 0i64;
                loop {
                    let cond = around.iter().all(|a| start.saturating_mul(i) < effective_max.saturating_sub(*a));
                    if !cond {
                        break;
                    }
                    i += 1;
                    for number in start..=end {
                        if around_only_zero {
                            let c = number.saturating_mul(i);
                            if c <= effective_max {
                                menge.insert(c);
                            }
                        } else {
                            for a in &around {
                                let c = number.saturating_mul(i) + *a;
                                if c <= effective_max {
                                    menge.insert(c);
                                }
                                let d = number.saturating_mul(i) - *a;
                                if d > 0 && d < effective_max {
                                    menge.insert(d);
                                }
                            }
                        }
                    }
                }
            }
        }

        let result: BTreeSet<i64> = dazu.difference(&hinfort).copied().collect();
        if allow_less_eq_zero {
            result
        } else {
            result.into_iter().filter(|x| *x > 0).collect()
        }
    }

    fn multiples_py(a: i64, mul1: bool) -> Vec<(i64, i64)> {
        let mut menge: BTreeSet<(i64, i64)> = BTreeSet::new();
        let mut b = 2i64;
        while b * b <= a {
            if a % b == 0 {
                menge.insert((a / b, b));
            }
            b += 1;
        }
        let mut out: Vec<(i64, i64)> = menge.into_iter().collect();
        if mul1 {
            out.push((a, 1));
        }
        out
    }

    fn teiler_py(zahlen_bereichs_angabe: &str) -> BTreeSet<i64> {
        let zahlen_bereich_menge = Self::bereich_to_numbers2_py(zahlen_bereichs_angabe, false, 0, false);
        let mut zahlen_wbereich_menge: BTreeSet<i64> = BTreeSet::new();
        for each1 in zahlen_bereich_menge {
            for each2 in Self::multiples_py(each1, true) {
                zahlen_wbereich_menge.insert(each2.0);
                zahlen_wbereich_menge.insert(each2.1);
            }
        }
        if zahlen_wbereich_menge != BTreeSet::from([1]) {
            zahlen_wbereich_menge.remove(&1);
        }
        zahlen_wbereich_menge
    }

    fn prim_fak_py(n: i64) -> Vec<i64> {
        if n < 2 {
            return vec![];
        }
        let mut z = n;
        let mut out: Vec<i64> = Vec::new();
        while z > 1 {
            let mut i = 2i64;
            let mut p = z;
            let mut gefunden = false;
            while i * i <= z && !gefunden {
                if z % i == 0 {
                    gefunden = true;
                    p = i;
                } else {
                    i += 1;
                }
            }
            out.push(p);
            z /= p;
        }
        out
    }

    fn prim_repeat2_py(factors: &[i64]) -> Vec<(i64, i64)> {
        let mut counts: BTreeMap<i64, i64> = BTreeMap::new();
        for &f in factors {
            *counts.entry(f).or_insert(0) += 1;
        }
        counts.into_iter().collect()
    }

    fn moonsun_rows_py(&self, moon_not_sun: bool, num_range: &BTreeSet<i64>) -> BTreeSet<i64> {
        let mut out = BTreeSet::new();
        for n in num_range {
            if Self::moon_number_is_py(*n) == moon_not_sun {
                out.insert(*n);
            }
        }
        out
    }

    fn is_prim_multiple_py(is_it: i64, multiples1: &[i64]) -> bool {
        if multiples1.is_empty() {
            return false;
        }
        if multiples1.contains(&is_it) {
            return true;
        }
        let mut seen_primes: BTreeSet<i64> = BTreeSet::new();
        for p in Self::prim_fak_py(is_it) {
            if seen_primes.insert(p) {
                let multiple2 = is_it / p;
                if multiples1.contains(&multiple2) {
                    return true;
                }
            }
        }
        false
    }

    fn filter_original_lines_py(
        &self,
        mut num_range: BTreeSet<i64>,
        param_lines: &[String],
        max_row: i64,
    ) -> BTreeSet<i64> {
        num_range.remove(&0);

        let relevant_params: BTreeSet<String> = param_lines
            .iter()
            .filter(|p| p.as_str() != "ka" && p.as_str() != "ka2")
            .cloned()
            .collect();

        if param_lines.iter().any(|p| p == "all") || relevant_params.is_empty() || !self.ifZeilenSetted {
            num_range = (1..=max_row).collect();
        } else {
            num_range.clear();
        }

        let mut if_a_at_all = false;
        let mut mehrere: Vec<String> = Vec::new();
        let mut if_teiler = false;
        for condition in param_lines {
            if condition.starts_with("_a_") && condition.len() > 3 {
                if_a_at_all = true;
                mehrere.push(condition[3..].to_string());
            }
            if condition.starts_with("_w_") {
                if_teiler = true;
            }
        }
        if if_a_at_all {
            let joined = mehrere.join(",");
            num_range.extend(Self::bereich_to_numbers2_py(&joined, false, max_row + 1, false));
            if if_teiler {
                num_range.extend(Self::teiler_py(&joined));
            }
            if !num_range.is_empty() {
                for eins in joined.split(',') {
                    let mut eins = eins.to_string();
                    let ja1 = eins.starts_with('-');
                    let ja2 = eins.starts_with("v-");
                    if ja1 || ja2 {
                        if ja1 {
                            eins = eins[1..].to_string();
                        }
                        if ja2 {
                            eins = format!("v{}", &eins[2..]);
                        }
                        let minus = Self::bereich_to_numbers2_py(&eins, false, max_row + 1, false);
                        num_range = num_range.difference(&minus).copied().collect();
                    }
                }
            }
        }

        let mut if_b_at_all = false;
        mehrere.clear();
        let mut num_range_yes_z: BTreeSet<i64> = BTreeSet::new();
        for condition in param_lines {
            if condition.starts_with("_b_") && condition.len() > 3 {
                if_b_at_all = true;
                mehrere.push(condition[3..].to_string());
            }
        }
        if if_b_at_all {
            if num_range.is_empty() && !if_a_at_all && !param_lines.iter().any(|p| p == "all") {
                num_range = (1..=max_row).collect();
            }
            let joined = mehrere.join(",");
            num_range_yes_z.extend(Self::bereich_to_numbers2_py(&joined, true, max_row + 1, false));
            if !num_range_yes_z.is_empty() {
                num_range = num_range.intersection(&num_range_yes_z).copied().collect();
            }
            if !num_range.is_empty() {
                for eins in joined.split(',') {
                    let mut eins = eins.to_string();
                    let ja1 = eins.starts_with('-');
                    let ja2 = eins.starts_with("v-");
                    if ja1 || ja2 {
                        if ja1 { eins = eins[1..].to_string(); }
                        if ja2 { eins = format!("v{}", &eins[2..]); }
                        let minus = Self::bereich_to_numbers2_py(&eins, true, max_row + 1, false);
                        num_range = num_range.difference(&minus).copied().collect();
                    }
                }
            }
        }

        num_range_yes_z.clear();
        let mut if_zeit_at_all = false;
        for condition in param_lines {
            if condition == "=" {
                if_zeit_at_all = true;
                if max_row >= 10 { num_range_yes_z.insert(10); }
            } else if condition == "<" {
                if_zeit_at_all = true;
                num_range_yes_z.extend(1..=max_row.min(9));
            } else if condition == ">" {
                if_zeit_at_all = true;
                if max_row >= 11 { num_range_yes_z.extend(11..=max_row); }
            }
        }
        if if_zeit_at_all {
            if num_range.is_empty() && !if_b_at_all && !if_a_at_all && !param_lines.iter().any(|p| p == "all") && num_range_yes_z.is_empty() {
                num_range = (1..=max_row).collect();
            }
            if if_a_at_all || param_lines.iter().any(|p| p == "all") || if_b_at_all {
                num_range = num_range.intersection(&num_range_yes_z).copied().collect();
            } else {
                num_range.extend(num_range_yes_z.iter().copied());
            }
        }

        num_range_yes_z.clear();
        let mut if_zaehlungen_at_all = false;
        mehrere.clear();
        for condition in param_lines {
            if condition.starts_with("_n_") && condition.len() > 3 {
                num_range_yes_z.extend(Self::bereich_to_numbers2_py(&condition[3..], false, max_row + 1, false));
                if_zaehlungen_at_all = true;
                mehrere.push(condition[3..].to_string());
            } else if let Some(rest) = condition.strip_prefix("zaehlung=") {
                num_range_yes_z.extend(Self::bereich_to_numbers2_py(rest, false, max_row + 1, false));
                if_zaehlungen_at_all = true;
                mehrere.push(rest.to_string());
            }
        }
        if if_zaehlungen_at_all {
            let mut num_range_yes_z2: BTreeSet<i64> = BTreeSet::new();
            if num_range.is_empty() && !if_a_at_all && !if_b_at_all && !param_lines.iter().any(|p| p == "all") {
                num_range = (1..=max_row).collect();
            }
            for n in &num_range {
                for z in &num_range_yes_z {
                    if Self::zeile_which_zaehlung_py(*n) == *z {
                        num_range_yes_z2.insert(*n);
                    }
                }
            }
            if !num_range_yes_z2.is_empty() && !num_range.is_empty() {
                num_range = num_range.intersection(&num_range_yes_z2).copied().collect();
            } else if num_range.is_empty() {
                num_range = num_range_yes_z2;
            }
            if !num_range.is_empty() {
                let mut minus_bereiche: BTreeSet<i64> = BTreeSet::new();
                for eins in mehrere.join(",").split(',') {
                    let mut eins = eins.to_string();
                    let ja1 = eins.starts_with('-');
                    let ja2 = eins.starts_with("v-");
                    if ja1 || ja2 {
                        if ja1 { eins = eins[1..].to_string(); }
                        if ja2 { eins = format!("v{}", &eins[2..]); }
                        minus_bereiche.extend(Self::bereich_to_numbers2_py(&eins, false, max_row + 1, false));
                    }
                }
                if !minus_bereiche.is_empty() {
                    let current_rows: Vec<i64> = num_range.iter().copied().collect();
                    for n in current_rows {
                        for z in &minus_bereiche {
                            if Self::zeile_which_zaehlung_py(n) == *z {
                                num_range.remove(&n);
                            }
                        }
                    }
                }
            }
        }

        num_range_yes_z.clear();
        if num_range.is_empty() && !relevant_params.is_empty() {
            num_range = (1..=max_row).collect();
        }
        let primzahl_filter_present = param_lines.iter().any(|p| matches!(p.as_str(), "aussenerste" | "innenerste" | "aussenalle" | "innenalle"));
        if primzahl_filter_present {
            let mut innen_aussen: BTreeMap<i64, (bool, bool, bool)> = BTreeMap::new();
            innen_aussen.insert(1, (true, false, true));
            for n in num_range.iter().copied().filter(|n| *n > 3) {
                let prim_zahlen = Self::prim_fak_py(n);
                let nur_eine_zahl = prim_zahlen.len() == 1;
                let ein_fach_vorkommen = nur_eine_zahl;
                let mut innen = false;
                let mut aussen = false;
                for prim_zahl in prim_zahlen {
                    if prim_zahl >= 4 {
                        let innen_or_aussen = prim_zahl % 6;
                        innen = innen || innen_or_aussen == 1;
                        aussen = aussen || innen_or_aussen == 5;
                    }
                }
                innen_aussen.insert(n, (innen, aussen, ein_fach_vorkommen));
            }
            if param_lines.iter().any(|p| p == "aussenerste") {
                num_range_yes_z.extend(innen_aussen.iter().filter_map(|(n, t)| if t.0 && t.2 { Some(*n) } else { None }));
            }
            if param_lines.iter().any(|p| p == "innenerste") {
                num_range_yes_z.extend(innen_aussen.iter().filter_map(|(n, t)| if t.1 && t.2 { Some(*n) } else { None }));
            }
            if param_lines.iter().any(|p| p == "aussenalle") {
                num_range_yes_z.extend(innen_aussen.iter().filter_map(|(n, t)| if t.0 { Some(*n) } else { None }));
            }
            if param_lines.iter().any(|p| p == "innenalle") {
                num_range_yes_z.extend(innen_aussen.iter().filter_map(|(n, t)| if t.1 { Some(*n) } else { None }));
            }
            let if_primtyp_at_all = !num_range_yes_z.is_empty();
            num_range = Self::cutset_py(if_primtyp_at_all, &num_range, &num_range_yes_z);
        }

        let mut if_typ_at_all = false;
        num_range_yes_z.clear();
        if num_range.is_empty() && !relevant_params.is_empty() {
            num_range = (1..=max_row).collect();
        }
        for condition in param_lines {
            if condition.contains("mond") {
                num_range_yes_z.extend(self.moonsun_rows_py(true, &num_range));
                if_typ_at_all = true;
            } else if condition.contains("schwarzesonne") {
                for n in &num_range { if *n % 3 == 0 { num_range_yes_z.insert(*n); } }
                if_typ_at_all = true;
            } else if condition.contains("sonne") {
                num_range_yes_z.extend(self.moonsun_rows_py(false, &num_range));
                if_typ_at_all = true;
            } else if condition.contains("planet") {
                for n in &num_range { if *n % 2 == 0 { num_range_yes_z.insert(*n); } }
                if_typ_at_all = true;
            } else if condition.contains("SonneMitMondanteil") {
                for n in &num_range {
                    let factors = Self::prim_repeat2_py(&Self::prim_fak_py(*n));
                    let booleans: BTreeSet<bool> = factors.iter().map(|(_, faktor)| *faktor == 1).collect();
                    if booleans.contains(&true) && booleans.contains(&false) {
                        num_range_yes_z.insert(*n);
                    }
                }
                if_typ_at_all = true;
            }
        }
        num_range = Self::cutset_py(if_typ_at_all, &num_range, &num_range_yes_z);

        let mut prim_multiples: Vec<i64> = Vec::new();
        let mut if_prim_at_all = false;
        for condition in param_lines {
            if condition.len() > 1 && condition.ends_with('p') {
                if let Ok(v) = condition[..condition.len()-1].parse::<i64>() {
                    if_prim_at_all = true;
                    prim_multiples.push(v);
                }
            }
        }
        num_range_yes_z.clear();
        if if_prim_at_all {
            if num_range.is_empty() && !if_b_at_all && !if_a_at_all && !param_lines.iter().any(|p| p == "all") && !if_typ_at_all {
                num_range = (1..=max_row).collect();
            }
            for n in &num_range {
                if Self::is_prim_multiple_py(*n, &prim_multiples) {
                    num_range_yes_z.insert(*n);
                }
            }
            num_range = Self::cutset_py(if_prim_at_all, &num_range, &num_range_yes_z);
        }

        let mut if_power_at_all = false;
        mehrere.clear();
        for condition in param_lines {
            if condition.starts_with("_^_") && condition.len() > 3 {
                if_power_at_all = true;
                mehrere.push(condition[3..].to_string());
            } else if condition.len() > 1 && condition.ends_with('^') {
                if_power_at_all = true;
                mehrere.push(condition[..condition.len()-1].to_string());
            }
        }
        let to_power_it: Vec<i64> = Self::bereich_to_numbers2_py(&mehrere.join(","), false, max_row + 1, false).into_iter().collect();
        if if_power_at_all {
            num_range_yes_z.clear();
            if num_range.is_empty() && !relevant_params.is_empty() {
                num_range = (1..=max_row).collect();
            }
            if let Some(&last_el) = num_range.iter().next_back() {
                for base in to_power_it {
                    let mut n = 0u32;
                    loop {
                        let Some(one_power) = base.checked_pow(n) else { break; };
                        if one_power > last_el {
                            break;
                        }
                        num_range_yes_z.insert(one_power);
                        n += 1;
                    }
                }
                num_range = Self::cutset_py(if_power_at_all, &num_range, &num_range_yes_z);
                num_range.remove(&1);
            }
        }

        let mut if_multiples_from_any_at_all = false;
        let mut any_multiples: Vec<i64> = Vec::new();
        for condition in param_lines {
            if condition.len() > 1 && condition.ends_with('v') && condition[..condition.len()-1].chars().all(|c| c.is_ascii_digit()) {
                if let Ok(v) = condition[..condition.len()-1].parse::<i64>() {
                    if_multiples_from_any_at_all = true;
                    any_multiples.push(v);
                }
            }
        }
        if if_multiples_from_any_at_all {
            num_range_yes_z.clear();
            for n in &num_range {
                for divisor in &any_multiples {
                    if *divisor != 0 && *n % *divisor == 0 {
                        num_range_yes_z.insert(*n);
                    }
                }
            }
            num_range = Self::cutset_py(if_multiples_from_any_at_all, &num_range, &num_range_yes_z);
        }

        let current_rows: Vec<i64> = num_range.iter().copied().collect();
        for n in current_rows {
            if !Self::moon_number_is_py(n) && n > max_row {
                num_range.remove(&n);
            }
        }

        let invertieren = param_lines.iter().any(|condition| condition.starts_with("_i_") || condition == "1i");
        if invertieren {
            let current: BTreeSet<i64> = num_range.clone();
            let mut num_range2_set = BTreeSet::new();
            for i in 1..=max_row {
                if (current.contains(&(i + 1)) || current.contains(&(i - 1))) && !current.contains(&i) {
                    num_range2_set.insert(i);
                }
            }
            num_range = num_range2_set;
        }

        let num_range_list: Vec<i64> = num_range.iter().copied().collect();
        let num_range2_map: BTreeMap<i64, i64> = num_range_list.iter().enumerate().map(|(i, a)| ((i as i64) + 1, *a)).collect();

        let mut z_ja = false;
        let mut num_range_neu2: BTreeSet<i64> = BTreeSet::new();
        for condition in param_lines {
            if condition.starts_with("_z_") && condition.len() > 3 {
                z_ja = true;
                let neu = Self::bereich_to_numbers2_py(&condition[3..], false, max_row + 1, false);
                for a in num_range2_map.keys().copied().collect::<BTreeSet<_>>().intersection(&neu).copied() {
                    if let Some(mapped) = num_range2_map.get(&a) {
                        num_range_neu2.insert(*mapped);
                    }
                }
            }
        }
        if z_ja {
            num_range = num_range.intersection(&num_range_neu2).copied().collect();
        }

        let mut y_ja = false;
        num_range_neu2.clear();
        for condition in param_lines {
            if condition.starts_with("_y_") && condition.len() > 3 {
                y_ja = true;
                let neu = Self::bereich_to_numbers2_py(&condition[3..], true, max_row + 1, false);
                for a in num_range2_map.keys().copied().collect::<BTreeSet<_>>().intersection(&neu).copied() {
                    if let Some(mapped) = num_range2_map.get(&a) {
                        num_range_neu2.insert(*mapped);
                    }
                }
            }
        }
        if y_ja {
            num_range = num_range.intersection(&num_range_neu2).copied().collect();
        }

        num_range
    }

    fn hypher_lang_py(_word: &str) -> Lang {
        Lang::German
    }

    fn hard_split_long_word_py(word: &str, width: usize) -> Vec<String> {
        if width <= 1 {
            return word.chars().map(|c| c.to_string()).collect();
        }

        let chars: Vec<char> = word.chars().collect();
        let mut out: Vec<String> = Vec::new();
        let mut i = 0usize;
        while i < chars.len() {
            let remaining = chars.len() - i;
            if remaining <= width {
                out.push(chars[i..].iter().collect());
                break;
            }
            let take = width - 1;
            let mut s: String = chars[i..i + take].iter().collect();
            s.push('-');
            out.push(s);
            i += take;
        }
        out
    }

fn split_long_word_py(word: &str, width: usize) -> Vec<String> {
    if width <= 1 || word.chars().count() <= width {
        return vec![word.to_string()];
    }

    // hypher panics for long words when alloc is disabled.
    // Keep display behavior unchanged for normal words and
    // hard-split only in the panic case.
    if word.len() > 45 {
        return Self::hard_split_long_word_py(word, width);
    }

    let lang = Self::hypher_lang_py(word);
    let syllables: Vec<String> = hyphenate(word, lang)
        .map(|s| s.to_string())
        .collect();

    if syllables.is_empty() {
        return Self::hard_split_long_word_py(word, width);
    }

    let mut out: Vec<String> = Vec::new();
    let mut current = String::new();

    for (idx, syl) in syllables.iter().enumerate() {
        let is_last = idx + 1 == syllables.len();
        let syl_len = syl.chars().count();
        let current_len = current.chars().count();
        let reserve_for_hyphen = if is_last { 0usize } else { 1usize };

        if current.is_empty() {
            if syl_len >= width {
                return Self::hard_split_long_word_py(word, width);
            }
            current.push_str(syl);
            continue;
        }

        if current_len + syl_len + reserve_for_hyphen <= width {
            current.push_str(syl);
        } else {
            let mut piece = current;
            piece.push('-');
            out.push(piece);
            current = syl.clone();
        }
    }

    if !current.is_empty() {
        out.push(current);
    }

    if out.is_empty() {
        Self::hard_split_long_word_py(word, width)
    } else {
        out
    }
}


    pub(crate) fn wrap_text_py(txt: &str, width: usize) -> Vec<String> {
        if width == 0 {
            return vec![txt.to_string()];
        }
        let mut out: Vec<String> = vec![];
        for part in txt.split('\n') {
            let mut current = String::new();
            for word in part.split_whitespace() {
                if current.is_empty() {
                    if word.chars().count() <= width {
                        current.push_str(word);
                    } else {
                        out.extend(Self::split_long_word_py(word, width));
                    }
                } else if current.chars().count() + 1 + word.chars().count() <= width {
                    current.push(' ');
                    current.push_str(word);
                } else {
                    out.push(current);
                    current = String::new();
                    if word.chars().count() <= width {
                        current.push_str(word);
                    } else {
                        let pieces = Self::split_long_word_py(word, width);
                        if let Some((last, rest)) = pieces.split_last() {
                            for piece in rest {
                                out.push(piece.clone());
                            }
                            current = last.clone();
                        }
                    }
                }
            }
            if !current.is_empty() {
                out.push(current);
            }
            if part.is_empty() {
                out.push(String::new());
            }
        }
        if out.is_empty() {
            vec![String::new()]
        } else {
            out
        }
    }



    fn moon_number_is_py(n: i64) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..n {
            let one_result = (n as f64).powf(1.0 / i as f64);
            if (one_result.round() * 100000.0).round() == (one_result * 100000.0).round() {
                return true;
            }
        }
        false
    }

    fn prim_fak_len_py(n: i64) -> usize {
        if n <= 1 {
            return 0;
        }
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
            faktoren.push(p);
            z /= p;
        }
        faktoren.len()
    }

    fn zeile_which_zaehlung_py(zeile: i64) -> i64 {
        if zeile <= 0 {
            return 0;
        }

        // Bitgenaue Ableitung aus Python:
        // setZaehlungen():
        //   isMoon startet bei True
        //   bei jedem Übergang wasMoon == True && isMoon == False
        //   wird die nächste Zählung begonnen
        //
        // moonNumber(i)[0] != []  <=> perfekte Potenz > 1
        let mut zaehlung = 0i64;
        let mut is_moon = true;

        for i in 1..=zeile {
            let was_moon = is_moon;
            let moon_now = Self::moon_number_is_py(i);
            is_moon = moon_now;

            if was_moon && !is_moon {
                zaehlung += 1;
            }
        }

        zaehlung
    }

    pub(crate) fn shell_style_py(row_number: Option<i64>, is_header: bool, rest: bool) -> &'static str {
        if is_header {
            return "[41m[30m[4m";
        }
        let n = row_number.unwrap_or(0);
        if n <= 0 {
            return "";
        }
        if rest {
            if n % 2 == 0 {
                return "[47m[30m";
            }
            return "[40m[37m";
        }
        if Self::moon_number_is_py(n) {
            if n % 2 == 0 {
                return "[106m[30m";
            }
            return "[46m[30m";
        }
        if Self::prim_fak_len_py(n) == 1 {
            if n % 2 == 0 {
                return "[103m[30m[1m";
            }
            return "[43m[30m";
        }
        if n % 2 == 0 {
            return "[47m[30m";
        }
        "[100m[37m"
    }

    pub(crate) fn styled_shell_text_py(text: &str, row_number: Option<i64>, is_header: bool, rest: bool, nocolor: bool) -> String {
        if nocolor || text.is_empty() {
            return text.to_string();
        }
        let style = Self::shell_style_py(row_number, is_header, rest);
        if style.is_empty() {
            text.to_string()
        } else {
            format!("{}{}[0m[0m", style, text)
        }
    }


    pub(crate) fn cliOut_py(
        &mut self,
        finallyDisplayLines: Vec<String>,
        newTable: Vec<Vec<String>>,
        numlen: i64,
        _rowsRange: Vec<i64>,
    ) -> Vec<Vec<String>> {
        let mut out_lines: Vec<String> = vec![];
        if newTable.is_empty() {
            self.finallyDisplayLines = out_lines.clone();
            self.numlen = numlen;
            self.finallyDisplayLinesByChunks = vec![];
            return newTable;
        }

        let col_count = newTable.iter().map(|row| row.len()).max().unwrap_or(0);
        if col_count == 0 {
            self.finallyDisplayLines = out_lines.clone();
            self.numlen = numlen;
            self.finallyDisplayLinesByChunks = vec![];
            return newTable;
        }

        // Python-like default shell width behavior:
        // without explicit --breite / --breiten, columns start at width 21.
        // Only explicit width arguments should override this.
        let mut widths: Vec<usize> = vec![21; col_count];
        for i in 0..col_count {
            let forced = if i < self.breiten.len() {
                self.breiten[i]
            } else {
                self.breite
            };
            if forced > 0 {
                widths[i] = forced as usize;
            }
        }

        let num_prefix_width = if self.nummeriere {
            finallyDisplayLines.iter().map(|s| s.chars().count()).max().unwrap_or(0)
        } else {
            0usize
        };

        let detected_shell_width = if self.shellWidth > 0 {
            self.shellWidth as usize
        } else {
            let detected = Self::detect_terminal_columns_py();
            if detected > 0 {
                detected as usize
            } else if self.textWidth > 0 {
                self.textWidth as usize
            } else {
                80usize
            }
        };
        let chunk_budget = detected_shell_width.max(21usize);

        let left_prefix = if self.nummeriere { num_prefix_width + 1 } else { 0usize };

        let mut chunks: Vec<(usize, usize)> = vec![];
        let mut start_col = 0usize;
        while start_col < col_count {
            let mut used = left_prefix;
            let mut end_col = start_col;

            while end_col < col_count {
                let add = if end_col == start_col {
                    widths[end_col]
                } else {
                    1 + widths[end_col]
                };

                if end_col > start_col && used + add > chunk_budget {
                    break;
                }
                used += add;
                end_col += 1;
            }

            if end_col == start_col {
                end_col += 1;
            }
            chunks.push((start_col, end_col));
            start_col = end_col;
        }

        let mut chunked_lines: Vec<Vec<String>> = vec![];

        for (chunk_start, chunk_end) in chunks.iter().cloned() {
            let mut one_chunk_lines: Vec<String> = vec![];

            for (row_idx, row) in newTable.iter().enumerate() {
                let mut wrapped_cells: Vec<Vec<String>> = vec![];
                let mut max_sub = 1usize;

                for i in chunk_start..chunk_end {
                    let cell = if i < row.len() { row[i].as_str() } else { "" };
                    let wrapped = Self::wrap_text_py(cell, widths[i]);
                    max_sub = max_sub.max(wrapped.len());
                    wrapped_cells.push(wrapped);
                }

                let mut should_skip_row = false;
                if self.keineleereninhalte {
                    let joined = (chunk_start..chunk_end)
                        .filter_map(|i| row.get(i))
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(" ");
                    let stripped = joined.replace('-', "").replace('?', "").trim().to_string();
                    if stripped.is_empty() {
                        should_skip_row = true;
                    }
                }
                if should_skip_row {
                    continue;
                }

                let row_number = finallyDisplayLines
                    .get(row_idx)
                    .and_then(|s| s.trim().parse::<i64>().ok());
                let is_header = row_number.is_none();

                for sub_idx in 0..max_sub {
                    let mut line = String::new();

                    if self.nummeriere {
                        let label = if sub_idx == 0 {
                            finallyDisplayLines.get(row_idx).cloned().unwrap_or_default()
                        } else {
                            String::new()
                        };
                        let prefix = if is_header {
                            " ".to_string()
                        } else if let Some(n) = row_number {
                            if Self::zeile_which_zaehlung_py(n) % 2 == 0 {
                                "█".to_string()
                            } else {
                                " ".to_string()
                            }
                        } else {
                            " ".to_string()
                        };
                        line.push_str(&prefix);
                        line.push_str(&format!("{:>width$} ", label, width = num_prefix_width));
                    }

                    for (local_i, abs_i) in (chunk_start..chunk_end).enumerate() {
                        let maybe_part = wrapped_cells[local_i].get(sub_idx).cloned();
                        let part = maybe_part.clone().unwrap_or_default();
                        let is_rest = maybe_part.is_none();
                        let rendered = format!("{:<width$}", part, width = widths[abs_i]);
                        line.push_str(&Self::styled_shell_text_py(
                            &rendered,
                            row_number,
                            is_header,
                            is_rest,
                            self.nocolor,
                        ));
                        if abs_i + 1 != chunk_end {
                            line.push(' ');
                        }
                    }

                    one_chunk_lines.push(line);
                }
            }

            chunked_lines.push(one_chunk_lines.clone());
            out_lines.extend(one_chunk_lines);
        }

        self.finallyDisplayLinesByChunks = chunked_lines;
        self.finallyDisplayLines = out_lines.clone();
        self.numlen = numlen;
        newTable
    }

    pub fn prepareFinallyDisplayLines(&mut self) {
        self.finallyDisplayLines = vec![];
        self.finallyDisplayLinesByChunks = vec![];
        for row in self.__resultingTable.clone() {
            let line = row.join(" ; ");
            self.finallyDisplayLines.push(line.clone());
            self.finallyDisplayLinesByChunks.push(vec![line]);
        }
    }

    pub fn determineNumlen(&mut self) {
        self.numlen = self.__resultingTable.len() as i64;
    }

    pub fn addResultingTableToTables(&mut self) {
        self.tables.push(self.__resultingTable.clone());
    }

    pub fn setOld2Rows(&mut self) {
        self.old2Rows = self.__resultingTable.clone();
    }

    pub fn setNewerTable(&mut self) {
        self.newerTable = self.__resultingTable.clone();
    }

    pub fn setOldRows(&mut self) {
        self.oldRows = self.__resultingTable.clone();
    }

    pub fn setNewerRows(&mut self) {
        self.newerRows = self.__resultingTable.clone();
    }

    pub fn setRowsOfcombi(&mut self) {
        self.rowsOfcombi = self.__resultingTable.clone();
    }

    pub fn setOldTable(&mut self) {
        self.oldTable = self.__resultingTable.clone();
    }

    pub fn setGeneratedSpaltenParameter(&mut self) {
        self.generatedSpaltenParameter = self.sideParas.clone();
    }

    pub fn setAllEquColumns(&mut self) {
        self.allEquColumns = self.spaltenNumbers.clone();
    }

    pub fn setFinallyDisplayTable(&mut self) {
        self.finallyDisplayTable = self.__resultingTable.clone();
    }

    pub fn printOrStoreLines(&mut self) {
        if !self.ifPrint {
            return;
        }
        if self.finallyDisplayLines.len() == 0 && self.cliErrors.len() == 0 {
            self.prepareFinallyDisplayLines();
        }
    }


}
