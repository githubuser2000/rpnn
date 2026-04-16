use std::collections::BTreeSet;
use std::sync::{Arc, Mutex, OnceLock};

use reedline::{Completer as ReedlineCompleter, Span, Suggestion};

use crate::domain::python_source_of_truth::{
    all_main_alias_groups, parameter_alias_groups_for_main, resolve_parameter_main_alias,
};
use crate::shared_words;

use super::python_like::{
    expand_kurz_kurz_befehl, looks_like_numeric_or_fraction_range, prompt_words, PromptModus,
};

pub const RP_META_COMMANDS: &[&str] = &[
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
    Value,
    NeitherNor,
    RetaAnfang,
    Unbekannt,
    SpaltenPara,
    KomiPara,
    KombiMetaPara,
    AusgabePara,
    SpaltenValPara,
    ZeilenValPara,
    KombiValPara,
    AusgabeValPara,
    BefehleNichtReta,
}

#[derive(Clone, Debug)]
pub struct CompletionRuntimeState {
    pub prompt_mode: PromptModus,
}

impl Default for CompletionRuntimeState {
    fn default() -> Self {
        Self {
            prompt_mode: PromptModus::Normal,
        }
    }
}

pub type CompletionRuntimeHandle = Arc<Mutex<CompletionRuntimeState>>;

#[derive(Clone, Debug)]
struct PromptContextCompleter {
    runtime: CompletionRuntimeHandle,
}

#[derive(Clone, Debug)]
struct CompletionCandidate {
    value: String,
    description: Option<String>,
    replace_start: usize,
    append_whitespace: bool,
}

#[derive(Clone, Debug)]
struct TokenSegment {
    text: String,
    start: usize,
}

#[derive(Clone, Debug)]
struct PythonCompletionState {
    options: Vec<String>,
    if_reta_anfang: bool,
    situation: ComplSitua,
    spalten_para_wort: Option<String>,
    kombi_para_wort: Option<String>,
    ausgabe_para_wort: Option<String>,
    zeilen_para_wort: Option<String>,
    neben_para_wort: Option<String>,
    last_commands: Vec<String>,
}

impl PythonCompletionState {
    fn new() -> Self {
        Self {
            options: ordered_prompt_commands(),
            if_reta_anfang: false,
            situation: ComplSitua::RetaAnfang,
            spalten_para_wort: None,
            kombi_para_wort: None,
            ausgabe_para_wort: None,
            zeilen_para_wort: None,
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

pub fn build_default_completer() -> Box<dyn ReedlineCompleter> {
    build_default_completer_with_runtime(new_completion_runtime_handle())
}

pub fn build_default_completer_with_runtime(
    runtime: CompletionRuntimeHandle,
) -> Box<dyn ReedlineCompleter> {
    Box::new(PromptContextCompleter { runtime })
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

impl ReedlineCompleter for PromptContextCompleter {
    fn complete(&mut self, line: &str, pos: usize) -> Vec<Suggestion> {
        let before_cursor = safe_prefix(line, pos);
        let prompt_mode = self
            .runtime
            .lock()
            .map(|state| state.prompt_mode)
            .unwrap_or(PromptModus::Normal);

        completion_candidates_for_line_in_mode(before_cursor, prompt_mode)
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

fn completion_candidates_for_line(before_cursor: &str) -> Vec<CompletionCandidate> {
    completion_candidates_for_line_in_mode(before_cursor, PromptModus::Normal)
}

fn completion_candidates_for_line_in_mode(
    before_cursor: &str,
    prompt_mode: PromptModus,
) -> Vec<CompletionCandidate> {
    if matches!(
        prompt_mode,
        PromptModus::LoeschenStart | PromptModus::LoeschenSelect
    ) {
        return Vec::new();
    }

    let tokens = split_tokens_with_positions(before_cursor);
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

    if mode_like_prompt_command(&previous_text_tokens) {
        return build_completion_candidates(
            vec!["vi".to_string(), "emacs".to_string()],
            &current_token,
            current_start,
            None,
            true,
        );
    }

    if shell_like_prompt_command(&previous_text_tokens, &current_token) {
        return Vec::new();
    }

    let mut state = PythonCompletionState::new();
    for token in &previous_text_tokens {
        consume_space_token(&mut state, token);
    }

    if let Some((parameter_token, value_fragment, replace_start)) =
        parse_value_context(&current_token, current_start)
    {
        return build_value_candidates_from_state(
            &state,
            &parameter_token,
            &value_fragment,
            replace_start,
        );
    }

    let mut candidates = state.options.clone();
    if previous_text_tokens.is_empty() && current_token.starts_with('-') {
        candidates = merge_unique(candidates, main_switches_vec());
    }

    build_completion_candidates(candidates, &current_token, current_start, None, true)
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
    parameter_token: &str,
    fragment: &str,
    replace_start: usize,
) -> Vec<CompletionCandidate> {
    let stripped = parameter_token.trim_start_matches('-');
    let key = stripped.trim_end_matches('=');
    let section = state.current_section();
    let candidates = match section {
        Some(RetaMainSection::Zeilen) => zeilen_value_candidates(key),
        Some(RetaMainSection::Spalten) => spalten_value_candidates(key),
        Some(RetaMainSection::Kombination) => kombi_value_candidates(key),
        Some(RetaMainSection::Ausgabe) => ausgabe_value_candidates(key),
        None => Vec::new(),
    };

    build_completion_candidates(candidates, fragment, replace_start, None, false)
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

fn parse_value_context(current_token: &str, token_start: usize) -> Option<(String, String, usize)> {
    let eq_index = current_token.find('=')?;
    let parameter_token = current_token[..eq_index].to_string();
    let raw_values = &current_token[eq_index + 1..];
    let value_offset = last_top_level_comma_index(raw_values)
        .map(|idx| idx + 1)
        .unwrap_or(0);
    let value_fragment = raw_values[value_offset..].to_string();
    let replace_start = token_start + parameter_token.len() + 1 + value_offset;
    Some((parameter_token, value_fragment, replace_start))
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
    matches!(
        token,
        "-zeilen" | "-spalten" | "-kombination" | "-ausgabe" | "-h" | "-help"
    )
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
    filter_candidate_values(&candidates, fragment, true)
        .into_iter()
        .map(|value| CompletionCandidate {
            append_whitespace: append_whitespace && !value.ends_with('='),
            description: description.clone(),
            replace_start,
            value,
        })
        .collect()
}

fn filter_candidate_values(
    candidates: &[String],
    fragment: &str,
    fallback_contains: bool,
) -> Vec<String> {
    let normalized_fragment = normalize_completion_text(fragment);
    let mut prefix_matches = Vec::new();
    let mut contains_matches = Vec::new();
    let mut prefix_seen = BTreeSet::new();
    let mut contains_seen = BTreeSet::new();

    for candidate in candidates {
        let normalized_candidate = normalize_completion_text(candidate);
        if normalized_fragment.is_empty() || normalized_candidate.starts_with(&normalized_fragment)
        {
            if prefix_seen.insert(normalized_candidate.clone()) {
                prefix_matches.push(candidate.clone());
            }
        } else if fallback_contains && normalized_candidate.contains(&normalized_fragment) {
            if contains_seen.insert(normalized_candidate) {
                contains_matches.push(candidate.clone());
            }
        }
    }

    if !prefix_matches.is_empty() {
        prefix_matches
    } else {
        contains_matches
    }
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

fn reta_main_switches() -> [&'static str; 6] {
    [
        "-zeilen",
        "-spalten",
        "-kombination",
        "-ausgabe",
        "-h",
        "-help",
    ]
}

fn zeilen_parameter_tokens() -> [&'static str; 14] {
    [
        "--zeit=",
        "--zaehlung=",
        "--vorhervonausschnitt=",
        "--vorhervonausschnittteiler",
        "--primzahlvielfache=",
        "--nachtraeglichneuabzaehlung=",
        "--nachtraeglichneuabzaehlungvielfache=",
        "--alles",
        "--potenzenvonzahlen=",
        "--typ=",
        "--vielfachevonzahlen=",
        "--oberesmaximum=",
        "--primzahlen=",
        "--invertieren",
    ]
}

fn ausgabe_parameter_tokens() -> [&'static str; 11] {
    [
        "--art=",
        "--breite=",
        "--breiten=",
        "--justtext",
        "--keineleereninhalte",
        "--keinenummerierung",
        "--keineueberschriften",
        "--nocolor",
        "--onetable",
        "--spaltenreihenfolgeundnurdiese=",
        "--*=",
    ]
}

fn kombi_parameter_tokens() -> [&'static str; 3] {
    ["--galaxie=", "--universum=", "--*="]
}

fn spalten_parameter_tokens() -> Vec<String> {
    let words = shared_words();
    let mut out = BTreeSet::new();
    for group in all_main_alias_groups(words) {
        for alias in group.aliases {
            out.insert(format!("--{alias}="));
        }
    }
    out.insert("--*=".to_string());
    out.into_iter().collect()
}

fn zeilen_value_candidates(key: &str) -> Vec<String> {
    match key {
        "typ" => with_negative_variants([
            "sonne",
            "mond",
            "planet",
            "schwarzesonne",
            "SonneMitMondanteil",
            "*",
        ]),
        "primzahlen" => {
            with_negative_variants(["aussenerste", "innenerste", "aussenalle", "innenalle", "*"])
        }
        "zeit" => with_negative_variants(["heute", "gestern", "morgen", "*"]),
        "*" => {
            let mut set = BTreeSet::new();
            for value in zeilen_value_candidates("typ") {
                set.insert(value);
            }
            for value in zeilen_value_candidates("primzahlen") {
                set.insert(value);
            }
            for value in zeilen_value_candidates("zeit") {
                set.insert(value);
            }
            set.into_iter().collect()
        }
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
    match key {
        "art" | "*" => [
            "bbcode", "html", "csv", "shell", "markdown", "emacs", "nichts",
        ]
        .into_iter()
        .map(str::to_string)
        .collect(),
        "breite" | "breiten" => (10..100).map(|n| n.to_string()).collect(),
        _ => Vec::new(),
    }
}

fn kombi_value_candidates(key: &str) -> Vec<String> {
    let words = shared_words();
    let mut set = BTreeSet::new();

    let add_flattened = |target: &mut BTreeSet<String>,
                         values: &indexmap::IndexMap<i64, Vec<String>>| {
        for entries in values.values() {
            for entry in entries {
                target.insert(entry.clone());
            }
        }
    };

    match key {
        "galaxie" => add_flattened(&mut set, &words.kombiParaNdataMatrix),
        "universum" => add_flattened(&mut set, &words.kombiParaNdataMatrix2),
        "*" => {
            add_flattened(&mut set, &words.kombiParaNdataMatrix);
            add_flattened(&mut set, &words.kombiParaNdataMatrix2);
        }
        _ => {}
    }

    set.into_iter().collect()
}

fn spalten_value_candidates(key: &str) -> Vec<String> {
    if key == "breite" || key == "breiten" {
        return (10..100).map(|n| n.to_string()).collect();
    }

    let words = shared_words();
    let mut out = BTreeSet::new();

    if key == "*" || key.is_empty() {
        for group in all_main_alias_groups(words) {
            for parameter_group in parameter_alias_groups_for_main(words, &group.canonical) {
                for alias in parameter_group.aliases {
                    out.insert(alias);
                }
            }
        }
        return out.into_iter().collect();
    }

    if let Some(canonical_main) = resolve_parameter_main_alias(words, key) {
        for parameter_group in parameter_alias_groups_for_main(words, &canonical_main) {
            for alias in parameter_group.aliases {
                out.insert(alias);
            }
        }
    }

    out.into_iter().collect()
}

fn with_negative_variants<const N: usize>(values: [&'static str; N]) -> Vec<String> {
    let mut out = BTreeSet::new();
    for value in values {
        out.insert(value.to_string());
        if value != "*" {
            out.insert(format!("-{value}"));
        }
    }
    out.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::{candidates_for_input, normalize_completion_text, PromptModus};

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
    fn delete_mode_disables_completion_candidates() {
        let values = super::candidates_for_input_in_mode(
            "reta -zeilen --zeit=h",
            PromptModus::LoeschenSelect,
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
}
