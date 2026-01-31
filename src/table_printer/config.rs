// table_printer/config.rs
pub const MIN_COLUMN_WIDTH: usize = 10;
pub const MAX_COLUMNS_CAP: usize = 6;
pub const MAX_COLUMN_WIDTH: usize = 34;
pub const COLUMN_OVERHEAD: usize = 5;

#[derive(Copy, Clone)]
pub enum ColumnKind {
    Id,
    Number,
    ShortText,
    LongText,
}

impl ColumnKind {
    pub fn infer_from_header(header: &str) -> ColumnKind {
        let h = header.to_lowercase();
        if h == "id" || h.ends_with("_id") {
            ColumnKind::Id
        } else if h.contains("count") || h.contains("num") {
            ColumnKind::Number
        } else if h.contains("name") || h.contains("title") {
            ColumnKind::ShortText
        } else {
            ColumnKind::LongText
        }
    }

    pub fn min_width(&self) -> usize {
        match self {
            ColumnKind::Id => 6,
            ColumnKind::Number => 8,
            ColumnKind::ShortText => 14,
            ColumnKind::LongText => 20,
        }
    }
}
