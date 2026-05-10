#![allow(non_snake_case)]

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{OnceLock, RwLock};

use crate::shared::reta_py::Program;


static SHARED_CSV_ROOT: OnceLock<PathBuf> = OnceLock::new();
static PARSED_CSV_CACHE: OnceLock<RwLock<HashMap<PathBuf, Vec<Vec<String>>>>> = OnceLock::new();
static PROCESSED_RELIGION_CACHE: OnceLock<RwLock<HashMap<(String, String), Vec<Vec<String>>>>> = OnceLock::new();

fn shared_csv_root() -> &'static PathBuf {
    SHARED_CSV_ROOT.get_or_init(resolve_csv_root)
}

fn resolve_csv_root() -> PathBuf {
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(path) = std::env::var("RETA_CSV_PATH") {
        let path = path.trim();
        if !path.is_empty() {
            candidates.push(PathBuf::from(path));
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("csv"));
            candidates.push(dir.join("..").join("csv"));
            candidates.push(dir.join("share").join("reta").join("csv"));
            candidates.push(dir.join("..").join("share").join("reta").join("csv"));
            candidates.push(dir.join("..").join("..").join("csv"));
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd.join("csv"));
    }

    candidates.push(Path::new(env!("CARGO_MANIFEST_DIR")).join("csv"));

    for candidate in dedup_csv_root_candidates(candidates) {
        if candidate.join("religion.csv").is_file() {
            return candidate;
        }
    }

    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("csv")
}

fn dedup_csv_root_candidates(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut deduped = Vec::new();
    for path in paths {
        if !deduped.iter().any(|existing| existing == &path) {
            deduped.push(path);
        }
    }
    deduped
}

fn parsed_csv_cache() -> &'static RwLock<HashMap<PathBuf, Vec<Vec<String>>>> {
    PARSED_CSV_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn processed_religion_cache() -> &'static RwLock<HashMap<(String, String), Vec<Vec<String>>>> {
    PROCESSED_RELIGION_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn preload_common_csv_tables() -> io::Result<()> {
    let program = Program::new(vec!["reta".to_string()]);
    let csv_file_names = CsvFileNames::new();
    let mut names = vec![csv_file_names.religion, csv_file_names.kombi13, csv_file_names.kombi15];
    let change_candidates = [
        "kr-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
        "cn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
        "vn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv",
    ];
    names.extend(change_candidates.into_iter().map(str::to_string));

    for name in names {
        let _ = program.load_csv_rows_semicolon_exact_path(&name)?;
    }
    Ok(())
}

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
        shared_csv_root().clone()
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

    fn parse_semicolon_csv_python_like(&self, text: &str) -> Vec<Vec<String>> {
        let mut rows: Vec<Vec<String>> = vec![];
        let mut row: Vec<String> = vec![];
        let mut cell = String::new();
        let mut chars = text.chars().peekable();
        let mut in_quotes = false;

        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    if in_quotes {
                        if matches!(chars.peek(), Some('"')) {
                            cell.push('"');
                            chars.next();
                        } else {
                            in_quotes = false;
                        }
                    } else {
                        in_quotes = true;
                    }
                }
                ';' if !in_quotes => {
                    row.push(std::mem::take(&mut cell));
                }
                '\n' if !in_quotes => {
                    row.push(std::mem::take(&mut cell));
                    rows.push(std::mem::take(&mut row));
                }
                '\r' if !in_quotes => {
                    if matches!(chars.peek(), Some('\n')) {
                        chars.next();
                    }
                    row.push(std::mem::take(&mut cell));
                    rows.push(std::mem::take(&mut row));
                }
                _ => cell.push(ch),
            }
        }

        if !cell.is_empty() || !row.is_empty() || text.ends_with(';') {
            row.push(cell);
        }
        if !row.is_empty() {
            rows.push(row);
        }
        rows
    }

    pub fn load_csv_rows_semicolon_exact_path(&self, csvFileName: &str) -> io::Result<Vec<Vec<String>>> {
        let path = self.csv_path(csvFileName);

        if let Ok(cache) = parsed_csv_cache().read() {
            if let Some(rows) = cache.get(&path) {
                return Ok(rows.clone());
            }
        }

        let text = fs::read_to_string(&path)?;
        let rows = self.parse_semicolon_csv_python_like(&text);

        if let Ok(mut cache) = parsed_csv_cache().write() {
            cache.entry(path).or_insert_with(|| rows.clone());
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

    fn extract_mode_value_from_python_jsonish_cell(&self, ccc: &str, mode: &str) -> Option<String> {
        if !(ccc.starts_with("|{") && ccc.ends_with("}|")) {
            return None;
        }
        let json_text = &ccc[2..ccc.len() - 2];
        let lookup = if mode.is_empty() { "\"\"".to_string() } else { format!("\"{}\"", mode) };
        let needle = format!("{}:", lookup);
        let pos = json_text.find(&needle)?;
        let rest = &json_text[pos + needle.len()..];
        let first_quote = rest.find('"')?;
        let rest = &rest[first_quote + 1..];
        let mut out = String::new();
        let mut escaped = false;
        for ch in rest.chars() {
            if escaped {
                out.push(ch);
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                return Some(out);
            } else {
                out.push(ch);
            }
        }
        None
    }

    fn decode_cell_exact_py(&self, ccc: &str, mode: &str) -> String {
        if let Some(found) = self.extract_mode_value_from_python_jsonish_cell(ccc, mode) {
            return found;
        }
        if mode.is_empty() {
            if let Some(found) = self.extract_mode_value_from_python_jsonish_cell(ccc, "") {
                return found;
            }
        }
        if mode == "html" {
            self.escape_html_exact_py(ccc)
        } else {
            ccc.to_string()
        }
    }

    fn build_processed_religion_table_exact(&self, mode: &str, change_motives_column: &str) -> io::Result<Vec<Vec<String>>> {
        let csvFileNames = self.csv_file_names();
        let rows = self.load_csv_rows_semicolon_exact_path(&csvFileNames.religion)?;
        let mut relitable: Vec<Vec<String>> = Vec::with_capacity(rows.len());
        for row in rows {
            let mut col: Vec<String> = Vec::with_capacity(row.len());
            for ccc in row {
                col.push(self.decode_cell_exact_py(&ccc, mode));
            }
            relitable.push(col);
        }
        if !change_motives_column.is_empty() {
            let rows = self.load_csv_rows_semicolon_exact_path(change_motives_column)?;
            for (i, col) in rows.into_iter().enumerate() {
                if let Some(first) = col.first() {
                    if let Some(existing_row) = relitable.get_mut(i) {
                        if existing_row.len() > 10 {
                            existing_row[10] = first.clone();
                        }
                    }
                }
            }
        }
        Ok(relitable)
    }

    fn processed_religion_table_exact(&self, mode: &str, change_motives_column: &str) -> io::Result<Vec<Vec<String>>> {
        let key = (mode.to_string(), change_motives_column.to_string());
        if let Ok(cache) = processed_religion_cache().read() {
            if let Some(rows) = cache.get(&key) {
                return Ok(rows.clone());
            }
        }

        let relitable = self.build_processed_religion_table_exact(mode, change_motives_column)?;

        if let Ok(mut cache) = processed_religion_cache().write() {
            cache.entry(key).or_insert_with(|| relitable.clone());
        }

        Ok(relitable)
    }

    pub fn load_religion_csv_exact(&mut self) -> io::Result<()> {
        let art_bbcode = self.argvWithoutProgram.iter().any(|a| a == "--art=bbcode");
        let art_html = self.argvWithoutProgram.iter().any(|a| a == "--art=html");
        let mode = if art_bbcode { "bbcode" } else if art_html { "html" } else { "" };
        let change_motives_column = self.change_motives_file_py();

        self.relitable = self.processed_religion_table_exact(mode, &change_motives_column)?;
        self.RowsLen = self.relitable.first().map(|row| row.len() as i64).unwrap_or(0);

        if self.hoechsteZeile > 0 && !self.relitable.is_empty() {
            let target = (self.hoechsteZeile + 1) as usize;
            while self.relitable.len() < target {
                self.relitable.push(vec![String::new(); self.relitable[0].len()]);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod csv_source_guard_tests {
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::Path;

    const RETA_CSV_SOURCE_MANIFEST: &str = include_str!("../../tools/reta_csv_source_manifest.tsv");

    #[derive(Clone, Debug)]
    struct CsvManifestEntry {
        name: &'static str,
        bytes: usize,
        fnv1a64: u64,
    }

    fn fnv1a64(data: &[u8]) -> u64 {
        let mut value = 0xcbf29ce484222325u64;
        for byte in data {
            value ^= u64::from(*byte);
            value = value.wrapping_mul(0x100000001b3);
        }
        value
    }

    fn parse_manifest() -> Vec<CsvManifestEntry> {
        RETA_CSV_SOURCE_MANIFEST
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with("file\t")
            })
            .map(|line| {
                let mut parts = line.split('\t');
                let name = parts.next().expect("manifest file column missing");
                let bytes = parts
                    .next()
                    .expect("manifest bytes column missing")
                    .parse::<usize>()
                    .expect("manifest bytes must be usize");
                let fnv1a64 = u64::from_str_radix(
                    parts.next().expect("manifest fnv1a64 column missing"),
                    16,
                )
                .expect("manifest fnv1a64 must be hexadecimal u64");
                CsvManifestEntry { name, bytes, fnv1a64 }
            })
            .collect()
    }

    fn assert_csv_dir_matches_reta_manifest(relative_dir: &str, manifest: &[CsvManifestEntry]) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let csv_dir = root.join(relative_dir);
        let expected_names = manifest
            .iter()
            .map(|entry| entry.name.to_string())
            .collect::<BTreeSet<_>>();
        let actual_names = fs::read_dir(&csv_dir)
            .unwrap_or_else(|err| panic!("cannot read {csv_dir:?}: {err}"))
            .filter_map(|entry| {
                let entry = entry.expect("csv directory entry must be readable");
                let path = entry.path();
                (path.extension().and_then(|ext| ext.to_str()) == Some("csv"))
                    .then(|| entry.file_name().to_string_lossy().into_owned())
            })
            .collect::<BTreeSet<_>>();

        assert_eq!(
            actual_names, expected_names,
            "{relative_dir} must contain exactly the reta.py source CSV files"
        );

        for entry in manifest {
            let path = csv_dir.join(entry.name);
            let data = fs::read(&path).unwrap_or_else(|err| panic!("cannot read {path:?}: {err}"));
            assert_eq!(
                data.len(),
                entry.bytes,
                "{path:?} byte length drifted away from reta.py CSV source"
            );
            assert_eq!(
                fnv1a64(&data),
                entry.fnv1a64,
                "{path:?} content drifted away from reta.py CSV source"
            );
        }
    }

    #[test]
    fn checked_in_csv_files_match_reta_py_source_manifest() {
        let manifest = parse_manifest();
        assert_eq!(manifest.len(), 79, "reta.py source CSV inventory changed");
        assert_csv_dir_matches_reta_manifest("csv", &manifest);
        assert_csv_dir_matches_reta_manifest("python_reference/csv", &manifest);
    }
}
