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
    pub program_name: &'static str,
    pub start_with_vi_mode: bool,
    pub implicit_logging: bool,
    pub one_shot: bool,
}

impl PromptFrontendProfile {
    pub const fn rp() -> Self {
        Self {
            kind: PromptFrontendKind::Rp,
            program_name: "rp",
            start_with_vi_mode: true,
            implicit_logging: true,
            one_shot: false,
        }
    }

    pub const fn rpl() -> Self {
        Self {
            kind: PromptFrontendKind::Rpl,
            program_name: "rpl",
            start_with_vi_mode: true,
            implicit_logging: false,
            one_shot: false,
        }
    }

    pub const fn rpb() -> Self {
        Self {
            kind: PromptFrontendKind::Rpb,
            program_name: "rpb",
            start_with_vi_mode: true,
            implicit_logging: false,
            one_shot: true,
        }
    }

    pub const fn rpe() -> Self {
        Self {
            kind: PromptFrontendKind::Rpe,
            program_name: "rpe",
            start_with_vi_mode: true,
            implicit_logging: false,
            one_shot: true,
        }
    }

    pub fn for_kind(kind: PromptFrontendKind, fallback_vi_mode: bool) -> Self {
        match kind {
            PromptFrontendKind::Rp => Self::rp(),
            PromptFrontendKind::Rpl => Self::rpl(),
            PromptFrontendKind::Rpb => Self::rpb(),
            PromptFrontendKind::Rpe => Self::rpe(),
            PromptFrontendKind::Auto => Self {
                kind,
                program_name: "rp",
                start_with_vi_mode: fallback_vi_mode,
                implicit_logging: false,
                one_shot: false,
            },
        }
    }

    pub fn from_program_name(program_name: &str, fallback_vi_mode: bool) -> Self {
        match program_name {
            "rp" => Self::rp(),
            "rpl" => Self::rpl(),
            "rpb" => Self::rpb(),
            "rpe" => Self::rpe(),
            _ => Self {
                kind: PromptFrontendKind::Auto,
                program_name: "rp",
                start_with_vi_mode: fallback_vi_mode,
                implicit_logging: false,
                one_shot: false,
            },
        }
    }

    pub fn default_exact_mode(&self, argv: &[String]) -> bool {
        match self.kind {
            PromptFrontendKind::Rp => false,
            PromptFrontendKind::Rpl => !argv.iter().any(|arg| arg == "-debug"),
            PromptFrontendKind::Rpb | PromptFrontendKind::Rpe => true,
            PromptFrontendKind::Auto => false,
        }
    }

    pub const fn emacs_output_mode(&self) -> bool {
        match self.kind {
            PromptFrontendKind::Rpe => true,
            _ => false,
        }
    }

    pub fn program_name_or<'a>(&self, detected: &'a str) -> &'a str {
        if self.kind == PromptFrontendKind::Auto {
            detected
        } else {
            self.program_name
        }
    }
}
