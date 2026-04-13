const KNOWN_MAIN_PARAMETERS: &[&str] = &["zeilen", "spalten", "kombination", "ausgabe", "debug", "h", "help"];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationResult {
    pub seen_main_parameters: Vec<String>,
    pub errors: Vec<String>,
}

impl ValidationResult {
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

pub fn validate_cli_structure(argv_without_program: &[String]) -> ValidationResult {
    let mut seen_main_parameters = Vec::new();
    let mut errors = Vec::new();
    let mut last_main_parameter: Option<String> = None;

    for token in argv_without_program {
        if token.starts_with("--") {
            if last_main_parameter.is_none() {
                errors.push(format!("Nebenparameter ohne Hauptparameter: {}", token));
            }
            continue;
        }
        if token.starts_with('-') {
            let cmd = token.trim_start_matches('-').to_string();
            if KNOWN_MAIN_PARAMETERS.iter().any(|known| *known == cmd) {
                seen_main_parameters.push(cmd.clone());
                last_main_parameter = Some(cmd);
            } else {
                errors.push(format!("Unbekannter Hauptparameter: {}", token));
                last_main_parameter = None;
            }
        }
    }

    ValidationResult {
        seen_main_parameters,
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_main_side_sequence_is_ok() {
        let argv = vec![
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-3".to_string(),
            "-spalten".to_string(),
            "--alles".to_string(),
        ];
        let result = validate_cli_structure(&argv);
        assert!(result.is_ok());
    }

    #[test]
    fn side_parameter_without_main_is_reported() {
        let argv = vec!["--alles".to_string()];
        let result = validate_cli_structure(&argv);
        assert!(!result.is_ok());
    }
}
