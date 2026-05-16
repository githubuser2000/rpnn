//! HTML class/style projection for materialized table-view cells.
//!
//! Stage 31 connects the Stage-21 `htmlclassesPy.jsonl` catalog to the
//! Stage-23/30 table-view output path.  The catalog was previously kept as a
//! witness only.  This module turns it into a typed, policy-controlled HTML
//! cell attribute morphism.  It remains disabled by default so the legacy output
//! path stays the behaviour oracle until a later guarded commit can prove byte
//! parity.

use serde::{Deserialize, Serialize};

use crate::html_class_catalog::{html_class_record, HtmlClassRecord, HTML_CLASS_RECORDS};
use crate::table_view::{MaterializedTableViewCell, MaterializedTableViewRow};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TableViewHtmlAttributePolicy {
    /// Render plain `<td>…</td>` cells; keep catalog data diagnostic only.
    #[default]
    Plain,
    /// Use catalog class strings, but do not copy inline style attributes.
    ClassOnly,
    /// Use the catalog raw opening tag when available.
    RawOpenTag,
    /// Prefer the catalog raw HTML witness only when it matches the cell text;
    /// otherwise fall back to `RawOpenTag`/`ClassOnly` generation.
    RawHtmlWitness,
}

impl TableViewHtmlAttributePolicy {
    pub fn canonical(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::ClassOnly => "class-only",
            Self::RawOpenTag => "raw-open-tag",
            Self::RawHtmlWitness => "raw-html-witness",
        }
    }

    pub fn uses_catalog(self) -> bool {
        !matches!(self, Self::Plain)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewHtmlAttributeConfig {
    pub enabled: bool,
    pub policy: TableViewHtmlAttributePolicy,
    /// Try an exact `(column,row)` catalog match before text/fallback lookup.
    pub prefer_exact_row: bool,
    /// If the legacy CSV/source column and HTML-output column numbering differ,
    /// use `(row,text)` as a safer witness key before falling back to the raw
    /// column number.  This is important for `493 -> M Kontinuum`, whose
    /// HTML-class witness lives at a different rendered-output column index.
    pub prefer_row_text_match: bool,
    /// Use `(column,None)` witnesses for virtual/non-direct columns. Direct cells only use this fallback when the witness text is compatible.
    pub allow_column_fallback: bool,
    /// Keep inline style attributes when `RawOpenTag` is selected.  Disabled for
    /// class-only mode and for `--nocolor`-like projections.
    pub include_inline_style: bool,
    pub table_open_tag: String,
    pub table_close_tag: String,
}

impl Default for TableViewHtmlAttributeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            policy: TableViewHtmlAttributePolicy::Plain,
            prefer_exact_row: true,
            prefer_row_text_match: true,
            allow_column_fallback: true,
            include_inline_style: true,
            table_open_tag: r#"<table border=0 id="bigtable">"#.to_string(),
            table_close_tag: "</table>".to_string(),
        }
    }
}

impl TableViewHtmlAttributeConfig {
    pub fn class_only() -> Self {
        Self {
            enabled: true,
            policy: TableViewHtmlAttributePolicy::ClassOnly,
            ..Self::default()
        }
    }

    pub fn raw_open_tag() -> Self {
        Self {
            enabled: true,
            policy: TableViewHtmlAttributePolicy::RawOpenTag,
            ..Self::default()
        }
    }

    pub fn raw_html_witness() -> Self {
        Self {
            enabled: true,
            policy: TableViewHtmlAttributePolicy::RawHtmlWitness,
            ..Self::default()
        }
    }

    pub fn disabled() -> Self {
        Self::default()
    }

    pub fn activates_catalog(&self) -> bool {
        self.enabled && self.policy.uses_catalog()
    }

    pub fn without_inline_style(mut self) -> Self {
        self.include_inline_style = false;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewHtmlCellAttribute {
    pub column_legacy: usize,
    pub source_row_zero_based: usize,
    pub record_found: bool,
    pub row_specific: bool,
    pub tag: String,
    pub class_string: String,
    pub raw_open_tag: String,
    pub raw_html: String,
    pub text_witness: String,
    pub rendered_open_tag: String,
    pub rendered_close_tag: String,
    pub source: String,
}

impl TableViewHtmlCellAttribute {
    pub fn plain(cell: &MaterializedTableViewCell) -> Self {
        Self {
            column_legacy: cell.column_legacy,
            source_row_zero_based: cell.source_row_zero_based,
            record_found: false,
            row_specific: false,
            tag: "td".to_string(),
            class_string: String::new(),
            raw_open_tag: "<td>".to_string(),
            raw_html: String::new(),
            text_witness: String::new(),
            rendered_open_tag: "<td>".to_string(),
            rendered_close_tag: "</td>".to_string(),
            source: "plain".to_string(),
        }
    }

    pub fn has_class(&self) -> bool {
        !self.class_string.trim().is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewHtmlAttributeRow {
    pub asset_name: String,
    pub source_row_zero_based: usize,
    pub cells: Vec<TableViewHtmlCellAttribute>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewHtmlAttributeReport {
    pub class: String,
    pub enabled: bool,
    pub policy: String,
    pub row_count: usize,
    pub cell_count: usize,
    pub attributed_cell_count: usize,
    pub class_string_cell_count: usize,
    pub row_specific_match_count: usize,
    pub fallback_match_count: usize,
    pub virtual_744_attributed: bool,
    pub direct_493_attributed: bool,
    pub rendered_line_count: usize,
    pub rendered_lines: Vec<String>,
    pub rows: Vec<TableViewHtmlAttributeRow>,
    pub universal_property: String,
}

impl TableViewHtmlAttributeReport {
    pub fn contains_text(&self, needle: &str) -> bool {
        self.rendered_lines.iter().any(|line| line.contains(needle))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewHtmlAttributeSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub policies: Vec<String>,
    pub default_enabled: bool,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewHtmlAttributeBundle;

impl TableViewHtmlAttributeBundle {
    pub fn snapshot(&self) -> TableViewHtmlAttributeSnapshot {
        TableViewHtmlAttributeSnapshot {
            class: "TableViewHtmlAttributeSnapshot".to_string(),
            morphisms: vec![
                "html_attribute_for_cell".to_string(),
                "html_attribute_rows_for_view_rows".to_string(),
                "render_html_table_with_attributes".to_string(),
                "render_html_cell_with_attribute".to_string(),
                "continuum_m_html_attribute_smoke".to_string(),
            ],
            policies: vec![
                TableViewHtmlAttributePolicy::Plain.canonical().to_string(),
                TableViewHtmlAttributePolicy::ClassOnly.canonical().to_string(),
                TableViewHtmlAttributePolicy::RawOpenTag.canonical().to_string(),
                TableViewHtmlAttributePolicy::RawHtmlWitness.canonical().to_string(),
            ],
            default_enabled: TableViewHtmlAttributeConfig::default().enabled,
            universal_property:
                "each materialized cell has at most one deterministic HTML attribute witness for a chosen policy"
                    .to_string(),
        }
    }

    pub fn report(
        &self,
        rows: &[MaterializedTableViewRow],
        config: &TableViewHtmlAttributeConfig,
    ) -> TableViewHtmlAttributeReport {
        html_attribute_report_for_rows(rows, config, false, true)
    }
}

pub fn bootstrap_table_view_html_attributes() -> TableViewHtmlAttributeBundle {
    TableViewHtmlAttributeBundle
}

pub fn html_attribute_for_cell(
    cell: &MaterializedTableViewCell,
    config: &TableViewHtmlAttributeConfig,
) -> TableViewHtmlCellAttribute {
    if !config.activates_catalog() {
        return TableViewHtmlCellAttribute::plain(cell);
    }
    let column = cell.column_legacy as i64;
    let row = cell.source_row_zero_based as i64;
    let exact = config
        .prefer_exact_row
        .then(|| html_class_record(column, Some(row)))
        .flatten()
        .filter(|record| record_text_is_compatible(record.text, &cell.value));
    if let Some(record) = exact {
        return attribute_from_record(cell, record, true, "row-specific", config);
    }

    let text_match = config
        .prefer_row_text_match
        .then(|| find_html_record_by_row_text(row, &cell.value))
        .flatten();
    if let Some(record) = text_match {
        return attribute_from_record(cell, record, true, "row-text-match", config);
    }

    let fallback = config
        .allow_column_fallback
        .then(|| html_class_record(column, None))
        .flatten()
        .filter(|record| cell.is_virtual() || record_text_is_compatible(record.text, &cell.value));
    match fallback {
        Some(record) => attribute_from_record(cell, record, false, "column-fallback", config),
        None => TableViewHtmlCellAttribute::plain(cell),
    }
}

fn attribute_from_record(
    cell: &MaterializedTableViewCell,
    record: HtmlClassRecord,
    row_specific: bool,
    source_label: &str,
    config: &TableViewHtmlAttributeConfig,
) -> TableViewHtmlCellAttribute {
    let rendered_open_tag = match config.policy {
        TableViewHtmlAttributePolicy::Plain => "<td>".to_string(),
        TableViewHtmlAttributePolicy::ClassOnly => class_only_open_tag(record.class_string),
        TableViewHtmlAttributePolicy::RawOpenTag | TableViewHtmlAttributePolicy::RawHtmlWitness => {
            cleaned_open_tag(record.raw_open_tag, config.include_inline_style)
                .unwrap_or_else(|| class_only_open_tag(record.class_string))
        }
    };
    TableViewHtmlCellAttribute {
        column_legacy: cell.column_legacy,
        source_row_zero_based: cell.source_row_zero_based,
        record_found: true,
        row_specific,
        tag: if record.tag.is_empty() { "td" } else { record.tag }.to_string(),
        class_string: record.class_string.to_string(),
        raw_open_tag: record.raw_open_tag.to_string(),
        raw_html: record.raw_html.to_string(),
        text_witness: record.text.to_string(),
        rendered_open_tag,
        rendered_close_tag: "</td>".to_string(),
        source: source_label.to_string(),
    }
}

pub fn find_html_record_by_row_text(row_number: i64, text: &str) -> Option<HtmlClassRecord> {
    let wanted = normalize_html_witness_text(text);
    if wanted.is_empty() {
        return None;
    }
    HTML_CLASS_RECORDS.iter().copied().find(|record| {
        record.row_number == Some(row_number)
            && record_text_is_compatible(record.text, text)
            && normalize_html_witness_text(record.text) == wanted
    })
}

fn record_text_is_compatible(record_text: &str, cell_value: &str) -> bool {
    let record = normalize_html_witness_text(record_text);
    let cell = normalize_html_witness_text(cell_value);
    record.is_empty() || record == cell
}

fn normalize_html_witness_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn html_attribute_rows_for_view_rows(
    rows: &[MaterializedTableViewRow],
    config: &TableViewHtmlAttributeConfig,
    suppress_headers: bool,
    include_empty_rows: bool,
) -> Vec<TableViewHtmlAttributeRow> {
    rows.iter()
        .filter(|row| !(suppress_headers && row.source_row_zero_based == 0))
        .filter(|row| include_empty_rows || !row.cells.is_empty())
        .map(|row| TableViewHtmlAttributeRow {
            asset_name: row.asset_name.clone(),
            source_row_zero_based: row.source_row_zero_based,
            cells: row
                .cells
                .iter()
                .map(|cell| html_attribute_for_cell(cell, config))
                .collect(),
        })
        .collect()
}

pub fn html_attribute_report_for_rows(
    rows: &[MaterializedTableViewRow],
    config: &TableViewHtmlAttributeConfig,
    suppress_headers: bool,
    include_empty_rows: bool,
) -> TableViewHtmlAttributeReport {
    let attribute_rows = html_attribute_rows_for_view_rows(
        rows,
        config,
        suppress_headers,
        include_empty_rows,
    );
    let rendered_lines = render_html_table_with_attribute_rows(rows, &attribute_rows, config);
    let cells = attribute_rows
        .iter()
        .flat_map(|row| row.cells.iter())
        .collect::<Vec<_>>();
    TableViewHtmlAttributeReport {
        class: "TableViewHtmlAttributeReport".to_string(),
        enabled: config.enabled,
        policy: config.policy.canonical().to_string(),
        row_count: attribute_rows.len(),
        cell_count: cells.len(),
        attributed_cell_count: cells.iter().filter(|cell| cell.record_found).count(),
        class_string_cell_count: cells.iter().filter(|cell| cell.has_class()).count(),
        row_specific_match_count: cells.iter().filter(|cell| cell.row_specific).count(),
        fallback_match_count: cells
            .iter()
            .filter(|cell| cell.record_found && !cell.row_specific)
            .count(),
        virtual_744_attributed: cells
            .iter()
            .any(|cell| cell.column_legacy == 744 && cell.record_found),
        direct_493_attributed: cells
            .iter()
            .any(|cell| cell.column_legacy == 493 && cell.record_found),
        rendered_line_count: rendered_lines.len(),
        rendered_lines,
        rows: attribute_rows,
        universal_property:
            "html-class witnesses remain policy-controlled local sections before visible HTML commit"
                .to_string(),
    }
}

pub fn render_html_table_with_attributes(
    rows: &[MaterializedTableViewRow],
    config: &TableViewHtmlAttributeConfig,
    suppress_headers: bool,
    include_empty_rows: bool,
) -> Vec<String> {
    let attribute_rows = html_attribute_rows_for_view_rows(
        rows,
        config,
        suppress_headers,
        include_empty_rows,
    );
    render_html_table_with_attribute_rows(rows, &attribute_rows, config)
}

fn render_html_table_with_attribute_rows(
    rows: &[MaterializedTableViewRow],
    attribute_rows: &[TableViewHtmlAttributeRow],
    config: &TableViewHtmlAttributeConfig,
) -> Vec<String> {
    if attribute_rows.is_empty() {
        return Vec::new();
    }
    let mut out = vec![config.table_open_tag.clone()];
    for attribute_row in attribute_rows {
        out.push("<tr>".to_string());
        if let Some(row) = rows.iter().find(|row| {
            row.asset_name == attribute_row.asset_name
                && row.source_row_zero_based == attribute_row.source_row_zero_based
        }) {
            for (cell, attr) in row.cells.iter().zip(attribute_row.cells.iter()) {
                out.push(render_html_cell_with_attribute(cell, attr, config));
            }
        }
        out.push("</tr>".to_string());
    }
    out.push(config.table_close_tag.clone());
    out
}

pub fn render_html_cell_with_attribute(
    cell: &MaterializedTableViewCell,
    attribute: &TableViewHtmlCellAttribute,
    config: &TableViewHtmlAttributeConfig,
) -> String {
    if config.policy == TableViewHtmlAttributePolicy::RawHtmlWitness
        && attribute.record_found
        && !attribute.raw_html.trim().is_empty()
        && raw_html_text_matches(&attribute.raw_html, &cell.value)
    {
        return attribute.raw_html.clone();
    }
    format!(
        "{}{}{}",
        attribute.rendered_open_tag,
        html_escape_cell_for_value(&cell.value),
        attribute.rendered_close_tag
    )
}

fn class_only_open_tag(class_string: &str) -> String {
    let class_string = class_string.trim();
    if class_string.is_empty() {
        "<td>".to_string()
    } else {
        format!("<td class=\"{}\">", html_escape_attribute(class_string))
    }
}

fn cleaned_open_tag(raw_open_tag: &str, include_inline_style: bool) -> Option<String> {
    let raw = raw_open_tag.trim();
    if raw.is_empty() || !raw.starts_with("<td") || !raw.ends_with('>') {
        return None;
    }
    if include_inline_style {
        return Some(raw.to_string());
    }
    Some(strip_style_attribute(raw))
}

fn strip_style_attribute(raw_open_tag: &str) -> String {
    let mut out = raw_open_tag.to_string();
    while let Some(start) = out.find(" style=\"") {
        let value_start = start + " style=\"".len();
        if let Some(end_rel) = out[value_start..].find('"') {
            out.replace_range(start..value_start + end_rel + 1, "");
        } else {
            break;
        }
    }
    out
}

fn html_escape_cell_for_value(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn html_escape_attribute(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn raw_html_text_matches(raw_html: &str, cell_value: &str) -> bool {
    let normalized_cell = cell_value.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized_cell.is_empty() {
        return true;
    }
    raw_html.contains(&normalized_cell)
}

pub fn continuum_m_html_attribute_smoke() -> TableViewHtmlAttributeReport {
    let args = [
        "reta",
        "-zeilen",
        "--vorhervonausschnitt=1-1",
        "-spalten",
        "--kontinuum=m",
        "-ausgabe",
        "--spaltenreihenfolgeundnurdiese=744,493",
    ];
    let view = crate::table_view::view_for_cli_args(
        &args,
        &crate::table_materialization::TableMaterializationConfig::default(),
        &crate::table_view::MaterializedTableViewConfig::default()
            .with_virtual_policy(crate::table_view::VirtualColumnDisplayPolicy::TagSummary),
    );
    html_attribute_report_for_rows(
        &view.rows,
        &TableViewHtmlAttributeConfig::class_only(),
        false,
        true,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table_materialization::TableMaterializationConfig;
    use crate::table_view::{view_for_cli_args, MaterializedTableViewConfig, VirtualColumnDisplayPolicy};

    #[test]
    fn continuum_m_html_attribute_smoke_keeps_493_and_treats_744_as_direct_csv_cell() {
        let report = continuum_m_html_attribute_smoke();
        assert!(report.enabled);
        assert_eq!(report.policy, "class-only");
        assert!(report.direct_493_attributed);
        assert!(!report.virtual_744_attributed);
        assert!(report.class_string_cell_count > 0);
        assert!(report.contains_text("class=\""));
        assert!(report.contains_text("M Kontinuum"));
        assert!(report.contains_text("Neues M"));
        assert!(!report.contains_text("744:sternPolygon"));
    }

    #[test]
    fn html_attribute_policy_is_plain_by_default() {
        let args = [
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--kontinuum=m",
        ];
        let view = view_for_cli_args(
            &args,
            &TableMaterializationConfig::default(),
            &MaterializedTableViewConfig::default(),
        );
        let report = html_attribute_report_for_rows(
            &view.rows,
            &TableViewHtmlAttributeConfig::default(),
            false,
            true,
        );
        assert!(!report.enabled);
        assert_eq!(report.attributed_cell_count, 0);
    }

    #[test]
    fn raw_open_tag_can_strip_inline_style() {
        let args = [
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--kontinuum=m",
        ];
        let view = view_for_cli_args(
            &args,
            &TableMaterializationConfig::default(),
            &MaterializedTableViewConfig::default()
                .with_virtual_policy(VirtualColumnDisplayPolicy::TagSummary),
        );
        let config = TableViewHtmlAttributeConfig::raw_open_tag().without_inline_style();
        let report = html_attribute_report_for_rows(&view.rows, &config, false, true);
        assert!(!report.rendered_lines.iter().any(|line| line.contains("style=\"")));
    }
}
