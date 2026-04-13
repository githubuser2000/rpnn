use crate::domain::python_source_of_truth::{
    all_parameter_main_alias_groups, exact_all_direct_columns_for_pair_alias,
    parameter_alias_groups_for_main_alias, ExactPythonColumn,
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
    pub fn new(
        parameter_main_name: impl Into<String>,
        parameter_name: impl Into<String>,
        parameter_main_aliases: Vec<String>,
        parameter_aliases: Vec<String>,
    ) -> Self {
        Self {
            parameter_main_name: parameter_main_name.into(),
            parameter_name: parameter_name.into(),
            parameter_main_aliases,
            parameter_aliases,
        }
    }

    pub fn cli_pair(&self) -> (String, String) {
        (self.parameter_main_name.clone(), self.parameter_name.clone())
    }

    pub fn exact_columns(&self, words: &Words) -> Vec<ExactPythonColumn> {
        exact_all_direct_columns_for_pair_alias(words, &self.parameter_main_name, &self.parameter_name)
    }
}

fn first_alias(group: &[String]) -> String {
    group.first().cloned().unwrap_or_default()
}

pub fn parse_spalten_anfrage(
    words: &Words,
    parameter_main_name: &str,
    parameter_name: &str,
) -> Result<SpaltenAnfrage, String> {
    let known_mains = all_parameter_main_alias_groups(words);
    let main_group = known_mains
        .iter()
        .find(|group| group.iter().any(|name| name == parameter_main_name))
        .cloned()
        .ok_or_else(|| format!("Unbekannte Oberkategorie: {}", parameter_main_name))?;

    let known_parameters = parameter_alias_groups_for_main_alias(words, parameter_main_name);
    let parameter_group = known_parameters
        .iter()
        .find(|group| group.iter().any(|name| name == parameter_name))
        .cloned()
        .ok_or_else(|| {
            format!(
                "Unbekannte Unterkategorie für {}: {}",
                parameter_main_name, parameter_name
            )
        })?;

    Ok(SpaltenAnfrage::new(
        first_alias(&main_group),
        first_alias(&parameter_group),
        main_group,
        parameter_group,
    ))
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
    fn parse_known_alias_pair_works() {
        let words = Words::new();
        let request = parse_spalten_anfrage(&words, "menschliches", "motive").unwrap();
        assert_eq!(request.cli_pair(), ("Menschliches".to_string(), "Motive".to_string()));
        assert!(request.parameter_main_aliases.iter().any(|value| value == "menschliches"));
        assert!(request.parameter_aliases.iter().any(|value| value == "motive"));
    }

    #[test]
    fn parse_unknown_main_fails() {
        let words = Words::new();
        assert!(parse_spalten_anfrage(&words, "does-not-exist", "x").is_err());
    }
}
