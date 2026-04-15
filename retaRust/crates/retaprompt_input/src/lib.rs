#![allow(non_snake_case)]

//! Own-command-input layer for rp/rpl/rpe.
//!
//! This crate intentionally owns the interactive self-entered frontend side, but
//! it also depends on `retaprompt_commands` so the shared prompt command layer
//! remains the single common command API for all retaPrompt frontends.

pub use retaprompt_commands::{
    commands_text,
    compile_command,
    execute_command,
    help_text,
    profile_rp,
    profile_rpe,
    profile_rpl,
    EditModeKind,
    PromptCommand,
    PromptOutput,
    PromptModus,
    SessionState,
};

pub fn run_rp(argv: Vec<String>) -> i32 {
    reta::prompt::run_prompt_input_frontend_with_profile(argv, profile_rp())
}

pub fn run_rpl(argv: Vec<String>) -> i32 {
    reta::prompt::run_prompt_input_frontend_with_profile(argv, profile_rpl())
}

pub fn run_rpe(argv: Vec<String>) -> i32 {
    reta::prompt::run_prompt_input_frontend_with_profile(argv, profile_rpe())
}

pub fn run_rp_from_env() -> i32 {
    run_rp(std::env::args().collect())
}

pub fn run_rpl_from_env() -> i32 {
    run_rpl(std::env::args().collect())
}

pub fn run_rpe_from_env() -> i32 {
    run_rpe(std::env::args().collect())
}
