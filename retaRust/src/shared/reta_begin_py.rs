use indexmap::IndexMap;
use std::collections::BTreeSet;

use crate::shared::reta_program_types::{dedup_preserve_order_i64, PairStr, Program, SpaltenTyp};
use crate::shared::words_py::{PyValue, StoreParameterEntry, Words};

impl Program {
    pub(crate) fn help_lines_py(&self) -> Vec<String> {
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

    pub(crate) fn parse_simple_numeric_list_py(&self, txt: &str) -> Vec<i64> {
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

    pub(crate) fn parametersCmdWithSomeBereich_py(&self, txt: &str, suffix: &str, neg: &str, keineNegBeruecksichtigung: bool) -> Vec<String> {
        let mut out = vec![];
        for v in self.parse_simple_numeric_list_py(txt) {
            if neg.is_empty() || keineNegBeruecksichtigung {
                out.push(format!("{}{}", v, suffix));
            }
        }
        out
    }

    pub(crate) fn push_unique_string(target: &mut Vec<String>, value: String) {
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

    pub(crate) fn init_dataDict_and_spaltenTypeNaming_python_like(&mut self) {
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

}
}