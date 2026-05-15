//! Guarded runtime activation switches for the Rust architecture path.
//!
//! The previous stages created many typed Rust morphisms, but deliberately did
//! not route visible `reta`/`retaPrompt` behaviour through them.  This module is
//! the missing control surface: it parses explicit activation flags, strips
//! internal flags before legacy-compatible execution, and turns each candidate
//! morphism into a small gate decision with rollback metadata.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

const OFF_VALUES: &[&str] = &["", "0", "off", "false", "no", "legacy"];
const OBSERVE_VALUES: &[&str] = &["observe", "trace", "audit", "plan"];
const DRY_RUN_VALUES: &[&str] = &["dry-run", "dryrun", "compare", "shadow"];
const ADAPTER_VALUES: &[&str] = &["adapter", "adapters", "partial", "staged"];
const COMMIT_VALUES: &[&str] = &["commit", "activate", "on", "true", "1"];
const FORCE_VALUES: &[&str] = &["force", "unsafe-force"];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum ArchitectureSwitchMode {
    /// Do not use the Rust architecture path for behaviour; keep legacy parity path.
    #[default]
    Legacy,
    /// Build architecture plans/snapshots only.  No behavioural routing.
    Observe,
    /// Execute architecture morphisms as shadow/dry-run candidates.
    DryRun,
    /// Allow selected adapter morphisms to provide intermediate values.
    Adapter,
    /// Allow guarded commit of explicitly selected morphisms.
    Commit,
    /// Developer-only override; still represented so callers can audit it.
    Force,
}

impl ArchitectureSwitchMode {
    pub fn canonical(self) -> &'static str {
        match self {
            Self::Legacy => "legacy",
            Self::Observe => "observe",
            Self::DryRun => "dry-run",
            Self::Adapter => "adapter",
            Self::Commit => "commit",
            Self::Force => "force",
        }
    }

    pub fn from_value(value: &str) -> Self {
        let normalized = value.trim().to_ascii_lowercase();
        let raw = normalized.as_str();
        if OFF_VALUES.contains(&raw) {
            Self::Legacy
        } else if OBSERVE_VALUES.contains(&raw) {
            Self::Observe
        } else if DRY_RUN_VALUES.contains(&raw) {
            Self::DryRun
        } else if ADAPTER_VALUES.contains(&raw) {
            Self::Adapter
        } else if COMMIT_VALUES.contains(&raw) {
            Self::Commit
        } else if FORCE_VALUES.contains(&raw) {
            Self::Force
        } else {
            Self::Observe
        }
    }

    pub fn behaviour_may_change(self) -> bool {
        matches!(self, Self::Adapter | Self::Commit | Self::Force)
    }

    pub fn should_shadow_execute(self) -> bool {
        matches!(
            self,
            Self::DryRun | Self::Adapter | Self::Commit | Self::Force
        )
    }

    pub fn can_commit(self) -> bool {
        matches!(self, Self::Commit | Self::Force)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureSwitchConfig {
    pub mode: ArchitectureSwitchMode,
    pub source: String,
    pub trace: bool,
    pub compare_with_python_arch: bool,
    pub compare_with_python_legacy: bool,
    pub rollback_anchor: Option<String>,
    pub allowed_morphisms: BTreeSet<String>,
    pub blocked_morphisms: BTreeSet<String>,
}

impl Default for ArchitectureSwitchConfig {
    fn default() -> Self {
        Self {
            mode: ArchitectureSwitchMode::Legacy,
            source: "defaults".to_string(),
            trace: false,
            compare_with_python_arch: true,
            compare_with_python_legacy: true,
            rollback_anchor: None,
            allowed_morphisms: BTreeSet::new(),
            blocked_morphisms: BTreeSet::new(),
        }
    }
}

impl ArchitectureSwitchConfig {
    pub fn from_environment() -> Self {
        let mut config = Self::default();
        if let Ok(value) = std::env::var("RETA_ARCHITECTURE_MODE")
            .or_else(|_| std::env::var("RETA_ARCH_MODE"))
            .or_else(|_| std::env::var("RETA_ARCH"))
        {
            config.mode = ArchitectureSwitchMode::from_value(&value);
            config.source = "environment".to_string();
        }
        if let Ok(value) = std::env::var("RETA_ARCH_TRACE") {
            config.trace = truthy(&value);
            config.source = "environment".to_string();
        }
        if let Ok(value) = std::env::var("RETA_ARCH_COMPARE_PY") {
            config.compare_with_python_legacy = truthy(&value);
            config.source = "environment".to_string();
        }
        if let Ok(value) = std::env::var("RETA_ARCH_COMPARE_PY_ARCH") {
            config.compare_with_python_arch = truthy(&value);
            config.source = "environment".to_string();
        }
        if let Ok(value) = std::env::var("RETA_ARCH_ROLLBACK_ANCHOR") {
            let value = value.trim();
            if !value.is_empty() {
                config.rollback_anchor = Some(value.to_string());
                config.source = "environment".to_string();
            }
        }
        if let Ok(value) = std::env::var("RETA_ARCH_ALLOW") {
            config.allowed_morphisms = parse_morphism_list(&value);
            config.source = "environment".to_string();
        }
        if let Ok(value) = std::env::var("RETA_ARCH_BLOCK") {
            config.blocked_morphisms = parse_morphism_list(&value);
            config.source = "environment".to_string();
        }
        config
    }

    pub fn with_mode(mut self, mode: ArchitectureSwitchMode, source: impl Into<String>) -> Self {
        self.mode = mode;
        self.source = source.into();
        self
    }

    pub fn allow(mut self, morphism: impl Into<String>) -> Self {
        self.allowed_morphisms.insert(morphism.into());
        self
    }

    pub fn block(mut self, morphism: impl Into<String>) -> Self {
        self.blocked_morphisms.insert(morphism.into());
        self
    }

    pub fn gate_for_morphism(&self, morphism: &str) -> SwitchGateDecision {
        if self.blocked_morphisms.contains(morphism) {
            return SwitchGateDecision::blocked(morphism, "explicitly_blocked");
        }
        if self.mode == ArchitectureSwitchMode::Legacy {
            return SwitchGateDecision::blocked(morphism, "legacy_mode");
        }
        if !self.allowed_morphisms.is_empty() && !self.allowed_morphisms.contains(morphism) {
            return SwitchGateDecision::shadow_only(morphism, "not_in_allow_list", self.mode);
        }
        match self.mode {
            ArchitectureSwitchMode::Legacy => SwitchGateDecision::blocked(morphism, "legacy_mode"),
            ArchitectureSwitchMode::Observe => {
                SwitchGateDecision::shadow_only(morphism, "observe_only", self.mode)
            }
            ArchitectureSwitchMode::DryRun => {
                SwitchGateDecision::shadow_only(morphism, "dry_run_only", self.mode)
            }
            ArchitectureSwitchMode::Adapter => {
                let may_commit = morphism.starts_with("table_adapters")
                    || morphism.starts_with("prompt_interaction")
                    || morphism.starts_with("parallel_execution")
                    || morphism.starts_with("table_materialization")
                    || morphism.starts_with("table_view")
                    || morphism.starts_with("table_view_html_attributes")
                    || morphism.starts_with("table_view_cell_styles")
                    || morphism.starts_with("table_view_style_composition")
                    || morphism.starts_with("table_view_style_parity")
                    || morphism.starts_with("table_view_commit_audit")
                    || morphism.starts_with("table_view_activation_transaction")
                    || morphism.starts_with("table_view_activation_journal")
                    || morphism.starts_with("table_view_activation_replay")
                    || morphism.starts_with("table_view_layout")
                    || morphism.starts_with("table_view_numbering")
                    || morphism.starts_with("table_view_row_styles")
                    || morphism.starts_with("table_view_output.parity")
                    || morphism.starts_with("table_view_output.semantic_diff")
                    || morphism.starts_with("shadow_pipeline.table_adapter")
                    || morphism.starts_with("shadow_pipeline.table_commit")
                    || morphism.starts_with("shadow_pipeline.table_view_output_adapter")
                    || morphism.starts_with("shadow_pipeline.table_view_output_commit")
                    || morphism.starts_with("shadow_pipeline.table_view_output_semantic_diff")
                    || morphism.starts_with("shadow_pipeline.prompt_adapter")
                    || morphism.starts_with("shadow_pipeline.prompt_commit");
                if may_commit {
                    SwitchGateDecision::allowed(morphism, "adapter_gate", self.mode)
                } else {
                    SwitchGateDecision::shadow_only(morphism, "adapter_mode_shadow", self.mode)
                }
            }
            ArchitectureSwitchMode::Commit | ArchitectureSwitchMode::Force => {
                SwitchGateDecision::allowed(morphism, "commit_gate", self.mode)
            }
        }
    }

    pub fn visible_behaviour_may_change(&self) -> bool {
        self.mode.behaviour_may_change()
    }

    pub fn snapshot(&self) -> ArchitectureSwitchSnapshot {
        ArchitectureSwitchSnapshot {
            class: "ArchitectureSwitchConfig".to_string(),
            mode: self.mode.canonical().to_string(),
            source: self.source.clone(),
            trace: self.trace,
            compare_with_python_arch: self.compare_with_python_arch,
            compare_with_python_legacy: self.compare_with_python_legacy,
            rollback_anchor: self.rollback_anchor.clone(),
            allowed_morphisms: self.allowed_morphisms.iter().cloned().collect(),
            blocked_morphisms: self.blocked_morphisms.iter().cloned().collect(),
            visible_behaviour_may_change: self.visible_behaviour_may_change(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SwitchGateDecision {
    pub morphism: String,
    pub allowed_to_commit: bool,
    pub shadow_execution: bool,
    pub reason: String,
    pub mode: String,
}

impl SwitchGateDecision {
    pub fn allowed(morphism: &str, reason: &str, mode: ArchitectureSwitchMode) -> Self {
        Self {
            morphism: morphism.to_string(),
            allowed_to_commit: true,
            shadow_execution: true,
            reason: reason.to_string(),
            mode: mode.canonical().to_string(),
        }
    }

    pub fn shadow_only(morphism: &str, reason: &str, mode: ArchitectureSwitchMode) -> Self {
        Self {
            morphism: morphism.to_string(),
            allowed_to_commit: false,
            shadow_execution: mode.should_shadow_execute()
                || mode == ArchitectureSwitchMode::Observe,
            reason: reason.to_string(),
            mode: mode.canonical().to_string(),
        }
    }

    pub fn blocked(morphism: &str, reason: &str) -> Self {
        Self {
            morphism: morphism.to_string(),
            allowed_to_commit: false,
            shadow_execution: false,
            reason: reason.to_string(),
            mode: "legacy".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureSwitchSnapshot {
    pub class: String,
    pub mode: String,
    pub source: String,
    pub trace: bool,
    pub compare_with_python_arch: bool,
    pub compare_with_python_legacy: bool,
    pub rollback_anchor: Option<String>,
    pub allowed_morphisms: Vec<String>,
    pub blocked_morphisms: Vec<String>,
    pub visible_behaviour_may_change: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeSwitchBundle {
    pub default_config: ArchitectureSwitchConfig,
    pub known_morphisms: Vec<String>,
}

impl RuntimeSwitchBundle {
    pub fn gate_matrix(&self, config: &ArchitectureSwitchConfig) -> Vec<SwitchGateDecision> {
        self.known_morphisms
            .iter()
            .map(|morphism| config.gate_for_morphism(morphism))
            .collect()
    }

    pub fn snapshot(&self) -> RuntimeSwitchBundleSnapshot {
        RuntimeSwitchBundleSnapshot {
            class: "RuntimeSwitchBundle".to_string(),
            known_morphisms: self.known_morphisms.clone(),
            default_config: self.default_config.snapshot(),
            universal_property:
                "guarded_activation_preserves_legacy_output_until_commit_gate_passes".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeSwitchBundleSnapshot {
    pub class: String,
    pub known_morphisms: Vec<String>,
    pub default_config: ArchitectureSwitchSnapshot,
    pub universal_property: String,
}

pub fn bootstrap_runtime_switch(config: Option<ArchitectureSwitchConfig>) -> RuntimeSwitchBundle {
    RuntimeSwitchBundle {
        default_config: config.unwrap_or_else(ArchitectureSwitchConfig::from_environment),
        known_morphisms: vec![
            "runtime_switch.extract_architecture_switch_from_argv".to_string(),
            "table_adapters.prepare".to_string(),
            "table_adapters.render".to_string(),
            "table_materialization.csv_projection".to_string(),
            "table_materialization.generation_plan".to_string(),
            "table_materialization.column_order_override".to_string(),
            "table_materialization.row_order_override".to_string(),
            "table_view.materialized_view".to_string(),
            "table_view.render_lines".to_string(),
            "table_view_virtual_columns.policy".to_string(),
            "table_view_virtual_columns.tag_summary".to_string(),
            "table_view_virtual_columns.placeholder".to_string(),
            "table_view_virtual_columns.witness".to_string(),
            "table_view_virtual_parity.direct_cell_identity".to_string(),
            "table_view_virtual_parity.added_virtual_only".to_string(),
            "table_view_virtual_parity.raw_commit_guard".to_string(),
            "table_view_virtual_parity.cli_policy_lift".to_string(),
            "table_view_output.commit_virtual_guard".to_string(),
            "table_view_commit_audit.audit_report".to_string(),
            "table_view_commit_audit.required_guards".to_string(),
            "table_view_commit_audit.semantic_witness".to_string(),
            "table_view_commit_audit.virtual_direct_identity".to_string(),
            "table_view_activation_transaction.select_visible_source".to_string(),
            "table_view_activation_transaction.rollback_witness".to_string(),
            "table_view_activation_transaction.commit_audit_gate".to_string(),
            "table_view_activation_journal.record_transaction".to_string(),
            "table_view_activation_journal.replay_selected_lines".to_string(),
            "table_view_activation_journal.rollback_checksum_witness".to_string(),
            "table_view_activation_replay.guard_journal_replay".to_string(),
            "table_view_activation_replay.match_transaction_id".to_string(),
            "table_view_activation_replay.match_legacy_checksum".to_string(),
            "table_view_activation_replay.rollback_to_legacy_lines".to_string(),
            "table_view_html_attributes.class_projection".to_string(),
            "table_view_html_attributes.raw_open_tag".to_string(),
            "table_view_html_attributes.raw_html_witness".to_string(),
            "table_view_cell_styles.legacy_generate_cell".to_string(),
            "table_view_cell_styles.html_cell_wrapper".to_string(),
            "table_view_cell_styles.bbcode_cell_wrapper".to_string(),
            "table_view_style_composition.html_cell_merge".to_string(),
            "table_view_style_composition.attribute_cell_style_law".to_string(),
            "table_view_style_parity.markup_document_normalize".to_string(),
            "table_view_style_parity.bbcode_styled_td".to_string(),
            "table_view_style_parity.raw_commit_guard".to_string(),
            "table_view_layout.column_widths".to_string(),
            "table_view_layout.horizontal_pages".to_string(),
            "table_view_layout.shell_padding".to_string(),
            "table_view_numbering.legacy_prefix".to_string(),
            "table_view_numbering.zaehlung".to_string(),
            "table_view_numbering.nummerierung".to_string(),
            "table_view_row_styles.legacy_colored_begin_col".to_string(),
            "table_view_row_styles.html_row_color".to_string(),
            "table_view_row_styles.bbcode_row_color".to_string(),
            "table_view_shell_styles.legacy_colorize".to_string(),
            "table_view_shell_styles.ansi_cell_wrapper".to_string(),
            "table_view_shell_styles.strip_ansi_parity".to_string(),
            "table_view_output.render".to_string(),
            "table_view_output.mode_projection".to_string(),
            "table_view_output.output_flags".to_string(),
            "table_view_output.width_wrapping".to_string(),
            "table_view_output.header_filter".to_string(),
            "table_view_output.parity_normalize".to_string(),
            "table_view_output.semantic_diff".to_string(),
            "table_view_output.commit".to_string(),
            "shadow_pipeline.table_adapter".to_string(),
            "shadow_pipeline.table_commit".to_string(),
            "shadow_pipeline.table_view_output_adapter".to_string(),
            "shadow_pipeline.table_view_output_commit".to_string(),
            "shadow_pipeline.prompt_adapter".to_string(),
            "shadow_pipeline.prompt_commit".to_string(),
            "shadow_pipeline.cli_plan".to_string(),
            "parallel_execution.rows".to_string(),
            "prompt_interaction.plan".to_string(),
            "prompt_execution.argv".to_string(),
            "completion_nested.candidates".to_string(),
            "generated_columns.registry".to_string(),
            "concat_csv.presheaf_glue".to_string(),
            "combi_join.table_join".to_string(),
        ],
    }
}

pub fn extract_architecture_switch_from_argv(
    argv: &[String],
    inherited: Option<ArchitectureSwitchConfig>,
) -> (Vec<String>, ArchitectureSwitchConfig) {
    let mut config = inherited.unwrap_or_else(ArchitectureSwitchConfig::from_environment);
    let mut clean = Vec::with_capacity(argv.len());
    let mut skip_next = false;
    let mut recognised = false;

    for (index, arg) in argv.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        match arg.as_str() {
            "--reta-arch" | "--arch-mode" | "--architecture-mode" => {
                if let Some(value) = argv.get(index + 1) {
                    config.mode = ArchitectureSwitchMode::from_value(value);
                    config.source = "argv".to_string();
                    recognised = true;
                    skip_next = true;
                }
            }
            "--reta-arch-off" | "--arch-off" | "--architecture-off" => {
                config.mode = ArchitectureSwitchMode::Legacy;
                config.source = "argv".to_string();
                recognised = true;
            }
            "--reta-arch-observe" | "--arch-observe" => {
                config.mode = ArchitectureSwitchMode::Observe;
                config.source = "argv".to_string();
                recognised = true;
            }
            "--reta-arch-dry-run" | "--arch-dry-run" | "--arch-shadow" => {
                config.mode = ArchitectureSwitchMode::DryRun;
                config.source = "argv".to_string();
                recognised = true;
            }
            "--reta-arch-adapters" | "--arch-adapters" => {
                config.mode = ArchitectureSwitchMode::Adapter;
                config.source = "argv".to_string();
                recognised = true;
            }
            "--reta-arch-commit" | "--arch-commit" => {
                config.mode = ArchitectureSwitchMode::Commit;
                config.source = "argv".to_string();
                recognised = true;
            }
            "--reta-arch-force" | "--arch-force" => {
                config.mode = ArchitectureSwitchMode::Force;
                config.source = "argv".to_string();
                recognised = true;
            }
            "--reta-arch-trace" | "--arch-trace" => {
                config.trace = true;
                config.source = "argv".to_string();
                recognised = true;
            }
            "--no-reta-arch-trace" | "--no-arch-trace" => {
                config.trace = false;
                config.source = "argv".to_string();
                recognised = true;
            }
            _ if arg.starts_with("--reta-arch=")
                || arg.starts_with("--arch-mode=")
                || arg.starts_with("--architecture-mode=") =>
            {
                if let Some((_, value)) = arg.split_once('=') {
                    config.mode = ArchitectureSwitchMode::from_value(value);
                    config.source = "argv".to_string();
                    recognised = true;
                }
            }
            _ if arg.starts_with("--reta-arch-allow=") || arg.starts_with("--arch-allow=") => {
                if let Some((_, value)) = arg.split_once('=') {
                    config.allowed_morphisms.extend(parse_morphism_list(value));
                    config.source = "argv".to_string();
                    recognised = true;
                }
            }
            _ if arg.starts_with("--reta-arch-block=") || arg.starts_with("--arch-block=") => {
                if let Some((_, value)) = arg.split_once('=') {
                    config.blocked_morphisms.extend(parse_morphism_list(value));
                    config.source = "argv".to_string();
                    recognised = true;
                }
            }
            _ if arg.starts_with("--reta-arch-rollback=")
                || arg.starts_with("--arch-rollback=") =>
            {
                if let Some((_, value)) = arg.split_once('=') {
                    let value = value.trim();
                    if !value.is_empty() {
                        config.rollback_anchor = Some(value.to_string());
                    }
                    config.source = "argv".to_string();
                    recognised = true;
                }
            }
            _ => clean.push(arg.clone()),
        }
    }

    if recognised && config.source != "argv" {
        config.source = "argv".to_string();
    }
    (clean, config)
}

fn parse_morphism_list(value: &str) -> BTreeSet<String> {
    value
        .split(|ch: char| ch == ',' || ch == ';' || ch.is_whitespace())
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn truthy(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "y" | "on" | "trace"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argv_switch_flags_are_stripped_before_legacy_execution() {
        let argv = vec![
            "reta".to_string(),
            "--reta-arch=dry-run".to_string(),
            "--reta-arch-allow=table_adapters.prepare,prompt_interaction.plan".to_string(),
            "-zeilen".to_string(),
        ];
        let (clean, config) =
            extract_architecture_switch_from_argv(&argv, Some(Default::default()));
        assert_eq!(clean, vec!["reta".to_string(), "-zeilen".to_string()]);
        assert_eq!(config.mode, ArchitectureSwitchMode::DryRun);
        assert!(config.allowed_morphisms.contains("table_adapters.prepare"));
    }

    #[test]
    fn adapter_mode_allows_only_adapter_family_to_commit_by_default() {
        let config =
            ArchitectureSwitchConfig::default().with_mode(ArchitectureSwitchMode::Adapter, "test");
        assert!(
            config
                .gate_for_morphism("table_adapters.prepare")
                .allowed_to_commit
        );
        assert!(
            !config
                .gate_for_morphism("prompt_execution.argv")
                .allowed_to_commit
        );
    }
}
