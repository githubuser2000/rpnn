use crate::domain::parser::legacy_cli_typed::{fold_cli_case, matches_any_alias};

pub fn normalize_category_key(s: &str) -> String {
    fold_cli_case(s)
}

pub fn contains_any_alias(token: &str, aliases: &[&str]) -> bool {
    matches_any_alias(token, aliases)
}

pub fn match_any_alias(token: &str, aliases: &[&str]) -> bool {
    matches_any_alias(token, aliases)
}
