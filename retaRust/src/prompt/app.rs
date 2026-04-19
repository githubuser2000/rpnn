use std::borrow::Cow;
use std::path::PathBuf;

use reedline::{
    default_emacs_keybindings, default_vi_insert_keybindings, default_vi_normal_keybindings,
    ColumnarMenu, DefaultHinter, DefaultValidator, Emacs, FileBackedHistory, KeyCode,
    KeyModifiers, Keybindings, MenuBuilder, Prompt, PromptEditMode, PromptHistorySearch,
    Reedline, ReedlineEvent, ReedlineMenu, Signal, Vi,
};

use super::commands::{
    compile_command_with_state, execute_command, help_text, take_auto_prompt_command,
    EditModeKind, PromptCommand, PromptOutput, SessionState,
};
use super::completion::{
    build_default_completer_with_runtime, new_completion_runtime_handle,
    set_completion_runtime_context, CompletionRuntimeHandle,
};
use super::history::{default_history_path, default_log_path};
use super::frontend_profile::PromptFrontendProfile;
use super::preset::PromptFrontendPreset;
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
    let mut shortened = text.chars().take(MAX_PROMPT_PLACEHOLDER_CHARS).collect::<String>();
    if text.chars().count() > MAX_PROMPT_PLACEHOLDER_CHARS {
        shortened.push('…');
    }
    Some(shortened)
}

fn prompt_text_for_state(state: &SessionState) -> String {
    match state.prompt_mode {
        super::python_like::PromptModus::Speichern => "was speichern> ".to_string(),
        super::python_like::PromptModus::LoeschenStart
        | super::python_like::PromptModus::LoeschenSelect => "was löschen> ".to_string(),
        super::python_like::PromptModus::Normal | super::python_like::PromptModus::AusgabeSelektiv => {
            if let Some(placeholder) = prompt_placeholder_text(state) {
                format!("{placeholder} > ")
            } else {
                "> ".to_string()
            }
        }
        super::python_like::PromptModus::SpeicherungAusgaben
        | super::python_like::PromptModus::SpeicherungAusgabenMitZusatz => "o> ".to_string(),
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

    format!(
        "{trimmed} keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
    )
}

fn input_starts_with_reta(input: &str) -> bool {
    match super::tokenize::split_shell_like(input.trim()) {
        Ok(tokenized) => matches!(tokenized.tokens.first(), Some(token) if token == "reta"),
        Err(_) => false,
    }
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

fn should_append_exact_suffix(input: &str) -> bool {
    let tokenized = match super::tokenize::split_shell_like(input) {
        Ok(tokens) => tokens,
        Err(_) => return false,
    };

    if tokenized.tokens.is_empty() {
        return false;
    }

    let first = tokenized.tokens[0].as_str();

    if first == "reta" {
        return false;
    }
    if first.starts_with('-') {
        return false;
    }
    if first.starts_with(':') {
        return false;
    }

    if tokenized.tokens.iter().any(|token| {
        matches!(
            token.as_str(),
            "s"
                | "S"
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
        "help"
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

pub fn run_prompt_input_frontend_with_profile(argv: Vec<String>, profile: PromptFrontendProfile) -> i32 {
    let preset = PromptFrontendPreset::from_profile_and_argv(profile, &argv);
    run_prompt_frontend_with_preset(argv, preset)
}

pub fn run_prompt_command_frontend_with_profile(argv: Vec<String>, profile: PromptFrontendProfile) -> i32 {
    let mut preset = PromptFrontendPreset::from_profile_and_argv(profile, &argv);
    preset.one_shot = true;
    run_prompt_frontend_with_preset(argv, preset)
}

fn run_prompt_frontend_with_preset(argv: Vec<String>, preset: PromptFrontendPreset) -> i32 {
    let program_name = program_name_from_argv(&argv);
    let startup = parse_startup_args(&argv, &preset);

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
        let input = startup.command_text.unwrap_or_else(|| startup.trailing_args.join(" "));
        return run_one_shot(input, &log_path, preset.emacs_output_mode, &mut state);
    }

    run_interactive_loop(history_path, log_path, startup.exact_mode, &mut state)
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
) -> i32 {
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

    state.previous_input = state.last_input.clone();
    state.last_input = input.clone();
    state.history_lines.push(input.clone());

    if state.logging_enabled {
        append_log_line(log_path, "input", &input);
    }

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

    if matches!(&compiled, PromptCommand::Exit) {
        if state.logging_enabled {
            append_log_line(log_path, "session", "exit command received in one-shot mode");
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
    state: &mut SessionState,
) -> i32 {
    let completion_runtime = new_completion_runtime_handle();
    set_completion_runtime_context(
        &completion_runtime,
        state.prompt_mode,
        &state.stored_expanded_tokens,
        &state.stored_commands,
    );

    let mut editor = match build_editor(
        &history_path,
        state.current_mode(),
        state.logging_enabled,
        &completion_runtime,
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
                || previous_logging_enabled != state.logging_enabled;

            if rebuild_editor {
                editor = match build_editor(
                    &history_path,
                    state.current_mode(),
                    state.logging_enabled,
                    &completion_runtime,
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
                state.previous_input = state.last_input.clone();
                state.last_input = input.clone();
                if !input.is_empty() {
                    state.history_lines.push(input.clone());
                }

                if state.logging_enabled {
                    append_log_line(&log_path, "input", &input);
                }

                let compile_input = if exact_mode_enabled {
                    apply_exact_mode_to_input(&input)
                } else {
                    input.clone()
                };

                let previous_editor_mode = state.current_mode();
                let previous_logging_enabled = state.logging_enabled;
                let compiled = match compile_command_with_state(&compile_input, state) {
                    Ok(command) => {
                        if state.program_name == "rpe" {
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
                    || previous_logging_enabled != state.logging_enabled;

                if rebuild_editor {
                    editor = match build_editor(
                        &history_path,
                        state.current_mode(),
                        state.logging_enabled,
                        &completion_runtime,
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

fn menu_aware_navigation(menu_event: ReedlineEvent, fallback_event: ReedlineEvent) -> ReedlineEvent {
    ReedlineEvent::UntilFound(vec![menu_event, fallback_event])
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
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Right,
        menu_aware_navigation(ReedlineEvent::MenuRight, ReedlineEvent::Right),
    );
}

fn build_editor(
    history_path: &PathBuf,
    mode: EditModeKind,
    logging_enabled: bool,
    completion_runtime: &CompletionRuntimeHandle,
) -> Result<Reedline, String> {
    let effective_history_path = if logging_enabled {
        history_path.clone()
    } else {
        PathBuf::from("/dev/null")
    };

    let history = Box::new(
        FileBackedHistory::with_file(2_000, effective_history_path)
            .map_err(|err| format!("History-Datei konnte nicht geöffnet werden: {err}"))?,
    );

    let completer = build_default_completer_with_runtime(completion_runtime.clone());
    let completion_menu = Box::new(ColumnarMenu::default().with_name(COMPLETION_MENU_NAME));

    let mut editor = Reedline::create()
        .with_history(history)
        .with_hinter(Box::new(DefaultHinter::default()))
        .with_validator(Box::new(DefaultValidator))
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu));

    editor = match mode {
        EditModeKind::Emacs => {
            let mut keybindings = default_emacs_keybindings();
            add_completion_keybindings(&mut keybindings);
            let edit_mode = Box::new(Emacs::new(keybindings));
            editor.with_edit_mode(edit_mode)
        }
        EditModeKind::Vi => {
            let mut insert_keybindings = default_vi_insert_keybindings();
            let mut normal_keybindings = default_vi_normal_keybindings();
            add_completion_keybindings(&mut insert_keybindings);
            add_completion_keybindings(&mut normal_keybindings);
            let edit_mode = Box::new(Vi::new(insert_keybindings, normal_keybindings));
            editor.with_edit_mode(edit_mode)
        }
    };

    Ok(editor)
}

fn append_log_line(path: &PathBuf, kind: &str, text: &str) {
    use std::io::Write;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

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
