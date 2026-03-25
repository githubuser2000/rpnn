use std::collections::{BTreeSet, HashMap};

use colored::*;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use crate::reta_ausgabe::output_syntax::OutputSyntax;
use crate::reta_ausgabe::table_cell::{TableCell, TableRow};
use crate::reta_ausgabe::tables::Tables;
use crate::reta_ausgabe::utils::{unicode_pad, word_wrap};

#[derive(Debug)]
pub struct CliOutput<'a> {
    pub out_type: OutputSyntax,
    pub color_enabled: bool,
    pub one_table: bool,
    pub table_width: usize,
    pub column_widths: Vec<usize>,
    pub line_numbering: bool,
    pub resulting_output: Vec<String>,
    pub tables_ref: &'a Tables,
    pub pretty_output: bool,
}

impl<'a> CliOutput<'a> {
    pub fn new(tables: &'a Tables, out_type: OutputSyntax) -> Self {
        CliOutput {
            out_type,
            color_enabled: true,
            one_table: false,
            table_width: 80,
            column_widths: Vec::new(),
            line_numbering: true,
            resulting_output: Vec::new(),
            tables_ref: tables,
            pretty_output: false,
        }
    }

    fn is_perfect_power(n: i32) -> bool {
        if n < 4 {
            return false;
        }
        let n64 = n as i64;
        let max_exp = 31 - (n as u32).leading_zeros();
        for exp in 2..=max_exp {
            let base = (n as f64).powf(1.0 / exp as f64).round() as i64;
            if base > 1 && base.pow(exp) == n64 {
                return true;
            }
        }
        false
    }

    fn is_prime(n: i32) -> bool {
        if n <= 1 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let mut d = 3;
        while d * d <= n {
            if n % d == 0 {
                return false;
            }
            d += 2;
        }
        true
    }


    fn pretty_print_text(&self, text: &str) -> String {
        if !self.pretty_output || matches!(self.out_type, OutputSyntax::Plain | OutputSyntax::Nichts) {
            return text.to_string();
        }
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = ts.themes.get("base16-ocean.dark").or_else(|| ts.themes.values().next());
let syntax = match self.out_type {
    OutputSyntax::HTML => ps.find_syntax_by_extension("html"),
    OutputSyntax::Markdown => ps.find_syntax_by_extension("md"),
    OutputSyntax::CSV => ps.find_syntax_by_extension("csv"),
    OutputSyntax::Emacs => ps.find_syntax_by_extension("el"),
    OutputSyntax::BBCode => {
        ps.find_syntax_by_extension("xml")
            .or_else(|| Some(ps.find_syntax_plain_text()))
    }
    _ => Some(ps.find_syntax_plain_text()),
}
.unwrap();
       let Some(theme) = theme else { return text.to_string(); };
        let mut h = HighlightLines::new(syntax, theme);
        let mut out = String::new();
        for line in LinesWithEndings::from(text) {
            match h.highlight_line(line, &ps) {
                Ok(ranges) => out.push_str(&as_24_bit_terminal_escaped(&ranges[..], false)),
                Err(_) => out.push_str(line),
            }
        }
        out
    }
    pub fn colorize(&self, text: &str, line_num: i32, is_empty: bool) -> String {
        if !self.color_enabled {
            return text.to_string();
        }
        match self.out_type {
            OutputSyntax::Plain => {
                if line_num == 0 {
                    return text.red().on_white().bold().to_string();
                }
                if is_empty {
                    return if line_num % 2 == 0 {
                        text.black().on_white().to_string()
                    } else {
                        text.white().on_black().to_string()
                    };
                }
                if Self::is_perfect_power(line_num) {
                    return if line_num % 2 == 0 {
                        text.black().on_cyan().to_string()
                    } else {
                        text.black().on_bright_cyan().to_string()
                    };
                }
                if Self::is_prime(line_num) {
                    return if line_num % 2 == 0 {
                        text.black().on_yellow().bold().to_string()
                    } else {
                        text.black().on_bright_yellow().to_string()
                    };
                }
                if line_num % 2 == 0 {
                    text.black().on_white().to_string()
                } else {
                    text.white().on_bright_black().to_string()
                }
            }
            _ => text.to_string(),
        }
    }

    pub fn cliout2(&mut self, text: &str) {
        self.resulting_output.push(text.to_string());
        if !matches!(self.out_type, OutputSyntax::Nichts) {
            let shown = self.pretty_print_text(text);
            print!("{}", shown);
        }
    }

    fn effective_width_for_col(&self, col_idx: usize, fallback: usize) -> usize {
        self.column_widths.get(col_idx).copied().unwrap_or(fallback)
    }

    fn wrapped_cell_lines(&self, cell: &TableCell, width: usize) -> Vec<String> {
        if matches!(self.out_type, OutputSyntax::Plain) {
            word_wrap(&cell.original_content, width)
        } else {
            vec![cell.original_content.clone()]
        }
    }

    fn row_wrapped_lines(&self, row: &TableRow, visible_col_indices: &[usize]) -> Vec<Vec<String>> {
        visible_col_indices
            .iter()
            .map(|&col_idx| {
                let width = self.effective_width_for_col(col_idx, self.table_width);
                if let Some(cell) = row.cells.get(col_idx) {
                    self.wrapped_cell_lines(cell, width)
                } else {
                    vec![String::new()]
                }
            })
            .collect()
    }

    fn visible_columns_for_row(&self, row: &TableRow) -> Vec<usize> {
        let numbering_width = if self.line_numbering { 5 } else { 0 };
        let mut total_width = numbering_width;
        let mut visible = Vec::new();
        for col_idx in 0..row.cells.len() {
            let width = self.effective_width_for_col(col_idx, self.table_width);
            let additional = width + 1;
            if self.one_table || total_width + additional < self.table_width {
                visible.push(col_idx);
                total_width += additional;
            } else if visible.is_empty() {
                visible.push(col_idx);
                break;
            } else {
                break;
            }
        }
        visible
    }

fn extract_header_meta(full: &str) -> (String, Option<String>) {
    let without_idx = if let Some(sep) = full.find("\u{1f}IDX:") {
        &full[..sep]
    } else {
        full
    };

    if let Some(start) = without_idx.find("p1_") {
        let visible = without_idx[..start].trim().to_string();
        let meta = without_idx[start..].trim().to_string();
        return (visible, Some(meta));
    }

    let visible = without_idx.trim().to_string();
    let abs_id = Self::extract_id_suffix(&visible);
    let visible = Self::strip_id_suffix(&visible);
    let meta = abs_id.and_then(Self::exact_header_meta_class_by_id);
    (visible, meta)
}
    fn extract_id_suffix(text: &str) -> Option<usize> {
        let pos = text.rfind("(ID_")?;
        let tail = &text[pos + 4..];
        let end = tail.find(')')?;
        tail[..end].parse().ok()
    }

    fn strip_id_suffix(text: &str) -> String {
        let mut out = text.trim().to_string();
        loop {
            if let Some(pos) = out.rfind("(ID_") {
                if out.ends_with(')') {
                    out = out[..pos].trim_end().to_string();
                    continue;
                }
            }
            break;
        }
        out.trim_matches('"').to_string()
    }

    fn exact_header_meta_class_by_id(id: usize) -> Option<String> {
        let s = match id {
            11 => "p1_✗Wichtigstes_zum_verstehen,✗Grundstrukturen,✗Menschliches,, p2_p3_0_Wichtigste,p3_1_Paradigmen_sind_Absichten_(13),p3_2_Motive,p3_3_,p3_4_,p3_5_,p3_6_, p4_0,3",
            19 => "p1_✗Menschliches,, p2_p3_0_Motive,p3_1_,p3_2_,p3_3_,p3_4_, p4_0,3",
            43 => "p1_✗Grundstrukturen,✗Grundstrukturen,✗Menschliches,, p2_p3_0_Reziprokes,p3_1_Paradigmen_sind_Absichten_(13),p3_2_Motive,p3_3_,p3_4_,p3_5_,p3_6_, p4_3,1",
            150 => "p1_✗Galaxie,✗Menschliches,, p2_p3_0_Transzendentalien_innen_außen,p3_1_Motive,p3_2_,p3_3_,p3_4_,p3_5_, p4_3,4,0",
            168 => "p1_✗Menschliches,, p2_p3_0_Motive,p3_1_,p3_2_,p3_3_,p3_4_, p4_3,1,0",
            169 => "p1_✗Menschliches,, p2_p3_0_Motive,p3_1_,p3_2_,p3_3_,p3_4_, p4_3,1,0",
            230 => "p1_✗Grundstrukturen,✗Multiversum,✗Grundstrukturen,✗Multiversum,✗Menschliches,✗Menschliches,, p2_p3_0_Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),p3_1_Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),p3_2_Geist_(15),p3_3_Geist_(15),p3_4_Motive,p3_5_Bewusstsein_und_Wahrnehmung,p3_6_,p3_7_,p3_8_,p3_9_, p4_4,0",
            231 => "p1_✗Menschliches,✗Menschliches,, p2_p3_0_Gefühle,p3_1_Motive,p3_2_,p3_3_,p3_4_,p3_5_, p4_4,0",
            242 => "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gesellschaftsschicht,p3_1_Klassen_(20),p3_2_,p3_3_,p3_4_,p3_5_,p3_6_, p4_5,0,3",
            243 => "p1_✗Universum,✗Grundstrukturen,✗Grundstrukturen,✗Multiversum,, p2_p3_0_Geist__(15),p3_1_nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15),p3_2_Geist_(15),p3_3_Geist_(15),p3_4_,p3_5_,p3_6_,p3_7_,p3_8_,p3_9_,p3_10_, p4_4,0",
            427 => "p1_✗Universum,✗Grundstrukturen,✗Multiversum,, p2_p3_0_Geist__(15),p3_1_Geist_(15),p3_2_Geist_(15),p3_3_,p3_4_,p3_5_,p3_6_,p3_7_,p3_8_,p3_9_, p4_4,0",
            552 => "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Feudalwesen,p3_1_Gesellschaftsklassen,p3_2_Leibeigenschaft,p3_3_,p3_4_,p3_5_,p3_6_, p4_8,0",
            556 => "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Klassen_(Fünfzehn_-_->_-_->_Zwanzig),p3_1_Gesellschaftsklassen,p3_2_,p3_3_,p3_4_,p3_5_,p3_6_, p4_5,0,3",
            585 => "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_SithSith_andUnd_jediJedi_asAls_oneEine_socialSoziale_societyGesellschafts_classSchicht_antagonisticAntagonistisch_toZu_hierarchyHierarchie_asAls_flashBlitz_electricityElektrizitaet_shatteringErschutternde_thunderDonnernde_structureStruktur_complicatedKomplitziert_difficultSchwierig_insteadAnstelle_ofVon_complexKomplex_cristallineKristalline_(antagonisticAntagonistisch_hierarchyHierarchie)_organizationOrganisations_waysWege_pathsPfade_netNetzwerk_webNetz_ofVon_theDer_galaxyGalaxie_oneEine_yourEure_galaxyGalaxie_topSpitzen_infrastructureInfrastruktur_societyGesellschaft_organizedOrganisiert_buildungBauen_architectureArchitektur_designEntwurf_waysWege_streetsStrassen_pathsPfade_peopleVolk_humansMenschen,_galaxyGalaxie_renamingUmbenennung_designationBezeichnung_warKrieg_ofDer_socialSoziale_classesSchichten_notNicht_anymoreMehr_pentagramPentagramme_warsKriege_starKrieg_warsDer-Sterne_galaxyGalaxie_insideInnerhalb_clusterCluster_localLokale_groupGruppe_galaxiesGalaxien_(fifeFuenf_pentagramPentagramm_E_E_vsGegenueber_T_T_twentyZwanzig_icosigramIkosigramm)_socialSoziale_classesKlassen_fightKampf,p3_1_,p3_2_,p3_3_,p3_4_, p4_5,0,3",
            698 => "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gesellschaftsklassen_20_gegenüber_Hierarchie_12,p3_1_,p3_2_,p3_3_,p3_4_, p4_5,0,3",
            _ => return None,
        };
        Some(s.to_string())
    }

    fn escape_html(content: &str) -> String {
        content
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    fn normalize_meta_label(label: &str, ober: &str) -> String {
        let mut out = label.replace(' ', "_");
        out = out.replace(",", "");
        out = out.replace('/', "_");
        out = out.replace('→', "_");
        out = out.replace("(", "_(");
        while out.contains("__") {
            out = out.replace("__", "_");
        }
        if ober == "Universum" && (out == "Geist_(15)" || out == "Geist(15)") {
            return "Geist__(15)".to_string();
        }
        out
    }

    fn header_meta_class(global_idx: usize) -> Option<String> {
        None
    }
fn render_html_table(&mut self, display_lines_list: &[usize], table: &[TableRow]) {
    self.cliout2("<table border=0 id=\"bigtable\">\n");

    for &display_line_idx in display_lines_list {
        let Some(row) = table.get(display_line_idx) else { continue; };
        if display_line_idx == 0 && self.tables_ref.keine_ueberschriften {
            continue;
        }

        let bg_fg = match row.original_line_num {
            0 => ("#ff2222", "#002222"),
            1 => ("#555500", "#aaaaff"),
            2 => ("#66ff66", "#000000"),
            3 => ("#009900", "#ffffff"),
            _ => ("#555500", "#aaaaff"),
        };

        let mut line = format!(
            "<tr style=\"background-color:{};color:{};\"> ",
            bg_fg.0, bg_fg.1
        );

        for (col_idx, cell) in row.cells.iter().enumerate() {
            let (visible_content, meta) = Self::extract_header_meta(&cell.original_content);
            let content = if display_line_idx == 0 && (col_idx == 0 || col_idx == 1) {
                String::new()
            } else {
                Self::escape_html(&visible_content)
            };

            if display_line_idx == 0 {
                let class_attr = if let Some(meta_str) = meta {
    format!(" class=\"z_0 r_{} {}\"", col_idx, meta_str)
} else {
    format!(" class=\"z_0 r_{}\"", col_idx)
};
                if col_idx == 0 {
                    line.push_str(&format!(
                        "<td{} style=\"background-color:#ffffff;color:#000000;\"> {} </td> ",
                        class_attr, content
                    ));
                } else {
                    line.push_str(&format!("<td{}> {} </td> ", class_attr, content));
                }
            } else {
                if col_idx == 0 {
                    line.push_str(&format!(
                        "<td style=\"background-color:#ffffff;color:#000000;\"> {} </td> ",
                        content
                    ));
                } else {
                    line.push_str(&format!("<td> {} </td> ", content));
                }
            }
        }

        line.push_str("</tr>\n");
        self.cliout2(&line);
    }

    self.cliout2("</table>");
}
    pub fn cli_out(
        &mut self,
        finally_display_lines: &BTreeSet<usize>,
        table: &[TableRow],
        _rows_range: std::ops::Range<usize>,
    ) -> Vec<String> {
        if finally_display_lines.is_empty() {
            return Vec::new();
        }

        let mut display_lines_list: Vec<usize> = finally_display_lines.iter().copied().collect();
        display_lines_list.sort();

        if matches!(self.out_type, OutputSyntax::HTML) {
            self.render_html_table(&display_lines_list, table);
            return self.resulting_output.clone();
        }

        if matches!(self.out_type, OutputSyntax::BBCode) {
            self.cliout2(self.out_type.begin_table());
        }

        for &display_line_idx in &display_lines_list {
            let Some(row) = table.get(display_line_idx) else { continue; };
            if display_line_idx == 0 && self.tables_ref.keine_ueberschriften {
                continue;
            }
            let visible_cols = self.visible_columns_for_row(row);
            if visible_cols.is_empty() {
                continue;
            }
            let wrapped_columns = self.row_wrapped_lines(row, &visible_cols);
            let row_height = wrapped_columns.iter().map(|lines| lines.len()).max().unwrap_or(1);

            for subline_idx in 0..row_height {
                let mut line_parts = Vec::new();
                if self.line_numbering {
                    let num_str = if row.original_line_num > 0 && subline_idx == 0 {
                        format!("{:4} ", row.original_line_num)
                    } else {
                        "     ".to_string()
                    };
                    line_parts.push(self.colorize(&num_str, row.original_line_num, false));
                }
                let mut entries_in_row = 0usize;
                let mut empty_entries = 0usize;
                for (visible_pos, &col_idx) in visible_cols.iter().enumerate() {
                    let width = self.effective_width_for_col(col_idx, self.table_width);
                    let content = wrapped_columns
                        .get(visible_pos)
                        .and_then(|lines| lines.get(subline_idx))
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    entries_in_row += 1;
                    if content.trim().is_empty() {
                        empty_entries += 1;
                    }
                    let formatted_content = if matches!(self.out_type, OutputSyntax::CSV) {
                        content.to_string()
                    } else {
                        let padded = unicode_pad(content, width, true);
                        self.colorize(&padded, row.original_line_num, content.trim().is_empty())
                    };
                    if matches!(self.out_type, OutputSyntax::CSV) {
                        line_parts.push(formatted_content);
                    } else {
                        line_parts.push(formatted_content);
                        line_parts.push(" ".to_string());
                    }
                }
                if empty_entries == entries_in_row {
                    continue;
                }
                match self.out_type {
                    OutputSyntax::CSV => {
                        self.cliout2(&(line_parts.join(";") + "\n"));
                    }
                    OutputSyntax::Markdown => {
                        let mut md_line = String::new();
                        if self.line_numbering {
                            md_line.push_str("| ");
                        }
                        md_line.push_str(&line_parts.join(" | "));
                        md_line.push_str(" |\n");
                        self.cliout2(&md_line);
                        if display_line_idx == 0 && subline_idx == 0 {
                            let separator = if self.line_numbering {
                                "|:---".repeat(visible_cols.len() + 1) + "|\n"
                            } else {
                                "|:---".repeat(visible_cols.len()) + "|\n"
                            };
                            self.cliout2(&separator);
                        }
                    }
                    OutputSyntax::Emacs => {
                        let mut line = String::new();
                        line.push('|');
                        line.push_str(&line_parts.join("|"));
                        line.push_str("\n");
                        self.cliout2(&line);
                    }
                    OutputSyntax::BBCode => {
                        let mut line = String::from("[tr]");
                        for part in &line_parts {
                            line.push_str("[td]");
                            line.push_str(part);
                            line.push_str("[/td]");
                        }
                        line.push_str("[/tr]\n");
                        self.cliout2(&line);
                    }
                    OutputSyntax::Plain => {
                        let mut full_line = String::new();
                        for part in &line_parts { full_line.push_str(part); }
                        full_line.push('\n');
                        self.cliout2(&full_line);
                    }
                    OutputSyntax::Nichts => {}
                    OutputSyntax::HTML => unreachable!(),
                }
            }
        }
        if matches!(self.out_type, OutputSyntax::BBCode) {
            self.cliout2(self.out_type.end_table());
        }
        self.resulting_output.clone()
    }

    pub fn find_max_cell_text_len(
        &self,
        _display_lines: &BTreeSet<usize>,
        _table: &[TableRow],
        _rows_range: &std::ops::Range<usize>,
    ) -> HashMap<usize, usize> {
        HashMap::new()
    }
}
