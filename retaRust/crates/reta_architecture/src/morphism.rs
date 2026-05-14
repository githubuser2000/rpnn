//! Morphism layer transcompiled from
//! `python_arch_reference/reta_architecture/morphisms.py`.
//!
//! Earlier stages only carried the graph-level `MorphismEdge`.  Stage 13 adds
//! the Python architecture's concrete morphism bundles: alias resolution, row
//! range parsing, prompt splitting and output-mode application.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::output_semantics::{OutputConfig, OutputModeApplication, RetaOutputSemantics};
use crate::sheaf::{ParameterSemanticsSheaf, SheafBundle};
use crate::topology::RetaContextTopology;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MorphismKind {
    Parse,
    Resolve,
    Select,
    Derive,
    Generate,
    Format,
    Annotate,
    Enqueue,
    Dequeue,
    Dispatch,
    Glue,
    Render,
}

impl MorphismKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Parse => "parse",
            Self::Resolve => "resolve",
            Self::Select => "select",
            Self::Derive => "derive",
            Self::Generate => "generate",
            Self::Format => "format",
            Self::Annotate => "annotate",
            Self::Enqueue => "enqueue",
            Self::Dequeue => "dequeue",
            Self::Dispatch => "dispatch",
            Self::Glue => "glue",
            Self::Render => "render",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MorphismEdge {
    pub name: String,
    pub source: String,
    pub target: String,
    pub kind: MorphismKind,
    pub owner: String,
}

impl MorphismEdge {
    pub fn new(
        name: impl Into<String>,
        source: impl Into<String>,
        target: impl Into<String>,
        kind: MorphismKind,
        owner: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            source: source.into(),
            target: target.into(),
            kind,
            owner: owner.into(),
        }
    }

    pub fn then(&self, next: &Self, composed_name: impl Into<String>) -> Option<Self> {
        if self.target != next.source {
            return None;
        }
        Some(Self {
            name: composed_name.into(),
            source: self.source.clone(),
            target: next.target.clone(),
            kind: next.kind,
            owner: format!("{} ∘ {}", next.owner, self.owner),
        })
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct MorphismGraph {
    pub edges: Vec<MorphismEdge>,
}

impl MorphismGraph {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    pub fn add(&mut self, edge: MorphismEdge) {
        self.edges.push(edge);
    }

    pub fn outgoing<'a>(&'a self, source: &'a str) -> impl Iterator<Item = &'a MorphismEdge> + 'a {
        self.edges.iter().filter(move |edge| edge.source == source)
    }

    pub fn compose_named(&self, first: &str, second: &str, name: &str) -> Option<MorphismEdge> {
        let first = self.edges.iter().find(|edge| edge.name == first)?;
        let second = self.edges.iter().find(|edge| edge.name == second)?;
        first.then(second, name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AliasMorphisms {
    pub topology: RetaContextTopology,
    pub parameter_semantics: ParameterSemanticsSheaf,
}

impl AliasMorphisms {
    pub fn resolve_main_alias(&self, main_name: &str) -> Option<String> {
        self.parameter_semantics.resolve_main_alias(main_name)
    }

    pub fn resolve_parameter_alias(
        &self,
        main_name: &str,
        parameter_name: &str,
    ) -> Option<String> {
        self.parameter_semantics
            .resolve_parameter_alias(main_name, parameter_name)
    }

    pub fn canonicalize_pair(
        &self,
        main_name: &str,
        parameter_name: &str,
    ) -> Option<(String, String)> {
        self.parameter_semantics
            .canonicalize_pair(main_name, parameter_name)
    }

    pub fn column_numbers_for_pair(&self, main_name: &str, parameter_name: &str) -> Vec<i64> {
        self.parameter_semantics
            .column_numbers_for_pair(main_name, parameter_name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RangeMorphisms {
    pub topology: RetaContextTopology,
}

impl RangeMorphisms {
    pub fn parse_row_range<I, F>(&self, text: &str, parser: F) -> Vec<i64>
    where
        I: IntoIterator<Item = i64>,
        F: Fn(&str) -> I,
    {
        parser(text).into_iter().collect::<BTreeSet<_>>().into_iter().collect()
    }

    pub fn parse_row_range_with<I, F>(&self, text: &str, parser: F) -> Vec<i64>
    where
        I: IntoIterator<Item = i64>,
        F: Fn(&str) -> I,
    {
        self.parse_row_range(text, parser)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptMorphisms {
    pub topology: RetaContextTopology,
}

impl PromptMorphisms {
    pub fn split<F>(&self, text: &str, splitter: F) -> Vec<String>
    where
        F: Fn(&str) -> Vec<String>,
    {
        splitter(text)
    }

    pub fn split_with<F>(&self, text: &str, splitter: F) -> Vec<String>
    where
        F: Fn(&str) -> Vec<String>,
    {
        self.split(text, splitter)
    }

    pub fn split_prompt_text<F>(&self, text: &str, splitter: F) -> Vec<String>
    where
        F: Fn(&str) -> Vec<String>,
    {
        self.split(text, splitter)
    }

    pub fn split_prompt_text_with<F>(&self, text: &str, splitter: F) -> Vec<String>
    where
        F: Fn(&str) -> Vec<String>,
    {
        self.split_prompt_text(text, splitter)
    }

    pub fn split_command_words<F>(&self, text: &str, splitter: F) -> Vec<String>
    where
        F: Fn(&str) -> Vec<String>,
    {
        if !text.starts_with("reta") {
            return self.split(text, splitter);
        }
        text.split_whitespace()
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(ToOwned::to_owned)
            .collect()
    }

    pub fn split_command_words_with<F>(&self, text: &str, splitter: F) -> Vec<String>
    where
        F: Fn(&str) -> Vec<String>,
    {
        self.split_command_words(text, splitter)
    }

    pub fn expand_shorthand<T, F>(
        &self,
        prompt_mode: &str,
        stext: &str,
        text_dazu: &str,
        expander: F,
    ) -> T
    where
        F: Fn(&str, &str, &str) -> T,
    {
        expander(prompt_mode, stext, text_dazu)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RendererMorphisms {
    pub topology: RetaContextTopology,
    pub output_semantics: RetaOutputSemantics,
}

impl RendererMorphisms {
    pub fn output_mode_for_tables(&self, config: &OutputConfig) -> String {
        self.output_semantics
            .mode_for_config(config)
            .canonical_name()
            .to_string()
    }

    pub fn output_mode_for_config(&self, config: &OutputConfig) -> String {
        self.output_mode_for_tables(config)
    }

    pub fn output_mode_for_name(&self, mode: Option<&str>) -> Option<String> {
        self.output_semantics.canonicalize(mode)
    }

    pub fn apply_output_mode(
        &self,
        config: &mut OutputConfig,
        mode: &str,
    ) -> Option<OutputModeApplication> {
        self.output_semantics.apply_mode_to_config(config, mode)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MorphismBundle {
    pub alias: AliasMorphisms,
    pub ranges: RangeMorphisms,
    pub prompt: PromptMorphisms,
    pub renderers: RendererMorphisms,
}

impl MorphismBundle {
    pub fn from_topology_and_sheaves(
        topology: &RetaContextTopology,
        sheaves: &SheafBundle,
        output_semantics: Option<RetaOutputSemantics>,
    ) -> Self {
        let output_semantics = output_semantics.unwrap_or_default();
        Self {
            alias: AliasMorphisms {
                topology: topology.clone(),
                parameter_semantics: sheaves.parameter_semantics.clone(),
            },
            ranges: RangeMorphisms {
                topology: topology.clone(),
            },
            prompt: PromptMorphisms {
                topology: topology.clone(),
            },
            renderers: RendererMorphisms {
                topology: topology.clone(),
                output_semantics,
            },
        }
    }

    pub fn snapshot(&self) -> MorphismBundleSnapshot {
        MorphismBundleSnapshot {
            available: vec![
                "alias".to_string(),
                "ranges".to_string(),
                "prompt".to_string(),
                "renderers".to_string(),
            ],
            main_alias_count: self.alias.parameter_semantics.main_alias_map.len(),
            parameter_alias_main_count: self
                .alias
                .parameter_semantics
                .parameter_alias_groups
                .len(),
            output_mode_count: self.renderers.output_semantics.mode_specs.len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MorphismBundleSnapshot {
    pub available: Vec<String>,
    pub main_alias_count: usize,
    pub parameter_alias_main_count: usize,
    pub output_mode_count: usize,
}

pub fn bootstrap_semantic_morphisms(
    topology: &RetaContextTopology,
    sheaves: &SheafBundle,
    output_semantics: Option<RetaOutputSemantics>,
) -> MorphismBundle {
    MorphismBundle::from_topology_and_sheaves(topology, sheaves, output_semantics)
}

pub fn morphism_snapshot_terms() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("AliasMorphisms", "resolve main/sub aliases and canonical parameter pairs"),
        ("RangeMorphisms", "parse row range sections through a supplied parser"),
        ("PromptMorphisms", "split prompt text and command words"),
        ("RendererMorphisms", "map output syntax into table/output config"),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ParameterMatrixEntry, RetaContextSchema};
    use crate::sheaf::bootstrap_sheaves;

    #[test]
    fn semantic_morphism_bundle_resolves_alias_columns() {
        let schema = RetaContextSchema {
            parameters_main: vec![vec!["spalten".into(), "s".into()]],
            para_n_data_matrix: vec![ParameterMatrixEntry {
                main_aliases: vec!["spalten".into()],
                parameter_aliases: vec!["kontinuum".into(), "m".into()],
                columns: vec![493, 744],
            }],
            ..Default::default()
        };
        let sheaves = bootstrap_sheaves(Some(&schema));
        let bundle = bootstrap_semantic_morphisms(
            &RetaContextTopology::standard(),
            &sheaves,
            Some(RetaOutputSemantics::default()),
        );
        assert_eq!(bundle.alias.resolve_main_alias("s"), Some("spalten".into()));
        assert_eq!(bundle.alias.column_numbers_for_pair("s", "m"), vec![493, 744]);
        assert_eq!(
            bundle.renderers.output_mode_for_name(Some("md")),
            Some("markdown".to_string())
        );
    }

    #[test]
    fn prompt_command_words_match_python_reta_branch() {
        let bundle = PromptMorphisms {
            topology: RetaContextTopology::standard(),
        };
        let words = bundle.split_command_words_with("reta -zeilen --alles", |text| {
            text.split(',').map(ToOwned::to_owned).collect()
        });
        assert_eq!(words, vec!["reta", "-zeilen", "--alles"]);
    }
}
