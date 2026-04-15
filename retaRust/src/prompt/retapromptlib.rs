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
    run_prompt_frontend_with_profile(env_args(), PromptFrontendProfile::for_kind(PromptFrontendKind::Auto, true))
}

pub fn run_one_shot_direct(argv: Vec<String>) -> i32 {
    run_rp_one_shot(argv, true)
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
