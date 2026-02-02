// reta_ausgabe-table_cell.rs
// src/reta_ausgabe/table_cell.rs
use crate::reta_ausgabe::utils::word_wrap;  // oder use super::utils::word_wrap;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub struct TableCell {
    pub lines: Vec<String>,
    pub original_content: String,
}

impl TableCell {
    pub fn new(content: String, width: usize) -> Self {
        // Wende Wortumbruch an
        let lines = word_wrap(&content, width);
        
        TableCell { 
            lines,
            original_content: content,
        }
    }
    
    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        self.lines.get(line_num).map(|s| s.as_str())
    }
    
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    
    // Neue Methode für Unicode-Breite einer Zeile
    pub fn get_line_width(&self, line_num: usize) -> usize {
        self.get_line(line_num)
            .map(|line| UnicodeWidthStr::width(line))
            .unwrap_or(0)
    }
}

#[derive(Debug, Clone)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub original_line_num: i32,
    pub display_line_num: i32,
}

impl TableRow {
    pub fn new(cells: Vec<TableCell>, original_line_num: i32, display_line_num: i32) -> Self {
        TableRow {
            cells,
            original_line_num,
            display_line_num,
        }
    }
    
    pub fn max_line_count(&self) -> usize {
        self.cells.iter()
            .map(|cell| cell.line_count())
            .max()
            .unwrap_or(0)
    }
}
