use crate::input_help::input_validation::{validate_cli_sequence, ValidationIssue};
use crate::{run_reta_from_args, RetaRunResult};

#[derive(Clone, Debug, Default)]
pub struct CliCall {
    pub argv: Vec<String>,
    pub main_parameters: Vec<String>,
    pub side_parameters: Vec<String>,
    pub validation_issues: Vec<ValidationIssue>,
}

pub fn split_main_and_side_args(argv: &[String]) -> CliCall {
    let mut main_parameters = Vec::new();
    let mut side_parameters = Vec::new();
    for arg in argv.iter().skip(1) {
        if arg.starts_with('-') && !arg.starts_with("--") {
            main_parameters.push(arg.clone());
        } else if arg.starts_with("--") {
            side_parameters.push(arg.clone());
        }
    }
    CliCall {
        argv: argv.to_vec(),
        main_parameters,
        side_parameters,
        validation_issues: validate_cli_sequence(argv),
    }
}

pub fn run_cli_call(argv: Vec<String>) -> RetaRunResult {
    run_reta_from_args(argv)
}
