use std::{env, fs};

fn main() {
    let mut legacy_lines_file: Option<String> = None;
    let mut reta_args = Vec::new();
    let mut iter = env::args().skip(1).peekable();
    while let Some(arg) = iter.next() {
        if arg == "--legacy-lines-file" {
            legacy_lines_file = iter.next();
        } else if let Some(path) = arg.strip_prefix("--legacy-lines-file=") {
            legacy_lines_file = Some(path.to_string());
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

    let parsed = reta_architecture::bootstrap_parameter_runtime().parse_cli_args(&reta_args);
    let mode = parsed.selected_output_mode.unwrap_or(reta_architecture::OutputMode::Shell);
    let output_report = reta_architecture::render_table_view_for_cli_args(
        &reta_args,
        &reta_architecture::TableMaterializationConfig::from_cli_args(&reta_args),
        &reta_architecture::TableViewOutputConfig::default().with_mode(mode),
    );
    let parity = reta_architecture::compare_table_view_output_to_legacy(
        &output_report,
        &legacy_lines,
        &reta_architecture::TableViewOutputParityConfig::default().with_mode(mode),
    );
    let out = serde_json::json!({
        "args": reta_args,
        "mode": mode.canonical_name(),
        "legacy_line_count": legacy_lines.len(),
        "output_report": output_report,
        "parity": parity,
    });
    match serde_json::to_string_pretty(&out) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("failed to serialize table view output parity report: {error}");
            std::process::exit(1);
        }
    }
}
