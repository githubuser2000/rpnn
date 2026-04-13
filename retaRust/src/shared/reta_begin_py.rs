#![allow(non_snake_case)]
use indexmap::IndexMap;

use crate::shared::reta_program_types::{Generated2Selection, Program, SpaltenTyp};
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


    pub(crate) fn bereich_to_numbers2_ausgabe_py(txt: &str) -> Vec<i64> {
        let parsed: Vec<i64> = Self::bereich_to_numbers2_py(txt, false, 0, false)
            .into_iter()
            .collect();
        if parsed.is_empty() {
            let mut fallback = vec![];
            for part in Self::split_top_level_commas_py(txt) {
                let trimmed = part.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if let Ok(v) = trimmed.parse::<i64>() {
                    fallback.push(v);
                }
            }
            fallback
        } else {
            parsed
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

    fn is_zeilen_angabe_between_kommas_py(txt: &str) -> bool {
        let txt = txt.trim();
        if txt.is_empty() {
            return false;
        }
        let txt = if let Some(rest) = txt.strip_prefix('v') { rest } else { txt };
        let txt = if let Some(rest) = txt.strip_prefix('-') { rest } else { txt };
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

    pub fn parametersToCommandsAndNumbers(&mut self, argv: Vec<String>, neg: &str, _words: &Words) -> (Vec<String>, Vec<i64>, Vec<Vec<String>>, Vec<i64>, Vec<i64>, Vec<i64>) {
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
                for part in Self::split_top_level_commas_py(tail) {
                    let part = part.trim();
                    if part.is_empty() {
                        continue;
                    }
                    if let Some((a, b)) = part.split_once('-') {
                        let start = a.parse::<i64>().unwrap_or(0);
                        let end = b.parse::<i64>().unwrap_or(0);
                        if start > 0 && end >= start {
                            for v in start..=end {
                                self.rowRange.push(v);
                            }
                        }
                    } else if let Ok(v) = part.parse::<i64>() {
                        if v > 0 {
                            self.rowRange.push(v);
                        }
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

    fn parameter_main_name_matches_local_py(stored: &str, cmd: &str) -> bool {
        let normalize = |value: &str| -> String {
            match value.trim().to_ascii_lowercase().as_str() {
                "multiplikationen" | "primvielfache" => "primvielfache".to_string(),
                other => other.to_string(),
            }
        };
        normalize(stored) == normalize(cmd)
    }

    fn entry_matches_main_and_sub_py(entry: &StoreParameterEntry, main_name: &str, sub_name: &str) -> bool {
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

    fn append_generated_family_from_entry_py(
        &self,
        entry: &StoreParameterEntry,
        generated1_pairs: &mut Vec<(i64, i64)>,
        generated2_codes: &mut Vec<String>,
        generated2_selections: &mut Vec<Generated2Selection>,
        bool_and_tuple_set1_options: &mut Vec<Option<i64>>,
        metakonkret_pairs: &mut Vec<(i64, i64)>,
    ) {
        for value in entry.datas.get(self.spaltenTypeNaming.generated1.1).into_iter().flatten() {
            if let PyValue::Tuple(inner) = value {
                let numbers: Vec<i64> = inner.iter().filter_map(|item| match item { PyValue::Int(n) => Some(*n), _ => None }).collect();
                if numbers.len() >= 2 {
                    Self::push_unique_pair_py(generated1_pairs, (numbers[0], numbers[1]));
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
                }
            }
        }
    }

    fn parse_exact_generator_selections_from_words_py(&self, words: &Words) -> (Vec<(i64, i64)>, Vec<String>, Vec<Generated2Selection>, Vec<Option<i64>>, Vec<(i64, i64)>) {
        let mut generated1_pairs: Vec<(i64, i64)> = vec![];
        let mut generated2_codes: Vec<String> = vec![];
        let mut generated2_selections: Vec<Generated2Selection> = vec![];
        let mut bool_and_tuple_set1_options: Vec<Option<i64>> = vec![];
        let mut metakonkret_pairs: Vec<(i64, i64)> = vec![];

        let spalten_side_paras = self.side_paras_for_spalten_context_py();
        let run_all_generator_families = spalten_side_paras.iter().any(|token| token == "--alles");

        if run_all_generator_families {
            for entry in &words.paraNdataMatrix {
                self.append_generated_family_from_entry_py(
                    entry,
                    &mut generated1_pairs,
                    &mut generated2_codes,
                    &mut generated2_selections,
                    &mut bool_and_tuple_set1_options,
                    &mut metakonkret_pairs,
                );
            }
        }

        for side_para in spalten_side_paras {
            if side_para == "--alles" || !side_para.starts_with("--") {
                continue;
            }
            let Some((main_name_raw, sub_names_raw)) = side_para[2..].split_once('=') else {
                continue;
            };
            let main_name = main_name_raw.trim();
            let sub_names = Self::split_parameter_values_py(sub_names_raw);

            for sub_name in sub_names {
                for entry in &words.paraNdataMatrix {
                    if Self::entry_matches_main_and_sub_py(entry, main_name, &sub_name) {
                        self.append_generated_family_from_entry_py(
                            entry,
                            &mut generated1_pairs,
                            &mut generated2_codes,
                            &mut generated2_selections,
                            &mut bool_and_tuple_set1_options,
                            &mut metakonkret_pairs,
                        );
                    }
                }
            }
        }

        (generated1_pairs, generated2_codes, generated2_selections, bool_and_tuple_set1_options, metakonkret_pairs)
    }



    fn apply_kombination_args_after_reverse_dicts_py(&mut self, neg: &str) {
        let mut in_kombination = false;
        for arg in self.argvWithoutProgram.clone() {
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
        let (generated1Pairs_exact, generated2Codes_exact, generated2Selections_exact, boolAndTupleSet1Options_exact, metakonkretPairs_exact) =
            self.parse_exact_generator_selections_from_words_py(words);
        self.generated1Pairs = generated1Pairs_exact;
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

        let has_alles_spalten = self.argvWithoutProgram.iter().any(|a| a == "--alles");
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
        self.__invertAlles = true;
    }
}