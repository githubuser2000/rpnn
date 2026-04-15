#![allow(non_snake_case)]

//! Command-topic layer for rp/rpl/rpe/rpb.
//!
//! This crate intentionally does not expose the interactive self-input frontend
//! API from `retaprompt_input`. It centralizes the command-facing runtime for
//! all retaPrompt binaries and keeps the thin frontend executables minimal.

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromptCommandFrontendKind {
    Rp,
    Rpl,
    Rpb,
    Rpe,
}

impl PromptCommandFrontendKind {
    pub fn from_abi_value(kind: i32) -> Option<Self> {
        match kind {
            1 => Some(Self::Rp),
            2 => Some(Self::Rpl),
            3 => Some(Self::Rpb),
            4 => Some(Self::Rpe),
            _ => None,
        }
    }

    pub fn from_program_name(program_name: &str) -> Option<Self> {
        let base = std::path::Path::new(program_name)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(program_name);
        match base {
            "rp" => Some(Self::Rp),
            "rpl" => Some(Self::Rpl),
            "rpb" => Some(Self::Rpb),
            "rpe" => Some(Self::Rpe),
            _ => None,
        }
    }

    pub fn from_argv(argv: &[String]) -> Option<Self> {
        argv.first().and_then(|arg0| Self::from_program_name(arg0))
    }

    pub fn profile(self) -> PromptFrontendProfile {
        match self {
            Self::Rp => PromptFrontendProfile::rp(),
            Self::Rpl => PromptFrontendProfile::rpl(),
            Self::Rpb => PromptFrontendProfile::rpb(),
            Self::Rpe => PromptFrontendProfile::rpe(),
        }
    }
}

pub fn profile_rp() -> PromptFrontendProfile {
    PromptCommandFrontendKind::Rp.profile()
}

pub fn profile_rpl() -> PromptFrontendProfile {
    PromptCommandFrontendKind::Rpl.profile()
}

pub fn profile_rpb() -> PromptFrontendProfile {
    PromptCommandFrontendKind::Rpb.profile()
}

pub fn profile_rpe() -> PromptFrontendProfile {
    PromptCommandFrontendKind::Rpe.profile()
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

pub fn run_kind(argv: Vec<String>, kind: PromptCommandFrontendKind) -> i32 {
    reta::prompt::run_prompt_command_frontend_with_profile(argv, kind.profile())
}

pub fn run_kind_from_env(kind: PromptCommandFrontendKind) -> i32 {
    run_kind(std::env::args().collect(), kind)
}

pub fn run_current_executable(argv: Vec<String>) -> i32 {
    match PromptCommandFrontendKind::from_argv(&argv) {
        Some(kind) => run_kind(argv, kind),
        None => {
            let arg0 = argv.first().cloned().unwrap_or_else(|| "<unknown>".to_string());
            eprintln!(
                "retaprompt_commands cannot infer frontend kind from executable name: {arg0}"
            );
            1
        }
    }
}

pub fn run_current_executable_from_env() -> i32 {
    run_current_executable(std::env::args().collect())
}

pub fn run_rp(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptCommandFrontendKind::Rp)
}

pub fn run_rpl(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptCommandFrontendKind::Rpl)
}

pub fn run_rpb(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptCommandFrontendKind::Rpb)
}

pub fn run_rpe(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptCommandFrontendKind::Rpe)
}

pub fn run_rp_from_env() -> i32 {
    run_kind_from_env(PromptCommandFrontendKind::Rp)
}

pub fn run_rpl_from_env() -> i32 {
    run_kind_from_env(PromptCommandFrontendKind::Rpl)
}

pub fn run_rpb_from_env() -> i32 {
    run_kind_from_env(PromptCommandFrontendKind::Rpb)
}

pub fn run_rpe_from_env() -> i32 {
    run_kind_from_env(PromptCommandFrontendKind::Rpe)
}

pub fn run_kind_from_abi_value(kind: i32) -> i32 {
    match PromptCommandFrontendKind::from_abi_value(kind) {
        Some(kind) => run_kind_from_env(kind),
        None => {
            eprintln!("invalid retaprompt command kind: {kind}");
            1
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_run_kind_from_env(kind: i32) -> i32 {
    run_kind_from_abi_value(kind)
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_run_current_executable_from_env() -> i32 {
    run_current_executable_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_run_rp_from_env() -> i32 {
    run_rp_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_run_rpl_from_env() -> i32 {
    run_rpl_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_run_rpb_from_env() -> i32 {
    run_rpb_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_run_rpe_from_env() -> i32 {
    run_rpe_from_env()
}
