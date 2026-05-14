//! Shell-like table-view layout and horizontal viewport projection.
//!
//! Stage 30 separates the historical shell layout question from the value
//! renderer.  Earlier stages could materialize cells and render them in output
//! modes; this module adds the missing width/padding/page morphism that mirrors
//! the old `cliOut` loop at a safer, typed level.  It is policy-controlled and
//! disabled by default in the visible path: callers can use it for shadow diffs
//! before any commit gate is allowed to route through it.

use serde::{Deserialize, Serialize};

use crate::table_view::MaterializedTableViewRow;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLayoutConfig {
    /// Enable the layout projection.  Disabled means callers should keep the
    /// old unpadded join semantics.
    pub enabled: bool,
    /// Separator between rendered cells for shell-like modes.
    pub separator: String,
    /// Optional terminal/screen width.  When set and `onetable` is false,
    /// columns are split into horizontal pages that fit this width.
    pub max_screen_width: Option<usize>,
    /// Per-column width limits, corresponding to the old `breiten` vector.
    /// A value of zero means “use measured maximum”.  A positive value smaller
    /// than the measured maximum limits the cell width.
    pub width_overrides: Vec<usize>,
    /// Keep all columns in one horizontal page regardless of `max_screen_width`.
    pub onetable: bool,
    /// Pad cells to the page width.  This is the historical shell behaviour;
    /// disabling it keeps compact lines while still producing page diagnostics.
    pub pad_cells: bool,
    /// Emit an empty separator line between horizontal pages.
    pub include_page_separator: bool,
    pub page_separator: String,
}

impl Default for TableViewLayoutConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            separator: " | ".to_string(),
            max_screen_width: None,
            width_overrides: Vec::new(),
            onetable: false,
            pad_cells: true,
            include_page_separator: false,
            page_separator: String::new(),
        }
    }
}

impl TableViewLayoutConfig {
    pub fn enabled_shell() -> Self {
        Self {
            enabled: true,
            ..Self::default()
        }
    }

    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    pub fn with_width_overrides(mut self, widths: Vec<usize>) -> Self {
        self.width_overrides = widths;
        self
    }

    pub fn with_max_screen_width(mut self, width: Option<usize>) -> Self {
        self.max_screen_width = width.filter(|value| *value > 0);
        self
    }

    pub fn with_onetable(mut self, onetable: bool) -> Self {
        self.onetable = onetable;
        self
    }

    pub fn activates_layout(&self) -> bool {
        self.enabled
            || self.max_screen_width.is_some()
            || !self.width_overrides.is_empty()
            || self.onetable
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewColumnPage {
    pub page_index: usize,
    pub start_column: usize,
    pub end_column_exclusive: usize,
    pub column_count: usize,
    pub width: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLayoutReport {
    pub class: String,
    pub enabled: bool,
    pub source_row_count: usize,
    pub column_count: usize,
    pub column_widths: Vec<usize>,
    pub page_count: usize,
    pub pages: Vec<TableViewColumnPage>,
    pub rendered_line_count: usize,
    pub rendered_lines: Vec<String>,
    pub max_screen_width: Option<usize>,
    pub onetable: bool,
    pub pad_cells: bool,
    pub width_override_count: usize,
    pub universal_property: String,
}

impl TableViewLayoutReport {
    pub fn rendered_text(&self) -> String {
        self.rendered_lines.join("\n")
    }

    pub fn contains_text(&self, needle: &str) -> bool {
        self.rendered_lines.iter().any(|line| line.contains(needle))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLayoutSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub default_enabled: bool,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLayoutBundle;

impl TableViewLayoutBundle {
    pub fn snapshot(&self) -> TableViewLayoutSnapshot {
        TableViewLayoutSnapshot {
            class: "TableViewLayoutSnapshot".to_string(),
            morphisms: vec![
                "measure_column_widths".to_string(),
                "effective_column_widths".to_string(),
                "column_pages_for_widths".to_string(),
                "layout_value_rows".to_string(),
                "layout_materialized_rows".to_string(),
                "layout_smoke_report".to_string(),
            ],
            default_enabled: TableViewLayoutConfig::default().enabled,
            universal_property:
                "one ordered matrix has deterministic shell pages for every width policy"
                    .to_string(),
        }
    }

    pub fn layout_values(
        &self,
        rows: &[Vec<String>],
        config: &TableViewLayoutConfig,
    ) -> TableViewLayoutReport {
        layout_value_rows(rows, config)
    }

    pub fn layout_rows(
        &self,
        rows: &[MaterializedTableViewRow],
        config: &TableViewLayoutConfig,
    ) -> TableViewLayoutReport {
        layout_materialized_rows(rows, config)
    }
}

pub fn bootstrap_table_view_layout() -> TableViewLayoutBundle {
    TableViewLayoutBundle
}

pub fn measure_column_widths(rows: &[Vec<String>]) -> Vec<usize> {
    let column_count = rows.iter().map(Vec::len).max().unwrap_or_default();
    let mut widths = vec![0usize; column_count];
    for row in rows {
        for (index, cell) in row.iter().enumerate() {
            widths[index] = widths[index].max(display_width(cell));
        }
    }
    widths
}

pub fn effective_column_widths(rows: &[Vec<String>], overrides: &[usize]) -> Vec<usize> {
    let measured = measure_column_widths(rows);
    measured
        .iter()
        .enumerate()
        .map(|(index, measured_width)| {
            let override_width = overrides.get(index).copied().unwrap_or_default();
            if override_width == 0 || override_width > *measured_width {
                *measured_width
            } else {
                override_width
            }
        })
        .collect()
}

pub fn column_pages_for_widths(
    widths: &[usize],
    separator_width: usize,
    max_screen_width: Option<usize>,
    onetable: bool,
) -> Vec<TableViewColumnPage> {
    if widths.is_empty() {
        return Vec::new();
    }
    if onetable || max_screen_width.unwrap_or_default() == 0 {
        return vec![page_from_range(0, 0, widths.len(), widths, separator_width)];
    }
    let max_width = max_screen_width.unwrap_or(usize::MAX).max(1);
    let mut pages = Vec::new();
    let mut start = 0usize;
    let mut current_width = 0usize;
    for (index, width) in widths.iter().enumerate() {
        let proposed = if index == start {
            *width
        } else {
            current_width
                .saturating_add(separator_width)
                .saturating_add(*width)
        };
        if index > start && proposed > max_width {
            pages.push(page_from_range(
                pages.len(),
                start,
                index,
                widths,
                separator_width,
            ));
            start = index;
            current_width = *width;
        } else {
            current_width = proposed;
        }
    }
    if start < widths.len() {
        pages.push(page_from_range(
            pages.len(),
            start,
            widths.len(),
            widths,
            separator_width,
        ));
    }
    pages
}

pub fn layout_value_rows(
    rows: &[Vec<String>],
    config: &TableViewLayoutConfig,
) -> TableViewLayoutReport {
    let column_widths = effective_column_widths(rows, &config.width_overrides);
    let pages = column_pages_for_widths(
        &column_widths,
        display_width(&config.separator),
        config.max_screen_width,
        config.onetable,
    );
    let rendered_lines = if config.activates_layout() {
        render_layout_pages(rows, &column_widths, &pages, config)
    } else {
        rows.iter().map(|row| row.join(&config.separator)).collect()
    };
    TableViewLayoutReport {
        class: "TableViewLayoutReport".to_string(),
        enabled: config.activates_layout(),
        source_row_count: rows.len(),
        column_count: column_widths.len(),
        column_widths,
        page_count: pages.len(),
        pages,
        rendered_line_count: rendered_lines.len(),
        rendered_lines,
        max_screen_width: config.max_screen_width,
        onetable: config.onetable,
        pad_cells: config.pad_cells,
        width_override_count: config.width_overrides.len(),
        universal_property: "horizontal column pages glue back to the same ordered row/cell matrix"
            .to_string(),
    }
}

pub fn layout_materialized_rows(
    rows: &[MaterializedTableViewRow],
    config: &TableViewLayoutConfig,
) -> TableViewLayoutReport {
    let value_rows = rows
        .iter()
        .map(|row| row.rendered_values())
        .collect::<Vec<_>>();
    layout_value_rows(&value_rows, config)
}

fn render_layout_pages(
    rows: &[Vec<String>],
    widths: &[usize],
    pages: &[TableViewColumnPage],
    config: &TableViewLayoutConfig,
) -> Vec<String> {
    if pages.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for (page_index, page) in pages.iter().enumerate() {
        if page_index > 0 && config.include_page_separator {
            out.push(config.page_separator.clone());
        }
        for row in rows {
            out.push(render_page_row(row, widths, page, config));
        }
    }
    out
}

fn render_page_row(
    row: &[String],
    widths: &[usize],
    page: &TableViewColumnPage,
    config: &TableViewLayoutConfig,
) -> String {
    let mut values = Vec::new();
    for column in page.start_column..page.end_column_exclusive {
        let value = row.get(column).cloned().unwrap_or_default();
        if config.pad_cells {
            values.push(pad_to_width(
                &value,
                widths.get(column).copied().unwrap_or_default(),
            ));
        } else {
            values.push(value);
        }
    }
    values.join(&config.separator)
}

pub fn pad_to_width(value: &str, width: usize) -> String {
    let actual = display_width(value);
    if actual >= width {
        value.to_string()
    } else {
        format!("{}{}", value, " ".repeat(width - actual))
    }
}

pub fn display_width(value: &str) -> usize {
    value.chars().count()
}

fn page_from_range(
    page_index: usize,
    start_column: usize,
    end_column_exclusive: usize,
    widths: &[usize],
    separator_width: usize,
) -> TableViewColumnPage {
    let column_count = end_column_exclusive.saturating_sub(start_column);
    let content_width = widths[start_column..end_column_exclusive]
        .iter()
        .copied()
        .sum::<usize>();
    let separator_width = separator_width.saturating_mul(column_count.saturating_sub(1));
    TableViewColumnPage {
        page_index,
        start_column,
        end_column_exclusive,
        column_count,
        width: content_width.saturating_add(separator_width),
    }
}

pub fn layout_smoke_report() -> TableViewLayoutReport {
    let rows = vec![
        vec!["A".to_string(), "Breit".to_string(), "C".to_string()],
        vec!["eins".to_string(), "zwei".to_string(), "drei".to_string()],
    ];
    layout_value_rows(
        &rows,
        &TableViewLayoutConfig::enabled_shell()
            .with_width_overrides(vec![0, 4, 0])
            .with_max_screen_width(Some(10)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measured_widths_follow_max_cell_lengths() {
        let rows = vec![
            vec!["A".to_string(), "BBBB".to_string()],
            vec!["AAA".to_string(), "B".to_string()],
        ];
        assert_eq!(measure_column_widths(&rows), vec![3, 4]);
    }

    #[test]
    fn width_override_limits_but_zero_keeps_measured_width() {
        let rows = vec![vec!["abcdef".to_string(), "xy".to_string()]];
        assert_eq!(effective_column_widths(&rows, &[3, 0]), vec![3, 2]);
        assert_eq!(effective_column_widths(&rows, &[10, 1]), vec![6, 1]);
    }

    #[test]
    fn viewport_splits_columns_without_reordering_them() {
        let widths = vec![3, 3, 3];
        let pages = column_pages_for_widths(&widths, 1, Some(7), false);
        assert_eq!(pages.len(), 2);
        assert_eq!(pages[0].start_column, 0);
        assert_eq!(pages[0].end_column_exclusive, 2);
        assert_eq!(pages[1].start_column, 2);
    }

    #[test]
    fn layout_pads_shell_cells_when_enabled() {
        let rows = vec![
            vec!["A".to_string(), "Long".to_string()],
            vec!["ABC".to_string(), "x".to_string()],
        ];
        let report = layout_value_rows(&rows, &TableViewLayoutConfig::enabled_shell());
        assert_eq!(report.rendered_lines[0], "A   | Long");
        assert_eq!(report.rendered_lines[1], "ABC | x   ");
    }

    #[test]
    fn disabled_layout_keeps_compact_join() {
        let rows = vec![vec!["A".to_string(), "Long".to_string()]];
        let report = layout_value_rows(&rows, &TableViewLayoutConfig::default());
        assert!(!report.enabled);
        assert_eq!(report.rendered_lines, vec!["A | Long".to_string()]);
    }
}
