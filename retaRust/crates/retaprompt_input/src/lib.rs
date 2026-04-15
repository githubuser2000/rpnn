#![allow(non_snake_case)]

pub use reta::prompt::frontend_profile::{PromptFrontendKind, PromptFrontendProfile};
pub use reta::prompt::frontends::{
    run_rp_frontend_from_env,
    run_rpe_frontend_from_env,
    run_rpl_frontend_from_env,
};

pub fn run_with_profile(argv: Vec<String>, profile: PromptFrontendProfile) -> i32 {
    reta::prompt::app::run_prompt_frontend_with_profile(argv, profile)
}

pub fn run_with_kind(argv: Vec<String>, kind: PromptFrontendKind) -> i32 {
    match kind {
        PromptFrontendKind::Rp => run_rp(argv),
        PromptFrontendKind::Rpl => run_rpl(argv),
        PromptFrontendKind::Rpe => run_rpe(argv),
        PromptFrontendKind::Rpb | PromptFrontendKind::Auto => {
            let profile = PromptFrontendProfile::for_kind(kind, true);
            reta::prompt::app::run_prompt_frontend_with_profile(argv, profile)
        }
    }
}

pub fn run_rp(argv: Vec<String>) -> i32 {
    reta::prompt::app::run_prompt_frontend_with_profile(argv, PromptFrontendProfile::rp())
}

pub fn run_rpl(argv: Vec<String>) -> i32 {
    reta::prompt::app::run_prompt_frontend_with_profile(argv, PromptFrontendProfile::rpl())
}

pub fn run_rpe(argv: Vec<String>) -> i32 {
    reta::prompt::app::run_prompt_frontend_with_profile(argv, PromptFrontendProfile::rpe())
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
