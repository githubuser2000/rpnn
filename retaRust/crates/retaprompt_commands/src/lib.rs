#![allow(non_snake_case)]

pub use reta::prompt::commands::{
    commands_text,
    compile_command,
    execute_command,
    help_text,
    EditModeKind,
    PromptCommand,
    PromptOutput,
    SessionState,
};
pub use reta::prompt::frontend_profile::{PromptFrontendKind, PromptFrontendProfile};
pub use reta::prompt::python_like::PromptModus;

pub fn run_with_profile(argv: Vec<String>, profile: PromptFrontendProfile) -> i32 {
    reta::prompt::retapromptlib::run_retaprompt_with_profile(argv, profile)
}

pub fn run_with_kind(argv: Vec<String>, kind: PromptFrontendKind) -> i32 {
    reta::prompt::retapromptlib::run_retaprompt_with_kind(argv, kind)
}

pub fn run_rp(argv: Vec<String>) -> i32 {
    run_with_kind(argv, PromptFrontendKind::Rp)
}

pub fn run_rpl(argv: Vec<String>) -> i32 {
    run_with_kind(argv, PromptFrontendKind::Rpl)
}

pub fn run_rpb(argv: Vec<String>) -> i32 {
    run_with_kind(argv, PromptFrontendKind::Rpb)
}

pub fn run_rpe(argv: Vec<String>) -> i32 {
    run_with_kind(argv, PromptFrontendKind::Rpe)
}

pub fn run_rp_from_env() -> i32 {
    run_rp(std::env::args().collect())
}

pub fn run_rpl_from_env() -> i32 {
    run_rpl(std::env::args().collect())
}

pub fn run_rpb_from_env() -> i32 {
    run_rpb(std::env::args().collect())
}

pub fn run_rpe_from_env() -> i32 {
    run_rpe(std::env::args().collect())
}

pub fn compile_for_rp(input: &str) -> Result<PromptCommand, String> {
    compile_command(input, PromptModus::Normal)
}

pub fn compile_for_rpl(input: &str) -> Result<PromptCommand, String> {
    compile_command(input, PromptModus::Normal)
}

pub fn compile_for_rpb(input: &str) -> Result<PromptCommand, String> {
    compile_command(input, PromptModus::Normal)
}

pub fn compile_for_rpe(input: &str) -> Result<PromptCommand, String> {
    compile_command(input, PromptModus::Normal)
}
