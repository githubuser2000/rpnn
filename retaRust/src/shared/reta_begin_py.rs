#![allow(non_snake_case)]
use indexmap::IndexMap;

use crate::doc_tools::markdown_reader::read_doc_file;
use crate::shared::reta_program_types::{dedup_preserve_order_i64, Generated2Selection, GeneratorPairSelection, Program, SpaltenTyp};
use crate::shared::reta_runtime_cache::{shared_reta_static_data, GeneratorFamilyData};
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
            "  --hoehemaximal=2".to_string(),
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
            "  --justtext".to_string(),
            "  --art=shell,html,csv,markdown,bbcode,emacs,nichts".to_string(),
            "  --onetable".to_string(),
            "  --endlessscreen".to_string(),
            "  --endless".to_string(),
            "  --dontwrap".to_string(),
            "  --breite=50".to_string(),
            "  --breiten=20,30,40".to_string(),
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

    fn help_lines_from_readme_py(&self) -> Option<Vec<String>> {
        read_doc_file("readme-reta.md")
            .ok()
            .map(|text| text.replace("\r\n", "\n"))
            .map(|text| text.lines().map(|line| line.to_string()).collect())
    }

    pub fn helpPage(&mut self) -> bool {
        if self.argvWithoutProgram.iter().any(|a| a == "-h" || a == "-help" || a == "--help") {
            self.finallyDisplayLines = self
                .help_lines_from_readme_py()
                .unwrap_or_else(|| self.help_lines_py());
            return true;
        }
        false
    }

    pub fn collect_side_paras_from_argv(&mut self) {
        self.sideParas.clear();
        for token in &self.argvWithoutProgram {
            if token.starts_with("--") {
                self.sideParas.push(token.clone());
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


    pub(crate) fn bereich_to_numbers2_ausgabe_py(txt: &str) -> Vec<i64> {
        let mut ordered: Vec<i64> = vec![];
        for part in Self::split_top_level_commas_py(txt) {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some((a, b)) = trimmed.split_once('-') {
                let a = a.trim();
                let b = b.trim();
                if a.chars().all(|c| c.is_ascii_digit()) && b.chars().all(|c| c.is_ascii_digit()) {
                    let start = a.parse::<i64>().unwrap_or(0);
                    let end = b.parse::<i64>().unwrap_or(0);
                    if start > 0 && end >= start {
                        for value in start..=end {
                            ordered.push(value);
                        }
                        continue;
                    }
                }
            }
            if trimmed.chars().all(|c| c.is_ascii_digit()) {
                ordered.push(trimmed.parse::<i64>().unwrap_or(0));
            }
        }
        if !ordered.is_empty() {
            ordered
        } else {
            Self::bereich_to_numbers2_py(txt, false, 0, false)
                .into_iter()
                .collect()
        }
    }

    fn split_top_level_commas_py(txt: &str) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut current = String::new();
        let mut depth_round = 0i32;
        let mut depth_square = 0i32;
        let mut depth_curly = 0i32;

        for ch in txt.chars() {
            match ch {
                '(' => {
                    depth_round += 1;
                    current.push(ch);
                }
                ')' => {
                    depth_round -= 1;
                    current.push(ch);
                }
                '[' => {
                    depth_square += 1;
                    current.push(ch);
                }
                ']' => {
                    depth_square -= 1;
                    current.push(ch);
                }
                '{' => {
                    depth_curly += 1;
                    current.push(ch);
                }
                '}' => {
                    depth_curly -= 1;
                    current.push(ch);
                }
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

    fn is_plain_zeilen_angabe_between_kommas_py(txt: &str) -> bool {
        let txt = txt.trim();
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
        if !first_ok {
            return false;
        }
        parts.all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
    }

    fn is_zeilen_angabe_between_kommas_py(txt: &str) -> bool {
        let txt = txt.trim();
        if txt.is_empty() {
            return false;
        }
        let stripped_v = txt.strip_prefix('v').unwrap_or(txt);
        let stripped_plain = stripped_v.strip_prefix('-').unwrap_or(stripped_v);
        let generated_after_first = txt
            .char_indices()
            .nth(1)
            .map(|(idx, _)| &txt[idx..])
            .and_then(Self::parse_python_like_int_set_expr_py)
            .is_some();
        (!stripped_plain.is_empty() && Self::is_plain_zeilen_angabe_between_kommas_py(stripped_plain))
            || Self::parse_python_like_int_set_expr_py(txt).is_some()
            || generated_after_first
    }

    fn is_zeilen_angabe_py(txt: &str) -> bool {
        let parts = Self::split_top_level_commas_py(txt);
        let any_at_all = parts.iter().any(|part| !part.is_empty());
        if !any_at_all {
            return false;
        }
        parts.iter().all(|part| part.is_empty() || Self::is_zeilen_angabe_between_kommas_py(part))
    }

    pub(crate) fn parametersCmdWithSomeBereich_py(&self, txt: &str, suffix: &str, neg: &str, keineNegBeruecksichtigung: bool) -> Vec<String> {
        let mut out = vec![];
        if keineNegBeruecksichtigung {
            if Self::is_zeilen_angabe_py(txt) {
                out.push(format!("_{}_{}", suffix, txt));
            }
            return out;
        }
        for ein_bereich in Self::split_top_level_commas_py(txt) {
            if ein_bereich.is_empty() {
                continue;
            }
            let starts_without_minus = !ein_bereich.starts_with('-');
            let starts_with_neg = !neg.is_empty() && ein_bereich.starts_with(neg);
            if (neg.is_empty() && starts_without_minus) || starts_with_neg {
                let payload = if starts_with_neg {
                    ein_bereich[neg.len()..].to_string()
                } else {
                    ein_bereich.clone()
                };
                if Self::is_zeilen_angabe_between_kommas_py(&payload) {
                    out.push(format!("_{}_{}", suffix, payload));
                }
            }
        }
        out
    }

    pub(crate) fn push_unique_string(target: &mut Vec<String>, value: String) {
        if !target.contains(&value) {
            target.push(value);
        }
    }

    pub fn parametersToCommandsAndNumbers(&mut self, argv: &[String], neg: &str, _words: &Words) -> (Vec<String>, Vec<i64>, Vec<Vec<String>>, Vec<i64>, Vec<i64>, Vec<i64>) {
        let mut paramLines: Vec<String> = vec![];
        if argv.len() == 1 && neg.is_empty() {
            self.cliErrors.push("Versuche Parameter -h".to_string());
        }
        let rowsAsNumbers: Vec<i64> = vec![];
        let _rowsOfcombi: Vec<Vec<String>> = vec![];
        let mut spaltenreihenfolgeundnurdiese: Vec<i64> = vec![];
        let puniverseprims_only: Vec<i64> = vec![];
        let generRows: Vec<i64> = vec![];
        self.bigParamaeter.clear();
        self.__willBeOverwritten_rowsOfcombi.clear();

        for arg in argv.iter().skip(1) {
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
                        self.textHeight = tail.trim().parse::<i64>().unwrap_or(self.textHeight);
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
                } else if sub == "keineueberschriften" && neg.is_empty() {
                    self.keineUeberschriften = true;
                } else if sub == "keinenummerierung" && neg.is_empty() {
                    self.nummeriere = false;
                } else if sub == "keineleereninhalte" && neg.is_empty() {
                    self.keineleereninhalte = true;
                } else if let Some(tail) = sub.strip_prefix("spaltenreihenfolgeundnurdiese=") {
                    spaltenreihenfolgeundnurdiese = Self::bereich_to_numbers2_ausgabe_py(tail);
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

        let breite_ist_null = "--breite=0";
        if argv.iter().any(|arg| arg == breite_ist_null) {
            self.breiteBreitenSysArgvPara(&breite_ist_null[2..], "");
        }

        if !self.oneTable {
            self.setShellRowsAmount();
            let normalized_text_width = if self.shellRowsAmount > self.textWidth + 7 || self.shellRowsAmount <= 0 {
                self.textWidth
            } else {
                self.shellRowsAmount - 7
            };
            self.set_text_width_property_py(normalized_text_width);
        }
        self.ifZeilenSetted = self.obZeilenBereicheAngegeben;
        (paramLines, rowsAsNumbers, self.__willBeOverwritten_rowsOfcombi.clone(), spaltenreihenfolgeundnurdiese, puniverseprims_only, generRows)
    }

    pub fn propInfoLog(&mut self, txt: &str) {
        self.finallyDisplayLines.push(txt.to_string());
    }

    pub fn setRowRangeFromArgv(&mut self) {
        self.rowRange = vec![];
        for arg in &self.argvWithoutProgram {
            if let Some(tail) = arg.strip_prefix("--vorhervonausschnitt=") {
                self.rowRange.extend(Self::bereich_to_numbers2_py(tail, false, 0, false));
            }
        }
        self.rowRange = dedup_preserve_order_i64(std::mem::take(&mut self.rowRange));
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

    fn main_parameter_name_py(token: &str) -> Option<&'static str> {
        match token {
            "-debug" => Some("debug"),
            "-zeilen" => Some("zeilen"),
            "-spalten" => Some("spalten"),
            "-kombination" => Some("kombination"),
            "-ausgabe" => Some("ausgabe"),
            "-h" => Some("h"),
            "-help" | "--help" => Some("help"),
            _ => None,
        }
    }

    fn cli_context_error_py(arg: &str) -> String {
        format!(
            "Es muss ein Hauptparameter, bzw. der richtige, gesetzt sein, damit ein Nebenparameter, wie möglicherweise: \"{}\" ausgeführt werden kann. Hauptparameter sind: -zeilen -spalten -kombination -ausgabe -debug -h -help",
            arg
        )
    }

    fn has_numeric_payload_py(raw: &str) -> bool {
        raw.chars().any(|c| c.is_ascii_digit())
    }

    fn is_valid_zeilen_side_parameter_py(arg: &str) -> bool {
        let sub = arg.strip_prefix("--").unwrap_or(arg);
        if matches!(sub, "alles" | "vorhervonausschnittteiler" | "invertieren") {
            return true;
        }
        if let Some(tail) = sub.strip_prefix("hoehemaximal=") {
            return tail.trim().chars().all(|c| c.is_ascii_digit());
        }
        let prefixes = [
            "zeit=",
            "zaehlung=",
            "typ=",
            "primzahlen=",
            "vielfachevonzahlen=",
            "primzahlvielfache=",
            "potenzenvonzahlen=",
            "oberesmaximum=",
            "vorhervonausschnitt=",
            "nachtraeglichneuabzaehlung=",
            "nachtraeglichneuabzaehlungvielfache=",
        ];
        prefixes.iter().any(|prefix| {
            sub.strip_prefix(prefix)
                .map(|tail| !tail.trim().is_empty())
                .unwrap_or(false)
        })
    }

    fn is_valid_ausgabe_side_parameter_py(arg: &str) -> bool {
        let sub = arg.strip_prefix("--").unwrap_or(arg);
        if matches!(
            sub,
            "keineueberschriften"
                | "keinenummerierung"
                | "keineleereninhalte"
                | "nocolor"
                | "justtext"
                | "endlessscreen"
                | "endless"
                | "dontwrap"
                | "onetable"
        ) {
            return true;
        }
        if let Some(tail) = sub.strip_prefix("art=") {
            return matches!(tail, "shell" | "nichts" | "csv" | "bbcode" | "html" | "emacs" | "markdown");
        }
        if let Some(tail) = sub.strip_prefix("spaltenreihenfolgeundnurdiese=") {
            return Self::has_numeric_payload_py(tail);
        }
        sub.starts_with("breite=") || sub.starts_with("breiten=")
    }

    fn is_valid_kombination_side_parameter_py(arg: &str) -> bool {
        let sub = arg.strip_prefix("--").unwrap_or(arg);
        sub.strip_prefix("galaxie=")
            .or_else(|| sub.strip_prefix("universum="))
            .map(|tail| !tail.trim().is_empty())
            .unwrap_or(false)
    }

    fn is_valid_spalten_side_parameter_py(&self, arg: &str) -> bool {
        let sub = arg.strip_prefix("--").unwrap_or(arg);
        if matches!(sub, "alles" | "keinenummerierung") || sub.starts_with("breite=") || sub.starts_with("breiten=") {
            return true;
        }
        let sub = sub.strip_suffix('-').unwrap_or(sub);
        if let Some((main, values)) = sub.split_once('=') {
            if main.trim().is_empty() {
                return false;
            }
            let known_main = self
                .paraDict
                .keys()
                .any(|(stored_main, _)| Self::parameter_main_name_matches_py(stored_main, main));
            if !known_main {
                return false;
            }
            let values = Self::split_parameter_values_py(values);
            if values.is_empty() {
                return false;
            }
            return values.iter().all(|value| {
                let value = value.strip_prefix('-').unwrap_or(value.as_str());
                self.paraDict.keys().any(|(stored_main, stored_value)| {
                    Self::parameter_main_name_matches_py(stored_main, main) && stored_value == value
                }) || Self::has_numeric_payload_py(value)
            });
        }
        self.paraDict
            .keys()
            .any(|(stored_main, _)| Self::parameter_main_name_matches_py(stored_main, sub))
    }

    pub fn validate_cli_context_like_python(&mut self) {
        let mut current_main: Option<&'static str> = None;
        let mut emitted: Vec<String> = Vec::new();
        for arg in self.argvWithoutProgram.clone() {
            if let Some(main) = Self::main_parameter_name_py(&arg) {
                current_main = Some(main);
                continue;
            }
            if !arg.starts_with("--") {
                continue;
            }
            let valid = match current_main {
                Some("zeilen") => Self::is_valid_zeilen_side_parameter_py(&arg),
                Some("spalten") => self.is_valid_spalten_side_parameter_py(&arg),
                Some("kombination") => Self::is_valid_kombination_side_parameter_py(&arg),
                Some("ausgabe") => Self::is_valid_ausgabe_side_parameter_py(&arg),
                _ => false,
            };
            if !valid {
                let msg = Self::cli_context_error_py(&arg);
                if !emitted.contains(&msg) && !self.cliErrors.contains(&msg) {
                    emitted.push(msg.clone());
                    self.cliErrors.push(msg);
                }
            }
        }
    }

    fn side_parameter_seen_under_main_py(&self, expected_main: &str, expected_side: &str) -> bool {
        let mut current_main: Option<&'static str> = None;
        for arg in &self.argvWithoutProgram {
            if let Some(main) = Self::main_parameter_name_py(arg) {
                current_main = Some(main);
                continue;
            }
            if arg == expected_side && current_main.map(|main| main == expected_main).unwrap_or(false) {
                return true;
            }
        }
        false
    }


    fn is_main_parameter_token_py(token: &str) -> bool {
        matches!(token, "-debug" | "-zeilen" | "-spalten" | "-kombination" | "-ausgabe" | "-h" | "-help" | "--help")
    }

    fn side_paras_for_spalten_context_py(&self) -> Vec<String> {
        let mut out: Vec<String> = vec![];
        let mut in_spalten = false;
        for token in &self.argvWithoutProgram {
            if Self::is_main_parameter_token_py(token) {
                in_spalten = token == "-spalten";
                continue;
            }
            if in_spalten && token.starts_with("--") {
                out.push(token.clone());
            }
        }
        out
    }

    fn split_parameter_values_py(raw: &str) -> Vec<String> {
        raw.split(',')
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string())
            .collect()
    }

    fn push_unique_pair_py(target: &mut Vec<(i64, i64)>, value: (i64, i64)) {
        if !target.contains(&value) {
            target.push(value);
        }
    }

    fn push_unique_option_i64_py(target: &mut Vec<Option<i64>>, value: Option<i64>) {
        if !target.contains(&value) {
            target.push(value);
        }
    }

    fn push_unique_string_py(target: &mut Vec<String>, value: String) {
        if !target.iter().any(|existing| existing == &value) {
            target.push(value);
        }
    }

    pub(crate) fn parameter_main_name_matches_local_py(stored: &str, cmd: &str) -> bool {
        let normalize = |value: &str| -> String {
            match value.trim().to_ascii_lowercase().as_str() {
                "multiplikationen" | "primvielfache" => "primvielfache".to_string(),
                other => other.to_string(),
            }
        };
        normalize(stored) == normalize(cmd)
    }

    pub(crate) fn entry_matches_main_and_sub_py(entry: &StoreParameterEntry, main_name: &str, sub_name: &str) -> bool {
        entry.parameterMainNames
            .iter()
            .any(|candidate| Self::parameter_main_name_matches_local_py(candidate, main_name))
            && entry
                .parameterNames
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(sub_name))
    }

    fn push_unique_generated2_selection_py(target: &mut Vec<Generated2Selection>, value: Generated2Selection) {
        if !target.iter().any(|existing| existing == &value) {
            target.push(value);
        }
    }

    fn push_unique_pair_selection_py(target: &mut Vec<GeneratorPairSelection>, value: GeneratorPairSelection) {
        if !target.iter().any(|existing| existing == &value) {
            target.push(value);
        }
    }

    pub(crate) fn append_generated_family_from_entry_py(
        &self,
        entry: &StoreParameterEntry,
        generated1_pairs: &mut Vec<(i64, i64)>,
        generated1_selections: &mut Vec<GeneratorPairSelection>,
        generated2_codes: &mut Vec<String>,
        generated2_selections: &mut Vec<Generated2Selection>,
        bool_and_tuple_set1_options: &mut Vec<Option<i64>>,
        metakonkret_pairs: &mut Vec<(i64, i64)>,
        metakonkret_selections: &mut Vec<GeneratorPairSelection>,
    ) {
        for value in entry.datas.get(self.spaltenTypeNaming.generated1.1).into_iter().flatten() {
            if let PyValue::Tuple(inner) = value {
                let numbers: Vec<i64> = inner.iter().filter_map(|item| match item { PyValue::Int(n) => Some(*n), _ => None }).collect();
                if numbers.len() >= 2 {
                    Self::push_unique_pair_py(generated1_pairs, (numbers[0], numbers[1]));
                    Self::push_unique_pair_selection_py(generated1_selections, GeneratorPairSelection {
                        parameter_main_name: entry.parameterMainNames.first().cloned().unwrap_or_default(),
                        parameter_name: entry.parameterNames.first().cloned().unwrap_or_default(),
                        left: numbers[0],
                        right: numbers[1],
                    });
                }
            }
        }

        for value in entry.datas.get(self.spaltenTypeNaming.generated2.1).into_iter().flatten() {
            if let PyValue::Str(code) = value {
                Self::push_unique_string_py(generated2_codes, code.clone());
                Self::push_unique_generated2_selection_py(
                    generated2_selections,
                    Generated2Selection {
                        parameter_main_name: entry.parameterMainNames.first().cloned().unwrap_or_default(),
                        parameter_name: entry.parameterNames.first().cloned().unwrap_or_default(),
                        code: code.clone(),
                    },
                );
            }
        }

        for value in entry.datas.get(self.spaltenTypeNaming.boolAndTupleSet1.1).into_iter().flatten() {
            if let PyValue::Tuple(inner) = value {
                let option = inner.iter().find_map(|item| match item {
                    PyValue::Int(n) => Some(Some(*n)),
                    PyValue::NoneValue => Some(None),
                    _ => None,
                });
                if let Some(option) = option {
                    Self::push_unique_option_i64_py(bool_and_tuple_set1_options, option);
                }
            }
        }

        for value in entry.datas.get(self.spaltenTypeNaming.metakonkret.1).into_iter().flatten() {
            if let PyValue::Tuple(inner) = value {
                let numbers: Vec<i64> = inner.iter().filter_map(|item| match item { PyValue::Int(n) => Some(*n), _ => None }).collect();
                if numbers.len() >= 2 {
                    Self::push_unique_pair_py(metakonkret_pairs, (numbers[0], numbers[1]));
                    Self::push_unique_pair_selection_py(metakonkret_selections, GeneratorPairSelection {
                        parameter_main_name: entry.parameterMainNames.first().cloned().unwrap_or_default(),
                        parameter_name: entry.parameterNames.first().cloned().unwrap_or_default(),
                        left: numbers[0],
                        right: numbers[1],
                    });
                }
            }
        }
    }

    fn merge_generator_family_from_cache_py(target: &mut GeneratorFamilyData, source: &GeneratorFamilyData) {
        for value in &source.generated1_pairs {
            Self::push_unique_pair_py(&mut target.generated1_pairs, *value);
        }
        for value in &source.generated1_selections {
            Self::push_unique_pair_selection_py(&mut target.generated1_selections, value.clone());
        }
        for value in &source.generated2_codes {
            Self::push_unique_string_py(&mut target.generated2_codes, value.clone());
        }
        for value in &source.generated2_selections {
            Self::push_unique_generated2_selection_py(&mut target.generated2_selections, value.clone());
        }
        for value in &source.bool_and_tuple_set1_options {
            Self::push_unique_option_i64_py(&mut target.bool_and_tuple_set1_options, *value);
        }
        for value in &source.metakonkret_pairs {
            Self::push_unique_pair_py(&mut target.metakonkret_pairs, *value);
        }
        for value in &source.metakonkret_selections {
            Self::push_unique_pair_selection_py(&mut target.metakonkret_selections, value.clone());
        }
    }

    fn parse_exact_generator_selections_from_words_py(&self, words: &Words) -> (Vec<(i64, i64)>, Vec<GeneratorPairSelection>, Vec<String>, Vec<Generated2Selection>, Vec<Option<i64>>, Vec<(i64, i64)>, Vec<GeneratorPairSelection>) {
        let cached = shared_reta_static_data(words);
        let spalten_side_paras = self.side_paras_for_spalten_context_py();
        let run_all_generator_families = spalten_side_paras.iter().any(|token| token == "--alles");
        let mut merged = GeneratorFamilyData::default();

        if run_all_generator_families {
            Self::merge_generator_family_from_cache_py(&mut merged, &cached.generator_all);
        }

        for side_para in spalten_side_paras {
            if side_para == "--alles" || !side_para.starts_with("--") {
                continue;
            }
            let Some((main_name_raw, sub_names_raw)) = side_para[2..].split_once('=') else {
                continue;
            };
            let normalized_main = match main_name_raw.trim().to_ascii_lowercase().as_str() {
                "multiplikationen" | "primvielfache" => "primvielfache".to_string(),
                other => other.to_string(),
            };
            let sub_names = Self::split_parameter_values_py(sub_names_raw);

            for sub_name in sub_names {
                let normalized_sub = sub_name.trim().to_ascii_lowercase();
                let key = (normalized_main.clone(), normalized_sub.clone());
                if let Some(found) = cached.generator_lookup.get(&key) {
                    Self::merge_generator_family_from_cache_py(&mut merged, found);
                    continue;
                }

                let mut fallback = GeneratorFamilyData::default();
                for entry in &words.paraNdataMatrix {
                    if Self::entry_matches_main_and_sub_py(entry, &normalized_main, &normalized_sub) {
                        self.append_generated_family_from_entry_py(
                            entry,
                            &mut fallback.generated1_pairs,
                            &mut fallback.generated1_selections,
                            &mut fallback.generated2_codes,
                            &mut fallback.generated2_selections,
                            &mut fallback.bool_and_tuple_set1_options,
                            &mut fallback.metakonkret_pairs,
                            &mut fallback.metakonkret_selections,
                        );
                    }
                }
                Self::merge_generator_family_from_cache_py(&mut merged, &fallback);
            }
        }

        (
            merged.generated1_pairs,
            merged.generated1_selections,
            merged.generated2_codes,
            merged.generated2_selections,
            merged.bool_and_tuple_set1_options,
            merged.metakonkret_pairs,
            merged.metakonkret_selections,
        )
    }



    fn apply_kombination_args_after_reverse_dicts_py(&mut self, neg: &str) {
        let mut in_kombination = false;
        for arg in &self.argvWithoutProgram {
            if arg == "-kombination" {
                in_kombination = neg.is_empty();
                continue;
            }
            if arg == "--kombination" {
                continue;
            }
            if arg.starts_with('-') && !arg.starts_with("--") {
                in_kombination = false;
                continue;
            }
            if !in_kombination || !arg.starts_with("--") {
                continue;
            }
            let sub = &arg[2..];
            let Some((left, right)) = sub.split_once('=') else {
                continue;
            };
            if left == "galaxie" {
                for raw_single in right.split(',') {
                    let single = raw_single.trim();
                    if single.is_empty() {
                        continue;
                    }
                    let starts_with_neg = single.starts_with('-');
                    let lookup = if starts_with_neg { &single[1..] } else { single };
                    let yes1 = if starts_with_neg { neg == "-" } else { neg.is_empty() };
                    if !yes1 {
                        continue;
                    }
                    if let Some(v) = self.kombiReverseDict.get(lookup) {
                        self.spaltenArtenKey_SpaltennummernValue
                            .entry(self.spaltenTypeNaming.kombi1)
                            .or_default()
                            .insert(*v);
                    }
                }
            } else if left == "universum" {
                for raw_single in right.split(',') {
                    let single = raw_single.trim();
                    if single.is_empty() {
                        continue;
                    }
                    let starts_with_neg = single.starts_with('-');
                    let lookup = if starts_with_neg { &single[1..] } else { single };
                    let yes1 = if starts_with_neg { neg == "-" } else { neg.is_empty() };
                    if !yes1 {
                        continue;
                    }
                    if let Some(v) = self.kombiReverseDict2.get(lookup) {
                        self.spaltenArtenKey_SpaltennummernValue
                            .entry(self.spaltenTypeNaming.kombi2)
                            .or_default()
                            .insert(*v);
                    }
                }
            }
        }
    }

    pub fn bringAllImportantBeginThings(&mut self, argv: Vec<String>, words: &Words) -> (i64, Vec<String>, Vec<String>, Vec<Vec<String>>, Vec<i64>) {
        self.argvWithoutProgram = argv.iter().skip(1).cloned().collect();
        let _ = self.load_religion_csv_exact();
        self.htmlOrBBcode = false;
        self.breiteORbreiten = false;
        self.keineleereninhalte = false;
        self.keineUeberschriften = false;
        self.nummeriere = true;
        self.oneTable = false;
        self.nocolor = false;
        self.outType = "shell".to_string();
        self.breite = 0;
        self.breiten = vec![];
        self.textWidth = 21;
        self.textHeight = 0;
        self.shellRowsAmount = 0;
        self.shellWidth = 0;
        self.spaltenreihenfolgeundnurdiese = vec![];
        self.generatedSpaltenParameter_Exact.clear();
        self.generatedSpaltenParameter_Tags.clear();
        self.breiteHasBeenOnceZero = false;

        let (paramLines0, _rowsAsNumbers0, _rowsOfcombi0, spaltenreihenfolgeundnurdiese0, _prims1, _generRows1) =
            self.parametersToCommandsAndNumbers(&argv, "", words);
        let (paramLinesNot0, rowsAsNumbersNot0, rowsOfcombiNot0, _spaltenreihenfolgeundnurdieseNot, prims2, generRows2) =
            self.parametersToCommandsAndNumbers(&argv, "-", words);

        let cached_runtime = shared_reta_static_data(words);
        self.init_dataDict_and_spaltenTypeNaming_python_like();
        self.init_spalten_arten_python_like();
        if self.__invertAlles {
            self.storeParamtersForColumns(words);
        } else {
            self.paraMainDict = cached_runtime.paraMainDict.clone();
            self.paraDict = cached_runtime.paraDict.clone();
            self.dataDicts = cached_runtime.dataDicts.clone();
            self.dataDict = cached_runtime.dataDict.clone();
            self.kombiReverseDict = cached_runtime.kombiReverseDict.clone();
            self.kombiReverseDict2 = cached_runtime.kombiReverseDict2.clone();
            self.paraDictGenerated = cached_runtime.paraDictGenerated.clone();
            self.paraDictGenerated4htmlTags = cached_runtime.paraDictGenerated4htmlTags.clone();
            self.spaltenTypeNaming = cached_runtime.spaltenTypeNaming.clone();
            self.AllSimpleCommandSpalten = cached_runtime.AllSimpleCommandSpalten.clone();
            self.spaltenArtenKey_SpaltennummernValue = cached_runtime.spaltenArtenKeyTemplate.clone();
        }
        self.produceAllSpaltenNumbers("");
        self.apply_kombination_args_after_reverse_dicts_py("");
        self.apply_kombination_args_after_reverse_dicts_py("-");

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
        let (generated1Pairs_exact, generated1Selections_exact, generated2Codes_exact, generated2Selections_exact, boolAndTupleSet1Options_exact, metakonkretPairs_exact, metakonkretSelections_exact) =
            self.parse_exact_generator_selections_from_words_py(words);
        self.generated1Pairs = generated1Pairs_exact;
        self.generated1Selections = generated1Selections_exact;
        let mut ones = vec![];
        for a in self.onlyGenerated.clone() {
            if a.len() == 1 {
                ones.extend(a);
            }
        }
        self.getConcat_ones = ones;

        self.generated2Codes = generated2Codes_exact;
        self.generated2Selections = generated2Selections_exact;
        self.boolAndTupleSet1Options = boolAndTupleSet1Options_exact;
        self.metakonkretPairs = metakonkretPairs_exact;
        self.metakonkretSelections = metakonkretSelections_exact;

        let has_alles_spalten = self.side_parameter_seen_under_main_py("spalten", "--alles");
        if has_alles_spalten {
            let mut merged_direct = self.rowsAsNumbers.clone();
            for n in self.AllSimpleCommandSpalten.iter().copied() {
                if !merged_direct.contains(&n) {
                    merged_direct.push(n);
                }
            }
            self.rowsAsNumbers = merged_direct;
            let ordinary_key = self.spaltenTypeNaming.ordinary;
            let ordinary_set = self.spaltenArtenKey_SpaltennummernValue.entry(ordinary_key).or_default();
            for n in self.rowsAsNumbers.iter().copied() {
                ordinary_set.insert(n);
            }
        }

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
        self.apply_concat_csv_generators_py();
        self.rowsAsNumbersNot = rowsAsNumbersNot0;
        self.rowsOfcombiNot = rowsOfcombiNot0;
        self.spaltenreihenfolgeundnurdiese = spaltenreihenfolgeundnurdiese0;
        self.puniverseprimsNot = prims2;
        self.generRowsNot = generRows2;

        self.setShellRowsAmount();
        self.setShellWidth();
        if self.htmlOrBBcode && !self.breiteORbreiten {
            self.shellRowsAmount = 0;
            self.set_text_width_property_py(0);
        }

        self.setRowRangeFromArgv();
        self.setIfZeilenSetToInf();
        self.helpPage();
        self.validate_cli_context_like_python();
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
            let werte_list: Vec<i64> = Self::bereich_to_numbers2_py(tail, false, 0, false)
                .into_iter()
                .map(|a| a.saturating_add(1))
                .collect();
            werte = werte_list
                .into_iter()
                .map(|w| std::cmp::max(w, 1024))
                .collect();
            return (werte, false);
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
        self.__invertAlles = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn empty_words() -> Words {
        Words {
            paraNdataMatrix: vec![],
            kombiParaNdataMatrix: IndexMap::new(),
            kombiParaNdataMatrix2: IndexMap::new(),
        }
    }

    #[test]
    fn help_page_uses_same_readme_source_as_python() {
        let mut program = Program::new(vec!["reta".to_string(), "-h".to_string()]);
        assert!(program.helpPage());

        let expected_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("doc")
            .join("readme-reta.md");
        let expected = fs::read_to_string(expected_path).expect("readme-reta.md must exist");
        let expected_lines: Vec<String> = expected
            .replace("\r\n", "\n")
            .lines()
            .map(|line| line.to_string())
            .collect();

        assert_eq!(program.finallyDisplayLines, expected_lines);
        assert!(program.finallyDisplayLines.iter().any(|line| line.contains("## -zeilen")));
    }

    #[test]
    fn zeilen_parser_accepts_all_python_parameters_and_sets_height() {
        let words = empty_words();
        let argv = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--alles".to_string(),
            "--zeit=gestern,heute,morgen".to_string(),
            "--zaehlung=1,2,3".to_string(),
            "--hoehemaximal=2".to_string(),
            "--typ=sonne,mond,planet,schwarzesonne,SonneMitMondanteil".to_string(),
            "--primzahlen=aussenalle,innenalle,aussenerste,innenerste".to_string(),
            "--vielfachevonzahlen=2,3".to_string(),
            "--primzahlvielfache=2,3".to_string(),
            "--vorhervonausschnitt=1-3,5".to_string(),
            "--vorhervonausschnittteiler".to_string(),
            "--nachtraeglichneuabzaehlung=2".to_string(),
            "--nachtraeglichneuabzaehlungvielfache=2".to_string(),
            "--potenzenvonzahlen=2,3".to_string(),
            "--oberesmaximum=2000".to_string(),
            "--invertieren".to_string(),
        ];
        let mut program = Program::new(argv.clone());
        let _ = program.parametersToCommandsAndNumbers(&argv, "", &words);

        assert!(program.cliErrors.is_empty(), "unexpected zeilen parser errors: {:?}", program.cliErrors);
        assert_eq!(program.textHeight, 2);
    }
}

