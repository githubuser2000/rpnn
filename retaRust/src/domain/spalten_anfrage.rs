use crate::shared::reta_py::Program;
use crate::shared::words_py::{StoreParameterEntry, Words};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpaltenAnfrage {
    pub ober_cli: String,
    pub unter_cli: String,
    pub ober_canonical: String,
    pub matched_main_names: Vec<String>,
    pub matched_parameter_names: Vec<String>,
}

impl SpaltenAnfrage {
    pub fn ober_unter_cli_pair(&self) -> (String, String) {
        (self.ober_cli.clone(), self.unter_cli.clone())
    }
}

fn matching_entries<'a>(words: &'a Words, ober: &str, unter: &str) -> Vec<&'a StoreParameterEntry> {
    let canonical = Program::canonical_spalten_main_cli_name_py(ober).to_string();
    words.paraNdataMatrix.iter().filter(|entry| {
        entry.parameterMainNames.iter().any(|name| name == &canonical || name == ober)
            && entry.parameterNames.iter().any(|name| name == unter)
    }).collect()
}

pub fn parse_spalten_anfrage(words: &Words, ober: &str, unter: &str) -> Result<SpaltenAnfrage, String> {
    let canonical = Program::canonical_spalten_main_cli_name_py(ober).to_string();
    let entries = matching_entries(words, ober, unter);
    if entries.is_empty() {
        return Err(format!("Keine exakte Python-Spaltenanfrage gefunden: --{}={}", ober, unter));
    }

    let mut matched_main_names = Vec::new();
    let mut matched_parameter_names = Vec::new();
    for entry in entries {
        for name in &entry.parameterMainNames {
            if !matched_main_names.contains(name) {
                matched_main_names.push(name.clone());
            }
        }
        for name in &entry.parameterNames {
            if !matched_parameter_names.contains(name) {
                matched_parameter_names.push(name.clone());
            }
        }
    }

    Ok(SpaltenAnfrage {
        ober_cli: ober.to_string(),
        unter_cli: unter.to_string(),
        ober_canonical: canonical,
        matched_main_names,
        matched_parameter_names,
    })
}
