use indexmap::IndexMap;
use std::collections::BTreeSet;
use hypher::{hyphenate, Lang};

use crate::shared::reta_program_types::{dedup_preserve_order_i64, PairStr, Program, SpaltenTyp};
use crate::shared::words_py::{PyValue, StoreParameterEntry, Words};

impl Program {
    pub(crate) fn prepare4out_py(
        &mut self,
        _paramLines: Vec<String>,
        _paramLinesNot: Vec<String>,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
    ) -> (Vec<String>, Vec<Vec<String>>, i64, Vec<i64>, Vec<i64>) {
        let mut newTable: Vec<Vec<String>> = vec![];
        let mut finallyDisplayLines: Vec<String> = vec![];
        let mut old2newTable: Vec<i64> = vec![];

        if relitable.is_empty() {
            return (finallyDisplayLines, newTable, 0, vec![], old2newTable);
        }

        let mut selected_rows: Vec<i64> = if self.rowRange.is_empty() {
            (0..(relitable.len() as i64)).collect()
        } else {
            self.rowRange.clone()
        };
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
        let selected_cols_set: BTreeSet<i64> = selected_cols.iter().cloned().collect();

        for row_no in selected_rows.iter() {
            let idx = *row_no as usize;
            if idx >= relitable.len() {
                continue;
            }
            let mut new2Lines: Vec<String> = vec![];
            for (t, cell) in relitable[idx].iter().enumerate() {
                if selected_cols_set.contains(&(t as i64)) {
                    new2Lines.push(cell.clone());
                }
            }
            newTable.push(new2Lines);
            old2newTable.push(*row_no);
        }

        if newTable.is_empty() {
            newTable = relitable.clone();
            old2newTable = (0..(relitable.len() as i64)).collect();
        }

        finallyDisplayLines = old2newTable.iter().map(|n| n.to_string()).collect();
        if !finallyDisplayLines.is_empty() && !self.keineUeberschriften {
            finallyDisplayLines[0] = "".to_string();
        }

        let rowsRange: Vec<i64> = if newTable.is_empty() {
            vec![]
        } else {
            (0..(newTable[0].len() as i64)).collect()
        };
        let numlen = old2newTable.last().map(|v| v.to_string().len() as i64).unwrap_or(0);
        (finallyDisplayLines, newTable, numlen, rowsRange, old2newTable)
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
    let syllables: Vec<String> = hyphenate(word, lang)
        .map(|s| s.to_string())
        .collect();

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



    fn moon_number_is_py(n: i64) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..n {
            let one_result = (n as f64).powf(1.0 / i as f64);
            if (one_result.round() * 100000.0).round() == (one_result * 100000.0).round() {
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

    fn zeile_which_zaehlung_py(zeile: i64) -> i64 {
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

    pub(crate) fn shell_style_py(row_number: Option<i64>, is_header: bool, rest: bool) -> &'static str {
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

    pub(crate) fn styled_shell_text_py(text: &str, row_number: Option<i64>, is_header: bool, rest: bool, nocolor: bool) -> String {
        if nocolor || text.is_empty() {
            return text.to_string();
        }
        let style = Self::shell_style_py(row_number, is_header, rest);
        if style.is_empty() {
            text.to_string()
        } else {
            format!("{}{}[0m[0m", style, text)
        }
    }


    pub(crate) fn cliOut_py(
        &mut self,
        finallyDisplayLines: Vec<String>,
        newTable: Vec<Vec<String>>,
        numlen: i64,
        _rowsRange: Vec<i64>,
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

        // Python-like default shell width behavior:
        // without explicit --breite / --breiten, columns start at width 21.
        // Only explicit width arguments should override this.
        let mut widths: Vec<usize> = vec![21; col_count];
        for i in 0..col_count {
            let forced = if i < self.breiten.len() {
                self.breiten[i]
            } else {
                self.breite
            };
            if forced > 0 {
                widths[i] = forced as usize;
            }
        }

        let num_prefix_width = if self.nummeriere {
            finallyDisplayLines.iter().map(|s| s.chars().count()).max().unwrap_or(0)
        } else {
            0usize
        };

        let detected_shell_width = if self.shellWidth > 0 {
            self.shellWidth as usize
        } else {
            let detected = Self::detect_terminal_columns_py();
            if detected > 0 {
                detected as usize
            } else if self.textWidth > 0 {
                self.textWidth as usize
            } else {
                80usize
            }
        };
        let chunk_budget = detected_shell_width.max(21usize);

        let left_prefix = if self.nummeriere { num_prefix_width + 1 } else { 0usize };

        let mut chunks: Vec<(usize, usize)> = vec![];
        let mut start_col = 0usize;
        while start_col < col_count {
            let mut used = left_prefix;
            let mut end_col = start_col;

            while end_col < col_count {
                let add = if end_col == start_col {
                    widths[end_col]
                } else {
                    1 + widths[end_col]
                };

                if end_col > start_col && used + add > chunk_budget {
                    break;
                }
                used += add;
                end_col += 1;
            }

            if end_col == start_col {
                end_col += 1;
            }
            chunks.push((start_col, end_col));
            start_col = end_col;
        }

        let mut chunked_lines: Vec<Vec<String>> = vec![];

        for (chunk_start, chunk_end) in chunks.iter().cloned() {
            let mut one_chunk_lines: Vec<String> = vec![];

            for (row_idx, row) in newTable.iter().enumerate() {
                let mut wrapped_cells: Vec<Vec<String>> = vec![];
                let mut max_sub = 1usize;

                for i in chunk_start..chunk_end {
                    let cell = if i < row.len() { row[i].as_str() } else { "" };
                    let wrapped = Self::wrap_text_py(cell, widths[i]);
                    max_sub = max_sub.max(wrapped.len());
                    wrapped_cells.push(wrapped);
                }

                let mut should_skip_row = false;
                if self.keineleereninhalte {
                    let joined = (chunk_start..chunk_end)
                        .filter_map(|i| row.get(i))
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(" ");
                    let stripped = joined.replace('-', "").replace('?', "").trim().to_string();
                    if stripped.is_empty() {
                        should_skip_row = true;
                    }
                }
                if should_skip_row {
                    continue;
                }

                let row_number = finallyDisplayLines
                    .get(row_idx)
                    .and_then(|s| s.trim().parse::<i64>().ok());
                let is_header = row_number.is_none();

                for sub_idx in 0..max_sub {
                    let mut line = String::new();

                    if self.nummeriere {
                        let label = if sub_idx == 0 {
                            finallyDisplayLines.get(row_idx).cloned().unwrap_or_default()
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
                        let rendered = format!("{:<width$}", part, width = widths[abs_i]);
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
