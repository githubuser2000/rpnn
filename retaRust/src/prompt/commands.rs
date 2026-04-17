use crate::{run_reta_from_args, RetaRunResult};

use super::completion::candidates_for_prefix;
use super::python_like::{
    build_reta_argv_from_prompt_tokens, build_reta_calls_from_prompt_tokens,
    custom_split_whitespace_parenthesized, expand_kurz_kurz_befehl,
    finalize_prompt_tokens_for_execution, looks_like_numeric_or_fraction_range,
    prepare_prompt_big_output_for_stored_reta, prepare_prompt_big_output_for_stored_rows,
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
    StoreCurrentInput(String),
    StoreInline(String),
    DeleteStoredStart,
    DeleteStoredSelection(String),
    ShowStored(Option<String>),
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
    Immediate(PromptOutput),
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
    pub previous_input: String,
    pub last_input: String,
    pub prompt_mode: PromptModus,
    pub stored_placeholder: String,
    pub stored_commands: Vec<String>,
    pub stored_expanded_tokens: Vec<String>,
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
            previous_input: String::new(),
            last_input: String::new(),
            prompt_mode: PromptModus::Normal,
            stored_placeholder: String::new(),
            stored_commands: Vec::new(),
            stored_expanded_tokens: Vec::new(),
        }
    }

    pub fn current_mode(&self) -> EditModeKind {
        if self.vi_mode {
            EditModeKind::Vi
        } else {
            EditModeKind::Emacs
        }
    }

    pub fn has_stored_placeholder(&self) -> bool {
        !self.stored_expanded_tokens.is_empty()
    }
}

fn compile_command_inner(input: &str, prompt_mode: PromptModus) -> Result<PromptCommand, String> {
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
        "o" | "BefehlSpeicherungAusgeben" => return Ok(PromptCommand::ShowStored(None)),
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
    if !matches!(
        effective_tokens.first().map(String::as_str),
        Some("shell" | "python" | "abstand")
    ) {
        effective_tokens = finalize_prompt_tokens_for_execution(&effective_tokens);
    }

    if effective_tokens[0] == "shell" {
        let shell_text = trimmed
            .strip_prefix("shell")
            .unwrap_or("")
            .trim()
            .to_string();
        return Ok(PromptCommand::Shell(shell_text));
    }
    if effective_tokens[0] == "python" {
        let command_text = trimmed
            .strip_prefix("python")
            .unwrap_or("")
            .trim()
            .to_string();
        return Ok(PromptCommand::Python(command_text));
    }
    if effective_tokens[0] == "math" {
        let command_text = trimmed
            .strip_prefix("math")
            .unwrap_or("")
            .trim()
            .to_string();
        return Ok(PromptCommand::Math(command_text));
    }
    if let Some(output) = compile_direct_number_command(&effective_tokens) {
        return Ok(PromptCommand::Immediate(output));
    }
    if let Some(output) = compile_abc_abcd_command(&effective_tokens) {
        return Ok(PromptCommand::Immediate(output));
    }
    if effective_tokens[0] == "reta" {
        return Ok(PromptCommand::Reta(effective_tokens));
    }
    if effective_tokens[0].starts_with('-') {
        let mut argv = vec!["reta".to_string()];
        argv.extend(effective_tokens);
        return Ok(PromptCommand::Reta(argv));
    }
    let calls = build_reta_calls_from_prompt_tokens(&effective_tokens);
    if !calls.is_empty() {
        return if calls.len() == 1 {
            Ok(PromptCommand::Reta(calls.into_iter().next().unwrap()))
        } else {
            Ok(PromptCommand::RetaBatch(calls))
        };
    }
    if let Some(argv) = build_reta_argv_from_prompt_tokens(&effective_tokens) {
        return Ok(PromptCommand::Reta(argv));
    }

    Err(format!(
        "Unbekannter rp-Befehl: {trimmed}\nVersuche 'help', 'befehle', ':ui' oder beginne mit 'reta ...' bzw. '-zeilen ...'."
    ))
}

pub fn compile_command(input: &str, prompt_mode: PromptModus) -> Result<PromptCommand, String> {
    compile_command_inner(input, prompt_mode)
}

pub fn compile_command_with_state(
    input: &str,
    state: &SessionState,
) -> Result<PromptCommand, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        if state.has_stored_placeholder()
            && matches!(
                state.prompt_mode,
                PromptModus::Normal | PromptModus::AusgabeSelektiv
            )
        {
            return Ok(PromptCommand::ShowStored(None));
        }
        return Ok(PromptCommand::Noop);
    }

    if matches!(state.prompt_mode, PromptModus::Speichern) {
        return Ok(PromptCommand::StoreCurrentInput(trimmed.to_string()));
    }

    if matches!(
        state.prompt_mode,
        PromptModus::LoeschenStart | PromptModus::LoeschenSelect
    ) {
        return Ok(PromptCommand::DeleteStoredSelection(trimmed.to_string()));
    }

    let tokenized = split_shell_like(trimmed)?;
    if tokenized.tokens.is_empty() {
        return Ok(PromptCommand::Noop);
    }

    if let Some(command) = compile_inline_storage_command(&tokenized.tokens) {
        return Ok(command);
    }

    if let Some(prepared) =
        prepare_prompt_big_output_for_stored_reta(&state.stored_expanded_tokens, &tokenized.tokens)
    {
        return Ok(PromptCommand::Reta(prepared.tokens));
    }
    if let Some(prepared) =
        prepare_prompt_big_output_for_stored_rows(&state.stored_expanded_tokens, &tokenized.tokens)
    {
        return Ok(PromptCommand::Reta(prepared.tokens));
    }

    if raw_input_bypasses_stored_merge(trimmed, &tokenized.tokens)
        || !state.has_stored_placeholder()
    {
        return compile_command_inner(trimmed, state.prompt_mode);
    }

    let effective_input =
        compose_input_with_stored_placeholder(&state.stored_expanded_tokens, &tokenized.tokens);

    compile_command_inner(&effective_input, PromptModus::AusgabeSelektiv)
}

fn compile_inline_storage_command(tokens: &[String]) -> Option<PromptCommand> {
    if tokens.len() <= 1 {
        return None;
    }

    if tokens
        .iter()
        .any(|token| is_store_before_token(token) || is_store_after_token(token))
    {
        let payload = tokens
            .iter()
            .filter(|token| !is_store_before_token(token) && !is_store_after_token(token))
            .cloned()
            .collect::<Vec<_>>()
            .join(" ");
        if !payload.trim().is_empty() {
            return Some(PromptCommand::StoreInline(payload));
        }
    }

    if tokens.iter().any(|token| is_show_stored_token(token)) {
        let payload = tokens
            .iter()
            .filter(|token| !is_show_stored_token(token))
            .cloned()
            .collect::<Vec<_>>()
            .join(" ");
        return Some(PromptCommand::ShowStored(
            (!payload.trim().is_empty()).then_some(payload),
        ));
    }

    None
}

fn raw_input_bypasses_stored_merge(trimmed: &str, tokens: &[String]) -> bool {
    if matches!(
        trimmed,
        "q" | ":q"
            | "exit"
            | "quit"
            | "ende"
            | "help"
            | "hilfe"
            | "befehle"
            | "kurzbefehle"
            | "s"
            | "BefehlSpeichernDavor"
            | "S"
            | "BefehlSpeichernDanach"
            | "l"
            | "BefehlSpeicherungLöschen"
            | "o"
            | "BefehlSpeicherungAusgeben"
            | "leeren"
            | "clear"
            | ":ui"
            | ":preview"
            | ":history"
            | ":mode vi"
            | ":mode emacs"
            | "loggen"
            | "nichtloggen"
    ) {
        return true;
    }

    matches!(
        tokens.first().map(String::as_str),
        Some("shell" | "python" | "math" | ":mode")
    )
}

fn is_store_before_token(token: &str) -> bool {
    matches!(token, "s" | "BefehlSpeichernDavor")
}

fn is_store_after_token(token: &str) -> bool {
    matches!(token, "S" | "BefehlSpeichernDanach")
}

fn is_show_stored_token(token: &str) -> bool {
    matches!(token, "o" | "BefehlSpeicherungAusgeben")
}

fn compose_input_with_stored_placeholder(
    stored_tokens: &[String],
    input_tokens: &[String],
) -> String {
    if stored_tokens.is_empty() {
        return input_tokens.join(" ");
    }

    if input_tokens.is_empty() {
        return stored_tokens.join(" ");
    }

    let mut combined = Vec::with_capacity(stored_tokens.len() + input_tokens.len());
    if matches!(input_tokens.first().map(String::as_str), Some("reta"))
        && !matches!(stored_tokens.first().map(String::as_str), Some("reta"))
    {
        combined.extend(input_tokens.iter().cloned());
        combined.extend(stored_tokens.iter().cloned());
    } else {
        combined.extend(stored_tokens.iter().cloned());
        combined.extend(input_tokens.iter().cloned());
    }

    combined.join(" ")
}

fn split_storage_text(text: &str) -> Vec<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    match split_shell_like(trimmed) {
        Ok(tokenized) if !tokenized.tokens.is_empty() => tokenized.tokens,
        _ => custom_split_whitespace_parenthesized(trimmed),
    }
}

fn prepare_stored_prefix_tokens_from_text(text: &str) -> Vec<String> {
    let tokens = split_storage_text(text);
    prepare_stored_prefix_tokens(&tokens)
}

fn prepare_stored_prefix_tokens(tokens: &[String]) -> Vec<String> {
    if tokens.is_empty() {
        return Vec::new();
    }

    let (_, expanded) = expand_kurz_kurz_befehl(PromptModus::AusgabeSelektiv, tokens);
    let mut effective_tokens = if expanded.is_empty() {
        tokens.to_vec()
    } else {
        expanded
    };

    if !matches!(
        effective_tokens.first().map(String::as_str),
        Some("shell" | "python" | "abstand")
    ) {
        effective_tokens = finalize_prompt_tokens_for_execution(&effective_tokens);
    }

    effective_tokens
}

fn merge_stored_placeholder(existing: &str, incoming: &str) -> String {
    let existing_tokens = split_storage_text(existing);
    let incoming_tokens = split_storage_text(incoming);

    if existing_tokens.is_empty() {
        return incoming_tokens.join(" ");
    }
    if incoming_tokens.is_empty() {
        return existing_tokens.join(" ");
    }

    let existing_prepared = prepare_stored_prefix_tokens(&existing_tokens);
    let incoming_prepared = prepare_stored_prefix_tokens(&incoming_tokens);

    if let Some(prepared) =
        prepare_prompt_big_output_for_stored_reta(&existing_prepared, &incoming_tokens)
    {
        return prepared.tokens.join(" ");
    }
    if let Some(prepared) =
        prepare_prompt_big_output_for_stored_reta(&incoming_prepared, &existing_tokens)
    {
        return prepared.tokens.join(" ");
    }
    if let Some(prepared) =
        prepare_prompt_big_output_for_stored_rows(&existing_prepared, &incoming_tokens)
    {
        return prepared.tokens.join(" ");
    }
    if let Some(prepared) =
        prepare_prompt_big_output_for_stored_rows(&incoming_prepared, &existing_tokens)
    {
        return prepared.tokens.join(" ");
    }

    let mut left_tokens = existing_tokens;
    let mut right_tokens = incoming_tokens;
    let left_has_reta = matches!(left_tokens.first().map(String::as_str), Some("reta"));
    let right_has_reta = matches!(right_tokens.first().map(String::as_str), Some("reta"));

    if left_has_reta {
        left_tokens.remove(0);
    }
    if right_has_reta {
        right_tokens.remove(0);
    }

    if left_has_reta || right_has_reta {
        let mut merged = vec!["reta".to_string()];
        if right_has_reta && !left_has_reta {
            merged.extend(right_tokens);
            merged.extend(left_tokens);
        } else {
            merged.extend(left_tokens);
            merged.extend(right_tokens);
        }
        return merged.join(" ");
    }

    let mut plain_tokens = Vec::new();
    let mut long_prompt_commands = Vec::new();
    for token in left_tokens.into_iter().chain(right_tokens) {
        if prompt_words().befehle_set.contains(&token) && token.len() > 1 {
            long_prompt_commands.push(token);
        } else {
            plain_tokens.push(token);
        }
    }

    let mut merged = prepare_stored_prefix_tokens(&plain_tokens);
    merged.extend(long_prompt_commands);
    merged.join(" ")
}

fn refresh_stored_placeholder_cache(state: &mut SessionState) {
    state.stored_placeholder = state.stored_placeholder.trim().to_string();
    state.stored_commands = split_storage_text(&state.stored_placeholder);
    state.stored_expanded_tokens =
        prepare_stored_prefix_tokens_from_text(&state.stored_placeholder);
}

fn store_text_in_placeholder(state: &mut SessionState, text: &str) {
    let merged = merge_stored_placeholder(&state.stored_placeholder, text);
    state.stored_placeholder = merged;
    refresh_stored_placeholder_cache(state);
}

fn render_stored_placeholder_text(state: &SessionState) -> String {
    if state.stored_commands.is_empty() {
        return "Noch kein gespeicherter Platzhalter vorhanden.".to_string();
    }

    state
        .stored_commands
        .iter()
        .enumerate()
        .map(|(index, entry)| format!("{:>4}: {}", index + 1, entry))
        .collect::<Vec<_>>()
        .join("\n")
}

fn delete_from_stored_placeholder(state: &mut SessionState, selection_text: &str) {
    let mut tokens = split_storage_text(&state.stored_placeholder);
    let trimmed = selection_text.trim();

    if trimmed.is_empty() || tokens.is_empty() {
        state.prompt_mode = PromptModus::Normal;
        refresh_stored_placeholder_cache(state);
        return;
    }

    let delete_by_index = should_delete_stored_by_index(trimmed, &tokens);
    if delete_by_index {
        if let Some(indexes) = parse_delete_selection_indexes(trimmed) {
            let index_set = indexes
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>();
            tokens = tokens
                .into_iter()
                .enumerate()
                .filter_map(|(index, token)| (!index_set.contains(&(index + 1))).then_some(token))
                .collect();
        }
    } else {
        let delete_tokens = split_storage_text(trimmed);
        tokens.retain(|token| !delete_tokens.iter().any(|delete| delete == token));
    }

    state.stored_placeholder = tokens.join(" ");
    state.prompt_mode = PromptModus::Normal;
    refresh_stored_placeholder_cache(state);
}

fn should_delete_stored_by_index(selection_text: &str, stored_tokens: &[String]) -> bool {
    if selection_text.chars().all(|ch| ch.is_ascii_digit())
        && stored_tokens.iter().any(|token| token == selection_text)
    {
        return false;
    }

    looks_like_numeric_or_fraction_range(selection_text)
}

fn parse_delete_selection_indexes(selection_text: &str) -> Option<Vec<usize>> {
    let numbers = parse_row_numbers_from_tokens(&[selection_text.to_string()])?;
    let mut out = Vec::new();
    for number in numbers {
        if number > 0 {
            out.push(number as usize);
        }
    }
    (!out.is_empty()).then_some(out)
}

fn run_nested_prompt_input(
    input: &str,
    state: &mut SessionState,
) -> Result<Option<PromptOutput>, String> {
    let nested_command = compile_command_inner(input, PromptModus::AusgabeSelektiv)?;
    match nested_command {
        PromptCommand::Noop => Ok(None),
        PromptCommand::Exit => {
            Err("Gespeicherte Platzhalter dürfen keinen Exit-Befehl auslösen.".to_string())
        }
        PromptCommand::SaveBefore
        | PromptCommand::SaveAfter
        | PromptCommand::StoreCurrentInput(_)
        | PromptCommand::StoreInline(_)
        | PromptCommand::DeleteStoredStart
        | PromptCommand::DeleteStoredSelection(_)
        | PromptCommand::ShowStored(_) => Err(
            "Gespeicherte Platzhalter dürfen keine Speicher-Kommandos rekursiv auslösen."
                .to_string(),
        ),
        PromptCommand::LaunchUi => Err(
            "Gespeicherte Platzhalter dürfen die interaktive Vorschau nicht rekursiv starten."
                .to_string(),
        ),
        other => execute_command(other, state),
    }
}

pub fn execute_command(
    command: PromptCommand,
    state: &mut SessionState,
) -> Result<Option<PromptOutput>, String> {
    match command {
        PromptCommand::Noop => Ok(None),
        PromptCommand::Exit => Ok(None),
        PromptCommand::SaveBefore => {
            let previous_input = state.previous_input.trim().to_string();
            if previous_input.is_empty() {
                return Ok(Some(PromptOutput {
                    title: "speichern".to_string(),
                    text: "Kein vorheriger Eingabetext zum Speichern vorhanden.".to_string(),
                    exit_code: 0,
                }));
            }

            store_text_in_placeholder(state, &previous_input);
            state.prompt_mode = PromptModus::Normal;

            Ok(Some(PromptOutput {
                title: "speichern".to_string(),
                text: format!(
                    "Gespeicherter Platzhalter:\n{}",
                    render_stored_placeholder_text(state)
                ),
                exit_code: 0,
            }))
        }
        PromptCommand::SaveAfter => {
            state.prompt_mode = PromptModus::Speichern;
            Ok(Some(PromptOutput {
                title: "speichern".to_string(),
                text: "Speicher-Modus aktiviert. Die nächste Eingabe wird nur im Platzhalter abgelegt.".to_string(),
                exit_code: 0,
            }))
        }
        PromptCommand::StoreCurrentInput(text) | PromptCommand::StoreInline(text) => {
            store_text_in_placeholder(state, &text);
            state.prompt_mode = PromptModus::Normal;
            Ok(Some(PromptOutput {
                title: "speichern".to_string(),
                text: format!(
                    "Gespeicherter Platzhalter:\n{}",
                    render_stored_placeholder_text(state)
                ),
                exit_code: 0,
            }))
        }
        PromptCommand::DeleteStoredStart => {
            state.prompt_mode = PromptModus::LoeschenSelect;
            Ok(Some(PromptOutput {
                title: "loeschen".to_string(),
                text: format!(
                    "Lösch-Modus aktiviert. Welche Einträge sollen entfernt werden?\n{}",
                    render_stored_placeholder_text(state)
                ),
                exit_code: 0,
            }))
        }
        PromptCommand::DeleteStoredSelection(selection_text) => {
            delete_from_stored_placeholder(state, &selection_text);
            let text = if state.stored_commands.is_empty() {
                "Gespeicherter Platzhalter ist jetzt leer.".to_string()
            } else {
                format!(
                    "Gespeicherter Platzhalter nach dem Löschen:\n{}",
                    render_stored_placeholder_text(state)
                )
            };
            Ok(Some(PromptOutput {
                title: "loeschen".to_string(),
                text,
                exit_code: 0,
            }))
        }
        PromptCommand::ShowStored(additional_text) => {
            let effective_input = match additional_text {
                Some(text) => {
                    let additional_tokens = split_storage_text(&text);
                    if let Some(prepared) = prepare_prompt_big_output_for_stored_reta(
                        &state.stored_expanded_tokens,
                        &additional_tokens,
                    ) {
                        prepared.tokens.join(" ")
                    } else if let Some(prepared) = prepare_prompt_big_output_for_stored_rows(
                        &state.stored_expanded_tokens,
                        &additional_tokens,
                    ) {
                        prepared.tokens.join(" ")
                    } else if state.has_stored_placeholder() {
                        compose_input_with_stored_placeholder(
                            &state.stored_expanded_tokens,
                            &additional_tokens,
                        )
                    } else {
                        additional_tokens.join(" ")
                    }
                }
                None if state.has_stored_placeholder() => state.stored_expanded_tokens.join(" "),
                None => String::new(),
            };

            if effective_input.trim().is_empty() {
                return Ok(Some(PromptOutput {
                    title: "stored".to_string(),
                    text: "Noch kein gespeicherter Platzhalter vorhanden.".to_string(),
                    exit_code: 0,
                }));
            }

            run_nested_prompt_input(&effective_input, state)
        }
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
                    if state.logging_enabled {
                        "aktiv"
                    } else {
                        "inaktiv"
                    }
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
        PromptCommand::Immediate(output) => Ok(Some(output)),
        PromptCommand::Reta(argv) => {
            let result: RetaRunResult = run_reta_from_args(argv);
            Ok(Some(PromptOutput {
                title: "reta".to_string(),
                text: result.render_text(),
                exit_code: result.exit_code(),
            }))
        }
        PromptCommand::RetaBatch(argvs) => {
            let mut combined = String::new();
            let mut exit_code = 0;
            for argv in argvs {
                let result: RetaRunResult = run_reta_from_args(argv);
                if !combined.is_empty() && !combined.ends_with('\n') {
                    combined.push('\n');
                }
                combined.push_str(&result.render_text());
                if !combined.ends_with('\n') {
                    combined.push('\n');
                }
                exit_code = exit_code.max(result.exit_code());
            }
            Ok(Some(PromptOutput {
                title: "reta".to_string(),
                text: combined.trim_end_matches('\n').to_string(),
                exit_code,
            }))
        }
    }
}

fn parse_row_numbers_from_tokens(tokens: &[String]) -> Option<Vec<i64>> {
    let mut out: Vec<i64> = Vec::new();
    for token in tokens {
        if token.contains('/') {
            continue;
        }
        for part in token.split(',').filter(|p| !p.trim().is_empty()) {
            let part = part.trim();
            if let Some((a, b)) = part.split_once('-') {
                if !a.is_empty()
                    && !b.is_empty()
                    && a.chars().all(|c| c.is_ascii_digit())
                    && b.chars().all(|c| c.is_ascii_digit())
                {
                    let start: i64 = a.parse().ok()?;
                    let end: i64 = b.parse().ok()?;
                    if start <= end {
                        for n in start..=end {
                            out.push(n);
                        }
                    } else {
                        for n in (end..=start).rev() {
                            out.push(n);
                        }
                    }
                    continue;
                }
            }
            if part.chars().all(|c| c.is_ascii_digit()) {
                out.push(part.parse().ok()?);
            }
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn prime_factors(n: i64, modulo24: bool) -> Vec<i64> {
    let mut z = n.abs();
    let mut factors = Vec::new();
    if z <= 1 {
        return factors;
    }
    while z > 1 {
        let mut i = 2;
        let mut found = None;
        while i * i <= z {
            if z % i == 0 {
                found = Some(i);
                break;
            }
            i += 1;
        }
        let p = found.unwrap_or(z);
        factors.push(if modulo24 { p % 24 } else { p });
        z /= p;
    }
    factors
}

fn prime_repeat_display(mut factors: Vec<i64>) -> String {
    if factors.is_empty() {
        return String::new();
    }
    factors.reverse();
    let mut grouped: Vec<(i64, i64)> = Vec::new();
    let mut prev: Option<i64> = None;
    let mut count = 0i64;
    for a in factors {
        if prev == Some(a) {
            count += 1;
        } else {
            if let Some(p) = prev {
                grouped.push((p, count));
            }
            prev = Some(a);
            count = 1;
        }
    }
    if let Some(p) = prev {
        grouped.push((p, count));
    }
    grouped.reverse();
    grouped
        .into_iter()
        .map(|(e, g)| {
            if g == 1 {
                e.to_string()
            } else {
                format!("{e}^{g}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn factor_pairs(a: i64) -> Vec<(i64, i64)> {
    let mut pairs = Vec::new();
    if a <= 0 {
        return pairs;
    }
    let mut b = 2i64;
    while b <= (a as f64).sqrt().floor() as i64 {
        if a % b == 0 {
            pairs.push((a / b, b));
        }
        b += 1;
    }
    pairs.push((a, 1));
    pairs
}

fn factor_pairs_without_ones(a: i64) -> Vec<(i64, i64)> {
    factor_pairs(a)
        .into_iter()
        .filter(|(x, y)| *x != 1 && *y != 1)
        .collect()
}

fn factor_triples(a: i64) -> Vec<(i64, i64, i64)> {
    let mut set = std::collections::BTreeSet::new();
    for (m0, m1) in factor_pairs(a) {
        let (o, n) = if m0 > m1 { (m0, m1) } else { (m1, m0) };
        for (a1, b1) in factor_pairs(o) {
            let mut v = [n, a1, b1];
            v.sort();
            if !v.contains(&1) {
                set.insert((v[0], v[1], v[2]));
            }
        }
    }
    set.into_iter().collect()
}

fn modulo_remainders_display(n: i64) -> String {
    if n == 0 {
        return "0: Divisionen nicht definiert".to_string();
    }
    let upper = n.abs();
    let parts = (1..=upper)
        .map(|divisor| format!("{}→{}", divisor, n.rem_euclid(divisor)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{}: {}", n, parts)
}

fn compile_direct_number_command(tokens: &[String]) -> Option<PromptOutput> {
    if tokens.iter().any(|t| t == "abc" || t == "abcd") {
        return None;
    }
    let numbers = parse_row_numbers_from_tokens(tokens)?;
    let token_set = tokens
        .iter()
        .map(|s| s.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut lines: Vec<String> = Vec::new();
    let mut matched = false;

    if token_set.contains("prim") || token_set.contains("primfaktorzerlegung") {
        matched = true;
        for n in &numbers {
            lines.push(format!(
                "{}: {}",
                n,
                prime_repeat_display(prime_factors(*n, false))
            ));
        }
    }
    if token_set.contains("prim24") || token_set.contains("primfaktorzerlegungModulo24") {
        matched = true;
        for n in &numbers {
            lines.push(format!(
                "{}: {}",
                n,
                prime_repeat_display(prime_factors(*n, true))
            ));
        }
    }
    if token_set.contains("multis") {
        matched = true;
        let mulpri_info = !(token_set.contains("mulpri") || token_set.contains("p"));
        for n in &numbers {
            let pairs = factor_pairs_without_ones(*n);
            if !pairs.is_empty() || mulpri_info {
                lines.push(format!("{}: {:?}", n, pairs));
            } else {
                lines.push(format!("{}: {} (Primzahl)", n, n));
            }
        }
    }
    if token_set.contains("multis3") {
        matched = true;
        for n in &numbers {
            lines.push(format!("{}: {:?}", n, factor_triples(*n)));
        }
    }
    if token_set.contains("primfaktorenvergleich") && !numbers.is_empty() {
        matched = true;
        let mut common = prime_factors(numbers[0], false);
        for n in numbers.iter().skip(1) {
            let mut next = prime_factors(*n, false);
            let mut out = Vec::new();
            for c in common {
                if let Some(pos) = next.iter().position(|x| *x == c) {
                    out.push(c);
                    next.remove(pos);
                }
            }
            common = out;
        }
        let product = common
            .iter()
            .copied()
            .fold(1i64, |acc, x| acc.saturating_mul(x));
        let common_text = if common.is_empty() {
            "1".to_string()
        } else {
            prime_repeat_display(common.clone())
        };
        lines.push(format!("gemeinsame Primfaktoren: {}", common_text));
        lines.push(format!("ggT: {}", product));
    }
    if token_set.contains("modulo") {
        matched = true;
        for n in &numbers {
            lines.push(modulo_remainders_display(*n));
        }
    }
    if token_set.contains("abstand") || token_set.contains("abstandPrim") {
        matched = true;
        if numbers.len() > 1 {
            let anchor = *numbers.iter().max().unwrap();
            for n in &numbers {
                if *n == anchor {
                    continue;
                }
                if token_set.contains("abstand") {
                    lines.push(format!("{}->: {}: {}", n, anchor, (anchor - *n).abs()));
                }
                if token_set.contains("abstandPrim") {
                    let diff = (anchor - *n).abs();
                    lines.push(format!(
                        "{}->: {}: {}",
                        n,
                        anchor,
                        prime_repeat_display(prime_factors(diff, false))
                    ));
                }
            }
        }
    }

    if !matched {
        return None;
    }

    Some(PromptOutput {
        title: "prompt-zahlen".to_string(),
        text: lines.join("\n"),
        exit_code: 0,
    })
}

fn compile_abc_abcd_command(tokens: &[String]) -> Option<PromptOutput> {
    if tokens.len() != 2 {
        return None;
    }
    let is_abc = tokens.iter().any(|t| t == "abc" || t == "abcd");
    if !is_abc {
        return None;
    }
    let buchstaben = if tokens[0] == "abc" || tokens[0] == "abcd" {
        tokens[1].clone()
    } else {
        tokens[0].clone()
    };
    let converted = buchstaben
        .chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .map(|ch| ((ch.to_ascii_lowercase() as u8) - b'a' + 1).to_string())
        .collect::<Vec<_>>()
        .join(" ");
    Some(PromptOutput {
        title: "abc".to_string(),
        text: converted,
        exit_code: 0,
    })
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

pub fn apply_storage_mode(_state: &mut SessionState, argv: Vec<String>) -> Vec<String> {
    argv
}

pub fn help_text() -> String {
    [
        "rp / rpl / rpe / rpb – Prompt-Schicht für reta",
        "",
        "Grundidee:",
        "  - 'reta ...' führt den normalen reta-Befehl in der eingebetteten Lib aus.",
        "  - Eine Eingabe, die direkt mit '-zeilen', '-spalten', '--...' beginnt,",
        "    wird automatisch als reta-Befehl behandelt.",
        "  - Kurzbefehle wie 'a1/2', 'u 3/4', 'G7', 'mond 12' werden Python-nah in reta-Argumente expandiert.",
        "  - ':ui' öffnet die ratatui-Ansicht mit Vorschau, History, Kandidaten und Status.",
        "",
        "Startup-Argumente:",
        "  -vi                  startet im Vi-Modus",
        "  -log                 aktiviert Logging sofort",
        "  -e                   ergänzt Python-nah",
        "                       'keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar'",
        "                       bei normalen Prompt-Ausgabebefehlen",
        "  -befehl <text>       führt den Rest als One-Shot-Befehl aus",
        "  -command <text>      Alias zu -befehl",
        "  -h | -help | --help  zeigt diese Hilfe",
        "  -debug               reserviert Debug-Verhalten; bei rpl unterdrückt es das implizite -e",
        "",
        "Python-nahe Prompt-Befehle:",
        "  s / BefehlSpeichernDavor      vorherige Eingabe sofort in den Platzhalter übernehmen",
        "  S / BefehlSpeichernDanach     nächste Eingabe nur im Platzhalter speichern",
        "  l / BefehlSpeicherungLöschen  Lösch-Auswahl für den Platzhalter starten",
        "  o / BefehlSpeicherungAusgeben gespeicherten Platzhalter ausführen",
        "  Enter bei gespeichertem Platzhalter  führt denselben Platzhalter ebenfalls aus",
        "  loggen | nichtloggen          Logging umschalten",
        "",
        "Meta-Befehle:",
        "  help | hilfe            Diese Hilfe",
        "  befehle | kurzbefehle   Befehlsübersicht",
        "  :ui | :preview          Vollbild-Vorschau mit ratatui",
        "  :history                Zeigt die Sitzungs-History",
        "  :mode vi                Wechselt reedline in den Vi-Modus",
        "  :mode emacs             Wechselt reedline in den Emacs-Modus",
        "  clear | leeren          Terminal leeren",
        "  q | :q | exit | quit    rp beenden",
        "",
        "Autocomplete:",
        "  Tab                     Vorschlagsliste öffnen / nächsten Kandidaten wählen",
        "  Shift+Tab               vorherigen Kandidaten wählen",
        "  Pfeiltasten             im geöffneten Completion-Menü navigieren",
        "  r\"absi\"               filtert Kandidaten Python-nah per Muster/Teiltreffer",
        "  * / -*                  zeigt alle bzw. alle negativen Kandidaten im aktuellen Kontext",
        "",
        "Spezial:",
        "  shell <cmd>             Führt einen Shell-Befehl aus",
        "  python <code>           Führt Python-Code aus",
        "  math <expr>             Wertet einen Python-Ausdruck aus",
        "",
        "Beispiele:",
        "  rp",
        "  rpl",
        "  rpl -debug",
        "  rpb av12-15",
        "  rpe av12-15",
        "  rp -e -befehl av12-15",
        "  rp -befehl \"reta -zeilen --vorhervonausschnitt=1-3 -spalten --alles\"",
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

#[cfg(test)]
mod tests {
    use super::{
        compile_command_with_state, merge_stored_placeholder, refresh_stored_placeholder_cache,
        PromptCommand, SessionState,
    };

    #[test]
    fn empty_input_executes_stored_placeholder_like_python_prompt() {
        let mut state = SessionState::new("rp".to_string(), true, false);
        state.stored_placeholder = "reta -zeilen --zeit=heute".to_string();
        refresh_stored_placeholder_cache(&mut state);

        let command = compile_command_with_state("", &state).unwrap();
        assert!(matches!(command, PromptCommand::ShowStored(None)));
    }

    #[test]
    fn empty_input_without_placeholder_stays_noop() {
        let state = SessionState::new("rp".to_string(), true, false);
        let command = compile_command_with_state("", &state).unwrap();
        assert!(matches!(command, PromptCommand::Noop));
    }

    #[test]
    fn merge_stored_placeholder_prefers_incoming_reta_order_like_python() {
        let merged = merge_stored_placeholder("emotion 12", "reta -spalten --geist");
        assert_eq!(merged, "reta -spalten --geist emotion 12");
    }

    #[test]
    fn stored_reta_placeholder_row_input_rewrites_zeilen_section() {
        let mut state = SessionState::new("rp".to_string(), true, false);
        state.stored_placeholder = "reta -zeilen --zeit=heute -spalten --thomas".to_string();
        refresh_stored_placeholder_cache(&mut state);

        let command = compile_command_with_state("12-15", &state).unwrap();
        match command {
            PromptCommand::Reta(argv) => assert_eq!(
                argv,
                vec![
                    "reta".to_string(),
                    "-zeilen".to_string(),
                    "--vorhervonausschnitt=12-15".to_string(),
                    "--oberesmaximum=1025".to_string(),
                    "-spalten".to_string(),
                    "--thomas".to_string(),
                ]
            ),
            other => panic!("expected PromptCommand::Reta, got {other:?}"),
        }
    }

    #[test]
    fn stored_row_placeholder_injects_rows_into_incoming_reta_command() {
        let mut state = SessionState::new("rp".to_string(), true, false);
        state.stored_placeholder = "4,7-10 ee".to_string();
        refresh_stored_placeholder_cache(&mut state);

        let command = compile_command_with_state("reta -spalten --licht", &state).unwrap();
        match command {
            PromptCommand::Reta(argv) => assert_eq!(
                argv,
                vec![
                    "reta".to_string(),
                    "-zeilen".to_string(),
                    "--vorhervonausschnitt=4,7-10".to_string(),
                    "--oberesmaximum=1025".to_string(),
                    "-spalten".to_string(),
                    "--licht".to_string(),
                    "-ausgabe".to_string(),
                    "--keineueberschriften".to_string(),
                ]
            ),
            other => panic!("expected PromptCommand::Reta, got {other:?}"),
        }
    }

    #[test]
    fn merge_stored_row_placeholder_with_reta_normalizes_to_real_reta_command() {
        let merged = merge_stored_placeholder("4,7-10", "-spalten --licht");
        assert_eq!(
            merged,
            "reta -zeilen --vorhervonausschnitt=4,7-10 --oberesmaximum=1025 -spalten --licht"
        );
    }
}
