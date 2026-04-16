use std::collections::BTreeSet;
use std::fmt;

use crate::cli::TextBereich;
use crate::domain::categories::{KategorieMap, KategorieProvider, OberkategorieEntry, UnterkategorieEntry};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseCliValueError {
    EmptyList { flag_name: String },
    InvalidNumber { flag_name: String, value: String },
}

impl fmt::Display for ParseCliValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyList { flag_name } => write!(f, "{} darf nicht leer sein", flag_name),
            Self::InvalidNumber { flag_name, value } => {
                write!(f, "Ungültige Liste für {}: '{}' ist keine Zahl", flag_name, value)
            }
        }
    }
}

impl std::error::Error for ParseCliValueError {}


fn collect_oberkategorie_names<T>(provider: &T) -> BTreeSet<String>
where
    T: KategorieProvider,
{
    provider
        .hauptkategorien()
        .iter()
        .filter_map(|haupt| {
            let ok = haupt.ober_name().trim();
            (!ok.is_empty()).then(|| ok.to_string())
        })
        .collect()
}

fn collect_unterkategorie_names<T>(provider: &T, oberkategorie: &str) -> BTreeSet<String>
where
    T: KategorieProvider,
{
    let needle = oberkategorie.to_lowercase();
    provider
        .hauptkategorien()
        .iter()
        .filter(|haupt| haupt.ober_name().to_lowercase() == needle)
        .flat_map(|haupt| haupt.unterkategorien().iter())
        .filter_map(|unter| {
            let uk = unter.unter_name().trim();
            (!uk.is_empty()).then(|| uk.to_string())
        })
        .collect()
}

pub fn is_flag(s: &str) -> bool {
    s.starts_with('-')
}

pub fn parse_pypy_number_set(text: &str) -> BTreeSet<usize> {
    text.split(',')
        .filter_map(|part| {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                return None;
            }
            trimmed.parse::<isize>().ok().map(|v| v.unsigned_abs())
        })
        .filter(|&v| v > 1)
        .collect()
}

pub fn try_parse_usize_csv_list(text: &str, flag_name: &str) -> Result<Vec<usize>, ParseCliValueError> {
    let values: Vec<usize> = text
        .split(',')
        .map(|s| {
            let trimmed = s.trim();
            trimmed.parse::<usize>().map_err(|_| ParseCliValueError::InvalidNumber {
                flag_name: flag_name.to_string(),
                value: trimmed.to_string(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    if values.is_empty() {
        return Err(ParseCliValueError::EmptyList {
            flag_name: flag_name.to_string(),
        });
    }

    Ok(values)
}

pub fn parse_usize_csv_list(text: &str, flag_name: &str) -> Vec<usize> {
    try_parse_usize_csv_list(text, flag_name).unwrap_or_else(|err| panic!("{}", err))
}

pub fn apply_pypy_compat_arg(bereich: &mut TextBereich, arg: &str) -> bool {
    let mut parts = arg.splitn(2, '=');
    let key = parts.next().unwrap_or("");
    let value = parts.next().unwrap_or("");
    let numbers = parse_pypy_number_set(value);
    if numbers.is_empty() {
        return false;
    }

    match key {
        "--gebrochengalaxie" => bereich.pypy_compat.gebrochengalaxie.extend(numbers),
        "--gebrochenuniversum" => bereich.pypy_compat.gebrochenuniversum.extend(numbers),
        "--gebrochenemotion" => bereich.pypy_compat.gebrochenemotion.extend(numbers),
        "--gebrochengroesse" => bereich.pypy_compat.gebrochengroesse.extend(numbers),
        "--galaxie" => bereich.pypy_compat.kombi_galaxie.extend(numbers),
        "--universum" => bereich.pypy_compat.kombi_universum.extend(numbers),
        _ => return false,
    }

    true
}

pub fn print_all_oberkategorien(kategorie_map: Option<&KategorieMap>) {
    if let Some(kategorie_map) = kategorie_map {
        println!("Mögliche erste Wörter nach --spaltenname:");
        for item in collect_oberkategorie_names(kategorie_map) {
            println!("{item}");
        }
    } else {
        println!("Keine Kategoriedaten verfügbar.");
    }
}

pub fn print_passende_unterkategorien(
    kategorie_map: Option<&KategorieMap>,
    oberkategorie: &str,
) {
    if let Some(kategorie_map) = kategorie_map {
        let set = collect_unterkategorie_names(kategorie_map, oberkategorie);

        if set.is_empty() {
            println!("Keine passenden zweiten Wörter für '{oberkategorie}' gefunden.");
        } else {
            println!("Mögliche zweite Wörter für '{oberkategorie}':");
            for item in set {
                println!("{item}");
            }
        }
    } else {
        println!("Keine Kategoriedaten verfügbar.");
    }
}
