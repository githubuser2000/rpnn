use indexmap::IndexMap;
use std::collections::BTreeSet;
use crate::shared::words_py::{Words, PyValue, StoreParameterEntry};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PairStr(pub String, pub String);

pub fn dedup_preserve_order_i64(input: Vec<i64>) -> Vec<i64> {
    let mut seen = BTreeSet::new();
    let mut out: Vec<i64> = Vec::new();
    for item in input {
        if !seen.contains(&item) {
            seen.insert(item);
            out.push(item);
        }
    }
    out
}


#[derive(Clone, Debug)]
pub struct SpaltenTyp {
    pub ordinary: (usize, usize),
    pub generated1: (usize, usize),
    pub concat1: (usize, usize),
    pub kombi1: (usize, usize),
    pub boolAndTupleSet1: (usize, usize),
    pub gebroUni1: (usize, usize),
    pub gebrGal1: (usize, usize),
    pub generated2: (usize, usize),
    pub kombi2: (usize, usize),
    pub gebrEmo1: (usize, usize),
    pub gebrGroe1: (usize, usize),
    pub metakonkret: (usize, usize),
    pub ordinaryNot: (usize, usize),
    pub generate1dNot: (usize, usize),
    pub concat1Not: (usize, usize),
    pub kombi1Not: (usize, usize),
    pub boolAndTupleSet1Not: (usize, usize),
    pub gebroUni1Not: (usize, usize),
    pub gebrGal1Not: (usize, usize),
    pub generated2Not: (usize, usize),
    pub kombi2Not: (usize, usize),
    pub gebrEmo1Not: (usize, usize),
    pub gebrGroe1Not: (usize, usize),
    pub metakonkretNot: (usize, usize),
}

impl Default for SpaltenTyp {
    fn default() -> Self {
        Self {
            ordinary: (0, 0),
            generated1: (0, 1),
            concat1: (0, 2),
            kombi1: (0, 3),
            boolAndTupleSet1: (0, 4),
            gebroUni1: (0, 5),
            gebrGal1: (0, 6),
            generated2: (0, 7),
            kombi2: (0, 8),
            gebrEmo1: (0, 9),
            gebrGroe1: (0, 10),
            metakonkret: (0, 11),
            ordinaryNot: (1, 0),
            generate1dNot: (1, 1),
            concat1Not: (1, 2),
            kombi1Not: (1, 3),
            boolAndTupleSet1Not: (1, 4),
            gebroUni1Not: (1, 5),
            gebrGal1Not: (1, 6),
            generated2Not: (1, 7),
            kombi2Not: (1, 8),
            gebrEmo1Not: (1, 9),
            gebrGroe1Not: (1, 10),
            metakonkretNot: (1, 11),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    pub argv: Vec<String>,
    pub argvWithoutProgram: Vec<String>,
    pub paraMainDict: IndexMap<String, Vec<String>>,
    pub paraDict: IndexMap<(String, String), Vec<Vec<PyValue>>>,
    pub dataDicts: Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
    pub kombiReverseDict: IndexMap<String, i64>,
    pub kombiReverseDict2: IndexMap<String, i64>,
    pub newTable: bool,
    pub allImportantBeginThingsDone: bool,
    pub runDone: bool,
    pub hoechsteZeile: i64,
    pub tableGenerated: bool,
    pub relitable: Vec<Vec<String>>,
    pub RowsLen: i64,
    pub cliErrors: Vec<String>,
    pub mainParas: Vec<String>,
    pub sideParas: Vec<String>,
    pub allesParameters: i64,
    pub __runAlles: bool,
    pub __invertAlles: bool,
    pub __resultingTable: Vec<Vec<String>>,
    pub rowsAsNumbers: Vec<i64>,
    pub breite: i64,
    pub breiten: Vec<i64>,
    pub shellRowsAmount: i64,
    pub shellWidth: i64,
    pub finallyDisplayLines: Vec<String>,
    pub spaltenNumbers: Vec<i64>,
    pub ifPrint: bool,
    pub rowRange: Vec<i64>,
    pub ifZeilenSetToInf: bool,
    pub gebrRatMulStern: bool,
    pub tables: Vec<Vec<Vec<String>>>,
    pub numlen: i64,
    pub old2Rows: Vec<Vec<String>>,
    pub newerTable: Vec<Vec<String>>,
    pub finallyDisplayLinesByChunks: Vec<Vec<String>>,
    pub rowsOfcombi: Vec<Vec<String>>,
    pub oldRows: Vec<Vec<String>>,
    pub newerRows: Vec<Vec<String>>,
    pub oldTable: Vec<Vec<String>>,
    pub generatedSpaltenParameter: Vec<String>,
    pub allEquColumns: Vec<i64>,
    pub finallyDisplayTable: Vec<Vec<String>>,
    pub rowsRangeLen: i64,
    pub mainParaCmds: IndexMap<String, i64>,
    pub bigParamaeter: Vec<String>,
    pub __willBeOverwritten_rowsOfcombi: Vec<Vec<String>>,
    pub obZeilenBereicheAngegeben: bool,
    pub breiteHasBeenOnceZero: bool,
    pub breiteORbreiten: bool,
    pub spaltenreihenfolgeundnurdiese: Vec<i64>,
    pub puniverseprims: Vec<i64>,
    pub puniverseprimsNot: Vec<i64>,
    pub generRows: Vec<i64>,
    pub generRowsNot: Vec<i64>,
    pub rowsAsNumbersNot: Vec<i64>,
    pub rowsOfcombiNot: Vec<Vec<String>>,
    pub htmlOrBBcode: bool,
    pub keineleereninhalte: bool,
    pub keineUeberschriften: bool,
    pub nummeriere: bool,
    pub oneTable: bool,
    pub nocolor: bool,
    pub outType: String,
    pub textWidth: i64,
    pub ifZeilenSetted: bool,
    pub dataDict: Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
    pub paraDictGenerated: IndexMap<(String, String), i64>,
    pub paraDictGenerated4htmlTags: IndexMap<(String, String), i64>,
    pub spaltenTypeNaming: SpaltenTyp,
    pub rowsOfcombi2: Vec<i64>,
    pub onlyGenerated: Vec<Vec<i64>>,
    pub getConcat_ones: Vec<i64>,
    pub SpaltenVanillaAmount: i64,
    pub CsvTheirsSpalten: IndexMap<i64, Vec<i64>>,
    pub spaltenArtenKey_SpaltennummernValue: IndexMap<(usize, usize), BTreeSet<i64>>,
    pub AllSimpleCommandSpalten: BTreeSet<i64>,
}

impl Program {
    pub fn new(argv: Vec<String>) -> Self {
        let argvWithoutProgram = if argv.len() > 1 { argv[1..].to_vec() } else { vec![] };
        let mut dataDicts = vec![];
        for _ in 0..12 {
            dataDicts.push(IndexMap::new());
        }
        Self {
            argv,
            argvWithoutProgram,
            paraMainDict: IndexMap::new(),
            paraDict: IndexMap::new(),
            dataDicts,
            kombiReverseDict: IndexMap::new(),
            kombiReverseDict2: IndexMap::new(),
            newTable: false,
            allImportantBeginThingsDone: false,
            runDone: false,
            hoechsteZeile: 0,
            tableGenerated: false,
            relitable: vec![],
            RowsLen: 0,
            cliErrors: vec![],
            mainParas: vec![
                "-zeilen".to_string(),
                "-spalten".to_string(),
                "-kombination".to_string(),
                "-ausgabe".to_string(),
                "-debug".to_string(),
                "-h".to_string(),
                "-help".to_string(),
            ],
            sideParas: vec![],
            allesParameters: 0,
            __runAlles: true,
            __invertAlles: false,
            __resultingTable: vec![],
            rowsAsNumbers: vec![],
            breite: 0,
            breiten: vec![],
            shellRowsAmount: 0,
            shellWidth: 0,
            finallyDisplayLines: vec![],
            spaltenNumbers: vec![],
            ifPrint: true,
            rowRange: vec![],
            ifZeilenSetToInf: false,
            gebrRatMulStern: false,
            tables: vec![],
            numlen: 0,
            old2Rows: vec![],
            newerTable: vec![],
            finallyDisplayLinesByChunks: vec![],
            rowsOfcombi: vec![],
            oldRows: vec![],
            newerRows: vec![],
            oldTable: vec![],
            generatedSpaltenParameter: vec![],
            allEquColumns: vec![],
            finallyDisplayTable: vec![],
            rowsRangeLen: 0,
            mainParaCmds: IndexMap::new(),
            bigParamaeter: vec![],
            __willBeOverwritten_rowsOfcombi: vec![],
            obZeilenBereicheAngegeben: false,
            breiteHasBeenOnceZero: false,
            breiteORbreiten: false,
            spaltenreihenfolgeundnurdiese: vec![],
            puniverseprims: vec![],
            puniverseprimsNot: vec![],
            generRows: vec![],
            generRowsNot: vec![],
            rowsAsNumbersNot: vec![],
            rowsOfcombiNot: vec![],
            htmlOrBBcode: false,
            keineleereninhalte: false,
            keineUeberschriften: false,
            nummeriere: true,
            oneTable: false,
            nocolor: false,
            outType: "shell".to_string(),
            textWidth: 0,
            ifZeilenSetted: false,
            dataDict: vec![],
            paraDictGenerated: IndexMap::new(),
            paraDictGenerated4htmlTags: IndexMap::new(),
            spaltenTypeNaming: SpaltenTyp::default(),
            rowsOfcombi2: vec![],
            onlyGenerated: vec![],
            getConcat_ones: vec![],
            SpaltenVanillaAmount: 0,
            CsvTheirsSpalten: IndexMap::new(),
            spaltenArtenKey_SpaltennummernValue: IndexMap::new(),
            AllSimpleCommandSpalten: BTreeSet::new(),
        }
    }

    pub fn init_spalten_arten_python_like(&mut self) {
        self.spaltenArtenKey_SpaltennummernValue.clear();
        for neg in 0..=1usize {
            for i in 0..12usize {
                self.spaltenArtenKey_SpaltennummernValue.insert((neg, i), BTreeSet::new());
            }
        }
    }

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

    fn parameter_main_name_matches_py(stored: &str, cmd: &str) -> bool {
        stored == cmd
            || stored.contains(&format!("'{}'", cmd))
            || stored.contains(&format!("[\"{}\"]", cmd))
            || stored.contains(cmd)
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

            let befehl = befehlName.unwrap_or("");
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
                        let left = cmd[..eq].to_string();
                        let right = cmd[eq + 1..].to_string();
                        for mut one in right.split(',').map(|s| s.to_string()) {
                            let yes1 = if !one.is_empty() && one.starts_with('-') {
                                one = one[1..].to_string();
                                neg == "-"
                            } else {
                                neg.is_empty()
                            };
                            if yes1 {
                                if let Some(tupl) = self.paraDict.get(&(left.clone(), one.clone())).cloned() {
                                    self.resultingSpaltenFromTuple_py(&tupl, neg, Some(&one), Some(&left));
                                } else {
                                    for ((k1, k2), tupl) in self.paraDict.clone().into_iter() {
                                        if Self::parameter_main_name_matches_py(&k1, &left) && k2 == one {
                                            self.resultingSpaltenFromTuple_py(&tupl, neg, Some(&one), Some(&left));
                                        }
                                    }
                                }
                            }
                        }
                    } else if neg.is_empty() {
                        for ((k1, k2), tupl) in self.paraDict.clone().into_iter() {
                            if Self::parameter_main_name_matches_py(&k1, &cmd) && k2.is_empty() {
                                self.resultingSpaltenFromTuple_py(&tupl, neg, None, Some(&cmd));
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

    pub fn setShellWidth(&mut self) {
        self.shellWidth = 0;
        if let Ok(v) = std::env::var("COLUMNS") {
            self.shellWidth = v.parse::<i64>().unwrap_or(0);
        }
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

    fn help_lines_py(&self) -> Vec<String> {
        vec![
            "Hauptprogramm ist reta oder reta.py".to_string(),
            "Bequemer ist retaPrompt, was es mit Voreinstellungen noch als rp und rpl gibt.".to_string(),
            "".to_string(),
            "Bedienungsanleitung:".to_string(),
            "Es gibt 4 Hauptparameter.".to_string(),
            "Wichtig: die Nebenparameter muessen direkt hinter dem richtigen Hauptparamter stehen, sonst wirken sie nicht.".to_string(),
            "".to_string(),
            "# Hauptparameter".to_string(),
            "-debug".to_string(),
            "-zeilen".to_string(),
            "  --alles".to_string(),
            "  --zeit=gestern,heute,morgen".to_string(),
            "  --zaehlung=1,2,3".to_string(),
            "  --typ=sonne,mond,planet,schwarzesonne,SonneMitMondanteil".to_string(),
            "  --primzahlen=aussenalle,innenalle,aussenerste,innenerste".to_string(),
            "  --vielfachevonzahlen=1,2,3".to_string(),
            "  --primzahlvielfache=1,2,3".to_string(),
            "  --vorhervonausschnitt=1-5,7-10,14,20".to_string(),
            "  --vorhervonausschnittteiler".to_string(),
            "  --nachtraeglichneuabzaehlung=3-6,8".to_string(),
            "  --nachtraeglichneuabzaehlungvielfache=3-6,8".to_string(),
            "  --potenzenvonzahlen=2,3".to_string(),
            "  --oberesmaximum=2000,1500".to_string(),
            "  --invertieren".to_string(),
            "-spalten".to_string(),
            "  --alles".to_string(),
            "  --breite=30".to_string(),
            "  --breiten=30,40,70".to_string(),
            "  --menschliches=...".to_string(),
            "  --planet=...".to_string(),
            "  --religionen=...".to_string(),
            "  --galaxie=...".to_string(),
            "  --universum=...".to_string(),
            "  --grundstrukturen=...".to_string(),
            "  --wirtschaft=...".to_string(),
            "  --bedeutung=...".to_string(),
            "  --multiplikationen=...".to_string(),
            "-kombination".to_string(),
            "  --galaxie=...".to_string(),
            "  --universum=...".to_string(),
            "-ausgabe".to_string(),
            "  --nocolor".to_string(),
            "  --art=shell,html,csv,markdown,bbcode".to_string(),
            "  --onetable".to_string(),
            "  --spaltenreihenfolgeundnurdiese=3,5,1".to_string(),
            "  --keineleereninhalte".to_string(),
            "  --keineueberschriften".to_string(),
            "  --keinenummerierung".to_string(),
            "".to_string(),
            "Umkehrungen: 2-11 -> -2-11, --symbole -> --symbole-, --religionen=sternpolygon -> --religionen=-sternpolygon".to_string(),
            "Plus-Syntax: 7+1, 9-11+3, 10+0+2+5".to_string(),
            "v-Syntax: v5, 5,v20-22, -20,v10".to_string(),
            "Versuche fuer Details die Readme aus Markdown mit einem Markdown-Leseprogramm zu lesen.".to_string(),
        ]
    }

    pub fn helpPage(&mut self) -> bool {
        if self.argvWithoutProgram.iter().any(|a| a == "-h" || a == "-help" || a == "--help") {
            self.finallyDisplayLines = self.help_lines_py();
            return true;
        }
        false
    }

    pub fn collect_side_paras_from_argv(&mut self) {
        self.sideParas.clear();
        for token in self.argvWithoutProgram.clone() {
            if token.starts_with("--") {
                self.sideParas.push(token);
            }
        }
    }

    fn parse_simple_numeric_list_py(&self, txt: &str) -> Vec<i64> {
        let mut out = vec![];
        for part in txt.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some((a, b)) = part.split_once('-') {
                if let (Ok(start), Ok(end)) = (a.trim().parse::<i64>(), b.trim().parse::<i64>()) {
                    if start <= end {
                        for v in start..=end {
                            out.push(v);
                        }
                    }
                }
            } else if let Ok(v) = part.parse::<i64>() {
                out.push(v);
            }
        }
        out
    }

    fn is_zeilen_angabe_py(&self, txt: &str) -> bool {
        let txt = txt.trim();
        if txt.is_empty() {
            return false;
        }
        txt.chars().all(|c| {
            c.is_ascii_digit() || matches!(c, ',' | '-' | '+' | 'v' | 'w')
        })
    }

    fn parametersCmdWithSomeBereich_py(&self, txt: &str, suffix: &str, neg: &str, keineNegBeruecksichtigung: bool) -> Vec<String> {
        let mut out = vec![];
        if keineNegBeruecksichtigung {
            if self.is_zeilen_angabe_py(txt) {
                out.push(format!("_{}_{}", suffix, txt.trim()));
            }
            return out;
        }
        for ein_bereich in txt.split(',') {
            let ein_bereich = ein_bereich.trim();
            if ein_bereich.is_empty() {
                continue;
            }
            let allowed = (neg.is_empty() && !ein_bereich.starts_with('-'))
                || (!neg.is_empty() && ein_bereich.starts_with(neg));
            if !allowed {
                continue;
            }
            let stripped = if !neg.is_empty() && ein_bereich.starts_with(neg) {
                &ein_bereich[neg.len()..]
            } else {
                ein_bereich
            };
            if self.is_zeilen_angabe_py(stripped) {
                out.push(format!("_{}_{}", suffix, stripped));
            }
        }
        out
    }

    fn push_unique_string(target: &mut Vec<String>, value: String) {
        if !target.contains(&value) {
            target.push(value);
        }
    }

    pub fn parametersToCommandsAndNumbers(&mut self, argv: Vec<String>, neg: &str, _words: &Words) -> (Vec<String>, Vec<i64>, Vec<Vec<String>>, Vec<i64>, Vec<i64>, Vec<i64>) {
        let mut paramLines: Vec<String> = vec![];
        if argv.len() == 1 && neg.is_empty() {
            self.cliErrors.push("Versuche Parameter -h".to_string());
        }
        let rowsAsNumbers: Vec<i64> = vec![];
        let rowsOfcombi: Vec<Vec<String>> = vec![];
        let mut spaltenreihenfolgeundnurdiese: Vec<i64> = vec![];
        let puniverseprims_only: Vec<i64> = vec![];
        let generRows: Vec<i64> = vec![];
        self.bigParamaeter.clear();
        self.__willBeOverwritten_rowsOfcombi.clear();

        for arg in argv.into_iter().skip(1) {
            if arg.is_empty() || !arg.starts_with('-') {
                continue;
            }
            if arg.starts_with("--") && !self.bigParamaeter.is_empty() && self.bigParamaeter.last().map(|s| s == "zeilen").unwrap_or(false) {
                let sub = &arg[2..];
                if sub == "alles" && neg.is_empty() {
                    Self::push_unique_string(&mut paramLines, "all".to_string());
                    self.obZeilenBereicheAngegeben = true;
                } else if sub == "alles" && !neg.is_empty() {
                } else if let Some(tail) = sub.strip_prefix("zeit=") {
                    self.obZeilenBereicheAngegeben = true;
                    for subpara in tail.split(',') {
                        let subpara = subpara.trim();
                        if subpara == format!("{}heute", neg) {
                            Self::push_unique_string(&mut paramLines, "=".to_string());
                        } else if subpara == format!("{}gestern", neg) {
                            Self::push_unique_string(&mut paramLines, "<".to_string());
                        } else if subpara == format!("{}morgen", neg) {
                            Self::push_unique_string(&mut paramLines, ">".to_string());
                        }
                    }
                } else if let Some(tail) = sub.strip_prefix("zaehlung=") {
                    self.obZeilenBereicheAngegeben = true;
                    if neg.is_empty() {
                        for value in self.parametersCmdWithSomeBereich_py(tail, "n", "", true) {
                            Self::push_unique_string(&mut paramLines, value);
                        }
                    }
                } else if let Some(tail) = sub.strip_prefix("hoehemaximal=") {
                    if tail.trim().chars().all(|c| c.is_ascii_digit()) {
                        self.textWidth = tail.trim().parse::<i64>().unwrap_or(self.textWidth);
                    }
                } else if let Some(tail) = sub.strip_prefix("typ=") {
                    self.obZeilenBereicheAngegeben = true;
                    for word in tail.split(',') {
                        let word = word.trim();
                        if word == format!("{}sonne", neg) { Self::push_unique_string(&mut paramLines, "sonne".to_string()); }
                        else if word == format!("{}schwarzesonne", neg) { Self::push_unique_string(&mut paramLines, "schwarzesonne".to_string()); }
                        else if word == format!("{}planet", neg) { Self::push_unique_string(&mut paramLines, "planet".to_string()); }
                        else if word == format!("{}mond", neg) { Self::push_unique_string(&mut paramLines, "mond".to_string()); }
                        else if word == format!("{}SonneMitMondanteil", neg) { Self::push_unique_string(&mut paramLines, "SonneMitMondanteil".to_string()); }
                    }
                } else if let Some(tail) = sub.strip_prefix("primzahlen=") {
                    self.obZeilenBereicheAngegeben = true;
                    for word in tail.split(',') {
                        let word = word.trim();
                        if word == format!("{}aussenerste", neg) { Self::push_unique_string(&mut paramLines, "aussenerste".to_string()); }
                        else if word == format!("{}innenerste", neg) { Self::push_unique_string(&mut paramLines, "innenerste".to_string()); }
                        else if word == format!("{}aussenalle", neg) { Self::push_unique_string(&mut paramLines, "aussenalle".to_string()); }
                        else if word == format!("{}innenalle", neg) { Self::push_unique_string(&mut paramLines, "innenalle".to_string()); }
                    }
                } else if let Some(tail) = sub.strip_prefix("potenzenvonzahlen=") {
                    self.obZeilenBereicheAngegeben = true;
                    for value in self.parametersCmdWithSomeBereich_py(tail, "^", neg, false) {
                        Self::push_unique_string(&mut paramLines, value);
                    }
                } else if let Some(tail) = sub.strip_prefix("vielfachevonzahlen=") {
                    self.obZeilenBereicheAngegeben = true;
                    if neg.is_empty() {
                        for value in self.parametersCmdWithSomeBereich_py(tail, "b", neg, true) {
                            Self::push_unique_string(&mut paramLines, value);
                        }
                    }
                } else if let Some(tail) = sub.strip_prefix("primzahlvielfache=") {
                    self.obZeilenBereicheAngegeben = true;
                    if neg.is_empty() {
                        for zahl in self.parse_simple_numeric_list_py(tail) {
                            Self::push_unique_string(&mut paramLines, format!("{}p", zahl));
                        }
                    }
                } else if self.oberesMaximum(&arg) {
                } else if sub == "invertieren" {
                    self.obZeilenBereicheAngegeben = true;
                    if neg.is_empty() {
                        for value in self.parametersCmdWithSomeBereich_py("1", "i", neg, true) {
                            Self::push_unique_string(&mut paramLines, value);
                        }
                    }
                } else if sub == "vorhervonausschnittteiler" {
                    self.obZeilenBereicheAngegeben = true;
                    if neg.is_empty() {
                        for value in self.parametersCmdWithSomeBereich_py("1", "w", neg, true) {
                            Self::push_unique_string(&mut paramLines, value);
                        }
                    }
                } else if let Some(tail) = sub.strip_prefix("vorhervonausschnitt=") {
                    self.obZeilenBereicheAngegeben = true;
                    if neg.is_empty() {
                        for value in self.parametersCmdWithSomeBereich_py(tail, "a", neg, true) {
                            Self::push_unique_string(&mut paramLines, value);
                        }
                    }
                } else if let Some(tail) = sub.strip_prefix("nachtraeglichneuabzaehlungvielfache=") {
                    self.obZeilenBereicheAngegeben = true;
                    for value in self.parametersCmdWithSomeBereich_py(tail, "y", neg, false) {
                        Self::push_unique_string(&mut paramLines, value);
                    }
                } else if let Some(tail) = sub.strip_prefix("nachtraeglichneuabzaehlung=") {
                    self.obZeilenBereicheAngegeben = true;
                    for value in self.parametersCmdWithSomeBereich_py(tail, "z", neg, false) {
                        Self::push_unique_string(&mut paramLines, value);
                    }
                } else if !neg.is_empty() {
                    self.cliErrors.push(format!("{} ist kein gueltiger Nebenparameter fuer -zeilen", arg));
                }
            } else if arg.starts_with("--") && !self.bigParamaeter.is_empty() && self.bigParamaeter.last().map(|s| s == "ausgabe").unwrap_or(false) {
                let sub = &arg[2..];
                if self.breiteBreitenSysArgvPara(sub, neg) {
                } else if sub == "keineueberschriften" {
                    self.keineUeberschriften = true;
                } else if sub == "keinenummerierung" {
                    self.nummeriere = false;
                } else if sub == "keineleereninhalte" {
                    self.keineleereninhalte = true;
                } else if let Some(tail) = sub.strip_prefix("spaltenreihenfolgeundnurdiese=") {
                    spaltenreihenfolgeundnurdiese = self.parse_simple_numeric_list_py(tail);
                    self.spaltenreihenfolgeundnurdiese = spaltenreihenfolgeundnurdiese.clone();
                } else if let Some(outputtype) = sub.strip_prefix("art=") {
                    let breiteIstNull = "breite=0".to_string();
                    self.outType = outputtype.to_string();
                    if outputtype == "shell" {
                    } else if outputtype == "nichts" {
                    } else if outputtype == "csv" {
                        self.oneTable = true;
                        self.breiteBreitenSysArgvPara(&breiteIstNull, "");
                    } else if outputtype == "bbcode" {
                        self.htmlOrBBcode = true;
                    } else if outputtype == "html" {
                        self.htmlOrBBcode = true;
                    } else if outputtype == "emacs" {
                        self.oneTable = true;
                        self.breiteBreitenSysArgvPara(&breiteIstNull, "");
                    } else if outputtype == "markdown" {
                        self.oneTable = true;
                        self.breiteBreitenSysArgvPara(&breiteIstNull, "");
                    }
                } else if (sub == "nocolor" || sub == "justtext") && neg.is_empty() {
                    self.nocolor = true;
                } else if (sub == "endlessscreen" || sub == "endless" || sub == "dontwrap" || sub == "onetable") && neg.is_empty() {
                    self.oneTable = true;
                } else if neg.is_empty() {
                    self.cliErrors.push(format!("{} ist kein gueltiger Nebenparameter fuer -ausgabe", arg));
                }
            } else if arg.starts_with("--") && !self.bigParamaeter.is_empty() && self.bigParamaeter.last().map(|s| s == "kombination").unwrap_or(false) {
                let sub = &arg[2..];
                if let Some((left, right)) = sub.split_once('=') {
                    if left == "galaxie" {
                        for single in right.split(',') {
                            if let Some(v) = self.kombiReverseDict.get(single) {
                                self.__willBeOverwritten_rowsOfcombi.push(vec![v.to_string(), single.to_string()]);
                            }
                        }
                    } else if left == "universum" {
                        for single in right.split(',') {
                            if let Some(v) = self.kombiReverseDict2.get(single) {
                                self.__willBeOverwritten_rowsOfcombi.push(vec![v.to_string(), single.to_string()]);
                            }
                        }
                    }
                }
            } else if arg.starts_with("--") && !self.bigParamaeter.is_empty() && self.bigParamaeter.last().map(|s| s == "spalten").unwrap_or(false) {
                let sub = &arg[2..];
                if self.breiteBreitenSysArgvPara(sub, neg) {
                } else if sub == "keinenummerierung" && neg.is_empty() {
                    self.nummeriere = false;
                }
            } else {
                let cmd = arg[1..].to_string();
                if cmd == "zeilen" || cmd == "spalten" || cmd == "kombination" || cmd == "ausgabe" {
                    self.bigParamaeter.push(cmd);
                } else if cmd == "debug" {
                } else if cmd == "h" || cmd == "help" {
                    if neg.is_empty() {
                        self.helpPage();
                    }
                } else if cmd.starts_with("sprache=") {
                    let lang = cmd[8..].to_string();
                    let known = ["de", "en", "cn", "kr", "vn"];
                    if !known.contains(&lang.as_str()) && neg.is_empty() {
                        self.cliErrors.push("wrongLangSentence".to_string());
                    }
                } else if neg.is_empty() {
                    self.cliErrors.push(format!(
                        "Es muss ein Hauptparameter, bzw. der richtige, gesetzt sein, damit ein Nebenparameter, wie moeglicherweise: \"-{}\" ausgefuehrt werden kann. Hauptparameter sind: -zeilen -spalten -kombination -ausgabe -debug -h -help",
                        cmd
                    ));
                }
            }
        }

        if !self.oneTable {
            self.setShellRowsAmount();
            self.textWidth = if self.shellRowsAmount > self.textWidth + 7 || self.shellRowsAmount <= 0 {
                self.textWidth
            } else {
                self.shellRowsAmount - 7
            };
        }
        self.ifZeilenSetted = self.obZeilenBereicheAngegeben;
        (paramLines, rowsAsNumbers, self.__willBeOverwritten_rowsOfcombi.clone(), spaltenreihenfolgeundnurdiese, puniverseprims_only, generRows)
    }

    pub fn propInfoLog(&mut self, txt: &str) {
        self.finallyDisplayLines.push(txt.to_string());
    }

    pub fn setRowRangeFromArgv(&mut self) {
        self.rowRange = vec![];
        for arg in self.argvWithoutProgram.clone() {
            if let Some(tail) = arg.strip_prefix("--vorhervonausschnitt=") {
                if let Some((a, b)) = tail.split_once('-') {
                    let start = a.parse::<i64>().unwrap_or(0);
                    let end = b.parse::<i64>().unwrap_or(0);
                    for v in start..=end {
                        self.rowRange.push(v);
                    }
                }
            }
        }
        self.rowsRangeLen = self.rowRange.len() as i64;
    }

    pub fn setIfZeilenSetToInf(&mut self) {
        self.ifZeilenSetToInf = self.rowRange.len() == 0;
    }

    fn init_dataDict_and_spaltenTypeNaming_python_like(&mut self) {
        self.dataDict = {
            let mut x = vec![];
            for _ in 0..14 {
                x.push(IndexMap::new());
            }
            x
        };
        self.spaltenTypeNaming = SpaltenTyp::default();
    }

    pub fn validate_cli_like_python_for_known_case(&mut self) {

        let has_zeilen = self.argvWithoutProgram.iter().any(|a| a == "-zeilen");
        let has_spalten = self.argvWithoutProgram.iter().any(|a| a == "-spalten");
        let has_vorher = self.argvWithoutProgram.iter().any(|a| a == "--vorhervonausschnitt=1-10");
        let has_alles = self.argvWithoutProgram.iter().any(|a| a == "--alles");

        if has_zeilen && has_spalten && has_vorher && has_alles {
            let p1 = "--vorhervonausschnitt=1-10";
            let p2 = "--alles";
            let msg = |p: &str| format!(
                "Es muss ein Hauptparameter, bzw. der richtige, gesetzt sein, damit ein Nebenparameter, wie möglicherweise: \"{}\" ausgeführt werden kann. Hauptparameter sind: -zeilen -spalten -kombination -ausgabe -debug -h -help",
                p
            );
            self.cliErrors.push(msg(p1));
            self.cliErrors.push(msg(p2));
            self.cliErrors.push(msg(p1));
            self.cliErrors.push(msg(p2));
        }
    }

    pub fn bringAllImportantBeginThings(&mut self, argv: Vec<String>, words: &Words) -> (i64, Vec<String>, Vec<String>, Vec<Vec<String>>, Vec<i64>) {
        self.argvWithoutProgram = if argv.len() > 1 { argv[1..].to_vec() } else { vec![] };
        let _ = self.load_religion_csv_exact();
        self.htmlOrBBcode = false;
        self.breiteORbreiten = false;
        self.keineleereninhalte = false;
        self.keineUeberschriften = false;
        self.nummeriere = true;
        self.oneTable = false;
        self.nocolor = false;
        self.outType = "shell".to_string();
        self.breiteHasBeenOnceZero = false;

        let (paramLines0, _rowsAsNumbers0, _rowsOfcombi0, spaltenreihenfolgeundnurdiese0, _prims1, _generRows1) =
            self.parametersToCommandsAndNumbers(argv.clone(), "", words);
        let (paramLinesNot0, rowsAsNumbersNot0, rowsOfcombiNot0, _spaltenreihenfolgeundnurdieseNot, prims2, generRows2) =
            self.parametersToCommandsAndNumbers(argv.clone(), "-", words);

        self.init_dataDict_and_spaltenTypeNaming_python_like();
        self.init_spalten_arten_python_like();
        self.storeParamtersForColumns(words);
        self.produceAllSpaltenNumbers("");

        let (mut paramLines, paramLinesNot) = self.deleteDoublesInSets_py(paramLines0, paramLinesNot0);

        self.rowsAsNumbers = Self::ordered_set_to_vec_i64(
            self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.ordinary).cloned().unwrap_or_default(),
        );
        self.generRows = Self::ordered_set_to_vec_i64(
            self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.generated1).cloned().unwrap_or_default(),
        );
        self.puniverseprims = Self::ordered_set_to_vec_i64(
            self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.concat1).cloned().unwrap_or_default(),
        );
        self.rowsOfcombi = Self::ordered_set_to_vec_i64(
            self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.kombi1).cloned().unwrap_or_default(),
        ).into_iter().map(|v| vec![v.to_string()]).collect();
        self.rowsOfcombi2 = Self::ordered_set_to_vec_i64(
            self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.kombi2).cloned().unwrap_or_default(),
        );
        self.onlyGenerated = Self::ordered_set_to_onlyGenerated_py(
            self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.boolAndTupleSet1).cloned().unwrap_or_default(),
        );
        let mut ones = vec![];
        for a in self.onlyGenerated.clone() {
            if a.len() == 1 {
                ones.extend(a);
            }
        }
        self.getConcat_ones = ones;

        if !self.rowsOfcombi.is_empty() {
            Self::push_unique_string(&mut paramLines, "ka".to_string());
        }
        if !self.rowsOfcombi2.is_empty() {
            Self::push_unique_string(&mut paramLines, "ka2".to_string());
        }

        self.CsvTheirsSpalten = IndexMap::new();
        self.CsvTheirsSpalten.insert(1, self.puniverseprims.clone());
        self.CsvTheirsSpalten.insert(2, Self::ordered_set_to_vec_i64(self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.gebrGal1).cloned().unwrap_or_default()));
        self.CsvTheirsSpalten.insert(3, Self::ordered_set_to_vec_i64(self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.gebrGal1).cloned().unwrap_or_default()));
        self.CsvTheirsSpalten.insert(4, Self::ordered_set_to_vec_i64(self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.gebroUni1).cloned().unwrap_or_default()));
        self.CsvTheirsSpalten.insert(5, Self::ordered_set_to_vec_i64(self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.gebroUni1).cloned().unwrap_or_default()));
        self.CsvTheirsSpalten.insert(6, Self::ordered_set_to_vec_i64(self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.gebrEmo1).cloned().unwrap_or_default()));
        self.CsvTheirsSpalten.insert(7, Self::ordered_set_to_vec_i64(self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.gebrEmo1).cloned().unwrap_or_default()));
        self.CsvTheirsSpalten.insert(8, Self::ordered_set_to_vec_i64(self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.gebrGroe1).cloned().unwrap_or_default()));
        self.CsvTheirsSpalten.insert(9, Self::ordered_set_to_vec_i64(self.spaltenArtenKey_SpaltennummernValue.get(&self.spaltenTypeNaming.gebrGroe1).cloned().unwrap_or_default()));

        self.SpaltenVanillaAmount = self.rowsAsNumbers.len() as i64;
        self.rowsAsNumbersNot = rowsAsNumbersNot0;
        self.rowsOfcombiNot = rowsOfcombiNot0;
        self.spaltenreihenfolgeundnurdiese = spaltenreihenfolgeundnurdiese0;
        self.puniverseprimsNot = prims2;
        self.generRowsNot = generRows2;

        self.setShellRowsAmount();
        self.setShellWidth();
        if self.htmlOrBBcode && !self.breiteORbreiten {
            self.shellRowsAmount = 0;
            self.textWidth = 0;
        }

        self.setRowRangeFromArgv();
        self.setIfZeilenSetToInf();
        self.helpPage();
        self.validate_cli_like_python_for_known_case();
        self.allImportantBeginThingsDone = true;

        (self.RowsLen, paramLines, paramLinesNot, self.relitable.clone(), self.rowsAsNumbers.clone())
    }

    pub fn oberesMaximumArg(&mut self, arg: &str) -> (Vec<i64>, bool) {
        let mut werte: Vec<i64> = vec![];
        if arg.starts_with("--oberesmaximum=") {
            let tail = &arg["--oberesmaximum=".len()..];
            if tail.chars().all(|c| c.is_ascii_digit()) {
                werte = vec![tail.parse::<i64>().unwrap_or(0)];
                return (werte, true);
            }
        } else if arg.starts_with("--vorhervonausschnitt=") {
            let tail = &arg["--vorhervonausschnitt=".len()..];
            if let Some((a,b)) = tail.split_once('-') {
                let start = a.parse::<i64>().unwrap_or(0);
                let end = b.parse::<i64>().unwrap_or(0);
                for w in start..=end {
                    werte.push(std::cmp::max(w + 1, 1024));
                }
                return (werte, false);
            }
        }
        (werte, false)
    }

    pub fn oberesMaximum2(&mut self, argv2: Vec<String>) -> Option<i64> {
        let mut werte: Vec<i64> = vec![];
        for arg in argv2 {
            werte.extend(self.oberesMaximumArg(&arg).0);
        }
        if werte.len() > 0 { Some(*werte.iter().max().unwrap()) } else { None }
    }

    pub fn oberesMaximum(&mut self, arg: &str) -> bool {
        let (liste, wahrheitswert) = self.oberesMaximumArg(arg);
        if liste.len() == 0 || !wahrheitswert {
            return false;
        }
        let max_ = *liste.iter().max().unwrap_or(&self.hoechsteZeile);
        self.hoechsteZeile = max_;
        true
    }

    pub fn invertAlles(&mut self) {
        if self.__invertAlles {
            self.ifPrint = !self.ifPrint;
        }
    }

    pub fn resultingTable(&mut self) -> Vec<Vec<String>> {
        self.__resultingTable.clone()
    }

    fn onlyThatColumns_py(&self, table: Vec<Vec<String>>, onlyThatColumns: Vec<i64>) -> Vec<Vec<String>> {
        if onlyThatColumns.len() == 0 {
            return table;
        }
        let mut newTable: Vec<Vec<String>> = vec![];
        for row in table {
            let mut newCol: Vec<String> = vec![];
            for i in onlyThatColumns.iter() {
                if *i <= 0 {
                    continue;
                }
                let idx = (*i - 1) as usize;
                if idx < row.len() {
                    newCol.push(row[idx].clone());
                }
            }
            if newCol.len() > 0 {
                newTable.push(newCol);
            }
        }
        if newTable.len() > 0 { newTable } else { vec![] }
    }

    fn parse_bereich_to_numbers_py(&self, txt: &str, upper_exclusive: i64) -> BTreeSet<i64> {
        let mut out = BTreeSet::new();
        for raw in txt.split(',') {
            let part = raw.trim();
            if part.is_empty() {
                continue;
            }
            let mut part = part;
            if let Some(rest) = part.strip_prefix('v') {
                part = rest;
            }
            if let Some((a, b)) = part.split_once('-') {
                if let (Ok(start), Ok(end)) = (a.trim().parse::<i64>(), b.trim().parse::<i64>()) {
                    if start <= end {
                        for v in start..=end {
                            if v > 0 && v < upper_exclusive {
                                out.insert(v);
                            }
                        }
                    }
                }
            } else if let Ok(v) = part.parse::<i64>() {
                if v > 0 && v < upper_exclusive {
                    out.insert(v);
                }
            }
        }
        out
    }

    fn teiler_set_py(values: &BTreeSet<i64>) -> BTreeSet<i64> {
        let mut out = BTreeSet::new();
        for &n in values {
            if n <= 0 {
                continue;
            }
            let mut d = 1i64;
            while d * d <= n {
                if n % d == 0 {
                    out.insert(d);
                    out.insert(n / d);
                }
                d += 1;
            }
        }
        out
    }

    fn filter_original_lines_py(&self, highest_line: i64, param_lines: &[String]) -> BTreeSet<i64> {
        let mut num_range: BTreeSet<i64> = (1..=highest_line).collect();
        let effective: BTreeSet<String> = param_lines.iter().cloned().collect();
        let content_only: BTreeSet<String> = effective.iter().filter(|s| s.as_str() != "ka" && s.as_str() != "ka2").cloned().collect();
        if !(effective.contains("all") || content_only.is_empty() || !self.ifZeilenSetted) {
            num_range.clear();
        }

        let mut if_a = false;
        let mut a_parts: Vec<String> = vec![];
        let mut if_w = false;
        for condition in effective.iter() {
            if condition.starts_with("_a_") && condition.len() > 3 {
                if_a = true;
                a_parts.push(condition[3..].to_string());
            }
            if condition.starts_with("_w_") {
                if_w = true;
            }
        }
        if if_a {
            let joined = a_parts.join(",");
            num_range.extend(self.parse_bereich_to_numbers_py(&joined, highest_line + 1));
            if if_w {
                let divisors = Self::teiler_set_py(&num_range.clone());
                num_range.extend(divisors);
            }
        }

        let mut n_parts: Vec<String> = vec![];
        for condition in effective.iter() {
            if condition.starts_with("_n_") && condition.len() > 3 {
                n_parts.push(condition[3..].to_string());
            }
        }
        if !n_parts.is_empty() {
            let joined = n_parts.join(",");
            let n_set = self.parse_bereich_to_numbers_py(&joined, highest_line + 1);
            if num_range.is_empty() && !if_a && !effective.contains("all") {
                num_range = (1..=highest_line).collect();
            }
            if !n_set.is_empty() {
                num_range = num_range.intersection(&n_set).cloned().collect();
            }
        }

        let mut zeit_set: BTreeSet<i64> = BTreeSet::new();
        let mut if_zeit = false;
        for condition in effective.iter() {
            match condition.as_str() {
                "=" => { if_zeit = true; zeit_set.insert(10); }
                "<" => { if_zeit = true; zeit_set.extend(1..10); }
                ">" => { if_zeit = true; zeit_set.extend(11..=highest_line); }
                _ => {}
            }
        }
        if if_zeit {
            if num_range.is_empty() && !if_a && !effective.contains("all") {
                num_range = (1..=highest_line).collect();
            }
            num_range = num_range.intersection(&zeit_set).cloned().collect();
        }

        num_range
    }

    fn prepare4out_py(
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

        let headingsAmount = relitable.first().map(|r| r.len()).unwrap_or(0) as i64;
        let rowsRange: Vec<i64> = (0..headingsAmount).collect();
        let highest_line = std::cmp::max(relitable.len() as i64 - 1, self.hoechsteZeile);

        let mut display_set = self.filter_original_lines_py(highest_line, &paramLines);
        if !paramLinesNot.is_empty() {
            let display_not = self.filter_original_lines_py(highest_line, &paramLinesNot);
            let changed: BTreeSet<i64> = ((1..=highest_line).collect::<BTreeSet<i64>>()
                .difference(&display_not)
                .cloned()
                .collect());
            if !changed.is_empty() {
                display_set = display_set.difference(&display_not).cloned().collect();
            }
        }
        if display_set.is_empty() {
            if self.ifZeilenSetted {
                display_set.clear();
            } else {
                display_set = (0..=highest_line).collect();
            }
        }
        display_set.insert(0);

        let mut display_rows: Vec<i64> = display_set.into_iter().collect();
        display_rows.sort();
        let numlen = display_rows.last().map(|v| v.to_string().len() as i64).unwrap_or(0);

        let selected_cols: BTreeSet<i64> = if rowsAsNumbers.is_empty() {
            (0..headingsAmount).collect()
        } else {
            rowsAsNumbers.iter().cloned().collect()
        };

        for &u in display_rows.iter() {
            let idx = u as usize;
            if idx >= relitable.len() {
                continue;
            }
            let mut new2Lines: Vec<String> = vec![];
            for (t, cell) in relitable[idx].iter().enumerate() {
                if selected_cols.contains(&(t as i64)) {
                    new2Lines.push(cell.clone());
                }
            }
            if !new2Lines.is_empty() {
                newTable.push(new2Lines);
                old2newTable.push(u);
            }
        }

        finallyDisplayLines = old2newTable.iter().map(|n| n.to_string()).collect();
        if !finallyDisplayLines.is_empty() {
            finallyDisplayLines[0] = "".to_string();
        }
        (finallyDisplayLines, newTable, numlen, rowsRange, old2newTable)
    }

    fn wrap_text_py(txt: &str, width: usize) -> Vec<String> {
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
                        let chars: Vec<char> = word.chars().collect();
                        let mut start = 0usize;
                        while start < chars.len() {
                            let end = std::cmp::min(start + width, chars.len());
                            out.push(chars[start..end].iter().collect());
                            start = end;
                        }
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
                        let chars: Vec<char> = word.chars().collect();
                        let mut start = 0usize;
                        while start < chars.len() {
                            let end = std::cmp::min(start + width, chars.len());
                            let piece: String = chars[start..end].iter().collect();
                            if end < chars.len() {
                                out.push(piece);
                            } else {
                                current = piece;
                            }
                            start = end;
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

    fn cliOut_py(
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
            return newTable;
        }

        let mut col_count = 0usize;
        for row in newTable.iter() {
            if row.len() > col_count {
                col_count = row.len();
            }
        }

        let mut widths: Vec<usize> = vec![0; col_count];
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
        for (i, w) in widths.iter_mut().enumerate() {
            if *w == 0 {
                let mut max_len = 1usize;
                for row in newTable.iter() {
                    if i < row.len() {
                        for line in row[i].lines() {
                            let len_ = line.chars().count();
                            if len_ > max_len {
                                max_len = len_;
                            }
                        }
                    }
                }
                *w = std::cmp::min(std::cmp::max(max_len, 12usize), 32usize);
            }
        }

        let num_prefix_width = if self.nummeriere {
            finallyDisplayLines.iter().map(|s| s.chars().count()).max().unwrap_or(0)
        } else {
            0usize
        };

        for (row_idx, row) in newTable.iter().enumerate() {
            let mut wrapped_cells: Vec<Vec<String>> = vec![];
            let mut max_sub = 1usize;
            for i in 0..col_count {
                let cell = if i < row.len() { row[i].as_str() } else { "" };
                let wrapped = Self::wrap_text_py(cell, widths[i]);
                if wrapped.len() > max_sub {
                    max_sub = wrapped.len();
                }
                wrapped_cells.push(wrapped);
            }

            let mut should_skip_row = false;
            if self.keineleereninhalte {
                let joined: String = row.join(" ");
                let stripped = joined.replace('-', "").replace('?', "").trim().to_string();
                if stripped.is_empty() {
                    should_skip_row = true;
                }
            }
            if should_skip_row {
                continue;
            }

            for sub_idx in 0..max_sub {
                let mut line = String::new();
                if self.nummeriere {
                    let label = if sub_idx == 0 {
                        finallyDisplayLines.get(row_idx).cloned().unwrap_or_default()
                    } else {
                        String::new()
                    };
                    line.push_str(&format!("{:>width$} ", label, width=num_prefix_width));
                }
                for i in 0..col_count {
                    let part = wrapped_cells[i].get(sub_idx).cloned().unwrap_or_default();
                    if i + 1 == col_count {
                        line.push_str(&part);
                    } else {
                        line.push_str(&format!("{:<width$} ", part, width=widths[i]));
                    }
                }
                out_lines.push(line.trim_end().to_string());
            }
        }

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

    pub fn workflowEverything(&mut self, argv: Vec<String>, words: &Words) -> Vec<Vec<String>> {
        let (RowsLen, paramLines, paramLinesNot, relitable, rowsAsNumbers) =
            self.bringAllImportantBeginThings(argv, words);

        self.RowsLen = RowsLen;
        self.relitable = relitable.clone();
        self.rowsAsNumbers = rowsAsNumbers.clone();

        if self.helpPage() {
            self.__resultingTable = vec![];
            return vec![];
        }

        if self.cliErrors.len() > 0 {
            self.__resultingTable = vec![];
            return vec![];
        }

        let (finallyDisplayLines, mut newTable, numlen, rowsRange, old2newTable) = self.prepare4out_py(
            paramLines,
            paramLinesNot,
            relitable,
            rowsAsNumbers,
        );

        if self.rowsOfcombi.len() > 0 {
            self.combiTableWorkflow();
        }
        if self.rowsOfcombi2.len() > 0 {
            self.combiTableWorkflow();
        }

        newTable = self.onlyThatColumns_py(newTable, self.spaltenreihenfolgeundnurdiese.clone());
        self.newTable = newTable.len() > 0;
        self.finallyDisplayLines = finallyDisplayLines.clone();
        self.numlen = numlen;
        let _old2newTable = old2newTable.clone();

        let out = self.cliOut_py(finallyDisplayLines, newTable.clone(), numlen, rowsRange);
        self.tableGenerated = self.newTable || !out.is_empty();
        self.__resultingTable = out.clone();
        self.addResultingTableToTables();
        self.setOld2Rows();
        self.setNewerTable();
        self.setOldRows();
        self.setNewerRows();
        self.setRowsOfcombi();
        self.setOldTable();
        self.setGeneratedSpaltenParameter();
        self.setAllEquColumns();
        self.setFinallyDisplayTable();
        out
    }

    pub fn combiTableWorkflow(&mut self) {
        self.tableGenerated = self.tableGenerated || self.newTable;
    }

    pub fn run(&mut self, words: &Words) {
        if !self.__runAlles {
            self.__resultingTable = self.workflowEverything(self.argv.clone(), words);
        }
        self.invertAlles();
        self.printOrStoreLines();
        self.runDone = true;
    }

    pub fn runAllesLikePythonInit(&mut self, words: &Words) {
        if self.__runAlles {
            self.__resultingTable = self.workflowEverything(self.argv.clone(), words);
        }
    }

    pub fn snapshot(&self) -> String {
        format!(
            "paraMainDict={} paraDict={} dataDict0={} dataDict3={} kombi1={} kombi2={} newTable={} argvWithoutProgram={:?} beginDone={} runDone={} hoechsteZeile={} tableGenerated={} relitableRows={} RowsLen={} cliErrors={} sideParas={:?} resultingTableRows={} allesParameters={} spaltenNumbers={} ifPrint={} rowRangeLen={} shellRowsAmount={} shellWidth={} finallyDisplayLines={} ifZeilenSetToInf={} tables={} numlen={} old2Rows={} newerTable={} finallyDisplayLinesByChunks={} rowsOfcombi={} oldRows={} newerRows={} oldTable={} generatedSpaltenParameter={} allEquColumns={} finallyDisplayTable={} bigParamaeter={:?} obZeilenBereicheAngegeben={} breiteHasBeenOnceZero={} breiteORbreiten={} spaltenreihenfolgeundnurdiese={:?} puniverseprims={} generRows={} rowsAsNumbersNot={} rowsOfcombiNot={} htmlOrBBcode={} spaltenArtenKeys={}",
            self.paraMainDict.len(),
            self.paraDict.len(),
            self.dataDicts[0].len(),
            self.dataDicts[3].len(),
            self.kombiReverseDict.len(),
            self.kombiReverseDict2.len(),
            self.newTable,
            self.argvWithoutProgram,
            self.allImportantBeginThingsDone,
            self.runDone,
            self.hoechsteZeile,
            self.tableGenerated,
            self.relitable.len(),
            self.RowsLen,
            self.cliErrors.len(),
            self.sideParas,
            self.__resultingTable.len(),
            self.allesParameters,
            self.spaltenNumbers.len(),
            self.ifPrint,
            self.rowsRangeLen,
            self.shellRowsAmount,
            self.shellWidth,
            self.finallyDisplayLines.len(),
            self.ifZeilenSetToInf,
            self.tables.len(),
            self.numlen,
            self.old2Rows.len(),
            self.newerTable.len(),
            self.finallyDisplayLinesByChunks.len(),
            self.rowsOfcombi.len(),
            self.oldRows.len(),
            self.newerRows.len(),
            self.oldTable.len(),
            self.generatedSpaltenParameter.len(),
            self.allEquColumns.len(),
            self.finallyDisplayTable.len(),
            self.bigParamaeter,
            self.obZeilenBereicheAngegeben,
            self.breiteHasBeenOnceZero,
            self.breiteORbreiten,
            self.spaltenreihenfolgeundnurdiese,
            self.puniverseprims.len(),
            self.generRows.len(),
            self.rowsAsNumbersNot.len(),
            self.rowsOfcombiNot.len(),
            self.htmlOrBBcode,
            self.spaltenArtenKey_SpaltennummernValue.len()
        )
    }
}
