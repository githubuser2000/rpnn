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
pub mod row_ranges;
pub mod sheaf;
pub mod tag_schema;
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
pub use row_ranges::{
    bootstrap_row_range_morphisms, is_fraction_or_integer_range, is_fraction_range,
    is_fraction_range_token, is_integer_range_token, is_row_range, is_row_range_token,
    range_to_numbers, str_as_generator_to_set, RowRangeMorphismBundle, RowRangeSyntax,
};
pub use sheaf::{GluedSection, Sheaf, SheafBundle};
pub use tag_schema::{
    bootstrap_tag_schema, columns_for_tags_in_selector, kombi_table2_tags_for_column,
    kombi_table_tags_for_column, ordinary_columns_for_tags, ordinary_tags_for_column,
    reverse_map_for_selector, groups_for_selector, TagGroup, TagSchemaBundle,
    TagSchemaSnapshot, TagTableSelector, TableTag, KOMBI_TABLE2_TAG_GROUPS,
    KOMBI_TABLE_TAG_GROUPS, ORDINARY_TAG_GROUPS,
};
pub use topology::{ContextDimension, ContextSelection, RetaContextTopology};
pub use universal::{merge_parameter_dicts, normalize_column_buckets, UniversalBundle};
