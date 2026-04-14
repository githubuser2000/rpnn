#![allow(non_snake_case)]
pub mod shared;

pub mod runtime;
pub mod support;
pub mod doc_tools;
pub mod domain;
pub mod prompt;

use std::sync::OnceLock;

use shared::csv_loader_py::preload_common_csv_tables;
use shared::reta_py::Program;
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

pub fn shared_words() -> &'static Words {
    SHARED_WORDS.get_or_init(Words::new)
}


pub fn preload_reta_runtime() -> Result<(), String> {
    let words = shared_words();
    let _ = shared_reta_static_data(words);
    preload_common_csv_tables().map_err(|err| format!("reta-Runtime konnte nicht vorladen: {err}"))
}

pub fn run_reta_from_args(argv: Vec<String>) -> RetaRunResult {
    let mut program = Program::new(argv);
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
