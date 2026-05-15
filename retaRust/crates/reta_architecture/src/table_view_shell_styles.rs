//! Policy-controlled ANSI shell style projection for materialized table-view output.
//!
//! Stage 36 connects the old shell `colorize` semantics to the new
//! `MaterializedTableView` renderer.  HTML and BBCode styling already have
//! row/cell wrapper stages; shell output needs a separate ANSI projection
//! because the bytes live inside the cell text itself.  The projection remains
//! disabled by default and is intended for shadow/parity diagnostics before any
//! guarded commit can use it visibly.

use serde::{Deserialize, Serialize};

use crate::output_syntax::OutputMode;
use crate::table_output::colorize;
use crate::table_view::{MaterializedTableCellSource, MaterializedTableViewRow};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum TableViewShellStylePolicy {
    /// Plain shell text, no ANSI escape projection.
    #[default]
    Plain,
    /// Use the Rust image of Python's shell `colorize` branch.
    LegacyColorize,
    /// Same projection, but all candidates are retained as diagnostic witnesses.
    LegacyColorizeWitness,
}

impl TableViewShellStylePolicy {
    pub fn canonical(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::LegacyColorize => "legacy-colorize",
            Self::LegacyColorizeWitness => "legacy-colorize-witness",
        }
    }

    pub fn uses_colorize(self) -> bool {
        matches!(self, Self::LegacyColorize | Self::LegacyColorizeWitness)
    }

    pub fn is_witness(self) -> bool {
        matches!(self, Self::LegacyColorizeWitness)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewShellStyleConfig {
    pub enabled: bool,
    pub policy: TableViewShellStylePolicy,
    /// Colour the source/header row.  The historical `colorize(num=0)` branch
    /// is red/underlined and therefore kept opt-in through this policy.
    pub include_header_row: bool,
    /// Colour numbering prefix cells when numbering is explicitly enabled.
    pub include_numbering_prefixes: bool,
    /// Continuation lines caused by wrapping stay plain by default so ANSI
    /// bytes do not disturb width/layout diagnostics.
    pub rest_lines_plain: bool,
    pub apply_shell: bool,
}

impl Default for TableViewShellStyleConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            policy: TableViewShellStylePolicy::Plain,
            include_header_row: true,
            include_numbering_prefixes: true,
            rest_lines_plain: true,
            apply_shell: true,
        }
    }
}

impl TableViewShellStyleConfig {
    pub fn disabled() -> Self {
        Self::default()
    }

    pub fn legacy_colorize() -> Self {
        Self {
            enabled: true,
            policy: TableViewShellStylePolicy::LegacyColorize,
            ..Self::default()
        }
    }

    pub fn legacy_colorize_witness() -> Self {
        Self {
            enabled: true,
            policy: TableViewShellStylePolicy::LegacyColorizeWitness,
            ..Self::default()
        }
    }

    pub fn activates_mode(&self, mode: OutputMode) -> bool {
        self.enabled && self.apply_shell && mode == OutputMode::Shell && self.policy.uses_colorize()
    }

    pub fn without_color(mut self) -> Self {
        self.enabled = false;
        self.policy = TableViewShellStylePolicy::Plain;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewShellStyle {
    pub mode: String,
    pub source_row_zero_based: usize,
    pub display_cell_index: usize,
    pub data_cell_index: Option<usize>,
    pub column_legacy: Option<usize>,
    pub pseudo_column: Option<i64>,
    pub continuation_line: bool,
    pub virtual_cell: bool,
    pub row_number_for_style: i64,
    pub styled: bool,
    pub ansi_present: bool,
    pub plain_preview: String,
    pub styled_preview: String,
    pub color_signature: String,
    pub source: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewShellStyleReport {
    pub class: String,
    pub enabled: bool,
    pub policy: String,
    pub row_count: usize,
    pub line_count: usize,
    pub cell_count: usize,
    pub styled_cell_count: usize,
    pub ansi_cell_count: usize,
    pub header_cell_count: usize,
    pub numbering_prefix_cell_count: usize,
    pub virtual_cell_count: usize,
    pub cells: Vec<TableViewShellStyle>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewShellStyleSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub policies: Vec<String>,
    pub default_enabled: bool,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewShellStyleBundle;

impl TableViewShellStyleBundle {
    pub fn snapshot(&self) -> TableViewShellStyleSnapshot {
        TableViewShellStyleSnapshot {
            class: "TableViewShellStyleSnapshot".to_string(),
            morphisms: vec![
                "shell_style_for_output_value".to_string(),
                "colorize_shell_output_value".to_string(),
                "shell_style_report_for_rows".to_string(),
                "continuum_m_shell_style_smoke".to_string(),
            ],
            policies: vec![
                TableViewShellStylePolicy::Plain.canonical().to_string(),
                TableViewShellStylePolicy::LegacyColorize
                    .canonical()
                    .to_string(),
                TableViewShellStylePolicy::LegacyColorizeWitness
                    .canonical()
                    .to_string(),
            ],
            default_enabled: TableViewShellStyleConfig::default().enabled,
            universal_property:
                "ANSI shell colouring is a local output projection; stripping ANSI preserves cells"
                    .to_string(),
        }
    }

    pub fn report(
        &self,
        rows: &[MaterializedTableViewRow],
        config: &TableViewShellStyleConfig,
        suppress_headers: bool,
        include_empty_rows: bool,
        prefix_column_count: usize,
    ) -> TableViewShellStyleReport {
        shell_style_report_for_rows(
            rows,
            config,
            suppress_headers,
            include_empty_rows,
            prefix_column_count,
        )
    }
}

pub fn bootstrap_table_view_shell_styles() -> TableViewShellStyleBundle {
    TableViewShellStyleBundle
}

pub fn shell_style_for_output_value(
    row: &MaterializedTableViewRow,
    value: &str,
    display_cell_index: usize,
    continuation_line: bool,
    prefix_column_count: usize,
    config: &TableViewShellStyleConfig,
) -> TableViewShellStyle {
    let pseudo_column = if display_cell_index < prefix_column_count {
        Some(if prefix_column_count >= 2 && display_cell_index == 0 {
            -2
        } else {
            -1
        })
    } else {
        None
    };
    let data_cell_index = pseudo_column
        .is_none()
        .then_some(display_cell_index - prefix_column_count);
    let data_cell = data_cell_index.and_then(|index| row.cells.get(index));
    let column_legacy = data_cell.map(|cell| cell.column_legacy);
    let virtual_cell = data_cell
        .map(|cell| matches!(cell.source, MaterializedTableCellSource::Virtual { .. }))
        .unwrap_or(false);
    let is_header = row.source_row_zero_based == 0;
    let is_prefix = pseudo_column.is_some();
    let should_style = config.activates_mode(OutputMode::Shell)
        && (config.include_header_row || !is_header)
        && (config.include_numbering_prefixes || !is_prefix)
        && (!continuation_line || !config.rest_lines_plain);
    let row_number = row.source_row_zero_based as i64;
    let styled_preview = if should_style {
        colorize(value, row_number, continuation_line)
    } else {
        value.to_string()
    };
    let ansi_present = styled_preview.contains("\u{1b}[");
    let styled = should_style && (config.policy.is_witness() || ansi_present);
    TableViewShellStyle {
        mode: OutputMode::Shell.canonical_name().to_string(),
        source_row_zero_based: row.source_row_zero_based,
        display_cell_index,
        data_cell_index,
        column_legacy,
        pseudo_column,
        continuation_line,
        virtual_cell,
        row_number_for_style: row_number,
        styled,
        ansi_present,
        plain_preview: value.to_string(),
        styled_preview,
        color_signature: shell_color_signature(row_number, continuation_line),
        source: if should_style {
            "table_output.colorize".to_string()
        } else {
            "plain-shell-cell".to_string()
        },
    }
}

pub fn colorize_shell_output_value(
    row: &MaterializedTableViewRow,
    value: &str,
    display_cell_index: usize,
    continuation_line: bool,
    prefix_column_count: usize,
    config: &TableViewShellStyleConfig,
) -> String {
    shell_style_for_output_value(
        row,
        value,
        display_cell_index,
        continuation_line,
        prefix_column_count,
        config,
    )
    .styled_preview
}

pub fn shell_style_report_for_rows(
    rows: &[MaterializedTableViewRow],
    config: &TableViewShellStyleConfig,
    suppress_headers: bool,
    include_empty_rows: bool,
    prefix_column_count: usize,
) -> TableViewShellStyleReport {
    let mut cells = Vec::new();
    let mut line_count = 0usize;
    for row in rows
        .iter()
        .filter(|row| !(suppress_headers && row.source_row_zero_based == 0))
        .filter(|row| include_empty_rows || !row.cells.is_empty())
    {
        let value_count = row.cells.len() + prefix_column_count;
        if value_count == 0 {
            line_count += 1;
            continue;
        }
        line_count += 1;
        for value_index in 0..value_count {
            let value = if value_index < prefix_column_count {
                if prefix_column_count >= 2 && value_index == 0 {
                    "Zählung"
                } else {
                    "Nummerierung"
                }
            } else {
                row.cells
                    .get(value_index - prefix_column_count)
                    .map(|cell| cell.value.as_str())
                    .unwrap_or("")
            };
            cells.push(shell_style_for_output_value(
                row,
                value,
                value_index,
                false,
                prefix_column_count,
                config,
            ));
        }
    }
    let styled_cell_count = cells.iter().filter(|cell| cell.styled).count();
    let ansi_cell_count = cells.iter().filter(|cell| cell.ansi_present).count();
    let header_cell_count = cells
        .iter()
        .filter(|cell| cell.source_row_zero_based == 0 && cell.styled)
        .count();
    let numbering_prefix_cell_count = cells
        .iter()
        .filter(|cell| cell.pseudo_column.is_some() && cell.styled)
        .count();
    let virtual_cell_count = cells.iter().filter(|cell| cell.virtual_cell).count();
    TableViewShellStyleReport {
        class: "TableViewShellStyleReport".to_string(),
        enabled: config.enabled,
        policy: config.policy.canonical().to_string(),
        row_count: rows.len(),
        line_count,
        cell_count: cells.len(),
        styled_cell_count,
        ansi_cell_count,
        header_cell_count,
        numbering_prefix_cell_count,
        virtual_cell_count,
        cells,
        universal_property:
            "ANSI style projection changes only shell escape bytes; cell values remain recoverable by stripping ANSI"
                .to_string(),
    }
}

pub fn shell_color_signature(row_number: i64, rest: bool) -> String {
    if row_number == 0 {
        return "header-red-underlined".to_string();
    }
    if rest {
        return if row_number % 2 == 0 {
            "rest-even-light".to_string()
        } else {
            "rest-odd-dark".to_string()
        };
    }
    let sample = colorize("x", row_number, false);
    if sample.contains("[106m") || sample.contains("[46m") {
        "moon".to_string()
    } else if sample.contains("[103m") || sample.contains("[43m") {
        "prime".to_string()
    } else if sample.contains("[47m") {
        "even".to_string()
    } else if sample.contains("[100m") {
        "odd".to_string()
    } else {
        "plain".to_string()
    }
}

pub fn continuum_m_shell_style_smoke() -> TableViewShellStyleReport {
    let view = crate::table_view::view_for_cli_args(
        &[
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--kontinuum=m",
        ],
        &crate::table_materialization::TableMaterializationConfig::default(),
        &crate::table_view::MaterializedTableViewConfig::default(),
    );
    shell_style_report_for_rows(
        &view.rows,
        &TableViewShellStyleConfig::legacy_colorize(),
        false,
        true,
        0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_style_is_disabled_by_default() {
        let report = continuum_m_shell_style_smoke();
        assert!(report.ansi_cell_count > 0);
        assert!(!TableViewShellStyleConfig::default().activates_mode(OutputMode::Shell));
    }

    #[test]
    fn colorized_shell_cell_contains_ansi_and_preserves_plain_preview() {
        let view = crate::table_view::view_for_cli_args(
            &[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=1-1",
                "-spalten",
                "--kontinuum=m",
            ],
            &crate::table_materialization::TableMaterializationConfig::default(),
            &crate::table_view::MaterializedTableViewConfig::default(),
        );
        let row = view.rows.first().expect("header row exists");
        let value = row
            .cells
            .first()
            .map(|cell| cell.value.as_str())
            .unwrap_or("");
        let style = shell_style_for_output_value(
            row,
            value,
            0,
            false,
            0,
            &TableViewShellStyleConfig::legacy_colorize(),
        );
        assert_eq!(style.plain_preview, value);
        assert!(style.styled_preview.contains("\u{1b}["));
        assert!(style.ansi_present);
    }

    #[test]
    fn nocolor_policy_can_disable_shell_styles() {
        let config = TableViewShellStyleConfig::legacy_colorize().without_color();
        assert!(!config.activates_mode(OutputMode::Shell));
    }
}
