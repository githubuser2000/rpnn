//! Output syntax morphisms transcompiled from
//! `python_arch_reference/reta_architecture/output_syntax.py`.
//!
//! This module owns the renderer mode vocabulary and the mode-specific table,
//! row and cell wrappers.  The heavy historical renderer can keep producing
//! bytes for now; this layer gives Rust code the same syntax choices in typed
//! form.

use serde::{Deserialize, Serialize};

use crate::number_theory::prime_creativity;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OutputMode {
    Shell,
    Nichts,
    Csv,
    Bbcode,
    Html,
    Emacs,
    Markdown,
}

impl OutputMode {
    pub const fn canonical_name(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Nichts => "nichts",
            Self::Csv => "csv",
            Self::Bbcode => "bbcode",
            Self::Html => "html",
            Self::Emacs => "emacs",
            Self::Markdown => "markdown",
        }
    }

    pub const fn syntax_class_name(self) -> &'static str {
        match self {
            Self::Shell => "OutputSyntax",
            Self::Nichts => "NichtsSyntax",
            Self::Csv => "csvSyntax",
            Self::Bbcode => "bbCodeSyntax",
            Self::Html => "htmlSyntax",
            Self::Emacs => "emacsSyntax",
            Self::Markdown => "markdownSyntax",
        }
    }

    pub const fn force_one_table(self) -> bool {
        matches!(self, Self::Csv | Self::Emacs | Self::Markdown)
    }

    pub const fn force_zero_width(self) -> bool {
        matches!(self, Self::Csv | Self::Emacs | Self::Markdown)
    }

    pub const fn marks_html_or_bbcode(self) -> bool {
        matches!(self, Self::Bbcode | Self::Html)
    }

    pub fn from_name(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "shell" => Some(Self::Shell),
            "nichts" | "nothing" | "none" => Some(Self::Nichts),
            "csv" => Some(Self::Csv),
            "bbcode" | "bb" => Some(Self::Bbcode),
            "html" => Some(Self::Html),
            "emacs" => Some(Self::Emacs),
            "markdown" | "md" => Some(Self::Markdown),
            _ => None,
        }
    }

    pub fn syntax_markup(self) -> SyntaxMarkup {
        match self {
            Self::Nichts => SyntaxMarkup::empty(self),
            Self::Shell => SyntaxMarkup::empty(self),
            Self::Csv => SyntaxMarkup::empty(self),
            Self::Emacs => SyntaxMarkup {
                mode: self,
                begin_table: "".to_string(),
                end_table: "".to_string(),
                begin_cell: "|".to_string(),
                end_cell: "".to_string(),
                begin_row: "".to_string(),
                end_row: "|".to_string(),
            },
            Self::Markdown => SyntaxMarkup {
                mode: self,
                begin_table: "".to_string(),
                end_table: "".to_string(),
                begin_cell: "|".to_string(),
                end_cell: "".to_string(),
                begin_row: "".to_string(),
                end_row: "|".to_string(),
            },
            Self::Bbcode => SyntaxMarkup {
                mode: self,
                begin_table: "[table]".to_string(),
                end_table: "[/table]".to_string(),
                begin_cell: "[td]".to_string(),
                end_cell: "[/td]".to_string(),
                begin_row: "[tr]".to_string(),
                end_row: "[/tr]".to_string(),
            },
            Self::Html => SyntaxMarkup {
                mode: self,
                begin_table: r#"<table border=0 id="bigtable">"#.to_string(),
                end_table: "</table>\n".to_string(),
                begin_cell: "<td>\n".to_string(),
                end_cell: "\n</td>\n".to_string(),
                begin_row: "".to_string(),
                end_row: "</tr>\n".to_string(),
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SyntaxMarkup {
    pub mode: OutputMode,
    pub begin_table: String,
    pub end_table: String,
    pub begin_cell: String,
    pub end_cell: String,
    pub begin_row: String,
    pub end_row: String,
}

impl SyntaxMarkup {
    pub fn empty(mode: OutputMode) -> Self {
        Self {
            mode,
            begin_table: String::new(),
            end_table: String::new(),
            begin_cell: String::new(),
            end_cell: String::new(),
            begin_row: String::new(),
            end_row: String::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutputModeSpec {
    pub canonical_name: String,
    pub cli_value: String,
    pub syntax_class: String,
    pub force_one_table: bool,
    pub force_zero_width: bool,
    pub marks_html_or_bbcode: bool,
    pub aliases: Vec<String>,
}

impl OutputModeSpec {
    pub fn from_mode(mode: OutputMode) -> Self {
        let canonical_name = mode.canonical_name().to_string();
        Self {
            canonical_name: canonical_name.clone(),
            cli_value: canonical_name.clone(),
            syntax_class: mode.syntax_class_name().to_string(),
            force_one_table: mode.force_one_table(),
            force_zero_width: mode.force_zero_width(),
            marks_html_or_bbcode: mode.marks_html_or_bbcode(),
            aliases: vec![canonical_name],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutputSyntaxSnapshot {
    pub class: String,
    pub modes: Vec<OutputModeSpec>,
    pub legacy_owner: String,
    pub architecture_owner: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutputSyntaxBundle;

impl OutputSyntaxBundle {
    pub fn modes(&self) -> Vec<OutputMode> {
        vec![
            OutputMode::Shell,
            OutputMode::Nichts,
            OutputMode::Csv,
            OutputMode::Bbcode,
            OutputMode::Html,
            OutputMode::Emacs,
            OutputMode::Markdown,
        ]
    }

    pub fn spec_for(&self, mode: OutputMode) -> OutputModeSpec {
        OutputModeSpec::from_mode(mode)
    }

    pub fn class_for(&self, mode: OutputMode) -> &'static str {
        mode.syntax_class_name()
    }

    pub fn snapshot(&self) -> OutputSyntaxSnapshot {
        OutputSyntaxSnapshot {
            class: "OutputSyntaxBundle".to_string(),
            modes: self.modes().into_iter().map(OutputModeSpec::from_mode).collect(),
            legacy_owner: "libs.lib4tables".to_string(),
            architecture_owner: "reta_architecture.output_syntax".to_string(),
        }
    }
}

pub fn bootstrap_output_syntax() -> OutputSyntaxBundle {
    OutputSyntaxBundle
}

pub fn colored_begin_col(mode: OutputMode, num: i64, rest: bool) -> String {
    match mode {
        OutputMode::Bbcode => colored_begin_col_bbcode(num, rest),
        OutputMode::Html => colored_begin_col_html(num, rest),
        _ => mode.syntax_markup().begin_row,
    }
}

fn colored_begin_col_bbcode(num: i64, rest: bool) -> String {
    let number_type = prime_creativity(num);
    if rest {
        return "[tr]".to_string();
    }
    match (number_type, num % 2 == 0, num) {
        (1, true, _) => r#"[tr="background-color:#66ff66;color:#000000;"]"#.to_string(),
        (1, false, _) => r#"[tr="background-color:#009900;color:#ffffff;"]"#.to_string(),
        (2, true, _) | (_, true, 1) => r#"[tr="background-color:#ffff66;color:#000099;"]"#.to_string(),
        (2, false, _) | (_, false, 1) => r#"[tr="background-color:#555500;color:#aaaaff;"]"#.to_string(),
        (3, true, _) => r#"[tr="background-color:#9999ff;color:#202000;"]"#.to_string(),
        (3, false, _) => r#"[tr="background-color:#000099;color:#ffff66;"]"#.to_string(),
        (_, _, 0) => r#"[tr="background-color:#ff2222;color:#002222;"]"#.to_string(),
        _ => "[tr]".to_string(),
    }
}

fn colored_begin_col_html(num: i64, rest: bool) -> String {
    let number_type = prime_creativity(num);
    if rest {
        return "<tr>\n".to_string();
    }
    match (number_type, num % 2 == 0, num) {
        (1, true, _) => r#"<tr style="background-color:#66ff66;color:#000000;">
"#.to_string(),
        (1, false, _) => r#"<tr style="background-color:#009900;color:#ffffff;">
"#.to_string(),
        (2, true, _) | (_, true, 1) => r#"<tr style="background-color:#ffff66;color:#000099;">
"#.to_string(),
        (2, false, _) | (_, false, 1) => r#"<tr style="background-color:#555500;color:#aaaaff;">
"#.to_string(),
        (3, true, _) => r#"<tr style="background-color:#9999ff;color:#202000;">
"#.to_string(),
        (3, false, _) => r#"<tr style="background-color:#000099;color:#ffff66;">
"#.to_string(),
        (_, _, 0) => r#"<tr style="background-color:#ff2222;color:#002222;">
"#.to_string(),
        _ => "<tr>\n".to_string(),
    }
}

pub fn generate_cell_begin(
    mode: OutputMode,
    spalte: i64,
    content: Option<i64>,
    zeile: Option<i64>,
    header_tags: &[String],
) -> String {
    match mode {
        OutputMode::Nichts => String::new(),
        OutputMode::Bbcode => {
            let adjusted = spalte + 2;
            let color = if adjusted == 0 {
                match content {
                    Some(value) if value % 2 == 0 => r#"="background-color:#000000;color:#ffffff""#,
                    Some(_) => r#"="background-color:#ffffff;color:#000000""#,
                    None => "=\"\"",
                }
            } else {
                "=\"\""
            };
            format!("[td{color}]")
        }
        OutputMode::Html => {
            let adjusted = spalte + 2;
            let mut attributes = String::new();
            if zeile == Some(0) {
                attributes.push_str(&format!(r#" class="z_{} r_{}""#, zeile.unwrap_or(0), adjusted));
            }
            if adjusted == 0 || adjusted == 1 {
                if let Some(value) = content {
                    if value % 2 == 0 {
                        attributes.push_str(r#" style="background-color:#000000;color:#ffffff;""#);
                    } else {
                        attributes.push_str(r#" style="background-color:#ffffff;color:#000000;""#);
                    }
                }
            } else if header_tags.iter().any(|tag| tag == "Symbole") {
                attributes.push_str(r#" class="tdSymbole" style="background-image: url();background-size: cover;background-repeat: no-repeat;background-position: right; ""#);
            }
            format!("<td{attributes}>\n")
        }
        _ => mode.syntax_markup().begin_cell,
    }
}

pub fn output_syntax_snapshot() -> OutputSyntaxSnapshot {
    bootstrap_output_syntax().snapshot()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax_flags_match_python_classes() {
        assert!(OutputMode::Csv.force_one_table());
        assert!(OutputMode::Markdown.force_zero_width());
        assert!(OutputMode::Html.marks_html_or_bbcode());
        assert_eq!(OutputMode::from_name("md"), Some(OutputMode::Markdown));
    }

    #[test]
    fn colored_rows_follow_prime_creativity() {
        assert!(colored_begin_col(OutputMode::Html, 7, false).contains("009900"));
        assert!(colored_begin_col(OutputMode::Bbcode, 8, false).contains("9999ff"));
        assert_eq!(colored_begin_col(OutputMode::Csv, 8, false), "");
    }

    #[test]
    fn cell_begin_uses_mode_syntax() {
        assert_eq!(generate_cell_begin(OutputMode::Markdown, 1, None, None, &[]), "|");
        assert!(generate_cell_begin(OutputMode::Html, -2, Some(2), Some(1), &[]).contains("td"));
    }
}
