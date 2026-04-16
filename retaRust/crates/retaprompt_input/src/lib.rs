#![allow(non_snake_case)]

//! Own launcher/input layer for rp/rpl/rpe.
//!
//! This crate intentionally owns the Python-like launcher defaults for the
//! frontends that ship together (`rp`, `rpl`, `rpe`) and depends on
//! `retaprompt_commands` so the shared command/runtime layer remains the single
//! common API foundation for all retaPrompt frontends.
//!
//! It also provides the highest-level launcher-facing dispatch API: for `rp`,
//! `rpl`, and `rpe` it resolves the normal frontend profile directly, and for
//! `rpb` it forwards into `retaprompt_commands`. This keeps the executable
//! binary layer as small as possible while preserving the dependency direction
//! `retaprompt_input -> retaprompt_commands -> reta`.

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromptLauncherKind {
    Rp,
    Rpl,
    Rpb,
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

impl PromptLauncherKind {
    pub fn from_program_name(program_name: &str) -> Option<Self> {
        match PromptCommandFrontendKind::from_program_name(program_name) {
            Some(PromptCommandFrontendKind::Rp) => Some(Self::Rp),
            Some(PromptCommandFrontendKind::Rpl) => Some(Self::Rpl),
            Some(PromptCommandFrontendKind::Rpb) => Some(Self::Rpb),
            Some(PromptCommandFrontendKind::Rpe) => Some(Self::Rpe),
            None => None,
        }
    }

    pub fn from_argv(argv: &[String]) -> Option<Self> {
        argv.first().and_then(|arg0| Self::from_program_name(arg0))
    }

    pub fn from_abi_value(kind: i32) -> Option<Self> {
        match kind {
            1 => Some(Self::Rp),
            2 => Some(Self::Rpl),
            3 => Some(Self::Rpb),
            4 => Some(Self::Rpe),
            _ => None,
        }
    }
}

pub fn run_kind(argv: Vec<String>, kind: PromptInputFrontendKind) -> i32 {
    reta::prompt::run_prompt_frontend_with_profile(argv, kind.command_kind().profile())
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

pub fn run_launcher_kind(argv: Vec<String>, kind: PromptLauncherKind) -> i32 {
    match kind {
        PromptLauncherKind::Rp => run_kind(argv, PromptInputFrontendKind::Rp),
        PromptLauncherKind::Rpl => run_kind(argv, PromptInputFrontendKind::Rpl),
        PromptLauncherKind::Rpb => retaprompt_commands::run_kind(argv, PromptCommandFrontendKind::Rpb),
        PromptLauncherKind::Rpe => run_kind(argv, PromptInputFrontendKind::Rpe),
    }
}

pub fn run_launcher_kind_from_env(kind: PromptLauncherKind) -> i32 {
    run_launcher_kind(std::env::args().collect(), kind)
}

pub fn run_any_current_executable(argv: Vec<String>) -> i32 {
    match PromptLauncherKind::from_argv(&argv) {
        Some(kind) => run_launcher_kind(argv, kind),
        None => {
            let arg0 = argv.first().cloned().unwrap_or_else(|| "<unknown>".to_string());
            eprintln!(
                "retaprompt_input cannot infer launcher kind from executable name: {arg0}"
            );
            eprintln!("expected one of: rp, rpl, rpb, rpe");
            1
        }
    }
}

pub fn run_any_current_executable_from_env() -> i32 {
    run_any_current_executable(std::env::args().collect())
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

pub fn run_launcher_kind_from_abi_value(kind: i32) -> i32 {
    match PromptLauncherKind::from_abi_value(kind) {
        Some(kind) => run_launcher_kind_from_env(kind),
        None => {
            eprintln!("invalid retaprompt launcher kind: {kind}");
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
pub extern "C" fn retaprompt_input_run_any_current_executable_from_env() -> i32 {
    run_any_current_executable_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_launcher_kind_from_env(kind: i32) -> i32 {
    run_launcher_kind_from_abi_value(kind)
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


