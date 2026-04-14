use crate::{run_reta_from_args, RetaRunResult};

use super::completion::candidates_for_prefix;
use super::tokenize::split_shell_like;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditModeKind {
    Emacs,
    Vi,
}

#[derive(Clone, Debug, Default)]
pub struct PromptOutput {
    pub title: String,
    pub text: String,
    pub exit_code: i32,
}

#[derive(Clone, Debug)]
pub enum PromptCommand {
    Noop,
    Exit,
    Clear,
    LaunchUi,
    PrintHelp,
    PrintCommands,
    PrintHistory,
    SwitchMode(EditModeKind),
    Shell(String),
    Reta(Vec<String>),
}

#[derive(Clone, Debug)]
pub struct SessionState {
    pub program_name: String,
    pub vi_mode: bool,
    pub implicit_logging: bool,
    pub history_lines: Vec<String>,
    pub last_output: PromptOutput,
    pub last_input: String,
}

impl SessionState {
    pub fn new(program_name: String, vi_mode: bool, implicit_logging: bool) -> Self {
        Self {
            program_name,
            vi_mode,
            implicit_logging,
            history_lines: Vec::new(),
            last_output: PromptOutput::default(),
            last_input: String::new(),
        }
    }

    pub fn current_mode(&self) -> EditModeKind {
        if self.vi_mode {
            EditModeKind::Vi
        } else {
            EditModeKind::Emacs
        }
    }
}

pub fn compile_command(input: &str) -> Result<PromptCommand, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(PromptCommand::Noop);
    }

    match trimmed {
        "q" | ":q" | "exit" | "quit" | "ende" => return Ok(PromptCommand::Exit),
        "help" | "hilfe" => return Ok(PromptCommand::PrintHelp),
        "befehle" | "kurzbefehle" => return Ok(PromptCommand::PrintCommands),
        "leeren" | "clear" => return Ok(PromptCommand::Clear),
        ":ui" | ":preview" => return Ok(PromptCommand::LaunchUi),
        ":history" => return Ok(PromptCommand::PrintHistory),
        ":mode vi" => return Ok(PromptCommand::SwitchMode(EditModeKind::Vi)),
        ":mode emacs" => return Ok(PromptCommand::SwitchMode(EditModeKind::Emacs)),
        _ => {}
    }

    let tokenized = split_shell_like(trimmed)?;
    if tokenized.tokens.is_empty() {
        return Ok(PromptCommand::Noop);
    }

    if tokenized.tokens[0] == "shell" {
        let shell_text = trimmed.strip_prefix("shell").unwrap_or("").trim().to_string();
        return Ok(PromptCommand::Shell(shell_text));
    }

    if tokenized.tokens[0] == "reta" {
        return Ok(PromptCommand::Reta(tokenized.tokens));
    }

    if tokenized.tokens[0].starts_with('-') {
        let mut argv = vec!["reta".to_string()];
        argv.extend(tokenized.tokens);
        return Ok(PromptCommand::Reta(argv));
    }

    Err(format!(
        "Unbekannter rp-Befehl: {trimmed}\nVersuche 'help', 'befehle', ':ui' oder beginne mit 'reta ...' bzw. '-zeilen ...'."
    ))
}

pub fn execute_command(command: PromptCommand, state: &mut SessionState) -> Result<Option<PromptOutput>, String> {
    match command {
        PromptCommand::Noop => Ok(None),
        PromptCommand::Exit => Ok(None),
        PromptCommand::Clear => {
            print!("\x1b[2J\x1b[H");
            Ok(Some(PromptOutput {
                title: "clear".to_string(),
                text: String::new(),
                exit_code: 0,
            }))
        }
        PromptCommand::LaunchUi => Ok(None),
        PromptCommand::PrintHelp => Ok(Some(PromptOutput {
            title: "help".to_string(),
            text: help_text(),
            exit_code: 0,
        })),
        PromptCommand::PrintCommands => Ok(Some(PromptOutput {
            title: "befehle".to_string(),
            text: commands_text(),
            exit_code: 0,
        })),
        PromptCommand::PrintHistory => Ok(Some(PromptOutput {
            title: "history".to_string(),
            text: render_history_text(&state.history_lines),
            exit_code: 0,
        })),
        PromptCommand::SwitchMode(mode) => {
            state.vi_mode = matches!(mode, EditModeKind::Vi);
            Ok(Some(PromptOutput {
                title: "mode".to_string(),
                text: format!(
                    "Editiermodus gesetzt: {}",
                    if state.vi_mode { "vi" } else { "emacs" }
                ),
                exit_code: 0,
            }))
        }
        PromptCommand::Shell(command_text) => {
            if command_text.is_empty() {
                return Err("Nach 'shell' fehlt der eigentliche Shell-Befehl".to_string());
            }
            let output = std::process::Command::new("sh")
                .arg("-lc")
                .arg(&command_text)
                .output()
                .map_err(|err| format!("Shell-Befehl konnte nicht ausgeführt werden: {err}"))?;

            let mut text = String::new();
            text.push_str(&String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                if !text.is_empty() && !text.ends_with('\n') {
                    text.push('\n');
                }
                text.push_str(&String::from_utf8_lossy(&output.stderr));
            }
            Ok(Some(PromptOutput {
                title: "shell".to_string(),
                text,
                exit_code: output.status.code().unwrap_or(1),
            }))
        }
        PromptCommand::Reta(argv) => {
            let result: RetaRunResult = run_reta_from_args(argv);
            Ok(Some(PromptOutput {
                title: "reta".to_string(),
                text: result.render_text(),
                exit_code: result.exit_code(),
            }))
        }
    }
}

pub fn help_text() -> String {
    [
        "rp / rpl – interaktive Prompt-Schicht für reta",
        "",
        "Grundidee:",
        "  - 'reta ...' führt den normalen reta-Befehl in der eingebetteten Lib aus.",
        "  - Eine Eingabe, die direkt mit '-zeilen', '-spalten', '--...' beginnt,",
        "    wird automatisch als reta-Befehl behandelt.",
        "  - ':ui' öffnet die ratatui-Ansicht mit Vorschau, History, Kandidaten und Status.",
        "",
        "Meta-Befehle:",
        "  help | hilfe            Diese Hilfe",
        "  befehle | kurzbefehle   Befehlsübersicht",
        "  :ui | :preview         Vollbild-Vorschau mit ratatui",
        "  :history               Zeigt die Sitzungs-History",
        "  :mode vi               Wechselt reedline in den Vi-Modus",
        "  :mode emacs            Wechselt reedline in den Emacs-Modus",
        "  clear | leeren         Terminal leeren",
        "  q | :q | exit | quit   rp beenden",
        "",
        "Spezial:",
        "  shell <cmd>            Führt einen Shell-Befehl aus",
        "",
        "Beispiele:",
        "  reta -zeilen --vorhervonausschnitt=1-3 -spalten --alles",
        "  -zeilen --vorhervonausschnitt=12-15 -kombination --galaxie=Lebewesen -ausgabe --breite=90",
        "  :ui",
    ]
    .join("\n")
}

pub fn commands_text() -> String {
    [
        "Bekannte rp-Kommandos:",
        "",
        "  help, hilfe, befehle, kurzbefehle",
        "  :ui, :preview, :history",
        "  :mode vi, :mode emacs",
        "  clear, leeren",
        "  shell <cmd>",
        "  reta <reta-parameter>",
        "  q, :q, exit, quit, ende",
        "",
        "Completion-Kandidatenbeispiele:",
        &candidates_for_prefix("--").join("\n  "),
    ]
    .join("\n")
}

pub fn render_history_text(history: &[String]) -> String {
    if history.is_empty() {
        return "Noch keine Sitzungs-History vorhanden.".to_string();
    }

    history
        .iter()
        .enumerate()
        .map(|(index, line)| format!("{:>4}: {}", index + 1, line))
        .collect::<Vec<_>>()
        .join("\n")
}
