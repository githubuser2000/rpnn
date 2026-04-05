use std::fs;
use std::io;
use std::path::PathBuf;

use crate::shared::reta_py::Program;

#[derive(Clone, Debug)]
pub struct CsvFileNames {
    pub religion: String,
    pub kombi13: String,
    pub kombi15: String,
    pub prim: String,
    pub bruch15: String,
    pub bruch13: String,
    pub bruch7: String,
    pub bruchStrukGroesse: String,
    pub kombi_17_13_15: String,
    pub kombi_11_15: String,
    pub kombi_10_15: String,
    pub kreis18: String,
    pub sunMoon: String,
    pub meaningOfLife: String,
    pub dualitaetenTrinities: String,
}

impl CsvFileNames {
    pub fn new() -> Self {
        Self {
            religion: "religion.csv".to_string(),
            kombi13: "kombi.csv".to_string(),
            kombi15: "kombi-meta.csv".to_string(),
            prim: "primenumbers.csv".to_string(),
            bruch15: "gebrochen-rational-universum.csv".to_string(),
            bruch13: "gebrochen-rational-galaxie.csv".to_string(),
            bruch7: "gebrochen-rational-emotionen.csv".to_string(),
            bruchStrukGroesse: "gebrochen-rational-strukturgroesse.csv".to_string(),
            kombi_17_13_15: "kombi-gedanken17-absichten13-bewusstsein15.csv".to_string(),
            kombi_11_15: "kombi-meta-systeme.csv".to_string(),
            kombi_10_15: "kombi-universelle-wirklichkeit.csv".to_string(),
            kreis18: "kreisVomTyp18.csv".to_string(),
            sunMoon: "sunMoonEtc.csv".to_string(),
            meaningOfLife: "meaningOfLife.csv".to_string(),
            dualitaetenTrinities: "dualism-trinities-etc.csv".to_string(),
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
        for line in text.lines() {
            rows.push(line.split(';').map(|s| s.to_string()).collect());
        }
        Ok(rows)
    }

    pub fn load_religion_csv_exact(&mut self) -> io::Result<()> {
        let csvFileNames = self.csv_file_names();
        let rows = self.load_csv_rows_semicolon_exact_path(&csvFileNames.religion)?;
        self.relitable = rows;
        self.RowsLen = if self.relitable.len() > 0 { self.relitable[0].len() as i64 } else { 0 };
        Ok(())
    }
}
