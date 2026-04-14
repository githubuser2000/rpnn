use std::collections::BTreeSet;
use std::sync::OnceLock;

use reedline::DefaultCompleter;

use crate::domain::python_source_of_truth::{all_main_alias_groups, parameter_alias_groups_for_main};
use crate::shared_words;

pub const RP_META_COMMANDS: &[&str] = &[
    "help",
    "hilfe",
    "befehle",
    "kurzbefehle",
    "q",
    ":q",
    "exit",
    "quit",
    "ende",
    "leeren",
    "clear",
    ":ui",
    ":preview",
    ":history",
    ":mode vi",
    ":mode emacs",
    "shell",
    "reta",
];

#[derive(Clone, Debug, Default)]
pub struct PromptMetadata {
    pub vocabulary: Vec<String>,
}

static PROMPT_METADATA: OnceLock<PromptMetadata> = OnceLock::new();

pub fn prompt_metadata() -> &'static PromptMetadata {
    PROMPT_METADATA.get_or_init(build_prompt_metadata)
}

fn build_prompt_metadata() -> PromptMetadata {
    let words = shared_words();
    let mut items = BTreeSet::new();

    for item in RP_META_COMMANDS {
        items.insert((*item).to_string());
    }

    for main in ["-zeilen", "-spalten", "-kombination", "-ausgabe", "-debug", "-h", "-help"] {
        items.insert(main.to_string());
    }

    for side in [
        "--vorhervonausschnitt=",
        "--vorhervonausschnittteiler",
        "--zaehlung=",
        "--zeit=",
        "--alles",
        "--potenzenvonzahlen=",
        "--typ=",
        "--vielfachevonzahlen=",
        "--oberesmaximum=",
        "--primzahlen=",
        "--invertieren",
        "--primzahlvielfache=",
        "--nachtraeglichneuabzaehlung=",
        "--nachtraeglichneuabzaehlungvielfache=",
        "--breite=",
        "--breiten=",
        "--keinenummerierung",
        "--keineueberschriften",
        "--art=",
        "--onetable",
        "--justtext",
        "--nocolor",
        "--spaltenreihenfolgeundnurdiese=",
        "--galaxie=",
        "--universum=",
        "--*="
    ] {
        items.insert(side.to_string());
    }

    for main_group in all_main_alias_groups(words) {
        for alias in &main_group.aliases {
            items.insert(alias.clone());
        }
        for parameter_group in parameter_alias_groups_for_main(words, &main_group.canonical) {
            for alias in &parameter_group.aliases {
                items.insert(alias.clone());
            }
        }
    }

    PromptMetadata {
        vocabulary: items.into_iter().collect(),
    }
}

pub fn completion_vocabulary() -> Vec<String> {
    prompt_metadata().vocabulary.clone()
}

pub fn build_default_completer() -> Box<DefaultCompleter> {
    Box::new(DefaultCompleter::new_with_wordlen(
        completion_vocabulary(),
        2,
    ))
}

pub fn candidates_for_prefix(prefix: &str) -> Vec<String> {
    let lower = prefix.to_lowercase();
    prompt_metadata()
        .vocabulary
        .iter()
        .filter(|candidate| candidate.to_lowercase().contains(&lower))
        .cloned()
        .collect()
}
