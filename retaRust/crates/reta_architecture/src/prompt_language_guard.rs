//! Prompt language guard for `rretaPrompt`.
//!
//! Stage 64 added language-switch and language-value completions.  This module
//! turns those prompt-local witnesses into a guard: the prompt's compiled `reta`
//! argv must see the same language coverage/sync state as the TableView path,
//! especially for the Stage-55/62 `kontinuum=m -> 493,744` case.

use serde::{Deserialize, Serialize};

use crate::csv_catalog::csv_language_from_cli_args;
use crate::prompt_execution::{bootstrap_prompt_execution, plan_prompt_execution, PromptExecutionPlan};
use crate::prompt_language_completion::{
    prompt_language_completion_for_text, prompt_text_to_reta_argv, PromptLanguageCompletionPolicy,
    PromptLanguageCompletionReport,
};
use crate::prompt_preparation::{bootstrap_prompt_preparation, PreparedPromptOutput};
use crate::prompt_session::PromptTextState;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLanguageGuardPolicy {
    pub require_reta_prompt: bool,
    pub require_completion_ready: bool,
    pub require_language_coverage_ready: bool,
    pub require_language_sync_ready: bool,
    pub require_direct_744_for_continuum_m: bool,
    pub require_compiled_language_matches_prompt: bool,
    pub include_execution_plan_preview: bool,
    pub max_argv_preview: usize,
}

impl Default for PromptLanguageGuardPolicy {
    fn default() -> Self {
        Self {
            require_reta_prompt: true,
            require_completion_ready: true,
            require_language_coverage_ready: true,
            require_language_sync_ready: true,
            require_direct_744_for_continuum_m: true,
            require_compiled_language_matches_prompt: true,
            include_execution_plan_preview: true,
            max_argv_preview: 16,
        }
    }
}

impl PromptLanguageGuardPolicy {
    pub fn strict() -> Self {
        Self::default()
    }

    pub fn diagnostic() -> Self {
        Self {
            require_reta_prompt: false,
            require_completion_ready: false,
            require_language_coverage_ready: false,
            require_language_sync_ready: false,
            require_direct_744_for_continuum_m: false,
            require_compiled_language_matches_prompt: false,
            ..Self::default()
        }
    }

    pub fn from_cli_args<S: AsRef<str>>(args: &[S]) -> Self {
        let mut policy = Self::default();
        for arg in args {
            let arg = arg.as_ref();
            match arg {
                "--prompt-language-guard-strict" | "--prompt-guard-strict" => {
                    policy = Self::strict();
                }
                "--prompt-language-guard-diagnostic" | "--prompt-guard-diagnostic" => {
                    policy = Self::diagnostic();
                }
                "--prompt-language-guard-ignore-sync" | "--prompt-guard-ignore-sync" => {
                    policy.require_language_sync_ready = false;
                }
                "--prompt-language-guard-require-sync" | "--prompt-guard-require-sync" => {
                    policy.require_language_sync_ready = true;
                }
                "--prompt-language-guard-ignore-coverage" | "--prompt-guard-ignore-coverage" => {
                    policy.require_language_coverage_ready = false;
                }
                "--prompt-language-guard-require-coverage" | "--prompt-guard-require-coverage" => {
                    policy.require_language_coverage_ready = true;
                }
                "--prompt-language-guard-ignore-direct-744" | "--prompt-guard-ignore-direct-744" => {
                    policy.require_direct_744_for_continuum_m = false;
                }
                "--prompt-language-guard-require-direct-744" | "--prompt-guard-require-direct-744" => {
                    policy.require_direct_744_for_continuum_m = true;
                }
                "--prompt-language-guard-ignore-reta" | "--prompt-guard-ignore-reta" => {
                    policy.require_reta_prompt = false;
                }
                "--prompt-language-guard-require-reta" | "--prompt-guard-require-reta" => {
                    policy.require_reta_prompt = true;
                }
                _ if arg.starts_with("--prompt-language-guard-preview=") || arg.starts_with("--prompt-guard-preview=") => {
                    if let Some((_, value)) = arg.split_once('=') {
                        if let Ok(parsed) = value.parse::<usize>() {
                            policy.max_argv_preview = parsed;
                        }
                    }
                }
                _ => {}
            }
        }
        policy
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLanguageGuardReport {
    pub class: String,
    pub prompt_text: String,
    pub prompt_argv: Vec<String>,
    pub execution_argv_preview: Vec<String>,
    pub execution_argv_count: usize,
    pub is_reta_prompt: bool,
    pub contains_continuum_m: bool,
    pub prompt_language: String,
    pub compiled_language: String,
    pub language_completion: PromptLanguageCompletionReport,
    pub completion_ready: bool,
    pub language_coverage_ready: bool,
    pub language_sync_ready: bool,
    pub direct_744_available_for_prompt_language: bool,
    pub compiled_language_matches_prompt: bool,
    pub status: String,
    pub failed_guards: Vec<String>,
    pub universal_property: String,
}

impl PromptLanguageGuardReport {
    pub fn ready(&self) -> bool {
        self.status == "ready"
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLanguageGuardSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub smoke_status: String,
    pub smoke_failed_guard_count: usize,
    pub smoke_prompt_language: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptLanguageGuardBundle {
    pub morphisms: Vec<String>,
    pub universal_property: String,
}

impl PromptLanguageGuardBundle {
    pub fn guard_prompt_language(&self, text: &str, policy: &PromptLanguageGuardPolicy) -> PromptLanguageGuardReport {
        prompt_language_guard_for_text(text, policy)
    }

    pub fn snapshot(&self) -> PromptLanguageGuardSnapshot {
        let smoke = continuum_m_prompt_language_guard_smoke();
        PromptLanguageGuardSnapshot {
            class: "PromptLanguageGuardSnapshot".to_string(),
            morphisms: self.morphisms.clone(),
            smoke_status: smoke.status,
            smoke_failed_guard_count: smoke.failed_guards.len(),
            smoke_prompt_language: smoke.prompt_language,
            universal_property: self.universal_property.clone(),
        }
    }
}

pub fn bootstrap_prompt_language_guard() -> PromptLanguageGuardBundle {
    PromptLanguageGuardBundle {
        morphisms: vec![
            "prompt_language_guard.prompt_to_reta_argv".to_string(),
            "prompt_language_guard.language_completion_ready".to_string(),
            "prompt_language_guard.language_coverage_ready".to_string(),
            "prompt_language_guard.language_sync_ready".to_string(),
            "prompt_language_guard.direct_744_prompt_guard".to_string(),
        ],
        universal_property: "prompt_language_guard_must_commute_with_table_view_language_cover_before_prompt_activation".to_string(),
    }
}

pub fn prompt_language_guard_for_text(
    text: &str,
    policy: &PromptLanguageGuardPolicy,
) -> PromptLanguageGuardReport {
    let prompt_argv = prompt_text_to_reta_argv(text);
    let language_completion = prompt_language_completion_for_text(
        text,
        &PromptLanguageCompletionPolicy::default(),
    );
    let prepared = prepare_prompt_for_guard(text);
    let text_state = PromptTextState::new(text);
    let execution_bundle = bootstrap_prompt_execution();
    let execution_plan = plan_prompt_execution(&execution_bundle, &prepared, &text_state);
    prompt_language_guard_from_parts(text, prompt_argv, execution_plan, language_completion, policy)
}

pub fn prompt_language_guard_from_argv<S: AsRef<str>>(
    args: &[S],
    policy: &PromptLanguageGuardPolicy,
) -> PromptLanguageGuardReport {
    let text = args.iter().map(|arg| arg.as_ref()).collect::<Vec<_>>().join(" ");
    prompt_language_guard_for_text(&text, policy)
}

fn prompt_language_guard_from_parts(
    text: &str,
    prompt_argv: Vec<String>,
    execution_plan: PromptExecutionPlan,
    language_completion: PromptLanguageCompletionReport,
    policy: &PromptLanguageGuardPolicy,
) -> PromptLanguageGuardReport {
    let is_reta_prompt = text.split_whitespace().next() == Some("reta");
    let contains_continuum_m = prompt_argv.iter().any(|arg| arg.contains("kontinuum=m"));
    let prompt_language = csv_language_from_cli_args(&prompt_argv).canonical().to_string();
    let execution_argv = if execution_plan.reta_argv.is_empty() {
        prompt_argv.clone()
    } else {
        execution_plan.reta_argv.clone()
    };
    let compiled_language = csv_language_from_cli_args(&execution_argv).canonical().to_string();
    let compiled_language_matches_prompt = prompt_language == compiled_language;
    let mut failed_guards = Vec::new();
    if policy.require_reta_prompt && !is_reta_prompt {
        failed_guards.push("prompt_is_not_a_reta_command".to_string());
    }
    if policy.require_completion_ready && !language_completion.ready() {
        failed_guards.push("prompt_language_completion_not_ready".to_string());
    }
    if policy.require_language_coverage_ready && !language_completion.language_coverage_ready {
        failed_guards.push("prompt_language_coverage_not_ready".to_string());
    }
    if policy.require_language_sync_ready && !language_completion.language_sync_ready {
        failed_guards.push("prompt_language_sync_not_ready".to_string());
    }
    if policy.require_direct_744_for_continuum_m
        && contains_continuum_m
        && !language_completion.direct_744_available_for_selected_language
    {
        failed_guards.push("prompt_language_direct_744_not_available".to_string());
    }
    if policy.require_compiled_language_matches_prompt && !compiled_language_matches_prompt {
        failed_guards.push("compiled_language_differs_from_prompt_language".to_string());
    }
    let status = if failed_guards.is_empty() { "ready" } else { "blocked" }.to_string();
    let preview_len = execution_argv.len().min(policy.max_argv_preview);
    PromptLanguageGuardReport {
        class: "PromptLanguageGuardReport".to_string(),
        prompt_text: text.to_string(),
        prompt_argv,
        execution_argv_preview: execution_argv.into_iter().take(preview_len).collect(),
        execution_argv_count: execution_plan.reta_argv.len(),
        is_reta_prompt,
        contains_continuum_m,
        prompt_language,
        compiled_language,
        completion_ready: language_completion.ready(),
        language_coverage_ready: language_completion.language_coverage_ready,
        language_sync_ready: language_completion.language_sync_ready,
        direct_744_available_for_prompt_language: language_completion.direct_744_available_for_selected_language,
        compiled_language_matches_prompt,
        language_completion,
        status,
        failed_guards,
        universal_property: "prompt_language_guard_glues_prompt_completion_prompt_execution_and_table_view_language_cover_for_the_same_reta_argv".to_string(),
    }
}

fn prepare_prompt_for_guard(text: &str) -> PreparedPromptOutput {
    bootstrap_prompt_preparation().prepare_large_output(
        "",
        crate::prompt_language::PromptModus::Normal,
        crate::prompt_language::PromptModus::Normal,
        crate::prompt_language::PromptModus::Normal,
        text,
        &[],
    )
}

pub fn continuum_m_prompt_language_guard_smoke() -> PromptLanguageGuardReport {
    prompt_language_guard_for_text(
        "reta -language=english -spalten --kontinuum=m",
        &PromptLanguageGuardPolicy::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_language_guard_accepts_synced_continuum_m() {
        let report = continuum_m_prompt_language_guard_smoke();
        assert!(report.ready(), "{:?}", report.failed_guards);
        assert_eq!(report.prompt_language, "en");
        assert!(report.direct_744_available_for_prompt_language);
        assert!(report.compiled_language_matches_prompt);
    }

    #[test]
    fn prompt_language_guard_blocks_non_reta_prompt_by_default() {
        let report = prompt_language_guard_for_text("help -language=english", &PromptLanguageGuardPolicy::default());
        assert!(!report.ready());
        assert!(report.failed_guards.contains(&"prompt_is_not_a_reta_command".to_string()));
    }

    #[test]
    fn prompt_language_guard_policy_from_cli_can_be_diagnostic() {
        let policy = PromptLanguageGuardPolicy::from_cli_args(&["--prompt-language-guard-diagnostic"]);
        assert!(!policy.require_reta_prompt);
        assert!(!policy.require_language_sync_ready);
    }
}
