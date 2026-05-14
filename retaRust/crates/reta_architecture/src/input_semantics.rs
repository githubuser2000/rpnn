//! Input semantics transcompiled from
//! `python_arch_reference/reta_architecture/input_semantics.py`.
//!
//! This bridges schema aliases, row-range syntax and prompt vocabulary in one
//! typed bundle.  Python builds the concrete lists from runtime i18n objects;
//! Rust stores the same vocabulary surface and can be filled from the split
//! schema or by later generated i18n loaders.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::row_ranges::{bootstrap_row_range_morphisms, RowRangeMorphismBundle, RowRangeSyntax};
use crate::schema::RetaContextSchema;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptVocabulary {
    pub main_parameters: Vec<String>,
    pub spalten: Vec<String>,
    pub eigs_n: Vec<String>,
    pub eigs_r: Vec<String>,
    pub spalten_dict: BTreeMap<String, Vec<String>>,
    pub ausgabe_paras: Vec<String>,
    pub kombi_main_paras: Vec<String>,
    pub zeilen_paras: Vec<String>,
    pub haupt_for_neben: Vec<String>,
    pub not_parameter_values: Vec<String>,
    pub haupt_for_neben_set: BTreeSet<String>,
    pub ausgabe_art: Vec<String>,
    pub zeilen_typen: Vec<String>,
    pub zeilen_zeit: Vec<String>,
    pub zeilen_typen_b: Vec<String>,
    pub gebrochen_erlaubte_zahlen: BTreeSet<i64>,
    pub befehle: Vec<String>,
    pub befehle2: BTreeSet<String>,
}

impl PromptVocabulary {
    pub fn snapshot(&self) -> PromptVocabularySnapshot {
        PromptVocabularySnapshot {
            main_parameters_len: self.main_parameters.len(),
            spalten_len: self.spalten.len(),
            spalten_dict_keys: self.spalten_dict.len(),
            ausgabe_paras_len: self.ausgabe_paras.len(),
            kombi_main_paras_len: self.kombi_main_paras.len(),
            zeilen_paras_len: self.zeilen_paras.len(),
            haupt_for_neben_len: self.haupt_for_neben.len(),
            ausgabe_art_len: self.ausgabe_art.len(),
            befehle_len: self.befehle.len(),
            befehle2_len: self.befehle2.len(),
            gebrochen_erlaubte_zahlen_len: self.gebrochen_erlaubte_zahlen.len(),
        }
    }
}

impl Default for PromptVocabulary {
    fn default() -> Self {
        Self {
            main_parameters: Vec::new(),
            spalten: Vec::new(),
            eigs_n: Vec::new(),
            eigs_r: Vec::new(),
            spalten_dict: BTreeMap::new(),
            ausgabe_paras: Vec::new(),
            kombi_main_paras: Vec::new(),
            zeilen_paras: Vec::new(),
            haupt_for_neben: Vec::new(),
            not_parameter_values: Vec::new(),
            haupt_for_neben_set: BTreeSet::new(),
            ausgabe_art: Vec::new(),
            zeilen_typen: Vec::new(),
            zeilen_zeit: Vec::new(),
            zeilen_typen_b: Vec::new(),
            gebrochen_erlaubte_zahlen: BTreeSet::new(),
            befehle: Vec::new(),
            befehle2: BTreeSet::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptVocabularySnapshot {
    pub main_parameters_len: usize,
    pub spalten_len: usize,
    pub spalten_dict_keys: usize,
    pub ausgabe_paras_len: usize,
    pub kombi_main_paras_len: usize,
    pub zeilen_paras_len: usize,
    pub haupt_for_neben_len: usize,
    pub ausgabe_art_len: usize,
    pub befehle_len: usize,
    pub befehle2_len: usize,
    pub gebrochen_erlaubte_zahlen_len: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptVocabularyBuilder {
    pub schema: RetaContextSchema,
    pub row_ranges: RowRangeSyntax,
}

impl PromptVocabularyBuilder {
    pub fn new(schema: RetaContextSchema, row_ranges: RowRangeSyntax) -> Self {
        Self { schema, row_ranges }
    }

    pub fn build_from_schema(&self) -> PromptVocabulary {
        let main_parameters = self
            .schema
            .main_alias_groups()
            .into_iter()
            .map(|group| format!("-{}", group.canonical))
            .collect::<Vec<_>>();

        let mut spalten = self
            .schema
            .sub_parameter_alias_groups()
            .keys()
            .map(|main| format!("--{main}="))
            .collect::<Vec<_>>();
        if !spalten.iter().any(|value| value == "--=") {
            spalten.push("--=".to_string());
        }

        let ausgabe_paras = self
            .schema
            .output_parameters
            .values()
            .map(|value| format!("--{value}="))
            .chain(std::iter::once("--*=".to_string()))
            .collect::<Vec<_>>();
        let kombi_main_paras = self
            .schema
            .combination_parameters
            .values()
            .map(|value| format!("--{value}="))
            .chain(std::iter::once("--*=".to_string()))
            .collect::<Vec<_>>();
        let zeilen_paras = self
            .schema
            .row_parameters
            .values()
            .map(|value| format!("--{value}="))
            .chain(std::iter::once("--*=".to_string()))
            .collect::<Vec<_>>();
        let haupt_for_neben = self
            .schema
            .scopes
            .values()
            .map(|value| format!("-{value}"))
            .collect::<Vec<_>>();
        let haupt_for_neben_set = haupt_for_neben.iter().cloned().collect::<BTreeSet<_>>();
        let ausgabe_art = self
            .schema
            .output_modes
            .values()
            .cloned()
            .collect::<Vec<_>>();

        let mut not_parameter_values = Vec::new();
        not_parameter_values.extend(ausgabe_paras.iter().cloned());
        not_parameter_values.extend(zeilen_paras.iter().cloned());
        not_parameter_values.extend(kombi_main_paras.iter().cloned());
        not_parameter_values.extend(spalten.iter().cloned());
        not_parameter_values.extend(main_parameters.iter().cloned());

        let mut befehle = vec![
            "reta".to_string(),
            "exit".to_string(),
            "quit".to_string(),
            "help".to_string(),
        ];
        befehle.extend(main_parameters.iter().cloned());
        let befehle2 = befehle
            .iter()
            .filter(|cmd| cmd.as_str() != "reta")
            .cloned()
            .collect();

        PromptVocabulary {
            main_parameters,
            spalten,
            ausgabe_paras,
            kombi_main_paras,
            zeilen_paras,
            haupt_for_neben,
            not_parameter_values,
            haupt_for_neben_set,
            ausgabe_art,
            zeilen_typen: vec![
                "sonne".to_string(),
                "mond".to_string(),
                "planet".to_string(),
                "*".to_string(),
            ],
            zeilen_zeit: vec![
                "heute".to_string(),
                "gestern".to_string(),
                "morgen".to_string(),
                "*".to_string(),
            ],
            zeilen_typen_b: vec![
                "aussenerste".to_string(),
                "innenerste".to_string(),
                "aussenalle".to_string(),
                "innenalle".to_string(),
                "*".to_string(),
            ],
            befehle,
            befehle2,
            ..PromptVocabulary::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InputBundle {
    pub schema: RetaContextSchema,
    pub row_ranges: RowRangeMorphismBundle,
    pub prompt_vocabulary_builder: PromptVocabularyBuilder,
}

impl InputBundle {
    pub fn from_schema(schema: RetaContextSchema, syntax: Option<RowRangeSyntax>) -> Self {
        let row_ranges = bootstrap_row_range_morphisms(syntax);
        let prompt_vocabulary_builder =
            PromptVocabularyBuilder::new(schema.clone(), row_ranges.syntax.clone());
        Self {
            schema,
            row_ranges,
            prompt_vocabulary_builder,
        }
    }

    pub fn build_prompt_vocabulary(&self) -> PromptVocabulary {
        self.prompt_vocabulary_builder.build_from_schema()
    }

    pub fn snapshot(&self) -> InputBundleSnapshot {
        InputBundleSnapshot {
            row_ranges_multiple_prefix: self.row_ranges.syntax.multiple_prefix.clone(),
            row_ranges_stage: self.row_ranges.activated_stage,
            prompt_vocabulary_builder_available: true,
            schema_main_alias_groups: self.schema.main_alias_groups().len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InputBundleSnapshot {
    pub row_ranges_multiple_prefix: String,
    pub row_ranges_stage: u32,
    pub prompt_vocabulary_builder_available: bool,
    pub schema_main_alias_groups: usize,
}

pub fn bootstrap_input_semantics(schema: Option<RetaContextSchema>) -> InputBundle {
    InputBundle::from_schema(schema.unwrap_or_else(crate::schema::bootstrap_schema), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_vocabulary_counts_main_aliases() {
        let bundle = bootstrap_input_semantics(None);
        let vocab = bundle.build_prompt_vocabulary();
        assert!(vocab
            .main_parameters
            .iter()
            .any(|value| value == "-spalten"));
        assert!(bundle.snapshot().prompt_vocabulary_builder_available);
    }
}
