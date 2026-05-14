//! Split-i18n proxy transcompiled from
//! `python_arch_reference/reta_architecture/split_i18n.py`.
//!
//! Rust cannot import Python modules dynamically, so this module records the
//! source-module cover and offers a deterministic merge target for generated
//! i18n loaders.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub const DEFAULT_MODULE_NAMES: &[&str] = &[
    "i18n.words_context",
    "i18n.words_matrix",
    "i18n.words_runtime",
];

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SplitI18nProxy {
    pub source_modules: Vec<String>,
    pub values: BTreeMap<String, String>,
}

impl SplitI18nProxy {
    pub fn new(module_names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            source_modules: module_names.into_iter().map(Into::into).collect(),
            values: BTreeMap::new(),
        }
    }

    pub fn with_value(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.values.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    pub fn snapshot(&self) -> SplitI18nProxySnapshot {
        SplitI18nProxySnapshot {
            class: "SplitI18nProxy".to_string(),
            source_modules: self.source_modules.clone(),
            value_count: self.values.len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SplitI18nProxySnapshot {
    pub class: String,
    pub source_modules: Vec<String>,
    pub value_count: usize,
}

pub fn build_split_i18n_proxy(module_names: Option<Vec<String>>) -> SplitI18nProxy {
    SplitI18nProxy::new(module_names.unwrap_or_else(|| {
        DEFAULT_MODULE_NAMES
            .iter()
            .map(|value| value.to_string())
            .collect()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_proxy_names_split_modules() {
        let proxy = build_split_i18n_proxy(None);
        assert!(proxy
            .source_modules
            .contains(&"i18n.words_context".to_string()));
    }
}
