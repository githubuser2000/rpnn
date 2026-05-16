//! Prompt language-completion witness for `rretaPrompt`.
//!
//! Stages 55-63 hardened the TableView language path after `religion.csv`
//! gained direct column 744.  This module brings the same language surface into
//! the prompt-completion path: language switches are suggested, language values
//! are completed, and the prompt can expose the language coverage/sync witnesses
//! that will later govern the `reta` command it compiles.

use serde::{Deserialize, Serialize};

use crate::csv_catalog::{csv_language_from_cli_args, normalize_language_value, CsvLanguage};
use crate::table_view_language_coverage::{
    language_coverage_for_cli_args, TableViewLanguageCoveragePolicy, TableViewLanguageCoverageReport,
};
use crate::table_view_language_sync::{
    language_sync_for_cli_args, TableViewLanguageSyncPolicy, TableViewLanguageSyncReport,
};

pub const LANGUAGE_PARAMETER_COMPLETIONS: &[&str] = &[
    "-language=",
    "--language=",
    "-languages=",
    "--languages=",
    "-lang=",
    "--lang=",
    "-sprache=",
    "--sprache=",
    "-sprachen=",
    "--sprachen=",
];

pub const LANGUAGE_VALUE_COMPLETIONS: &[&str] = &[
    "de",
    "deutsch",
    "german",
    "base",
    "en",
    "english",
    "englisch",
    "cn",
    "chinese",
    "chinesisch",
    "vn",
    "vietnamese",
    "vietnamesisch",
    "kr",
    "korean",
    "koreanisch",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PromptLanguageCompletionSituation {
    None,
    Parameter,
    Value,
}

impl PromptLanguageCompletionSituation {
    pub const fn canonical(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Parameter => "language_parameter",
            Self::Value => "language_value",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLanguageCompletionCandidate {
    pub text: String,
    pub start_position: isize,
    pub source: String,
    pub canonical_language: Option<String>,
    pub will_select_language: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLanguageCompletionContext {
    pub text: String,
    pub argv: Vec<String>,
    pub current_token: String,
    pub situation: PromptLanguageCompletionSituation,
    pub parameter_prefix: String,
    pub value_prefix: String,
    pub selected_language: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLanguageCompletionPolicy {
    pub include_parameter_candidates: bool,
    pub include_value_candidates: bool,
    pub fuzzy_alias_prefix: bool,
    pub include_language_sync_witness: bool,
    pub max_candidates: usize,
}

impl Default for PromptLanguageCompletionPolicy {
    fn default() -> Self {
        Self {
            include_parameter_candidates: true,
            include_value_candidates: true,
            fuzzy_alias_prefix: true,
            include_language_sync_witness: true,
            max_candidates: 32,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLanguageCompletionReport {
    pub class: String,
    pub context: PromptLanguageCompletionContext,
    pub candidates: Vec<PromptLanguageCompletionCandidate>,
    pub candidate_count: usize,
    pub language_coverage: TableViewLanguageCoverageReport,
    pub language_sync: TableViewLanguageSyncReport,
    pub language_coverage_ready: bool,
    pub language_sync_ready: bool,
    pub direct_744_available_for_selected_language: bool,
    pub continuum_m_uses_synced_language_asset: bool,
    pub status: String,
    pub failed_guards: Vec<String>,
    pub universal_property: String,
}

impl PromptLanguageCompletionReport {
    pub fn ready(&self) -> bool {
        self.status == "ready"
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLanguageCompletionSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub language_parameter_count: usize,
    pub language_value_count: usize,
    pub smoke_candidate_count: usize,
    pub smoke_language_sync_ready: bool,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLanguageCompletionBundle {
    pub morphisms: Vec<String>,
    pub language_parameters: Vec<String>,
    pub language_values: Vec<String>,
    pub universal_property: String,
}

impl PromptLanguageCompletionBundle {
    pub fn complete_prompt_language(&self, text: &str, policy: &PromptLanguageCompletionPolicy) -> PromptLanguageCompletionReport {
        prompt_language_completion_for_text(text, policy)
    }

    pub fn snapshot(&self) -> PromptLanguageCompletionSnapshot {
        let smoke = continuum_m_prompt_language_completion_smoke();
        PromptLanguageCompletionSnapshot {
            class: "PromptLanguageCompletionSnapshot".to_string(),
            morphisms: self.morphisms.clone(),
            language_parameter_count: self.language_parameters.len(),
            language_value_count: self.language_values.len(),
            smoke_candidate_count: smoke.candidate_count,
            smoke_language_sync_ready: smoke.language_sync_ready,
            universal_property: self.universal_property.clone(),
        }
    }
}

pub fn bootstrap_prompt_language_completion() -> PromptLanguageCompletionBundle {
    PromptLanguageCompletionBundle {
        morphisms: vec![
            "prompt_language_completion.language_parameter_candidates".to_string(),
            "prompt_language_completion.language_value_candidates".to_string(),
            "prompt_language_completion.prompt_to_reta_argv".to_string(),
            "prompt_language_completion.language_sync_witness".to_string(),
            "prompt_language_completion.direct_744_prompt_guard".to_string(),
        ],
        language_parameters: language_parameter_completions(),
        language_values: language_value_completions(),
        universal_property: "prompt_language_completion_must_glue_to_the_same_language_asset_cover_as_the_compiled_reta_command".to_string(),
    }
}

pub fn language_parameter_completions() -> Vec<String> {
    LANGUAGE_PARAMETER_COMPLETIONS.iter().map(|value| value.to_string()).collect()
}

pub fn language_value_completions() -> Vec<String> {
    LANGUAGE_VALUE_COMPLETIONS.iter().map(|value| value.to_string()).collect()
}

pub fn prompt_language_completion_for_text(
    text: &str,
    policy: &PromptLanguageCompletionPolicy,
) -> PromptLanguageCompletionReport {
    let context = prompt_language_completion_context(text);
    let mut candidates = prompt_language_completion_candidates(&context, policy);
    if candidates.len() > policy.max_candidates {
        candidates.truncate(policy.max_candidates);
    }
    let coverage = language_coverage_for_cli_args(&context.argv, &TableViewLanguageCoveragePolicy::default());
    let sync = language_sync_for_cli_args(&context.argv, &TableViewLanguageSyncPolicy::strict());
    let selected_language = csv_language_from_cli_args(&context.argv);
    let selected_language_name = selected_language.canonical();
    let direct_744_available = coverage
        .language_assets
        .iter()
        .find(|asset| asset.language == selected_language_name)
        .map(|asset| asset.direct_744_available)
        .unwrap_or(false);
    let continuum_m_uses_synced_language_asset = context
        .argv
        .iter()
        .any(|arg| arg.contains("kontinuum=m"))
        && coverage
            .language_assets
            .iter()
            .find(|asset| asset.language == selected_language_name)
            .map(|asset| asset.supports_required_columns && asset.direct_744_available)
            .unwrap_or(false);

    let mut failed_guards = Vec::new();
    if !coverage.ready() {
        failed_guards.push("language_coverage_not_ready_for_prompt".to_string());
    }
    if policy.include_language_sync_witness && !sync.ready() {
        failed_guards.push("language_sync_not_ready_for_prompt".to_string());
    }
    if context.argv.iter().any(|arg| arg.contains("kontinuum=m")) && !direct_744_available {
        failed_guards.push("prompt_language_does_not_have_direct_744".to_string());
    }
    let status = if failed_guards.is_empty() { "ready" } else { "blocked" }.to_string();
    PromptLanguageCompletionReport {
        class: "PromptLanguageCompletionReport".to_string(),
        candidate_count: candidates.len(),
        candidates,
        context,
        language_coverage_ready: coverage.ready(),
        language_sync_ready: sync.ready(),
        direct_744_available_for_selected_language: direct_744_available,
        continuum_m_uses_synced_language_asset,
        language_coverage: coverage,
        language_sync: sync,
        status,
        failed_guards,
        universal_property: "language_completion_candidates_and_prompt_language_flags_select_the_same_csv_language_cover_as_table_view_materialization".to_string(),
    }
}

pub fn prompt_language_completion_context(text: &str) -> PromptLanguageCompletionContext {
    let argv = prompt_text_to_reta_argv(text);
    let current_token = current_prompt_token(text);
    let selected_language = csv_language_from_cli_args(&argv).canonical().to_string();
    let (situation, parameter_prefix, value_prefix) = if let Some((_key, value_prefix)) = language_switch_key_value(&current_token) {
        (
            PromptLanguageCompletionSituation::Value,
            String::new(),
            value_prefix,
        )
    } else if current_token.starts_with('-') && language_parameter_completions()
        .iter()
        .any(|candidate| candidate.starts_with(&current_token) || candidate.trim_start_matches('-').starts_with(current_token.trim_start_matches('-')))
    {
        (
            PromptLanguageCompletionSituation::Parameter,
            current_token.clone(),
            String::new(),
        )
    } else if current_token.is_empty() && argv.first().is_some_and(|value| value == "reta") {
        (
            PromptLanguageCompletionSituation::Parameter,
            String::new(),
            String::new(),
        )
    } else {
        (
            PromptLanguageCompletionSituation::None,
            String::new(),
            String::new(),
        )
    };
    PromptLanguageCompletionContext {
        text: text.to_string(),
        argv,
        current_token,
        situation,
        parameter_prefix,
        value_prefix,
        selected_language,
    }
}

pub fn prompt_language_completion_candidates(
    context: &PromptLanguageCompletionContext,
    policy: &PromptLanguageCompletionPolicy,
) -> Vec<PromptLanguageCompletionCandidate> {
    let mut candidates = match context.situation {
        PromptLanguageCompletionSituation::Parameter if policy.include_parameter_candidates => {
            language_parameter_candidates(&context.parameter_prefix)
        }
        PromptLanguageCompletionSituation::Value if policy.include_value_candidates => {
            language_value_candidates(&context.value_prefix, policy.fuzzy_alias_prefix)
        }
        _ => Vec::new(),
    };
    candidates.sort_by(|left, right| left.text.cmp(&right.text));
    candidates.dedup_by(|left, right| left.text == right.text && left.start_position == right.start_position);
    candidates
}

pub fn language_parameter_candidates(prefix: &str) -> Vec<PromptLanguageCompletionCandidate> {
    language_parameter_completions()
        .into_iter()
        .filter(|candidate| prefix.is_empty() || candidate.starts_with(prefix) || candidate.trim_start_matches('-').starts_with(prefix.trim_start_matches('-')))
        .map(|text| PromptLanguageCompletionCandidate {
            start_position: -(prefix.chars().count() as isize),
            text,
            source: "language_parameter".to_string(),
            canonical_language: None,
            will_select_language: None,
        })
        .collect()
}

pub fn language_value_candidates(prefix: &str, fuzzy: bool) -> Vec<PromptLanguageCompletionCandidate> {
    let normalized_prefix = normalize_language_value(prefix);
    language_value_completions()
        .into_iter()
        .filter(|candidate| {
            if normalized_prefix.is_empty() {
                return true;
            }
            let normalized_candidate = normalize_language_value(candidate);
            if fuzzy {
                normalized_candidate.starts_with(&normalized_prefix) || normalized_candidate.contains(&normalized_prefix)
            } else {
                normalized_candidate.starts_with(&normalized_prefix)
            }
        })
        .map(|text| {
            let language = CsvLanguage::from_language_value(&text).map(|lang| lang.canonical().to_string());
            PromptLanguageCompletionCandidate {
                start_position: -(prefix.chars().count() as isize),
                text,
                source: "language_value".to_string(),
                canonical_language: language.clone(),
                will_select_language: language,
            }
        })
        .collect()
}

pub fn language_switch_key_value(token: &str) -> Option<(String, String)> {
    let body = token
        .trim()
        .strip_prefix("--")
        .or_else(|| token.trim().strip_prefix('-'))
        .unwrap_or_else(|| token.trim());
    let (key, value) = body.split_once('=')?;
    let key_normalized = key.trim().to_ascii_lowercase();
    if matches!(key_normalized.as_str(), "language" | "languages" | "lang" | "sprache" | "sprachen") {
        Some((key_normalized, value.to_string()))
    } else {
        None
    }
}

pub fn is_partial_language_parameter(token: &str) -> bool {
    let trimmed = token.trim();
    if !trimmed.starts_with('-') {
        return false;
    }
    language_parameter_completions()
        .iter()
        .any(|candidate| candidate.starts_with(trimmed) || candidate.trim_start_matches('-').starts_with(trimmed.trim_start_matches('-')))
}

pub fn prompt_text_to_reta_argv(text: &str) -> Vec<String> {
    let mut argv = text
        .split_whitespace()
        .map(|value| value.to_string())
        .collect::<Vec<_>>();
    if argv.first().map(String::as_str) != Some("reta") {
        argv.insert(0, "reta".to_string());
    }
    argv
}

pub fn current_prompt_token(text: &str) -> String {
    if text.chars().last().is_some_and(char::is_whitespace) {
        String::new()
    } else {
        text.split_whitespace().last().unwrap_or_default().to_string()
    }
}

pub fn continuum_m_prompt_language_completion_smoke() -> PromptLanguageCompletionReport {
    prompt_language_completion_for_text(
        "reta -language=english -spalten --kontinuum=m",
        &PromptLanguageCompletionPolicy::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_parameter_completion_suggests_language_switch() {
        let report = prompt_language_completion_for_text("reta -la", &PromptLanguageCompletionPolicy::default());
        assert_eq!(report.context.situation, PromptLanguageCompletionSituation::Parameter);
        assert!(report.candidates.iter().any(|candidate| candidate.text == "-language="));
    }

    #[test]
    fn language_value_completion_suggests_english_aliases() {
        let report = prompt_language_completion_for_text("reta -language=e", &PromptLanguageCompletionPolicy::default());
        assert_eq!(report.context.situation, PromptLanguageCompletionSituation::Value);
        assert!(report.candidates.iter().any(|candidate| candidate.text == "english"));
        assert!(report.candidates.iter().any(|candidate| candidate.text == "en"));
    }

    #[test]
    fn prompt_language_completion_sees_synced_744() {
        let report = continuum_m_prompt_language_completion_smoke();
        assert!(report.ready());
        assert!(report.language_coverage_ready);
        assert!(report.language_sync_ready);
        assert!(report.direct_744_available_for_selected_language);
        assert!(report.continuum_m_uses_synced_language_asset);
    }
}
