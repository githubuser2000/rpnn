#![allow(non_snake_case)]

use crate::shared::reta_program_types::Program;

impl Program {
    pub fn resultingTable(&mut self) -> Vec<Vec<String>> {
        self.__resultingTable.clone()
    }

    pub fn onlyThatColumns_py(&self, table: Vec<Vec<String>>, onlyThatColumns: Vec<i64>) -> Vec<Vec<String>> {
        if onlyThatColumns.is_empty() {
            return table;
        }

        let mut newTable: Vec<Vec<String>> = vec![];

        for row in &table {
            let mut newCol: Vec<String> = vec![];

            for i in onlyThatColumns.iter() {
                if *i <= 0 {
                    continue;
                }
                let idx = (*i - 1) as usize;
                if idx < row.len() {
                    newCol.push(row[idx].clone());
                }
            }

            // Python hängt auch leere Zeilen an
            newTable.push(newCol);
        }

        if !newTable.is_empty() {
            newTable
        } else {
            table
        }
    }

    pub fn onlyThatColumns_i64_py(&self, values: Vec<i64>, onlyThatColumns: Vec<i64>) -> Vec<i64> {
        if onlyThatColumns.is_empty() {
            return values;
        }

        let mut out: Vec<i64> = vec![];
        for i in onlyThatColumns.iter() {
            if *i <= 0 {
                continue;
            }
            let idx = (*i - 1) as usize;
            if idx < values.len() {
                out.push(values[idx]);
            }
        }

        out
    }
}
