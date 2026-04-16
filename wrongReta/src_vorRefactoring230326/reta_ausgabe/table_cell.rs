// src/reta_ausgabe/table_cell.rs
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub struct TableCell {
    pub original_content: String,
}

impl TableCell {
    pub fn new(content: String, _width: usize) -> Self {
        TableCell {
            original_content: content,
        }
    }

    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        if line_num == 0 {
            Some(self.original_content.as_str())
        } else {
            None
        }
    }

    pub fn line_count(&self) -> usize {
        1
    }

    pub fn get_line_width(&self, line_num: usize) -> usize {
        self.get_line(line_num)
            .map(UnicodeWidthStr::width)
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
        self.cells
            .iter()
            .map(|cell| cell.line_count())
            .max()
            .unwrap_or(0)
    }
}
