use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub const PYTHON_CATEGORY_THEORY_SNAPSHOT: &str = include_str!("../data/category_theory_snapshot.json");

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CategoryObjectSpec {
    pub name: String,
    pub code_owner: String,
    pub role: String,
}

impl CategoryObjectSpec {
    pub fn new(name: impl Into<String>, code_owner: impl Into<String>, role: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            code_owner: code_owner.into(),
            role: role.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CategoryMorphismSpec {
    pub name: String,
    pub source: String,
    pub target: String,
    pub code_owner: String,
    pub role: String,
}

impl CategoryMorphismSpec {
    pub fn new(
        name: impl Into<String>,
        source: impl Into<String>,
        target: impl Into<String>,
        code_owner: impl Into<String>,
        role: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            source: source.into(),
            target: target.into(),
            code_owner: code_owner.into(),
            role: role.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CategorySpec {
    pub name: String,
    pub description: String,
    pub objects: Vec<CategoryObjectSpec>,
    pub morphisms: Vec<CategoryMorphismSpec>,
    pub implemented_by: Vec<String>,
}

impl CategorySpec {
    pub fn object_named(&self, name: &str) -> Option<&CategoryObjectSpec> {
        self.objects.iter().find(|item| item.name == name)
    }

    pub fn morphism_named(&self, name: &str) -> Option<&CategoryMorphismSpec> {
        self.morphisms.iter().find(|item| item.name == name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctorSpec {
    pub name: String,
    pub source_category: String,
    pub target_category: String,
    pub variance: String,
    pub object_map: BTreeMap<String, String>,
    pub morphism_map: BTreeMap<String, String>,
    pub code_owner: String,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NaturalTransformationSpec {
    pub name: String,
    pub source_functor: String,
    pub target_functor: String,
    pub components: BTreeMap<String, String>,
    pub naturality_condition: String,
    pub code_owner: String,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParadigmTermSpec {
    pub term: String,
    pub meaning: String,
    pub implemented_by: Vec<String>,
    pub stage_status: String,
}

impl ParadigmTermSpec {
    pub fn new(term: &str, meaning: &str, implemented_by: &[&str], stage_status: &str) -> Self {
        Self {
            term: term.to_string(),
            meaning: meaning.to_string(),
            implemented_by: implemented_by.iter().map(|item| (*item).to_string()).collect(),
            stage_status: stage_status.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Stage27ArchitecturePlan {
    pub planned_before_stage_27: Vec<String>,
    pub implemented_in_stage_27: Vec<String>,
    pub already_implemented_before_stage_27: Vec<String>,
    pub behaviour_change: String,
}

impl Stage27ArchitecturePlan {
    pub fn snapshot(&self) -> BTreeMap<&'static str, Vec<String>> {
        BTreeMap::from([
            ("planned_before_stage_27", self.planned_before_stage_27.clone()),
            ("implemented_in_stage_27", self.implemented_in_stage_27.clone()),
            ("already_implemented_before_stage_27", self.already_implemented_before_stage_27.clone()),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CategoryTheoryBundle {
    pub categories: Vec<CategorySpec>,
    pub functors: Vec<FunctorSpec>,
    pub natural_transformations: Vec<NaturalTransformationSpec>,
    pub paradigm_terms: Vec<String>,
    pub python_snapshot_digest_hint: String,
}

impl CategoryTheoryBundle {
    pub fn category_named(&self, name: &str) -> Option<&CategorySpec> {
        self.categories.iter().find(|item| item.name == name)
    }

    pub fn functor_named(&self, name: &str) -> Option<&FunctorSpec> {
        self.functors.iter().find(|item| item.name == name)
    }

    pub fn natural_transformation_named(&self, name: &str) -> Option<&NaturalTransformationSpec> {
        self.natural_transformations
            .iter()
            .find(|item| item.name == name)
    }

    pub fn counts(&self) -> BTreeMap<&'static str, usize> {
        BTreeMap::from([
            ("categories", self.categories.len()),
            ("functors", self.functors.len()),
            (
                "natural_transformations",
                self.natural_transformations.len(),
            ),
            ("paradigm_terms", self.paradigm_terms.len()),
        ])
    }

    pub fn python_snapshot(&self) -> &'static str {
        PYTHON_CATEGORY_THEORY_SNAPSHOT
    }

    pub fn snapshot(&self) -> CategoryTheorySnapshot {
        CategoryTheorySnapshot {
            class: "CategoryTheoryBundle".to_string(),
            paradigm: _paradigm_terms().into_iter().map(|term| term.term).collect(),
            categories: self.categories.len(),
            functors: self.functors.len(),
            natural_transformations: self.natural_transformations.len(),
            paradigm_terms: self.paradigm_terms.len(),
            plan: _plan(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CategoryTheorySnapshot {
    pub class: String,
    pub paradigm: Vec<String>,
    pub categories: usize,
    pub functors: usize,
    pub natural_transformations: usize,
    pub paradigm_terms: usize,
    pub plan: Stage27ArchitecturePlan,
}

fn obj(name: &str, code_owner: &str, role: &str) -> CategoryObjectSpec {
    CategoryObjectSpec::new(name, code_owner, role)
}

fn mor(name: &str, source: &str, target: &str, code_owner: &str, role: &str) -> CategoryMorphismSpec {
    CategoryMorphismSpec::new(name, source, target, code_owner, role)
}

pub fn bootstrap_category_theory() -> CategoryTheoryBundle {
    let categories = vec![
        CategorySpec {
            name: "OpenRetaContextCategory".to_string(),
            description: "Symbolic open Reta contexts; morphisms are restrictions/refinements/covers.".to_string(),
            objects: vec![
                obj("ContextSelection", "reta_architecture::topology::ContextSelection", "basis-open context"),
                obj("RetaContextTopology", "reta_architecture::topology::RetaContextTopology", "topology over CLI/prompt/table scopes"),
                obj("ContextCover", "RetaContextTopology::cover_for_main", "local cover around a main parameter"),
            ],
            morphisms: vec![
                mor("refine", "ContextSelection", "ContextSelection", "ContextSelection::refine", "meet/intersection of restrictions"),
                mor("open_for", "RetaContextTopology", "ContextSelection", "RetaContextTopology::open_for", "basis open constructor"),
                mor("cover_for_main", "RetaContextTopology", "ContextCover", "RetaContextTopology::cover_for_main", "main parameter cover"),
            ],
            implemented_by: vec!["src/topology.rs".to_string()],
        },
        CategorySpec {
            name: "ExecutionNetworkCategory".to_string(),
            description: "Deterministic dataflow network for executable local tasks.".to_string(),
            objects: vec![
                obj("ExecutionTask", "reta_architecture::dataflow::ExecutionTask", "local executable section"),
                obj("FifoTaskQueue", "reta_architecture::dataflow::FifoTaskQueue", "FIFO queue discipline"),
                obj("LifoTaskStack", "reta_architecture::dataflow::LifoTaskStack", "LIFO stack discipline"),
                obj("PriorityTaskQueue", "reta_architecture::dataflow::PriorityTaskQueue", "priority discipline"),
                obj("HalfDuplexChannel", "reta_architecture::dataflow::HalfDuplexChannel", "request/response channel"),
                obj("FullDuplexChannel", "reta_architecture::dataflow::FullDuplexChannel", "bidirectional channel"),
                obj("ResourceSemaphore", "reta_architecture::dataflow::ResourceSemaphore", "bounded resource guard"),
            ],
            morphisms: vec![
                mor("enqueue_task", "ExecutionTask", "TaskQueue", "push", "insert task into chosen discipline"),
                mor("dequeue_task", "TaskQueue", "ExecutionTask", "pop", "extract next task"),
                mor("dispatch_task", "ExecutionTask", "ExecutionResult", "execute_tasks_deterministically", "run task handler"),
                mor("deterministic_reduce", "ExecutionResult", "GlobalValue", "deterministic_reduce", "glue local results"),
                mor("send_message", "Channel", "Channel", "send_*", "message flow"),
                mor("receive_message", "Channel", "Channel", "receive_*", "message flow"),
            ],
            implemented_by: vec!["src/dataflow.rs".to_string()],
        },
        CategorySpec {
            name: "PresheafCategory".to_string(),
            description: "Local sections indexed by Reta contexts before gluing.".to_string(),
            objects: vec![
                obj("LocalSection", "reta_architecture::presheaf::LocalSection", "context-indexed local payload"),
                obj("Presheaf", "reta_architecture::presheaf::Presheaf", "collection of local sections"),
            ],
            morphisms: vec![
                mor("restrict_section", "LocalSection", "LocalSection", "LocalSection::restrict", "context restriction"),
                mor("add_section", "Payload", "LocalSection", "Presheaf::add_section", "local materialisation"),
            ],
            implemented_by: vec!["src/presheaf.rs".to_string()],
        },
        CategorySpec {
            name: "SheafCategory".to_string(),
            description: "Compatible local sections glued into deterministic global semantics.".to_string(),
            objects: vec![
                obj("Sheaf", "reta_architecture::sheaf::Sheaf", "gluable semantic owner"),
                obj("GluedSection", "reta_architecture::sheaf::GluedSection", "global section"),
            ],
            morphisms: vec![
                mor("is_compatible", "LocalSection", "Boolean", "Sheaf::is_compatible", "overlap compatibility"),
                mor("glue", "LocalSections", "GluedSection", "Sheaf::glue", "deterministic union"),
            ],
            implemented_by: vec!["src/sheaf.rs".to_string()],
        },
        CategorySpec {
            name: "PromptRuntimeCategory".to_string(),
            description: "Prompt input, command compilation, completion and execution contexts.".to_string(),
            objects: vec![
                obj("PromptArchitectureContext", "reta_architecture::facade::PromptArchitectureContext", "prompt state context"),
                obj("PromptTokenStream", "rretaPrompt tokenize/completion modules", "prompt data stream"),
                obj("PromptCommand", "src/prompt/commands.rs", "compiled prompt command"),
            ],
            morphisms: vec![
                mor("split_prompt_text", "PromptInput", "PromptTokenStream", "src/prompt/tokenize.rs", "prompt splitter"),
                mor("compile_command", "PromptTokenStream", "PromptCommand", "src/prompt/commands.rs", "command morphism"),
                mor("execute_command", "PromptCommand", "RetaRun", "src/prompt/commands.rs", "runtime morphism"),
            ],
            implemented_by: vec![
                "src/prompt/*.rs".to_string(),
                "crates/retaprompt_input".to_string(),
                "crates/retaprompt_commands".to_string(),
            ],
        },
        CategorySpec {
            name: "OutputFunctorCategory".to_string(),
            description: "Display categories for CLI, HTML, BBCode and Emacs output.".to_string(),
            objects: vec![
                obj("SemanticTable", "src/shared/reta_workflow_py.rs", "generated table semantics"),
                obj("CliText", "src/reta_ausgabe/cli_output.rs", "terminal rendering"),
                obj("HtmlText", "src/domain/python_html_meta*.rs", "HTML rendering/reference"),
                obj("BbCodeText", "python_reference/libs/bbcode.py", "BBCode rendering/reference"),
                obj("EmacsText", "src/prompt/app.rs", "Emacs-oriented prompt output"),
            ],
            morphisms: vec![
                mor("render_cli", "SemanticTable", "CliText", "cliOut_py", "plain table renderer"),
                mor("render_html", "SemanticTable", "HtmlText", "python_html_meta", "HTML renderer/reference"),
                mor("render_prompt", "PromptOutput", "EmacsText", "print_output", "prompt display renderer"),
            ],
            implemented_by: vec!["src/reta_ausgabe".to_string(), "src/prompt".to_string()],
        },
    ];

    let functors = vec![
        FunctorSpec {
            name: "PyRetaArchToRustReta".to_string(),
            source_category: "OpenRetaContextCategory".to_string(),
            target_category: "OpenRetaContextCategory".to_string(),
            variance: "covariant".to_string(),
            object_map: BTreeMap::from([
                ("ContextSelection".to_string(), "ContextSelection".to_string()),
                ("RetaContextTopology".to_string(), "RetaContextTopology".to_string()),
            ]),
            morphism_map: BTreeMap::from([
                ("refine".to_string(), "refine".to_string()),
                ("open_for".to_string(), "open_for".to_string()),
            ]),
            code_owner: "crates/reta_architecture".to_string(),
            description: "Structure-preserving map from py reta arch topology to Rust topology.".to_string(),
        },
        FunctorSpec {
            name: "PromptToRetaExecution".to_string(),
            source_category: "PromptRuntimeCategory".to_string(),
            target_category: "ExecutionNetworkCategory".to_string(),
            variance: "covariant".to_string(),
            object_map: BTreeMap::from([
                ("PromptCommand".to_string(), "ExecutionTask".to_string()),
                ("PromptTokenStream".to_string(), "ExecutionTask".to_string()),
            ]),
            morphism_map: BTreeMap::from([
                ("compile_command".to_string(), "enqueue_task".to_string()),
                ("execute_command".to_string(), "dispatch_task".to_string()),
            ]),
            code_owner: "crates/retaprompt_*".to_string(),
            description: "Prompt commands become deterministic Reta execution tasks.".to_string(),
        },
        FunctorSpec {
            name: "SemanticTableToDisplay".to_string(),
            source_category: "SheafCategory".to_string(),
            target_category: "OutputFunctorCategory".to_string(),
            variance: "covariant".to_string(),
            object_map: BTreeMap::from([
                ("GluedSection".to_string(), "SemanticTable".to_string()),
                ("Sheaf".to_string(), "OutputFunctorCategory".to_string()),
            ]),
            morphism_map: BTreeMap::from([
                ("glue".to_string(), "render_cli".to_string()),
                ("is_compatible".to_string(), "render_html".to_string()),
            ]),
            code_owner: "src/reta_workflow_py.rs".to_string(),
            description: "A glued semantic table can be rendered by multiple output functors.".to_string(),
        },
    ];

    let natural_transformations = vec![
        NaturalTransformationSpec {
            name: "CliHtmlBbcodeEmacsOutputNaturality".to_string(),
            source_functor: "SemanticTableToDisplay".to_string(),
            target_functor: "SemanticTableToDisplay".to_string(),
            components: BTreeMap::from([
                ("CliText".to_string(), "render_cli".to_string()),
                ("HtmlText".to_string(), "render_html".to_string()),
                ("BbCodeText".to_string(), "render_bbcode".to_string()),
                ("EmacsText".to_string(), "render_prompt".to_string()),
            ]),
            naturality_condition: "Changing the renderer must not change selected semantic rows/columns.".to_string(),
            code_owner: "output parity tests".to_string(),
            description: "Natural transformation across output renderers.".to_string(),
        },
        NaturalTransformationSpec {
            name: "PyArchRustParity".to_string(),
            source_functor: "PyRetaArchToRustReta".to_string(),
            target_functor: "PyRetaArchToRustReta".to_string(),
            components: BTreeMap::from([
                ("ContextSelection".to_string(), "same restrictions".to_string()),
                ("ExecutionTask".to_string(), "same task order after deterministic reduce".to_string()),
            ]),
            naturality_condition: "Python architecture and Rust architecture commute on the same CLI/prompt context.".to_string(),
            code_owner: "tools/compare_reta.py + Rust architecture tests".to_string(),
            description: "Parity transformation for the staged transcompilation.".to_string(),
        },
    ];

    CategoryTheoryBundle {
        categories,
        functors,
        natural_transformations,
        paradigm_terms: vec![
            "topology".to_string(),
            "morphism".to_string(),
            "universal_property".to_string(),
            "presheaf".to_string(),
            "sheaf".to_string(),
            "category".to_string(),
            "functor".to_string(),
            "natural_transformation".to_string(),
        ],
        python_snapshot_digest_hint: format!(
            "python_snapshot_bytes={}",
            PYTHON_CATEGORY_THEORY_SNAPSHOT.as_bytes().len()
        ),
    }
}


/// Shared lookup helper matching Python `_find_by_name`.
pub fn _find_by_name<'a>(bundle: &'a CategoryTheoryBundle, name: &str, kind: &str) -> Option<&'a str> {
    match kind {
        "category" => bundle.category_named(name).map(|item| item.name.as_str()),
        "functor" => bundle.functor_named(name).map(|item| item.name.as_str()),
        "natural transformation" | "natural_transformation" => bundle.natural_transformation_named(name).map(|item| item.name.as_str()),
        _ => None,
    }
}

pub fn _paradigm_terms() -> Vec<ParadigmTermSpec> {
    vec![
        ParadigmTermSpec::new("topology", "open Reta contexts and refinements", &["topology.rs"], "ported"),
        ParadigmTermSpec::new("morphism", "typed semantic transitions", &["morphism.rs"], "ported"),
        ParadigmTermSpec::new("universal_property", "gluing invariants for local-to-global data", &["universal.rs", "dataflow.rs"], "ported"),
        ParadigmTermSpec::new("presheaf", "local context-indexed sections", &["presheaf.rs"], "ported"),
        ParadigmTermSpec::new("sheaf", "compatible glued semantic sections", &["sheaf.rs"], "ported"),
        ParadigmTermSpec::new("category", "metadata category layer", &["category.rs"], "ported"),
        ParadigmTermSpec::new("functor", "structure-preserving architecture maps", &["category.rs"], "ported"),
        ParadigmTermSpec::new("natural_transformation", "commuting parity/output diagrams", &["category.rs"], "ported"),
    ]
}

pub fn _natural_transformations() -> Vec<NaturalTransformationSpec> {
    bootstrap_category_theory().natural_transformations
}

pub fn _plan() -> Stage27ArchitecturePlan {
    Stage27ArchitecturePlan {
        planned_before_stage_27: vec!["make category/functor/natural-transformation layer explicit".to_string()],
        implemented_in_stage_27: vec!["category metadata".to_string(), "functor metadata".to_string(), "naturality metadata".to_string()],
        already_implemented_before_stage_27: vec!["topology".to_string(), "presheaf".to_string(), "sheaf".to_string(), "morphism".to_string()],
        behaviour_change: "none; metadata and validation only".to_string(),
    }
}

pub fn _stage32_categories() -> Vec<CategorySpec> {
    bootstrap_category_theory().categories
}

pub fn _stage32_functors() -> Vec<FunctorSpec> {
    bootstrap_category_theory().functors
}

pub fn _stage32_natural_transformations() -> Vec<NaturalTransformationSpec> {
    bootstrap_category_theory().natural_transformations
}

pub fn _stage33_categories() -> Vec<CategorySpec> {
    bootstrap_category_theory().categories
}

pub fn _stage33_functors() -> Vec<FunctorSpec> {
    bootstrap_category_theory().functors
}

pub fn _stage33_natural_transformations() -> Vec<NaturalTransformationSpec> {
    bootstrap_category_theory().natural_transformations
}

pub fn _stage34_categories() -> Vec<CategorySpec> {
    bootstrap_category_theory().categories
}

pub fn _stage34_functors() -> Vec<FunctorSpec> {
    bootstrap_category_theory().functors
}

pub fn _stage34_natural_transformations() -> Vec<NaturalTransformationSpec> {
    bootstrap_category_theory().natural_transformations
}

pub fn _stage35_categories() -> Vec<CategorySpec> {
    bootstrap_category_theory().categories
}

pub fn _stage35_functors() -> Vec<FunctorSpec> {
    bootstrap_category_theory().functors
}

pub fn _stage35_natural_transformations() -> Vec<NaturalTransformationSpec> {
    bootstrap_category_theory().natural_transformations
}

pub fn _stage36_categories() -> Vec<CategorySpec> {
    bootstrap_category_theory().categories
}

pub fn _stage36_functors() -> Vec<FunctorSpec> {
    bootstrap_category_theory().functors
}

pub fn _stage36_natural_transformations() -> Vec<NaturalTransformationSpec> {
    bootstrap_category_theory().natural_transformations
}

pub fn _stage37_categories() -> Vec<CategorySpec> {
    bootstrap_category_theory().categories
}

pub fn _stage37_functors() -> Vec<FunctorSpec> {
    bootstrap_category_theory().functors
}

pub fn _stage37_natural_transformations() -> Vec<NaturalTransformationSpec> {
    bootstrap_category_theory().natural_transformations
}

pub fn _stage38_categories() -> Vec<CategorySpec> {
    bootstrap_category_theory().categories
}

pub fn _stage38_functors() -> Vec<FunctorSpec> {
    bootstrap_category_theory().functors
}

pub fn _stage38_natural_transformations() -> Vec<NaturalTransformationSpec> {
    bootstrap_category_theory().natural_transformations
}

pub fn _stage39_categories() -> Vec<CategorySpec> {
    bootstrap_category_theory().categories
}

pub fn _stage39_functors() -> Vec<FunctorSpec> {
    bootstrap_category_theory().functors
}

pub fn _stage39_natural_transformations() -> Vec<NaturalTransformationSpec> {
    bootstrap_category_theory().natural_transformations
}

pub fn _stage40_categories() -> Vec<CategorySpec> {
    bootstrap_category_theory().categories
}

pub fn _stage40_functors() -> Vec<FunctorSpec> {
    bootstrap_category_theory().functors
}

pub fn _stage40_natural_transformations() -> Vec<NaturalTransformationSpec> {
    bootstrap_category_theory().natural_transformations
}

pub fn _stage41_categories() -> Vec<CategorySpec> {
    bootstrap_category_theory().categories
}

pub fn _stage41_functors() -> Vec<FunctorSpec> {
    bootstrap_category_theory().functors
}

pub fn _stage41_natural_transformations() -> Vec<NaturalTransformationSpec> {
    bootstrap_category_theory().natural_transformations
}

pub fn _stage43_categories() -> Vec<CategorySpec> {
    bootstrap_category_theory().categories
}

pub fn _stage43_functors() -> Vec<FunctorSpec> {
    bootstrap_category_theory().functors
}

pub fn _stage43_natural_transformations() -> Vec<NaturalTransformationSpec> {
    bootstrap_category_theory().natural_transformations
}

/// Python-level snapshot façade used by audit tooling.
pub fn snapshot() -> CategoryTheorySnapshot {
    bootstrap_category_theory().snapshot()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_contains_user_requested_architecture_terms() {
        let bundle = bootstrap_category_theory();
        assert!(bundle.category_named("ExecutionNetworkCategory").is_some());
        assert!(bundle.paradigm_terms.contains(&"presheaf".to_string()));
        assert!(bundle.paradigm_terms.contains(&"natural_transformation".to_string()));
        assert!(bundle.python_snapshot().contains("OpenRetaContextCategory"));
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// Marker-only names still need semantic Rust implementation before activation.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "_categories",
    "_functors",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}

// Stage 16 small-surface concrete wrappers.
pub fn _categories() -> Vec<CategorySpec> {
    _stage43_categories()
}

pub fn _functors() -> Vec<FunctorSpec> {
    _stage43_functors()
}
