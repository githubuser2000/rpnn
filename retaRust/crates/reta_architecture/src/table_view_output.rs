//! Output rendering for materialized table views.
//!
//! Stage 23 introduced deterministic output-mode projections for a
//! `MaterializedTableView`.  Stage 28 moves the next visible-output knobs into
//! the same typed path: output flags such as `--keineueberschriften`,
//! `--keineleereninhalte`, `--breite=…`, `--breiten=…`, `--dontwrap`,
//! `--nocolor`, `--justtext`, `--onetable`, `--endlessscreen` and `--endless`.
//! Stage 29 adds the legacy numbering/counting prefix as an explicit projection
//! instead of leaving it hidden inside the renderer.
//! Stage 31 connects `htmlclassesPy.jsonl` witnesses to the HTML projection via
//! a disabled-by-default class/style policy.
//! Stage 32 adds a disabled-by-default row-style projection backed by the
//! legacy `coloredBeginCol` output syntax.
//! Stage 33 adds a disabled-by-default cell-style projection backed by the
//! legacy `generateCell`/`generate_cell_begin` syntax.
//! Stage 36 adds a disabled-by-default shell ANSI style projection backed by
//! the legacy `table_output.colorize` function.
//! Stage 37 makes virtual/non-direct columns explicitly render-policy controlled
//! through CLI/shadow flags while keeping suppression as the default.
//! The legacy renderer is still the behaviour oracle; this module makes those
//! options inspectable and shadow-comparable before any guarded commit.

use serde::{Deserialize, Serialize};

use crate::output_syntax::OutputMode;
use crate::parameter_runtime::bootstrap_parameter_runtime;
use crate::table_materialization::{TableMaterializationConfig, bootstrap_table_materialization};
use crate::table_view::{
    MaterializedTableView, MaterializedTableViewCell, MaterializedTableViewConfig,
    MaterializedTableViewRow, VirtualColumnDisplayPolicy, bootstrap_table_view,
};
use crate::table_view_cell_styles::{
    TableViewCellStyleConfig, TableViewCellStylePolicy, TableViewCellStyleReport,
    cell_style_report_for_rows, styled_begin_cell_for_output_value, styled_end_cell_for_mode,
};
use crate::table_view_html_attributes::{
    TableViewHtmlAttributeConfig, TableViewHtmlAttributePolicy, TableViewHtmlAttributeReport,
    html_attribute_for_cell, html_attribute_report_for_rows, render_html_table_with_attributes,
};
use crate::table_view_layout::{TableViewLayoutConfig, TableViewLayoutReport, layout_value_rows};
use crate::table_view_numbering::{
    TableViewNumberingConfig, TableViewNumberingMode, numbering_values_for_source_row,
};
use crate::table_view_row_styles::{
    TableViewRowStyleConfig, TableViewRowStyleReport, row_style_report_for_rows,
    styled_begin_row_for_row,
};
use crate::table_view_shell_styles::{
    TableViewShellStyleConfig, TableViewShellStylePolicy, TableViewShellStyleReport,
    colorize_shell_output_value, shell_style_report_for_rows,
};
use crate::table_view_virtual_columns::{
    TableViewVirtualColumnCliOptions, parse_table_view_virtual_column_cli_options,
};
use crate::table_wrapping::alxwrap;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputCliOptions {
    pub nocolor: bool,
    pub justtext: bool,
    pub onetable: bool,
    pub endlessscreen: bool,
    pub endless: bool,
    pub dontwrap: bool,
    pub keineleereninhalte: bool,
    pub keinenummerierung: bool,
    pub keineueberschriften: bool,
    pub htmlclasses: bool,
    pub htmlrawclasses: bool,
    pub htmlclasswitness: bool,
    pub rowcolors: bool,
    pub rowcolorwitness: bool,
    pub cellstyles: bool,
    pub cellstylewitness: bool,
    pub shellcolors: bool,
    pub shellcolorwitness: bool,
    pub virtualcolumns: bool,
    pub virtualplaceholder: bool,
    pub virtualquestionmarks: bool,
    pub virtualwitness: bool,
    pub suppressvirtualcolumns: bool,
    pub virtual_column_options: TableViewVirtualColumnCliOptions,
    pub width: Option<usize>,
    pub widths: Vec<usize>,
    pub recognized_option_count: usize,
    pub unknown_output_options: Vec<String>,
}

impl Default for TableViewOutputCliOptions {
    fn default() -> Self {
        Self {
            nocolor: false,
            justtext: false,
            onetable: false,
            endlessscreen: false,
            endless: false,
            dontwrap: false,
            keineleereninhalte: false,
            keinenummerierung: false,
            keineueberschriften: false,
            htmlclasses: false,
            htmlrawclasses: false,
            htmlclasswitness: false,
            rowcolors: false,
            rowcolorwitness: false,
            cellstyles: false,
            cellstylewitness: false,
            shellcolors: false,
            shellcolorwitness: false,
            virtualcolumns: false,
            virtualplaceholder: false,
            virtualquestionmarks: false,
            virtualwitness: false,
            suppressvirtualcolumns: false,
            virtual_column_options: TableViewVirtualColumnCliOptions::default(),
            width: None,
            widths: Vec::new(),
            recognized_option_count: 0,
            unknown_output_options: Vec::new(),
        }
    }
}

impl TableViewOutputCliOptions {
    pub fn from_args<S: AsRef<str>>(args: &[S]) -> Self {
        parse_table_view_output_cli_options(args)
    }

    pub fn has_visible_layout_effect(&self) -> bool {
        self.keineleereninhalte
            || self.keineueberschriften
            || self.width.is_some()
            || !self.widths.is_empty()
            || self.dontwrap
            || self.onetable
            || self.endlessscreen
            || self.endless
            || self.htmlclasses
            || self.htmlrawclasses
            || self.htmlclasswitness
            || self.rowcolors
            || self.rowcolorwitness
            || self.cellstyles
            || self.cellstylewitness
            || self.shellcolors
            || self.shellcolorwitness
            || self.virtualcolumns
            || self.virtualplaceholder
            || self.virtualquestionmarks
            || self.virtualwitness
            || self.suppressvirtualcolumns
    }

    pub fn apply_to_config(&self, base: &TableViewOutputConfig) -> TableViewOutputConfig {
        let mut config = base.clone();
        config.nocolor |= self.nocolor;
        config.justtext |= self.justtext;
        config.onetable |= self.onetable;
        config.endlessscreen |= self.endlessscreen;
        config.endless |= self.endless;
        config.dontwrap |= self.dontwrap;
        config.suppress_headers |= self.keineueberschriften;
        if self.keineleereninhalte {
            config.include_empty_rows = false;
        }
        if self.keinenummerierung {
            config.include_row_numbers = false;
            config.numbering = config.numbering.clone().disabled_by_keinenummerierung();
        }
        if !self.widths.is_empty() {
            config.per_column_widths = self.widths.clone();
        }
        if self.htmlclasses || self.htmlrawclasses || self.htmlclasswitness {
            config.html_attributes.enabled = true;
            config.html_attributes.policy = if self.htmlclasswitness {
                config.virtual_column_policy = VirtualColumnDisplayPolicy::TagSummary;
                TableViewHtmlAttributePolicy::RawHtmlWitness
            } else if self.htmlrawclasses {
                TableViewHtmlAttributePolicy::RawOpenTag
            } else {
                TableViewHtmlAttributePolicy::ClassOnly
            };
        }
        if self.rowcolors || self.rowcolorwitness {
            config.row_styles = TableViewRowStyleConfig::legacy_colored();
        }
        if self.cellstyles || self.cellstylewitness {
            config.cell_styles = if self.cellstylewitness {
                TableViewCellStyleConfig::legacy_generate_cell_witness()
            } else {
                TableViewCellStyleConfig::legacy_generate_cell()
            };
        }
        if self.shellcolors || self.shellcolorwitness {
            config.shell_styles = if self.shellcolorwitness {
                TableViewShellStyleConfig::legacy_colorize_witness()
            } else {
                TableViewShellStyleConfig::legacy_colorize()
            };
        }
        if self.suppressvirtualcolumns {
            config.virtual_column_policy = VirtualColumnDisplayPolicy::Suppress;
            config.suppress_question_mark_virtuals = true;
        }
        if self.virtualcolumns {
            config.virtual_column_policy = VirtualColumnDisplayPolicy::TagSummary;
        }
        if self.virtualplaceholder || self.virtualquestionmarks {
            config.virtual_column_policy = VirtualColumnDisplayPolicy::Placeholder;
            config.suppress_question_mark_virtuals = false;
        }
        if self.virtualwitness {
            config.virtual_column_policy = VirtualColumnDisplayPolicy::Witness;
            config.suppress_question_mark_virtuals = false;
        }
        config.virtual_column_options = self.virtual_column_options.clone();
        if let Some(policy) = self.virtual_column_options.policy {
            config.virtual_column_policy = policy;
        }
        if let Some(suppress) = self.virtual_column_options.suppress_question_mark_virtuals {
            config.suppress_question_mark_virtuals = suppress;
        }
        if config.nocolor {
            config.html_attributes.include_inline_style = false;
            config.row_styles = config.row_styles.clone().without_color();
            config.cell_styles = config.cell_styles.clone().without_color();
            config.shell_styles = config.shell_styles.clone().without_color();
        }
        if self.width.is_some() || self.dontwrap {
            config.wrap_cell_width = self.width;
        }
        if config.dontwrap {
            config.wrap_cell_width = None;
            config.per_column_widths.clear();
        }
        config.layout.separator = config.shell_separator.clone();
        config.layout.width_overrides = config.per_column_widths.clone();
        config.layout.onetable = config.onetable;
        // Activate shell-layout only for explicit layout-affecting CLI options.
        // Default output stays byte-stable with earlier shadow stages.
        config.layout.enabled |= !config.per_column_widths.is_empty()
            || config.onetable
            || config.endlessscreen
            || config.endless;
        config.cli_options = self.clone();
        config
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputConfig {
    pub mode: OutputMode,
    pub shell_separator: String,
    pub csv_separator: char,
    pub include_markdown_header_separator: bool,
    pub include_empty_rows: bool,
    pub virtual_column_policy: VirtualColumnDisplayPolicy,
    pub suppress_question_mark_virtuals: bool,
    pub virtual_column_options: TableViewVirtualColumnCliOptions,
    /// Drop source/header rows before output rendering.  This corresponds to
    /// Python's `--keineueberschriften` flag.
    pub suppress_headers: bool,
    /// Rust keeps numbering disabled by default because older stages did not
    /// render row numbers.  The flag is still modeled so `--keinenummerierung`
    /// has an explicit target and future parity work can turn numbering on via
    /// policy instead of implicit renderer side effects.
    pub include_row_numbers: bool,
    pub row_number_header: String,
    pub numbering: TableViewNumberingConfig,
    pub layout: TableViewLayoutConfig,
    pub html_attributes: TableViewHtmlAttributeConfig,
    pub row_styles: TableViewRowStyleConfig,
    pub cell_styles: TableViewCellStyleConfig,
    pub shell_styles: TableViewShellStyleConfig,
    pub wrap_cell_width: Option<usize>,
    pub per_column_widths: Vec<usize>,
    pub dontwrap: bool,
    pub nocolor: bool,
    pub justtext: bool,
    pub onetable: bool,
    pub endlessscreen: bool,
    pub endless: bool,
    pub cli_options: TableViewOutputCliOptions,
}

impl Default for TableViewOutputConfig {
    fn default() -> Self {
        Self {
            mode: OutputMode::Shell,
            shell_separator: " | ".to_string(),
            csv_separator: ';',
            include_markdown_header_separator: true,
            include_empty_rows: true,
            virtual_column_policy: VirtualColumnDisplayPolicy::Suppress,
            suppress_question_mark_virtuals: true,
            virtual_column_options: TableViewVirtualColumnCliOptions::default(),
            suppress_headers: false,
            include_row_numbers: false,
            row_number_header: "#".to_string(),
            numbering: TableViewNumberingConfig::disabled(),
            layout: TableViewLayoutConfig::default(),
            html_attributes: TableViewHtmlAttributeConfig::default(),
            row_styles: TableViewRowStyleConfig::default(),
            cell_styles: TableViewCellStyleConfig::default(),
            shell_styles: TableViewShellStyleConfig::default(),
            wrap_cell_width: None,
            per_column_widths: Vec::new(),
            dontwrap: false,
            nocolor: false,
            justtext: false,
            onetable: false,
            endlessscreen: false,
            endless: false,
            cli_options: TableViewOutputCliOptions::default(),
        }
    }
}

impl TableViewOutputConfig {
    pub fn with_mode(mut self, mode: OutputMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_virtual_policy(mut self, policy: VirtualColumnDisplayPolicy) -> Self {
        self.virtual_column_policy = policy;
        self
    }

    pub fn with_cli_options(mut self, options: TableViewOutputCliOptions) -> Self {
        self = options.apply_to_config(&self);
        self
    }

    pub fn with_wrap_width(mut self, width: Option<usize>) -> Self {
        self.wrap_cell_width = width.filter(|value| *value > 0);
        self
    }

    pub fn with_legacy_numbering(mut self) -> Self {
        self.include_row_numbers = true;
        self.numbering = TableViewNumberingConfig::legacy_pair();
        self
    }

    pub fn with_html_attributes(mut self, html_attributes: TableViewHtmlAttributeConfig) -> Self {
        self.html_attributes = html_attributes;
        self
    }

    pub fn with_cell_styles(mut self, cell_styles: TableViewCellStyleConfig) -> Self {
        self.cell_styles = cell_styles;
        self
    }

    pub fn with_shell_styles(mut self, shell_styles: TableViewShellStyleConfig) -> Self {
        self.shell_styles = shell_styles;
        self
    }

    pub fn with_numbering(mut self, numbering: TableViewNumberingConfig) -> Self {
        self.include_row_numbers = numbering.is_enabled();
        self.numbering = numbering;
        self
    }

    pub fn width_for_cell(&self, cell_index: usize) -> Option<usize> {
        if self.dontwrap || self.mode.force_zero_width() {
            return None;
        }
        self.per_column_widths
            .get(cell_index)
            .copied()
            .filter(|value| *value > 0)
            .or(self.wrap_cell_width.filter(|value| *value > 0))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub output_modes: Vec<String>,
    pub default_virtual_policy: String,
    pub stage28_cli_options: Vec<String>,
    pub stage29_numbering_modes: Vec<String>,
    pub stage32_row_style_policies: Vec<String>,
    pub stage33_cell_style_policies: Vec<String>,
    pub stage34_style_composition_morphisms: Vec<String>,
    pub stage36_shell_style_policies: Vec<String>,
    pub stage37_virtual_column_policies: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputReport {
    pub class: String,
    pub mode: String,
    pub row_count: usize,
    pub rendered_row_count: usize,
    pub cell_count: usize,
    pub virtual_cell_count: usize,
    pub rendered_line_count: usize,
    pub rendered_lines: Vec<String>,
    pub rendered_text: String,
    pub table_view_policy: String,
    pub virtual_column_policy: String,
    pub suppress_question_mark_virtuals: bool,
    pub virtual_column_option_count: usize,
    pub suppress_headers: bool,
    pub include_empty_rows: bool,
    pub include_row_numbers: bool,
    pub numbering_mode: String,
    pub numbering_column_count: usize,
    pub layout_enabled: bool,
    pub layout_page_count: usize,
    pub layout_column_count: usize,
    pub layout_column_widths: Vec<usize>,
    pub html_attribute_enabled: bool,
    pub html_attribute_policy: String,
    pub html_attribute_cell_count: usize,
    pub html_attribute_class_cell_count: usize,
    pub html_attribute_report: Option<TableViewHtmlAttributeReport>,
    pub row_style_enabled: bool,
    pub row_style_policy: String,
    pub row_style_row_count: usize,
    pub row_style_colored_row_count: usize,
    pub row_style_report: Option<TableViewRowStyleReport>,
    pub cell_style_enabled: bool,
    pub cell_style_policy: String,
    pub cell_style_cell_count: usize,
    pub cell_style_styled_cell_count: usize,
    pub cell_style_virtual_cell_count: usize,
    pub cell_style_report: Option<TableViewCellStyleReport>,
    pub shell_style_enabled: bool,
    pub shell_style_policy: String,
    pub shell_style_cell_count: usize,
    pub shell_style_ansi_cell_count: usize,
    pub shell_style_report: Option<TableViewShellStyleReport>,
    pub html_cell_style_composition_enabled: bool,
    pub html_cell_style_composition_count: usize,
    pub html_attribute_only_cell_count: usize,
    pub wrap_cell_width: Option<usize>,
    pub per_column_width_count: usize,
    pub dontwrap: bool,
    pub nocolor: bool,
    pub justtext: bool,
    pub onetable: bool,
    pub endlessscreen: bool,
    pub endless: bool,
    pub cli_options: TableViewOutputCliOptions,
    pub continuum_m_direct_header_present: bool,
    pub continuum_m_virtual_744_kept_as_witness: bool,
    pub visible_output_is_empty: bool,
    pub universal_property: String,
}

impl TableViewOutputReport {
    pub fn contains_text(&self, needle: &str) -> bool {
        self.rendered_lines.iter().any(|line| line.contains(needle))
            || self.rendered_text.contains(needle)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputBundle;

impl TableViewOutputBundle {
    pub fn snapshot(&self) -> TableViewOutputSnapshot {
        TableViewOutputSnapshot {
            class: "TableViewOutputBundle".to_string(),
            morphisms: vec![
                "parse_table_view_output_cli_options".to_string(),
                "render_materialized_table_view".to_string(),
                "render_table_view_rows_as_mode".to_string(),
                "render_table_view_for_cli_args".to_string(),
                "rendered_row_value_lines".to_string(),
                "wrap_output_cell".to_string(),
                "numbering_values_for_source_row".to_string(),
                "layout_value_rows".to_string(),
                "column_pages_for_widths".to_string(),
                "html_attribute_report_for_rows".to_string(),
                "render_html_table_with_attributes".to_string(),
                "row_style_report_for_rows".to_string(),
                "styled_begin_row_for_row".to_string(),
                "cell_style_report_for_rows".to_string(),
                "styled_begin_cell_for_output_value".to_string(),
                "styled_end_cell_for_mode".to_string(),
                "shell_style_report_for_rows".to_string(),
                "colorize_shell_output_value".to_string(),
                "parse_table_view_virtual_column_cli_options".to_string(),
                "csv_escape_cell".to_string(),
                "html_escape_cell".to_string(),
                "markdown_escape_cell".to_string(),
            ],
            output_modes: vec![
                OutputMode::Shell.canonical_name().to_string(),
                OutputMode::Csv.canonical_name().to_string(),
                OutputMode::Html.canonical_name().to_string(),
                OutputMode::Bbcode.canonical_name().to_string(),
                OutputMode::Emacs.canonical_name().to_string(),
                OutputMode::Markdown.canonical_name().to_string(),
                OutputMode::Nichts.canonical_name().to_string(),
            ],
            default_virtual_policy: VirtualColumnDisplayPolicy::Suppress.canonical().to_string(),
            stage28_cli_options: vec![
                "nocolor".to_string(),
                "justtext".to_string(),
                "onetable".to_string(),
                "endlessscreen".to_string(),
                "endless".to_string(),
                "dontwrap".to_string(),
                "breite".to_string(),
                "breiten".to_string(),
                "keineleereninhalte".to_string(),
                "keinenummerierung".to_string(),
                "keineueberschriften".to_string(),
                "htmlclasses".to_string(),
                "htmlrawclasses".to_string(),
                "htmlclasswitness".to_string(),
                "rowcolors".to_string(),
                "zeilenfarben".to_string(),
                "rowcolorwitness".to_string(),
                "cellstyles".to_string(),
                "zellstyles".to_string(),
                "cellstylewitness".to_string(),
                "shellcolors".to_string(),
                "ansicolors".to_string(),
                "shellcolorwitness".to_string(),
                "virtualcolumns".to_string(),
                "virtualplaceholder".to_string(),
                "virtualquestionmarks".to_string(),
                "virtualwitness".to_string(),
                "suppressvirtualcolumns".to_string(),
            ],
            stage29_numbering_modes: vec![
                TableViewNumberingMode::Disabled.canonical().to_string(),
                TableViewNumberingMode::LegacyPair.canonical().to_string(),
                TableViewNumberingMode::NumberOnly.canonical().to_string(),
                TableViewNumberingMode::CountingOnly.canonical().to_string(),
            ],
            stage32_row_style_policies: vec![
                "plain".to_string(),
                "legacy-colored-begin-col".to_string(),
            ],
            stage33_cell_style_policies: vec![
                TableViewCellStylePolicy::Plain.canonical().to_string(),
                TableViewCellStylePolicy::LegacyGenerateCell.canonical().to_string(),
                TableViewCellStylePolicy::LegacyGenerateCellWitness.canonical().to_string(),
            ],
            stage34_style_composition_morphisms: vec![
                "html_begin_cell_for_output_value".to_string(),
                "compose_html_td_open_tags".to_string(),
                "html_cell_style_composition_counts".to_string(),
            ],
            stage36_shell_style_policies: vec![
                TableViewShellStylePolicy::Plain.canonical().to_string(),
                TableViewShellStylePolicy::LegacyColorize.canonical().to_string(),
                TableViewShellStylePolicy::LegacyColorizeWitness.canonical().to_string(),
            ],
            stage37_virtual_column_policies: vec![
                VirtualColumnDisplayPolicy::Suppress.canonical().to_string(),
                VirtualColumnDisplayPolicy::Placeholder.canonical().to_string(),
                VirtualColumnDisplayPolicy::TagSummary.canonical().to_string(),
                VirtualColumnDisplayPolicy::Witness.canonical().to_string(),
            ],
            universal_property:
                "one materialized table view has deterministic images in every output syntax, output-option and numbering context"
                    .to_string(),
        }
    }

    pub fn render_view(
        &self,
        view: &MaterializedTableView,
        config: &TableViewOutputConfig,
    ) -> TableViewOutputReport {
        render_materialized_table_view(view, config)
    }

    pub fn render_cli_args<S: AsRef<str>>(
        &self,
        args: &[S],
        materialization_config: &TableMaterializationConfig,
        config: &TableViewOutputConfig,
    ) -> TableViewOutputReport {
        render_table_view_for_cli_args(args, materialization_config, config)
    }
}

pub fn bootstrap_table_view_output() -> TableViewOutputBundle {
    TableViewOutputBundle
}

pub fn parse_table_view_output_cli_options<S: AsRef<str>>(args: &[S]) -> TableViewOutputCliOptions {
    let mut options = TableViewOutputCliOptions::default();
    let mut active_output_context = false;
    for arg in args {
        let raw = arg.as_ref();
        match raw {
            "-ausgabe" | "-a" => {
                active_output_context = true;
                continue;
            }
            "-zeilen" | "-z" | "-spalten" | "-s" | "-kombination" | "-k" | "-debug" => {
                active_output_context = false;
            }
            _ => {}
        }
        let Some(body) = raw.strip_prefix("--") else {
            continue;
        };
        let (key, value) = split_output_option(body);
        let recognized = match key.as_str() {
            "nocolor" => {
                options.nocolor = true;
                true
            }
            "justtext" => {
                options.justtext = true;
                true
            }
            "onetable" => {
                options.onetable = true;
                true
            }
            "endlessscreen" => {
                options.endlessscreen = true;
                true
            }
            "endless" => {
                options.endless = true;
                true
            }
            "dontwrap" => {
                options.dontwrap = true;
                true
            }
            "keineleereninhalte" => {
                options.keineleereninhalte = true;
                true
            }
            "keinenummerierung" => {
                options.keinenummerierung = true;
                true
            }
            "keineueberschriften" => {
                options.keineueberschriften = true;
                true
            }
            "htmlclasses" => {
                options.htmlclasses = true;
                true
            }
            "htmlrawclasses" => {
                options.htmlrawclasses = true;
                true
            }
            "htmlclasswitness" => {
                options.htmlclasswitness = true;
                true
            }
            "rowcolors" | "zeilenfarben" => {
                options.rowcolors = true;
                true
            }
            "rowcolorwitness" | "zeilenfarbenwitness" => {
                options.rowcolors = true;
                options.rowcolorwitness = true;
                true
            }
            "cellstyles" | "cellstyle" | "zellstyles" | "zellstile" | "zellfarben" => {
                options.cellstyles = true;
                true
            }
            "cellstylewitness" | "zellstylewitness" | "zellfarbenwitness" => {
                options.cellstyles = true;
                options.cellstylewitness = true;
                true
            }
            "shellcolors" | "shellcolor" | "ansicolors" | "ansicolor" => {
                options.shellcolors = true;
                true
            }
            "shellcolorwitness" | "ansicolorwitness" => {
                options.shellcolors = true;
                options.shellcolorwitness = true;
                true
            }
            "virtualcolumns" | "virtualcolumnsummary" | "virtualsummary" | "virtualtags"
            | "showvirtualcolumns" | "virtuellespalten" | "virtuellespaltenzusammenfassung"
            | "virtuellenspalten" => {
                options.virtualcolumns = true;
                true
            }
            "virtualplaceholder" | "virtualplaceholders" | "virtualquestionmarks"
            | "virtualquestionmark" | "virtuelleplatzhalter" | "virtuellefragezeichen" => {
                options.virtualplaceholder = true;
                options.virtualquestionmarks = true;
                true
            }
            "novirtualquestionmarks" | "suppressvirtualquestionmarks" => true,
            "virtualwitness" | "virtualcolumnwitness" | "virtualwitnesses"
            | "virtuellenspaltenwitness" | "virtuellenspaltenzeugen" => {
                options.virtualwitness = true;
                true
            }
            "suppressvirtualcolumns" | "hidevirtualcolumns" | "novirtualcolumns"
            | "keinevirtuellenspalten" => {
                options.suppressvirtualcolumns = true;
                true
            }
            "breite" => {
                if let Some(value) = value.as_deref() {
                    options.width = parse_positive_width(value);
                }
                true
            }
            "breiten" => {
                if let Some(value) = value.as_deref() {
                    options.widths = parse_width_list(value);
                }
                true
            }
            // `art` and `spaltenreihenfolgeundnurdiese` are parsed by the
            // parameter runtime.  They are output-context options, but they do
            // not change this renderer option bundle directly.
            "art" | "spaltenreihenfolgeundnurdiese" => true,
            _ => false,
        };
        if recognized {
            options.recognized_option_count += 1;
        } else if active_output_context {
            options.unknown_output_options.push(key);
        }
    }
    options.virtual_column_options = parse_table_view_virtual_column_cli_options(args);
    if options.virtual_column_options.suppress_question_mark_virtuals == Some(true) {
        options.virtualquestionmarks = false;
    }
    if options.dontwrap {
        options.width = None;
        options.widths.clear();
    }
    options
}

fn split_output_option(body: &str) -> (String, Option<String>) {
    match body.split_once('=') {
        Some((key, value)) => (key.to_string(), Some(value.to_string())),
        None => (body.to_string(), None),
    }
}

fn parse_positive_width(value: &str) -> Option<usize> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    let parsed = trimmed.parse::<i64>().ok()?.unsigned_abs() as usize;
    (parsed > 0).then_some(parsed)
}

fn parse_width_list(value: &str) -> Vec<usize> {
    value
        .split(',')
        .filter_map(parse_positive_width)
        .collect::<Vec<_>>()
}

pub fn render_table_view_for_cli_args<S: AsRef<str>>(
    args: &[S],
    materialization_config: &TableMaterializationConfig,
    config: &TableViewOutputConfig,
) -> TableViewOutputReport {
    let parsed = bootstrap_parameter_runtime().parse_cli_args(args);
    let cli_options = parse_table_view_output_cli_options(args);
    let mut mode_config = config.clone().with_cli_options(cli_options);
    let mode = parsed.selected_output_mode.unwrap_or(mode_config.mode);
    mode_config.mode = mode;
    if mode.force_zero_width() {
        mode_config.wrap_cell_width = None;
        mode_config.per_column_widths.clear();
    }
    let view_config = MaterializedTableViewConfig {
        virtual_column_policy: mode_config.virtual_column_policy,
        suppress_question_mark_virtuals: mode_config.suppress_question_mark_virtuals,
        ..MaterializedTableViewConfig::default()
    };
    let report = bootstrap_table_materialization()
        .materialize_command_sets(&parsed.command_sets, materialization_config);
    let view = bootstrap_table_view().view_from_report(&report, &view_config);
    render_materialized_table_view(&view, &mode_config)
}

pub fn render_materialized_table_view(
    view: &MaterializedTableView,
    config: &TableViewOutputConfig,
) -> TableViewOutputReport {
    let layout_report = shell_layout_report_for_rows(&view.rows, config);
    let html_attribute_report = if config.mode == OutputMode::Html && config.html_attributes.enabled
    {
        Some(html_attribute_report_for_rows(
            &view.rows,
            &config.html_attributes,
            config.suppress_headers,
            config.include_empty_rows,
        ))
    } else {
        None
    };
    let row_style_report = if config.row_styles.activates_mode(config.mode) {
        Some(row_style_report_for_rows(
            &view.rows,
            config.mode,
            &config.row_styles,
            config.suppress_headers,
            config.include_empty_rows,
        ))
    } else {
        None
    };
    let prefix_column_count = output_prefix_column_count(config);
    let cell_style_report = if config.cell_styles.activates_mode(config.mode) {
        Some(cell_style_report_for_rows(
            &view.rows,
            config.mode,
            &config.cell_styles,
            config.suppress_headers,
            config.include_empty_rows,
            prefix_column_count,
        ))
    } else {
        None
    };
    let shell_style_report =
        if config.shell_styles.activates_mode(config.mode) && !config.layout.activates_layout() {
            Some(shell_style_report_for_rows(
                &view.rows,
                &config.shell_styles,
                config.suppress_headers,
                config.include_empty_rows,
                prefix_column_count,
            ))
        } else {
            None
        };
    let (html_cell_style_composition_count, html_attribute_only_cell_count) =
        html_cell_style_composition_counts(&view.rows, config);
    let rendered_lines = render_table_view_rows_as_mode(&view.rows, config);
    let rendered_text = rendered_lines.join("\n");
    let visible_output_is_empty = rendered_text.is_empty();
    let cell_count = view.rows.iter().map(|row| row.cells.len()).sum::<usize>();
    let rendered_row_count = filtered_output_rows(&view.rows, config).len();
    TableViewOutputReport {
        class: "TableViewOutputReport".to_string(),
        mode: config.mode.canonical_name().to_string(),
        row_count: view.row_count,
        rendered_row_count,
        cell_count,
        virtual_cell_count: view.rendered_virtual_cell_count,
        rendered_line_count: rendered_lines.len(),
        rendered_lines,
        rendered_text,
        table_view_policy: view.policy.clone(),
        virtual_column_policy: config.virtual_column_policy.canonical().to_string(),
        suppress_question_mark_virtuals: config.suppress_question_mark_virtuals,
        virtual_column_option_count: config.virtual_column_options.recognized_option_count,
        suppress_headers: config.suppress_headers,
        include_empty_rows: config.include_empty_rows,
        include_row_numbers: config.include_row_numbers,
        numbering_mode: config.numbering.mode.canonical().to_string(),
        numbering_column_count: config.numbering.column_count(),
        layout_enabled: layout_report.enabled,
        layout_page_count: layout_report.page_count,
        layout_column_count: layout_report.column_count,
        layout_column_widths: layout_report.column_widths,
        html_attribute_enabled: config.html_attributes.enabled,
        html_attribute_policy: config.html_attributes.policy.canonical().to_string(),
        html_attribute_cell_count: html_attribute_report
            .as_ref()
            .map(|report| report.attributed_cell_count)
            .unwrap_or_default(),
        html_attribute_class_cell_count: html_attribute_report
            .as_ref()
            .map(|report| report.class_string_cell_count)
            .unwrap_or_default(),
        html_attribute_report,
        row_style_enabled: config.row_styles.enabled,
        row_style_policy: config.row_styles.policy.canonical().to_string(),
        row_style_row_count: row_style_report.as_ref().map(|report| report.row_count).unwrap_or_default(),
        row_style_colored_row_count: row_style_report.as_ref().map(|report| report.colored_row_count).unwrap_or_default(),
        row_style_report,
        cell_style_enabled: config.cell_styles.enabled,
        cell_style_policy: config.cell_styles.policy.canonical().to_string(),
        cell_style_cell_count: cell_style_report.as_ref().map(|report| report.cell_count).unwrap_or_default(),
        cell_style_styled_cell_count: cell_style_report.as_ref().map(|report| report.styled_cell_count).unwrap_or_default(),
        cell_style_virtual_cell_count: cell_style_report.as_ref().map(|report| report.virtual_cell_style_count).unwrap_or_default(),
        cell_style_report,
        shell_style_enabled: config.shell_styles.enabled,
        shell_style_policy: config.shell_styles.policy.canonical().to_string(),
        shell_style_cell_count: shell_style_report.as_ref().map(|report| report.cell_count).unwrap_or_default(),
        shell_style_ansi_cell_count: shell_style_report.as_ref().map(|report| report.ansi_cell_count).unwrap_or_default(),
        shell_style_report,
        html_cell_style_composition_enabled: config.mode == OutputMode::Html
            && config.html_attributes.activates_catalog()
            && config.cell_styles.activates_mode(OutputMode::Html),
        html_cell_style_composition_count,
        html_attribute_only_cell_count,
        wrap_cell_width: config.wrap_cell_width,
        per_column_width_count: config.per_column_widths.len(),
        dontwrap: config.dontwrap,
        nocolor: config.nocolor,
        justtext: config.justtext,
        onetable: config.onetable,
        endlessscreen: config.endlessscreen,
        endless: config.endless,
        cli_options: config.cli_options.clone(),
        continuum_m_direct_header_present: view.continuum_m_direct_header_present,
        continuum_m_virtual_744_kept_as_witness: view.continuum_m_virtual_744_kept_as_witness,
        visible_output_is_empty,
        universal_property:
            "output flags and numbering change only the selected output projection; materialized local sections stay unchanged"
                .to_string(),
    }
}

pub fn render_table_view_rows_as_mode(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    match config.mode {
        OutputMode::Nichts => Vec::new(),
        OutputMode::Shell => render_shell_rows(rows, config),
        OutputMode::Csv => render_csv_rows_with_config(rows, config),
        OutputMode::Markdown => render_markdown_rows(rows, config),
        OutputMode::Emacs => render_pipe_rows_with_config(rows, config),
        OutputMode::Html => render_html_rows_with_config(rows, config),
        OutputMode::Bbcode => render_bbcode_rows_with_config(rows, config),
    }
}

pub fn filtered_output_rows<'a>(
    rows: &'a [MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<&'a MaterializedTableViewRow> {
    rows.iter()
        .filter(|row| !(config.suppress_headers && row.source_row_zero_based == 0))
        .filter(|row| config.include_empty_rows || !row_values(row).is_empty())
        .collect()
}

pub fn rendered_row_value_lines(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    for (display_index, row) in filtered_output_rows(rows, config).into_iter().enumerate() {
        out.extend(expand_row_to_value_lines(row, config, display_index));
    }
    out
}

pub fn expand_row_to_value_lines(
    row: &MaterializedTableViewRow,
    config: &TableViewOutputConfig,
    display_index: usize,
) -> Vec<Vec<String>> {
    let values = row_values_with_options(row, config, display_index);
    if values.is_empty() {
        return vec![Vec::new()];
    }
    let wrapped = values
        .iter()
        .enumerate()
        .map(|(index, value)| wrap_output_cell(value, config.width_for_cell(index)))
        .collect::<Vec<_>>();
    let height = wrapped.iter().map(Vec::len).max().unwrap_or(1).max(1);
    (0..height)
        .map(|line_index| {
            wrapped
                .iter()
                .map(|cell_lines| cell_lines.get(line_index).cloned().unwrap_or_default())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn row_values_with_options(
    row: &MaterializedTableViewRow,
    config: &TableViewOutputConfig,
    display_index: usize,
) -> Vec<String> {
    let mut values = row_values(row);
    if config.numbering.is_enabled() {
        let mut prefix = numbering_values_for_source_row(
            row.source_row_zero_based,
            display_index,
            &config.numbering,
        );
        prefix.extend(values);
        return prefix;
    }
    if config.include_row_numbers {
        let value = if row.source_row_zero_based == 0 {
            config.row_number_header.clone()
        } else {
            row.source_row_zero_based.to_string()
        };
        let fallback = (display_index + 1).to_string();
        values.insert(0, if value.is_empty() { fallback } else { value });
    }
    values
}

pub fn wrap_output_cell(value: &str, width: Option<usize>) -> Vec<String> {
    match width.filter(|width| *width > 0) {
        Some(width) if value.chars().count() > width => alxwrap(value, width, None),
        _ => vec![value.to_string()],
    }
}

pub fn shell_layout_report_for_rows(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> TableViewLayoutReport {
    let value_lines = rendered_row_value_lines(rows, config);
    let mut layout_config = config.layout.clone();
    layout_config.separator = config.shell_separator.clone();
    if layout_config.width_overrides.is_empty() {
        layout_config.width_overrides = config.per_column_widths.clone();
    }
    layout_value_rows(&value_lines, &layout_config)
}

pub fn render_shell_rows(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    if config.shell_styles.activates_mode(OutputMode::Shell) && !config.layout.activates_layout() {
        let prefix_column_count = output_prefix_column_count(config);
        let mut out = Vec::new();
        for (display_index, row) in filtered_output_rows(rows, config).into_iter().enumerate() {
            let expanded = expand_row_to_value_lines(row, config, display_index);
            for (line_index, values) in expanded.into_iter().enumerate() {
                if values.is_empty() && !config.include_empty_rows {
                    continue;
                }
                let styled_values = values
                    .iter()
                    .enumerate()
                    .map(|(value_index, value)| {
                        colorize_shell_output_value(
                            row,
                            value,
                            value_index,
                            line_index > 0,
                            prefix_column_count,
                            &config.shell_styles,
                        )
                    })
                    .collect::<Vec<_>>();
                out.push(styled_values.join(&config.shell_separator));
            }
        }
        return out;
    }
    let value_lines = rendered_row_value_lines(rows, config);
    if config.layout.activates_layout() {
        let mut layout_config = config.layout.clone();
        layout_config.separator = config.shell_separator.clone();
        if layout_config.width_overrides.is_empty() {
            layout_config.width_overrides = config.per_column_widths.clone();
        }
        return layout_value_rows(&value_lines, &layout_config).rendered_lines;
    }
    value_lines
        .into_iter()
        .filter_map(|values| {
            if values.is_empty() && !config.include_empty_rows {
                None
            } else {
                Some(values.join(&config.shell_separator))
            }
        })
        .collect()
}

pub fn render_csv_rows(
    rows: &[MaterializedTableViewRow],
    separator: char,
    include_empty_rows: bool,
) -> Vec<String> {
    let config = TableViewOutputConfig {
        csv_separator: separator,
        include_empty_rows,
        mode: OutputMode::Csv,
        ..TableViewOutputConfig::default()
    };
    render_csv_rows_with_config(rows, &config)
}

pub fn render_csv_rows_with_config(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    rendered_row_value_lines(rows, config)
        .into_iter()
        .filter_map(|values| {
            if values.is_empty() && !config.include_empty_rows {
                None
            } else {
                Some(
                    values
                        .iter()
                        .map(|cell| csv_escape_cell(cell, config.csv_separator))
                        .collect::<Vec<_>>()
                        .join(&config.csv_separator.to_string()),
                )
            }
        })
        .collect()
}

pub fn render_pipe_rows(
    rows: &[MaterializedTableViewRow],
    include_empty_rows: bool,
) -> Vec<String> {
    let config = TableViewOutputConfig {
        include_empty_rows,
        mode: OutputMode::Emacs,
        ..TableViewOutputConfig::default()
    };
    render_pipe_rows_with_config(rows, &config)
}

pub fn render_pipe_rows_with_config(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    rendered_row_value_lines(rows, config)
        .into_iter()
        .filter_map(|values| {
            if values.is_empty() && !config.include_empty_rows {
                None
            } else {
                Some(format!("|{}|", values.join("|")))
            }
        })
        .collect()
}

pub fn render_markdown_rows(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    let mut out = Vec::new();
    let mut emitted_data_row_count = 0usize;
    for values in rendered_row_value_lines(rows, config) {
        if values.is_empty() && !config.include_empty_rows {
            continue;
        }
        let escaped = values
            .iter()
            .map(|cell| markdown_escape_cell(cell))
            .collect::<Vec<_>>();
        out.push(format!("| {} |", escaped.join(" | ")));
        if emitted_data_row_count == 0
            && config.include_markdown_header_separator
            && !config.suppress_headers
            && !escaped.is_empty()
        {
            out.push(format!(
                "| {} |",
                escaped
                    .iter()
                    .map(|_| "---".to_string())
                    .collect::<Vec<_>>()
                    .join(" | ")
            ));
        }
        emitted_data_row_count += 1;
    }
    out
}

pub fn html_begin_cell_for_output_value(
    row: &MaterializedTableViewRow,
    display_cell_index: usize,
    continuation_line: bool,
    prefix_column_count: usize,
    config: &TableViewOutputConfig,
) -> String {
    let cell_style_begin = styled_begin_cell_for_output_value(
        row,
        OutputMode::Html,
        display_cell_index,
        continuation_line,
        prefix_column_count,
        &config.cell_styles,
    );
    if !config.html_attributes.activates_catalog() {
        return cell_style_begin;
    }
    let Some(cell) = data_cell_for_output_value(row, display_cell_index, prefix_column_count)
    else {
        return cell_style_begin;
    };
    let attribute = html_attribute_for_cell(cell, &config.html_attributes);
    if !attribute.record_found && attribute.rendered_open_tag.trim() == "<td>" {
        return cell_style_begin;
    }
    if config.cell_styles.activates_mode(OutputMode::Html) {
        compose_html_td_open_tags(&cell_style_begin, &attribute.rendered_open_tag)
    } else {
        attribute.rendered_open_tag
    }
}

pub fn data_cell_for_output_value<'a>(
    row: &'a MaterializedTableViewRow,
    display_cell_index: usize,
    prefix_column_count: usize,
) -> Option<&'a MaterializedTableViewCell> {
    display_cell_index
        .checked_sub(prefix_column_count)
        .and_then(|data_index| row.cells.get(data_index))
}

pub fn html_cell_style_composition_counts(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> (usize, usize) {
    if config.mode != OutputMode::Html || !config.html_attributes.activates_catalog() {
        return (0, 0);
    }
    let prefix_column_count = output_prefix_column_count(config);
    let mut composed = 0usize;
    let mut attribute_only = 0usize;
    for row in filtered_output_rows(rows, config) {
        for display_cell_index in prefix_column_count..prefix_column_count + row.cells.len() {
            let Some(cell) =
                data_cell_for_output_value(row, display_cell_index, prefix_column_count)
            else {
                continue;
            };
            let attribute = html_attribute_for_cell(cell, &config.html_attributes);
            if !attribute.record_found && attribute.rendered_open_tag.trim() == "<td>" {
                continue;
            }
            let style = crate::table_view_cell_styles::cell_style_for_output_value(
                row,
                OutputMode::Html,
                display_cell_index,
                false,
                prefix_column_count,
                &config.cell_styles,
            );
            if config.cell_styles.activates_mode(OutputMode::Html) && style.styled {
                composed += 1;
            } else {
                attribute_only += 1;
            }
        }
    }
    (composed, attribute_only)
}

pub fn compose_html_td_open_tags(style_begin: &str, attribute_begin: &str) -> String {
    let style = normalize_td_open_tag(style_begin).unwrap_or_else(|| "<td>".to_string());
    let attribute = normalize_td_open_tag(attribute_begin).unwrap_or_else(|| "<td>".to_string());
    if is_plain_td(&attribute) {
        return style;
    }
    if is_plain_td(&style) {
        return attribute;
    }

    let class_values = dedup_words(
        quoted_attr_values(&style, "class")
            .into_iter()
            .chain(quoted_attr_values(&attribute, "class"))
            .flat_map(|value| {
                value
                    .split_whitespace()
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .collect(),
    );
    let style_values = quoted_attr_values(&style, "style")
        .into_iter()
        .chain(quoted_attr_values(&attribute, "style"))
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>();
    let mut other_attrs = Vec::new();
    for tag in [&style, &attribute] {
        let stripped = remove_quoted_attr(&remove_quoted_attr(tag, "class"), "style");
        if let Some(inner) = td_inner_attrs(&stripped) {
            let inner = inner.trim();
            if !inner.is_empty() {
                other_attrs.push(inner.to_string());
            }
        }
    }
    let mut attrs = Vec::new();
    if !class_values.is_empty() {
        attrs.push(format!(
            "class=\"{}\"",
            html_escape_attr_value(&class_values.join(" "))
        ));
    }
    if !style_values.is_empty() {
        let merged_style = style_values
            .iter()
            .map(|value| value.trim().trim_end_matches(';'))
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>()
            .join(";");
        if !merged_style.is_empty() {
            attrs.push(format!(
                "style=\"{};\"",
                html_escape_attr_value(&merged_style)
            ));
        }
    }
    attrs.extend(other_attrs);
    if attrs.is_empty() {
        "<td>".to_string()
    } else {
        format!("<td {}>", attrs.join(" "))
    }
}

fn normalize_td_open_tag(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.starts_with("<td") && trimmed.ends_with('>') {
        Some(trimmed.replace('\n', ""))
    } else {
        None
    }
}

fn is_plain_td(value: &str) -> bool {
    value.trim() == "<td>" || value.trim() == "<td>\n"
}

fn td_inner_attrs(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if !trimmed.starts_with("<td") || !trimmed.ends_with('>') {
        return None;
    }
    Some(trimmed[3..trimmed.len() - 1].trim().to_string())
}

fn quoted_attr_values(tag: &str, name: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut cursor = 0usize;
    let needle = format!("{name}=\"");
    while let Some(rel) = tag[cursor..].find(&needle) {
        let start = cursor + rel;
        let before_is_boundary = start == 0
            || tag[..start]
                .chars()
                .last()
                .map(|ch| ch.is_whitespace() || ch == '<')
                .unwrap_or(true);
        let value_start = start + needle.len();
        if before_is_boundary {
            if let Some(end_rel) = tag[value_start..].find('"') {
                values.push(tag[value_start..value_start + end_rel].to_string());
                cursor = value_start + end_rel + 1;
                continue;
            }
        }
        cursor = value_start;
    }
    values
}

fn remove_quoted_attr(tag: &str, name: &str) -> String {
    let mut out = tag.to_string();
    let needle = format!("{name}=\"");
    loop {
        let Some(pos) = out.find(&needle) else { break };
        let start = out[..pos]
            .char_indices()
            .rev()
            .find(|(_, ch)| !ch.is_whitespace())
            .map(|(idx, ch)| idx + ch.len_utf8())
            .unwrap_or(0);
        let value_start = pos + needle.len();
        let Some(end_rel) = out[value_start..].find('"') else {
            break;
        };
        out.replace_range(start..value_start + end_rel + 1, "");
    }
    out
}

fn dedup_words(values: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    for value in values {
        if value.trim().is_empty() {
            continue;
        }
        if seen.insert(value.clone()) {
            out.push(value);
        }
    }
    out
}

fn html_escape_attr_value(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn render_html_rows(
    rows: &[MaterializedTableViewRow],
    include_empty_rows: bool,
) -> Vec<String> {
    let config = TableViewOutputConfig {
        include_empty_rows,
        mode: OutputMode::Html,
        ..TableViewOutputConfig::default()
    };
    render_html_rows_with_config(rows, &config)
}

pub fn render_html_rows_with_config(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    if config.html_attributes.enabled
        && !config.row_styles.activates_mode(OutputMode::Html)
        && !config.cell_styles.activates_mode(OutputMode::Html)
        && config.wrap_cell_width.is_none()
        && !config.include_row_numbers
        && !config.numbering.is_enabled()
    {
        return render_html_table_with_attributes(
            rows,
            &config.html_attributes,
            config.suppress_headers,
            config.include_empty_rows,
        );
    }
    let filtered_rows = filtered_output_rows(rows, config);
    if filtered_rows.is_empty() {
        return Vec::new();
    }
    let mut out = vec![r#"<table border=0 id="bigtable">"#.to_string()];
    for (display_index, row) in filtered_rows.into_iter().enumerate() {
        let expanded = expand_row_to_value_lines(row, config, display_index);
        for (line_index, values) in expanded.into_iter().enumerate() {
            if values.is_empty() && !config.include_empty_rows {
                continue;
            }
            let begin =
                styled_begin_row_for_row(row, OutputMode::Html, line_index > 0, &config.row_styles);
            out.push(clean_row_begin(&begin, "<tr>"));
            let prefix_column_count = output_prefix_column_count(config);
            for (value_index, value) in values.into_iter().enumerate() {
                let begin = html_begin_cell_for_output_value(
                    row,
                    value_index,
                    line_index > 0,
                    prefix_column_count,
                    config,
                );
                let end = styled_end_cell_for_mode(OutputMode::Html);
                out.push(format!("{}{}{}", begin, html_escape_cell(&value), end));
            }
            out.push("</tr>".to_string());
        }
    }
    out.push("</table>".to_string());
    out
}

pub fn render_bbcode_rows(
    rows: &[MaterializedTableViewRow],
    include_empty_rows: bool,
) -> Vec<String> {
    let config = TableViewOutputConfig {
        include_empty_rows,
        mode: OutputMode::Bbcode,
        ..TableViewOutputConfig::default()
    };
    render_bbcode_rows_with_config(rows, &config)
}

pub fn render_bbcode_rows_with_config(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    let filtered_rows = filtered_output_rows(rows, config);
    if filtered_rows.is_empty() {
        return Vec::new();
    }
    let mut out = vec!["[table]".to_string()];
    for (display_index, row) in filtered_rows.into_iter().enumerate() {
        let expanded = expand_row_to_value_lines(row, config, display_index);
        for (line_index, values) in expanded.into_iter().enumerate() {
            if values.is_empty() && !config.include_empty_rows {
                continue;
            }
            let begin = styled_begin_row_for_row(
                row,
                OutputMode::Bbcode,
                line_index > 0,
                &config.row_styles,
            );
            let prefix_column_count = output_prefix_column_count(config);
            let cells = values
                .iter()
                .enumerate()
                .map(|(value_index, value)| {
                    let begin = styled_begin_cell_for_output_value(
                        row,
                        OutputMode::Bbcode,
                        value_index,
                        line_index > 0,
                        prefix_column_count,
                        &config.cell_styles,
                    );
                    let end = styled_end_cell_for_mode(OutputMode::Bbcode);
                    format!("{}{}{}", begin, bbcode_escape_cell(value), end)
                })
                .collect::<Vec<_>>()
                .join("");
            out.push(format!("{}{cells}[/tr]", clean_row_begin(&begin, "[tr]")));
        }
    }
    out.push("[/table]".to_string());
    out
}

fn clean_row_begin(begin: &str, fallback: &str) -> String {
    let trimmed = begin.trim();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn output_prefix_column_count(config: &TableViewOutputConfig) -> usize {
    if config.numbering.is_enabled() {
        config.numbering.column_count()
    } else if config.include_row_numbers {
        1
    } else {
        0
    }
}

pub fn row_values(row: &MaterializedTableViewRow) -> Vec<String> {
    row.cells.iter().map(|cell| cell.value.clone()).collect()
}

pub fn csv_escape_cell(value: &str, separator: char) -> String {
    let must_quote = value.contains(separator)
        || value.contains('\n')
        || value.contains('\r')
        || value.contains('"');
    if !must_quote {
        return value.to_string();
    }
    format!("\"{}\"", value.replace('"', "\"\""))
}

pub fn html_escape_cell(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn markdown_escape_cell(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace('\n', " ")
}

pub fn bbcode_escape_cell(value: &str) -> String {
    value.replace('[', "&#91;").replace(']', "&#93;")
}

pub fn continuum_m_table_view_output_smoke(mode: OutputMode) -> TableViewOutputReport {
    let args = vec![
        "reta".to_string(),
        "-zeilen".to_string(),
        "--vorhervonausschnitt=1-1".to_string(),
        "-spalten".to_string(),
        "--kontinuum=m".to_string(),
        "-ausgabe".to_string(),
        format!("--art={}", mode.canonical_name()),
        "--breite=0".to_string(),
    ];
    render_table_view_for_cli_args(
        &args,
        &TableMaterializationConfig::default(),
        &TableViewOutputConfig::default().with_mode(mode),
    )
}

pub fn output_layout_smoke() -> TableViewOutputReport {
    let args = vec![
        "reta".to_string(),
        "-zeilen".to_string(),
        "--vorhervonausschnitt=1-2".to_string(),
        "-spalten".to_string(),
        "--kontinuum=m".to_string(),
        "-ausgabe".to_string(),
        "--breiten=4,12".to_string(),
        "--breite=0".to_string(),
    ];
    let mut config = TableViewOutputConfig::default();
    config.layout = TableViewLayoutConfig::enabled_shell();
    render_table_view_for_cli_args(&args, &TableMaterializationConfig::default(), &config)
}

pub fn output_flags_smoke() -> TableViewOutputReport {
    let args = vec![
        "reta".to_string(),
        "-zeilen".to_string(),
        "--vorhervonausschnitt=1-1".to_string(),
        "-spalten".to_string(),
        "--kontinuum=m".to_string(),
        "-ausgabe".to_string(),
        "--keineueberschriften".to_string(),
        "--keineleereninhalte".to_string(),
        "--breite=8".to_string(),
    ];
    render_table_view_for_cli_args(
        &args,
        &TableMaterializationConfig::default(),
        &TableViewOutputConfig::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table_view::{MaterializedTableCellSource, MaterializedTableViewCell};

    #[test]
    fn shell_output_renders_continuum_m_direct_column_without_virtual_744_by_default() {
        let report = continuum_m_table_view_output_smoke(OutputMode::Shell);
        assert_eq!(report.mode, "shell");
        assert!(report.continuum_m_direct_header_present);
        assert!(!report.continuum_m_virtual_744_kept_as_witness);
        assert_eq!(report.virtual_cell_count, 0);
        assert!(report.contains_text("M Kontinuum"));
        assert!(report.contains_text("Neues M"));
        assert!(!report.contains_text("744:sternPolygon"));
    }

    #[test]
    fn csv_output_quotes_separator_and_html_output_escapes_markup() {
        assert_eq!(csv_escape_cell("a;b", ';'), "\"a;b\"");
        assert_eq!(csv_escape_cell("plain", ';'), "plain");
        assert_eq!(html_escape_cell("a<b&c>"), "a&lt;b&amp;c&gt;");
    }

    #[test]
    fn markdown_output_adds_separator_after_header() {
        let report = continuum_m_table_view_output_smoke(OutputMode::Markdown);
        assert_eq!(report.mode, "markdown");
        assert!(
            report
                .rendered_lines
                .iter()
                .any(|line| line.contains("---"))
        );
    }

    #[test]
    fn nichts_output_is_empty_but_keeps_report_metadata() {
        let report = continuum_m_table_view_output_smoke(OutputMode::Nichts);
        assert_eq!(report.mode, "nichts");
        assert!(report.rendered_lines.is_empty());
        assert!(report.visible_output_is_empty);
        assert!(!report.continuum_m_virtual_744_kept_as_witness);
    }

    #[test]
    fn output_cli_options_parse_width_and_header_flags() {
        let options = parse_table_view_output_cli_options(&[
            "reta",
            "-ausgabe",
            "--keineueberschriften",
            "--keineleereninhalte",
            "--breite=8",
            "--breiten=3,5,0",
            "--nocolor",
        ]);
        assert!(options.keineueberschriften);
        assert!(options.keineleereninhalte);
        assert_eq!(options.width, Some(8));
        assert_eq!(options.widths, vec![3, 5]);
        assert!(options.nocolor);
    }

    #[test]
    fn keineueberschriften_suppresses_header_but_keeps_data() {
        let report = output_flags_smoke();
        assert!(report.suppress_headers);
        assert!(!report.contains_text("M Kontinuum"));
        assert!(report.contains_text("Wege"));
    }

    #[test]
    fn positive_width_wraps_shell_cells_but_breite_zero_does_not() {
        assert_eq!(wrap_output_cell("abcdef", Some(2)), vec!["ab", "cd", "ef"]);
        assert_eq!(wrap_output_cell("abcdef", Some(0)), vec!["abcdef"]);
        assert_eq!(wrap_output_cell("abcdef", None), vec!["abcdef"]);
    }

    #[test]
    fn keineleereninhalte_filters_empty_rows() {
        let row = MaterializedTableViewRow {
            asset_name: "test.csv".to_string(),
            source_row_zero_based: 1,
            cells: Vec::new(),
        };
        let config = TableViewOutputConfig {
            include_empty_rows: false,
            ..TableViewOutputConfig::default()
        };
        assert!(render_shell_rows(&[row], &config).is_empty());
    }

    #[test]
    fn legacy_numbering_projection_adds_zaehlung_and_nummerierung_columns() {
        let args = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
        ];
        let report = render_table_view_for_cli_args(
            &args,
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default().with_legacy_numbering(),
        );
        assert_eq!(report.numbering_mode, "legacy-pair");
        assert_eq!(report.numbering_column_count, 2);
        assert!(report.rendered_lines[0].contains("Zählung"));
        assert!(report.rendered_lines[0].contains("Nummerierung"));
    }

    #[test]
    fn layout_is_disabled_by_default_but_can_be_enabled_by_breiten() {
        let report = output_layout_smoke();
        assert!(report.layout_enabled);
        assert!(report.layout_column_count >= 1);
        assert!(report.layout_page_count >= 1);
        assert!(!report.layout_column_widths.is_empty());
    }

    #[test]
    fn htmlclasses_option_uses_catalog_but_default_html_stays_plain() {
        let plain = continuum_m_table_view_output_smoke(OutputMode::Html);
        assert!(!plain.html_attribute_enabled);
        assert_eq!(plain.html_attribute_cell_count, 0);

        let args = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
            "-ausgabe".to_string(),
            "--art=html".to_string(),
            "--htmlclasses".to_string(),
        ];
        let report = render_table_view_for_cli_args(
            &args,
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert!(report.html_attribute_enabled);
        assert_eq!(report.html_attribute_policy, "class-only");
        assert!(report.html_attribute_cell_count > 0);
        assert!(
            report
                .rendered_lines
                .iter()
                .any(|line| line.contains("class=\""))
        );
    }

    #[test]
    fn nocolor_strips_inline_style_from_raw_html_class_output() {
        let args = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
            "-ausgabe".to_string(),
            "--art=html".to_string(),
            "--htmlrawclasses".to_string(),
            "--nocolor".to_string(),
        ];
        let report = render_table_view_for_cli_args(
            &args,
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert!(report.html_attribute_enabled);
        assert!(
            !report
                .rendered_lines
                .iter()
                .any(|line| line.contains("style=\""))
        );
    }

    #[test]
    fn rowcolors_option_styles_html_and_bbcode_rows_but_nocolor_disables() {
        let html_args = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
            "-ausgabe".to_string(),
            "--art=html".to_string(),
            "--rowcolors".to_string(),
        ];
        let html_report = render_table_view_for_cli_args(
            &html_args,
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert!(html_report.row_style_enabled);
        assert!(html_report.row_style_colored_row_count >= 1);
        assert!(
            html_report
                .rendered_lines
                .iter()
                .any(|line| line.contains("background-color"))
        );

        let bbcode_args = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
            "-ausgabe".to_string(),
            "--art=bbcode".to_string(),
            "--zeilenfarben".to_string(),
        ];
        let bbcode_report = render_table_view_for_cli_args(
            &bbcode_args,
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert!(
            bbcode_report
                .rendered_lines
                .iter()
                .any(|line| line.starts_with("[tr="))
        );

        let nocolor_args = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
            "-ausgabe".to_string(),
            "--art=html".to_string(),
            "--rowcolors".to_string(),
            "--nocolor".to_string(),
        ];
        let nocolor_report = render_table_view_for_cli_args(
            &nocolor_args,
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert!(!nocolor_report.row_style_enabled);
        assert!(
            !nocolor_report
                .rendered_lines
                .iter()
                .any(|line| line.contains("background-color"))
        );
    }

    #[test]
    fn row_number_policy_is_explicit_and_does_not_default_on() {
        let row = MaterializedTableViewRow {
            asset_name: "test.csv".to_string(),
            source_row_zero_based: 7,
            cells: vec![MaterializedTableViewCell {
                column_legacy: 493,
                source_row_zero_based: 7,
                value: "x".to_string(),
                source: MaterializedTableCellSource::Csv {
                    asset_name: "test.csv".to_string(),
                    source_column_index: 493,
                },
            }],
        };
        let default_values = row_values_with_options(&row, &TableViewOutputConfig::default(), 0);
        assert_eq!(default_values, vec!["x".to_string()]);
        let numbered = row_values_with_options(
            &row,
            &TableViewOutputConfig {
                include_row_numbers: true,
                ..TableViewOutputConfig::default()
            },
            0,
        );
        assert_eq!(numbered, vec!["7".to_string(), "x".to_string()]);
    }

    #[test]
    fn cellstyles_option_uses_legacy_generate_cell_for_html_and_bbcode() {
        let html_args = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
            "-ausgabe".to_string(),
            "--art=html".to_string(),
            "--cellstyles".to_string(),
        ];
        let html_report = render_table_view_for_cli_args(
            &html_args,
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert!(html_report.cell_style_enabled);
        assert_eq!(html_report.cell_style_policy, "legacy-generate-cell");
        assert!(html_report.cell_style_styled_cell_count > 0);
        assert!(
            html_report
                .rendered_lines
                .iter()
                .any(|line| line.contains("<td"))
        );

        let bbcode_args = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
            "-ausgabe".to_string(),
            "--art=bbcode".to_string(),
            "--cellstylewitness".to_string(),
        ];
        let bbcode_report = render_table_view_for_cli_args(
            &bbcode_args,
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert!(bbcode_report.cell_style_enabled);
        assert_eq!(
            bbcode_report.cell_style_policy,
            "legacy-generate-cell-witness"
        );
        assert!(
            bbcode_report
                .rendered_lines
                .iter()
                .any(|line| line.contains("[td"))
        );
    }

    #[test]
    fn htmlclasses_and_cellstyles_compose_in_one_td_begin_tag() {
        let args = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
            "-ausgabe".to_string(),
            "--art=html".to_string(),
            "--htmlclasses".to_string(),
            "--cellstyles".to_string(),
        ];
        let report = render_table_view_for_cli_args(
            &args,
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert!(report.html_attribute_enabled);
        assert!(report.cell_style_enabled);
        assert!(report.html_cell_style_composition_enabled);
        assert!(report.html_cell_style_composition_count > 0);
        assert!(
            report
                .rendered_lines
                .iter()
                .any(|line| line.contains("<td") && line.contains("class=\""))
        );
    }

    #[test]
    fn compose_html_td_open_tags_merges_class_and_style_attributes() {
        let merged = compose_html_td_open_tags(
            "<td class=\"z_0 r_493\" style=\"background:#fff;\">",
            "<td class=\"catalog witness\" style=\"color:#000;\">",
        );
        assert!(merged.contains("class=\"z_0 r_493 catalog witness\""));
        assert!(merged.contains("background:#fff"));
        assert!(merged.contains("color:#000"));
        assert!(!merged.contains("class=\"z_0 r_493\" class="));
    }

    #[test]
    fn nocolor_disables_cellstyles_like_other_style_projections() {
        let args = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
            "-ausgabe".to_string(),
            "--art=html".to_string(),
            "--cellstyles".to_string(),
            "--nocolor".to_string(),
        ];
        let report = render_table_view_for_cli_args(
            &args,
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert!(!report.cell_style_enabled);
        assert_eq!(report.cell_style_styled_cell_count, 0);
    }

    #[test]
    fn shell_colors_flag_activates_ansi_projection_without_touching_plain_cells() {
        let report = render_table_view_for_cli_args(
            &[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=1-1",
                "-spalten",
                "--kontinuum=m",
                "-ausgabe",
                "--shellcolors",
            ],
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert!(report.shell_style_enabled);
        assert!(report.shell_style_ansi_cell_count > 0);
        assert!(report.rendered_text.contains("\u{1b}["));
    }

    #[test]
    fn nocolor_disables_shell_color_projection() {
        let report = render_table_view_for_cli_args(
            &[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=1-1",
                "-spalten",
                "--kontinuum=m",
                "-ausgabe",
                "--shellcolors",
                "--nocolor",
            ],
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert!(!report.shell_style_enabled);
        assert_eq!(report.shell_style_ansi_cell_count, 0);
        assert!(!report.rendered_text.contains("\u{1b}["));
    }

    #[test]
    fn virtual_columns_flag_is_inert_for_direct_744_and_still_renders_non_direct_columns() {
        let plain = render_table_view_for_cli_args(
            &[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=1-1",
                "-spalten",
                "--kontinuum=m",
                "-ausgabe",
                "--spaltenreihenfolgeundnurdiese=744,493",
            ],
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert_eq!(plain.virtual_column_policy, "suppress");
        assert!(!plain.rendered_text.contains("744:sternPolygon"));

        let direct_summary = render_table_view_for_cli_args(
            &[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=1-1",
                "-spalten",
                "--kontinuum=m",
                "-ausgabe",
                "--spaltenreihenfolgeundnurdiese=744,493",
                "--virtualcolumns",
            ],
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert_eq!(direct_summary.virtual_column_policy, "tag-summary");
        assert!(!direct_summary.rendered_text.contains("744:sternPolygon,keinParaOdMetaP"));
        assert_eq!(direct_summary.virtual_cell_count, 0);

        let non_direct_summary = render_table_view_for_cli_args(
            &[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=1-1",
                "-spalten",
                "--religion=999",
                "--virtualcolumns",
            ],
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert_eq!(non_direct_summary.virtual_column_policy, "tag-summary");
        assert!(non_direct_summary.rendered_text.contains("999:untagged"));
        assert!(non_direct_summary.virtual_cell_count > 0);
    }

    #[test]
    fn virtual_placeholder_flag_can_emit_question_mark_virtuals() {
        let report = render_table_view_for_cli_args(
            &[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=1-1",
                "-spalten",
                "--religion=999",
                "--virtualplaceholder",
            ],
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default(),
        );
        assert_eq!(report.virtual_column_policy, "placeholder");
        assert!(!report.suppress_question_mark_virtuals);
        assert!(report.rendered_text.contains('?'));
    }

}
