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
        // Stage 18: build sparse reverse dictionaries from the matrix entries
        // themselves.  Earlier stages materialised every known column against
        // every parameter pair; that mirrored a broad Python surface but made
        // the architecture runtime unnecessarily expensive.  The sparse form is
        // closer to the mathematical sheaf reading: each local alias pair maps
        // only to its own section columns.
        let mut para_n_data_matrix = self.schema.para_n_data_matrix.clone();
        let (all_values, all_simple_command_columns) = self.collect_all_values(&para_n_data_matrix);
        if !self.alles_parameter_names.is_empty() {
            para_n_data_matrix.push(ParameterMatrixEntry {
                main_aliases: self.alles_parameter_names.clone(),
                parameter_aliases: Vec::new(),
                columns: Vec::new(),
            });
        }

        let mut para_main_dict = BTreeMap::<String, Vec<String>>::new();
        let mut para_dict = BTreeMap::<(String, String), Vec<String>>::new();
        let mut data_dict = if self.initial_data_dict.is_empty() {
            Vec::<BTreeMap<String, Vec<(String, String)>>>::from(vec![BTreeMap::new(); 14])
        } else {
            self.initial_data_dict.clone()
        };
        if data_dict.len() < 14 {
            data_dict.resize_with(14, BTreeMap::new);
        }

        for entry in &para_n_data_matrix {
            let parameter_aliases = if entry.parameter_aliases.is_empty() {
                vec![String::new()]
            } else {
                entry.parameter_aliases.clone()
            };
            let column_strings = entry
                .columns
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            for main in &entry.main_aliases {
                let main_values = para_main_dict.entry(main.clone()).or_default();
                for parameter in &parameter_aliases {
                    if !main_values.contains(parameter) {
                        main_values.push(parameter.clone());
                    }
                    let pair = (main.clone(), parameter.clone());
                    para_dict.insert(pair.clone(), column_strings.clone());
                    for column in &entry.columns {
                        let bucket = data_dict[0].entry(column.to_string()).or_default();
                        if !bucket.contains(&pair) {
                            bucket.push(pair.clone());
                        }
                    }
                    if let Ok(numeric_parameter) = parameter.parse::<i64>() {
                        let bucket = data_dict[2]
                            .entry(numeric_parameter.to_string())
                            .or_default();
                        if !bucket.contains(&pair) {
                            bucket.push(pair.clone());
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
        // Stage 18: keep architecture snapshots cheap.  The full Python-style
        // build constructs large reverse dictionaries; snapshots only need the
        // same observable counts, so compute them directly from the schema.
        let mut main_aliases = BTreeSet::new();
        let mut pair_count = 0usize;
        let mut all_simple_command_columns = BTreeSet::new();
        for entry in &self.builder.schema.para_n_data_matrix {
            for main in &entry.main_aliases {
                main_aliases.insert(main.clone());
            }
            pair_count += entry.main_aliases.len() * entry.parameter_aliases.len().max(1);
            all_simple_command_columns.extend(entry.columns.iter().copied());
        }
        let extra_alles = usize::from(!self.builder.alles_parameter_names.is_empty());
        ParameterSemanticsBuildSnapshot {
            class: "ParameterSemanticsBuildResult".to_string(),
            para_main_dict_len: main_aliases.len() + extra_alles,
            para_dict_len: pair_count,
            data_dict_len: 14,
            para_n_data_matrix_len: self.builder.schema.para_n_data_matrix.len() + extra_alles,
            kombi_reverse_dict_len: 0,
            kombi_reverse_dict2_len: 0,
            all_simple_command_columns_len: all_simple_command_columns.len(),
            all_values_len: 12,
        }
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
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &["__init__"];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}

// Stage 16 small-surface concrete wrappers.
pub fn __init__() -> SemanticsBuilderBundle {
    bootstrap_semantics_builder(None)
}
