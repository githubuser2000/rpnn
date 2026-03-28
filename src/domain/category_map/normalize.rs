use crate::domain::parser::legacy_cli_typed::fold_cli_case;

pub fn normalize_key(s: &str) -> String {
    fold_cli_case(s)
}

pub fn names_equal(left: &str, right: &str) -> bool {
    fold_cli_case(left) == fold_cli_case(right)
}
