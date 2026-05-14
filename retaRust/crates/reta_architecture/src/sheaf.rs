//! Sheaf and semantic gluing layer transcompiled from
//! `python_arch_reference/reta_architecture/sheaves.py`.
//!
//! Stage 13 moves this file beyond a generic `Sheaf` container.  The Python
//! architecture exposes `ParameterSemanticsSheaf` as the object that resolves
//! main aliases, sub-parameter aliases and canonical `(main, parameter)` pairs.
//! Rust now has the same typed owner, so later adapters do not have to reach
//! back into legacy Python-shaped globals.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::presheaf::LocalSection;
use crate::schema::{AliasGroup, ParameterMatrixEntry, RetaContextSchema};
use crate::topology::ContextSelection;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GluedSection {
    pub context: ContextSelection,
    pub payload: BTreeMap<String, String>,
    pub sources: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Sheaf {
    pub name: String,
    pub sections: BTreeMap<String, BTreeMap<String, String>>,
}

impl Sheaf {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sections: BTreeMap::new(),
        }
    }

    pub fn insert_section(&mut self, key: impl Into<String>, payload: BTreeMap<String, String>) {
        self.sections.insert(key.into(), payload);
    }

    pub fn is_compatible(&self, sections: &[LocalSection]) -> bool {
        let mut seen = BTreeMap::<String, String>::new();
        for section in sections {
            for (key, value) in &section.payload {
                if let Some(previous) = seen.get(key) {
                    if previous != value {
                        return false;
                    }
                } else {
                    seen.insert(key.clone(), value.clone());
                }
            }
        }
        true
    }

    pub fn glue(&self, sections: &[LocalSection]) -> Option<GluedSection> {
        if !self.is_compatible(sections) {
            return None;
        }
        let mut payload = BTreeMap::new();
        let mut sources = BTreeSet::new();
        let mut context = ContextSelection::empty();
        for section in sections {
            context = context.refine(&section.context);
            sources.insert(section.source.clone());
            for (key, value) in &section.payload {
                payload.insert(key.clone(), value.clone());
            }
        }
        Some(GluedSection {
            context,
            payload,
            sources,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ColumnParameterMeta {
    pub column_number: i64,
    pub parameter_main: String,
    pub parameter_main_aliases: Vec<String>,
    pub parameter: String,
    pub parameter_aliases: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterSemanticsSheaf {
    pub main_alias_map: BTreeMap<String, String>,
    pub main_alias_groups: Vec<AliasGroup>,
    pub parameter_alias_groups: BTreeMap<String, Vec<AliasGroup>>,
    pub pair_to_columns: BTreeMap<(String, String), Vec<i64>>,
    pub parameters_main: Vec<Vec<String>>,
    pub para_n_data_matrix: Vec<ParameterMatrixEntry>,
    pub kombi_para_n_data_matrix_size: usize,
    pub kombi_para_n_data_matrix2_size: usize,
    pub global_parameter_dict_size: usize,
    pub global_data_dicts_size: Vec<usize>,
}

impl Default for ParameterSemanticsSheaf {
    fn default() -> Self {
        Self {
            main_alias_map: BTreeMap::new(),
            main_alias_groups: Vec::new(),
            parameter_alias_groups: BTreeMap::new(),
            pair_to_columns: BTreeMap::new(),
            parameters_main: Vec::new(),
            para_n_data_matrix: Vec::new(),
            kombi_para_n_data_matrix_size: 0,
            kombi_para_n_data_matrix2_size: 0,
            global_parameter_dict_size: 0,
            global_data_dicts_size: Vec::new(),
        }
    }
}

impl ParameterSemanticsSheaf {
    pub fn from_schema(schema: &RetaContextSchema) -> Self {
        let mut sheaf = Self {
            parameters_main: schema.parameters_main.clone(),
            para_n_data_matrix: schema.para_n_data_matrix.clone(),
            kombi_para_n_data_matrix_size: schema.kombi_para_n_data_matrix_size,
            kombi_para_n_data_matrix2_size: schema.kombi_para_n_data_matrix2_size,
            ..Self::default()
        };
        sheaf.rebuild_alias_maps();
        sheaf
    }

    pub fn _rebuild_alias_maps(&mut self) {
        self.rebuild_alias_maps();
    }

    pub fn rebuild_alias_maps(&mut self) {
        self.main_alias_map.clear();
        self.main_alias_groups.clear();

        for group in &self.parameters_main {
            let aliases = group
                .iter()
                .filter(|value| !value.is_empty())
                .cloned()
                .collect::<Vec<_>>();
            let Some(canonical) = aliases.first().cloned() else {
                continue;
            };
            self.main_alias_groups.push(AliasGroup {
                canonical: canonical.clone(),
                aliases: aliases.clone(),
            });
            self.main_alias_map
                .insert(canonical.clone(), canonical.clone());
            for alias in aliases {
                self.main_alias_map.insert(alias, canonical.clone());
            }
        }

        for entry in &self.para_n_data_matrix {
            let aliases = entry
                .main_aliases
                .iter()
                .filter(|value| !value.is_empty())
                .cloned()
                .collect::<Vec<_>>();
            let Some(canonical) = aliases.first().cloned() else {
                continue;
            };
            if !self.main_alias_groups.iter().any(|group| group.canonical == canonical) {
                self.main_alias_groups.push(AliasGroup {
                    canonical: canonical.clone(),
                    aliases: aliases.clone(),
                });
            }
            self.main_alias_map.insert(canonical.clone(), canonical.clone());
            for alias in aliases {
                self.main_alias_map.insert(alias, canonical.clone());
            }
        }

        let mut parameter_groups: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
        let mut pair_to_columns: BTreeMap<(String, String), BTreeSet<i64>> = BTreeMap::new();

        for entry in &self.para_n_data_matrix {
            let main_aliases = entry
                .main_aliases
                .iter()
                .filter(|value| !value.is_empty())
                .cloned()
                .collect::<Vec<_>>();
            let parameter_aliases = entry
                .parameter_aliases
                .iter()
                .filter(|value| !value.is_empty())
                .cloned()
                .collect::<Vec<_>>();
            if main_aliases.is_empty() || parameter_aliases.is_empty() {
                continue;
            }
            let main_canonical = self
                .main_alias_map
                .get(&main_aliases[0])
                .cloned()
                .unwrap_or_else(|| main_aliases[0].clone());
            let parameter_canonical = parameter_aliases[0].clone();
            let aliases = parameter_groups
                .entry(main_canonical.clone())
                .or_default()
                .entry(parameter_canonical.clone())
                .or_default();
            for alias in parameter_aliases {
                if !aliases.contains(&alias) {
                    aliases.push(alias);
                }
            }
            let columns = pair_to_columns
                .entry((main_canonical, parameter_canonical))
                .or_default();
            for column in &entry.columns {
                columns.insert(*column);
            }
        }

        self.parameter_alias_groups = parameter_groups
            .into_iter()
            .map(|(main, groups)| {
                let mut groups = groups
                    .into_iter()
                    .map(|(canonical, mut aliases)| {
                        aliases.sort();
                        AliasGroup { canonical, aliases }
                    })
                    .collect::<Vec<_>>();
                groups.sort_by(|left, right| left.canonical.cmp(&right.canonical));
                (main, groups)
            })
            .collect();

        self.pair_to_columns = pair_to_columns
            .into_iter()
            .map(|(pair, columns)| (pair, columns.into_iter().collect()))
            .collect();
    }

    pub fn canonical_main_alias_groups(&self) -> Vec<AliasGroup> {
        self.main_alias_groups.clone()
    }

    pub fn resolve_main_alias(&self, main_name: &str) -> Option<String> {
        self.main_alias_map.get(main_name).cloned()
    }

    pub fn parameter_alias_groups_for_main(&self, main_name: &str) -> Vec<AliasGroup> {
        let canonical_main = self
            .resolve_main_alias(main_name)
            .unwrap_or_else(|| main_name.to_string());
        self.parameter_alias_groups
            .get(&canonical_main)
            .cloned()
            .unwrap_or_default()
    }

    pub fn resolve_parameter_alias(
        &self,
        main_name: &str,
        parameter_name: &str,
    ) -> Option<String> {
        let canonical_main = self
            .resolve_main_alias(main_name)
            .unwrap_or_else(|| main_name.to_string());
        for group in self.parameter_alias_groups.get(&canonical_main)? {
            if group.aliases.iter().any(|alias| alias == parameter_name) {
                return Some(group.canonical.clone());
            }
        }
        None
    }

    pub fn canonicalize_pair(
        &self,
        main_name: &str,
        parameter_name: &str,
    ) -> Option<(String, String)> {
        let canonical_main = self.resolve_main_alias(main_name)?;
        let canonical_parameter = self.resolve_parameter_alias(&canonical_main, parameter_name)?;
        Some((canonical_main, canonical_parameter))
    }

    pub fn column_numbers_for_pair(&self, main_name: &str, parameter_name: &str) -> Vec<i64> {
        let Some(pair) = self.canonicalize_pair(main_name, parameter_name) else {
            return Vec::new();
        };
        self.pair_to_columns.get(&pair).cloned().unwrap_or_default()
    }

    pub fn reverse_map_canonical_pairs(&self) -> BTreeMap<i64, Vec<(String, String)>> {
        let mut out: BTreeMap<i64, Vec<(String, String)>> = BTreeMap::new();
        for (pair, columns) in &self.pair_to_columns {
            for column in columns {
                let pairs = out.entry(*column).or_default();
                if !pairs.contains(pair) {
                    pairs.push(pair.clone());
                }
            }
        }
        for pairs in out.values_mut() {
            pairs.sort();
        }
        out
    }

    pub fn exact_meta_for_column(&self, column_number: i64) -> Vec<ColumnParameterMeta> {
        let mut matches = Vec::new();
        for entry in &self.para_n_data_matrix {
            if !entry.columns.contains(&column_number) {
                continue;
            }
            let main_aliases = entry.main_aliases.clone();
            let parameter_aliases = entry.parameter_aliases.clone();
            let Some(parameter_main) = main_aliases.first().cloned() else {
                continue;
            };
            let Some(parameter) = parameter_aliases.first().cloned() else {
                continue;
            };
            matches.push(ColumnParameterMeta {
                column_number,
                parameter_main,
                parameter_main_aliases: main_aliases,
                parameter,
                parameter_aliases,
            });
        }
        matches
    }

    pub fn sync_program_semantics(
        &mut self,
        global_parameter_dict_size: usize,
        global_data_dict_sizes: impl IntoIterator<Item = usize>,
    ) {
        self.global_parameter_dict_size = global_parameter_dict_size;
        self.global_data_dicts_size = global_data_dict_sizes.into_iter().collect();
    }

    pub fn snapshot(&self) -> ParameterSemanticsSheafSnapshot {
        ParameterSemanticsSheafSnapshot {
            main_alias_groups: self.main_alias_groups.clone(),
            parameter_main_count: self.parameters_main.len(),
            parameter_alias_main_count: self.parameter_alias_groups.len(),
            pair_to_columns_count: self.pair_to_columns.len(),
            para_n_data_matrix_size: self.para_n_data_matrix.len(),
            kombi_para_n_data_matrix_size: self.kombi_para_n_data_matrix_size,
            kombi_para_n_data_matrix2_size: self.kombi_para_n_data_matrix2_size,
            global_parameter_dict_size: self.global_parameter_dict_size,
            global_data_dict_sizes: self.global_data_dicts_size.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterSemanticsSheafSnapshot {
    pub main_alias_groups: Vec<AliasGroup>,
    pub parameter_main_count: usize,
    pub parameter_alias_main_count: usize,
    pub pair_to_columns_count: usize,
    pub para_n_data_matrix_size: usize,
    pub kombi_para_n_data_matrix_size: usize,
    pub kombi_para_n_data_matrix2_size: usize,
    pub global_parameter_dict_size: usize,
    pub global_data_dict_sizes: Vec<usize>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratedColumnsSheaf {
    pub generated_spalten_parameter: BTreeMap<i64, String>,
    pub generated_spalten_parameter_tags: BTreeMap<i64, Vec<String>>,
}

impl GeneratedColumnsSheaf {
    pub fn sync_from_tables(
        &mut self,
        generated_spalten_parameter: BTreeMap<i64, String>,
        generated_spalten_parameter_tags: BTreeMap<i64, Vec<String>>,
    ) {
        self.generated_spalten_parameter = generated_spalten_parameter;
        self.generated_spalten_parameter_tags = generated_spalten_parameter_tags;
    }

    pub fn snapshot(&self) -> GeneratedColumnsSheafSnapshot {
        GeneratedColumnsSheafSnapshot {
            generated_spalten_parameter_size: self.generated_spalten_parameter.len(),
            generated_spalten_parameter_tags_size: self.generated_spalten_parameter_tags.len(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratedColumnsSheafSnapshot {
    pub generated_spalten_parameter_size: usize,
    pub generated_spalten_parameter_tags_size: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableOutputSection {
    pub resulting_table: Vec<Vec<String>>,
    pub finally_display_lines: Option<Vec<i64>>,
    pub rows_range: Option<Vec<i64>>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableOutputSheaf {
    pub sections: BTreeMap<String, TableOutputSection>,
}

impl TableOutputSheaf {
    pub fn sync_from_tables(
        &mut self,
        output_mode: impl Into<String>,
        resulting_table: Vec<Vec<String>>,
        finally_display_lines: Option<Vec<i64>>,
        rows_range: Option<Vec<i64>>,
    ) {
        self.sections.insert(
            output_mode.into(),
            TableOutputSection {
                resulting_table,
                finally_display_lines,
                rows_range,
            },
        );
    }

    pub fn snapshot(&self) -> BTreeMap<String, TableOutputSection> {
        self.sections.clone()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct HtmlReferenceSheaf {
    pub reference_map: BTreeMap<i64, BTreeMap<String, String>>,
}

impl HtmlReferenceSheaf {
    pub fn from_jsonl(path: Option<&Path>) -> Self {
        let Some(path) = path else {
            return Self::default();
        };
        let Ok(text) = std::fs::read_to_string(path) else {
            return Self::default();
        };
        Self::from_jsonl_text(&text)
    }

    pub fn from_jsonl_text(text: &str) -> Self {
        let mut reference_map = BTreeMap::new();
        for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            let Some(column_number) = extract_json_int_field(line, "column_number") else {
                continue;
            };
            if extract_json_int_field(line, "row_number") != Some(0) {
                continue;
            }
            let mut payload = BTreeMap::new();
            payload.insert("raw_json".to_string(), line.to_string());
            if let Some(text) = extract_json_string_field(line, "text") {
                payload.insert("text".to_string(), text);
            }
            reference_map.insert(column_number, payload);
        }
        Self { reference_map }
    }

    pub fn html_meta_for_column(&self, column_number: i64) -> BTreeMap<String, String> {
        self.reference_map
            .get(&column_number)
            .cloned()
            .unwrap_or_default()
    }

    pub fn snapshot(&self) -> BTreeMap<String, BTreeMap<String, String>> {
        self.reference_map
            .iter()
            .map(|(key, value)| (key.to_string(), value.clone()))
            .collect()
    }
}

fn extract_json_int_field(line: &str, field: &str) -> Option<i64> {
    let needle = format!("\"{field}\"");
    let start = line.find(&needle)? + needle.len();
    let after_colon = line[start..].find(':')? + start + 1;
    let rest = line[after_colon..].trim_start();
    let number = rest
        .chars()
        .take_while(|ch| ch.is_ascii_digit() || *ch == '-')
        .collect::<String>();
    if number.is_empty() {
        None
    } else {
        number.parse().ok()
    }
}

fn extract_json_string_field(line: &str, field: &str) -> Option<String> {
    let needle = format!("\"{field}\"");
    let start = line.find(&needle)? + needle.len();
    let after_colon = line[start..].find(':')? + start + 1;
    let rest = line[after_colon..].trim_start();
    let rest = rest.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SheafBundle {
    pub parameter_semantics: ParameterSemanticsSheaf,
    pub generated_columns: GeneratedColumnsSheaf,
    pub table_output: TableOutputSheaf,
    pub html_reference: HtmlReferenceSheaf,
}

impl SheafBundle {
    pub fn from_schema(schema: &RetaContextSchema) -> Self {
        Self {
            parameter_semantics: ParameterSemanticsSheaf::from_schema(schema),
            generated_columns: GeneratedColumnsSheaf::default(),
            table_output: TableOutputSheaf::default(),
            html_reference: HtmlReferenceSheaf::default(),
        }
    }

    pub fn from_repo(repo_root: &Path, schema: &RetaContextSchema) -> Self {
        Self {
            parameter_semantics: ParameterSemanticsSheaf::from_schema(schema),
            generated_columns: GeneratedColumnsSheaf::default(),
            table_output: TableOutputSheaf::default(),
            html_reference: HtmlReferenceSheaf::from_jsonl(Some(&repo_root.join("htmlclassesPy.jsonl"))),
        }
    }

    pub fn snapshot(&self) -> SheafBundleSnapshot {
        SheafBundleSnapshot {
            parameter_semantics: self.parameter_semantics.snapshot(),
            generated_columns: self.generated_columns.snapshot(),
            table_output_sections: self.table_output.sections.len(),
            html_reference_size: self.html_reference.reference_map.len(),
        }
    }
}

impl Default for SheafBundle {
    fn default() -> Self {
        Self {
            parameter_semantics: ParameterSemanticsSheaf::default(),
            generated_columns: GeneratedColumnsSheaf::default(),
            table_output: TableOutputSheaf::default(),
            html_reference: HtmlReferenceSheaf::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SheafBundleSnapshot {
    pub parameter_semantics: ParameterSemanticsSheafSnapshot,
    pub generated_columns: GeneratedColumnsSheafSnapshot,
    pub table_output_sections: usize,
    pub html_reference_size: usize,
}

pub fn bootstrap_sheaves(schema: Option<&RetaContextSchema>) -> SheafBundle {
    match schema {
        Some(schema) => SheafBundle::from_schema(schema),
        None => SheafBundle::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_semantics_resolves_main_and_parameter_aliases() {
        let schema = RetaContextSchema {
            parameters_main: vec![vec!["spalten".into(), "s".into()]],
            para_n_data_matrix: vec![ParameterMatrixEntry {
                main_aliases: vec!["spalten".into(), "s".into()],
                parameter_aliases: vec!["kontinuum".into(), "m".into()],
                columns: vec![493, 744],
            }],
            ..Default::default()
        };
        let sheaf = ParameterSemanticsSheaf::from_schema(&schema);
        assert_eq!(sheaf.resolve_main_alias("s"), Some("spalten".into()));
        assert_eq!(
            sheaf.resolve_parameter_alias("s", "m"),
            Some("kontinuum".into())
        );
        assert_eq!(sheaf.column_numbers_for_pair("s", "m"), vec![493, 744]);
        assert_eq!(
            sheaf.reverse_map_canonical_pairs().get(&744).cloned(),
            Some(vec![("spalten".into(), "kontinuum".into())])
        );
    }


    #[test]
    fn bootstrap_sheaf_resolves_generated_kontinuum_aliases() {
        let sheaves = bootstrap_sheaves(None);
        let parameter = &sheaves.parameter_semantics;
        assert_eq!(parameter.resolve_main_alias("kontinuum"), Some("Kontinuum".to_string()));
        assert_eq!(parameter.resolve_parameter_alias("kontinuum", "m"), Some("M".to_string()));
        let columns = parameter.column_numbers_for_pair("kontinuum", "m");
        assert!(columns.contains(&493));
        assert!(columns.contains(&744));
    }

    #[test]
    fn html_reference_parses_jsonl_header_rows() {
        let sheaf = HtmlReferenceSheaf::from_jsonl_text(
            r#"{"column_number":744,"row_number":0,"text":"Neues M"}
{"column_number":744,"row_number":1,"text":"ignored"}"#,
        );
        let meta = sheaf.html_meta_for_column(744);
        assert!(meta.get("raw_json").is_some());
        assert_eq!(meta.get("text"), Some(&"Neues M".to_string()));
    }

    #[test]
    fn sheaf_glue_rejects_conflicting_payloads() {
        let sheaf = Sheaf::new("test");
        let mut left = BTreeMap::new();
        left.insert("a".to_string(), "1".to_string());
        let mut right = BTreeMap::new();
        right.insert("a".to_string(), "2".to_string());
        let sections = vec![
            LocalSection::new(ContextSelection::cli(), left, "left"),
            LocalSection::new(ContextSelection::cli(), right, "right"),
        ];
        assert!(!sheaf.is_compatible(&sections));
        assert!(sheaf.glue(&sections).is_none());
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "__init__",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}

// Stage 16 small-surface concrete wrappers.
pub fn __init__() -> SheafBundle {
    bootstrap_sheaves(None)
}
