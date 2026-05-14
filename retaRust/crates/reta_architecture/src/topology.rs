use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

const DIMENSIONS: [&str; 8] = [
    "language",
    "main_parameters",
    "sub_parameters",
    "row_parameters",
    "output_modes",
    "tag_names",
    "combination_parameters",
    "scopes",
];

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContextSelection {
    pub language: Option<BTreeSet<String>>,
    pub main_parameters: Option<BTreeSet<String>>,
    pub sub_parameters: Option<BTreeSet<String>>,
    pub row_parameters: Option<BTreeSet<String>>,
    pub output_modes: Option<BTreeSet<String>>,
    pub tag_names: Option<BTreeSet<String>>,
    pub combination_parameters: Option<BTreeSet<String>>,
    pub scopes: Option<BTreeSet<String>>,
}

impl ContextSelection {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn for_scope(scope: impl Into<String>) -> Self {
        let mut scopes = BTreeSet::new();
        scopes.insert(scope.into());
        Self {
            scopes: Some(scopes),
            ..Self::default()
        }
    }

    pub fn prompt() -> Self {
        Self::for_scope("prompt")
    }

    pub fn cli() -> Self {
        Self::for_scope("cli")
    }

    pub fn restrict_dimension<I, S>(mut self, dimension: &str, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let collected = values.into_iter().map(Into::into).collect::<BTreeSet<_>>();
        match dimension {
            "language" => self.language = Some(collected),
            "main_parameters" => self.main_parameters = Some(collected),
            "sub_parameters" => self.sub_parameters = Some(collected),
            "row_parameters" => self.row_parameters = Some(collected),
            "output_modes" => self.output_modes = Some(collected),
            "tag_names" => self.tag_names = Some(collected),
            "combination_parameters" => self.combination_parameters = Some(collected),
            "scopes" => self.scopes = Some(collected),
            _ => {}
        }
        self
    }

    pub fn refine(&self, other: &Self) -> Self {
        Self {
            language: meet_sets(&self.language, &other.language),
            main_parameters: meet_sets(&self.main_parameters, &other.main_parameters),
            sub_parameters: meet_sets(&self.sub_parameters, &other.sub_parameters),
            row_parameters: meet_sets(&self.row_parameters, &other.row_parameters),
            output_modes: meet_sets(&self.output_modes, &other.output_modes),
            tag_names: meet_sets(&self.tag_names, &other.tag_names),
            combination_parameters: meet_sets(
                &self.combination_parameters,
                &other.combination_parameters,
            ),
            scopes: meet_sets(&self.scopes, &other.scopes),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.dimension_values()
            .values()
            .any(|value| matches!(value, Some(items) if items.is_empty()))
    }

    pub fn dimension_values(&self) -> BTreeMap<&'static str, Option<BTreeSet<String>>> {
        BTreeMap::from([
            ("language", self.language.clone()),
            ("main_parameters", self.main_parameters.clone()),
            ("sub_parameters", self.sub_parameters.clone()),
            ("row_parameters", self.row_parameters.clone()),
            ("output_modes", self.output_modes.clone()),
            ("tag_names", self.tag_names.clone()),
            ("combination_parameters", self.combination_parameters.clone()),
            ("scopes", self.scopes.clone()),
        ])
    }

    pub fn as_dict(&self) -> BTreeMap<&'static str, Option<Vec<String>>> {
        self.dimension_values()
            .into_iter()
            .map(|(key, value)| (key, value.map(|items| items.into_iter().collect())))
            .collect()
    }

    pub fn from_cli_args(args: &[String]) -> Self {
        let mut selection = Self::cli();
        let mut main_parameters = BTreeSet::new();
        let mut sub_parameters = BTreeSet::new();
        let mut output_modes = BTreeSet::new();
        let mut row_parameters = BTreeSet::new();
        let mut combination_parameters = BTreeSet::new();

        for arg in args {
            let trimmed = arg.trim();
            if trimmed.starts_with("-") && !trimmed.starts_with("--") {
                let name = trimmed.trim_start_matches('-').split('=').next().unwrap_or("");
                if !name.is_empty() {
                    main_parameters.insert(name.to_string());
                }
            } else if trimmed.starts_with("--") {
                let name = trimmed.trim_start_matches('-').split('=').next().unwrap_or("");
                if name.contains("zeile") || name.contains("ausschnitt") || name.contains("alles") {
                    row_parameters.insert(name.to_string());
                } else if name.contains("art") || name.contains("breite") || name.contains("ausgabe") {
                    output_modes.insert(name.to_string());
                } else if name.contains("kombi") || name.contains("galaxie") || name.contains("universum") {
                    combination_parameters.insert(name.to_string());
                } else if !name.is_empty() {
                    sub_parameters.insert(name.to_string());
                }
            }
        }

        if !main_parameters.is_empty() {
            selection.main_parameters = Some(main_parameters);
        }
        if !sub_parameters.is_empty() {
            selection.sub_parameters = Some(sub_parameters);
        }
        if !output_modes.is_empty() {
            selection.output_modes = Some(output_modes);
        }
        if !row_parameters.is_empty() {
            selection.row_parameters = Some(row_parameters);
        }
        if !combination_parameters.is_empty() {
            selection.combination_parameters = Some(combination_parameters);
        }
        selection
    }

    pub fn from_prompt_input(program_name: &str, input: &str) -> Self {
        let mut selection = Self::prompt();
        let mut scopes = BTreeSet::from(["prompt".to_string(), program_name.to_string()]);
        if input.trim_start().starts_with("reta") {
            scopes.insert("embedded_reta".to_string());
        }
        selection.scopes = Some(scopes);
        selection
    }
}

fn meet_sets(
    left: &Option<BTreeSet<String>>,
    right: &Option<BTreeSet<String>>,
) -> Option<BTreeSet<String>> {
    match (left, right) {
        (None, None) => None,
        (Some(value), None) | (None, Some(value)) => Some(value.clone()),
        (Some(left), Some(right)) => Some(left.intersection(right).cloned().collect()),
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContextDimension {
    pub name: String,
    pub values: BTreeSet<String>,
    pub aliases: BTreeMap<String, String>,
}

impl ContextDimension {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            values: BTreeSet::new(),
            aliases: BTreeMap::new(),
        }
    }

    pub fn include<I, S>(&mut self, canonical: impl Into<String>, aliases: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let canonical = canonical.into();
        self.values.insert(canonical.clone());
        self.aliases.insert(canonical.clone(), canonical.clone());
        for alias in aliases {
            let alias = alias.into();
            if !alias.is_empty() {
                self.aliases.insert(alias, canonical.clone());
            }
        }
    }

    pub fn canonicalize(&self, value: Option<&str>) -> Option<String> {
        let value = value?;
        if self.values.contains(value) {
            return Some(value.to_string());
        }
        self.aliases.get(value).cloned()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetaContextTopology {
    pub dimensions: BTreeMap<String, ContextDimension>,
}

impl Default for RetaContextTopology {
    fn default() -> Self {
        Self::standard()
    }
}

impl RetaContextTopology {
    pub fn standard() -> Self {
        let mut dimensions = BTreeMap::new();
        for name in DIMENSIONS {
            dimensions.insert(name.to_string(), ContextDimension::new(name));
        }

        if let Some(scope) = dimensions.get_mut("scopes") {
            scope.include("cli", ["reta"]);
            scope.include("prompt", ["rp", "rpl", "rpb", "rpe"]);
            scope.include("csv", ["table"]);
            scope.include("i18n", ["words"]);
        }
        if let Some(output) = dimensions.get_mut("output_modes") {
            output.include("cli", ["text"]);
            output.include("html", ["browser"]);
            output.include("bbcode", ["forum"]);
            output.include("emacs", ["org"]);
        }
        if let Some(language) = dimensions.get_mut("language") {
            language.include("de", ["german"]);
            language.include("en", ["english"]);
            language.include("vn", ["vietnamese"]);
            language.include("cn", ["chinese"]);
            language.include("kr", ["korean"]);
        }

        Self { dimensions }
    }

    pub fn canonicalize(&self, dimension: &str, value: Option<&str>) -> Option<String> {
        self.dimensions.get(dimension)?.canonicalize(value)
    }

    pub fn open_for<I, S>(&self, dimension: &str, values: I) -> ContextSelection
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let canonical = values
            .into_iter()
            .map(Into::into)
            .filter_map(|value: String| {
                self.canonicalize(dimension, Some(&value)).or(Some(value))
            })
            .collect::<BTreeSet<_>>();
        ContextSelection::empty().restrict_dimension(dimension, canonical)
    }

    pub fn basis_open_sets(&self) -> BTreeMap<String, ContextSelection> {
        self.dimensions
            .iter()
            .map(|(name, dimension)| {
                (
                    name.clone(),
                    ContextSelection::empty().restrict_dimension(name, dimension.values.clone()),
                )
            })
            .collect()
    }

    pub fn cover_for_main(&self, main_name: &str) -> Vec<ContextSelection> {
        let canonical_main = self
            .canonicalize("main_parameters", Some(main_name))
            .unwrap_or_else(|| main_name.to_string());
        vec![
            ContextSelection::empty().restrict_dimension("main_parameters", [canonical_main]),
            ContextSelection::empty().restrict_dimension("scopes", ["spalten".to_string()]),
        ]
    }

    pub fn refine<I>(&self, selections: I) -> ContextSelection
    where
        I: IntoIterator<Item = ContextSelection>,
    {
        selections
            .into_iter()
            .fold(ContextSelection::empty(), |acc, selection| acc.refine(&selection))
    }

    pub fn snapshot_dimensions(&self) -> BTreeMap<String, BTreeMap<&'static str, Vec<String>>> {
        self.dimensions
            .iter()
            .map(|(name, dimension)| {
                let aliases = dimension
                    .aliases
                    .iter()
                    .map(|(key, value)| format!("{key}->{value}"))
                    .collect::<Vec<_>>();
                (
                    name.clone(),
                    BTreeMap::from([
                        ("values", dimension.values.iter().cloned().collect()),
                        ("aliases", aliases),
                    ]),
                )
            })
            .collect()
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "__init__",
    "_meet",
    "from_schema",
    "restrict",
    "snapshot",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
