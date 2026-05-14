//! Console/help/wrapping morphisms transcompiled from
//! `python_arch_reference/reta_architecture/console_io.py`.
//!
//! The Rust layer keeps these operations pure where possible.  Printing is
//! represented as rendered text decisions; callers may perform the actual side
//! effect at the facade boundary.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DefaultOrderedDictSnapshot {
    pub class: String,
    pub default_factory: Option<String>,
    pub keys: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConsoleIOSnapshot {
    pub class: String,
    pub stage: u32,
    pub legacy_owner: String,
    pub capsule: String,
    pub secondary_capsule: String,
    pub category: String,
    pub functor: String,
    pub natural_transformation: String,
    pub repo_root: String,
    pub morphisms: Vec<String>,
    pub compatibility_names: Vec<String>,
    pub observable_invariant: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TextWrapRuntimeSnapshot {
    pub shell_width: usize,
    pub shell_rows_amount: usize,
    pub hyphenator: String,
    pub fill: String,
}

pub fn chunks<T: Clone>(sequence: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        return vec![sequence.to_vec()];
    }
    sequence.chunks(size).map(|chunk| chunk.to_vec()).collect()
}

pub fn unique_everseen<T, F>(iterable: impl IntoIterator<Item = T>, key: Option<F>) -> Vec<T>
where
    T: Clone + Ord,
    F: Fn(&T) -> String,
{
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for element in iterable {
        let marker = key
            .as_ref()
            .map(|func| func(&element))
            .unwrap_or_else(|| format_marker(&element));
        if seen.insert(marker) {
            out.push(element);
        }
    }
    out
}

fn format_marker<T: Ord>(value: &T) -> String {
    // The Python implementation stores the value itself in a set.  Rust keeps a
    // deterministic marker without requiring `Hash` on every caller type.
    format!("{:p}", value)
}

pub fn unique_strings_everseen(iterable: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for element in iterable {
        if seen.insert(element.clone()) {
            out.push(element);
        }
    }
    out
}

pub fn cli_output_text(
    text: impl AsRef<str>,
    color: bool,
    stype: impl AsRef<str>,
    output_enabled: bool,
) -> Option<String> {
    if !output_enabled {
        return None;
    }
    let text = text.as_ref();
    if color && !text.is_empty() {
        Some(format!(
            "<syntax type=\"{}\">{}</syntax>",
            stype.as_ref(),
            text.split_whitespace().collect::<Vec<_>>().join(" ")
        ))
    } else {
        Some(text.to_string())
    }
}

pub fn debug_pair_text(
    text1: impl AsRef<str>,
    text: impl ToString,
    info_log: bool,
    output_enabled: bool,
) -> Option<String> {
    (info_log && output_enabled).then(|| format!("{}: {}", text1.as_ref(), text.to_string()))
}

pub fn debug_value_text(
    text: impl ToString,
    info_log: bool,
    output_enabled: bool,
) -> Option<String> {
    (info_log && output_enabled).then(|| text.to_string())
}

pub fn doc_path(repo_root: impl AsRef<Path>, readme_filename: impl AsRef<str>) -> PathBuf {
    let mut path = repo_root.as_ref().to_path_buf();
    path.push("doc");
    let basename = Path::new(readme_filename.as_ref())
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or(readme_filename.as_ref());
    path.push(basename);
    path
}

pub fn strip_markdown_anchors(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("{#") {
        out.push_str(&rest[..start]);
        if let Some(end) = rest[start..].find('}') {
            rest = &rest[start + end + 1..];
        } else {
            rest = &rest[start..];
            break;
        }
    }
    out.push_str(rest);
    out
}

pub fn reta_prompt_help_text_from_markdown(markdown_text: &str) -> String {
    let without_anchors = strip_markdown_anchors(markdown_text);
    match without_anchors
        .get(2..)
        .and_then(|tail| tail.find("+++").map(|idx| idx + 2))
    {
        Some(start) => without_anchors[start + 3..].to_string(),
        None => without_anchors,
    }
}

pub fn get_text_wrap_things(max_len: Option<usize>) -> TextWrapRuntimeSnapshot {
    let shell_width = max_len.filter(|value| *value > 0).unwrap_or_else(|| {
        std::env::var("COLUMNS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(80)
    });
    let shell_rows_amount = std::env::var("LINES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(80);
    TextWrapRuntimeSnapshot {
        shell_width,
        shell_rows_amount,
        hyphenator: "rust-fallback".to_string(),
        fill: "std-wrap".to_string(),
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConsoleIOMorphismBundle {
    pub repo_root: String,
    pub legacy_owner: String,
    pub activated_stage: u32,
}

impl ConsoleIOMorphismBundle {
    pub fn chunks<T: Clone>(&self, sequence: &[T], size: usize) -> Vec<Vec<T>> {
        chunks(sequence, size)
    }

    pub fn unique_strings_everseen(
        &self,
        iterable: impl IntoIterator<Item = String>,
    ) -> Vec<String> {
        unique_strings_everseen(iterable)
    }

    pub fn cliout(
        &self,
        text: impl AsRef<str>,
        color: bool,
        stype: impl AsRef<str>,
        output_enabled: bool,
    ) -> Option<String> {
        cli_output_text(text, color, stype, output_enabled)
    }

    pub fn debug_pair(
        &self,
        text1: impl AsRef<str>,
        text: impl ToString,
        info_log: bool,
        output_enabled: bool,
    ) -> Option<String> {
        debug_pair_text(text1, text, info_log, output_enabled)
    }

    pub fn debug_value(
        &self,
        text: impl ToString,
        info_log: bool,
        output_enabled: bool,
    ) -> Option<String> {
        debug_value_text(text, info_log, output_enabled)
    }

    pub fn doc_path(&self, readme_filename: impl AsRef<str>) -> PathBuf {
        doc_path(&self.repo_root, readme_filename)
    }

    pub fn reta_prompt_help_text_from_markdown(&self, markdown_text: &str) -> String {
        reta_prompt_help_text_from_markdown(markdown_text)
    }

    pub fn text_wrap_runtime(&self, max_len: Option<usize>) -> TextWrapRuntimeSnapshot {
        get_text_wrap_things(max_len)
    }

    pub fn default_ordered_dict_snapshot(&self) -> DefaultOrderedDictSnapshot {
        DefaultOrderedDictSnapshot {
            class: "OrderedDefaultDict".to_string(),
            default_factory: None,
            keys: Vec::new(),
        }
    }

    pub fn snapshot(&self) -> ConsoleIOSnapshot {
        ConsoleIOSnapshot {
            class: "ConsoleIOMorphismBundle".to_string(),
            stage: self.activated_stage,
            legacy_owner: self.legacy_owner.clone(),
            capsule: "OutputRenderingCapsule".to_string(),
            secondary_capsule: "InputPromptCapsule".to_string(),
            category: "ActivatedConsoleIOCategory".to_string(),
            functor: "ConsoleIOActivationFunctor".to_string(),
            natural_transformation: "CenterConsoleIOToArchitectureTransformation".to_string(),
            repo_root: self.repo_root.clone(),
            morphisms: vec![
                "reta_prompt_help_text".to_string(),
                "print_reta_prompt_help".to_string(),
                "reta_help_text".to_string(),
                "print_reta_help".to_string(),
                "get_text_wrap_things".to_string(),
                "cli_output".to_string(),
                "debug_pair".to_string(),
                "debug_value".to_string(),
                "chunks".to_string(),
                "unique_everseen".to_string(),
                "DefaultOrderedDict".to_string(),
            ],
            compatibility_names: vec![
                "retaPromptHilfe".to_string(),
                "retaHilfe".to_string(),
                "getTextWrapThings".to_string(),
                "cliout".to_string(),
                "x".to_string(),
                "alxp".to_string(),
                "chunks".to_string(),
                "unique_everseen".to_string(),
                "DefaultOrderedDict".to_string(),
            ],
            observable_invariant: "center console/help/utility wrappers and ConsoleIOMorphismBundle expose the same visible output and finite-section helper results".to_string(),
        }
    }
}

pub fn bootstrap_console_io_morphisms(repo_root: Option<String>) -> ConsoleIOMorphismBundle {
    ConsoleIOMorphismBundle {
        repo_root: repo_root.unwrap_or_else(|| ".".to_string()),
        legacy_owner: "libs.center".to_string(),
        activated_stage: 39,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunks_preserve_order() {
        assert_eq!(
            chunks(&[1, 2, 3, 4, 5], 2),
            vec![vec![1, 2], vec![3, 4], vec![5]]
        );
    }

    #[test]
    fn prompt_help_strips_anchors_after_frontmatter() {
        let result = reta_prompt_help_text_from_markdown("xx+++\n# A {#anchor}\nbody");
        assert!(result.contains("# A"));
        assert!(!result.contains("{#anchor}"));
    }
}
