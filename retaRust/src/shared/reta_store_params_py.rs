#![allow(non_snake_case)]
use indexmap::IndexMap;
use std::collections::BTreeSet;

use crate::shared::reta_program_types::{PairStr, Program};
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
    fn push_unique_pyvalue_py(target: &mut Vec<PyValue>, value: PyValue) {
        if !target.iter().any(|existing| existing == &value) {
            target.push(value);
        }
    }

    fn pyvalue_as_decimal_i64_py(value: &PyValue) -> Option<i64> {
        match value {
            PyValue::Int(n) => Some(*n),
            PyValue::Str(text) if !text.is_empty() && text.chars().all(|c| c.is_ascii_digit()) => text.parse::<i64>().ok(),
            _ => None,
        }
    }

    fn py_string_repr_inside_tuple_py(value: &str) -> String {
        let mut escaped = String::with_capacity(value.len() + 2);
        escaped.push('\'');
        for ch in value.chars() {
            match ch {
                '\\' => escaped.push_str("\\\\"),
                '\'' => escaped.push_str("\\'"),
                _ => escaped.push(ch),
            }
        }
        escaped.push('\'');
        escaped
    }

    fn pyvalue_repr_inside_tuple_py(value: &PyValue) -> String {
        match value {
            PyValue::Int(n) => n.to_string(),
            PyValue::Str(text) => Self::py_string_repr_inside_tuple_py(text),
            PyValue::Bool(true) => "True".to_string(),
            PyValue::Bool(false) => "False".to_string(),
            PyValue::NoneValue => "None".to_string(),
            PyValue::Tuple(inner) => Self::python_tuple_repr_from_pyvalues_py(inner),
        }
    }

    fn python_tuple_repr_from_pyvalues_py(values: &[PyValue]) -> String {
        let parts = values
            .iter()
            .map(Self::pyvalue_repr_inside_tuple_py)
            .collect::<Vec<_>>();
        if parts.len() == 1 {
            format!("({},)", parts[0])
        } else {
            format!("({})", parts.join(", "))
        }
    }

    fn pyvalue_data_dict_key_py(value: &PyValue) -> String {
        match value {
            PyValue::Int(n) => n.to_string(),
            PyValue::Str(text) => text.clone(),
            PyValue::Bool(true) => "True".to_string(),
            PyValue::Bool(false) => "False".to_string(),
            PyValue::NoneValue => "None".to_string(),
            PyValue::Tuple(inner) => Self::python_tuple_repr_from_pyvalues_py(inner),
        }
    }

    fn parameter_name_to_case2_key_py(parameter_name: &str, has_parameter_names: bool) -> String {
        if !has_parameter_names {
            "None".to_string()
        } else if !parameter_name.is_empty() && parameter_name.chars().all(|c| c.is_ascii_digit()) {
            parameter_name.parse::<i64>().unwrap_or(0).to_string()
        } else {
            parameter_name.to_string()
        }
    }

    fn is_case1_bool_payload_py(value: &PyValue) -> bool {
        match value {
            PyValue::Bool(_) => true,
            PyValue::Tuple(inner) => inner.first().map(|first| matches!(first, PyValue::Bool(_))).unwrap_or(false),
            _ => false,
        }
    }

    fn all_simple_command_spalten_python_like(words: &Words) -> Vec<i64> {
        let mut out: Vec<i64> = Vec::new();
        for possibleCommands in words.paraNdataMatrix.iter() {
            if let Some(command_values) = possibleCommands.datas.get(0) {
                for value in command_values {
                    if let PyValue::Int(n) = value {
                        if !out.contains(n) {
                            out.push(*n);
                        }
                    }
                }
            }
        }
        out
    }

    pub(crate) fn build_alles_entry_python_like(&self, words: &Words) -> StoreParameterEntry {
        let mut allValues: Vec<Vec<PyValue>> = (0..12).map(|_| Vec::new()).collect();
        let mut gebrochenSpaltenMaximumPlus1 = 2i64;

        for possibleCommands in words.paraNdataMatrix.iter() {
            for (i, commandValue) in possibleCommands.datas.iter().enumerate() {
                if i >= allValues.len() {
                    continue;
                }
                for spaltenNummerOderEtc in commandValue {
                    Self::push_unique_pyvalue_py(&mut allValues[i], spaltenNummerOderEtc.clone());
                    if matches!(i, 5 | 6 | 9 | 10) {
                        if let Some(n) = Self::pyvalue_as_decimal_i64_py(spaltenNummerOderEtc) {
                            if n + 1 > gebrochenSpaltenMaximumPlus1 {
                                gebrochenSpaltenMaximumPlus1 = n + 1;
                            }
                        }
                    }
                }
            }
        }

        let allowedPrimNumbersForCommand: Vec<PyValue> = (2..32)
            .filter(|num| Self::primCreativity_py(*num) == 1)
            .map(PyValue::Int)
            .collect();

        allValues[2] = allowedPrimNumbersForCommand;
        allValues[3] = words
            .kombiParaNdataMatrix
            .keys()
            .copied()
            .map(PyValue::Int)
            .collect();
        allValues[5] = (2..gebrochenSpaltenMaximumPlus1).map(PyValue::Int).collect();
        allValues[6] = (2..gebrochenSpaltenMaximumPlus1).map(PyValue::Int).collect();
        allValues[8] = words
            .kombiParaNdataMatrix2
            .keys()
            .copied()
            .map(PyValue::Int)
            .collect();
        allValues[9] = (2..gebrochenSpaltenMaximumPlus1).map(PyValue::Int).collect();
        allValues[10] = (2..gebrochenSpaltenMaximumPlus1).map(PyValue::Int).collect();

        if self.__invertAlles {
            let max0 = allValues[0]
                .iter()
                .filter_map(Self::pyvalue_as_decimal_i64_py)
                .max()
                .unwrap_or(0);
            let mut aus_generated1: BTreeSet<i64> = BTreeSet::new();
            for value in &allValues[1] {
                if let PyValue::Tuple(inner) = value {
                    if let Some(first) = inner.get(0).and_then(Self::pyvalue_as_decimal_i64_py) {
                        aus_generated1.insert(first);
                    }
                    if let Some(second) = inner.get(1).and_then(Self::pyvalue_as_decimal_i64_py) {
                        aus_generated1.insert(second);
                    }
                }
            }
            let already: BTreeSet<i64> = allValues[0]
                .iter()
                .filter_map(Self::pyvalue_as_decimal_i64_py)
                .collect();
            let mut inverted = Vec::new();
            for n in 0..max0 {
                if !already.contains(&n) && !aus_generated1.contains(&n) {
                    inverted.push(PyValue::Int(n));
                }
            }
            allValues[0] = inverted;
            for zahl in 1..11usize {
                allValues[zahl].clear();
            }
        }

        StoreParameterEntry {
            parameterMainNames: vec!["alles".to_string()],
            parameterNames: vec![],
            datas: allValues,
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
        self.AllSimpleCommandSpalten = Self::all_simple_command_spalten_python_like(words);
        let alles_entry = self.build_alles_entry_python_like(words);
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
            if parameterNames.is_empty() {
                paraDict.insert((name1.clone(), "".to_string()), datas.clone());
            }
        }

        let mut dataDicts: Vec<IndexMap<String, Vec<Vec<PairStr>>>> = (0..12).map(|_| IndexMap::new()).collect();
        let has_parameter_names = !parameterNames.is_empty();

        for (i, d) in datas.iter().enumerate() {
            if i >= dataDicts.len() {
                continue;
            }
            for spaltenNummerOderEtc in d {
                let mut into_flat: Vec<PairStr> = vec![];
                let mut into_case2: Vec<Vec<PairStr>> = vec![];
                let mut parameterMainNamePerLoop: Vec<String> = vec![];
                let mut case_: i64 = -1;

                for parameterMainName in parameterMainNames {
                    let parameterNames_local = if has_parameter_names {
                        parameterNames.clone()
                    } else {
                        vec!["".to_string()]
                    };
                    for parameterName in parameterNames_local {
                        if i == 4 && Self::is_case1_bool_payload_py(spaltenNummerOderEtc) {
                            case_ = 1;
                            into_flat.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                        } else if matches!(i, 5 | 6 | 9 | 10) {
                            case_ = 2;
                            let pair = PairStr(parameterMainName.clone(), parameterName.clone());
                            into_case2.push(vec![pair]);
                            parameterMainNamePerLoop.push(parameterName.clone());
                        } else if i == 2 && matches!(spaltenNummerOderEtc, PyValue::Str(_)) {
                            case_ = 2;
                            let pair = PairStr(parameterMainName.clone(), parameterName.clone());
                            into_case2.push(vec![pair]);
                            parameterMainNamePerLoop.push(parameterName.clone());
                        } else if i == 4 && matches!(spaltenNummerOderEtc, PyValue::Tuple(_)) {
                            case_ = 4;
                            into_flat.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                        } else {
                            case_ = 3;
                            into_flat.push(PairStr(parameterMainName.clone(), parameterName.clone()));
                        }
                    }
                }

                let index1 = if case_ != 1 { i } else { 3 };
                if index1 >= dataDicts.len() {
                    continue;
                }

                let index2a: Vec<String> = match case_ {
                    1 => vec!["('bool', 0)".to_string()],
                    2 => parameterMainNamePerLoop
                        .iter()
                        .map(|para| Self::parameter_name_to_case2_key_py(para, has_parameter_names))
                        .collect(),
                    3 | 4 => vec![Self::pyvalue_data_dict_key_py(spaltenNummerOderEtc)],
                    _ => vec!["None".to_string()],
                };

                let intoA: Vec<Vec<PairStr>> = if case_ == 2 {
                    into_case2.clone()
                } else {
                    vec![into_flat.clone()]
                };

                let fillvalue = if case_ == 2 {
                    into_case2.iter().flatten().cloned().collect::<Vec<PairStr>>()
                } else {
                    into_flat.clone()
                };
                let max_len = std::cmp::max(index2a.len(), intoA.len());
                for pos in 0..max_len {
                    let index2 = if pos < index2a.len() {
                        index2a[pos].clone()
                    } else {
                        // Python zip_longest(..., fillvalue=into) can only land here for malformed
                        // generated data; keep a deterministic Python-like key instead of Rust Debug.
                        "None".to_string()
                    };
                    let into2 = if pos < intoA.len() {
                        intoA[pos].clone()
                    } else {
                        fillvalue.clone()
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
