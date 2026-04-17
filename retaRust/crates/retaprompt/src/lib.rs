#![allow(non_snake_case)]

pub use retaprompt_commands::{PromptFrontendKind, PromptFrontendProfile};

pub fn run_with_kind(argv: Vec<String>, kind: PromptFrontendKind) -> i32 {
    match kind {
        PromptFrontendKind::Auto => retaprompt_commands::run_prompt_frontend(argv, true),
        _ => retaprompt_commands::run_prompt_frontend_with_profile(
            argv,
            PromptFrontendProfile::for_kind(kind, true),
        ),
    }
}

pub fn run_with_profile(argv: Vec<String>, profile: PromptFrontendProfile) -> i32 {
    retaprompt_commands::run_prompt_frontend_with_profile(argv, profile)
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

pub fn run_auto_from_env() -> i32 {
    retaprompt_commands::run_prompt_frontend_from_env(true)
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

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_run_kind_from_env(kind: i32) -> i32 {
    let kind = match kind {
        1 => PromptFrontendKind::Rp,
        2 => PromptFrontendKind::Rpl,
        3 => PromptFrontendKind::Rpb,
        4 => PromptFrontendKind::Rpe,
        _ => PromptFrontendKind::Auto,
    };
    run_with_kind(std::env::args().collect(), kind)
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_run_auto_from_env() -> i32 {
    run_auto_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_run_rp_from_env() -> i32 {
    run_rp_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_run_rpl_from_env() -> i32 {
    run_rpl_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_run_rpb_from_env() -> i32 {
    run_rpb_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_run_rpe_from_env() -> i32 {
    run_rpe_from_env()
}
