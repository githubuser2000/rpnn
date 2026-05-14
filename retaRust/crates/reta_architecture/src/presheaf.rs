use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::topology::ContextSelection;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalSection {
    pub context: ContextSelection,
    pub payload: BTreeMap<String, String>,
    pub source: String,
}

impl LocalSection {
    pub fn new(
        context: ContextSelection,
        payload: BTreeMap<String, String>,
        source: impl Into<String>,
    ) -> Self {
        Self {
            context,
            payload,
            source: source.into(),
        }
    }

    pub fn restrict(&self, context: &ContextSelection) -> Self {
        Self {
            context: self.context.refine(context),
            payload: self.payload.clone(),
            source: self.source.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Presheaf {
    pub name: String,
    pub sections: Vec<LocalSection>,
}

impl Presheaf {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sections: Vec::new(),
        }
    }

    pub fn add_section(
        &mut self,
        context: ContextSelection,
        payload: BTreeMap<String, String>,
        source: impl Into<String>,
    ) {
        self.sections.push(LocalSection::new(context, payload, source));
    }

    pub fn sections_over(&self, context: &ContextSelection) -> Vec<LocalSection> {
        self.sections
            .iter()
            .map(|section| section.restrict(context))
            .filter(|section| !section.context.is_empty())
            .collect()
    }

    pub fn update_prompt_state(&mut self, raw_text: &str, tokens: &[String]) {
        let mut payload = BTreeMap::new();
        payload.insert("raw_text".to_string(), raw_text.to_string());
        payload.insert("tokens".to_string(), tokens.join("\u{1f}"));
        self.sections.clear();
        self.add_section(ContextSelection::prompt(), payload, "prompt");
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PresheafBundle {
    pub csv: Presheaf,
    pub translations: Presheaf,
    pub assets: Presheaf,
    pub prompt_state: Presheaf,
}

impl Default for PresheafBundle {
    fn default() -> Self {
        Self {
            csv: Presheaf::new("csv"),
            translations: Presheaf::new("translations"),
            assets: Presheaf::new("assets"),
            prompt_state: Presheaf::new("prompt_state"),
        }
    }
}


impl LocalSection {
    pub fn snapshot(&self) -> LocalSectionSnapshot {
        LocalSectionSnapshot {
            context: self.context.as_dict().into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
            payload_keys: self.payload.keys().cloned().collect(),
            source: self.source.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalSectionSnapshot {
    pub context: BTreeMap<String, Option<Vec<String>>>,
    pub payload_keys: Vec<String>,
    pub source: String,
}

impl Presheaf {
    pub fn snapshot(&self) -> PresheafSnapshot {
        PresheafSnapshot {
            name: self.name.clone(),
            section_count: self.sections.len(),
            sources: self.sections.iter().map(|section| section.source.clone()).collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PresheafSnapshot {
    pub name: String,
    pub section_count: usize,
    pub sources: Vec<String>,
}



#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FilesystemPresheaf {
    pub inner: Presheaf,
    pub root: PathBuf,
}

impl FilesystemPresheaf {
    pub fn new(name: impl Into<String>, root: impl Into<PathBuf>) -> Self {
        Self {
            inner: Presheaf::new(name),
            root: root.into(),
        }
    }

    pub fn discover<F>(&mut self, paths: impl IntoIterator<Item = PathBuf>, context_builder: F)
    where
        F: Fn(&Path) -> ContextSelection,
    {
        for path in paths {
            let context = context_builder(&path);
            let mut payload = BTreeMap::new();
            let rel = path.strip_prefix(&self.root).unwrap_or(path.as_path());
            payload.insert("path".to_string(), rel.to_string_lossy().to_string());
            payload.insert(
                "suffix".to_string(),
                path.extension()
                    .map(|value| format!(".{}", value.to_string_lossy()))
                    .unwrap_or_default(),
            );
            payload.insert(
                "name".to_string(),
                path.file_name()
                    .map(|value| value.to_string_lossy().to_string())
                    .unwrap_or_default(),
            );
            self.inner.add_section(context, payload, path.to_string_lossy().to_string());
        }
    }

    pub fn snapshot(&self) -> PresheafSnapshot {
        self.inner.snapshot()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptStatePresheaf {
    pub inner: Presheaf,
    pub raw_text: String,
    pub tokenized_text: Vec<String>,
}

impl Default for PromptStatePresheaf {
    fn default() -> Self {
        Self {
            inner: Presheaf::new("prompt_state"),
            raw_text: String::new(),
            tokenized_text: Vec::new(),
        }
    }
}

impl PromptStatePresheaf {
    pub fn update(&mut self, raw_text: &str, tokens: &[String], context: Option<ContextSelection>) {
        self.raw_text = raw_text.to_string();
        self.tokenized_text = tokens.to_vec();
        self.inner.sections.clear();
        let mut payload = BTreeMap::new();
        payload.insert("raw_text".to_string(), self.raw_text.clone());
        payload.insert("tokens".to_string(), self.tokenized_text.join("\u{1f}"));
        self.inner
            .add_section(context.unwrap_or_else(ContextSelection::prompt), payload, "prompt");
    }

    pub fn snapshot(&self) -> PresheafSnapshot {
        self.inner.snapshot()
    }
}

fn collect_regular_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(root) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(collect_regular_files(&path));
        } else if path.is_file() {
            out.push(path);
        }
    }
    out.sort();
    out
}

fn file_extension(path: &Path) -> String {
    path.extension()
        .map(|value| value.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default()
}

impl PresheafBundle {
    pub fn csv_context(path: &Path) -> ContextSelection {
        let stem = path.file_stem().map(|value| value.to_string_lossy().to_string()).unwrap_or_default();
        let language = stem.split_once('-').and_then(|(maybe, _)| {
            if (2..=3).contains(&maybe.len()) { Some(maybe.to_string()) } else { None }
        });
        let mut selection = ContextSelection::empty().restrict_dimension("scopes", ["csv".to_string()]);
        if let Some(language) = language {
            selection = selection.restrict_dimension("language", [language]);
        }
        selection
    }

    pub fn translation_context(path: &Path) -> ContextSelection {
        let mut language = None;
        let parts = path.iter().map(|part| part.to_string_lossy().to_string()).collect::<Vec<_>>();
        if let Some(pos) = parts.iter().position(|part| part == "i18n") {
            if let Some(maybe) = parts.get(pos + 1) {
                if ["de", "en", "vn", "cn", "kr"].contains(&maybe.as_str()) {
                    language = Some(maybe.clone());
                }
            }
        }
        let mut selection = ContextSelection::empty().restrict_dimension("scopes", ["i18n".to_string()]);
        if let Some(language) = language {
            selection = selection.restrict_dimension("language", [language]);
        }
        selection
    }

    pub fn asset_context(path: &Path) -> ContextSelection {
        let scope = file_extension(path);
        ContextSelection::empty().restrict_dimension(
            "scopes",
            [if scope.is_empty() { "file".to_string() } else { scope }],
        )
    }

    pub fn discover(repo_root: &Path) -> Self {
        let mut bundle = Self::default();
        let all_files = collect_regular_files(repo_root);
        for path in all_files {
            let rel = path.strip_prefix(repo_root).unwrap_or(path.as_path());
            let rel_text = rel.to_string_lossy();
            let ext = file_extension(&path);
            let mut payload = BTreeMap::new();
            payload.insert("path".to_string(), rel_text.to_string());
            payload.insert("suffix".to_string(), if ext.is_empty() { String::new() } else { format!(".{ext}") });
            payload.insert("name".to_string(), path.file_name().map(|v| v.to_string_lossy().to_string()).unwrap_or_default());

            if rel_text.starts_with("csv/") && ext == "csv" {
                bundle.csv.add_section(Self::csv_context(&path), payload, path.to_string_lossy().to_string());
            } else if rel_text.starts_with("i18n/") && ["po", "mo", "pot"].contains(&ext.as_str()) {
                bundle.translations.add_section(Self::translation_context(&path), payload, path.to_string_lossy().to_string());
            } else if ["md", "org", "alx", "jsonl", "js", "ts"].contains(&ext.as_str())
                || rel_text.starts_with("doc/")
            {
                bundle.assets.add_section(Self::asset_context(&path), payload, path.to_string_lossy().to_string());
            }
        }
        bundle
    }

    pub fn from_context(context: &ContextSelection) -> Self {
        let mut bundle = Self::default();
        let mut payload = BTreeMap::new();
        payload.insert("context".to_string(), format!("{:?}", context.as_dict()));
        bundle.csv.add_section(context.clone(), payload.clone(), "csv");
        bundle.translations.add_section(context.clone(), payload.clone(), "translations");
        bundle.assets.add_section(context.clone(), payload.clone(), "assets");
        bundle.prompt_state.add_section(context.clone(), payload, "prompt_state");
        bundle
    }

    pub fn snapshot(&self) -> PresheafBundleSnapshot {
        PresheafBundleSnapshot {
            csv: self.csv.snapshot(),
            translations: self.translations.snapshot(),
            assets: self.assets.snapshot(),
            prompt_state: self.prompt_state.snapshot(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PresheafBundleSnapshot {
    pub csv: PresheafSnapshot,
    pub translations: PresheafSnapshot,
    pub assets: PresheafSnapshot,
    pub prompt_state: PresheafSnapshot,
}

pub fn bootstrap_presheaves(context: Option<&ContextSelection>) -> PresheafBundle {
    match context {
        Some(context) => PresheafBundle::from_context(context),
        None => PresheafBundle::default(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_context_detects_language_prefix() {
        let context = PresheafBundle::csv_context(Path::new("csv/en-religion.csv"));
        let dict = context.as_dict();
        assert_eq!(dict.get("scopes").cloned().flatten(), Some(vec!["csv".to_string()]));
        assert_eq!(dict.get("language").cloned().flatten(), Some(vec!["en".to_string()]));
    }

    #[test]
    fn prompt_state_update_replaces_section() {
        let mut prompt = PromptStatePresheaf::default();
        prompt.update("reta -zeilen", &["reta".into(), "-zeilen".into()], None);
        let snapshot = prompt.snapshot();
        assert_eq!(snapshot.section_count, 1);
        assert_eq!(snapshot.sources, vec!["prompt".to_string()]);
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "__init__",
    "as_dict",
    "sections",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
