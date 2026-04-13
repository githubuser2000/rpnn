use crate::input_help::input_validation::{validate_cli_structure, ValidationResult};
use crate::{run_reta_from_args, RetaRunResult};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedCli {
    pub main_parameters: Vec<String>,
    pub validation: ValidationResult,
}

pub fn parse_cli(argv: &[String]) -> ParsedCli {
    let argv_without_program = if argv.len() > 1 { &argv[1..] } else { &[] };
    let validation = validate_cli_structure(argv_without_program);
    ParsedCli {
        main_parameters: validation.seen_main_parameters.clone(),
        validation,
    }
}

pub fn run_cli(argv: Vec<String>) -> (ParsedCli, RetaRunResult) {
    let parsed = parse_cli(&argv);
    let result = run_reta_from_args(argv);
    (parsed, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cli_collects_main_parameters() {
        let argv = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-3".to_string(),
            "-ausgabe".to_string(),
            "--justtext".to_string(),
        ];
        let parsed = parse_cli(&argv);
        assert_eq!(parsed.main_parameters, vec!["zeilen".to_string(), "ausgabe".to_string()]);
    }
}
