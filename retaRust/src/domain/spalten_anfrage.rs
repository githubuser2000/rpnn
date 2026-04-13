use crate::domain::python_source_of_truth::{
    canonicalize_pair, column_numbers_for_pair, parameter_alias_names, resolve_parameter_main_alias,
};
use crate::shared::words_py::Words;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpaltenAnfrage {
    pub parameter_main: String,
    pub parameter: String,
}

impl SpaltenAnfrage {
    pub fn ober_unter_cli_pair(&self) -> (String, String) {
        (self.parameter_main.clone(), self.parameter.clone())
    }

    pub fn parameter_main_aliases(&self, words: &Words) -> Vec<String> {
        let canonical = resolve_parameter_main_alias(words, &self.parameter_main)
            .unwrap_or_else(|| self.parameter_main.clone());
        crate::domain::python_source_of_truth::all_main_alias_groups(words)
            .into_iter()
            .find(|g| g.canonical == canonical)
            .map(|g| g.aliases)
            .unwrap_or_default()
    }

    pub fn parameter_aliases(&self, words: &Words) -> Vec<String> {
        let canonical_main = resolve_parameter_main_alias(words, &self.parameter_main)
            .unwrap_or_else(|| self.parameter_main.clone());
        parameter_alias_names(words, &canonical_main)
    }

    pub fn exact_column_numbers(&self, words: &Words) -> Vec<i64> {
        column_numbers_for_pair(words, &self.parameter_main, &self.parameter)
    }
}

pub fn parse_spalten_anfrage(words: &Words, parameter_main: &str, parameter: &str) -> Result<SpaltenAnfrage, String> {
    let Some((canonical_main, canonical_parameter)) = canonicalize_pair(words, parameter_main, parameter) else {
        return Err(format!(
            "Unknown Python pair: {} / {}",
            parameter_main, parameter
        ));
    };
    Ok(SpaltenAnfrage {
        parameter_main: canonical_main,
        parameter: canonical_parameter,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::words_py::Words;

    #[test]
    fn parse_known_pair_works() {
        let words = Words::new();
        let req = parse_spalten_anfrage(&words, "Menschliches", "Motive").unwrap();
        assert_eq!(req.ober_unter_cli_pair().0, "Menschliches");
    }

    #[test]
    fn parse_known_alias_pair_works() {
        let words = Words::new();
        let req = parse_spalten_anfrage(&words, "menschliches", "motive").unwrap();
        assert_eq!(req.ober_unter_cli_pair().0, "Menschliches");
    }

    #[test]
    fn parse_unknown_main_fails() {
        let words = Words::new();
        assert!(parse_spalten_anfrage(&words, "unbekannt", "motive").is_err());
    }

    #[test]
    fn aliases_follow_canonical_request() {
        let words = Words::new();
        let req = parse_spalten_anfrage(&words, "menschliches", "motive").unwrap();
        assert!(req.parameter_main_aliases(&words).iter().any(|a| a == "menschliches"));
        assert!(req.parameter_aliases(&words).iter().any(|a| a.to_lowercase() == "motive"));
    }
}
