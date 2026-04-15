#![allow(non_snake_case)]

//! This crate is intentionally narrow: it only offers the self-entered frontend side
//! for rp/rpl/rpe and forwards into `reta` without copying implementation.

//! Own-command-input layer for rp/rpl/rpe.
//!
//! This crate intentionally only exposes the self-entered frontend side.
//! Command semantics stay in `retaprompt_commands`.

pub fn run_rp(argv: Vec<String>) -> i32 {
    reta::prompt::app::run_prompt_frontend_with_profile(
        argv,
        reta::prompt::frontend_profile::PromptFrontendProfile::rp(),
    )
}

pub fn run_rpl(argv: Vec<String>) -> i32 {
    reta::prompt::app::run_prompt_frontend_with_profile(
        argv,
        reta::prompt::frontend_profile::PromptFrontendProfile::rpl(),
    )
}

pub fn run_rpe(argv: Vec<String>) -> i32 {
    reta::prompt::app::run_prompt_frontend_with_profile(
        argv,
        reta::prompt::frontend_profile::PromptFrontendProfile::rpe(),
    )
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
