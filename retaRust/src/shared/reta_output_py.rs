#![allow(non_snake_case)]

use std::collections::BTreeSet;

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
        let original_lines: BTreeSet<i64> = (0..=max_row).collect();
        let mut finally_display_set = self.filter_original_lines_py(original_lines.clone(), &paramLines, max_row);
        if !paramLinesNot.is_empty() {
            let finally_display_not = self.filter_original_lines_py(finally_display_set.clone(), &paramLinesNot, max_row);
            let has_anything_changed: BTreeSet<i64> = original_lines.difference(&finally_display_not).copied().filter(|n| *n != 0).collect();
            if !has_anything_changed.is_empty() {
                finally_display_set = finally_display_set.difference(&finally_display_not).copied().collect();
            }
        }

        if finally_display_set.is_empty() {
            if self.ifZeilenSetted {
                finally_display_set = BTreeSet::new();
            } else {
                finally_display_set = (0..=max_row).collect();
            }
        }

        if !self.keineUeberschriften {
            finally_display_set.insert(0);
        }

        let selected_rows: Vec<i64> = finally_display_set.iter().copied().collect();

        let mut selected_cols: Vec<i64> = if rowsAsNumbers.is_empty() {
            if relitable[0].is_empty() { vec![] } else { (0..(relitable[0].len() as i64)).collect() }
        } else {
            rowsAsNumbers.clone()
        };
        selected_cols = dedup_preserve_order_i64(selected_cols);
        let selected_cols_set: BTreeSet<i64> = selected_cols.iter().cloned().collect();

        for row_no in &selected_rows {
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

        finallyDisplayLines = old2newTable.iter().map(|n| n.to_string()).collect();
        if !finallyDisplayLines.is_empty() && !self.keineUeberschriften {
            finallyDisplayLines[0] = "".to_string();
        }

        let rowsRange: Vec<i64> = if newTable.is_empty() { vec![] } else { (0..(newTable[0].len() as i64)).collect() };
        let numlen = finallyDisplayLines.iter().filter(|s| !s.is_empty()).map(|s| s.len() as i64).max().unwrap_or(0);
        (finallyDisplayLines, newTable, numlen, rowsRange, old2newTable)
    }

    fn cutset_py(whether: bool, a: &BTreeSet<i64>, b: &BTreeSet<i64>) -> BTreeSet<i64> {
        if whether {
            return a.intersection(b).copied().collect();
        }
        a.clone()
    }

    fn prim_fak_py(mut n: i64) -> Vec<i64> {
        let mut out = Vec::new();
        if n < 2 { return out; }
        while n % 2 == 0 {
            out.push(2);
            n /= 2;
        }
        let mut d = 3;
        while d * d <= n {
            while n % d == 0 {
                out.push(d);
                n /= d;
            }
            d += 2;
        }
        if n > 1 { out.push(n); }
        out
    }

    fn prim_repeat2_py(n: &[i64]) -> Vec<(i64, i64)> {
        let mut out: Vec<(i64, i64)> = Vec::new();
        for &a in n {
            if let Some(last) = out.last_mut() {
                if last.0 == a {
                    last.1 += 1;
                    continue;
                }
            }
            out.push((a, 1));
        }
        out
    }

    fn is_prim_multiple_py(is_it: i64, multiples1: &[i64]) -> bool {
        let mut multiples2: Vec<(i64, i64)> = vec![(1, is_it)];
        for prim in Self::prim_repeat2_py(&Self::prim_fak_py(is_it)) {
            multiples2.push((prim.0, is_it / prim.0));
        }
        for multiple1 in multiples1 {
            for multiple2 in &multiples2 {
                if *multiple1 == multiple2.1 {
                    return true;
                }
            }
        }
        false
    }

    fn filter_original_lines_py(&self, mut num_range: BTreeSet<i64>, param_lines: &[String], max_row: i64) -> BTreeSet<i64> {
        num_range.remove(&0);
        let has_non_ka = param_lines.iter().any(|p| p != "ka" && p != "ka2");
        if param_lines.iter().any(|p| p == "all") || !has_non_ka || !self.ifZeilenSetted {
            num_range = (1..=max_row).collect();
        } else {
            num_range.clear();
        }

        let mut if_a_at_all = false;
        let mut mehrere: Vec<String> = vec![];
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
            for v in Self::bereich_to_numbers2_py(&joined, false, max_row + 1, false) {
                if v <= max_row { num_range.insert(v); }
            }
            if if_teiler {
                for v in Self::teiler_py(&num_range.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",")) {
                    if v <= max_row { num_range.insert(v); }
                }
            }
            if !num_range.is_empty() {
                for eins0 in joined.split(',') {
                    let mut eins = eins0.to_string();
                    let ja1 = eins.starts_with('-');
                    let ja2 = eins.starts_with("v-");
                    if ja1 || ja2 {
                        if ja1 {
                            eins = eins[1..].to_string();
                        }
                        if ja2 {
                            eins = format!("v{}", &eins[2..]);
                        }
                        for v in Self::bereich_to_numbers2_py(&eins, false, max_row + 1, false) {
                            num_range.remove(&v);
                        }
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
            for v in Self::bereich_to_numbers2_py(&mehrere.join(","), true, max_row + 1, false) {
                if v <= max_row { num_range_yes_z.insert(v); }
            }
            if !num_range_yes_z.is_empty() {
                num_range = num_range.intersection(&num_range_yes_z).copied().collect();
            }
            if !num_range.is_empty() {
                for eins0 in mehrere.join(",").split(',') {
                    let mut eins = eins0.to_string();
                    let ja1 = eins.starts_with('-');
                    let ja2 = eins.starts_with("v-");
                    if ja1 || ja2 {
                        if ja1 { eins = eins[1..].to_string(); }
                        if ja2 { eins = format!("v{}", &eins[2..]); }
                        for v in Self::bereich_to_numbers2_py(&eins, true, max_row + 1, false) {
                            num_range.remove(&v);
                        }
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
                for n in 1..=std::cmp::min(9, max_row) { num_range_yes_z.insert(n); }
            } else if condition == ">" {
                if_zeit_at_all = true;
                for n in 11..=max_row { num_range_yes_z.insert(n); }
            }
        }
        if if_zeit_at_all {
            if num_range.is_empty() && !if_b_at_all && !if_a_at_all && !param_lines.iter().any(|p| p == "all") && num_range_yes_z.is_empty() {
                num_range = (1..=max_row).collect();
            }
            if if_a_at_all || if_b_at_all || param_lines.iter().any(|p| p == "all") {
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
                for v in Self::bereich_to_numbers2_py(&condition[3..], false, max_row + 1, false) {
                    num_range_yes_z.insert(v);
                }
                if_zaehlungen_at_all = true;
                mehrere.push(condition[3..].to_string());
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
                for eins0 in mehrere.join(",").split(',') {
                    let mut eins = eins0.to_string();
                    let ja1 = eins.starts_with('-');
                    let ja2 = eins.starts_with("v-");
                    if ja1 || ja2 {
                        if ja1 { eins = eins[1..].to_string(); }
                        if ja2 { eins = format!("v{}", &eins[2..]); }
                        for v in Self::bereich_to_numbers2_py(&eins, false, max_row + 1, false) {
                            minus_bereiche.insert(v);
                        }
                    }
                }
                if !minus_bereiche.is_empty() {
                    let snapshot: Vec<i64> = num_range.iter().copied().collect();
                    for n in snapshot {
                        for z in &minus_bereiche {
                            if Self::zeile_which_zaehlung_py(n) == *z {
                                num_range.remove(&n);
                            }
                        }
                    }
                }
            }
        }

        let mut if_typ_at_all = false;
        num_range_yes_z.clear();
        if num_range.is_empty() && has_non_ka {
            num_range = (1..=max_row).collect();
        }

        if param_lines.iter().any(|p| ["aussenerste","innenerste","aussenalle","innenalle"].contains(&p.as_str())) {
            use std::collections::BTreeMap;
            let mut innen_aussen: BTreeMap<i64, (bool, bool, bool)> = BTreeMap::new();
            innen_aussen.insert(1, (true, false, true));
            for n in num_range.iter().copied().filter(|n| *n > 3) {
                let primzahlen = Self::prim_fak_py(n);
                let nur_eine = primzahlen.len() == 1;
                let mut innen = false;
                let mut aussen = false;
                for prim in primzahlen {
                    if prim >= 4 {
                        let r = prim % 6;
                        innen = innen || r == 1;
                        aussen = aussen || r == 5;
                    }
                }
                innen_aussen.insert(n, (innen, aussen, nur_eine));
            }
            if param_lines.iter().any(|p| p == "aussenerste") {
                for (n, tup) in &innen_aussen { if tup.0 && tup.2 { num_range_yes_z.insert(*n); } }
            }
            if param_lines.iter().any(|p| p == "innenerste") {
                for (n, tup) in &innen_aussen { if tup.1 && tup.2 { num_range_yes_z.insert(*n); } }
            }
            if param_lines.iter().any(|p| p == "aussenalle") {
                for (n, tup) in &innen_aussen { if tup.0 { num_range_yes_z.insert(*n); } }
            }
            if param_lines.iter().any(|p| p == "innenalle") {
                for (n, tup) in &innen_aussen { if tup.1 { num_range_yes_z.insert(*n); } }
            }
            if_typ_at_all = !num_range_yes_z.is_empty();
            num_range = Self::cutset_py(if_typ_at_all, &num_range, &num_range_yes_z);
        }

        if_typ_at_all = false;
        num_range_yes_z.clear();
        if num_range.is_empty() && has_non_ka {
            num_range = (1..=max_row).collect();
        }
        for condition in param_lines {
            if condition.contains("mond") {
                if_typ_at_all = true;
                for n in &num_range { if Self::moon_number_is_py(*n) { num_range_yes_z.insert(*n); } }
            } else if condition == "schwarzesonne" {
                if_typ_at_all = true;
                for n in &num_range { if *n % 3 == 0 { num_range_yes_z.insert(*n); } }
            } else if condition == "sonne" {
                if_typ_at_all = true;
                for n in &num_range { if !Self::moon_number_is_py(*n) { num_range_yes_z.insert(*n); } }
            } else if condition == "planet" {
                if_typ_at_all = true;
                for n in &num_range { if *n % 2 == 0 { num_range_yes_z.insert(*n); } }
            } else if condition == "SonneMitMondanteil" {
                if_typ_at_all = true;
                for n in &num_range {
                    let repeats = Self::prim_repeat2_py(&Self::prim_fak_py(*n));
                    let mut has_true = false;
                    let mut has_false = false;
                    for (_prim, faktor) in repeats {
                        if faktor == 1 { has_true = true; } else { has_false = true; }
                    }
                    if has_true && has_false { num_range_yes_z.insert(*n); }
                }
            }
        }
        num_range = Self::cutset_py(if_typ_at_all, &num_range, &num_range_yes_z);

        let mut prim_multiples: Vec<i64> = vec![];
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
            }
        }
        let to_power_it = Self::bereich_to_numbers2_py(&mehrere.join(","), false, 0, false);
        if if_power_at_all {
            num_range_yes_z.clear();
            if num_range.is_empty() && has_non_ka {
                num_range = (1..=max_row).collect();
            }
            if !num_range.is_empty() {
                let last_el = *num_range.iter().max().unwrap_or(&0);
                for base in &to_power_it {
                    let mut n = 0u32;
                    loop {
                        let one_power = (*base).pow(n);
                        if one_power > last_el { break; }
                        num_range_yes_z.insert(one_power);
                        n += 1;
                    }
                }
                num_range = Self::cutset_py(if_power_at_all, &num_range, &num_range_yes_z);
                num_range.remove(&1);
            }
        }

        let mut if_multiples_from_any_at_all = false;
        let mut any_multiples: Vec<i64> = vec![];
        for condition in param_lines {
            if condition.len() > 1 && condition.ends_with('v') && condition[..condition.len()-1].chars().all(|c| c.is_ascii_digit()) {
                if_multiples_from_any_at_all = true;
                if let Ok(v) = condition[..condition.len()-1].parse::<i64>() { any_multiples.push(v); }
            }
        }
        if if_multiples_from_any_at_all {
            num_range_yes_z.clear();
            for n in &num_range {
                for divisor in &any_multiples {
                    if *divisor != 0 && *n % *divisor == 0 { num_range_yes_z.insert(*n); }
                }
            }
            num_range = Self::cutset_py(if_multiples_from_any_at_all, &num_range, &num_range_yes_z);
        }

        let cutoff114 = std::cmp::min(114, max_row);
        let snapshot: Vec<i64> = num_range.iter().copied().collect();
        for n in snapshot {
            if !Self::moon_number_is_py(n) && n > cutoff114 {
                num_range.remove(&n);
            }
        }

        let invertieren = param_lines.iter().any(|condition| condition.starts_with("_i_"));
        if invertieren {
            let mut num_range2 = BTreeSet::new();
            for i in 1..=max_row {
                if (num_range.contains(&(i + 1)) || num_range.contains(&(i - 1))) && !num_range.contains(&i) {
                    num_range2.insert(i);
                }
            }
            num_range = num_range2;
        }

        let num_range_list: Vec<i64> = num_range.iter().copied().collect();
        let mut num_range2map = std::collections::BTreeMap::new();
        for (i, a) in num_range_list.iter().enumerate() {
            num_range2map.insert((i + 1) as i64, *a);
        }
        let mut z_ja = false;
        let mut num_range_neu2: BTreeSet<i64> = BTreeSet::new();
        for condition in param_lines {
            if condition.starts_with("_z_") && condition.len() > 3 {
                z_ja = true;
                let keys: BTreeSet<i64> = num_range2map.keys().copied().collect();
                let nums: BTreeSet<i64> = Self::bereich_to_numbers2_py(&condition[3..], false, max_row + 1, false).into_iter().collect();
                let intersection: Vec<i64> = keys.intersection(&nums).copied().collect();
                for a in intersection { if let Some(v) = num_range2map.get(&a) { num_range_neu2.insert(*v); } }
            }
        }
        if z_ja { num_range = num_range.intersection(&num_range_neu2).copied().collect(); }

        let mut y_ja = false;
        num_range_neu2.clear();
        for condition in param_lines {
            if condition.starts_with("_y_") && condition.len() > 3 {
                y_ja = true;
                let keys: BTreeSet<i64> = num_range2map.keys().copied().collect();
                let nums: BTreeSet<i64> = Self::bereich_to_numbers2_py(&condition[3..], true, max_row + 1, false).into_iter().collect();
                let intersection: Vec<i64> = keys.intersection(&nums).copied().collect();
                for a in intersection { if let Some(v) = num_range2map.get(&a) { num_range_neu2.insert(*v); } }
            }
        }
        if y_ja { num_range = num_range.intersection(&num_range_neu2).copied().collect(); }

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
