// file: column_manager/mod.rs
mod column_query_builder;
mod column_selector;
pub(crate) use column_selector::collect_spalten_nummern;
mod row_query_builder;
mod validation;

pub use column_query_builder::build_column_query;
pub use column_selector::get_column_names;
