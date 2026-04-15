use super::frontend_profile::PromptFrontendProfile;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PromptFrontendPreset {
    pub start_with_vi_mode: bool,
    pub implicit_logging: bool,
    pub one_shot: bool,
}

impl PromptFrontendPreset {
    pub fn from_program_name(program_name: &str, fallback_vi_mode: bool) -> Self {
        let profile = PromptFrontendProfile::from_program_name(program_name, fallback_vi_mode);
        Self::from_profile(profile)
    }

    pub fn from_profile(profile: PromptFrontendProfile) -> Self {
        Self {
            start_with_vi_mode: profile.start_with_vi_mode,
            implicit_logging: profile.implicit_logging,
            one_shot: profile.one_shot,
        }
    }
}
