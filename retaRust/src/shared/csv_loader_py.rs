use std::fs;
use std::io;
use std::path::PathBuf;

use serde_json::Value;

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
            let mut cols: Vec<String> = vec![];
            for cell in row.split(';') {
                let c = cell.to_string();
                cols.push(c);
            }
            rows.push(cols);
        }
        Ok(rows)
    }

    fn language_from_argv_py(&self) -> String {
        for arg in &self.argvWithoutProgram {
            if let Some(lang) = arg.strip_prefix("-sprache=") {
                return lang.to_string();
            }
        }
        String::new()
    }

    fn change_motives_file_py(&self) -> String {
        let lang = self.language_from_argv_py();
        match lang.as_str() {
            "kr" => "kr-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv".to_string(),
            "cn" => "cn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv".to_string(),
            "vn" => "vn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv".to_string(),
            _ => String::new(),
        }
    }

    fn escape_html_exact_py(&self, value: &str) -> String {
        value
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace("'", "&#x27;")
    }

    fn decode_cell_exact_py(&self, ccc: &str, mode: &str) -> String {
        if ccc.starts_with("|{") && ccc.ends_with("}|") {
            let json_text = &ccc[1..ccc.len()-1];
            if let Ok(val) = serde_json::from_str::<Value>(json_text) {
                if let Some(obj) = val.as_object() {
                    if let Some(found) = obj.get(mode).and_then(|v| v.as_str()) {
                        return found.to_string();
                    }
                    if mode.is_empty() {
                        if let Some(found) = obj.get("").and_then(|v| v.as_str()) {
                            return found.to_string();
                        }
                    }
                }
            }
        }
        if mode == "html" {
            self.escape_html_exact_py(ccc)
        } else {
            ccc.to_string()
        }
    }

    pub fn load_religion_csv_exact(&mut self) -> io::Result<()> {
        let csvFileNames = self.csv_file_names();
        let art_bbcode = self.argvWithoutProgram.iter().any(|a| a == "--art=bbcode");
        let art_html = self.argvWithoutProgram.iter().any(|a| a == "--art=html");
        let mode = if art_bbcode { "bbcode" } else if art_html { "html" } else { "" };

        let rows = self.load_csv_rows_semicolon_exact_path(&csvFileNames.religion)?;
        self.relitable = vec![];
        self.RowsLen = 0;
        for row in rows {
            let mut col: Vec<String> = vec![];
            for ccc in row {
                col.push(self.decode_cell_exact_py(&ccc, mode));
            }
            if self.RowsLen == 0 {
                self.RowsLen = col.len() as i64;
            }
            self.relitable.push(col);
        }
        if self.hoechsteZeile > 0 && !self.relitable.is_empty() {
            let target = (self.hoechsteZeile + 1) as usize;
            while self.relitable.len() < target {
                self.relitable.push(vec![String::new(); self.relitable[0].len()]);
            }
        }

        let change_motives_column = self.change_motives_file_py();
        if !change_motives_column.is_empty() && change_motives_column != "de" {
            let rows = self.load_csv_rows_semicolon_exact_path(&change_motives_column)?;
            for (i, col) in rows.into_iter().enumerate() {
                if let Some(first) = col.first() {
                    if let Some(existing_row) = self.relitable.get_mut(i) {
                        if existing_row.len() > 10 {
                            existing_row[10] = first.clone();
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
