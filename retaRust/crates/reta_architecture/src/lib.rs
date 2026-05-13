#![allow(non_snake_case)]

//! Shared typed architecture layer for `rreta` and `rretaPrompt`.
//!
//! This crate is the first Rust transcompilation layer for the modular
//! `py reta arch` source.  It deliberately keeps visible Reta behaviour stable:
//! the old output/parity code still renders.  The new structures make the
//! architecture explicit so later ports can target Rust modules instead of the
//! historical Python monolith.

pub mod category;
pub mod dataflow;
pub mod facade;
pub mod morphism;
pub mod presheaf;
pub mod sheaf;
pub mod topology;
pub mod universal;

pub use category::{
    bootstrap_category_theory, CategoryMorphismSpec, CategoryObjectSpec, CategorySpec,
    CategoryTheoryBundle, FunctorSpec, NaturalTransformationSpec,
    PYTHON_CATEGORY_THEORY_SNAPSHOT,
};
pub use dataflow::{
    bootstrap_execution_network, deterministic_reduce, execute_tasks_deterministically,
    DataflowDiscipline, ExecutionNetworkBundle, ExecutionNetworkConfig, ExecutionResult,
    ExecutionRunResult, ExecutionTask, FifoTaskQueue, FullDuplexChannel, HalfDuplexChannel,
    LifoTaskStack, PriorityTaskQueue, ResourceSemaphore, EXECUTION_NETWORK_SNAPSHOT,
};
pub use facade::{
    bootstrap_architecture_runtime, ArchitectureRuntime, ArchitectureSnapshotRef,
    PromptArchitectureContext, RetaRunArchitecture, ARCHITECTURE_COUNTS_SNAPSHOT,
};
pub use morphism::{MorphismEdge, MorphismGraph, MorphismKind};
pub use presheaf::{LocalSection, Presheaf, PresheafBundle};
pub use sheaf::{GluedSection, Sheaf, SheafBundle};
pub use topology::{ContextDimension, ContextSelection, RetaContextTopology};
pub use universal::{merge_parameter_dicts, normalize_column_buckets, UniversalBundle};
