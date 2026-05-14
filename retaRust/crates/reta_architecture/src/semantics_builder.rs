//! Parameter semantics builder transcompiled from
//! `python_arch_reference/reta_architecture/semantics_builder.py`.
//!
//! This is the typed Rust owner for the old `reta.py` parameter-globalisation
//! phase.  It keeps canonical main/sub parameter lookup tables and the special
//! `alles` aggregate command shape without mutating the source schema.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::number_theory::prime_factors;
use crate::schema::{ParameterMatrixEntry, RetaContextSchema};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterSemanticsBuildResult {
    pub para_main_dict: BTreeMap<String, Vec<String>>,
    pub para_dict: BTreeMap<(String, String), Vec<String>>,
    pub data_dict: Vec<BTreeMap<String, Vec<(String, String)>>>,
    pub para_n_data_matrix: Vec<ParameterMatrixEntry>,
    pub kombi_para_n_data_matrix_size: usize,
    pub kombi_para_n_data_matrix2_size: usize,
    pub kombi_reverse_dict: BTreeMap<String, i64>,
    pub kombi_reverse_dict2: BTreeMap<String, i64>,
    pub all_simple_command_columns: BTreeSet<i64>,
    pub all_values: Vec<BTreeSet<i64>>,
}

impl ParameterSemanticsBuildResult {
    pub fn snapshot(&self) -> ParameterSemanticsBuildSnapshot {
        ParameterSemanticsBuildSnapshot {
            class: "ParameterSemanticsBuildResult".to_string(),
            para_main_dict_len: self.para_main_dict.len(),
            para_dict_len: self.para_dict.len(),
            data_dict_len: self.data_dict.len(),
            para_n_data_matrix_len: self.para_n_data_matrix.len(),
            kombi_reverse_dict_len: self.kombi_reverse_dict.len(),
            kombi_reverse_dict2_len: self.kombi_reverse_dict2.len(),
            all_simple_command_columns_len: self.all_simple_command_columns.len(),
            all_values_len: self.all_values.len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterSemanticsBuildSnapshot {
    pub class: String,
    pub para_main_dict_len: usize,
    pub para_dict_len: usize,
    pub data_dict_len: usize,
    pub para_n_data_matrix_len: usize,
    pub kombi_reverse_dict_len: usize,
    pub kombi_reverse_dict2_len: usize,
    pub all_simple_command_columns_len: usize,
    pub all_values_len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterSemanticsBuilder {
    pub schema: RetaContextSchema,
    pub gebrochen_spalten_maximum_plus1: i64,
    pub invert_alles: bool,
    pub initial_data_dict: Vec<BTreeMap<String, Vec<(String, String)>>>,
    pub alles_parameter_names: Vec<String>,
}

impl ParameterSemanticsBuilder {
    pub fn new(
        schema: RetaContextSchema,
        gebrochen_spalten_maximum_plus1: i64,
        invert_alles: bool,
        initial_data_dict: Vec<BTreeMap<String, Vec<(String, String)>>>,
        alles_parameter_names: Vec<String>,
    ) -> Self {
        Self {
            schema,
            gebrochen_spalten_maximum_plus1,
            invert_alles,
            initial_data_dict,
            alles_parameter_names,
        }
    }

    pub fn allowed_prim_numbers_for_command(&self) -> Vec<String> {
        (2..32)
            .filter(|num| prime_factors(*num).len() == 1)
            .map(|num| num.to_string())
            .collect()
    }

    pub fn build_reverse_lookup(
        &self,
        mapping: &BTreeMap<i64, Vec<String>>,
    ) -> BTreeMap<String, i64> {
        let mut reverse = BTreeMap::new();
        for (key, values) in mapping {
            for value in values {
                reverse.insert(value.clone(), *key);
            }
        }
        reverse
    }

    pub fn collect_all_values(
        &self,
        para_n_data_matrix: &[ParameterMatrixEntry],
    ) -> (Vec<BTreeSet<i64>>, BTreeSet<i64>) {
        let mut all_values = vec![BTreeSet::new(); 12];
        for entry in para_n_data_matrix {
            for column in &entry.columns {
                all_values[0].insert(*column);
            }
            for alias in &entry.parameter_aliases {
                if let Ok(value) = alias.parse::<i64>() {
                    all_values[2].insert(value);
                }
            }
        }
        let all_simple_command_columns = all_values[0].clone();
        if self.invert_alles && !all_values[0].is_empty() {
            let max_value = *all_values[0].iter().max().unwrap_or(&0);
            all_values[0] = (0..max_value)
                .filter(|value| !all_simple_command_columns.contains(value))
                .collect();
        }
        all_values[2] = self
            .allowed_prim_numbers_for_command()
            .into_iter()
            .filter_map(|value| value.parse().ok())
            .collect();
        for idx in [5usize, 6, 9, 10] {
            all_values[idx] = (2..self.gebrochen_spalten_maximum_plus1).collect();
        }
        if self.invert_alles {
            for values in all_values.iter_mut().take(11).skip(1) {
                values.clear();
            }
        }
        (all_values, all_simple_command_columns)
    }

    pub fn into_parameter_datatype(
        &self,
        parameter_main_names: &[String],
        parameter_names: &[String],
        datas: &[BTreeSet<i64>],
    ) -> (
        BTreeMap<String, Vec<String>>,
        BTreeMap<(String, String), Vec<String>>,
        Vec<BTreeMap<String, Vec<(String, String)>>>,
    ) {
        let mut para_main_dict = BTreeMap::new();
        let names = if parameter_names.is_empty() {
            vec![String::new()]
        } else {
            parameter_names.to_vec()
        };
        for name in parameter_main_names {
            para_main_dict.insert(name.clone(), names.clone());
        }
        let mut para_dict = BTreeMap::new();
        for name1 in parameter_main_names {
            for name2 in &names {
                para_dict.insert(
                    (name1.clone(), name2.clone()),
                    datas.iter().map(|set| set.len().to_string()).collect(),
                );
            }
        }
        let mut data_dicts: Vec<BTreeMap<String, Vec<(String, String)>>> =
            vec![BTreeMap::new(); 14];
        for (index, data_set) in datas.iter().enumerate() {
            let target = index.min(data_dicts.len() - 1);
            for value in data_set {
                let key = value.to_string();
                let entry = data_dicts[target].entry(key).or_default();
                for main in parameter_main_names {
                    for sub in &names {
                        let pair = (main.clone(), sub.clone());
                        if !entry.contains(&pair) {
                            entry.push(pair);
                        }
                    }
                }
            }
        }
        (para_main_dict, para_dict, data_dicts)
    }

    pub fn build(&self) -> ParameterSemanticsBuildResult {
        let mut para_n_data_matrix = self.schema.para_n_data_matrix.clone();
        let (all_values, all_simple_command_columns) = self.collect_all_values(&para_n_data_matrix);
        if !self.alles_parameter_names.is_empty() {
            para_n_data_matrix.push(ParameterMatrixEntry {
                main_aliases: self.alles_parameter_names.clone(),
                parameter_aliases: Vec::new(),
                columns: Vec::new(),
            });
        }

        let mut para_main_dict = BTreeMap::new();
        let mut para_dict = BTreeMap::new();
        let mut data_dict = if self.initial_data_dict.is_empty() {
            Vec::<BTreeMap<String, Vec<(String, String)>>>::from(vec![BTreeMap::new(); 14])
        } else {
            self.initial_data_dict.clone()
        };

        for entry in &para_n_data_matrix {
            let (local_main, local_para, local_data) = self.into_parameter_datatype(
                &entry.main_aliases,
                &entry.parameter_aliases,
                &all_values,
            );
            para_main_dict.extend(local_main);
            para_dict.extend(local_para);
            for (index, local) in local_data.into_iter().enumerate() {
                if index >= data_dict.len() {
                    data_dict.push(local);
                    continue;
                }
                for (key, values) in local {
                    let entry = data_dict[index].entry(key).or_default();
                    for value in values {
                        if !entry.contains(&value) {
                            entry.push(value);
                        }
                    }
                }
            }
        }

        ParameterSemanticsBuildResult {
            para_main_dict,
            para_dict,
            data_dict,
            para_n_data_matrix,
            kombi_para_n_data_matrix_size: self.schema.kombi_para_n_data_matrix_size,
            kombi_para_n_data_matrix2_size: self.schema.kombi_para_n_data_matrix2_size,
            kombi_reverse_dict: BTreeMap::new(),
            kombi_reverse_dict2: BTreeMap::new(),
            all_simple_command_columns,
            all_values,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SemanticsBuilderBundle {
    pub builder: ParameterSemanticsBuilder,
}

impl SemanticsBuilderBundle {
    pub fn build(&self) -> ParameterSemanticsBuildResult {
        self.builder.build()
    }

    pub fn snapshot(&self) -> ParameterSemanticsBuildSnapshot {
        self.builder.build().snapshot()
    }
}

pub fn bootstrap_semantics_builder(schema: Option<RetaContextSchema>) -> SemanticsBuilderBundle {
    SemanticsBuilderBundle {
        builder: ParameterSemanticsBuilder::new(
            schema.unwrap_or_else(crate::schema::bootstrap_schema),
            1025,
            false,
            Vec::new(),
            vec!["alles".to_string()],
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_prime_numbers_are_generated() {
        let bundle = bootstrap_semantics_builder(None);
        let primes = bundle.builder.allowed_prim_numbers_for_command();
        assert!(primes.contains(&"2".to_string()));
        assert!(primes.contains(&"31".to_string()));
        assert!(!primes.contains(&"4".to_string()));
    }

    #[test]
    fn semantics_builder_uses_generated_column_projection() {
        let bundle = bootstrap_semantics_builder(None);
        let result = bundle.build();
        assert!(result.all_simple_command_columns.contains(&744));
        assert!(result.para_n_data_matrix.len() >= 400);
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "__init__",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}

// Stage 16 small-surface concrete wrappers.
pub fn __init__() -> SemanticsBuilderBundle {
    bootstrap_semantics_builder(None)
}
