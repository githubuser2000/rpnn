use super::frontend_profile::{PromptFrontendKind, PromptFrontendProfile};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PromptFrontendPreset {
    pub start_with_vi_mode: bool,
    pub implicit_logging: bool,
    pub default_exact_mode: bool,
    pub one_shot: bool,
    pub emacs_output_mode: bool,
    pub persistent_history: bool,
}

impl PromptFrontendPreset {
    pub fn from_program_name(program_name: &str, fallback_vi_mode: bool) -> Self {
        let profile = PromptFrontendProfile::from_program_name(program_name, fallback_vi_mode);
        Self::from_profile(profile)
    }

    pub fn from_profile(profile: PromptFrontendProfile) -> Self {
        Self::from_profile_and_argv(profile, &[])
    }

    pub fn from_profile_and_argv(profile: PromptFrontendProfile, argv: &[String]) -> Self {
        Self {
            start_with_vi_mode: profile.start_with_vi_mode,
            implicit_logging: profile.implicit_logging,
            default_exact_mode: profile.default_exact_mode(argv),
            one_shot: profile.one_shot,
            emacs_output_mode: profile.emacs_output_mode(),
            persistent_history: matches!(profile.kind, PromptFrontendKind::Rpl),
        }
    }
}
