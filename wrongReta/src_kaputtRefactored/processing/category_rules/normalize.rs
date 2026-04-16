pub fn normalize_category_key(s: &str) -> String {
    s.to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
}

pub fn contains_any_alias(token: &str, aliases: &[&str]) -> bool {
    let t = normalize_category_key(token);
    aliases.iter().any(|a| normalize_category_key(a) == t)
}

pub fn match_any_alias(token: &str, aliases: &[&str]) -> bool {
    aliases.iter().any(|a| normalize_category_key(a) == token)
}
