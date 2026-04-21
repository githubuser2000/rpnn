#![allow(non_snake_case)]

use std::collections::BTreeSet;

use crate::shared::reta_program_types::{dedup_preserve_order_i64, Program};
use crate::reta_runtime_bridge::terminal_width_override;
use crate::shared::words_py::PyValue;

impl Program {
    pub(crate) fn push_set_entries_exact(set_: &mut BTreeSet<i64>, values: Vec<i64>) {
        for v in values {
            set_.insert(v);
        }
    }

    pub(crate) fn pyvalue_list_to_i64_vec(values: &Vec<PyValue>) -> Vec<i64> {
        let mut out = vec![];
        for v in values {
            match v {
                PyValue::Int(n) => out.push(*n),
                PyValue::Bool(b) => out.push(if *b { 1 } else { 0 }),
                PyValue::Tuple(inner) => {
                    for v2 in inner {
                        if let PyValue::Int(n2) = v2 {
                            out.push(*n2);
                        }
                    }
                }
                _ => {}
            }
        }
        out
    }

    pub(crate) fn parse_python_decimal_csv_exact(para_values: &str) -> Vec<i64> {
        let mut out = Vec::new();
        for chosen in para_values.split(',') {
            if chosen.chars().all(|c| c.is_ascii_digit()) {
                if let Ok(n) = chosen.parse::<i64>() {
                    let abs_n = n.abs();
                    if abs_n != 0 && abs_n != 1 {
                        out.push(abs_n);
                    }
                }
            }
        }
        out
    }

    pub(crate) fn lambda_gebr_univ_und_galax_py(para_values: &str) -> BTreeSet<i64> {
        Self::parse_python_decimal_csv_exact(para_values).into_iter().collect()
    }

    pub(crate) fn lambda_prim_galax_py(para_values: &str) -> BTreeSet<i64> {
        Self::parse_python_decimal_csv_exact(para_values)
            .into_iter()
            .filter(|n| Self::primCreativity_py(*n) == 1)
            .collect()
    }

    pub(crate) fn canonical_spalten_main_cli_name_py(cmd: &str) -> &'static str {
        match cmd {
            "primvielfache" | "multiplikationen" => "multiplikationen",
            "gebrochenuniversum2" | "gebrochenuniversum" => "gebrochenuniversum",
            "gebrochengalaxie2" | "gebrochengalaxie" => "gebrochengalaxie",
            "gebrochenemotion2" | "gebrochenemotion" => "gebrochenemotion",
            "gebrochengroesse2" | "gebrochengroesse" => "gebrochengroesse",
            _ => "",
        }
    }

    pub(crate) fn internal_spalten_group_name_py(cmd: &str) -> &'static str {
        match cmd {
            "primvielfache" | "multiplikationen" => "primvielfache",
            "gebrochenuniversum2" | "gebrochenuniversum" => "gebrochenuniversum",
            "gebrochengalaxie2" | "gebrochengalaxie" => "gebrochengalaxie",
            "gebrochenemotion2" | "gebrochenemotion" => "gebrochenemotion",
            "gebrochengroesse2" | "gebrochengroesse" => "gebrochengroesse",
            _ => "",
        }
    }

    pub(crate) fn spalten_main_name_candidates_py(cmd: &str) -> Vec<String> {
        let mut out = Vec::new();
        if !cmd.is_empty() {
            out.push(cmd.to_string());
        }
        let canonical = Self::canonical_spalten_main_cli_name_py(cmd);
        if !canonical.is_empty() && !out.iter().any(|s| s == canonical) {
            out.push(canonical.to_string());
        }
        let internal = Self::internal_spalten_group_name_py(cmd);
        if !internal.is_empty() && !out.iter().any(|s| s == internal) {
            out.push(internal.to_string());
        }
        out
    }

    pub(crate) fn parameter_main_name_matches_py(stored: &str, cmd: &str) -> bool {
        for candidate in Self::spalten_main_name_candidates_py(cmd) {
            if stored == candidate
                || stored.contains(&format!("'{}'", candidate))
                || stored.contains(&format!("[\"{}\"]", candidate))
                || stored.contains(&candidate)
            {
                return true;
            }
        }
        false
    }

    pub(crate) fn resultingSpaltenFromTuple_py(&mut self, tupl: &Vec<Vec<PyValue>>, neg: &str, paraValue: Option<&str>, befehlName: Option<&str>) {
        for (i, raw_values) in tupl.iter().enumerate() {
            let mut normalized_values: Vec<i64> = Vec::new();
            if !raw_values.is_empty() {
                match &raw_values[0] {
                    PyValue::Bool(_) => {
                        normalized_values = Self::pyvalue_list_to_i64_vec(raw_values);
                    }
                    PyValue::Tuple(inner) => {
                        normalized_values = Self::pyvalue_list_to_i64_vec(inner);
                    }
                    _ => {
                        normalized_values = Self::pyvalue_list_to_i64_vec(raw_values);
                    }
                }
            }

            let befehl_raw = befehlName.unwrap_or("");
            let befehl = {
                let canonical = Self::canonical_spalten_main_cli_name_py(befehl_raw);
                if canonical.is_empty() { befehl_raw } else { canonical }
            };
            let para = paraValue.unwrap_or("");

            let gebr_idx = match befehl {
                "multiplikationen" => Some(2usize),
                "gebrochenuniversum" | "gebrochenuniversum2" => Some(5usize),
                "gebrochengalaxie" | "gebrochengalaxie2" => Some(6usize),
                "gebrochenemotion" | "gebrochenemotion2" => Some(9usize),
                "gebrochengroesse" | "gebrochengroesse2" => Some(10usize),
                _ => None,
            };

            if i == 2 && (!raw_values.is_empty() || gebr_idx.is_some()) {
                if let Some(target_idx) = gebr_idx {
                    let generated = if befehl == "multiplikationen" {
                        Self::lambda_prim_galax_py(para)
                    } else {
                        Self::lambda_gebr_univ_und_galax_py(para)
                    };
                    if let Some(target) = self.spaltenArtenKey_SpaltennummernValue.get_mut(&(neg.len(), target_idx)) {
                        *target = target.union(&generated).cloned().collect();
                    }
                    continue;
                }
            }

            if para == "beschrieben" && (befehl.contains("prim") || befehl == "multiplikationen") {
                if let Some(target) = self.spaltenArtenKey_SpaltennummernValue.get_mut(&(neg.len(), 2)) {
                    target.insert(2);
                }
                continue;
            }

            if normalized_values.is_empty() {
                continue;
            }
            if let Some(target) = self.spaltenArtenKey_SpaltennummernValue.get_mut(&(neg.len(), i)) {
                Self::push_set_entries_exact(target, normalized_values);
            }
        }
    }

    pub(crate) fn spalten_removeDoublesNthenRemoveOneFromAnother_py(&mut self) {
        let half_len = self.spaltenArtenKey_SpaltennummernValue.len() / 2;
        for el2_type in 0..half_len {
            let pos = self.spaltenArtenKey_SpaltennummernValue.get(&(0, el2_type)).cloned().unwrap_or_default();
            let neg = self.spaltenArtenKey_SpaltennummernValue.get(&(1, el2_type)).cloned().unwrap_or_default();
            let mut result = pos.clone();
            for v in pos.intersection(&neg) {
                result.remove(v);
            }
            self.spaltenArtenKey_SpaltennummernValue.insert((0, el2_type), result);
        }
        for el2_type in 0..half_len {
            let neg = self.spaltenArtenKey_SpaltennummernValue.shift_remove(&(1, el2_type)).unwrap_or_default();
            let mut pos = self.spaltenArtenKey_SpaltennummernValue.get(&(0, el2_type)).cloned().unwrap_or_default();
            for v in neg {
                pos.remove(&v);
            }
            self.spaltenArtenKey_SpaltennummernValue.insert((0, el2_type), pos);
        }
    }

    pub(crate) fn deleteDoublesInSets_py(&self, pos: Vec<String>, neg: Vec<String>) -> (Vec<String>, Vec<String>) {
        let negset: BTreeSet<String> = neg.iter().cloned().collect();
        let mut pos2 = Vec::new();
        for p in pos {
            if !negset.contains(&p) {
                pos2.push(p);
            }
        }
        (pos2, neg)
    }

    pub(crate) fn ordered_set_to_vec_i64(set_: BTreeSet<i64>) -> Vec<i64> {
        dedup_preserve_order_i64(set_.into_iter().collect())
    }

    pub(crate) fn ordered_set_to_onlyGenerated_py(set_: BTreeSet<i64>) -> Vec<Vec<i64>> {
        let mut out: Vec<Vec<i64>> = Vec::new();
        let flat: Vec<i64> = dedup_preserve_order_i64(set_.into_iter().collect());
        for v in flat {
            out.push(vec![v]);
        }
        out
    }

    pub(crate) fn language_parameter_value_py(cmd_without_dash: &str) -> Option<&str> {
        cmd_without_dash
            .strip_prefix("language=")
            .or_else(|| cmd_without_dash.strip_prefix("sprache="))
    }

    pub(crate) fn is_known_language_value_py(lang: &str) -> bool {
        matches!(
            lang,
            "english"
                | "englisch"
                | "deutsch"
                | "german"
                | "vietnamesisch"
                | "vietnamese"
                | "tiếngviệt"
                | "chinesisch"
                | "chinese"
                | "中國人"
                | "koreanisch"
                | "korean"
                | "한국인"
                // Keep the short Rust-era spellings accepted as aliases, but
                // prefer the Python spellings above in generated tests.
                | "de"
                | "en"
                | "cn"
                | "kr"
                | "vn"
        )
    }

    pub fn produceAllSpaltenNumbers(&mut self, neg: &str) -> Vec<i64> {
        if self.spaltenArtenKey_SpaltennummernValue.is_empty() {
            self.init_spalten_arten_python_like();
        }
        self.mainParaCmds.clear();
        self.mainParaCmds.insert("zeilen".to_string(), 0);
        self.mainParaCmds.insert("spalten".to_string(), 1);
        self.mainParaCmds.insert("kombination".to_string(), 2);
        self.mainParaCmds.insert("ausgabe".to_string(), 3);
        self.mainParaCmds.insert("debug".to_string(), -1);
        self.mainParaCmds.insert("h".to_string(), -1);
        self.mainParaCmds.insert("help".to_string(), -1);

        let mut last_main_cmd: i64 = -1;
        let argv_without_program_snapshot = self.argvWithoutProgram.clone();
        for cmd_ref in argv_without_program_snapshot {
            let mut cmd = cmd_ref;
            if cmd.len() > 1 && cmd.starts_with('-') && !cmd.starts_with("--") {
                let plain = cmd[1..].to_string();
                if let Some(v) = self.mainParaCmds.get(&plain) {
                    last_main_cmd = *v;
                } else if plain == "nichts" || plain == "nothing" {
                } else if let Some(lang) = Self::language_parameter_value_py(&plain) {
                    if !Self::is_known_language_value_py(lang) && neg.is_empty() {
                        self.cliErrors.push("wrongLangSentence".to_string());
                    }
                } else if neg.is_empty() {
                    self.cliErrors.push(format!(
                        "Es muss ein Hauptparameter, bzw. der richtige, gesetzt sein, damit ein Nebenparameter, wie möglicherweise: \"{}\" ausgeführt werden kann. Hauptparameter sind: -zeilen -spalten -kombination -ausgabe -debug -h -help",
                        cmd
                    ));
                }
            } else if cmd.starts_with("--") {
                if last_main_cmd == 1 {
                    cmd = cmd[2..].to_string();
                    let eq = cmd.find('=');
                    if self.breiteBreitenSysArgvPara(&cmd, neg) {
                    } else if cmd == "keinenummerierung" && neg.is_empty() {
                        self.nummeriere = false;
                    } else if let Some(eq) = eq {
                        let left_raw = cmd[..eq].to_string();
                        let left = {
                            let canonical = Self::canonical_spalten_main_cli_name_py(&left_raw);
                            if canonical.is_empty() {
                                left_raw.clone()
                            } else {
                                canonical.to_string()
                            }
                        };
                        let right = cmd[eq + 1..].to_string();
                        for mut one in right.split(',').map(|s| s.to_string()) {
                            let yes1 = if !one.is_empty() && one.starts_with('-') {
                                one = one[1..].to_string();
                                neg == "-"
                            } else {
                                neg.is_empty()
                            };
                            if yes1 {
                                let mut found_exact = false;
                                for candidate in Self::spalten_main_name_candidates_py(&left_raw) {
                                    if let Some(tupl) = self.paraDict.get(&(candidate.clone(), one.clone())).cloned() {
                                        self.resultingSpaltenFromTuple_py(&tupl, neg, Some(&one), Some(&left));
                                        found_exact = true;
                                        break;
                                    }
                                }
                                if !found_exact {
                                    let matching_tuples: Vec<Vec<Vec<PyValue>>> = self
                                        .paraDict
                                        .iter()
                                        .filter_map(|((k1, k2), tupl)| {
                                            if Self::parameter_main_name_matches_py(k1, &left_raw) && k2 == &one {
                                                Some(tupl.clone())
                                            } else {
                                                None
                                            }
                                        })
                                        .collect();
                                    for tupl in matching_tuples {
                                        self.resultingSpaltenFromTuple_py(&tupl, neg, Some(&one), Some(&left));
                                    }
                                }
                            }
                        }
                    } else {
                        let cmd_raw_with_possible_neg_suffix = cmd.clone();
                        let has_neg_suffix = cmd_raw_with_possible_neg_suffix.ends_with('-');
                        let should_apply_like_python = !cmd_raw_with_possible_neg_suffix.is_empty()
                            && ((has_neg_suffix && neg == "-") != (neg.is_empty() && !has_neg_suffix));

                        if should_apply_like_python {
                            let cmd_raw = if has_neg_suffix && !neg.is_empty() {
                                cmd_raw_with_possible_neg_suffix[..cmd_raw_with_possible_neg_suffix.len() - 1].to_string()
                            } else {
                                cmd_raw_with_possible_neg_suffix
                            };
                            let cmd_canonical = {
                                let canonical = Self::canonical_spalten_main_cli_name_py(&cmd_raw);
                                if canonical.is_empty() {
                                    cmd_raw.clone()
                                } else {
                                    canonical.to_string()
                                }
                            };
                            let matching_tuples: Vec<Vec<Vec<PyValue>>> = self
                                .paraDict
                                .iter()
                                .filter_map(|((k1, k2), tupl)| {
                                    if Self::parameter_main_name_matches_py(k1, &cmd_raw) && k2.is_empty() {
                                        Some(tupl.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            for tupl in matching_tuples {
                                self.resultingSpaltenFromTuple_py(&tupl, neg, None, Some(&cmd_canonical));
                            }
                        }
                    }
                } else if last_main_cmd == 2 {
                    let gal_prefix = "--galaxie=";
                    let uni_prefix = "--universum=";
                    let (right, target_kind, use_second_reverse) = if let Some(tail) = cmd.strip_prefix(gal_prefix) {
                        (tail, 3usize, false)
                    } else if let Some(tail) = cmd.strip_prefix(uni_prefix) {
                        (tail, 8usize, true)
                    } else {
                        ("", usize::MAX, false)
                    };

                    if target_kind != usize::MAX {
                        for mut one_kombi_spalte in right.split(',').map(|part| part.to_string()) {
                            let yes1 = if !one_kombi_spalte.is_empty() && one_kombi_spalte.starts_with('-') {
                                one_kombi_spalte = one_kombi_spalte[1..].to_string();
                                neg == "-"
                            } else {
                                neg.is_empty()
                            };
                            if !yes1 {
                                continue;
                            }

                            let maybe_value = if use_second_reverse {
                                self.kombiReverseDict2.get(&one_kombi_spalte).copied()
                            } else {
                                self.kombiReverseDict.get(&one_kombi_spalte).copied()
                            };
                            if let Some(value) = maybe_value {
                                self.spaltenArtenKey_SpaltennummernValue
                                    .entry((neg.len(), target_kind))
                                    .or_default()
                                    .insert(value);
                            }
                        }
                    } else if neg.is_empty() {
                        self.cliErrors.push(format!("{} ist kein gueltiger Nebenparameter fuer -kombination", cmd));
                    }
                }
            }
        }

        if neg.is_empty() {
            self.produceAllSpaltenNumbers("-");
            self.spalten_removeDoublesNthenRemoveOneFromAnother_py();
        }

        let spalten_numbers = Self::ordered_set_to_vec_i64(
            self.spaltenArtenKey_SpaltennummernValue
                .get(&(0, 0))
                .cloned()
                .unwrap_or_default(),
        );
        self.spaltenNumbers = spalten_numbers.clone();
        self.rowsAsNumbers = spalten_numbers.clone();
        self.generRows = Self::ordered_set_to_vec_i64(
            self.spaltenArtenKey_SpaltennummernValue
                .get(&(0, 1))
                .cloned()
                .unwrap_or_default(),
        );
        self.puniverseprims = Self::ordered_set_to_vec_i64(
            self.spaltenArtenKey_SpaltennummernValue
                .get(&(0, 2))
                .cloned()
                .unwrap_or_default(),
        );
        self.rowsOfcombi2 = Self::ordered_set_to_vec_i64(
            self.spaltenArtenKey_SpaltennummernValue
                .get(&(0, 8))
                .cloned()
                .unwrap_or_default(),
        );
        self.onlyGenerated = Self::ordered_set_to_onlyGenerated_py(
            self.spaltenArtenKey_SpaltennummernValue
                .get(&(0, 4))
                .cloned()
                .unwrap_or_default(),
        );
        spalten_numbers
    }

    pub fn breiteBreitenSysArgvPara(&mut self, cmd: &str, neg: &str) -> bool {
        if let Some(tail) = cmd.strip_prefix("breite=") {
            self.setShellRowsAmount();
            if self.breiteHasBeenOnceZero {
                self.shellRowsAmount = 0;
                self.set_text_width_property_py(0);
                self.breiteORbreiten = true;
                return true;
            }
            if tail.chars().all(|c| c.is_ascii_digit()) {
                let mut breite = tail.parse::<i64>().unwrap_or(0).abs();
                if breite == 0 {
                    self.breiteHasBeenOnceZero = true;
                    self.shellRowsAmount = 0;
                } else if self.shellRowsAmount > 7 && breite > self.shellRowsAmount - 7 {
                    breite = self.shellRowsAmount - 7;
                }
                self.breite = breite;
                let new_text_width = if breite > self.textWidth {
                    breite
                } else {
                    self.textWidth
                };
                self.set_text_width_property_py(new_text_width);
                self.breiteORbreiten = true;
            }
            return true;
        }
        if let Some(tail) = cmd.strip_prefix("breiten=") {
            if neg.is_empty() {
                let mut parsed_breiten: Vec<i64> = vec![];
                for breite in tail.split(',') {
                    if breite.trim().chars().all(|c| c.is_ascii_digit()) {
                        parsed_breiten.push(breite.trim().parse::<i64>().unwrap_or(0));
                        self.breiteORbreiten = true;
                    }
                }
                self.breiten = self.normalize_breiten_list_py(parsed_breiten);
            }
            return true;
        }
        false
    }

    fn normalize_text_width_py(&self, value: i64) -> i64 {
        let shell_width = Self::detect_terminal_columns_py();
        if (shell_width > value + 7 || shell_width == 0)
            && (value != 0 || self.outType == "bbcode" || self.outType == "html" || self.oneTable)
        {
            value
        } else {
            shell_width - 7
        }
    }

    pub(crate) fn set_text_width_property_py(&mut self, value: i64) {
        self.textWidth = self.normalize_text_width_py(value);
    }

    fn normalize_breite_value_py(shell_width: i64, value: i64) -> i64 {
        if shell_width > value + 7 || shell_width == 0 {
            value
        } else {
            shell_width - 7
        }
    }

    pub(crate) fn normalize_breiten_list_py(&self, values: Vec<i64>) -> Vec<i64> {
        let shell_width = Self::detect_terminal_columns_py();
        values
            .into_iter()
            .map(|value| Self::normalize_breite_value_py(shell_width, value))
            .collect()
    }

    pub fn setShellRowsAmount(&mut self) {
        self.shellRowsAmount = Self::detect_terminal_columns_py();
    }

    pub(crate) fn detect_terminal_columns_py() -> i64 {
        if let Some(width) = terminal_width_override() {
            return width;
        }

        if let Ok(v) = std::env::var("COLUMNS") {
            if let Ok(n) = v.trim().parse::<i64>() {
                if n > 0 {
                    return n;
                }
            }
        }

        let try_cmd = |cmd: &str| -> Option<i64> {
            let output = std::process::Command::new("sh")
                .arg("-lc")
                .arg(cmd)
                .output()
                .ok()?;
            if !output.status.success() {
                return None;
            }
            let txt = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let n = txt.parse::<i64>().ok()?;
            if n > 0 { Some(n) } else { None }
        };

        if let Some(n) = try_cmd("stty size < /dev/tty 2>/dev/null | awk '{print $2}'") {
            return n;
        }
        if let Some(n) = try_cmd("tput cols 2>/dev/null") {
            return n;
        }
        80
    }

    pub fn setShellWidth(&mut self) {
        self.shellWidth = Self::detect_terminal_columns_py();
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produce_all_spalten_numbers_accepts_python_language_parameter() {
        let mut program = Program::new(vec!["reta".to_string(), "-language=english".to_string()]);
        program.produceAllSpaltenNumbers("");
        assert!(program.cliErrors.is_empty(), "valid -language=english must be ignored like Python");
    }

    #[test]
    fn produce_all_spalten_numbers_rejects_unknown_language_like_python() {
        let mut program = Program::new(vec!["reta".to_string(), "-language=xx".to_string()]);
        program.produceAllSpaltenNumbers("");
        assert_eq!(program.cliErrors, vec!["wrongLangSentence".to_string()]);
    }
}
