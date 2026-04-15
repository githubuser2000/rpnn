#![allow(non_snake_case)]

//! Command-topic layer for rp/rpl/rpe/rpb.
//!
//! This crate intentionally does not expose the interactive self-input frontend
//! API from `retaprompt_input`. It exposes command compilation/execution and
//! direct command entry for rp/rpl/rpe/rpb.

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
pub use reta::prompt::frontend_profile::PromptFrontendProfile;
pub use reta::prompt::python_like::PromptModus;

pub fn profile_rp() -> PromptFrontendProfile {
    reta::prompt::frontend_profile::PromptFrontendProfile::rp()
}

pub fn profile_rpl() -> PromptFrontendProfile {
    reta::prompt::frontend_profile::PromptFrontendProfile::rpl()
}

pub fn profile_rpb() -> PromptFrontendProfile {
    reta::prompt::frontend_profile::PromptFrontendProfile::rpb()
}

pub fn profile_rpe() -> PromptFrontendProfile {
    reta::prompt::frontend_profile::PromptFrontendProfile::rpe()
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

pub fn run_rp(argv: Vec<String>) -> i32 {
    reta::prompt::run_prompt_command_frontend_with_profile(
        argv,
        profile_rp(),
    )
}

pub fn run_rpl(argv: Vec<String>) -> i32 {
    reta::prompt::run_prompt_command_frontend_with_profile(
        argv,
        profile_rpl(),
    )
}

pub fn run_rpb(argv: Vec<String>) -> i32 {
    reta::prompt::run_prompt_command_frontend_with_profile(
        argv,
        profile_rpb(),
    )
}

pub fn run_rpe(argv: Vec<String>) -> i32 {
    reta::prompt::run_prompt_command_frontend_with_profile(
        argv,
        profile_rpe(),
    )
}

pub fn run_rp_from_env() -> i32 {
    reta::prompt::run_command_rp_from_env()
}

pub fn run_rpl_from_env() -> i32 {
    reta::prompt::run_command_rpl_from_env()
}

pub fn run_rpb_from_env() -> i32 {
    reta::prompt::run_command_rpb_from_env()
}

pub fn run_rpe_from_env() -> i32 {
    reta::prompt::run_command_rpe_from_env()
}
