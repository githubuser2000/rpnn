use serde::Serialize;

#[derive(Serialize)]
struct PromptActivationReadinessInspect {
    input: String,
    switch: reta_architecture::ArchitectureSwitchSnapshot,
    prompt_report: reta_architecture::ShadowPromptReport,
    legacy: reta_architecture::ShadowPromptLegacyCommand,
    prompt_commit_policy: reta_architecture::ShadowPromptCommitPolicy,
    prompt_commit: reta_architecture::ShadowPromptCommitDecision,
    readiness_policy: reta_architecture::PromptActivationReadinessPolicy,
    readiness: reta_architecture::PromptActivationReadinessReport,
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
    let prompt_report = pipeline.shadow_prompt(
        &reta_architecture::ShadowPromptInput::new("rreta_arch_prompt_activation_readiness", input.clone()),
        &switch_config,
    );
    let legacy = reta_architecture::ShadowPromptLegacyCommand::reta(prompt_report.planned_argv.clone());
    let prompt_commit_policy = reta_architecture::ShadowPromptCommitPolicy::from_cli_args(&clean_args);
    let prompt_commit = reta_architecture::evaluate_shadow_prompt_commit(
        &prompt_report,
        &legacy,
        &switch_config,
        &prompt_commit_policy,
    );
    let readiness_policy = reta_architecture::PromptActivationReadinessPolicy::from_cli_args(&clean_args);
    let readiness = reta_architecture::prompt_activation_readiness_from_reports(
        &prompt_report,
        &legacy,
        &prompt_commit,
        &readiness_policy,
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&PromptActivationReadinessInspect {
            input,
            switch: switch_config.snapshot(),
            prompt_report,
            legacy,
            prompt_commit_policy,
            prompt_commit,
            readiness_policy,
            readiness,
        })?
    );
    Ok(())
}
