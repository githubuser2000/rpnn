use indexmap::IndexMap;
use crate::shared::words_py::{Words, PyValue};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PairStr(pub String, pub String);

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
        }
    }

    pub fn produceAllSpaltenNumbers(&mut self) -> Vec<i64> {
        let mut spaltenNumbers: Vec<i64> = vec![];
        for key in self.dataDicts[0].keys() {
            let cleaned = key
                .replace("Int(", "")
                .replace(")", "")
                .replace("PyValue::", "")
                .trim()
                .to_string();
            if let Ok(v) = cleaned.parse::<i64>() {
                spaltenNumbers.push(v);
            }
        }
        spaltenNumbers.sort();
        spaltenNumbers.dedup();
        self.spaltenNumbers = spaltenNumbers.clone();
        spaltenNumbers
    }

    pub fn breiteBreitenSysArgvPara(&mut self, argv: Vec<String>) {
        self.breite = 0;
        self.breiten = vec![];
        for arg in argv {
            if let Some(tail) = arg.strip_prefix("--breite=") {
                self.breite = tail.parse::<i64>().unwrap_or(0);
            }
            if let Some(tail) = arg.strip_prefix("--breiten=") {
                self.breiten = tail
                    .split(',')
                    .filter_map(|x| x.parse::<i64>().ok())
                    .collect::<Vec<i64>>();
            }
        }
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
        let mut dataDicts = {
            let mut x = vec![];
            for _ in 0..12 { x.push(IndexMap::new()); }
            x
        };

        for parameterEntry in words.paraNdataMatrix.iter() {
            let (paraMainDict2, paraDict2, dataDicts2) = self.intoParameterDatatype(
                &parameterEntry.parameterMainNames,
                &parameterEntry.parameterNames,
                &parameterEntry.datas,
            );
            let (paraMainDict3, paraDict3, dataDicts3) = self.mergeParameterDicts(
                self.paraMainDict.clone(),
                self.paraDict.clone(),
                dataDicts.clone(),
                paraMainDict2,
                paraDict2,
                dataDicts2,
            );

            self.paraMainDict = paraMainDict3;
            self.paraDict = paraDict3;
            dataDicts = dataDicts3;
        }

        self.dataDicts = dataDicts;
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
        let mut paraDict: IndexMap<(String, String), Vec<Vec<PyValue>>> = IndexMap::new();

        for parameterMainName in parameterMainNames {
            paraMainDict.insert(parameterMainName.clone(), parameterNames.clone());
            if parameterNames.len() > 0 {
                for parameterName in parameterNames {
                    paraDict.insert((parameterMainName.clone(), parameterName.clone()), datas.clone());
                }
            } else {
                paraDict.insert((parameterMainName.clone(), "".to_string()), datas.clone());
            }
        }

        let mut dataDicts: Vec<IndexMap<String, Vec<Vec<PairStr>>>> = vec![];
        for _ in 0..12 {
            dataDicts.push(IndexMap::new());
        }

        for (i, d) in datas.iter().enumerate() {
            for spaltenNummerOderEtc in d {
                let mut into: Vec<PairStr> = vec![];
                let mut parameterMainNamePerLoop: Vec<String> = vec![];

                for parameterMainName in parameterMainNames {
                    let parameterNames2 =
                        if parameterNames.len() > 0 { parameterNames.clone() } else { vec!["".to_string()] };

                    for parameterName in parameterNames2 {
                        into.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                        if matches!(i, 5 | 6 | 9 | 10) {
                            parameterMainNamePerLoop.push(parameterName.clone());
                        }
                    }
                }

                let case_num =
                    if i == 4 && matches!(spaltenNummerOderEtc, PyValue::Bool(_)) { 1 }
                    else if matches!(i, 5 | 6 | 9 | 10) { 2 }
                    else if i == 4 { 4 }
                    else { 3 };

                let index1 = if case_num != 1 { i } else { 3 };
                let index2a: Vec<String>;
                let intoA: Vec<Vec<PairStr>>;

                if case_num == 1 {
                    index2a = vec!["('bool', 0)".to_string()];
                    intoA = vec![into.clone()];
                } else if case_num == 2 {
                    if let PyValue::Tuple(inner) = spaltenNummerOderEtc {
                        index2a = inner.iter().map(|x| format!("{:?}", x)).collect();
                    } else {
                        index2a = vec![format!("{:?}", parameterMainNamePerLoop)];
                    }
                    intoA = into.iter().map(|x| vec![x.clone()]).collect();
                } else {
                    index2a = vec![format!("{:?}", spaltenNummerOderEtc)];
                    intoA = vec![into.clone()];
                }

                let max_len = if index2a.len() > intoA.len() { index2a.len() } else { intoA.len() };
                for pos in 0..max_len {
                    let index2 = if pos < index2a.len() { index2a[pos].clone() } else { "None".to_string() };
                    let into2 = if pos < intoA.len() { intoA[pos].clone() } else { vec![] };
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
        let paraMainDict1 =
            paraMainDict1.into_iter().chain(paraMainDict2.into_iter()).collect::<IndexMap<String, Vec<String>>>();
        let paraDict1 =
            paraDict1.into_iter().chain(paraDict2.into_iter()).collect::<IndexMap<(String, String), Vec<Vec<PyValue>>>>();
        let mut dataDicts3 = dataDicts1.clone();

        let max_len = if dataDicts1.len() > dataDicts2.len() { dataDicts1.len() } else { dataDicts2.len() };
        for i in 0..max_len {
            let dict1 = dataDicts1.get(i);
            let dict2 = dataDicts2.get(i);
            match (dict1, dict2) {
                (Some(d1), Some(d2)) => {
                    if dataDicts3[i].keys().len() == 0 {
                        dataDicts3[i] = d2.clone();
                    } else {
                        for (key1, value1) in d1 {
                            for (key2, value2) in d2 {
                                if key2 == key1 {
                                    let entry = dataDicts3[i].entry(key1.clone()).or_insert_with(Vec::new);
                                    entry.extend(value2.clone());
                                } else if !dataDicts3[i].contains_key(key2) {
                                    dataDicts3[i].insert(key2.clone(), value2.clone());
                                }
                            }
                            if !dataDicts3[i].contains_key(key1) {
                                dataDicts3[i].insert(key1.clone(), value1.clone());
                            }
                        }
                    }
                }
                (Some(d1), None) => dataDicts3[i] = d1.clone(),
                (None, Some(d2)) => {
                    if i >= dataDicts3.len() { dataDicts3.push(d2.clone()); } else { dataDicts3[i] = d2.clone(); }
                }
                (None, None) => {}
            }
        }

        (paraMainDict1, paraDict1, dataDicts3)
    }

    pub fn helpPage(&mut self) -> bool {
        if self.argvWithoutProgram.iter().any(|a| a == "-h" || a == "-help" || a == "--help") {
            self.finallyDisplayLines = vec!["help".to_string()];
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

    pub fn parametersToCommandsAndNumbers(&mut self, words: &Words) {
        self.storeParamtersForColumns(words);
        self.collect_side_paras_from_argv();
        let mut lastParameterType = "".to_string();

        for token in self.argvWithoutProgram.clone() {
            if token.starts_with("-") && !token.starts_with("--") {
                lastParameterType = token[1..].to_string();
                continue;
            }

            if token.starts_with("--") {
                let cmd = token[2..].to_string();

                if lastParameterType == "spalten" {
                    if let Some(eq) = cmd.find('=') {
                        let main = cmd[..eq].to_string();
                        let right = cmd[eq + 1..].to_string();

                        for single in right.split(',') {
                            if self.paraDict.contains_key(&(main.clone(), single.to_string())) {
                                self.newTable = true;
                            }
                        }
                    } else if self.paraDict.contains_key(&(cmd.clone(), "".to_string()))
                        || self.paraMainDict.contains_key(&cmd)
                    {
                        self.newTable = true;
                    }
                } else if lastParameterType == "kombi" {
                    if let Some(eq) = cmd.find('=') {
                        let left = cmd[..eq].to_string();
                        let right = cmd[eq + 1..].to_string();

                        if left == "galaxie" {
                            for single in right.split(',') {
                                if self.kombiReverseDict.contains_key(single) {
                                    self.newTable = true;
                                }
                            }
                        } else if left == "universum" {
                            for single in right.split(',') {
                                if self.kombiReverseDict2.contains_key(single) {
                                    self.newTable = true;
                                }
                            }
                        }
                    }
                } else if cmd == "alles" {
                    self.allesParameters += 1;
                }
            }
        }
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

    pub fn validate_cli_like_python_for_known_case(&mut self) {
        self.cliErrors.clear();

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
        self.breiteBreitenSysArgvPara(self.argvWithoutProgram.clone());
        self.setShellRowsAmount();
        self.setShellWidth();
        self.parametersToCommandsAndNumbers(words);
        self.produceAllSpaltenNumbers();
        self.setRowRangeFromArgv();
        self.setIfZeilenSetToInf();
        self.helpPage();
        self.validate_cli_like_python_for_known_case();
        self.allImportantBeginThingsDone = true;

        (self.RowsLen, vec![], vec![], self.relitable.clone(), self.rowsAsNumbers.clone())
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
        let (_RowsLen, _paramLines, _paramLinesNot, relitable, _rowsAsNumbers) =
            self.bringAllImportantBeginThings(argv, words);

        self.tableGenerated = self.newTable;
        self.__resultingTable = relitable.clone();
        self.determineNumlen();
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
        relitable
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
            "paraMainDict={} paraDict={} dataDict0={} dataDict3={} kombi1={} kombi2={} newTable={} argvWithoutProgram={:?} beginDone={} runDone={} hoechsteZeile={} tableGenerated={} relitableRows={} RowsLen={} cliErrors={} sideParas={:?} resultingTableRows={} allesParameters={} spaltenNumbers={} ifPrint={} rowRangeLen={} shellRowsAmount={} shellWidth={} finallyDisplayLines={} ifZeilenSetToInf={} tables={} numlen={} old2Rows={} newerTable={} finallyDisplayLinesByChunks={} rowsOfcombi={} oldRows={} newerRows={} oldTable={} generatedSpaltenParameter={} allEquColumns={} finallyDisplayTable={}",
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
            self.finallyDisplayTable.len()
        )
    }
}
