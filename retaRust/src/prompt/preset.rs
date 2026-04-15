#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PromptFrontendPreset {
    pub start_with_vi_mode: bool,
    pub implicit_logging: bool,
    pub one_shot: bool,
}

impl PromptFrontendPreset {
    pub fn from_program_name(program_name: &str, fallback_vi_mode: bool) -> Self {
        match program_name {
            "rp" => Self {
                start_with_vi_mode: true,
                implicit_logging: false,
                one_shot: false,
            },
            "rpl" => Self {
                start_with_vi_mode: true,
                implicit_logging: true,
                one_shot: false,
            },
            "rpb" => Self {
                start_with_vi_mode: true,
                implicit_logging: false,
                one_shot: true,
            },
            "rpe" => Self {
                start_with_vi_mode: false,
                implicit_logging: false,
                one_shot: false,
            },
            _ => Self {
                start_with_vi_mode: fallback_vi_mode,
                implicit_logging: false,
                one_shot: false,
            },
        }
    }
}
