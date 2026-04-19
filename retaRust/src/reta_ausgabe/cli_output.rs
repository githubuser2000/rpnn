use crate::shared::reta_py::Program;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutputKind {
    Shell,
    Html,
    Bbcode,
    Csv,
    Markdown,
    Emacs,
    Nichts,
}

impl OutputKind {
    pub fn from_program(program: &Program) -> Self {
        match program.outType.as_str() {
            "html" => Self::Html,
            "bbcode" => Self::Bbcode,
            "csv" => Self::Csv,
            "markdown" => Self::Markdown,
            "emacs" => Self::Emacs,
            "nichts" => Self::Nichts,
            _ => Self::Shell,
        }
    }
}

pub fn rendered_output(program: &Program) -> String {
    match OutputKind::from_program(program) {
        OutputKind::Nichts => String::new(),
        _ if !program.finallyDisplayLines.is_empty() => program.finallyDisplayLines.join("\n"),
        // Python has no public debug-snapshot fallback on stdout.
        _ => String::new(),
    }
}


#[cfg(test)]
mod tests {
    use super::rendered_output;
    use crate::shared::reta_py::Program;

    #[test]
    fn rendered_output_empty_program_does_not_emit_snapshot() {
        let program = Program::new(vec!["reta".to_string()]);
        assert_eq!(rendered_output(&program), "");
    }

    #[test]
    fn rendered_output_joins_final_display_lines_without_mutating_bytes() {
        let mut program = Program::new(vec!["reta".to_string()]);
        program.finallyDisplayLines = vec!["eins".to_string(), "zwei".to_string()];
        assert_eq!(rendered_output(&program), "eins\nzwei");
    }
}
