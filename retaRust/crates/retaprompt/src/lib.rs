#![allow(non_snake_case)]

pub use reta::prompt::{
    run_retaprompt_auto_from_env,
    run_retaprompt_rp,
    run_retaprompt_rpb,
    run_retaprompt_rpe,
    run_retaprompt_rpl,
    PromptFrontendKind,
    PromptFrontendProfile,
};

pub fn run_with_kind(argv: Vec<String>, kind: PromptFrontendKind) -> i32 {
    reta::prompt::run_retaprompt_with_kind(argv, kind)
}

pub fn run_with_profile(argv: Vec<String>, profile: PromptFrontendProfile) -> i32 {
    reta::prompt::run_retaprompt_with_profile(argv, profile)
}

pub fn run_rp(argv: Vec<String>) -> i32 {
    reta::prompt::run_retaprompt_rp(argv)
}

pub fn run_rpl(argv: Vec<String>) -> i32 {
    reta::prompt::run_retaprompt_rpl(argv)
}

pub fn run_rpb(argv: Vec<String>) -> i32 {
    reta::prompt::run_retaprompt_rpb(argv)
}

pub fn run_rpe(argv: Vec<String>) -> i32 {
    reta::prompt::run_retaprompt_rpe(argv)
}

pub fn run_auto_from_env() -> i32 {
    reta::prompt::run_retaprompt_auto_from_env()
}

pub fn run_rp_from_env() -> i32 {
    reta::prompt::run_retaprompt_rp_from_env()
}

pub fn run_rpl_from_env() -> i32 {
    reta::prompt::run_retaprompt_rpl_from_env()
}

pub fn run_rpb_from_env() -> i32 {
    reta::prompt::run_retaprompt_rpb_from_env()
}

pub fn run_rpe_from_env() -> i32 {
    reta::prompt::run_retaprompt_rpe_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_run_kind_from_env(kind: i32) -> i32 {
    reta::prompt::retaprompt_run_kind_from_env(kind)
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_run_auto_from_env() -> i32 {
    run_auto_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_run_rp_from_env() -> i32 {
    reta::prompt::retaprompt_run_rp_from_env_abi()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_run_rpl_from_env() -> i32 {
    reta::prompt::retaprompt_run_rpl_from_env_abi()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_run_rpb_from_env() -> i32 {
    reta::prompt::retaprompt_run_rpb_from_env_abi()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_run_rpe_from_env() -> i32 {
    reta::prompt::retaprompt_run_rpe_from_env_abi()
}
