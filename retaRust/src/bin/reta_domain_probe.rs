use reta::domain::python_html_meta::html_meta_for_column;
use reta::domain::python_source_of_truth::{
    alias_summary_for_column, all_main_alias_groups, column_numbers_for_pair, exact_meta_for_column,
    parameter_alias_groups_for_main,
};
use reta::domain::spalten_anfrage::parse_spalten_anfrage;
use reta::shared::words_py::Words;

fn print_usage() {
    println!("reta_domain_probe mains");
    println!("reta_domain_probe params <main>");
    println!("reta_domain_probe pair <main> <param>");
    println!("reta_domain_probe column <number>");
    println!("reta_domain_probe html <number>");
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let words = Words::new();
    let Some(cmd) = argv.get(1).map(|s| s.as_str()) else {
        print_usage();
        std::process::exit(1);
    };

    match cmd {
        "mains" => {
            for group in all_main_alias_groups(&words) {
                println!("{} => {}", group.canonical, group.aliases.join(", "));
            }
        }
        "params" => {
            let Some(main) = argv.get(2) else {
                print_usage();
                std::process::exit(1);
            };
            let groups = parameter_alias_groups_for_main(&words, main);
            if groups.is_empty() {
                eprintln!("No parameter groups for main: {main}");
                std::process::exit(2);
            }
            for group in groups {
                println!("{} => {}", group.canonical, group.aliases.join(", "));
            }
        }
        "pair" => {
            let (Some(main), Some(param)) = (argv.get(2), argv.get(3)) else {
                print_usage();
                std::process::exit(1);
            };
            match parse_spalten_anfrage(&words, main, param) {
                Ok(req) => {
                    let pair = req.ober_unter_cli_pair();
                    println!("canonical={} / {}", pair.0, pair.1);
                    println!("columns={:?}", column_numbers_for_pair(&words, &pair.0, &pair.1));
                }
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(2);
                }
            }
        }
        "column" => {
            let Some(number_txt) = argv.get(2) else {
                print_usage();
                std::process::exit(1);
            };
            let Ok(number) = number_txt.parse::<i64>() else {
                eprintln!("Invalid column number: {number_txt}");
                std::process::exit(2);
            };
            let metas = exact_meta_for_column(&words, number);
            if metas.is_empty() {
                eprintln!("Unknown column: {number}");
                std::process::exit(2);
            }
            for meta in metas {
                println!(
                    "{} => {} / {} | main_aliases=[{}] | parameter_aliases=[{}]",
                    meta.column_number,
                    meta.parameter_main,
                    meta.parameter,
                    meta.parameter_main_aliases.join(", "),
                    meta.parameter_aliases.join(", "),
                );
            }
            if let Some(summary) = alias_summary_for_column(&words, number) {
                println!("summary_pairs={:?}", summary.canonical_pairs);
            }
        }
        "html" => {
            let Some(number_txt) = argv.get(2) else {
                print_usage();
                std::process::exit(1);
            };
            let Ok(number) = number_txt.parse::<i64>() else {
                eprintln!("Invalid column number: {number_txt}");
                std::process::exit(2);
            };
            let Some(meta) = html_meta_for_column(&words, number) else {
                eprintln!("Unknown column: {number}");
                std::process::exit(2);
            };
            println!("classes={}", meta.classes.join(" "));
            for (k, v) in meta.data_attributes {
                println!("{}={}", k, v);
            }
        }
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}
