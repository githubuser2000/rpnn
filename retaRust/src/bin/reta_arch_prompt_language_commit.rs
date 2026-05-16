use serde::Serialize;

#[derive(Serialize)]
struct PromptLanguageCommitInspect {
    input: String,
    switch: reta_architecture::ArchitectureSwitchSnapshot,
    report: reta_architecture::ShadowPromptReport,
    legacy: reta_architecture::ShadowPromptLegacyCommand,
    policy: reta_architecture::ShadowPromptCommitPolicy,
    commit: reta_architecture::ShadowPromptCommitDecision,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_args = std::env::args().collect::<Vec<_>>();
    let (clean_args, switch_config) = reta_architecture::extract_architecture_switch_from_argv(&raw_args, None);
    let mut input_parts = Vec::new();
    let mut iter = clean_args.iter().skip(1).peekable();
    while let Some(arg) = iter.next() {
        if arg == "--prompt-text" || arg == "--input" {
            if let Some(value) = iter.next() {
                input_parts.push(value.to_string());
            }
        } else if let Some(value) = arg.strip_prefix("--prompt-text=").or_else(|| arg.strip_prefix("--input=")) {
            input_parts.push(value.to_string());
        } else {
            input_parts.push(arg.to_string());
        }
    }
    let input = if input_parts.is_empty() {
        "reta -language=english -spalten --kontinuum=m".to_string()
    } else {
        input_parts.join(" ")
    };
    let pipeline = reta_architecture::bootstrap_shadow_pipeline();
    let report = pipeline.shadow_prompt(
        &reta_architecture::ShadowPromptInput::new("rreta_arch_prompt_language_commit", input.clone()),
        &switch_config,
    );
    let legacy = reta_architecture::ShadowPromptLegacyCommand::reta(report.planned_argv.clone());
    let policy = reta_architecture::ShadowPromptCommitPolicy::from_cli_args(&clean_args);
    let commit = reta_architecture::evaluate_shadow_prompt_commit(&report, &legacy, &switch_config, &policy);
    println!(
        "{}",
        serde_json::to_string_pretty(&PromptLanguageCommitInspect {
            input,
            switch: switch_config.snapshot(),
            report,
            legacy,
            policy,
            commit,
        })?
    );
    Ok(())
}
