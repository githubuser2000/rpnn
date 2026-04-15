use super::app::{run_prompt_frontend_with_profile, run_rp_one_shot};
use super::frontend_profile::{PromptFrontendKind, PromptFrontendProfile};

fn env_args() -> Vec<String> {
    std::env::args().collect::<Vec<_>>()
}

pub fn run_with_kind(argv: Vec<String>, kind: PromptFrontendKind) -> i32 {
    let profile = PromptFrontendProfile::for_kind(kind, true);
    run_prompt_frontend_with_profile(argv, profile)
}

pub fn run_with_profile(argv: Vec<String>, profile: PromptFrontendProfile) -> i32 {
    run_prompt_frontend_with_profile(argv, profile)
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
    run_rp(env_args())
}

pub fn run_rpl_from_env() -> i32 {
    run_rpl(env_args())
}

pub fn run_rpb_from_env() -> i32 {
    run_rpb(env_args())
}

pub fn run_rpe_from_env() -> i32 {
    run_rpe(env_args())
}

pub fn run_auto_from_env() -> i32 {
    run_prompt_frontend_with_profile(
        env_args(),
        PromptFrontendProfile::for_kind(PromptFrontendKind::Auto, true),
    )
}

pub fn run_one_shot_direct(argv: Vec<String>) -> i32 {
    run_rp_one_shot(argv, true)
}

pub fn run_retaprompt_with_kind(argv: Vec<String>, kind: PromptFrontendKind) -> i32 {
    run_with_kind(argv, kind)
}

pub fn run_retaprompt_with_profile(argv: Vec<String>, profile: PromptFrontendProfile) -> i32 {
    run_with_profile(argv, profile)
}

pub fn run_retaprompt_rp(argv: Vec<String>) -> i32 {
    run_rp(argv)
}

pub fn run_retaprompt_rpl(argv: Vec<String>) -> i32 {
    run_rpl(argv)
}

pub fn run_retaprompt_rpb(argv: Vec<String>) -> i32 {
    run_rpb(argv)
}

pub fn run_retaprompt_rpe(argv: Vec<String>) -> i32 {
    run_rpe(argv)
}

pub fn run_retaprompt_rp_from_env() -> i32 {
    run_rp_from_env()
}

pub fn run_retaprompt_rpl_from_env() -> i32 {
    run_rpl_from_env()
}

pub fn run_retaprompt_rpb_from_env() -> i32 {
    run_rpb_from_env()
}

pub fn run_retaprompt_rpe_from_env() -> i32 {
    run_rpe_from_env()
}

pub fn run_retaprompt_auto_from_env() -> i32 {
    run_auto_from_env()
}

fn kind_from_abi_value(kind: i32) -> PromptFrontendKind {
    match kind {
        1 => PromptFrontendKind::Rp,
        2 => PromptFrontendKind::Rpl,
        3 => PromptFrontendKind::Rpb,
        4 => PromptFrontendKind::Rpe,
        _ => PromptFrontendKind::Auto,
    }
}

pub fn retaprompt_run_kind_from_env(kind: i32) -> i32 {
    let resolved = kind_from_abi_value(kind);
    if resolved == PromptFrontendKind::Auto {
        run_auto_from_env()
    } else {
        run_with_kind(env_args(), resolved)
    }
}

pub fn retaprompt_run_rp_from_env_abi() -> i32 {
    run_rp_from_env()
}

pub fn retaprompt_run_rpl_from_env_abi() -> i32 {
    run_rpl_from_env()
}

pub fn retaprompt_run_rpb_from_env_abi() -> i32 {
    run_rpb_from_env()
}

pub fn retaprompt_run_rpe_from_env_abi() -> i32 {
    run_rpe_from_env()
}


#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_run_kind_from_env(kind: i32) -> i32 {
    retaprompt_run_kind_from_env(kind)
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_run_auto_from_env() -> i32 {
    run_retaprompt_auto_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_run_rp_from_env() -> i32 {
    retaprompt_run_rp_from_env_abi()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_run_rpl_from_env() -> i32 {
    retaprompt_run_rpl_from_env_abi()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_run_rpb_from_env() -> i32 {
    retaprompt_run_rpb_from_env_abi()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_run_rpe_from_env() -> i32 {
    retaprompt_run_rpe_from_env_abi()
}
