//! Text wrapping and display-width morphisms transcompiled from
//! `python_arch_reference/reta_architecture/table_wrapping.py`.
//!
//! The Python source can delegate to `pyphen`/`pyhyphen`.  This Rust layer keeps
//! the same architecture and boundary rules, while using a deterministic
//! dependency-free fallback for actual splitting.  That makes the wrapping
//! morphism available to `rreta` before the final legacy renderer is replaced.

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum WrapType {
    Pyphen,
    #[default]
    Pyhyphen,
    Nohyphen,
}

impl WrapType {
    pub const fn py_name(self) -> &'static str {
        match self {
            Self::Pyphen => "pyphen",
            Self::Pyhyphen => "pyhyphen",
            Self::Nohyphen => "nohyphen",
        }
    }

    pub fn from_py_name(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "pyphen" => Some(Self::Pyphen),
            "pyhyphen" => Some(Self::Pyhyphen),
            "nohyphen" | "none" | "no" => Some(Self::Nohyphen),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TextWrapRuntime {
    pub shell_rows_amount: Option<i64>,
    pub has_hyphenator: bool,
    pub has_dictionary: bool,
    pub has_fill: bool,
    pub wrapping_type: WrapType,
}

impl Default for TextWrapRuntime {
    fn default() -> Self {
        Self {
            shell_rows_amount: Some(0),
            has_hyphenator: false,
            has_dictionary: false,
            has_fill: false,
            wrapping_type: WrapType::Pyhyphen,
        }
    }
}

impl TextWrapRuntime {
    pub fn snapshot(&self) -> TextWrapRuntimeSnapshot {
        TextWrapRuntimeSnapshot {
            class: "TextWrapRuntime".to_string(),
            shell_rows_amount: self.shell_rows_amount,
            has_hyphenator: self.has_hyphenator,
            has_dictionary: self.has_dictionary,
            has_fill: self.has_fill,
            wrapping_type: self.wrapping_type.py_name().to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TextWrapRuntimeSnapshot {
    pub class: String,
    pub shell_rows_amount: Option<i64>,
    pub has_hyphenator: bool,
    pub has_dictionary: bool,
    pub has_fill: bool,
    pub wrapping_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableWidthContext {
    pub shell_rows_amount: Option<i64>,
    pub rows_as_numbers_len: usize,
    pub breiten: Vec<i64>,
    pub textwidth: i64,
}

impl Default for TableWidthContext {
    fn default() -> Self {
        Self {
            shell_rows_amount: Some(0),
            rows_as_numbers_len: 0,
            breiten: Vec::new(),
            textwidth: 21,
        }
    }
}

pub fn chunks(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }
    let chars = text.chars().collect::<Vec<_>>();
    if chars.is_empty() {
        return vec![String::new()];
    }
    chars
        .chunks(width)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}

pub fn split_more_if_not_small<S: AsRef<str>>(text_list: &[S], len_to_be: usize) -> Vec<String> {
    if len_to_be == 0 {
        return text_list
            .iter()
            .map(|item| item.as_ref().to_string())
            .collect();
    }
    let needed = text_list
        .iter()
        .any(|item| item.as_ref().chars().count() > len_to_be);
    if !needed {
        return text_list
            .iter()
            .map(|item| item.as_ref().to_string())
            .collect();
    }

    let mut out = Vec::new();
    for item in text_list {
        let text = item.as_ref();
        if text.chars().count() > len_to_be {
            out.extend(chunks(text, len_to_be));
        } else {
            out.push(text.to_string());
        }
    }
    out
}

pub fn alxwrap(text: &str, len_: usize, wrapping_type: Option<WrapType>) -> Vec<String> {
    if len_ == 0 || matches!(wrapping_type, Some(WrapType::Nohyphen)) {
        return vec![text.to_string()];
    }

    // Python prefers dictionary/hyphenator backends when available.  This Rust
    // fallback preserves the local-to-global morphism boundary and guarantees
    // stable deterministic sections without introducing a new dependency.
    let words = text.split('\n').collect::<Vec<_>>();
    split_more_if_not_small(&words, len_)
}

pub fn wrap_cell_text(
    text: &str,
    length: usize,
    wrapping_type: Option<WrapType>,
) -> Option<Vec<String>> {
    (length != 0 && text.chars().count() > length).then(|| alxwrap(text, length, wrapping_type))
}

pub fn width_for_row_context(
    context: &TableWidthContext,
    row_to_display: usize,
    combi_rows1: usize,
) -> i64 {
    if context.shell_rows_amount == Some(0) {
        return 0;
    }
    let combi_rows = if combi_rows1 != 0 {
        combi_rows1
    } else {
        context.rows_as_numbers_len
    };
    let breiten = if context.rows_as_numbers_len.saturating_sub(combi_rows) < context.breiten.len()
    {
        context.breiten[context.rows_as_numbers_len.saturating_sub(combi_rows)..].to_vec()
    } else {
        Vec::new()
    };
    let index = row_to_display.saturating_sub(1);
    breiten.get(index).copied().unwrap_or(context.textwidth)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableWrappingBundle {
    pub runtime: TextWrapRuntime,
}

impl TableWrappingBundle {
    pub fn wrap_text(&self, text: &str, length: usize) -> Option<Vec<String>> {
        wrap_cell_text(text, length, Some(self.runtime.wrapping_type))
    }

    pub fn width_for_row(
        &self,
        context: &TableWidthContext,
        row_to_display: usize,
        combi_rows1: usize,
    ) -> i64 {
        width_for_row_context(context, row_to_display, combi_rows1)
    }

    pub fn snapshot(&self) -> TableWrappingBundleSnapshot {
        TableWrappingBundleSnapshot {
            class: "TableWrappingBundle".to_string(),
            runtime: self.runtime.snapshot(),
            morphisms: vec![
                "alxwrap".to_string(),
                "wrap_cell_text".to_string(),
                "width_for_row".to_string(),
                "split_more_if_not_small".to_string(),
            ],
            legacy_owner: "libs.lib4tables_prepare.Prepare".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableWrappingBundleSnapshot {
    pub class: String,
    pub runtime: TextWrapRuntimeSnapshot,
    pub morphisms: Vec<String>,
    pub legacy_owner: String,
}

pub fn bootstrap_table_wrapping() -> TableWrappingBundle {
    TableWrappingBundle {
        runtime: TextWrapRuntime::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_more_chunks_long_entries_only_when_needed() {
        let result = split_more_if_not_small(&["abcde", "xy"], 2);
        assert_eq!(result, vec!["ab", "cd", "e", "xy"]);
        let unchanged = split_more_if_not_small(&["ab", "xy"], 2);
        assert_eq!(unchanged, vec!["ab", "xy"]);
    }

    #[test]
    fn width_context_follows_python_zero_shell_rule() {
        let context = TableWidthContext::default();
        assert_eq!(width_for_row_context(&context, 1, 0), 0);
        let context = TableWidthContext {
            shell_rows_amount: Some(80),
            rows_as_numbers_len: 3,
            breiten: vec![5, 6],
            textwidth: 21,
        };
        assert_eq!(width_for_row_context(&context, 1, 0), 5);
        assert_eq!(width_for_row_context(&context, 3, 0), 21);
    }
}
