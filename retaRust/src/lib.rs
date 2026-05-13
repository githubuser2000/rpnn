pub mod doc_tools;
pub mod domain;
pub mod runtime;
pub mod shared;

pub mod reta_begin_py;
pub mod reta_output_py;
pub mod reta_program_types;
pub mod reta_resulting_table_py;
pub mod reta_spalten_py;
pub mod reta_workflow_py;

mod reta_runtime_bridge;

pub mod ffi;

pub use reta_architecture as architecture;

#[path = "prompt/semantic_choices.rs"]
pub mod semantic_choices;

use std::sync::OnceLock;

use crate::shared::reta_program_types::Program;

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

static SHARED_WORDS: OnceLock<crate::shared::words_py::Words> = OnceLock::new();
static SHARED_PROGRAM_TEMPLATE: OnceLock<Program> = OnceLock::new();
static SHARED_PRELOAD_RESULT: OnceLock<Result<(), String>> = OnceLock::new();
static SHARED_ARCHITECTURE: OnceLock<reta_architecture::ArchitectureRuntime> = OnceLock::new();

pub fn shared_architecture() -> &'static reta_architecture::ArchitectureRuntime {
    SHARED_ARCHITECTURE.get_or_init(reta_architecture::bootstrap_architecture_runtime)
}

pub fn shared_words() -> &'static crate::shared::words_py::Words {
    SHARED_WORDS.get_or_init(crate::shared::words_py::Words::new)
}

fn shared_program_template() -> &'static Program {
    SHARED_PROGRAM_TEMPLATE.get_or_init(|| Program::new(vec!["reta".to_string()]))
}

pub fn fresh_program_from_template(argv: Vec<String>) -> Program {
    let mut program = shared_program_template().clone();
    program.argv = argv.clone();
    program.argvWithoutProgram = if argv.len() > 1 {
        argv[1..].to_vec()
    } else {
        vec![]
    };
    program
}

pub fn preload_reta_runtime() -> Result<(), String> {
    SHARED_PRELOAD_RESULT
        .get_or_init(|| {
            let _ = shared_words();
            let _ = shared_architecture();
            Ok(())
        })
        .clone()
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

pub fn run_reta_from_env_args() -> RetaRunResult {
    let argv = std::env::args().collect::<Vec<_>>();
    run_reta_from_args(argv)
}

pub fn print_reta_result(result: &RetaRunResult) {
    eprint!("{}", result.stderr);
    print!("{}", result.stdout);
}

pub fn run_reta_and_print_from_env() -> i32 {
    let result = run_reta_from_env_args();
    let exit_code = result.exit_code();
    print_reta_result(&result);
    exit_code
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_run_and_print_from_env_ffi() -> i32 {
    match std::panic::catch_unwind(run_reta_and_print_from_env) {
        Ok(exit_code) => exit_code,
        Err(_) => {
            eprintln!("panic inside reta_run_and_print_from_env_ffi");
            101
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RetaRunResult;

    #[test]
    fn render_text_empty_response_stays_empty() {
        assert_eq!(RetaRunResult::default().render_text(), "");
    }

    #[test]
    fn render_text_keeps_stdout_bytes_without_extra_newline() {
        let result = RetaRunResult {
            stdout: "alpha\n".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };
        assert_eq!(result.render_text(), "alpha\n");
    }

    #[test]
    fn render_text_separates_stdout_and_stderr_only_when_needed() {
        let result = RetaRunResult {
            stdout: "alpha".to_string(),
            stderr: "beta\n".to_string(),
            exit_code: 1,
        };
        assert_eq!(result.render_text(), "alpha\nbeta\n");
    }
}
