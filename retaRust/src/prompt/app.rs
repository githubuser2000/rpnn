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
use super::history::default_history_path;
use super::tui::launch_preview_ui;

pub fn run_rp_from_env(start_with_vi_mode: bool) -> i32 {
    let argv = std::env::args().collect::<Vec<_>>();
    run_rp(argv, start_with_vi_mode)
}

pub fn run_rp(argv: Vec<String>, start_with_vi_mode: bool) -> i32 {
    let program_name = PathBuf::from(
        argv.first()
            .cloned()
            .unwrap_or_else(|| "rp".to_string()),
    )
    .file_name()
    .map(|s| s.to_string_lossy().to_string())
    .unwrap_or_else(|| "rp".to_string());

    let implicit_logging = program_name == "rpl";

    let mut state = SessionState::new(program_name.clone(), start_with_vi_mode, implicit_logging);
    let history_path = default_history_path(&program_name);
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

                let compiled = match compile_command(&input, state.prompt_mode) {
                    Ok(command) => command,
                    Err(err) => {
                        print_output(&mut state, PromptOutput {
                            title: "error".to_string(),
                            text: err,
                            exit_code: 1,
                        });
                        continue;
                    }
                };

                if matches!(compiled, PromptCommand::Exit) {
                    break;
                }

                if matches!(compiled, PromptCommand::LaunchUi) {
                    if let Err(err) = launch_preview_ui(&state) {
                        print_output(&mut state, PromptOutput {
                            title: "ui-error".to_string(),
                            text: format!("Die ratatui-Vorschau konnte nicht gestartet werden: {err}"),
                            exit_code: 1,
                        });
                    }
                    continue;
                }

                let rebuild_editor = matches!(compiled, PromptCommand::SwitchMode(_));

                match execute_command(compiled, &mut state) {
                    Ok(Some(output)) => {
                        print_output(&mut state, output);
                    }
                    Ok(None) => {}
                    Err(err) => {
                        print_output(&mut state, PromptOutput {
                            title: "error".to_string(),
                            text: err,
                            exit_code: 1,
                        });
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
                println!("^C");
                continue;
            }
            Ok(Signal::CtrlD) => {
                println!();
                break;
            }
            Ok(other) => {
                print_output(&mut state, PromptOutput {
                    title: "signal".to_string(),
                    text: format!("Nicht direkt behandeltes reedline-Signal: {other:?}"),
                    exit_code: 0,
                });
            }
            Err(err) => {
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

fn print_output(state: &mut SessionState, output: PromptOutput) {
    state.last_output = output.clone();
    if !output.text.is_empty() {
        println!("{}", output.text);
    }
}
