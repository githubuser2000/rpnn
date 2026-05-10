use std::collections::BTreeSet;
use std::sync::{Arc, Mutex, OnceLock};

use reedline::{Completer as ReedlineCompleter, Hinter, History, SearchQuery, Span, Suggestion};

use crate::domain::python_source_of_truth::{
    all_main_alias_groups, parameter_alias_groups_for_main, resolve_parameter_main_alias,
};
use crate::shared_words;

use super::python_like::{
    expand_kurz_kurz_befehl, looks_like_numeric_or_fraction_range, prompt_words,
    regex_like_search as python_regex_like_search, PromptModus,
};
use super::semantic_choices::{
    semantic_wahl15_value, semantic_wahl16_value, RETAPROMPT_AUSGABE_ART_PARAMETER,
    RETAPROMPT_AUSGABE_ART_VALUES, RETAPROMPT_AUSGABE_BREITE_PARAMETER,
    RETAPROMPT_AUSGABE_BREITEN_PARAMETER, RETAPROMPT_AUSGABE_PARAMETER_TOKENS,
    RETAPROMPT_KOMBINATION_GALAXIE_PARAMETER, RETAPROMPT_KOMBINATION_PARAMETER_TOKENS,
    RETAPROMPT_KOMBINATION_UNIVERSUM_PARAMETER, RETAPROMPT_RETA_MAIN_SWITCHES,
    RETAPROMPT_ZEILEN_PARAMETER_TOKENS, RETAPROMPT_ZEILEN_PRIMZAHLEN_PARAMETER,
    RETAPROMPT_ZEILEN_PRIMZAHLEN_VALUES, RETAPROMPT_ZEILEN_TYP_PARAMETER,
    RETAPROMPT_ZEILEN_TYP_VALUES, RETAPROMPT_ZEILEN_ZEIT_PARAMETER,
    RETAPROMPT_ZEILEN_ZEIT_VALUES,
};

pub const RP_META_COMMANDS: &[&str] = &[
    "HELP",
    "help",
    "hilfe",
    "befehle",
    "kurzbefehle",
    "q",
    ":q",
    "exit",
    "quit",
    "ende",
    "leeren",
    "clear",
    ":ui",
    ":preview",
    ":history",
    ":mode",
    "loggen",
    "nichtloggen",
    "shell",
    "python",
    "math",
    "reta",
];

#[derive(Clone, Debug, Default)]
pub struct PromptMetadata {
    pub vocabulary: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RetaMainSection {
    Zeilen,
    Spalten,
    Kombination,
    Ausgabe,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ComplSitua {
    HauptPara,
    ZeilenPara,
    RetaAnfang,
    SpaltenPara,
    KomiPara,
    AusgabePara,
    BefehleNichtReta,
}

#[derive(Clone, Debug)]
pub struct CompletionRuntimeState {
    pub prompt_mode: PromptModus,
    pub stored_prefix_tokens: Vec<String>,
    pub stored_commands: Vec<String>,
}

impl Default for CompletionRuntimeState {
    fn default() -> Self {
        Self {
            prompt_mode: PromptModus::Normal,
            stored_prefix_tokens: Vec::new(),
            stored_commands: Vec::new(),
        }
    }
}

pub type CompletionRuntimeHandle = Arc<Mutex<CompletionRuntimeState>>;

#[derive(Clone, Debug)]
struct PromptContextCompleter {
    runtime: CompletionRuntimeHandle,
}

#[derive(Clone, Debug)]
struct PromptContextHinter {
    runtime: CompletionRuntimeHandle,
    current_hint: AutosuggestionHint,
}

#[derive(Clone, Debug, Default)]
struct AutosuggestionHint {
    display: String,
    insert: String,
}

#[derive(Clone, Debug)]
struct CompletionCandidate {
    value: String,
    description: Option<String>,
    replace_start: usize,
    append_whitespace: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CandidateMatchMode {
    Normal,
    DashInsensitiveStructural,
}

#[derive(Clone, Debug)]
struct TokenSegment {
    text: String,
    start: usize,
}

#[derive(Clone, Debug)]
struct ValueCompletionContext {
    parameter_token: String,
    value_fragment: String,
    value_replace_start: usize,
    token_start: usize,
    value_prefix: String,
}

#[derive(Clone, Debug)]
struct PythonCompletionState {
    options: Vec<String>,
    if_reta_anfang: bool,
    situation: ComplSitua,
    neben_para_wort: Option<String>,
    last_commands: Vec<String>,
}

impl PythonCompletionState {
    fn new() -> Self {
        Self {
            options: ordered_prompt_commands(),
            if_reta_anfang: false,
            situation: ComplSitua::RetaAnfang,
            neben_para_wort: None,
            last_commands: Vec::new(),
        }
    }

    fn push_last_command(&mut self, token: &str) {
        if !self.last_commands.iter().any(|entry| entry == token) {
            self.last_commands.push(token.to_string());
        }
    }

    fn current_section(&self) -> Option<RetaMainSection> {
        self.neben_para_wort
            .as_deref()
            .and_then(section_from_main_token)
    }
}

static PROMPT_METADATA: OnceLock<PromptMetadata> = OnceLock::new();

pub fn prompt_metadata() -> &'static PromptMetadata {
    PROMPT_METADATA.get_or_init(build_prompt_metadata)
}

fn build_prompt_metadata() -> PromptMetadata {
    let words = shared_words();
    let mut items = Vec::new();
    let mut seen = BTreeSet::new();

    for item in ordered_prompt_commands() {
        push_unique_ordered(&mut items, &mut seen, item);
    }
    for item in reta_main_switches() {
        push_unique_ordered(&mut items, &mut seen, item);
    }
    for item in zeilen_parameter_tokens() {
        push_unique_ordered(&mut items, &mut seen, item);
    }
    for item in ausgabe_parameter_tokens() {
        push_unique_ordered(&mut items, &mut seen, item);
    }
    for item in kombi_parameter_tokens() {
        push_unique_ordered(&mut items, &mut seen, item);
    }
    for item in spalten_parameter_tokens() {
        push_unique_ordered(&mut items, &mut seen, item);
    }

    for main_group in all_main_alias_groups(words) {
        for alias in &main_group.aliases {
            push_unique_ordered(&mut items, &mut seen, alias.clone());
        }
        for parameter_group in parameter_alias_groups_for_main(words, &main_group.canonical) {
            for alias in &parameter_group.aliases {
                push_unique_ordered(&mut items, &mut seen, alias.clone());
            }
        }
    }

    for value in zeilen_value_candidates("typ") {
        push_unique_ordered(&mut items, &mut seen, value);
    }
    for value in zeilen_value_candidates("primzahlen") {
        push_unique_ordered(&mut items, &mut seen, value);
    }
    for value in zeilen_value_candidates("zeit") {
        push_unique_ordered(&mut items, &mut seen, value);
    }
    for value in ausgabe_value_candidates("art") {
        push_unique_ordered(&mut items, &mut seen, value);
    }
    for value in kombi_value_candidates("galaxie") {
        push_unique_ordered(&mut items, &mut seen, value);
    }
    for value in kombi_value_candidates("universum") {
        push_unique_ordered(&mut items, &mut seen, value);
    }

    PromptMetadata { vocabulary: items }
}

pub fn completion_vocabulary() -> Vec<String> {
    prompt_metadata().vocabulary.clone()
}

pub fn new_completion_runtime_handle() -> CompletionRuntimeHandle {
    Arc::new(Mutex::new(CompletionRuntimeState::default()))
}

pub fn set_completion_prompt_mode(runtime: &CompletionRuntimeHandle, prompt_mode: PromptModus) {
    if let Ok(mut state) = runtime.lock() {
        state.prompt_mode = prompt_mode;
    }
}

pub fn set_completion_runtime_context(
    runtime: &CompletionRuntimeHandle,
    prompt_mode: PromptModus,
    stored_prefix_tokens: &[String],
    stored_commands: &[String],
) {
    if let Ok(mut state) = runtime.lock() {
        state.prompt_mode = prompt_mode;
        state.stored_prefix_tokens = stored_prefix_tokens.to_vec();
        state.stored_commands = stored_commands.to_vec();
    }
}

pub fn build_default_completer() -> Box<dyn ReedlineCompleter> {
    build_default_completer_with_runtime(new_completion_runtime_handle())
}

pub fn build_default_completer_with_runtime(
    runtime: CompletionRuntimeHandle,
) -> Box<dyn ReedlineCompleter> {
    Box::new(PromptContextCompleter { runtime })
}

pub fn build_default_hinter_with_runtime(runtime: CompletionRuntimeHandle) -> Box<dyn Hinter> {
    Box::new(PromptContextHinter {
        runtime,
        current_hint: AutosuggestionHint::default(),
    })
}

pub fn autosuggestion_for_input(input: &str) -> Option<String> {
    autosuggestion_for_input_in_mode_with_context(input, PromptModus::Normal, &[], &[])
}

pub fn autosuggestion_for_input_in_mode_with_context(
    input: &str,
    prompt_mode: PromptModus,
    stored_prefix_tokens: &[String],
    stored_commands: &[String],
) -> Option<String> {
    autosuggestion_from_context_candidates(
        input,
        prompt_mode,
        stored_prefix_tokens,
        stored_commands,
    )
    .map(|hint| hint.display)
}

pub fn candidates_for_prefix(prefix: &str) -> Vec<String> {
    filter_candidate_values(&prompt_metadata().vocabulary, prefix, false)
}

pub fn candidates_for_input(input: &str) -> Vec<String> {
    completion_candidates_for_line(input)
        .into_iter()
        .map(|candidate| candidate.value)
        .collect()
}

pub fn candidates_for_input_in_mode(input: &str, prompt_mode: PromptModus) -> Vec<String> {
    completion_candidates_for_line_in_mode(input, prompt_mode)
        .into_iter()
        .map(|candidate| candidate.value)
        .collect()
}

pub fn candidates_for_input_in_mode_with_context(
    input: &str,
    prompt_mode: PromptModus,
    stored_prefix_tokens: &[String],
    stored_commands: &[String],
) -> Vec<String> {
    completion_candidates_for_line_in_mode_with_context(
        input,
        prompt_mode,
        stored_prefix_tokens,
        stored_commands,
    )
    .into_iter()
    .map(|candidate| candidate.value)
    .collect()
}

impl ReedlineCompleter for PromptContextCompleter {
    fn complete(&mut self, line: &str, pos: usize) -> Vec<Suggestion> {
        let before_cursor = safe_prefix(line, pos);
        let runtime_state = self
            .runtime
            .lock()
            .map(|state| state.clone())
            .unwrap_or_default();

        completion_candidates_for_line_in_mode_with_context(
            before_cursor,
            runtime_state.prompt_mode,
            &runtime_state.stored_prefix_tokens,
            &runtime_state.stored_commands,
        )
        .into_iter()
        .map(|candidate| Suggestion {
            value: candidate.value,
            display_override: None,
            description: candidate.description,
            style: None,
            extra: None,
            span: Span::new(candidate.replace_start, before_cursor.len()),
            append_whitespace: candidate.append_whitespace,
            match_indices: None,
        })
        .collect()
    }
}

impl Hinter for PromptContextHinter {
    fn handle(
        &mut self,
        line: &str,
        pos: usize,
        history: &dyn History,
        use_ansi_coloring: bool,
        _cwd: &str,
    ) -> String {
        self.current_hint = AutosuggestionHint::default();

        let Some(before_cursor) = safe_line_end_prefix(line, pos) else {
            return String::new();
        };

        let runtime_state = self
            .runtime
            .lock()
            .map(|state| state.clone())
            .unwrap_or_default();

        if let Some(hint) = autosuggestion_from_context_candidates(
            before_cursor,
            runtime_state.prompt_mode,
            &runtime_state.stored_prefix_tokens,
            &runtime_state.stored_commands,
        ) {
            let display = hint.display.clone();
            self.current_hint = hint;
            return render_autosuggestion_hint(&display, use_ansi_coloring);
        }

        if let Some(hint) = autosuggestion_from_history(before_cursor, history) {
            self.current_hint = AutosuggestionHint::insertable(hint);
            return render_autosuggestion_hint(&self.current_hint.display, use_ansi_coloring);
        }

        String::new()
    }

    fn complete_hint(&self) -> String {
        self.current_hint.insert.clone()
    }

    fn next_hint_token(&self) -> String {
        first_autosuggestion_token(&self.current_hint.insert)
    }
}

impl AutosuggestionHint {
    fn insertable(text: impl Into<String>) -> Self {
        let text = text.into();
        Self {
            display: text.clone(),
            insert: text,
        }
    }

    fn display_only(text: impl Into<String>) -> Self {
        Self {
            display: text.into(),
            insert: String::new(),
        }
    }
}

fn autosuggestion_from_context_candidates(
    before_cursor: &str,
    prompt_mode: PromptModus,
    stored_prefix_tokens: &[String],
    stored_commands: &[String],
) -> Option<AutosuggestionHint> {
    if before_cursor.trim().is_empty() {
        return None;
    }

    completion_candidates_for_line_in_mode_with_context(
        before_cursor,
        prompt_mode,
        stored_prefix_tokens,
        stored_commands,
    )
    .into_iter()
    .filter_map(|candidate| autosuggestion_hint_for_candidate(before_cursor, &candidate))
    .find(|hint| !hint.display.trim().is_empty())
}

fn autosuggestion_hint_for_candidate(
    before_cursor: &str,
    candidate: &CompletionCandidate,
) -> Option<AutosuggestionHint> {
    if let Some(hint) = suffix_hint_for_candidate(before_cursor, candidate) {
        return Some(AutosuggestionHint::insertable(hint));
    }

    structural_dash_replacement_hint_for_candidate(before_cursor, candidate)
}

fn suffix_hint_for_candidate(
    before_cursor: &str,
    candidate: &CompletionCandidate,
) -> Option<String> {
    if candidate.replace_start > before_cursor.len()
        || !before_cursor.is_char_boundary(candidate.replace_start)
    {
        return None;
    }

    let mut completed = String::with_capacity(
        candidate.replace_start + candidate.value.len() + usize::from(candidate.append_whitespace),
    );
    completed.push_str(&before_cursor[..candidate.replace_start]);
    completed.push_str(&candidate.value);
    if candidate.append_whitespace {
        completed.push(' ');
    }

    if completed.len() <= before_cursor.len() || !completed.starts_with(before_cursor) {
        return None;
    }

    Some(completed[before_cursor.len()..].to_string())
}

fn structural_dash_replacement_hint_for_candidate(
    before_cursor: &str,
    candidate: &CompletionCandidate,
) -> Option<AutosuggestionHint> {
    if candidate.replace_start > before_cursor.len()
        || !before_cursor.is_char_boundary(candidate.replace_start)
    {
        return None;
    }

    let typed_fragment = &before_cursor[candidate.replace_start..];
    if typed_fragment.trim().is_empty() || typed_fragment.chars().any(char::is_whitespace) {
        return None;
    }
    if !is_structural_dash_candidate(&candidate.value) {
        return None;
    }

    let normalized_typed = normalize_dash_insensitive_fragment(typed_fragment);
    if normalized_typed.is_empty() {
        return None;
    }

    let normalized_candidate = normalize_dash_insensitive_fragment(&candidate.value);
    if !normalized_candidate.starts_with(&normalized_typed) {
        return None;
    }

    let mut canonical = candidate.value.clone();
    if candidate.append_whitespace {
        canonical.push(' ');
    }

    if normalize_completion_text(typed_fragment) == normalize_completion_text(&canonical) {
        return None;
    }

    Some(AutosuggestionHint::display_only(format!(" → {canonical}")))
}

fn autosuggestion_from_history(before_cursor: &str, history: &dyn History) -> Option<String> {
    if before_cursor.trim().is_empty() {
        return None;
    }

    let entries = history
        .search(SearchQuery::last_with_prefix(
            before_cursor.to_string(),
            history.session(),
        ))
        .ok()?;

    entries.into_iter().find_map(|entry| {
        let command_line = entry.command_line;
        if command_line.len() <= before_cursor.len() || !command_line.starts_with(before_cursor) {
            return None;
        }

        let suffix = command_line[before_cursor.len()..].to_string();
        (!suffix.trim().is_empty()).then_some(suffix)
    })
}

fn render_autosuggestion_hint(hint: &str, use_ansi_coloring: bool) -> String {
    if use_ansi_coloring && !hint.is_empty() {
        format!("\x1b[90m{hint}\x1b[0m")
    } else {
        hint.to_string()
    }
}

fn first_autosuggestion_token(hint: &str) -> String {
    let mut out = String::new();
    let mut saw_non_whitespace = false;

    for ch in hint.chars() {
        if ch.is_whitespace() {
            if saw_non_whitespace {
                break;
            }
            out.push(ch);
        } else {
            saw_non_whitespace = true;
            out.push(ch);
        }
    }

    out
}

fn safe_line_end_prefix(line: &str, pos: usize) -> Option<&str> {
    if pos != line.len() {
        return None;
    }
    Some(safe_prefix(line, pos))
}

fn completion_candidates_for_line(before_cursor: &str) -> Vec<CompletionCandidate> {
    completion_candidates_for_line_in_mode_with_context(
        before_cursor,
        PromptModus::Normal,
        &[],
        &[],
    )
}

fn completion_candidates_for_line_in_mode(
    before_cursor: &str,
    prompt_mode: PromptModus,
) -> Vec<CompletionCandidate> {
    completion_candidates_for_line_in_mode_with_context(before_cursor, prompt_mode, &[], &[])
}

fn completion_candidates_for_line_in_mode_with_context(
    before_cursor: &str,
    prompt_mode: PromptModus,
    stored_prefix_tokens: &[String],
    _stored_commands: &[String],
) -> Vec<CompletionCandidate> {
    let tokens = split_tokens_with_positions(before_cursor);
    let all_text_tokens = tokens
        .iter()
        .map(|segment| segment.text.clone())
        .collect::<Vec<_>>();
    let ends_with_whitespace = before_cursor
        .chars()
        .last()
        .map(|ch| ch.is_whitespace())
        .unwrap_or(false);

    let (current_token, current_start, previous_tokens) = if ends_with_whitespace {
        (String::new(), before_cursor.len(), tokens)
    } else if let Some(last) = tokens.last() {
        (
            last.text.clone(),
            last.start,
            tokens[..tokens.len().saturating_sub(1)].to_vec(),
        )
    } else {
        (String::new(), 0usize, Vec::new())
    };

    let previous_text_tokens = previous_tokens
        .iter()
        .map(|segment| segment.text.clone())
        .collect::<Vec<_>>();

    if matches!(prompt_mode, PromptModus::LoeschenSelect) {
        // Python `promptInput()` passes `completer=None` in delete-selection
        // mode.  Keep the pure candidate API empty too, so tests and reedline
        // agree with the Python UI contract.
        return Vec::new();
    }

    let contextual_previous_tokens = build_contextual_previous_tokens(
        before_cursor,
        &all_text_tokens,
        &previous_text_tokens,
        stored_prefix_tokens,
    );

    if mode_like_prompt_command(&contextual_previous_tokens) {
        return build_completion_candidates(
            vec!["vi".to_string(), "emacs".to_string()],
            &current_token,
            current_start,
            None,
            true,
        );
    }

    if shell_like_prompt_command(&contextual_previous_tokens, &current_token) {
        return Vec::new();
    }

    let mut state = PythonCompletionState::new();
    for token in &contextual_previous_tokens {
        consume_space_token(&mut state, token);
    }

    if let Some(value_context) = parse_value_context(&current_token, current_start) {
        return build_value_candidates_from_state(&state, &value_context);
    }

    let mut candidates = state.options.clone();
    if contextual_previous_tokens.is_empty()
        && should_offer_main_switches_for_fragment(&current_token)
    {
        candidates = merge_unique(candidates, main_switches_vec());
    }

    build_structural_completion_candidates(candidates, &current_token, current_start, None, true)
}

fn build_contextual_previous_tokens(
    before_cursor: &str,
    raw_text_tokens: &[String],
    previous_text_tokens: &[String],
    stored_prefix_tokens: &[String],
) -> Vec<String> {
    if !should_apply_stored_context(before_cursor, raw_text_tokens, stored_prefix_tokens) {
        return previous_text_tokens.to_vec();
    }

    if !stored_context_precedes_current_input(raw_text_tokens, stored_prefix_tokens) {
        return previous_text_tokens.to_vec();
    }

    let mut contextual = stored_prefix_tokens.to_vec();
    contextual.extend(previous_text_tokens.iter().cloned());
    contextual
}

fn should_apply_stored_context(
    before_cursor: &str,
    raw_text_tokens: &[String],
    stored_prefix_tokens: &[String],
) -> bool {
    !stored_prefix_tokens.is_empty()
        && (before_cursor.trim().is_empty()
            || !completion_bypasses_stored_context(before_cursor.trim(), raw_text_tokens))
}

fn stored_context_precedes_current_input(
    raw_text_tokens: &[String],
    stored_prefix_tokens: &[String],
) -> bool {
    !(matches!(raw_text_tokens.first().map(String::as_str), Some("reta"))
        && !matches!(
            stored_prefix_tokens.first().map(String::as_str),
            Some("reta")
        ))
}

fn completion_bypasses_stored_context(trimmed: &str, tokens: &[String]) -> bool {
    if matches!(
        trimmed,
        "q" | ":q"
            | "exit"
            | "quit"
            | "ende"
            | "HELP"
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

    if matches!(
        tokens.first().map(String::as_str),
        Some("shell" | "python" | "math" | ":mode")
    ) {
        return true;
    }

    prompt_command_prefix_like_python_bypasses_context(tokens)
}

fn prompt_command_prefix_like_python_bypasses_context(tokens: &[String]) -> bool {
    let Some(first_token) = tokens.first().map(String::as_str) else {
        return false;
    };

    if first_token.is_empty()
        || first_token.starts_with('-')
        || looks_like_numeric_or_fraction_range(first_token)
    {
        return false;
    }

    let fragment = normalize_completion_text(first_token);
    if fragment.is_empty() {
        return false;
    }

    ordered_prompt_commands().into_iter().any(|candidate| {
        let candidate = normalize_completion_text(&candidate);
        candidate.starts_with(&fragment)
            || fuzzy_completion_score(&candidate, &fragment)
                .map(|score| score.start == 0 || fragment.chars().count() > 1)
                .unwrap_or(false)
    })
}
fn consume_space_token(state: &mut PythonCompletionState, first_term: &str) {
    state.push_last_command(first_term);

    if state.situation == ComplSitua::RetaAnfang && first_term == "reta" {
        state.if_reta_anfang = true;
        state.options = main_switches_vec();
        state.situation = ComplSitua::HauptPara;
        return;
    }

    if matches!(
        state.situation,
        ComplSitua::RetaAnfang | ComplSitua::BefehleNichtReta
    ) && !is_main_switch(first_term)
    {
        let has_prompt_command = state
            .last_commands
            .iter()
            .any(|token| token != "reta" && is_prompt_non_reta_command(token));
        let has_row_spec = state
            .last_commands
            .iter()
            .any(|token| looks_like_numeric_or_fraction_range(token));
        let expanded_like_python =
            expand_kurz_kurz_befehl(PromptModus::Normal, &state.last_commands).0;

        let mut options = prompt_non_reta_commands();
        if (has_prompt_command && has_row_spec) || expanded_like_python || !state.if_reta_anfang {
            options = merge_unique(options, main_switches_vec());
        } else {
            state.if_reta_anfang = false;
        }
        state.options = options;
        state.situation = ComplSitua::BefehleNichtReta;
        return;
    }

    if is_main_switch(first_term)
        || state
            .neben_para_wort
            .as_deref()
            .map(is_main_switch)
            .unwrap_or(false)
    {
        let active_main = if is_main_switch(first_term) {
            first_term
        } else {
            state.neben_para_wort.as_deref().unwrap_or("")
        };

        let (mut options, next_situation) = parameter_options_for_main(active_main);
        if !state.if_reta_anfang {
            options = merge_unique(options, prompt_non_reta_commands());
        }
        if !is_main_switch(first_term) {
            options = merge_unique(options, main_switches_vec());
        }
        state.options = options;
        state.situation = next_situation;
        if is_main_switch(first_term) {
            state.neben_para_wort = Some(first_term.to_string());
        }
    }
}

fn is_prompt_non_reta_command(token: &str) -> bool {
    token != "reta"
        && ordered_prompt_commands().into_iter().any(|candidate| {
            normalize_completion_text(&candidate) == normalize_completion_text(token)
        })
}

fn parameter_options_for_main(main_switch: &str) -> (Vec<String>, ComplSitua) {
    match main_switch {
        "-zeilen" => (
            zeilen_parameter_tokens()
                .into_iter()
                .map(str::to_string)
                .collect(),
            ComplSitua::ZeilenPara,
        ),
        "-spalten" => (spalten_parameter_tokens(), ComplSitua::SpaltenPara),
        "-kombination" => (
            kombi_parameter_tokens()
                .into_iter()
                .map(str::to_string)
                .collect(),
            ComplSitua::KomiPara,
        ),
        "-ausgabe" => (
            ausgabe_parameter_tokens()
                .into_iter()
                .map(str::to_string)
                .collect(),
            ComplSitua::AusgabePara,
        ),
        _ => (Vec::new(), ComplSitua::HauptPara),
    }
}

fn build_value_candidates_from_state(
    state: &PythonCompletionState,
    value_context: &ValueCompletionContext,
) -> Vec<CompletionCandidate> {
    let stripped = value_context.parameter_token.trim_start_matches('-');
    let raw_key = stripped.trim_end_matches('=');
    let key = normalize_completion_text(raw_key);
    let section = state.current_section();
    let mut candidates = match section {
        Some(RetaMainSection::Zeilen) => zeilen_value_candidates(&key),
        Some(RetaMainSection::Spalten) => spalten_value_candidates(&key),
        Some(RetaMainSection::Kombination) => kombi_value_candidates(&key),
        Some(RetaMainSection::Ausgabe) => ausgabe_value_candidates(&key),
        None => Vec::new(),
    };

    if candidates.is_empty() && !key.is_empty() {
        candidates = close_parameter_key_candidates(section, &key);
    }

    if let Some(canonical_parameter_token) = canonical_value_parameter_token(section, raw_key) {
        let typed_parameter_token = parameter_token_with_equals(&value_context.parameter_token);
        if normalize_completion_text(&typed_parameter_token)
            != normalize_completion_text(&canonical_parameter_token)
        {
            return build_canonicalized_value_completion_candidates(
                candidates,
                &value_context.value_fragment,
                value_context.token_start,
                &canonical_parameter_token,
                &value_context.value_prefix,
            );
        }
    }

    build_completion_candidates(
        candidates,
        &value_context.value_fragment,
        value_context.value_replace_start,
        None,
        true,
    )
}

fn build_canonicalized_value_completion_candidates(
    candidates: Vec<String>,
    fragment: &str,
    replace_start: usize,
    canonical_parameter_token: &str,
    value_prefix: &str,
) -> Vec<CompletionCandidate> {
    filter_candidate_values(&candidates, fragment, true)
        .into_iter()
        .map(|value| {
            let description = semantic_choice_completion_description(&value);
            CompletionCandidate {
                append_whitespace: !value.ends_with('='),
                description,
                replace_start,
                value: format!("{canonical_parameter_token}{value_prefix}{value}"),
            }
        })
        .collect()
}

fn parameter_token_with_equals(parameter_token: &str) -> String {
    if parameter_token.ends_with('=') {
        parameter_token.to_string()
    } else {
        format!("{parameter_token}=")
    }
}

fn canonical_value_parameter_token(
    section: Option<RetaMainSection>,
    raw_key: &str,
) -> Option<String> {
    let normalized_key = normalize_completion_text(raw_key);
    value_parameter_tokens_for_section(section)
        .into_iter()
        .find(|candidate| {
            candidate.ends_with('=')
                && normalize_completion_text(
                    candidate.trim_start_matches('-').trim_end_matches('='),
                ) == normalized_key
        })
}

fn value_parameter_tokens_for_section(section: Option<RetaMainSection>) -> Vec<String> {
    match section {
        Some(RetaMainSection::Zeilen) => zeilen_parameter_tokens()
            .into_iter()
            .map(str::to_string)
            .collect(),
        Some(RetaMainSection::Spalten) => spalten_parameter_tokens(),
        Some(RetaMainSection::Kombination) => kombi_parameter_tokens()
            .into_iter()
            .map(str::to_string)
            .collect(),
        Some(RetaMainSection::Ausgabe) => ausgabe_parameter_tokens()
            .into_iter()
            .map(str::to_string)
            .collect(),
        None => Vec::new(),
    }
}

fn close_parameter_key_candidates(section: Option<RetaMainSection>, key: &str) -> Vec<String> {
    let normalized_key = normalize_completion_text(key);
    if normalized_key.is_empty() {
        return Vec::new();
    }

    let mut scored = Vec::new();
    let mut seen = BTreeSet::new();
    for (index, candidate) in value_parameter_keys_for_section(section)
        .into_iter()
        .enumerate()
    {
        let normalized_candidate = normalize_completion_text(&candidate);
        if !seen.insert(normalized_candidate.clone()) {
            continue;
        }
        if let Some(score) = close_parameter_key_score(&normalized_candidate, &normalized_key) {
            scored.push((score, index, normalized_candidate, candidate));
        }
    }

    scored.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| left.1.cmp(&right.1))
            .then_with(|| left.2.cmp(&right.2))
    });

    scored
        .into_iter()
        .map(|(_, _, _, candidate)| candidate)
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct CloseParameterKeyScore {
    tier: usize,
    distance: usize,
    start: usize,
    span: usize,
    candidate_len: usize,
}

fn close_parameter_key_score(candidate: &str, key: &str) -> Option<CloseParameterKeyScore> {
    let candidate_len = candidate.chars().count();
    if candidate.starts_with(key) {
        return Some(CloseParameterKeyScore {
            tier: 0,
            distance: 0,
            start: 0,
            span: key.chars().count(),
            candidate_len,
        });
    }

    if let Some(start_byte) = candidate.find(key) {
        let start = candidate[..start_byte].chars().count();
        return Some(CloseParameterKeyScore {
            tier: 1,
            distance: 0,
            start,
            span: key.chars().count(),
            candidate_len,
        });
    }

    if let Some(score) = fuzzy_completion_score(candidate, key) {
        return Some(CloseParameterKeyScore {
            tier: 2,
            distance: score.gaps,
            start: score.start,
            span: score.span,
            candidate_len,
        });
    }

    let distance = levenshtein_distance(candidate, key);
    let key_len = key.chars().count();
    let threshold = 2usize.max(key_len / 3);
    (distance <= threshold).then_some(CloseParameterKeyScore {
        tier: 3,
        distance,
        start: 0,
        span: candidate_len,
        candidate_len,
    })
}

fn levenshtein_distance(left: &str, right: &str) -> usize {
    let right_chars = right.chars().collect::<Vec<_>>();
    let mut previous = (0..=right_chars.len()).collect::<Vec<_>>();
    let mut current = vec![0usize; right_chars.len() + 1];

    for (left_index, left_char) in left.chars().enumerate() {
        current[0] = left_index + 1;
        for (right_index, right_char) in right_chars.iter().enumerate() {
            let substitution_cost = if left_char == *right_char { 0 } else { 1 };
            current[right_index + 1] = (previous[right_index + 1] + 1)
                .min(current[right_index] + 1)
                .min(previous[right_index] + substitution_cost);
        }
        std::mem::swap(&mut previous, &mut current);
    }

    previous[right_chars.len()]
}

fn value_parameter_keys_for_section(section: Option<RetaMainSection>) -> Vec<String> {
    match section {
        Some(RetaMainSection::Zeilen) => zeilen_value_parameter_keys()
            .into_iter()
            .map(str::to_string)
            .collect(),
        Some(RetaMainSection::Spalten) => spalten_value_parameter_keys(),
        Some(RetaMainSection::Kombination) => kombi_value_parameter_keys()
            .into_iter()
            .map(str::to_string)
            .collect(),
        Some(RetaMainSection::Ausgabe) => ausgabe_value_parameter_keys()
            .into_iter()
            .map(str::to_string)
            .collect(),
        None => Vec::new(),
    }
}

fn zeilen_value_parameter_keys() -> [&'static str; 25] {
    [
        "alles",
        "gestern",
        "heute",
        "hoehemaximal",
        "mond",
        "morgen",
        "nachtraeglichneuabzaehlung",
        "nachtraeglichneuabzaehlungvielfache",
        "oberesmaximum",
        "planet",
        "potenzenvonzahlen",
        "primzahlvielfache",
        "schwarzesonne",
        "sonne",
        "typ",
        "vielfachevonzahlen",
        "vorhervonausschnitt",
        "vorhervonausschnittteiler",
        "zaehlung",
        "zeit",
        "primzahlen",
        "aussenerste",
        "innenerste",
        "aussenalle",
        "innenalle",
    ]
}

fn ausgabe_value_parameter_keys() -> [&'static str; 14] {
    [
        "nocolor",
        "justtext",
        "art",
        "onetable",
        "spaltenreihenfolgeundnurdiese",
        "endlessscreen",
        "endless",
        "dontwrap",
        "breite",
        "breiten",
        "keineleereninhalte",
        "keinenummerierung",
        "keineueberschriften",
        "*",
    ]
}

fn kombi_value_parameter_keys() -> [&'static str; 3] {
    ["galaxie", "universum", "*"]
}

fn spalten_value_parameter_keys() -> Vec<String> {
    let words = shared_words();
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();

    for entry in &words.paraNdataMatrix {
        for alias in &entry.parameterMainNames {
            push_unique_ordered(&mut out, &mut seen, alias.clone());
        }
    }

    for extra in ["breite", "breiten", "*"] {
        push_unique_ordered(&mut out, &mut seen, extra);
    }

    out
}

fn safe_prefix(line: &str, pos: usize) -> &str {
    let mut end = pos.min(line.len());
    while end > 0 && !line.is_char_boundary(end) {
        end -= 1;
    }
    &line[..end]
}

fn split_tokens_with_positions(input: &str) -> Vec<TokenSegment> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut start: Option<usize> = None;
    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;

    for (idx, ch) in input.char_indices() {
        let is_top_level_whitespace = ch.is_whitespace() && round == 0 && square == 0 && curly == 0;
        if is_top_level_whitespace {
            if let Some(token_start) = start.take() {
                if !current.trim().is_empty() {
                    out.push(TokenSegment {
                        text: std::mem::take(&mut current),
                        start: token_start,
                    });
                } else {
                    current.clear();
                }
            }
            continue;
        }

        if start.is_none() {
            start = Some(idx);
        }

        match ch {
            '(' => round += 1,
            ')' => round -= 1,
            '[' => square += 1,
            ']' => square -= 1,
            '{' => curly += 1,
            '}' => curly -= 1,
            _ => {}
        }

        current.push(ch);
    }

    if let Some(token_start) = start {
        if !current.trim().is_empty() {
            out.push(TokenSegment {
                text: current,
                start: token_start,
            });
        }
    }

    out
}

fn parse_value_context(current_token: &str, token_start: usize) -> Option<ValueCompletionContext> {
    let eq_index = current_token.find('=')?;
    let parameter_token = current_token[..eq_index].to_string();
    let raw_values = &current_token[eq_index + 1..];
    let value_offset = last_top_level_comma_index(raw_values)
        .map(|idx| idx + 1)
        .unwrap_or(0);
    let value_fragment = raw_values[value_offset..].to_string();
    let value_replace_start = token_start + parameter_token.len() + 1 + value_offset;
    Some(ValueCompletionContext {
        parameter_token,
        value_fragment,
        value_replace_start,
        token_start,
        value_prefix: raw_values[..value_offset].to_string(),
    })
}

fn last_top_level_comma_index(text: &str) -> Option<usize> {
    let mut last = None;
    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;

    for (idx, ch) in text.char_indices() {
        match ch {
            '(' => round += 1,
            ')' => round -= 1,
            '[' => square += 1,
            ']' => square -= 1,
            '{' => curly += 1,
            '}' => curly -= 1,
            ',' if round == 0 && square == 0 && curly == 0 => last = Some(idx),
            _ => {}
        }
    }

    last
}

fn mode_like_prompt_command(previous_tokens: &[String]) -> bool {
    previous_tokens
        .last()
        .map(String::as_str)
        .map(|token| token == ":mode")
        .unwrap_or(false)
}

fn shell_like_prompt_command(previous_tokens: &[String], current_token: &str) -> bool {
    let first = previous_tokens
        .first()
        .map(String::as_str)
        .or_else(|| (!current_token.is_empty()).then_some(current_token));

    matches!(first, Some("shell") | Some("python") | Some("math")) && !previous_tokens.is_empty()
}

fn section_from_main_token(token: &str) -> Option<RetaMainSection> {
    match token {
        "-zeilen" => Some(RetaMainSection::Zeilen),
        "-spalten" => Some(RetaMainSection::Spalten),
        "-kombination" => Some(RetaMainSection::Kombination),
        "-ausgabe" => Some(RetaMainSection::Ausgabe),
        _ => None,
    }
}

fn is_main_switch(token: &str) -> bool {
    let normalized = normalize_completion_text(token);
    reta_main_switches().any(|candidate| normalize_completion_text(candidate) == normalized)
}

fn should_offer_main_switches_for_fragment(fragment: &str) -> bool {
    if fragment.is_empty() {
        return false;
    }

    let normalized_body = normalize_dash_insensitive_fragment(fragment);
    fragment.starts_with('-')
        || reta_main_switches().any(|candidate| {
            normalize_dash_insensitive_fragment(candidate).starts_with(&normalized_body)
        })
}

fn ordered_prompt_commands() -> Vec<String> {
    let mut commands = prompt_words().befehle.clone();
    commands.sort_by(|left, right| {
        prompt_command_sort_key(left)
            .cmp(&prompt_command_sort_key(right))
            .then_with(|| normalize_completion_text(left).cmp(&normalize_completion_text(right)))
    });

    let mut out = Vec::new();
    let mut seen = BTreeSet::new();

    for item in [
        "HELP",
        "help",
        "hilfe",
        "kurzbefehle",
        "universum",
        "thomas",
        "befehle",
        "groesse",
        "reta",
        "bewusstsein",
        "geist",
        "emotion",
        "impulse",
        "loggen",
        "nichtloggen",
        "q",
        ":q",
        "exit",
        "quit",
        "ende",
    ] {
        push_unique_ordered(&mut out, &mut seen, item);
    }

    for item in commands {
        push_unique_ordered(&mut out, &mut seen, item);
    }

    for item in prompt_eig_commands_ordered() {
        push_unique_ordered(&mut out, &mut seen, item);
    }

    for item in RP_META_COMMANDS {
        push_unique_ordered(&mut out, &mut seen, *item);
    }

    out
}

fn prompt_non_reta_commands() -> Vec<String> {
    ordered_prompt_commands()
        .into_iter()
        .filter(|command| normalize_completion_text(command) != "reta")
        .collect()
}

fn prompt_eig_commands_ordered() -> Vec<String> {
    let words = shared_words();
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    let eig_prefixes = &prompt_words().eig_prefixes;

    for entry in &words.paraNdataMatrix {
        let prefix = if entry
            .parameterMainNames
            .iter()
            .any(|name| normalize_completion_text(name) == "konzept")
        {
            eig_prefixes.0.as_str()
        } else if entry
            .parameterMainNames
            .iter()
            .any(|name| normalize_completion_text(name) == "konzept2")
        {
            eig_prefixes.1.as_str()
        } else {
            continue;
        };

        for alias in &entry.parameterNames {
            push_unique_ordered(&mut out, &mut seen, format!("{prefix}{alias}"));
        }
    }

    out
}

fn prompt_command_sort_key(command: &str) -> (u8, String) {
    let normalized = normalize_completion_text(command);
    let eig_prefixes = &prompt_words().eig_prefixes;
    let looks_like_eig = normalized.starts_with(&normalize_completion_text(&eig_prefixes.0))
        || normalized.starts_with(&normalize_completion_text(&eig_prefixes.1));

    let bucket = if matches!(normalized.as_str(), "absicht" | "hilfe" | "kurzbefehle") {
        0
    } else if matches!(
        normalized.as_str(),
        "universum" | "thomas" | "befehle" | "groesse"
    ) {
        1
    } else if matches!(
        normalized.as_str(),
        "reta" | "bewusstsein" | "geist" | "emotion" | "impulse"
    ) {
        2
    } else if matches!(
        normalized.as_str(),
        "loggen" | "nichtloggen" | "exit" | "quit" | "ende" | "q" | ":q"
    ) {
        3
    } else if looks_like_eig {
        19
    } else if normalized.len() == 1 {
        7
    } else if normalized == "15" || normalized.starts_with("15_") {
        8
    } else if normalized == "16" || normalized.starts_with("16_") {
        9
    } else if normalized.starts_with('1') {
        10
    } else {
        5
    };

    (bucket, normalized)
}

fn build_completion_candidates(
    candidates: Vec<String>,
    fragment: &str,
    replace_start: usize,
    description: Option<String>,
    append_whitespace: bool,
) -> Vec<CompletionCandidate> {
    build_completion_candidates_with_match_mode(
        candidates,
        fragment,
        replace_start,
        description,
        append_whitespace,
        CandidateMatchMode::Normal,
    )
}

fn build_structural_completion_candidates(
    candidates: Vec<String>,
    fragment: &str,
    replace_start: usize,
    description: Option<String>,
    append_whitespace: bool,
) -> Vec<CompletionCandidate> {
    build_completion_candidates_with_match_mode(
        candidates,
        fragment,
        replace_start,
        description,
        append_whitespace,
        CandidateMatchMode::DashInsensitiveStructural,
    )
}

fn build_completion_candidates_with_match_mode(
    candidates: Vec<String>,
    fragment: &str,
    replace_start: usize,
    description: Option<String>,
    append_whitespace: bool,
    match_mode: CandidateMatchMode,
) -> Vec<CompletionCandidate> {
    filter_candidate_values_with_match_mode(&candidates, fragment, true, match_mode)
        .into_iter()
        .map(|value| CompletionCandidate {
            append_whitespace: append_whitespace && !value.ends_with('='),
            description: description
                .clone()
                .or_else(|| semantic_choice_completion_description(&value)),
            replace_start,
            value,
        })
        .collect()
}

fn semantic_choice_completion_description(value: &str) -> Option<String> {
    let normalized = normalize_completion_text(value);

    if normalized == "16_15" {
        return semantic_wahl15_value("15")
            .map(|choice| format!("wahl15[15] = {choice}"));
    }
    if let Some(suffix) = normalized.strip_prefix("16_15_") {
        return semantic_wahl15_value(suffix)
            .map(|choice| format!("wahl15[{suffix}] = {choice}"));
    }
    if let Some(suffix) = normalized.strip_prefix("15_") {
        return semantic_wahl15_value(suffix)
            .map(|choice| format!("wahl15[{suffix}] = {choice}"));
    }
    if let Some(suffix) = normalized.strip_prefix("16_") {
        return semantic_wahl16_value(suffix)
            .map(|choice| format!("wahl16[{suffix}] = {choice}"));
    }

    None
}

#[derive(Clone, Debug)]
enum SpecialFragmentMatcher {
    Any {
        negative_only: bool,
    },
    Glob {
        negative_only: bool,
        pattern: String,
    },
    RegexLike {
        negative_only: bool,
        pattern: String,
    },
}

fn filter_candidate_values(
    candidates: &[String],
    fragment: &str,
    fallback_contains: bool,
) -> Vec<String> {
    filter_candidate_values_with_match_mode(
        candidates,
        fragment,
        fallback_contains,
        CandidateMatchMode::Normal,
    )
}

fn filter_candidate_values_with_match_mode(
    candidates: &[String],
    fragment: &str,
    fallback_contains: bool,
    match_mode: CandidateMatchMode,
) -> Vec<String> {
    if let Some(matches) = filter_candidates_for_special_fragment(candidates, fragment) {
        return matches;
    }

    let normalized_fragment = normalize_completion_text(fragment);
    let dash_normalized_fragment = normalize_dash_insensitive_fragment(fragment);
    let mut prefix_matches = Vec::new();
    let mut seen = BTreeSet::new();

    for candidate in candidates {
        let normalized_candidate = normalize_completion_text(candidate);
        if normalized_fragment.is_empty()
            || normalized_candidate.starts_with(&normalized_fragment)
            || structural_dash_insensitive_prefix_match(
                candidate,
                &dash_normalized_fragment,
                match_mode,
            )
        {
            if seen.insert(normalized_candidate) {
                prefix_matches.push(candidate.clone());
            }
        }
    }

    if !fallback_contains || normalized_fragment.is_empty() {
        return prefix_matches;
    }

    let mut fuzzy_matches = Vec::new();
    for (index, candidate) in candidates.iter().enumerate() {
        let normalized_candidate = normalize_completion_text(candidate);
        if seen.contains(&normalized_candidate) {
            continue;
        }
        if let Some(score) = fuzzy_score_for_candidate_fragment(
            candidate,
            &normalized_candidate,
            &normalized_fragment,
            &dash_normalized_fragment,
            match_mode,
        ) {
            fuzzy_matches.push((score, index, normalized_candidate, candidate.clone()));
        }
    }

    fuzzy_matches.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| left.1.cmp(&right.1))
            .then_with(|| left.2.cmp(&right.2))
    });

    let mut out = prefix_matches;
    for (_, _, normalized_candidate, candidate) in fuzzy_matches {
        if seen.insert(normalized_candidate) {
            out.push(candidate);
        }
    }

    out
}

fn structural_dash_insensitive_prefix_match(
    candidate: &str,
    dash_normalized_fragment: &str,
    match_mode: CandidateMatchMode,
) -> bool {
    if match_mode != CandidateMatchMode::DashInsensitiveStructural
        || !is_structural_dash_candidate(candidate)
    {
        return false;
    }

    normalize_dash_insensitive_fragment(candidate).starts_with(dash_normalized_fragment)
}

fn fuzzy_score_for_candidate_fragment(
    candidate: &str,
    normalized_candidate: &str,
    normalized_fragment: &str,
    dash_normalized_fragment: &str,
    match_mode: CandidateMatchMode,
) -> Option<FuzzyCompletionScore> {
    if match_mode == CandidateMatchMode::DashInsensitiveStructural
        && !dash_normalized_fragment.is_empty()
        && is_structural_dash_candidate(candidate)
    {
        let dash_normalized_candidate = normalize_dash_insensitive_fragment(candidate);
        if let Some(score) =
            fuzzy_completion_score(&dash_normalized_candidate, dash_normalized_fragment)
        {
            return Some(score);
        }
    }

    fuzzy_completion_score(normalized_candidate, normalized_fragment)
}

fn normalize_dash_insensitive_fragment(text: &str) -> String {
    normalize_completion_text(text.trim_start_matches('-'))
}

fn is_structural_dash_candidate(candidate: &str) -> bool {
    if !candidate.starts_with('-') {
        return false;
    }

    is_main_switch(candidate) || candidate.starts_with("--")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct FuzzyCompletionScore {
    start: usize,
    span: usize,
    gaps: usize,
    candidate_len: usize,
}

fn fuzzy_completion_score(candidate: &str, fragment: &str) -> Option<FuzzyCompletionScore> {
    if fragment.is_empty() {
        return Some(FuzzyCompletionScore {
            start: 0,
            span: 0,
            gaps: 0,
            candidate_len: candidate.chars().count(),
        });
    }

    let mut fragment_chars = fragment.chars();
    let mut wanted = fragment_chars.next()?;
    let mut matched_positions = Vec::new();

    for (index, ch) in candidate.chars().enumerate() {
        if ch == wanted {
            matched_positions.push(index);
            if let Some(next) = fragment_chars.next() {
                wanted = next;
            } else {
                let start = matched_positions[0];
                let end = *matched_positions.last().unwrap_or(&start);
                let span = end.saturating_sub(start) + 1;
                let gaps = span.saturating_sub(matched_positions.len());
                return Some(FuzzyCompletionScore {
                    start,
                    span,
                    gaps,
                    candidate_len: candidate.chars().count(),
                });
            }
        }
    }

    None
}

fn filter_candidates_for_special_fragment(
    candidates: &[String],
    fragment: &str,
) -> Option<Vec<String>> {
    let matcher = parse_special_fragment_matcher(fragment)?;
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();

    for candidate in candidates {
        if !special_fragment_matches_candidate(candidate, &matcher) {
            continue;
        }
        let key = normalize_completion_text(candidate);
        if seen.insert(key) {
            out.push(candidate.clone());
        }
    }

    Some(out)
}

fn parse_special_fragment_matcher(fragment: &str) -> Option<SpecialFragmentMatcher> {
    let trimmed = fragment.trim();
    if trimmed.is_empty() {
        return None;
    }

    let (negative_only, core) = if let Some(rest) = trimmed.strip_prefix('-') {
        (true, rest)
    } else {
        (false, trimmed)
    };

    if core == "*" {
        return Some(SpecialFragmentMatcher::Any { negative_only });
    }

    if let Some(pattern) = parse_python_raw_regex_fragment(core) {
        return Some(SpecialFragmentMatcher::RegexLike {
            negative_only,
            pattern: pattern.to_string(),
        });
    }

    if core.contains('*') {
        return Some(SpecialFragmentMatcher::Glob {
            negative_only,
            pattern: core.to_string(),
        });
    }

    None
}

fn parse_python_raw_regex_fragment(text: &str) -> Option<&str> {
    if let Some(rest) = text.strip_prefix("r\"") {
        return Some(rest.strip_suffix('\"').unwrap_or(rest));
    }
    if let Some(rest) = text.strip_prefix("r'") {
        return Some(rest.strip_suffix('\'').unwrap_or(rest));
    }
    if let Some(rest) = text.strip_prefix("R\"") {
        return Some(rest.strip_suffix('\"').unwrap_or(rest));
    }
    if let Some(rest) = text.strip_prefix("R'") {
        return Some(rest.strip_suffix('\'').unwrap_or(rest));
    }
    None
}
fn special_fragment_matches_candidate(candidate: &str, matcher: &SpecialFragmentMatcher) -> bool {
    let (negative_only, body) = match matcher {
        SpecialFragmentMatcher::Any { negative_only }
        | SpecialFragmentMatcher::Glob { negative_only, .. }
        | SpecialFragmentMatcher::RegexLike { negative_only, .. } => (*negative_only, candidate),
    };

    let Some(candidate_body) = strip_optional_negative_prefix(body, negative_only) else {
        return false;
    };

    match matcher {
        SpecialFragmentMatcher::Any { .. } => true,
        SpecialFragmentMatcher::Glob { pattern, .. } => glob_like_match(
            &normalize_completion_text(pattern),
            &normalize_completion_text(candidate_body),
        ),
        SpecialFragmentMatcher::RegexLike { pattern, .. } => regex_like_search(
            &normalize_completion_text(pattern),
            &normalize_completion_text(candidate_body),
        ),
    }
}

fn strip_optional_negative_prefix<'a>(candidate: &'a str, negative_only: bool) -> Option<&'a str> {
    if !negative_only {
        return Some(candidate);
    }
    candidate.strip_prefix('-')
}

fn glob_like_match(pattern: &str, text: &str) -> bool {
    if pattern.is_empty() {
        return text.is_empty();
    }

    let pattern_chars = pattern.chars().collect::<Vec<_>>();
    let text_chars = text.chars().collect::<Vec<_>>();
    let mut memo = std::collections::BTreeMap::new();
    glob_like_match_chars(&pattern_chars, &text_chars, 0, 0, &mut memo)
}

fn glob_like_match_chars(
    pattern: &[char],
    text: &[char],
    pattern_index: usize,
    text_index: usize,
    memo: &mut std::collections::BTreeMap<(usize, usize), bool>,
) -> bool {
    if let Some(cached) = memo.get(&(pattern_index, text_index)) {
        return *cached;
    }

    let result = if pattern_index == pattern.len() {
        text_index == text.len()
    } else if pattern[pattern_index] == '*' {
        glob_like_match_chars(pattern, text, pattern_index + 1, text_index, memo)
            || (text_index < text.len()
                && glob_like_match_chars(pattern, text, pattern_index, text_index + 1, memo))
    } else {
        text_index < text.len()
            && pattern[pattern_index] == text[text_index]
            && glob_like_match_chars(pattern, text, pattern_index + 1, text_index + 1, memo)
    };

    memo.insert((pattern_index, text_index), result);
    result
}

fn regex_like_search(pattern: &str, text: &str) -> bool {
    python_regex_like_search(pattern, text)
}

fn normalize_completion_text(text: &str) -> String {
    text.trim().replace('ß', "ss").to_lowercase()
}

fn push_unique_ordered(
    target: &mut Vec<String>,
    seen: &mut BTreeSet<String>,
    value: impl Into<String>,
) {
    let value = value.into();
    let key = normalize_completion_text(&value);
    if seen.insert(key) {
        target.push(value);
    }
}

fn merge_unique(mut left: Vec<String>, right: Vec<String>) -> Vec<String> {
    let mut seen = left
        .iter()
        .map(|value| normalize_completion_text(value))
        .collect::<BTreeSet<_>>();
    for value in right {
        push_unique_ordered(&mut left, &mut seen, value);
    }
    left
}

fn main_switches_vec() -> Vec<String> {
    reta_main_switches()
        .into_iter()
        .map(str::to_string)
        .collect()
}

fn reta_main_switches() -> impl Iterator<Item = &'static str> {
    RETAPROMPT_RETA_MAIN_SWITCHES.iter().copied()
}

fn zeilen_parameter_tokens() -> impl Iterator<Item = &'static str> {
    RETAPROMPT_ZEILEN_PARAMETER_TOKENS.iter().copied()
}

fn ausgabe_parameter_tokens() -> impl Iterator<Item = &'static str> {
    RETAPROMPT_AUSGABE_PARAMETER_TOKENS.iter().copied()
}

fn kombi_parameter_tokens() -> impl Iterator<Item = &'static str> {
    RETAPROMPT_KOMBINATION_PARAMETER_TOKENS.iter().copied()
}

fn spalten_parameter_tokens() -> Vec<String> {
    let words = shared_words();
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();

    for entry in &words.paraNdataMatrix {
        for alias in &entry.parameterMainNames {
            push_unique_ordered(&mut out, &mut seen, format!("--{alias}="));
        }
    }

    for extra in [
        "--=",
        "--breite=",
        "--breiten=",
        "--keinenummerierung",
        "--*=",
    ] {
        push_unique_ordered(&mut out, &mut seen, extra);
    }

    out
}

fn zeilen_value_candidates(key: &str) -> Vec<String> {
    let normalized = normalize_completion_text(key);
    if normalized == normalize_completion_text(RETAPROMPT_ZEILEN_TYP_PARAMETER) {
        return with_negative_variants_and_any(RETAPROMPT_ZEILEN_TYP_VALUES);
    }
    if normalized == normalize_completion_text(RETAPROMPT_ZEILEN_PRIMZAHLEN_PARAMETER) {
        return with_negative_variants_and_any(RETAPROMPT_ZEILEN_PRIMZAHLEN_VALUES);
    }
    if normalized == normalize_completion_text(RETAPROMPT_ZEILEN_ZEIT_PARAMETER) {
        return with_negative_variants_and_any(RETAPROMPT_ZEILEN_ZEIT_VALUES);
    }
    if normalized == "*" {
        let mut out = Vec::new();
        let mut seen = BTreeSet::new();
        for value in zeilen_value_candidates(RETAPROMPT_ZEILEN_TYP_PARAMETER) {
            push_unique_ordered(&mut out, &mut seen, value);
        }
        for value in zeilen_value_candidates(RETAPROMPT_ZEILEN_PRIMZAHLEN_PARAMETER) {
            push_unique_ordered(&mut out, &mut seen, value);
        }
        for value in zeilen_value_candidates(RETAPROMPT_ZEILEN_ZEIT_PARAMETER) {
            push_unique_ordered(&mut out, &mut seen, value);
        }
        return out;
    }

    match normalized.as_str() {
        "zaehlung"
        | "vorhervonausschnitt"
        | "primzahlvielfache"
        | "nachtraeglichneuabzaehlung"
        | "nachtraeglichneuabzaehlungvielfache"
        | "potenzenvonzahlen"
        | "vielfachevonzahlen"
        | "oberesmaximum" => (0..100).map(|n| n.to_string()).collect(),
        _ => Vec::new(),
    }
}

fn ausgabe_value_candidates(key: &str) -> Vec<String> {
    let normalized = normalize_completion_text(key);
    if normalized == normalize_completion_text(RETAPROMPT_AUSGABE_ART_PARAMETER)
        || normalized == "*"
    {
        return RETAPROMPT_AUSGABE_ART_VALUES
            .iter()
            .map(|value| (*value).to_string())
            .collect();
    }
    if normalized == normalize_completion_text(RETAPROMPT_AUSGABE_BREITE_PARAMETER)
        || normalized == normalize_completion_text(RETAPROMPT_AUSGABE_BREITEN_PARAMETER)
    {
        return (10..100).map(|n| n.to_string()).collect();
    }

    Vec::new()
}

fn kombi_value_candidates(key: &str) -> Vec<String> {
    let words = shared_words();
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();

    let add_flattened = |target: &mut Vec<String>,
                         seen: &mut BTreeSet<String>,
                         values: &indexmap::IndexMap<i64, Vec<String>>| {
        for entries in values.values() {
            for entry in entries {
                push_unique_ordered(target, seen, entry.clone());
            }
        }
    };

    let normalized = normalize_completion_text(key);
    if normalized == normalize_completion_text(RETAPROMPT_KOMBINATION_GALAXIE_PARAMETER) {
        add_flattened(&mut out, &mut seen, &words.kombiParaNdataMatrix);
    } else if normalized == normalize_completion_text(RETAPROMPT_KOMBINATION_UNIVERSUM_PARAMETER) {
        add_flattened(&mut out, &mut seen, &words.kombiParaNdataMatrix2);
    } else if normalized == "*" {
        add_flattened(&mut out, &mut seen, &words.kombiParaNdataMatrix);
        add_flattened(&mut out, &mut seen, &words.kombiParaNdataMatrix2);
    }

    out
}

fn spalten_value_candidates(key: &str) -> Vec<String> {
    if key == "breite" || key == "breiten" {
        return (10..100).map(|n| n.to_string()).collect();
    }

    let words = shared_words();
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();

    if key == "*" || key.is_empty() {
        for entry in &words.paraNdataMatrix {
            for alias in &entry.parameterNames {
                push_unique_ordered(&mut out, &mut seen, alias.clone());
            }
        }
        return out;
    }

    let Some(canonical_main) = resolve_parameter_main_alias(words, key) else {
        return out;
    };
    let wanted = normalize_completion_text(&canonical_main);

    for entry in &words.paraNdataMatrix {
        if entry
            .parameterMainNames
            .iter()
            .any(|name| normalize_completion_text(name) == wanted)
        {
            for alias in &entry.parameterNames {
                push_unique_ordered(&mut out, &mut seen, alias.clone());
            }
        }
    }

    out
}

fn with_negative_variants_and_any(values: &[&'static str]) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for value in values.iter().copied().chain(std::iter::once("*")) {
        push_unique_ordered(&mut out, &mut seen, value);
        push_unique_ordered(&mut out, &mut seen, format!("-{value}"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        autosuggestion_for_input, autosuggestion_for_input_in_mode_with_context,
        candidates_for_input, candidates_for_input_in_mode_with_context, normalize_completion_text,
        PromptModus,
    };

    fn contains_normalized(values: &[String], expected: &str) -> bool {
        let expected = normalize_completion_text(expected);
        values
            .iter()
            .any(|value| normalize_completion_text(value) == expected)
    }

    #[test]
    fn prompt_top_level_contains_reta_and_help() {
        let values = candidates_for_input("");
        assert!(contains_normalized(&values, "reta"));
        assert!(contains_normalized(&values, "help"));
    }

    #[test]
    fn direct_cli_context_suggests_zeilen_parameter() {
        let values = candidates_for_input("-zeilen --ze");
        assert!(contains_normalized(&values, "--zeit="));
    }

    #[test]
    fn prompt_prefix_keeps_zeilen_context_for_parameters() {
        let values = candidates_for_input("a -zeilen --ze");
        assert!(contains_normalized(&values, "--zeit="));
    }

    #[test]
    fn prompt_prefix_keeps_spalten_context_after_reta_commands() {
        let values = candidates_for_input("a reta -spalten --mens");
        assert!(values
            .iter()
            .any(|value| normalize_completion_text(value).contains("menschliches=")));
    }

    #[test]
    fn non_reta_context_after_row_spec_offers_main_switches() {
        let values = candidates_for_input("a 1/2 ");
        assert!(contains_normalized(&values, "-zeilen"));
        assert!(contains_normalized(&values, "-spalten"));
    }

    #[test]
    fn non_reta_section_context_keeps_prompt_commands_like_python() {
        let values = candidates_for_input("a -zeilen ");
        assert!(contains_normalized(&values, "--zeit="));
        assert!(contains_normalized(&values, "help"));
    }

    #[test]
    fn reta_section_context_stays_stricter_than_prompt_prefix() {
        let values = candidates_for_input("reta -zeilen ");
        assert!(contains_normalized(&values, "--zeit="));
        assert!(!contains_normalized(&values, "help"));
    }

    #[test]
    fn spalten_value_completion_uses_python_parameter_aliases() {
        let values = candidates_for_input("reta -spalten --menschliches=bew");
        assert!(contains_normalized(&values, "bewusstsein"));
    }

    #[test]
    fn zeilen_time_completion_suggests_today_alias() {
        let values = candidates_for_input("reta -zeilen --zeit=h");
        assert!(contains_normalized(&values, "heute"));
    }

    #[test]
    fn value_completion_ignores_commas_inside_brackets() {
        let values = candidates_for_input("reta -zeilen --zeit=[heute,gestern],m");
        assert!(contains_normalized(&values, "morgen"));
    }

    #[test]
    fn mode_completion_suggests_vi() {
        let values = candidates_for_input(":mode v");
        assert!(contains_normalized(&values, "vi"));
    }

    #[test]
    fn shell_context_does_not_fall_back_to_reta_completion() {
        let values = candidates_for_input("shell -l");
        assert!(values.is_empty());
    }

    #[test]
    fn delete_select_mode_disables_completion_candidates_like_python_promptinput() {
        let values = super::candidates_for_input_in_mode(
            "reta -zeilen --zeit=h",
            PromptModus::LoeschenSelect,
        );
        assert!(values.is_empty());

        let values = candidates_for_input_in_mode_with_context(
            "1-",
            PromptModus::LoeschenSelect,
            &[],
            &[
                "reta".to_string(),
                "-zeilen".to_string(),
                "--zeit=heute".to_string(),
            ],
        );
        assert!(values.is_empty());
    }

    #[test]
    fn help_stays_near_the_front_of_top_level_candidates() {
        let values = candidates_for_input("");
        let help_index = values
            .iter()
            .position(|value| normalize_completion_text(value) == "help")
            .unwrap();
        let fifteen_index = values
            .iter()
            .position(|value| normalize_completion_text(value).starts_with("15_"))
            .unwrap();
        assert!(help_index < fifteen_index);
    }

    #[test]
    fn numeric_value_candidates_keep_python_like_order() {
        let values = candidates_for_input("reta -zeilen --zaehlung=");
        assert_eq!(
            values.iter().take(5).cloned().collect::<Vec<_>>(),
            vec![
                "0".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
            ]
        );
    }

    #[test]
    fn stored_prefix_context_suggests_section_parameters_without_retyping_prefix() {
        let values = candidates_for_input_in_mode_with_context(
            "--ze",
            PromptModus::Normal,
            &["reta".to_string(), "-zeilen".to_string()],
            &[],
        );
        assert!(contains_normalized(&values, "--zeit="));
    }

    #[test]
    fn stored_prefix_context_is_ignored_when_raw_reta_input_starts_first() {
        let values = candidates_for_input_in_mode_with_context(
            "reta -zeilen --ze",
            PromptModus::Normal,
            &["a".to_string(), "1/2".to_string()],
            &[],
        );
        assert!(contains_normalized(&values, "--zeit="));
        assert!(!contains_normalized(&values, "help"));
    }

    #[test]
    fn prompt_top_level_includes_python_eig_commands() {
        let values = candidates_for_input("EIGNwei");
        assert!(contains_normalized(&values, "EIGNweisheit"));
    }

    #[test]
    fn spalten_parameters_include_python_special_passthrough_options() {
        let values = candidates_for_input("reta -spalten --");
        assert!(contains_normalized(&values, "--="));
        assert!(contains_normalized(&values, "--breite="));
        assert!(contains_normalized(&values, "--keinenummerierung"));
    }

    #[test]
    fn ausgabe_parameters_include_python_endless_variants() {
        let values = candidates_for_input("reta -ausgabe --en");
        assert!(contains_normalized(&values, "--endlessscreen"));
        assert!(contains_normalized(&values, "--endless"));
    }

    #[test]
    fn wildcard_zeilen_values_follow_python_like_source_order() {
        let values = candidates_for_input("reta -zeilen --*=");
        assert_eq!(
            values.iter().take(5).cloned().collect::<Vec<_>>(),
            vec![
                "sonne".to_string(),
                "-sonne".to_string(),
                "mond".to_string(),
                "-mond".to_string(),
                "planet".to_string(),
            ]
        );
    }
    #[test]
    fn raw_regex_fragment_matches_prompt_commands_like_python() {
        let values = candidates_for_input(r#"2 r"absi"#);
        assert!(contains_normalized(&values, "absicht"));
        assert!(contains_normalized(&values, "absichten"));
    }

    #[test]
    fn wildcard_value_fragment_expands_to_all_section_values() {
        let values = candidates_for_input("reta -zeilen --zeit=*");
        assert!(contains_normalized(&values, "heute"));
        assert!(contains_normalized(&values, "gestern"));
        assert!(contains_normalized(&values, "morgen"));
    }

    #[test]
    fn negative_wildcard_fragment_keeps_only_negative_value_candidates() {
        let values = candidates_for_input("reta -zeilen --typ=-*");
        assert!(contains_normalized(&values, "-sonne"));
        assert!(contains_normalized(&values, "-mond"));
        assert!(!contains_normalized(&values, "sonne"));
    }

    #[test]
    fn raw_regex_fragment_after_equals_filters_value_candidates() {
        let values = candidates_for_input(r#"reta -zeilen --zeit=r"mor"#);
        assert!(contains_normalized(&values, "morgen"));
        assert!(!contains_normalized(&values, "heute"));
    }

    #[test]
    fn zeilen_parameters_include_python_wildcard_value_parameter() {
        let values = candidates_for_input("reta -zeilen --*");
        assert!(contains_normalized(&values, "--*="));
    }

    #[test]
    fn fuzzy_prompt_completion_matches_python_fuzzy_word_completer() {
        let values = candidates_for_input("unv");
        assert!(contains_normalized(&values, "universum"));
    }

    #[test]
    fn fuzzy_value_completion_matches_python_fuzzy_word_completer() {
        let values = candidates_for_input("reta -zeilen --typ=snn");
        assert!(contains_normalized(&values, "sonne"));
    }

    #[test]
    fn zeilen_negative_wildcard_value_is_available_like_python() {
        let values = candidates_for_input("reta -zeilen --typ=-");
        assert!(contains_normalized(&values, "-*"));
    }


    #[test]
    fn prompt_top_level_includes_full_python_wahl15_wahl16_commands() {
        let values = candidates_for_input("15_1pro3");
        assert!(contains_normalized(&values, "15_1pro30"));
        assert!(contains_normalized(&values, "15_1pro3"));

        let values = candidates_for_input("16_1");
        assert!(contains_normalized(&values, "16_1"));
        assert!(contains_normalized(&values, "16_10"));
        assert!(contains_normalized(&values, "16_15"));
        assert!(contains_normalized(&values, "16_16"));
    }

    #[test]
    fn raw_regex_fragment_matches_prompt_commands_with_python_groups() {
        let values = candidates_for_input(r#"r"^(prim|multis)[0-9]+$""#);
        assert!(contains_normalized(&values, "prim24"));
        assert!(contains_normalized(&values, "multis3"));
        assert!(!contains_normalized(&values, "prim"));
    }

    #[test]
    fn raw_regex_fragment_after_equals_supports_python_alternation() {
        let values = candidates_for_input(r#"reta -zeilen --zeit=r"^(heute|morgen)$""#);
        assert!(contains_normalized(&values, "heute"));
        assert!(contains_normalized(&values, "morgen"));
        assert!(!contains_normalized(&values, "gestern"));
    }

    #[test]
    fn raw_regex_fragment_after_equals_supports_python_char_classes_and_plus() {
        let values = candidates_for_input(r#"reta -ausgabe --art=r"^h[a-z]+$""#);
        assert!(contains_normalized(&values, "html"));
        assert!(!contains_normalized(&values, "csv"));
    }

    #[test]
    fn reta_main_switches_include_python_nichts() {
        let values = candidates_for_input("reta -n");
        assert!(contains_normalized(&values, "-nichts"));
    }

    #[test]
    fn mistyped_value_parameter_suggests_close_python_dictionary_keys() {
        let values = candidates_for_input("reta -zeilen --ty=");
        assert!(contains_normalized(&values, "typ"));
    }

    #[test]
    fn stored_reta_context_does_not_hide_partial_prompt_commands() {
        let values = candidates_for_input_in_mode_with_context(
            "hel",
            PromptModus::Normal,
            &["reta".to_string(), "-zeilen".to_string()],
            &[],
        );
        assert!(contains_normalized(&values, "help"));
        assert!(!contains_normalized(&values, "--zeit="));
    }

    #[test]
    fn stored_reta_context_does_not_hide_fuzzy_prompt_commands() {
        let values = candidates_for_input_in_mode_with_context(
            "unv",
            PromptModus::Normal,
            &["reta".to_string(), "-zeilen".to_string()],
            &[],
        );
        assert!(contains_normalized(&values, "universum"));
    }

    #[test]
    fn value_parameter_lookup_is_case_insensitive_like_python_completion() {
        let values = candidates_for_input("reta -zeilen --Typ=s");
        assert!(contains_normalized(&values, "sonne"));
    }

    #[test]
    fn autosuggestion_uses_python_like_top_level_completion() {
        assert_eq!(autosuggestion_for_input("he").as_deref(), Some("lp "));
    }

    #[test]
    fn autosuggestion_uses_current_reta_section_context() {
        assert_eq!(
            autosuggestion_for_input("reta -zeilen --ze").as_deref(),
            Some("it=")
        );
    }

    #[test]
    fn autosuggestion_supports_stored_prefix_context() {
        assert_eq!(
            autosuggestion_for_input_in_mode_with_context(
                "--ze",
                PromptModus::Normal,
                &["reta".to_string(), "-zeilen".to_string()],
                &[],
            )
            .as_deref(),
            Some("it=")
        );
    }

    #[test]
    fn structural_completion_matches_main_switches_without_or_with_extra_dashes() {
        let values = candidates_for_input("reta ze");
        assert!(contains_normalized(&values, "-zeilen"));

        let values = candidates_for_input("reta --ze");
        assert!(contains_normalized(&values, "-zeilen"));
    }

    #[test]
    fn structural_completion_matches_parameters_without_or_with_extra_dashes() {
        let values = candidates_for_input("reta -zeilen zeit");
        assert!(contains_normalized(&values, "--zeit="));

        let values = candidates_for_input("reta -zeilen ---zeit");
        assert!(contains_normalized(&values, "--zeit="));
    }

    #[test]
    fn value_completion_canonicalizes_parameter_minus_prefix() {
        let values = candidates_for_input("reta -zeilen zeit=h");
        assert!(contains_normalized(&values, "--zeit=heute"));

        let values = candidates_for_input("reta -zeilen ---zeit=h");
        assert!(contains_normalized(&values, "--zeit=heute"));
    }

    #[test]
    fn autosuggestion_shows_main_switches_even_without_dash_prefix() {
        assert_eq!(autosuggestion_for_input("reta ze").as_deref(), Some(" → -zeilen "));
        assert_eq!(autosuggestion_for_input("reta --ze").as_deref(), Some(" → -zeilen "));
    }

    #[test]
    fn autosuggestion_shows_parameter_switches_even_without_dash_prefix() {
        assert_eq!(
            autosuggestion_for_input("reta -zeilen zeit").as_deref(),
            Some(" → --zeit=")
        );
        assert_eq!(
            autosuggestion_for_input("reta -zeilen ---zeit").as_deref(),
            Some(" → --zeit=")
        );
    }

    #[test]
    fn autosuggestion_does_not_insert_fuzzy_replacements_that_are_not_suffixes() {
        assert_eq!(autosuggestion_for_input("unv"), None);
    }

    #[test]
    fn autosuggestion_is_disabled_in_delete_select_mode_like_python_completer() {
        assert_eq!(
            autosuggestion_for_input_in_mode_with_context(
                "reta -zeilen --ze",
                PromptModus::LoeschenSelect,
                &[],
                &[],
            ),
            None
        );
    }

    #[test]
    fn semantic_15_completion_uses_full_python_wahl15_inventory() {
        let values = candidates_for_input("15_13_");
        assert!(contains_normalized(&values, "15_13_6"));
        assert!(contains_normalized(&values, "15_13_17"));
        assert!(contains_normalized(&values, "15_13_13"));
        assert!(contains_normalized(&values, "15_13_1pro8"));
    }

    #[test]
    fn semantic_16_completion_uses_python_wahl16_inventory_without_old_stub() {
        let values = candidates_for_input("16_");
        assert!(contains_normalized(&values, "16_1"));
        assert!(contains_normalized(&values, "16_2"));
        assert!(contains_normalized(&values, "16_20"));
        assert!(!contains_normalized(&values, "16_11"));
    }

    #[test]
    fn nested_16_15_completion_uses_full_python_wahl15_inventory() {
        let values = candidates_for_input("16_15_1pro");
        assert!(contains_normalized(&values, "16_15_1pro12"));
        assert!(contains_normalized(&values, "16_15_1pro13"));
        assert!(contains_normalized(&values, "16_15_1pro19"));
    }

    #[test]
    fn reta_main_completion_uses_generated_python_switch_table() {
        let values = candidates_for_input("reta -d");
        assert!(contains_normalized(&values, "-debug"));
    }

    #[test]
    fn semantic_choice_completion_descriptions_are_data_driven() {
        let candidates = super::completion_candidates_for_line("15_9_");
        let candidate = candidates
            .iter()
            .find(|candidate| normalize_completion_text(&candidate.value) == "15_9_6")
            .expect("15_9_6 semantic completion candidate");
        assert_eq!(
            candidate.description.as_deref(),
            Some("wahl15[9_6] = Größenordnung")
        );
    }

}
