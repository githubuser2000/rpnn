//! Repository manifest and integrity morphisms transcompiled from
//! `python_arch_reference/reta_architecture/package_integrity.py`.
//!
//! This module is intentionally std-only.  It walks a repository tree,
//! excludes runtime artifacts, records CSV line counts and computes a stable
//! manifest digest from file paths and contents.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::persistence::stable_digest_text;

pub const IGNORED_DIR_NAMES: &[&str] = &["__pycache__", ".git", ".pytest_cache", ".mypy_cache"];
pub const IGNORED_SUFFIXES: &[&str] = &[".pyc", ".pyo"];
pub const REQUIRED_SOURCE_PATHS: &[&str] = &[
    "reta.py",
    "retaPrompt.py",
    "reta_architecture/__init__.py",
    "reta_architecture/facade.py",
    "reta_architecture/topology.py",
    "reta_architecture/presheaves.py",
    "reta_architecture/sheaves.py",
    "reta_architecture/morphisms.py",
    "reta_architecture/universal.py",
    "reta_architecture/category_theory.py",
    "reta_architecture/architecture_map.py",
    "reta_architecture/parallel_execution.py",
    "reta_architecture/persistence.py",
    "reta_architecture/schema.py",
    "reta_architecture/tag_schema.py",
    "reta_architecture/semantics_builder.py",
    "reta_architecture/input_semantics.py",
    "reta_architecture/row_ranges.py",
    "reta_architecture/arithmetic.py",
    "reta_architecture/console_io.py",
    "reta_architecture/completion_word.py",
    "reta_architecture/completion_nested.py",
    "reta_architecture/column_selection.py",
    "reta_architecture/parameter_runtime.py",
    "reta_architecture/program_workflow.py",
    "reta_architecture/table_output.py",
    "reta_architecture/prompt_execution.py",
    "i18n/words.py",
    "i18n/words_context.py",
    "i18n/words_matrix.py",
    "i18n/words_runtime.py",
    "csv/religion.csv",
    "csv/vn-religion.csv",
];

pub fn normalise_path(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .to_string_lossy()
        .replace('\\', "/")
        .trim_start_matches("./")
        .to_string()
}

pub fn is_runtime_artifact(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    if path.components().any(|component| {
        let part = component.as_os_str().to_string_lossy();
        IGNORED_DIR_NAMES.contains(&part.as_ref())
    }) {
        return true;
    }
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| IGNORED_SUFFIXES.contains(&format!(".{ext}").as_str()))
        .unwrap_or(false)
}

fn iter_regular_files(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    let mut entries = entries.filter_map(Result::ok).collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            iter_regular_files(&path, out);
        } else if path.is_file() {
            out.push(path);
        }
    }
}

pub fn iter_manifest_files(root: impl AsRef<Path>) -> Vec<PathBuf> {
    let root = root.as_ref();
    let mut files = Vec::new();
    iter_regular_files(root, &mut files);
    files
        .into_iter()
        .filter(|path| {
            path.strip_prefix(root)
                .map(|relative| !is_runtime_artifact(relative))
                .unwrap_or(true)
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RepoManifest {
    pub root: String,
    pub file_count: usize,
    pub total_bytes: usize,
    pub digest: String,
    pub files: Vec<String>,
    pub missing_required: Vec<String>,
    pub runtime_artifact_count: usize,
    pub csv_line_counts: BTreeMap<String, usize>,
    pub suspicious_csvs: Vec<String>,
}

impl RepoManifest {
    pub fn from_tree(
        root: impl AsRef<Path>,
        required_paths: Option<&[&str]>,
    ) -> std::io::Result<Self> {
        let root = root
            .as_ref()
            .canonicalize()
            .unwrap_or_else(|_| root.as_ref().to_path_buf());
        let mut all_regular = Vec::new();
        iter_regular_files(&root, &mut all_regular);
        let mut runtime_artifact_count = 0usize;
        let mut files = Vec::new();
        for path in &all_regular {
            let relative = path.strip_prefix(&root).unwrap_or(path);
            if is_runtime_artifact(relative) {
                runtime_artifact_count += 1;
            } else {
                files.push(normalise_path(relative));
            }
        }
        files.sort();

        let mut total_bytes = 0usize;
        let mut digest_text = String::new();
        let mut csv_line_counts = BTreeMap::new();
        for relative in &files {
            let path = root.join(relative);
            let data = fs::read(&path)?;
            total_bytes += data.len();
            let file_digest = stable_digest_text(&String::from_utf8_lossy(&data));
            digest_text.push_str(relative);
            digest_text.push('\0');
            digest_text.push_str(&file_digest);
            if relative.starts_with("csv/") && relative.ends_with(".csv") {
                csv_line_counts.insert(
                    relative.clone(),
                    String::from_utf8_lossy(&data).lines().count(),
                );
            }
        }
        let file_set = files
            .iter()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        let required = required_paths.unwrap_or(REQUIRED_SOURCE_PATHS);
        let missing_required = required
            .iter()
            .map(|path| normalise_path(path))
            .filter(|path| !file_set.contains(path))
            .collect::<Vec<_>>();
        let suspicious_csvs = csv_line_counts
            .iter()
            .filter(|(relative, count)| relative.ends_with("religion.csv") && **count < 500)
            .map(|(relative, _)| relative.clone())
            .collect::<Vec<_>>();

        Ok(Self {
            root: normalise_path(&root),
            file_count: files.len(),
            total_bytes,
            digest: stable_digest_text(&digest_text),
            files,
            missing_required,
            runtime_artifact_count,
            csv_line_counts,
            suspicious_csvs,
        })
    }

    pub fn snapshot(&self, include_files: bool) -> RepoManifestSnapshot {
        RepoManifestSnapshot {
            root: self.root.clone(),
            file_count: self.file_count,
            total_bytes: self.total_bytes,
            digest: self.digest.clone(),
            missing_required: self.missing_required.clone(),
            runtime_artifact_count: self.runtime_artifact_count,
            suspicious_csvs: self.suspicious_csvs.clone(),
            csv_line_counts: self.csv_line_counts.clone(),
            files: include_files.then(|| self.files.clone()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RepoManifestSnapshot {
    pub root: String,
    pub file_count: usize,
    pub total_bytes: usize,
    pub digest: String,
    pub missing_required: Vec<String>,
    pub runtime_artifact_count: usize,
    pub suspicious_csvs: Vec<String>,
    pub csv_line_counts: BTreeMap<String, usize>,
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackageIntegrityBundle {
    pub required_source_paths: Vec<String>,
}

impl PackageIntegrityBundle {
    pub fn manifest_from_tree(&self, root: impl AsRef<Path>) -> std::io::Result<RepoManifest> {
        let required = self
            .required_source_paths
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        RepoManifest::from_tree(root, Some(&required))
    }

    pub fn snapshot(&self) -> PackageIntegritySnapshot {
        PackageIntegritySnapshot {
            class: "PackageIntegrityBundle".to_string(),
            required_source_paths_len: self.required_source_paths.len(),
            ignored_dir_names: IGNORED_DIR_NAMES.iter().map(|v| v.to_string()).collect(),
            ignored_suffixes: IGNORED_SUFFIXES.iter().map(|v| v.to_string()).collect(),
            morphisms: vec![
                "is_runtime_artifact".to_string(),
                "iter_manifest_files".to_string(),
                "RepoManifest.from_tree".to_string(),
            ],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackageIntegritySnapshot {
    pub class: String,
    pub required_source_paths_len: usize,
    pub ignored_dir_names: Vec<String>,
    pub ignored_suffixes: Vec<String>,
    pub morphisms: Vec<String>,
}

pub fn bootstrap_package_integrity() -> PackageIntegrityBundle {
    PackageIntegrityBundle {
        required_source_paths: REQUIRED_SOURCE_PATHS
            .iter()
            .map(|value| value.to_string())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_artifact_suffix_is_filtered() {
        assert!(is_runtime_artifact("a/__pycache__/x.pyc"));
        assert!(!is_runtime_artifact("reta.py"));
    }
}

// Stage 16 continued: concrete package_integrity.py compatibility wrappers.
pub fn _normalise_path(path: impl AsRef<Path>) -> String { normalise_path(path) }
pub fn _iter_all_regular_files(root: impl AsRef<Path>) -> Vec<PathBuf> { iter_manifest_files(root) }
pub fn _manifest_file_entry(root: impl AsRef<Path>, path: impl AsRef<Path>) -> String { normalise_path(path.as_ref().strip_prefix(root.as_ref()).unwrap_or(path.as_ref())) }
pub fn _manifest_file_worker(path: impl AsRef<Path>) -> String { normalise_path(path) }
pub fn _manifest_entries_parallel(root: impl AsRef<Path>) -> Vec<String> { iter_manifest_files(root).into_iter().map(normalise_path).collect() }

// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "_iter_all_regular_files",
    "_manifest_entries_parallel",
    "_manifest_file_entry",
    "_manifest_file_worker",
    "_normalise_path",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
