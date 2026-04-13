use reta::domain::python_html_meta::html_meta_for_column;
use reta::domain::python_source_of_truth::{
    all_main_alias_groups, canonicalize_pair, column_numbers_for_pair, exact_meta_for_column,
    parameter_alias_groups_for_main, reverse_map_canonical_pairs,
};
use reta::shared::words_py::Words;
use std::collections::BTreeMap;

fn help_text(program_name: &str) -> String {
    format!(
        r#"{program_name} - Python-nahe Referenz- und Inspektionshilfe für reta

Aufruf:
  {program_name} -h
  {program_name} --help
  {program_name} mains
  {program_name} params <hauptparameter>
  {program_name} pair <hauptparameter> <unterparameter>
  {program_name} column <spaltennummer>
  {program_name} html <spaltennummer>

Befehle:
  mains
      Zeigt alle kanonischen Oberkategorien und ihre Aliase.

  params <hauptparameter>
      Zeigt alle kanonischen Unterkategorien und ihre Aliase
      für eine Oberkategorie.

  pair <hauptparameter> <unterparameter>
      Kanonisiert das Paar auf die Python-Form und zeigt
      die direkten Spaltennummern.

  column <spaltennummer>
      Zeigt die direkten Python-Metaeinträge für die Spalte
      sowie die kanonischen Paare aus der Rückwärtsabbildung.

  html <spaltennummer>
      Zeigt die aus der Referenzschicht abgeleiteten Klassen
      und data-* Attribute.

Beispiele:
  {program_name} mains
  {program_name} params Menschliches
  {program_name} pair Menschliches Motive
  {program_name} pair menschliches motive
  {program_name} column 240
  {program_name} html 240
"#
    )
}

fn print_help(program_name: &str) {
    print!("{}", help_text(program_name));
}

fn parse_i64_or_exit(raw: &str, field_name: &str) -> i64 {
    match raw.parse::<i64>() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("{field_name} ist keine gültige Zahl: {raw}");
            std::process::exit(2);
        }
    }
}

fn print_mains(words: &Words) {
    for group in all_main_alias_groups(words) {
        println!("{} => {}", group.canonical, group.aliases.join(", "));
    }
}

fn print_params(words: &Words, main_name: &str) {
    for group in parameter_alias_groups_for_main(words, main_name) {
        println!("{} => {}", group.canonical, group.aliases.join(", "));
    }
}

fn print_pair(words: &Words, main_name: &str, parameter_name: &str) {
    match canonicalize_pair(words, main_name, parameter_name) {
        Some((canonical_main, canonical_parameter)) => {
            println!("canonical={} / {}", canonical_main, canonical_parameter);
            println!(
                "columns={:?}",
                column_numbers_for_pair(words, &canonical_main, &canonical_parameter)
            );
        }
        None => {
            eprintln!("Unbekanntes Paar: {} / {}", main_name, parameter_name);
            std::process::exit(2);
        }
    }
}

fn print_column(words: &Words, column_number: i64) {
    let direct_meta = exact_meta_for_column(words, column_number);
    if direct_meta.is_empty() {
        eprintln!("Unbekannte oder nicht-direkte Spalte: {}", column_number);
        std::process::exit(2);
    }

    for entry in &direct_meta {
        println!("{} => {:?}", column_number, entry);
    }

    let summary_map: BTreeMap<i64, Vec<(String, String)>> = reverse_map_canonical_pairs(words);
    let summary_pairs = summary_map.get(&column_number).cloned().unwrap_or_default();
    println!("summary_pairs={:?}", summary_pairs);
}

fn print_html(words: &Words, column_number: i64) {
    match html_meta_for_column(words, column_number) {
        Some(meta) => {
            println!("classes={}", meta.classes.join(" "));
            for (key, value) in meta.data_attributes {
                println!("{}={}", key, value);
            }
        }
        None => {
            eprintln!("Keine HTML-Meta für Spalte {}", column_number);
            std::process::exit(2);
        }
    }
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let program_name = argv
        .first()
        .cloned()
        .unwrap_or_else(|| "reta_domain_probe".to_string());

    if argv.len() <= 1 {
        print_help(&program_name);
        return;
    }

    let words = Words::new();

    match argv[1].as_str() {
        "-h" | "--help" | "help" => print_help(&program_name),
        "mains" => print_mains(&words),
        "params" => {
            if argv.len() != 3 {
                eprintln!("Erwartet: {} params <hauptparameter>", program_name);
                std::process::exit(2);
            }
            print_params(&words, &argv[2]);
        }
        "pair" => {
            if argv.len() != 4 {
                eprintln!(
                    "Erwartet: {} pair <hauptparameter> <unterparameter>",
                    program_name
                );
                std::process::exit(2);
            }
            print_pair(&words, &argv[2], &argv[3]);
        }
        "column" => {
            if argv.len() != 3 {
                eprintln!("Erwartet: {} column <spaltennummer>", program_name);
                std::process::exit(2);
            }
            let column_number = parse_i64_or_exit(&argv[2], "spaltennummer");
            print_column(&words, column_number);
        }
        "html" => {
            if argv.len() != 3 {
                eprintln!("Erwartet: {} html <spaltennummer>", program_name);
                std::process::exit(2);
            }
            let column_number = parse_i64_or_exit(&argv[2], "spaltennummer");
            print_html(&words, column_number);
        }
        other => {
            eprintln!("Unbekannter Befehl: {}", other);
            eprintln!();
            print_help(&program_name);
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_mentions_core_commands() {
        let text = help_text("reta_domain_probe");
        assert!(text.contains("mains"));
        assert!(text.contains("params"));
        assert!(text.contains("pair"));
        assert!(text.contains("column"));
        assert!(text.contains("html"));
        assert!(text.contains("--help"));
    }
}
