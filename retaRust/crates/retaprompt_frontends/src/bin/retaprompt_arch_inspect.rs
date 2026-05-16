fn legacy_prompt_command_snapshot(
    command: &retaprompt_commands::PromptCommand,
) -> retaprompt_input::reta_architecture::ShadowPromptLegacyCommand {
    match command {
        retaprompt_commands::PromptCommand::Reta(argv) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::reta(argv.clone())
        }
        retaprompt_commands::PromptCommand::RetaBatch(argvs) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::reta_batch(argvs.clone())
        }
        retaprompt_commands::PromptCommand::Sequence(commands) => {
            let description = format!("legacy_prompt_command_sequence_len_{}", commands.len());
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "sequence",
                description,
            )
        }
        retaprompt_commands::PromptCommand::Noop => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "noop",
                "legacy_prompt_command_noop",
            )
        }
        retaprompt_commands::PromptCommand::Exit => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "exit",
                "legacy_prompt_command_exit",
            )
        }
        retaprompt_commands::PromptCommand::SaveBefore => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "save_before",
                "legacy_prompt_command_save_before",
            )
        }
        retaprompt_commands::PromptCommand::SaveAfter => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "save_after",
                "legacy_prompt_command_save_after",
            )
        }
        retaprompt_commands::PromptCommand::StoreCurrentInput(_) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "store_current_input",
                "legacy_prompt_command_store_current_input",
            )
        }
        retaprompt_commands::PromptCommand::StoreInline(_) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "store_inline",
                "legacy_prompt_command_store_inline",
            )
        }
        retaprompt_commands::PromptCommand::DeleteStoredStart => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "delete_stored_start",
                "legacy_prompt_command_delete_stored_start",
            )
        }
        retaprompt_commands::PromptCommand::DeleteStoredSelection(_) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "delete_stored_selection",
                "legacy_prompt_command_delete_stored_selection",
            )
        }
        retaprompt_commands::PromptCommand::EnterStoredOutputMode(_) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "enter_stored_output_mode",
                "legacy_prompt_command_enter_stored_output_mode",
            )
        }
        retaprompt_commands::PromptCommand::ShowStored(_) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "show_stored",
                "legacy_prompt_command_show_stored",
            )
        }
        retaprompt_commands::PromptCommand::Clear => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "clear",
                "legacy_prompt_command_clear",
            )
        }
        retaprompt_commands::PromptCommand::LaunchUi => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "launch_ui",
                "legacy_prompt_command_launch_ui",
            )
        }
        retaprompt_commands::PromptCommand::PrintHelp => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "print_help",
                "legacy_prompt_command_print_help",
            )
        }
        retaprompt_commands::PromptCommand::PrintCommands => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "print_commands",
                "legacy_prompt_command_print_commands",
            )
        }
        retaprompt_commands::PromptCommand::PrintHistory => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "print_history",
                "legacy_prompt_command_print_history",
            )
        }
        retaprompt_commands::PromptCommand::SwitchMode(_) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "switch_mode",
                "legacy_prompt_command_switch_mode",
            )
        }
        retaprompt_commands::PromptCommand::ToggleLogging(_) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "toggle_logging",
                "legacy_prompt_command_toggle_logging",
            )
        }
        retaprompt_commands::PromptCommand::Shell(_) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "shell",
                "legacy_prompt_command_shell",
            )
        }
        retaprompt_commands::PromptCommand::Python(_) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "python",
                "legacy_prompt_command_python",
            )
        }
        retaprompt_commands::PromptCommand::Math(_) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "math",
                "legacy_prompt_command_math",
            )
        }
        retaprompt_commands::PromptCommand::Immediate(_) => {
            retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
                "immediate",
                "legacy_prompt_command_immediate",
            )
        }
    }
}

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    let (cleaned_argv, switch_config) =
        retaprompt_input::reta_architecture::extract_architecture_switch_from_argv(&argv, None);
    let program_name = cleaned_argv
        .first()
        .map(|arg| {
            std::path::Path::new(arg)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(arg)
                .to_string()
        })
        .unwrap_or_else(|| "rp".to_string());
    let prompt_text = cleaned_argv.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
    let pipeline = retaprompt_input::reta_architecture::bootstrap_shadow_pipeline();
    let report = pipeline.shadow_prompt(
        &retaprompt_input::reta_architecture::ShadowPromptInput::new(
            program_name.clone(),
            prompt_text.clone(),
        ),
        &switch_config,
    );
    let legacy = match retaprompt_commands::compile_for_rp(&prompt_text) {
        Ok(command) => legacy_prompt_command_snapshot(&command),
        Err(error) => retaprompt_input::reta_architecture::ShadowPromptLegacyCommand::other(
            "compile_error",
            error,
        ),
    };
    let prompt_commit_policy = retaprompt_input::reta_architecture::ShadowPromptCommitPolicy::from_cli_args(&cleaned_argv);
    let commit = retaprompt_input::reta_architecture::evaluate_shadow_prompt_commit(
        &report,
        &legacy,
        &switch_config,
        &prompt_commit_policy,
    );
    let json = serde_json::json!({
        "program_name": program_name,
        "prompt_text": prompt_text,
        "cleaned_args": cleaned_argv,
        "switch": switch_config.snapshot(),
        "shadow_report": report,
        "legacy": legacy,
        "prompt_commit_policy": prompt_commit_policy,
        "commit": commit,
    });
    match serde_json::to_string_pretty(&json) {
        Ok(text) => println!("{text}"),
        Err(error) => {
            eprintln!("rretaprompt_arch_inspect could not serialize prompt plan: {error}");
            std::process::exit(2);
        }
    }
}
