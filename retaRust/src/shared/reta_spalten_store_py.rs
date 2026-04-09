use crate::shared::reta_program_types::{dedup_preserve_order_i64, Program};
use crate::shared::words_py::{PyValue, StoreParameterEntry, Words};
use std::collections::BTreeSet;

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
        for mut cmd in self.argvWithoutProgram.clone() {
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
                                    for ((k1, k2), tupl) in self.paraDict.clone().into_iter() {
                                        if Self::parameter_main_name_matches_py(&k1, &left_raw) && k2 == one {
                                            self.resultingSpaltenFromTuple_py(&tupl, neg, Some(&one), Some(&left));
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
                        for ((k1, k2), tupl) in self.paraDict.clone().into_iter() {
                            if Self::parameter_main_name_matches_py(&k1, &cmd_raw) && k2.is_empty() {
                                self.resultingSpaltenFromTuple_py(&tupl, neg, None, Some(&cmd_canonical));
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

    pub fn breiteBreitenSysArgvPara(&mut self, cmd: &str, neg: &str) -> bool {
        if let Some(tail) = cmd.strip_prefix("breite=") {
            self.setShellRowsAmount();
            if self.breiteHasBeenOnceZero {
                self.shellRowsAmount = 0;
                self.textWidth = 0;
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
                if neg.is_empty() {
                    self.breite = breite;
                    if breite > self.textWidth {
                        self.textWidth = breite;
                    }
                }
                self.breiteORbreiten = true;
            }
            return true;
        }
        if let Some(tail) = cmd.strip_prefix("breiten=") {
            if neg.is_empty() {
                self.breiten = vec![];
                for breite in tail.split(',') {
                    if breite.trim().chars().all(|c| c.is_ascii_digit()) {
                        self.breiten.push(breite.trim().parse::<i64>().unwrap_or(0));
                        self.breiteORbreiten = true;
                    }
                }
            }
            return true;
        }
        false
    }

    pub fn setShellRowsAmount(&mut self) {
        self.shellRowsAmount = 0;
        if let Ok(v) = std::env::var("LINES") {
            self.shellRowsAmount = v.parse::<i64>().unwrap_or(0);
        }
    }

    fn detect_terminal_columns_py() -> i64 {
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
        0
    }

    pub fn setShellWidth(&mut self) {
        self.shellWidth = Self::detect_terminal_columns_py();
    }


    fn primCreativity_py(n: i64) -> i64 {
        if n < 2 {
            return 0;
        }
        if n == 2 || n == 3 {
            return 1;
        }
        if n % 2 == 0 {
            return 0;
        }
        let mut d = 3i64;
        while d * d <= n {
            if n % d == 0 {
                return 0;
            }
            d += 2;
        }
        1
    }

    fn build_alles_entry_python_like(&self, words: &Words) -> StoreParameterEntry {
        let mut allValues: Vec<BTreeSet<i64>> = (0..12).map(|_| BTreeSet::new()).collect();
        let mut gebrochenSpaltenMaximumPlus1 = 2i64;

        for possibleCommands in words.paraNdataMatrix.iter() {
            for (i, commandValue) in possibleCommands.datas.iter().enumerate() {
                for spaltenNummerOderEtc in commandValue {
                    match spaltenNummerOderEtc {
                        PyValue::Int(n) => {
                            allValues[i].insert(*n);
                            if [5usize, 6usize, 9usize, 10usize].contains(&i) && *n + 1 > gebrochenSpaltenMaximumPlus1 {
                                gebrochenSpaltenMaximumPlus1 = *n + 1;
                            }
                        }
                        PyValue::Tuple(inner) => {
                            for vv in inner {
                                if let PyValue::Int(n) = vv {
                                    allValues[i].insert(*n);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        let allowedPrimNumbersForCommand: Vec<i64> = (2..32)
            .filter(|num| Self::primCreativity_py(*num) == 1)
            .collect();

        allValues[2] = allowedPrimNumbersForCommand.into_iter().collect();
        allValues[3] = words.kombiParaNdataMatrix.keys().cloned().collect();
        allValues[5] = (2..gebrochenSpaltenMaximumPlus1).collect();
        allValues[6] = (2..gebrochenSpaltenMaximumPlus1).collect();
        allValues[8] = words.kombiParaNdataMatrix2.keys().cloned().collect();
        allValues[9] = (2..gebrochenSpaltenMaximumPlus1).collect();
        allValues[10] = (2..gebrochenSpaltenMaximumPlus1).collect();

        if self.__invertAlles {
            let max0 = *allValues[0].iter().max().unwrap_or(&0);
            let mut inverted = BTreeSet::new();
            for n in 0..max0 {
                if !allValues[0].contains(&n) {
                    inverted.insert(n);
                }
            }
            allValues[0] = inverted;
            for zahl in 1..11usize {
                allValues[zahl].clear();
            }
        }

        let datas = allValues.into_iter().map(|set| set.into_iter().map(PyValue::Int).collect::<Vec<PyValue>>()).collect::<Vec<Vec<PyValue>>>();
        StoreParameterEntry {
            parameterMainNames: vec!["alles".to_string()],
            parameterNames: vec![],
            datas,
        }
    }

    pub fn storeParamtersForColumns(&mut self, words: &Words) {
        self.kombiReverseDict = IndexMap::new();
        for (key, value) in words.kombiParaNdataMatrix.iter() {
            for valuesInValuess in value {
                self.kombiReverseDict.insert(valuesInValuess.clone(), *key);
            }
        }

        self.kombiReverseDict2 = IndexMap::new();
        for (key, value) in words.kombiParaNdataMatrix2.iter() {
            for valuesInValuess in value {
                self.kombiReverseDict2.insert(valuesInValuess.clone(), *key);
            }
        }

        self.paraMainDict = IndexMap::new();
        self.paraDict = IndexMap::new();
        self.dataDicts = (0..12).map(|_| IndexMap::new()).collect();
        self.paraDictGenerated = IndexMap::new();
        self.paraDictGenerated4htmlTags = IndexMap::new();

        let mut paraNdataMatrix = words.paraNdataMatrix.clone();
        let alles_entry = self.build_alles_entry_python_like(words);
        self.AllSimpleCommandSpalten = alles_entry.datas.get(0)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| if let PyValue::Int(n) = v { Some(n) } else { None })
            .collect();
        paraNdataMatrix.push(alles_entry);

        for parameterEntry in paraNdataMatrix.iter() {
            let into = self.intoParameterDatatype(
                &parameterEntry.parameterMainNames,
                &parameterEntry.parameterNames,
                &parameterEntry.datas,
            );
            let (paraMainDict3, paraDict3, dataDicts3) = self.mergeParameterDicts(
                self.paraMainDict.clone(),
                self.paraDict.clone(),
                self.dataDicts.clone(),
                into.0,
                into.1,
                into.2,
            );
            self.paraMainDict = paraMainDict3;
            self.paraDict = paraDict3;
            self.dataDicts = dataDicts3;
        }

        self.dataDict = self.dataDicts.clone();
        while self.dataDict.len() < 14 {
            self.dataDict.push(IndexMap::new());
        }
        self.dataDict[3] = IndexMap::new();
        for (key, value) in words.kombiParaNdataMatrix.iter() {
            self.dataDict[3].insert(
                key.to_string(),
                value.iter().map(|txt| vec![PairStr("kombi".to_string(), txt.clone())]).collect(),
            );
        }
        self.dataDict[8] = IndexMap::new();
        for (key, value) in words.kombiParaNdataMatrix2.iter() {
            self.dataDict[8].insert(
                key.to_string(),
                value.iter().map(|txt| vec![PairStr("kombi2".to_string(), txt.clone())]).collect(),
            );
        }
    }

    pub fn intoParameterDatatype(
        &self,
        parameterMainNames: &Vec<String>,
        parameterNames: &Vec<String>,
        datas: &Vec<Vec<PyValue>>,
    ) -> (
        IndexMap<String, Vec<String>>,
        IndexMap<(String, String), Vec<Vec<PyValue>>>,
        Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
    ) {
        let mut paraMainDict: IndexMap<String, Vec<String>> = IndexMap::new();
        for name in parameterMainNames {
            paraMainDict.insert(name.clone(), parameterNames.clone());
        }

        let mut paraDict: IndexMap<(String, String), Vec<Vec<PyValue>>> = IndexMap::new();
        for name1 in parameterMainNames {
            for name2 in parameterNames {
                paraDict.insert((name1.clone(), name2.clone()), datas.clone());
            }
            if parameterNames.len() == 0 {
                paraDict.insert((name1.clone(), "".to_string()), datas.clone());
            }
        }

        let mut dataDicts: Vec<IndexMap<String, Vec<Vec<PairStr>>>> = (0..12).map(|_| IndexMap::new()).collect();

        for (i, d) in datas.iter().enumerate() {
            for spaltenNummerOderEtc in d {
                let mut into: Vec<PairStr> = vec![];
                let mut parameterMainNamePerLoop: Vec<String> = vec![];
                let mut case_: i64 = -1;
                let mut spaltenNummerOderEtc_local = spaltenNummerOderEtc.clone();

                for parameterMainName in parameterMainNames {
                    let parameterNames_local = if parameterNames.len() > 0 {
                        parameterNames.clone()
                    } else {
                        vec!["".to_string()]
                    };
                    for parameterName in parameterNames_local {
                        if i == 4 && matches!(spaltenNummerOderEtc_local, PyValue::Bool(_)) {
                            case_ = 1;
                            into.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                        } else if matches!(i, 5 | 6 | 9 | 10) {
                            case_ = 2;
                            into.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                            parameterMainNamePerLoop.push(parameterName.clone());
                        } else if i == 2 && matches!(spaltenNummerOderEtc_local, PyValue::Str(_)) {
                            case_ = 2;
                            parameterMainNamePerLoop.push(parameterName.clone());
                            into.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                        } else if i == 4 && matches!(spaltenNummerOderEtc_local, PyValue::Tuple(_)) {
                            case_ = 4;
                            into.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                        } else {
                            case_ = 3;
                            into.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                        }
                    }
                }

                let index1 = if case_ != 1 { i } else { 3 };
                let index2a: Vec<String> = if case_ == 3 {
                    vec![format!("{:?}", spaltenNummerOderEtc_local)]
                } else if case_ == 4 {
                    match &spaltenNummerOderEtc_local {
                        PyValue::Tuple(inner) => vec![format!("{:?}", inner)],
                        _ => vec![format!("{:?}", spaltenNummerOderEtc_local)],
                    }
                } else if case_ == 1 {
                    vec!["('bool', 0)".to_string()]
                } else if case_ == 2 {
                    vec![format!("{:?}", parameterMainNamePerLoop)]
                } else {
                    vec!["None".to_string()]
                };

                let intoA: Vec<Vec<PairStr>> = if case_ == 2 {
                    into.iter().map(|x| vec![x.clone()]).collect()
                } else {
                    vec![into.clone()]
                };

                let max_len = std::cmp::max(index2a.len(), intoA.len());
                for pos in 0..max_len {
                    let index2 = if pos < index2a.len() {
                        index2a[pos].clone()
                    } else {
                        format!("{:?}", index2a)
                    };
                    let into2 = if pos < intoA.len() {
                        intoA[pos].clone()
                    } else {
                        into.clone()
                    };
                    let entry = dataDicts[index1].entry(index2).or_insert_with(Vec::new);
                    if !entry.iter().any(|e| e == &into2) {
                        entry.push(into2);
                    }
                }
            }
        }

        (paraMainDict, paraDict, dataDicts)
    }

    pub fn mergeParameterDicts(
        &self,
        paraMainDict1: IndexMap<String, Vec<String>>,
        paraDict1: IndexMap<(String, String), Vec<Vec<PyValue>>>,
        dataDicts1: Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
        paraMainDict2: IndexMap<String, Vec<String>>,
        paraDict2: IndexMap<(String, String), Vec<Vec<PyValue>>>,
        dataDicts2: Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
    ) -> (
        IndexMap<String, Vec<String>>,
        IndexMap<(String, String), Vec<Vec<PyValue>>>,
        Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
    ) {
        let mut paraMainDict3 = paraMainDict1.clone();
        for (k, v) in paraMainDict2.iter() {
            paraMainDict3.insert(k.clone(), v.clone());
        }

        let mut paraDict3 = paraDict1.clone();
        for (k, v) in paraDict2.iter() {
            paraDict3.insert(k.clone(), v.clone());
        }

        let mut dataDicts3 = dataDicts1.clone();
        let max_len = if dataDicts1.len() > dataDicts2.len() {
            dataDicts1.len()
        } else {
            dataDicts2.len()
        };

        while dataDicts3.len() < max_len {
            dataDicts3.push(IndexMap::new());
        }

        for i in 0..max_len {
            let dict1 = dataDicts1.get(i);
            let dict2 = dataDicts2.get(i);

            if dict1.is_some() && dict2.is_some() {
                let d1 = dict1.unwrap();
                let d2 = dict2.unwrap();
                if dataDicts3[i].keys().len() == 0 {
                    dataDicts3[i] = d2.clone();
                } else {
                    for (key1, _value1) in d1.iter() {
                        for (key2, value2) in d2.iter() {
                            if key2 == key1 {
                                let entry = dataDicts3[i].entry(key1.clone()).or_insert_with(Vec::new);
                                entry.extend(value2.clone());
                            } else if !dataDicts3[i].contains_key(key2) {
                                dataDicts3[i].insert(key2.clone(), value2.clone());
                            }
                        }
                    }
                }
            } else if dict1.is_some() && dict2.is_none() {
                dataDicts3[i] = dict1.unwrap().clone();
            } else if dict1.is_none() && dict2.is_some() {
                dataDicts3[i] = dict2.unwrap().clone();
            }
        }

        (paraMainDict3, paraDict3, dataDicts3)
    }
}
