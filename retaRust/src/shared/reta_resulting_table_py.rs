#![allow(non_snake_case)]

use crate::shared::reta_program_types::Program;

impl Program {
    pub fn resultingTable(&mut self) -> Vec<Vec<String>> {
        self.__resultingTable.clone()
    }

pub fn onlyThatColumns_py(&self, table: Vec<Vec<String>>, onlyThatColumns: Vec<i64>) -> Vec<Vec<String>> {
    if onlyThatColumns.len() == 0 {
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

    if newTable.len() > 0 {
        newTable
    } else {
        table
    }
}
}
