use std::fs;
use std::io;
use std::path::PathBuf;

use crate::shared::reta_py::Program;

#[derive(Clone, Debug)]
pub struct CsvFileNames {
    pub religion: String,
    pub kombi13: String,
    pub kombi15: String,
}

impl CsvFileNames {
    pub fn new() -> Self {
        Self {
            religion: "religion.csv".to_string(),
            kombi13: "kombi.csv".to_string(),
            kombi15: "kombi-meta.csv".to_string(),
        }
    }
}

impl Program {
    pub fn csv_root(&self) -> PathBuf {
        let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        path.push("csv");
        path
    }

    pub fn csv_path(&self, csvFileName: &str) -> PathBuf {
        let mut path = self.csv_root();
        path.push(csvFileName);
        path
    }

    pub fn csv_file_names(&self) -> CsvFileNames {
        CsvFileNames::new()
    }

    pub fn load_csv_text_exact_path(&self, csvFileName: &str) -> io::Result<String> {
        fs::read_to_string(self.csv_path(csvFileName))
    }

    pub fn load_csv_rows_semicolon_exact_path(&self, csvFileName: &str) -> io::Result<Vec<Vec<String>>> {
        let text = self.load_csv_text_exact_path(csvFileName)?;
        let mut rows: Vec<Vec<String>> = vec![];
        for row in text.lines() {
            rows.push(row.split(';').map(|s| s.to_string()).collect());
        }
        Ok(rows)
    }

    pub fn load_religion_csv_exact(&mut self) -> io::Result<()> {
        let csvFileNames = self.csv_file_names();
        let rows = self.load_csv_rows_semicolon_exact_path(&csvFileNames.religion)?;
        self.relitable = rows.clone();
        self.RowsLen = rows.len() as i64;
        Ok(())
    }
}
