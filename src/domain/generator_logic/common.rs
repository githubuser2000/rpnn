use std::collections::BTreeSet;

pub fn normalize_token(s: &str) -> String { s.trim().to_lowercase() }

pub fn contains_any_alias(tokens: &BTreeSet<String>, aliases: &[&str]) -> bool {
    aliases.iter().any(|alias| tokens.contains(&normalize_token(alias)))
}

pub fn selected_by_pair(
    tokens: &BTreeSet<String>,
    first_aliases: &[&str],
    second_aliases: &[&str],
) -> bool {
    contains_any_alias(tokens, first_aliases) && contains_any_alias(tokens, second_aliases)
}
