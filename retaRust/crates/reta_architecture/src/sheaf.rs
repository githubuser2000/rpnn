use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::presheaf::LocalSection;
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
pub struct SheafBundle {
    pub parameter_semantics: Sheaf,
    pub generated_columns: Sheaf,
    pub table_output: Sheaf,
    pub html_reference: Sheaf,
}

impl Default for SheafBundle {
    fn default() -> Self {
        Self {
            parameter_semantics: Sheaf::new("parameter_semantics"),
            generated_columns: Sheaf::new("generated_columns"),
            table_output: Sheaf::new("table_output"),
            html_reference: Sheaf::new("html_reference"),
        }
    }
}
