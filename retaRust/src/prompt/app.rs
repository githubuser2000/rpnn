use std::borrow::Cow;
use std::path::PathBuf;

use reedline::{
    default_emacs_keybindings, default_vi_insert_keybindings, default_vi_normal_keybindings,
    ColumnarMenu, DefaultValidator, EditCommand, EditMode, Emacs, KeyCode, KeyModifiers, Keybindings,
    MenuBuilder, Prompt, PromptEditMode, PromptHistorySearch, Reedline,
    ReedlineEvent, ReedlineMenu, ReedlineRawEvent, Signal, Vi,
};

use super::commands::{
    compile_command_with_state, execute_command, help_text, take_auto_prompt_command, EditModeKind,
    PromptCommand, PromptOutput, SessionState,
};
use super::completion::{
    build_default_completer_with_runtime, build_default_hinter_with_runtime_and_right_state,
    new_completion_runtime_handle, new_right_arrow_autosuggest_state,
    set_completion_runtime_context, CompletionRuntimeHandle, RightArrowAcceptAction,
    RightArrowAutosuggestSnapshot, RightArrowAutosuggestState,
};
use super::frontend_profile::PromptFrontendProfile;
use super::history::{
    default_history_path, default_log_path, should_append_history_string_like_python,
    should_scrub_history_string_after_reedline_append_like_python, PromptToolkitFileHistory,
};
use super::preset::PromptFrontendPreset;
use super::python_like::{libreta_prompt_custom_split, PromptModus};
use super::tui::launch_preview_ui;

#[derive(Clone, Debug)]
struct StartupArgs {
    start_with_vi_mode: bool,
    logging_enabled: bool,
    one_shot: Option<bool>,
    show_help: bool,
    exact_mode: bool,
    command_text: Option<String>,
    trailing_args: Vec<String>,
}

#[derive(Clone, Debug)]
struct RpPrompt {
    text: String,
}

const COMPLETION_MENU_NAME: &str = "completion_menu";

impl Prompt for RpPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        Cow::Owned(self.text.clone())
    }

    fn render_prompt_right(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _prompt_mode: PromptEditMode) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        Cow::Borrowed("... ")
    }

    fn render_prompt_history_search_indicator(
        &self,
        _history_search: PromptHistorySearch,
    ) -> Cow<'_, str> {
        Cow::Borrowed("(history) ")
    }
}

fn prompt_placeholder_text(state: &SessionState) -> Option<String> {
    if !state.has_stored_placeholder() {
        return None;
    }

    let text = state.stored_placeholder.trim();
    if text.is_empty() {
        return None;
    }

    const MAX_PROMPT_PLACEHOLDER_CHARS: usize = 72;
    let mut shortened = text
        .chars()
        .take(MAX_PROMPT_PLACEHOLDER_CHARS)
        .collect::<String>();
    if text.chars().count() > MAX_PROMPT_PLACEHOLDER_CHARS {
        shortened.push('…');
    }
    Some(shortened)
}

fn prompt_text_for_state(state: &SessionState) -> String {
    match state.prompt_mode {
        PromptModus::Speichern => "was speichern> ".to_string(),
        PromptModus::LoeschenStart | PromptModus::LoeschenSelect => "was löschen> ".to_string(),
        PromptModus::Normal | PromptModus::AusgabeSelektiv => {
            if let Some(placeholder) = prompt_placeholder_text(state) {
                format!("{placeholder} > ")
            } else {
                "> ".to_string()
            }
        }
        PromptModus::SpeicherungAusgaben | PromptModus::SpeicherungAusgabenMitZusatz => {
            "o> ".to_string()
        }
    }
}

fn parse_startup_args(argv: &[String], preset: &PromptFrontendPreset) -> StartupArgs {
    let mut startup = StartupArgs {
        start_with_vi_mode: preset.start_with_vi_mode,
        logging_enabled: preset.implicit_logging,
        one_shot: None,
        show_help: false,
        exact_mode: preset.default_exact_mode,
        command_text: None,
        trailing_args: Vec::new(),
    };

    let mut index = 1usize;
    while index < argv.len() {
        match argv[index].as_str() {
            "-vi" => {
                startup.start_with_vi_mode = true;
                index += 1;
            }
            "-log" => {
                startup.logging_enabled = true;
                index += 1;
            }
            "-e" => {
                startup.exact_mode = true;
                index += 1;
            }
            "-h" | "-help" | "--help" => {
                startup.show_help = true;
                index += 1;
            }
            "-debug" => {
                index += 1;
            }
            "-befehl" | "-command" => {
                startup.one_shot = Some(true);
                startup.command_text = Some(argv[index + 1..].join(" "));
                return finalize_startup_args(startup);
            }
            _ => {
                startup.trailing_args = argv[index..].to_vec();
                break;
            }
        }
    }

    if startup.command_text.is_none() && preset.one_shot && !startup.trailing_args.is_empty() {
        startup.command_text = Some(startup.trailing_args.join(" "));
    }

    finalize_startup_args(startup)
}

fn finalize_startup_args(mut startup: StartupArgs) -> StartupArgs {
    if startup.exact_mode {
        if let Some(command_text) = startup.command_text.take() {
            startup.command_text = Some(apply_exact_mode_to_input(&command_text));
        }
    }
    startup
}

fn apply_exact_mode_to_input(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if !should_append_exact_suffix(trimmed) {
        return trimmed.to_string();
    }

    format!("{trimmed} keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar")
}

fn input_starts_with_reta(input: &str) -> bool {
    matches!(
        libreta_prompt_custom_split(input.trim()).first(),
        Some(token) if token == "reta"
    )
}

fn rpe_output_group() -> Vec<String> {
    vec![
        "-ausgabe".to_string(),
        "--art=emacs".to_string(),
        "--keineueberschriften".to_string(),
    ]
}

fn apply_rpe_emacs_output_to_argv(
    mut argv: Vec<String>,
    append_after_user_args: bool,
) -> Vec<String> {
    let output_group = rpe_output_group();

    if argv.is_empty() {
        return output_group;
    }

    if append_after_user_args {
        argv.extend(output_group);
        argv
    } else {
        let mut rebuilt = vec![argv[0].clone()];
        rebuilt.extend(output_group);
        rebuilt.extend(argv.into_iter().skip(1));
        rebuilt
    }
}

fn apply_rpe_emacs_output_to_command(command: PromptCommand, input: &str) -> PromptCommand {
    let append_after_user_args = input_starts_with_reta(input);

    match command {
        PromptCommand::Reta(argv) => {
            PromptCommand::Reta(apply_rpe_emacs_output_to_argv(argv, append_after_user_args))
        }
        PromptCommand::RetaBatch(argvs) => PromptCommand::RetaBatch(
            argvs
                .into_iter()
                .map(|argv| apply_rpe_emacs_output_to_argv(argv, append_after_user_args))
                .collect(),
        ),
        PromptCommand::Sequence(commands) => PromptCommand::Sequence(
            commands
                .into_iter()
                .map(|command| apply_rpe_emacs_output_to_command(command, input))
                .collect(),
        ),
        other => other,
    }
}

fn architecture_prompt_modus(mode: PromptModus) -> reta_architecture::PromptModus {
    match mode {
        PromptModus::Normal => reta_architecture::PromptModus::Normal,
        PromptModus::Speichern => reta_architecture::PromptModus::Speichern,
        PromptModus::LoeschenStart => reta_architecture::PromptModus::LoeschenStart,
        PromptModus::SpeicherungAusgaben => reta_architecture::PromptModus::SpeicherungAusgaben,
        PromptModus::LoeschenSelect => reta_architecture::PromptModus::LoeschenSelect,
        PromptModus::SpeicherungAusgabenMitZusatz => {
            reta_architecture::PromptModus::SpeicherungAusgabenMitZusatz
        }
        PromptModus::AusgabeSelektiv => reta_architecture::PromptModus::AusgabeSelektiv,
    }
}

fn legacy_prompt_command_snapshot(command: &PromptCommand) -> reta_architecture::ShadowPromptLegacyCommand {
    match command {
        PromptCommand::Reta(argv) => reta_architecture::ShadowPromptLegacyCommand::reta(argv.clone()),
        PromptCommand::RetaBatch(argvs) => {
            reta_architecture::ShadowPromptLegacyCommand::reta_batch(argvs.clone())
        }
        PromptCommand::Sequence(commands) => {
            let description = format!("legacy_prompt_command_sequence_len_{}", commands.len());
            reta_architecture::ShadowPromptLegacyCommand::other("sequence", description)
        }
        PromptCommand::Noop => reta_architecture::ShadowPromptLegacyCommand::other("noop", "legacy_prompt_command_noop"),
        PromptCommand::Exit => reta_architecture::ShadowPromptLegacyCommand::other("exit", "legacy_prompt_command_exit"),
        PromptCommand::SaveBefore => reta_architecture::ShadowPromptLegacyCommand::other("save_before", "legacy_prompt_command_save_before"),
        PromptCommand::SaveAfter => reta_architecture::ShadowPromptLegacyCommand::other("save_after", "legacy_prompt_command_save_after"),
        PromptCommand::StoreCurrentInput(_) => reta_architecture::ShadowPromptLegacyCommand::other("store_current_input", "legacy_prompt_command_store_current_input"),
        PromptCommand::StoreInline(_) => reta_architecture::ShadowPromptLegacyCommand::other("store_inline", "legacy_prompt_command_store_inline"),
        PromptCommand::DeleteStoredStart => reta_architecture::ShadowPromptLegacyCommand::other("delete_stored_start", "legacy_prompt_command_delete_stored_start"),
        PromptCommand::DeleteStoredSelection(_) => reta_architecture::ShadowPromptLegacyCommand::other("delete_stored_selection", "legacy_prompt_command_delete_stored_selection"),
        PromptCommand::EnterStoredOutputMode(_) => reta_architecture::ShadowPromptLegacyCommand::other("enter_stored_output_mode", "legacy_prompt_command_enter_stored_output_mode"),
        PromptCommand::ShowStored(_) => reta_architecture::ShadowPromptLegacyCommand::other("show_stored", "legacy_prompt_command_show_stored"),
        PromptCommand::Clear => reta_architecture::ShadowPromptLegacyCommand::other("clear", "legacy_prompt_command_clear"),
        PromptCommand::LaunchUi => reta_architecture::ShadowPromptLegacyCommand::other("launch_ui", "legacy_prompt_command_launch_ui"),
        PromptCommand::PrintHelp => reta_architecture::ShadowPromptLegacyCommand::other("print_help", "legacy_prompt_command_print_help"),
        PromptCommand::PrintCommands => reta_architecture::ShadowPromptLegacyCommand::other("print_commands", "legacy_prompt_command_print_commands"),
        PromptCommand::PrintHistory => reta_architecture::ShadowPromptLegacyCommand::other("print_history", "legacy_prompt_command_print_history"),
        PromptCommand::SwitchMode(_) => reta_architecture::ShadowPromptLegacyCommand::other("switch_mode", "legacy_prompt_command_switch_mode"),
        PromptCommand::ToggleLogging(_) => reta_architecture::ShadowPromptLegacyCommand::other("toggle_logging", "legacy_prompt_command_toggle_logging"),
        PromptCommand::Shell(_) => reta_architecture::ShadowPromptLegacyCommand::other("shell", "legacy_prompt_command_shell"),
        PromptCommand::Python(_) => reta_architecture::ShadowPromptLegacyCommand::other("python", "legacy_prompt_command_python"),
        PromptCommand::Math(_) => reta_architecture::ShadowPromptLegacyCommand::other("math", "legacy_prompt_command_math"),
        PromptCommand::Immediate(_) => reta_architecture::ShadowPromptLegacyCommand::other("immediate", "legacy_prompt_command_immediate"),
    }
}

fn prompt_shadow_input_for_state(
    input: &str,
    state: &SessionState,
) -> reta_architecture::ShadowPromptInput {
    reta_architecture::ShadowPromptInput {
        program_name: state.program_name.clone(),
        prompt_text: input.to_string(),
        placeholder: state.stored_placeholder.clone(),
        prompt_mode: architecture_prompt_modus(state.prompt_mode),
    }
}

fn apply_prompt_shadow_commit_if_safe(
    compiled: PromptCommand,
    input: &str,
    log_path: &PathBuf,
    state: &SessionState,
    architecture_switch_config: &reta_architecture::ArchitectureSwitchConfig,
) -> PromptCommand {
    if !architecture_switch_config.mode.should_shadow_execute()
        && !architecture_switch_config.visible_behaviour_may_change()
    {
        return compiled;
    }

    let pipeline = reta_architecture::bootstrap_shadow_pipeline();
    let shadow_input = prompt_shadow_input_for_state(input, state);
    let report = pipeline.shadow_prompt(&shadow_input, architecture_switch_config);
    let legacy = legacy_prompt_command_snapshot(&compiled);
    let commit = pipeline.prompt_commit_decision(&report, &legacy, architecture_switch_config);

    if state.logging_enabled || architecture_switch_config.trace {
        append_log_line(
            log_path,
            "ARCH_PROMPT_SHADOW",
            &format!(
                "mode={} legacy={} planned_argv={} commit={} reason={} same_argv={} language_guard_ready={} language={}",
                report.switch_mode,
                legacy.kind,
                report.planned_argv.len(),
                commit.use_shadow_prompt_plan,
                commit.reason,
                commit.same_argv,
                commit.prompt_language_guard_ready,
                commit.prompt_language_guard_language
            ),
        );
    }

    if commit.use_shadow_prompt_plan {
        PromptCommand::Reta(report.planned_argv)
    } else {
        compiled
    }
}

fn should_append_exact_suffix(input: &str) -> bool {
    let tokens = libreta_prompt_custom_split(input);

    if tokens.is_empty() {
        return false;
    }

    let first = tokens[0].as_str();

    if first == "reta" {
        return false;
    }
    if first.starts_with('-') {
        return false;
    }
    if first.starts_with(':') {
        return false;
    }

    if tokens.iter().any(|token| {
        matches!(
            token.as_str(),
            "s" | "S"
                | "l"
                | "o"
                | "BefehlSpeichernDavor"
                | "BefehlSpeichernDanach"
                | "BefehlSpeicherungLöschen"
                | "BefehlSpeicherungAusgeben"
        )
    }) {
        return false;
    }

    !matches!(
        first,
        "HELP"
            | "help"
            | "hilfe"
            | "befehle"
            | "kurzbefehle"
            | "shell"
            | "python"
            | "math"
            | "q"
            | "quit"
            | "exit"
            | "ende"
            | "clear"
            | "leeren"
            | "loggen"
            | "nichtloggen"
    )
}

fn should_record_prompt_history(input: &str) -> bool {
    should_append_history_string_like_python(true, input)
}

fn record_prompt_input(state: &mut SessionState, input: &str) {
    if state.logging_enabled && should_record_prompt_history(input) {
        state.history_lines.push(input.to_string());
    }
}

fn append_prompt_input_log(path: &PathBuf, state: &SessionState, input: &str) {
    if state.logging_enabled && should_record_prompt_history(input) {
        append_log_line(path, "input", input);
    }
}

fn scrub_prompt_history_line_if_python_togglehistory_would_skip(
    history_path: &PathBuf,
    logging_enabled: bool,
    input: &str,
) -> bool {
    if !should_scrub_history_string_after_reedline_append_like_python(logging_enabled, input) {
        return false;
    }

    remove_last_history_line_matching(history_path, input)
}

fn remove_last_history_line_matching(history_path: &PathBuf, input: &str) -> bool {
    let Ok(raw) = std::fs::read_to_string(history_path) else {
        return false;
    };

    let wanted = input.trim();
    if wanted.is_empty() {
        return false;
    }

    let had_trailing_newline = raw.ends_with('\n');
    let mut lines = raw.lines().map(str::to_string).collect::<Vec<_>>();
    let Some(index) = lines.iter().rposition(|line| line.trim() == wanted) else {
        return false;
    };
    lines.remove(index);

    let mut rebuilt = lines.join("\n");
    if had_trailing_newline && !rebuilt.is_empty() {
        rebuilt.push('\n');
    }
    std::fs::write(history_path, rebuilt).is_ok()
}

#[allow(non_snake_case)]
fn promptInput(
    state: &mut SessionState,
    input: &str,
    history_path: Option<&PathBuf>,
    log_path: &PathBuf,
) -> bool {
    state.previous_input = state.last_input.clone();
    state.last_input = input.to_string();
    record_prompt_input(state, input);
    let history_line_scrubbed = history_path
        .map(|history_path| {
            scrub_prompt_history_line_if_python_togglehistory_would_skip(
                history_path,
                state.logging_enabled,
                input,
            )
        })
        .unwrap_or(false);
    append_prompt_input_log(log_path, state, input);
    history_line_scrubbed
}

pub fn run_rp_one_shot(argv: Vec<String>, start_with_vi_mode: bool) -> i32 {
    let program_name = program_name_from_argv(&argv);
    let profile = PromptFrontendProfile::from_program_name(&program_name, start_with_vi_mode);
    let mut preset = PromptFrontendPreset::from_profile_and_argv(profile, &argv);
    preset.one_shot = true;
    run_prompt_frontend_with_preset(argv, preset)
}

pub fn run_prompt_frontend_from_env(fallback_vi_mode: bool) -> i32 {
    let argv = std::env::args().collect::<Vec<_>>();
    run_prompt_frontend(argv, fallback_vi_mode)
}

pub fn run_prompt_frontend_with_profile_from_env(profile: PromptFrontendProfile) -> i32 {
    let argv = std::env::args().collect::<Vec<_>>();
    run_prompt_frontend_with_profile(argv, profile)
}

pub fn run_rp_from_env(start_with_vi_mode: bool) -> i32 {
    run_prompt_frontend_from_env(start_with_vi_mode)
}

pub fn run_prompt_frontend(argv: Vec<String>, fallback_vi_mode: bool) -> i32 {
    let program_name = program_name_from_argv(&argv);
    let profile = PromptFrontendProfile::from_program_name(&program_name, fallback_vi_mode);
    run_prompt_frontend_with_profile(argv, profile)
}

pub fn run_prompt_frontend_with_profile(argv: Vec<String>, profile: PromptFrontendProfile) -> i32 {
    let preset = PromptFrontendPreset::from_profile_and_argv(profile, &argv);
    run_prompt_frontend_with_preset(argv, preset)
}

pub fn run_prompt_input_frontend_with_profile(
    argv: Vec<String>,
    profile: PromptFrontendProfile,
) -> i32 {
    let preset = PromptFrontendPreset::from_profile_and_argv(profile, &argv);
    run_prompt_frontend_with_preset(argv, preset)
}

pub fn run_prompt_command_frontend_with_profile(
    argv: Vec<String>,
    profile: PromptFrontendProfile,
) -> i32 {
    let mut preset = PromptFrontendPreset::from_profile_and_argv(profile, &argv);
    preset.one_shot = true;
    run_prompt_frontend_with_preset(argv, preset)
}

fn run_prompt_frontend_with_preset(argv: Vec<String>, preset: PromptFrontendPreset) -> i32 {
    let program_name = program_name_from_argv(&argv);
    let (architecture_clean_argv, architecture_switch_config) =
        reta_architecture::extract_architecture_switch_from_argv(&argv, None);
    let startup = parse_startup_args(&architecture_clean_argv, &preset);
    let _frontend_architecture = reta_architecture::PromptArchitectureContext::from_prompt_input(
        &program_name,
        startup.command_text.as_deref().unwrap_or(""),
    );

    let mut state = SessionState::new(
        program_name.clone(),
        startup.start_with_vi_mode,
        preset.implicit_logging,
    );
    state.logging_enabled = startup.logging_enabled;

    let history_path = default_history_path(&program_name);
    let log_path = default_log_path(&program_name);

    if startup.show_help {
        print_output(
            &mut state,
            PromptOutput {
                title: "help".to_string(),
                text: help_text(),
                exit_code: 0,
            },
        );
        return 0;
    }

    let effective_one_shot = startup.one_shot.unwrap_or(preset.one_shot);

    if state.logging_enabled {
        append_log_line(
            &log_path,
            "session",
            &format!(
                "start program={} vi_mode={} implicit_logging={} logging_enabled={} one_shot={}",
                program_name,
                state.vi_mode,
                preset.implicit_logging,
                state.logging_enabled,
                effective_one_shot,
            ),
        );
    }

    if effective_one_shot {
        let input = startup
            .command_text
            .unwrap_or_else(|| startup.trailing_args.join(" "));
        return run_one_shot(
            input,
            &log_path,
            preset.emacs_output_mode,
            &mut state,
            &architecture_switch_config,
        );
    }

    run_interactive_loop(
        history_path,
        log_path,
        startup.exact_mode,
        preset.emacs_output_mode,
        preset.persistent_history,
        &mut state,
    )
}

pub fn run_rp(argv: Vec<String>, start_with_vi_mode: bool) -> i32 {
    run_prompt_frontend(argv, start_with_vi_mode)
}

fn program_name_from_argv(argv: &[String]) -> String {
    PathBuf::from(argv.first().cloned().unwrap_or_else(|| "rp".to_string()))
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "rp".to_string())
}

fn run_one_shot(
    input: String,
    log_path: &PathBuf,
    emacs_output_mode: bool,
    state: &mut SessionState,
    architecture_switch_config: &reta_architecture::ArchitectureSwitchConfig,
) -> i32 {
    let _prompt_architecture =
        reta_architecture::PromptArchitectureContext::from_prompt_input(&state.program_name, &input);
    if input.trim().is_empty() {
        let output = PromptOutput {
            title: "usage".to_string(),
            text: format!(
                "{} erwartet einen direkten Befehl als Argument, z. B. 'rpb av12-15' oder 'rpb reta -zeilen --alles'.",
                state.program_name
            ),
            exit_code: 1,
        };
        print_output(state, output.clone());
        return output.exit_code;
    }

    let _ = promptInput(state, &input, None, log_path);

    let compiled = match compile_command_with_state(&input, state) {
        Ok(command) => {
            if emacs_output_mode {
                apply_rpe_emacs_output_to_command(command, &input)
            } else {
                command
            }
        }
        Err(err) => {
            if state.logging_enabled {
                append_log_line(log_path, "compile-error", &err);
            }
            let output = PromptOutput {
                title: "error".to_string(),
                text: err,
                exit_code: 1,
            };
            print_output(state, output.clone());
            return output.exit_code;
        }
    };

    let compiled = apply_prompt_shadow_commit_if_safe(
        compiled,
        &input,
        log_path,
        state,
        architecture_switch_config,
    );

    if matches!(&compiled, PromptCommand::Exit) {
        if state.logging_enabled {
            append_log_line(
                log_path,
                "session",
                "exit command received in one-shot mode",
            );
        }
        return 0;
    }

    if matches!(&compiled, PromptCommand::LaunchUi) {
        let output = PromptOutput {
            title: "ui-error".to_string(),
            text: format!(
                "{} unterstützt keinen interaktiven UI-Start ohne Shellöffnung.",
                state.program_name
            ),
            exit_code: 1,
        };
        print_output(state, output.clone());
        return output.exit_code;
    }

    match execute_command(compiled, state) {
        Ok(Some(output)) => {
            if state.logging_enabled {
                append_log_output(log_path, &output);
            }
            print_output(state, output.clone());
            output.exit_code
        }
        Ok(None) => {
            if let Some(auto_command) = take_auto_prompt_command(state) {
                match execute_command(auto_command, state) {
                    Ok(Some(output)) => {
                        if state.logging_enabled {
                            append_log_output(log_path, &output);
                        }
                        print_output(state, output.clone());
                        output.exit_code
                    }
                    Ok(None) => 0,
                    Err(err) => {
                        if state.logging_enabled {
                            append_log_line(log_path, "execute-error", &err);
                        }
                        let output = PromptOutput {
                            title: "error".to_string(),
                            text: err,
                            exit_code: 1,
                        };
                        print_output(state, output.clone());
                        output.exit_code
                    }
                }
            } else {
                0
            }
        }
        Err(err) => {
            if state.logging_enabled {
                append_log_line(log_path, "execute-error", &err);
            }
            let output = PromptOutput {
                title: "error".to_string(),
                text: err,
                exit_code: 1,
            };
            print_output(state, output.clone());
            output.exit_code
        }
    }
}

fn run_interactive_loop(
    history_path: PathBuf,
    log_path: PathBuf,
    exact_mode_enabled: bool,
    emacs_output_mode: bool,
    persistent_history_allowed: bool,
    state: &mut SessionState,
) -> i32 {
    let completion_runtime = new_completion_runtime_handle();
    let right_arrow_state = new_right_arrow_autosuggest_state();
    set_completion_runtime_context(
        &completion_runtime,
        state.prompt_mode,
        &state.stored_expanded_tokens,
        &state.stored_commands,
    );

    let mut editor = match newSession(
        &history_path,
        state.current_mode(),
        persistent_history_allowed,
        state.logging_enabled,
        state.prompt_mode,
        &completion_runtime,
        right_arrow_state.clone(),
    ) {
        Ok(editor) => editor,
        Err(err) => {
            eprintln!("rp konnte reedline nicht initialisieren: {err}");
            return 1;
        }
    };

    loop {
        if let Some(auto_command) = take_auto_prompt_command(state) {
            let previous_editor_mode = state.current_mode();
            let previous_logging_enabled = state.logging_enabled;
            let previous_prompt_mode = state.prompt_mode;

            match execute_command(auto_command, state) {
                Ok(Some(output)) => {
                    if state.logging_enabled {
                        append_log_output(&log_path, &output);
                    }
                    print_output(state, output);
                }
                Ok(None) => {}
                Err(err) => {
                    if state.logging_enabled {
                        append_log_line(&log_path, "execute-error", &err);
                    }
                    print_output(
                        state,
                        PromptOutput {
                            title: "error".to_string(),
                            text: err,
                            exit_code: 1,
                        },
                    );
                }
            }

            let rebuild_editor = previous_editor_mode != state.current_mode()
                || previous_logging_enabled != state.logging_enabled
                || previous_prompt_mode != state.prompt_mode;

            if rebuild_editor {
                editor = match newSession(
                    &history_path,
                    state.current_mode(),
                    persistent_history_allowed,
                    state.logging_enabled,
                    state.prompt_mode,
                    &completion_runtime,
                    right_arrow_state.clone(),
                ) {
                    Ok(editor) => editor,
                    Err(err) => {
                        eprintln!("rp konnte reedline nicht neu initialisieren: {err}");
                        return 1;
                    }
                };
            }

            continue;
        }

        set_completion_runtime_context(
            &completion_runtime,
            state.prompt_mode,
            &state.stored_expanded_tokens,
            &state.stored_commands,
        );

        let prompt = RpPrompt {
            text: prompt_text_for_state(state),
        };

        match editor.read_line(&prompt) {
            Ok(Signal::Success(buffer)) => {
                let input = buffer.trim().to_string();
                let history_line_scrubbed = promptInput(state, &input, None, &log_path);

                let compile_input = if exact_mode_enabled {
                    apply_exact_mode_to_input(&input)
                } else {
                    input.clone()
                };

                let previous_editor_mode = state.current_mode();
                let previous_logging_enabled = state.logging_enabled;
                let previous_prompt_mode = state.prompt_mode;
                let compiled = match compile_command_with_state(&compile_input, state) {
                    Ok(command) => {
                        if emacs_output_mode {
                            apply_rpe_emacs_output_to_command(command, &compile_input)
                        } else {
                            command
                        }
                    }
                    Err(err) => {
                        if state.logging_enabled {
                            append_log_line(&log_path, "compile-error", &err);
                            append_log_line(&log_path, "ui-error", &err);
                        }
                        print_output(
                            state,
                            PromptOutput {
                                title: "error".to_string(),
                                text: err,
                                exit_code: 1,
                            },
                        );
                        if history_line_scrubbed {
                            editor = match newSession(
                                &history_path,
                                state.current_mode(),
                                persistent_history_allowed,
                                state.logging_enabled,
                                state.prompt_mode,
                                &completion_runtime,
                                right_arrow_state.clone(),
                            ) {
                                Ok(editor) => editor,
                                Err(err) => {
                                    eprintln!("rp konnte reedline nicht neu initialisieren: {err}");
                                    return 1;
                                }
                            };
                        }
                        continue;
                    }
                };

                if matches!(&compiled, PromptCommand::Exit) {
                    if state.logging_enabled {
                        append_log_line(&log_path, "session", "exit command received");
                    }
                    break;
                }

                if matches!(&compiled, PromptCommand::LaunchUi) {
                    if state.logging_enabled {
                        append_log_line(&log_path, "ui", "launch_preview_ui");
                    }
                    if let Err(err) = launch_preview_ui(state) {
                        print_output(
                            state,
                            PromptOutput {
                                title: "ui-error".to_string(),
                                text: format!(
                                    "Die ratatui-Vorschau konnte nicht gestartet werden: {err}"
                                ),
                                exit_code: 1,
                            },
                        );
                    }
                    continue;
                }

                match execute_command(compiled, state) {
                    Ok(Some(output)) => {
                        if state.logging_enabled {
                            append_log_output(&log_path, &output);
                        }
                        print_output(state, output);
                    }
                    Ok(None) => {}
                    Err(err) => {
                        if state.logging_enabled {
                            append_log_line(&log_path, "execute-error", &err);
                        }
                        print_output(
                            state,
                            PromptOutput {
                                title: "error".to_string(),
                                text: err,
                                exit_code: 1,
                            },
                        );
                    }
                }

                let rebuild_editor = previous_editor_mode != state.current_mode()
                    || previous_logging_enabled != state.logging_enabled
                    || previous_prompt_mode != state.prompt_mode
                    || history_line_scrubbed;

                if rebuild_editor {
                    editor = match newSession(
                        &history_path,
                        state.current_mode(),
                        persistent_history_allowed,
                        state.logging_enabled,
                        state.prompt_mode,
                        &completion_runtime,
                        right_arrow_state.clone(),
                    ) {
                        Ok(editor) => editor,
                        Err(err) => {
                            eprintln!("rp konnte reedline nicht neu initialisieren: {err}");
                            return 1;
                        }
                    };
                }
            }
            Ok(Signal::CtrlC) => {
                if state.logging_enabled {
                    append_log_line(&log_path, "signal", "CtrlC");
                }
                println!("^C");
                continue;
            }
            Ok(Signal::CtrlD) => {
                if state.logging_enabled {
                    append_log_line(&log_path, "signal", "CtrlD");
                }
                println!();
                break;
            }
            Ok(other) => {
                print_output(
                    state,
                    PromptOutput {
                        title: "signal".to_string(),
                        text: format!("Nicht direkt behandeltes reedline-Signal: {other:?}"),
                        exit_code: 0,
                    },
                );
            }
            Err(err) => {
                if state.logging_enabled {
                    append_log_line(&log_path, "read-error", &err.to_string());
                }
                eprintln!("Fehler beim Lesen der Eingabe: {err}");
                return 1;
            }
        }
    }

    0
}

fn completion_menu_event(next_event: ReedlineEvent) -> ReedlineEvent {
    ReedlineEvent::UntilFound(vec![
        ReedlineEvent::Menu(COMPLETION_MENU_NAME.to_string()),
        next_event,
    ])
}

fn menu_aware_navigation(
    menu_event: ReedlineEvent,
    fallback_event: ReedlineEvent,
) -> ReedlineEvent {
    ReedlineEvent::UntilFound(vec![menu_event, fallback_event])
}

fn cursor_move_right_event() -> ReedlineEvent {
    ReedlineEvent::Edit(vec![EditCommand::MoveRight { select: false }])
}

fn right_arrow_event(snapshot: RightArrowAutosuggestSnapshot) -> ReedlineEvent {
    if !snapshot.cursor_at_end {
        return cursor_move_right_event();
    }

    match snapshot.accept_action {
        RightArrowAcceptAction::Insert(text) if !text.is_empty() => {
            ReedlineEvent::UntilFound(vec![
                ReedlineEvent::HistoryHintComplete,
                cursor_move_right_event(),
            ])
        }
        RightArrowAcceptAction::ReplaceRange {
            replace_start,
            replace_len,
            replacement,
        } => ReedlineEvent::Edit(vec![
            EditCommand::MoveToPosition {
                position: replace_start,
                select: false,
            },
            EditCommand::ReplaceChars(replace_len, replacement),
        ]),
        _ => cursor_move_right_event(),
    }
}

struct RightAwareEditMode {
    inner: Box<dyn EditMode>,
    right_arrow_state: RightArrowAutosuggestState,
}

impl RightAwareEditMode {
    fn new(inner: Box<dyn EditMode>, right_arrow_state: RightArrowAutosuggestState) -> Self {
        Self {
            inner,
            right_arrow_state,
        }
    }
}

impl EditMode for RightAwareEditMode {
    fn parse_event(&mut self, event: ReedlineRawEvent) -> ReedlineEvent {
        match self.inner.parse_event(event) {
            ReedlineEvent::Right => right_arrow_event(self.right_arrow_state.snapshot()),
            event => event,
        }
    }

    fn edit_mode(&self) -> PromptEditMode {
        self.inner.edit_mode()
    }

}

fn add_completion_keybindings(keybindings: &mut Keybindings) {
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        completion_menu_event(ReedlineEvent::MenuNext),
    );
    keybindings.add_binding(
        KeyModifiers::SHIFT,
        KeyCode::BackTab,
        completion_menu_event(ReedlineEvent::MenuPrevious),
    );
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::BackTab,
        completion_menu_event(ReedlineEvent::MenuPrevious),
    );
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Up,
        menu_aware_navigation(ReedlineEvent::MenuUp, ReedlineEvent::Up),
    );
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Down,
        menu_aware_navigation(ReedlineEvent::MenuDown, ReedlineEvent::Down),
    );
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Left,
        menu_aware_navigation(ReedlineEvent::MenuLeft, ReedlineEvent::Left),
    );
    keybindings.add_binding(KeyModifiers::NONE, KeyCode::Right, ReedlineEvent::Right);
    keybindings.add_binding(
        KeyModifiers::CONTROL,
        KeyCode::Right,
        ReedlineEvent::HistoryHintWordComplete,
    );
    keybindings.add_binding(
        KeyModifiers::ALT,
        KeyCode::Right,
        ReedlineEvent::HistoryHintWordComplete,
    );
}

#[allow(non_snake_case)]
fn newSession(
    history_path: &PathBuf,
    mode: EditModeKind,
    persistent_history_allowed: bool,
    logging_enabled: bool,
    prompt_mode: PromptModus,
    completion_runtime: &CompletionRuntimeHandle,
    right_arrow_state: RightArrowAutosuggestState,
) -> Result<Reedline, String> {
    let history = if persistent_history_allowed {
        PromptToolkitFileHistory::with_file_and_append_policy(
            2_000,
            history_path.clone(),
            logging_enabled,
        )
    } else {
        PromptToolkitFileHistory::in_memory(2_000)
    }
    .map_err(|err| format!("History konnte nicht initialisiert werden: {err}"))?;

    let completion_enabled = !matches!(prompt_mode, PromptModus::LoeschenSelect);
    let mut editor = Reedline::create()
        .with_history(Box::new(history))
        .with_hinter(build_default_hinter_with_runtime_and_right_state(
            completion_runtime.clone(),
            right_arrow_state.clone(),
        ))
        .with_validator(Box::new(DefaultValidator));

    if completion_enabled {
        let completer = build_default_completer_with_runtime(completion_runtime.clone());
        let completion_menu = Box::new(ColumnarMenu::default().with_name(COMPLETION_MENU_NAME));
        editor = editor
            .with_completer(completer)
            .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
            .with_quick_completions(true)
            .with_partial_completions(true);
    }

    editor = match mode {
        EditModeKind::Emacs => {
            let mut keybindings = default_emacs_keybindings();
            if completion_enabled {
                add_completion_keybindings(&mut keybindings);
            }
            let edit_mode = Box::new(RightAwareEditMode::new(
                Box::new(Emacs::new(keybindings)),
                right_arrow_state.clone(),
            ));
            editor.with_edit_mode(edit_mode)
        }
        EditModeKind::Vi => {
            let mut insert_keybindings = default_vi_insert_keybindings();
            let mut normal_keybindings = default_vi_normal_keybindings();
            if completion_enabled {
                add_completion_keybindings(&mut insert_keybindings);
                add_completion_keybindings(&mut normal_keybindings);
            }
            let edit_mode = Box::new(RightAwareEditMode::new(
                Box::new(Vi::new(insert_keybindings, normal_keybindings)),
                right_arrow_state.clone(),
            ));
            editor.with_edit_mode(edit_mode)
        }
    };

    Ok(editor)
}

fn append_log_line(path: &PathBuf, kind: &str, text: &str) {
    if std::env::var_os("RETA_PROMPT_SESSION_LOG").is_none() {
        return;
    }

    use std::io::Write;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = writeln!(file, "[{timestamp}] {kind}: {text}");
    }
}

fn append_log_output(path: &PathBuf, output: &PromptOutput) {
    let headline = format!("title={} exit_code={}", output.title, output.exit_code);
    append_log_line(path, "output-meta", &headline);
    if !output.text.is_empty() {
        append_log_line(path, "output-text", &output.text);
    }
}

fn print_output(state: &mut SessionState, output: PromptOutput) {
    state.last_output = output.clone();
    if !output.text.is_empty() {
        println!("{}", output.text);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        apply_exact_mode_to_input, apply_rpe_emacs_output_to_command, record_prompt_input,
        remove_last_history_line_matching,
        scrub_prompt_history_line_if_python_togglehistory_would_skip, should_append_exact_suffix,
        should_record_prompt_history, PromptCommand, SessionState,
    };

    #[test]
    fn toggle_history_commands_are_filtered_like_python_togglehistory() {
        assert!(!should_record_prompt_history("loggen"));
        assert!(!should_record_prompt_history("nichtloggen"));
        assert!(!should_record_prompt_history("12 loggen"));
        assert!(should_record_prompt_history("12 emotion"));
    }

    #[test]
    fn prompt_input_history_respects_python_logging_switch() {
        let mut rp_state = SessionState::new("rp".to_string(), true, false);
        record_prompt_input(&mut rp_state, "12 emotion");
        assert!(rp_state.history_lines.is_empty());

        let mut rpl_state = SessionState::new("rpl".to_string(), true, true);
        record_prompt_input(&mut rpl_state, "12 emotion");
        record_prompt_input(&mut rpl_state, "nichtloggen");
        assert_eq!(rpl_state.history_lines, vec!["12 emotion".to_string()]);
    }

    #[test]
    fn history_scrub_removes_only_the_just_appended_matching_line() {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "reta_prompt_history_scrub_{}_{}.txt",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        std::fs::write(&path, "12 emotion\nnichtloggen\n12 emotion\n").unwrap();
        assert!(
            scrub_prompt_history_line_if_python_togglehistory_would_skip(
                &path,
                false,
                "12 emotion"
            )
        );
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "12 emotion\nnichtloggen\n"
        );

        assert!(remove_last_history_line_matching(&path, "nichtloggen"));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "12 emotion\n");
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn rpb_exact_mode_appends_suffix_to_prompt_numbers_not_raw_reta() {
        assert_eq!(
            apply_exact_mode_to_input("12"),
            "12 keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
        );
        assert_eq!(
            apply_exact_mode_to_input("2/3"),
            "2/3 keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
        );
        assert_eq!(
            apply_exact_mode_to_input("reta -zeilen --zaehlung=12"),
            "reta -zeilen --zaehlung=12"
        );
        assert_eq!(
            apply_exact_mode_to_input("-zeilen --zaehlung=12"),
            "-zeilen --zaehlung=12"
        );
    }

    #[test]
    fn rpb_exact_mode_keeps_control_storage_and_process_commands_unmodified() {
        for input in [
            "s",
            "S",
            "l",
            "o",
            "HELP",
            "help",
            "shell echo hi",
            "python print(1)",
            "math 1+1",
            ":ui",
        ] {
            assert!(
                !should_append_exact_suffix(input),
                "{input} must not get exact suffix"
            );
            assert_eq!(apply_exact_mode_to_input(input), input);
        }
    }

    #[test]
    fn rpe_emacs_mode_adds_output_group_to_nested_reta_command() {
        let command = PromptCommand::Reta(vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--zaehlung=12".to_string(),
        ]);
        let PromptCommand::Reta(argv) =
            apply_rpe_emacs_output_to_command(command, "reta -zeilen --zaehlung=12")
        else {
            panic!("rpe must preserve reta command kind");
        };
        assert_eq!(argv[0], "reta");
        assert!(argv.ends_with(&[
            "-ausgabe".to_string(),
            "--art=emacs".to_string(),
            "--keineueberschriften".to_string(),
        ]));
    }
}
