use crate::{run_reta_from_args, RetaRunResult};

use super::completion::candidates_for_prefix;
use super::python_like::{
    build_reta_calls_from_prompt_tokens, expand_kurz_kurz_befehl, normalize_prompt_tokens,
    prompt_words, PromptModus,
};
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
    SaveBefore,
    SaveAfter,
    DeleteStoredStart,
    ShowStored,
    Clear,
    LaunchUi,
    PrintHelp,
    PrintCommands,
    PrintHistory,
    SwitchMode(EditModeKind),
    ToggleLogging(bool),
    Shell(String),
    Python(String),
    Math(String),
    Reta(Vec<String>),
    RetaBatch(Vec<Vec<String>>),
}

#[derive(Clone, Debug)]
pub struct SessionState {
    pub program_name: String,
    pub vi_mode: bool,
    pub logging_enabled: bool,
    pub implicit_logging: bool,
    pub history_lines: Vec<String>,
    pub last_output: PromptOutput,
    pub last_input: String,
    pub prompt_mode: PromptModus,
    pub stored_commands: Vec<String>,
}

impl SessionState {
    pub fn new(program_name: String, vi_mode: bool, implicit_logging: bool) -> Self {
        Self {
            program_name,
            vi_mode,
            logging_enabled: implicit_logging,
            implicit_logging,
            history_lines: Vec::new(),
            last_output: PromptOutput::default(),
            last_input: String::new(),
            prompt_mode: PromptModus::Normal,
            stored_commands: Vec::new(),
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

pub fn compile_command(input: &str, prompt_mode: PromptModus) -> Result<PromptCommand, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(PromptCommand::Noop);
    }

    match trimmed {
        "q" | ":q" | "exit" | "quit" | "ende" => return Ok(PromptCommand::Exit),
        "help" | "hilfe" => return Ok(PromptCommand::PrintHelp),
        "befehle" | "kurzbefehle" => return Ok(PromptCommand::PrintCommands),
        "s" | "BefehlSpeichernDavor" => return Ok(PromptCommand::SaveBefore),
        "S" | "BefehlSpeichernDanach" => return Ok(PromptCommand::SaveAfter),
        "l" | "BefehlSpeicherungLöschen" => return Ok(PromptCommand::DeleteStoredStart),
        "o" | "BefehlSpeicherungAusgeben" => return Ok(PromptCommand::ShowStored),
        "leeren" | "clear" => return Ok(PromptCommand::Clear),
        ":ui" | ":preview" => return Ok(PromptCommand::LaunchUi),
        ":history" => return Ok(PromptCommand::PrintHistory),
        ":mode vi" => return Ok(PromptCommand::SwitchMode(EditModeKind::Vi)),
        ":mode emacs" => return Ok(PromptCommand::SwitchMode(EditModeKind::Emacs)),
        "loggen" => return Ok(PromptCommand::ToggleLogging(true)),
        "nichtloggen" => return Ok(PromptCommand::ToggleLogging(false)),
        _ => {}
    }

    let tokenized = split_shell_like(trimmed)?;
    if tokenized.tokens.is_empty() {
        return Ok(PromptCommand::Noop);
    }

    let (_, expanded) = expand_kurz_kurz_befehl(prompt_mode, &tokenized.tokens);
    let mut effective_tokens = if expanded.is_empty() {
        tokenized.tokens.clone()
    } else {
        expanded
    };
    effective_tokens = normalize_prompt_tokens(&effective_tokens);

    if effective_tokens[0] == "shell" {
        let shell_text = trimmed.strip_prefix("shell").unwrap_or("").trim().to_string();
        return Ok(PromptCommand::Shell(shell_text));
    }
    if effective_tokens[0] == "python" {
        let command_text = trimmed.strip_prefix("python").unwrap_or("").trim().to_string();
        return Ok(PromptCommand::Python(command_text));
    }
    if effective_tokens[0] == "math" {
        let command_text = trimmed.strip_prefix("math").unwrap_or("").trim().to_string();
        return Ok(PromptCommand::Math(command_text));
    }
    if effective_tokens[0] == "reta" {
        return Ok(PromptCommand::Reta(effective_tokens));
    }
    if effective_tokens[0].starts_with('-') {
        let mut argv = vec!["reta".to_string()];
        argv.extend(effective_tokens);
        return Ok(PromptCommand::Reta(argv));
    }
    let prompt_calls = build_reta_calls_from_prompt_tokens(&effective_tokens);
    if !prompt_calls.is_empty() {
        if prompt_calls.len() == 1 {
            return Ok(PromptCommand::Reta(prompt_calls[0].argv.clone()));
        }
        return Ok(PromptCommand::RetaBatch(prompt_calls.into_iter().map(|call| call.argv).collect()));
    }

    Err(format!(
        "Unbekannter rp-Befehl: {trimmed}\nVersuche 'help', 'befehle', ':ui' oder beginne mit 'reta ...' bzw. '-zeilen ...'."
    ))
}

pub fn execute_command(
    command: PromptCommand,
    state: &mut SessionState,
) -> Result<Option<PromptOutput>, String> {
    match command {
        PromptCommand::Noop => Ok(None),
        PromptCommand::Exit => Ok(None),
        PromptCommand::SaveBefore => {
            state.prompt_mode = PromptModus::Speichern;
            Ok(Some(PromptOutput {
                title: "speichern".to_string(),
                text: "Speicher-Modus aktiviert. Nächster reta-Befehl wird vorne gespeichert.".to_string(),
                exit_code: 0,
            }))
        }
        PromptCommand::SaveAfter => {
            state.prompt_mode = PromptModus::SpeicherungAusgaben;
            Ok(Some(PromptOutput {
                title: "speichern".to_string(),
                text: "Speicher-Modus aktiviert. Nächster reta-Befehl wird hinten gespeichert.".to_string(),
                exit_code: 0,
            }))
        }
        PromptCommand::DeleteStoredStart => {
            state.prompt_mode = PromptModus::LoeschenStart;
            Ok(Some(PromptOutput {
                title: "loeschen".to_string(),
                text: format!(
                    "Lösch-Modus aktiviert. Aktuell gespeicherte Befehle:\n{}",
                    render_history_text(&state.stored_commands)
                ),
                exit_code: 0,
            }))
        }
        PromptCommand::ShowStored => Ok(Some(PromptOutput {
            title: "stored".to_string(),
            text: render_history_text(&state.stored_commands),
            exit_code: 0,
        })),
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
        PromptCommand::ToggleLogging(enabled) => {
            state.logging_enabled = enabled;
            Ok(Some(PromptOutput {
                title: "logging".to_string(),
                text: format!(
                    "Logging ist jetzt {}.",
                    if state.logging_enabled { "aktiv" } else { "inaktiv" }
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
        PromptCommand::Python(command_text) => run_python_command(&command_text),
        PromptCommand::Math(command_text) => run_math_command(&command_text),
        PromptCommand::Reta(argv) => {
            let argv = apply_storage_mode(state, argv);
            let result: RetaRunResult = run_reta_from_args(argv);
            Ok(Some(PromptOutput {
                title: "reta".to_string(),
                text: result.render_text(),
                exit_code: result.exit_code(),
            }))
        }
        PromptCommand::RetaBatch(arg_sets) => {
            let mut rendered = Vec::new();
            let mut exit_code = 0;
            for argv in arg_sets {
                let argv = apply_storage_mode(state, argv);
                let result: RetaRunResult = run_reta_from_args(argv);
                exit_code = exit_code.max(result.exit_code());
                let text = result.render_text();
                if !text.is_empty() {
                    rendered.push(text);
                }
            }
            Ok(Some(PromptOutput {
                title: "reta".to_string(),
                text: rendered.join("\n\n"),
                exit_code,
            }))
        }
    }
}

fn run_python_command(command_text: &str) -> Result<Option<PromptOutput>, String> {
    if command_text.is_empty() {
        return Err("Nach 'python' fehlt der eigentliche Python-Befehl".to_string());
    }
    let output = std::process::Command::new("python3")
        .arg("-c")
        .arg(command_text)
        .output()
        .map_err(|err| format!("Python-Befehl konnte nicht ausgeführt werden: {err}"))?;
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        text.push_str(&String::from_utf8_lossy(&output.stderr));
    }
    Ok(Some(PromptOutput {
        title: "python".to_string(),
        text,
        exit_code: output.status.code().unwrap_or(1),
    }))
}

fn run_math_command(command_text: &str) -> Result<Option<PromptOutput>, String> {
    if command_text.is_empty() {
        return Err("Nach 'math' fehlt der eigentliche Ausdruck".to_string());
    }
    let python_code = format!("print({command_text})");
    let output = std::process::Command::new("python3")
        .arg("-c")
        .arg(&python_code)
        .output()
        .map_err(|err| format!("Math-Befehl konnte nicht ausgeführt werden: {err}"))?;
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        text.push_str(&String::from_utf8_lossy(&output.stderr));
    }
    Ok(Some(PromptOutput {
        title: "math".to_string(),
        text,
        exit_code: output.status.code().unwrap_or(1),
    }))
}

pub fn apply_storage_mode(state: &mut SessionState, argv: Vec<String>) -> Vec<String> {
    if argv.is_empty() {
        return argv;
    }

    match state.prompt_mode {
        PromptModus::Speichern => {
            if argv.len() > 1 {
                state.stored_commands.insert(0, argv[1..].join(" "));
            }
            state.prompt_mode = PromptModus::Normal;
        }
        PromptModus::SpeicherungAusgaben | PromptModus::SpeicherungAusgabenMitZusatz => {
            if argv.len() > 1 {
                state.stored_commands.push(argv[1..].join(" "));
            }
            state.prompt_mode = PromptModus::Normal;
        }
        PromptModus::LoeschenStart | PromptModus::LoeschenSelect => {
            if argv.len() > 1 {
                let joined = argv[1..].join(" ");
                state.stored_commands.retain(|s| s != &joined);
            }
            state.prompt_mode = PromptModus::Normal;
        }
        PromptModus::AusgabeSelektiv | PromptModus::Normal => {}
    }

    let mut out = vec![argv[0].clone()];
    for stored in &state.stored_commands {
        if let Ok(parts) = split_shell_like(stored) {
            out.extend(parts.tokens);
        }
    }
    out.extend(argv.into_iter().skip(1));
    out
}

pub fn help_text() -> String {
    [
        "rp / rpl – interaktive Prompt-Schicht für reta",
        "",
        "Grundidee:",
        "  - 'reta ...' führt den normalen reta-Befehl in der eingebetteten Lib aus.",
        "  - Eine Eingabe, die direkt mit '-zeilen', '-spalten', '--...' beginnt,",
        "    wird automatisch als reta-Befehl behandelt.",
        "  - Kurzbefehle wie 'a1/2', 'u 3/4', 'G7', 'mond 12' werden Python-nah in reta-Argumente expandiert.",
        "  - ':ui' öffnet die ratatui-Ansicht mit Vorschau, History, Kandidaten und Status.",
        "",
        "Python-nahe Prompt-Befehle:",
        "  s / BefehlSpeichernDavor     nächsten reta-Befehl vorne speichern",
        "  S / BefehlSpeichernDanach    nächsten reta-Befehl hinten speichern",
        "  l / BefehlSpeicherungLöschen gespeicherten Befehl löschen",
        "  o / BefehlSpeicherungAusgeben gespeicherte Befehle anzeigen",
        "  loggen | nichtloggen         Logging umschalten",
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
        "  python <code>          Führt Python-Code aus",
        "  math <expr>            Wertet einen Python-Ausdruck aus",
        "",
        "Beispiele:",
        "  reta -zeilen --vorhervonausschnitt=1-3 -spalten --alles",
        "  -zeilen --vorhervonausschnitt=12-15 -kombination --galaxie=Lebewesen -ausgabe --breite=90",
        "  av12-15",
        "  G1/2",
        "  mond 12",
        "  2,3-5",
        "  :ui",
    ]
    .join("\n")
}

pub fn commands_text() -> String {
    let mut lines = vec![
        "Bekannte rp-Kommandos:".to_string(),
        String::new(),
        "  help, hilfe, befehle, kurzbefehle".to_string(),
        "  :ui, :preview, :history".to_string(),
        "  :mode vi, :mode emacs".to_string(),
        "  clear, leeren".to_string(),
        "  loggen, nichtloggen".to_string(),
        "  shell <cmd>, python <code>, math <expr>".to_string(),
        "  reta <reta-parameter>".to_string(),
        "  q, :q, exit, quit, ende".to_string(),
        String::new(),
        "Completion-Kandidatenbeispiele:".to_string(),
    ];
    for candidate in candidates_for_prefix("--") {
        lines.push(format!("  {candidate}"));
    }
    lines.push(String::new());
    lines.push("Python-nahe Prompt-Befehle:".to_string());
    for cmd in &prompt_words().befehle {
        lines.push(format!("  {cmd}"));
    }
    lines.join("\n")
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
