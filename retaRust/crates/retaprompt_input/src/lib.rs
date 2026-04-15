#![allow(non_snake_case)]

//! Own-command-input layer for rp/rpl/rpe.
//!
//! This crate intentionally owns the interactive self-entered frontend side and
//! depends on `retaprompt_commands` so the shared command/runtime layer remains
//! the single common API foundation for all retaPrompt frontends.

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
    PromptCommandFrontendKind,
    PromptOutput,
    PromptModus,
    SessionState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromptInputFrontendKind {
    Rp,
    Rpl,
    Rpe,
}

impl PromptInputFrontendKind {
    pub fn from_abi_value(kind: i32) -> Option<Self> {
        match kind {
            1 => Some(Self::Rp),
            2 => Some(Self::Rpl),
            4 => Some(Self::Rpe),
            _ => None,
        }
    }

    pub fn from_command_kind(kind: PromptCommandFrontendKind) -> Option<Self> {
        match kind {
            PromptCommandFrontendKind::Rp => Some(Self::Rp),
            PromptCommandFrontendKind::Rpl => Some(Self::Rpl),
            PromptCommandFrontendKind::Rpe => Some(Self::Rpe),
            PromptCommandFrontendKind::Rpb => None,
        }
    }

    pub fn from_program_name(program_name: &str) -> Option<Self> {
        PromptCommandFrontendKind::from_program_name(program_name)
            .and_then(Self::from_command_kind)
    }

    pub fn from_argv(argv: &[String]) -> Option<Self> {
        PromptCommandFrontendKind::from_argv(argv).and_then(Self::from_command_kind)
    }

    pub fn command_kind(self) -> PromptCommandFrontendKind {
        match self {
            Self::Rp => PromptCommandFrontendKind::Rp,
            Self::Rpl => PromptCommandFrontendKind::Rpl,
            Self::Rpe => PromptCommandFrontendKind::Rpe,
        }
    }
}

pub fn run_kind(argv: Vec<String>, kind: PromptInputFrontendKind) -> i32 {
    reta::prompt::run_prompt_input_frontend_with_profile(argv, kind.command_kind().profile())
}

pub fn run_kind_from_env(kind: PromptInputFrontendKind) -> i32 {
    run_kind(std::env::args().collect(), kind)
}

pub fn run_current_executable(argv: Vec<String>) -> i32 {
    match PromptInputFrontendKind::from_argv(&argv) {
        Some(kind) => run_kind(argv, kind),
        None => {
            let arg0 = argv.first().cloned().unwrap_or_else(|| "<unknown>".to_string());
            eprintln!(
                "retaprompt_input cannot infer input frontend kind from executable name: {arg0}"
            );
            1
        }
    }
}

pub fn run_current_executable_from_env() -> i32 {
    run_current_executable(std::env::args().collect())
}

pub fn run_rp(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptInputFrontendKind::Rp)
}

pub fn run_rpl(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptInputFrontendKind::Rpl)
}

pub fn run_rpe(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptInputFrontendKind::Rpe)
}

pub fn run_rp_from_env() -> i32 {
    run_kind_from_env(PromptInputFrontendKind::Rp)
}

pub fn run_rpl_from_env() -> i32 {
    run_kind_from_env(PromptInputFrontendKind::Rpl)
}

pub fn run_rpe_from_env() -> i32 {
    run_kind_from_env(PromptInputFrontendKind::Rpe)
}

pub fn run_kind_from_abi_value(kind: i32) -> i32 {
    match PromptInputFrontendKind::from_abi_value(kind) {
        Some(kind) => run_kind_from_env(kind),
        None => {
            eprintln!("invalid retaprompt input kind: {kind}");
            1
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_kind_from_env(kind: i32) -> i32 {
    run_kind_from_abi_value(kind)
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_current_executable_from_env() -> i32 {
    run_current_executable_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_rp_from_env() -> i32 {
    run_rp_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_rpl_from_env() -> i32 {
    run_rpl_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_rpe_from_env() -> i32 {
    run_rpe_from_env()
}
