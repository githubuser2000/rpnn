use std::collections::BTreeSet;

use crate::shared::reta_program_types::{dedup_preserve_order_i64, Program};
pub use crate::shared::reta_program_types::{PairStr, SpaltenTyp};
use crate::shared::words_py::{Words, PyValue, StoreParameterEntry};

impl Program {
    fn push_set_entries_exact(set_: &mut BTreeSet<i64>, values: Vec<i64>) {
        for v in values {
            set_.insert(v);
        }
    }

    fn pyvalue_list_to_i64_vec(values: &Vec<PyValue>) -> Vec<i64> {
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

    fn parse_python_decimal_csv_exact(para_values: &str) -> Vec<i64> {
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

    fn lambda_gebr_univ_und_galax_py(para_values: &str) -> BTreeSet<i64> {
        Self::parse_python_decimal_csv_exact(para_values).into_iter().collect()
    }

    fn lambda_prim_galax_py(para_values: &str) -> BTreeSet<i64> {
        Self::parse_python_decimal_csv_exact(para_values)
            .into_iter()
            .filter(|n| Self::primCreativity_py(*n) == 1)
            .collect()
    }

    fn canonical_spalten_main_cli_name_py(cmd: &str) -> &'static str {
        match cmd {
            "primvielfache" | "multiplikationen" => "multiplikationen",
            "gebrochenuniversum2" | "gebrochenuniversum" => "gebrochenuniversum",
            "gebrochengalaxie2" | "gebrochengalaxie" => "gebrochengalaxie",
            "gebrochenemotion2" | "gebrochenemotion" => "gebrochenemotion",
            "gebrochengroesse2" | "gebrochengroesse" => "gebrochengroesse",
            _ => "",
        }
    }

    fn internal_spalten_group_name_py(cmd: &str) -> &'static str {
        match cmd {
            "primvielfache" | "multiplikationen" => "primvielfache",
            "gebrochenuniversum2" | "gebrochenuniversum" => "gebrochenuniversum",
            "gebrochengalaxie2" | "gebrochengalaxie" => "gebrochengalaxie",
            "gebrochenemotion2" | "gebrochenemotion" => "gebrochenemotion",
            "gebrochengroesse2" | "gebrochengroesse" => "gebrochengroesse",
            _ => "",
        }
    }

    fn spalten_main_name_candidates_py(cmd: &str) -> Vec<String> {
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

    fn parameter_main_name_matches_py(stored: &str, cmd: &str) -> bool {
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

    fn resultingSpaltenFromTuple_py(&mut self, tupl: &Vec<Vec<PyValue>>, neg: &str, paraValue: Option<&str>, befehlName: Option<&str>) {
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

    fn spalten_removeDoublesNthenRemoveOneFromAnother_py(&mut self) {
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

    fn deleteDoublesInSets_py(&self, pos: Vec<String>, neg: Vec<String>) -> (Vec<String>, Vec<String>) {
        let negset: BTreeSet<String> = neg.iter().cloned().collect();
        let mut pos2 = Vec::new();
        for p in pos {
            if !negset.contains(&p) {
                pos2.push(p);
            }
        }
        (pos2, neg)
    }

    fn ordered_set_to_vec_i64(set_: BTreeSet<i64>) -> Vec<i64> {
        dedup_preserve_order_i64(set_.into_iter().collect())
    }

    fn ordered_set_to_onlyGenerated_py(set_: BTreeSet<i64>) -> Vec<Vec<i64>> {
        let mut out: Vec<Vec<i64>> = Vec::new();
        let flat: Vec<i64> = dedup_preserve_order_i64(set_.into_iter().collect());
        for v in flat {
            out.push(vec![v]);
        }
        out
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
        for cmd_ref in &self.argvWithoutProgram {
            let mut cmd = cmd_ref.clone();
            if cmd.len() > 1 && cmd.starts_with('-') && !cmd.starts_with("--") {
                let plain = cmd[1..].to_string();
                if let Some(v) = self.mainParaCmds.get(&plain) {
                    last_main_cmd = *v;
                } else if plain == "nichts" || plain == "nothing" {
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
                                    for ((k1, k2), tupl) in self.paraDict.iter() {
                                        if Self::parameter_main_name_matches_py(k1, &left_raw) && k2 == &one {
                                            self.resultingSpaltenFromTuple_py(tupl, neg, Some(&one), Some(&left));
                                        }
                                    }
                                }
                            }
                        }
                    } else if neg.is_empty() {
                        let cmd_raw = cmd.clone();
                        let cmd_canonical = {
                            let canonical = Self::canonical_spalten_main_cli_name_py(&cmd_raw);
                            if canonical.is_empty() {
                                cmd_raw.clone()
                            } else {
                                canonical.to_string()
                            }
                        };
                        for ((k1, k2), tupl) in self.paraDict.iter() {
                            if Self::parameter_main_name_matches_py(k1, &cmd_raw) && k2.is_empty() {
                                self.resultingSpaltenFromTuple_py(tupl, neg, None, Some(&cmd_canonical));
                            }
                        }
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
}
