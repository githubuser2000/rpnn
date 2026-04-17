use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use crate::domain::python_source_of_truth::{
    all_main_alias_groups, parameter_alias_groups_for_main,
};
use crate::shared_words;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromptModus {
    Normal,
    Speichern,
    LoeschenStart,
    SpeicherungAusgaben,
    LoeschenSelect,
    SpeicherungAusgabenMitZusatz,
    AusgabeSelektiv,
}

#[derive(Clone, Debug)]
pub struct PromptWords {
    pub befehle: Vec<String>,
    pub befehle_set: BTreeSet<String>,
    pub eig_prefixes: (String, String),
    pub one_char_commands: BTreeSet<String>,
}

static PROMPT_WORDS: OnceLock<PromptWords> = OnceLock::new();

pub fn prompt_words() -> &'static PromptWords {
    PROMPT_WORDS.get_or_init(build_prompt_words)
}

fn build_prompt_words() -> PromptWords {
    let mut befehle: Vec<String> = Vec::new();

    for key in [
        "15", "2", "5", "7", "8", "10", "12", "13", "17", "18", "6", "9", "3", "16", "4", "1",
        "30", "14", "20", "37", "31", "11", "36", "21", "26", "19", "90",
    ] {
        befehle.push(format!("15_{key}"));
    }
    befehle.push("15_".to_string());
    for key in [
        "15", "2", "5", "7", "8", "10", "12", "13", "17", "18", "6", "9", "3", "16", "4", "1",
    ] {
        befehle.push(format!("16_15_{key}"));
    }
    for key in ["15", "10", "11"] {
        befehle.push(format!("16_{key}"));
    }
    befehle.push("16_".to_string());

    for cmd in [
        "invertieren",
        "netzwerk",
        "komplex",
        "ee",
        "groesse",
        "emotion",
        "freiheit",
        "gleichheit",
        "kurzbefehle",
        "leeren",
        "kugeln",
        "kreise",
        "mond",
        "reta",
        "absicht",
        "motiv",
        "thomas",
        "universum",
        "impulse",
        "motive",
        "absichten",
        "primfaktorenvergleich",
        "vielfache",
        "einzeln",
        "multis",
        "multis3",
        "modulo",
        "prim",
        "primfaktorzerlegung",
        "prim24",
        "primfaktorzerlegungModulo24",
        "help",
        "hilfe",
        "abc",
        "abcd",
        "alles",
        "geist",
        "a",
        "R",
        "range",
        "B",
        "bewusstsein",
        "E",
        "G",
        "u",
        "I",
        "T",
        "W",
        "wirklichkeit",
        "triebe",
        "befehle",
        "t",
        "richtung",
        "r",
        "v",
        "h",
        "p",
        "primzahlkreuz",
        "ende",
        "exit",
        "quit",
        "q",
        ":q",
        "shell",
        "s",
        "math",
        "loggen",
        "nichtloggen",
        "mulpri",
        "python",
        "w",
        "teiler",
        "BefehlSpeichernDanach",
        "S",
        "BefehlSpeicherungLöschen",
        "l",
        "BefehlSpeicherungAusgeben",
        "o",
        "e",
        "BefehlSpeichernDavor",
        "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar",
        "abstand",
        "abstandPrim",
    ] {
        befehle.push(cmd.to_string());
    }

    befehle.extend(concept_prefixed_prompt_tokens());

    let befehle_set = befehle.iter().cloned().collect::<BTreeSet<_>>();
    let one_char_commands = [
        "a", "R", "B", "E", "G", "u", "I", "T", "W", "t", "r", "v", "h", "p", "s", "S", "l", "o",
        "e", "w", "q",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect::<BTreeSet<_>>();

    PromptWords {
        befehle,
        befehle_set,
        eig_prefixes: ("EIGN".to_string(), "EIGR".to_string()),
        one_char_commands,
    }
}

fn normalize_match_text(text: &str) -> String {
    text.trim().replace('ß', "ss").to_lowercase()
}

fn push_unique_preserving_normalized(
    target: &mut Vec<String>,
    seen: &mut BTreeSet<String>,
    value: String,
) {
    let normalized = normalize_match_text(&value);
    if seen.insert(normalized) {
        target.push(value);
    }
}

fn concept_parameter_aliases(canonical_main: &str) -> Vec<String> {
    let words = shared_words();
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for group in parameter_alias_groups_for_main(words, canonical_main) {
        for alias in group.aliases {
            let trimmed = alias.trim();
            if !trimmed.is_empty() {
                push_unique_preserving_normalized(&mut out, &mut seen, trimmed.to_string());
            }
        }
    }
    out
}

fn concept_prefixed_prompt_tokens() -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();

    for (prefix, canonical_main) in [("EIGN", "konzept"), ("EIGR", "konzept2")] {
        for alias in concept_parameter_aliases(canonical_main) {
            push_unique_preserving_normalized(&mut out, &mut seen, format!("{prefix}{alias}"));
        }
    }

    out
}

fn numeric_value_candidates_for_regex() -> Vec<String> {
    (0..=128).map(|value| value.to_string()).collect()
}

fn reta_main_switch_tokens_for_regex() -> &'static [&'static str] {
    &["-zeilen", "-spalten", "-kombination", "-ausgabe"]
}

fn zeilen_parameter_inventory_for_regex() -> BTreeMap<String, Vec<String>> {
    let mut inventory = BTreeMap::new();
    let numeric_values = numeric_value_candidates_for_regex();

    for key in [
        "zaehlung",
        "vorhervonausschnitt",
        "primzahlvielfache",
        "nachtraeglichneuabzaehlung",
        "nachtraeglichneuabzaehlungvielfache",
        "potenzenvonzahlen",
        "vielfachevonzahlen",
        "oberesmaximum",
    ] {
        inventory.insert(key.to_string(), numeric_values.clone());
    }

    inventory.insert(
        "zeit".to_string(),
        ["gestern", "heute", "morgen"]
            .into_iter()
            .map(str::to_string)
            .collect(),
    );
    inventory.insert(
        "typ".to_string(),
        [
            "mond",
            "sonne",
            "planet",
            "schwarzesonne",
            "SonneMitMondanteil",
        ]
        .into_iter()
        .map(str::to_string)
        .collect(),
    );
    inventory.insert(
        "primzahlen".to_string(),
        ["aussenerste", "innenerste", "innenalle", "aussenalle"]
            .into_iter()
            .map(str::to_string)
            .collect(),
    );

    for flag in ["vorhervonausschnittteiler", "alles", "invertieren"] {
        inventory.insert(flag.to_string(), Vec::new());
    }

    inventory
}

fn ausgabe_parameter_inventory_for_regex() -> BTreeMap<String, Vec<String>> {
    let mut inventory = BTreeMap::new();
    inventory.insert(
        "art".to_string(),
        [
            "bbcode", "html", "csv", "shell", "markdown", "emacs", "nichts",
        ]
        .into_iter()
        .map(str::to_string)
        .collect(),
    );
    inventory.insert(
        "breite".to_string(),
        (0..=128).map(|value| value.to_string()).collect(),
    );
    inventory.insert(
        "breiten".to_string(),
        (0..=128).map(|value| value.to_string()).collect(),
    );

    for flag in [
        "nocolor",
        "justtext",
        "onetable",
        "spaltenreihenfolgeundnurdiese",
        "endlessscreen",
        "endless",
        "dontwrap",
        "keineleereninhalte",
        "keinenummerierung",
        "keineueberschriften",
    ] {
        inventory.entry(flag.to_string()).or_default();
    }

    inventory
}

fn kombination_parameter_inventory_for_regex() -> BTreeMap<String, Vec<String>> {
    let words = shared_words();
    let mut inventory = BTreeMap::new();
    let mut galaxie = Vec::new();
    let mut galaxie_seen = BTreeSet::new();
    for values in words.kombiParaNdataMatrix.values() {
        for value in values {
            push_unique_preserving_normalized(&mut galaxie, &mut galaxie_seen, value.clone());
        }
    }
    inventory.insert("galaxie".to_string(), galaxie);

    let mut universum = Vec::new();
    let mut universum_seen = BTreeSet::new();
    for values in words.kombiParaNdataMatrix2.values() {
        for value in values {
            push_unique_preserving_normalized(&mut universum, &mut universum_seen, value.clone());
        }
    }
    inventory.insert("universum".to_string(), universum);
    inventory
}

fn spalten_parameter_inventory_for_regex() -> BTreeMap<String, Vec<String>> {
    let words = shared_words();
    let mut inventory = BTreeMap::new();

    for main_group in all_main_alias_groups(words) {
        let mut parameter_aliases = Vec::new();
        let mut seen = BTreeSet::new();
        for parameter_group in parameter_alias_groups_for_main(words, &main_group.canonical) {
            for alias in parameter_group.aliases {
                let trimmed = alias.trim();
                if !trimmed.is_empty() {
                    push_unique_preserving_normalized(
                        &mut parameter_aliases,
                        &mut seen,
                        trimmed.to_string(),
                    );
                }
            }
        }

        for alias in main_group.aliases {
            let trimmed = alias.trim();
            if !trimmed.is_empty() {
                inventory.insert(trimmed.to_string(), parameter_aliases.clone());
            }
        }
    }

    inventory
}

fn reta_section_parameter_inventory_for_regex(section: &str) -> BTreeMap<String, Vec<String>> {
    match section {
        "-zeilen" => zeilen_parameter_inventory_for_regex(),
        "-ausgabe" => ausgabe_parameter_inventory_for_regex(),
        "-kombination" => kombination_parameter_inventory_for_regex(),
        "-spalten" => spalten_parameter_inventory_for_regex(),
        _ => BTreeMap::new(),
    }
}

fn reta_global_parameter_inventory_for_regex() -> BTreeMap<String, Vec<String>> {
    let mut inventory = BTreeMap::new();
    for section in reta_main_switch_tokens_for_regex() {
        for (parameter, values) in reta_section_parameter_inventory_for_regex(section) {
            inventory.entry(parameter).or_insert(values);
        }
    }
    inventory
}

fn concept_values_from_fragment(prefix: &str, fragment: &str) -> Vec<String> {
    let canonical_main = if prefix == prompt_words().eig_prefixes.0 {
        "konzept"
    } else {
        "konzept2"
    };
    let aliases = concept_parameter_aliases(canonical_main);
    let matcher = parse_special_fragment_matcher(fragment);
    if matcher.is_none() {
        return (!fragment.trim().is_empty())
            .then_some(vec![fragment.trim().to_string()])
            .unwrap_or_default();
    }

    let matcher = matcher.unwrap();
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for alias in aliases {
        if special_fragment_matches_candidate(&alias, &matcher) {
            push_unique_preserving_normalized(&mut out, &mut seen, alias);
        }
    }
    out
}

fn collect_concept_prefixed_values(tokens: &[String]) -> (Vec<String>, Vec<String>) {
    let mut eig_n = Vec::new();
    let mut eig_r = Vec::new();
    let mut eig_n_seen = BTreeSet::new();
    let mut eig_r_seen = BTreeSet::new();

    for token in tokens {
        if let Some(fragment) = token.strip_prefix(&prompt_words().eig_prefixes.0) {
            for value in concept_values_from_fragment(&prompt_words().eig_prefixes.0, fragment) {
                push_unique_preserving_normalized(&mut eig_n, &mut eig_n_seen, value);
            }
        }
        if let Some(fragment) = token.strip_prefix(&prompt_words().eig_prefixes.1) {
            for value in concept_values_from_fragment(&prompt_words().eig_prefixes.1, fragment) {
                push_unique_preserving_normalized(&mut eig_r, &mut eig_r_seen, value);
            }
        }
    }

    (eig_n, eig_r)
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

fn parse_python_raw_regex_fragment(text: &str) -> Option<&str> {
    if let Some(rest) = text.strip_prefix("r\"") {
        return Some(rest.strip_suffix('"').unwrap_or(rest));
    }
    if let Some(rest) = text.strip_prefix("r'") {
        return Some(rest.strip_suffix('\'').unwrap_or(rest));
    }
    if let Some(rest) = text.strip_prefix("R\"") {
        return Some(rest.strip_suffix('"').unwrap_or(rest));
    }
    if let Some(rest) = text.strip_prefix("R'") {
        return Some(rest.strip_suffix('\'').unwrap_or(rest));
    }
    None
}

fn token_has_python_regex_or_glob(text: &str) -> bool {
    text.contains('*')
        || parse_python_raw_regex_fragment(text).is_some()
        || text.contains("r\"")
        || text.contains("r'")
        || text.contains("R\"")
        || text.contains("R'")
}

fn parse_special_fragment_matcher(text: &str) -> Option<SpecialFragmentMatcher> {
    let trimmed = text.trim();
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

fn strip_optional_negative_prefix<'a>(candidate: &'a str, negative_only: bool) -> Option<&'a str> {
    if !negative_only {
        return Some(candidate);
    }
    candidate.strip_prefix('-')
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
            &normalize_match_text(pattern),
            &normalize_match_text(candidate_body),
        ),
        SpecialFragmentMatcher::RegexLike { pattern, .. } => regex_like_search(
            &normalize_match_text(pattern),
            &normalize_match_text(candidate_body),
        ),
    }
}

fn glob_like_match(pattern: &str, text: &str) -> bool {
    if pattern.is_empty() {
        return text.is_empty();
    }

    let pattern_chars = pattern.chars().collect::<Vec<_>>();
    let text_chars = text.chars().collect::<Vec<_>>();
    let mut memo = BTreeMap::new();
    glob_like_match_chars(&pattern_chars, &text_chars, 0, 0, &mut memo)
}

fn glob_like_match_chars(
    pattern: &[char],
    text: &[char],
    pattern_index: usize,
    text_index: usize,
    memo: &mut BTreeMap<(usize, usize), bool>,
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

fn strip_regex_like_anchors(pattern: &str) -> (bool, bool, &str) {
    let start_anchor = pattern.starts_with('^');
    let without_start = pattern.strip_prefix('^').unwrap_or(pattern);
    let end_anchor = without_start.ends_with('$');
    let core = without_start.strip_suffix('$').unwrap_or(without_start);
    (start_anchor, end_anchor, core)
}

fn contains_regex_like_metacharacters(pattern: &str) -> bool {
    pattern
        .chars()
        .any(|ch| matches!(ch, '.' | '*' | '^' | '$'))
}

fn regex_like_search(pattern: &str, text: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }

    let (start_anchor, end_anchor, core) = strip_regex_like_anchors(pattern);
    if core.is_empty() {
        return true;
    }

    if !contains_regex_like_metacharacters(core) {
        return text.contains(core);
    }

    let core_chars = core.chars().collect::<Vec<_>>();
    let text_chars = text.chars().collect::<Vec<_>>();
    let starts = if start_anchor {
        vec![0usize]
    } else {
        (0..=text_chars.len()).collect::<Vec<_>>()
    };

    for start in starts {
        let end_range = if end_anchor {
            text_chars.len()..=text_chars.len()
        } else {
            start..=text_chars.len()
        };

        for end in end_range {
            let mut memo = BTreeMap::new();
            if regex_like_full_match_chars(&core_chars, &text_chars[start..end], 0, 0, &mut memo) {
                return true;
            }
        }
    }

    false
}

fn regex_like_full_match_chars(
    pattern: &[char],
    text: &[char],
    pattern_index: usize,
    text_index: usize,
    memo: &mut BTreeMap<(usize, usize), bool>,
) -> bool {
    if let Some(cached) = memo.get(&(pattern_index, text_index)) {
        return *cached;
    }

    let result = if pattern_index == pattern.len() {
        text_index == text.len()
    } else {
        let first_match = text_index < text.len()
            && (pattern[pattern_index] == '.' || pattern[pattern_index] == text[text_index]);

        if pattern_index + 1 < pattern.len() && pattern[pattern_index + 1] == '*' {
            regex_like_full_match_chars(pattern, text, pattern_index + 2, text_index, memo)
                || (first_match
                    && regex_like_full_match_chars(
                        pattern,
                        text,
                        pattern_index,
                        text_index + 1,
                        memo,
                    ))
        } else {
            first_match
                && regex_like_full_match_chars(
                    pattern,
                    text,
                    pattern_index + 1,
                    text_index + 1,
                    memo,
                )
        }
    };

    memo.insert((pattern_index, text_index), result);
    result
}

fn expand_prompt_regex_like_token(token: &str) -> Vec<String> {
    let Some(matcher) = parse_special_fragment_matcher(token) else {
        return Vec::new();
    };

    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for candidate in prompt_words()
        .befehle
        .iter()
        .filter(|candidate| candidate.len() > 1)
    {
        if special_fragment_matches_candidate(candidate, &matcher) {
            push_unique_preserving_normalized(&mut out, &mut seen, candidate.clone());
        }
    }
    out
}

fn expand_reta_simple_regex_like_token(token: &str, current_section: Option<&str>) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();

    let parameter_fragment = token.strip_prefix("--").unwrap_or(token);
    if let Some(matcher) = parse_special_fragment_matcher(parameter_fragment) {
        let inventory = current_section
            .map(reta_section_parameter_inventory_for_regex)
            .unwrap_or_else(reta_global_parameter_inventory_for_regex);
        for (parameter, values) in inventory {
            if values.is_empty() && special_fragment_matches_candidate(&parameter, &matcher) {
                push_unique_preserving_normalized(&mut out, &mut seen, format!("--{parameter}"));
            }
        }
    }

    let main_fragment = token.strip_prefix('-').unwrap_or(token);
    if let Some(matcher) = parse_special_fragment_matcher(main_fragment) {
        for switch in reta_main_switch_tokens_for_regex() {
            if special_fragment_matches_candidate(switch, &matcher)
                || special_fragment_matches_candidate(switch.trim_start_matches('-'), &matcher)
            {
                push_unique_preserving_normalized(&mut out, &mut seen, (*switch).to_string());
            }
        }
    }

    out
}

fn expand_rhs_regex_pieces(pieces: &[&str], allowed_values: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();

    for piece in pieces {
        let trimmed = piece.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(matcher) = parse_special_fragment_matcher(trimmed) {
            for value in allowed_values {
                if special_fragment_matches_candidate(value, &matcher)
                    || special_fragment_matches_candidate(&format!("={value}"), &matcher)
                {
                    push_unique_preserving_normalized(&mut out, &mut seen, value.clone());
                }
            }
        } else {
            push_unique_preserving_normalized(&mut out, &mut seen, trimmed.to_string());
        }
    }

    out
}

fn expand_reta_equals_regex_like_token(
    left: &str,
    right: &str,
    current_section: Option<&str>,
) -> Vec<String> {
    let left_core = left.trim().strip_prefix("--").unwrap_or(left.trim());
    let inventory = current_section
        .map(reta_section_parameter_inventory_for_regex)
        .unwrap_or_else(reta_global_parameter_inventory_for_regex);

    let parameter_names = if let Some(matcher) = parse_special_fragment_matcher(left_core) {
        inventory
            .keys()
            .filter(|parameter| {
                special_fragment_matches_candidate(parameter, &matcher)
                    || special_fragment_matches_candidate(&format!("--{parameter}"), &matcher)
            })
            .cloned()
            .collect::<Vec<_>>()
    } else if inventory.contains_key(left_core) {
        vec![left_core.to_string()]
    } else {
        Vec::new()
    };

    if parameter_names.is_empty() {
        return Vec::new();
    }

    let right_pieces = right.split(',').collect::<Vec<_>>();
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for parameter in parameter_names {
        let allowed_values = inventory.get(&parameter).cloned().unwrap_or_default();
        let values = expand_rhs_regex_pieces(&right_pieces, &allowed_values);
        if values.is_empty() {
            if allowed_values.is_empty() {
                push_unique_preserving_normalized(&mut out, &mut seen, format!("--{parameter}"));
            }
            continue;
        }
        push_unique_preserving_normalized(
            &mut out,
            &mut seen,
            format!("--{parameter}={}", values.join(",")),
        );
    }

    out
}

pub fn expand_python_regex_like_tokens(tokens: &[String]) -> Vec<String> {
    if !tokens
        .iter()
        .any(|token| token_has_python_regex_or_glob(token))
    {
        return tokens.to_vec();
    }

    let input_is_reta = matches!(tokens.first().map(String::as_str), Some("reta"));
    let mut current_section: Option<&str> = None;
    let mut out = Vec::new();

    for token in tokens {
        if token == "reta" {
            out.push(token.clone());
            continue;
        }
        if input_is_reta && is_main_switch_token(token) {
            current_section = Some(token.as_str());
            out.push(token.clone());
            continue;
        }

        let expanded = if let Some((left, right)) = token.split_once('=') {
            if input_is_reta || left.starts_with("--") {
                expand_reta_equals_regex_like_token(left, right, current_section)
            } else {
                Vec::new()
            }
        } else if input_is_reta {
            expand_reta_simple_regex_like_token(token, current_section)
        } else {
            expand_prompt_regex_like_token(token)
        };

        if expanded.is_empty() {
            out.push(token.clone());
        } else {
            out.extend(expanded);
        }
    }

    out
}

pub fn replace_prompt_alias(token: &str) -> String {
    match token {
        "e" => "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar".to_string(),
        "G" => "geist".to_string(),
        "R" => "range".to_string(),
        "a" => "absicht".to_string(),
        "B" => "bewusstsein".to_string(),
        "E" => "emotion".to_string(),
        "u" => "universum".to_string(),
        "I" => "impulse".to_string(),
        "T" => "triebe".to_string(),
        "t" => "thomas".to_string(),
        "r" => "richtung".to_string(),
        "v" => "vielfache".to_string(),
        "h" => "help".to_string(),
        "w" => "teiler".to_string(),
        "S" => "BefehlSpeichernDanach".to_string(),
        "s" => "BefehlSpeichernDavor".to_string(),
        "l" => "BefehlSpeicherungLöschen".to_string(),
        "o" => "BefehlSpeicherungAusgeben".to_string(),
        "W" => "wirklichkeit".to_string(),
        _ => token.to_string(),
    }
}

pub fn normalize_prompt_tokens(tokens: &[String]) -> Vec<String> {
    tokens
        .iter()
        .map(|token| replace_prompt_alias(token))
        .collect()
}

pub fn expand_python_prompt_macros(tokens: &[String]) -> Vec<String> {
    let mut out = normalize_prompt_tokens(tokens);
    let has_mulpri = out.iter().any(|t| t == "mulpri" || t == "p");
    if has_mulpri {
        for extra in ["multis", "prim", "primfaktorenvergleich"] {
            if !out.iter().any(|t| t == extra) {
                out.push(extra.to_string());
            }
        }
    }
    out
}

pub fn finalize_prompt_tokens_for_execution(tokens: &[String]) -> Vec<String> {
    let normalized = expand_python_prompt_macros(tokens);
    expand_python_regex_like_tokens(&normalized)
}

pub fn is_15or16_command(text: &str) -> bool {
    if let Some(rest) = text.strip_prefix("15_") {
        return rest.is_empty() || prompt_words().befehle_set.contains(text);
    }
    if let Some(rest) = text.strip_prefix("16_") {
        if rest.is_empty() || prompt_words().befehle_set.contains(text) {
            return true;
        }
        if let Some(rest15) = text.strip_prefix("16_15_") {
            return rest15.is_empty() || prompt_words().befehle_set.contains(text);
        }
    }
    false
}

pub fn custom_split_whitespace_parenthesized(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;

    for ch in text.chars() {
        match ch {
            '(' => {
                round += 1;
                current.push(ch);
            }
            ')' => {
                round -= 1;
                current.push(ch);
            }
            '[' => {
                square += 1;
                current.push(ch);
            }
            ']' => {
                square -= 1;
                current.push(ch);
            }
            '{' => {
                curly += 1;
                current.push(ch);
            }
            '}' => {
                curly -= 1;
                current.push(ch);
            }
            c if c.is_whitespace() && round == 0 && square == 0 && curly == 0 => {
                if !current.trim().is_empty() {
                    out.push(current.trim().to_string());
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }

    if !current.trim().is_empty() {
        out.push(current.trim().to_string());
    }
    out
}

pub fn custom_split_delim_parenthesized(text: &str, delim: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;

    for ch in text.chars() {
        match ch {
            '(' => {
                round += 1;
                current.push(ch);
            }
            ')' => {
                round -= 1;
                current.push(ch);
            }
            '[' => {
                square += 1;
                current.push(ch);
            }
            ']' => {
                square -= 1;
                current.push(ch);
            }
            '{' => {
                curly += 1;
                current.push(ch);
            }
            '}' => {
                curly -= 1;
                current.push(ch);
            }
            c if c == delim && round == 0 && square == 0 && curly == 0 => {
                out.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }

    if !current.trim().is_empty() {
        out.push(current.trim().to_string());
    }
    out
}

pub fn looks_like_single_numeric_or_fraction_part(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return false;
    }
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        return looks_like_numeric_or_fraction_range(&trimmed[1..trimmed.len() - 1]);
    }
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        return looks_like_numeric_or_fraction_range(&trimmed[1..trimmed.len() - 1]);
    }
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        return looks_like_numeric_or_fraction_range(&trimmed[1..trimmed.len() - 1]);
    }
    if trimmed.contains(',') {
        return custom_split_delim_parenthesized(trimmed, ',')
            .into_iter()
            .all(|piece| looks_like_single_numeric_or_fraction_part(&piece));
    }
    if trimmed.contains('+') {
        return trimmed
            .split('+')
            .all(|piece| looks_like_single_numeric_or_fraction_part(piece));
    }
    if let Some((left, right)) = trimmed.split_once('-') {
        if left.is_empty() {
            return is_integer_or_fraction(right);
        }
        return is_integer_or_fraction(left) && is_integer_or_fraction(right);
    }
    is_integer_or_fraction(trimmed)
}

pub fn looks_like_numeric_or_fraction_range(text: &str) -> bool {
    looks_like_single_numeric_or_fraction_part(text)
}

pub fn is_row_spec_token(text: &str) -> bool {
    looks_like_numeric_or_fraction_range(text)
}

fn is_integer_or_fraction(text: &str) -> bool {
    let s = text.trim();
    if s.is_empty() {
        return false;
    }
    if let Some(rest) = s.strip_prefix('v') {
        return is_integer_or_fraction(rest);
    }
    if let Some((a, b)) = s.split_once('/') {
        return is_signed_integer(a) && is_signed_integer(b);
    }
    is_signed_integer(s)
}

fn is_signed_integer(text: &str) -> bool {
    let s = text.trim();
    let body = s.strip_prefix('-').unwrap_or(s);
    !body.is_empty() && body.chars().all(|c| c.is_ascii_digit())
}

pub fn expand_kurz_kurz_befehl(prompt_mode: PromptModus, tokens: &[String]) -> (bool, Vec<String>) {
    if tokens.is_empty() {
        return (false, Vec::new());
    }

    let xtext = tokens.join(" ");
    let stext2 = custom_split_whitespace_parenthesized(&xtext);
    let mut stext3: Vec<String> = Vec::new();
    let mut if_kurz_kurz = false;
    let words = prompt_words();

    for original in &stext2 {
        let s = original.trim_matches(',').to_string();
        let original_s = s.clone();
        let mut text_dazu: Vec<String> = Vec::new();

        let first_token_is_reta = tokens.first().map(|s| s == "reta").unwrap_or(false);
        let known_direct =
            is_15or16_command(&s) || words.befehle_set.contains(&s) || first_token_is_reta;

        if !known_direct {
            let parsed = parse_prefix_and_numeric_suffix(&s);
            if let Some((prefix, numeric)) = parsed {
                if looks_like_numeric_or_fraction_range(&numeric) {
                    let buchst = prefix
                        .chars()
                        .map(|c| c.to_string())
                        .filter(|c| words.one_char_commands.contains(c))
                        .collect::<Vec<_>>();
                    let set_text_len_is_1 = tokens
                        .iter()
                        .filter(|t| {
                            *t != "e"
                                && *t != "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
                        })
                        .count()
                        == 1;

                    if buchst.len() == prefix.chars().count() && !buchst.is_empty() {
                        if_kurz_kurz = true;
                        let buchst2 = buchst
                            .into_iter()
                            .map(|a| if a == "p" { "mulpri".to_string() } else { a })
                            .collect::<Vec<_>>();
                        text_dazu.extend(buchst2);
                        text_dazu.push(numeric.clone());
                    } else if set_text_len_is_1 && prompt_mode != PromptModus::AusgabeSelektiv {
                        if_kurz_kurz = true;
                        text_dazu.extend(
                            [
                                "mulpri",
                                "a",
                                "t",
                                "w",
                                "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar",
                            ]
                            .into_iter()
                            .map(|s| s.to_string()),
                        );
                        if tokens.iter().any(|t| t.contains('/')) {
                            text_dazu.extend(
                                ["u", "B", "G", "E", "groesse"]
                                    .into_iter()
                                    .map(|s| s.to_string()),
                            );
                        }
                    }
                }
            }
        } else if s == "ee" {
            text_dazu.push("-ausgabe".to_string());
            text_dazu.push("--keineueberschriften".to_string());
        } else {
            text_dazu.push(s.clone());
        }

        if text_dazu.is_empty() {
            stext3.push(original_s);
        } else {
            stext3.extend(text_dazu);
        }
    }

    for entry in &mut stext3 {
        if entry.starts_with('(') && entry.ends_with(')') && entry.len() >= 2 {
            let inner = &entry[1..entry.len() - 1];
            *entry = format!("[{inner}]");
        }
    }

    if matches!(
        tokens.first().map(|s| s.as_str()),
        Some("reta" | "shell" | "python")
    ) {
        (if_kurz_kurz, tokens.to_vec())
    } else {
        (if_kurz_kurz, stext3)
    }
}

struct PromptSemanticSpec {
    names: &'static [&'static str],
    integer_para: &'static str,
    fraction_para: Option<&'static str>,
    integer_cols: &'static str,
    fraction_cols: &'static str,
}

fn semantic_specs() -> &'static [PromptSemanticSpec] {
    static SPECS: OnceLock<Vec<PromptSemanticSpec>> = OnceLock::new();
    SPECS
        .get_or_init(|| {
            vec![
                PromptSemanticSpec {
                    names: &["thomas"],
                    integer_para: "--galaxie=thomas",
                    fraction_para: None,
                    integer_cols: "2",
                    fraction_cols: "2",
                },
                PromptSemanticSpec {
                    names: &["emotion"],
                    integer_para: "--grundstrukturen=emotion",
                    fraction_para: None,
                    integer_cols: "2,3",
                    fraction_cols: "4,5",
                },
                PromptSemanticSpec {
                    names: &["wirklichkeit"],
                    integer_para: "--grundstrukturen=wirklichkeit",
                    fraction_para: None,
                    integer_cols: "1,2",
                    fraction_cols: "5",
                },
                PromptSemanticSpec {
                    names: &["triebe"],
                    integer_para: "--grundstrukturen=triebe",
                    fraction_para: None,
                    integer_cols: "1",
                    fraction_cols: "2",
                },
                PromptSemanticSpec {
                    names: &["impulse"],
                    integer_para: "--grundstrukturen=impulse",
                    fraction_para: None,
                    integer_cols: "1,4",
                    fraction_cols: "3",
                },
                PromptSemanticSpec {
                    names: &["bewusstsein"],
                    integer_para: "--grundstrukturen=bewusstsein",
                    fraction_para: None,
                    integer_cols: "6",
                    fraction_cols: "7",
                },
                PromptSemanticSpec {
                    names: &["geist"],
                    integer_para: "--grundstrukturen=geist",
                    fraction_para: None,
                    integer_cols: "3",
                    fraction_cols: "4",
                },
                PromptSemanticSpec {
                    names: &["freiheit", "gleichheit"],
                    integer_para: "--planet=freiheit",
                    fraction_para: None,
                    integer_cols: "1-4,8",
                    fraction_cols: "5-7",
                },
                PromptSemanticSpec {
                    names: &["groesse"],
                    integer_para: "--strukturgroesse=organisation",
                    fraction_para: None,
                    integer_cols: "1-3",
                    fraction_cols: "99",
                },
                PromptSemanticSpec {
                    names: &["kugeln", "kreise"],
                    integer_para: "--universum=kugeln",
                    fraction_para: None,
                    integer_cols: "1-2",
                    fraction_cols: "99",
                },
                PromptSemanticSpec {
                    names: &["netzwerk"],
                    integer_para: "--universum=netzwerk",
                    fraction_para: None,
                    integer_cols: "1-3",
                    fraction_cols: "99",
                },
                PromptSemanticSpec {
                    names: &["komplex"],
                    integer_para: "--universum=komplex",
                    fraction_para: None,
                    integer_cols: "1",
                    fraction_cols: "3",
                },
                PromptSemanticSpec {
                    names: &["absicht", "absichten", "motiv", "motive"],
                    integer_para: "--menschliches=motivation",
                    fraction_para: None,
                    integer_cols: "1",
                    fraction_cols: "3",
                },
                PromptSemanticSpec {
                    names: &["universum"],
                    integer_para: "--universum=transzendentalien",
                    fraction_para: Some("--universum=transzendentaliereziproke"),
                    integer_cols: "1,4",
                    fraction_cols: "1,2",
                },
                PromptSemanticSpec {
                    names: &["richtung"],
                    integer_para: "--primzahlwirkung=galaxieabsicht",
                    fraction_para: None,
                    integer_cols: "1",
                    fraction_cols: "1",
                },
            ]
        })
        .as_slice()
}

fn semantic_wahl15() -> &'static BTreeMap<&'static str, &'static str> {
    static MAP: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| {
        BTreeMap::from([
            ("15", "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity,Biologischer_Baum_(15),Teilchen_anderes_Universum,pro_contra"),
            ("2", "Konkreta_und_Focus_(2)"),
            ("5", "Impulse_(5)"),
            ("7", "Gefühle_(7),Anführer_Arten_(7),Erlösung"),
            ("8", "Modus_und_Sein_(8),Bestrafung,Gewalt"),
            ("10", "Wirklichkeiten_Wahrheit_Wahrnehmung_(10)"),
            ("12", "Meta-Systeme_(12),Ordnung_und_Filterung_12_und_1pro12"),
            ("13", "Paradigmen_sind_Absichten_(13)"),
            ("17", "Gedanken_sind_Positionen_(17)"),
            ("18", "Verbundenheiten_(18)"),
            ("6", "Triebe_und_Bedürfnisse_(6),System"),
            ("9", "Lust_(9)"),
            ("3", "Reflexe_(3),Existenzialien_(3)"),
            ("16", "Funktionen_Vorstellungen_(16)"),
            ("4", "Achtung_(4)"),
            ("1", "Bewusstheit_statt_Bewusstsein_(1)"),
            ("30", "Energie_und_universelle_Eigenschaften_(30)"),
            ("14", "Stimmungen_Kombinationen_(14)"),
            ("20", "Klassen_(20)"),
            ("37", "Empathie_(37)"),
            ("31", "Garben_und_Verhalten_nachfühlen(31)"),
            ("11", "Verhalten_(11)"),
            ("36", "Attraktionen_(36)"),
            ("21", "Leidenschaften_(21)"),
            ("26", "Erwartungshaltungen_(26)"),
            ("19", "Extremalien_(19),Ziele_(19)"),
            ("90", "abhängige_Verbundenheit_(90)"),
        ])
    })
}

fn semantic_wahl16() -> &'static BTreeMap<&'static str, &'static str> {
    static MAP: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| {
        BTreeMap::from([
            ("1", "Meta-Physik-Teilchen_(1)"),
            ("2", "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Model_of_Hierarchical_Complexity"),
            ("3", "Teilchen_anderes_Universum"),
            ("5", "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Model_of_Hierarchical_Complexity,Biologischer_Baum_(16_->_5),P5"),
            ("6", "Geist_(15)"),
            ("10", "Struktur-Wissenschaften_(10)"),
            ("15", "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Model_of_Hierarchical_Complexity"),
            ("16", "Meta-Physik-Teilchen_(1),Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Model_of_Hierarchical_Complexity,Teilchen_anderes_Universum,Biologischer_Baum_(16_->_5),P5,Geist_(15),Struktur-Wissenschaften_(10),Muster-Wissenschaften_(20)"),
            ("20", "Muster-Wissenschaften_(20)"),
        ])
    })
}

fn contains_blocking_abc(tokens: &[String]) -> bool {
    tokens.iter().any(|t| t == "abc" || t == "abcd")
}

#[derive(Clone, Debug, Default)]
struct PythonRowBuckets {
    primary_row_specs: Vec<String>,
    reciprocal_row_specs: Vec<String>,
    raw_fraction_specs: Vec<String>,
}

fn push_unique_string(target: &mut Vec<String>, value: String) {
    if !target.contains(&value) {
        target.push(value);
    }
}

fn parse_simple_fraction_piece(piece: &str) -> Option<(i64, i64)> {
    let trimmed = piece.trim();
    let trimmed = trimmed.strip_prefix('v').unwrap_or(trimmed);
    let (left, right) = trimmed.split_once('/')?;
    let numerator = left.trim().parse::<i64>().ok()?;
    let denominator = right.trim().parse::<i64>().ok()?;
    if denominator == 0 {
        return None;
    }
    Some((numerator, denominator))
}

fn build_python_row_buckets(row_specs: &[String]) -> PythonRowBuckets {
    let mut buckets = PythonRowBuckets::default();

    for spec in row_specs {
        let trimmed = spec.trim();
        if trimmed.is_empty() {
            continue;
        }

        if !trimmed.contains('/') {
            push_unique_string(&mut buckets.primary_row_specs, trimmed.to_string());
            continue;
        }

        push_unique_string(&mut buckets.raw_fraction_specs, trimmed.to_string());

        for piece in custom_split_delim_parenthesized(trimmed, ',') {
            if let Some((numerator, denominator)) = parse_simple_fraction_piece(&piece) {
                if numerator == 0 {
                    continue;
                }
                let numerator_abs = numerator.abs();
                let denominator_abs = denominator.abs();
                if numerator_abs == 0 || denominator_abs == 0 {
                    continue;
                }
                if numerator_abs % denominator_abs == 0 {
                    push_unique_string(
                        &mut buckets.primary_row_specs,
                        (numerator_abs / denominator_abs).to_string(),
                    );
                }
                if denominator_abs % numerator_abs == 0 {
                    push_unique_string(
                        &mut buckets.reciprocal_row_specs,
                        (denominator_abs / numerator_abs).to_string(),
                    );
                }
            }
        }
    }

    buckets
}

fn prompt_python_default_oberesmaximum_seed() -> i64 {
    // Python retaPrompt liest hier letztlich `tables.hoechsteZeile[1024]` aus dem
    // laufenden Programm oder faellt auf das globale `retaProgram` zurueck.
    // Auf dem split prompt crate existiert diese Program-Template-Schicht nicht,
    // daher verwenden wir hier den gleichen Python-Defaultwert direkt.
    1024
}

fn another_oberesmaximum_from_row_specs(row_specs: &[String]) -> Option<String> {
    let numbers = parse_row_spec_numbers(row_specs)?;
    if numbers.is_empty() {
        return None;
    }
    let max_row = numbers.into_iter().map(i64::abs).max()?;
    let hoechste_zeile = prompt_python_default_oberesmaximum_seed();
    Some(format!(
        "--oberesmaximum={}",
        std::cmp::max(max_row, hoechste_zeile) + 1
    ))
}

#[derive(Clone, Debug, Default)]
struct BuiltRowSection {
    tokens: Vec<String>,
}

fn build_python_row_section(
    row_specs: &[String],
    use_range: bool,
    use_teiler: bool,
    use_vielfache: bool,
    invert: bool,
) -> Option<BuiltRowSection> {
    if row_specs.is_empty() {
        return None;
    }

    let base_specs = if use_teiler {
        divisors_from_row_specs(row_specs)?
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
    } else {
        row_specs.to_vec()
    };

    if base_specs.is_empty() {
        return None;
    }

    let mut tokens = Vec::new();

    if use_vielfache {
        if !use_teiler {
            tokens.push(format!("--vielfachevonzahlen={}", base_specs.join(",")));
        }
        let prefix = if use_range {
            "--zaehlung="
        } else {
            "--vorhervonausschnitt="
        };
        let mut suffix_parts = row_specs
            .iter()
            .filter_map(|spec| {
                let trimmed = spec.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(format!("v{trimmed}"))
                }
            })
            .collect::<Vec<_>>();
        let mut value_parts = base_specs.clone();
        if use_teiler {
            value_parts.extend(row_specs.iter().cloned());
        }
        value_parts.append(&mut suffix_parts);
        tokens.push(format!("{prefix}{}", value_parts.join(",")));
    } else {
        let prefix = if use_range {
            "--zaehlung="
        } else {
            "--vorhervonausschnitt="
        };
        tokens.push(format!("{prefix}{}", base_specs.join(",")));
        if let Some(oberesmaximum) = another_oberesmaximum_from_row_specs(&base_specs) {
            tokens.push(oberesmaximum);
        }
    }

    if invert {
        tokens.push("--invertieren".to_string());
    }

    Some(BuiltRowSection { tokens })
}

fn build_trailing_primary_zeilen_tokens(
    primary_row_specs: &[String],
    use_range: bool,
    use_teiler: bool,
    use_vielfache: bool,
) -> Vec<String> {
    build_python_row_section(
        primary_row_specs,
        use_range,
        use_teiler,
        use_vielfache,
        false,
    )
    .map(|section| {
        let mut tokens = vec!["-zeilen".to_string()];
        tokens.extend(section.tokens);
        tokens
    })
    .unwrap_or_default()
}

fn is_known_reta_parameter_token(token: &str) -> bool {
    if !token.starts_with("--") {
        return false;
    }
    let base = token
        .split_once('=')
        .map(|(head, _)| head)
        .unwrap_or(token)
        .trim_start_matches("--");
    reta_global_parameter_inventory_for_regex().contains_key(base)
}

fn is_spalten_parameter_token(token: &str) -> bool {
    if !token.starts_with("--") {
        return false;
    }
    let base = token
        .split_once('=')
        .map(|(head, _)| head)
        .unwrap_or(token)
        .trim_start_matches("--");
    spalten_parameter_inventory_for_regex().contains_key(base)
}

fn is_ausgabe_parameter_token(token: &str) -> bool {
    if !token.starts_with("--") {
        return false;
    }
    let base = token
        .split_once('=')
        .map(|(head, _)| head)
        .unwrap_or(token)
        .trim_start_matches("--");
    ausgabe_parameter_inventory_for_regex().contains_key(base)
}

fn extract_passthrough_reta_parameters(tokens: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for token in tokens {
        if is_known_reta_parameter_token(token) {
            push_unique_string(&mut out, token.clone());
        }
    }
    out
}

fn build_general_semantic_call(
    row_specs: &[String],
    use_range: bool,
    invert: bool,
    use_teiler: bool,
    use_vielfache: bool,
    suppress_empty: bool,
    no_headers: bool,
    para: &str,
    cols: Option<&str>,
    extra_params: &[String],
    trailing_tokens: &[String],
) -> Vec<String> {
    let mut argv = vec!["reta".to_string(), "-zeilen".to_string()];
    if let Some(section) =
        build_python_row_section(row_specs, use_range, use_teiler, use_vielfache, invert)
    {
        argv.extend(section.tokens);
    }
    argv.push("-spalten".to_string());
    argv.push(para.to_string());
    argv.push("--breite=0".to_string());
    argv.push("-ausgabe".to_string());
    if let Some(cols) = cols {
        argv.push(format!("--spaltenreihenfolgeundnurdiese={cols}"));
    }
    if suppress_empty {
        argv.push("--keineleereninhalte".to_string());
    }
    if no_headers {
        argv.push("--keineueberschriften".to_string());
    }
    for token in extra_params {
        if !argv.contains(token) {
            argv.push(token.clone());
        }
    }
    for token in trailing_tokens {
        argv.push(token.clone());
    }
    argv
}

fn build_single_semantic_call(
    spec: &PromptSemanticSpec,
    row_specs: &[String],
    reciprocal_kind: bool,
    use_range: bool,
    invert: bool,
    teiler: bool,
    vielfache: bool,
    suppress_empty: bool,
    no_headers: bool,
    extra_params: &[String],
    trailing_tokens: &[String],
) -> Option<Vec<String>> {
    if row_specs.is_empty() {
        return None;
    }

    let para = if reciprocal_kind {
        spec.fraction_para.unwrap_or(spec.integer_para)
    } else {
        spec.integer_para
    };
    let cols = if reciprocal_kind {
        spec.fraction_cols
    } else {
        spec.integer_cols
    };

    Some(build_general_semantic_call(
        row_specs,
        use_range,
        invert,
        teiler,
        vielfache,
        suppress_empty,
        no_headers,
        para,
        Some(cols),
        extra_params,
        trailing_tokens,
    ))
}

fn append_15_16_calls(
    calls: &mut Vec<Vec<String>>,
    normalized: &[String],
    row_specs: &[String],
    use_range: bool,
    invert: bool,
    teiler: bool,
    use_vielfache: bool,
    suppress_empty: bool,
    no_headers: bool,
    extra_params: &[String],
) {
    let mut values16: Vec<String> = Vec::new();
    let mut values15: Vec<String> = Vec::new();

    for token in normalized {
        if let Some(suffix) = token.strip_prefix("16_") {
            if !token.starts_with("16_15") {
                if let Some(value) = semantic_wahl16().get(suffix) {
                    values16.push((*value).to_string());
                }
            }
        }
        if token == "16_15" {
            if let Some(value) = semantic_wahl15().get("15") {
                values15.push((*value).to_string());
            }
            continue;
        }
        if let Some(suffix) = token.strip_prefix("16_15_") {
            if let Some(value) = semantic_wahl15().get(suffix) {
                values15.push((*value).to_string());
            }
            continue;
        }
        if let Some(suffix) = token.strip_prefix("15_") {
            if let Some(value) = semantic_wahl15().get(suffix) {
                values15.push((*value).to_string());
            }
        }
    }

    if !values16.is_empty() {
        calls.push(build_general_semantic_call(
            row_specs,
            use_range,
            invert,
            teiler,
            use_vielfache,
            suppress_empty,
            no_headers,
            &format!("--multiversum={}", values16.join(",")),
            None,
            extra_params,
            &[],
        ));
    }
    if !values15.is_empty() {
        calls.push(build_general_semantic_call(
            row_specs,
            use_range,
            invert,
            teiler,
            use_vielfache,
            suppress_empty,
            no_headers,
            &format!("--grundstrukturen={}", values15.join(",")),
            None,
            extra_params,
            &[],
        ));
    }
}

pub fn build_reta_calls_from_prompt_tokens(tokens: &[String]) -> Vec<Vec<String>> {
    let normalized = finalize_prompt_tokens_for_execution(tokens);
    if normalized.is_empty() || normalized[0] == "reta" || normalized[0].starts_with('-') {
        return Vec::new();
    }
    if normalized.iter().any(|t| {
        matches!(
            t.as_str(),
            "help"
                | "hilfe"
                | "befehle"
                | "kurzbefehle"
                | "shell"
                | "python"
                | "math"
                | "loggen"
                | "nichtloggen"
        )
    }) {
        return Vec::new();
    }

    let row_specs = normalized
        .iter()
        .filter(|token| is_row_spec_token(token))
        .cloned()
        .collect::<Vec<_>>();
    if row_specs.is_empty() {
        return Vec::new();
    }

    let suppress_empty = normalized
        .iter()
        .any(|t| t == "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar");
    let no_headers = normalized
        .iter()
        .any(|t| t == "ee" || t == "--keineueberschriften");
    let use_range = normalized.iter().any(|t| t == "range");
    let invert = normalized.iter().any(|t| t == "invertieren");
    let teiler = normalized.iter().any(|t| t == "teiler");
    let vielfache = normalized.iter().any(|t| t == "vielfache");
    let row_buckets = build_python_row_buckets(&row_specs);
    let extra_params = extract_passthrough_reta_parameters(&normalized);
    let mut calls: Vec<Vec<String>> = Vec::new();
    let mut seen_labels = BTreeSet::new();

    for token in &normalized {
        for spec in semantic_specs() {
            if spec.names.contains(&token.as_str()) {
                let label = spec.names[0].to_string();
                if seen_labels.insert(label) {
                    if let Some(call) = build_single_semantic_call(
                        spec,
                        &row_buckets.primary_row_specs,
                        false,
                        use_range,
                        invert,
                        teiler,
                        vielfache,
                        suppress_empty,
                        no_headers,
                        &extra_params,
                        &[],
                    ) {
                        calls.push(call);
                    }
                    if let Some(call) = build_single_semantic_call(
                        spec,
                        &row_buckets.reciprocal_row_specs,
                        true,
                        use_range,
                        invert,
                        teiler,
                        vielfache,
                        suppress_empty,
                        no_headers,
                        &extra_params,
                        &[],
                    ) {
                        calls.push(call);
                    }
                }
                break;
            }
        }
    }

    let (eig_n_values, eig_r_values) = collect_concept_prefixed_values(&normalized);
    if !eig_n_values.is_empty() && !row_buckets.primary_row_specs.is_empty() {
        calls.push(build_general_semantic_call(
            &row_buckets.primary_row_specs,
            use_range,
            invert,
            teiler,
            vielfache,
            suppress_empty,
            no_headers,
            &format!("--konzept={}", eig_n_values.join(",")),
            None,
            &extra_params,
            &[],
        ));
    }
    if !eig_r_values.is_empty() && !row_buckets.reciprocal_row_specs.is_empty() {
        let trailing = build_trailing_primary_zeilen_tokens(
            &row_buckets.primary_row_specs,
            use_range,
            teiler,
            vielfache,
        );
        calls.push(build_general_semantic_call(
            &row_buckets.reciprocal_row_specs,
            use_range,
            invert,
            teiler,
            vielfache,
            suppress_empty,
            no_headers,
            &format!("--konzept2={}", eig_r_values.join(",")),
            None,
            &extra_params,
            &trailing,
        ));
    }

    if !contains_blocking_abc(&normalized) {
        let rows_for_15_16 = if !row_buckets.primary_row_specs.is_empty() {
            row_buckets.primary_row_specs.as_slice()
        } else {
            row_specs.as_slice()
        };
        append_15_16_calls(
            &mut calls,
            &normalized,
            rows_for_15_16,
            use_range,
            invert,
            teiler,
            vielfache,
            suppress_empty,
            no_headers,
            &extra_params,
        );
    }

    calls
}

pub fn build_reta_argv_from_prompt_tokens(tokens: &[String]) -> Option<Vec<String>> {
    let semantic_calls = build_reta_calls_from_prompt_tokens(tokens);
    if semantic_calls.len() == 1 {
        return semantic_calls.into_iter().next();
    }

    let normalized = finalize_prompt_tokens_for_execution(tokens);
    if normalized.is_empty() {
        return None;
    }
    if normalized[0] == "reta" || normalized[0].starts_with('-') {
        return None;
    }
    if normalized.iter().any(|t| {
        matches!(
            t.as_str(),
            "help"
                | "hilfe"
                | "befehle"
                | "kurzbefehle"
                | "shell"
                | "python"
                | "math"
                | "loggen"
                | "nichtloggen"
        )
    }) {
        return None;
    }

    let mut row_specs: Vec<String> = Vec::new();
    let mut output_commands: Vec<String> = Vec::new();
    let mut output_flags: Vec<String> = Vec::new();

    for token in &normalized {
        if is_row_spec_token(token) {
            row_specs.push(token.clone());
            continue;
        }
        match token.as_str() {
            "vielfache"
            | "einzeln"
            | "teiler"
            | "invertieren"
            | "range"
            | "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar" => {}
            "ee" => push_unique_string(&mut output_flags, "--keineueberschriften".to_string()),
            "absicht" | "motiv" | "motive" | "absichten" | "universum" | "thomas" | "geist"
            | "bewusstsein" | "emotion" | "impulse" | "wirklichkeit" | "groesse" | "komplex"
            | "kugeln" | "kreise" | "freiheit" | "gleichheit" | "richtung" | "mond" | "alles"
            | "primzahlkreuz" => push_unique_string(&mut output_commands, token.clone()),
            other
                if is_15or16_command(other)
                    || other.starts_with(&prompt_words().eig_prefixes.0)
                    || other.starts_with(&prompt_words().eig_prefixes.1) =>
            {
                push_unique_string(&mut output_commands, other.to_string());
            }
            _ => {}
        }
    }

    if row_specs.is_empty() {
        return None;
    }

    let passthrough_params = extract_passthrough_reta_parameters(&normalized);
    let has_explicit_spalten_parameter = passthrough_params
        .iter()
        .any(|token| is_spalten_parameter_token(token));
    let has_explicit_ausgabe_parameter = passthrough_params
        .iter()
        .any(|token| is_ausgabe_parameter_token(token));

    if output_commands.is_empty() && !has_explicit_spalten_parameter {
        output_commands.extend(["absicht".to_string(), "thomas".to_string()]);
        if row_specs.iter().any(|t| t.contains('/')) {
            output_commands.extend([
                "universum".to_string(),
                "bewusstsein".to_string(),
                "geist".to_string(),
                "emotion".to_string(),
                "groesse".to_string(),
            ]);
        }
    }

    let use_range = normalized.iter().any(|t| t == "range");
    let invert = normalized.iter().any(|t| t == "invertieren");
    let teiler = normalized.iter().any(|t| t == "teiler");
    let vielfache = normalized.iter().any(|t| t == "vielfache");
    let row_buckets = build_python_row_buckets(&row_specs);
    let generic_rows = if !row_buckets.primary_row_specs.is_empty() {
        row_buckets.primary_row_specs.clone()
    } else {
        row_specs.clone()
    };
    let row_section =
        build_python_row_section(&generic_rows, use_range, teiler, vielfache, invert)?;

    let mut argv = vec!["reta".to_string(), "-zeilen".to_string()];
    argv.extend(row_section.tokens);

    if !output_commands.is_empty() || has_explicit_spalten_parameter {
        argv.push("-spalten".to_string());
        for command in output_commands {
            let token = format!("--{command}");
            if !argv.contains(&token) {
                argv.push(token);
            }
        }
        for token in &passthrough_params {
            if is_spalten_parameter_token(token) && !argv.contains(token) {
                argv.push(token.clone());
            }
        }
    }

    if !output_flags.is_empty() || has_explicit_ausgabe_parameter {
        argv.push("-ausgabe".to_string());
        for flag in output_flags {
            if !argv.contains(&flag) {
                argv.push(flag);
            }
        }
        for token in &passthrough_params {
            if is_ausgabe_parameter_token(token) && !argv.contains(token) {
                argv.push(token.clone());
            }
        }
    }

    for token in passthrough_params {
        if !argv.contains(&token)
            && !is_spalten_parameter_token(&token)
            && !is_ausgabe_parameter_token(&token)
        {
            argv.push(token);
        }
    }

    Some(argv)
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PreparedPromptBigOutput {
    pub tokens: Vec<String>,
    pub row_specs: Vec<String>,
    pub had_kurz_kurz: bool,
}

/// Python-Architekturanker fuer den gespeicherten `reta`-Platzhalter-Pfad.
///
/// Das bildet den fachlich wichtigsten Teil von
/// `verdreheWoReTaBefehl()` + `promptVorbereitungGrosseAusgabe()` nach:
/// Wenn ein gespeicherter Platzhalter bereits mit `reta` beginnt und die
/// neue Eingabe nur aus Zeilenbereichen plus prompt-typischen Modifikatoren
/// besteht, wird die alte `-zeilen`-Sektion ersetzt statt die neuen Tokens
/// blind anzuhängen.
pub fn prepare_prompt_big_output_for_stored_reta(
    stored_prefix_tokens: &[String],
    input_tokens: &[String],
) -> Option<PreparedPromptBigOutput> {
    if stored_prefix_tokens.first().map(String::as_str) != Some("reta") || input_tokens.is_empty() {
        return None;
    }

    let (had_kurz_kurz, expanded_input) =
        expand_kurz_kurz_befehl(PromptModus::AusgabeSelektiv, input_tokens);
    let mut effective_input = if expanded_input.is_empty() {
        input_tokens.to_vec()
    } else {
        expanded_input
    };
    effective_input = finalize_prompt_tokens_for_execution(&effective_input);

    let row_specs = effective_input
        .iter()
        .filter(|token| is_row_spec_token(token))
        .cloned()
        .collect::<Vec<_>>();
    if row_specs.is_empty() {
        return None;
    }

    if effective_input
        .iter()
        .any(|token| !is_row_spec_token(token) && !is_selective_reta_modifier(token))
    {
        return None;
    }

    let use_range = effective_input.iter().any(|token| token == "range");
    let use_teiler = effective_input.iter().any(|token| token == "teiler");
    let use_vielfache = effective_input.iter().any(|token| token == "vielfache");
    let use_invertieren = effective_input
        .iter()
        .any(|token| matches!(token.as_str(), "invertieren" | "--invertieren"));
    let request_no_headers = effective_input
        .iter()
        .any(|token| matches!(token.as_str(), "--keineueberschriften" | "ee"));

    let mut joined_rows = row_specs.join(",");
    if use_teiler {
        let divisors = divisors_from_row_specs(&row_specs)?;
        joined_rows = divisors
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",");
    }

    let row_argument = if use_vielfache {
        format!("--vielfachevonzahlen={joined_rows}")
    } else if use_range {
        format!("--zaehlung={joined_rows}")
    } else {
        format!("--vorhervonausschnitt={joined_rows}")
    };

    let mut new_zeilen_section = vec!["-zeilen".to_string(), row_argument];
    if use_invertieren {
        new_zeilen_section.push("--invertieren".to_string());
    }

    let mut rebuilt =
        replace_main_section_tokens(stored_prefix_tokens, "-zeilen", &new_zeilen_section);

    if request_no_headers {
        ensure_flag_in_main_section(&mut rebuilt, "-ausgabe", "--keineueberschriften");
    }

    Some(PreparedPromptBigOutput {
        tokens: rebuilt,
        row_specs,
        had_kurz_kurz,
    })
}

fn is_selective_reta_modifier(token: &str) -> bool {
    matches!(
        token,
        "range"
            | "teiler"
            | "vielfache"
            | "invertieren"
            | "--invertieren"
            | "-ausgabe"
            | "--keineueberschriften"
            | "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
    )
}

fn is_main_switch_token(token: &str) -> bool {
    token.starts_with('-') && !token.starts_with("--")
}

fn replace_main_section_tokens(
    tokens: &[String],
    section: &str,
    replacement: &[String],
) -> Vec<String> {
    let mut out = Vec::with_capacity(tokens.len() + replacement.len());
    let mut index = 0usize;
    let mut insert_at = if tokens.is_empty() { 0usize } else { 1usize };
    let mut found = false;

    while index < tokens.len() {
        if tokens[index] == section {
            if !found {
                insert_at = out.len();
                found = true;
            }
            index += 1;
            while index < tokens.len() && !is_main_switch_token(&tokens[index]) {
                index += 1;
            }
            continue;
        }
        out.push(tokens[index].clone());
        index += 1;
    }

    let insert_at = insert_at.min(out.len());
    let mut rebuilt = Vec::with_capacity(out.len() + replacement.len());
    rebuilt.extend(out[..insert_at].iter().cloned());
    rebuilt.extend(replacement.iter().cloned());
    rebuilt.extend(out[insert_at..].iter().cloned());
    rebuilt
}

fn ensure_flag_in_main_section(tokens: &mut Vec<String>, section: &str, flag: &str) {
    if tokens.iter().any(|token| token == flag) {
        return;
    }

    let mut index = 0usize;
    while index < tokens.len() {
        if tokens[index] == section {
            let mut insert_at = index + 1;
            while insert_at < tokens.len() && !is_main_switch_token(&tokens[insert_at]) {
                insert_at += 1;
            }
            tokens.insert(insert_at, flag.to_string());
            return;
        }
        index += 1;
    }

    tokens.push(section.to_string());
    tokens.push(flag.to_string());
}

fn divisors_from_row_specs(row_specs: &[String]) -> Option<Vec<i64>> {
    let numbers = parse_row_spec_numbers(row_specs)?;
    let mut divisors = BTreeSet::new();

    for number in numbers {
        let n = number.abs();
        if n == 0 {
            continue;
        }
        let mut divisor = 1i64;
        while divisor * divisor <= n {
            if n % divisor == 0 {
                divisors.insert(divisor);
                divisors.insert(n / divisor);
            }
            divisor += 1;
        }
    }

    if divisors.len() > 1 {
        divisors.remove(&1);
    }

    Some(divisors.into_iter().collect())
}

fn parse_row_spec_numbers(row_specs: &[String]) -> Option<Vec<i64>> {
    let mut numbers = Vec::new();
    for spec in row_specs {
        for piece in custom_split_delim_parenthesized(spec, ',') {
            let trimmed = piece.trim();
            if trimmed.is_empty() {
                continue;
            }
            let normalized = trimmed.strip_prefix('v').unwrap_or(trimmed);
            if normalized.contains('/') {
                return None;
            }
            if let Some((start, end)) = parse_integer_range_piece(normalized) {
                if start <= end {
                    for value in start..=end {
                        numbers.push(value);
                    }
                } else {
                    for value in (end..=start).rev() {
                        numbers.push(value);
                    }
                }
            } else {
                numbers.push(normalized.parse::<i64>().ok()?);
            }
        }
    }
    Some(numbers)
}

fn parse_integer_range_piece(piece: &str) -> Option<(i64, i64)> {
    let (left, right) = piece.split_once('-')?;
    if left.is_empty() || right.is_empty() {
        return None;
    }
    let start = left.trim().parse::<i64>().ok()?;
    let end = right.trim().parse::<i64>().ok()?;
    Some((start, end))
}

fn parse_prefix_and_numeric_suffix(text: &str) -> Option<(String, String)> {
    if text.is_empty() {
        return None;
    }
    let chars: Vec<char> = text.chars().collect();
    let mut split_at: Option<usize> = None;
    for (i, ch) in chars.iter().enumerate() {
        if ch.is_ascii_digit() || matches!(ch, '(' | '[' | '{') {
            split_at = Some(i);
            break;
        }
    }
    let split_at = split_at?;
    if split_at == 0 {
        return None;
    }
    let mut prefix = chars[..split_at].iter().collect::<String>();
    if prefix.ends_with('-') {
        prefix.pop();
        return Some((prefix, chars[split_at - 1..].iter().collect::<String>()));
    }
    let suffix = chars[split_at..].iter().collect::<String>();
    Some((prefix, suffix))
}

#[cfg(test)]
mod tests {
    use super::{
        build_reta_calls_from_prompt_tokens, expand_python_regex_like_tokens,
        prepare_prompt_big_output_for_stored_reta,
    };

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    #[test]
    fn stored_reta_placeholder_replaces_zeilen_section_python_like() {
        let prepared = prepare_prompt_big_output_for_stored_reta(
            &strings(&["reta", "-zeilen", "--zeit=heute", "-spalten", "--thomas"]),
            &strings(&["12-15"]),
        )
        .expect("stored reta placeholder should be rewritten");

        assert_eq!(
            prepared.tokens,
            strings(&[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=12-15",
                "-spalten",
                "--thomas",
            ])
        );
    }

    #[test]
    fn stored_reta_placeholder_supports_range_and_teiler_modifiers() {
        let prepared = prepare_prompt_big_output_for_stored_reta(
            &strings(&["reta", "-zeilen", "--zeit=heute", "-spalten", "--geist"]),
            &strings(&["R", "w12"]),
        )
        .expect("range + teiler path should be rewritten");

        assert_eq!(
            prepared.tokens,
            strings(&[
                "reta",
                "-zeilen",
                "--zaehlung=2,3,4,6,12",
                "-spalten",
                "--geist",
            ])
        );
    }

    #[test]
    fn stored_reta_placeholder_supports_vielfache_modifier() {
        let prepared = prepare_prompt_big_output_for_stored_reta(
            &strings(&["reta", "-zeilen", "--zeit=heute", "-spalten", "--impulse"]),
            &strings(&["v12-15"]),
        )
        .expect("vielfache path should be rewritten");

        assert_eq!(
            prepared.tokens,
            strings(&[
                "reta",
                "-zeilen",
                "--vielfachevonzahlen=12-15",
                "-spalten",
                "--impulse",
            ])
        );
    }

    #[test]
    fn prompt_execution_regex_expands_prompt_command_like_python() {
        let expanded = expand_python_regex_like_tokens(&strings(&["r\"emo.*\""]));
        assert_eq!(expanded, strings(&["emotion"]));
    }

    #[test]
    fn prompt_execution_regex_expands_reta_value_like_python() {
        let expanded =
            expand_python_regex_like_tokens(&strings(&["reta", "-zeilen", "--zeit=r\"heu.*\""]));
        assert_eq!(expanded, strings(&["reta", "-zeilen", "--zeit=heute"]));
    }

    #[test]
    fn build_reta_calls_supports_concept_prefixed_prompt_commands() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["12", "EIGNweisheit"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0].iter().any(|token| token == "--konzept=weisheit"));
    }

    #[test]
    fn build_reta_calls_supports_reciprocal_concept_prefixed_prompt_commands() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["1/2", "EIGRgleichheit"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--konzept2=gleichheit"));
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=2"));
    }

    #[test]
    fn build_reta_calls_transform_teiler_without_legacy_flag() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["emotion", "12", "teiler"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=2,3,4,6,12"));
        assert!(calls[0]
            .iter()
            .any(|token| token.starts_with("--oberesmaximum=")));
        assert!(!calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnittteiler"));
    }

    #[test]
    fn semantic_call_keeps_passthrough_reta_parameters() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&[
            "emotion",
            "12",
            "--nocolor",
            "--breite=80",
        ]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0].iter().any(|token| token == "--nocolor"));
        assert!(calls[0].iter().any(|token| token == "--breite=80"));
    }
}
