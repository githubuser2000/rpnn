//! Rust mirror of `python_arch_reference/reta_architecture/architecture_map.py`.
//!
//! This module names the architectural capsules, flows and legacy-owner mapping
//! that later activation gates use.  It is intentionally data-first: callers can
//! ask which capsule owns a legacy surface before committing any runtime change.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureCapsuleSpec {
    pub name: String,
    pub layer: String,
    pub contains: Vec<String>,
    pub code_owners: Vec<String>,
    pub paradigm_roles: Vec<String>,
    pub inbound: Vec<String>,
    pub outbound: Vec<String>,
    pub stage_span: String,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureFlowSpec {
    pub source: String,
    pub target: String,
    pub morphism: String,
    pub functor: String,
    pub owner: String,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetaPartMappingSpec {
    pub legacy_owner: String,
    pub capsule: String,
    pub target_owner: String,
    pub migration_status: String,
    pub evidence: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StageArchitectureStep {
    pub stage: String,
    pub focus: String,
    pub moved_from: Vec<String>,
    pub moved_to: Vec<String>,
    pub capsule: String,
    pub paradigm_shift: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapsuleContainmentSpec {
    pub parent: String,
    pub child: String,
    pub relationship: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MarkdownAuditSpec {
    pub source: String,
    pub covered_stages: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureMapSnapshot {
    pub class: String,
    pub capsules: usize,
    pub flows: usize,
    pub legacy_mappings: usize,
    pub stage_steps: usize,
    pub text_diagram: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureMapBundle {
    pub capsules: Vec<ArchitectureCapsuleSpec>,
    pub containment: Vec<CapsuleContainmentSpec>,
    pub flows: Vec<ArchitectureFlowSpec>,
    pub legacy_mappings: Vec<RetaPartMappingSpec>,
    pub stage_steps: Vec<StageArchitectureStep>,
    pub markdown_audit: MarkdownAuditSpec,
    pub text_diagram: String,
    pub mermaid_diagram: String,
}

impl ArchitectureMapBundle {
    pub fn capsule_names(&self) -> Vec<String> {
        self.capsules.iter().map(|capsule| capsule.name.clone()).collect()
    }

    pub fn capsule_named(&self, name: &str) -> Option<&ArchitectureCapsuleSpec> {
        self.capsules.iter().find(|capsule| capsule.name == name)
    }

    pub fn mappings_for_capsule(&self, capsule: &str) -> Vec<&RetaPartMappingSpec> {
        self.legacy_mappings
            .iter()
            .filter(|mapping| mapping.capsule == capsule)
            .collect()
    }

    pub fn owner_for_legacy_surface(&self, owner: &str) -> Option<&RetaPartMappingSpec> {
        self.legacy_mappings
            .iter()
            .find(|mapping| mapping.legacy_owner == owner)
    }

    pub fn snapshot(&self) -> ArchitectureMapSnapshot {
        ArchitectureMapSnapshot {
            class: "ArchitectureMapBundle".to_string(),
            capsules: self.capsules.len(),
            flows: self.flows.len(),
            legacy_mappings: self.legacy_mappings.len(),
            stage_steps: self.stage_steps.len(),
            text_diagram: self.text_diagram.clone(),
            universal_property: "legacy_surfaces_factor_through_typed_architecture_capsules".to_string(),
        }
    }
}

pub fn bootstrap_architecture_map() -> ArchitectureMapBundle {
    let capsules = default_capsules();
    ArchitectureMapBundle {
        containment: default_containment(),
        flows: default_flows(),
        legacy_mappings: default_legacy_mappings(),
        stage_steps: default_stage_steps(),
        markdown_audit: MarkdownAuditSpec {
            source: "python_arch_reference/reta_architecture".to_string(),
            covered_stages: (28..=42).map(|stage| format!("stage-{stage}")).collect(),
            status: "mirrored-as-rust-control-data".to_string(),
        },
        text_diagram: "RawInput -> Schema/Input -> Workflow -> Table/Prompt -> Output, guarded by Contracts/Validation".to_string(),
        mermaid_diagram: "flowchart LR\n  RawInput --> InputPromptCapsule --> WorkflowGluingCapsule --> TableCoreCapsule --> OutputRenderingCapsule\n  GovernanceCapsule -. gates .-> WorkflowGluingCapsule".to_string(),
        capsules,
    }
}

fn cap(name: &str, layer: &str, contains: &[&str], code_owners: &[&str], roles: &[&str], inbound: &[&str], outbound: &[&str], stage_span: &str, description: &str) -> ArchitectureCapsuleSpec {
    ArchitectureCapsuleSpec {
        name: name.to_string(),
        layer: layer.to_string(),
        contains: contains.iter().map(|v| (*v).to_string()).collect(),
        code_owners: code_owners.iter().map(|v| (*v).to_string()).collect(),
        paradigm_roles: roles.iter().map(|v| (*v).to_string()).collect(),
        inbound: inbound.iter().map(|v| (*v).to_string()).collect(),
        outbound: outbound.iter().map(|v| (*v).to_string()).collect(),
        stage_span: stage_span.to_string(),
        description: description.to_string(),
    }
}

fn default_capsules() -> Vec<ArchitectureCapsuleSpec> {
    vec![
        cap("RetaArchitectureRoot", "0 root/facade", &["SchemaTopologyCapsule", "WorkflowGluingCapsule", "GovernanceCapsule"], &["facade.rs", "lib.rs"], &["Gesamtfunktor", "Kapsel-Root"], &["CLI", "Prompt"], &["RuntimeSnapshot"], "28-42", "shared typed runtime facade for rreta and rretaPrompt"),
        cap("SchemaTopologyCapsule", "1 schema/topology", &["schema", "input_semantics", "topology", "tag_schema"], &["schema.rs", "input_semantics.rs", "topology.rs", "tag_schema.rs"], &["Objektklassifikation", "Topologie"], &["RawArgs"], &["ContextSelection"], "28-42", "names the open sets and parameter/tag objects"),
        cap("LocalSectionCapsule", "2 local sections", &["row_ranges", "column_selection", "parameter_runtime"], &["row_ranges.rs", "column_selection.rs", "parameter_runtime.rs"], &["Prägarbe", "Lokalschnitt"], &["ContextSelection"], &["SelectedRows", "SelectedColumns"], "37-42", "parses row and column local sections"),
        cap("SemanticSheafCapsule", "3 semantic sheaf", &["presheaf", "sheaf", "number_theory", "arithmetic"], &["presheaf.rs", "sheaf.rs", "number_theory.rs", "arithmetic.rs"], &["Garbenverklebung", "Morphismen"], &["LocalSections"], &["SemanticSections"], "28-42", "glues local mathematical semantics"),
        cap("InputPromptCapsule", "4 prompt/input", &["prompt_language", "prompt_runtime", "completion_runtime", "completion_word", "completion_nested"], &["prompt_language.rs", "prompt_runtime.rs", "completion_runtime.rs", "completion_word.rs", "completion_nested.rs"], &["Bidirektionaler Kanal", "Prompt-Funktor"], &["PromptText"], &["PromptPlan"], "40-42", "retaprompt lexical, completion and nested-command layer"),
        cap("WorkflowGluingCapsule", "5 workflow", &["program_workflow", "prompt_preparation", "prompt_execution", "prompt_interaction"], &["program_workflow.rs", "prompt_preparation.rs", "prompt_execution.rs", "prompt_interaction.rs"], &["natürliche Transformation", "Workflow-Kleber"], &["InputPlan"], &["ExecutionPlan"], "34-42", "compiles CLI/prompt context into executable plans"),
        cap("TableCoreCapsule", "6 table core", &["table_runtime", "table_state", "table_generation", "table_preparation", "row_filtering", "table_wrapping"], &["table_runtime.rs", "table_state.rs", "table_generation.rs", "table_preparation.rs", "row_filtering.rs", "table_wrapping.rs"], &["Tabellenfunktor", "Relationen"], &["ExecutionPlan"], &["PreparedTable"], "28-42", "typed table state, row filtering and preparation"),
        cap("GeneratedRelationCapsule", "7 generated relations", &["generated_columns", "meta_columns", "concat_csv", "combi_join"], &["generated_columns.rs", "meta_columns.rs", "concat_csv.rs", "combi_join.rs"], &["Relationenmorphismus", "CSV-Prägarbe"], &["PreparedTable"], &["GeneratedRelations"], "28-42", "generated, meta, concat and combination relations"),
        cap("OutputRenderingCapsule", "8 output", &["output_syntax", "output_semantics", "table_output", "console_io"], &["output_syntax.rs", "output_semantics.rs", "table_output.rs", "console_io.rs"], &["Renderer-Funktor", "Darstellung"], &["PreparedTable"], &["VisibleOutput"], "28-42", "shell/csv/html/bbcode/markdown rendering boundary"),
        cap("CompatibilityCapsule", "9 compatibility", &["runtime_compat", "split_i18n", "package_integrity", "persistence"], &["runtime_compat.rs", "split_i18n.rs", "package_integrity.rs", "persistence.rs"], &["Kompatibilitätsfaser", "Audit"], &["LegacyRuntime"], &["StableFacade"], "31-42", "keeps legacy surfaces stable while Rust takes ownership"),
        cap("ExecutionNetworkCapsule", "10 execution network", &["dataflow", "execution_network", "parallel_execution"], &["dataflow.rs", "execution_network.rs", "parallel_execution.rs"], &["FIFO", "LIFO", "Semaphore", "Datenstrom"], &["ExecutionTask"], &["ExecutionRunResult"], "39-42", "deterministic serial/threaded scheduling and gluing"),
        cap("GovernanceCapsule", "11 governance", &["runtime_switch", "migration_control", "parity_harness", "architecture_*"], &["runtime_switch.rs", "migration_control.rs", "parity_harness.rs", "architecture_validation.rs"], &["Verträge", "Zeugen", "Kohärenz", "Aktivierung"], &["MigrationCandidate"], &["ActivationDecision"], "29-42", "controls contracts, rehearsal, activation and parity gates"),
        cap("CategoricalMetaCapsule", "12 categorical meta", &["category", "morphism", "universal"], &["category.rs", "morphism.rs", "universal.rs"], &["Kategorie", "Funktor", "natürliche Transformation", "universelle Eigenschaft"], &["ArchitectureObjects"], &["ProofObligations"], "27-42", "mathematical reading of the refactor"),
    ]
}

fn default_containment() -> Vec<CapsuleContainmentSpec> {
    default_capsules()
        .into_iter()
        .filter(|capsule| capsule.name != "RetaArchitectureRoot")
        .map(|capsule| CapsuleContainmentSpec {
            parent: "RetaArchitectureRoot".to_string(),
            child: capsule.name,
            relationship: "root_contains_capsule".to_string(),
        })
        .collect()
}

fn flow(source: &str, target: &str, morphism: &str, functor: &str, owner: &str, description: &str) -> ArchitectureFlowSpec {
    ArchitectureFlowSpec { source: source.to_string(), target: target.to_string(), morphism: morphism.to_string(), functor: functor.to_string(), owner: owner.to_string(), description: description.to_string() }
}

fn default_flows() -> Vec<ArchitectureFlowSpec> {
    vec![
        flow("RawArgs", "ContextSelection", "parse_cli", "InputFunctor", "parameter_runtime.rs", "CLI arguments become typed context"),
        flow("PromptText", "PromptExecutionPlan", "compile_prompt", "PromptFunctor", "prompt_interaction.rs", "Prompt local text compiles to reta argv"),
        flow("ContextSelection", "SelectedRows", "row_range_section", "LocalSectionFunctor", "row_ranges.rs", "row range text becomes row set"),
        flow("SelectedColumns", "TagSections", "tag_schema_lookup", "ColumnTagFunctor", "tag_schema.rs", "column ids become tag sections"),
        flow("ExecutionPlan", "PreparedTable", "prepare_table", "TableFunctor", "table_preparation.rs", "runtime table state becomes prepared output table"),
        flow("PreparedTable", "VisibleOutput", "render_table", "OutputFunctor", "table_output.rs", "prepared table glues to visible output"),
        flow("ExecutionTaskCover", "OrderedResults", "deterministic_reduce", "ExecutionNetworkFunctor", "execution_network.rs", "scheduler cover glues to ordered output"),
        flow("MigrationCandidate", "ActivationDecision", "guarded_activation", "GovernanceFunctor", "architecture_activation.rs", "candidate is committed only when witnesses commute"),
    ]
}

fn map(legacy_owner: &str, capsule: &str, target_owner: &str, status: &str, evidence: &[&str]) -> RetaPartMappingSpec {
    RetaPartMappingSpec { legacy_owner: legacy_owner.to_string(), capsule: capsule.to_string(), target_owner: target_owner.to_string(), migration_status: status.to_string(), evidence: evidence.iter().map(|v| (*v).to_string()).collect() }
}

fn default_legacy_mappings() -> Vec<RetaPartMappingSpec> {
    vec![
        map("reta.py::Program", "WorkflowGluingCapsule", "program_workflow.rs", "shadow", &["RetaRunArchitecture", "parameter_runtime"]),
        map("reta.py::Program.__resultingTable", "TableCoreCapsule", "table_preparation.rs", "shadow", &["table_adapters.prepare", "table_preparation"]),
        map("reta.py::Program.finallyDisplayLines", "OutputRenderingCapsule", "table_output.rs", "shadow", &["table_adapters.render", "table_output"]),
        map("libs/lib4tables_Enum.py", "SchemaTopologyCapsule", "tag_schema.rs", "owned", &["column 744 regression", "TableTag"]),
        map("libs/tableHandling.py", "TableCoreCapsule", "table_generation.rs", "shadow", &["table_runtime", "table_generation"]),
        map("libs/lib4tables_concat.py", "GeneratedRelationCapsule", "concat_csv.rs", "shadow", &["concat_csv", "combi_join"]),
        map("retaPrompt.py", "InputPromptCapsule", "prompt_interaction.rs", "shadow", &["prompt_session", "prompt_execution"]),
        map("libs/LibRetaPrompt.py", "InputPromptCapsule", "prompt_runtime.rs", "shadow", &["completion_nested", "completion_word"]),
        map("multiprocessing rows", "ExecutionNetworkCapsule", "execution_network.rs", "planned", &["parallel_execution", "dataflow"]),
        map("architecture audit", "GovernanceCapsule", "architecture_validation.rs", "owned", &["contracts", "witnesses", "activation"]),
    ]
}

fn default_stage_steps() -> Vec<StageArchitectureStep> {
    vec![
        StageArchitectureStep { stage: "29".to_string(), focus: "contracts".to_string(), moved_from: vec!["implicit parity claims".to_string()], moved_to: vec!["architecture_contracts".to_string()], capsule: "GovernanceCapsule".to_string(), paradigm_shift: "implicit laws become named diagrams".to_string() },
        StageArchitectureStep { stage: "37".to_string(), focus: "row ranges".to_string(), moved_from: vec!["BereichToNumbers2".to_string()], moved_to: vec!["row_ranges.rs".to_string()], capsule: "LocalSectionCapsule".to_string(), paradigm_shift: "text ranges become typed local sections".to_string() },
        StageArchitectureStep { stage: "40".to_string(), focus: "word completion".to_string(), moved_from: vec!["word_completerAlx".to_string()], moved_to: vec!["completion_word.rs".to_string()], capsule: "InputPromptCapsule".to_string(), paradigm_shift: "interactive completion becomes morphism bundle".to_string() },
        StageArchitectureStep { stage: "42".to_string(), focus: "progress".to_string(), moved_from: vec!["manual progress notes".to_string()], moved_to: vec!["architecture_progress.rs".to_string()], capsule: "GovernanceCapsule".to_string(), paradigm_shift: "migration state becomes queryable runtime data".to_string() },
    ]
}



pub fn _legacy_mappings() -> Vec<RetaPartMappingSpec> {
    default_legacy_mappings()
}

pub fn _stage_steps() -> Vec<StageArchitectureStep> {
    default_stage_steps()
}

pub fn _markdown_audit() -> MarkdownAuditSpec {
    bootstrap_architecture_map().markdown_audit
}

pub fn _step(stage: &str, focus: &str, capsule: &str) -> StageArchitectureStep {
    StageArchitectureStep {
        stage: stage.to_string(),
        focus: focus.to_string(),
        moved_from: Vec::new(),
        moved_to: Vec::new(),
        capsule: capsule.to_string(),
        paradigm_shift: "typed migration step".to_string(),
    }
}

pub fn _stage32_capsules() -> Vec<ArchitectureCapsuleSpec> {
    default_capsules()
}

pub fn _stage32_containment() -> Vec<CapsuleContainmentSpec> {
    default_containment()
}

pub fn _stage32_flows() -> Vec<ArchitectureFlowSpec> {
    default_flows()
}

pub fn _stage32_legacy_mappings() -> Vec<RetaPartMappingSpec> {
    default_legacy_mappings()
}

pub fn _stage32_stage_steps() -> Vec<StageArchitectureStep> {
    default_stage_steps()
}

pub fn _stage33_capsules() -> Vec<ArchitectureCapsuleSpec> {
    default_capsules()
}

pub fn _stage33_containment() -> Vec<CapsuleContainmentSpec> {
    default_containment()
}

pub fn _stage33_flows() -> Vec<ArchitectureFlowSpec> {
    default_flows()
}

pub fn _stage33_legacy_mappings() -> Vec<RetaPartMappingSpec> {
    default_legacy_mappings()
}

pub fn _stage33_stage_steps() -> Vec<StageArchitectureStep> {
    default_stage_steps()
}

pub fn _stage34_capsules() -> Vec<ArchitectureCapsuleSpec> {
    default_capsules()
}

pub fn _stage34_containment() -> Vec<CapsuleContainmentSpec> {
    default_containment()
}

pub fn _stage34_flows() -> Vec<ArchitectureFlowSpec> {
    default_flows()
}

pub fn _stage34_legacy_mappings() -> Vec<RetaPartMappingSpec> {
    default_legacy_mappings()
}

pub fn _stage34_stage_steps() -> Vec<StageArchitectureStep> {
    default_stage_steps()
}

pub fn _stage35_capsules() -> Vec<ArchitectureCapsuleSpec> {
    default_capsules()
}

pub fn _stage35_containment() -> Vec<CapsuleContainmentSpec> {
    default_containment()
}

pub fn _stage35_flows() -> Vec<ArchitectureFlowSpec> {
    default_flows()
}

pub fn _stage35_legacy_mappings() -> Vec<RetaPartMappingSpec> {
    default_legacy_mappings()
}

pub fn _stage35_stage_steps() -> Vec<StageArchitectureStep> {
    default_stage_steps()
}

pub fn _stage36_capsules() -> Vec<ArchitectureCapsuleSpec> {
    default_capsules()
}

pub fn _stage36_containment() -> Vec<CapsuleContainmentSpec> {
    default_containment()
}

pub fn _stage36_flows() -> Vec<ArchitectureFlowSpec> {
    default_flows()
}

pub fn _stage36_legacy_mappings() -> Vec<RetaPartMappingSpec> {
    default_legacy_mappings()
}

pub fn _stage36_stage_steps() -> Vec<StageArchitectureStep> {
    default_stage_steps()
}

pub fn _stage37_capsules() -> Vec<ArchitectureCapsuleSpec> {
    default_capsules()
}

pub fn _stage37_containment() -> Vec<CapsuleContainmentSpec> {
    default_containment()
}

pub fn _stage37_flows() -> Vec<ArchitectureFlowSpec> {
    default_flows()
}

pub fn _stage37_legacy_mappings() -> Vec<RetaPartMappingSpec> {
    default_legacy_mappings()
}

pub fn _stage37_stage_steps() -> Vec<StageArchitectureStep> {
    default_stage_steps()
}

pub fn _stage38_capsules() -> Vec<ArchitectureCapsuleSpec> {
    default_capsules()
}

pub fn _stage38_containment() -> Vec<CapsuleContainmentSpec> {
    default_containment()
}

pub fn _stage38_flows() -> Vec<ArchitectureFlowSpec> {
    default_flows()
}

pub fn _stage38_legacy_mappings() -> Vec<RetaPartMappingSpec> {
    default_legacy_mappings()
}

pub fn _stage38_stage_steps() -> Vec<StageArchitectureStep> {
    default_stage_steps()
}

pub fn _stage39_capsules() -> Vec<ArchitectureCapsuleSpec> {
    default_capsules()
}

pub fn _stage39_containment() -> Vec<CapsuleContainmentSpec> {
    default_containment()
}

pub fn _stage39_flows() -> Vec<ArchitectureFlowSpec> {
    default_flows()
}

pub fn _stage39_legacy_mappings() -> Vec<RetaPartMappingSpec> {
    default_legacy_mappings()
}

pub fn _stage39_stage_steps() -> Vec<StageArchitectureStep> {
    default_stage_steps()
}

pub fn _stage40_capsules() -> Vec<ArchitectureCapsuleSpec> {
    default_capsules()
}

pub fn _stage40_containment() -> Vec<CapsuleContainmentSpec> {
    default_containment()
}

pub fn _stage40_flows() -> Vec<ArchitectureFlowSpec> {
    default_flows()
}

pub fn _stage40_legacy_mappings() -> Vec<RetaPartMappingSpec> {
    default_legacy_mappings()
}

pub fn _stage40_stage_steps() -> Vec<StageArchitectureStep> {
    default_stage_steps()
}

pub fn _stage41_capsules() -> Vec<ArchitectureCapsuleSpec> {
    default_capsules()
}

pub fn _stage41_containment() -> Vec<CapsuleContainmentSpec> {
    default_containment()
}

pub fn _stage41_flows() -> Vec<ArchitectureFlowSpec> {
    default_flows()
}

pub fn _stage41_legacy_mappings() -> Vec<RetaPartMappingSpec> {
    default_legacy_mappings()
}

pub fn _stage41_stage_steps() -> Vec<StageArchitectureStep> {
    default_stage_steps()
}

pub fn _stage42_legacy_mappings() -> Vec<RetaPartMappingSpec> {
    default_legacy_mappings()
}

pub fn _stage42_stage_steps() -> Vec<StageArchitectureStep> {
    default_stage_steps()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_names_core_capsules() {
        let map = bootstrap_architecture_map();
        assert!(map.capsule_names().contains(&"TableCoreCapsule".to_string()));
        assert!(map.owner_for_legacy_surface("libs/lib4tables_Enum.py").is_some());
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// Marker-only names still need semantic Rust implementation before activation.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "_capsule",
    "_capsules",
    "_containment",
    "_flows",
    "_mapping",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}

// Stage 16 governance concrete wrapper surface.
pub fn _capsule(name: &str) -> Option<ArchitectureCapsuleSpec> { bootstrap_architecture_map().capsules.into_iter().find(|c| c.name == name) }
pub fn _capsules() -> Vec<ArchitectureCapsuleSpec> { bootstrap_architecture_map().capsules }
pub fn _containment() -> Vec<CapsuleContainmentSpec> { bootstrap_architecture_map().containment }
pub fn _flows() -> Vec<ArchitectureFlowSpec> { bootstrap_architecture_map().flows }
pub fn _mapping() -> Vec<RetaPartMappingSpec> { bootstrap_architecture_map().legacy_mappings }
