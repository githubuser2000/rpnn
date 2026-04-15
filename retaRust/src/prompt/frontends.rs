use super::app::run_prompt_frontend_with_profile_from_env;
use super::frontend_profile::PromptFrontendProfile;

pub fn run_prompt_frontend_from_env() -> i32 {
    let argv = std::env::args().collect::<Vec<_>>();
    let program_name = argv
        .first()
        .and_then(|value| std::path::Path::new(value).file_name())
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "rp".to_string());
    let profile = PromptFrontendProfile::from_program_name(&program_name, true);
    run_prompt_frontend_with_profile_from_env(profile)
}

pub fn run_rp_frontend_from_env() -> i32 {
    run_prompt_frontend_with_profile_from_env(PromptFrontendProfile::rp())
}

pub fn run_rpl_frontend_from_env() -> i32 {
    run_prompt_frontend_with_profile_from_env(PromptFrontendProfile::rpl())
}

pub fn run_rpb_frontend_from_env() -> i32 {
    run_prompt_frontend_with_profile_from_env(PromptFrontendProfile::rpb())
}

pub fn run_rpe_frontend_from_env() -> i32 {
    run_prompt_frontend_with_profile_from_env(PromptFrontendProfile::rpe())
}
