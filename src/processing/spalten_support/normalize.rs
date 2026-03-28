use crate::domain::parser::legacy_cli_typed::{fold_cli_case, matches_any_alias, LegacyOberToken};

pub fn normalize_category_key(s: &str) -> String {
    fold_cli_case(s)
}

pub fn is_primzahlkreuz_pro_contra_request(ober: &str, unter: &str) -> bool {
    let ober = LegacyOberToken::parse(ober);

    matches!(ober, LegacyOberToken::Bedeutung | LegacyOberToken::ProContra | LegacyOberToken::Universum)
        && matches_any_alias(unter, &["Primzahlkreuz", "primzahlkreuz", "primzahlkreuzprocontra"])
}
