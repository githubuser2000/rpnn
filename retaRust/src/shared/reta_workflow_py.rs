#![allow(non_snake_case)]

use indexmap::IndexMap;
use std::collections::{BTreeMap, BTreeSet};

use crate::shared::lib4tables_enum_py::{
    tableTags2_kombiTable2_for_column, tableTags2_kombiTable_for_column,
};
use crate::shared::reta_program_types::{dedup_preserve_order_i64, PairStr, Program};
use crate::shared::words_py::Words;

impl Program {
    pub fn workflowEverything(&mut self, argv: Vec<String>, words: &Words) -> Vec<Vec<String>> {
        let (RowsLen, paramLines, paramLinesNot, relitable, rowsAsNumbers) =
            self.bringAllImportantBeginThings(argv, words);

        self.RowsLen = RowsLen;
        self.relitable = relitable;
        self.rowsAsNumbers = rowsAsNumbers;

        let finallyDisplayLinesEarly = self.prepare_display_lines_only_py(
            &paramLines,
            &paramLinesNot,
            self.relitable.len(),
        );
        let mut zeilenliste: Vec<i64> = finallyDisplayLinesEarly
            .iter()
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        zeilenliste.sort_unstable();
        self.lastLineNumber = zeilenliste.last().copied().unwrap_or(0);

        self.apply_post_csv_concat_generators_py();

        if self.helpPage() {
            self.__resultingTable = vec![];
            return vec![];
        }

        if self.cliErrors.len() > 0 {
            self.__resultingTable = vec![];
            return vec![];
        }

        let kombi13_rows = self.rows_of_combi_family1_numbers_py();
        let kombi15_rows = self.rowsOfcombi2.clone();

        let csv_names = self.csv_file_names();
        let mut rowsAsNumbers_after_generators = std::mem::take(&mut self.rowsAsNumbers);
        let (animalsProfessionsTable, kombiTable_Kombis, maintable2subtable_Relation) =
            if !kombi13_rows.is_empty() {
                self.readKombiCsv_py(&mut rowsAsNumbers_after_generators, &kombi13_rows, &csv_names.kombi13)
            } else {
                (vec![], vec![], (IndexMap::new(), IndexMap::new()))
            };
        let (animalsProfessionsTable2, kombiTable_Kombis2, maintable2subtable_Relation2) =
            if !kombi15_rows.is_empty() {
                self.readKombiCsv_py(&mut rowsAsNumbers_after_generators, &kombi15_rows, &csv_names.kombi15)
            } else {
                (vec![], vec![], (IndexMap::new(), IndexMap::new()))
            };
        self.rowsAsNumbers = dedup_preserve_order_i64(rowsAsNumbers_after_generators);

        let output_column_origins = self.selected_output_columns_py(&self.relitable, &self.rowsAsNumbers);

        if self.streamingMemoryMode
            && crate::reta_output_stream::active_streaming_enabled()
            && kombi13_rows.is_empty()
            && kombi15_rows.is_empty()
        {
            let relitable_for_streaming = std::mem::take(&mut self.relitable);
            let rows_for_streaming = self.rowsAsNumbers.clone();
            if self.stream_structured_output_direct_py(
                &paramLines,
                &paramLinesNot,
                relitable_for_streaming,
                rows_for_streaming,
            ) {
                self.relitable = Vec::new();
                self.rowsAsNumbers = Vec::new();
                self.CSVsAlreadRead.clear();
                self.tableGenerated = true;
                self.setGeneratedSpaltenParameter();
                self.setAllEquColumns();
                self.__resultingTable = Vec::new();
                self.tables = Vec::new();
                self.old2Rows = Vec::new();
                self.newerTable = Vec::new();
                self.oldRows = Vec::new();
                self.newerRows = Vec::new();
                self.oldTable = Vec::new();
                self.finallyDisplayTable = Vec::new();
                return Vec::new();
            }
            self.relitable = Vec::new();
        }

        let relitable_for_output = if self.streamingMemoryMode {
            std::mem::take(&mut self.relitable)
        } else {
            self.relitable.clone()
        };
        let (finallyDisplayLines, mut newTable, numlen, rowsRange, _old2newTable): (Vec<String>, Vec<Vec<String>>, i64, Vec<i64>, Vec<i64>) = self.prepare4out_py(
            paramLines.clone(),
            paramLinesNot.clone(),
            relitable_for_output,
            self.rowsAsNumbers.clone(),
        );

        if !kombi13_rows.is_empty() {
            let prepared_animalsProfessionsTable =
                self.prepare4out_kombi_table_py(&animalsProfessionsTable, &kombi13_rows);
            newTable = self.combiTableWorkflow_impl(
                prepared_animalsProfessionsTable,
                finallyDisplayLines.clone(),
                kombiTable_Kombis,
                maintable2subtable_Relation,
                newTable,
                _old2newTable.clone(),
                paramLines.clone(),
                &csv_names.kombi13,
                output_column_origins.clone(),
                &kombi13_rows,
            );
        }
        if !kombi15_rows.is_empty() {
            let prepared_animalsProfessionsTable2 =
                self.prepare4out_kombi_table_py(&animalsProfessionsTable2, &kombi15_rows);
            newTable = self.combiTableWorkflow_impl(
                prepared_animalsProfessionsTable2,
                finallyDisplayLines.clone(),
                kombiTable_Kombis2,
                maintable2subtable_Relation2,
                newTable,
                _old2newTable.clone(),
                paramLines.clone(),
                &csv_names.kombi15,
                output_column_origins.clone(),
                &kombi15_rows,
            );
        }

        let visible_rows_range = self.onlyThatColumns_i64_py(
            rowsRange.clone(),
            self.spaltenreihenfolgeundnurdiese.clone(),
        );
        newTable = self.onlyThatColumns_py(newTable, self.spaltenreihenfolgeundnurdiese.clone());
        self.newTable = !newTable.is_empty();
        if self.streamingMemoryMode {
            self.relitable = Vec::new();
            self.rowsAsNumbers = Vec::new();
            self.CSVsAlreadRead.clear();
        } else {
            self.finallyDisplayLines = finallyDisplayLines.clone();
        }
        self.numlen = numlen;

        let out: Vec<Vec<String>> = self.cliOut_py(finallyDisplayLines, newTable, numlen, visible_rows_range);
        let out_is_empty = out.is_empty();
        self.tableGenerated = self.newTable || !out_is_empty;
        self.setGeneratedSpaltenParameter();
        self.setAllEquColumns();

        if self.streamingMemoryMode {
            self.__resultingTable = Vec::new();
            self.tables = Vec::new();
            self.old2Rows = Vec::new();
            self.newerTable = Vec::new();
            self.oldRows = Vec::new();
            self.newerRows = Vec::new();
            self.oldTable = Vec::new();
            self.finallyDisplayTable = Vec::new();
            Vec::new()
        } else {
            self.__resultingTable = out.clone();
            self.addResultingTableToTables();
            self.setOld2Rows();
            self.setNewerTable();
            self.setOldRows();
            self.setNewerRows();
            self.setRowsOfcombi();
            self.setOldTable();
            self.setFinallyDisplayTable();
            out
        }
    }

    fn rows_of_combi_family1_numbers_py(&self) -> Vec<i64> {
        let mut out: Vec<i64> = vec![];
        for row in &self.rowsOfcombi {
            if let Some(first) = row.first() {
                if let Ok(v) = first.trim().parse::<i64>() {
                    out.push(v);
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    fn selected_output_columns_py(&self, relitable: &Vec<Vec<String>>, rowsAsNumbers: &Vec<i64>) -> Vec<i64> {
        if rowsAsNumbers.is_empty() {
            if relitable.is_empty() || relitable[0].is_empty() {
                vec![]
            } else {
                (0..(relitable[0].len() as i64)).collect()
            }
        } else {
            dedup_preserve_order_i64(rowsAsNumbers.clone())
        }
    }

    fn kombi_numbers_correct_test_and_set_py(input: &str, target: &mut Vec<i64>) {
        let num = input.trim();
        if num.len() > 2 && num.starts_with('(') && num.ends_with(')') {
            Self::kombi_numbers_correct_test_and_set_py(&num[1..num.len() - 1], target);
            return;
        }
        if !num.is_empty() && (num.chars().all(|c| c.is_ascii_digit()) || ((num.starts_with('+') || num.starts_with('-')) && num[1..].chars().all(|c| c.is_ascii_digit()))) {
            if let Ok(v) = num.parse::<i64>() {
                target.push(v.abs());
            }
            return;
        }
        if num.len() > 2 && num.contains('/') {
            if let Some((left, right)) = num.split_once('/') {
                Self::kombi_numbers_correct_test_and_set_py(left, target);
                Self::kombi_numbers_correct_test_and_set_py(right, target);
            }
        }
    }


    fn register_generated_kombi_parameter_exact_py(&mut self, spalte: i64, csvFileName: &str, csv_col_number: i64) {
        let mut parameter_groups: Vec<Vec<PairStr>> = vec![];
        let csv_key = csv_col_number.to_string();
        if csvFileName.contains("meta") {
            if let Some(entries) = self.dataDict.get(8).and_then(|dict| dict.get(&csv_key)) {
                let mut into: Vec<PairStr> = vec![];
                for entry in entries {
                    for pair in entry {
                        into.push(PairStr(
                            "Kombination_(Universum_und_Galaxie)_(14_mit_15)".to_string(),
                            pair.1.clone(),
                        ));
                    }
                }
                if !into.is_empty() {
                    parameter_groups.push(into);
                }
            }
        } else if let Some(entries) = self.dataDict.get(3).and_then(|dict| dict.get(&csv_key)) {
            let mut into: Vec<PairStr> = vec![];
            let mut needs_order_group = false;
            for entry in entries {
                for pair in entry {
                    let parameter_name = pair.1.clone();
                    if parameter_name == "tiere" || parameter_name == "berufe" || parameter_name == "intelligenz" {
                        needs_order_group = true;
                    }
                    into.push(PairStr(
                        "Kombination_(Galaxie_und_schwarzes_Loch)_(14_mit_13)".to_string(),
                        parameter_name,
                    ));
                }
            }
            if !into.is_empty() {
                parameter_groups.push(into);
            }
            if needs_order_group {
                parameter_groups.push(vec![PairStr(
                    "Wichtigstes_zum_gedanklich_einordnen".to_string(),
                    "Zweitwichtigste".to_string(),
                )]);
            }
        }

        if !parameter_groups.is_empty() {
            self.generatedSpaltenParameter_Exact.insert(spalte, parameter_groups);
        }

        let exact_tags = if csvFileName.contains("meta") {
            tableTags2_kombiTable2_for_column(csv_col_number)
        } else {
            tableTags2_kombiTable_for_column(csv_col_number)
        };
        if let Some(collected) = exact_tags {
            if !collected.is_empty() {
                self.generatedSpaltenParameter_Tags.insert(spalte, collected);
            }
        }
    }

    fn readKombiCsv_py(
        &mut self,
        rowsAsNumbers: &mut Vec<i64>,
        rowsOfcombi: &[i64],
        csvFileName: &str,
    ) -> (Vec<Vec<String>>, Vec<Vec<i64>>, (IndexMap<i64, i64>, IndexMap<i64, i64>)) {
        let rows = match self.load_csv_rows_semicolon_exact_path(csvFileName) {
            Ok(rows) => rows,
            Err(_) => return (vec![], vec![], (IndexMap::new(), IndexMap::new())),
        };

        let headingsAmount = self.relitable.first().map(|row| row.len()).unwrap_or(0) as i64;
        let mut maintable2subtable_Relation: (IndexMap<i64, i64>, IndexMap<i64, i64>) = (IndexMap::new(), IndexMap::new());
        let mut kombiTable: Vec<Vec<String>> = vec![];
        let mut kombiTable_Kombis: Vec<Vec<i64>> = vec![];
        let mut maxlen = 0usize;

        for (z, row) in rows.into_iter().enumerate() {
            let mut col = row.clone();
            if let Some(first) = col.first().cloned() {
                for i in 1..col.len() {
                    if !col[i].trim().is_empty() && !first.trim().is_empty() {
                        col[i] = format!("({}) {} ({})", first, col[i], first);
                    }
                }
            }
            maxlen = maxlen.max(col.len());
            if z > 0 && !col.is_empty() && !col[0].trim().is_empty() {
                let mut parsed: Vec<i64> = vec![];
                for num in col[0].split('|') {
                    Self::kombi_numbers_correct_test_and_set_py(num, &mut parsed);
                }
                kombiTable_Kombis.push(parsed);
            }
            kombiTable.push(col);
        }

        if maxlen > 0 {
            for row in kombiTable.iter_mut() {
                while row.len() < maxlen {
                    row.push(String::new());
                }
            }
        }

        if !self.relitable.is_empty() && !kombiTable.is_empty() {
            let added_cols = maxlen.saturating_sub(1);
            let header_animcol = &kombiTable[0];
            for t in 0..added_cols {
                let new_main_idx = self.relitable[0].len() as i64;
                let sub_idx = t as i64;
                maintable2subtable_Relation.0.insert(new_main_idx, sub_idx);
                maintable2subtable_Relation.1.insert(sub_idx, new_main_idx);
                let heading = header_animcol.get(t + 1).cloned().unwrap_or_default();
                self.relitable[0].push(heading.clone());
                if !heading.is_empty() {
                    self.generatedSpaltenParameter.push(heading);
                }
                self.register_generated_kombi_parameter_exact_py(new_main_idx, csvFileName, (t + 1) as i64);
                for i in 1..self.relitable.len() {
                    self.relitable[i].push(String::new());
                }
            }

            for a in rowsOfcombi.iter().copied() {
                let u = headingsAmount + a - 1;
                if !rowsAsNumbers.contains(&u) {
                    rowsAsNumbers.push(u);
                }
            }
        }

        (kombiTable, kombiTable_Kombis, maintable2subtable_Relation)
    }

    fn prepare_kombi_py(
        &self,
        paramLines: &[String],
        displayingZeilen: &[String],
        kombiTable_Kombis: &[Vec<i64>],
        kind: &str,
    ) -> BTreeMap<i64, BTreeSet<i64>> {
        let mut chosen: BTreeMap<i64, BTreeSet<i64>> = BTreeMap::new();
        let trigger = if kind == "kombi15" { "ka2" } else { "ka" };
        if !paramLines.iter().any(|p| p == trigger) {
            return chosen;
        }
        let displaying: BTreeSet<i64> = displayingZeilen
            .iter()
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();

        for (kombiLineNumber, kombiLine) in kombiTable_Kombis.iter().enumerate() {
            for kombiNumber in kombiLine {
                if displaying.contains(kombiNumber) {
                    chosen.entry(*kombiNumber).or_default().insert((kombiLineNumber + 1) as i64);
                }
            }
        }
        chosen
    }

    fn combo_certaintextwidth_py(&self, rowToDisplay: usize, combi_len: usize) -> usize {
        if self.shellRowsAmount == 0 {
            return 0;
        }
        let breiten: Vec<i64> = if self.rowsAsNumbers.len() >= combi_len {
            self.breiten
                .iter()
                .skip(self.rowsAsNumbers.len().saturating_sub(combi_len))
                .copied()
                .collect()
        } else {
            vec![]
        };
        let certain = if rowToDisplay >= 1 && rowToDisplay - 1 < breiten.len() {
            breiten[rowToDisplay - 1]
        } else {
            self.textWidth
        };
        if certain <= 0 {
            0
        } else {
            certain as usize
        }
    }

    fn prepare4out_kombi_table_py(
        &self,
        kombiTable: &Vec<Vec<String>>,
        rowsOfcombi: &[i64],
    ) -> Vec<Vec<String>> {
        let mut newerTable: Vec<Vec<String>> = vec![];
        if kombiTable.is_empty() {
            return newerTable;
        }
        let selected_cols = dedup_preserve_order_i64(rowsOfcombi.to_vec());

        for line in kombiTable.iter() {
            let mut new2Lines: Vec<String> = vec![];
            let mut rowToDisplay: usize = 0;
            for t in selected_cols.iter().copied() {
                let idx = t as usize;
                let cell = line.get(idx).cloned().unwrap_or_default();
                rowToDisplay += 1;
                let certaintextwidth = self.combo_certaintextwidth_py(rowToDisplay, selected_cols.len());
                let into = self.prepare_cell_work_py(&cell, certaintextwidth);
                new2Lines.push(into.join("\n"));
            }
            newerTable.push(new2Lines);
        }
        newerTable
    }

    fn removeOneNumber_lines_py(
        &self,
        cell_lines: &[String],
        colNum: i64,
        text_width: usize,
    ) -> Vec<String> {
        let mut text = cell_lines.join("");
        while text.ends_with('-') {
            text.pop();
        }

        let render = |value: String| -> Vec<String> {
            if text_width > 0 {
                value.split('\n').map(|s| s.to_string()).collect::<Vec<String>>()
            } else {
                vec![value.replace('\n', "; ")]
            }
        };

        let open_pos = match text.find('(') {
            Some(v) => v,
            None => return render(text),
        };

        // Python searches for the first ") " after the first "(" and then
        // rewrites only that leading number list.
        let close_rel = match text[open_pos..].find(") ") {
            Some(v) => v,
            None => return render(text),
        };
        let close_pos = open_pos + close_rel;
        let inside = &text[(open_pos + 1)..close_pos];

        let mut kept_top_level: Vec<String> = Vec::new();
        for element in inside.split('|') {
            // Python de-parenthesizes every top-level element that starts with
            // "(" via el[1:-1].  These number lists are ASCII, so byte slicing
            // is safe here and intentionally mirrors that behavior.
            let deparenthesized = if element.starts_with('(') && element.len() >= 2 {
                &element[1..(element.len() - 1)]
            } else {
                element
            };

            let fraction_parts: Vec<&str> = deparenthesized.split('/').collect();
            let mut kept_fraction_parts: Vec<&str> = Vec::new();

            for part in &fraction_parts {
                let remove = part
                    .trim()
                    .parse::<i64>()
                    .map(|value| colNum.abs() == value.abs() && fraction_parts.len() == 1)
                    .unwrap_or(false);
                if !remove {
                    kept_fraction_parts.push(*part);
                }
            }

            if !kept_fraction_parts.is_empty() {
                kept_top_level.push(kept_fraction_parts.join("/"));
            }
        }

        let rebuilt = if kept_top_level.is_empty() {
            // Python drops the complete "(...)" prefix but keeps the blank
            // after the closing parenthesis, yielding list items such as
            // " Mikroorganismen (1)" rather than "() Mikroorganismen (1)".
            text.get((close_pos + 1)..).unwrap_or("").to_string()
        } else {
            let mut value = String::new();
            value.push('(');
            value.push_str(&kept_top_level.join("|"));
            value.push_str(&text[close_pos..]);
            value
        };

        render(rebuilt)
    }

    fn combiTableWorkflow_impl(
        &mut self,
        preparedKombiTable: Vec<Vec<String>>,
        finallyDisplayLines: Vec<String>,
        kombiTable_Kombis: Vec<Vec<i64>>,
        maintable2subtable_Relation: (IndexMap<i64, i64>, IndexMap<i64, i64>),
        mut newTable: Vec<Vec<String>>,
        old2newTable: Vec<i64>,
        paramLines: Vec<String>,
        csvFileName: &str,
        output_column_origins: Vec<i64>,
        rowsOfcombi: &[i64],
    ) -> Vec<Vec<String>> {
        let kind = if csvFileName.contains("meta") { "kombi15" } else { "kombi13" };
        let chosen = self.prepare_kombi_py(&paramLines, &finallyDisplayLines, &kombiTable_Kombis, kind);
        if chosen.is_empty() {
            return newTable;
        }

        let mut output_to_preparedcol: BTreeMap<usize, usize> = BTreeMap::new();
        for (out_idx, orig_col) in output_column_origins.iter().copied().enumerate() {
            if let Some(sub_idx) = maintable2subtable_Relation.0.get(&orig_col) {
                let wanted_csv_col = *sub_idx + 1;
                if let Some(pos) = rowsOfcombi.iter().position(|v| *v == wanted_csv_col) {
                    output_to_preparedcol.insert(out_idx, pos);
                }
            }
        }
        if output_to_preparedcol.is_empty() {
            return newTable;
        }

        let oneLinePerLine = self.outType == "html" || self.outType == "bbcode";
        let remove_number_now =
            ((self.textWidth == 0 && self.oneTable) || self.outType == "html" || self.outType == "bbcode")
                && self.breiten.is_empty();

        for (display_row_idx, original_row_no) in old2newTable.iter().copied().enumerate() {
            if display_row_idx >= newTable.len() {
                continue;
            }
            let Some(kombi_line_numbers) = chosen.get(&original_row_no) else {
                continue;
            };

            for (out_col_idx, prepared_col_idx) in output_to_preparedcol.iter() {
                if *out_col_idx >= newTable[display_row_idx].len() {
                    continue;
                }

                let mut teile: Vec<String> = vec![];
                for kombi_line_no in kombi_line_numbers.iter().copied() {
                    let src_row_idx = kombi_line_no as usize;
                    if src_row_idx >= preparedKombiTable.len() {
                        continue;
                    }

                    let raw_prepared = preparedKombiTable[src_row_idx]
                        .get(*prepared_col_idx)
                        .cloned()
                        .unwrap_or_default();
                    let block = if remove_number_now {
                        let raw_lines: Vec<String> =
                            raw_prepared.split('\n').map(|s| s.to_string()).collect::<Vec<String>>();
                        self.removeOneNumber_lines_py(
                            &raw_lines,
                            original_row_no,
                            self.textWidth.max(0) as usize,
                        ).join("\n")
                    } else {
                        raw_prepared
                    };

                    teile.push(block);
                }

                if teile.is_empty() {
                    continue;
                }

                let merged = if oneLinePerLine {
                    if self.outType == "html" {
                        format!(
                            "<ul>{}</ul>",
                            teile
                                .into_iter()
                                .map(|t| format!("<li>{}</li>", t))
                                .collect::<Vec<_>>()
                                .join("")
                        )
                    } else if self.outType == "bbcode" {
                        format!(
                            "[list]{}[/list]",
                            teile
                                .into_iter()
                                .map(|t| format!("[*]{}", t))
                                .collect::<Vec<_>>()
                                .join("")
                        )
                    } else {
                        teile.join("\n")
                    }
                } else if self.textWidth == 0 && self.oneTable {
                    teile.join(" | ")
                } else {
                    teile.join("\n")
                };

                if newTable[display_row_idx][*out_col_idx].is_empty() {
                    newTable[display_row_idx][*out_col_idx] = merged;
                } else if !newTable[display_row_idx][*out_col_idx].contains(&merged) {
                    if oneLinePerLine || (self.textWidth == 0 && self.oneTable) {
                        newTable[display_row_idx][*out_col_idx].push_str(" | ");
                    } else {
                        newTable[display_row_idx][*out_col_idx].push('\n');
                    }
                    newTable[display_row_idx][*out_col_idx].push_str(&merged);
                }
            }
        }

        newTable
    }

    pub fn combiTableWorkflow(&mut self) {
        self.tableGenerated = self.tableGenerated || self.newTable;
    }

    /// Python `Program.__init__(..., runAlles=True)` runs the full workflow before
    /// `run()` is ever called and then protects `run()` from doing the same work
    /// again.  The Rust launcher calls this method first and then `run()`, so keep
    /// the Python ordering explicit instead of leaving the constructor behaviour
    /// split across unrelated calls.
    pub fn runAllesLikePythonInit(&mut self, words: &Words) {
        if self.__runAlles && self.allImportantBeginThingsDone {
            return;
        }
        self.__runAlles = true;
        self.__invertAlles = false;
        let resulting_table = self.workflowEverything(self.argv.clone(), words);
        if self.streamingMemoryMode {
            self.__resultingTable = Vec::new();
        } else {
            self.__resultingTable = resulting_table;
        }
    }

    /// Python `Program.run()` only performs the workflow when the constructor was
    /// created with `runAlles=False`.  It must not flip `__invertAlles`; that is a
    /// separate Python method and changing it after the workflow makes subsequent
    /// stateful calls diverge from the reference implementation.
    pub fn run(&mut self, words: &Words) {
        if !self.__runAlles {
            let resulting_table = self.workflowEverything(self.argv.clone(), words);
            if self.streamingMemoryMode {
                self.__resultingTable = Vec::new();
            } else {
                self.__resultingTable = resulting_table;
            }
        }
        self.printOrStoreLines();
        self.runDone = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    fn empty_words() -> Words {
        Words {
            paraNdataMatrix: vec![],
            kombiParaNdataMatrix: IndexMap::new(),
            kombiParaNdataMatrix2: IndexMap::new(),
        }
    }

    #[test]
    fn run_alles_like_python_init_runs_constructor_workflow_once() {
        let words = empty_words();
        let mut program = Program::new(vec!["reta".to_string(), "-h".to_string()]);

        program.runAllesLikePythonInit(&words);
        assert!(program.__runAlles);
        assert!(program.allImportantBeginThingsDone);
        assert!(!program.__invertAlles);
        let result_after_init = program.__resultingTable.clone();

        program.run(&words);
        assert_eq!(program.__resultingTable, result_after_init);
        assert!(program.runDone);
        assert!(!program.__invertAlles);
    }

    #[test]
    fn run_without_constructor_guard_does_not_force_invert_alles() {
        let words = empty_words();
        let mut program = Program::new(vec!["reta".to_string(), "-h".to_string()]);

        program.run(&words);
        assert!(!program.__runAlles);
        assert!(program.allImportantBeginThingsDone);
        assert!(program.runDone);
        assert!(!program.__invertAlles);
    }
}
