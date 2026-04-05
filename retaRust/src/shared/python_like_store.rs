use indexmap::IndexMap;
use crate::shared::exact_i18n::{I18nExact, PyAtom};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PairStr(pub String, pub String);

#[derive(Clone, Debug)]
pub struct ProgramState {
    pub argv: Vec<String>,
    pub paraMainDict: IndexMap<String, Vec<String>>,
    pub paraDict: IndexMap<(String, String), Vec<Vec<PyAtom>>>,
    pub dataDicts: Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
    pub kombiReverseDict: IndexMap<String, i64>,
    pub kombiReverseDict2: IndexMap<String, i64>,
    pub newTable: bool,
}

impl ProgramState {
    pub fn new(argv: Vec<String>) -> Self {
        let mut dataDicts = vec![];
        for _ in 0..12 {
            dataDicts.push(IndexMap::new());
        }
        Self {
            argv,
            paraMainDict: IndexMap::new(),
            paraDict: IndexMap::new(),
            dataDicts,
            kombiReverseDict: IndexMap::new(),
            kombiReverseDict2: IndexMap::new(),
            newTable: false,
        }
    }
}

pub fn intoParameterDatatype(
    parameterMainNames: &Vec<String>,
    parameterNames: &Vec<String>,
    datas: &Vec<Vec<PyAtom>>,
) -> (
    IndexMap<String, Vec<String>>,
    IndexMap<(String, String), Vec<Vec<PyAtom>>>,
    Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
) {
    let mut paraMainDict: IndexMap<String, Vec<String>> = IndexMap::new();
    for name in parameterMainNames {
        paraMainDict.insert(name.clone(), parameterNames.clone());
    }

    let mut paraDict: IndexMap<(String, String), Vec<Vec<PyAtom>>> = IndexMap::new();
    for name1 in parameterMainNames {
        for name2 in parameterNames {
            paraDict.insert((name1.clone(), name2.clone()), datas.clone());
        }
        if parameterNames.len() == 0 {
            paraDict.insert((name1.clone(), "".to_string()), datas.clone());
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
            let case_num: i64;

            for parameterMainName in parameterMainNames {
                let iter_parameter_names: Vec<String> =
                    if parameterNames.len() > 0 { parameterNames.clone() } else { vec!["".to_string()] };

                for parameterName in iter_parameter_names {
                    if i == 4 && matches!(spaltenNummerOderEtc, PyAtom::Bool(_)) {
                        into.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                    } else if matches!(i, 5 | 6 | 9 | 10) {
                        into.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                        parameterMainNamePerLoop.push(parameterName.clone());
                    } else {
                        into.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                    }
                }
            }

            if i == 4 && matches!(spaltenNummerOderEtc, PyAtom::Bool(_)) {
                case_num = 1;
            } else if matches!(i, 5 | 6 | 9 | 10) {
                case_num = 2;
            } else if i == 4 {
                case_num = 4;
            } else {
                case_num = 3;
            }

            let index1: usize = if case_num != 1 { i } else { 3 };

            let index2a: Vec<String>;
            let intoA: Vec<Vec<PairStr>>;

            if case_num == 1 {
                index2a = vec!["('bool', 0)".to_string()];
                intoA = vec![into.clone()];
            } else if case_num == 2 {
                if let PyAtom::Tuple(inner) = spaltenNummerOderEtc {
                    index2a = inner.iter().map(|x| format!("{:?}", x)).collect();
                } else {
                    index2a = vec![format!("{:?}", parameterMainNamePerLoop)];
                }
                intoA = into.iter().map(|x| vec![x.clone()]).collect();
            } else if case_num == 3 || case_num == 4 {
                index2a = vec![format!("{:?}", spaltenNummerOderEtc)];
                intoA = vec![into.clone()];
            } else {
                index2a = vec!["None".to_string()];
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
    paraMainDict1: IndexMap<String, Vec<String>>,
    paraDict1: IndexMap<(String, String), Vec<Vec<PyAtom>>>,
    dataDicts1: Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
    paraMainDict2: IndexMap<String, Vec<String>>,
    paraDict2: IndexMap<(String, String), Vec<Vec<PyAtom>>>,
    dataDicts2: Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
) -> (
    IndexMap<(String, String), Vec<Vec<PyAtom>>>,
    Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
) {
    let _paraMainDict1: IndexMap<String, Vec<String>> =
        paraMainDict1.into_iter().chain(paraMainDict2.into_iter()).collect();
    let paraDict1: IndexMap<(String, String), Vec<Vec<PyAtom>>> =
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
            (Some(d1), None) => {
                dataDicts3[i] = d1.clone();
            }
            (None, Some(d2)) => {
                if i >= dataDicts3.len() {
                    dataDicts3.push(d2.clone());
                } else {
                    dataDicts3[i] = d2.clone();
                }
            }
            (None, None) => {}
        }
    }

    (paraDict1, dataDicts3)
}

pub fn storeParamtersForColumns(i18n: &I18nExact, state: &mut ProgramState) {
    state.kombiReverseDict = IndexMap::new();
    for (key, value) in i18n.kombiParaNdataMatrix.iter() {
        for valuesInValuess in value {
            state.kombiReverseDict.insert(valuesInValuess.clone(), *key);
        }
    }

    state.kombiReverseDict2 = IndexMap::new();
    for (key, value) in i18n.kombiParaNdataMatrix2.iter() {
        for valuesInValuess in value {
            state.kombiReverseDict2.insert(valuesInValuess.clone(), *key);
        }
    }

    state.paraMainDict = IndexMap::new();
    state.paraDict = IndexMap::new();
    let mut dataDicts_local = {
        let mut x = vec![];
        for _ in 0..12 { x.push(IndexMap::new()); }
        x
    };

    for parameterEntry in i18n.paraNdataMatrix.iter() {
        let (paraMainDict2, paraDict2, dataDicts2) = intoParameterDatatype(
            &parameterEntry.parameterMainNames,
            &parameterEntry.parameterNames,
            &parameterEntry.datas,
        );
        let (paraDict3, dataDicts3) = mergeParameterDicts(
            state.paraMainDict.clone(),
            state.paraDict.clone(),
            dataDicts_local.clone(),
            paraMainDict2.clone(),
            paraDict2.clone(),
            dataDicts2.clone(),
        );

        for (k, v) in paraMainDict2 {
            state.paraMainDict.insert(k, v);
        }
        state.paraDict = paraDict3;
        dataDicts_local = dataDicts3;
    }

    state.dataDicts = dataDicts_local;
}

pub fn parametersToCommandsAndNumbers(state: &mut ProgramState, i18n: &I18nExact) {
    storeParamtersForColumns(i18n, state);

    let mut last_parameter_type: String = "".to_string();
    for token in state.argv[1..].iter() {
        if token.starts_with("-") && !token.starts_with("--") {
            last_parameter_type = token[1..].to_string();
            continue;
        }

        if token.starts_with("--") {
            let cmd = token[2..].to_string();

            if last_parameter_type == "spalten" {
                if let Some(eq) = cmd.find('=') {
                    let main = cmd[..eq].to_string();
                    let right = cmd[eq + 1..].to_string();

                    for single in right.split(',') {
                        if state.paraDict.contains_key(&(main.clone(), single.to_string())) {
                            state.newTable = true;
                        }
                    }
                } else if state.paraDict.contains_key(&(cmd.clone(), "".to_string()))
                    || state.paraMainDict.contains_key(&cmd)
                {
                    state.newTable = true;
                }
            } else if last_parameter_type == "kombi" {
                if let Some(eq) = cmd.find('=') {
                    let left = cmd[..eq].to_string();
                    let right = cmd[eq + 1..].to_string();

                    if left == "galaxie" {
                        for single in right.split(',') {
                            if state.kombiReverseDict.contains_key(single) {
                                state.newTable = true;
                            }
                        }
                    } else if left == "universum" {
                        for single in right.split(',') {
                            if state.kombiReverseDict2.contains_key(single) {
                                state.newTable = true;
                            }
                        }
                    }
                }
            }
        }
    }
}
