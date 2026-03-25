use std::collections::BTreeSet;

use crate::cli::TextBereich;
use crate::domain::categories::KategorieMap;

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

pub fn parse_usize_csv_list(text: &str, flag_name: &str) -> Vec<usize> {
    let values: Vec<usize> = text
        .split(',')
        .map(|s| {
            let trimmed = s.trim();
            trimmed.parse::<usize>().unwrap_or_else(|_| {
                panic!(
                    "Ungültige Liste für {}: '{}' ist keine Zahl",
                    flag_name, trimmed
                )
            })
        })
        .collect();

    if values.is_empty() {
        panic!("{} darf nicht leer sein", flag_name);
    }

    values
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
        let mut set = BTreeSet::new();

        for eintrag in &kategorie_map.alle_eintraege {
            let ok = eintrag.oberkategorie.trim();
            if !ok.is_empty() {
                set.insert(ok.to_string());
            }
        }

        println!("Mögliche erste Wörter nach --spaltenname:");
        for item in set {
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
        let mut set = BTreeSet::new();
        let needle = oberkategorie.to_lowercase();

        for eintrag in &kategorie_map.alle_eintraege {
            if eintrag.oberkategorie.to_lowercase() == needle {
                let uk = eintrag.unterkategorie.trim();
                if !uk.is_empty() {
                    set.insert(uk.to_string());
                }
            }
        }

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
