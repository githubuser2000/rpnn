#![allow(non_snake_case)]

use indexmap::IndexMap;
use std::collections::{BTreeMap, BTreeSet};

use crate::shared::reta_program_types::{dedup_preserve_order_i64, Program};
use crate::shared::words_py::Words;

impl Program {
    pub fn workflowEverything(&mut self, argv: Vec<String>, words: &Words) -> Vec<Vec<String>> {
        let (RowsLen, paramLines, paramLinesNot, relitable, rowsAsNumbers) =
            self.bringAllImportantBeginThings(argv, words);

        self.RowsLen = RowsLen;
        self.relitable = relitable.clone();
        self.rowsAsNumbers = rowsAsNumbers.clone();

        let (finallyDisplayLinesEarly, _newTableEarly, _numlenEarly, _rowsRangeEarly, _old2newTableEarly) = self.prepare4out_py(
            paramLines.clone(),
            paramLinesNot.clone(),
            self.relitable.clone(),
            self.rowsAsNumbers.clone(),
        );
        let mut zeilenliste: Vec<i64> = finallyDisplayLinesEarly
            .iter()
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        zeilenliste.sort_unstable();
        self.lastLineNumber = zeilenliste.last().copied().unwrap_or(0);

        self.apply_concat_generators_py();

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

        let (finallyDisplayLines, mut newTable, numlen, rowsRange, old2newTable): (Vec<String>, Vec<Vec<String>>, i64, Vec<i64>, Vec<i64>) = self.prepare4out_py(
            paramLines.clone(),
            paramLinesNot.clone(),
            self.relitable.clone(),
            self.rowsAsNumbers.clone(),
        );

        if !kombi13_rows.is_empty() {
            newTable = self.combiTableWorkflow_impl(
                animalsProfessionsTable,
                finallyDisplayLines.clone(),
                kombiTable_Kombis,
                maintable2subtable_Relation,
                newTable,
                old2newTable.clone(),
                paramLines.clone(),
                &csv_names.kombi13,
                output_column_origins.clone(),
            );
        }
        if !kombi15_rows.is_empty() {
            newTable = self.combiTableWorkflow_impl(
                animalsProfessionsTable2,
                finallyDisplayLines.clone(),
                kombiTable_Kombis2,
                maintable2subtable_Relation2,
                newTable,
                old2newTable.clone(),
                paramLines.clone(),
                &csv_names.kombi15,
                output_column_origins.clone(),
            );
        }

        newTable = self.onlyThatColumns_py(newTable, self.spaltenreihenfolgeundnurdiese.clone());
        self.newTable = newTable.len() > 0;
        self.finallyDisplayLines = finallyDisplayLines.clone();
        self.numlen = numlen;
        let _old2newTable = old2newTable.clone();

        let out: Vec<Vec<String>> = self.cliOut_py(finallyDisplayLines, newTable.clone(), numlen, rowsRange);
        self.tableGenerated = self.newTable || !out.is_empty();
        self.__resultingTable = out.clone();
        self.addResultingTableToTables();
        self.setOld2Rows();
        self.setNewerTable();
        self.setOldRows();
        self.setNewerRows();
        self.setRowsOfcombi();
        self.setOldTable();
        self.setGeneratedSpaltenParameter();
        self.setAllEquColumns();
        self.setFinallyDisplayTable();
        out
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

    fn remove_one_number_simple_py(&self, text: &str, colNum: i64) -> String {
        let target_a = format!("({}) ", colNum);
        let target_b = format!("|{}", colNum);
        let target_c = format!("{}|", colNum);
        let target_d = format!("/{}/", colNum);
        text.replace(&target_a, "(")
            .replace(&target_b, "")
            .replace(&target_c, "")
            .replace(&target_d, "/")
    }

    fn combiTableWorkflow_impl(
        &mut self,
        animalsProfessionsTable: Vec<Vec<String>>,
        finallyDisplayLines: Vec<String>,
        kombiTable_Kombis: Vec<Vec<i64>>,
        maintable2subtable_Relation: (IndexMap<i64, i64>, IndexMap<i64, i64>),
        mut newTable: Vec<Vec<String>>,
        old2newTable: Vec<i64>,
        paramLines: Vec<String>,
        csvFileName: &str,
        output_column_origins: Vec<i64>,
    ) -> Vec<Vec<String>> {
        let kind = if csvFileName.contains("meta") { "kombi15" } else { "kombi13" };
        let chosen = self.prepare_kombi_py(&paramLines, &finallyDisplayLines, &kombiTable_Kombis, kind);
        if chosen.is_empty() {
            return newTable;
        }

        let mut output_to_subcol: BTreeMap<usize, usize> = BTreeMap::new();
        for (out_idx, orig_col) in output_column_origins.iter().copied().enumerate() {
            if let Some(sub_idx) = maintable2subtable_Relation.0.get(&orig_col) {
                output_to_subcol.insert(out_idx, (*sub_idx as usize) + 1);
            }
        }
        if output_to_subcol.is_empty() {
            return newTable;
        }

        let oneLinePerLine = self.outType == "html" || self.outType == "bbcode";

        for (display_row_idx, original_row_no) in old2newTable.iter().copied().enumerate() {
            if display_row_idx >= newTable.len() {
                continue;
            }
            let Some(kombi_line_numbers) = chosen.get(&original_row_no) else {
                continue;
            };
            for (out_col_idx, csv_col_idx) in output_to_subcol.iter() {
                if *out_col_idx >= newTable[display_row_idx].len() {
                    continue;
                }
                let mut teile: Vec<String> = vec![];
                for kombi_line_no in kombi_line_numbers.iter().copied() {
                    let src_row_idx = kombi_line_no as usize;
                    if src_row_idx >= animalsProfessionsTable.len() {
                        continue;
                    }
                    let raw = animalsProfessionsTable[src_row_idx].get(*csv_col_idx).cloned().unwrap_or_default();
                    if raw.trim().is_empty() {
                        continue;
                    }
                    let cleaned = self.remove_one_number_simple_py(&raw, original_row_no);
                    if !cleaned.trim().is_empty() {
                        teile.push(cleaned);
                    }
                }
                if teile.is_empty() {
                    continue;
                }
                let merged = if oneLinePerLine {
                    if self.outType == "html" {
                        format!("<ul>{}</ul>", teile.into_iter().map(|t| format!("<li>{}</li>", t)).collect::<Vec<_>>().join(""))
                    } else if self.outType == "bbcode" {
                        format!("[list]{}[/list]", teile.into_iter().map(|t| format!("[*]{}", t)).collect::<Vec<_>>().join(""))
                    } else {
                        teile.join(" | ")
                    }
                } else {
                    teile.join(" | ")
                };
                if newTable[display_row_idx][*out_col_idx].is_empty() {
                    newTable[display_row_idx][*out_col_idx] = merged;
                } else if !newTable[display_row_idx][*out_col_idx].contains(&merged) {
                    newTable[display_row_idx][*out_col_idx].push_str(" | ");
                    newTable[display_row_idx][*out_col_idx].push_str(&merged);
                }
            }
        }

        newTable
    }

    pub fn combiTableWorkflow(&mut self) {
        self.tableGenerated = self.tableGenerated || self.newTable;
    }

    pub fn run(&mut self, words: &Words) {
        if !self.__runAlles {
            self.__resultingTable = self.workflowEverything(self.argv.clone(), words);
        }
        self.invertAlles();
        self.printOrStoreLines();
        self.runDone = true;
    }

    pub fn runAllesLikePythonInit(&mut self, _words: &Words) {
    }
}
