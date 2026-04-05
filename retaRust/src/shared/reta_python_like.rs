use indexmap::IndexMap;
use crate::shared::words_python_like::{Words, PyValue};

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
                    let iter_parameter_names: Vec<String> =
                        if parameterNames.len() > 0 { parameterNames.clone() } else { vec!["".to_string()] };
                    for parameterName in iter_parameter_names {
                        into.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                        if matches!(i, 5 | 6 | 9 | 10) {
                            parameterMainNamePerLoop.push(parameterName.clone());
                        }
                    }
                }

                let case_num: i64 =
                    if i == 4 && matches!(spaltenNummerOderEtc, PyValue::Bool(_)) {
                        1
                    } else if matches!(i, 5 | 6 | 9 | 10) {
                        2
                    } else if i == 4 {
                        4
                    } else {
                        3
                    };

                let index1: usize = if case_num != 1 { i } else { 3 };
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
        let paraMainDict1: IndexMap<String, Vec<String>> =
            paraMainDict1.into_iter().chain(paraMainDict2.into_iter()).collect();
        let paraDict1: IndexMap<(String, String), Vec<Vec<PyValue>>> =
            paraDict1.into_iter().chain(paraDict2.into_iter()).collect();
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

    pub fn parametersToCommandsAndNumbers(&mut self, words: &Words) {
        self.storeParamtersForColumns(words);
        let mut lastParameterType: String = "".to_string();

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
                }
            }
        }
    }

    pub fn bringAllImportantBeginThings(&mut self, words: &Words) {
        if self.allImportantBeginThingsDone {
            return;
        }
        self.argvWithoutProgram = if self.argv.len() > 1 { self.argv[1..].to_vec() } else { vec![] };
        self.parametersToCommandsAndNumbers(words);
        self.allImportantBeginThingsDone = true;
    }
}
