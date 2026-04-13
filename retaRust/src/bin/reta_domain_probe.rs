use reta::domain::python_html_meta::html_meta_for_column;
use reta::domain::python_source_of_truth::{
    all_main_alias_groups, canonicalize_pair, column_numbers_for_pair, exact_meta_for_column,
    parameter_alias_groups_for_main, reverse_map_canonical_pairs,
};
use reta::shared::words_py::Words;
use std::collections::{BTreeMap, BTreeSet};

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn json_string(s: &str) -> String {
    format!("\"{}\"", json_escape(s))
}

fn json_string_array(items: &[String]) -> String {
    format!(
        "[{}]",
        items.iter()
            .map(|s| json_string(s))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn json_i64_array(items: &[i64]) -> String {
    format!(
        "[{}]",
        items.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn help_text(program_name: &str) -> String {
    format!(
        r#"{program_name} - Python-nahe Referenz- und Inspektionshilfe für reta

Aufruf:
  {program_name} -h
  {program_name} --help
  {program_name} mains
  {program_name} params <hauptparameter>
  {program_name} pairs <hauptparameter>
  {program_name} pairs-json <hauptparameter>
  {program_name} main-columns <hauptparameter>
  {program_name} main-json <hauptparameter>
  {program_name} pair <hauptparameter> <unterparameter>
  {program_name} pair-json <hauptparameter> <unterparameter>
  {program_name} pair-html <hauptparameter> <unterparameter>
  {program_name} column <spaltennummer>
  {program_name} column-json <spaltennummer>
  {program_name} reverse <spaltennummer>
  {program_name} html <spaltennummer>

Befehle:
  mains
      Zeigt alle kanonischen Oberkategorien und ihre Aliase.

  params <hauptparameter>
      Zeigt alle kanonischen Unterkategorien und ihre Aliase
      für eine Oberkategorie.

  pairs <hauptparameter>
      Zeigt alle kanonischen (Oberkategorie, Unterkategorie)-Paare
      des Hauptparameters, für die direkte Spalten existieren.

  pairs-json <hauptparameter>
      Wie pairs, aber als maschinenlesbares JSON.

  main-columns <hauptparameter>
      Zeigt die Vereinigungsmenge aller direkten Spaltennummern,
      die unter einem Hauptparameter vorkommen.

  main-json <hauptparameter>
      Zeigt Hauptparameter, Aliase, alle Unterparameter,
      deren Aliase sowie direkte Spalten als JSON.

  pair <hauptparameter> <unterparameter>
      Kanonisiert das Paar auf die Python-Form und zeigt
      die direkten Spaltennummern.

  pair-json <hauptparameter> <unterparameter>
      Wie pair, aber als maschinenlesbares JSON.

  pair-html <hauptparameter> <unterparameter>
      Zeigt für alle direkten Spalten des Paares die HTML-Meta,
      also Klassen und data-* Attribute.

  column <spaltennummer>
      Zeigt die direkten Python-Metaeinträge für die Spalte
      sowie die kanonischen Paare aus der Rückwärtsabbildung.

  column-json <spaltennummer>
      Wie column, aber als maschinenlesbares JSON.

  reverse <spaltennummer>
      Zeigt nur die kanonischen Paare aus der Rückwärtsabbildung
      für eine Spaltennummer.

  html <spaltennummer>
      Zeigt die aus der Referenzschicht abgeleiteten Klassen
      und data-* Attribute.

Beispiele:
  {program_name} mains
  {program_name} params Menschliches
  {program_name} pairs Menschliches
  {program_name} pairs-json Menschliches
  {program_name} main-columns Menschliches
  {program_name} main-json Menschliches
  {program_name} pair Menschliches Motive
  {program_name} pair-json Menschliches Motive
  {program_name} pair-html Menschliches Motive
  {program_name} pair menschliches motive
  {program_name} column 240
  {program_name} column-json 240
  {program_name} reverse 240
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

fn print_pairs(words: &Words, main_name: &str) {
    for group in parameter_alias_groups_for_main(words, main_name) {
        let cols = column_numbers_for_pair(words, main_name, &group.canonical);
        if !cols.is_empty() {
            println!("{} / {} => {:?}", main_name, group.canonical, cols);
        }
    }
}

fn print_pairs_json(words: &Words, main_name: &str) {
    let mut out = vec![];
    for group in parameter_alias_groups_for_main(words, main_name) {
        let cols = column_numbers_for_pair(words, main_name, &group.canonical);
        if !cols.is_empty() {
            out.push(format!(
                "{{\"main\":{},\"parameter\":{},\"columns\":{}}}",
                json_string(main_name),
                json_string(&group.canonical),
                json_i64_array(&cols)
            ));
        }
    }
    println!("[{}]", out.join(","));
}

fn print_main_columns(words: &Words, main_name: &str) {
    let mut all = BTreeSet::new();
    for group in parameter_alias_groups_for_main(words, main_name) {
        for c in column_numbers_for_pair(words, main_name, &group.canonical) {
            all.insert(c);
        }
    }
    let cols: Vec<i64> = all.into_iter().collect();
    println!("main_columns={:?}", cols);
}

fn print_main_json(words: &Words, main_name: &str) {
    let mut main_aliases = vec![];
    for group in all_main_alias_groups(words) {
        if group.canonical == main_name {
            main_aliases = group.aliases;
            break;
        }
    }

    let mut all = BTreeSet::new();
    let mut pairs = vec![];

    for group in parameter_alias_groups_for_main(words, main_name) {
        let cols = column_numbers_for_pair(words, main_name, &group.canonical);
        for c in &cols {
            all.insert(*c);
        }
        if !cols.is_empty() {
            pairs.push(format!(
                "{{\"parameter\":{},\"aliases\":{},\"columns\":{}}}",
                json_string(&group.canonical),
                json_string_array(&group.aliases),
                json_i64_array(&cols)
            ));
        }
    }

    let all_cols: Vec<i64> = all.into_iter().collect();

    println!(
        "{{\"main\":{},\"aliases\":{},\"columns\":{},\"pairs\":[{}]}}",
        json_string(main_name),
        json_string_array(&main_aliases),
        json_i64_array(&all_cols),
        pairs.join(",")
    );
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

fn print_pair_json(words: &Words, main_name: &str, parameter_name: &str) {
    match canonicalize_pair(words, main_name, parameter_name) {
        Some((canonical_main, canonical_parameter)) => {
            let mut main_aliases = vec![];
            for group in all_main_alias_groups(words) {
                if group.canonical == canonical_main {
                    main_aliases = group.aliases;
                    break;
                }
            }

            let mut parameter_aliases = vec![];
            for group in parameter_alias_groups_for_main(words, &canonical_main) {
                if group.canonical == canonical_parameter {
                    parameter_aliases = group.aliases;
                    break;
                }
            }

            let cols = column_numbers_for_pair(words, &canonical_main, &canonical_parameter);

            println!(
                "{{\"input_main\":{},\"input_parameter\":{},\"canonical_main\":{},\"canonical_parameter\":{},\"main_aliases\":{},\"parameter_aliases\":{},\"columns\":{}}}",
                json_string(main_name),
                json_string(parameter_name),
                json_string(&canonical_main),
                json_string(&canonical_parameter),
                json_string_array(&main_aliases),
                json_string_array(&parameter_aliases),
                json_i64_array(&cols)
            );
        }
        None => {
            eprintln!("Unbekanntes Paar: {} / {}", main_name, parameter_name);
            std::process::exit(2);
        }
    }
}

fn print_pair_html(words: &Words, main_name: &str, parameter_name: &str) {
    match canonicalize_pair(words, main_name, parameter_name) {
        Some((canonical_main, canonical_parameter)) => {
            println!("canonical={} / {}", canonical_main, canonical_parameter);
            for column_number in column_numbers_for_pair(words, &canonical_main, &canonical_parameter) {
                println!("column={}", column_number);
                match html_meta_for_column(words, column_number) {
                    Some(meta) => {
                        println!("classes={}", meta.classes.join(" "));
                        for (key, value) in meta.data_attributes {
                            println!("{}={}", key, value);
                        }
                    }
                    None => {
                        println!("no_html_meta");
                    }
                }
            }
        }
        None => {
            eprintln!("Unbekanntes Paar: {} / {}", main_name, parameter_name);
            std::process::exit(2);
        }
    }
}

fn print_column(words: &Words, column_number: i64) {
    let meta = exact_meta_for_column(words, column_number);

    if meta.is_empty() {
        eprintln!("Unbekannte oder nicht-direkte Spalte: {}", column_number);
        std::process::exit(2);
    }

    for direct in &meta {
        println!("{column_number} => {:?}", direct);
    }

    let summary_map: BTreeMap<i64, Vec<(String, String)>> = reverse_map_canonical_pairs(words);
    let summary_pairs = summary_map.get(&column_number).cloned().unwrap_or_default();
    println!("summary_pairs={:?}", summary_pairs);
}

fn print_column_json(words: &Words, column_number: i64) {
    let meta = exact_meta_for_column(words, column_number);

    if meta.is_empty() {
        eprintln!("Unbekannte oder nicht-direkte Spalte: {}", column_number);
        std::process::exit(2);
    }

    let match_json = meta
        .iter()
        .map(|direct| {
            format!(
                "{{\"column_number\":{},\"parameter_main\":{},\"parameter_main_aliases\":{},\"parameter\":{},\"parameter_aliases\":{}}}",
                direct.column_number,
                json_string(&direct.parameter_main),
                json_string_array(&direct.parameter_main_aliases),
                json_string(&direct.parameter),
                json_string_array(&direct.parameter_aliases)
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let summary_map: BTreeMap<i64, Vec<(String, String)>> = reverse_map_canonical_pairs(words);
    let summary_pairs = summary_map.get(&column_number).cloned().unwrap_or_default();
    let summary_json = summary_pairs
        .iter()
        .map(|(main, parameter)| {
            format!(
                "{{\"main\":{},\"parameter\":{}}}",
                json_string(main),
                json_string(parameter)
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    println!(
        "{{\"column_number\":{},\"matches\":[{}],\"summary_pairs\":[{}]}}",
        column_number, match_json, summary_json
    );
}

fn print_reverse(words: &Words, column_number: i64) {
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
        "pairs" => {
            if argv.len() != 3 {
                eprintln!("Erwartet: {} pairs <hauptparameter>", program_name);
                std::process::exit(2);
            }
            print_pairs(&words, &argv[2]);
        }
        "pairs-json" => {
            if argv.len() != 3 {
                eprintln!("Erwartet: {} pairs-json <hauptparameter>", program_name);
                std::process::exit(2);
            }
            print_pairs_json(&words, &argv[2]);
        }
        "main-columns" => {
            if argv.len() != 3 {
                eprintln!("Erwartet: {} main-columns <hauptparameter>", program_name);
                std::process::exit(2);
            }
            print_main_columns(&words, &argv[2]);
        }
        "main-json" => {
            if argv.len() != 3 {
                eprintln!("Erwartet: {} main-json <hauptparameter>", program_name);
                std::process::exit(2);
            }
            print_main_json(&words, &argv[2]);
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
        "pair-json" => {
            if argv.len() != 4 {
                eprintln!(
                    "Erwartet: {} pair-json <hauptparameter> <unterparameter>",
                    program_name
                );
                std::process::exit(2);
            }
            print_pair_json(&words, &argv[2], &argv[3]);
        }
        "pair-html" => {
            if argv.len() != 4 {
                eprintln!(
                    "Erwartet: {} pair-html <hauptparameter> <unterparameter>",
                    program_name
                );
                std::process::exit(2);
            }
            print_pair_html(&words, &argv[2], &argv[3]);
        }
        "column" => {
            if argv.len() != 3 {
                eprintln!("Erwartet: {} column <spaltennummer>", program_name);
                std::process::exit(2);
            }
            let column_number = parse_i64_or_exit(&argv[2], "spaltennummer");
            print_column(&words, column_number);
        }
        "column-json" => {
            if argv.len() != 3 {
                eprintln!("Erwartet: {} column-json <spaltennummer>", program_name);
                std::process::exit(2);
            }
            let column_number = parse_i64_or_exit(&argv[2], "spaltennummer");
            print_column_json(&words, column_number);
        }
        "reverse" => {
            if argv.len() != 3 {
                eprintln!("Erwartet: {} reverse <spaltennummer>", program_name);
                std::process::exit(2);
            }
            let column_number = parse_i64_or_exit(&argv[2], "spaltennummer");
            print_reverse(&words, column_number);
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
        assert!(text.contains("pairs"));
        assert!(text.contains("pairs-json"));
        assert!(text.contains("main-columns"));
        assert!(text.contains("main-json"));
        assert!(text.contains("pair"));
        assert!(text.contains("pair-json"));
        assert!(text.contains("column"));
        assert!(text.contains("column-json"));
        assert!(text.contains("reverse"));
        assert!(text.contains("html"));
        assert!(text.contains("--help"));
    }

    #[test]
    fn json_escape_handles_quotes() {
        assert_eq!(json_escape(r#"a"b"#), r#"a\"b"#);
    }
}
