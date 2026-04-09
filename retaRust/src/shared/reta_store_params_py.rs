use indexmap::IndexMap;
use std::collections::BTreeSet;

fn push_unique_i64_preserve_order(into: &mut Vec<i64>, value: i64) {
    if !into.iter().any(|existing| *existing == value) {
        into.push(value);
    }
}

use crate::shared::reta_program_types::{dedup_preserve_order_i64, PairStr, Program, SpaltenTyp};
use crate::shared::words_py::{PyValue, StoreParameterEntry, Words};

impl Program {
    pub(crate) fn primCreativity_py(n: i64) -> i64 {
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

    pub(crate) fn build_alles_entry_python_like(&self, words: &Words) -> StoreParameterEntry {
        let mut allValues: Vec<Vec<i64>> = (0..12).map(|_| Vec::new()).collect();
        let mut gebrochenSpaltenMaximumPlus1 = 2i64;

        for possibleCommands in words.paraNdataMatrix.iter() {
            for (i, commandValue) in possibleCommands.datas.iter().enumerate() {
                for spaltenNummerOderEtc in commandValue {
                    match spaltenNummerOderEtc {
                        PyValue::Int(n) => {
                            push_unique_i64_preserve_order(&mut allValues[i], *n);
                            if [5usize, 6usize, 9usize, 10usize].contains(&i) && *n + 1 > gebrochenSpaltenMaximumPlus1 {
                                gebrochenSpaltenMaximumPlus1 = *n + 1;
                            }
                        }
                        PyValue::Tuple(inner) => {
                            for vv in inner {
                                if let PyValue::Int(n) = vv {
                                    push_unique_i64_preserve_order(&mut allValues[i], *n);
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

        allValues[2] = allowedPrimNumbersForCommand;
        allValues[3] = words.kombiParaNdataMatrix.keys().cloned().collect();
        allValues[5] = (2..gebrochenSpaltenMaximumPlus1).collect();
        allValues[6] = (2..gebrochenSpaltenMaximumPlus1).collect();
        allValues[8] = words.kombiParaNdataMatrix2.keys().cloned().collect();
        allValues[9] = (2..gebrochenSpaltenMaximumPlus1).collect();
        allValues[10] = (2..gebrochenSpaltenMaximumPlus1).collect();

        if self.__invertAlles {
            let max0 = *allValues[0].iter().max().unwrap_or(&0);
            let generated_pairs: BTreeSet<i64> = allValues[1].iter().copied().collect();
            let inverted = (0..max0)
                .filter(|n| !allValues[0].contains(n) && !generated_pairs.contains(n))
                .collect::<Vec<i64>>();
            allValues[0] = inverted;
            for zahl in 1..11usize {
                allValues[zahl].clear();
            }
        }

        let datas = allValues
            .into_iter()
            .map(|values| values.into_iter().map(PyValue::Int).collect::<Vec<PyValue>>())
            .collect::<Vec<Vec<PyValue>>>();
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
