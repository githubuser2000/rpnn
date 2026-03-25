use std::collections::HashMap;

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
        match value.trim().to_lowercase().as_str() {
            "shell" | "plain" => Some(Self::Plain),
            "markdown" | "md" => Some(Self::Markdown),
            "bbcode" => Some(Self::BBCode),
            "html" => Some(Self::HTML),
            "csv" => Some(Self::CSV),
            "emacs" | "org" | "orgmode" => Some(Self::Emacs),
            "nichts" | "nothing" | "none" => Some(Self::Nichts),
            _ => None,
        }
    }

    pub fn as_art_name(self) -> &'static str {
        match self {
            Self::Plain => "shell",
            Self::Markdown => "markdown",
            Self::BBCode => "bbcode",
            Self::HTML => "html",
            Self::CSV => "csv",
            Self::Emacs => "emacs",
            Self::Nichts => "nichts",
        }
    }

    pub fn begin_table(self) -> &'static str {
        match self {
            Self::HTML => "<table>",
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

    pub fn generate_cell(self, _col_index: i32, _params: &HashMap<String, String>, _line_num: i32) -> String {
        match self {
            Self::HTML => "<td>".to_string(),
            Self::BBCode => "[td]".to_string(),
            Self::Markdown | Self::Emacs => "|".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn end_cell(self) -> &'static str {
        match self {
            Self::HTML => "</td>",
            Self::BBCode => "[/td]",
            _ => "",
        }
    }

    pub fn colored_begin_col(self, _line_num: i32) -> &'static str {
        ""
    }

    pub fn end_zeile(self) -> &'static str {
        match self {
            Self::HTML => "</tr>
",
            Self::BBCode => "[/tr]
",
            Self::Markdown | Self::Emacs | Self::CSV | Self::Plain | Self::Nichts => "
",
        }
    }

    pub fn uses_terminal_colors(self) -> bool {
        matches!(self, Self::Plain)
    }
}
