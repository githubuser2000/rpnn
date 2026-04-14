#![allow(non_snake_case)]

use std::collections::{BTreeSet, HashMap};
use std::sync::OnceLock;

use indexmap::IndexMap;

use crate::shared::reta_program_types::{Generated2Selection, PairStr, Program, SpaltenTyp};
use crate::shared::words_py::{PyValue, StoreParameterEntry, Words};

#[derive(Clone, Debug, Default)]
pub struct GeneratorFamilyData {
    pub generated1_pairs: Vec<(i64, i64)>,
    pub generated2_codes: Vec<String>,
    pub generated2_selections: Vec<Generated2Selection>,
    pub bool_and_tuple_set1_options: Vec<Option<i64>>,
    pub metakonkret_pairs: Vec<(i64, i64)>,
}

impl GeneratorFamilyData {
    fn absorb_entry(&mut self, entry: &StoreParameterEntry, spalten: &SpaltenTyp) {
        for value in entry.datas.get(spalten.generated1.1).into_iter().flatten() {
            if let PyValue::Tuple(inner) = value {
                let numbers: Vec<i64> = inner
                    .iter()
                    .filter_map(|item| match item {
                        PyValue::Int(n) => Some(*n),
                        _ => None,
                    })
                    .collect();
                if numbers.len() >= 2 {
                    push_unique_pair(&mut self.generated1_pairs, (numbers[0], numbers[1]));
                }
            }
        }

        for value in entry.datas.get(spalten.generated2.1).into_iter().flatten() {
            if let PyValue::Str(code) = value {
                push_unique_string(&mut self.generated2_codes, code.clone());
                push_unique_generated2_selection(
                    &mut self.generated2_selections,
                    Generated2Selection {
                        parameter_main_name: entry.parameterMainNames.first().cloned().unwrap_or_default(),
                        parameter_name: entry.parameterNames.first().cloned().unwrap_or_default(),
                        code: code.clone(),
                    },
                );
            }
        }

        for value in entry.datas.get(spalten.boolAndTupleSet1.1).into_iter().flatten() {
            if let PyValue::Tuple(inner) = value {
                let option = inner.iter().find_map(|item| match item {
                    PyValue::Int(n) => Some(Some(*n)),
                    PyValue::NoneValue => Some(None),
                    _ => None,
                });
                if let Some(option) = option {
                    push_unique_option_i64(&mut self.bool_and_tuple_set1_options, option);
                }
            }
        }

        for value in entry.datas.get(spalten.metakonkret.1).into_iter().flatten() {
            if let PyValue::Tuple(inner) = value {
                let numbers: Vec<i64> = inner
                    .iter()
                    .filter_map(|item| match item {
                        PyValue::Int(n) => Some(*n),
                        _ => None,
                    })
                    .collect();
                if numbers.len() >= 2 {
                    push_unique_pair(&mut self.metakonkret_pairs, (numbers[0], numbers[1]));
                }
            }
        }
    }

    fn merge_from(&mut self, other: &GeneratorFamilyData) {
        for value in &other.generated1_pairs {
            push_unique_pair(&mut self.generated1_pairs, *value);
        }
        for value in &other.generated2_codes {
            push_unique_string(&mut self.generated2_codes, value.clone());
        }
        for value in &other.generated2_selections {
            push_unique_generated2_selection(&mut self.generated2_selections, value.clone());
        }
        for value in &other.bool_and_tuple_set1_options {
            push_unique_option_i64(&mut self.bool_and_tuple_set1_options, *value);
        }
        for value in &other.metakonkret_pairs {
            push_unique_pair(&mut self.metakonkret_pairs, *value);
        }
    }
}

#[derive(Clone, Debug)]
pub struct RetaStaticData {
    pub paraMainDict: IndexMap<String, Vec<String>>,
    pub paraDict: IndexMap<(String, String), Vec<Vec<PyValue>>>,
    pub dataDicts: Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
    pub dataDict: Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
    pub kombiReverseDict: IndexMap<String, i64>,
    pub kombiReverseDict2: IndexMap<String, i64>,
    pub paraDictGenerated: IndexMap<(String, String), i64>,
    pub paraDictGenerated4htmlTags: IndexMap<(String, String), i64>,
    pub spaltenTypeNaming: SpaltenTyp,
    pub AllSimpleCommandSpalten: Vec<i64>,
    pub spaltenArtenKeyTemplate: IndexMap<(usize, usize), BTreeSet<i64>>,
    pub generator_all: GeneratorFamilyData,
    pub generator_lookup: HashMap<(String, String), GeneratorFamilyData>,
}

static RETA_STATIC_DATA: OnceLock<RetaStaticData> = OnceLock::new();

pub fn shared_reta_static_data(words: &Words) -> &'static RetaStaticData {
    RETA_STATIC_DATA.get_or_init(|| build_static_data(words))
}

fn build_static_data(words: &Words) -> RetaStaticData {
    let mut program = Program::new(vec!["reta".to_string()]);
    program.init_dataDict_and_spaltenTypeNaming_python_like();
    program.init_spalten_arten_python_like();
    program.storeParamtersForColumns(words);

    let spalten = program.spaltenTypeNaming.clone();
    let mut generator_all = GeneratorFamilyData::default();
    let mut generator_lookup: HashMap<(String, String), GeneratorFamilyData> = HashMap::new();

    for entry in &words.paraNdataMatrix {
        let mut entry_data = GeneratorFamilyData::default();
        entry_data.absorb_entry(entry, &spalten);
        generator_all.merge_from(&entry_data);

        for main_name in &entry.parameterMainNames {
            let normalized_main = normalize_main_name(main_name);
            for sub_name in &entry.parameterNames {
                let key = (normalized_main.clone(), sub_name.trim().to_ascii_lowercase());
                generator_lookup
                    .entry(key)
                    .or_default()
                    .merge_from(&entry_data);
            }
        }
    }

    RetaStaticData {
        paraMainDict: program.paraMainDict.clone(),
        paraDict: program.paraDict.clone(),
        dataDicts: program.dataDicts.clone(),
        dataDict: program.dataDict.clone(),
        kombiReverseDict: program.kombiReverseDict.clone(),
        kombiReverseDict2: program.kombiReverseDict2.clone(),
        paraDictGenerated: program.paraDictGenerated.clone(),
        paraDictGenerated4htmlTags: program.paraDictGenerated4htmlTags.clone(),
        spaltenTypeNaming: spalten,
        AllSimpleCommandSpalten: program.AllSimpleCommandSpalten.clone(),
        spaltenArtenKeyTemplate: program.spaltenArtenKey_SpaltennummernValue.clone(),
        generator_all,
        generator_lookup,
    }
}

fn normalize_main_name(value: &str) -> String {
    let normalized = value.trim().to_ascii_lowercase();
    if Program::parameter_main_name_matches_local_py(&normalized, "primvielfache") {
        "primvielfache".to_string()
    } else {
        normalized
    }
}

fn push_unique_pair(target: &mut Vec<(i64, i64)>, value: (i64, i64)) {
    if !target.contains(&value) {
        target.push(value);
    }
}

fn push_unique_option_i64(target: &mut Vec<Option<i64>>, value: Option<i64>) {
    if !target.contains(&value) {
        target.push(value);
    }
}

fn push_unique_string(target: &mut Vec<String>, value: String) {
    if !target.iter().any(|existing| existing == &value) {
        target.push(value);
    }
}

fn push_unique_generated2_selection(target: &mut Vec<Generated2Selection>, value: Generated2Selection) {
    if !target.iter().any(|existing| existing == &value) {
        target.push(value);
    }
}
