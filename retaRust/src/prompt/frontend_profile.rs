#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromptFrontendKind {
    Rp,
    Rpl,
    Rpb,
    Rpe,
    Auto,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PromptFrontendProfile {
    pub kind: PromptFrontendKind,
    pub binary_name: &'static str,
    pub start_with_vi_mode: bool,
    pub implicit_logging: bool,
    pub one_shot: bool,
}

impl PromptFrontendProfile {
    pub const fn rp() -> Self {
        Self {
            kind: PromptFrontendKind::Rp,
            binary_name: "rp",
            start_with_vi_mode: true,
            implicit_logging: false,
            one_shot: false,
        }
    }

    pub const fn rpl() -> Self {
        Self {
            kind: PromptFrontendKind::Rpl,
            binary_name: "rpl",
            start_with_vi_mode: true,
            implicit_logging: true,
            one_shot: false,
        }
    }

    pub const fn rpb() -> Self {
        Self {
            kind: PromptFrontendKind::Rpb,
            binary_name: "rpb",
            start_with_vi_mode: true,
            implicit_logging: false,
            one_shot: true,
        }
    }

    pub const fn rpe() -> Self {
        Self {
            kind: PromptFrontendKind::Rpe,
            binary_name: "rpe",
            start_with_vi_mode: false,
            implicit_logging: false,
            one_shot: false,
        }
    }

    pub const fn fallback_vi(fallback_vi_mode: bool) -> Self {
        Self {
            kind: PromptFrontendKind::Auto,
            binary_name: "rp",
            start_with_vi_mode: fallback_vi_mode,
            implicit_logging: false,
            one_shot: false,
        }
    }

    pub fn from_program_name(program_name: &str, fallback_vi_mode: bool) -> Self {
        match program_name {
            "rp" => Self::rp(),
            "rpl" => Self::rpl(),
            "rpb" => Self::rpb(),
            "rpe" => Self::rpe(),
            _ => Self::fallback_vi(fallback_vi_mode),
        }
    }
}
