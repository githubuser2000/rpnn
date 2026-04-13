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
        _ => program.snapshot(),
    }
}
