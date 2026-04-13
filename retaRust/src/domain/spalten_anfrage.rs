use crate::domain::python_source_of_truth::{
    all_main_alias_names, canonicalize_pair, exact_all_direct_columns_for_pair, parameter_alias_names,
    ExactPythonColumn,
};
use crate::shared::words_py::Words;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpaltenAnfrage {
    pub parameter_main_name: String,
    pub parameter_name: String,
}

impl SpaltenAnfrage {
    pub fn new(parameter_main_name: impl Into<String>, parameter_name: impl Into<String>) -> Self {
        Self {
            parameter_main_name: parameter_main_name.into(),
            parameter_name: parameter_name.into(),
        }
    }

    pub fn cli_pair(&self) -> (String, String) {
        (self.parameter_main_name.clone(), self.parameter_name.clone())
    }

    pub fn exact_columns(&self, words: &Words) -> Vec<ExactPythonColumn> {
        exact_all_direct_columns_for_pair(words, &self.parameter_main_name, &self.parameter_name)
    }

    pub fn parameter_main_aliases(&self, words: &Words) -> Vec<String> {
        all_main_alias_names(words, &self.parameter_main_name)
    }

    pub fn parameter_aliases(&self, words: &Words) -> Vec<String> {
        parameter_alias_names(words, &self.parameter_main_name, &self.parameter_name)
    }

    pub fn exact_column_numbers(&self, words: &Words) -> Vec<i64> {
        let mut out = Vec::new();
        for entry in self.exact_columns(words) {
            for number in entry.column_numbers {
                if !out.contains(&number) {
                    out.push(number);
                }
            }
        }
        out
    }
}

pub fn parse_spalten_anfrage(
    words: &Words,
    parameter_main_name: &str,
    parameter_name: &str,
) -> Result<SpaltenAnfrage, String> {
    let (canonical_main, canonical_parameter) = canonicalize_pair(words, parameter_main_name, parameter_name)
        .ok_or_else(|| {
            format!(
                "Unbekannte Spaltenanfrage: {} / {}",
                parameter_main_name, parameter_name
            )
        })?;

    Ok(SpaltenAnfrage::new(canonical_main, canonical_parameter))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::words_py::Words;

    #[test]
    fn parse_known_pair_works() {
        let words = Words::new();
        let request = parse_spalten_anfrage(&words, "Menschliches", "Motive").unwrap();
        assert_eq!(request.cli_pair(), ("Menschliches".to_string(), "Motive".to_string()));
        assert!(!request.exact_columns(&words).is_empty());
    }

    #[test]
    fn parse_unknown_main_fails() {
        let words = Words::new();
        assert!(parse_spalten_anfrage(&words, "does-not-exist", "x").is_err());
    }

    #[test]
    fn parse_known_alias_pair_works() {
        let words = Words::new();
        let request = parse_spalten_anfrage(&words, "menschliches", "motive").unwrap();
        assert_eq!(request.cli_pair(), ("Menschliches".to_string(), "Motive".to_string()));
    }

    #[test]
    fn aliases_follow_canonical_request() {
        let words = Words::new();
        let request = parse_spalten_anfrage(&words, "menschliches", "motive").unwrap();
        assert!(request.parameter_main_aliases(&words).iter().any(|alias| alias == "menschliches"));
        assert!(request.parameter_aliases(&words).iter().any(|alias| alias.to_lowercase().contains("motiv")));
    }
}
