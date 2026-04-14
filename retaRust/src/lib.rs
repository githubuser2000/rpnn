#![allow(non_snake_case)]
pub mod shared;

pub mod runtime;
pub mod support;
pub mod doc_tools;
pub mod domain;
pub mod prompt;

use std::sync::OnceLock;

use shared::reta_program_types::Program;
use shared::reta_runtime_cache::shared_reta_static_data;
use shared::words_py::Words;

#[derive(Clone, Debug, Default)]
pub struct RetaRunResult {
    pub cli_errors: Vec<String>,
    pub display_lines: Vec<String>,
    pub snapshot: String,
}

impl RetaRunResult {
    pub fn render_text(&self) -> String {
        if !self.cli_errors.is_empty() {
            self.cli_errors.join("\n")
        } else if !self.display_lines.is_empty() {
            self.display_lines.join("\n")
        } else {
            self.snapshot.clone()
        }
    }

    pub fn exit_code(&self) -> i32 {
        if self.cli_errors.is_empty() { 0 } else { 1 }
    }
}

static SHARED_WORDS: OnceLock<Words> = OnceLock::new();
static SHARED_PROGRAM_TEMPLATE: OnceLock<Program> = OnceLock::new();
static SHARED_PRELOAD_RESULT: OnceLock<Result<(), String>> = OnceLock::new();

pub fn shared_words() -> &'static Words {
    SHARED_WORDS.get_or_init(Words::new)
}

fn shared_program_template() -> &'static Program {
    SHARED_PROGRAM_TEMPLATE.get_or_init(|| Program::new(vec!["reta".to_string()]))
}

pub fn fresh_program_from_template(argv: Vec<String>) -> Program {
    let mut program = shared_program_template().clone();
    program.argv = argv;
    program.argvWithoutProgram = if program.argv.len() > 1 {
        program.argv[1..].to_vec()
    } else {
        vec![]
    };
    program
}

pub fn preload_reta_runtime() -> Result<(), String> {
    SHARED_PRELOAD_RESULT
        .get_or_init(|| {
            let words = shared_words();
            let _ = shared_program_template();
            let _ = shared_reta_static_data(words);
            Ok(())
        })
        .clone()
}

pub fn run_reta_from_args(argv: Vec<String>) -> RetaRunResult {
    let _ = preload_reta_runtime();
    let mut program = fresh_program_from_template(argv);
    let words = shared_words();
    program.runAllesLikePythonInit(words);
    program.run(words);
    program.combiTableWorkflow();

    RetaRunResult {
        cli_errors: program.cliErrors.clone(),
        display_lines: program.finallyDisplayLines.clone(),
        snapshot: program.snapshot(),
    }
}

pub fn run_reta_from_env_args() -> RetaRunResult {
    let argv = std::env::args().collect::<Vec<_>>();
    run_reta_from_args(argv)
}

pub fn print_reta_result(result: &RetaRunResult) {
    println!("{}", result.render_text());
}

pub fn run_reta_and_print_from_env() -> i32 {
    let result = run_reta_from_env_args();
    let exit_code = result.exit_code();
    print_reta_result(&result);
    exit_code
}
