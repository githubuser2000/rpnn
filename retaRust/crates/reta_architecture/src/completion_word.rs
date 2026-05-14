//! Word-completion morphisms transcompiled from
//! `python_arch_reference/reta_architecture/completion_word.py`.
//!
//! This mirrors the legacy `word_completerAlx.WordCompleter` matching rules in
//! a prompt-toolkit-free Rust section.  It gives `rretaPrompt` a typed local
//! completion morphism before the complete nested completer is replaced.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptDocument {
    pub text: String,
    pub cursor_position: usize,
}

impl PromptDocument {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let cursor_position = text.chars().count();
        Self {
            text,
            cursor_position,
        }
    }

    pub fn with_cursor(text: impl Into<String>, cursor_position: usize) -> Self {
        Self {
            text: text.into(),
            cursor_position,
        }
    }

    pub fn text_before_cursor(&self) -> String {
        self.text.chars().take(self.cursor_position).collect()
    }

    pub fn get_word_before_cursor(&self, word: bool) -> String {
        let before = self.text_before_cursor();
        if word {
            return before
                .split_whitespace()
                .last()
                .unwrap_or_default()
                .to_string();
        }
        let mut reversed = Vec::new();
        for ch in before.chars().rev() {
            if ch.is_alphanumeric() || ch == '_' {
                reversed.push(ch);
            } else {
                break;
            }
        }
        reversed.into_iter().rev().collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompletionCandidate {
    pub text: String,
    pub start_position: isize,
    pub display: String,
    pub display_meta: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WordCompletionOptions {
    pub ignore_case: bool,
    pub word: bool,
    pub sentence: bool,
    pub match_middle: bool,
    pub display_dict: BTreeMap<String, String>,
    pub meta_dict: BTreeMap<String, String>,
}

impl Default for WordCompletionOptions {
    fn default() -> Self {
        Self {
            ignore_case: false,
            word: false,
            sentence: false,
            match_middle: false,
            display_dict: BTreeMap::new(),
            meta_dict: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WordCompletionMorphismBundle {
    pub legacy_owner: String,
    pub activated_stage: u32,
}

impl WordCompletionMorphismBundle {
    pub fn resolve_words(&self, words: &[String]) -> Vec<String> {
        resolve_words(words)
    }

    pub fn word_before_cursor(
        &self,
        document: &PromptDocument,
        options: &WordCompletionOptions,
    ) -> String {
        word_before_cursor(document, options.word, options.sentence)
    }

    pub fn matches(&self, word: &str, prefix: &str, options: &WordCompletionOptions) -> bool {
        word_completion_matches(word, prefix, options.ignore_case, options.match_middle)
    }

    pub fn completions(
        &self,
        words: &[String],
        document: &PromptDocument,
        options: &WordCompletionOptions,
    ) -> Vec<CompletionCandidate> {
        iter_word_completions(words, document, options)
    }

    pub fn sample_completions(&self, prefix: &str) -> Vec<String> {
        self.completions(
            &[
                "reta".to_string(),
                "religion".to_string(),
                "alpha".to_string(),
            ],
            &PromptDocument::new(prefix),
            &WordCompletionOptions::default(),
        )
        .into_iter()
        .map(|item| item.text)
        .collect()
    }

    pub fn snapshot(&self) -> WordCompletionSnapshot {
        WordCompletionSnapshot {
            class: "WordCompletionMorphismBundle".to_string(),
            stage: self.activated_stage,
            legacy_owner: self.legacy_owner.clone(),
            capsule: "InputPromptCapsule".to_string(),
            category: "ActivatedWordCompletionCategory".to_string(),
            functor: "WordCompletionActivationFunctor".to_string(),
            natural_transformation: "WordCompleterToArchitectureTransformation".to_string(),
            morphisms: vec![
                "resolve_words".to_string(),
                "word_before_cursor".to_string(),
                "word_completion_matches".to_string(),
                "iter_word_completions".to_string(),
                "create_completer".to_string(),
            ],
            compatibility_names: vec![
                "WordCompleter".to_string(),
                "word_completerAlx.WordCompleter".to_string(),
            ],
            sample_prefix: "re".to_string(),
            sample_texts: self.sample_completions("re"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WordCompletionSnapshot {
    pub class: String,
    pub stage: u32,
    pub legacy_owner: String,
    pub capsule: String,
    pub category: String,
    pub functor: String,
    pub natural_transformation: String,
    pub morphisms: Vec<String>,
    pub compatibility_names: Vec<String>,
    pub sample_prefix: String,
    pub sample_texts: Vec<String>,
}

pub fn bootstrap_word_completion_morphisms() -> WordCompletionMorphismBundle {
    WordCompletionMorphismBundle {
        legacy_owner: "libs.word_completerAlx".to_string(),
        activated_stage: 40,
    }
}

pub fn resolve_words(words: &[String]) -> Vec<String> {
    words.to_vec()
}

pub fn word_before_cursor(document: &PromptDocument, word: bool, sentence: bool) -> String {
    if sentence {
        document.text_before_cursor()
    } else {
        document.get_word_before_cursor(word)
    }
}

pub fn word_completion_matches(
    word: &str,
    prefix: &str,
    ignore_case: bool,
    match_middle: bool,
) -> bool {
    let candidate = if ignore_case {
        word.to_lowercase()
    } else {
        word.to_string()
    };
    let probe = if ignore_case {
        prefix.to_lowercase()
    } else {
        prefix.to_string()
    };
    let prefix_slice = probe
        .chars()
        .take(candidate.chars().count())
        .collect::<String>();
    if match_middle {
        candidate.contains(&prefix_slice)
    } else {
        candidate.starts_with(&prefix_slice)
    }
}

pub fn iter_word_completions(
    words: &[String],
    document: &PromptDocument,
    options: &WordCompletionOptions,
) -> Vec<CompletionCandidate> {
    let prefix = word_before_cursor(document, options.word, options.sentence);
    let mut out = Vec::new();
    for word in resolve_words(words) {
        if word_completion_matches(&word, &prefix, options.ignore_case, options.match_middle) {
            out.push(CompletionCandidate {
                text: word.clone(),
                start_position: -(prefix.chars().count() as isize),
                display: options
                    .display_dict
                    .get(&word)
                    .cloned()
                    .unwrap_or_else(|| word.clone()),
                display_meta: options.meta_dict.get(&word).cloned().unwrap_or_default(),
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_before_cursor_matches_fallback_document() {
        let doc = PromptDocument::new("alpha reta");
        assert_eq!(word_before_cursor(&doc, false, false), "reta");
        assert_eq!(word_before_cursor(&doc, false, true), "alpha reta");
    }

    #[test]
    fn middle_match_preserves_python_slice_shape() {
        assert!(word_completion_matches("religion", "lig", false, true));
        assert!(word_completion_matches(
            "religion",
            "religionXYZ",
            false,
            false
        ));
        assert!(!word_completion_matches("alpha", "re", false, false));
    }

    #[test]
    fn completions_use_negative_prefix_start_position() {
        let bundle = bootstrap_word_completion_morphisms();
        let completions = bundle.completions(
            &["reta".to_string(), "alpha".to_string()],
            &PromptDocument::new("re"),
            &WordCompletionOptions::default(),
        );
        assert_eq!(completions[0].text, "reta");
        assert_eq!(completions[0].start_position, -2);
    }
}
