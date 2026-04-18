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

    // Python LibRetaPrompt builds these directly from i18n.wahl15/wahl16:
    //   {"15_" + a for a in wahl15.keys()}
    //   {"16_15_" + a for a in wahl15.keys() if a != "15"}
    //   {"16_" + a for a in wahl16.keys()}
    // retaPrompt.py then adds the naked prefix commands "15_" and "16_".
    for key in semantic_wahl15_ordered_keys() {
        if !key.is_empty() {
            befehle.push(format!("15_{key}"));
        }
    }
    befehle.push("15_".to_string());
    for key in semantic_wahl15_ordered_keys() {
        if !key.is_empty() && *key != "15" {
            befehle.push(format!("16_15_{key}"));
        }
    }
    for key in semantic_wahl16_ordered_keys() {
        if !key.is_empty() {
            befehle.push(format!("16_{key}"));
        }
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
    pattern.chars().any(|ch| {
        matches!(
            ch,
            '.' | '*' | '+' | '?' | '^' | '$' | '|' | '(' | ')' | '[' | ']' | '\\'
        )
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RegexLikeQuantifier {
    ExactlyOne,
    ZeroOrMore,
    OneOrMore,
    ZeroOrOne,
}

#[derive(Clone, Debug)]
enum RegexLikeAtom {
    Literal(char),
    Any,
    Class {
        negated: bool,
        ranges: Vec<(char, char)>,
    },
    Group(Vec<Vec<RegexLikeToken>>),
}

#[derive(Clone, Debug)]
struct RegexLikeToken {
    atom: RegexLikeAtom,
    quantifier: RegexLikeQuantifier,
}

struct RegexLikeParser {
    chars: Vec<char>,
    pos: usize,
}

impl RegexLikeParser {
    fn new(pattern: &str) -> Self {
        Self {
            chars: pattern.chars().collect(),
            pos: 0,
        }
    }

    fn parse(pattern: &str) -> Option<Vec<Vec<RegexLikeToken>>> {
        let mut parser = Self::new(pattern);
        let alternatives = parser.parse_alternatives(None)?;
        (parser.pos == parser.chars.len()).then_some(alternatives)
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += 1;
        Some(ch)
    }

    fn parse_alternatives(&mut self, until: Option<char>) -> Option<Vec<Vec<RegexLikeToken>>> {
        let mut alternatives = Vec::new();
        loop {
            alternatives.push(self.parse_sequence(until)?);
            match (self.peek(), until) {
                (Some('|'), _) => {
                    self.pos += 1;
                }
                (Some(ch), Some(end)) if ch == end => {
                    self.pos += 1;
                    break;
                }
                (None, None) => break,
                (None, Some(_)) => return None,
                _ => break,
            }
        }
        Some(alternatives)
    }

    fn parse_sequence(&mut self, until: Option<char>) -> Option<Vec<RegexLikeToken>> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            if ch == '|' || until.is_some_and(|end| ch == end) {
                break;
            }
            let atom = self.parse_atom()?;
            let quantifier = match self.peek() {
                Some('*') => {
                    self.pos += 1;
                    RegexLikeQuantifier::ZeroOrMore
                }
                Some('+') => {
                    self.pos += 1;
                    RegexLikeQuantifier::OneOrMore
                }
                Some('?') => {
                    self.pos += 1;
                    RegexLikeQuantifier::ZeroOrOne
                }
                _ => RegexLikeQuantifier::ExactlyOne,
            };
            tokens.push(RegexLikeToken { atom, quantifier });
        }
        Some(tokens)
    }

    fn parse_atom(&mut self) -> Option<RegexLikeAtom> {
        match self.bump()? {
            '\\' => self.bump().map(RegexLikeAtom::Literal),
            '.' => Some(RegexLikeAtom::Any),
            '[' => self.parse_class(),
            '(' => self.parse_group(),
            ch => Some(RegexLikeAtom::Literal(ch)),
        }
    }

    fn parse_group(&mut self) -> Option<RegexLikeAtom> {
        if self.peek() == Some('?') && self.peek_next() == Some(':') {
            self.pos += 2;
        }
        let alternatives = self.parse_alternatives(Some(')'))?;
        Some(RegexLikeAtom::Group(alternatives))
    }

    fn parse_class_char(&mut self) -> Option<char> {
        match self.bump()? {
            '\\' => self.bump(),
            ch => Some(ch),
        }
    }

    fn parse_class(&mut self) -> Option<RegexLikeAtom> {
        let negated = matches!(self.peek(), Some('^') | Some('!'));
        if negated {
            self.pos += 1;
        }

        let mut ranges = Vec::new();
        let mut closed = false;
        while self.pos < self.chars.len() {
            if self.peek() == Some(']') {
                self.pos += 1;
                closed = true;
                break;
            }

            let start = self.parse_class_char()?;
            if self.peek() == Some('-') && self.peek_next() != Some(']') {
                self.pos += 1;
                let end = self.parse_class_char()?;
                if start <= end {
                    ranges.push((start, end));
                } else {
                    ranges.push((end, start));
                }
            } else {
                ranges.push((start, start));
            }
        }

        if !closed {
            return None;
        }

        Some(RegexLikeAtom::Class { negated, ranges })
    }
}

fn regex_like_atom_match_positions(
    atom: &RegexLikeAtom,
    text: &[char],
    start: usize,
) -> BTreeSet<usize> {
    let mut out = BTreeSet::new();
    match atom {
        RegexLikeAtom::Literal(expected) => {
            if text.get(start).is_some_and(|actual| actual == expected) {
                out.insert(start + 1);
            }
        }
        RegexLikeAtom::Any => {
            if start < text.len() {
                out.insert(start + 1);
            }
        }
        RegexLikeAtom::Class { negated, ranges } => {
            if let Some(actual) = text.get(start) {
                let contained = ranges
                    .iter()
                    .any(|(begin, end)| begin <= actual && actual <= end);
                if contained != *negated {
                    out.insert(start + 1);
                }
            }
        }
        RegexLikeAtom::Group(alternatives) => {
            for alternative in alternatives {
                out.extend(regex_like_sequence_match_positions(alternative, text, start));
            }
        }
    }
    out
}

fn regex_like_repeat_positions(
    atom: &RegexLikeAtom,
    text: &[char],
    start: usize,
) -> BTreeSet<usize> {
    let mut seen = BTreeSet::new();
    let mut frontier = BTreeSet::new();
    seen.insert(start);
    frontier.insert(start);

    while !frontier.is_empty() {
        let mut next_frontier = BTreeSet::new();
        for position in frontier {
            for next in regex_like_atom_match_positions(atom, text, position) {
                if next != position && seen.insert(next) {
                    next_frontier.insert(next);
                }
            }
        }
        frontier = next_frontier;
    }

    seen
}

fn regex_like_token_match_positions(
    token: &RegexLikeToken,
    text: &[char],
    start: usize,
) -> BTreeSet<usize> {
    match token.quantifier {
        RegexLikeQuantifier::ExactlyOne => {
            regex_like_atom_match_positions(&token.atom, text, start)
        }
        RegexLikeQuantifier::ZeroOrOne => {
            let mut out = BTreeSet::new();
            out.insert(start);
            out.extend(regex_like_atom_match_positions(&token.atom, text, start));
            out
        }
        RegexLikeQuantifier::ZeroOrMore => regex_like_repeat_positions(&token.atom, text, start),
        RegexLikeQuantifier::OneOrMore => {
            let mut out = BTreeSet::new();
            for first_end in regex_like_atom_match_positions(&token.atom, text, start) {
                out.extend(regex_like_repeat_positions(&token.atom, text, first_end));
            }
            out
        }
    }
}

fn regex_like_sequence_match_positions(
    tokens: &[RegexLikeToken],
    text: &[char],
    start: usize,
) -> BTreeSet<usize> {
    let mut positions = BTreeSet::new();
    positions.insert(start);

    for token in tokens {
        let mut next_positions = BTreeSet::new();
        for position in positions {
            next_positions.extend(regex_like_token_match_positions(token, text, position));
        }
        if next_positions.is_empty() {
            return BTreeSet::new();
        }
        positions = next_positions;
    }

    positions
}

pub(crate) fn regex_like_search(pattern: &str, text: &str) -> bool {
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

    let Some(alternatives) = RegexLikeParser::parse(core) else {
        return regex_like_search_fallback_dot_star(pattern, text);
    };
    let text_chars = text.chars().collect::<Vec<_>>();
    let starts = if start_anchor {
        vec![0usize]
    } else {
        (0..=text_chars.len()).collect::<Vec<_>>()
    };

    for start in starts {
        for alternative in &alternatives {
            let ends = regex_like_sequence_match_positions(alternative, &text_chars, start);
            if end_anchor {
                if ends.contains(&text_chars.len()) {
                    return true;
                }
            } else if !ends.is_empty() {
                return true;
            }
        }
    }

    false
}

fn regex_like_search_fallback_dot_star(pattern: &str, text: &str) -> bool {
    let (start_anchor, end_anchor, core) = strip_regex_like_anchors(pattern);
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

    if let Some(section) = current_section {
        let parameter_fragment = token.strip_prefix("--").unwrap_or(token);
        if let Some(matcher) = parse_special_fragment_matcher(parameter_fragment) {
            for (parameter, values) in reta_section_parameter_inventory_for_regex(section) {
                if values.is_empty() && special_fragment_matches_candidate(&parameter, &matcher) {
                    push_unique_preserving_normalized(
                        &mut out,
                        &mut seen,
                        format!("--{parameter}"),
                    );
                }
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

fn collapse_python_style_equals_tokens(tokens: &[String]) -> Vec<String> {
    fn flush_group(target: &mut Vec<String>, prefix: &mut Option<String>, values: &mut Vec<String>) {
        let Some(current_prefix) = prefix.take() else {
            return;
        };

        if values.is_empty() {
            target.push(current_prefix);
            return;
        }

        target.push(format!("{}{}", current_prefix, values.join(",")));
        values.clear();
    }

    let mut collapsed = Vec::new();
    let mut current_prefix: Option<String> = None;
    let mut current_values: Vec<String> = Vec::new();

    for token in tokens {
        if let Some(eq_index) = token.find('=') {
            let prefix = token[..=eq_index].to_string();
            let value = token[eq_index + 1..].to_string();
            if current_prefix.as_deref() == Some(prefix.as_str()) {
                current_values.push(value);
                continue;
            }
            flush_group(&mut collapsed, &mut current_prefix, &mut current_values);
            current_prefix = Some(prefix);
            current_values = vec![value];
            continue;
        }

        flush_group(&mut collapsed, &mut current_prefix, &mut current_values);
        collapsed.push(token.clone());
    }

    flush_group(&mut collapsed, &mut current_prefix, &mut current_values);
    collapsed
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
    let Some(section) = current_section else {
        return Vec::new();
    };

    let left_core = left.trim().strip_prefix("--").unwrap_or(left.trim());
    let inventory = reta_section_parameter_inventory_for_regex(section);

    let parameter_names = if left_core.trim().is_empty() {
        inventory.keys().cloned().collect::<Vec<_>>()
    } else if let Some(matcher) = parse_special_fragment_matcher(left_core) {
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
    let mut changed = false;

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
            if token_has_python_regex_or_glob(token) {
                changed = true;
                continue;
            }
            out.push(token.clone());
        } else {
            changed = true;
            out.extend(expanded);
        }
    }

    if changed {
        collapse_python_style_equals_tokens(&out)
    } else {
        out
    }
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
    if !trimmed.contains('/') && python_row_piece_is_integer_like(trimmed) {
        return true;
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
    reciprocal_whole_para: Option<&'static str>,
    integer_cols: &'static str,
    reciprocal_whole_cols: &'static str,
    non_whole_fraction_para: Option<&'static str>,
    equal_fraction_para: Option<&'static str>,
    equal_fraction_cols: Option<&'static str>,
    include_reverse_non_whole: bool,
    dynamic_universe_columns: bool,
}

fn semantic_specs() -> &'static [PromptSemanticSpec] {
    static SPECS: OnceLock<Vec<PromptSemanticSpec>> = OnceLock::new();
    SPECS
        .get_or_init(|| {
            vec![
                PromptSemanticSpec {
                    names: &["thomas"],
                    integer_para: "--galaxie=thomas",
                    reciprocal_whole_para: None,
                    integer_cols: "2",
                    reciprocal_whole_cols: "2",
                    non_whole_fraction_para: None,
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: false,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["emotion"],
                    integer_para: "--grundstrukturen=emotion",
                    reciprocal_whole_para: None,
                    integer_cols: "2,3",
                    reciprocal_whole_cols: "4,5",
                    non_whole_fraction_para: Some("--gebrochenemotion="),
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: true,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["wirklichkeit"],
                    integer_para: "--grundstrukturen=wirklichkeit",
                    reciprocal_whole_para: None,
                    integer_cols: "1,2",
                    reciprocal_whole_cols: "5",
                    non_whole_fraction_para: None,
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: false,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["triebe"],
                    integer_para: "--grundstrukturen=triebe",
                    reciprocal_whole_para: None,
                    integer_cols: "1",
                    reciprocal_whole_cols: "2",
                    non_whole_fraction_para: None,
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: false,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["impulse"],
                    integer_para: "--grundstrukturen=impulse",
                    reciprocal_whole_para: None,
                    integer_cols: "1,4",
                    reciprocal_whole_cols: "3",
                    non_whole_fraction_para: None,
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: false,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["bewusstsein"],
                    integer_para: "--grundstrukturen=bewusstsein",
                    reciprocal_whole_para: None,
                    integer_cols: "6",
                    reciprocal_whole_cols: "7",
                    non_whole_fraction_para: None,
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: false,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["geist"],
                    integer_para: "--grundstrukturen=geist",
                    reciprocal_whole_para: None,
                    integer_cols: "3",
                    reciprocal_whole_cols: "4",
                    non_whole_fraction_para: None,
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: false,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["freiheit", "gleichheit"],
                    integer_para: "--planet=freiheit",
                    reciprocal_whole_para: None,
                    integer_cols: "1-4,8",
                    reciprocal_whole_cols: "5-7",
                    non_whole_fraction_para: None,
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: false,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["groesse"],
                    integer_para: "--strukturgroesse=organisation",
                    reciprocal_whole_para: None,
                    integer_cols: "1-3",
                    reciprocal_whole_cols: "99",
                    non_whole_fraction_para: Some("--gebrochengroesse="),
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: true,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["kugeln", "kreise"],
                    integer_para: "--universum=kugeln",
                    reciprocal_whole_para: None,
                    integer_cols: "1-2",
                    reciprocal_whole_cols: "99",
                    non_whole_fraction_para: None,
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: false,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["netzwerk"],
                    integer_para: "--universum=netzwerk",
                    reciprocal_whole_para: None,
                    integer_cols: "1-3",
                    reciprocal_whole_cols: "99",
                    non_whole_fraction_para: None,
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: false,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["komplex"],
                    integer_para: "--universum=komplex",
                    reciprocal_whole_para: None,
                    integer_cols: "1",
                    reciprocal_whole_cols: "3",
                    non_whole_fraction_para: None,
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: false,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["absicht", "absichten", "motiv", "motive"],
                    integer_para: "--menschliches=motivation",
                    reciprocal_whole_para: None,
                    integer_cols: "1",
                    reciprocal_whole_cols: "3",
                    non_whole_fraction_para: Some("--gebrochengalaxie="),
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: true,
                    dynamic_universe_columns: false,
                },
                PromptSemanticSpec {
                    names: &["universum"],
                    integer_para: "--universum=transzendentalien",
                    reciprocal_whole_para: Some("--universum=transzendentaliereziproke"),
                    integer_cols: "1",
                    reciprocal_whole_cols: "1",
                    non_whole_fraction_para: Some("--gebrochenuniversum="),
                    equal_fraction_para: Some("--universum=verhaeltnisgleicherzahl"),
                    equal_fraction_cols: Some("1"),
                    include_reverse_non_whole: true,
                    dynamic_universe_columns: true,
                },
                PromptSemanticSpec {
                    names: &["richtung"],
                    integer_para: "--primzahlwirkung=galaxieabsicht",
                    reciprocal_whole_para: None,
                    integer_cols: "",
                    reciprocal_whole_cols: "",
                    non_whole_fraction_para: None,
                    equal_fraction_para: None,
                    equal_fraction_cols: None,
                    include_reverse_non_whole: false,
                    dynamic_universe_columns: false,
                },
            ]
        })
        .as_slice()
}

fn prompt_command_count(tokens: &[String]) -> usize {
    tokens
        .iter()
        .filter(|token| prompt_words().befehle_set.contains(*token))
        .count()
}

fn semantic_columns_for_spec(
    spec: &PromptSemanticSpec,
    normalized: &[String],
    reciprocal_kind: bool,
    suppress_empty: bool,
    no_headers: bool,
) -> String {
    if spec.dynamic_universe_columns {
        let show_extra_columns =
            prompt_command_count(normalized) <= 2 && !suppress_empty && !no_headers;
        let mut columns = String::from("1");
        if show_extra_columns {
            columns.push_str(if reciprocal_kind { ",2" } else { ",4" });
        }
        return columns;
    }

    if reciprocal_kind {
        spec.reciprocal_whole_cols.to_string()
    } else {
        spec.integer_cols.to_string()
    }
}

fn semantic_non_whole_fraction_reverse_columns(_spec: &PromptSemanticSpec) -> &'static str {
    "1"
}

fn semantic_non_whole_fraction_normal_columns(_spec: &PromptSemanticSpec) -> &'static str {
    "2"
}

fn semantic_wahl15_ordered_keys() -> &'static [&'static str] {
    &[
        "15",
        "2",
        "5",
        "7",
        "8",
        "10",
        "1pro30",
        "12",
        "13",
        "17",
        "18",
        "6",
        "9",
        "3",
        "13_6",
        "13_7",
        "13_10",
        "13_17",
        "10_4",
        "16",
        "4",
        "13_1pro8",
        "13_1pro6",
        "1pro15",
        "1",
        "30",
        "14",
        "14_6",
        "20",
        "37",
        "31",
        "11",
        "5_10",
        "17_6",
        "17_6_10mit4",
        "36",
        "13_16",
        "18_7",
        "18_10",
        "18_17",
        "1pro8",
        "1pro5",
        "1pro3",
        "10_4_18_6",
        "18_6",
        "21",
        "26",
        "19",
        "18_15",
        "18_15_n-vs-1pron",
        "1pro13",
        "1pro19",
        "90",
        "13_13",
        "1pro12",
        "39",
        "1pro6",
        "28",
        "24",
        "32",
        "gegen5",
        "9_6",
        "51",
        "13_4",
        "7mit6",
        "",
    ]
}

fn semantic_wahl16_ordered_keys() -> &'static [&'static str] {
    &["1", "2", "3", "5", "6", "15", "10", "16", "20", ""]
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
            ("1pro30", "analytische_Ontologie"),
            ("12", "Meta-Systeme_(12),Ordnung_und_Filterung_12_und_1pro12"),
            ("13", "Paradigmen_sind_Absichten_(13)"),
            ("17", "Gedanken_sind_Positionen_(17)"),
            ("18", "Verbundenheiten_(18)"),
            ("6", "Triebe_und_Bedürfnisse_(6),System"),
            ("9", "Lust_(9)"),
            ("3", "Reflexe_(3),Existenzialien_(3)"),
            ("13_6", "Absicht_6_ist_Vorteilsmaximierung"),
            ("13_7", "Absicht_7_ist_Selbstlosigkeit"),
            ("13_10", "Absicht_10_ist_Wirklichkeit_erkennen"),
            ("13_17", "Absicht_17_ist_zu_meinen"),
            ("10_4", "Zeit_(4)_als_Wirklichkeit"),
            ("16", "Funktionen_Vorstellungen_(16)"),
            ("4", "Achtung_(4)"),
            ("13_1pro8", "Absicht_1/8"),
            ("13_1pro6", "Absicht_1/6_ist_Reinigung_und_Klarheit"),
            ("1pro15", "Reflektion_und_Kategorien_(1/15)"),
            ("1", "Bewusstheit_statt_Bewusstsein_(1)"),
            ("30", "Energie_und_universelle_Eigenschaften_(30)"),
            ("14", "Stimmungen_Kombinationen_(14)"),
            ("14_6", "Rechnen"),
            ("20", "Klassen_(20)"),
            ("37", "Empathie_(37)"),
            ("31", "Garben_und_Verhalten_nachfühlen(31)"),
            ("11", "Verhalten_(11)"),
            ("5_10", "Bedeutung_(10)"),
            ("17_6", "Themen_(6)"),
            ("17_6_10mit4", "Optimierung_(10)"),
            ("36", "Attraktionen_(36)"),
            ("13_16", "Absicht_16_ist_zu_genügen"),
            ("18_7", "Liebe_(7)"),
            ("18_10", "Koalitionen_(10)"),
            ("18_17", "Ansichten_Standpunkte_(18_17)"),
            ("1pro8", "Prinzipien(1/8)"),
            ("1pro5", "Bestrebungen(1/5)"),
            ("1pro3", "Bedingung_und_Auslöser_(1/3)"),
            ("10_4_18_6", "relativer_Zeit-Betrag_(15_10_4_18_6)"),
            ("18_6", "Zahlenvergleich_(15_18_6)"),
            ("21", "Leidenschaften_(21)"),
            ("26", "Erwartungshaltungen_(26)"),
            ("19", "Extremalien_(19),Ziele_(19)"),
            ("18_15", "universeller_Komperativ_(18→15)"),
            ("18_15_n-vs-1pron", "Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)"),
            ("1pro13", "Sollen_Frage_Vorgehensweise_(1/13)"),
            ("1pro19", "Fundament_(1/19)"),
            ("90", "abhängige_Verbundenheit_(90)"),
            ("13_13", "Absicht_13_ist_Helfen"),
            ("1pro12", "Karte_Filter_und_Unterscheidung_(1/12)"),
            ("39", "Maßnahmen_(39)"),
            ("1pro6", "innere_Werte_1/6_der_Reinigung_und_Klarheit"),
            ("28", "Lebensbereiche_Problemklassen_(28)"),
            ("24", "Netzwerk"),
            ("32", "mathematisches_Design_(32)"),
            ("gegen5", "gegen_5"),
            ("9_6", "strukturgroesse"),
            ("51", "Kontroverse_(51)"),
            ("13_4", "Taetigkeiten"),
            ("7mit6", "Wohlbefinden_(7mit6)"),
            ("", "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity,Biologischer_Baum_(15),Teilchen_anderes_Universum,pro_contra"),
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
            ("15", "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Model_of_Hierarchical_Complexity"),
            ("10", "Struktur-Wissenschaften_(10)"),
            ("16", "Multiversalien_(16),P"),
            ("20", "Muster-Wissenschaften_(20)"),
            ("", "Multiversalien_(16),P"),
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
    equal_fraction_row_specs: Vec<String>,
    non_whole_fraction_denominator_groups: BTreeMap<i64, Vec<String>>,
    non_whole_fraction_numerator_groups: BTreeMap<i64, Vec<String>>,
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

fn strip_matching_row_wrappers(mut text: &str) -> &str {
    loop {
        let trimmed = text.trim();
        let stripped = if trimmed.starts_with('(') && trimmed.ends_with(')') {
            Some(&trimmed[1..trimmed.len() - 1])
        } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
            Some(&trimmed[1..trimmed.len() - 1])
        } else if trimmed.starts_with('{') && trimmed.ends_with('}') {
            Some(&trimmed[1..trimmed.len() - 1])
        } else {
            None
        };
        match stripped {
            Some(inner) => text = inner,
            None => return trimmed,
        }
    }
}

fn strip_row_piece_prefixes(piece: &str) -> (bool, &str) {
    let mut subtract = false;
    let mut rest = piece.trim();

    loop {
        let mut changed = false;
        if let Some(next) = rest.strip_prefix('-') {
            subtract = !subtract;
            rest = next.trim_start();
            changed = true;
        }
        if !changed {
            break;
        }
    }

    (subtract, strip_matching_row_wrappers(rest))
}

fn split_python_fraction_piece_prefixes(piece: &str) -> Option<(bool, String)> {
    let mut rest = strip_matching_row_wrappers(piece.trim());
    let mut subtract = false;
    let mut vielfache = false;

    loop {
        rest = rest.trim_start();
        if let Some(next) = rest.strip_prefix('-') {
            subtract = !subtract;
            rest = next.trim_start();
            continue;
        }
        if let Some(next) = rest.strip_prefix('v') {
            vielfache = true;
            rest = next.trim_start();
            if let Some(next) = rest.strip_prefix('-') {
                subtract = !subtract;
                rest = next.trim_start();
            }
            continue;
        }
        break;
    }

    let rest = strip_matching_row_wrappers(rest.trim());
    if rest.is_empty() || !rest.contains('/') {
        return None;
    }

    let mut normalized = String::new();
    if vielfache {
        normalized.push('v');
    }
    normalized.push_str(rest);
    Some((subtract, normalized))
}

fn split_fraction_operator(text: &str, operator: char) -> Option<(&str, &str)> {
    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;

    for (index, ch) in text.char_indices() {
        match ch {
            '(' => round += 1,
            ')' => round -= 1,
            '[' => square += 1,
            ']' => square -= 1,
            '{' => curly += 1,
            '}' => curly -= 1,
            _ if ch == operator && index > 0 && round == 0 && square == 0 && curly == 0 => {
                let left = text[..index].trim();
                let right = text[index + ch.len_utf8()..].trim();
                if !left.is_empty() && !right.is_empty() {
                    return Some((left, right));
                }
            }
            _ => {}
        }
    }

    None
}

fn inclusive_i64_range(start: i64, end: i64) -> Vec<i64> {
    if start <= end {
        (start..=end).collect()
    } else {
        (end..=start).rev().collect()
    }
}

const PYTHON_ROW_MULTIPLE_LIMIT: i64 = 1028;

fn python_fraction_allowed_numbers() -> &'static [i64] {
    static ALLOWED: OnceLock<Vec<i64>> = OnceLock::new();
    ALLOWED
        .get_or_init(|| {
            let mut values = BTreeSet::new();
            for entry in &shared_words().paraNdataMatrix {
                let is_fraction_parameter = entry.parameterMainNames.iter().any(|name| {
                    let normalized = normalize_match_text(name);
                    normalized.contains("gebrochen-rational")
                        || normalized.contains("gebrochenuniversum")
                        || normalized.contains("gebrochengalaxie")
                        || normalized.contains("gebrochenemotion")
                        || normalized.contains("gebrochengroesse")
                });
                if is_fraction_parameter {
                    for name in &entry.parameterNames {
                        if let Ok(value) = name.parse::<i64>() {
                            if value > 0 {
                                values.insert(value);
                            }
                        }
                    }
                }
            }

            if values.is_empty() {
                // Python LibRetaPrompt baut `gebrochenErlaubteZahlen` aus den
                // gebrochen-rationalen Parameterwerten und entfernt danach den
                // groessten Tabellen-/Sentinelwert. In den ausgelieferten Daten
                // ist das 23, so dass die effektive Menge 2..=22 ist.
                values.extend(2..=23);
            }

            if let Some(max_value) = values.iter().next_back().copied() {
                values.remove(&max_value);
            }

            values.into_iter().collect()
        })
        .as_slice()
}

fn expand_values_over_python_fraction_allowed_multiples(values: &[i64]) -> Vec<i64> {
    let mut out = BTreeSet::new();
    for value in values {
        let base = value.abs();
        if base == 0 {
            continue;
        }
        for allowed in python_fraction_allowed_numbers() {
            if *allowed > 0 && *allowed % base == 0 {
                out.insert(*allowed);
            }
        }
    }
    out.into_iter().collect()
}

fn parse_unsigned_row_i64(text: &str) -> Option<i64> {
    let trimmed = text.trim();
    if trimmed.is_empty() || !trimmed.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    trimmed.parse::<i64>().ok()
}

fn parse_python_row_piece_flags(piece: &str) -> Option<(bool, bool, &str)> {
    let mut rest = piece.trim();
    let mut subtract = false;
    let mut vielfache = false;

    loop {
        rest = rest.trim_start();
        if let Some(next) = rest.strip_prefix('v') {
            vielfache = true;
            rest = next;
            continue;
        }
        if let Some(next) = rest.strip_prefix('-') {
            subtract = !subtract;
            rest = next;
            continue;
        }
        break;
    }

    let body = rest.trim();
    if body.is_empty() {
        None
    } else {
        Some((subtract, vielfache, body))
    }
}

fn parse_python_integer_row_piece_core(body: &str) -> Option<(i64, i64, Vec<i64>)> {
    let body = strip_matching_row_wrappers(body.trim());
    if body.is_empty() || body.contains('/') {
        return None;
    }

    let mut split = body.split('+');
    let range_text = split.next()?.trim();
    if range_text.is_empty() {
        return None;
    }

    let mut around = Vec::new();
    for part in split {
        around.push(parse_unsigned_row_i64(part)?);
    }
    if around.is_empty() {
        around.push(0);
    }

    let (start, end) = if let Some((left, right)) = range_text.split_once('-') {
        let start = parse_unsigned_row_i64(left)?;
        let end = parse_unsigned_row_i64(right)?;
        (start, end)
    } else {
        let value = parse_unsigned_row_i64(range_text)?;
        (value, value)
    };

    if start == 0 || end == 0 {
        return None;
    }

    Some((start, end, around))
}

fn expand_python_row_numbers_plain(
    start: i64,
    end: i64,
    around: &[i64],
    max_zahl: Option<i64>,
) -> Vec<i64> {
    let limit = max_zahl.unwrap_or(i64::MAX / 4);
    let mut out = BTreeSet::new();

    for number in inclusive_i64_range(start, end) {
        for distance in around {
            let plus = number.saturating_add(*distance);
            if plus > 0 && plus < limit {
                out.insert(plus);
            }

            let minus = number.saturating_sub(*distance);
            if minus > 0 && minus < limit {
                out.insert(minus);
            }
        }
    }

    out.into_iter().collect()
}

fn expand_python_row_numbers_vielfache(
    start: i64,
    end: i64,
    around: &[i64],
    max_zahl: Option<i64>,
) -> Vec<i64> {
    let limit = max_zahl.unwrap_or(PYTHON_ROW_MULTIPLE_LIMIT);
    if start <= 0 || limit <= 0 {
        return Vec::new();
    }

    let mut out = BTreeSet::new();
    let mut multiplier = 0i64;
    let only_zero_distance = around.iter().all(|distance| *distance == 0);

    loop {
        let keep_going = around.iter().all(|distance| {
            start.saturating_mul(multiplier) < limit.saturating_sub(*distance)
        });
        if !keep_going {
            break;
        }

        multiplier += 1;
        for number in inclusive_i64_range(start, end) {
            let base = number.saturating_mul(multiplier);
            if only_zero_distance {
                if base > 0 && base <= limit {
                    out.insert(base);
                }
                continue;
            }

            for distance in around {
                let plus = base.saturating_add(*distance);
                if plus > 0 && plus <= limit {
                    out.insert(plus);
                }

                let minus = base.saturating_sub(*distance);
                if minus > 0 && minus < limit {
                    out.insert(minus);
                }
            }
        }

        if multiplier > limit.saturating_add(1) {
            break;
        }
    }

    out.into_iter().collect()
}


#[derive(Clone, Copy)]
struct PythonRowExprVar<'a> {
    name: &'a str,
    value: i64,
}

struct PythonRowExprParser<'a> {
    chars: Vec<char>,
    pos: usize,
    var: Option<PythonRowExprVar<'a>>,
}

impl<'a> PythonRowExprParser<'a> {
    fn new(text: &str, var: Option<(&'a str, i64)>) -> Self {
        Self {
            chars: text.chars().collect(),
            pos: 0,
            var: var.map(|(name, value)| PythonRowExprVar { name, value }),
        }
    }

    fn finished(&self) -> bool {
        self.pos >= self.chars.len()
    }

    fn skip_ws(&mut self) {
        while !self.finished() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn starts_with_text(&self, text: &str) -> bool {
        let mut index = self.pos;
        for expected in text.chars() {
            if self.chars.get(index) != Some(&expected) {
                return false;
            }
            index += 1;
        }
        true
    }

    fn consume_text(&mut self, text: &str) -> bool {
        self.skip_ws();
        if self.starts_with_text(text) {
            self.pos += text.chars().count();
            true
        } else {
            false
        }
    }

    fn parse_expr(&mut self) -> Option<i64> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Option<i64> {
        let mut value = self.parse_mul_div()?;
        loop {
            if self.consume_text("+") {
                value = value.checked_add(self.parse_mul_div()?)?;
            } else if self.consume_text("-") {
                value = value.checked_sub(self.parse_mul_div()?)?;
            } else {
                break;
            }
        }
        Some(value)
    }

    fn parse_mul_div(&mut self) -> Option<i64> {
        let mut value = self.parse_power()?;
        loop {
            if self.consume_text("//") {
                let rhs = self.parse_power()?;
                if rhs == 0 {
                    return None;
                }
                value = value.checked_div(rhs)?;
            } else if self.consume_text("%") {
                let rhs = self.parse_power()?;
                if rhs == 0 {
                    return None;
                }
                value = value.checked_rem(rhs)?;
            } else if self.consume_text("/") {
                let rhs = self.parse_power()?;
                if rhs == 0 || value % rhs != 0 {
                    return None;
                }
                value = value.checked_div(rhs)?;
            } else if self.consume_text("*") {
                value = value.checked_mul(self.parse_power()?)?;
            } else {
                break;
            }
        }
        Some(value)
    }

    fn parse_power(&mut self) -> Option<i64> {
        let base = self.parse_unary()?;
        if self.consume_text("**") {
            let exp = self.parse_power()?;
            if !(0..=31).contains(&exp) {
                return None;
            }
            base.checked_pow(exp as u32)
        } else {
            Some(base)
        }
    }

    fn parse_unary(&mut self) -> Option<i64> {
        if self.consume_text("+") {
            self.parse_unary()
        } else if self.consume_text("-") {
            self.parse_unary()?.checked_neg()
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Option<i64> {
        self.skip_ws();
        if self.consume_text("(") {
            let value = self.parse_expr()?;
            if !self.consume_text(")") {
                return None;
            }
            return Some(value);
        }

        if let Some(value) = self.parse_number() {
            return Some(value);
        }

        let ident = self.parse_identifier()?;
        if self.consume_text("(") {
            let value = self.parse_expr()?;
            if self.consume_text(",") {
                return None;
            }
            if !self.consume_text(")") {
                return None;
            }
            return match ident.as_str() {
                "abs" => value.checked_abs(),
                "int" | "round" => Some(value),
                _ => None,
            };
        }

        match self.var {
            Some(var) if var.name == ident => Some(var.value),
            _ => None,
        }
    }

    fn parse_number(&mut self) -> Option<i64> {
        self.skip_ws();
        let start = self.pos;
        while !self.finished() && self.chars[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        if self.pos == start {
            return None;
        }
        self.chars[start..self.pos]
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .ok()
    }

    fn parse_identifier(&mut self) -> Option<String> {
        self.skip_ws();
        if self.finished() {
            return None;
        }
        let first = self.chars[self.pos];
        if !(first == '_' || first.is_ascii_alphabetic()) {
            return None;
        }
        let start = self.pos;
        self.pos += 1;
        while !self.finished()
            && (self.chars[self.pos] == '_'
                || self.chars[self.pos].is_ascii_alphanumeric())
        {
            self.pos += 1;
        }
        Some(self.chars[start..self.pos].iter().collect())
    }
}

fn eval_python_row_expr(text: &str, var: Option<(&str, i64)>) -> Option<i64> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut parser = PythonRowExprParser::new(trimmed, var);
    let value = parser.parse_expr()?;
    parser.skip_ws();
    parser.finished().then_some(value)
}

fn is_python_row_identifier(text: &str) -> bool {
    let trimmed = text.trim();
    let mut chars = trimmed.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first == '_' || first.is_ascii_alphabetic())
        && chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn strip_python_collection_wrappers(text: &str) -> Option<&str> {
    let trimmed = text.trim();
    if trimmed.len() < 2 {
        return None;
    }
    let (open, close) = match (trimmed.chars().next()?, trimmed.chars().last()?) {
        ('[', ']') => ('[', ']'),
        ('{', '}') => ('{', '}'),
        ('(', ')') => ('(', ')'),
        _ => return None,
    };
    if !trimmed.starts_with(open) || !trimmed.ends_with(close) {
        return None;
    }
    Some(&trimmed[open.len_utf8()..trimmed.len() - close.len_utf8()])
}

fn find_top_level_keyword(text: &str, keyword: &str) -> Option<usize> {
    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;

    for (index, ch) in text.char_indices() {
        if round == 0 && square == 0 && curly == 0 && text[index..].starts_with(keyword) {
            return Some(index);
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
    }
    None
}

fn split_top_level_comparison(text: &str) -> Option<(&str, &str, &str)> {
    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;
    let operators = ["==", "!=", "<=", ">=", "<", ">"];

    for (index, ch) in text.char_indices() {
        if round == 0 && square == 0 && curly == 0 {
            for operator in operators {
                if text[index..].starts_with(operator) {
                    let left = text[..index].trim();
                    let right = text[index + operator.len()..].trim();
                    if !left.is_empty() && !right.is_empty() {
                        return Some((left, operator, right));
                    }
                }
            }
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
    }
    None
}

fn eval_python_row_condition(text: &str, var: Option<(&str, i64)>) -> Option<bool> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(index) = find_top_level_keyword(trimmed, " or ") {
        return Some(
            eval_python_row_condition(&trimmed[..index], var)?
                || eval_python_row_condition(&trimmed[index + 4..], var)?,
        );
    }
    if let Some(index) = find_top_level_keyword(trimmed, " and ") {
        return Some(
            eval_python_row_condition(&trimmed[..index], var)?
                && eval_python_row_condition(&trimmed[index + 5..], var)?,
        );
    }
    if let Some(rest) = trimmed.strip_prefix("not ") {
        return Some(!eval_python_row_condition(rest, var)?);
    }
    if let Some(index) = find_top_level_keyword(trimmed, " not in ") {
        let left = eval_python_row_expr(&trimmed[..index], var)?;
        let right_values = parse_python_iterable_values(&trimmed[index + " not in ".len()..])?;
        return Some(!right_values.contains(&left));
    }
    if let Some(index) = find_top_level_keyword(trimmed, " in ") {
        let left = eval_python_row_expr(&trimmed[..index], var)?;
        let right_values = parse_python_iterable_values(&trimmed[index + " in ".len()..])?;
        return Some(right_values.contains(&left));
    }
    if let Some((left, operator, right)) = split_top_level_comparison(trimmed) {
        let left = eval_python_row_expr(left, var)?;
        let right = eval_python_row_expr(right, var)?;
        return match operator {
            "==" => Some(left == right),
            "!=" => Some(left != right),
            "<=" => Some(left <= right),
            ">=" => Some(left >= right),
            "<" => Some(left < right),
            ">" => Some(left > right),
            _ => None,
        };
    }
    Some(eval_python_row_expr(trimmed, var)? != 0)
}

fn parse_python_range_values(text: &str) -> Option<Vec<i64>> {
    let trimmed = text.trim();
    let rest = trimmed.strip_prefix("range")?.trim_start();
    let inner = rest.strip_prefix('(')?.strip_suffix(')')?;
    let parts = custom_split_delim_parenthesized(inner, ',');
    let args = parts
        .iter()
        .map(|part| eval_python_row_expr(part, None))
        .collect::<Option<Vec<_>>>()?;
    let (start, stop, step) = match args.as_slice() {
        [stop] => (0, *stop, 1),
        [start, stop] => (*start, *stop, 1),
        [start, stop, step] => (*start, *stop, *step),
        _ => return None,
    };
    if step == 0 {
        return None;
    }

    let mut out = Vec::new();
    let mut current = start;
    while if step > 0 { current < stop } else { current > stop } {
        out.push(current);
        if out.len() > 20_000 {
            return None;
        }
        current = current.checked_add(step)?;
    }
    Some(out)
}

fn split_top_level_iterable_binary<'a>(text: &'a str, operator: char) -> Option<(&'a str, &'a str)> {
    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;

    for (index, ch) in text.char_indices() {
        if ch == operator && index > 0 && round == 0 && square == 0 && curly == 0 {
            let left = text[..index].trim();
            let right = text[index + ch.len_utf8()..].trim();
            if !left.is_empty() && !right.is_empty() {
                return Some((left, right));
            }
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
    }

    None
}

fn parse_python_call_inner<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let trimmed = text.trim();
    let rest = trimmed.strip_prefix(name)?.trim_start();
    if !rest.starts_with('(') || !rest.ends_with(')') {
        return None;
    }
    Some(&rest[1..rest.len() - 1])
}

fn parse_python_builtin_iterable_values(text: &str) -> Option<Vec<i64>> {
    for name in ["list", "tuple", "set", "frozenset", "sorted"] {
        if let Some(inner) = parse_python_call_inner(text, name) {
            let source = if name == "sorted" {
                custom_split_delim_parenthesized(inner, ',')
                    .into_iter()
                    .next()
                    .unwrap_or_default()
            } else {
                inner.to_string()
            };
            return parse_python_iterable_values(&source);
        }
    }
    None
}

fn parse_python_iterable_values(text: &str) -> Option<Vec<i64>> {
    let trimmed = text.trim();

    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        let inner = &trimmed[1..trimmed.len() - 1];
        if split_top_level_iterable_binary(inner, '|').is_some()
            || split_top_level_iterable_binary(inner, '+').is_some()
        {
            return parse_python_iterable_values(inner);
        }
    }

    if let Some((left, right)) = split_top_level_iterable_binary(trimmed, '|') {
        let mut out = parse_python_iterable_values(left)?;
        out.extend(parse_python_iterable_values(right)?);
        return Some(out.into_iter().collect::<BTreeSet<_>>().into_iter().collect());
    }

    if let Some((left, right)) = split_top_level_iterable_binary(trimmed, '+') {
        let mut out = parse_python_iterable_values(left)?;
        out.extend(parse_python_iterable_values(right)?);
        return Some(out);
    }

    parse_python_range_values(trimmed)
        .or_else(|| parse_python_builtin_iterable_values(trimmed))
        .or_else(|| parse_python_generated_row_values(trimmed))
}

fn parse_python_generated_row_values(text: &str) -> Option<Vec<i64>> {
    let inner = strip_python_collection_wrappers(text)?;
    let inner = inner.trim();
    if inner.is_empty() {
        return Some(Vec::new());
    }

    if let Some(for_index) = find_top_level_keyword(inner, " for ") {
        let expr_text = inner[..for_index].trim();
        let tail = inner[for_index + 5..].trim();
        let in_index = find_top_level_keyword(tail, " in ")?;
        let var_name = tail[..in_index].trim();
        if !is_python_row_identifier(var_name) {
            return None;
        }

        let mut source_text = tail[in_index + 4..].trim();
        let mut filter_text = None;
        if let Some(if_index) = find_top_level_keyword(source_text, " if ") {
            filter_text = Some(source_text[if_index + 4..].trim());
            source_text = source_text[..if_index].trim();
        }

        let source_values = parse_python_iterable_values(source_text)?;
        let mut out = BTreeSet::new();
        for value in source_values {
            let var = Some((var_name, value));
            let keep = match filter_text {
                Some(filter) => eval_python_row_condition(filter, var)?,
                None => true,
            };
            if keep {
                out.insert(eval_python_row_expr(expr_text, var)?);
            }
        }
        return Some(out.into_iter().collect());
    }

    let mut out = BTreeSet::new();
    for part in custom_split_delim_parenthesized(inner, ',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        out.insert(eval_python_row_expr(trimmed, None)?);
    }
    Some(out.into_iter().collect())
}

fn expand_python_row_generated_values_vielfache(
    values: &[i64],
    max_zahl: Option<i64>,
) -> Vec<i64> {
    let limit = max_zahl.unwrap_or(PYTHON_ROW_MULTIPLE_LIMIT);
    if limit <= 0 {
        return Vec::new();
    }

    let mut out = BTreeSet::new();
    for value in values {
        let base = value.abs();
        if base == 0 {
            continue;
        }
        let mut multiple = base;
        while multiple <= limit {
            out.insert(multiple);
            match multiple.checked_add(base) {
                Some(next) if next > multiple => multiple = next,
                _ => break,
            }
        }
    }
    out.into_iter().collect()
}

fn python_row_piece_is_integer_like(piece: &str) -> bool {
    let Some((_, _, body)) = parse_python_row_piece_flags(piece) else {
        return false;
    };
    parse_python_generated_row_values(body).is_some()
        || parse_python_integer_row_piece_core(body).is_some()
}

fn python_row_piece_to_numbers(
    piece: &str,
    inherited_vielfache: bool,
    max_zahl: Option<i64>,
) -> Option<(bool, Vec<i64>)> {
    let (subtract, inline_vielfache, body) = parse_python_row_piece_flags(piece)?;
    let use_vielfache = inherited_vielfache || inline_vielfache;

    if let Some(values) = parse_python_generated_row_values(body) {
        let values = if use_vielfache {
            expand_python_row_generated_values_vielfache(&values, max_zahl)
        } else {
            values
        };
        return (!values.is_empty()).then_some((subtract, values));
    }

    let (start, end, around) = parse_python_integer_row_piece_core(body)?;
    let values = if use_vielfache {
        expand_python_row_numbers_vielfache(start, end, &around, max_zahl)
    } else {
        expand_python_row_numbers_plain(start, end, &around, max_zahl)
    };

    (!values.is_empty()).then_some((subtract, values))
}

fn python_row_spec_to_numbers_with_options(
    spec: &str,
    inherited_vielfache: bool,
    max_zahl: Option<i64>,
) -> Option<Vec<i64>> {
    let mut dazu = BTreeSet::new();
    let mut hinfort = BTreeSet::new();
    let mut saw_piece = false;

    for piece in custom_split_delim_parenthesized(spec, ',') {
        let trimmed = piece.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.contains('/') && !python_row_piece_is_integer_like(trimmed) {
            return None;
        }

        let (subtract, values) =
            python_row_piece_to_numbers(trimmed, inherited_vielfache, max_zahl)?;
        saw_piece = true;
        for value in values {
            let value = value.abs();
            if value == 0 {
                continue;
            }
            if subtract {
                hinfort.insert(value);
            } else {
                dazu.insert(value);
            }
        }
    }

    for value in hinfort {
        dazu.remove(&value);
    }

    if saw_piece {
        Some(dazu.into_iter().collect())
    } else {
        None
    }
}

pub fn python_row_spec_to_numbers(spec: &str) -> Option<Vec<i64>> {
    python_row_spec_to_numbers_with_options(spec, false, Some(PYTHON_ROW_MULTIPLE_LIMIT))
}


fn expand_fraction_range_piece(piece: &str) -> Option<Vec<(i64, i64)>> {
    let (left, right) = split_fraction_operator(piece, '-')?;
    let (left_numerator, left_denominator) = parse_simple_fraction_piece(left)?;
    let (right_numerator, right_denominator) = parse_simple_fraction_piece(right)?;

    let numerators = inclusive_i64_range(left_numerator, right_numerator);
    let denominators = inclusive_i64_range(left_denominator, right_denominator);
    let mut out = Vec::new();

    for denominator in denominators {
        if denominator == 0 {
            continue;
        }
        for numerator in &numerators {
            if *numerator == 0 {
                continue;
            }
            out.push((*numerator, denominator));
        }
    }

    (!out.is_empty()).then_some(out)
}

fn expand_fraction_distance_piece(piece: &str) -> Option<Vec<(i64, i64)>> {
    let (left, right) = split_fraction_operator(piece, '+')?;
    let (base_numerator, base_denominator) = parse_simple_fraction_piece(left)?;
    let (delta_numerator, delta_denominator) = parse_simple_fraction_piece(right)?;

    let numerator_candidates = [
        base_numerator - delta_numerator,
        base_numerator + delta_numerator,
    ];
    let denominator_candidates = [
        base_denominator - delta_denominator,
        base_denominator + delta_denominator,
    ];

    let mut out = Vec::new();
    for denominator in denominator_candidates {
        if denominator == 0 {
            continue;
        }
        for numerator in numerator_candidates {
            if numerator == 0 {
                continue;
            }
            out.push((numerator, denominator));
        }
    }

    (!out.is_empty()).then_some(out)
}

fn expand_fraction_piece_values(piece: &str) -> Option<Vec<(i64, i64)>> {
    let inner = strip_matching_row_wrappers(piece.trim());
    expand_fraction_range_piece(inner)
        .or_else(|| expand_fraction_distance_piece(inner))
        .or_else(|| parse_simple_fraction_piece(inner).map(|fraction| vec![fraction]))
}

#[derive(Clone, Debug)]
struct PythonBruchSpaltPart {
    number_before_text: Vec<String>,
    text: Vec<String>,
    number_after_text: Vec<String>,
    only_numbers: bool,
}

impl PythonBruchSpaltPart {
    fn only(numbers: Vec<String>) -> Self {
        Self {
            number_before_text: numbers,
            text: Vec::new(),
            number_after_text: Vec::new(),
            only_numbers: true,
        }
    }

    fn mixed(
        number_before_text: Vec<String>,
        text: Vec<String>,
        number_after_text: Vec<String>,
    ) -> Self {
        Self {
            number_before_text,
            text,
            number_after_text,
            only_numbers: false,
        }
    }

    fn before_fraction_text(&self) -> Vec<String> {
        if self.only_numbers {
            Vec::new()
        } else {
            self.text.clone()
        }
    }

    fn after_fraction_text(&self) -> Vec<String> {
        if self.only_numbers {
            Vec::new()
        } else {
            self.text.clone()
        }
    }

    fn numerator_side_values(&self) -> Vec<String> {
        if self.only_numbers {
            self.number_before_text.clone()
        } else {
            self.number_after_text.clone()
        }
    }

    fn denominator_side_values(&self) -> Vec<String> {
        self.number_before_text.clone()
    }
}

#[derive(Clone, Debug)]
struct PythonBruchRanges {
    numerators: Vec<i64>,
    denominator_spec: String,
}

fn ordered_values_from_segment_map(map: &BTreeMap<usize, String>) -> Vec<String> {
    map.values().cloned().collect()
}

fn python_bruch_spalt(text: &str) -> Option<Vec<Vec<String>>> {
    let parts = text.split('/').collect::<Vec<_>>();
    if parts.len() < 2 {
        return None;
    }

    let mut parsed_parts: Vec<PythonBruchSpaltPart> = Vec::new();
    let mut out: Vec<Vec<String>> = Vec::new();

    for (index, part) in parts.iter().enumerate() {
        let mut numbers: BTreeMap<usize, String> = BTreeMap::new();
        let mut non_numbers: BTreeMap<usize, String> = BTreeMap::new();
        let mut was_number = false;
        let mut go_next = 0usize;

        for ch in part.chars() {
            if ch.is_ascii_digit() {
                if !was_number {
                    go_next = go_next.saturating_add(1);
                }
                numbers.entry(go_next).or_default().push(ch);
                was_number = true;
            } else {
                if was_number {
                    go_next = go_next.saturating_add(1);
                }
                non_numbers.entry(go_next).or_default().push(ch);
                was_number = false;
            }
        }

        if numbers.is_empty() {
            return None;
        }

        let all_comparison = non_numbers
            .keys()
            .zip(numbers.keys())
            .map(|(non_number_key, number_key)| number_key > non_number_key)
            .collect::<Vec<_>>();
        let is_first = index == 0;
        let is_last = index + 1 == parts.len();
        let valid = if is_first && all_comparison.iter().all(|value| *value) {
            true
        } else if is_last && !all_comparison.iter().any(|value| *value) {
            true
        } else if !is_first && !is_last && !non_numbers.is_empty() {
            let min_number = *numbers.keys().next()?;
            let max_number = *numbers.keys().last()?;
            non_numbers
                .keys()
                .all(|key| *key > min_number && *key < max_number)
        } else {
            false
        };
        if !valid {
            return None;
        }

        let parsed = if non_numbers.is_empty() {
            if is_first || is_last {
                PythonBruchSpaltPart::only(ordered_values_from_segment_map(&numbers))
            } else {
                return None;
            }
        } else {
            let min_non_number = *non_numbers.keys().next()?;
            let max_non_number = *non_numbers.keys().last()?;
            let number_before_text = numbers
                .iter()
                .filter(|(key, _)| **key < min_non_number)
                .map(|(_, value)| value.clone())
                .collect::<Vec<_>>();
            let number_after_text = numbers
                .iter()
                .filter(|(key, _)| **key > max_non_number)
                .map(|(_, value)| value.clone())
                .collect::<Vec<_>>();
            if is_last && !number_after_text.is_empty() {
                return None;
            }
            PythonBruchSpaltPart::mixed(
                number_before_text,
                ordered_values_from_segment_map(&non_numbers),
                number_after_text,
            )
        };

        parsed_parts.push(parsed);

        if index == 1 {
            let previous = &parsed_parts[0];
            let current = &parsed_parts[1];
            out.push(previous.before_fraction_text());
            let mut fraction = previous.numerator_side_values();
            fraction.extend(current.denominator_side_values());
            out.push(fraction);
            if is_last {
                out.push(current.after_fraction_text());
            }
        } else if is_last && index > 1 {
            let previous = &parsed_parts[index - 1];
            let current = &parsed_parts[index];
            out.push(previous.before_fraction_text());
            let mut fraction = previous.numerator_side_values();
            fraction.extend(current.denominator_side_values());
            out.push(fraction);
            out.push(current.after_fraction_text());
        } else if index > 1 {
            let previous = &parsed_parts[index - 1];
            let current = &parsed_parts[index];
            out.push(previous.before_fraction_text());
            let mut fraction = previous.numerator_side_values();
            fraction.extend(current.denominator_side_values());
            out.push(fraction);
        }
    }

    Some(out)
}

fn is_python_bruch_fraction_tuple(values: &[String]) -> bool {
    values.len() == 2 && values.iter().all(|value| parse_unsigned_row_i64(value).is_some())
}

fn parse_python_bruch_fraction_tuple(values: &[String]) -> Option<(i64, i64)> {
    if !is_python_bruch_fraction_tuple(values) {
        return None;
    }
    Some((
        parse_unsigned_row_i64(&values[0])?,
        parse_unsigned_row_i64(&values[1])?,
    ))
}

fn create_ranges_for_python_bruch_list(bruch_list: &[Vec<String>]) -> Option<PythonBruchRanges> {
    let special_simple_fraction = bruch_list.len() == 3
        && bruch_list[0].is_empty()
        && is_python_bruch_fraction_tuple(&bruch_list[1])
        && bruch_list[2].is_empty();
    if special_simple_fraction {
        let (numerator, denominator) = parse_python_bruch_fraction_tuple(&bruch_list[1])?;
        return Some(PythonBruchRanges {
            numerators: vec![numerator],
            denominator_spec: denominator.to_string(),
        });
    }

    let mut first_fraction_numerators: Vec<i64> = Vec::new();
    let mut first_fraction_denominators: Vec<i64> = Vec::new();
    let mut numerator_range: Vec<i64> = Vec::new();
    let mut numerator_range_origin: Vec<i64> = Vec::new();
    let mut flag = 0i32;
    let mut denominator_pieces: Vec<String> = Vec::new();

    for (index, values) in bruch_list.iter().enumerate() {
        if flag == -1 {
            return None;
        }
        if flag > 3 {
            return None;
        } else if flag == 3 {
            let left_denominator = *first_fraction_denominators.get(first_fraction_denominators.len().saturating_sub(2))?;
            let right_denominator = *first_fraction_denominators.last()?;
            denominator_pieces.push(left_denominator.to_string());
            denominator_pieces.push("-".to_string());
            denominator_pieces.push(right_denominator.to_string());

            let start = *first_fraction_numerators.get(first_fraction_numerators.len().saturating_sub(2))?;
            let end = *first_fraction_numerators.last()?;
            numerator_range = if start <= end {
                (start..=end).collect()
            } else {
                Vec::new()
            };
            numerator_range_origin = numerator_range.clone();
            flag = -1;
        }

        if let Some((numerator, denominator)) = parse_python_bruch_fraction_tuple(values) {
            let next_is_minus = bruch_list
                .get(index + 1)
                .map(|next| next.len() == 1 && next[0] == "-")
                .unwrap_or(false);
            let previous_is_minus = index > 0
                && bruch_list
                    .get(index - 1)
                    .map(|previous| previous.len() == 1 && previous[0] == "-")
                    .unwrap_or(false);
            if (next_is_minus && flag == 0) || (previous_is_minus && flag == 2) {
                first_fraction_numerators.push(numerator);
                first_fraction_denominators.push(denominator);
                flag += 1;
            } else {
                denominator_pieces.push(denominator.to_string());
                let previous_is_plus = index > 0
                    && bruch_list
                        .get(index - 1)
                        .map(|previous| previous.len() == 1 && previous[0] == "+")
                        .unwrap_or(false);
                if !numerator_range.is_empty() && previous_is_plus {
                    let mut next_range = Vec::new();
                    for origin in &numerator_range_origin {
                        next_range.push(origin.saturating_add(numerator));
                        next_range.push(origin.saturating_sub(numerator));
                    }
                    numerator_range = next_range;
                } else if numerator_range.is_empty() {
                    numerator_range = vec![numerator];
                    numerator_range_origin = numerator_range.clone();
                }
            }
        } else if values.len() == 1 && values[0] == "-" && flag > 0 {
            flag += 1;
        } else {
            flag = 0;
            denominator_pieces.extend(values.iter().cloned());
        }
    }

    Some(PythonBruchRanges {
        numerators: numerator_range,
        denominator_spec: denominator_pieces.join(""),
    })
}

fn parse_python_bruch_spalt_group_piece(piece: &str) -> Option<PythonFractionGroup> {
    let inner = strip_matching_row_wrappers(piece.trim());
    let bruch_list = python_bruch_spalt(inner)?;
    let ranges = create_ranges_for_python_bruch_list(&bruch_list)?;
    if ranges.denominator_spec.trim().is_empty() {
        return Some(PythonFractionGroup {
            numerator_values: Vec::new(),
            denominator_values: Vec::new(),
        });
    }

    let numerator_values = ranges
        .numerators
        .into_iter()
        .filter(|value| *value > 0)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let denominator_values = fraction_denominator_values_from_python_spec(&ranges.denominator_spec);
    Some(PythonFractionGroup {
        numerator_values,
        denominator_values,
    })
}


#[derive(Clone, Debug)]
struct PythonFractionGroup {
    numerator_values: Vec<i64>,
    denominator_values: Vec<i64>,
}

fn sorted_positive_values<I>(values: I) -> Vec<i64>
where
    I: IntoIterator<Item = i64>,
{
    values
        .into_iter()
        .map(i64::abs)
        .filter(|value| *value > 0)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn parse_fraction_pair_with_inline_vielfache(piece: &str) -> Option<(bool, i64, i64)> {
    let inner = strip_matching_row_wrappers(piece.trim());
    let (vielfache, inner) = match inner.strip_prefix('v') {
        Some(rest) => (true, rest.trim_start()),
        None => (false, inner),
    };
    let (left, right) = inner.split_once('/')?;
    let numerator = left.trim().parse::<i64>().ok()?;
    let denominator = right.trim().parse::<i64>().ok()?;
    if numerator == 0 || denominator == 0 {
        return None;
    }
    Some((vielfache, numerator, denominator))
}

fn fraction_denominator_values_from_python_spec(spec: &str) -> Vec<i64> {
    let trimmed = spec.trim();
    let (inline_vielfache, core) = match trimmed.strip_prefix('v') {
        Some(rest) => (true, rest.trim_start()),
        None => (false, trimmed),
    };

    let base_values = python_row_spec_to_numbers_with_options(
        core,
        false,
        Some(PYTHON_ROW_MULTIPLE_LIMIT),
    )
    .unwrap_or_else(|| {
        parse_unsigned_row_i64(core)
            .map(|value| vec![value])
            .unwrap_or_default()
    })
    .into_iter()
    .filter(|value| *value > 0)
    .collect::<BTreeSet<_>>()
    .into_iter()
    .collect::<Vec<_>>();

    if inline_vielfache {
        expand_values_over_python_fraction_allowed_multiples(&base_values)
    } else {
        base_values
    }
}

fn parse_python_fraction_group_piece(piece: &str) -> Option<PythonFractionGroup> {
    let inner = strip_matching_row_wrappers(piece.trim());
    if inner.is_empty() || !inner.contains('/') {
        return None;
    }

    if let Some(group) = parse_python_bruch_spalt_group_piece(inner) {
        return Some(group);
    }

    if let Some((left, right)) = split_fraction_operator(inner, '-') {
        let (vielfache, left_numerator, left_denominator) =
            parse_fraction_pair_with_inline_vielfache(left)?;
        let (_, right_numerator, right_denominator) =
            parse_fraction_pair_with_inline_vielfache(right)?;
        let denominator_spec = format!(
            "{}{}-{}",
            if vielfache { "v" } else { "" },
            left_denominator.abs(),
            right_denominator.abs()
        );
        let numerator_values = sorted_positive_values(inclusive_i64_range(
            left_numerator.abs(),
            right_numerator.abs(),
        ));
        let denominator_values = fraction_denominator_values_from_python_spec(&denominator_spec);
        return (!numerator_values.is_empty() && !denominator_values.is_empty()).then_some(
            PythonFractionGroup {
                numerator_values,
                denominator_values,
            },
        );
    }

    let plus_parts = custom_split_delim_parenthesized(inner, '+')
        .into_iter()
        .map(|part| part.trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if plus_parts.len() > 1 {
        let (vielfache, base_numerator, base_denominator) =
            parse_fraction_pair_with_inline_vielfache(&plus_parts[0])?;
        let base_numerator_abs = base_numerator.abs();
        let mut numerator_values: BTreeSet<i64> = BTreeSet::new();
        let mut denominator_parts = vec![base_denominator.abs().to_string()];
        for part in plus_parts.iter().skip(1) {
            let (_, delta_numerator, delta_denominator) =
                parse_fraction_pair_with_inline_vielfache(part)?;
            let plus = base_numerator_abs.saturating_add(delta_numerator.abs());
            let minus = base_numerator_abs.saturating_sub(delta_numerator.abs());
            if plus > 0 {
                numerator_values.insert(plus);
            }
            if minus > 0 {
                numerator_values.insert(minus);
            }
            denominator_parts.push(delta_denominator.abs().to_string());
        }
        let denominator_spec = format!(
            "{}{}",
            if vielfache { "v" } else { "" },
            denominator_parts.join("+")
        );
        let numerator_values = numerator_values.into_iter().collect::<Vec<_>>();
        let denominator_values = fraction_denominator_values_from_python_spec(&denominator_spec);
        return (!numerator_values.is_empty() && !denominator_values.is_empty()).then_some(
            PythonFractionGroup {
                numerator_values,
                denominator_values,
            },
        );
    }

    let (vielfache, numerator, denominator) = parse_fraction_pair_with_inline_vielfache(inner)?;
    let denominator_spec = format!(
        "{}{}",
        if vielfache { "v" } else { "" },
        denominator.abs()
    );
    let numerator_values = vec![numerator.abs()];
    let denominator_values = fraction_denominator_values_from_python_spec(&denominator_spec);
    (!numerator_values.is_empty() && !denominator_values.is_empty()).then_some(PythonFractionGroup {
        numerator_values,
        denominator_values,
    })
}

fn insert_fraction_group_value(map: &mut BTreeMap<i64, BTreeSet<i64>>, key: i64, value: i64) {
    if key <= 0 || value <= 0 {
        return;
    }
    map.entry(key).or_default().insert(value);
}

fn finalize_fraction_group_map(map: BTreeMap<i64, BTreeSet<i64>>) -> BTreeMap<i64, Vec<String>> {
    map.into_iter()
        .map(|(key, values)| {
            (
                key,
                values
                    .into_iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn should_use_python_reverse_fraction_groups(map: &BTreeMap<i64, BTreeSet<i64>>) -> bool {
    if map.is_empty() {
        return false;
    }

    // Python bruchBereichsManagementAndWbefehl sammelt zuerst je Zaehler-/Range-
    // Punkt die expandierten Nennerwerte, bildet daraus die Gesamtmenge und
    // invertiert genau dann mit invert_dict_B, wenn
    // len(gesamtmenge) / len(rangesBruecheDict) < 1 ist. Das passiert z.B. bei
    // 2/5-3/5: zwei Zaehlerpunkte teilen sich einen Nennerwert, also wird daraus
    // der Reverse-Pfad --gebrochen*=5 mit Zeilen 2,3 und Spaltenfilter 1.
    let mut combined_values = BTreeSet::new();
    for values in map.values() {
        for value in values {
            combined_values.insert(*value);
        }
    }

    combined_values.len() < map.len()
}

fn invert_python_fraction_groups(
    map: &BTreeMap<i64, BTreeSet<i64>>,
) -> BTreeMap<i64, BTreeSet<i64>> {
    let mut inverted: BTreeMap<i64, BTreeSet<i64>> = BTreeMap::new();
    for (key, values) in map {
        for value in values {
            insert_fraction_group_value(&mut inverted, *value, *key);
        }
    }
    inverted
}

fn expand_python_fraction_groups_for_global_vielfache(
    map: &BTreeMap<i64, BTreeSet<i64>>,
) -> BTreeMap<i64, BTreeSet<i64>> {
    let mut expanded: BTreeMap<i64, BTreeSet<i64>> = BTreeMap::new();

    for (numerator, denominators) in map {
        let numerator_multiples =
            expand_values_over_python_fraction_allowed_multiples(&[*numerator]);
        if numerator_multiples.is_empty() {
            continue;
        }

        let denominator_values = denominators.iter().copied().collect::<Vec<_>>();
        let denominator_multiples =
            expand_values_over_python_fraction_allowed_multiples(&denominator_values);
        if denominator_multiples.is_empty() {
            continue;
        }

        for numerator_multiple in &numerator_multiples {
            for denominator_multiple in &denominator_multiples {
                insert_fraction_group_value(
                    &mut expanded,
                    *numerator_multiple,
                    *denominator_multiple,
                );
            }
        }
    }

    expanded
}

fn add_python_fraction_integer_side_effects(
    groups: &BTreeMap<i64, BTreeSet<i64>>,
    primary_numbers: &mut BTreeSet<i64>,
    reciprocal_numbers: &mut BTreeSet<i64>,
    equal_fraction_numbers: &mut BTreeSet<i64>,
) {
    for (numerator, denominators) in groups {
        let numerator = (*numerator).abs();
        for denominator in denominators {
            let denominator = (*denominator).abs();
            if numerator == 0 || denominator == 0 {
                continue;
            }
            if numerator == denominator && numerator > 1 {
                equal_fraction_numbers.insert(numerator);
            }
            if numerator % denominator == 0 {
                primary_numbers.insert(numerator / denominator);
            }
            if denominator % numerator == 0 {
                reciprocal_numbers.insert(denominator / numerator);
            }
        }
    }
}

fn build_python_row_buckets_with_global_vielfache(
    row_specs: &[String],
    global_vielfache: bool,
) -> PythonRowBuckets {
    let mut buckets = PythonRowBuckets::default();

    let mut primary_numbers: BTreeSet<i64> = BTreeSet::new();
    let mut reciprocal_numbers: BTreeSet<i64> = BTreeSet::new();
    let mut negative_primary_numbers: BTreeSet<i64> = BTreeSet::new();
    let mut negative_reciprocal_numbers: BTreeSet<i64> = BTreeSet::new();
    let mut equal_fraction_numbers: BTreeSet<i64> = BTreeSet::new();
    let mut negative_equal_fraction_numbers: BTreeSet<i64> = BTreeSet::new();
    let mut non_whole_fraction_groups: BTreeMap<i64, BTreeSet<i64>> = BTreeMap::new();
    let mut negative_non_whole_fraction_groups: BTreeMap<i64, BTreeSet<i64>> = BTreeMap::new();

    for spec in row_specs {
        let trimmed = spec.trim();
        if trimmed.is_empty() {
            continue;
        }

        for piece in custom_split_delim_parenthesized(trimmed, ',') {
            let piece_trimmed = piece.trim();
            if piece_trimmed.is_empty() {
                continue;
            }

            let (subtract, core): (bool, std::borrow::Cow<'_, str>) = if let Some((
                fraction_subtract,
                fraction_core,
            )) = split_python_fraction_piece_prefixes(piece_trimmed)
            {
                (fraction_subtract, std::borrow::Cow::Owned(fraction_core))
            } else {
                let (piece_subtract, piece_core) = strip_row_piece_prefixes(piece_trimmed);
                (piece_subtract, std::borrow::Cow::Borrowed(piece_core))
            };
            let core = core.as_ref();

            if core.contains('/')
                && !python_row_piece_is_integer_like(piece_trimmed)
                && !python_row_piece_is_integer_like(core)
            {
                push_unique_string(&mut buckets.raw_fraction_specs, piece_trimmed.to_string());

                if let Some(group) = parse_python_fraction_group_piece(core) {
                    for numerator in &group.numerator_values {
                        for denominator in &group.denominator_values {
                            if *numerator == 0 || *denominator == 0 {
                                continue;
                            }
                            let numerator_abs = (*numerator).abs();
                            let denominator_abs = (*denominator).abs();

                            if numerator_abs == denominator_abs && numerator_abs > 1 {
                                if subtract {
                                    negative_equal_fraction_numbers.insert(numerator_abs);
                                } else {
                                    equal_fraction_numbers.insert(numerator_abs);
                                }
                            }
                            if numerator_abs % denominator_abs == 0 {
                                let value = numerator_abs / denominator_abs;
                                if subtract {
                                    negative_primary_numbers.insert(value);
                                } else {
                                    primary_numbers.insert(value);
                                }
                            }
                            if denominator_abs % numerator_abs == 0 {
                                let value = denominator_abs / numerator_abs;
                                if subtract {
                                    negative_reciprocal_numbers.insert(value);
                                } else {
                                    reciprocal_numbers.insert(value);
                                }
                            }
                        }

                        if *numerator == 1 {
                            for denominator in &group.denominator_values {
                                let denominator_abs = (*denominator).abs();
                                if denominator_abs == 0 {
                                    continue;
                                }
                                if subtract {
                                    negative_reciprocal_numbers.insert(denominator_abs);
                                } else {
                                    reciprocal_numbers.insert(denominator_abs);
                                }
                            }
                            continue;
                        }

                        for denominator in &group.denominator_values {
                            let numerator_abs = (*numerator).abs();
                            let denominator_abs = (*denominator).abs();
                            if numerator_abs == 0 || denominator_abs == 0 {
                                continue;
                            }
                            if subtract {
                                insert_fraction_group_value(
                                    &mut negative_non_whole_fraction_groups,
                                    numerator_abs,
                                    denominator_abs,
                                );
                            } else {
                                insert_fraction_group_value(
                                    &mut non_whole_fraction_groups,
                                    numerator_abs,
                                    denominator_abs,
                                );
                            }
                        }
                    }
                    continue;
                }

                if let Some(fractions) = expand_fraction_piece_values(core) {
                    for (numerator, denominator) in fractions {
                        let numerator_abs = numerator.abs();
                        let denominator_abs = denominator.abs();
                        if numerator_abs == 0 || denominator_abs == 0 {
                            continue;
                        }
                        if numerator_abs == denominator_abs && numerator_abs > 1 {
                            if subtract {
                                negative_equal_fraction_numbers.insert(numerator_abs);
                            } else {
                                equal_fraction_numbers.insert(numerator_abs);
                            }
                        }
                        if numerator_abs % denominator_abs == 0 {
                            let value = numerator_abs / denominator_abs;
                            if subtract {
                                negative_primary_numbers.insert(value);
                            } else {
                                primary_numbers.insert(value);
                            }
                        }
                        if denominator_abs % numerator_abs == 0 {
                            let value = denominator_abs / numerator_abs;
                            if subtract {
                                negative_reciprocal_numbers.insert(value);
                            } else {
                                reciprocal_numbers.insert(value);
                            }
                        }
                        if numerator_abs % denominator_abs != 0
                            && denominator_abs % numerator_abs != 0
                        {
                            if subtract {
                                insert_fraction_group_value(
                                    &mut negative_non_whole_fraction_groups,
                                    numerator_abs,
                                    denominator_abs,
                                );
                            } else {
                                insert_fraction_group_value(
                                    &mut non_whole_fraction_groups,
                                    numerator_abs,
                                    denominator_abs,
                                );
                            }
                        }
                    }
                }
                continue;
            }

            if let Some((piece_subtract, values)) =
                python_row_piece_to_numbers(piece_trimmed, false, Some(PYTHON_ROW_MULTIPLE_LIMIT))
            {
                for value in values {
                    let value_abs = value.abs();
                    if value_abs == 0 {
                        continue;
                    }
                    if piece_subtract {
                        negative_primary_numbers.insert(value_abs);
                    } else {
                        primary_numbers.insert(value_abs);
                    }
                }
            }
        }
    }

    if global_vielfache {
        non_whole_fraction_groups =
            expand_python_fraction_groups_for_global_vielfache(&non_whole_fraction_groups);
        negative_non_whole_fraction_groups =
            expand_python_fraction_groups_for_global_vielfache(
                &negative_non_whole_fraction_groups,
            );
        reciprocal_numbers = expand_values_over_python_fraction_allowed_multiples(
            &reciprocal_numbers.iter().copied().collect::<Vec<_>>(),
        )
        .into_iter()
        .collect();
        negative_reciprocal_numbers = expand_values_over_python_fraction_allowed_multiples(
            &negative_reciprocal_numbers
                .iter()
                .copied()
                .collect::<Vec<_>>(),
        )
        .into_iter()
        .collect();
        equal_fraction_numbers = expand_values_over_python_fraction_allowed_multiples(
            &equal_fraction_numbers.iter().copied().collect::<Vec<_>>(),
        )
        .into_iter()
        .collect();
        negative_equal_fraction_numbers = expand_values_over_python_fraction_allowed_multiples(
            &negative_equal_fraction_numbers
                .iter()
                .copied()
                .collect::<Vec<_>>(),
        )
        .into_iter()
        .collect();
    }

    for (key, values) in negative_non_whole_fraction_groups {
        if let Some(current) = non_whole_fraction_groups.get_mut(&key) {
            for value in values {
                current.remove(&value);
            }
        }
    }
    non_whole_fraction_groups.retain(|_, values| !values.is_empty());

    add_python_fraction_integer_side_effects(
        &non_whole_fraction_groups,
        &mut primary_numbers,
        &mut reciprocal_numbers,
        &mut equal_fraction_numbers,
    );

    for value in negative_primary_numbers {
        primary_numbers.remove(&value);
    }
    for value in negative_reciprocal_numbers {
        reciprocal_numbers.remove(&value);
    }
    for value in negative_equal_fraction_numbers {
        equal_fraction_numbers.remove(&value);
    }

    buckets.primary_row_specs = primary_numbers
        .into_iter()
        .map(|value| value.to_string())
        .collect();
    buckets.reciprocal_row_specs = reciprocal_numbers
        .into_iter()
        .map(|value| value.to_string())
        .collect();
    buckets.equal_fraction_row_specs = equal_fraction_numbers
        .into_iter()
        .map(|value| value.to_string())
        .collect();

    if should_use_python_reverse_fraction_groups(&non_whole_fraction_groups) {
        buckets.non_whole_fraction_denominator_groups = BTreeMap::new();
        buckets.non_whole_fraction_numerator_groups =
            finalize_fraction_group_map(invert_python_fraction_groups(&non_whole_fraction_groups));
    } else {
        buckets.non_whole_fraction_denominator_groups =
            finalize_fraction_group_map(non_whole_fraction_groups);
        buckets.non_whole_fraction_numerator_groups = BTreeMap::new();
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

fn another_oberesmaximum_from_row_specs_with_seed(row_specs: &[String], seed: i64) -> String {
    let max_row = parse_row_spec_numbers(row_specs)
        .and_then(|numbers| numbers.into_iter().map(i64::abs).max())
        .unwrap_or(seed);
    format!("--oberesmaximum={}", std::cmp::max(max_row, seed) + 1)
}

fn another_oberesmaximum_from_row_specs(row_specs: &[String]) -> Option<String> {
    if row_specs.is_empty() {
        return None;
    }

    Some(another_oberesmaximum_from_row_specs_with_seed(
        row_specs,
        prompt_python_default_oberesmaximum_seed(),
    ))
}

#[derive(Clone, Debug, Default)]
struct BuiltRowSection {
    tokens: Vec<String>,
}

fn build_python_row_section_with_custom_oberesmaximum(
    row_specs: &[String],
    use_range: bool,
    use_teiler: bool,
    use_vielfache: bool,
    invert: bool,
    forced_oberesmaximum_seed: Option<i64>,
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
        if let Some(seed) = forced_oberesmaximum_seed {
            tokens.push(another_oberesmaximum_from_row_specs_with_seed(
                &base_specs,
                seed,
            ));
        }
    } else {
        let prefix = if use_range {
            "--zaehlung="
        } else {
            "--vorhervonausschnitt="
        };
        tokens.push(format!("{prefix}{}", base_specs.join(",")));
        if let Some(seed) = forced_oberesmaximum_seed {
            tokens.push(another_oberesmaximum_from_row_specs_with_seed(
                &base_specs,
                seed,
            ));
        } else if let Some(oberesmaximum) = another_oberesmaximum_from_row_specs(&base_specs) {
            tokens.push(oberesmaximum);
        }
    }

    if invert {
        tokens.push("--invertieren".to_string());
    }

    Some(BuiltRowSection { tokens })
}

fn build_python_row_section(
    row_specs: &[String],
    use_range: bool,
    use_teiler: bool,
    use_vielfache: bool,
    invert: bool,
) -> Option<BuiltRowSection> {
    build_python_row_section_with_custom_oberesmaximum(
        row_specs,
        use_range,
        use_teiler,
        use_vielfache,
        invert,
        None,
    )
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

fn parameter_token_base(token: &str) -> Option<&str> {
    if !token.starts_with("--") {
        return None;
    }
    Some(
        token
            .split_once('=')
            .map(|(head, _)| head)
            .unwrap_or(token)
            .trim_start_matches("--"),
    )
}

fn is_known_reta_parameter_token(token: &str) -> bool {
    parameter_token_base(token)
        .map(|base| reta_global_parameter_inventory_for_regex().contains_key(base))
        .unwrap_or(false)
}

fn is_spalten_parameter_token(token: &str) -> bool {
    parameter_token_base(token)
        .map(|base| spalten_parameter_inventory_for_regex().contains_key(base))
        .unwrap_or(false)
}

fn is_ausgabe_parameter_token(token: &str) -> bool {
    parameter_token_base(token)
        .map(|base| ausgabe_parameter_inventory_for_regex().contains_key(base))
        .unwrap_or(false)
}

fn is_kombination_parameter_token(token: &str) -> bool {
    parameter_token_base(token)
        .map(|base| kombination_parameter_inventory_for_regex().contains_key(base))
        .unwrap_or(false)
}

fn is_zeilen_parameter_token(token: &str) -> bool {
    is_known_reta_parameter_token(token)
        && !is_spalten_parameter_token(token)
        && !is_ausgabe_parameter_token(token)
        && !is_kombination_parameter_token(token)
}

fn is_conflicting_generated_zeilen_parameter_token(token: &str) -> bool {
    matches!(
        parameter_token_base(token),
        Some(
            "zaehlung"
                | "vorhervonausschnitt"
                | "vorhervonausschnittteiler"
                | "primzahlvielfache"
                | "nachtraeglichneuabzaehlung"
                | "nachtraeglichneuabzaehlungvielfache"
                | "vielfachevonzahlen"
                | "oberesmaximum"
                | "invertieren"
        )
    )
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

fn append_passthrough_params_to_reta_argv(argv: &mut Vec<String>, extra_params: &[String]) {
    let mut zeilen = Vec::new();
    let mut spalten = Vec::new();
    let mut kombination = Vec::new();
    let mut ausgabe = Vec::new();
    let mut other = Vec::new();

    for token in extra_params {
        if is_zeilen_parameter_token(token) {
            if is_conflicting_generated_zeilen_parameter_token(token) {
                continue;
            }
            push_unique_string(&mut zeilen, token.clone());
        } else if is_spalten_parameter_token(token) {
            push_unique_string(&mut spalten, token.clone());
        } else if is_kombination_parameter_token(token) {
            push_unique_string(&mut kombination, token.clone());
        } else if is_ausgabe_parameter_token(token) {
            push_unique_string(&mut ausgabe, token.clone());
        } else {
            push_unique_string(&mut other, token.clone());
        }
    }

    merge_tokens_into_main_section(argv, "-zeilen", &zeilen);
    merge_tokens_into_main_section(argv, "-spalten", &spalten);
    merge_tokens_into_main_section(argv, "-kombination", &kombination);
    merge_tokens_into_main_section(argv, "-ausgabe", &ausgabe);
    for token in other {
        if !argv.contains(&token) {
            argv.push(token);
        }
    }
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
    argv.push("-ausgabe".to_string());
    argv.push("--breite=0".to_string());
    if let Some(cols) = cols {
        if !cols.trim().is_empty() {
            argv.push(format!("--spaltenreihenfolgeundnurdiese={cols}"));
        }
    }
    if suppress_empty {
        argv.push("--keineleereninhalte".to_string());
    }
    if no_headers {
        argv.push("--keineueberschriften".to_string());
    }
    append_passthrough_params_to_reta_argv(&mut argv, extra_params);
    for token in trailing_tokens {
        argv.push(token.clone());
    }
    argv
}

fn build_python_special_prompt_call(
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
) -> Vec<String> {
    build_general_semantic_call(
        row_specs,
        use_range,
        invert,
        use_teiler,
        use_vielfache,
        suppress_empty,
        no_headers,
        para,
        cols,
        extra_params,
        &[],
    )
}

fn build_primzahlkreuz_prompt_call(
    row_specs: &[String],
    use_range: bool,
    invert: bool,
    use_teiler: bool,
    use_vielfache: bool,
    suppress_empty: bool,
    no_headers: bool,
    extra_params: &[String],
) -> Vec<String> {
    let mut argv = vec!["reta".to_string(), "-zeilen".to_string()];
    if let Some(section) = build_python_row_section_with_custom_oberesmaximum(
        row_specs,
        use_range,
        use_teiler,
        use_vielfache,
        invert,
        Some(1028),
    ) {
        argv.extend(section.tokens);
    } else {
        argv.push(another_oberesmaximum_from_row_specs_with_seed(&[], 1028));
    }
    argv.push("-spalten".to_string());
    argv.push("--bedeutung=primzahlkreuz".to_string());
    argv.push("-ausgabe".to_string());
    argv.push("--breite=0".to_string());
    if suppress_empty {
        argv.push("--keineleereninhalte".to_string());
    }
    if no_headers {
        argv.push("--keineueberschriften".to_string());
    }
    append_passthrough_params_to_reta_argv(&mut argv, extra_params);
    argv
}

fn append_python_special_prompt_calls(
    calls: &mut Vec<Vec<String>>,
    normalized: &[String],
    row_buckets: &PythonRowBuckets,
    use_range: bool,
    invert: bool,
    use_teiler: bool,
    use_vielfache: bool,
    suppress_empty: bool,
    no_headers: bool,
    extra_params: &[String],
) {
    if normalized.iter().any(|token| token == "groesse")
        && !row_buckets.primary_row_specs.is_empty()
    {
        let call = build_python_special_prompt_call(
            &row_buckets.primary_row_specs,
            use_range,
            invert,
            use_teiler,
            use_vielfache,
            suppress_empty,
            no_headers,
            "--strukturgroesse=strukturgroesse",
            Some("1,2"),
            extra_params,
        );
        if !calls.contains(&call) {
            calls.push(call);
        }

        if !row_buckets.reciprocal_row_specs.is_empty() {
            let reciprocal_call = build_python_special_prompt_call(
                &row_buckets.reciprocal_row_specs,
                use_range,
                invert,
                use_teiler,
                use_vielfache,
                suppress_empty,
                no_headers,
                "--strukturgroesse=strukturgroesse",
                Some("4"),
                extra_params,
            );
            if !calls.contains(&reciprocal_call) {
                calls.push(reciprocal_call);
            }
        }
    }

    if normalized.iter().any(|token| token == "mond") {
        calls.push(build_python_special_prompt_call(
            &row_buckets.primary_row_specs,
            use_range,
            invert,
            use_teiler,
            use_vielfache,
            suppress_empty,
            no_headers,
            "--bedeutung=gestirn",
            Some("3-6"),
            extra_params,
        ));
    }

    if normalized.iter().any(|token| token == "alles") {
        calls.push(build_python_special_prompt_call(
            &row_buckets.primary_row_specs,
            use_range,
            invert,
            use_teiler,
            use_vielfache,
            suppress_empty,
            no_headers,
            "--alles",
            None,
            extra_params,
        ));
    }

    if normalized.iter().any(|token| token == "primzahlkreuz") {
        calls.push(build_primzahlkreuz_prompt_call(
            &row_buckets.primary_row_specs,
            use_range,
            invert,
            use_teiler,
            use_vielfache,
            suppress_empty,
            no_headers,
            extra_params,
        ));
    }
}

fn build_fractional_prompt_row_section(
    row_specs: &[String],
    use_range: bool,
    invert: bool,
) -> Option<BuiltRowSection> {
    if row_specs.is_empty() {
        return None;
    }

    let joined_rows = row_specs
        .iter()
        .map(|spec| spec.trim())
        .filter(|spec| !spec.is_empty())
        .collect::<Vec<_>>()
        .join(",");
    if joined_rows.is_empty() {
        return None;
    }

    let prefix = if use_range {
        "--zaehlung="
    } else {
        "--vorhervonausschnitt="
    };

    let mut tokens = vec![format!("{prefix}{joined_rows}")];
    if invert {
        tokens.push("--invertieren".to_string());
    }
    Some(BuiltRowSection { tokens })
}

fn build_non_whole_fraction_semantic_call(
    row_specs: &[String],
    use_range: bool,
    invert: bool,
    suppress_empty: bool,
    no_headers: bool,
    para: &str,
    cols: &str,
    extra_params: &[String],
) -> Option<Vec<String>> {
    let mut argv = vec!["reta".to_string(), "-zeilen".to_string()];
    let section = build_fractional_prompt_row_section(row_specs, use_range, invert)?;
    argv.extend(section.tokens);
    argv.push("-spalten".to_string());
    argv.push(para.to_string());
    argv.push("-ausgabe".to_string());
    argv.push("--breite=0".to_string());
    argv.push(format!("--spaltenreihenfolgeundnurdiese={cols}"));
    if suppress_empty {
        argv.push("--keineleereninhalte".to_string());
    }
    if no_headers {
        argv.push("--keineueberschriften".to_string());
    }
    append_passthrough_params_to_reta_argv(&mut argv, extra_params);
    Some(argv)
}

fn build_non_whole_fraction_semantic_calls(
    spec: &PromptSemanticSpec,
    row_buckets: &PythonRowBuckets,
    use_range: bool,
    invert: bool,
    suppress_empty: bool,
    no_headers: bool,
    extra_params: &[String],
) -> Vec<Vec<String>> {
    let Some(main_parameter_prefix) = spec.non_whole_fraction_para else {
        return Vec::new();
    };

    let mut calls = Vec::new();

    for (denominator, numerators) in &row_buckets.non_whole_fraction_denominator_groups {
        let row_spec = numerators.join(",");
        if let Some(call) = build_non_whole_fraction_semantic_call(
            &[row_spec],
            use_range,
            invert,
            suppress_empty,
            no_headers,
            &format!("{main_parameter_prefix}{denominator}"),
            semantic_non_whole_fraction_normal_columns(spec),
            extra_params,
        ) {
            if !calls.contains(&call) {
                calls.push(call);
            }
        }
    }

    if spec.include_reverse_non_whole {
        for (numerator, denominators) in &row_buckets.non_whole_fraction_numerator_groups {
            let row_spec = denominators.join(",");
            if let Some(call) = build_non_whole_fraction_semantic_call(
                &[row_spec],
                use_range,
                invert,
                suppress_empty,
                no_headers,
                &format!("{main_parameter_prefix}{numerator}"),
                semantic_non_whole_fraction_reverse_columns(spec),
                extra_params,
            ) {
                if !calls.contains(&call) {
                    calls.push(call);
                }
            }
        }
    }

    calls
}

fn build_equal_fraction_semantic_call(
    spec: &PromptSemanticSpec,
    row_buckets: &PythonRowBuckets,
    use_range: bool,
    invert: bool,
    suppress_empty: bool,
    no_headers: bool,
    extra_params: &[String],
) -> Option<Vec<String>> {
    let para = spec.equal_fraction_para?;
    let cols = spec.equal_fraction_cols?;
    if row_buckets.equal_fraction_row_specs.is_empty() {
        return None;
    }

    let mut argv = vec!["reta".to_string(), "-zeilen".to_string()];
    let section = build_fractional_prompt_row_section(
        &row_buckets.equal_fraction_row_specs,
        use_range,
        invert,
    )?;
    argv.extend(section.tokens);
    argv.push("-spalten".to_string());
    argv.push(para.to_string());
    argv.push("-ausgabe".to_string());
    argv.push("--breite=0".to_string());
    argv.push(format!("--spaltenreihenfolgeundnurdiese={cols}"));
    if suppress_empty {
        argv.push("--keineleereninhalte".to_string());
    }
    if no_headers {
        argv.push("--keineueberschriften".to_string());
    }
    append_passthrough_params_to_reta_argv(&mut argv, extra_params);
    Some(argv)
}

fn build_single_semantic_call(
    spec: &PromptSemanticSpec,
    normalized: &[String],
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
        spec.reciprocal_whole_para.unwrap_or(spec.integer_para)
    } else {
        spec.integer_para
    };
    let cols = semantic_columns_for_spec(
        spec,
        normalized,
        reciprocal_kind,
        suppress_empty,
        no_headers,
    );

    Some(build_general_semantic_call(
        row_specs,
        use_range,
        invert,
        teiler,
        vielfache,
        suppress_empty,
        no_headers,
        para,
        (!cols.trim().is_empty()).then_some(cols.as_str()),
        extra_params,
        trailing_tokens,
    ))
}

fn build_reciprocal_concept_call(
    reciprocal_row_specs: &[String],
    use_range: bool,
    invert: bool,
    suppress_empty: bool,
    no_headers: bool,
    para: &str,
    extra_params: &[String],
    trailing_tokens: &[String],
) -> Vec<String> {
    let mut argv = vec!["reta".to_string(), "-zeilen".to_string()];

    if reciprocal_row_specs.is_empty() {
        argv.push("--vorhervonausschnitt=0".to_string());
        if invert {
            argv.push("--invertieren".to_string());
        }
    } else if let Some(section) =
        build_fractional_prompt_row_section(reciprocal_row_specs, use_range, invert)
    {
        argv.extend(section.tokens);
    }

    argv.push("-spalten".to_string());
    argv.push(para.to_string());
    argv.push("-ausgabe".to_string());
    argv.push("--breite=0".to_string());
    if suppress_empty {
        argv.push("--keineleereninhalte".to_string());
    }
    if no_headers {
        argv.push("--keineueberschriften".to_string());
    }
    append_passthrough_params_to_reta_argv(&mut argv, extra_params);
    for token in trailing_tokens {
        argv.push(token.clone());
    }
    argv
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
    let einzeln = normalized.iter().any(|t| t == "einzeln");
    let vielfache = !einzeln && normalized.iter().any(|t| t == "vielfache");
    let row_buckets = build_python_row_buckets_with_global_vielfache(&row_specs, vielfache);
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
                        &normalized,
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
                        &normalized,
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
                    calls.extend(build_non_whole_fraction_semantic_calls(
                        spec,
                        &row_buckets,
                        use_range,
                        invert,
                        suppress_empty,
                        no_headers,
                        &extra_params,
                    ));
                    if let Some(call) = build_equal_fraction_semantic_call(
                        spec,
                        &row_buckets,
                        use_range,
                        invert,
                        suppress_empty,
                        no_headers,
                        &extra_params,
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
    if !eig_r_values.is_empty() {
        let trailing = build_trailing_primary_zeilen_tokens(
            &row_buckets.primary_row_specs,
            use_range,
            teiler,
            vielfache,
        );
        calls.push(build_reciprocal_concept_call(
            &row_buckets.reciprocal_row_specs,
            use_range,
            invert,
            suppress_empty,
            no_headers,
            &format!("--konzept2={}", eig_r_values.join(",")),
            &extra_params,
            &trailing,
        ));
    }

    append_python_special_prompt_calls(
        &mut calls,
        &normalized,
        &row_buckets,
        use_range,
        invert,
        teiler,
        vielfache,
        suppress_empty,
        no_headers,
        &extra_params,
    );

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
    let einzeln = normalized.iter().any(|t| t == "einzeln");
    let vielfache = !einzeln && normalized.iter().any(|t| t == "vielfache");
    let row_buckets = build_python_row_buckets_with_global_vielfache(&row_specs, vielfache);
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

    append_passthrough_params_to_reta_argv(&mut argv, &passthrough_params);

    Some(argv)
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PreparedPromptBigOutput {
    pub tokens: Vec<String>,
    pub row_specs: Vec<String>,
    pub had_kurz_kurz: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct SelectivePromptRowInput {
    row_specs: Vec<String>,
    use_range: bool,
    use_teiler: bool,
    use_vielfache: bool,
    use_invertieren: bool,
    request_no_headers: bool,
    suppress_empty: bool,
    zeilen_passthrough: Vec<String>,
    spalten_passthrough: Vec<String>,
    kombination_passthrough: Vec<String>,
    ausgabe_passthrough: Vec<String>,
}

fn parse_selective_prompt_row_input(tokens: &[String]) -> Option<SelectivePromptRowInput> {
    let row_specs = tokens
        .iter()
        .filter(|token| is_row_spec_token(token))
        .cloned()
        .collect::<Vec<_>>();
    if row_specs.is_empty() {
        return None;
    }

    if tokens
        .iter()
        .any(|token| !is_row_spec_token(token) && !is_selective_reta_modifier(token))
    {
        return None;
    }

    let mut selective = SelectivePromptRowInput {
        row_specs,
        ..SelectivePromptRowInput::default()
    };

    for token in tokens {
        if is_row_spec_token(token) {
            continue;
        }

        match token.as_str() {
            "range" => selective.use_range = true,
            "teiler" => selective.use_teiler = true,
            "vielfache" => selective.use_vielfache = true,
            "invertieren" | "--invertieren" => selective.use_invertieren = true,
            "ee" | "--keineueberschriften" => selective.request_no_headers = true,
            "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar" | "--keineleereninhalte" => {
                selective.suppress_empty = true
            }
            "-zeilen" | "-spalten" | "-kombination" | "-ausgabe" => {}
            _ if is_zeilen_parameter_token(token) => {
                if is_conflicting_generated_zeilen_parameter_token(token) {
                    return None;
                }
                push_unique_string(&mut selective.zeilen_passthrough, token.clone());
            }
            _ if is_spalten_parameter_token(token) => {
                push_unique_string(&mut selective.spalten_passthrough, token.clone());
            }
            _ if is_kombination_parameter_token(token) => {
                push_unique_string(&mut selective.kombination_passthrough, token.clone());
            }
            _ if is_ausgabe_parameter_token(token) => {
                push_unique_string(&mut selective.ausgabe_passthrough, token.clone());
            }
            _ => return None,
        }
    }

    Some(selective)
}

fn selective_prompt_row_input_from_raw(
    input_tokens: &[String],
) -> Option<(SelectivePromptRowInput, bool)> {
    if input_tokens.is_empty() {
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

    parse_selective_prompt_row_input(&effective_input).map(|selective| (selective, had_kurz_kurz))
}

fn apply_selective_prompt_row_input_to_reta_tokens(
    reta_tokens: &[String],
    selective: &SelectivePromptRowInput,
) -> Option<Vec<String>> {
    if reta_tokens.first().map(String::as_str) != Some("reta") {
        return None;
    }

    let row_section = build_python_row_section(
        &selective.row_specs,
        selective.use_range,
        selective.use_teiler,
        selective.use_vielfache,
        selective.use_invertieren,
    )?;

    let mut new_zeilen_section = vec!["-zeilen".to_string()];
    new_zeilen_section.extend(row_section.tokens);
    for token in &selective.zeilen_passthrough {
        if !new_zeilen_section.contains(token) {
            new_zeilen_section.push(token.clone());
        }
    }

    let mut rebuilt = replace_main_section_tokens(reta_tokens, "-zeilen", &new_zeilen_section);
    merge_tokens_into_main_section(&mut rebuilt, "-spalten", &selective.spalten_passthrough);
    merge_tokens_into_main_section(
        &mut rebuilt,
        "-kombination",
        &selective.kombination_passthrough,
    );
    merge_tokens_into_main_section(&mut rebuilt, "-ausgabe", &selective.ausgabe_passthrough);

    if selective.request_no_headers {
        ensure_flag_in_main_section(&mut rebuilt, "-ausgabe", "--keineueberschriften");
    }
    if selective.suppress_empty {
        ensure_flag_in_main_section(&mut rebuilt, "-ausgabe", "--keineleereninhalte");
    }

    Some(rebuilt)
}

fn normalize_reta_like_tokens(tokens: &[String]) -> Option<Vec<String>> {
    if tokens.is_empty() {
        return None;
    }

    if tokens.first().map(String::as_str) == Some("reta") {
        return Some(tokens.to_vec());
    }
    if is_main_switch_token(&tokens[0]) {
        let mut argv = vec!["reta".to_string()];
        argv.extend(tokens.iter().cloned());
        return Some(argv);
    }

    None
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
    if stored_prefix_tokens.first().map(String::as_str) != Some("reta") {
        return None;
    }

    let (selective, had_kurz_kurz) = selective_prompt_row_input_from_raw(input_tokens)?;
    let rebuilt =
        apply_selective_prompt_row_input_to_reta_tokens(stored_prefix_tokens, &selective)?;

    Some(PreparedPromptBigOutput {
        tokens: rebuilt,
        row_specs: selective.row_specs,
        had_kurz_kurz,
    })
}

/// Gegenpfad zur Python-Architektur aus `verdreheWoReTaBefehl()`:
/// Wenn der gespeicherte Platzhalter nur aus Zeilen-/Modifier-Tokens besteht
/// und die aktuelle Eingabe ein roher `reta`-Befehl ist, wird die gespeicherte
/// Zeilen-Sektion in den neuen `reta`-Aufruf eingebaut statt hinten
/// drangehängt.
pub fn prepare_prompt_big_output_for_stored_rows(
    stored_prefix_tokens: &[String],
    input_tokens: &[String],
) -> Option<PreparedPromptBigOutput> {
    let selective = parse_selective_prompt_row_input(stored_prefix_tokens)?;
    let reta_tokens = normalize_reta_like_tokens(input_tokens)?;
    let rebuilt = apply_selective_prompt_row_input_to_reta_tokens(&reta_tokens, &selective)?;

    Some(PreparedPromptBigOutput {
        tokens: rebuilt,
        row_specs: selective.row_specs,
        had_kurz_kurz: false,
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
            | "ee"
            | "-zeilen"
            | "-spalten"
            | "-kombination"
            | "-ausgabe"
            | "--keineueberschriften"
            | "--keineleereninhalte"
            | "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
    ) || is_known_reta_parameter_token(token)
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

fn main_section_order_index(section: &str) -> Option<usize> {
    match section {
        "-zeilen" => Some(0),
        "-spalten" => Some(1),
        "-kombination" => Some(2),
        "-ausgabe" => Some(3),
        _ => None,
    }
}

fn insertion_index_for_missing_main_section(tokens: &[String], section: &str) -> usize {
    let Some(target_order) = main_section_order_index(section) else {
        return tokens.len();
    };

    for (index, token) in tokens.iter().enumerate() {
        if let Some(order) = main_section_order_index(token) {
            if order > target_order {
                return index;
            }
        }
    }

    tokens.len()
}

fn merge_tokens_into_main_section(tokens: &mut Vec<String>, section: &str, additions: &[String]) {
    if additions.is_empty() {
        return;
    }

    let mut pending = Vec::new();
    for token in additions {
        if !tokens.iter().any(|existing| existing == token)
            && !pending.iter().any(|existing| existing == token)
        {
            pending.push(token.clone());
        }
    }
    if pending.is_empty() {
        return;
    }

    let mut index = 0usize;
    while index < tokens.len() {
        if tokens[index] == section {
            let mut insert_at = index + 1;
            while insert_at < tokens.len() && !is_main_switch_token(&tokens[insert_at]) {
                insert_at += 1;
            }
            tokens.splice(insert_at..insert_at, pending);
            return;
        }
        index += 1;
    }

    let insert_at = insertion_index_for_missing_main_section(tokens, section);
    let mut inserted = Vec::with_capacity(1 + pending.len());
    inserted.push(section.to_string());
    inserted.extend(pending);
    tokens.splice(insert_at..insert_at, inserted);
}

fn ensure_flag_in_main_section(tokens: &mut Vec<String>, section: &str, flag: &str) {
    if tokens.iter().any(|token| token == flag) {
        return;
    }

    merge_tokens_into_main_section(tokens, section, &[flag.to_string()]);
}

fn has_main_section(tokens: &[String], section: &str) -> bool {
    tokens.iter().any(|token| token == section)
}

fn extract_main_section_payload(tokens: &[String], section: &str) -> Vec<String> {
    let mut index = 0usize;
    while index < tokens.len() {
        if tokens[index] == section {
            let mut payload = Vec::new();
            index += 1;
            while index < tokens.len() && !is_main_switch_token(&tokens[index]) {
                payload.push(tokens[index].clone());
                index += 1;
            }
            return payload;
        }
        index += 1;
    }
    Vec::new()
}

fn extract_non_section_tokens(tokens: &[String]) -> Vec<String> {
    if tokens.len() <= 1 {
        return Vec::new();
    }

    let mut out = Vec::new();
    let mut index = 1usize;
    while index < tokens.len() {
        if is_main_switch_token(&tokens[index]) {
            index += 1;
            while index < tokens.len() && !is_main_switch_token(&tokens[index]) {
                index += 1;
            }
            continue;
        }

        out.push(tokens[index].clone());
        index += 1;
    }

    out
}

fn merge_base_reta_tokens_into_overlay(
    base_tokens: &[String],
    overlay_tokens: &[String],
) -> Option<Vec<String>> {
    let base = normalize_reta_like_tokens(base_tokens)?;
    let overlay = normalize_reta_like_tokens(overlay_tokens)?;
    let mut merged = overlay;

    if !has_main_section(&merged, "-zeilen") {
        let payload = extract_main_section_payload(&base, "-zeilen");
        merge_tokens_into_main_section(&mut merged, "-zeilen", &payload);
    }

    for section in ["-spalten", "-kombination", "-ausgabe"] {
        let payload = extract_main_section_payload(&base, section);
        merge_tokens_into_main_section(&mut merged, section, &payload);
    }

    for token in extract_non_section_tokens(&base) {
        if token != "reta" && !merged.iter().any(|existing| existing == &token) {
            merged.push(token);
        }
    }

    Some(merged)
}

fn build_prompt_overlay_reta_calls(input_tokens: &[String]) -> Vec<Vec<String>> {
    if input_tokens.is_empty() || normalize_reta_like_tokens(input_tokens).is_some() {
        return Vec::new();
    }

    let calls = build_reta_calls_from_prompt_tokens(input_tokens);
    if !calls.is_empty() {
        return calls;
    }

    build_reta_argv_from_prompt_tokens(input_tokens)
        .into_iter()
        .collect()
}

/// Python-nahe Weiterfuehrung des gespeicherten `reta`-Platzhalter-Pfads:
/// ein gespeicherter roher `reta`-Befehl bleibt die Basisschicht, waehrend eine
/// neue Prompt-Eingabe zuerst selbst in einen oder mehrere `reta`-Aufrufe
/// ueberfuehrt und danach in die gespeicherte Basis eingemischt wird.
///
/// Dadurch uebernimmt die aktuelle Prompt-Eingabe ihre eigene `-zeilen`- und
/// Semantik-Struktur, waehrend gespeicherte Ausgabeflags oder Zusatzsektionen
/// erhalten bleiben. Das schliesst die bis dahin noch fehlende Luecke zwischen
/// reinem Selective-Row-Rewrite und blindem Token-Anhaengen.
pub fn prepare_prompt_big_output_for_stored_reta_prompt_overlay(
    stored_prefix_tokens: &[String],
    input_tokens: &[String],
) -> Option<Vec<Vec<String>>> {
    if stored_prefix_tokens.first().map(String::as_str) != Some("reta") {
        return None;
    }

    let overlay_calls = build_prompt_overlay_reta_calls(input_tokens);
    if overlay_calls.is_empty() {
        return None;
    }

    let mut merged_calls = Vec::new();
    for overlay in overlay_calls {
        let Some(merged) = merge_base_reta_tokens_into_overlay(stored_prefix_tokens, &overlay)
        else {
            continue;
        };
        if !merged_calls.contains(&merged) {
            merged_calls.push(merged);
        }
    }

    (!merged_calls.is_empty()).then_some(merged_calls)
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
        let mut expanded = python_row_spec_to_numbers(spec)?;
        numbers.append(&mut expanded);
    }
    Some(numbers)
}


fn rotate_python_trailing_prompt_prefix(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let chars = text.chars().collect::<Vec<_>>();
    let has_open_wrapper = chars.iter().any(|ch| matches!(ch, '(' | '[' | '{'));
    let contains_reta = text.contains("reta");
    let mut trailing_prefix_len = 0usize;

    for (reverse_index, ch) in chars.iter().rev().enumerate() {
        if ch.is_ascii_digit()
            || (matches!(ch, ')' | ']' | '}') && !contains_reta && has_open_wrapper)
        {
            trailing_prefix_len = reverse_index;
            break;
        }
    }

    if trailing_prefix_len == 0 || trailing_prefix_len >= chars.len() {
        return text.to_string();
    }

    let split_at = chars.len() - trailing_prefix_len;
    chars[split_at..]
        .iter()
        .chain(chars[..split_at].iter())
        .collect()
}

fn parse_prefix_and_numeric_suffix(text: &str) -> Option<(String, String)> {
    if text.is_empty() {
        return None;
    }

    let rotated = rotate_python_trailing_prompt_prefix(text);
    let chars: Vec<char> = rotated.chars().collect();
    let has_close_wrapper = chars.iter().any(|ch| matches!(ch, ')' | ']' | '}'));
    let contains_reta = rotated.contains("reta");
    let mut split_at: Option<usize> = None;

    for (i, ch) in chars.iter().enumerate() {
        if ch.is_ascii_digit()
            || (matches!(ch, '(' | '[' | '{') && !contains_reta && has_close_wrapper)
        {
            split_at = Some(i);
            break;
        }
    }

    let mut split_at = split_at?;
    if split_at == 0 {
        return None;
    }
    if split_at > 0 && chars[split_at - 1] == '-' {
        split_at -= 1;
    }
    if split_at == 0 {
        return None;
    }

    let prefix = chars[..split_at].iter().collect::<String>();
    let suffix = chars[split_at..].iter().collect::<String>();
    Some((prefix, suffix))
}

#[cfg(test)]
mod tests {
    use super::{
        build_reta_calls_from_prompt_tokens, expand_kurz_kurz_befehl,
        expand_python_regex_like_tokens, prepare_prompt_big_output_for_stored_reta,
        prepare_prompt_big_output_for_stored_reta_prompt_overlay,
        prepare_prompt_big_output_for_stored_rows, python_row_spec_to_numbers, PromptModus,
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
                "--oberesmaximum=1025",
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
                "--oberesmaximum=1025",
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
                "--vorhervonausschnitt=12-15,v12-15",
                "-spalten",
                "--impulse",
            ])
        );
    }

    #[test]
    fn stored_row_placeholder_injects_rows_into_raw_reta_command() {
        let prepared = prepare_prompt_big_output_for_stored_rows(
            &strings(&["12-15", "ee"]),
            &strings(&["reta", "-spalten", "--thomas"]),
        )
        .expect("stored row placeholder should be injected into reta command");

        assert_eq!(
            prepared.tokens,
            strings(&[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=12-15",
                "--oberesmaximum=1025",
                "-spalten",
                "--thomas",
                "-ausgabe",
                "--keineueberschriften",
            ])
        );
    }

    #[test]
    fn stored_reta_placeholder_merges_passthrough_sections_into_rewritten_command() {
        let prepared = prepare_prompt_big_output_for_stored_reta(
            &strings(&["reta", "-zeilen", "--zeit=heute", "-spalten", "--thomas"]),
            &strings(&[
                "12-15",
                "--zeit=morgen",
                "--geist",
                "--galaxie=Lebewesen",
                "--nocolor",
            ]),
        )
        .expect("stored reta placeholder should absorb extra reta parameters");

        assert_eq!(
            prepared.tokens,
            strings(&[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=12-15",
                "--oberesmaximum=1025",
                "--zeit=morgen",
                "-spalten",
                "--thomas",
                "--geist",
                "-kombination",
                "--galaxie=Lebewesen",
                "-ausgabe",
                "--nocolor",
            ])
        );
    }

    #[test]
    fn stored_row_placeholder_merges_passthrough_sections_into_raw_reta_command() {
        let prepared = prepare_prompt_big_output_for_stored_rows(
            &strings(&[
                "12-15",
                "ee",
                "--zeit=morgen",
                "--geist",
                "--galaxie=Lebewesen",
                "--nocolor",
            ]),
            &strings(&["reta", "-spalten", "--licht"]),
        )
        .expect("stored row placeholder should inject extra reta parameters too");

        assert_eq!(
            prepared.tokens,
            strings(&[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=12-15",
                "--oberesmaximum=1025",
                "--zeit=morgen",
                "-spalten",
                "--licht",
                "--geist",
                "-kombination",
                "--galaxie=Lebewesen",
                "-ausgabe",
                "--nocolor",
                "--keineueberschriften",
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
    fn prompt_execution_regex_supports_python_alternation_and_groups() {
        let expanded = expand_python_regex_like_tokens(&strings(&["r\"^(emotion|freiheit)$\""]));
        assert_eq!(expanded, strings(&["emotion", "freiheit"]));
    }

    #[test]
    fn prompt_execution_regex_supports_python_char_classes_and_plus() {
        let expanded = expand_python_regex_like_tokens(&strings(&["r\"^prim[0-9]+$\""]));
        assert_eq!(expanded, strings(&["prim24"]));
    }

    #[test]
    fn prompt_execution_regex_expands_reta_values_with_alternation() {
        let expanded = expand_python_regex_like_tokens(&strings(&[
            "reta",
            "-ausgabe",
            "--art=r\"^(html|csv)$\"",
        ]));
        assert_eq!(
            expanded,
            strings(&["reta", "-ausgabe", "--art=html,csv"])
        );
    }

    #[test]
    fn prompt_execution_regex_allows_double_dash_wildcard_parameter_name_like_python() {
        let expanded =
            expand_python_regex_like_tokens(&strings(&["reta", "-zeilen", "--=r\"heu.*\""]));
        assert_eq!(expanded, strings(&["reta", "-zeilen", "--zeit=heute"]));
    }

    #[test]
    fn prompt_execution_regex_collapses_repeated_equals_tokens_like_python() {
        let expanded = expand_python_regex_like_tokens(&strings(&[
            "reta",
            "-zeilen",
            "--zeit=r\"heu.*\"",
            "--zeit=morgen",
        ]));
        assert_eq!(expanded, strings(&["reta", "-zeilen", "--zeit=heute,morgen"]));
    }

    #[test]
    fn prompt_execution_regex_at_reta_root_does_not_expand_section_flags() {
        let expanded = expand_python_regex_like_tokens(&strings(&["reta", "r\"^end.*\""]));
        assert_eq!(expanded, strings(&["reta"]));
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

    #[test]
    fn semantic_call_sections_passthrough_params_python_like() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&[
            "emotion",
            "12",
            "--zeit=morgen",
            "--geist",
            "--galaxie=Lebewesen",
            "--nocolor",
        ]));
        assert_eq!(calls.len(), 1);
        let call = &calls[0];
        assert!(call.iter().any(|token| token == "--zeit=morgen"));
        assert!(call.iter().any(|token| token == "--geist"));
        assert!(call.iter().any(|token| token == "--galaxie=Lebewesen"));
        assert!(call.iter().any(|token| token == "--nocolor"));
        assert_eq!(
            call.iter().position(|token| token == "--zeit=morgen"),
            Some(4)
        );
        assert_eq!(call.iter().position(|token| token == "--geist"), Some(7));
        assert_eq!(
            call.iter().position(|token| token == "-kombination"),
            Some(8)
        );
        assert_eq!(
            call.iter().position(|token| token == "--galaxie=Lebewesen"),
            Some(9)
        );
        assert_eq!(call.iter().position(|token| token == "-ausgabe"), Some(10));
        assert_eq!(call.iter().position(|token| token == "--nocolor"), Some(13));
    }

    #[test]
    fn fractional_emotion_builds_python_oriented_gebrochenemotion_call() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["emotion", "2/3"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0].iter().any(|token| token == "--gebrochenemotion=2"));
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=3"));
        assert!(calls[0]
            .iter()
            .any(|token| token == "--spaltenreihenfolgeundnurdiese=2"));
    }

    #[test]
    fn universum_fraction_uses_python_normal_fraction_mapping() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["universum", "2/3"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--gebrochenuniversum=2"));
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=3"));
        assert!(calls[0]
            .iter()
            .any(|token| token == "--spaltenreihenfolgeundnurdiese=2"));
        assert!(!calls[0]
            .iter()
            .any(|token| token == "--gebrochenuniversum=3"));
    }

    #[test]
    fn universum_equal_fraction_range_emits_verhaeltnisgleicherzahl_call() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["universum", "2-4/2-4"]));
        assert!(calls.iter().any(|call| call
            .iter()
            .any(|token| token == "--universum=verhaeltnisgleicherzahl")));
        assert!(calls.iter().any(|call| call
            .iter()
            .any(|token| token == "--vorhervonausschnitt=2,3,4")));
    }

    #[test]
    fn fraction_rectangle_expands_like_python_create_ranges_for_bruch_lists() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["emotion", "1/2-3/3"]));
        assert!(calls.iter().any(|call| call
            .iter()
            .any(|token| token == "--grundstrukturen=emotion")));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenemotion=2")));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenemotion=3")));
        assert!(calls.iter().any(|call| call
            .iter()
            .any(|token| token == "--vorhervonausschnitt=2,3")));
    }

    #[test]
    fn fraction_distance_expands_like_python_prompt_examples() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["absicht", "4/5+2/2"]));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochengalaxie=2")));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochengalaxie=6")));
        assert!(calls.iter().any(|call| call
            .iter()
            .any(|token| token == "--vorhervonausschnitt=3,7")));
        assert!(!calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochengalaxie=3")));
    }

    #[test]
    fn fraction_mixed_range_and_distance_uses_python_bruch_spalt_pipeline() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["emotion", "1/2-3/4+5/6"]));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenemotion=6")));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenemotion=7")));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenemotion=8")));
        assert!(calls.iter().any(|call| call
            .iter()
            .any(|token| token == "--vorhervonausschnitt=8,9,10")));
        assert!(!calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenemotion=2")));
    }

    #[test]
    fn descending_fraction_range_keeps_python_empty_range_semantics() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["emotion", "3/4-1/2"]));
        assert!(calls.is_empty());
    }

    #[test]
    fn fraction_subtraction_removes_python_fraction_group_values() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["emotion", "-2/3,2/5"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0].iter().any(|token| token == "--gebrochenemotion=2"));
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=5"));
        assert!(!calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=3"));
    }

    #[test]
    fn fraction_v_minus_prefix_subtracts_like_python_bruch_management() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["emotion", "v1/2,v-1/2"]));
        assert!(calls.is_empty());
    }

    #[test]
    fn repeated_denominator_fraction_range_uses_python_reverse_mapping() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["emotion", "2/5-3/5"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--gebrochenemotion=5"));
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=2,3"));
        assert!(calls[0]
            .iter()
            .any(|token| token == "--spaltenreihenfolgeundnurdiese=1"));
        assert!(!calls[0]
            .iter()
            .any(|token| token == "--gebrochenemotion=2"));
        assert!(!calls[0]
            .iter()
            .any(|token| token == "--gebrochenemotion=3"));
    }

    #[test]
    fn fraction_global_vielfache_expands_non_whole_numerators_like_python() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&[
            "emotion",
            "vielfache",
            "2/3",
            "-4/3",
        ]));
        assert!(calls.iter().any(|call| call
            .iter()
            .any(|token| token == "--gebrochenemotion=3")
            && call
                .iter()
                .any(|token| token == "--vorhervonausschnitt=2,4,6,8,10,12,14,16,18,20,22")
            && call
                .iter()
                .any(|token| token == "--spaltenreihenfolgeundnurdiese=1")));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenemotion=6")));
        assert!(!calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenemotion=2")));
        assert!(!calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenemotion=4")));
        assert!(!calls
            .iter()
            .flat_map(|call| call.iter())
            .any(|token| token.contains("1026")));
    }

    #[test]
    fn fraction_global_vielfache_adds_integer_side_effect_rows_like_python() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["emotion", "vielfache", "2/3"]));
        assert!(calls.iter().any(|call| call
            .iter()
            .any(|token| token == "--grundstrukturen=emotion")
            && call.iter().any(|token| {
                token.starts_with("--vielfachevonzahlen=") && token.contains('2')
            })));
    }

    #[test]
    fn fraction_global_vielfache_adds_equal_fraction_side_effect_like_python() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["universum", "vielfache", "2/3"]));
        assert!(calls.iter().any(|call| call
            .iter()
            .any(|token| token == "--universum=verhaeltnisgleicherzahl")));
    }

    #[test]
    fn python_fraction_allowed_numbers_remove_python_sentinel_maximum() {
        let allowed = python_fraction_allowed_numbers();
        assert!(allowed.contains(&22));
        assert!(!allowed.contains(&23));
    }

    #[test]
    fn repeated_denominator_range_with_equal_fraction_keeps_side_effects() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["universum", "3/5-5/5"]));
        assert!(calls.iter().any(|call| call
            .iter()
            .any(|token| token == "--gebrochenuniversum=5")
            && call
                .iter()
                .any(|token| token == "--vorhervonausschnitt=3,4,5")
            && call
                .iter()
                .any(|token| token == "--spaltenreihenfolgeundnurdiese=1")));
        assert!(calls.iter().any(|call| call
            .iter()
            .any(|token| token == "--universum=verhaeltnisgleicherzahl")
            && call
                .iter()
                .any(|token| token == "--vorhervonausschnitt=5")));
    }

    #[test]
    fn einzeln_disables_python_vielfache_row_rewrite() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&[
            "emotion",
            "12",
            "vielfache",
            "einzeln",
        ]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=12"));
        assert!(!calls[0]
            .iter()
            .any(|token| token.starts_with("--vielfachevonzahlen=")));
    }

    #[test]
    fn stored_reta_placeholder_merges_generated_prompt_call_sections() {
        let calls = prepare_prompt_big_output_for_stored_reta_prompt_overlay(
            &strings(&["reta", "-ausgabe", "--nocolor"]),
            &strings(&["emotion", "12"]),
        )
        .expect("stored reta base should absorb prompt-generated semantic call");

        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0],
            strings(&[
                "reta",
                "-zeilen",
                "--vorhervonausschnitt=12",
                "--oberesmaximum=1025",
                "-spalten",
                "--grundstrukturen=emotion",
                "-ausgabe",
                "--breite=0",
                "--spaltenreihenfolgeundnurdiese=2,3",
                "--nocolor",
            ])
        );
    }

    #[test]
    fn stored_reta_placeholder_merges_generated_prompt_batches() {
        let calls = prepare_prompt_big_output_for_stored_reta_prompt_overlay(
            &strings(&["reta", "-ausgabe", "--nocolor"]),
            &strings(&["universum", "2/3"]),
        )
        .expect("stored reta base should absorb batched prompt-generated calls too");

        assert_eq!(calls.len(), 1);
        assert!(calls
            .iter()
            .all(|call| call.iter().any(|token| token == "--nocolor")));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenuniversum=2")));
        assert!(!calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenuniversum=3")));
    }

    #[test]
    fn mond_command_builds_python_bedeutung_call() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["mond", "12"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0].iter().any(|token| token == "--bedeutung=gestirn"));
        assert!(calls[0]
            .iter()
            .any(|token| token == "--spaltenreihenfolgeundnurdiese=3-6"));
    }

    #[test]
    fn alles_command_builds_python_alles_call() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["alles", "12"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0].iter().any(|token| token == "--alles"));
        assert!(!calls[0]
            .iter()
            .any(|token| token.starts_with("--spaltenreihenfolgeundnurdiese=")));
    }

    #[test]
    fn primzahlkreuz_command_uses_python_upper_bound_seed() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["primzahlkreuz", "12"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--bedeutung=primzahlkreuz"));
        assert!(calls[0]
            .iter()
            .any(|token| token == "--oberesmaximum=1029"));
    }

    #[test]
    fn groesse_command_emits_second_python_reta_execute_call() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["groesse", "12"]));
        assert_eq!(calls.len(), 2);
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--strukturgroesse=organisation")));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--strukturgroesse=strukturgroesse")
                && call
                    .iter()
                    .any(|token| token == "--spaltenreihenfolgeundnurdiese=1,2")));
    }

    #[test]
    fn richtung_command_keeps_python_style_without_column_filter() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["richtung", "12"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--primzahlwirkung=galaxieabsicht"));
        assert!(!calls[0]
            .iter()
            .any(|token| token.starts_with("--spaltenreihenfolgeundnurdiese=")));
    }

    #[test]
    fn trailing_short_commands_after_rows_are_rotated_like_python() {
        let (_, expanded) = expand_kurz_kurz_befehl(PromptModus::Normal, &strings(&["12at"]));
        assert_eq!(expanded, strings(&["a", "t", "12"]));
        let calls = build_reta_calls_from_prompt_tokens(&expanded);
        assert_eq!(calls.len(), 2);
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--menschliches=motivation")));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--galaxie=thomas")));
        assert!(calls.iter().all(|call| call
            .iter()
            .any(|token| token == "--vorhervonausschnitt=12")));
    }

    #[test]
    fn trailing_short_commands_after_fraction_are_rotated_like_python() {
        let (_, expanded) = expand_kurz_kurz_befehl(PromptModus::Normal, &strings(&["2/3u"]));
        assert_eq!(expanded, strings(&["u", "2/3"]));
        let calls = build_reta_calls_from_prompt_tokens(&expanded);
        assert!(!calls.is_empty());
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenuniversum=2")));
        assert!(!calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenuniversum=3")));
    }

    #[test]
    fn reciprocal_concept_runs_for_integer_rows_like_python() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["12", "EIGRweisheit"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--konzept2=weisheit"));
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=0"));
        assert!(calls[0]
            .windows(2)
            .any(|window| window[0] == "-zeilen" && window[1] == "--vorhervonausschnitt=12"));
    }

    #[test]
    fn python_row_ranges_accept_offsets_and_inline_vielfache() {
        assert_eq!(python_row_spec_to_numbers("3+1"), Some(vec![2, 4]));
        assert_eq!(
            python_row_spec_to_numbers("10-12+2"),
            Some(vec![8, 9, 10, 12, 13, 14])
        );

        let multiples = python_row_spec_to_numbers("v12").expect("v-prefix expands multiples");
        assert!(multiples.contains(&12));
        assert!(multiples.contains(&24));
        assert!(multiples.contains(&1020));
        assert!(!multiples.contains(&1032));
    }

    #[test]
    fn prompt_build_uses_python_row_range_expansion() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["3+1", "thomas"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=2,4"));
    }

    #[test]
    fn python_generator_rows_expand_like_py_retaprompt() {
        assert_eq!(
            python_row_spec_to_numbers("{n*2+1 for n in range(3)}"),
            Some(vec![1, 3, 5])
        );
        let calls = build_reta_calls_from_prompt_tokens(&strings(&[
            "emotion",
            "{n*2+1 for n in range(3)}",
        ]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=1,3,5"));
    }

    #[test]
    fn python_generated_row_deletion_matches_bereich_to_numbers2() {
        assert_eq!(
            python_row_spec_to_numbers("-[5 * n for n in range(5)],19-21"),
            Some(vec![19, 21])
        );
        let calls = build_reta_calls_from_prompt_tokens(&strings(&[
            "emotion",
            "-[5 * n for n in range(5)],19-21",
        ]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=19,21"));
    }

    #[test]
    fn python_calculation_row_list_is_accepted() {
        assert_eq!(python_row_spec_to_numbers("[2*3]"), Some(vec![6]));
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["emotion", "[2*3]"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=6"));
    }

    #[test]
    fn python_generator_rows_accept_literal_iterable_sources() {
        assert_eq!(
            python_row_spec_to_numbers("[n * 3 for n in [1,2,3] if n != 2]"),
            Some(vec![3, 9])
        );
        let calls = build_reta_calls_from_prompt_tokens(&strings(&[
            "emotion",
            "{n + 10 for n in {1,2,3} if n >= 2}",
        ]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=12,13"));
    }

    #[test]
    fn python_generator_rows_support_membership_filters_and_simple_builtins() {
        assert_eq!(
            python_row_spec_to_numbers("(n for n in (1,2,3,4) if n not in (2,4))"),
            Some(vec![1, 3])
        );
        assert_eq!(
            python_row_spec_to_numbers("[abs(n-3) for n in range(1,6) if n in {1,3,5}]"),
            Some(vec![0, 2])
        );
    }

    #[test]
    fn python_generator_rows_accept_builtin_iterable_wrappers_like_eval() {
        assert_eq!(
            python_row_spec_to_numbers("[n for n in list(range(1,5)) if n in set([1,3,4])]"),
            Some(vec![1, 3, 4])
        );
        assert_eq!(
            python_row_spec_to_numbers("{n * 2 for n in sorted({3,1,2})}"),
            Some(vec![2, 4, 6])
        );
    }

    #[test]
    fn python_generator_rows_accept_eval_style_iterable_unions_and_concats() {
        assert_eq!(
            python_row_spec_to_numbers("[n for n in ([1,2] + [3,4]) if n >= 2]"),
            Some(vec![2, 3, 4])
        );
        assert_eq!(
            python_row_spec_to_numbers("[n for n in ({1,2} | {2,5}) if n != 1]"),
            Some(vec![2, 5])
        );
    }

    #[test]
    fn fraction_inline_vielfache_uses_python_gebrochen_allowed_numbers() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["emotion", "v2/3"]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--gebrochenemotion=2"));
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=3,6,9,12,15,18,21"));
        assert!(!calls[0].iter().any(|token| token.contains("1026")));
    }

    #[test]
    fn fraction_global_vielfache_expands_numerator_like_python() {
        let calls = build_reta_calls_from_prompt_tokens(&strings(&["emotion", "2/3", "vielfache"]));
        assert!(!calls.is_empty());
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenemotion=3")));
        assert!(calls.iter().any(|call| call
            .iter()
            .any(|token| token == "--vorhervonausschnitt=2,4,6,8,10,12,14,16,18,20,22")));
        assert!(!calls
            .iter()
            .flat_map(|call| call.iter())
            .any(|token| token.contains("1026")));
    }

}
