// retaAusgabe-output_syntax.rs
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
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
    pub fn begin_table(self) -> &'static str {
        match self {
            OutputSyntax::HTML => "<table>",
            OutputSyntax::BBCode => "[table]",
            _ => "",
        }
    }
    
    pub fn end_table(self) -> &'static str {
        match self {
            OutputSyntax::HTML => "</table>",
            OutputSyntax::BBCode => "[/table]",
            _ => "",
        }
    }
    
    pub fn generate_cell(self, _col_index: i32, _params: &HashMap<String, String>, _line_num: i32) -> String {
        match self {
            OutputSyntax::HTML => "<td>".to_string(),
            OutputSyntax::BBCode => "[td]".to_string(),
            OutputSyntax::Markdown => "|".to_string(),
            _ => "".to_string(),
        }
    }
    
    pub fn end_cell(self) -> &'static str {
        match self {
            OutputSyntax::HTML => "</td>",
            OutputSyntax::BBCode => "[/td]",
            _ => "",
        }
    }
    
    pub fn colored_begin_col(self, _line_num: i32) -> &'static str {
        ""
    }
    
    pub fn end_zeile(self) -> &'static str {
        match self {
            OutputSyntax::HTML => "</tr>",
            OutputSyntax::BBCode => "[/tr]",
            OutputSyntax::Markdown => "|",
            _ => "\n",
        }
    }
}
