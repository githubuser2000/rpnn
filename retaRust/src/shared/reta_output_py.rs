#![allow(non_snake_case)]

use std::collections::{BTreeMap, BTreeSet};

use crate::shared::lib4tables_enum_py::{tableTags2_for_column, ST};
use crate::shared::parallel_runtime::{self, ParallelArea};
use crate::shared::words_py::Words;
use hypher::{hyphenate, Lang};

use crate::shared::reta_program_types::{dedup_preserve_order_i64, PairStr, Program};

struct PyLikeIntExprParser<'a> {
    chars: Vec<char>,
    pos: usize,
    variable: Option<(&'a str, i64)>,
}

impl<'a> PyLikeIntExprParser<'a> {
    fn parse(text: &str, variable: Option<(&'a str, i64)>) -> Option<i64> {
        let mut parser = Self {
            chars: text.chars().collect(),
            pos: 0,
            variable,
        };
        let value = parser.parse_expr()?;
        parser.skip_ws();
        if parser.pos == parser.chars.len() {
            Some(value)
        } else {
            None
        }
    }

    fn skip_ws(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn starts_with(&mut self, needle: &str) -> bool {
        self.skip_ws();
        let mut idx = self.pos;
        for expected in needle.chars() {
            if self.chars.get(idx).copied() != Some(expected) {
                return false;
            }
            idx += 1;
        }
        true
    }

    fn consume_str(&mut self, needle: &str) -> bool {
        if !self.starts_with(needle) {
            return false;
        }
        self.pos += needle.chars().count();
        true
    }

    fn consume_char(&mut self, needle: char) -> bool {
        self.skip_ws();
        if self.chars.get(self.pos).copied() == Some(needle) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn parse_expr(&mut self) -> Option<i64> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Option<i64> {
        let mut value = self.parse_mul_mod()?;
        loop {
            if self.consume_char('+') {
                value = value.checked_add(self.parse_mul_mod()?)?;
            } else if self.consume_char('-') {
                value = value.checked_sub(self.parse_mul_mod()?)?;
            } else {
                break;
            }
        }
        Some(value)
    }

    fn parse_mul_mod(&mut self) -> Option<i64> {
        let mut value = self.parse_unary()?;
        loop {
            if self.consume_str("//") {
                let rhs = self.parse_unary()?;
                if rhs == 0 {
                    return None;
                }
                value = value.checked_div(rhs)?;
            } else if self.starts_with("**") {
                break;
            } else if self.consume_char('*') {
                value = value.checked_mul(self.parse_unary()?)?;
            } else if self.consume_char('%') {
                let rhs = self.parse_unary()?;
                if rhs == 0 {
                    return None;
                }
                value = value.checked_rem(rhs)?;
            } else {
                break;
            }
        }
        Some(value)
    }

    fn parse_unary(&mut self) -> Option<i64> {
        if self.consume_char('+') {
            self.parse_unary()
        } else if self.consume_char('-') {
            self.parse_unary()?.checked_neg()
        } else {
            self.parse_power()
        }
    }

    fn parse_power(&mut self) -> Option<i64> {
        let base = self.parse_primary()?;
        if self.consume_str("**") {
            let exponent = self.parse_unary()?;
            if exponent < 0 {
                return None;
            }
            base.checked_pow(exponent as u32)
        } else {
            Some(base)
        }
    }

    fn parse_primary(&mut self) -> Option<i64> {
        if self.consume_char('(') {
            let value = self.parse_expr()?;
            if !self.consume_char(')') {
                return None;
            }
            return Some(value);
        }

        self.skip_ws();
        let ch = self.chars.get(self.pos).copied()?;
        if ch.is_ascii_digit() {
            return self.parse_number();
        }
        if ch.is_ascii_alphabetic() || ch == '_' {
            return self.parse_identifier();
        }
        None
    }

    fn parse_number(&mut self) -> Option<i64> {
        self.skip_ws();
        let start = self.pos;
        while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        if start == self.pos {
            return None;
        }
        self.chars[start..self.pos]
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .ok()
    }

    fn parse_identifier(&mut self) -> Option<i64> {
        self.skip_ws();
        let start = self.pos;
        if self.pos >= self.chars.len() {
            return None;
        }
        let first = self.chars[self.pos];
        if !(first.is_ascii_alphabetic() || first == '_') {
            return None;
        }
        self.pos += 1;
        while self.pos < self.chars.len() {
            let ch = self.chars[self.pos];
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let name = self.chars[start..self.pos].iter().collect::<String>();
        if let Some((variable_name, variable_value)) = self.variable {
            if name == variable_name {
                return Some(variable_value);
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
struct StructuredRowRenderPy {
    line: String,
    is_header: bool,
    cells_len: usize,
}

impl Program {
    fn displayed_column_numbers_for_html_py(rowsRange: &[i64]) -> Vec<Option<u32>> {
        rowsRange
            .iter()
            .copied()
            .map(|v| if v >= 0 { Some(v as u32) } else { None })
            .collect()
    }

    fn output_row_number_type_py(num: i64) -> i64 {
        if num == 0 {
            return 0;
        }
        if num.abs() == 1 {
            return 2;
        }
        let factors = Self::prim_repeat2_py(&Self::prim_fak_py(num.abs()));
        if factors.len() == 1 && factors[0].1 == 1 {
            return 1;
        }
        if factors.len() == 1 {
            return 3;
        }
        if factors.is_empty() {
            return 0;
        }
        let mut intersection: Option<BTreeSet<i64>> = None;
        for (_, amount) in factors {
            let divisors: BTreeSet<i64> = (2..=amount)
                .filter(|divisor| amount % divisor == 0)
                .collect();
            if divisors.is_empty() {
                return 2;
            }
            intersection = Some(match intersection {
                Some(previous) => previous.intersection(&divisors).copied().collect(),
                None => divisors,
            });
        }
        match intersection {
            Some(values) if !values.is_empty() => 3,
            _ => 2,
        }
    }

    fn html_row_style_py(row_number: Option<i64>, is_header: bool) -> String {
        let num = if is_header {
            0
        } else {
            row_number.unwrap_or(0)
        };
        let style = match Self::output_row_number_type_py(num) {
            1 if num % 2 == 0 => "background-color:#66ff66;color:#000000;",
            1 => "background-color:#009900;color:#ffffff;",
            2 if num % 2 == 0 => "background-color:#ffff66;color:#000099;",
            2 => "background-color:#555500;color:#aaaaff;",
            3 if num % 2 == 0 => "background-color:#9999ff;color:#202000;",
            3 => "background-color:#000099;color:#ffff66;",
            _ => "background-color:#ff2222;color:#002222;",
        };
        format!(r#" style="{}""#, style)
    }

    fn bbcode_row_begin_py(row_number: Option<i64>, is_header: bool) -> String {
        let num = if is_header {
            0
        } else {
            row_number.unwrap_or(0)
        };
        match Self::output_row_number_type_py(num) {
            1 if num % 2 == 0 => r#"[tr="background-color:#66ff66;color:#000000;"]"#.to_string(),
            1 => r#"[tr="background-color:#009900;color:#ffffff;"]"#.to_string(),
            2 if num % 2 == 0 => r#"[tr="background-color:#ffff66;color:#000099;"]"#.to_string(),
            2 => r#"[tr="background-color:#555500;color:#aaaaff;"]"#.to_string(),
            3 if num % 2 == 0 => r#"[tr="background-color:#9999ff;color:#202000;"]"#.to_string(),
            3 => r#"[tr="background-color:#000099;color:#ffff66;"]"#.to_string(),
            _ => r#"[tr="background-color:#ff2222;color:#002222;"]"#.to_string(),
        }
    }

    fn html_exact_header_attrs_py(
        words: &Words,
        original_col: Option<u32>,
        html_col_idx: usize,
    ) -> String {
        let _ = words;

        if original_col.is_none() && html_col_idx == 0 {
            return r#" class="z_0 r_0 p1_✗Zählung,, p2_p3_0_, p4_" style="background-color:#ffffff;color:#000000;""#
            .to_string();
        }
        if original_col.is_none() && html_col_idx == 1 {
            return r#" class="z_0 r_1 p1_✗Nummerierung,, p2_p3_0_, p4_""#.to_string();
        }

        let r_part = format!("z_0 r_{}", html_col_idx);
        if let Some(original_col) = original_col {
            format!(r#" class="{} c_{}""#, r_part, original_col)
        } else {
            format!(r#" class="{}""#, r_part)
        }
    }

    fn html_escape_attr_value_py(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\n', " ")
            .replace('\r', " ")
    }

    fn html_syntax_tuple_for_special_column_exact_py(spalte: i64) -> Option<Vec<Vec<PairStr>>> {
        match spalte {
            -2 => Some(vec![vec![PairStr("Zählung".to_string(), String::new())]]),
            -1 => Some(vec![vec![PairStr("Nummerierung".to_string(), String::new())]]),
            _ => None,
        }
    }

    fn html_python_class_name_from_pair_exact_py(
        pair: &PairStr,
        para_num: usize,
        column_group: usize,
    ) -> String {
        if para_num == 0 {
            pair.0.clone()
        } else {
            format!("p3_{}_{}", column_group, pair.1)
        }
    }

    fn html_python_cell_parts_exact_py(
        &self,
        spalte_for_metadata: i64,
    ) -> Option<(String, String, bool)> {
        let tuple_of_lists_of_couples = if let Some(special) =
            Self::html_syntax_tuple_for_special_column_exact_py(spalte_for_metadata)
        {
            special
        } else if let Some(found) = self.generatedSpaltenParameter_Exact.get(&spalte_for_metadata) {
            found.clone()
        } else if let Some(found) = self
            .dataDict
            .get(0)
            .and_then(|dict| dict.get(&spalte_for_metadata.to_string()))
        {
            found.clone()
        } else {
            return None;
        };

        let mut things1: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        for (column_group, couples) in tuple_of_lists_of_couples.iter().enumerate() {
            let Some(first_pair) = couples.first() else {
                continue;
            };
            for para_num in 0..=1usize {
                things1
                    .entry(para_num)
                    .or_default()
                    .push(Self::html_python_class_name_from_pair_exact_py(
                        first_pair,
                        para_num,
                        column_group,
                    ));
            }
        }

        let has_symbole = things1
            .get(&0)
            .map(|values| values.iter().any(|value| value == "Symbole"))
            .unwrap_or(false);

        let mut rendered_parts: BTreeMap<usize, String> = BTreeMap::new();
        for (key, values) in things1 {
            let mut rendered = String::new();
            for value in values {
                if value == "alles" {
                    continue;
                }
                if key == 0 {
                    rendered.push('✗');
                }
                rendered.push_str(&value);
                rendered.push(',');
            }
            rendered_parts.insert(key, rendered);
        }

        if rendered_parts.len() < 2 {
            return None;
        }

        Some((
            rendered_parts.remove(&0).unwrap_or_default(),
            rendered_parts.remove(&1).unwrap_or_default(),
            has_symbole,
        ))
    }

    fn html_p4_values_exact_py(&self, spalte_for_metadata: i64) -> String {
        self.generatedSpaltenParameter_Tags
            .get(&spalte_for_metadata)
            .map(|tags| {
                tags.iter()
                    .map(|tag| tag.py_value().to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .unwrap_or_default()
    }

    fn html_cell_content_is_even_exact_py(content: Option<&str>) -> bool {
        content
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .and_then(|text| text.parse::<i64>().ok())
            .map(|value| value % 2 == 0)
            .unwrap_or(false)
    }

    fn html_symbol_attrs_exact_py() -> &'static str {
        r#" class="tdSymbole" style="background-image: url();background-size: cover;background-repeat: no-repeat;background-position: right; ""#
    }

    fn html_compact_body_cell_attrs_py(
        &self,
        spalte_for_metadata: Option<i64>,
        html_col_idx: usize,
        content: Option<&str>,
    ) -> String {
        let mut attrs = String::new();

        if html_col_idx == 0 {
            if Self::html_cell_content_is_even_exact_py(content) {
                attrs.push_str(r#" style="background-color:#000000;color:#ffffff;""#);
            } else {
                attrs.push_str(r#" style="background-color:#ffffff;color:#000000;""#);
            }
            return attrs;
        }

        if let Some(spalte_for_metadata) = spalte_for_metadata {
            if self
                .html_python_cell_parts_exact_py(spalte_for_metadata)
                .map(|(_, _, has_symbole)| has_symbole)
                .unwrap_or(false)
            {
                attrs.push_str(Self::html_symbol_attrs_exact_py());
            }
        }

        attrs
    }

    fn html_python_cell_attrs_exact_py(
        &self,
        spalte_for_metadata: Option<i64>,
        html_col_idx: usize,
        content: Option<&str>,
        _row_number: Option<i64>,
        is_header: bool,
    ) -> String {
        // Critical size guard: the normal CLI HTML output must not attach
        // header/column metadata to body cells.  Some data rows can fail
        // row-number parsing and therefore have `row_number == None`; that
        // must still never make them header cells.  Only the explicit
        // `is_header` flag may enable the large `p1_... p2_... p4_...`
        // header class string.
        if !is_header {
            return self.html_compact_body_cell_attrs_py(spalte_for_metadata, html_col_idx, content);
        }

        let mut attrs = String::new();

        let Some(spalte_for_metadata) = spalte_for_metadata else {
            return String::new();
        };

        let Some((p1_part, p2_part, has_symbole)) =
            self.html_python_cell_parts_exact_py(spalte_for_metadata)
        else {
            return Self::html_exact_header_attrs_py(
                &Words::new(),
                Some(html_col_idx as u32),
                html_col_idx,
            );
        };

        attrs.push_str(&format!(
            r#" class="z_0 r_{} p1_{}, p2_{} p4_{}""#,
            html_col_idx,
            Self::html_escape_attr_value_py(&p1_part),
            Self::html_escape_attr_value_py(&p2_part),
            Self::html_escape_attr_value_py(&self.html_p4_values_exact_py(spalte_for_metadata))
        ));

        if html_col_idx == 0 {
            if Self::html_cell_content_is_even_exact_py(content) {
                attrs.push_str(r#" style="background-color:#000000;color:#ffffff;""#);
            } else {
                attrs.push_str(r#" style="background-color:#ffffff;color:#000000;""#);
            }
        } else if has_symbole {
            attrs.push_str(Self::html_symbol_attrs_exact_py());
        }

        attrs
    }

    fn ordinary_column_tags_exact_py(&self, original_col: i64) -> Option<BTreeSet<ST>> {
        let mut tags: BTreeSet<ST> = BTreeSet::new();
        let in_set = |key: (usize, usize)| -> bool {
            self.spaltenArtenKey_SpaltennummernValue
                .get(&key)
                .map(|set| set.contains(&original_col))
                .unwrap_or(false)
        };

        if let Some(exact_tags) = tableTags2_for_column(original_col) {
            tags.extend(exact_tags);
        }

        if self.puniverseprims.contains(&original_col) {
            tags.extend([ST::sternPolygon, ST::universum, ST::galaxie]);
        }
        if in_set(self.spaltenTypeNaming.gebrGal1) {
            tags.extend([
                ST::sternPolygon,
                ST::galaxie,
                ST::gleichfoermigesPolygon,
                ST::gebrRat,
            ]);
        }
        if in_set(self.spaltenTypeNaming.gebroUni1) {
            tags.extend([
                ST::sternPolygon,
                ST::universum,
                ST::gleichfoermigesPolygon,
                ST::gebrRat,
            ]);
        }
        if in_set(self.spaltenTypeNaming.gebrEmo1) {
            tags.extend([
                ST::sternPolygon,
                ST::keinParaOdMetaP,
                ST::gleichfoermigesPolygon,
                ST::gebrRat,
            ]);
        }
        if in_set(self.spaltenTypeNaming.gebrGroe1) {
            tags.extend([
                ST::sternPolygon,
                ST::gleichfoermigesPolygon,
                ST::gebrRat,
                ST::keinParaOdMetaP,
            ]);
        }

        if tags.is_empty() {
            None
        } else {
            Some(tags)
        }
    }

    pub(crate) fn register_visible_column_metadata_exact_py(&mut self, original_col: i64) {
        if original_col < 0 {
            return;
        }
        if !self
            .generatedSpaltenParameter_Exact
            .contains_key(&original_col)
        {
            if let Some(entries) = self
                .dataDict
                .get(0)
                .and_then(|dict| dict.get(&original_col.to_string()))
                .cloned()
            {
                self.generatedSpaltenParameter_Exact
                    .insert(original_col, entries);
            }
        }
        if !self
            .generatedSpaltenParameter_Tags
            .contains_key(&original_col)
        {
            if let Some(tags) = self.ordinary_column_tags_exact_py(original_col) {
                self.generatedSpaltenParameter_Tags
                    .insert(original_col, tags);
            }
        }
    }

    fn prepare4out_width_for_display_col_py(
        &self,
        row_to_display_1_based: usize,
        combi_rows: usize,
    ) -> usize {
        // Python Prepare.setWidth(): if no terminal width/wrap context exists, keep the
        // cell as one string. Otherwise prefer --breiten values aligned to the visible
        // output columns, falling back to textWidth.
        if self.shellRowsAmount == 0 {
            return 0;
        }

        let selected_len = if combi_rows == 0 {
            self.rowsAsNumbers.len()
        } else {
            combi_rows
        };
        let start = self.rowsAsNumbers.len().saturating_sub(selected_len);
        let effective_breiten: Vec<i64> = if start < self.breiten.len() {
            self.breiten.iter().skip(start).copied().collect()
        } else {
            vec![]
        };

        let certain = if row_to_display_1_based >= 1
            && row_to_display_1_based - 1 < effective_breiten.len()
        {
            effective_breiten[row_to_display_1_based - 1]
        } else {
            self.textWidth
        };

        if certain <= 0 {
            0
        } else {
            certain as usize
        }
    }

    pub(crate) fn prepare_cell_work_py(&self, cell: &str, certaintextwidth: usize) -> Vec<String> {
        let stripped = cell.trim();
        if certaintextwidth == 0 {
            return vec![stripped.to_string()];
        }
        Self::wrap_text_py(stripped, certaintextwidth)
    }

    pub(crate) fn prepare4out_py(
        &mut self,
        paramLines: Vec<String>,
        paramLinesNot: Vec<String>,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
    ) -> (Vec<String>, Vec<Vec<String>>, i64, Vec<i64>, Vec<i64>) {
        let mut newTable: Vec<Vec<String>> = vec![];
        let mut finallyDisplayLines: Vec<String> = vec![];
        let mut old2newTable: Vec<i64> = vec![];

        if relitable.is_empty() {
            return (finallyDisplayLines, newTable, 0, vec![], old2newTable);
        }

        let physical_max_row = relitable.len().saturating_sub(1) as i64;
        // Python Prepare uses tables.hoechsteZeile[1024] as the default upper
        // row boundary.  The CSV may contain physical rows after 1024, but
        // --alles and generated row filters must not include them unless
        // --oberesmaximum/--vorhervonausschnitt raised the table limit.
        let max_row = if self.hoechsteZeile > 0 {
            std::cmp::min(physical_max_row, self.hoechsteZeile)
        } else {
            physical_max_row
        };
        let mut selected_rows: Vec<i64> =
            self.selected_rows_from_param_lines_py(&paramLines, &paramLinesNot, max_row);
        selected_rows = dedup_preserve_order_i64(selected_rows);
        if !self.keineUeberschriften && !selected_rows.contains(&0) {
            selected_rows.insert(0, 0);
        }

        let mut selected_cols: Vec<i64> = if rowsAsNumbers.is_empty() {
            if relitable[0].is_empty() {
                vec![]
            } else {
                (0..(relitable[0].len() as i64)).collect()
            }
        } else {
            rowsAsNumbers.clone()
        };
        selected_cols = dedup_preserve_order_i64(selected_cols);

        // The selected columns and wrap widths are fixed for every row in this
        // preparation pass.  Compute the widths once here instead of rebuilding
        // the effective --breiten view for every single cell below.
        let display_widths: Vec<usize> = (1..=selected_cols.len())
            .map(|row_to_display| {
                self.prepare4out_width_for_display_col_py(row_to_display, selected_cols.len())
            })
            .collect();

        let prepare_row = |row_no: i64| -> Option<(i64, Vec<String>)> {
            if row_no < 0 {
                return None;
            }
            let idx = row_no as usize;
            if idx >= relitable.len() {
                return None;
            }

            let mut new2Lines: Vec<String> = Vec::new();
            let mut row_to_display = 0usize;
            for original_col in selected_cols.iter().copied() {
                if original_col < 0 {
                    continue;
                }
                let col_idx = original_col as usize;
                if let Some(cell) = relitable[idx].get(col_idx) {
                    row_to_display += 1;
                    let certaintextwidth = display_widths
                        .get(row_to_display.saturating_sub(1))
                        .copied()
                        .unwrap_or(0);
                    let prepared = if certaintextwidth == 0 {
                        cell.trim().to_string()
                    } else {
                        Self::wrap_text_py(cell.trim(), certaintextwidth).join("\n")
                    };
                    new2Lines.push(prepared);
                }
            }
            Some((row_no, new2Lines))
        };

        if let Some((guard, ranges)) =
            parallel_runtime::reserve_ranges(ParallelArea::Output, selected_rows.len(), 4)
        {
            let prepared_rows: Vec<(i64, Vec<String>)> = std::thread::scope(|scope| {
                let _budget_guard = guard;
                let mut handles = Vec::new();
                for (start, end) in ranges {
                    let selected_rows = &selected_rows;
                    let prepare_row = &prepare_row;
                    handles.push(scope.spawn(move || {
                        let _depth_guard = parallel_runtime::enter_parallel_worker_scope();
                        let mut local_rows: Vec<(i64, Vec<String>)> = Vec::new();
                        for row_no in selected_rows[start..end].iter().copied() {
                            if let Some(prepared_row) = prepare_row(row_no) {
                                local_rows.push(prepared_row);
                            }
                        }
                        local_rows
                    }));
                }

                let mut rows: Vec<(i64, Vec<String>)> = Vec::new();
                for handle in handles {
                    match handle.join() {
                        Ok(mut local_rows) => rows.append(&mut local_rows),
                        Err(payload) => std::panic::resume_unwind(payload),
                    }
                }
                rows
            });

            for (old_row_no, prepared_row) in prepared_rows {
                newTable.push(prepared_row);
                old2newTable.push(old_row_no);
            }
        } else {
            for row_no in selected_rows.iter().copied() {
                if let Some((old_row_no, prepared_row)) = prepare_row(row_no) {
                    newTable.push(prepared_row);
                    old2newTable.push(old_row_no);
                }
            }
        }

        if newTable.is_empty() {
            newTable = relitable.clone();
            old2newTable = (0..(relitable.len() as i64)).collect();
        }

        finallyDisplayLines = old2newTable.iter().map(|n| n.to_string()).collect();
        if !finallyDisplayLines.is_empty() && !self.keineUeberschriften {
            finallyDisplayLines[0] = "".to_string();
        }

        let rowsRange: Vec<i64> = selected_cols.clone();
        for original_col in rowsRange.iter().copied() {
            self.register_visible_column_metadata_exact_py(original_col);
        }
        let numlen = old2newTable
            .last()
            .map(|v| v.to_string().len() as i64)
            .unwrap_or(0);
        (
            finallyDisplayLines,
            newTable,
            numlen,
            rowsRange,
            old2newTable,
        )
    }

    fn hoechste_zeile_114_py(&self, max_row_1024: i64) -> i64 {
        // Python Tables(None) starts with hoechsteZeile = {1024: 1024, 114: 163}.
        // The setter later writes both entries to the same explicit maximum.
        // Rust stores only the 1024 side as `hoechsteZeile`, so derive the
        // historical 114-side default here instead of widening the struct.
        let py_114 = if self.hoechsteZeile == 1024 {
            163
        } else {
            self.hoechsteZeile
        };
        std::cmp::min(max_row_1024, py_114)
    }

    fn selected_rows_from_param_lines_py(
        &self,
        param_lines: &[String],
        param_lines_not: &[String],
        max_row: i64,
    ) -> Vec<i64> {
        let mut selected_rows: Vec<i64> = self
            .filter_original_lines_py(BTreeSet::new(), param_lines, max_row)
            .into_iter()
            .collect();
        if selected_rows.is_empty() && !self.rowRange.is_empty() {
            selected_rows = self
                .rowRange
                .iter()
                .copied()
                .filter(|row| *row > 0 && *row <= max_row)
                .collect();
        }
        if !param_lines_not.is_empty() {
            let exclude_rows =
                self.filter_original_lines_py(BTreeSet::new(), param_lines_not, max_row);
            selected_rows.retain(|row| !exclude_rows.contains(row));
        }
        selected_rows.sort_unstable();
        selected_rows
    }

    fn cutset_py(wether: bool, a: &BTreeSet<i64>, b: &BTreeSet<i64>) -> BTreeSet<i64> {
        if wether {
            a.intersection(b).copied().collect()
        } else {
            a.clone()
        }
    }

    fn split_top_level_commas_filter_py(txt: &str) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut current = String::new();
        let mut depth_round = 0i32;
        let mut depth_square = 0i32;
        let mut depth_curly = 0i32;

        for ch in txt.chars() {
            match ch {
                '(' => {
                    depth_round += 1;
                    current.push(ch);
                }
                ')' => {
                    depth_round -= 1;
                    current.push(ch);
                }
                '[' => {
                    depth_square += 1;
                    current.push(ch);
                }
                ']' => {
                    depth_square -= 1;
                    current.push(ch);
                }
                '{' => {
                    depth_curly += 1;
                    current.push(ch);
                }
                '}' => {
                    depth_curly -= 1;
                    current.push(ch);
                }
                ',' if depth_round == 0 && depth_square == 0 && depth_curly == 0 => {
                    if !current.is_empty() {
                        out.push(current.clone());
                    }
                    current.clear();
                }
                _ => current.push(ch),
            }
        }
        if !current.is_empty() {
            out.push(current);
        }
        out
    }

    fn is_plain_zeilen_angabe_between_kommas_filter_py(txt: &str) -> bool {
        let txt = txt.trim();
        if txt.is_empty() {
            return false;
        }
        let mut parts = txt.split('+');
        let first = parts.next().unwrap_or_default();
        let first_ok = if let Some((a, b)) = first.split_once('-') {
            !a.is_empty()
                && !b.is_empty()
                && a.chars().all(|c| c.is_ascii_digit())
                && b.chars().all(|c| c.is_ascii_digit())
        } else {
            first.chars().all(|c| c.is_ascii_digit())
        };
        first_ok && parts.all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
    }

    fn is_python_identifier_py(txt: &str) -> bool {
        let mut chars = txt.chars();
        let Some(first) = chars.next() else {
            return false;
        };
        if !(first.is_ascii_alphabetic() || first == '_') {
            return false;
        }
        chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    }

    fn eval_python_int_expr_py(text: &str, variable: Option<(&str, i64)>) -> Option<i64> {
        PyLikeIntExprParser::parse(text, variable)
    }

    fn parse_python_like_range_values_py(start: i64, stop: i64, step: i64) -> Option<Vec<i64>> {
        if step == 0 {
            return None;
        }
        let mut values: Vec<i64> = vec![];
        let mut current = start;
        if step > 0 {
            while current < stop {
                values.push(current);
                current = current.checked_add(step)?;
            }
        } else {
            while current > stop {
                values.push(current);
                current = current.checked_add(step)?;
            }
        }
        Some(values)
    }

    fn parse_python_like_range_comprehension_py(inner: &str) -> Option<BTreeSet<i64>> {
        let (expr, rest) = inner.split_once(" for ")?;
        let (variable, iterable) = rest.split_once(" in ")?;
        let variable = variable.trim();
        if !Self::is_python_identifier_py(variable) {
            return None;
        }
        let iterable = iterable.trim();
        if !(iterable.starts_with("range(") && iterable.ends_with(')')) {
            return None;
        }
        let args_txt = &iterable["range(".len()..iterable.len() - 1];
        let args_parts: Vec<String> = Self::split_top_level_commas_filter_py(args_txt)
            .into_iter()
            .map(|part| part.trim().to_string())
            .filter(|part| !part.is_empty())
            .collect();
        if args_parts.is_empty() || args_parts.len() > 3 {
            return None;
        }
        let mut args: Vec<i64> = vec![];
        for part in args_parts {
            args.push(Self::eval_python_int_expr_py(&part, None)?);
        }
        let (start, stop, step) = match args.as_slice() {
            [stop] => (0, *stop, 1),
            [start, stop] => (*start, *stop, 1),
            [start, stop, step] => (*start, *stop, *step),
            _ => return None,
        };
        let mut out = BTreeSet::new();
        for value in Self::parse_python_like_range_values_py(start, stop, step)? {
            out.insert(Self::eval_python_int_expr_py(
                expr.trim(),
                Some((variable, value)),
            )?);
        }
        Some(out)
    }

    pub(crate) fn parse_python_like_int_set_expr_py(text: &str) -> Option<BTreeSet<i64>> {
        let trimmed = text.trim();
        if trimmed.len() < 2 {
            return None;
        }
        let normalized = if trimmed.starts_with('(') && trimmed.ends_with(')') {
            format!("[{}]", &trimmed[1..trimmed.len() - 1])
        } else {
            trimmed.to_string()
        };
        let inner = if (normalized.starts_with('[') && normalized.ends_with(']'))
            || (normalized.starts_with('{') && normalized.ends_with('}'))
        {
            &normalized[1..normalized.len() - 1]
        } else {
            return None;
        };
        let inner = inner.trim();
        if inner.is_empty() {
            return Some(BTreeSet::new());
        }
        if let Some(values) = Self::parse_python_like_range_comprehension_py(inner) {
            return Some(values);
        }
        let mut out = BTreeSet::new();
        let parts: Vec<String> = Self::split_top_level_commas_filter_py(inner)
            .into_iter()
            .map(|part| part.trim().to_string())
            .filter(|part| !part.is_empty())
            .collect();
        if parts.is_empty() {
            return None;
        }
        for part in parts {
            out.insert(Self::eval_python_int_expr_py(&part, None)?);
        }
        Some(out)
    }

    fn is_zeilen_angabe_between_kommas_filter_py(txt: &str) -> bool {
        let txt = txt.trim();
        if txt.is_empty() {
            return false;
        }
        let stripped_v = txt.strip_prefix('v').unwrap_or(txt);
        let stripped_plain = stripped_v.strip_prefix('-').unwrap_or(stripped_v);
        let generated_after_first = txt
            .char_indices()
            .nth(1)
            .map(|(idx, _)| &txt[idx..])
            .and_then(Self::parse_python_like_int_set_expr_py)
            .is_some();
        (!stripped_plain.is_empty()
            && Self::is_plain_zeilen_angabe_between_kommas_filter_py(stripped_plain))
            || Self::parse_python_like_int_set_expr_py(txt).is_some()
            || generated_after_first
    }

    fn is_zeilen_angabe_filter_py(txt: &str) -> bool {
        let parts = Self::split_top_level_commas_filter_py(txt);
        let any_at_all = parts.iter().any(|part| !part.is_empty());
        any_at_all
            && parts.iter().all(|part| {
                part.is_empty() || Self::is_zeilen_angabe_between_kommas_filter_py(part)
            })
    }

    pub(crate) fn bereich_to_numbers2_py(
        txt: &str,
        vielfache: bool,
        max_zahl: i64,
        allow_less_eq_zero: bool,
    ) -> BTreeSet<i64> {
        let cleaned_parts = Self::split_top_level_commas_filter_py(txt);
        let cleaned = cleaned_parts
            .iter()
            .filter(|s| !s.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join(",");
        if cleaned.is_empty() || !Self::is_zeilen_angabe_filter_py(&cleaned) {
            return BTreeSet::new();
        }

        // Python `center.BereichToNumbers2` changes `maxZahl == 0` to
        // `float("inf")` only for the outer non-`vielfache` call.  If a
        // single range then starts with `v`, Python passes 1028 into the
        // `vielfache` helper.  Keeping one global Rust maximum here used to
        // turn `v2` with open maximum into an almost unbounded loop.
        let python_global_max_is_inf = !vielfache && max_zahl == 0;
        let python_global_max = if python_global_max_is_inf {
            i64::MAX / 4
        } else {
            max_zahl
        };
        let mut dazu: BTreeSet<i64> = BTreeSet::new();
        let mut hinfort: BTreeSet<i64> = BTreeSet::new();

        for mut ein_bereich in Self::split_top_level_commas_filter_py(&cleaned) {
            if ein_bereich.is_empty() {
                continue;
            }
            if ein_bereich.len() > 1 && ein_bereich.starts_with('-') {
                if let Some(generated) = Self::parse_python_like_int_set_expr_py(&ein_bereich[1..])
                {
                    hinfort.extend(generated);
                    continue;
                }
            } else if !ein_bereich.starts_with('-') {
                if let Some(generated) = Self::parse_python_like_int_set_expr_py(&ein_bereich) {
                    dazu.extend(generated);
                    continue;
                }
            }

            let mut vielfache2 = vielfache;
            if let Some(rest) = ein_bereich.strip_prefix('v') {
                ein_bereich = rest.to_string();
                vielfache2 = true;
            }
            let range_max = if vielfache2 && python_global_max_is_inf {
                1028
            } else {
                python_global_max
            };

            let mut remove = false;
            if let Some(rest) = ein_bereich.strip_prefix('-') {
                remove = true;
                ein_bereich = rest.to_string();
            }

            let menge = if remove { &mut hinfort } else { &mut dazu };
            let mut around: Vec<i64> = Vec::new();
            let plus_parts: Vec<&str> = ein_bereich.split('+').collect();
            let normalized = if ein_bereich.chars().all(|c| c.is_ascii_digit()) {
                format!("{0}-{0}", ein_bereich)
            } else if !plus_parts.is_empty() && plus_parts[0].chars().all(|c| c.is_ascii_digit()) {
                let mut rebuilt = format!("{0}-{0}", plus_parts[0]);
                if plus_parts.len() > 1 {
                    rebuilt.push('+');
                    rebuilt.push_str(&plus_parts[1..].join("+"));
                }
                rebuilt
            } else {
                ein_bereich.clone()
            };

            let Some((left, right_all)) = normalized.split_once('-') else {
                continue;
            };
            if !left.chars().all(|c| c.is_ascii_digit()) || left == "0" {
                continue;
            }
            let right_parts: Vec<&str> = right_all.split('+').collect();
            if right_parts.is_empty() || !right_parts[0].chars().all(|c| c.is_ascii_digit()) {
                continue;
            }
            let start = left.parse::<i64>().unwrap_or(0);
            let end = right_parts[0].parse::<i64>().unwrap_or(0);
            if start <= 0 || end <= 0 || end < start {
                continue;
            }
            if right_parts.len() < 2 {
                around.push(0);
            } else {
                let mut ok = true;
                for part in &right_parts[1..] {
                    if part.chars().all(|c| c.is_ascii_digit()) {
                        around.push(part.parse::<i64>().unwrap_or(0));
                    } else {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }
            }

            if !vielfache2 {
                for number in start..=end {
                    for a in &around {
                        let c = number + *a;
                        if c < range_max {
                            menge.insert(c);
                        }
                        let d = number - *a;
                        if d > 0 && d < range_max {
                            menge.insert(d);
                        }
                    }
                }
            } else {
                let around_only_zero = around.is_empty() || around.iter().all(|a| *a == 0);
                let mut i = 0i64;
                loop {
                    let cond = around
                        .iter()
                        .all(|a| start.saturating_mul(i) < range_max.saturating_sub(*a));
                    if !cond {
                        break;
                    }
                    i += 1;
                    for number in start..=end {
                        if around_only_zero {
                            let c = number.saturating_mul(i);
                            if c <= range_max {
                                menge.insert(c);
                            }
                        } else {
                            for a in &around {
                                let c = number.saturating_mul(i) + *a;
                                if c <= range_max {
                                    menge.insert(c);
                                }
                                let d = number.saturating_mul(i) - *a;
                                if d > 0 && d < range_max {
                                    menge.insert(d);
                                }
                            }
                        }
                    }
                }
            }
        }

        let result: BTreeSet<i64> = dazu.difference(&hinfort).copied().collect();
        if allow_less_eq_zero {
            result
        } else {
            result.into_iter().filter(|x| *x > 0).collect()
        }
    }

    fn multiples_py(a: i64, mul1: bool) -> Vec<(i64, i64)> {
        let mut menge: BTreeSet<(i64, i64)> = BTreeSet::new();
        let mut b = 2i64;
        while b * b <= a {
            if a % b == 0 {
                menge.insert((a / b, b));
            }
            b += 1;
        }
        let mut out: Vec<(i64, i64)> = menge.into_iter().collect();
        if mul1 {
            out.push((a, 1));
        }
        out
    }

    fn teiler_py(zahlen_bereichs_angabe: &str) -> BTreeSet<i64> {
        let zahlen_bereich_menge =
            Self::bereich_to_numbers2_py(zahlen_bereichs_angabe, false, 0, false);
        let mut zahlen_wbereich_menge: BTreeSet<i64> = BTreeSet::new();
        for each1 in zahlen_bereich_menge {
            for each2 in Self::multiples_py(each1, true) {
                zahlen_wbereich_menge.insert(each2.0);
                zahlen_wbereich_menge.insert(each2.1);
            }
        }
        if zahlen_wbereich_menge != BTreeSet::from([1]) {
            zahlen_wbereich_menge.remove(&1);
        }
        zahlen_wbereich_menge
    }

    fn prim_fak_py(n: i64) -> Vec<i64> {
        if n < 2 {
            return vec![];
        }
        let mut z = n;
        let mut out: Vec<i64> = Vec::new();
        while z > 1 {
            let mut i = 2i64;
            let mut p = z;
            let mut gefunden = false;
            while i * i <= z && !gefunden {
                if z % i == 0 {
                    gefunden = true;
                    p = i;
                } else {
                    i += 1;
                }
            }
            out.push(p);
            z /= p;
        }
        out
    }

    fn prim_repeat2_py(factors: &[i64]) -> Vec<(i64, i64)> {
        let mut counts: BTreeMap<i64, i64> = BTreeMap::new();
        for &f in factors {
            *counts.entry(f).or_insert(0) += 1;
        }
        counts.into_iter().collect()
    }

    fn moonsun_rows_py(&self, moon_not_sun: bool, num_range: &BTreeSet<i64>) -> BTreeSet<i64> {
        let mut out = BTreeSet::new();
        for n in num_range {
            if Self::moon_number_is_py(*n) == moon_not_sun {
                out.insert(*n);
            }
        }
        out
    }

    fn is_prim_multiple_py(is_it: i64, multiples1: &[i64]) -> bool {
        if multiples1.is_empty() {
            return false;
        }
        if multiples1.contains(&is_it) {
            return true;
        }
        let mut seen_primes: BTreeSet<i64> = BTreeSet::new();
        for p in Self::prim_fak_py(is_it) {
            if seen_primes.insert(p) {
                let multiple2 = is_it / p;
                if multiples1.contains(&multiple2) {
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn filter_original_lines_py(
        &self,
        mut num_range: BTreeSet<i64>,
        param_lines: &[String],
        max_row: i64,
    ) -> BTreeSet<i64> {
        num_range.remove(&0);

        let relevant_params: BTreeSet<String> = param_lines
            .iter()
            .filter(|p| p.as_str() != "ka" && p.as_str() != "ka2")
            .cloned()
            .collect();

        if param_lines.iter().any(|p| p == "all")
            || relevant_params.is_empty()
            || !self.ifZeilenSetted
        {
            num_range = (1..=max_row).collect();
        } else {
            num_range.clear();
        }

        let mut if_a_at_all = false;
        let mut mehrere: Vec<String> = Vec::new();
        let mut if_teiler = false;
        for condition in param_lines {
            if condition.starts_with("_a_") && condition.len() > 3 {
                if_a_at_all = true;
                mehrere.push(condition[3..].to_string());
            }
            if condition.starts_with("_w_") {
                if_teiler = true;
            }
        }
        if if_a_at_all {
            let joined = mehrere.join(",");
            num_range.extend(Self::bereich_to_numbers2_py(
                &joined,
                false,
                max_row + 1,
                false,
            ));
            if if_teiler {
                // Python calls teiler() with the current selected row set, not
                // with the raw --vorhervonausschnitt argument.  That matters
                // when the range was already combined with `all` or other row
                // filters before --vorhervonausschnittteiler is applied.
                let current_rows_as_range = num_range
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                num_range.extend(Self::teiler_py(&current_rows_as_range));
            }
            if !num_range.is_empty() {
                for eins in joined.split(',') {
                    let mut eins = eins.to_string();
                    let ja1 = eins.starts_with('-');
                    let ja2 = eins.starts_with("v-");
                    if ja1 || ja2 {
                        if ja1 {
                            eins = eins[1..].to_string();
                        }
                        if ja2 {
                            eins = format!("v{}", &eins[2..]);
                        }
                        let minus = Self::bereich_to_numbers2_py(&eins, false, max_row + 1, false);
                        num_range = num_range.difference(&minus).copied().collect();
                    }
                }
            }
        }

        let mut if_b_at_all = false;
        mehrere.clear();
        let mut num_range_yes_z: BTreeSet<i64> = BTreeSet::new();
        for condition in param_lines {
            if condition.starts_with("_b_") && condition.len() > 3 {
                if_b_at_all = true;
                mehrere.push(condition[3..].to_string());
            }
        }
        if if_b_at_all {
            let max_row_114 = self.hoechste_zeile_114_py(max_row);
            if num_range.is_empty() && !if_a_at_all && !param_lines.iter().any(|p| p == "all") {
                num_range = (1..=max_row_114).collect();
            }
            let joined = mehrere.join(",");
            num_range_yes_z.extend(Self::bereich_to_numbers2_py(
                &joined,
                true,
                max_row_114 + 1,
                false,
            ));
            if !num_range_yes_z.is_empty() {
                num_range = num_range.intersection(&num_range_yes_z).copied().collect();
            }
            if !num_range.is_empty() {
                for eins in joined.split(',') {
                    let mut eins = eins.to_string();
                    let ja1 = eins.starts_with('-');
                    let ja2 = eins.starts_with("v-");
                    if ja1 || ja2 {
                        if ja1 {
                            eins = eins[1..].to_string();
                        }
                        if ja2 {
                            eins = format!("v{}", &eins[2..]);
                        }
                        let minus = Self::bereich_to_numbers2_py(&eins, true, max_row + 1, false);
                        num_range = num_range.difference(&minus).copied().collect();
                    }
                }
            }
        }

        num_range_yes_z.clear();
        let mut if_zeit_at_all = false;
        for condition in param_lines {
            if condition == "=" {
                if_zeit_at_all = true;
                if max_row >= 10 {
                    num_range_yes_z.insert(10);
                }
            } else if condition == "<" {
                if_zeit_at_all = true;
                num_range_yes_z.extend(1..=max_row.min(9));
            } else if condition == ">" {
                if_zeit_at_all = true;
                if max_row >= 11 {
                    num_range_yes_z.extend(11..=max_row);
                }
            }
        }
        if if_zeit_at_all {
            if num_range.is_empty()
                && !if_b_at_all
                && !if_a_at_all
                && !param_lines.iter().any(|p| p == "all")
                && num_range_yes_z.is_empty()
            {
                num_range = (1..=max_row).collect();
            }
            if if_a_at_all || param_lines.iter().any(|p| p == "all") || if_b_at_all {
                num_range = num_range.intersection(&num_range_yes_z).copied().collect();
            } else {
                num_range.extend(num_range_yes_z.iter().copied());
            }
        }

        num_range_yes_z.clear();
        let mut if_zaehlungen_at_all = false;
        mehrere.clear();
        for condition in param_lines {
            if condition.starts_with("_n_") && condition.len() > 3 {
                num_range_yes_z.extend(Self::bereich_to_numbers2_py(
                    &condition[3..],
                    false,
                    max_row + 1,
                    false,
                ));
                if_zaehlungen_at_all = true;
                mehrere.push(condition[3..].to_string());
            } else if let Some(rest) = condition.strip_prefix("zaehlung=") {
                num_range_yes_z.extend(Self::bereich_to_numbers2_py(
                    rest,
                    false,
                    max_row + 1,
                    false,
                ));
                if_zaehlungen_at_all = true;
                mehrere.push(rest.to_string());
            }
        }
        if if_zaehlungen_at_all {
            let mut num_range_yes_z2: BTreeSet<i64> = BTreeSet::new();
            if num_range.is_empty()
                && !if_a_at_all
                && !if_b_at_all
                && !param_lines.iter().any(|p| p == "all")
            {
                num_range = (1..=max_row).collect();
            }
            for n in &num_range {
                for z in &num_range_yes_z {
                    if Self::zeile_which_zaehlung_py(*n) == *z {
                        num_range_yes_z2.insert(*n);
                    }
                }
            }
            if !num_range_yes_z2.is_empty() && !num_range.is_empty() {
                num_range = num_range.intersection(&num_range_yes_z2).copied().collect();
            } else if num_range.is_empty() {
                num_range = num_range_yes_z2;
            }
            if !num_range.is_empty() {
                let mut minus_bereiche: BTreeSet<i64> = BTreeSet::new();
                for eins in mehrere.join(",").split(',') {
                    let mut eins = eins.to_string();
                    let ja1 = eins.starts_with('-');
                    let ja2 = eins.starts_with("v-");
                    if ja1 || ja2 {
                        if ja1 {
                            eins = eins[1..].to_string();
                        }
                        if ja2 {
                            eins = format!("v{}", &eins[2..]);
                        }
                        minus_bereiche.extend(Self::bereich_to_numbers2_py(
                            &eins,
                            false,
                            max_row + 1,
                            false,
                        ));
                    }
                }
                if !minus_bereiche.is_empty() {
                    let current_rows: Vec<i64> = num_range.iter().copied().collect();
                    for n in current_rows {
                        for z in &minus_bereiche {
                            if Self::zeile_which_zaehlung_py(n) == *z {
                                num_range.remove(&n);
                            }
                        }
                    }
                }
            }
        }

        num_range_yes_z.clear();
        if num_range.is_empty() && !relevant_params.is_empty() {
            num_range = (1..=max_row).collect();
        }
        let primzahl_filter_present = param_lines.iter().any(|p| {
            matches!(
                p.as_str(),
                "aussenerste" | "innenerste" | "aussenalle" | "innenalle"
            )
        });
        if primzahl_filter_present {
            let mut innen_aussen: BTreeMap<i64, (bool, bool, bool)> = BTreeMap::new();
            innen_aussen.insert(1, (true, false, true));
            for n in num_range.iter().copied().filter(|n| *n > 3) {
                let prim_zahlen = Self::prim_fak_py(n);
                let nur_eine_zahl = prim_zahlen.len() == 1;
                let ein_fach_vorkommen = nur_eine_zahl;
                let mut innen = false;
                let mut aussen = false;
                for prim_zahl in prim_zahlen {
                    if prim_zahl >= 4 {
                        let innen_or_aussen = prim_zahl % 6;
                        innen = innen || innen_or_aussen == 1;
                        aussen = aussen || innen_or_aussen == 5;
                    }
                }
                innen_aussen.insert(n, (innen, aussen, ein_fach_vorkommen));
            }
            if param_lines.iter().any(|p| p == "aussenerste") {
                num_range_yes_z.extend(innen_aussen.iter().filter_map(|(n, t)| {
                    if t.0 && t.2 {
                        Some(*n)
                    } else {
                        None
                    }
                }));
            }
            if param_lines.iter().any(|p| p == "innenerste") {
                num_range_yes_z.extend(innen_aussen.iter().filter_map(|(n, t)| {
                    if t.1 && t.2 {
                        Some(*n)
                    } else {
                        None
                    }
                }));
            }
            if param_lines.iter().any(|p| p == "aussenalle") {
                num_range_yes_z.extend(innen_aussen.iter().filter_map(|(n, t)| {
                    if t.0 {
                        Some(*n)
                    } else {
                        None
                    }
                }));
            }
            if param_lines.iter().any(|p| p == "innenalle") {
                num_range_yes_z.extend(innen_aussen.iter().filter_map(|(n, t)| {
                    if t.1 {
                        Some(*n)
                    } else {
                        None
                    }
                }));
            }
            let if_primtyp_at_all = !num_range_yes_z.is_empty();
            num_range = Self::cutset_py(if_primtyp_at_all, &num_range, &num_range_yes_z);
        }

        let mut if_typ_at_all = false;
        num_range_yes_z.clear();
        if num_range.is_empty() && !relevant_params.is_empty() {
            num_range = (1..=max_row).collect();
        }
        for condition in param_lines {
            if condition.contains("mond") {
                num_range_yes_z.extend(self.moonsun_rows_py(true, &num_range));
                if_typ_at_all = true;
            } else if condition.contains("schwarzesonne") {
                for n in &num_range {
                    if *n % 3 == 0 {
                        num_range_yes_z.insert(*n);
                    }
                }
                if_typ_at_all = true;
            } else if condition.contains("sonne") {
                num_range_yes_z.extend(self.moonsun_rows_py(false, &num_range));
                if_typ_at_all = true;
            } else if condition.contains("planet") {
                for n in &num_range {
                    if *n % 2 == 0 {
                        num_range_yes_z.insert(*n);
                    }
                }
                if_typ_at_all = true;
            } else if condition.contains("SonneMitMondanteil") {
                for n in &num_range {
                    let factors = Self::prim_repeat2_py(&Self::prim_fak_py(*n));
                    let booleans: BTreeSet<bool> =
                        factors.iter().map(|(_, faktor)| *faktor == 1).collect();
                    if booleans.contains(&true) && booleans.contains(&false) {
                        num_range_yes_z.insert(*n);
                    }
                }
                if_typ_at_all = true;
            }
        }
        num_range = Self::cutset_py(if_typ_at_all, &num_range, &num_range_yes_z);

        let mut prim_multiples: Vec<i64> = Vec::new();
        let mut if_prim_at_all = false;
        for condition in param_lines {
            if condition.len() > 1 && condition.ends_with('p') {
                if let Ok(v) = condition[..condition.len() - 1].parse::<i64>() {
                    if_prim_at_all = true;
                    prim_multiples.push(v);
                }
            }
        }
        num_range_yes_z.clear();
        if if_prim_at_all {
            if num_range.is_empty()
                && !if_b_at_all
                && !if_a_at_all
                && !param_lines.iter().any(|p| p == "all")
                && !if_typ_at_all
            {
                num_range = (1..=max_row).collect();
            }
            for n in &num_range {
                if Self::is_prim_multiple_py(*n, &prim_multiples) {
                    num_range_yes_z.insert(*n);
                }
            }
            num_range = Self::cutset_py(if_prim_at_all, &num_range, &num_range_yes_z);
        }

        let mut if_power_at_all = false;
        // Python intentionally reuses `mehrere` here instead of clearing it
        // after the zaehlung block.  This means combined _n_ and _^_ filters
        // feed both ranges into the power-base list.  Keep that quirk for
        // stdout parity.
        for condition in param_lines {
            if condition.starts_with("_^_") && condition.len() > 3 {
                if_power_at_all = true;
                mehrere.push(condition[3..].to_string());
            } else if condition.len() > 1 && condition.ends_with('^') {
                if_power_at_all = true;
                mehrere.push(condition[..condition.len() - 1].to_string());
            }
        }
        let to_power_it: Vec<i64> =
            Self::bereich_to_numbers2_py(&mehrere.join(","), false, max_row + 1, false)
                .into_iter()
                .collect();
        if if_power_at_all {
            num_range_yes_z.clear();
            if num_range.is_empty() && !relevant_params.is_empty() {
                num_range = (1..=max_row).collect();
            }
            if let Some(&last_el) = num_range.iter().next_back() {
                for base in to_power_it {
                    let mut n = 0u32;
                    loop {
                        let Some(one_power) = base.checked_pow(n) else {
                            break;
                        };
                        if one_power > last_el {
                            break;
                        }
                        num_range_yes_z.insert(one_power);
                        n += 1;
                    }
                }
                num_range = Self::cutset_py(if_power_at_all, &num_range, &num_range_yes_z);
                num_range.remove(&1);
            }
        }

        let mut if_multiples_from_any_at_all = false;
        let mut any_multiples: Vec<i64> = Vec::new();
        for condition in param_lines {
            if condition.len() > 1
                && condition.ends_with('v')
                && condition[..condition.len() - 1]
                    .chars()
                    .all(|c| c.is_ascii_digit())
            {
                if let Ok(v) = condition[..condition.len() - 1].parse::<i64>() {
                    if_multiples_from_any_at_all = true;
                    any_multiples.push(v);
                }
            }
        }
        if if_multiples_from_any_at_all {
            num_range_yes_z.clear();
            for n in &num_range {
                for divisor in &any_multiples {
                    if *divisor != 0 && *n % *divisor == 0 {
                        num_range_yes_z.insert(*n);
                    }
                }
            }
            num_range = Self::cutset_py(if_multiples_from_any_at_all, &num_range, &num_range_yes_z);
        }

        let max_row_114 = self.hoechste_zeile_114_py(max_row);
        let current_rows: Vec<i64> = num_range.iter().copied().collect();
        for n in current_rows {
            if !Self::moon_number_is_py(n) && n > max_row_114 {
                num_range.remove(&n);
            }
        }

        let invertieren = param_lines
            .iter()
            .any(|condition| condition.starts_with("_i_") || condition == "1i");
        if invertieren {
            let current: BTreeSet<i64> = num_range.clone();
            let mut num_range2_set = BTreeSet::new();
            for i in 1..=max_row {
                if (current.contains(&(i + 1)) || current.contains(&(i - 1)))
                    && !current.contains(&i)
                {
                    num_range2_set.insert(i);
                }
            }
            num_range = num_range2_set;
        }

        let num_range_list: Vec<i64> = num_range.iter().copied().collect();
        let num_range2_map: BTreeMap<i64, i64> = num_range_list
            .iter()
            .enumerate()
            .map(|(i, a)| ((i as i64) + 1, *a))
            .collect();

        let mut z_ja = false;
        let mut num_range_neu2: BTreeSet<i64> = BTreeSet::new();
        for condition in param_lines {
            if condition.starts_with("_z_") && condition.len() > 3 {
                z_ja = true;
                let neu = Self::bereich_to_numbers2_py(&condition[3..], false, max_row + 1, false);
                for a in num_range2_map
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    .intersection(&neu)
                    .copied()
                {
                    if let Some(mapped) = num_range2_map.get(&a) {
                        num_range_neu2.insert(*mapped);
                    }
                }
            }
        }
        if z_ja {
            num_range = num_range.intersection(&num_range_neu2).copied().collect();
        }

        let mut y_ja = false;
        num_range_neu2.clear();
        for condition in param_lines {
            if condition.starts_with("_y_") && condition.len() > 3 {
                y_ja = true;
                let neu = Self::bereich_to_numbers2_py(&condition[3..], true, max_row + 1, false);
                for a in num_range2_map
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    .intersection(&neu)
                    .copied()
                {
                    if let Some(mapped) = num_range2_map.get(&a) {
                        num_range_neu2.insert(*mapped);
                    }
                }
            }
        }
        if y_ja {
            num_range = num_range.intersection(&num_range_neu2).copied().collect();
        }

        num_range
    }

    fn hypher_lang_py(_word: &str) -> Lang {
        Lang::German
    }

    fn hard_split_long_word_py(word: &str, width: usize) -> Vec<String> {
        if width <= 1 {
            return word.chars().map(|c| c.to_string()).collect();
        }

        let chars: Vec<char> = word.chars().collect();
        let mut out: Vec<String> = Vec::new();
        let mut i = 0usize;
        while i < chars.len() {
            let remaining = chars.len() - i;
            if remaining <= width {
                out.push(chars[i..].iter().collect());
                break;
            }
            let take = width - 1;
            let mut s: String = chars[i..i + take].iter().collect();
            s.push('-');
            out.push(s);
            i += take;
        }
        out
    }

    fn split_long_word_py(word: &str, width: usize) -> Vec<String> {
        if width <= 1 || word.chars().count() <= width {
            return vec![word.to_string()];
        }

        // hypher panics for long words when alloc is disabled.
        // Keep display behavior unchanged for normal words and
        // hard-split only in the panic case.
        if word.len() > 45 {
            return Self::hard_split_long_word_py(word, width);
        }

        let lang = Self::hypher_lang_py(word);
        let syllables: Vec<String> = hyphenate(word, lang).map(|s| s.to_string()).collect();

        if syllables.is_empty() {
            return Self::hard_split_long_word_py(word, width);
        }

        let mut out: Vec<String> = Vec::new();
        let mut current = String::new();

        for (idx, syl) in syllables.iter().enumerate() {
            let is_last = idx + 1 == syllables.len();
            let syl_len = syl.chars().count();
            let current_len = current.chars().count();
            let reserve_for_hyphen = if is_last { 0usize } else { 1usize };

            if current.is_empty() {
                if syl_len >= width {
                    return Self::hard_split_long_word_py(word, width);
                }
                current.push_str(syl);
                continue;
            }

            if current_len + syl_len + reserve_for_hyphen <= width {
                current.push_str(syl);
            } else {
                let mut piece = current;
                piece.push('-');
                out.push(piece);
                current = syl.clone();
            }
        }

        if !current.is_empty() {
            out.push(current);
        }

        if out.is_empty() {
            Self::hard_split_long_word_py(word, width)
        } else {
            out
        }
    }

    pub(crate) fn wrap_text_py(txt: &str, width: usize) -> Vec<String> {
        if width == 0 {
            return vec![txt.to_string()];
        }
        let mut out: Vec<String> = vec![];
        for part in txt.split('\n') {
            let mut current = String::new();
            for word in part.split_whitespace() {
                if current.is_empty() {
                    if word.chars().count() <= width {
                        current.push_str(word);
                    } else {
                        out.extend(Self::split_long_word_py(word, width));
                    }
                } else if current.chars().count() + 1 + word.chars().count() <= width {
                    current.push(' ');
                    current.push_str(word);
                } else {
                    out.push(current);
                    current = String::new();
                    if word.chars().count() <= width {
                        current.push_str(word);
                    } else {
                        let pieces = Self::split_long_word_py(word, width);
                        if let Some((last, rest)) = pieces.split_last() {
                            for piece in rest {
                                out.push(piece.clone());
                            }
                            current = last.clone();
                        }
                    }
                }
            }
            if !current.is_empty() {
                out.push(current);
            }
            if part.is_empty() {
                out.push(String::new());
            }
        }
        if out.is_empty() {
            vec![String::new()]
        } else {
            out
        }
    }

    fn py_round_positive_i64(value: f64) -> i64 {
        let floor = value.floor();
        let frac = value - floor;
        if frac < 0.5 {
            floor as i64
        } else if frac > 0.5 {
            floor as i64 + 1
        } else {
            let floor_i = floor as i64;
            if floor_i % 2 == 0 {
                floor_i
            } else {
                floor_i + 1
            }
        }
    }

    pub(crate) fn moon_number_is_py(n: i64) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..n {
            let one_result = (n as f64).powf(1.0 / i as f64);
            if Self::py_round_positive_i64(one_result) * 100000
                == Self::py_round_positive_i64(one_result * 100000.0)
            {
                return true;
            }
        }
        false
    }

    fn prim_fak_len_py(n: i64) -> usize {
        if n <= 1 {
            return 0;
        }
        let mut faktoren: Vec<i64> = Vec::new();
        let mut z = n;
        while z > 1 {
            let mut i = 2i64;
            let mut gefunden = false;
            let mut p = z;
            while i * i <= n && !gefunden {
                if z % i == 0 {
                    gefunden = true;
                    p = i;
                } else {
                    i += 1;
                }
            }
            faktoren.push(p);
            z /= p;
        }
        faktoren.len()
    }

    pub(crate) fn zeile_which_zaehlung_py(zeile: i64) -> i64 {
        if zeile <= 0 {
            return 0;
        }

        // Bitgenaue Ableitung aus Python:
        // setZaehlungen():
        //   isMoon startet bei True
        //   bei jedem Übergang wasMoon == True && isMoon == False
        //   wird die nächste Zählung begonnen
        //
        // moonNumber(i)[0] != []  <=> perfekte Potenz > 1
        let mut zaehlung = 0i64;
        let mut is_moon = true;

        for i in 1..=zeile {
            let was_moon = is_moon;
            let moon_now = Self::moon_number_is_py(i);
            is_moon = moon_now;

            if was_moon && !is_moon {
                zaehlung += 1;
            }
        }

        zaehlung
    }

    pub(crate) fn shell_style_py(
        row_number: Option<i64>,
        is_header: bool,
        rest: bool,
    ) -> &'static str {
        if is_header {
            return "[41m[30m[4m";
        }
        let n = row_number.unwrap_or(0);
        if n <= 0 {
            return "";
        }
        if rest {
            if n % 2 == 0 {
                return "[47m[30m";
            }
            return "[40m[37m";
        }
        if Self::moon_number_is_py(n) {
            if n % 2 == 0 {
                return "[106m[30m";
            }
            return "[46m[30m";
        }
        if Self::prim_fak_len_py(n) == 1 {
            if n % 2 == 0 {
                return "[103m[30m[1m";
            }
            return "[43m[30m";
        }
        if n % 2 == 0 {
            return "[47m[30m";
        }
        "[100m[37m"
    }

    fn shell_reset_py(row_number: Option<i64>, is_header: bool, rest: bool) -> &'static str {
        if is_header {
            return "[0m";
        }
        let n = row_number.unwrap_or(0);
        if n <= 0 {
            return "";
        }
        if rest {
            return "[0m[0m";
        }
        if Self::moon_number_is_py(n) {
            return "[0m[0m";
        }
        if Self::prim_fak_len_py(n) == 1 {
            if n % 2 == 0 {
                return "[0m";
            }
            return "[0m[0m";
        }
        "[0m[0m"
    }

    pub(crate) fn styled_shell_text_py(
        text: &str,
        row_number: Option<i64>,
        is_header: bool,
        rest: bool,
        nocolor: bool,
    ) -> String {
        if nocolor || text.is_empty() {
            return text.to_string();
        }
        let style = Self::shell_style_py(row_number, is_header, rest);
        if style.is_empty() {
            text.to_string()
        } else {
            format!(
                "{}{}{}",
                style,
                text,
                Self::shell_reset_py(row_number, is_header, rest)
            )
        }
    }

    fn csv_escape_cell_py(text: &str) -> String {
        if text.contains(';') || text.contains('"') || text.contains('\n') || text.contains('\r') {
            format!("\"{}\"", text.replace('"', "\"\""))
        } else {
            text.to_string()
        }
    }

    fn markdown_escape_cell_py(text: &str) -> String {
        text.replace('|', "\\|").replace('\n', "<br>")
    }

    fn html_escape_cell_py(text: &str) -> String {
        // Cells are already decoded/escaped according to the selected output type
        // in csv_loader_py.  In HTML mode, raw CSV text is already HTML-escaped,
        // while generated fragments such as <ul>, <li>, <br> and nested tables are
        // intentionally trusted HTML.  Escaping here again turns `&gt;` into
        // `&amp;gt;` and `<ul>` into visible `&lt;ul&gt;`.  The renderer only has to
        // preserve line breaks in the compact one-line row format.
        text.replace('\n', "<br>")
    }

    fn row_prefix_text_py(&self, row_number: Option<i64>, is_header: bool) -> String {
        if !self.nummeriere {
            return String::new();
        }
        if is_header {
            return " ".to_string();
        }
        row_number
            .map(Self::zeile_which_zaehlung_py)
            .map(|value| value.to_string())
            .unwrap_or_else(|| " ".to_string())
    }

    fn limit_cell_height_py(&self, cell: &str) -> String {
        if self.textHeight <= 0 {
            return cell.to_string();
        }
        cell.split('\n')
            .take(self.textHeight as usize)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn should_skip_structured_row_py(&self, row: &[String], row_number: Option<i64>) -> bool {
        if self.keineUeberschriften && row_number.is_none() {
            return true;
        }
        if self.keineleereninhalte {
            let joined = row.join(" ");
            let stripped = joined.replace('-', "").replace('?', "").trim().to_string();
            if stripped.is_empty() {
                return true;
            }
        }
        false
    }

    fn render_structured_rows_ordered_py<F>(
        newTable: &[Vec<String>],
        min_rows_per_worker: usize,
        render_row: F,
    ) -> Vec<StructuredRowRenderPy>
    where
        F: Fn(usize, &Vec<String>) -> Option<StructuredRowRenderPy> + Sync,
    {
        let Some((guard, ranges)) =
            parallel_runtime::reserve_ranges(ParallelArea::Output, newTable.len(), min_rows_per_worker)
        else {
            let mut rows = Vec::new();
            for (row_idx, row) in newTable.iter().enumerate() {
                if let Some(rendered) = render_row(row_idx, row) {
                    rows.push(rendered);
                }
            }
            return rows;
        };

        std::thread::scope(|scope| {
            let _budget_guard = guard;
            let render_row = &render_row;
            let mut handles = Vec::new();
            for (start, end) in ranges {
                handles.push(scope.spawn(move || {
                    let _depth_guard = parallel_runtime::enter_parallel_worker_scope();
                    let mut rows = Vec::new();
                    for (offset, row) in newTable[start..end].iter().enumerate() {
                        let row_idx = start + offset;
                        if let Some(rendered) = render_row(row_idx, row) {
                            rows.push(rendered);
                        }
                    }
                    rows
                }));
            }

            let mut rows = Vec::new();
            for handle in handles {
                match handle.join() {
                    Ok(mut rendered) => rows.append(&mut rendered),
                    Err(payload) => std::panic::resume_unwind(payload),
                }
            }
            rows
        })
    }

    fn max_cell_widths_parallel_py(newTable: &[Vec<String>], col_count: usize) -> Vec<usize> {
        let compute_serial = || {
            let mut max_cell_widths: Vec<usize> = vec![0; col_count];
            for row in newTable {
                for col_idx in 0..col_count {
                    let cell = row.get(col_idx).map(String::as_str).unwrap_or("");
                    let cell_width = cell
                        .split('\n')
                        .map(|part| part.chars().count())
                        .max()
                        .unwrap_or(0);
                    if cell_width > max_cell_widths[col_idx] {
                        max_cell_widths[col_idx] = cell_width;
                    }
                }
            }
            max_cell_widths
        };

        let Some((guard, ranges)) =
            parallel_runtime::reserve_ranges(ParallelArea::Widths, newTable.len(), 32)
        else {
            return compute_serial();
        };

        std::thread::scope(|scope| {
            let _budget_guard = guard;
            let mut handles = Vec::new();
            for (start, end) in ranges {
                handles.push(scope.spawn(move || {
                    let _depth_guard = parallel_runtime::enter_parallel_worker_scope();
                    let mut local_widths: Vec<usize> = vec![0; col_count];
                    for row in &newTable[start..end] {
                        for col_idx in 0..col_count {
                            let cell = row.get(col_idx).map(String::as_str).unwrap_or("");
                            let cell_width = cell
                                .split('\n')
                                .map(|part| part.chars().count())
                                .max()
                                .unwrap_or(0);
                            if cell_width > local_widths[col_idx] {
                                local_widths[col_idx] = cell_width;
                            }
                        }
                    }
                    local_widths
                }));
            }

            let mut max_cell_widths: Vec<usize> = vec![0; col_count];
            for handle in handles {
                match handle.join() {
                    Ok(local_widths) => {
                        for (col_idx, width) in local_widths.into_iter().enumerate() {
                            if width > max_cell_widths[col_idx] {
                                max_cell_widths[col_idx] = width;
                            }
                        }
                    }
                    Err(payload) => std::panic::resume_unwind(payload),
                }
            }
            max_cell_widths
        })
    }

    fn render_shell_chunk_range_py(
        &self,
        finallyDisplayLines: &[String],
        newTable: &[Vec<String>],
        widths: &[usize],
        chunk_start: usize,
        chunk_end: usize,
        num_prefix_width: usize,
        row_start: usize,
        row_end: usize,
    ) -> Vec<String> {
        let mut one_chunk_lines: Vec<String> = vec![];

        for (offset, row) in newTable[row_start..row_end].iter().enumerate() {
            let row_idx = row_start + offset;
            if self.keineleereninhalte {
                let joined = (chunk_start..chunk_end)
                    .filter_map(|i| row.get(i))
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ");
                let stripped = joined.replace('-', "").replace('?', "").trim().to_string();
                if stripped.is_empty() {
                    continue;
                }
            }

            let row_number = finallyDisplayLines
                .get(row_idx)
                .and_then(|s| s.trim().parse::<i64>().ok());
            let is_header = row_number.is_none();

            let mut wrapped_cells: Vec<Vec<String>> = vec![];
            let mut max_sub = 1usize;
            for col_idx in chunk_start..chunk_end {
                let cell = row.get(col_idx).map(String::as_str).unwrap_or("");
                let wrapped = if widths[col_idx] == 0 {
                    let mut parts: Vec<String> =
                        cell.split('\n').map(|part| part.to_string()).collect();
                    if parts.is_empty() {
                        parts.push(String::new());
                    }
                    parts
                } else {
                    Self::wrap_text_py(cell, widths[col_idx])
                };
                if wrapped.len() > max_sub {
                    max_sub = wrapped.len();
                }
                wrapped_cells.push(wrapped);
            }

            let visible_sub_count = if self.textHeight > 0 {
                max_sub.min(self.textHeight as usize)
            } else {
                max_sub
            };

            for sub_idx in 0..visible_sub_count {
                let mut line = String::new();

                if self.nummeriere {
                    let label = if sub_idx == 0 {
                        finallyDisplayLines
                            .get(row_idx)
                            .cloned()
                            .unwrap_or_default()
                    } else {
                        String::new()
                    };
                    let prefix = if is_header {
                        " ".to_string()
                    } else if let Some(n) = row_number {
                        if Self::zeile_which_zaehlung_py(n) % 2 == 0 {
                            "█".to_string()
                        } else {
                            " ".to_string()
                        }
                    } else {
                        " ".to_string()
                    };
                    line.push_str(&prefix);
                    line.push_str(&format!("{:>width$} ", label, width = num_prefix_width));
                }

                for (local_i, abs_i) in (chunk_start..chunk_end).enumerate() {
                    let maybe_part = wrapped_cells[local_i].get(sub_idx).cloned();
                    let part = maybe_part.clone().unwrap_or_default();
                    let is_rest = maybe_part.is_none();
                    let rendered = if widths[abs_i] == 0 {
                        part
                    } else {
                        format!("{:<width$}", part, width = widths[abs_i])
                    };
                    line.push_str(&Self::styled_shell_text_py(
                        &rendered,
                        row_number,
                        is_header,
                        is_rest,
                        self.nocolor,
                    ));
                    if abs_i + 1 != chunk_end {
                        line.push(' ');
                    }
                }

                one_chunk_lines.push(line);
            }
        }

        one_chunk_lines
    }

    fn render_shell_chunk_ordered_py(
        &self,
        finallyDisplayLines: &[String],
        newTable: &[Vec<String>],
        widths: &[usize],
        chunk_start: usize,
        chunk_end: usize,
        num_prefix_width: usize,
    ) -> Vec<String> {
        let Some((guard, ranges)) =
            parallel_runtime::reserve_ranges(ParallelArea::Output, newTable.len(), 16)
        else {
            return self.render_shell_chunk_range_py(
                finallyDisplayLines,
                newTable,
                widths,
                chunk_start,
                chunk_end,
                num_prefix_width,
                0,
                newTable.len(),
            );
        };

        std::thread::scope(|scope| {
            let _budget_guard = guard;
            let mut handles = Vec::new();
            for (row_start, row_end) in ranges {
                handles.push(scope.spawn(move || {
                    let _depth_guard = parallel_runtime::enter_parallel_worker_scope();
                    self.render_shell_chunk_range_py(
                        finallyDisplayLines,
                        newTable,
                        widths,
                        chunk_start,
                        chunk_end,
                        num_prefix_width,
                        row_start,
                        row_end,
                    )
                }));
            }

            let mut one_chunk_lines = Vec::new();
            for handle in handles {
                match handle.join() {
                    Ok(mut rendered) => one_chunk_lines.append(&mut rendered),
                    Err(payload) => std::panic::resume_unwind(payload),
                }
            }
            one_chunk_lines
        })
    }

    fn render_structured_output_py(
        &mut self,
        finallyDisplayLines: &[String],
        newTable: &[Vec<String>],
        numlen: i64,
        rowsRange: &[i64],
    ) {
        let mut out_lines: Vec<String> = vec![];
        let mut chunked_lines: Vec<Vec<String>> = vec![];
        let out_type = self.outType.clone();
        let this: &Program = &*self;
        match out_type.as_str() {
            "nichts" => {}
            "csv" => {
                let rendered_rows =
                    Self::render_structured_rows_ordered_py(newTable, 32, |row_idx, row| {
                        let row_number = finallyDisplayLines
                            .get(row_idx)
                            .and_then(|s| s.trim().parse::<i64>().ok());
                        if this.should_skip_structured_row_py(row, row_number) {
                            return None;
                        }
                        let is_header = row_number.is_none();
                        let mut fields: Vec<String> = vec![];
                        if this.nummeriere {
                            fields.push(Self::csv_escape_cell_py(
                                &this.row_prefix_text_py(row_number, is_header),
                            ));
                            let label = finallyDisplayLines
                                .get(row_idx)
                                .cloned()
                                .unwrap_or_default();
                            fields.push(Self::csv_escape_cell_py(&label));
                        }
                        for cell in row {
                            let limited = this.limit_cell_height_py(cell);
                            fields.push(Self::csv_escape_cell_py(&limited));
                        }
                        let cells_len = fields.len();
                        Some(StructuredRowRenderPy {
                            line: fields.join(";"),
                            is_header,
                            cells_len,
                        })
                    });
                for rendered in rendered_rows {
                    out_lines.push(rendered.line.clone());
                    chunked_lines.push(vec![rendered.line]);
                }
            }
            "markdown" => {
                let rendered_rows =
                    Self::render_structured_rows_ordered_py(newTable, 32, |row_idx, row| {
                        let row_number = finallyDisplayLines
                            .get(row_idx)
                            .and_then(|s| s.trim().parse::<i64>().ok());
                        if this.should_skip_structured_row_py(row, row_number) {
                            return None;
                        }
                        let is_header = row_number.is_none();
                        let mut cells: Vec<String> = vec![];
                        if this.nummeriere {
                            cells.push(Self::markdown_escape_cell_py(
                                &this.row_prefix_text_py(row_number, is_header),
                            ));
                            cells.push(Self::markdown_escape_cell_py(
                                &finallyDisplayLines
                                    .get(row_idx)
                                    .cloned()
                                    .unwrap_or_default(),
                            ));
                        }
                        for cell in row {
                            let limited = this.limit_cell_height_py(cell);
                            cells.push(Self::markdown_escape_cell_py(&limited));
                        }
                        let cells_len = cells.len();
                        Some(StructuredRowRenderPy {
                            line: format!("|{}|", cells.join("|")),
                            is_header,
                            cells_len,
                        })
                    });
                let mut header_sep_done = false;
                for rendered in rendered_rows {
                    out_lines.push(rendered.line.clone());
                    let mut block = vec![rendered.line];
                    if rendered.is_header && !header_sep_done {
                        let sep = format!("|{}|", vec![":--:"; rendered.cells_len].join("|"));
                        out_lines.push(sep.clone());
                        block.push(sep);
                        header_sep_done = true;
                    }
                    chunked_lines.push(block);
                }
            }
            "emacs" => {
                let rendered_rows =
                    Self::render_structured_rows_ordered_py(newTable, 32, |row_idx, row| {
                        let row_number = finallyDisplayLines
                            .get(row_idx)
                            .and_then(|s| s.trim().parse::<i64>().ok());
                        if this.should_skip_structured_row_py(row, row_number) {
                            return None;
                        }
                        let is_header = row_number.is_none();
                        let mut cells: Vec<String> = vec![];
                        if this.nummeriere {
                            cells.push(this.row_prefix_text_py(row_number, is_header));
                            cells.push(
                                finallyDisplayLines
                                    .get(row_idx)
                                    .cloned()
                                    .unwrap_or_default(),
                            );
                        }
                        cells.extend(row.iter().map(|cell| this.limit_cell_height_py(cell)));
                        let cells_len = cells.len();
                        Some(StructuredRowRenderPy {
                            line: format!("|{}|", cells.join("|")),
                            is_header,
                            cells_len,
                        })
                    });
                for rendered in rendered_rows {
                    out_lines.push(rendered.line.clone());
                    let mut block = vec![rendered.line];
                    if rendered.is_header {
                        let sep = format!("|{}|", vec!["----"; rendered.cells_len].join("+"));
                        out_lines.push(sep.clone());
                        block.push(sep);
                    }
                    chunked_lines.push(block);
                }
            }
            "html" => {
                let displayed_columns = Self::displayed_column_numbers_for_html_py(rowsRange);
                let rendered_rows =
                    Self::render_structured_rows_ordered_py(newTable, 16, |row_idx, row| {
                        let row_number = finallyDisplayLines
                            .get(row_idx)
                            .and_then(|s| s.trim().parse::<i64>().ok());
                        if this.should_skip_structured_row_py(row, row_number) {
                            return None;
                        }
                        let is_header = row_idx == 0 && row_number.is_none();
                        let mut cells: Vec<String> = vec![];
                        if this.nummeriere {
                            let prefix_raw = this.row_prefix_text_py(row_number, is_header);
                            let label_raw = finallyDisplayLines
                                .get(row_idx)
                                .cloned()
                                .unwrap_or_default();
                            let prefix_text = Self::html_escape_cell_py(&prefix_raw);
                            let label_text = Self::html_escape_cell_py(&label_raw);
                            let prefix_attrs = this.html_python_cell_attrs_exact_py(
                                Some(-2),
                                0,
                                Some(&prefix_raw),
                                row_number,
                                is_header,
                            );
                            let label_attrs = this.html_python_cell_attrs_exact_py(
                                Some(-1),
                                1,
                                Some(&label_raw),
                                row_number,
                                is_header,
                            );
                            cells.push(format!(r#"<td{}>{}</td>"#, prefix_attrs, prefix_text));
                            cells.push(format!(r#"<td{}>{}</td>"#, label_attrs, label_text));
                        }
                        for (visible_idx, cell) in row.iter().enumerate() {
                            let html_col_idx = if this.nummeriere {
                                visible_idx + 2
                            } else {
                                visible_idx
                            };
                            let original_col =
                                displayed_columns.get(visible_idx).cloned().flatten();
                            let limited = this.limit_cell_height_py(cell);
                            let escaped = Self::html_escape_cell_py(&limited);
                            let attrs = this.html_python_cell_attrs_exact_py(
                                original_col.map(|col| col as i64),
                                html_col_idx,
                                Some(&limited),
                                row_number,
                                is_header,
                            );
                            cells.push(format!(r#"<td{}>{}</td>"#, attrs, escaped));
                        }
                        let cells_len = cells.len();
                        Some(StructuredRowRenderPy {
                            line: format!(
                                "<tr{}>{}</tr>",
                                Self::html_row_style_py(row_number, is_header),
                                cells.join(" ")
                            ),
                            is_header,
                            cells_len,
                        })
                    });
                out_lines.push(r#"<table border=0 id="bigtable">"#.to_string());
                let mut current_block = vec![r#"<table border=0 id="bigtable">"#.to_string()];
                for rendered in rendered_rows {
                    out_lines.push(rendered.line.clone());
                    current_block.push(rendered.line);
                }
                out_lines.push("</table>".to_string());
                current_block.push("</table>".to_string());
                chunked_lines.push(current_block);
            }
            "bbcode" => {
                let rendered_rows =
                    Self::render_structured_rows_ordered_py(newTable, 32, |row_idx, row| {
                        let row_number = finallyDisplayLines
                            .get(row_idx)
                            .and_then(|s| s.trim().parse::<i64>().ok());
                        if this.should_skip_structured_row_py(row, row_number) {
                            return None;
                        }
                        let is_header = row_number.is_none();
                        let mut cells: Vec<String> = vec![];
                        if this.nummeriere {
                            cells.push(format!(
                                "[td]{}[/td]",
                                this.row_prefix_text_py(row_number, is_header)
                            ));
                            cells.push(format!(
                                "[td]{}[/td]",
                                finallyDisplayLines
                                    .get(row_idx)
                                    .cloned()
                                    .unwrap_or_default()
                            ));
                        }
                        for cell in row {
                            let limited = this.limit_cell_height_py(cell);
                            cells.push(format!("[td]{}[/td]", limited.replace('\n', "<br>")));
                        }
                        let cells_len = cells.len();
                        Some(StructuredRowRenderPy {
                            line: format!(
                                "{}{}[/tr]",
                                Self::bbcode_row_begin_py(row_number, is_header),
                                cells.join("")
                            ),
                            is_header,
                            cells_len,
                        })
                    });
                out_lines.push("[table]".to_string());
                let mut current_block = vec!["[table]".to_string()];
                for rendered in rendered_rows {
                    out_lines.push(rendered.line.clone());
                    current_block.push(rendered.line);
                }
                out_lines.push("[/table]".to_string());
                current_block.push("[/table]".to_string());
                chunked_lines.push(current_block);
            }
            _ => {
                self.finallyDisplayLines = vec![];
                self.finallyDisplayLinesByChunks = vec![];
                self.numlen = numlen;
                return;
            }
        }
        self.finallyDisplayLinesByChunks = chunked_lines;
        self.finallyDisplayLines = out_lines;
        self.numlen = numlen;
    }

    pub(crate) fn cliOut_py(
        &mut self,
        finallyDisplayLines: Vec<String>,
        newTable: Vec<Vec<String>>,
        numlen: i64,
        rowsRange: Vec<i64>,
    ) -> Vec<Vec<String>> {
        let mut out_lines: Vec<String> = vec![];
        if newTable.is_empty() {
            self.finallyDisplayLines = out_lines.clone();
            self.numlen = numlen;
            self.finallyDisplayLinesByChunks = vec![];
            return newTable;
        }

        let col_count = newTable.iter().map(|row| row.len()).max().unwrap_or(0);
        if col_count == 0 {
            self.finallyDisplayLines = out_lines.clone();
            self.numlen = numlen;
            self.finallyDisplayLinesByChunks = vec![];
            return newTable;
        }

        if self.outType != "shell" {
            self.render_structured_output_py(&finallyDisplayLines, &newTable, numlen, &rowsRange);
            return newTable;
        }

        let max_cell_widths = Self::max_cell_widths_parallel_py(&newTable, col_count);

        let mut widths: Vec<usize> = vec![0; col_count];
        for col_idx in 0..col_count {
            let certain = if col_idx < self.breiten.len() {
                self.breiten[col_idx]
            } else {
                self.textWidth
            };
            widths[col_idx] = if certain > max_cell_widths[col_idx] as i64 || certain == 0 {
                max_cell_widths[col_idx]
            } else if certain < 0 {
                0
            } else {
                certain as usize
            };
        }

        let num_prefix_width = if self.nummeriere {
            finallyDisplayLines
                .iter()
                .map(|s| s.chars().count())
                .max()
                .unwrap_or(0)
        } else {
            0usize
        };

        let detected_shell_width = if self.shellWidth > 0 {
            self.shellWidth as usize
        } else {
            let detected = Self::detect_terminal_columns_py();
            if detected > 0 {
                detected as usize
            } else {
                0usize
            }
        };
        let chunk_budget = if detected_shell_width > 0 {
            detected_shell_width.saturating_sub(if self.nummeriere {
                num_prefix_width + 1
            } else {
                0
            })
        } else {
            0usize
        };

        let chunks: Vec<(usize, usize)> = if self.oneTable || chunk_budget == 0 {
            vec![(0, col_count)]
        } else {
            let mut chunks: Vec<(usize, usize)> = vec![];
            let mut start_col = 0usize;
            while start_col < col_count {
                let mut used = 0usize;
                let mut end_col = start_col;

                while end_col < col_count {
                    let add = widths[end_col].saturating_add(1);
                    if end_col > start_col && used.saturating_add(add) >= chunk_budget {
                        break;
                    }
                    used = used.saturating_add(add);
                    end_col += 1;
                }

                if end_col == start_col {
                    end_col += 1;
                }
                chunks.push((start_col, end_col));
                start_col = end_col;
            }
            chunks
        };

        let mut chunked_lines: Vec<Vec<String>> = vec![];

        for (chunk_index, (chunk_start, chunk_end)) in chunks.iter().cloned().enumerate() {
            let one_chunk_lines = self.render_shell_chunk_ordered_py(
                &finallyDisplayLines,
                &newTable,
                &widths,
                chunk_start,
                chunk_end,
                num_prefix_width,
            );
            if chunk_index > 0 && !one_chunk_lines.is_empty() {
                out_lines.push(String::new());
            }
            chunked_lines.push(one_chunk_lines.clone());
            out_lines.extend(one_chunk_lines);
        }

        self.finallyDisplayLinesByChunks = chunked_lines;
        self.finallyDisplayLines = out_lines.clone();
        self.numlen = numlen;
        newTable
    }

    pub fn prepareFinallyDisplayLines(&mut self) {
        self.finallyDisplayLines = vec![];
        self.finallyDisplayLinesByChunks = vec![];
        for row in self.__resultingTable.clone() {
            let line = row.join(" ; ");
            self.finallyDisplayLines.push(line.clone());
            self.finallyDisplayLinesByChunks.push(vec![line]);
        }
    }

    pub fn determineNumlen(&mut self) {
        self.numlen = self.__resultingTable.len() as i64;
    }

    pub fn addResultingTableToTables(&mut self) {
        self.tables.push(self.__resultingTable.clone());
    }

    pub fn setOld2Rows(&mut self) {
        self.old2Rows = self.__resultingTable.clone();
    }

    pub fn setNewerTable(&mut self) {
        self.newerTable = self.__resultingTable.clone();
    }

    pub fn setOldRows(&mut self) {
        self.oldRows = self.__resultingTable.clone();
    }

    pub fn setNewerRows(&mut self) {
        self.newerRows = self.__resultingTable.clone();
    }

    pub fn setRowsOfcombi(&mut self) {
        self.rowsOfcombi = self.__resultingTable.clone();
    }

    pub fn setOldTable(&mut self) {
        self.oldTable = self.__resultingTable.clone();
    }

    pub fn setGeneratedSpaltenParameter(&mut self) {
        self.generatedSpaltenParameter = self.sideParas.clone();
    }

    pub fn setAllEquColumns(&mut self) {
        self.allEquColumns = self.spaltenNumbers.clone();
    }

    pub fn setFinallyDisplayTable(&mut self) {
        self.finallyDisplayTable = self.__resultingTable.clone();
    }

    pub fn printOrStoreLines(&mut self) {
        if !self.ifPrint {
            return;
        }
        if self.finallyDisplayLines.len() == 0 && self.cliErrors.len() == 0 {
            self.prepareFinallyDisplayLines();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::reta_program_types::{PairStr, Program};

    #[test]
    fn bereich_to_numbers2_v_prefix_open_max_matches_python_cap() {
        let values = Program::bereich_to_numbers2_py("v2", false, 0, false);
        assert_eq!(values.len(), 514);
        assert!(values.contains(&2));
        assert!(values.contains(&1028));
        assert!(!values.contains(&1030));
    }

    #[test]
    fn bereich_to_numbers2_outer_vielfache_zero_max_stays_python_empty() {
        let values = Program::bereich_to_numbers2_py("2", true, 0, false);
        assert!(values.is_empty());
    }

    #[test]
    fn filter_original_lines_uses_python_114_default_for_multiples() {
        let mut program = Program::new(vec!["reta".to_string()]);
        program.ifZeilenSetted = true;
        let values = program.filter_original_lines_py(
            std::collections::BTreeSet::new(),
            &["_b_2".to_string()],
            1024,
        );
        assert!(values.contains(&162));
        assert!(!values.contains(&164));
    }

    #[test]
    fn filter_original_lines_uses_raised_max_for_multiples_after_python_setter() {
        let mut program = Program::new(vec!["reta".to_string()]);
        program.ifZeilenSetted = true;
        program.hoechsteZeile = 200;
        let values = program.filter_original_lines_py(
            std::collections::BTreeSet::new(),
            &["_b_2".to_string()],
            200,
        );
        assert!(values.contains(&200));
    }

    #[test]
    fn prepare4out_all_respects_python_default_1024_row_limit() {
        let mut program = Program::new(vec!["reta".to_string()]);
        program.ifZeilenSetted = true;
        let relitable: Vec<Vec<String>> = (0..1043).map(|row| vec![row.to_string()]).collect();

        let (_, _, _, _, old2new) =
            program.prepare4out_py(vec!["all".to_string()], vec![], relitable, vec![0]);

        assert_eq!(old2new.last().copied(), Some(1024));
        assert!(!old2new.contains(&1025));
    }

    #[test]
    fn prepare4out_all_honors_raised_python_row_limit() {
        let mut program = Program::new(vec!["reta".to_string()]);
        program.ifZeilenSetted = true;
        program.hoechsteZeile = 1040;
        let relitable: Vec<Vec<String>> = (0..1043).map(|row| vec![row.to_string()]).collect();

        let (_, _, _, _, old2new) =
            program.prepare4out_py(vec!["all".to_string()], vec![], relitable, vec![0]);

        assert_eq!(old2new.last().copied(), Some(1040));
        assert!(!old2new.contains(&1041));
    }


    #[test]
    fn html_cell_renderer_keeps_trusted_fragments_and_existing_entities() {
        assert_eq!(
            Program::html_escape_cell_py("<ul><li>10*n+m mit m&gt;0</li></ul>\nweiter"),
            "<ul><li>10*n+m mit m&gt;0</li></ul><br>weiter"
        );
    }

    #[test]
    fn html_cell_attrs_do_not_emit_runtime_debug_metadata_in_body_cells() {
        let mut program = Program::new(vec!["reta".to_string()]);
        program.generatedSpaltenParameter_Exact.insert(
            0,
            vec![vec![PairStr(
                "Religionen".to_string(),
                "Sternpolygon".to_string(),
            )]],
        );

        let header_attrs = program.html_python_cell_attrs_exact_py(Some(0), 2, None, None, true);
        assert!(header_attrs.contains("p1_✗Religionen,"));
        assert!(header_attrs.contains("p2_p3_0_Sternpolygon,"));
        assert!(!header_attrs.contains("data-column-number"));
        assert!(!header_attrs.contains("p1_col_"));

        let body_attrs = program.html_python_cell_attrs_exact_py(Some(0), 2, None, Some(1), false);
        assert_eq!(body_attrs, "");

        let body_attrs_without_parseable_row_number =
            program.html_python_cell_attrs_exact_py(Some(0), 2, None, None, false);
        assert_eq!(body_attrs_without_parseable_row_number, "");

        program.generatedSpaltenParameter_Exact.insert(
            1,
            vec![vec![PairStr("Symbole".to_string(), String::new())]],
        );
        let symbol_body_attrs =
            program.html_python_cell_attrs_exact_py(Some(1), 3, Some("☉"), Some(1), false);
        assert!(symbol_body_attrs.contains("tdSymbole"));
        assert!(!symbol_body_attrs.contains("data-column-number"));
        assert!(!symbol_body_attrs.contains("p1_col_"));
    }

    #[test]
    fn structured_row_prefix_uses_python_zaehlung_number() {
        let program = Program::new(vec!["reta".to_string()]);
        assert_eq!(program.row_prefix_text_py(Some(1), false), "1");
        assert_eq!(program.row_prefix_text_py(Some(5), false), "2");
    }

    #[test]
    fn text_height_limits_wrapped_shell_output_lines() {
        let mut program = Program::new(vec!["reta".to_string()]);
        program.outType = "shell".to_string();
        program.nocolor = true;
        program.nummeriere = false;
        program.oneTable = true;
        program.textWidth = 3;
        program.textHeight = 2;

        let table = vec![vec!["abcdefghi".to_string()]];
        let _ = program.cliOut_py(vec!["1".to_string()], table.clone(), 1, vec![1]);

        assert_eq!(
            program.finallyDisplayLines,
            vec!["abc".to_string(), "def".to_string()]
        );
    }

    #[test]
    fn text_height_limits_structured_cells_too() {
        let mut program = Program::new(vec!["reta".to_string()]);
        program.outType = "csv".to_string();
        program.nummeriere = false;
        program.textHeight = 2;

        let table = vec![vec!["a\nb\nc".to_string()]];
        let _ = program.cliOut_py(vec!["1".to_string()], table, 1, vec![1]);

        let joined = program.finallyDisplayLines.join("\n");
        assert!(joined.contains("a\nb"));
        assert!(!joined.contains("c"));
    }
}
