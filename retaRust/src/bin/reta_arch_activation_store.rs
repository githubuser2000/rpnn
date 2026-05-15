use std::{env, fs};

fn main() {
    let mut legacy_lines_file: Option<String> = None;
    let mut store_file: Option<String> = None;
    let mut reta_args = Vec::new();
    let mut iter = env::args().skip(1).peekable();
    while let Some(arg) = iter.next() {
        if arg == "--legacy-lines-file" {
            legacy_lines_file = iter.next();
        } else if let Some(path) = arg.strip_prefix("--legacy-lines-file=") {
            legacy_lines_file = Some(path.to_string());
        } else if arg == "--store-file" {
            store_file = iter.next();
        } else if let Some(path) = arg.strip_prefix("--store-file=") {
            store_file = Some(path.to_string());
        } else {
            reta_args.push(arg);
        }
    }

    if reta_args.is_empty() {
        reta_args.push("reta".to_string());
    } else {
        let first = std::path::Path::new(&reta_args[0])
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or(reta_args[0].as_str());
        if first != "reta" && first != "reta.exe" {
            reta_args.insert(0, "reta".to_string());
        }
    }

    let legacy_lines = match legacy_lines_file.as_ref() {
        Some(path) => match fs::read_to_string(path) {
            Ok(text) => text.lines().map(ToString::to_string).collect::<Vec<_>>(),
            Err(error) => {
                eprintln!("failed to read legacy lines file {path:?}: {error}");
                std::process::exit(2);
            }
        },
        None => Vec::new(),
    };

    let (_, switch_config) =
        reta_architecture::extract_architecture_switch_from_argv(&reta_args, None);
    let policy = reta_architecture::TableViewActivationStorePolicy::default();
    let store = reta_architecture::activation_store_for_cli_args(
        &reta_args,
        &legacy_lines,
        &switch_config,
        &policy,
    );
    let parsed_from_generated = reta_architecture::parse_activation_store_text(
        &store.store_text,
        &legacy_lines,
        store.latest_transaction_id.as_deref(),
        &policy,
    );
    let parsed_from_file = match store_file.as_ref() {
        Some(path) => match fs::read_to_string(path) {
            Ok(text) => Some(reta_architecture::parse_activation_store_text(
                &text,
                &legacy_lines,
                store.latest_transaction_id.as_deref(),
                &policy,
            )),
            Err(error) => {
                eprintln!("failed to read activation store file {path:?}: {error}");
                std::process::exit(2);
            }
        },
        None => None,
    };

    let out = serde_json::json!({
        "args": reta_args,
        "legacy_line_count": legacy_lines.len(),
        "store_file": store_file,
        "policy": policy,
        "store": store,
        "parsed_from_generated": parsed_from_generated,
        "parsed_from_file": parsed_from_file,
    });
    match serde_json::to_string_pretty(&out) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("failed to serialize table-view activation store: {error}");
            std::process::exit(1);
        }
    }
}
