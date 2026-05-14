//! Output-mode semantics transcompiled from
//! `python_arch_reference/reta_architecture/output_semantics.py`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::output_syntax::{bootstrap_output_syntax, OutputMode, OutputModeSpec as OutputSyntaxModeSpec};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutputModeApplication {
    pub canonical_name: String,
    pub syntax_class_name: String,
    pub force_one_table: bool,
    pub force_zero_width: bool,
    pub marks_html_or_bbcode: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutputConfig {
    pub mode: OutputMode,
    pub one_table: bool,
    pub text_width: Option<i64>,
    pub marks_html_or_bbcode: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            mode: OutputMode::Shell,
            one_table: false,
            text_width: Some(21),
            marks_html_or_bbcode: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetaOutputSemantics {
    pub mode_specs: BTreeMap<String, OutputSyntaxModeSpec>,
    pub alias_to_mode: BTreeMap<String, String>,
}

impl RetaOutputSemantics {
    pub fn new() -> Self {
        let syntax = bootstrap_output_syntax();
        let mut mode_specs = BTreeMap::new();
        let mut alias_to_mode = BTreeMap::new();
        for mode in syntax.modes() {
            let spec = syntax.spec_for(mode);
            let canonical = spec.canonical_name.clone();
            alias_to_mode.insert(canonical.clone(), canonical.clone());
            alias_to_mode.insert(spec.cli_value.clone(), canonical.clone());
            for alias in &spec.aliases {
                alias_to_mode.insert(alias.clone(), canonical.clone());
            }
            mode_specs.insert(canonical, spec);
        }
        alias_to_mode.insert("md".to_string(), "markdown".to_string());
        alias_to_mode.insert("bb".to_string(), "bbcode".to_string());
        alias_to_mode.insert("nothing".to_string(), "nichts".to_string());
        Self {
            mode_specs,
            alias_to_mode,
        }
    }

    pub fn canonicalize(&self, value: Option<&str>) -> Option<String> {
        value.and_then(|raw| self.alias_to_mode.get(&raw.trim().to_ascii_lowercase()).cloned())
    }

    pub fn spec_for(&self, value: Option<&str>) -> Option<&OutputModeSpec> {
        let canonical = self.canonicalize(value)?;
        self.mode_specs.get(&canonical)
    }

    pub fn mode_for_name(&self, value: Option<&str>) -> Option<OutputMode> {
        self.canonicalize(value)
            .as_deref()
            .and_then(OutputMode::from_name)
    }

    pub fn mode_for_config(&self, config: &OutputConfig) -> OutputMode {
        config.mode
    }

    pub fn is_mode(&self, config: &OutputConfig, mode: &str) -> bool {
        self.mode_for_name(Some(mode)) == Some(config.mode)
    }

    pub fn apply_mode_to_config(
        &self,
        config: &mut OutputConfig,
        mode: &str,
    ) -> Option<OutputModeApplication> {
        let canonical = self.canonicalize(Some(mode))?;
        let mode_value = OutputMode::from_name(&canonical)?;
        let spec = self.mode_specs.get(&canonical)?;
        config.mode = mode_value;
        if spec.force_one_table {
            config.one_table = true;
        }
        if spec.force_zero_width {
            config.text_width = Some(0);
        }
        if spec.marks_html_or_bbcode {
            config.marks_html_or_bbcode = true;
        }
        Some(OutputModeApplication {
            canonical_name: spec.canonical_name.clone(),
            syntax_class_name: spec.syntax_class.clone(),
            force_one_table: spec.force_one_table,
            force_zero_width: spec.force_zero_width,
            marks_html_or_bbcode: spec.marks_html_or_bbcode,
        })
    }

    pub fn snapshot(&self) -> OutputSemanticsSnapshot {
        OutputSemanticsSnapshot {
            available_modes: self.mode_specs.keys().cloned().collect(),
            mode_specs: self.mode_specs.clone(),
        }
    }
}

impl Default for RetaOutputSemantics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutputSemanticsSnapshot {
    pub available_modes: Vec<String>,
    pub mode_specs: BTreeMap<String, OutputSyntaxModeSpec>,
}

pub fn bootstrap_output_semantics() -> RetaOutputSemantics {
    RetaOutputSemantics::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_aliases_canonicalize() {
        let semantics = bootstrap_output_semantics();
        assert_eq!(semantics.canonicalize(Some("md")), Some("markdown".to_string()));
        assert_eq!(semantics.canonicalize(Some("bbcode")), Some("bbcode".to_string()));
        assert_eq!(semantics.mode_for_name(Some("html")), Some(OutputMode::Html));
    }

    #[test]
    fn apply_mode_sets_legacy_side_effects() {
        let semantics = bootstrap_output_semantics();
        let mut config = OutputConfig::default();
        let application = semantics.apply_mode_to_config(&mut config, "csv").unwrap();
        assert_eq!(application.canonical_name, "csv");
        assert!(config.one_table);
        assert_eq!(config.text_width, Some(0));
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "__init__",
    "_bootstrap_output_semantics",
    "apply_mode_to_tables",
    "create_syntax",
    "mode_for_output_syntax",
    "mode_for_tables",
    "OutputModeSpec",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}

// Stage 16 small-surface concrete wrappers.
pub type OutputModeSpec = OutputSyntaxModeSpec;

pub fn __init__() -> RetaOutputSemantics {
    RetaOutputSemantics::new()
}

pub fn _bootstrap_output_semantics() -> RetaOutputSemantics {
    bootstrap_output_semantics()
}

pub fn create_syntax() -> crate::output_syntax::OutputSyntaxBundle {
    bootstrap_output_syntax()
}

pub fn mode_for_tables(config: &OutputConfig) -> OutputMode {
    config.mode
}

pub fn mode_for_output_syntax(mode: &str) -> Option<OutputMode> {
    OutputMode::from_name(mode)
}

pub fn apply_mode_to_tables(config: &mut OutputConfig, mode: &str) -> Option<OutputModeApplication> {
    RetaOutputSemantics::new().apply_mode_to_config(config, mode)
}
