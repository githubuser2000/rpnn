use crate::{run_reta_from_args, RetaRunResult};

use super::semantic_choices::{RETAPROMPT_RETA_MAIN_SWITCHES, RETAPROMPT_RETA_SECTION_SWITCHES};
use super::python_like::{
    libreta_prompt_custom_split,
    prepare_prompt_big_output_for_stored_reta,
    python_row_spec_to_numbers, prepare_prompt_big_output_for_stored_reta_prompt_overlay,
    prepare_prompt_big_output_for_stored_rows, prompt_words, PromptGrosseAusgabe,
    PromptLoescheVorSpeicherungBefehle, PromptModus,
    PromptSonderBefehlAktion, PromptVonGrosserAusgabeSonderBefehlAusgaben,
};

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
    EnterStoredOutputMode(Option<String>),
    ShowStored(Option<String>),
    Clear,
    LaunchUi,
    PrintHelp,
    PrintCommands,
    PrintHistory,
    SwitchMode(EditModeKind),
    ToggleLogging(bool),
    Shell(Vec<String>),
    Python(String),
    Math(String),
    Immediate(PromptOutput),
    Sequence(Vec<PromptCommand>),
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
    pub pending_show_stored_suffix: Option<String>,
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
            pending_show_stored_suffix: None,
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

fn raw_prompt_tokens_like_python(text: &str) -> Vec<String> {
    libreta_prompt_custom_split(text)
}

fn compile_python_process_command_from_tokens(tokens: &[String]) -> Option<PromptCommand> {
    let decision = PromptVonGrosserAusgabeSonderBefehlAusgaben(false, tokens, false);
    let command = match decision.aktion? {
        PromptSonderBefehlAktion::Shell(args) => PromptCommand::Shell(args),
        PromptSonderBefehlAktion::Python(command_text) => PromptCommand::Python(command_text),
        PromptSonderBefehlAktion::Math(command_text) => PromptCommand::Math(command_text),
    };
    Some(append_sonder_logging_toggle_like_python(command, tokens))
}

fn compile_python_process_command_from_raw_text(text: &str) -> Option<PromptCommand> {
    let tokens = raw_prompt_tokens_like_python(text);
    compile_python_process_command_from_tokens(&tokens)
}

fn compile_normalized_control_command(tokens: &[String]) -> Option<PromptCommand> {
    match tokens {
        [single] => match single.as_str() {
            "q" | ":q" | "exit" | "quit" | "ende" => Some(PromptCommand::Exit),
            "help" | "hilfe" => Some(PromptCommand::PrintHelp),
            "befehle" | "kurzbefehle" => Some(PromptCommand::PrintCommands),
            "BefehlSpeichernDavor" => Some(PromptCommand::SaveBefore),
            "BefehlSpeichernDanach" => Some(PromptCommand::SaveAfter),
            "BefehlSpeicherungLöschen" => Some(PromptCommand::DeleteStoredStart),
            "BefehlSpeicherungAusgeben" => Some(PromptCommand::EnterStoredOutputMode(None)),
            "leeren" | "clear" => Some(PromptCommand::Clear),
            "loggen" => Some(PromptCommand::ToggleLogging(true)),
            "nichtloggen" => Some(PromptCommand::ToggleLogging(false)),
            _ => None,
        },
        [mode, value] if mode == ":mode" && value == "vi" => {
            Some(PromptCommand::SwitchMode(EditModeKind::Vi))
        }
        [mode, value] if mode == ":mode" && value == "emacs" => {
            Some(PromptCommand::SwitchMode(EditModeKind::Emacs))
        }
        _ => None,
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
        "o" | "BefehlSpeicherungAusgeben" => {
            return Ok(PromptCommand::EnterStoredOutputMode(None))
        }
        "leeren" | "clear" => return Ok(PromptCommand::Clear),
        ":ui" | ":preview" => return Ok(PromptCommand::LaunchUi),
        ":history" => return Ok(PromptCommand::PrintHistory),
        ":mode vi" => return Ok(PromptCommand::SwitchMode(EditModeKind::Vi)),
        ":mode emacs" => return Ok(PromptCommand::SwitchMode(EditModeKind::Emacs)),
        "loggen" => return Ok(PromptCommand::ToggleLogging(true)),
        "nichtloggen" => return Ok(PromptCommand::ToggleLogging(false)),
        _ => {}
    }

    if let Some(command) = compile_python_process_command_from_raw_text(trimmed) {
        return Ok(command);
    }

    let tokens = raw_prompt_tokens_like_python(trimmed);
    if tokens.is_empty() {
        return Ok(PromptCommand::Noop);
    }

    let grosse_ausgabe = PromptGrosseAusgabe(
        "",
        prompt_mode,
        prompt_mode,
        PromptModus::Normal,
        trimmed,
        &[],
    );
    let effective_tokens = grosse_ausgabe.liste.clone();
    if effective_tokens.is_empty() {
        return Ok(PromptCommand::Noop);
    }

    if let Some(command) = compile_normalized_control_command(&effective_tokens) {
        return Ok(command);
    }
    if let Some(command) = compile_python_process_command_from_tokens(&effective_tokens) {
        return Ok(command);
    }
    let direct_number_output = compile_direct_number_command(&effective_tokens);
    if let Some(output) = compile_abc_abcd_command(&effective_tokens) {
        return Ok(PromptCommand::Immediate(output));
    }
    if effective_tokens[0] == "reta" {
        return Ok(append_sonder_logging_toggle_like_python(
            PromptCommand::Reta(effective_tokens.clone()),
            &effective_tokens,
        ));
    }
    if effective_tokens[0].starts_with('-') {
        let mut argv = vec!["reta".to_string()];
        argv.extend(effective_tokens.clone());
        return Ok(append_sonder_logging_toggle_like_python(
            PromptCommand::Reta(argv),
            &effective_tokens,
        ));
    }
    let calls = grosse_ausgabe.retaCalls.clone();
    if !calls.is_empty() {
        let command = append_direct_number_output_like_python(
            prompt_command_from_reta_calls(calls),
            direct_number_output,
        );
        return Ok(append_sonder_logging_toggle_like_python(command, &effective_tokens));
    }
    if let Some(output) = direct_number_output {
        return Ok(append_sonder_logging_toggle_like_python(
            PromptCommand::Immediate(output),
            &effective_tokens,
        ));
    }
    if let Some(argv) = grosse_ausgabe.retaArgv.clone() {
        return Ok(append_sonder_logging_toggle_like_python(
            PromptCommand::Reta(argv),
            &effective_tokens,
        ));
    }
    if let Some(enabled) = PromptVonGrosserAusgabeSonderBefehlAusgaben(false, &effective_tokens, false).loggingCommand {
        return Ok(PromptCommand::ToggleLogging(enabled));
    }

    Err(format!(
        "Unbekannter rp-Befehl: {trimmed}\nVersuche 'help', 'befehle', ':ui' oder beginne mit 'reta ...' bzw. '-zeilen ...'."
    ))
}

pub fn compile_command(input: &str, prompt_mode: PromptModus) -> Result<PromptCommand, String> {
    compile_command_inner(input, prompt_mode)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PromptScopeRoute {
    EmptyRunsStored,
    EmptyNoop,
    SpeichernInput,
    LoeschenInput,
    ProcessCommand,
    InlineStorageCommand,
    StoredRetaRowRewrite,
    StoredRowsIntoRawReta,
    StoredRetaPromptOverlay,
    BypassStoredMerge,
    MergeWithStored,
}

#[allow(non_snake_case)]
fn PromptScope_route_for_input(
    trimmed: &str,
    tokens: &[String],
    state: &SessionState,
) -> PromptScopeRoute {
    if trimmed.is_empty() {
        if state.has_stored_placeholder()
            && matches!(
                state.prompt_mode,
                PromptModus::Normal | PromptModus::AusgabeSelektiv
            )
        {
            return PromptScopeRoute::EmptyRunsStored;
        }
        return PromptScopeRoute::EmptyNoop;
    }

    // Python `PromptScope()` runs `promptSpeicherungA()` immediately after
    // reading the next line while the prompt is in speichern-mode. Process,
    // control and reta commands must therefore be stored as plain text here.
    if matches!(state.prompt_mode, PromptModus::Speichern) {
        return PromptScopeRoute::SpeichernInput;
    }

    if matches!(
        state.prompt_mode,
        PromptModus::LoeschenStart | PromptModus::LoeschenSelect
    ) {
        return PromptScopeRoute::LoeschenInput;
    }

    if compile_python_process_command_from_tokens(tokens).is_some() {
        return PromptScopeRoute::ProcessCommand;
    }

    if tokens.is_empty() {
        return PromptScopeRoute::EmptyNoop;
    }

    if compile_inline_storage_command(tokens).is_some() {
        return PromptScopeRoute::InlineStorageCommand;
    }

    if prepare_prompt_big_output_for_stored_reta(&state.stored_expanded_tokens, tokens).is_some() {
        return PromptScopeRoute::StoredRetaRowRewrite;
    }
    if prepare_prompt_big_output_for_stored_rows(&state.stored_expanded_tokens, tokens).is_some() {
        return PromptScopeRoute::StoredRowsIntoRawReta;
    }
    if prepare_prompt_big_output_for_stored_reta_prompt_overlay(
        &state.stored_expanded_tokens,
        tokens,
    )
    .is_some()
    {
        return PromptScopeRoute::StoredRetaPromptOverlay;
    }

    if raw_input_bypasses_stored_merge(trimmed, tokens) || !state.has_stored_placeholder() {
        return PromptScopeRoute::BypassStoredMerge;
    }

    PromptScopeRoute::MergeWithStored
}

pub fn compile_command_with_state(
    input: &str,
    state: &SessionState,
) -> Result<PromptCommand, String> {
    let trimmed = input.trim();
    let tokens = if trimmed.is_empty() {
        Vec::new()
    } else {
        raw_prompt_tokens_like_python(trimmed)
    };

    match PromptScope_route_for_input(trimmed, &tokens, state) {
        PromptScopeRoute::EmptyRunsStored => Ok(PromptCommand::ShowStored(None)),
        PromptScopeRoute::EmptyNoop => Ok(PromptCommand::Noop),
        PromptScopeRoute::SpeichernInput => Ok(PromptCommand::StoreCurrentInput(trimmed.to_string())),
        PromptScopeRoute::LoeschenInput => Ok(PromptCommand::DeleteStoredSelection(trimmed.to_string())),
        PromptScopeRoute::ProcessCommand => Ok(
            compile_python_process_command_from_raw_text(trimmed)
                .expect("PromptScope route checked process command"),
        ),
        PromptScopeRoute::InlineStorageCommand => Ok(
            compile_inline_storage_command(&tokens)
                .expect("PromptScope route checked inline storage command"),
        ),
        PromptScopeRoute::StoredRetaRowRewrite => {
            let prepared = prepare_prompt_big_output_for_stored_reta(
                &state.stored_expanded_tokens,
                &tokens,
            )
            .expect("PromptScope route checked stored reta row rewrite");
            Ok(PromptCommand::Reta(prepared.tokens))
        }
        PromptScopeRoute::StoredRowsIntoRawReta => {
            let prepared = prepare_prompt_big_output_for_stored_rows(
                &state.stored_expanded_tokens,
                &tokens,
            )
            .expect("PromptScope route checked stored rows into raw reta");
            Ok(PromptCommand::Reta(prepared.tokens))
        }
        PromptScopeRoute::StoredRetaPromptOverlay => {
            let calls = prepare_prompt_big_output_for_stored_reta_prompt_overlay(
                &state.stored_expanded_tokens,
                &tokens,
            )
            .expect("PromptScope route checked stored reta prompt overlay");
            Ok(prompt_command_from_reta_calls(calls))
        }
        PromptScopeRoute::BypassStoredMerge => compile_command_inner(trimmed, state.prompt_mode),
        PromptScopeRoute::MergeWithStored => {
            let effective_input =
                compose_input_with_stored_placeholder(&state.stored_expanded_tokens, &tokens);
            compile_command_inner(&effective_input, PromptModus::AusgabeSelektiv)
        }
    }
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
        return Some(PromptCommand::EnterStoredOutputMode(
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

    compile_python_process_command_from_tokens(tokens).is_some()
        || matches!(tokens.first().map(String::as_str), Some(":mode"))
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

fn normalize_optional_storage_payload(text: Option<String>) -> Option<String> {
    text.and_then(|payload| {
        let trimmed = payload.trim();
        (!trimmed.is_empty()).then_some(trimmed.to_string())
    })
}

fn enter_stored_output_mode(state: &mut SessionState, additional_text: Option<String>) {
    state.pending_show_stored_suffix = normalize_optional_storage_payload(additional_text);
    state.prompt_mode = if state.pending_show_stored_suffix.is_some() {
        PromptModus::SpeicherungAusgabenMitZusatz
    } else {
        PromptModus::SpeicherungAusgaben
    };
}

pub fn take_auto_prompt_command(state: &mut SessionState) -> Option<PromptCommand> {
    let additional_text = state.pending_show_stored_suffix.take();
    let command = match state.prompt_mode {
        PromptModus::SpeicherungAusgaben => Some(PromptCommand::ShowStored(None)),
        PromptModus::SpeicherungAusgabenMitZusatz => {
            Some(PromptCommand::ShowStored(additional_text))
        }
        _ => None,
    }?;

    state.prompt_mode = PromptModus::Normal;
    Some(command)
}

fn nested_input_starts_with_reta(input: &str) -> bool {
    matches!(
        raw_prompt_tokens_like_python(input.trim()).first(),
        Some(token) if token == "reta"
    )
}

fn rpe_output_group_for_nested_execution() -> Vec<String> {
    vec![
        "-ausgabe".to_string(),
        "--art=emacs".to_string(),
        "--keineueberschriften".to_string(),
    ]
}

fn apply_rpe_emacs_output_to_nested_argv(
    mut argv: Vec<String>,
    append_after_user_args: bool,
) -> Vec<String> {
    let output_group = rpe_output_group_for_nested_execution();

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

fn apply_nested_frontend_overrides(
    command: PromptCommand,
    input: &str,
    state: &SessionState,
) -> PromptCommand {
    if state.program_name != "rpe" {
        return command;
    }

    let append_after_user_args = nested_input_starts_with_reta(input);
    match command {
        PromptCommand::Reta(argv) => PromptCommand::Reta(apply_rpe_emacs_output_to_nested_argv(
            argv,
            append_after_user_args,
        )),
        PromptCommand::RetaBatch(argvs) => PromptCommand::RetaBatch(
            argvs
                .into_iter()
                .map(|argv| {
                    apply_rpe_emacs_output_to_nested_argv(argv, append_after_user_args)
                })
                .collect(),
        ),
        PromptCommand::Sequence(commands) => PromptCommand::Sequence(
            commands
                .into_iter()
                .map(|command| apply_nested_frontend_overrides(command, input, state))
                .collect(),
        ),
        other => other,
    }
}

fn prompt_command_from_reta_calls(calls: Vec<Vec<String>>) -> PromptCommand {
    if calls.len() == 1 {
        PromptCommand::Reta(calls.into_iter().next().unwrap())
    } else {
        PromptCommand::RetaBatch(calls)
    }
}

fn append_direct_number_output_like_python(
    command: PromptCommand,
    direct_number_output: Option<PromptOutput>,
) -> PromptCommand {
    match direct_number_output {
        Some(output) => PromptCommand::Sequence(vec![command, PromptCommand::Immediate(output)]),
        None => command,
    }
}

fn append_sonder_logging_toggle_like_python(
    command: PromptCommand,
    tokens: &[String],
) -> PromptCommand {
    let decision = PromptVonGrosserAusgabeSonderBefehlAusgaben(false, tokens, false);
    let Some(enabled) = decision.loggingCommand else {
        return command;
    };

    match command {
        PromptCommand::ToggleLogging(_) => PromptCommand::ToggleLogging(enabled),
        PromptCommand::Sequence(mut commands) => {
            if !commands
                .iter()
                .any(|command| matches!(command, PromptCommand::ToggleLogging(_)))
            {
                commands.push(PromptCommand::ToggleLogging(enabled));
            }
            PromptCommand::Sequence(commands)
        }
        other => PromptCommand::Sequence(vec![other, PromptCommand::ToggleLogging(enabled)]),
    }
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

    raw_prompt_tokens_like_python(trimmed)
}

fn prepare_stored_prefix_tokens_from_text(text: &str) -> Vec<String> {
    let tokens = split_storage_text(text);
    prepare_stored_prefix_tokens(&tokens)
}

fn prepare_stored_prefix_tokens(tokens: &[String]) -> Vec<String> {
    if tokens.is_empty() {
        return Vec::new();
    }

    let prepared = PromptGrosseAusgabe(
        "",
        PromptModus::AusgabeSelektiv,
        PromptModus::AusgabeSelektiv,
        PromptModus::Normal,
        &tokens.join(" "),
        &[],
    );
    prepared.liste
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
        // Python `speichern()` keeps the already stored placeholder before the
        // newly saved text once a leading `reta` marker has been stripped from
        // either side:
        //     "reta " + " ".join(TxtPlatzhalter.liste) + " " + " ".join(Txt.liste)
        // An incoming `reta ...` selects the raw-reta storage path, but it does
        // not reverse stored and incoming tokens. The execution-time swap is
        // handled separately by `compose_input_with_stored_placeholder()`, which
        // mirrors Python `verdreheWoReTaBefehl()`.
        merged.extend(left_tokens);
        merged.extend(right_tokens);
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
    let result = PromptLoescheVorSpeicherungBefehle(
        &state.stored_placeholder,
        state.prompt_mode,
        selection_text,
    );
    state.stored_placeholder = result.platzhalter;
    state.prompt_mode = result.promptMode;
    refresh_stored_placeholder_cache(state);
}


#[allow(non_snake_case)]
fn speichern(state: &mut SessionState, text: &str) -> PromptOutput {
    store_text_in_placeholder(state, text);
    state.prompt_mode = PromptModus::Normal;
    PromptOutput {
        title: "speichern".to_string(),
        text: format!(
            "Gespeicherter Platzhalter:\n{}",
            render_stored_placeholder_text(state)
        ),
        exit_code: 0,
    }
}

#[allow(non_snake_case)]
fn promptSpeicherungA(state: &mut SessionState, text: &str) -> PromptOutput {
    speichern(state, text)
}

#[allow(non_snake_case)]
fn promptSpeicherungB(state: &SessionState, additional_text: Option<String>) -> String {
    match additional_text {
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
    }
}

fn run_nested_prompt_input(
    input: &str,
    state: &mut SessionState,
) -> Result<Option<PromptOutput>, String> {
    let nested_command = apply_nested_frontend_overrides(
        compile_command_inner(input, PromptModus::AusgabeSelektiv)?,
        input,
        state,
    );
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
        | PromptCommand::EnterStoredOutputMode(_)
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

            Ok(Some(promptSpeicherungA(state, &previous_input)))
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
            Ok(Some(promptSpeicherungA(state, &text)))
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
        PromptCommand::EnterStoredOutputMode(additional_text) => {
            enter_stored_output_mode(state, additional_text);
            Ok(None)
        }
        PromptCommand::ShowStored(additional_text) => {
            if let Some(text) = &additional_text {
                let additional_tokens = split_storage_text(text);
                if let Some(calls) = prepare_prompt_big_output_for_stored_reta_prompt_overlay(
                    &state.stored_expanded_tokens,
                    &additional_tokens,
                ) {
                    let nested_command = apply_nested_frontend_overrides(
                        prompt_command_from_reta_calls(calls),
                        text,
                        state,
                    );
                    return execute_command(nested_command, state);
                }
            }

            let effective_input = promptSpeicherungB(state, additional_text);

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
        PromptCommand::Shell(args) => {
            let Some((program, rest)) = args.split_first() else {
                return Err("Nach 'shell' fehlt der eigentliche Shell-Befehl".to_string());
            };
            let output = std::process::Command::new(program)
                .args(rest)
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
        PromptCommand::Sequence(commands) => execute_command_sequence(commands, state),
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

fn append_output_text(combined: &mut String, text: &str) {
    if text.is_empty() {
        return;
    }
    if !combined.is_empty() && !combined.ends_with('\n') {
        combined.push('\n');
    }
    combined.push_str(text);
    if !combined.ends_with('\n') {
        combined.push('\n');
    }
}

fn execute_command_sequence(
    commands: Vec<PromptCommand>,
    state: &mut SessionState,
) -> Result<Option<PromptOutput>, String> {
    let mut combined = String::new();
    let mut exit_code = 0;
    let mut titles: Vec<String> = Vec::new();

    for command in commands {
        if let Some(output) = execute_command(command, state)? {
            if !output.title.trim().is_empty() {
                titles.push(output.title.clone());
            }
            append_output_text(&mut combined, &output.text);
            exit_code = exit_code.max(output.exit_code);
        }
    }

    if combined.is_empty() && titles.is_empty() {
        Ok(None)
    } else {
        Ok(Some(PromptOutput {
            title: if titles.is_empty() {
                "prompt".to_string()
            } else {
                titles.join("+")
            },
            text: combined.trim_end_matches('\n').to_string(),
            exit_code,
        }))
    }
}

fn parse_row_numbers_from_tokens(tokens: &[String]) -> Option<Vec<i64>> {
    let mut out: Vec<i64> = Vec::new();
    for token in tokens {
        if token.contains('/') {
            continue;
        }

        if let Some(numbers) = python_row_spec_to_numbers(token) {
            out.extend(numbers);
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

fn prime_factor_product_display(factors: &[i64]) -> String {
    if factors.is_empty() {
        "1".to_string()
    } else {
        factors
            .iter()
            .map(|factor| factor.to_string())
            .collect::<Vec<_>>()
            .join(" * ")
    }
}

fn unique_numbers_preserving_prompt_order(numbers: &[i64]) -> Vec<i64> {
    let mut seen = std::collections::BTreeSet::new();
    let mut unique = Vec::new();
    for n in numbers {
        if seen.insert(*n) {
            unique.push(*n);
        }
    }
    unique
}

fn common_prime_factor_multiset(numbers: &[i64]) -> Vec<i64> {
    let Some((first, rest)) = numbers.split_first() else {
        return Vec::new();
    };
    let mut common = prime_factors(*first, false);
    for n in rest {
        let mut next = prime_factors(*n, false);
        let mut intersection = Vec::new();
        for candidate in common {
            if let Some(position) = next.iter().position(|value| *value == candidate) {
                intersection.push(candidate);
                next.remove(position);
            }
        }
        common = intersection;
    }
    common
}

fn render_primfaktorenvergleich_like_python(numbers: &[i64]) -> Vec<String> {
    let common = common_prime_factor_multiset(numbers);
    let product = common
        .iter()
        .copied()
        .fold(1i64, |acc, factor| acc.saturating_mul(factor));
    let mut lines = vec![format!(
        "Gemeinsamkeiten: {} := {}",
        product,
        prime_factor_product_display(&common)
    )];

    for n in numbers {
        let quotient = if product == 0 { *n } else { *n / product };
        let remaining = prime_factors(quotient, false);
        lines.push(format!(
            "{:<5} := {:<5} / {:<5} -> {}",
            quotient,
            n,
            product,
            prime_factor_product_display(&remaining)
        ));
    }
    lines
}


const PY_SET_LINEAR_PROBES: usize = 9;
const PY_SET_PERTURB_SHIFT: u32 = 5;
const PY_HASH_XXPRIME_1: u64 = 11_400_714_785_074_694_791;
const PY_HASH_XXPRIME_2: u64 = 14_029_467_366_897_019_727;
const PY_HASH_XXPRIME_5: u64 = 2_870_177_450_012_600_261;

fn python_int_hash_bits(value: i64) -> u64 {
    // CPython hashes small/medium ints as their integer value, except -1 is
    // remapped to -2 because -1 is the C-level error sentinel.
    if value == -1 {
        (-2i64) as u64
    } else {
        value as u64
    }
}

fn python_tuple_hash_bits<const N: usize>(values: &[i64; N]) -> u64 {
    // CPython tuplehash in 3.8+ (still true for the bundled Python 3.13
    // reference): xxHash primes, unsigned overflow, then signed -1 remap.
    let mut acc = PY_HASH_XXPRIME_5;
    for value in values {
        let lane = python_int_hash_bits(*value);
        acc = acc.wrapping_add(lane.wrapping_mul(PY_HASH_XXPRIME_2));
        acc = acc.rotate_left(31);
        acc = acc.wrapping_mul(PY_HASH_XXPRIME_1);
    }
    acc = acc.wrapping_add((N as u64) ^ (PY_HASH_XXPRIME_5 ^ 3_527_539));
    if acc == u64::MAX {
        1_546_275_796
    } else {
        acc
    }
}

#[derive(Clone, Debug)]
struct PythonIntTupleSet<const N: usize> {
    table: Vec<Option<[i64; N]>>,
    used: usize,
    fill: usize,
}

impl<const N: usize> PythonIntTupleSet<N> {
    fn new() -> Self {
        Self {
            table: vec![None; 8],
            used: 0,
            fill: 0,
        }
    }

    fn items(&self) -> Vec<[i64; N]> {
        self.table.iter().filter_map(|entry| *entry).collect()
    }

    fn insert_clean(&mut self, item: [i64; N]) -> bool {
        let hash = python_tuple_hash_bits(&item);
        let mask = self.table.len() - 1;
        let mut index = (hash as usize) & mask;
        let mut perturb = hash;

        loop {
            let mut probes = if index + PY_SET_LINEAR_PROBES <= mask {
                PY_SET_LINEAR_PROBES
            } else {
                0
            };

            loop {
                match self.table[index] {
                    None => {
                        self.table[index] = Some(item);
                        return false;
                    }
                    Some(existing) if existing == item => return true,
                    Some(_) => {}
                }

                if probes == 0 {
                    break;
                }
                index += 1;
                probes -= 1;
            }

            perturb >>= PY_SET_PERTURB_SHIFT;
            index = index
                .wrapping_mul(5)
                .wrapping_add(1)
                .wrapping_add(perturb as usize)
                & mask;
        }
    }

    fn resize(&mut self, min_used: usize) {
        let old = self.items();
        let mut new_size = 8usize;
        while new_size <= min_used {
            new_size = new_size.saturating_mul(2);
        }
        self.table = vec![None; new_size];
        self.fill = self.used;
        for item in old {
            let _ = self.insert_clean(item);
        }
    }

    fn add_like_set_add(&mut self, item: [i64; N]) {
        if self.insert_clean(item) {
            return;
        }
        self.used += 1;
        self.fill += 1;
        let mask = self.table.len() - 1;
        if self.fill.saturating_mul(5) >= mask.saturating_mul(3) {
            let factor = if self.used > 50_000 { 2 } else { 4 };
            self.resize(self.used.saturating_mul(factor));
        }
    }

    fn update_like_set_merge<I>(&mut self, source_order: I)
    where
        I: IntoIterator<Item = [i64; N]>,
    {
        let source = source_order.into_iter().collect::<Vec<_>>();
        if source.is_empty() {
            return;
        }

        let mask = self.table.len() - 1;
        if self
            .fill
            .saturating_add(source.len())
            .saturating_mul(5)
            >= mask.saturating_mul(3)
        {
            self.resize(self.used.saturating_add(source.len()).saturating_mul(2));
        }

        for item in source {
            if !self.insert_clean(item) {
                self.used += 1;
                self.fill += 1;
            }
        }
    }

    fn from_add_order<I>(source_order: I) -> Self
    where
        I: IntoIterator<Item = [i64; N]>,
    {
        let mut out = Self::new();
        for item in source_order {
            out.add_like_set_add(item);
        }
        out
    }

    fn from_set_copy_order<I>(source_order: I) -> Self
    where
        I: IntoIterator<Item = [i64; N]>,
    {
        let mut out = Self::new();
        out.update_like_set_merge(source_order);
        out
    }
}

fn raw_factor_pairs_like_python(a: i64) -> Vec<[i64; 2]> {
    if a < 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let upper = ((a as f64).sqrt() + 1.0).floor() as i64;
    for b in 2..upper {
        let c = ((a as f64 / b as f64) * 1000.0).round() / 1000.0;
        if c == c.round() {
            out.push([c as i64, b]);
        }
    }
    out
}

fn factor_pairs(a: i64) -> Vec<(i64, i64)> {
    let mut menge = PythonIntTupleSet::<2>::new();
    for pair in raw_factor_pairs_like_python(a) {
        // Python center.multiples uses `menge |= {(int(c), b)}`.  That goes
        // through set-merge, not plain set.add, and it resizes before the
        // singleton is inserted.  The distinction changes visible list order.
        menge.update_like_set_merge(std::iter::once(pair));
    }
    let mut out = menge
        .items()
        .into_iter()
        .map(|[left, right]| (left, right))
        .collect::<Vec<_>>();
    if a >= 0 {
        out.push((a, 1));
    }
    out
}

fn factor_pairs_without_ones(a: i64) -> Vec<(i64, i64)> {
    factor_pairs(a)
        .into_iter()
        .filter(|(x, y)| *x != 1 && *y != 1)
        .collect()
}

fn factor_triples(a: i64) -> Vec<(i64, i64, i64)> {
    let mut m3 = PythonIntTupleSet::<3>::new();
    let mut o3 = PythonIntTupleSet::<3>::new();

    for (m0, m1) in factor_pairs(a) {
        let (o, n) = if m0 > m1 { (m0, m1) } else { (m1, m0) };
        let o2 = PythonIntTupleSet::<2>::from_add_order(
            factor_pairs(o)
                .into_iter()
                .map(|(left, right)| [left, right]),
        );

        for [left, right] in o2.items() {
            let mut triple = [n, left, right];
            triple.sort();
            o3.update_like_set_merge(std::iter::once(triple));
        }

        let o3_copy = PythonIntTupleSet::<3>::from_set_copy_order(o3.items());
        let o6 = o3_copy
            .items()
            .into_iter()
            .filter(|triple| !triple.contains(&1))
            .collect::<Vec<_>>();
        let set_o6 = PythonIntTupleSet::<3>::from_add_order(o6);
        m3.update_like_set_merge(set_o6.items());
    }

    m3.items()
        .into_iter()
        .map(|[a, b, c]| (a, b, c))
        .collect()
}

fn modulo_classification_text(value: i64) -> &'static str {
    match value {
        0 => "ja",
        1 => "Gegenteil",
        2 => "ähnlich",
        3 => "entferntes Gegenteil",
        4 => "entfernt ähnlich",
        _ => "None",
    }
}

fn modulo_remainders_display(n: i64) -> String {
    (2..=25)
        .map(|divisor| {
            let remainder = n.rem_euclid(divisor);
            let complement = divisor - remainder;
            format!(
                "{} % {} = {} {}, {} {}",
                n,
                divisor,
                remainder,
                modulo_classification_text(remainder),
                complement,
                modulo_classification_text(complement)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn abstand_usage_text() -> String {
    "der Befehl 'abstand' verlangt mindestens 2 Zahlenangaben, wie 'abstand 7 17-25'"
        .to_string()
}

fn parse_integer_row_numbers_from_spec(spec: &str) -> Option<Vec<i64>> {
    if spec.contains('/') {
        return None;
    }
    python_row_spec_to_numbers(spec).filter(|numbers| !numbers.is_empty())
}

fn parse_integer_row_set_from_spec(spec: &str) -> Option<std::collections::BTreeSet<i64>> {
    parse_integer_row_numbers_from_spec(spec).map(|numbers| numbers.into_iter().collect())
}

fn format_python_dict_items(entries: Vec<(i64, String)>) -> String {
    entries
        .into_iter()
        .map(|(key, value)| format!("{key}: {value}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_abstand_like_python(
    tokens: &[String],
    render_distance: bool,
    render_prime_distance: bool,
) -> Vec<String> {
    let mut groups: Vec<(String, std::collections::BTreeSet<i64>)> = Vec::new();
    let mut seen = std::collections::BTreeSet::new();

    for token in tokens {
        let Some(group) = parse_integer_row_set_from_spec(token) else {
            continue;
        };
        let key = group.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(",");
        if seen.insert(key) {
            groups.push((token.clone(), group));
        }
    }

    if groups.len() <= 1 {
        return Vec::new();
    }

    let all_are_single_numbers = groups
        .iter()
        .all(|(source, _)| source.chars().all(|ch| ch.is_ascii_digit()));
    let largest_index = groups
        .iter()
        .enumerate()
        .max_by_key(|(_, (_, group))| group.len())
        .map(|(index, _)| index)
        .unwrap_or(0);

    let mut distance_rows: std::collections::BTreeMap<i64, String> =
        std::collections::BTreeMap::new();
    let mut prime_rows: std::collections::BTreeMap<i64, String> =
        std::collections::BTreeMap::new();

    for (_, target_group) in &groups {
        for (source_index, (_, source_group)) in groups.iter().enumerate() {
            if source_index == largest_index || source_group == target_group {
                continue;
            }

            for source_value in source_group {
                if render_distance {
                    let entries = target_group
                        .iter()
                        .map(|target_value| {
                            (
                                *target_value,
                                (*source_value - *target_value).abs().to_string(),
                            )
                        })
                        .collect::<Vec<_>>();
                    if entries.len() > 1 || all_are_single_numbers {
                        distance_rows.insert(*source_value, format_python_dict_items(entries));
                    }
                }

                if render_prime_distance {
                    let entries = target_group
                        .iter()
                        .map(|target_value| {
                            let diff = (*source_value - *target_value).abs();
                            (
                                *target_value,
                                prime_repeat_display(prime_factors(diff, false)),
                            )
                        })
                        .collect::<Vec<_>>();
                    if entries.len() > 1 || all_are_single_numbers {
                        prime_rows.insert(*source_value, format_python_dict_items(entries));
                    }
                }
            }
        }
    }

    let mut lines = Vec::new();
    lines.extend(
        distance_rows
            .into_iter()
            .map(|(key, value)| format!("{key}->: {value}")),
    );
    lines.extend(
        prime_rows
            .into_iter()
            .map(|(key, value)| format!("{key}->: {value}")),
    );
    lines
}

fn compile_direct_number_command(tokens: &[String]) -> Option<PromptOutput> {
    if tokens.iter().any(|t| t == "abc" || t == "abcd") {
        return None;
    }
    let token_set = tokens
        .iter()
        .map(|s| s.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let wants_abstand = token_set.contains("abstand") || token_set.contains("abstandPrim");
    let numbers = parse_row_numbers_from_tokens(tokens).unwrap_or_default();
    if numbers.is_empty() && !wants_abstand {
        return None;
    }
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
        // Python `multis3.mult3()` returns from inside its first numeric loop.
        // retaPrompt therefore prints only the first computed number, even if
        // the row parser produced several numbers.
        if let Some(n) = numbers.first() {
            lines.push(format!("{}: {:?}", n, factor_triples(*n)));
        }
    }
    if token_set.contains("primfaktorenvergleich") && !numbers.is_empty() {
        let comparison_numbers = unique_numbers_preserving_prompt_order(&numbers);
        let render_comparison = comparison_numbers.len() > 1
            || !(token_set.contains("mulpri") || token_set.contains("p"));
        if render_comparison {
            matched = true;
            lines.extend(render_primfaktorenvergleich_like_python(&comparison_numbers));
        }
    }
    if token_set.contains("modulo") {
        matched = true;
        for n in &numbers {
            lines.push(modulo_remainders_display(*n));
        }
    }
    if wants_abstand {
        let rendered = render_abstand_like_python(
            tokens,
            token_set.contains("abstand"),
            token_set.contains("abstandPrim"),
        );
        if rendered.is_empty() {
            if token_set.contains("abstand") {
                matched = true;
                lines.push(abstand_usage_text());
            }
        } else {
            matched = true;
            lines.extend(rendered);
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
        .flat_map(|ch| ch.to_lowercase())
        .map(|ch| ((ch as i64) - 96).to_string())
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
        "Frontend-Profile:",
        "  rp   interaktiv im Vi-Modus, ohne implizites Logging",
        "  rpl  interaktiv im Vi-Modus, mit vollem implizitem Logging",
        "  rpe  interaktiv im Emacs-Modus, mit Emacs-Ausgabeparametern",
        "  rpb  One-Shot-Befehl mit Exact-Modus",
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
    let mut candidates = RETAPROMPT_RETA_MAIN_SWITCHES
        .iter()
        .chain(RETAPROMPT_RETA_SECTION_SWITCHES.iter())
        .copied()
        .collect::<Vec<_>>();
    candidates.sort_unstable();
    candidates.dedup();
    for candidate in candidates {
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
        compile_command, compile_command_with_state, execute_command, factor_pairs_without_ones,
        factor_triples, merge_stored_placeholder, promptSpeicherungB,
        refresh_stored_placeholder_cache, take_auto_prompt_command, PromptCommand,
        PromptModus, PromptScopeRoute, PromptScope_route_for_input, SessionState,
    };

    fn compile_rp(input: &str) -> PromptCommand {
        let state = SessionState::new("rp".to_string(), true, false);
        compile_command_with_state(input, &state).unwrap()
    }

    fn collect_reta_argvs(command: &PromptCommand, out: &mut Vec<Vec<String>>) {
        match command {
            PromptCommand::Reta(argv) => out.push(argv.clone()),
            PromptCommand::RetaBatch(argvs) => out.extend(argvs.iter().cloned()),
            PromptCommand::Sequence(commands) => {
                for command in commands {
                    collect_reta_argvs(command, out);
                }
            }
            _ => {}
        }
    }

    fn reta_argvs(command: &PromptCommand) -> Vec<Vec<String>> {
        let mut out = Vec::new();
        collect_reta_argvs(command, &mut out);
        out
    }

    fn has_immediate_output(command: &PromptCommand) -> bool {
        match command {
            PromptCommand::Immediate(_) => true,
            PromptCommand::Sequence(commands) => commands.iter().any(has_immediate_output),
            _ => false,
        }
    }

    fn has_logging_toggle(command: &PromptCommand, enabled: bool) -> bool {
        match command {
            PromptCommand::ToggleLogging(value) => *value == enabled,
            PromptCommand::Sequence(commands) => {
                commands.iter().any(|command| has_logging_toggle(command, enabled))
            }
            _ => false,
        }
    }

    fn argv_has(argv: &[String], token: &str) -> bool {
        argv.iter().any(|entry| entry == token)
    }

    #[test]
    fn golden_rp_befehl_bare_number_uses_python_default_prompt_pipeline() {
        let command = compile_rp("12");
        let argvs = reta_argvs(&command);
        assert!(!argvs.is_empty(), "expected generated reta calls, got {command:?}");
        assert!(has_immediate_output(&command));
        assert!(argvs
            .iter()
            .any(|argv| argv_has(argv, "--menschliches=motivation")));
        assert!(argvs.iter().any(|argv| argv_has(argv, "--galaxie=thomas")));
        assert!(argvs
            .iter()
            .all(|argv| argv_has(argv, "--vorhervonausschnitt=2,3,4,6,12")));
        assert!(argvs
            .iter()
            .all(|argv| argv_has(argv, "--keineleereninhalte")));
    }

    #[test]
    fn golden_rpb_exact_bare_number_adds_no_headers() {
        let command = compile_rp("12 keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar");
        let argvs = reta_argvs(&command);
        assert!(!argvs.is_empty(), "expected generated reta calls, got {command:?}");
        assert!(argvs
            .iter()
            .all(|argv| argv_has(argv, "--keineueberschriften")));
        assert!(argvs
            .iter()
            .all(|argv| argv_has(argv, "--keineleereninhalte")));
    }

    #[test]
    fn golden_rp_befehl_fraction_defaults_cover_reciprocal_semantics() {
        let command = compile_rp("2/3");
        let argvs = reta_argvs(&command);
        assert!(!argvs.is_empty(), "expected generated reta calls, got {command:?}");
        assert!(argvs
            .iter()
            .any(|argv| argv_has(argv, "--gebrochenuniversum=2")));
        assert!(argvs
            .iter()
            .any(|argv| argv_has(argv, "--gebrochenemotion=2")));
        assert!(argvs
            .iter()
            .any(|argv| argv_has(argv, "--strukturgroesse=strukturgroesse")));
    }

    #[test]
    fn golden_rp_befehl_w_and_v_modifiers_rewrite_rows_python_like() {
        let teiler = compile_rp("12 w");
        let teiler_argvs = reta_argvs(&teiler);
        assert!(teiler_argvs
            .iter()
            .any(|argv| argv_has(argv, "--vorhervonausschnitt=2,3,4,6,12")));

        let vielfache = compile_rp("12 v");
        let vielfache_argvs = reta_argvs(&vielfache);
        assert!(vielfache_argvs.iter().any(|argv| argv
            .iter()
            .any(|token| token == "--vielfachevonzahlen=12")));
        assert!(vielfache_argvs
            .iter()
            .any(|argv| argv_has(argv, "--vorhervonausschnitt=12,v12")));
    }

    #[test]
    fn golden_rp_befehl_repeated_denominator_fraction_uses_reverse_bucket() {
        let command = compile_rp("2/5-3/5");
        let argvs = reta_argvs(&command);
        assert!(!argvs.is_empty(), "expected generated reta calls, got {command:?}");
        assert!(argvs.iter().any(|argv| {
            argv_has(argv, "--gebrochenuniversum=5")
                && argv_has(argv, "--vorhervonausschnitt=2,3")
                && argv_has(argv, "--spaltenreihenfolgeundnurdiese=1")
        }));
    }

    #[test]
    fn golden_rp_befehl_prompt_regex_expands_before_compilation() {
        let command = compile_rp(r#"r"absi" 12"#);
        let argvs = reta_argvs(&command);
        assert!(argvs
            .iter()
            .any(|argv| argv_has(argv, "--menschliches=motivation")));
    }

    #[test]
    fn golden_rp_befehl_literal_reta_command_is_not_rewritten() {
        let command = compile_rp("reta -zeilen --zaehlung=12");
        match command {
            PromptCommand::Reta(argv) => assert_eq!(
                argv,
                vec![
                    "reta".to_string(),
                    "-zeilen".to_string(),
                    "--zaehlung=12".to_string(),
                ]
            ),
            other => panic!("expected literal reta command, got {other:?}"),
        }
    }

    #[test]
    fn prompt_scope_route_keeps_speichern_mode_before_process_commands() {
        let mut state = SessionState::new("rp".to_string(), true, false);
        state.prompt_mode = PromptModus::Speichern;
        let tokens = super::raw_prompt_tokens_like_python("shell echo hi");
        assert_eq!(
            PromptScope_route_for_input("shell echo hi", &tokens, &state),
            PromptScopeRoute::SpeichernInput
        );

        let command = compile_command_with_state("shell echo hi", &state).unwrap();
        match command {
            PromptCommand::StoreCurrentInput(text) => assert_eq!(text, "shell echo hi"),
            other => panic!("expected StoreCurrentInput, got {other:?}"),
        }
    }

    #[test]
    fn prompt_scope_route_prefers_inline_storage_before_execution() {
        let state = SessionState::new("rp".to_string(), true, false);
        let tokens = super::raw_prompt_tokens_like_python("12 s");
        assert_eq!(
            PromptScope_route_for_input("12 s", &tokens, &state),
            PromptScopeRoute::InlineStorageCommand
        );

        let command = compile_command_with_state("12 s", &state).unwrap();
        match command {
            PromptCommand::StoreInline(text) => assert_eq!(text, "12"),
            other => panic!("expected StoreInline, got {other:?}"),
        }
    }

    #[test]
    fn prompt_speicherung_b_composes_placeholder_and_suffix_like_python() {
        let mut state = SessionState::new("rp".to_string(), true, false);
        state.stored_placeholder = "reta -ausgabe --nocolor".to_string();
        refresh_stored_placeholder_cache(&mut state);

        let effective = promptSpeicherungB(&state, Some("12 t".to_string()));
        assert!(effective.starts_with("reta "), "{effective}");
        assert!(effective.contains("--nocolor"), "{effective}");
        assert!(effective.split_whitespace().any(|part| part == "12"), "{effective}");
        assert!(effective.split_whitespace().any(|part| part == "t"), "{effective}");
    }

    #[test]
    fn delete_stored_placeholder_uses_python_numeric_word_collision_rules() {
        let mut state = SessionState::new("rp".to_string(), true, false);
        state.stored_placeholder = "1 2 3".to_string();
        refresh_stored_placeholder_cache(&mut state);
        super::delete_from_stored_placeholder(&mut state, "2");
        assert_eq!(state.stored_placeholder, "1 3");

        state.stored_placeholder = "a b c d".to_string();
        refresh_stored_placeholder_cache(&mut state);
        super::delete_from_stored_placeholder(&mut state, "2");
        assert_eq!(state.stored_placeholder, "a c d");
    }

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
    fn merge_stored_placeholder_keeps_existing_before_incoming_reta_like_python() {
        let merged = merge_stored_placeholder("emotion 12", "reta -spalten --geist");
        assert_eq!(merged, "reta emotion 12 -spalten --geist");
    }

    #[test]
    fn merge_stored_placeholder_strips_reta_marker_on_both_sides_like_python() {
        let merged = merge_stored_placeholder("reta -ausgabe --nocolor", "reta -spalten --geist");
        assert_eq!(merged, "reta -ausgabe --nocolor -spalten --geist");
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
    fn direct_number_commands_use_python_row_generators() {
        let output = compile_command("prim [2,3,4]", PromptModus::Normal)
            .expect("generated row list should compile");
        let PromptCommand::Immediate(output) = output else {
            panic!("expected immediate number output");
        };
        assert!(output.text.contains("2:"));
        assert!(output.text.contains("3:"));
        assert!(output.text.contains("4:"));
    }

    #[test]
    fn multis_pairs_keep_cpython_set_order_from_center_multiples() {
        assert_eq!(
            factor_pairs_without_ones(60),
            vec![(10, 6), (20, 3), (15, 4), (30, 2), (12, 5)]
        );
        assert_eq!(
            factor_pairs_without_ones(360),
            vec![
                (90, 4),
                (72, 5),
                (120, 3),
                (36, 10),
                (30, 12),
                (40, 9),
                (180, 2),
                (60, 6),
                (24, 15),
                (20, 18),
                (45, 8),
            ]
        );
    }

    #[test]
    fn multis3_triples_follow_python_mult3_visible_list_order() {
        assert_eq!(
            factor_triples(60),
            vec![(3, 4, 5), (2, 2, 15), (2, 3, 10), (2, 5, 6)]
        );
        assert_eq!(
            factor_triples(120),
            vec![
                (2, 4, 15),
                (3, 4, 10),
                (2, 6, 10),
                (2, 2, 30),
                (3, 5, 8),
                (4, 5, 6),
                (2, 3, 20),
                (2, 5, 12),
            ]
        );
    }

    #[test]
    fn multis3_prompt_keeps_python_first_number_only_bug() {
        let output = compile_command("multis3 12,18", PromptModus::Normal)
            .expect("multis3 should compile");
        let PromptCommand::Immediate(output) = output else {
            panic!("expected immediate multis3 output");
        };
        assert_eq!(output.text.lines().count(), 1);
    }

    #[test]
    fn abstand_uses_python_row_generators() {
        let output = compile_command("abstand [2,4] [3,5]", PromptModus::Normal)
            .expect("generated row list should compile");
        let PromptCommand::Immediate(output) = output else {
            panic!("expected immediate abstand output");
        };
        assert!(output.text.contains("3->:"));
        assert!(output.text.contains("5->:"));
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

    #[test]
    fn stored_reta_placeholder_prompt_overlay_compiles_to_real_reta_call() {
        let mut state = SessionState::new("rp".to_string(), true, false);
        state.stored_placeholder = "reta -ausgabe --nocolor".to_string();
        refresh_stored_placeholder_cache(&mut state);

        let command = compile_command_with_state("emotion 12", &state).unwrap();
        match command {
            PromptCommand::Reta(argv) => {
                assert!(argv.iter().any(|token| token == "--grundstrukturen=emotion"));
                assert!(argv.iter().any(|token| token == "--nocolor"));
                assert!(argv.iter().any(|token| token == "--vorhervonausschnitt=12"));
            }
            other => panic!("expected PromptCommand::Reta, got {other:?}"),
        }
    }

    #[test]
    fn stored_reta_placeholder_prompt_overlay_keeps_batches() {
        let mut state = SessionState::new("rp".to_string(), true, false);
        state.stored_placeholder = "reta -ausgabe --nocolor".to_string();
        refresh_stored_placeholder_cache(&mut state);

        let command = compile_command_with_state("universum 2/3", &state).unwrap();
        match command {
            PromptCommand::RetaBatch(argvs) => {
                assert_eq!(argvs.len(), 2);
                assert!(argvs
                    .iter()
                    .all(|argv| argv.iter().any(|token| token == "--nocolor")));
            }
            other => panic!("expected PromptCommand::RetaBatch, got {other:?}"),
        }
    }

    #[test]
    fn show_stored_enters_python_output_mode_before_execution() {
        let state = SessionState::new("rp".to_string(), true, false);
        let command = compile_command_with_state("o 12-15 emotion", &state).unwrap();
        assert!(matches!(
            command,
            PromptCommand::EnterStoredOutputMode(Some(ref payload)) if payload == "12-15 emotion"
        ));
    }

    #[test]
    fn stored_output_mode_produces_auto_prompt_command_like_python_loop() {
        let mut state = SessionState::new("rp".to_string(), true, false);
        execute_command(
            PromptCommand::EnterStoredOutputMode(Some("12-15 emotion".to_string())),
            &mut state,
        )
        .unwrap();

        let command = take_auto_prompt_command(&mut state).unwrap();
        assert!(matches!(
            command,
            PromptCommand::ShowStored(Some(ref payload)) if payload == "12-15 emotion"
        ));
        assert_eq!(state.prompt_mode, super::PromptModus::Normal);
        assert_eq!(state.pending_show_stored_suffix, None);
    }

    #[test]
    fn abstand_uses_python_group_distance_output() {
        let state = SessionState::new("rp".to_string(), true, false);
        let command = compile_command_with_state("abstand 7 17-19", &state).unwrap();
        match command {
            PromptCommand::Immediate(output) => {
                assert_eq!(output.text, "7->: 17: 10, 18: 11, 19: 12");
            }
            other => panic!("expected immediate abstand output, got {other:?}"),
        }
    }

    #[test]
    fn abstand_without_enough_ranges_prints_python_usage() {
        let state = SessionState::new("rp".to_string(), true, false);
        let command = compile_command_with_state("abstand", &state).unwrap();
        match command {
            PromptCommand::Immediate(output) => {
                assert!(output.text.contains("verlangt mindestens 2 Zahlenangaben"));
            }
            other => panic!("expected immediate abstand usage output, got {other:?}"),
        }
    }

    #[test]
    fn primfaktorenvergleich_renders_python_gemeinsamkeiten_block() {
        let state = SessionState::new("rp".to_string(), true, false);
        let command = compile_command_with_state("primfaktorenvergleich 12,18", &state).unwrap();
        match command {
            PromptCommand::Immediate(output) => {
                assert_eq!(
                    output.text,
                    "Gemeinsamkeiten: 6 := 2 * 3\n2     := 12    / 6     -> 2\n3     := 18    / 6     -> 3"
                );
            }
            other => panic!("expected immediate primfaktorenvergleich output, got {other:?}"),
        }
    }

    #[test]
    fn mulpri_single_number_suppresses_python_comparison_side_output() {
        let state = SessionState::new("rp".to_string(), true, false);
        let command = compile_command_with_state("mulpri 7", &state).unwrap();
        match command {
            PromptCommand::Immediate(output) => {
                assert!(!output.text.contains("Gemeinsamkeiten:"));
                assert!(output.text.contains("7:"));
            }
            other => panic!("expected immediate mulpri output, got {other:?}"),
        }
    }

    #[test]
    fn modulo_renders_python_moduloa_rows() {
        let state = SessionState::new("rp".to_string(), true, false);
        let command = compile_command_with_state("modulo 7", &state).unwrap();
        match command {
            PromptCommand::Immediate(output) => {
                assert!(output.text.starts_with("7 % 2 = 1 Gegenteil, 1 Gegenteil"));
                assert!(output.text.contains("7 % 5 = 2 ähnlich, 3 entferntes Gegenteil"));
                assert!(output.text.contains("7 % 25 = 7 None, 18 None"));
            }
            other => panic!("expected immediate modulo output, got {other:?}"),
        }
    }

    #[test]
    fn direct_number_side_effects_do_not_suppress_semantic_reta_calls_like_python() {
        let state = SessionState::new("rp".to_string(), false, false);
        let command = compile_command_with_state("mulpri emotion 12", &state).unwrap();
        match command {
            PromptCommand::Sequence(commands) => {
                assert!(commands.iter().any(|command| matches!(command, PromptCommand::Reta(_))));
                assert!(commands.iter().any(|command| matches!(command, PromptCommand::Immediate(_))));
            }
            other => panic!("expected PromptCommand::Sequence, got {other:?}"),
        }
    }

    #[test]
    fn bare_number_compiles_to_default_reta_calls_and_number_output() {
        let state = SessionState::new("rp".to_string(), false, false);
        let command = compile_command_with_state("12", &state).unwrap();
        match command {
            PromptCommand::Sequence(commands) => {
                assert!(commands.iter().any(|command| {
                    matches!(command, PromptCommand::Reta(_) | PromptCommand::RetaBatch(_))
                }));
                assert!(commands.iter().any(|command| matches!(command, PromptCommand::Immediate(_))));
            }
            other => panic!("expected default bare-number sequence, got {other:?}"),
        }
    }

    #[test]
    fn bare_fraction_compiles_to_default_fraction_prompt_batch() {
        let state = SessionState::new("rp".to_string(), false, false);
        let command = compile_command_with_state("2/3", &state).unwrap();
        match command {
            PromptCommand::Sequence(commands) => {
                assert!(commands.iter().any(|command| matches!(command, PromptCommand::RetaBatch(_))));
                assert!(commands.iter().any(|command| matches!(command, PromptCommand::Immediate(_))));
            }
            PromptCommand::RetaBatch(_) => {}
            other => panic!("expected default bare-fraction batch or sequence, got {other:?}"),
        }
    }

    #[test]
    fn direct_number_only_command_still_bypasses_generic_prompt_reta_fallback() {
        let state = SessionState::new("rp".to_string(), false, false);
        let command = compile_command_with_state("modulo 7", &state).unwrap();
        assert!(matches!(command, PromptCommand::Immediate(_)));
    }

    #[test]
    fn one_letter_help_alias_is_control_after_python_alias_expansion() {
        let command = compile_command("h", PromptModus::Normal).unwrap();
        assert!(matches!(command, PromptCommand::PrintHelp));
    }

    #[test]
    fn abc_abcd_uses_python_ord_for_every_character() {
        let command = compile_command("abc a1!", PromptModus::Normal).unwrap();
        match command {
            PromptCommand::Immediate(output) => assert_eq!(output.text, "1 -47 -63"),
            other => panic!("expected immediate abc output, got {other:?}"),
        }
    }


    #[test]
    fn prompt_tokenizer_keeps_quotes_like_python_custom_split() {
        let command = compile_command("abc \"az\"", PromptModus::Normal).unwrap();
        match command {
            PromptCommand::Immediate(output) => assert_eq!(output.text, "-62 1 26 -62"),
            other => panic!("expected immediate abc output, got {other:?}"),
        }
    }

    #[test]
    fn shell_command_keeps_python_argv_instead_of_sh_lc_string() {
        let command = compile_command("shell echo hi", PromptModus::Normal).unwrap();
        match command {
            PromptCommand::Shell(args) => assert_eq!(args, vec!["echo", "hi"]),
            other => panic!("expected shell argv command, got {other:?}"),
        }
    }

    #[test]
    fn python_process_command_uses_prompt_custom_split_joining() {
        let command = compile_command("python print(\"a b\")", PromptModus::Normal).unwrap();
        match command {
            PromptCommand::Python(code) => assert_eq!(code, "print(\"a b\")"),
            other => panic!("expected python command, got {other:?}"),
        }
    }

    #[test]
    fn shell_python_process_commands_respect_python_abc_escape_rule() {
        let command = compile_command("shell abc", PromptModus::Normal).unwrap();
        match command {
            PromptCommand::Immediate(output) => {
                assert_eq!(output.title, "abc");
                assert_eq!(output.text, "19 8 5 12 12");
            }
            other => panic!("expected abc immediate command, got {other:?}"),
        }

        let command = compile_command("python abc", PromptModus::Normal).unwrap();
        match command {
            PromptCommand::Immediate(output) => {
                assert_eq!(output.title, "abc");
                assert_eq!(output.text, "16 25 20 8 15 14");
            }
            other => panic!("expected abc immediate command, got {other:?}"),
        }
    }

    #[test]
    fn prompt_logging_token_appends_toggle_after_python_output_phase() {
        let command = compile_command("12 a loggen", PromptModus::Normal).unwrap();
        assert!(has_logging_toggle(&command, true), "{command:?}");
        assert!(!reta_argvs(&command).is_empty(), "{command:?}");

        let command = compile_command("12 a nichtloggen", PromptModus::Normal).unwrap();
        assert!(has_logging_toggle(&command, false), "{command:?}");
        assert!(!reta_argvs(&command).is_empty(), "{command:?}");
    }

}
