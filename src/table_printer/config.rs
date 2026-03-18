// table_printer/config.rs

pub const MIN_COLUMN_WIDTH: usize = 3;
pub const MAX_COLUMNS_CAP: usize = 12;
pub const MAX_COLUMN_WIDTH: usize = 38;
pub const COLUMN_OVERHEAD: usize = 1;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ColumnKind {
    Compact,
    Normal,
    Wide,
}

impl ColumnKind {
    /// Keine Wortsuche mehr im Header.
    /// Der Header allein ist zu unzuverlässig für semantische Typisierung.
    /// Die eigentliche Breitenlogik soll aus den Inhalten kommen.
    pub fn infer_from_header(_header: &str) -> ColumnKind {
        ColumnKind::Normal
    }

    pub fn min_width(&self) -> usize {
        match self {
            ColumnKind::Compact => 2,
            ColumnKind::Normal => 4,
            ColumnKind::Wide => 8,
        }
    }

    pub fn soft_width(&self) -> usize {
        match self {
            ColumnKind::Compact => 6,
            ColumnKind::Normal => 12,
            ColumnKind::Wide => 24,
        }
    }

    pub fn growth_weight(&self) -> usize {
        match self {
            ColumnKind::Compact => 1,
            ColumnKind::Normal => 2,
            ColumnKind::Wide => 4,
        }
    }

    pub fn prefers_compact_layout(&self) -> bool {
        matches!(self, ColumnKind::Compact | ColumnKind::Normal)
    }
}
