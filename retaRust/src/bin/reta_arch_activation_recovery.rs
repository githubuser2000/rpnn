use std::{env, fs};

fn main() {
    let mut legacy_lines_file: Option<String> = None;
    let mut recovery_file: Option<String> = None;
    let mut allow_replay = false;
    let mut reta_args = Vec::new();
    let mut iter = env::args().skip(1).peekable();
    while let Some(arg) = iter.next() {
        if arg == "--legacy-lines-file" {
            legacy_lines_file = iter.next();
        } else if let Some(path) = arg.strip_prefix("--legacy-lines-file=") {
            legacy_lines_file = Some(path.to_string());
        } else if arg == "--activation-recovery-file"
            || arg == "--activation-recover-file"
            || arg == "--activation-store-recover"
            || arg == "--reta-arch-recovery-file"
            || arg == "--reta-arch-recover-file"
        {
            recovery_file = iter.next();
        } else if let Some(path) = arg.strip_prefix("--activation-recovery-file=") {
            recovery_file = Some(path.to_string());
        } else if let Some(path) = arg.strip_prefix("--activation-recover-file=") {
            recovery_file = Some(path.to_string());
        } else if let Some(path) = arg.strip_prefix("--activation-store-recover=") {
            recovery_file = Some(path.to_string());
        } else if let Some(path) = arg.strip_prefix("--reta-arch-recovery-file=") {
            recovery_file = Some(path.to_string());
        } else if let Some(path) = arg.strip_prefix("--reta-arch-recover-file=") {
            recovery_file = Some(path.to_string());
        } else if arg == "--activation-recovery-allow-replay" {
            allow_replay = true;
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

    if let Some(path) = recovery_file.as_ref() {
        reta_args.push(format!("--activation-recovery-file={path}"));
    }
    if allow_replay {
        reta_args.push("--activation-recovery-allow-replay".to_string());
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
    let (policy, enabled_from_cli) = reta_architecture::activation_recovery_policy_from_cli_args(
        &reta_args,
        &reta_architecture::TableViewActivationRecoveryPolicy::default(),
    );
    let report = reta_architecture::activation_recovery_for_cli_args(
        &reta_args,
        &legacy_lines,
        &switch_config,
        &policy,
    );

    let out = serde_json::json!({
        "args": reta_args,
        "legacy_line_count": legacy_lines.len(),
        "enabled_from_cli_or_env": enabled_from_cli,
        "policy": policy,
        "report": report,
    });
    match serde_json::to_string_pretty(&out) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("failed to serialize activation recovery report: {error}");
            std::process::exit(1);
        }
    }
}
