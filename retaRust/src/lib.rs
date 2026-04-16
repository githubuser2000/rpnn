pub mod domain;
pub mod doc_tools;
pub mod prompt;
pub mod runtime;
pub mod shared;

pub mod reta_begin_py;
pub mod reta_output_py;
pub mod reta_program_types;
pub mod reta_resulting_table_py;
pub mod reta_spalten_py;
pub mod reta_workflow_py;

pub mod ffi;

use std::sync::OnceLock;

pub use crate::reta_begin_py::{build_cli_request, parse_cli_options};
pub use crate::reta_program_types::{
    DiagnosticLevel, ResultingTable, RetaDiagnostic, RetaError, RetaInput, RetaMetadata,
    RetaOptions, RetaRequest, RetaResponse, RetaRuntime,
};
pub use crate::reta_workflow_py::run_reta;

#[derive(Debug, Clone, Default)]
pub struct RetaRunResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl RetaRunResult {
    pub fn render_text(&self) -> String {
        match (self.stdout.is_empty(), self.stderr.is_empty()) {
            (true, true) => String::new(),
            (false, true) => self.stdout.clone(),
            (true, false) => self.stderr.clone(),
            (false, false) => {
                let mut combined = String::with_capacity(self.stdout.len() + self.stderr.len() + 1);
                combined.push_str(&self.stdout);
                if !combined.ends_with('\n') && !self.stderr.starts_with('\n') {
                    combined.push('\n');
                }
                combined.push_str(&self.stderr);
                combined
            }
        }
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }
}

impl From<RetaResponse> for RetaRunResult {
    fn from(response: RetaResponse) -> Self {
        Self {
            stdout: response.rendered_text,
            stderr: response.stderr_text,
            exit_code: response.exit_code,
        }
    }
}

pub fn run_reta_from_args<A>(argv: A) -> RetaRunResult
where
    A: AsRef<[String]>,
{
    run_reta_from_args_with_runtime(argv, None, RetaRuntime::default())
}

pub fn run_reta_from_args_with_runtime<A>(
    argv: A,
    stdin_text: Option<String>,
    runtime: RetaRuntime,
) -> RetaRunResult
where
    A: AsRef<[String]>,
{
    let request = build_cli_request(argv.as_ref(), stdin_text, runtime);
    match run_reta(request) {
        Ok(response) => response.into(),
        Err(error) => RetaRunResult {
            stdout: String::new(),
            stderr: format!("{error}\n"),
            exit_code: error.exit_code(),
        },
    }
}

pub fn shared_words() -> &'static crate::shared::words_py::Words {
    static WORDS: OnceLock<crate::shared::words_py::Words> = OnceLock::new();
    WORDS.get_or_init(|| crate::shared::words_py::Words::new())
}
