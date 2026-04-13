use crate::domain::python_source_of_truth::{
    all_parameter_main_names, canonicalize_pair, exact_all_direct_columns_for_pair, parameter_names_for_main,
    parameter_alias_groups_for_main, ExactPythonColumn,
};
use crate::shared::words_py::Words;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpaltenAnfrage {
    pub parameter_main_name: String,
    pub parameter_name: String,
    pub parameter_main_aliases: Vec<String>,
    pub parameter_aliases: Vec<String>,
}

impl SpaltenAnfrage {
    pub fn new(parameter_main_name: impl Into<String>, parameter_name: impl Into<String>) -> Self {
        let parameter_main_name = parameter_main_name.into();
        let parameter_name = parameter_name.into();
        Self {
            parameter_main_name,
            parameter_name,
            parameter_main_aliases: Vec::new(),
            parameter_aliases: Vec::new(),
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
    let Some((canonical_main, canonical_parameter)) = canonicalize_pair(words, parameter_main_name, parameter_name) else {
        if !known_mains.iter().any(|name| name == parameter_main_name) {
            return Err(format!("Unbekannte Oberkategorie: {}", parameter_main_name));
        }
        let known_parameters = parameter_names_for_main(words, parameter_main_name);
        return Err(format!(
            "Unbekannte Unterkategorie für {}: {}. Bekannt: {}",
            parameter_main_name,
            parameter_name,
            known_parameters.join(", ")
        ));
    };

    let parameter_aliases = parameter_alias_groups_for_main(words, &canonical_main)
        .into_iter()
        .find(|group| group.first() == Some(&canonical_parameter))
        .unwrap_or_else(|| vec![canonical_parameter.clone()]);

    Ok(SpaltenAnfrage {
        parameter_main_name: canonical_main.clone(),
        parameter_name: canonical_parameter,
        parameter_main_aliases: vec![canonical_main],
        parameter_aliases,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::words_py::Words;

    #[test]
    fn parse_known_pair_works() {
        let words = Words::new();
        let parsed = parse_spalten_anfrage(&words, "Menschliches", "Motive").expect("known pair should parse");
        assert_eq!(parsed.cli_pair(), ("Menschliches".to_string(), "Motive".to_string()));
    }

    #[test]
    fn parse_known_alias_pair_works() {
        let words = Words::new();
        let parsed = parse_spalten_anfrage(&words, "menschliches", "motive").expect("known alias pair should parse");
        assert_eq!(parsed.parameter_main_name, "Menschliches");
        assert_eq!(parsed.parameter_name, "Motive");
    }

    #[test]
    fn parse_unknown_main_fails() {
        let words = Words::new();
        let err = parse_spalten_anfrage(&words, "Unbekannt", "Motive").expect_err("unknown main should fail");
        assert!(err.contains("Unbekannte Oberkategorie"));
    }
}
