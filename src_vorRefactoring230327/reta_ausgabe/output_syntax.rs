#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputSyntax {
    Plain,
    Markdown,
    BBCode,
    HTML,
    CSV,
    Emacs,
    Nichts,
}

impl Default for OutputSyntax {
    fn default() -> Self {
        Self::Plain
    }
}

impl OutputSyntax {
    pub fn from_art_value(value: &str) -> Option<Self> {
        match value.trim() {
            "shell" => Some(Self::Plain),
            "markdown" => Some(Self::Markdown),
            "bbcode" => Some(Self::BBCode),
            "html" => Some(Self::HTML),
            "csv" => Some(Self::CSV),
            "emacs" => Some(Self::Emacs),
            "nichts" => Some(Self::Nichts),
            _ => None,
        }
    }

    pub fn begin_table(self) -> &'static str {
        match self {
            Self::HTML => "<table border=0 id=\"bigtable\">\n",
            Self::BBCode => "[table]",
            _ => "",
        }
    }

    pub fn end_table(self) -> &'static str {
        match self {
            Self::HTML => "</table>",
            Self::BBCode => "[/table]",
            _ => "",
        }
    }

    pub fn uses_terminal_colors(self) -> bool {
        matches!(self, Self::Plain)
    }
}
