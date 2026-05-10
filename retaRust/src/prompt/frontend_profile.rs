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
            implicit_logging: false,
            one_shot: false,
        }
    }

    pub const fn rpl() -> Self {
        Self {
            kind: PromptFrontendKind::Rpl,
            program_name: "rpl",
            start_with_vi_mode: true,
            implicit_logging: true,
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
            start_with_vi_mode: false,
            implicit_logging: false,
            one_shot: false,
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
            "rp" | "rrp" => Self::rp(),
            "rpl" | "rrpl" => Self::rpl(),
            "rpb" | "rrpb" => Self::rpb(),
            "rpe" | "rrpe" => Self::rpe(),
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

#[cfg(test)]
mod tests {
    use super::{PromptFrontendKind, PromptFrontendProfile};

    #[test]
    fn rp_profile_is_vi_without_implicit_logging() {
        let profile = PromptFrontendProfile::rp();
        assert_eq!(profile.kind, PromptFrontendKind::Rp);
        assert!(profile.start_with_vi_mode);
        assert!(!profile.implicit_logging);
        assert!(!profile.one_shot);
        assert!(!profile.default_exact_mode(&["rp".to_string()]));
        assert!(!profile.emacs_output_mode());
    }

    #[test]
    fn rpl_profile_is_vi_with_full_implicit_logging() {
        let profile = PromptFrontendProfile::rpl();
        assert_eq!(profile.kind, PromptFrontendKind::Rpl);
        assert!(profile.start_with_vi_mode);
        assert!(profile.implicit_logging);
        assert!(!profile.one_shot);
        assert!(profile.default_exact_mode(&["rpl".to_string()]));
        assert!(!profile.emacs_output_mode());
    }

    #[test]
    fn rpe_profile_is_interactive_emacs_output_mode() {
        let profile = PromptFrontendProfile::rpe();
        assert_eq!(profile.kind, PromptFrontendKind::Rpe);
        assert!(!profile.start_with_vi_mode);
        assert!(!profile.implicit_logging);
        assert!(!profile.one_shot);
        assert!(profile.default_exact_mode(&["rpe".to_string()]));
        assert!(profile.emacs_output_mode());
    }

    #[test]
    fn rpb_profile_stays_one_shot_exact_command_frontend() {
        let profile = PromptFrontendProfile::rpb();
        assert_eq!(profile.kind, PromptFrontendKind::Rpb);
        assert!(profile.start_with_vi_mode);
        assert!(!profile.implicit_logging);
        assert!(profile.one_shot);
        assert!(profile.default_exact_mode(&["rpb".to_string()]));
        assert!(!profile.emacs_output_mode());
    }
}
