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

impl OutputSyntax {
    pub fn from_art_value(value: &str) -> Option<Self> {
        match value.trim() {
            "shell" => Some(Self::Plain),
            "html" => Some(Self::HTML),
            "bbcode" => Some(Self::BBCode),
            "csv" => Some(Self::CSV),
            "markdown" => Some(Self::Markdown),
            "emacs" => Some(Self::Emacs),
            "nichts" => Some(Self::Nichts),
            _ => None,
        }
    }

    pub fn uses_terminal_colors(self) -> bool {
        matches!(self, Self::Plain)
    }

    pub fn begin_table(self) -> &'static str {
        match self {
            Self::HTML => "<table border=0 id=\"bigtable\">",
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

    pub fn generate_cell(
        self,
        _col_index: i32,
        _params: &HashMap<String, String>,
        _line_num: i32,
    ) -> String {
        match self {
            Self::HTML => "<td>".to_string(),
            Self::BBCode => "[td]".to_string(),
            Self::Markdown => "|".to_string(),
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
            Self::HTML => "</tr>",
            Self::BBCode => "[/tr]",
            Self::Markdown => "|",
            _ => "\n",
        }
    }
}
