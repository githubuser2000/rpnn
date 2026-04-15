use std::path::PathBuf;

use reedline::{
    default_emacs_keybindings, default_vi_insert_keybindings, default_vi_normal_keybindings,
    ColumnarMenu, DefaultHinter, DefaultPrompt, DefaultValidator, Emacs, FileBackedHistory,
    MenuBuilder, Reedline, ReedlineMenu, Signal, Vi,
};

use super::commands::{
    compile_command, execute_command, EditModeKind, PromptCommand, PromptOutput, SessionState,
};
use super::completion::build_default_completer;
use super::history::{default_history_path, default_log_path};
use super::preset::PromptFrontendPreset;
use super::tui::launch_preview_ui;

pub fn run_rp_one_shot(argv: Vec<String>, start_with_vi_mode: bool) -> i32 {
    use std::path::PathBuf;

    let program_name = PathBuf::from(
        argv.first().cloned().unwrap_or_else(|| "rpb".to_string()),
    )
    .file_name()
    .map(|s| s.to_string_lossy().to_string())
    .unwrap_or_else(|| "rpb".to_string());

    let implicit_logging = program_name == "rpl";
    let mut state = SessionState::new(program_name.clone(), start_with_vi_mode, implicit_logging);

    let input = if argv.len() > 1 {
        argv[1..].join(" ")
    } else {
        String::new()
    };

    let compiled = match compile_command(&input, state.prompt_mode) {
        Ok(cmd) => cmd,
        Err(err) => {
            eprintln!("{err}");
            return 1;
        }
    };

    match execute_command(compiled, &mut state) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{err}");
            1
        }
    }
}

pub fn run_prompt_frontend_from_env(fallback_vi_mode: bool) -> i32 {
    let argv = std::env::args().collect::<Vec<_>>();
    run_prompt_frontend(argv, fallback_vi_mode)
}

pub fn run_rp_from_env(start_with_vi_mode: bool) -> i32 {
    run_prompt_frontend_from_env(start_with_vi_mode)
}

pub fn run_prompt_frontend(argv: Vec<String>, fallback_vi_mode: bool) -> i32 {
    let program_name = program_name_from_argv(&argv);
    let preset = PromptFrontendPreset::from_program_name(&program_name, fallback_vi_mode);
    let mut state = SessionState::new(
        program_name.clone(),
        preset.start_with_vi_mode,
        preset.implicit_logging,
    );
    let history_path = default_history_path(&program_name);
    let log_path = default_log_path(&program_name);

    if state.logging_enabled {
        append_log_line(
            &log_path,
            "session",
            &format!(
                "start program={} vi_mode={} implicit_logging={} one_shot={}",
                program_name,
                preset.start_with_vi_mode,
                preset.implicit_logging,
                preset.one_shot,
            ),
        );
    }

    if preset.one_shot {
        return run_one_shot(argv, &log_path, &mut state);
    }

    run_interactive_loop(history_path, log_path, &mut state)
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

fn run_one_shot(argv: Vec<String>, log_path: &PathBuf, state: &mut SessionState) -> i32 {
    let input = argv.into_iter().skip(1).collect::<Vec<_>>().join(" ");
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

    state.last_input = input.clone();
    state.history_lines.push(input.clone());

    if state.logging_enabled {
        append_log_line(log_path, "input", &input);
    }

    let compiled = match compile_command(&input, state.prompt_mode) {
        Ok(command) => command,
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

    if matches!(compiled, PromptCommand::Exit) {
        if state.logging_enabled {
            append_log_line(log_path, "session", "exit command received in one-shot mode");
        }
        return 0;
    }

    if matches!(compiled, PromptCommand::LaunchUi) {
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
}

fn run_interactive_loop(history_path: PathBuf, log_path: PathBuf, state: &mut SessionState) -> i32 {
    let prompt = DefaultPrompt::default();

    let mut editor = match build_editor(&history_path, state.current_mode()) {
        Ok(editor) => editor,
        Err(err) => {
            eprintln!("rp konnte reedline nicht initialisieren: {err}");
            return 1;
        }
    };

    loop {
        match editor.read_line(&prompt) {
            Ok(Signal::Success(buffer)) => {
                let input = buffer.trim().to_string();
                state.last_input = input.clone();
                if !input.is_empty() {
                    state.history_lines.push(input.clone());
                }

                if state.logging_enabled {
                    append_log_line(&log_path, "input", &input);
                }

                let compiled = match compile_command(&input, state.prompt_mode) {
                    Ok(command) => command,
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

                if matches!(compiled, PromptCommand::Exit) {
                    if state.logging_enabled {
                        append_log_line(&log_path, "session", "exit command received");
                    }
                    break;
                }

                if matches!(compiled, PromptCommand::LaunchUi) {
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

                let rebuild_editor = matches!(compiled, PromptCommand::SwitchMode(_));

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

                if rebuild_editor {
                    editor = match build_editor(&history_path, state.current_mode()) {
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

fn build_editor(history_path: &PathBuf, mode: EditModeKind) -> Result<Reedline, String> {
    let history = Box::new(
        FileBackedHistory::with_file(2_000, history_path.clone())
            .map_err(|err| format!("History-Datei konnte nicht geöffnet werden: {err}"))?,
    );

    let completer = build_default_completer();
    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));

    let mut editor = Reedline::create()
        .with_history(history)
        .with_hinter(Box::new(DefaultHinter::default()))
        .with_validator(Box::new(DefaultValidator))
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu));

    editor = match mode {
        EditModeKind::Emacs => {
            let edit_mode = Box::new(Emacs::new(default_emacs_keybindings()));
            editor.with_edit_mode(edit_mode)
        }
        EditModeKind::Vi => {
            let edit_mode = Box::new(Vi::new(
                default_vi_insert_keybindings(),
                default_vi_normal_keybindings(),
            ));
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
