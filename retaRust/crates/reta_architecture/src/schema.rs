//! Reta context schema transcompiled from
//! `python_arch_reference/reta_architecture/schema.py`.
//!
//! Python extracts this from the split i18n modules.  Rust stores the same
//! schema shape as data so parameter, prompt and table morphisms can share a
//! typed vocabulary instead of reaching into the old monolith.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AliasGroup {
    pub canonical: String,
    pub aliases: Vec<String>,
}

impl AliasGroup {
    pub fn new(
        canonical: impl Into<String>,
        aliases: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            canonical: canonical.into(),
            aliases: aliases.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterMatrixEntry {
    pub main_aliases: Vec<String>,
    pub parameter_aliases: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetaContextSchema {
    pub language_aliases: BTreeMap<String, String>,
    pub translation_domains: BTreeMap<String, String>,
    pub parameters_main: Vec<Vec<String>>,
    pub row_parameters: BTreeMap<String, String>,
    pub output_parameters: BTreeMap<String, String>,
    pub output_modes: BTreeMap<String, String>,
    pub combination_parameters: BTreeMap<String, String>,
    pub scopes: BTreeMap<String, String>,
    pub para_n_data_matrix: Vec<ParameterMatrixEntry>,
    pub kombi_para_n_data_matrix_size: usize,
    pub kombi_para_n_data_matrix2_size: usize,
    pub tag_names: Vec<String>,
    pub schema_modules: BTreeMap<String, String>,
}

impl Default for RetaContextSchema {
    fn default() -> Self {
        Self {
            language_aliases: BTreeMap::new(),
            translation_domains: BTreeMap::new(),
            parameters_main: Vec::new(),
            row_parameters: BTreeMap::new(),
            output_parameters: BTreeMap::new(),
            output_modes: BTreeMap::new(),
            combination_parameters: BTreeMap::new(),
            scopes: BTreeMap::new(),
            para_n_data_matrix: Vec::new(),
            kombi_para_n_data_matrix_size: 0,
            kombi_para_n_data_matrix2_size: 0,
            tag_names: Vec::new(),
            schema_modules: BTreeMap::new(),
        }
    }
}

impl RetaContextSchema {
    pub fn from_parts(
        language_aliases: BTreeMap<String, String>,
        translation_domains: BTreeMap<String, String>,
        parameters_main: Vec<Vec<String>>,
        para_n_data_matrix: Vec<ParameterMatrixEntry>,
        schema_modules: BTreeMap<String, String>,
    ) -> Self {
        Self {
            language_aliases,
            translation_domains,
            parameters_main,
            para_n_data_matrix,
            schema_modules,
            ..Self::default()
        }
    }

    pub fn main_alias_groups(&self) -> Vec<AliasGroup> {
        self.parameters_main
            .iter()
            .filter_map(|group| {
                let aliases = group
                    .iter()
                    .filter(|v| !v.is_empty())
                    .cloned()
                    .collect::<Vec<_>>();
                aliases
                    .first()
                    .cloned()
                    .map(|canonical| AliasGroup { canonical, aliases })
            })
            .collect()
    }

    pub fn main_alias_map(&self) -> BTreeMap<String, String> {
        let mut alias_map = BTreeMap::new();
        for group in self.main_alias_groups() {
            alias_map.insert(group.canonical.clone(), group.canonical.clone());
            for alias in group.aliases {
                alias_map.insert(alias, group.canonical.clone());
            }
        }
        alias_map
    }

    pub fn sub_parameter_alias_groups(&self) -> BTreeMap<String, Vec<AliasGroup>> {
        let main_alias_map = self.main_alias_map();
        let mut groups: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
        for entry in &self.para_n_data_matrix {
            let main_aliases = entry
                .main_aliases
                .iter()
                .filter(|v| !v.is_empty())
                .cloned()
                .collect::<Vec<_>>();
            let parameter_aliases = entry
                .parameter_aliases
                .iter()
                .filter(|v| !v.is_empty())
                .cloned()
                .collect::<Vec<_>>();
            if main_aliases.is_empty() || parameter_aliases.is_empty() {
                continue;
            }
            let main_canonical = main_alias_map
                .get(&main_aliases[0])
                .cloned()
                .unwrap_or_else(|| main_aliases[0].clone());
            let parameter_canonical = parameter_aliases[0].clone();
            let aliases = groups
                .entry(main_canonical)
                .or_default()
                .entry(parameter_canonical)
                .or_default();
            for alias in parameter_aliases {
                if !aliases.contains(&alias) {
                    aliases.push(alias);
                }
            }
        }
        groups
            .into_iter()
            .map(|(main, parameters)| {
                let groups = parameters
                    .into_iter()
                    .map(|(canonical, mut aliases)| {
                        aliases.sort();
                        AliasGroup { canonical, aliases }
                    })
                    .collect();
                (main, groups)
            })
            .collect()
    }

    pub fn snapshot(&self) -> RetaContextSchemaSnapshot {
        RetaContextSchemaSnapshot {
            languages: self.language_aliases.clone(),
            translation_domains: self.translation_domains.clone(),
            main_alias_groups: self.main_alias_groups(),
            row_parameters: self.row_parameters.clone(),
            output_parameters: self.output_parameters.clone(),
            output_modes: self.output_modes.clone(),
            combination_parameters: self.combination_parameters.clone(),
            scopes: self.scopes.clone(),
            tag_names: self.tag_names.clone(),
            para_n_data_matrix_size: self.para_n_data_matrix.len(),
            kombi_para_n_data_matrix_size: self.kombi_para_n_data_matrix_size,
            kombi_para_n_data_matrix2_size: self.kombi_para_n_data_matrix2_size,
            schema_modules: self.schema_modules.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetaContextSchemaSnapshot {
    pub languages: BTreeMap<String, String>,
    pub translation_domains: BTreeMap<String, String>,
    pub main_alias_groups: Vec<AliasGroup>,
    pub row_parameters: BTreeMap<String, String>,
    pub output_parameters: BTreeMap<String, String>,
    pub output_modes: BTreeMap<String, String>,
    pub combination_parameters: BTreeMap<String, String>,
    pub scopes: BTreeMap<String, String>,
    pub tag_names: Vec<String>,
    pub para_n_data_matrix_size: usize,
    pub kombi_para_n_data_matrix_size: usize,
    pub kombi_para_n_data_matrix2_size: usize,
    pub schema_modules: BTreeMap<String, String>,
}

pub fn bootstrap_schema() -> RetaContextSchema {
    let mut schema = RetaContextSchema::default();
    schema
        .schema_modules
        .insert("context".to_string(), "i18n.words_context".to_string());
    schema
        .schema_modules
        .insert("matrix".to_string(), "i18n.words_matrix".to_string());
    schema
        .schema_modules
        .insert("runtime".to_string(), "i18n.words_runtime".to_string());
    schema.parameters_main = vec![
        vec!["zeilen".to_string(), "z".to_string()],
        vec!["spalten".to_string(), "s".to_string()],
        vec!["kombination".to_string(), "k".to_string()],
        vec!["ausgabe".to_string(), "a".to_string()],
        vec!["debug".to_string(), "d".to_string()],
    ];
    schema
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alias_map_uses_first_alias_as_canonical() {
        let schema = RetaContextSchema {
            parameters_main: vec![vec!["spalten".into(), "s".into()]],
            ..Default::default()
        };
        assert_eq!(
            schema.main_alias_map().get("s"),
            Some(&"spalten".to_string())
        );
    }
}
