//! Prompt activation readiness for `rretaPrompt`.
//!
//! Stage 66 attached the prompt language guard to the prompt shadow commit.  This
//! module folds the local prompt witnesses into a single readiness report before
//! a prompt shadow plan may be treated as promotable behaviour.

use serde::{Deserialize, Serialize};

use crate::runtime_switch::{ArchitectureSwitchConfig, ArchitectureSwitchMode};
use crate::shadow_pipeline::{
    bootstrap_shadow_pipeline, evaluate_shadow_prompt_commit, ShadowPromptCommitDecision,
    ShadowPromptCommitPolicy, ShadowPromptInput,
    ShadowPromptLegacyCommand, ShadowPromptReport,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptActivationReadinessPolicy {
    pub require_prompt_commit: bool,
    pub require_prompt_language_guard_ready: bool,
    pub require_same_argv: bool,
    pub require_legacy_reta_kind: bool,
    pub require_gate_allowed_to_commit: bool,
    pub require_no_prompt_language_guard_failures: bool,
    pub allow_force_as_ready: bool,
    pub include_argv_preview: bool,
    pub max_argv_preview: usize,
}

impl Default for PromptActivationReadinessPolicy {
    fn default() -> Self {
        Self {
            require_prompt_commit: true,
            require_prompt_language_guard_ready: true,
            require_same_argv: true,
            require_legacy_reta_kind: true,
            require_gate_allowed_to_commit: true,
            require_no_prompt_language_guard_failures: true,
            allow_force_as_ready: false,
            include_argv_preview: true,
            max_argv_preview: 16,
        }
    }
}

impl PromptActivationReadinessPolicy {
    pub fn strict() -> Self {
        Self::default()
    }

    pub fn diagnostic() -> Self {
        Self {
            require_prompt_commit: false,
            require_prompt_language_guard_ready: false,
            require_same_argv: false,
            require_legacy_reta_kind: false,
            require_gate_allowed_to_commit: false,
            require_no_prompt_language_guard_failures: false,
            allow_force_as_ready: true,
            ..Self::default()
        }
    }

    pub fn from_cli_args<S: AsRef<str>>(args: &[S]) -> Self {
        let mut policy = Self::default();
        for arg in args {
            let arg = arg.as_ref();
            match arg {
                "--prompt-activation-readiness-strict" | "--prompt-readiness-strict" => {
                    policy = Self::strict();
                }
                "--prompt-activation-readiness-diagnostic" | "--prompt-readiness-diagnostic" => {
                    policy = Self::diagnostic();
                }
                "--prompt-activation-readiness-require-commit" | "--prompt-readiness-require-commit" => {
                    policy.require_prompt_commit = true;
                }
                "--prompt-activation-readiness-ignore-commit" | "--prompt-readiness-ignore-commit" => {
                    policy.require_prompt_commit = false;
                }
                "--prompt-activation-readiness-require-language-guard" | "--prompt-readiness-require-language-guard" => {
                    policy.require_prompt_language_guard_ready = true;
                    policy.require_no_prompt_language_guard_failures = true;
                }
                "--prompt-activation-readiness-ignore-language-guard" | "--prompt-readiness-ignore-language-guard" => {
                    policy.require_prompt_language_guard_ready = false;
                    policy.require_no_prompt_language_guard_failures = false;
                }
                "--prompt-activation-readiness-require-same-argv" | "--prompt-readiness-require-same-argv" => {
                    policy.require_same_argv = true;
                }
                "--prompt-activation-readiness-ignore-same-argv" | "--prompt-readiness-ignore-same-argv" => {
                    policy.require_same_argv = false;
                }
                "--prompt-activation-readiness-require-gate" | "--prompt-readiness-require-gate" => {
                    policy.require_gate_allowed_to_commit = true;
                }
                "--prompt-activation-readiness-ignore-gate" | "--prompt-readiness-ignore-gate" => {
                    policy.require_gate_allowed_to_commit = false;
                }
                "--prompt-activation-readiness-allow-force" | "--prompt-readiness-allow-force" => {
                    policy.allow_force_as_ready = true;
                }
                "--prompt-activation-readiness-no-force" | "--prompt-readiness-no-force" => {
                    policy.allow_force_as_ready = false;
                }
                "--prompt-activation-readiness-no-preview" | "--prompt-readiness-no-preview" => {
                    policy.include_argv_preview = false;
                }
                "--prompt-activation-readiness-include-preview" | "--prompt-readiness-include-preview" => {
                    policy.include_argv_preview = true;
                }
                _ if arg.starts_with("--prompt-activation-readiness-preview=")
                    || arg.starts_with("--prompt-readiness-preview=") =>
                {
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

    pub fn required_guard_names(&self) -> Vec<&'static str> {
        let mut names = Vec::new();
        if self.require_prompt_commit {
            names.push("prompt_shadow_commit_uses_shadow_plan");
        }
        if self.require_prompt_language_guard_ready {
            names.push("prompt_language_guard_ready");
        }
        if self.require_same_argv {
            names.push("prompt_argv_equal_to_legacy_argv");
        }
        if self.require_legacy_reta_kind {
            names.push("legacy_prompt_kind_is_reta");
        }
        if self.require_gate_allowed_to_commit {
            names.push("prompt_commit_gate_allowed");
        }
        if self.require_no_prompt_language_guard_failures {
            names.push("prompt_language_guard_has_no_failed_guards");
        }
        names
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptActivationReadinessCheck {
    pub name: String,
    pub required: bool,
    pub passed: bool,
    pub reason: String,
}

impl PromptActivationReadinessCheck {
    pub fn new(name: impl Into<String>, required: bool, passed: bool, reason: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            required,
            passed,
            reason: reason.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptActivationReadinessReport {
    pub class: String,
    pub status: String,
    pub ready_for_prompt_activation: bool,
    pub promotion_level: String,
    pub switch_mode: String,
    pub legacy_kind: String,
    pub gate_allowed_to_commit: bool,
    pub gate_reason: String,
    pub use_shadow_prompt_plan: bool,
    pub same_argv: bool,
    pub prompt_language_guard_ready: bool,
    pub prompt_language_guard_language: String,
    pub prompt_language_guard_compiled_language: String,
    pub prompt_language_guard_failed_guard_count: usize,
    pub required_check_count: usize,
    pub passed_required_check_count: usize,
    pub failed_required_checks: Vec<String>,
    pub checks: Vec<PromptActivationReadinessCheck>,
    pub planned_argv_preview: Vec<String>,
    pub legacy_argv_preview: Vec<String>,
    pub rollback_anchor: Option<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptActivationReadinessSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub smoke_status: String,
    pub smoke_ready: bool,
    pub smoke_failed_required_check_count: usize,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptActivationReadinessBundle {
    pub morphisms: Vec<String>,
    pub universal_property: String,
}

impl PromptActivationReadinessBundle {
    pub fn readiness_from_reports(
        &self,
        report: &ShadowPromptReport,
        legacy: &ShadowPromptLegacyCommand,
        commit: &ShadowPromptCommitDecision,
        policy: &PromptActivationReadinessPolicy,
    ) -> PromptActivationReadinessReport {
        prompt_activation_readiness_from_reports(report, legacy, commit, policy)
    }

    pub fn snapshot(&self) -> PromptActivationReadinessSnapshot {
        let smoke = continuum_m_prompt_activation_readiness_smoke();
        PromptActivationReadinessSnapshot {
            class: "PromptActivationReadinessSnapshot".to_string(),
            morphisms: self.morphisms.clone(),
            smoke_status: smoke.status,
            smoke_ready: smoke.ready_for_prompt_activation,
            smoke_failed_required_check_count: smoke.failed_required_checks.len(),
            universal_property: self.universal_property.clone(),
        }
    }
}

pub fn bootstrap_prompt_activation_readiness() -> PromptActivationReadinessBundle {
    PromptActivationReadinessBundle {
        morphisms: vec![
            "prompt_activation_readiness.fold_shadow_prompt_witnesses".to_string(),
            "prompt_activation_readiness.prompt_commit_guard".to_string(),
            "prompt_activation_readiness.prompt_language_guard_summary".to_string(),
            "prompt_activation_readiness.same_argv_guard".to_string(),
        ],
        universal_property:
            "prompt_activation_readiness_collects_prompt_shadow_commit_and_language_guard_before_visible_prompt_promotion".to_string(),
    }
}

pub fn prompt_activation_readiness_from_reports(
    report: &ShadowPromptReport,
    legacy: &ShadowPromptLegacyCommand,
    commit: &ShadowPromptCommitDecision,
    policy: &PromptActivationReadinessPolicy,
) -> PromptActivationReadinessReport {
    let mut checks = Vec::new();
    checks.push(PromptActivationReadinessCheck::new(
        "prompt_shadow_commit_uses_shadow_plan",
        policy.require_prompt_commit,
        commit.use_shadow_prompt_plan,
        if commit.use_shadow_prompt_plan { "shadow_prompt_plan_selected" } else { commit.reason.as_str() },
    ));
    checks.push(PromptActivationReadinessCheck::new(
        "prompt_language_guard_ready",
        policy.require_prompt_language_guard_ready,
        report.prompt_language_guard_ready && commit.prompt_language_guard_ready,
        if report.prompt_language_guard_ready { "prompt_language_guard_ready" } else { "prompt_language_guard_blocked" },
    ));
    checks.push(PromptActivationReadinessCheck::new(
        "prompt_argv_equal_to_legacy_argv",
        policy.require_same_argv,
        commit.same_argv,
        if commit.same_argv { "planned_argv_matches_legacy" } else { "planned_argv_differs_from_legacy" },
    ));
    checks.push(PromptActivationReadinessCheck::new(
        "legacy_prompt_kind_is_reta",
        policy.require_legacy_reta_kind,
        legacy.kind == "reta",
        legacy.kind.clone(),
    ));
    checks.push(PromptActivationReadinessCheck::new(
        "prompt_commit_gate_allowed",
        policy.require_gate_allowed_to_commit,
        commit.gate_allowed_to_commit,
        commit.gate_reason.clone(),
    ));
    checks.push(PromptActivationReadinessCheck::new(
        "prompt_language_guard_has_no_failed_guards",
        policy.require_no_prompt_language_guard_failures,
        report.prompt_language_guard.failed_guards.is_empty(),
        if report.prompt_language_guard.failed_guards.is_empty() {
            "no_failed_prompt_language_guards".to_string()
        } else {
            report.prompt_language_guard.failed_guards.join(",")
        },
    ));
    let force_ready = policy.allow_force_as_ready
        && commit.force_override
        && commit.gate_allowed_to_commit
        && report.prompt_language_guard_ready;
    checks.push(PromptActivationReadinessCheck::new(
        "force_override_is_explicit_and_language_safe",
        false,
        force_ready,
        if force_ready { "force_override_language_safe" } else { "force_override_not_used" },
    ));

    let required_check_count = checks.iter().filter(|check| check.required).count();
    let passed_required_check_count = checks
        .iter()
        .filter(|check| check.required && check.passed)
        .count();
    let failed_required_checks = checks
        .iter()
        .filter(|check| check.required && !check.passed)
        .map(|check| check.name.clone())
        .collect::<Vec<_>>();
    let ready_for_prompt_activation = failed_required_checks.is_empty() || force_ready;
    let status = if ready_for_prompt_activation { "ready" } else { "blocked" }.to_string();
    let promotion_level = if ready_for_prompt_activation && commit.use_shadow_prompt_plan {
        "prompt_shadow_commit_ready"
    } else if ready_for_prompt_activation {
        "prompt_diagnostic_ready"
    } else {
        "blocked"
    }
    .to_string();
    let preview_len = policy.max_argv_preview;
    let planned_argv_preview = if policy.include_argv_preview {
        report.planned_argv.iter().take(preview_len).cloned().collect()
    } else {
        Vec::new()
    };
    let legacy_argv_preview = if policy.include_argv_preview {
        legacy.argv.iter().take(preview_len).cloned().collect()
    } else {
        Vec::new()
    };

    PromptActivationReadinessReport {
        class: "PromptActivationReadinessReport".to_string(),
        status,
        ready_for_prompt_activation,
        promotion_level,
        switch_mode: commit.switch_mode.clone(),
        legacy_kind: legacy.kind.clone(),
        gate_allowed_to_commit: commit.gate_allowed_to_commit,
        gate_reason: commit.gate_reason.clone(),
        use_shadow_prompt_plan: commit.use_shadow_prompt_plan,
        same_argv: commit.same_argv,
        prompt_language_guard_ready: report.prompt_language_guard_ready,
        prompt_language_guard_language: report.prompt_language_guard_language.clone(),
        prompt_language_guard_compiled_language: report.prompt_language_guard_compiled_language.clone(),
        prompt_language_guard_failed_guard_count: report.prompt_language_guard_failed_guard_count,
        required_check_count,
        passed_required_check_count,
        failed_required_checks,
        checks,
        planned_argv_preview,
        legacy_argv_preview,
        rollback_anchor: commit.rollback_anchor.clone(),
        universal_property:
            "prompt_activation_readiness_ready_only_when_prompt_shadow_commit_language_guard_gate_and_argv_diagram_commute".to_string(),
    }
}

pub fn prompt_activation_readiness_for_text(
    text: &str,
    config: &ArchitectureSwitchConfig,
    policy: &PromptActivationReadinessPolicy,
) -> PromptActivationReadinessReport {
    let pipeline = bootstrap_shadow_pipeline();
    let report = pipeline.shadow_prompt(&ShadowPromptInput::new("prompt_activation_readiness", text), config);
    let legacy = ShadowPromptLegacyCommand::reta(report.planned_argv.clone());
    let commit_policy = ShadowPromptCommitPolicy::from_cli_args(&report.planned_argv);
    let commit = evaluate_shadow_prompt_commit(&report, &legacy, config, &commit_policy);
    prompt_activation_readiness_from_reports(&report, &legacy, &commit, policy)
}

pub fn prompt_activation_readiness_for_cli_args<S: AsRef<str>>(
    args: &[S],
    config: &ArchitectureSwitchConfig,
    policy: &PromptActivationReadinessPolicy,
) -> PromptActivationReadinessReport {
    let text = prompt_text_from_cli_args(args);
    prompt_activation_readiness_for_text(&text, config, policy)
}

pub fn prompt_text_from_cli_args<S: AsRef<str>>(args: &[S]) -> String {
    let values = args.iter().map(|arg| arg.as_ref()).collect::<Vec<_>>();
    if values.len() <= 1 {
        values.join(" ")
    } else {
        values[1..].join(" ")
    }
}

pub fn continuum_m_prompt_activation_readiness_smoke() -> PromptActivationReadinessReport {
    let config = ArchitectureSwitchConfig::default()
        .with_mode(ArchitectureSwitchMode::Commit, "prompt_activation_readiness_smoke")
        .allow("shadow_pipeline.prompt_commit")
        .allow("prompt_language_guard.shadow_prompt_commit_guard");
    prompt_activation_readiness_for_text(
        "reta -language=english -spalten --kontinuum=m",
        &config,
        &PromptActivationReadinessPolicy::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continuum_prompt_activation_readiness_is_ready_in_commit_mode() {
        let report = continuum_m_prompt_activation_readiness_smoke();
        assert!(report.ready_for_prompt_activation, "{:?}", report.failed_required_checks);
        assert_eq!(report.prompt_language_guard_language, "en");
        assert!(report.same_argv);
    }

    #[test]
    fn diagnostic_policy_does_not_require_commit_gate() {
        let config = ArchitectureSwitchConfig::default();
        let report = prompt_activation_readiness_for_text(
            "reta -language=english -spalten --kontinuum=m",
            &config,
            &PromptActivationReadinessPolicy::diagnostic(),
        );
        assert!(report.ready_for_prompt_activation);
        assert_eq!(report.status, "ready");
    }

    #[test]
    fn policy_from_cli_accepts_preview() {
        let policy = PromptActivationReadinessPolicy::from_cli_args(&[
            "--prompt-activation-readiness-diagnostic",
            "--prompt-activation-readiness-preview=3",
        ]);
        assert!(!policy.require_prompt_commit);
        assert_eq!(policy.max_argv_preview, 3);
    }
}
