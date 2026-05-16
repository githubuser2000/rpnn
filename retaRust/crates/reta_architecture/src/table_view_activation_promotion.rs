//! Promotion decision for guarded TableView activation.
//!
//! `table_view_activation_readiness` folds the local witnesses into one report.
//! This module is the next, narrower morphism: it decides whether that ready
//! report is eligible to become a *default visible* Rust TableView activation.
//! It still does not relax commit safety.  Raw equality, commit gates and direct
//! CSV-cell identity remain the hard guards; semantic equality is kept as a
//! diagnostic witness.

use serde::{Deserialize, Serialize};

use crate::runtime_switch::{ArchitectureSwitchConfig, ArchitectureSwitchMode, SwitchGateDecision};
use crate::table_view_activation_readiness::{
    activation_readiness_for_cli_args, TableViewActivationReadinessPolicy,
    TableViewActivationReadinessReport,
};

pub const PROMOTION_MORPHISM: &str = "table_view_activation_promotion.default_visible_source";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationPromotionPolicy {
    pub require_readiness_ready: bool,
    pub require_commit_gate: bool,
    pub require_commit_mode: bool,
    pub require_table_view_source: bool,
    pub require_raw_equal: bool,
    pub require_virtual_direct_identity: bool,
    pub require_language_parity_ready: bool,
    pub require_language_coverage_ready: bool,
    pub allow_force_when_not_ready: bool,
    pub include_selected_lines: bool,
    pub preview_limit: usize,
}

impl Default for TableViewActivationPromotionPolicy {
    fn default() -> Self {
        Self {
            require_readiness_ready: true,
            require_commit_gate: true,
            require_commit_mode: true,
            require_table_view_source: true,
            require_raw_equal: true,
            require_virtual_direct_identity: true,
            require_language_parity_ready: true,
            require_language_coverage_ready: true,
            allow_force_when_not_ready: false,
            include_selected_lines: false,
            preview_limit: 8,
        }
    }
}

impl TableViewActivationPromotionPolicy {
    pub fn strict() -> Self {
        Self::default()
    }

    pub fn diagnostic() -> Self {
        Self {
            require_readiness_ready: false,
            require_commit_gate: false,
            require_commit_mode: false,
            require_table_view_source: false,
            require_raw_equal: false,
            require_virtual_direct_identity: false,
            require_language_parity_ready: false,
            require_language_coverage_ready: false,
            allow_force_when_not_ready: false,
            include_selected_lines: false,
            preview_limit: 8,
        }
    }

    pub fn with_selected_lines(mut self, include: bool) -> Self {
        self.include_selected_lines = include;
        self
    }

    pub fn with_preview_limit(mut self, limit: usize) -> Self {
        self.preview_limit = limit;
        self
    }

    pub fn from_cli_args(args: &[String], base: &Self) -> (Self, bool) {
        let mut policy = base.clone();
        let mut recognized = false;
        for arg in args {
            match arg.as_str() {
                "--activation-promotion-strict" | "--promotion-strict" => {
                    policy = Self::strict();
                    recognized = true;
                }
                "--activation-promotion-diagnostic" | "--promotion-diagnostic" => {
                    policy = Self::diagnostic();
                    recognized = true;
                }
                "--activation-promotion-allow-force" | "--promotion-allow-force" => {
                    policy.allow_force_when_not_ready = true;
                    recognized = true;
                }
                "--activation-promotion-no-force" | "--promotion-no-force" => {
                    policy.allow_force_when_not_ready = false;
                    recognized = true;
                }
                "--activation-promotion-require-commit-mode"
                | "--promotion-require-commit-mode" => {
                    policy.require_commit_mode = true;
                    recognized = true;
                }
                "--activation-promotion-ignore-commit-mode" | "--promotion-ignore-commit-mode" => {
                    policy.require_commit_mode = false;
                    recognized = true;
                }
                "--activation-promotion-require-readiness" | "--promotion-require-readiness" => {
                    policy.require_readiness_ready = true;
                    recognized = true;
                }
                "--activation-promotion-ignore-readiness" | "--promotion-ignore-readiness" => {
                    policy.require_readiness_ready = false;
                    recognized = true;
                }
                "--activation-promotion-require-language-parity"
                | "--promotion-require-language-parity" => {
                    policy.require_language_parity_ready = true;
                    recognized = true;
                }
                "--activation-promotion-ignore-language-parity"
                | "--promotion-ignore-language-parity" => {
                    policy.require_language_parity_ready = false;
                    recognized = true;
                }
                "--activation-promotion-require-language-coverage"
                | "--promotion-require-language-coverage" => {
                    policy.require_language_coverage_ready = true;
                    recognized = true;
                }
                "--activation-promotion-ignore-language-coverage"
                | "--promotion-ignore-language-coverage" => {
                    policy.require_language_coverage_ready = false;
                    recognized = true;
                }
                "--activation-promotion-no-selected-lines" | "--promotion-no-selected-lines" => {
                    policy.include_selected_lines = false;
                    recognized = true;
                }
                "--activation-promotion-include-selected-lines"
                | "--promotion-include-selected-lines" => {
                    policy.include_selected_lines = true;
                    recognized = true;
                }
                _ => {
                    if let Some(value) = arg
                        .strip_prefix("--activation-promotion-preview=")
                        .or_else(|| arg.strip_prefix("--promotion-preview="))
                    {
                        if let Ok(limit) = value.parse::<usize>() {
                            policy.preview_limit = limit;
                            recognized = true;
                        }
                    }
                }
            }
        }
        (policy, recognized)
    }

    pub fn required_guard_names(&self) -> Vec<&'static str> {
        let mut guards = Vec::new();
        if self.require_readiness_ready {
            guards.push("activation_readiness_ready");
        }
        if self.require_commit_gate {
            guards.push("promotion_commit_gate_allowed");
        }
        if self.require_commit_mode {
            guards.push("runtime_mode_can_commit");
        }
        if self.require_table_view_source {
            guards.push("selected_source_is_table_view_output");
        }
        if self.require_raw_equal {
            guards.push("raw_line_diff_equal");
        }
        if self.require_virtual_direct_identity {
            guards.push("virtual_direct_cells_equal");
        }
        if self.require_language_parity_ready {
            guards.push("language_parity_ready");
        }
        if self.require_language_coverage_ready {
            guards.push("language_coverage_ready");
        }
        guards
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationPromotionCheck {
    pub name: String,
    pub required: bool,
    pub passed: bool,
    pub value: String,
    pub reason: String,
}

impl TableViewActivationPromotionCheck {
    pub fn new(
        name: impl Into<String>,
        required: bool,
        passed: bool,
        value: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            required,
            passed,
            value: value.into(),
            reason: reason.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationPromotionReport {
    pub class: String,
    pub morphism: String,
    pub status: String,
    pub promotion_level: String,
    pub promotion_action: String,
    pub ready_for_default_promotion: bool,
    pub visible_output_source: String,
    pub gate: SwitchGateDecision,
    pub switch_mode: String,
    pub readiness_status: String,
    pub readiness_ready: bool,
    pub raw_equal: bool,
    pub semantic_equal: bool,
    pub virtual_direct_cells_equal: bool,
    pub language_parity_ready: bool,
    pub language_requested_language: String,
    pub language_effective_asset_name: String,
    pub language_fallback_applied: bool,
    pub language_coverage_ready: bool,
    pub language_coverage_status: String,
    pub language_coverage_stale_language_count: usize,
    pub language_coverage_languages_missing_744: Vec<String>,
    pub selected_source: String,
    pub selected_line_count: usize,
    pub selected_lines_checksum: u64,
    pub selected_lines_preview: Vec<String>,
    pub selected_lines: Vec<String>,
    pub required_check_count: usize,
    pub passed_required_check_count: usize,
    pub failed_required_checks: Vec<String>,
    pub diagnostic_check_count: usize,
    pub checks: Vec<TableViewActivationPromotionCheck>,
    pub readiness_failed_required_checks: Vec<String>,
    pub readiness_required_check_count: usize,
    pub readiness_passed_required_check_count: usize,
    pub rollback_anchors: Vec<String>,
    pub universal_property: String,
}

impl TableViewActivationPromotionReport {
    pub fn is_ready(&self) -> bool {
        self.ready_for_default_promotion
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationPromotionSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub required_guards: Vec<String>,
    pub diagnostic_guards: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationPromotionBundle;

impl TableViewActivationPromotionBundle {
    pub fn snapshot(&self) -> TableViewActivationPromotionSnapshot {
        TableViewActivationPromotionSnapshot {
            class: "TableViewActivationPromotionSnapshot".to_string(),
            morphisms: vec![
                "table_view_activation_promotion.default_visible_source".to_string(),
                "table_view_activation_promotion.policy_from_cli".to_string(),
                "table_view_activation_promotion.guard_summary".to_string(),
                "table_view_activation_promotion.rollback_to_legacy".to_string(),
            ],
            required_guards: TableViewActivationPromotionPolicy::default()
                .required_guard_names()
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            diagnostic_guards: vec![
                "semantic_rows_equal".to_string(),
                "readiness_failed_required_checks".to_string(),
                "rollback_anchor_available".to_string(),
            ],
            universal_property:
                "readiness_witnesses_glue_to_one_guarded_default_promotion_decision".to_string(),
        }
    }

    pub fn promotion_from_readiness(
        &self,
        readiness: &TableViewActivationReadinessReport,
        config: &ArchitectureSwitchConfig,
        policy: &TableViewActivationPromotionPolicy,
    ) -> TableViewActivationPromotionReport {
        activation_promotion_from_readiness(readiness, config, policy)
    }
}

pub fn bootstrap_table_view_activation_promotion() -> TableViewActivationPromotionBundle {
    TableViewActivationPromotionBundle
}

pub fn activation_promotion_from_readiness(
    readiness: &TableViewActivationReadinessReport,
    config: &ArchitectureSwitchConfig,
    policy: &TableViewActivationPromotionPolicy,
) -> TableViewActivationPromotionReport {
    let gate = config.gate_for_morphism(PROMOTION_MORPHISM);
    let mode_can_commit = config.mode.can_commit();
    let force_override =
        config.mode == ArchitectureSwitchMode::Force && policy.allow_force_when_not_ready;
    let selected_source_is_view = readiness.selected_source == "table_view_output"
        || readiness.transaction_replaces_visible_output;

    let mut checks = Vec::new();
    checks.push(TableViewActivationPromotionCheck::new(
        "activation_readiness_ready",
        policy.require_readiness_ready,
        readiness.ready_for_visible_activation || force_override,
        readiness.status.clone(),
        "folded activation readiness must be ready before default promotion",
    ));
    checks.push(TableViewActivationPromotionCheck::new(
        "promotion_commit_gate_allowed",
        policy.require_commit_gate,
        gate.allowed_to_commit,
        gate.reason.clone(),
        "runtime gate must explicitly allow the promotion morphism",
    ));
    checks.push(TableViewActivationPromotionCheck::new(
        "runtime_mode_can_commit",
        policy.require_commit_mode,
        mode_can_commit,
        config.mode.canonical(),
        "runtime mode must be commit or force for default visible promotion",
    ));
    checks.push(TableViewActivationPromotionCheck::new(
        "selected_source_is_table_view_output",
        policy.require_table_view_source,
        selected_source_is_view,
        readiness.selected_source.clone(),
        "activation transaction must select the Rust TableView output source",
    ));
    checks.push(TableViewActivationPromotionCheck::new(
        "raw_line_diff_equal",
        policy.require_raw_equal,
        readiness.raw_equal,
        readiness.raw_equal.to_string(),
        "visible default promotion still requires raw line parity",
    ));
    checks.push(TableViewActivationPromotionCheck::new(
        "virtual_direct_cells_equal",
        policy.require_virtual_direct_identity,
        readiness.virtual_direct_cells_equal,
        readiness.virtual_direct_cells_equal.to_string(),
        "virtual column policies must be identity on direct CSV-backed cells",
    ));
    checks.push(TableViewActivationPromotionCheck::new(
        "language_parity_ready",
        policy.require_language_parity_ready,
        readiness.language_parity_ready,
        format!(
            "requested_language={} effective_asset={} fallback_applied={} failed_guards={:?}",
            readiness.language_requested_language,
            readiness.language_effective_asset_name,
            readiness.language_fallback_applied,
            readiness.language_failed_guards
        ),
        "localized materialization may be promoted only when all requested direct columns are safely covered",
    ));
    checks.push(TableViewActivationPromotionCheck::new(
        "language_coverage_ready",
        policy.require_language_coverage_ready,
        readiness.language_coverage_ready,
        format!(
            "status={} stale_languages={} missing_744={:?} failed_guards={:?}",
            readiness.language_coverage_status,
            readiness.language_coverage_stale_language_count,
            readiness.language_coverage_languages_missing_744,
            readiness.language_coverage_failed_guards
        ),
        "localized materialization may become default only when requested direct columns are covered by the effective language asset or fallback",
    ));
    checks.push(TableViewActivationPromotionCheck::new(
        "semantic_rows_equal",
        false,
        readiness.semantic_equal,
        readiness.semantic_equal.to_string(),
        "semantic equality is diagnostic and does not replace raw parity",
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
    let ready_for_default_promotion = failed_required_checks.is_empty();

    let promotion_level = if ready_for_default_promotion {
        "default-visible-ready"
    } else if readiness.ready_for_visible_activation {
        "readiness-ready-but-promotion-blocked"
    } else if readiness.semantic_equal {
        "semantic-shadow-candidate"
    } else {
        "shadow-only"
    };
    let promotion_action = if ready_for_default_promotion {
        "promote_table_view_output"
    } else {
        "keep_legacy_visible_output"
    };
    let visible_output_source = if ready_for_default_promotion {
        "table_view_output"
    } else {
        "legacy"
    };

    let selected_lines_preview = readiness
        .selected_lines
        .iter()
        .take(policy.preview_limit)
        .cloned()
        .collect::<Vec<_>>();
    let selected_lines = if policy.include_selected_lines {
        readiness.selected_lines.clone()
    } else {
        Vec::new()
    };

    TableViewActivationPromotionReport {
        class: "TableViewActivationPromotionReport".to_string(),
        morphism: PROMOTION_MORPHISM.to_string(),
        status: if ready_for_default_promotion {
            "ready"
        } else {
            "blocked"
        }
        .to_string(),
        promotion_level: promotion_level.to_string(),
        promotion_action: promotion_action.to_string(),
        ready_for_default_promotion,
        visible_output_source: visible_output_source.to_string(),
        gate,
        switch_mode: config.mode.canonical().to_string(),
        readiness_status: readiness.status.clone(),
        readiness_ready: readiness.ready_for_visible_activation,
        raw_equal: readiness.raw_equal,
        semantic_equal: readiness.semantic_equal,
        virtual_direct_cells_equal: readiness.virtual_direct_cells_equal,
        language_parity_ready: readiness.language_parity_ready,
        language_requested_language: readiness.language_requested_language.clone(),
        language_effective_asset_name: readiness.language_effective_asset_name.clone(),
        language_fallback_applied: readiness.language_fallback_applied,
        language_coverage_ready: readiness.language_coverage_ready,
        language_coverage_status: readiness.language_coverage_status.clone(),
        language_coverage_stale_language_count: readiness.language_coverage_stale_language_count,
        language_coverage_languages_missing_744: readiness.language_coverage_languages_missing_744.clone(),
        selected_source: readiness.selected_source.clone(),
        selected_line_count: readiness.selected_line_count,
        selected_lines_checksum: readiness.selected_lines_checksum,
        selected_lines_preview,
        selected_lines,
        required_check_count,
        passed_required_check_count,
        failed_required_checks,
        diagnostic_check_count: checks.iter().filter(|check| !check.required).count(),
        checks,
        readiness_failed_required_checks: readiness.failed_required_checks.clone(),
        readiness_required_check_count: readiness.required_check_count,
        readiness_passed_required_check_count: readiness.passed_required_check_count,
        rollback_anchors: readiness.rollback_anchors.clone(),
        universal_property: "readiness_witnesses_glue_to_one_guarded_default_promotion_decision"
            .to_string(),
    }
}

pub fn activation_promotion_for_cli_args(
    args: &[String],
    legacy_lines: &[String],
    config: &ArchitectureSwitchConfig,
    policy: &TableViewActivationPromotionPolicy,
) -> TableViewActivationPromotionReport {
    let (readiness_policy, _) = TableViewActivationReadinessPolicy::from_cli_args(
        args,
        &TableViewActivationReadinessPolicy::default(),
    );
    let readiness =
        activation_readiness_for_cli_args(args, legacy_lines, config, &readiness_policy);
    activation_promotion_from_readiness(&readiness, config, policy)
}

pub fn activation_promotion_checks_from_readiness(
    readiness: &TableViewActivationReadinessReport,
    config: &ArchitectureSwitchConfig,
    policy: &TableViewActivationPromotionPolicy,
) -> Vec<TableViewActivationPromotionCheck> {
    activation_promotion_from_readiness(readiness, config, policy).checks
}

pub fn continuum_m_activation_promotion_smoke() -> TableViewActivationPromotionReport {
    let args = vec![
        "reta".to_string(),
        "--reta-arch=commit".to_string(),
        "-zeilen".to_string(),
        "--vorhervonausschnitt=1-1".to_string(),
        "-spalten".to_string(),
        "--kontinuum=m".to_string(),
        "--breite=0".to_string(),
    ];
    let legacy_lines = vec!["M Kontinuum (dreizehn)".to_string()];
    let (_, config) = crate::runtime_switch::extract_architecture_switch_from_argv(&args, None);
    activation_promotion_for_cli_args(
        &args,
        &legacy_lines,
        &config,
        &TableViewActivationPromotionPolicy::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_readiness() -> TableViewActivationReadinessReport {
        TableViewActivationReadinessReport {
            class: "TableViewActivationReadinessReport".to_string(),
            morphism: "table_view_activation_readiness.fold_local_witnesses".to_string(),
            status: "ready".to_string(),
            ready_for_visible_activation: true,
            promotion_level: "candidate".to_string(),
            selected_source: "table_view_output".to_string(),
            selected_line_count: 1,
            selected_lines_checksum: 42,
            selected_lines_preview: vec!["line".to_string()],
            selected_lines: vec!["line".to_string()],
            switch_mode: "commit".to_string(),
            raw_equal: true,
            semantic_equal: true,
            virtual_direct_cells_equal: true,
            virtual_added_column_count: 1,
            language_parity_ready: true,
            language_requested_language: "base".to_string(),
            language_effective_asset_name: "religion.csv".to_string(),
            language_fallback_applied: false,
            language_failed_guards: Vec::new(),
            language_coverage_ready: true,
            language_coverage_status: "ready".to_string(),
            language_coverage_stale_language_count: 0,
            language_coverage_languages_missing_744: Vec::new(),
            language_coverage_failed_guards: Vec::new(),
            commit_decision: true,
            audit_safe: true,
            transaction_safe: true,
            transaction_replaces_visible_output: true,
            journal_replayable: true,
            replay_safe: true,
            ledger_ready: true,
            store_ready: true,
            persistence_ready: true,
            recovery_enabled: false,
            recovery_ready: false,
            recovery_replays_visible_output: false,
            required_check_count: 10,
            passed_required_check_count: 10,
            failed_required_checks: Vec::new(),
            diagnostic_check_count: 2,
            checks: vec![
                crate::table_view_activation_readiness::TableViewActivationReadinessCheck::new(
                    "synthetic",
                    true,
                    true,
                    "ok",
                    "synthetic readiness",
                ),
            ],
            rollback_anchors: vec!["legacy".to_string()],
            universal_property: "synthetic".to_string(),
        }
    }

    #[test]
    fn promotion_requires_commit_gate() {
        let readiness = synthetic_readiness();
        let config = ArchitectureSwitchConfig::default();
        let report = activation_promotion_from_readiness(
            &readiness,
            &config,
            &TableViewActivationPromotionPolicy::default(),
        );
        assert!(!report.ready_for_default_promotion);
        assert!(report
            .failed_required_checks
            .contains(&"promotion_commit_gate_allowed".to_string()));
    }

    #[test]
    fn promotion_can_be_ready_in_commit_mode() {
        let readiness = synthetic_readiness();
        let config =
            ArchitectureSwitchConfig::default().with_mode(ArchitectureSwitchMode::Commit, "test");
        let report = activation_promotion_from_readiness(
            &readiness,
            &config,
            &TableViewActivationPromotionPolicy::default(),
        );
        assert!(report.ready_for_default_promotion);
        assert_eq!(report.visible_output_source, "table_view_output");
    }

    #[test]
    fn promotion_policy_from_cli_parses_preview() {
        let args = vec![
            "reta".to_string(),
            "--activation-promotion-diagnostic".to_string(),
            "--activation-promotion-preview=2".to_string(),
        ];
        let (policy, recognized) = TableViewActivationPromotionPolicy::from_cli_args(
            &args,
            &TableViewActivationPromotionPolicy::default(),
        );
        assert!(recognized);
        assert_eq!(policy.preview_limit, 2);
        assert!(!policy.require_commit_gate);
    }
}
