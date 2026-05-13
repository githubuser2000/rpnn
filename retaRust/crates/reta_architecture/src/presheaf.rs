use std::collections::BTreeMap;

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
