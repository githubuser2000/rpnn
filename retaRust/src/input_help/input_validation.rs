use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationIssue {
    pub argument: String,
    pub message: String,
}

fn known_main_parameters() -> BTreeSet<&'static str> {
    BTreeSet::from([
        "-zeilen",
        "-spalten",
        "-kombination",
        "-ausgabe",
        "-debug",
        "-h",
        "-help",
    ])
}

pub fn validate_cli_sequence(argv: &[String]) -> Vec<ValidationIssue> {
    let known = known_main_parameters();
    let mut issues = Vec::new();
    let mut active_main: Option<String> = None;

    for arg in argv.iter().skip(1) {
        if known.contains(arg.as_str()) {
            active_main = Some(arg.clone());
            continue;
        }
        if arg.starts_with('-') && !arg.starts_with("--") {
            issues.push(ValidationIssue {
                argument: arg.clone(),
                message: "Unbekannter Hauptparameter".to_string(),
            });
            active_main = Some(arg.clone());
            continue;
        }
        if arg.starts_with("--") && active_main.is_none() {
            issues.push(ValidationIssue {
                argument: arg.clone(),
                message: "Nebenparameter ohne vorherigen Hauptparameter".to_string(),
            });
        }
    }

    issues
}

pub fn collect_main_parameters(argv: &[String]) -> Vec<String> {
    let known = known_main_parameters();
    argv.iter()
        .skip(1)
        .filter(|arg| known.contains(arg.as_str()))
        .cloned()
        .collect()
}
