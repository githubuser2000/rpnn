pub mod categories;
pub mod exact_generator_bridge;
pub mod exact_mappings;
pub mod generator_registry;
pub mod pypy_compat;
pub mod reverse_request_report;
pub mod selection_state;
pub mod tabellen_utils;
pub mod generator_logic;
pub mod category_map;

pub mod python_source_of_truth;
pub mod decl_model;
pub mod html_meta_builder;
pub mod html_header_model;

pub mod errors;
pub mod indices;
pub mod spalten_anfrage;
pub mod request_pipeline;
pub mod request_bridge;
pub mod python_html_meta;
pub mod resolve_cli_legacy_adapter;

pub mod ids {
    pub mod domain_id;
}

pub mod model {
    pub mod spalten_anfrage;
}

pub mod parser {
    pub mod cli_alias_parser;
}

pub mod resolver {
    pub mod request_resolver;
    pub mod resolve_cli;
}
