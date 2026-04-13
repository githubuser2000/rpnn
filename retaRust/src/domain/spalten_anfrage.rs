use crate::domain::python_source_of_truth::{
    all_parameter_main_names, exact_all_direct_columns_for_pair, parameter_names_for_main, ExactPythonColumn,
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
}

pub fn parse_spalten_anfrage(
    words: &Words,
    parameter_main_name: &str,
    parameter_name: &str,
) -> Result<SpaltenAnfrage, String> {
    let known_mains = all_parameter_main_names(words);
    if !known_mains.iter().any(|name| name == parameter_main_name) {
        return Err(format!("Unbekannte Oberkategorie: {}", parameter_main_name));
    }

    let known_parameters = parameter_names_for_main(words, parameter_main_name);
    if !known_parameters.iter().any(|name| name == parameter_name) {
        return Err(format!(
            "Unbekannte Unterkategorie für {}: {}",
            parameter_main_name, parameter_name
        ));
    }

    Ok(SpaltenAnfrage::new(parameter_main_name, parameter_name))
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
}
