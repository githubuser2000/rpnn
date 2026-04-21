use std::collections::{BTreeMap, BTreeSet};

use indexmap::IndexMap;
use std::sync::OnceLock;

use crate::shared::words_py::PyValue;
use crate::domain::python_source_of_truth::{
    all_main_alias_groups, parameter_alias_groups_for_main,
};
use crate::shared_words;

use super::semantic_choices::{
    semantic_wahl15_ordered_keys, semantic_wahl16_ordered_keys, semantic_wahl15_value,
    semantic_wahl16_value, RETAPROMPT_AUSGABE_ART_PARAMETER, RETAPROMPT_AUSGABE_ART_VALUES,
    RETAPROMPT_AUSGABE_REGEX_PARAMETERS, RETAPROMPT_KOMBINATION_GALAXIE_PARAMETER,
    RETAPROMPT_KOMBINATION_UNIVERSUM_PARAMETER, RETAPROMPT_RETA_MAIN_SWITCHES,
    RETAPROMPT_RETA_SECTION_SWITCHES, RETAPROMPT_ZEILEN_PRIMZAHLEN_PARAMETER,
    RETAPROMPT_ZEILEN_PRIMZAHLEN_VALUES, RETAPROMPT_ZEILEN_REGEX_PARAMETERS,
    RETAPROMPT_ZEILEN_TYP_PARAMETER, RETAPROMPT_ZEILEN_TYP_VALUES,
    RETAPROMPT_ZEILEN_ZEIT_PARAMETER, RETAPROMPT_ZEILEN_ZEIT_VALUES,
};

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
    // Python `is15or16command` accepts the naked `16_15` branch even though
    // the generated `befehle` inventory mostly contains `16_15_<wahl15-key>`.
    befehle.push("16_15".to_string());
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

fn reta_main_switch_tokens_for_regex() -> &'static [&'static str] {
    // Python regExReplace iteriert ueber i18n.hauptForNeben.values(), nicht nur
    // ueber die vier datenfuehrenden Abschnitte. Deshalb gehoeren -h, -help,
    // -debug und -nichts in die Regex-Expansion, auch wenn nur die ersten vier
    // Abschnitte Nebenparameter-Inventare haben.
    RETAPROMPT_RETA_MAIN_SWITCHES
}

fn reta_section_switch_tokens_for_regex() -> &'static [&'static str] {
    RETAPROMPT_RETA_SECTION_SWITCHES
}

fn zeilen_parameter_inventory_for_regex() -> BTreeMap<String, Vec<String>> {
    let mut inventory = BTreeMap::new();

    // Python regExReplace baut fuer -zeilen zuerst
    // {zeilenPara: {''} for zeilenPara in i18n.haupt2neben['zeilen']}
    // und ueberschreibt nur zeit/typ/primzahlen mit echten Wertemengen.
    // Numeric-Parameter wie zaehlung oder oberesmaximum haben dort keine
    // Zahlenliste; ein Regex auf deren RHS ergibt Python-artig den Flag-Token.
    for key in RETAPROMPT_ZEILEN_REGEX_PARAMETERS {
        inventory.insert((*key).to_string(), Vec::new());
    }

    inventory.insert(
        RETAPROMPT_ZEILEN_ZEIT_PARAMETER.to_string(),
        RETAPROMPT_ZEILEN_ZEIT_VALUES
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    );
    inventory.insert(
        RETAPROMPT_ZEILEN_TYP_PARAMETER.to_string(),
        RETAPROMPT_ZEILEN_TYP_VALUES
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    );
    inventory.insert(
        RETAPROMPT_ZEILEN_PRIMZAHLEN_PARAMETER.to_string(),
        RETAPROMPT_ZEILEN_PRIMZAHLEN_VALUES
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    );

    inventory
}

fn ausgabe_parameter_inventory_for_regex() -> BTreeMap<String, Vec<String>> {
    let mut inventory = BTreeMap::new();

    // Python regExReplace gibt nur --art echte Werte aus i18n.ausgabeArt.
    // breite/breiten sind in der Completion numerisch, in der Regex-Expansion
    // aber wie Python {''}-Parameter ohne generierte Zahlenliste.
    for key in RETAPROMPT_AUSGABE_REGEX_PARAMETERS {
        inventory.insert((*key).to_string(), Vec::new());
    }

    inventory.insert(
        RETAPROMPT_AUSGABE_ART_PARAMETER.to_string(),
        RETAPROMPT_AUSGABE_ART_VALUES
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    );

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
    inventory.insert(RETAPROMPT_KOMBINATION_GALAXIE_PARAMETER.to_string(), galaxie);

    let mut universum = Vec::new();
    let mut universum_seen = BTreeSet::new();
    for values in words.kombiParaNdataMatrix2.values() {
        for value in values {
            push_unique_preserving_normalized(&mut universum, &mut universum_seen, value.clone());
        }
    }
    inventory.insert(RETAPROMPT_KOMBINATION_UNIVERSUM_PARAMETER.to_string(), universum);
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
    for section in reta_section_switch_tokens_for_regex() {
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

pub fn regex_like_search(pattern: &str, text: &str) -> bool {
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
    let rhs_contains_regex_or_glob = right_pieces
        .iter()
        .any(|piece| parse_special_fragment_matcher(piece.trim()).is_some());
    let all_selected_parameters_are_value_less = parameter_names.iter().all(|parameter| {
        inventory
            .get(parameter)
            .map(|values| values.is_empty())
            .unwrap_or(true)
    });

    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for parameter in parameter_names {
        let allowed_values = inventory.get(&parameter).cloned().unwrap_or_default();
        let values = expand_rhs_regex_pieces(&right_pieces, &allowed_values);
        if values.is_empty() {
            if allowed_values.is_empty()
                && (!rhs_contains_regex_or_glob || all_selected_parameters_are_value_less)
            {
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
        return rest.is_empty() || semantic_wahl15_value(rest).is_some();
    }
    if let Some(rest) = text.strip_prefix("16_") {
        if rest.is_empty() || semantic_wahl16_value(rest).is_some() {
            return true;
        }
        if rest == "15" {
            return true;
        }
        if let Some(rest15) = text.strip_prefix("16_15_") {
            return semantic_wahl15_value(rest15).is_some();
        }
    }
    false
}

pub fn libreta_prompt_custom_split(text: &str) -> Vec<String> {
    let mut stack: Vec<char> = Vec::new();
    let mut result: Vec<String> = Vec::new();
    let mut start = 0usize;

    for (idx, ch) in text.char_indices() {
        if matches!(ch, '(' | '{' | '[') {
            stack.push(ch);
        } else if matches!(ch, ')' | '}' | ']') {
            if !stack.is_empty() {
                stack.pop();
            }
        } else if ch.is_whitespace() && stack.is_empty() {
            result.push(text[start..idx].to_string());
            start = idx + ch.len_utf8();
        }
    }

    if start < text.len() {
        result.push(text[start..].to_string());
    }
    result
}

pub fn libreta_prompt_custom_split2(input_string: &str, delimiter: char) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut temp = String::new();
    let mut stack: Vec<char> = Vec::new();

    for ch in input_string.chars() {
        if matches!(ch, '(' | '{' | '[') {
            stack.push('(');
            temp.push(ch);
        } else if matches!(ch, ')' | '}' | ']') {
            if stack.last().map(|last| "({[".contains(*last)).unwrap_or(false) {
                stack.pop();
                temp.push(ch);
            } else {
                temp.push(ch);
            }
        } else if ch == delimiter && stack.is_empty() {
            result.push(temp);
            temp = String::new();
        } else {
            temp.push(ch);
        }
    }

    if !temp.is_empty() {
        result.push(temp);
    }
    result
}

pub fn custom_split_whitespace_parenthesized(text: &str) -> Vec<String> {
    libreta_prompt_custom_split(text)
        .into_iter()
        .map(|piece| piece.trim().to_string())
        .filter(|piece| !piece.is_empty())
        .collect()
}

pub fn custom_split_delim_parenthesized(text: &str, delim: char) -> Vec<String> {
    libreta_prompt_custom_split2(text, delim)
        .into_iter()
        .map(|piece| piece.trim().to_string())
        .filter(|piece| !piece.is_empty())
        .collect()
}

fn split_kpattern_comma_here(tail_after_comma: &str) -> bool {
    for ch in tail_after_comma.chars() {
        if matches!(ch, ']' | '}' | ')') {
            return false;
        }
        if matches!(ch, '[' | '{' | '(') {
            return true;
        }
    }
    true
}

/// Python `center.kpattern`: r",(?![^\[\]\{\}\(\)]*[\]\}\)])".
///
/// Unlike `libreta_prompt_custom_split2`, this keeps Python's trailing empty
/// split element and intentionally follows the regex look-ahead rather than a
/// balanced-bracket stack.  `LibRetaPrompt.verifyBruchNganzZahlCommaList` uses
/// exactly this splitter.
pub fn libreta_prompt_split_kpattern_commas_py(input_string: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0usize;

    for (idx, ch) in input_string.char_indices() {
        if ch == ',' && split_kpattern_comma_here(&input_string[idx + ch.len_utf8()..]) {
            result.push(input_string[start..idx].to_string());
            start = idx + ch.len_utf8();
        }
    }

    result.push(input_string[start..].to_string());
    result
}

fn ascii_digit_run(chars: &[char], pos: &mut usize) -> bool {
    let start = *pos;
    while *pos < chars.len() && chars[*pos].is_ascii_digit() {
        *pos += 1;
    }
    *pos > start
}

fn python_between_commas_integer_range_shape(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return false;
    }

    let mut pos = 0usize;
    if chars.get(pos) == Some(&'v') {
        pos += 1;
    }
    if chars.get(pos) == Some(&'-') {
        pos += 1;
    }
    if !ascii_digit_run(&chars, &mut pos) {
        return false;
    }

    if chars.get(pos) == Some(&'-') {
        pos += 1;
        if !ascii_digit_run(&chars, &mut pos) {
            return false;
        }
    }

    while chars.get(pos) == Some(&'+') {
        pos += 1;
        if !ascii_digit_run(&chars, &mut pos) {
            return false;
        }
    }

    pos == chars.len()
}

#[allow(non_snake_case)]
pub fn isZeilenAngabe_betweenKommas(g: &str) -> bool {
    if python_between_commas_integer_range_shape(g) {
        return true;
    }
    if parse_python_str_as_generator_values(g).is_some() {
        return true;
    }

    let mut chars = g.chars();
    if chars.next().is_none() {
        return false;
    }
    let without_first = chars.collect::<String>();
    parse_python_str_as_generator_values(&without_first).is_some()
}

pub fn is_zeilen_angabe_between_kommas_py(g: &str) -> bool {
    isZeilenAngabe_betweenKommas(g)
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VerifyBruchGanzZahlBetweenCommasResult {
    pub bruchAndGanzZahlEtwaKorrekterBereich: Vec<bool>,
    pub bruchBereichsAngaben: Vec<String>,
    pub bruchRanges: Vec<Vec<String>>,
    pub zahlenAngaben_: Vec<String>,
    pub bruchAndGanzZahlEtwaKorrekterBereichAllTrue: bool,
}

#[allow(non_snake_case)]
pub fn verifyBruchNganzZahlBetweenCommas(
    mut bruchAndGanzZahlEtwaKorrekterBereich: Vec<bool>,
    bruchBereichsAngabe: &str,
    mut bruchBereichsAngaben: Vec<String>,
    bruchRange: Vec<String>,
    mut bruchRanges: Vec<Vec<String>>,
    etwaBruch: &str,
    mut zahlenAngaben_: Vec<String>,
) -> VerifyBruchGanzZahlBetweenCommasResult {
    let isBruch = isZeilenAngabe_betweenKommas(bruchBereichsAngabe);
    let isGanzZahl = isZeilenAngabe_betweenKommas(etwaBruch);

    if isBruch != isGanzZahl {
        bruchAndGanzZahlEtwaKorrekterBereich.push(true);
        if isBruch {
            bruchRanges.push(bruchRange);
            bruchBereichsAngaben.push(bruchBereichsAngabe.to_string());
        } else if isGanzZahl {
            zahlenAngaben_.push(etwaBruch.to_string());
        }
    } else {
        bruchAndGanzZahlEtwaKorrekterBereich.push(false);
    }

    let bruchAndGanzZahlEtwaKorrekterBereichAllTrue =
        bruchAndGanzZahlEtwaKorrekterBereich.iter().all(|value| *value);

    VerifyBruchGanzZahlBetweenCommasResult {
        bruchAndGanzZahlEtwaKorrekterBereich,
        bruchBereichsAngaben,
        bruchRanges,
        zahlenAngaben_,
        bruchAndGanzZahlEtwaKorrekterBereichAllTrue,
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VerifyBruchGanzZahlCommaListResult {
    pub bruchAndGanzZahlEtwaKorrekterBereich: Vec<Vec<bool>>,
    pub bruchBereichsAngaben: Vec<Vec<String>>,
    pub bruchRanges: Vec<Vec<Vec<String>>>,
    pub zahlenAngaben_: Vec<Vec<String>>,
    pub fullBlockIsZahlenbereichAndBruch_Z: bool,
}

#[allow(non_snake_case)]
pub fn verifyBruchNganzZahlCommaList(
    mut bruchAndGanzZahlEtwaKorrekterBereich1: Vec<bool>,
    bruchBereichsAngabe: &str,
    mut bruchBereichsAngaben1: Vec<String>,
    bruchRange: Vec<String>,
    mut bruchRanges1: Vec<Vec<String>>,
    commaListe: &str,
    mut zahlenAngaben_1: Vec<String>,
) -> VerifyBruchGanzZahlCommaListResult {
    let mut split_count = 0usize;

    for etwaBruch in libreta_prompt_split_kpattern_commas_py(commaListe) {
        split_count += 1;
        let verified = verifyBruchNganzZahlBetweenCommas(
            bruchAndGanzZahlEtwaKorrekterBereich1,
            bruchBereichsAngabe,
            bruchBereichsAngaben1,
            bruchRange.clone(),
            bruchRanges1,
            &etwaBruch,
            zahlenAngaben_1,
        );
        bruchAndGanzZahlEtwaKorrekterBereich1 =
            verified.bruchAndGanzZahlEtwaKorrekterBereich;
        bruchBereichsAngaben1 = verified.bruchBereichsAngaben;
        bruchRanges1 = verified.bruchRanges;
        zahlenAngaben_1 = verified.zahlenAngaben_;
    }

    // Python appends the *same mutated list objects* into the outer result on
    // every iteration.  After the loop all outer slots therefore render as the
    // final accumulated inner list, not as incremental snapshots.
    let bruchAndGanzZahlEtwaKorrekterBereich =
        vec![bruchAndGanzZahlEtwaKorrekterBereich1.clone(); split_count];
    let bruchBereichsAngaben = vec![bruchBereichsAngaben1.clone(); split_count];
    let bruchRanges = vec![bruchRanges1.clone(); split_count];
    let zahlenAngaben_ = vec![zahlenAngaben_1.clone(); split_count];

    // Python calls `all()` on a list of lists here.  Non-empty inner lists are
    // truthy even when they contain `False`, so this deliberately does not fold
    // the contained booleans.
    let fullBlockIsZahlenbereichAndBruch_Z = bruchAndGanzZahlEtwaKorrekterBereich
        .iter()
        .all(|entry| !entry.is_empty());

    VerifyBruchGanzZahlCommaListResult {
        bruchAndGanzZahlEtwaKorrekterBereich,
        bruchBereichsAngaben,
        bruchRanges,
        zahlenAngaben_,
        fullBlockIsZahlenbereichAndBruch_Z,
    }
}


#[allow(non_snake_case)]
pub fn is15or16command(text: &str) -> bool {
    is_15or16_command(text)
}

#[allow(non_snake_case)]
pub fn isReTaParameter(t: &str) -> bool {
    if t.is_empty() || !t.starts_with('-') || looks_like_numeric_or_fraction_range(t) {
        return false;
    }

    let token_without_value = t.split_once('=').map(|(head, _)| head).unwrap_or(t);
    if reta_main_switch_tokens_for_regex()
        .iter()
        .any(|candidate| *candidate == token_without_value)
    {
        return true;
    }

    if matches!(token_without_value, "--" | "--*") {
        return true;
    }

    parameter_token_base(token_without_value)
        .map(|base| reta_global_parameter_inventory_for_regex().contains_key(base))
        .unwrap_or(false)
}

pub fn verkuerze_dict(dictionary: &[(String, String)]) -> Vec<(String, String)> {
    let mut dict2: Vec<(String, String)> = Vec::new();
    for (key, value) in dictionary {
        if !dict2.iter().any(|(_, existing_value)| existing_value == value) {
            dict2.push((key.clone(), value.clone()));
        }
    }
    dict2
}

pub fn looks_like_single_numeric_or_fraction_part(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return false;
    }
    if !trimmed.contains('/') && python_row_piece_is_integer_like(trimmed) {
        return true;
    }
    if trimmed
        .chars()
        .any(|ch| matches!(ch, '(' | ')' | '[' | ']' | '{' | '}'))
    {
        return false;
    }
    if trimmed.contains(',') {
        return custom_split_delim_parenthesized(trimmed, ',')
            .into_iter()
            .all(|piece| looks_like_single_numeric_or_fraction_part(&piece));
    }
    if trimmed.contains('+') {
        return custom_split_delim_parenthesized(trimmed, '+')
            .into_iter()
            .all(|piece| looks_like_single_numeric_or_fraction_part(&piece));
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


fn python_short_default_has_single_effective_token(tokens: &[String]) -> bool {
    let mut distinct = BTreeSet::new();
    for token in tokens {
        if token != "e"
            && token != "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
        {
            distinct.insert(token.as_str());
        }
    }
    distinct.len() == 1
}

fn default_short_commands_for_bare_numeric_token(tokens: &[String]) -> Vec<String> {
    let mut out = [
        "mulpri",
        "a",
        "t",
        "w",
        "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

    if tokens.iter().any(|t| t.contains('/')) {
        out.extend(
            ["u", "B", "G", "E", "groesse"]
                .into_iter()
                .map(|s| s.to_string()),
        );
    }

    if tokens
        .iter()
        .any(|t| t == "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar")
    {
        out.push("-ausgabe".to_string());
        out.push("--keineueberschriften".to_string());
    }

    out
}

pub fn expand_kurz_kurz_befehl(prompt_mode: PromptModus, tokens: &[String]) -> (bool, Vec<String>) {
    if tokens.is_empty() {
        return (false, Vec::new());
    }

    let xtext = tokens.join(" ");
    let stext2 = libreta_prompt_custom_split(&xtext);
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
            let set_text_len_is_1 = python_short_default_has_single_effective_token(tokens);
            let parsed = parse_prefix_and_numeric_suffix(&s);
            if let Some((prefix, numeric)) = parsed {
                if looks_like_numeric_or_fraction_range(&numeric) {
                    let buchst = prefix
                        .chars()
                        .map(|c| c.to_string())
                        .filter(|c| words.one_char_commands.contains(c))
                        .collect::<Vec<_>>();

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
                        text_dazu.push(numeric.clone());
                        text_dazu.extend(default_short_commands_for_bare_numeric_token(tokens));
                    }
                }
            } else if set_text_len_is_1
                && prompt_mode != PromptModus::AusgabeSelektiv
                && looks_like_numeric_or_fraction_range(&s)
            {
                if_kurz_kurz = true;
                text_dazu.push(s.clone());
                text_dazu.extend(default_short_commands_for_bare_numeric_token(tokens));
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

const PYTHON_DEFAULT_OBERESMAXIMUM_FALLBACK: i64 = 1024;
const PYTHON_ROW_MULTIPLE_LIMIT_FALLBACK: i64 = 1028;

fn py_value_max_int(value: &PyValue) -> Option<i64> {
    match value {
        PyValue::Int(value) => Some(*value),
        PyValue::Tuple(values) => values.iter().filter_map(py_value_max_int).max(),
        PyValue::Str(_) | PyValue::Bool(_) | PyValue::NoneValue => None,
    }
}

fn prompt_python_table_maximum_seed() -> i64 {
    static MAXIMUM: OnceLock<i64> = OnceLock::new();
    *MAXIMUM.get_or_init(|| {
        let words_maximum = shared_words()
            .paraNdataMatrix
            .iter()
            .flat_map(|entry| entry.datas.iter())
            .flat_map(|row| row.iter())
            .filter_map(py_value_max_int)
            .filter(|value| *value > 0)
            .max()
            .unwrap_or(PYTHON_DEFAULT_OBERESMAXIMUM_FALLBACK);

        // Python `reta.Program.oberesMaximumArg` never lets the dynamic table
        // maximum fall below the canonical 1024-row prompt universe. Scanning
        // the generated words snapshot keeps Rust tied to the Python data when
        // future tables grow, while preserving the shipped 1024 baseline.
        std::cmp::max(words_maximum, PYTHON_DEFAULT_OBERESMAXIMUM_FALLBACK)
    })
}

fn python_row_multiple_limit() -> i64 {
    let dynamic_limit = prompt_python_table_maximum_seed().saturating_add(4);
    std::cmp::max(dynamic_limit, PYTHON_ROW_MULTIPLE_LIMIT_FALLBACK)
}

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
    let limit = max_zahl.unwrap_or_else(python_row_multiple_limit);
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


fn checked_python_floor_div(left: i64, right: i64) -> Option<i64> {
    if right == 0 {
        return None;
    }
    let quotient = left.checked_div(right)?;
    let remainder = left.checked_rem(right)?;
    if remainder != 0 && ((remainder > 0) != (right > 0)) {
        quotient.checked_sub(1)
    } else {
        Some(quotient)
    }
}

fn checked_python_mod(left: i64, right: i64) -> Option<i64> {
    if right == 0 {
        return None;
    }
    let quotient = checked_python_floor_div(left, right)?;
    let product = quotient.checked_mul(right)?;
    left.checked_sub(product)
}

fn checked_python_shift_left(left: i64, right: i64) -> Option<i64> {
    if !(0..=62).contains(&right) {
        return None;
    }
    left.checked_shl(right as u32)
}

fn checked_python_shift_right(left: i64, right: i64) -> Option<i64> {
    if !(0..=62).contains(&right) {
        return None;
    }
    left.checked_shr(right as u32)
}

fn checked_python_abs(value: i64) -> Option<i64> {
    value.checked_abs()
}

fn checked_python_gcd_pair(left: i64, right: i64) -> Option<i64> {
    let mut a = checked_python_abs(left)?;
    let mut b = checked_python_abs(right)?;
    while b != 0 {
        let remainder = a.checked_rem(b)?;
        a = b;
        b = remainder;
    }
    Some(a)
}

fn checked_python_gcd_many(values: &[i64]) -> Option<i64> {
    let mut iter = values.iter().copied();
    let first = checked_python_abs(iter.next()?)?;
    iter.try_fold(first, checked_python_gcd_pair)
}

fn checked_python_lcm_many(values: &[i64]) -> Option<i64> {
    let mut acc = 1i64;
    for value in values {
        let value = checked_python_abs(*value)?;
        if acc == 0 || value == 0 {
            acc = 0;
            continue;
        }
        let gcd = checked_python_gcd_pair(acc, value)?;
        acc = acc.checked_div(gcd)?.checked_mul(value)?;
    }
    Some(acc)
}

fn checked_python_factorial(value: i64) -> Option<i64> {
    if !(0..=20).contains(&value) {
        return None;
    }
    (1..=value).try_fold(1i64, |acc, factor| acc.checked_mul(factor))
}

fn checked_python_isqrt(value: i64) -> Option<i64> {
    if value < 0 {
        return None;
    }
    let mut low = 0i64;
    let mut high = value.min(3_037_000_499) + 1;
    while low + 1 < high {
        let mid = low + (high - low) / 2;
        match mid.checked_mul(mid) {
            Some(square) if square <= value => low = mid,
            _ => high = mid,
        }
    }
    Some(low)
}

fn checked_python_perm(n: i64, k: i64) -> Option<i64> {
    if n < 0 || k < 0 {
        return None;
    }
    if k > n {
        return Some(0);
    }
    let mut out = 1i64;
    for factor in (n - k + 1)..=n {
        out = out.checked_mul(factor)?;
    }
    Some(out)
}

fn checked_python_comb(n: i64, k: i64) -> Option<i64> {
    if n < 0 || k < 0 {
        return None;
    }
    if k > n {
        return Some(0);
    }
    let k = k.min(n - k);
    let mut out = 1i64;
    for i in 1..=k {
        out = out.checked_mul(n - k + i)?.checked_div(i)?;
    }
    Some(out)
}

fn parse_python_int_literal(raw: &str, radix: u32, allow_prefix_underscore: bool) -> Option<i64> {
    if raw.is_empty() || raw.ends_with('_') || raw.contains("__") {
        return None;
    }
    if !allow_prefix_underscore && raw.starts_with('_') {
        return None;
    }
    let cleaned = raw.replace('_', "");
    if cleaned.is_empty() {
        return None;
    }
    i64::from_str_radix(&cleaned, radix).ok()
}


#[derive(Clone, Copy)]
struct PythonRowExprVar<'a> {
    name: &'a str,
    value: i64,
}

struct PythonRowExprParser<'a> {
    chars: Vec<char>,
    pos: usize,
    vars: Vec<PythonRowExprVar<'a>>,
}

impl<'a> PythonRowExprParser<'a> {
    fn new(text: &str, var: Option<(&'a str, i64)>) -> Self {
        let vars = var
            .map(|(name, value)| vec![PythonRowExprVar { name, value }])
            .unwrap_or_default();
        Self {
            chars: text.chars().collect(),
            pos: 0,
            vars,
        }
    }

    fn with_vars(text: &str, vars: &'a BTreeMap<String, i64>) -> Self {
        Self {
            chars: text.chars().collect(),
            pos: 0,
            vars: vars
                .iter()
                .map(|(name, value)| PythonRowExprVar {
                    name: name.as_str(),
                    value: *value,
                })
                .collect(),
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
        self.parse_bit_or()
    }

    fn parse_bit_or(&mut self) -> Option<i64> {
        let mut value = self.parse_bit_xor()?;
        loop {
            if self.consume_text("|") {
                value |= self.parse_bit_xor()?;
            } else {
                break;
            }
        }
        Some(value)
    }

    fn parse_bit_xor(&mut self) -> Option<i64> {
        let mut value = self.parse_bit_and()?;
        loop {
            if self.consume_text("^") {
                value ^= self.parse_bit_and()?;
            } else {
                break;
            }
        }
        Some(value)
    }

    fn parse_bit_and(&mut self) -> Option<i64> {
        let mut value = self.parse_shift()?;
        loop {
            if self.consume_text("&") {
                value &= self.parse_shift()?;
            } else {
                break;
            }
        }
        Some(value)
    }

    fn parse_shift(&mut self) -> Option<i64> {
        let mut value = self.parse_add_sub()?;
        loop {
            if self.consume_text("<<") {
                value = checked_python_shift_left(value, self.parse_add_sub()?)?;
            } else if self.consume_text(">>") {
                value = checked_python_shift_right(value, self.parse_add_sub()?)?;
            } else {
                break;
            }
        }
        Some(value)
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
        let mut value = self.parse_unary()?;
        loop {
            if self.consume_text("//") {
                value = checked_python_floor_div(value, self.parse_unary()?)?;
            } else if self.consume_text("%") {
                value = checked_python_mod(value, self.parse_unary()?)?;
            } else if self.consume_text("/") {
                // Python eval() keeps `/` as floating point division.  The
                // original retaPrompt accepts generated row collections only
                // when every evaluated element has exact type `int`, so even
                // expressions such as `[4/2]` are rejected there.  Keep `//` as
                // the integer-division form and reject `/` in this row-expression
                // subset instead of silently turning it into integer division.
                return None;
            } else if self.consume_text("*") {
                value = value.checked_mul(self.parse_unary()?)?;
            } else {
                break;
            }
        }
        Some(value)
    }

    fn parse_unary(&mut self) -> Option<i64> {
        if self.consume_text("+") {
            self.parse_unary()
        } else if self.consume_text("-") {
            self.parse_unary()?.checked_neg()
        } else if self.consume_text("~") {
            Some(!self.parse_unary()?)
        } else {
            self.parse_power()
        }
    }

    fn parse_power(&mut self) -> Option<i64> {
        let base = self.parse_primary()?;
        if self.consume_text("**") {
            let exp = self.parse_unary()?;
            if !(0..=31).contains(&exp) {
                return None;
            }
            base.checked_pow(exp as u32)
        } else {
            Some(base)
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
            let args_start = self.pos;
            if let Some(args) = self.parse_call_args_after_open_paren() {
                match ident.as_str() {
                    "abs" if args.len() == 1 => return args[0].checked_abs(),
                    "int" | "round" if args.len() == 1 => return Some(args[0]),
                    "min" if !args.is_empty() => return args.into_iter().min(),
                    "max" if !args.is_empty() => return args.into_iter().max(),
                    "pow" if args.len() == 2 && (0..=31).contains(&args[1]) => {
                        return args[0].checked_pow(args[1] as u32);
                    }
                    "round" if args.len() == 2 => return Some(args[0]),
                    "bool" if args.len() == 1 => return Some(if args[0] == 0 { 0 } else { 1 }),
                    "math.gcd" if args.is_empty() => return Some(0),
                    "math.gcd" => return checked_python_gcd_many(&args),
                    "math.lcm" if args.is_empty() => return Some(1),
                    "math.lcm" => return checked_python_lcm_many(&args),
                    "math.factorial" if args.len() == 1 => return checked_python_factorial(args[0]),
                    "math.isqrt" if args.len() == 1 => return checked_python_isqrt(args[0]),
                    "math.floor" | "math.ceil" | "math.trunc" if args.len() == 1 => {
                        return Some(args[0]);
                    }
                    "math.comb" if args.len() == 2 => return checked_python_comb(args[0], args[1]),
                    "math.perm" if args.len() == 1 => return checked_python_factorial(args[0]),
                    "math.perm" if args.len() == 2 => return checked_python_perm(args[0], args[1]),
                    _ => {}
                }
            }

            self.pos = args_start;
            let inner = self.take_call_inner_after_open_paren()?;
            let vars = self.vars_as_map();
            if matches!(ident.as_str(), "sum" | "math.prod") {
                let parts = custom_split_delim_parenthesized(&inner, ',');
                if (1..=2).contains(&parts.len()) {
                    let start = if parts.len() == 2 {
                        let start_text = parts[1]
                            .trim()
                            .strip_prefix("start=")
                            .unwrap_or(parts[1].trim());
                        eval_python_row_expr_with_vars(start_text, &vars)?
                    } else if ident == "sum" {
                        0
                    } else {
                        1
                    };
                    let values = parse_python_iterable_values_with_vars(&parts[0], &vars)?;
                    return if ident == "sum" {
                        values.into_iter().try_fold(start, |acc, value| acc.checked_add(value))
                    } else {
                        values.into_iter().try_fold(start, |acc, value| acc.checked_mul(value))
                    };
                }
            }
            let values = parse_python_iterable_values_with_vars(&inner, &vars);
            return match ident.as_str() {
                "sum" => values?
                    .into_iter()
                    .try_fold(0i64, |acc, value| acc.checked_add(value)),
                "math.prod" => values?
                    .into_iter()
                    .try_fold(1i64, |acc, value| acc.checked_mul(value)),
                "len" => Some(values?.len() as i64),
                "min" => values?.into_iter().min(),
                "max" => values?.into_iter().max(),
                "all" => Some(if values?.into_iter().all(|value| value != 0) { 1 } else { 0 }),
                "any" => Some(if values?.into_iter().any(|value| value != 0) { 1 } else { 0 }),
                "bool" => Some(if values?.is_empty() { 0 } else { 1 }),
                _ => None,
            };
        }

        self.vars
            .iter()
            .find(|var| var.name == ident)
            .map(|var| var.value)
    }

    fn parse_call_args_after_open_paren(&mut self) -> Option<Vec<i64>> {
        let mut args = Vec::new();
        self.skip_ws();
        if self.consume_text(")") {
            return Some(args);
        }

        loop {
            args.push(self.parse_expr()?);
            self.skip_ws();
            if self.consume_text(")") {
                break;
            }
            if !self.consume_text(",") {
                return None;
            }
        }

        Some(args)
    }

    fn take_call_inner_after_open_paren(&mut self) -> Option<String> {
        let start = self.pos;
        let mut round = 1i32;
        let mut square = 0i32;
        let mut curly = 0i32;

        while !self.finished() {
            let ch = self.chars[self.pos];
            match ch {
                '(' => round += 1,
                ')' if square == 0 && curly == 0 => {
                    round -= 1;
                    if round == 0 {
                        let inner = self.chars[start..self.pos].iter().collect();
                        self.pos += 1;
                        return Some(inner);
                    }
                }
                ')' => round -= 1,
                '[' => square += 1,
                ']' => square -= 1,
                '{' => curly += 1,
                '}' => curly -= 1,
                _ => {}
            }
            self.pos += 1;
        }

        None
    }

    fn vars_as_map(&self) -> BTreeMap<String, i64> {
        self.vars
            .iter()
            .map(|var| (var.name.to_string(), var.value))
            .collect()
    }

    fn parse_number(&mut self) -> Option<i64> {
        self.skip_ws();
        let start = self.pos;
        if self.finished() || !self.chars[self.pos].is_ascii_digit() {
            return None;
        }

        if self.chars[self.pos] == '0' {
            if let Some(prefix) = self.chars.get(self.pos + 1).copied() {
                let radix = match prefix {
                    'b' | 'B' => Some(2),
                    'o' | 'O' => Some(8),
                    'x' | 'X' => Some(16),
                    _ => None,
                };
                if let Some(radix) = radix {
                    self.pos += 2;
                    let digits_start = self.pos;
                    while !self.finished()
                        && (self.chars[self.pos] == '_'
                            || self.chars[self.pos].is_digit(radix))
                    {
                        self.pos += 1;
                    }
                    if self.pos == digits_start {
                        self.pos = start;
                        return None;
                    }
                    let raw = self.chars[digits_start..self.pos]
                        .iter()
                        .collect::<String>();
                    return parse_python_int_literal(&raw, radix, true).or_else(|| {
                        self.pos = start;
                        None
                    });
                }
            }
        }

        while !self.finished()
            && (self.chars[self.pos] == '_' || self.chars[self.pos].is_ascii_digit())
        {
            self.pos += 1;
        }
        let raw = self.chars[start..self.pos].iter().collect::<String>();
        parse_python_int_literal(&raw, 10, false).or_else(|| {
            self.pos = start;
            None
        })
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
                || self.chars[self.pos] == '.'
                || self.chars[self.pos].is_ascii_alphanumeric())
        {
            self.pos += 1;
        }
        let ident = self.chars[start..self.pos].iter().collect::<String>();
        if ident.ends_with('.') || ident.contains("..") {
            None
        } else {
            Some(ident)
        }
    }
}

fn eval_python_row_expr_with_vars(text: &str, vars: &BTreeMap<String, i64>) -> Option<i64> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(value) = eval_python_row_subscript_with_vars(trimmed, vars) {
        return Some(value);
    }
    if let Some((true_expr, condition, false_expr)) = split_top_level_conditional_expr(trimmed) {
        return if eval_python_row_condition_with_vars(condition, vars)? {
            eval_python_row_expr_with_vars(true_expr, vars)
        } else {
            eval_python_row_expr_with_vars(false_expr, vars)
        };
    }
    let mut parser = if vars.is_empty() {
        PythonRowExprParser::new(trimmed, None)
    } else {
        PythonRowExprParser::with_vars(trimmed, vars)
    };
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PythonCollectionKind {
    SequenceLike,
    SetOrDictLike,
}

impl PythonCollectionKind {
    fn deduplicates_like_python_set_conversion(self) -> bool {
        matches!(self, Self::SetOrDictLike)
    }
}

fn strip_python_collection_wrappers_with_kind(text: &str) -> Option<(&str, PythonCollectionKind)> {
    let trimmed = text.trim();
    if trimmed.len() < 2 {
        return None;
    }
    let (open, close, kind) = match (trimmed.chars().next()?, trimmed.chars().last()?) {
        ('[', ']') => ('[', ']', PythonCollectionKind::SequenceLike),
        ('{', '}') => ('{', '}', PythonCollectionKind::SetOrDictLike),
        ('(', ')') => ('(', ')', PythonCollectionKind::SequenceLike),
        _ => return None,
    };
    if !trimmed.starts_with(open) || !trimmed.ends_with(close) {
        return None;
    }
    Some((&trimmed[open.len_utf8()..trimmed.len() - close.len_utf8()], kind))
}

fn strip_top_level_wrapping_parens(text: &str) -> Option<&str> {
    let trimmed = text.trim();
    if !trimmed.starts_with('(') || !trimmed.ends_with(')') {
        return None;
    }

    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;
    for (index, ch) in trimmed.char_indices() {
        match ch {
            '(' => round += 1,
            ')' => {
                round -= 1;
                if round == 0 && square == 0 && curly == 0 && index + ch.len_utf8() < trimmed.len() {
                    return None;
                }
            }
            '[' => square += 1,
            ']' => square -= 1,
            '{' => curly += 1,
            '}' => curly -= 1,
            _ => {}
        }
        if round < 0 || square < 0 || curly < 0 {
            return None;
        }
    }

    (round == 0 && square == 0 && curly == 0).then_some(&trimmed[1..trimmed.len() - 1])
}

fn split_top_level_trailing_subscript(text: &str) -> Option<(&str, &str)> {
    let trimmed = text.trim();
    if !trimmed.ends_with(']') {
        return None;
    }

    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;
    let mut candidate_start = None;

    for (index, ch) in trimmed.char_indices() {
        match ch {
            '(' => round += 1,
            ')' => round -= 1,
            '[' if round == 0 && square == 0 && curly == 0 => {
                candidate_start = Some(index);
                square += 1;
            }
            '[' => square += 1,
            ']' => {
                square -= 1;
                if round < 0 || square < 0 || curly < 0 {
                    return None;
                }
                if round == 0 && square == 0 && curly == 0 && index + ch.len_utf8() == trimmed.len() {
                    let start = candidate_start?;
                    let base = trimmed[..start].trim();
                    let index_text = trimmed[start + 1..index].trim();
                    return (!base.is_empty() && !index_text.is_empty()).then_some((base, index_text));
                }
            }
            '{' => curly += 1,
            '}' => curly -= 1,
            _ => {}
        }
        if round < 0 || square < 0 || curly < 0 {
            return None;
        }
    }

    None
}

fn python_normalize_index(index: i64, len: usize) -> Option<usize> {
    let len_i64 = i64::try_from(len).ok()?;
    let index = if index < 0 { len_i64.checked_add(index)? } else { index };
    if (0..len_i64).contains(&index) {
        usize::try_from(index).ok()
    } else {
        None
    }
}

fn python_slice_bound(value: Option<i64>, len: i64, step: i64, is_start: bool) -> i64 {
    let default = match (step > 0, is_start) {
        (true, true) => 0,
        (true, false) => len,
        (false, true) => len - 1,
        (false, false) => -1,
    };
    let Some(mut value) = value else {
        return default;
    };
    if value < 0 {
        value += len;
    }
    if step > 0 {
        value.clamp(0, len)
    } else {
        value.clamp(-1, len - 1)
    }
}

fn split_python_slice_parts_preserving_empty(index_text: &str) -> Option<Vec<String>> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;

    for ch in index_text.chars() {
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
            ':' if round == 0 && square == 0 && curly == 0 => {
                out.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
        if round < 0 || square < 0 || curly < 0 {
            return None;
        }
    }

    if round != 0 || square != 0 || curly != 0 {
        return None;
    }
    out.push(current.trim().to_string());
    Some(out)
}

fn python_slice_values(
    values: &[i64],
    index_text: &str,
    vars: &BTreeMap<String, i64>,
) -> Option<Vec<i64>> {
    let parts = split_python_slice_parts_preserving_empty(index_text)?;
    if parts.len() < 2 || parts.len() > 3 {
        return None;
    }

    let parse_optional = |part: &str| {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            Some(None)
        } else {
            eval_python_row_expr_with_vars(trimmed, vars).map(Some)
        }
    };

    let start = parse_optional(&parts[0])?;
    let stop = parse_optional(&parts[1])?;
    let step = if parts.len() == 3 {
        parse_optional(&parts[2])?.unwrap_or(1)
    } else {
        1
    };
    if step == 0 {
        return None;
    }

    let len = i64::try_from(values.len()).ok()?;
    let mut index = python_slice_bound(start, len, step, true);
    let stop = python_slice_bound(stop, len, step, false);
    let mut out = Vec::new();

    if step > 0 {
        while index < stop {
            out.push(values[usize::try_from(index).ok()?]);
            index = index.checked_add(step)?;
        }
    } else {
        while index > stop {
            if index >= 0 {
                out.push(values[usize::try_from(index).ok()?]);
            }
            index = index.checked_add(step)?;
        }
    }

    Some(out)
}

fn eval_python_row_subscript_with_vars(text: &str, vars: &BTreeMap<String, i64>) -> Option<i64> {
    let (base, index_text) = split_top_level_trailing_subscript(text)?;
    if index_text.contains(':') {
        return None;
    }
    let values = parse_python_iterable_values_with_vars(base, vars)?;
    let index = eval_python_row_expr_with_vars(index_text, vars)?;
    let index = python_normalize_index(index, values.len())?;
    values.get(index).copied()
}

fn parse_python_iterable_subscript_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
) -> Option<Vec<i64>> {
    let (base, index_text) = split_top_level_trailing_subscript(text)?;
    if !index_text.contains(':') {
        return None;
    }
    let values = parse_python_iterable_values_with_vars(base, vars)?;
    python_slice_values(&values, index_text, vars)
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

fn find_first_top_level_keyword<'a>(
    text: &str,
    keywords: &'a [&'a str],
) -> Option<(usize, &'a str)> {
    let mut best: Option<(usize, &'a str)> = None;
    for keyword in keywords {
        if let Some(index) = find_top_level_keyword(text, keyword) {
            if best.map(|(best_index, _)| index < best_index).unwrap_or(true) {
                best = Some((index, *keyword));
            }
        }
    }
    best
}

fn split_top_level_conditional_expr(text: &str) -> Option<(&str, &str, &str)> {
    let if_index = find_top_level_keyword(text, " if ")?;
    let true_expr = text[..if_index].trim();
    let after_if = &text[if_index + " if ".len()..];
    let else_index = find_top_level_keyword(after_if, " else ")?;
    let condition = after_if[..else_index].trim();
    let false_expr = after_if[else_index + " else ".len()..].trim();
    if true_expr.is_empty() || condition.is_empty() || false_expr.is_empty() {
        return None;
    }
    Some((true_expr, condition, false_expr))
}

fn split_top_level_comparison_chain<'a>(text: &'a str) -> Option<(Vec<&'a str>, Vec<&'a str>)> {
    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;
    let operators = ["==", "!=", "<=", ">=", "<", ">"];
    let mut operands = Vec::new();
    let mut ops = Vec::new();
    let mut last_start = 0usize;
    let mut skip_until = 0usize;

    for (index, ch) in text.char_indices() {
        if index < skip_until {
            continue;
        }
        if round == 0 && square == 0 && curly == 0 {
            for operator in operators {
                if text[index..].starts_with(operator) {
                    let left = text[last_start..index].trim();
                    if left.is_empty() {
                        return None;
                    }
                    operands.push(left);
                    ops.push(operator);
                    last_start = index + operator.len();
                    skip_until = last_start;
                    break;
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

    if ops.is_empty() {
        return None;
    }
    let right = text[last_start..].trim();
    if right.is_empty() {
        return None;
    }
    operands.push(right);
    Some((operands, ops))
}

fn compare_python_row_i64(left: i64, operator: &str, right: i64) -> Option<bool> {
    match operator {
        "==" => Some(left == right),
        "!=" => Some(left != right),
        "<=" => Some(left <= right),
        ">=" => Some(left >= right),
        "<" => Some(left < right),
        ">" => Some(left > right),
        _ => None,
    }
}

fn eval_python_row_condition_with_vars(text: &str, vars: &BTreeMap<String, i64>) -> Option<bool> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    match trimmed {
        "True" => return Some(true),
        "False" | "None" => return Some(false),
        _ => {}
    }
    if let Some(inner) = strip_top_level_wrapping_parens(trimmed) {
        return eval_python_row_condition_with_vars(inner, vars);
    }
    if let Some(inner) = parse_python_call_inner(trimmed, "all") {
        let values = parse_python_truth_iterable_values_with_vars(inner, vars)?;
        return Some(values.into_iter().all(|value| value));
    }
    if let Some(inner) = parse_python_call_inner(trimmed, "any") {
        let values = parse_python_truth_iterable_values_with_vars(inner, vars)?;
        return Some(values.into_iter().any(|value| value));
    }
    if let Some(index) = find_top_level_keyword(trimmed, " or ") {
        return Some(
            eval_python_row_condition_with_vars(&trimmed[..index], vars)?
                || eval_python_row_condition_with_vars(&trimmed[index + 4..], vars)?,
        );
    }
    if let Some(index) = find_top_level_keyword(trimmed, " and ") {
        return Some(
            eval_python_row_condition_with_vars(&trimmed[..index], vars)?
                && eval_python_row_condition_with_vars(&trimmed[index + 5..], vars)?,
        );
    }
    if let Some(rest) = trimmed.strip_prefix("not ") {
        return Some(!eval_python_row_condition_with_vars(rest, vars)?);
    }
    if let Some(index) = find_top_level_keyword(trimmed, " not in ") {
        let left = eval_python_row_expr_with_vars(&trimmed[..index], vars)?;
        let right_values =
            parse_python_iterable_values_with_vars(&trimmed[index + " not in ".len()..], vars)?;
        return Some(!right_values.contains(&left));
    }
    if let Some(index) = find_top_level_keyword(trimmed, " in ") {
        let left = eval_python_row_expr_with_vars(&trimmed[..index], vars)?;
        let right_values =
            parse_python_iterable_values_with_vars(&trimmed[index + " in ".len()..], vars)?;
        return Some(right_values.contains(&left));
    }
    if let Some((operands, operators)) = split_top_level_comparison_chain(trimmed) {
        for (index, operator) in operators.iter().enumerate() {
            let left = eval_python_row_expr_with_vars(operands[index], vars)?;
            let right = eval_python_row_expr_with_vars(operands[index + 1], vars)?;
            if !compare_python_row_i64(left, operator, right)? {
                return Some(false);
            }
        }
        return Some(true);
    }
    Some(eval_python_row_expr_with_vars(trimmed, vars)? != 0)
}

fn parse_python_range_values_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
) -> Option<Vec<i64>> {
    let trimmed = text.trim();
    let rest = trimmed.strip_prefix("range")?.trim_start();
    let inner = rest.strip_prefix('(')?.strip_suffix(')')?;
    let parts = custom_split_delim_parenthesized(inner, ',');
    let args = parts
        .iter()
        .map(|part| eval_python_row_expr_with_vars(part, vars))
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

fn split_top_level_iterable_binary_rightmost<'a>(
    text: &'a str,
    operator: char,
) -> Option<(&'a str, &'a str)> {
    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;
    let mut last_match: Option<usize> = None;

    for (index, ch) in text.char_indices() {
        if ch == operator && index > 0 && round == 0 && square == 0 && curly == 0 {
            let left = text[..index].trim();
            let right = text[index + ch.len_utf8()..].trim();
            if !left.is_empty() && !right.is_empty() {
                last_match = Some(index);
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

    let index = last_match?;
    Some((
        text[..index].trim(),
        text[index + operator.len_utf8()..].trim(),
    ))
}


fn split_top_level_dict_key_value(text: &str) -> Option<(&str, &str)> {
    let mut round = 0i32;
    let mut square = 0i32;
    let mut curly = 0i32;

    for (index, ch) in text.char_indices() {
        if ch == ':' && round == 0 && square == 0 && curly == 0 {
            let key = text[..index].trim();
            let value = text[index + ch.len_utf8()..].trim();
            if !key.is_empty() && !value.is_empty() {
                return Some((key, value));
            }
            return None;
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

fn parse_python_lambda_parts<'a>(text: &'a str) -> Option<(Vec<String>, &'a str)> {
    let trimmed = text.trim();
    let body = trimmed.strip_prefix("lambda ")?.trim_start();
    let (args_text, expr_text) = split_top_level_dict_key_value(body)?;
    let arg_names = parse_python_binding_names(args_text)?;
    let expr_text = expr_text.trim();
    (!expr_text.is_empty()).then_some((arg_names, expr_text))
}

fn eval_python_lambda_expr_with_vars(
    lambda_text: &str,
    args: &[i64],
    outer_vars: &BTreeMap<String, i64>,
) -> Option<i64> {
    let (arg_names, expr_text) = parse_python_lambda_parts(lambda_text)?;
    if arg_names.len() != args.len() {
        return None;
    }
    let mut vars = outer_vars.clone();
    assign_python_binding_vars(&mut vars, &arg_names, args)?;
    eval_python_row_expr_with_vars(expr_text, &vars)
}

fn eval_python_lambda_condition_with_vars(
    lambda_text: &str,
    args: &[i64],
    outer_vars: &BTreeMap<String, i64>,
) -> Option<bool> {
    let (arg_names, expr_text) = parse_python_lambda_parts(lambda_text)?;
    if arg_names.len() != args.len() {
        return None;
    }
    let mut vars = outer_vars.clone();
    assign_python_binding_vars(&mut vars, &arg_names, args)?;
    eval_python_row_condition_with_vars(expr_text, &vars)
}

fn eval_python_map_function_with_vars(
    function_name: &str,
    args: &[i64],
    vars: &BTreeMap<String, i64>,
) -> Option<i64> {
    let function_name = function_name.trim();
    if parse_python_lambda_parts(function_name).is_some() {
        return eval_python_lambda_expr_with_vars(function_name, args, vars);
    }
    match function_name {
        "abs" if args.len() == 1 => args[0].checked_abs(),
        "int" | "round" if args.len() == 1 => Some(args[0]),
        "bool" if args.len() == 1 => Some(if args[0] == 0 { 0 } else { 1 }),
        "pow" if args.len() == 2 && (0..=31).contains(&args[1]) => {
            args[0].checked_pow(args[1] as u32)
        }
        "min" if !args.is_empty() => args.iter().copied().min(),
        "max" if !args.is_empty() => args.iter().copied().max(),
        "math.gcd" if args.is_empty() => Some(0),
        "math.gcd" => checked_python_gcd_many(args),
        "math.lcm" if args.is_empty() => Some(1),
        "math.lcm" => checked_python_lcm_many(args),
        "math.factorial" if args.len() == 1 => checked_python_factorial(args[0]),
        "math.isqrt" if args.len() == 1 => checked_python_isqrt(args[0]),
        "math.floor" | "math.ceil" | "math.trunc" if args.len() == 1 => Some(args[0]),
        "math.comb" if args.len() == 2 => checked_python_comb(args[0], args[1]),
        "math.perm" if args.len() == 1 => checked_python_factorial(args[0]),
        "math.perm" if args.len() == 2 => checked_python_perm(args[0], args[1]),
        _ => None,
    }
}

fn eval_python_filter_predicate_with_vars(
    predicate: &str,
    value: i64,
    vars: &BTreeMap<String, i64>,
) -> Option<bool> {
    let predicate = predicate.trim();
    if predicate == "None" {
        return Some(value != 0);
    }
    if parse_python_lambda_parts(predicate).is_some() {
        return eval_python_lambda_condition_with_vars(predicate, &[value], vars);
    }
    match predicate {
        "bool" | "abs" | "int" | "round" => Some(value != 0),
        _ => eval_python_map_function_with_vars(predicate, &[value], vars)
            .map(|mapped| mapped != 0),
    }
}

fn parse_python_map_iterable_columns_with_vars(
    parts: &[String],
    vars: &BTreeMap<String, i64>,
) -> Option<Vec<Vec<i64>>> {
    if parts.len() < 2 {
        return None;
    }
    parts
        .iter()
        .skip(1)
        .map(|part| parse_python_iterable_values_with_vars(part, vars))
        .collect::<Option<Vec<_>>>()
}

fn python_map_min_len(columns: &[Vec<i64>]) -> usize {
    columns.iter().map(Vec::len).min().unwrap_or(0)
}

fn parse_python_keyword_bool_arg(text: &str, name: &str) -> Option<bool> {
    let (key, value) = text.split_once('=')?;
    if key.trim() != name {
        return None;
    }
    match value.trim() {
        "True" => Some(true),
        "False" => Some(false),
        _ => None,
    }
}

fn parse_python_keyword_none_arg(text: &str, name: &str) -> Option<()> {
    let (key, value) = text.split_once('=')?;
    (key.trim() == name && value.trim() == "None").then_some(())
}

fn parse_python_sorted_options(parts: &[String]) -> Option<bool> {
    let mut reverse = false;
    for part in parts.iter().skip(1) {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(value) = parse_python_keyword_bool_arg(trimmed, "reverse") {
            reverse = value;
            continue;
        }
        if parse_python_keyword_none_arg(trimmed, "key").is_some() {
            continue;
        }
        return None;
    }
    Some(reverse)
}

fn repeat_python_iterable_values(values: Vec<i64>, count: i64) -> Option<Vec<i64>> {
    if count <= 0 {
        return Some(Vec::new());
    }
    let count = usize::try_from(count).ok()?;
    if !values.is_empty() && values.len().checked_mul(count)? > 20_000 {
        return None;
    }
    let mut out = Vec::with_capacity(values.len().saturating_mul(count));
    for _ in 0..count {
        out.extend(values.iter().copied());
    }
    Some(out)
}

fn parse_python_iterable_repeat_values_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
) -> Option<Vec<i64>> {
    let (left, right) = split_top_level_iterable_binary_rightmost(text, '*')?;

    if let Some(values) = parse_python_iterable_values_with_vars(left, vars) {
        let count = eval_python_row_expr_with_vars(right, vars)?;
        return repeat_python_iterable_values(values, count);
    }

    let count = eval_python_row_expr_with_vars(left, vars)?;
    let values = parse_python_iterable_values_with_vars(right, vars)?;
    repeat_python_iterable_values(values, count)
}

fn parse_python_builtin_iterable_values_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
) -> Option<Vec<i64>> {
    if let Some(inner) = parse_python_call_inner(text, "divmod") {
        let args = custom_split_delim_parenthesized(inner, ',')
            .iter()
            .map(|part| eval_python_row_expr_with_vars(part, vars))
            .collect::<Option<Vec<_>>>()?;
        if args.len() == 2 {
            let quotient = checked_python_floor_div(args[0], args[1])?;
            let remainder = checked_python_mod(args[0], args[1])?;
            return Some(vec![quotient, remainder]);
        }
        return None;
    }

    if let Some(inner) = parse_python_call_inner(text, "filter") {
        let parts = custom_split_delim_parenthesized(inner, ',');
        if parts.len() != 2 {
            return None;
        }
        let predicate = parts[0].trim();
        let values = parse_python_iterable_values_with_vars(&parts[1], vars)?;
        let mut out = Vec::new();
        for value in values {
            if eval_python_filter_predicate_with_vars(predicate, value, vars)? {
                out.push(value);
            }
        }
        return Some(out);
    }

    if let Some(inner) = parse_python_call_inner(text, "map") {
        let parts = custom_split_delim_parenthesized(inner, ',');
        let function_name = parts.first().map(String::as_str)?.trim();
        let columns = parse_python_map_iterable_columns_with_vars(&parts, vars)?;
        let min_len = python_map_min_len(&columns);
        let mut out = Vec::new();
        for index in 0..min_len {
            let args = columns.iter().map(|column| column[index]).collect::<Vec<_>>();
            out.push(eval_python_map_function_with_vars(function_name, &args, vars)?);
        }
        return Some(out);
    }

    for name in ["list", "tuple", "set", "frozenset", "sorted", "reversed"] {
        if let Some(inner) = parse_python_call_inner(text, name) {
            if inner.trim().is_empty() {
                return match name {
                    "list" | "tuple" | "set" | "frozenset" => Some(Vec::new()),
                    _ => None,
                };
            }
            let parts = custom_split_delim_parenthesized(inner, ',');
            if name == "sorted" {
                let reverse = parse_python_sorted_options(&parts)?;
                let mut values = parse_python_iterable_values_with_vars(
                    parts.first().map(String::as_str).unwrap_or_default(),
                    vars,
                )?;
                values.sort();
                if reverse {
                    values.reverse();
                }
                return Some(values);
            }
            let source = if name == "reversed" {
                if parts.len() != 1 {
                    return None;
                }
                parts.first().cloned().unwrap_or_default()
            } else {
                inner.to_string()
            };
            let mut values = parse_python_iterable_values_with_vars(&source, vars)?;
            if name == "set" || name == "frozenset" {
                values = values.into_iter().collect::<BTreeSet<_>>().into_iter().collect();
            }
            if name == "reversed" {
                values.reverse();
            }
            return Some(values);
        }
    }
    None
}

fn parse_python_iterable_values_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
) -> Option<Vec<i64>> {
    let trimmed = text.trim();

    if let Some(values) = parse_python_dict_view_values_with_vars(trimmed, vars) {
        return Some(values);
    }

    if let Some(values) = parse_python_iterable_subscript_with_vars(trimmed, vars) {
        return Some(values);
    }

    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        let inner = &trimmed[1..trimmed.len() - 1];
        if ['|', '^', '&', '-', '+', '*']
            .iter()
            .any(|operator| split_top_level_iterable_binary(inner, *operator).is_some())
        {
            return parse_python_iterable_values_with_vars(inner, vars);
        }
    }

    if let Some((left, right)) = split_top_level_iterable_binary(trimmed, '|') {
        let mut out = parse_python_iterable_values_with_vars(left, vars)?;
        out.extend(parse_python_iterable_values_with_vars(right, vars)?);
        return Some(out.into_iter().collect::<BTreeSet<_>>().into_iter().collect());
    }

    if let Some((left, right)) = split_top_level_iterable_binary(trimmed, '^') {
        let left_values = parse_python_iterable_values_with_vars(left, vars)?;
        let right_values = parse_python_iterable_values_with_vars(right, vars)?;
        let left_set = left_values.into_iter().collect::<BTreeSet<_>>();
        let right_set = right_values.into_iter().collect::<BTreeSet<_>>();
        return Some(
            left_set
                .symmetric_difference(&right_set)
                .copied()
                .collect::<Vec<_>>(),
        );
    }

    if let Some((left, right)) = split_top_level_iterable_binary(trimmed, '&') {
        let left_values = parse_python_iterable_values_with_vars(left, vars)?;
        let right_values = parse_python_iterable_values_with_vars(right, vars)?;
        let right_set = right_values.into_iter().collect::<BTreeSet<_>>();
        return Some(
            left_values
                .into_iter()
                .filter(|value| right_set.contains(value))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
        );
    }

    if let Some((left, right)) = split_top_level_iterable_binary_rightmost(trimmed, '-') {
        let left_values = parse_python_iterable_values_with_vars(left, vars)?;
        let right_values = parse_python_iterable_values_with_vars(right, vars)?;
        let right_set = right_values.into_iter().collect::<BTreeSet<_>>();
        return Some(
            left_values
                .into_iter()
                .filter(|value| !right_set.contains(value))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
        );
    }

    if let Some((left, right)) = split_top_level_iterable_binary(trimmed, '+') {
        let mut out = parse_python_iterable_values_with_vars(left, vars)?;
        out.extend(parse_python_iterable_values_with_vars(right, vars)?);
        return Some(out);
    }

    if let Some(values) = parse_python_iterable_repeat_values_with_vars(trimmed, vars) {
        return Some(values);
    }

    parse_python_range_values_with_vars(trimmed, vars)
        .or_else(|| parse_python_builtin_iterable_values_with_vars(trimmed, vars))
        .or_else(|| parse_python_bare_generator_row_values_with_vars(trimmed, vars))
        .or_else(|| parse_python_generated_row_values_with_vars(trimmed, vars))
}

fn parse_python_iterable_values(text: &str) -> Option<Vec<i64>> {
    parse_python_iterable_values_with_vars(text, &BTreeMap::new())
}

fn python_str_as_generator_source(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.len() < 2 {
        return None;
    }

    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        return Some(format!("[{}]", &trimmed[1..trimmed.len() - 1]));
    }

    if (trimmed.starts_with('[') && trimmed.ends_with(']'))
        || (trimmed.starts_with('{') && trimmed.ends_with('}'))
    {
        return Some(trimmed.to_string());
    }

    None
}

fn parse_python_str_as_generator_values(text: &str) -> Option<Vec<i64>> {
    let source = python_str_as_generator_source(text)?;
    let values = parse_python_iterable_values(&source)?;
    Some(values.into_iter().collect::<BTreeSet<_>>().into_iter().collect())
}

#[derive(Clone, Debug)]
struct PythonComprehensionClause {
    var_names: Vec<String>,
    source_text: String,
    filters: Vec<String>,
}

fn parse_python_binding_names(text: &str) -> Option<Vec<String>> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    let body = if let Some(inner) = strip_top_level_wrapping_parens(trimmed) {
        inner.trim()
    } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
        trimmed[1..trimmed.len() - 1].trim()
    } else {
        trimmed
    };
    if body.is_empty() {
        return None;
    }

    let mut names = Vec::new();
    for part in custom_split_delim_parenthesized(body, ',') {
        let name = part.trim();
        if name.is_empty() {
            continue;
        }
        if !is_python_row_identifier(name) {
            return None;
        }
        names.push(name.to_string());
    }

    (!names.is_empty()).then_some(names)
}

fn parse_python_comprehension_clauses(mut tail: &str) -> Option<Vec<PythonComprehensionClause>> {
    let mut clauses = Vec::new();

    loop {
        tail = tail.trim_start();
        if let Some(next_tail) = tail.strip_prefix("for ") {
            tail = next_tail.trim_start();
        }

        let in_index = find_top_level_keyword(tail, " in ")?;
        let var_names = parse_python_binding_names(tail[..in_index].trim())?;

        let rest_after_in = tail[in_index + 4..].trim_start();
        let (source_text, rest_after_source) = match find_first_top_level_keyword(
            rest_after_in,
            &[" if ", " for "],
        ) {
            Some((index, _)) => (
                rest_after_in[..index].trim().to_string(),
                rest_after_in[index..].trim_start(),
            ),
            None => (rest_after_in.trim().to_string(), ""),
        };
        if source_text.is_empty() {
            return None;
        }

        let mut filters = Vec::new();
        let mut rest = rest_after_source;
        loop {
            rest = rest.trim_start();
            if rest.is_empty() {
                clauses.push(PythonComprehensionClause {
                    var_names,
                    source_text,
                    filters,
                });
                return Some(clauses);
            }
            if let Some(next_for) = rest.strip_prefix("for ") {
                clauses.push(PythonComprehensionClause {
                    var_names,
                    source_text,
                    filters,
                });
                tail = next_for.trim_start();
                break;
            }
            let Some(after_if) = rest.strip_prefix("if ") else {
                return None;
            };
            let after_if = after_if.trim_start();
            let (filter_text, next_rest) = match find_first_top_level_keyword(
                after_if,
                &[" if ", " for "],
            ) {
                Some((index, _)) => (
                    after_if[..index].trim().to_string(),
                    after_if[index..].trim_start(),
                ),
                None => (after_if.trim().to_string(), ""),
            };
            if filter_text.is_empty() {
                return None;
            }
            filters.push(filter_text);
            rest = next_rest;
        }
    }
}

fn python_dict_pairs_finish(pairs: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut map = BTreeMap::new();
    for (key, value) in pairs {
        map.insert(key, value);
    }
    map.into_iter().collect()
}

fn expand_python_dict_pair_comprehension_values(
    clauses: &[PythonComprehensionClause],
    clause_index: usize,
    key_expr: &str,
    value_expr: &str,
    vars: &mut BTreeMap<String, i64>,
    out: &mut Vec<(i64, i64)>,
) -> Option<()> {
    if clause_index >= clauses.len() {
        out.push((
            eval_python_row_expr_with_vars(key_expr, vars)?,
            eval_python_row_expr_with_vars(value_expr, vars)?,
        ));
        return Some(());
    }

    let clause = &clauses[clause_index];
    let binding_rows = parse_python_iterable_binding_rows_with_vars(
        &clause.source_text,
        vars,
        clause.var_names.len(),
    )?;
    let previous = clause
        .var_names
        .iter()
        .map(|name| vars.get(name).copied())
        .collect::<Vec<_>>();

    for row in binding_rows {
        assign_python_binding_vars(vars, &clause.var_names, &row)?;
        let keep = clause
            .filters
            .iter()
            .map(|filter| eval_python_row_condition_with_vars(filter, vars))
            .collect::<Option<Vec<_>>>()?
            .into_iter()
            .all(|value| value);
        if keep {
            expand_python_dict_pair_comprehension_values(
                clauses,
                clause_index + 1,
                key_expr,
                value_expr,
                vars,
                out,
            )?;
        }
    }

    restore_python_binding_vars(vars, &clause.var_names, previous);
    Some(())
}

fn parse_python_dict_pairs_with_vars(
    text: &str,
    outer_vars: &BTreeMap<String, i64>,
) -> Option<Vec<(i64, i64)>> {
    let (inner, kind) = strip_python_collection_wrappers_with_kind(text)?;
    if !matches!(kind, PythonCollectionKind::SetOrDictLike) {
        return None;
    }
    let inner = inner.trim();
    if inner.is_empty() {
        return Some(Vec::new());
    }

    if let Some(for_index) = find_top_level_keyword(inner, " for ") {
        let expr_text = inner[..for_index].trim();
        let (key_expr, value_expr) = split_top_level_dict_key_value(expr_text)?;
        let tail = inner[for_index + 5..].trim();
        let clauses = parse_python_comprehension_clauses(tail)?;
        let mut vars = outer_vars.clone();
        let mut out = Vec::new();
        expand_python_dict_pair_comprehension_values(
            &clauses,
            0,
            key_expr,
            value_expr,
            &mut vars,
            &mut out,
        )?;
        return Some(python_dict_pairs_finish(out));
    }

    let mut pairs = Vec::new();
    for part in custom_split_delim_parenthesized(inner, ',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (key, value) = split_top_level_dict_key_value(part)?;
        pairs.push((
            eval_python_row_expr_with_vars(key, outer_vars)?,
            eval_python_row_expr_with_vars(value, outer_vars)?,
        ));
    }
    Some(python_dict_pairs_finish(pairs))
}

fn parse_python_dict_view_values_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
) -> Option<Vec<i64>> {
    let trimmed = text.trim();
    for (suffix, take_values) in [(".keys()", false), (".values()", true)] {
        if let Some(base) = trimmed.strip_suffix(suffix) {
            let pairs = parse_python_dict_pairs_with_vars(base.trim(), vars)?;
            return Some(
                pairs
                    .into_iter()
                    .map(|(key, value)| if take_values { value } else { key })
                    .collect(),
            );
        }
    }
    None
}

fn restore_python_binding_vars(
    vars: &mut BTreeMap<String, i64>,
    names: &[String],
    previous: Vec<Option<i64>>,
) {
    for (name, previous_value) in names.iter().zip(previous.into_iter()) {
        if let Some(value) = previous_value {
            vars.insert(name.clone(), value);
        } else {
            vars.remove(name);
        }
    }
}

fn assign_python_binding_vars(
    vars: &mut BTreeMap<String, i64>,
    names: &[String],
    values: &[i64],
) -> Option<()> {
    if names.len() != values.len() {
        return None;
    }
    for (name, value) in names.iter().zip(values.iter().copied()) {
        vars.insert(name.clone(), value);
    }
    Some(())
}

fn parse_python_tuple_row_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
    arity: usize,
) -> Option<Vec<i64>> {
    if arity == 0 {
        return None;
    }
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    if arity == 1 {
        return Some(vec![eval_python_row_expr_with_vars(trimmed, vars)?]);
    }

    let body = if let Some(inner) = strip_top_level_wrapping_parens(trimmed) {
        inner.trim()
    } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
        trimmed[1..trimmed.len() - 1].trim()
    } else {
        trimmed
    };
    let parts = custom_split_delim_parenthesized(body, ',');
    let mut values = Vec::new();
    for part in parts {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        values.push(eval_python_row_expr_with_vars(part, vars)?);
    }
    (values.len() == arity).then_some(values)
}

fn parse_python_literal_tuple_rows_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
    arity: usize,
) -> Option<Vec<Vec<i64>>> {
    if arity <= 1 {
        return None;
    }
    let (inner, _) = strip_python_collection_wrappers_with_kind(text)?;
    let inner = inner.trim();
    if inner.is_empty() {
        return Some(Vec::new());
    }

    let mut rows = Vec::new();
    for part in custom_split_delim_parenthesized(inner, ',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        rows.push(parse_python_tuple_row_with_vars(part, vars, arity)?);
    }
    Some(rows)
}

fn parse_python_zip_rows_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
    arity: usize,
) -> Option<Vec<Vec<i64>>> {
    if arity <= 1 {
        return None;
    }
    let inner = parse_python_call_inner(text, "zip")?;
    let parts = custom_split_delim_parenthesized(inner, ',');
    if parts.len() != arity {
        return None;
    }
    let columns = parts
        .iter()
        .map(|part| parse_python_iterable_values_with_vars(part, vars))
        .collect::<Option<Vec<_>>>()?;
    let min_len = columns.iter().map(Vec::len).min().unwrap_or(0);
    let mut rows = Vec::new();
    for index in 0..min_len {
        rows.push(columns.iter().map(|column| column[index]).collect());
    }
    Some(rows)
}

fn parse_python_enumerate_rows_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
    arity: usize,
) -> Option<Vec<Vec<i64>>> {
    if arity != 2 {
        return None;
    }
    let inner = parse_python_call_inner(text, "enumerate")?;
    let parts = custom_split_delim_parenthesized(inner, ',');
    if parts.is_empty() || parts.len() > 2 {
        return None;
    }
    let values = parse_python_iterable_values_with_vars(&parts[0], vars)?;
    let start = if parts.len() == 2 {
        let start_text = parts[1]
            .trim()
            .strip_prefix("start=")
            .unwrap_or(parts[1].trim());
        eval_python_row_expr_with_vars(start_text, vars)?
    } else {
        0
    };
    values
        .into_iter()
        .enumerate()
        .map(|(index, value)| {
            let index = i64::try_from(index).ok()?;
            Some(vec![start.checked_add(index)?, value])
        })
        .collect()
}

fn parse_python_dict_items_rows_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
    arity: usize,
) -> Option<Vec<Vec<i64>>> {
    if arity != 2 {
        return None;
    }
    let base = text.trim().strip_suffix(".items()")?.trim();
    let pairs = parse_python_dict_pairs_with_vars(base, vars)?;
    Some(pairs.into_iter().map(|(key, value)| vec![key, value]).collect())
}

fn python_binding_rows_finish(rows: Vec<Vec<i64>>, kind: PythonCollectionKind) -> Vec<Vec<i64>> {
    if kind.deduplicates_like_python_set_conversion() {
        rows.into_iter().collect::<BTreeSet<_>>().into_iter().collect()
    } else {
        rows
    }
}

fn expand_python_binding_comprehension_rows(
    clauses: &[PythonComprehensionClause],
    clause_index: usize,
    expr_text: &str,
    arity: usize,
    vars: &mut BTreeMap<String, i64>,
    out: &mut Vec<Vec<i64>>,
) -> Option<()> {
    if clause_index >= clauses.len() {
        out.push(parse_python_tuple_row_with_vars(expr_text, vars, arity)?);
        return Some(());
    }

    let clause = &clauses[clause_index];
    let binding_rows = parse_python_iterable_binding_rows_with_vars(
        &clause.source_text,
        vars,
        clause.var_names.len(),
    )?;
    let previous = clause
        .var_names
        .iter()
        .map(|name| vars.get(name).copied())
        .collect::<Vec<_>>();

    for row in binding_rows {
        assign_python_binding_vars(vars, &clause.var_names, &row)?;
        let keep = clause
            .filters
            .iter()
            .map(|filter| eval_python_row_condition_with_vars(filter, vars))
            .collect::<Option<Vec<_>>>()?
            .into_iter()
            .all(|value| value);
        if keep {
            expand_python_binding_comprehension_rows(
                clauses,
                clause_index + 1,
                expr_text,
                arity,
                vars,
                out,
            )?;
        }
    }

    restore_python_binding_vars(vars, &clause.var_names, previous);
    Some(())
}

fn parse_python_generated_binding_rows_inner_with_kind(
    inner: &str,
    kind: PythonCollectionKind,
    outer_vars: &BTreeMap<String, i64>,
    arity: usize,
) -> Option<Vec<Vec<i64>>> {
    if arity <= 1 {
        return None;
    }
    let inner = inner.trim();
    let for_index = find_top_level_keyword(inner, " for ")?;
    let expr_text = inner[..for_index].trim();
    if expr_text.is_empty() {
        return None;
    }
    let tail = inner[for_index + 5..].trim();
    let clauses = parse_python_comprehension_clauses(tail)?;
    let mut vars = outer_vars.clone();
    let mut out = Vec::new();
    expand_python_binding_comprehension_rows(&clauses, 0, expr_text, arity, &mut vars, &mut out)?;
    Some(python_binding_rows_finish(out, kind))
}

fn parse_python_bare_generator_binding_rows_with_vars(
    text: &str,
    outer_vars: &BTreeMap<String, i64>,
    arity: usize,
) -> Option<Vec<Vec<i64>>> {
    parse_python_generated_binding_rows_inner_with_kind(
        text,
        PythonCollectionKind::SequenceLike,
        outer_vars,
        arity,
    )
}

fn parse_python_generated_binding_rows_with_vars(
    text: &str,
    outer_vars: &BTreeMap<String, i64>,
    arity: usize,
) -> Option<Vec<Vec<i64>>> {
    let (inner, kind) = strip_python_collection_wrappers_with_kind(text)?;
    parse_python_generated_binding_rows_inner_with_kind(inner, kind, outer_vars, arity)
}

fn parse_python_builtin_binding_rows_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
    arity: usize,
) -> Option<Vec<Vec<i64>>> {
    if arity == 0 {
        return None;
    }

    if let Some(inner) = parse_python_call_inner(text, "map") {
        let parts = custom_split_delim_parenthesized(inner, ',');
        let function_name = parts.first().map(String::as_str)?.trim();
        if parse_python_lambda_parts(function_name).is_none() {
            return None;
        }
        let columns = parse_python_map_iterable_columns_with_vars(&parts, vars)?;
        let min_len = python_map_min_len(&columns);
        let mut rows = Vec::new();
        for index in 0..min_len {
            let args = columns.iter().map(|column| column[index]).collect::<Vec<_>>();
            let (arg_names, expr_text) = parse_python_lambda_parts(function_name)?;
            if arg_names.len() != args.len() {
                return None;
            }
            let mut scoped_vars = vars.clone();
            assign_python_binding_vars(&mut scoped_vars, &arg_names, &args)?;
            rows.push(parse_python_tuple_row_with_vars(expr_text, &scoped_vars, arity)?);
        }
        return Some(rows);
    }

    for name in ["list", "tuple", "set", "frozenset", "sorted", "reversed"] {
        if let Some(inner) = parse_python_call_inner(text, name) {
            if inner.trim().is_empty() {
                return match name {
                    "list" | "tuple" | "set" | "frozenset" => Some(Vec::new()),
                    _ => None,
                };
            }
            let parts = custom_split_delim_parenthesized(inner, ',');
            if name == "sorted" {
                let reverse = parse_python_sorted_options(&parts)?;
                let mut rows = parse_python_iterable_binding_rows_with_vars(
                    parts.first().map(String::as_str).unwrap_or_default(),
                    vars,
                    arity,
                )?;
                rows.sort();
                if reverse {
                    rows.reverse();
                }
                return Some(rows);
            }
            let source = if name == "reversed" {
                if parts.len() != 1 {
                    return None;
                }
                parts.first().cloned().unwrap_or_default()
            } else {
                inner.to_string()
            };
            let mut rows = parse_python_iterable_binding_rows_with_vars(&source, vars, arity)?;
            if name == "set" || name == "frozenset" {
                rows = rows.into_iter().collect::<BTreeSet<_>>().into_iter().collect();
            }
            if name == "reversed" {
                rows.reverse();
            }
            return Some(rows);
        }
    }

    None
}

fn parse_python_iterable_binding_rows_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
    arity: usize,
) -> Option<Vec<Vec<i64>>> {
    if arity == 0 {
        return None;
    }
    if arity == 1 {
        return parse_python_iterable_values_with_vars(text, vars)
            .map(|values| values.into_iter().map(|value| vec![value]).collect());
    }

    parse_python_builtin_binding_rows_with_vars(text, vars, arity)
        .or_else(|| parse_python_zip_rows_with_vars(text, vars, arity))
        .or_else(|| parse_python_enumerate_rows_with_vars(text, vars, arity))
        .or_else(|| parse_python_dict_items_rows_with_vars(text, vars, arity))
        .or_else(|| parse_python_bare_generator_binding_rows_with_vars(text, vars, arity))
        .or_else(|| parse_python_generated_binding_rows_with_vars(text, vars, arity))
        .or_else(|| parse_python_literal_tuple_rows_with_vars(text, vars, arity))
}

fn expand_python_comprehension_values(
    clauses: &[PythonComprehensionClause],
    clause_index: usize,
    expr_text: &str,
    vars: &mut BTreeMap<String, i64>,
    out: &mut Vec<i64>,
) -> Option<()> {
    if clause_index >= clauses.len() {
        out.push(eval_python_row_expr_with_vars(expr_text, vars)?);
        return Some(());
    }

    let clause = &clauses[clause_index];
    let binding_rows = parse_python_iterable_binding_rows_with_vars(
        &clause.source_text,
        vars,
        clause.var_names.len(),
    )?;
    let previous = clause
        .var_names
        .iter()
        .map(|name| vars.get(name).copied())
        .collect::<Vec<_>>();

    for row in binding_rows {
        assign_python_binding_vars(vars, &clause.var_names, &row)?;
        let keep = clause
            .filters
            .iter()
            .map(|filter| eval_python_row_condition_with_vars(filter, vars))
            .collect::<Option<Vec<_>>>()?
            .into_iter()
            .all(|value| value);
        if keep {
            expand_python_comprehension_values(clauses, clause_index + 1, expr_text, vars, out)?;
        }
    }

    restore_python_binding_vars(vars, &clause.var_names, previous);

    Some(())
}

fn python_collection_values_finish(values: Vec<i64>, kind: PythonCollectionKind) -> Vec<i64> {
    if kind.deduplicates_like_python_set_conversion() {
        values.into_iter().collect::<BTreeSet<_>>().into_iter().collect()
    } else {
        values
    }
}

fn parse_python_generated_row_values_inner_with_kind(
    inner: &str,
    kind: PythonCollectionKind,
    outer_vars: &BTreeMap<String, i64>,
) -> Option<Vec<i64>> {
    let inner = inner.trim();
    if inner.is_empty() {
        return Some(Vec::new());
    }

    if let Some(for_index) = find_top_level_keyword(inner, " for ") {
        let expr_text = inner[..for_index].trim();
        let expr_text = split_top_level_dict_key_value(expr_text)
            .map(|(key, _)| key)
            .unwrap_or(expr_text);
        let tail = inner[for_index + 5..].trim();
        let clauses = parse_python_comprehension_clauses(tail)?;
        let mut vars = outer_vars.clone();
        let mut out = Vec::new();
        expand_python_comprehension_values(&clauses, 0, expr_text, &mut vars, &mut out)?;
        return Some(python_collection_values_finish(out, kind));
    }

    let mut out = Vec::new();
    for part in custom_split_delim_parenthesized(inner, ',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(starred_iterable) = trimmed.strip_prefix('*') {
            out.extend(parse_python_iterable_values_with_vars(starred_iterable, outer_vars)?);
            continue;
        }
        let expr_text = split_top_level_dict_key_value(trimmed)
            .map(|(key, _)| key)
            .unwrap_or(trimmed);
        out.push(eval_python_row_expr_with_vars(expr_text, outer_vars)?);
    }
    Some(python_collection_values_finish(out, kind))
}

fn parse_python_bare_generator_row_values_with_vars(
    text: &str,
    outer_vars: &BTreeMap<String, i64>,
) -> Option<Vec<i64>> {
    find_top_level_keyword(text, " for ")?;
    parse_python_generated_row_values_inner_with_kind(
        text,
        PythonCollectionKind::SequenceLike,
        outer_vars,
    )
}

fn parse_python_generated_row_values_with_vars(
    text: &str,
    outer_vars: &BTreeMap<String, i64>,
) -> Option<Vec<i64>> {
    let (inner, kind) = strip_python_collection_wrappers_with_kind(text)?;
    parse_python_generated_row_values_inner_with_kind(inner, kind, outer_vars)
}

fn python_truth_values_finish(values: Vec<bool>, kind: PythonCollectionKind) -> Vec<bool> {
    if kind.deduplicates_like_python_set_conversion() {
        values.into_iter().collect::<BTreeSet<_>>().into_iter().collect()
    } else {
        values
    }
}

fn expand_python_truth_comprehension_values(
    clauses: &[PythonComprehensionClause],
    clause_index: usize,
    expr_text: &str,
    vars: &mut BTreeMap<String, i64>,
    out: &mut Vec<bool>,
) -> Option<()> {
    if clause_index >= clauses.len() {
        out.push(eval_python_row_condition_with_vars(expr_text, vars)?);
        return Some(());
    }

    let clause = &clauses[clause_index];
    let binding_rows = parse_python_iterable_binding_rows_with_vars(
        &clause.source_text,
        vars,
        clause.var_names.len(),
    )?;
    let previous = clause
        .var_names
        .iter()
        .map(|name| vars.get(name).copied())
        .collect::<Vec<_>>();

    for row in binding_rows {
        assign_python_binding_vars(vars, &clause.var_names, &row)?;
        let keep = clause
            .filters
            .iter()
            .map(|filter| eval_python_row_condition_with_vars(filter, vars))
            .collect::<Option<Vec<_>>>()?
            .into_iter()
            .all(|value| value);
        if keep {
            expand_python_truth_comprehension_values(clauses, clause_index + 1, expr_text, vars, out)?;
        }
    }

    restore_python_binding_vars(vars, &clause.var_names, previous);

    Some(())
}

fn parse_python_truth_generated_values_inner_with_kind(
    inner: &str,
    kind: PythonCollectionKind,
    outer_vars: &BTreeMap<String, i64>,
) -> Option<Vec<bool>> {
    let inner = inner.trim();
    if inner.is_empty() {
        return Some(Vec::new());
    }

    if let Some(for_index) = find_top_level_keyword(inner, " for ") {
        let expr_text = inner[..for_index].trim();
        let expr_text = split_top_level_dict_key_value(expr_text)
            .map(|(key, _)| key)
            .unwrap_or(expr_text);
        let tail = inner[for_index + 5..].trim();
        let clauses = parse_python_comprehension_clauses(tail)?;
        let mut vars = outer_vars.clone();
        let mut out = Vec::new();
        expand_python_truth_comprehension_values(&clauses, 0, expr_text, &mut vars, &mut out)?;
        return Some(python_truth_values_finish(out, kind));
    }

    let mut out = Vec::new();
    for part in custom_split_delim_parenthesized(inner, ',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(starred_iterable) = trimmed.strip_prefix('*') {
            out.extend(parse_python_truth_iterable_values_with_vars(starred_iterable, outer_vars)?);
            continue;
        }
        let expr_text = split_top_level_dict_key_value(trimmed)
            .map(|(key, _)| key)
            .unwrap_or(trimmed);
        out.push(eval_python_row_condition_with_vars(expr_text, outer_vars)?);
    }
    Some(python_truth_values_finish(out, kind))
}

fn parse_python_bare_generator_truth_values_with_vars(
    text: &str,
    outer_vars: &BTreeMap<String, i64>,
) -> Option<Vec<bool>> {
    find_top_level_keyword(text, " for ")?;
    parse_python_truth_generated_values_inner_with_kind(
        text,
        PythonCollectionKind::SequenceLike,
        outer_vars,
    )
}

fn parse_python_truth_generated_values_with_vars(
    text: &str,
    outer_vars: &BTreeMap<String, i64>,
) -> Option<Vec<bool>> {
    let (inner, kind) = strip_python_collection_wrappers_with_kind(text)?;
    parse_python_truth_generated_values_inner_with_kind(inner, kind, outer_vars)
}

fn parse_python_truth_builtin_iterable_values_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
) -> Option<Vec<bool>> {
    if let Some(inner) = parse_python_call_inner(text, "filter") {
        let parts = custom_split_delim_parenthesized(inner, ',');
        if parts.len() != 2 {
            return None;
        }
        let predicate = parts[0].trim();
        let values = parse_python_iterable_values_with_vars(&parts[1], vars)?;
        let mut out = Vec::new();
        for value in values {
            if eval_python_filter_predicate_with_vars(predicate, value, vars)? {
                out.push(value != 0);
            }
        }
        return Some(out);
    }

    if let Some(inner) = parse_python_call_inner(text, "map") {
        let parts = custom_split_delim_parenthesized(inner, ',');
        let function_name = parts.first().map(String::as_str)?.trim();
        let columns = parse_python_map_iterable_columns_with_vars(&parts, vars)?;
        let min_len = python_map_min_len(&columns);
        let mut out = Vec::new();
        for index in 0..min_len {
            let args = columns.iter().map(|column| column[index]).collect::<Vec<_>>();
            let value = if parse_python_lambda_parts(function_name).is_some() {
                eval_python_lambda_condition_with_vars(function_name, &args, vars)?
            } else {
                eval_python_map_function_with_vars(function_name, &args, vars)? != 0
            };
            out.push(value);
        }
        return Some(out);
    }

    for name in ["list", "tuple", "set", "frozenset", "sorted", "reversed"] {
        if let Some(inner) = parse_python_call_inner(text, name) {
            if inner.trim().is_empty() {
                return match name {
                    "list" | "tuple" | "set" | "frozenset" => Some(Vec::new()),
                    _ => None,
                };
            }
            let parts = custom_split_delim_parenthesized(inner, ',');
            if name == "sorted" {
                let reverse = parse_python_sorted_options(&parts)?;
                let mut values = parse_python_truth_iterable_values_with_vars(
                    parts.first().map(String::as_str).unwrap_or_default(),
                    vars,
                )?;
                values.sort();
                if reverse {
                    values.reverse();
                }
                return Some(values);
            }
            let source = if name == "reversed" {
                if parts.len() != 1 {
                    return None;
                }
                parts.first().cloned().unwrap_or_default()
            } else {
                inner.to_string()
            };
            let mut values = parse_python_truth_iterable_values_with_vars(&source, vars)?;
            if name == "set" || name == "frozenset" {
                values = values.into_iter().collect::<BTreeSet<_>>().into_iter().collect();
            }
            if name == "reversed" {
                values.reverse();
            }
            return Some(values);
        }
    }
    None
}

fn parse_python_truth_iterable_values_with_vars(
    text: &str,
    vars: &BTreeMap<String, i64>,
) -> Option<Vec<bool>> {
    parse_python_iterable_values_with_vars(text, vars)
        .map(|values| values.into_iter().map(|value| value != 0).collect())
        .or_else(|| parse_python_truth_builtin_iterable_values_with_vars(text, vars))
        .or_else(|| parse_python_bare_generator_truth_values_with_vars(text, vars))
        .or_else(|| parse_python_truth_generated_values_with_vars(text, vars))
}


fn expand_python_row_generated_values_vielfache(
    values: &[i64],
    max_zahl: Option<i64>,
) -> Vec<i64> {
    let limit = max_zahl.unwrap_or_else(python_row_multiple_limit);
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
    parse_python_str_as_generator_values(body).is_some()
        || parse_python_integer_row_piece_core(body).is_some()
}

fn python_row_piece_to_numbers(
    piece: &str,
    inherited_vielfache: bool,
    max_zahl: Option<i64>,
) -> Option<(bool, Vec<i64>)> {
    let (subtract, inline_vielfache, body) = parse_python_row_piece_flags(piece)?;
    let use_vielfache = inherited_vielfache || inline_vielfache;

    if let Some(values) = parse_python_str_as_generator_values(body) {
        let values = if use_vielfache {
            expand_python_row_generated_values_vielfache(&values, max_zahl)
        } else {
            values
        };
        return Some((subtract, values));
    }

    let (start, end, around) = parse_python_integer_row_piece_core(body)?;
    let values = if use_vielfache {
        expand_python_row_numbers_vielfache(start, end, &around, max_zahl)
    } else {
        expand_python_row_numbers_plain(start, end, &around, max_zahl)
    };

    Some((subtract, values))
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
            if value <= 0 {
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
    python_row_spec_to_numbers_with_options(spec, false, Some(python_row_multiple_limit()))
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



#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TXT {
    text: String,
    platzhalter: String,
    stext: Vec<String>,
    stextS: Vec<String>,
    e: Vec<String>,
    befehlDavor: String,
}

#[allow(non_snake_case)]
impl TXT {
    pub fn new(txt: &str) -> Self {
        let mut out = Self::default();
        out.set_text(txt);
        out
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn platzhalter(&self) -> &str {
        &self.platzhalter
    }

    pub fn liste(&self) -> &[String] {
        &self.stext
    }

    pub fn listeS(&self) -> &[String] {
        &self.stextS
    }

    pub fn e(&self) -> &[String] {
        &self.e
    }

    pub fn listeE(&self) -> Vec<String> {
        self.stext.iter().chain(self.e.iter()).cloned().collect()
    }

    pub fn menge(&self) -> BTreeSet<String> {
        self.stext.iter().cloned().collect()
    }

    pub fn mengeE(&self) -> BTreeSet<String> {
        self.stext.iter().chain(self.e.iter()).cloned().collect()
    }

    pub fn befehlDavor(&self) -> &str {
        &self.befehlDavor
    }

    pub fn set_befehlDavor(&mut self, value: &str) {
        self.befehlDavor = value.to_string();
    }

    pub fn set_platzhalter(&mut self, value: &str) {
        self.platzhalter = value.trim().to_string();
    }

    pub fn set_text(&mut self, value: &str) {
        let trimmed = value.trim().to_string();
        self.text = trimmed.clone();
        if trimmed.starts_with("reta") {
            self.stext = trimmed
                .split_whitespace()
                .filter(|part| !part.trim().is_empty())
                .map(|part| part.trim().to_string())
                .collect();
            self.stextS = trimmed.split_whitespace().map(|part| part.to_string()).collect();
        } else {
            self.stext = libreta_prompt_custom_split(&trimmed);
            self.stextS = self.stext.clone();
        }
    }

    pub fn set_liste(&mut self, value: &[String]) {
        self.stext = value
            .iter()
            .map(|entry| entry.trim().to_string())
            .filter(|entry| !entry.is_empty())
            .collect();
        self.stextS = value
            .iter()
            .flat_map(|entry| libreta_prompt_custom_split(entry))
            .collect();
    }

    pub fn set_e(&mut self, value: Vec<String>) {
        self.e = value;
    }

    pub fn has(&self, hasSet: &BTreeSet<String>) -> bool {
        let menge = self.menge();
        hasSet.iter().any(|entry| menge.contains(entry))
    }

    pub fn hasWithoutABC(&self, hasSet: &BTreeSet<String>) -> bool {
        let menge = self.menge();
        hasSet.iter().any(|entry| menge.contains(entry))
            && !menge.contains("abc")
            && !menge.contains("abcd")
    }
}

/// Python `retaPrompt.dictToList`: return values in insertion order.
#[allow(non_snake_case)]
pub fn dictToList<V: Clone>(dict_: &IndexMap<String, V>) -> Vec<V> {
    dict_.values().cloned().collect()
}

/// Python `retaPrompt.getDictLimtedByKeyList`: OrderedDict over the requested key order.
#[allow(non_snake_case)]
pub fn getDictLimtedByKeyList<V: Clone>(d: &IndexMap<String, V>, keys: &[String]) -> IndexMap<String, V> {
    let mut out = IndexMap::new();
    for key in keys {
        if let Some(value) = d.get(key) {
            out.insert(key.clone(), value.clone());
        }
    }
    out
}

/// Python `retaPrompt.grKl`.
///
/// Returns the elements of `a` greater than `max(b)` and the elements of `a`
/// smaller than `min(b)`.  `BTreeSet` gives the deterministic, set-shaped Rust
/// equivalent while preserving Python's visible membership semantics.
#[allow(non_snake_case)]
pub fn grKl(a: &BTreeSet<i64>, b: &BTreeSet<i64>) -> (BTreeSet<i64>, BTreeSet<i64>) {
    if b.is_empty() {
        return (a.clone(), a.clone());
    }
    let min_b = *b.iter().next().expect("non-empty set has a minimum");
    let max_b = *b.iter().next_back().expect("non-empty set has a maximum");
    let greater = a.iter().copied().filter(|value| *value > max_b).collect();
    let smaller = a.iter().copied().filter(|value| *value < min_b).collect();
    (greater, smaller)
}

/// Python `retaPrompt.returnOnlyParasAsList`.
#[allow(non_snake_case)]
pub fn returnOnlyParasAsList(textList: &[String]) -> Vec<String> {
    textList
        .iter()
        .filter(|token| isReTaParameter(token))
        .cloned()
        .collect()
}

/// Python `retaPrompt.bruchSpalt`.
///
/// The Python function returns `[]` for non-string/invalid input.  Rust callers
/// pass a string slice, so invalid input maps to an empty vector.
#[allow(non_snake_case)]
pub fn bruchSpalt(text: &str) -> Vec<Vec<String>> {
    python_bruch_spalt(text).unwrap_or_default()
}

/// Python `retaPrompt.createRangesForBruchLists`.
///
/// Python returns either `(listenRange, ergebnis2)` or `[]` for illegal shapes.
/// Rust exposes that as `Some((range_values, denominator_spec))` or `None`.
#[allow(non_snake_case)]
pub fn createRangesForBruchLists(bruchList: &[Vec<String>]) -> Option<(Vec<i64>, String)> {
    create_ranges_for_python_bruch_list(bruchList)
        .map(|ranges| (ranges.numerators, ranges.denominator_spec))
}

fn row_numbers_like_python_BereichToNumbers2(text: &str) -> Vec<i64> {
    python_row_spec_to_numbers(text).unwrap_or_default()
}

fn row_numbers_like_python_BereichToNumbers2_unbounded(text: &str) -> Vec<i64> {
    python_row_spec_to_numbers_with_options(text, false, None).unwrap_or_default()
}

/// Python `retaPrompt.findEqualNennerZaehler`.
#[allow(non_snake_case)]
pub fn findEqualNennerZaehler(
    hierBereich: &str,
    nenner: &str,
    mut nennerZaehlerGleich: Vec<String>,
) -> Vec<String> {
    let hier_bereich = row_numbers_like_python_BereichToNumbers2(hierBereich);
    let nenner_values = row_numbers_like_python_BereichToNumbers2(nenner);
    for nn3 in nenner_values {
        for h_b3 in &hier_bereich {
            if nn3 == *h_b3 && nn3 != 0 && nn3 != 1 {
                nennerZaehlerGleich.push(nn3.to_string());
            }
        }
    }
    nennerZaehlerGleich
}

/// Python `retaPrompt.findNennerZaehlerMakesWholeNum` for positive row specs.
#[allow(non_snake_case)]
pub fn findNennerZaehlerMakesWholeNum(
    zaehler: &str,
    nenner: &str,
    mut wholeNumList: Vec<String>,
    mut wholeNumListReziproke: Vec<String>,
) -> (Vec<String>, Vec<String>) {
    let zaehler_values = row_numbers_like_python_BereichToNumbers2(zaehler);
    let nenner_values = row_numbers_like_python_BereichToNumbers2(nenner);
    for nn3 in nenner_values {
        for zz3 in &zaehler_values {
            if *zz3 == 0 || nn3 == 0 {
                continue;
            }
            if nn3 % *zz3 == 0 {
                wholeNumList.push((nn3 / *zz3).to_string());
            }
            if *zz3 % nn3 == 0 {
                wholeNumListReziproke.push((*zz3 / nn3).to_string());
            }
        }
    }
    (wholeNumList, wholeNumListReziproke)
}

/// Python `retaPrompt.anotherOberesMaximum`, parameterized with the Python
/// `tables.hoechsteZeile[1024]` fallback so callers can stay side-effect free.
#[allow(non_snake_case)]
pub fn anotherOberesMaximum(zahlenBereichC: &str, maxNum: i64, max1024: i64) -> String {
    let max_num2 = row_numbers_like_python_BereichToNumbers2_unbounded(zahlenBereichC)
        .into_iter()
        .max()
        .unwrap_or(maxNum);
    format!(
        "--oberesmaximum={}",
        std::cmp::max(std::cmp::max(maxNum, max_num2), max1024) + 1
    )
}

/// Python `retaPrompt.verdreheWoReTaBefehl`.
#[allow(non_snake_case)]
pub fn verdreheWoReTaBefehl(
    text1: &str,
    text2: &str,
    text3: &[String],
    _PromptMode: PromptModus,
) -> (String, String, Vec<String>) {
    if text2.starts_with("reta") && !text1.starts_with("reta") && !text3.is_empty() {
        return (
            text2.to_string(),
            text1.to_string(),
            libreta_prompt_custom_split(text2),
        );
    }
    (text1.to_string(), text2.to_string(), text3.to_vec())
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromptLoescheVorSpeicherungResult {
    pub platzhalter: String,
    pub promptMode: PromptModus,
    pub text: String,
}

fn txt_liste_like_python_retaPrompt(text: &str) -> Vec<String> {
    TXT::new(text).liste().to_vec()
}

fn isZeilenAngabe_like_python_retaPrompt(text: &str) -> bool {
    let parts = custom_split_delim_parenthesized(text, ',');
    let any_at_all = parts.iter().any(|part| !part.is_empty());
    parts
        .iter()
        .all(|part| isZeilenAngabe_betweenKommas(part) || (part.is_empty() && any_at_all))
}

/// Python `retaPrompt.PromptLoescheVorSpeicherungBefehle` without prompt-toolkit side effects.
#[allow(non_snake_case)]
pub fn PromptLoescheVorSpeicherungBefehle(
    platzhalter: &str,
    _promptMode: PromptModus,
    text: &str,
) -> PromptLoescheVorSpeicherungResult {
    let text_trimmed = text.trim().to_string();
    let delete_tokens = txt_liste_like_python_retaPrompt(&text_trimmed);
    let mut placeholder_tokens = txt_liste_like_python_retaPrompt(platzhalter)
        .into_iter()
        .map(Some)
        .collect::<Vec<Option<String>>>();

    let mut remove_by_word = true;
    if isZeilenAngabe_like_python_retaPrompt(&text_trimmed) {
        let placeholder_has_text = placeholder_tokens
            .iter()
            .flatten()
            .any(|token| token == &text_trimmed);
        if !placeholder_has_text || !text_trimmed.chars().all(|ch| ch.is_ascii_digit()) {
            remove_by_word = false;
            for todel in row_numbers_like_python_BereichToNumbers2(&text_trimmed) {
                if todel > 0 {
                    let index = (todel - 1) as usize;
                    if index < placeholder_tokens.len() {
                        placeholder_tokens[index] = None;
                    }
                }
            }
        }
    }

    let (platzhalter_out, text_out) = if remove_by_word {
        let delete_set = delete_tokens.into_iter().collect::<BTreeSet<_>>();
        let kept = placeholder_tokens
            .into_iter()
            .flatten()
            .filter(|token| !delete_set.contains(token))
            .collect::<Vec<_>>();
        (kept.join(" "), String::new())
    } else {
        (
            placeholder_tokens.into_iter().flatten().collect::<Vec<_>>().join(" "),
            text_trimmed,
        )
    };

    PromptLoescheVorSpeicherungResult {
        platzhalter: platzhalter_out,
        promptMode: PromptModus::Normal,
        text: text_out,
    }
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
        Some(python_row_multiple_limit()),
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
                python_row_piece_to_numbers(piece_trimmed, false, Some(python_row_multiple_limit()))
            {
                for value in values {
                    if value <= 0 {
                        continue;
                    }
                    if piece_subtract {
                        negative_primary_numbers.insert(value);
                    } else {
                        primary_numbers.insert(value);
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

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BruchBereichsManagementResult {
    pub zahlenBereichC: String,
    pub zahlenAngaben_: Vec<String>,
    pub zahlenAngabenMehrere: Vec<String>,
    pub rowSpecs: Vec<String>,
    pub primaryRowSpecs: Vec<String>,
    pub reciprocalRowSpecs: Vec<String>,
    pub rawFractionSpecs: Vec<String>,
    pub equalFractionRowSpecs: Vec<String>,
    pub nonWholeFractionDenominatorGroups: BTreeMap<i64, Vec<String>>,
    pub nonWholeFractionNumeratorGroups: BTreeMap<i64, Vec<String>>,
    pub useRange: bool,
    pub useTeiler: bool,
    pub useVielfache: bool,
    pub invertieren: bool,
    pub suppressEmpty: bool,
    pub noHeaders: bool,
    pub extraParams: Vec<String>,
}

impl BruchBereichsManagementResult {
    fn row_buckets(&self) -> PythonRowBuckets {
        PythonRowBuckets {
            primary_row_specs: self.primaryRowSpecs.clone(),
            reciprocal_row_specs: self.reciprocalRowSpecs.clone(),
            raw_fraction_specs: self.rawFractionSpecs.clone(),
            equal_fraction_row_specs: self.equalFractionRowSpecs.clone(),
            non_whole_fraction_denominator_groups: self.nonWholeFractionDenominatorGroups.clone(),
            non_whole_fraction_numerator_groups: self.nonWholeFractionNumeratorGroups.clone(),
        }
    }
}

fn push_unique_many(target: &mut Vec<String>, values: impl IntoIterator<Item = String>) {
    for value in values {
        push_unique_string(target, value);
    }
}

fn bruch_bereichs_management_from_normalized(
    zahlen_bereich_c: &str,
    normalized: &[String],
    zahlen_angaben: &[String],
) -> BruchBereichsManagementResult {
    let row_specs = normalized
        .iter()
        .filter(|token| is_row_spec_token(token))
        .cloned()
        .collect::<Vec<_>>();
    let use_range = normalized.iter().any(|t| t == "range");
    let invertieren = normalized.iter().any(|t| t == "invertieren");
    let use_teiler = normalized.iter().any(|t| t == "teiler");
    let einzeln = normalized.iter().any(|t| t == "einzeln");
    let use_vielfache = !einzeln && normalized.iter().any(|t| t == "vielfache");
    let suppress_empty = normalized
        .iter()
        .any(|t| t == "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar");
    let no_headers = normalized
        .iter()
        .any(|t| t == "ee" || t == "--keineueberschriften");
    let extra_params = extract_passthrough_reta_parameters(normalized);
    let row_buckets = build_python_row_buckets_with_global_vielfache(&row_specs, use_vielfache);

    let mut zahlen_angaben_mehrere = zahlen_angaben.to_vec();
    push_unique_many(&mut zahlen_angaben_mehrere, row_specs.iter().cloned());
    push_unique_many(
        &mut zahlen_angaben_mehrere,
        row_buckets.primary_row_specs.iter().cloned(),
    );
    push_unique_many(
        &mut zahlen_angaben_mehrere,
        row_buckets.reciprocal_row_specs.iter().cloned(),
    );
    push_unique_many(
        &mut zahlen_angaben_mehrere,
        row_buckets.equal_fraction_row_specs.iter().cloned(),
    );

    let zahlen_bereich_c = if zahlen_bereich_c.trim().is_empty() {
        row_specs.join(",")
    } else {
        zahlen_bereich_c.trim().to_string()
    };

    BruchBereichsManagementResult {
        zahlenBereichC: zahlen_bereich_c,
        zahlenAngaben_: zahlen_angaben.to_vec(),
        zahlenAngabenMehrere: zahlen_angaben_mehrere,
        rowSpecs: row_specs,
        primaryRowSpecs: row_buckets.primary_row_specs,
        reciprocalRowSpecs: row_buckets.reciprocal_row_specs,
        rawFractionSpecs: row_buckets.raw_fraction_specs,
        equalFractionRowSpecs: row_buckets.equal_fraction_row_specs,
        nonWholeFractionDenominatorGroups: row_buckets.non_whole_fraction_denominator_groups,
        nonWholeFractionNumeratorGroups: row_buckets.non_whole_fraction_numerator_groups,
        useRange: use_range,
        useTeiler: use_teiler,
        useVielfache: use_vielfache,
        invertieren,
        suppressEmpty: suppress_empty,
        noHeaders: no_headers,
        extraParams: extra_params,
    }
}

/// Python `retaPrompt.bruchBereichsManagementAndWbefehl` as a named,
/// side-effect-free prompt phase.  It keeps the Python separation between raw
/// row/fraction recognition and later `reta` argv rendering: numbers, reciprocal
/// whole-number rows, equal fractions and non-whole fraction groups are collected
/// here, while callers decide which semantic command uses which bucket.
#[allow(non_snake_case)]
pub fn bruchBereichsManagementAndWbefehl(
    zahlenBereichC: &str,
    stext: &[String],
    zahlenAngaben_: &[String],
) -> BruchBereichsManagementResult {
    let normalized = finalize_prompt_tokens_for_execution(stext);
    bruch_bereichs_management_from_normalized(zahlenBereichC, &normalized, zahlenAngaben_)
}


fn prompt_python_default_oberesmaximum_seed() -> i64 {
    // Python retaPrompt liest hier letztlich `tables.hoechsteZeile[1024]` aus dem
    // laufenden Programm oder faellt auf das globale `retaProgram` zurueck.
    // Der Rust-Prompt leitet denselben Seed aus dem generierten Python-Words-
    // Snapshot ab und begrenzt ihn wie Python mindestens auf 1024.
    prompt_python_table_maximum_seed()
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


#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Zeiln1234CreateResult {
    pub zeiln1: String,
    pub zeiln2: String,
    pub zeiln3: String,
    pub zeiln4: String,
    pub zeilenTokens: Vec<String>,
    pub bruchZeilenTokens: Vec<String>,
}

fn prompt_token_set(tokens: &[String]) -> BTreeSet<&str> {
    tokens.iter().map(|token| token.as_str()).collect()
}

fn split_zeiln1234_tokens(tokens: &[String]) -> (String, String) {
    let mut zeiln1 = String::new();
    let mut zeiln2 = String::new();

    for token in tokens {
        if token.trim().is_empty() || token == "--invertieren" {
            continue;
        }

        if token.starts_with("--vielfachevonzahlen=") {
            if zeiln1.is_empty() {
                zeiln1 = token.clone();
            } else if zeiln2.is_empty() {
                zeiln2 = token.clone();
            }
            continue;
        }

        if token.starts_with("--vorhervonausschnitt=") || token.starts_with("--zaehlung=") {
            if zeiln1.is_empty() {
                zeiln1 = token.clone();
            } else if zeiln2.is_empty() {
                zeiln2 = token.clone();
            }
            continue;
        }

        if token.starts_with("--oberesmaximum=") {
            if zeiln2.is_empty() {
                zeiln2 = token.clone();
            }
        }
    }

    (zeiln1, zeiln2)
}

fn row_specs_from_python_text(text: &str) -> Vec<String> {
    let inner = strip_matching_row_wrappers(text.trim());
    custom_split_delim_parenthesized(inner, ',')
        .into_iter()
        .map(|part| part.trim().to_string())
        .filter(|part| !part.is_empty())
        .collect()
}

fn prompt_flags_for_reta_execute(
    invert: bool,
    suppress_empty: bool,
    no_headers: bool,
) -> Vec<String> {
    let mut flags = Vec::new();
    if invert {
        flags.push("invertieren".to_string());
    }
    if suppress_empty {
        flags.push("keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar".to_string());
    }
    if no_headers {
        flags.push("ee".to_string());
    }
    flags
}

fn zeiln1234create_from_row_options(
    row_specs: &[String],
    use_range: bool,
    use_teiler: bool,
    use_vielfache: bool,
    invert: bool,
    bruch_ganzzahl_reziproke: &[String],
    max_num: i64,
) -> Zeiln1234CreateResult {
    let forced_seed = (max_num > prompt_python_default_oberesmaximum_seed()).then_some(max_num);
    let zeilen_tokens = build_python_row_section_with_custom_oberesmaximum(
        row_specs,
        use_range,
        use_teiler,
        use_vielfache,
        invert,
        forced_seed,
    )
    .map(|section| section.tokens)
    .unwrap_or_else(|| vec!["--vorhervonausschnitt=0".to_string()]);
    let (zeiln1, zeiln2) = split_zeiln1234_tokens(&zeilen_tokens);

    let bruch_zeilen_tokens = if bruch_ganzzahl_reziproke.is_empty() {
        vec!["--vorhervonausschnitt=0".to_string()]
    } else {
        build_fractional_prompt_row_section(bruch_ganzzahl_reziproke, use_range, false)
            .map(|section| section.tokens)
            .unwrap_or_else(|| vec!["--vorhervonausschnitt=0".to_string()])
    };
    let (zeiln3, zeiln4) = split_zeiln1234_tokens(&bruch_zeilen_tokens);

    Zeiln1234CreateResult {
        zeiln1,
        zeiln2,
        zeiln3,
        zeiln4,
        zeilenTokens: zeilen_tokens,
        bruchZeilenTokens: bruch_zeilen_tokens,
    }
}

/// Python `retaPrompt.zeiln1234create` as a side-effect-free row-section phase.
///
/// The Python function returns four already formatted `-zeilen` parameters that
/// `retaExecuteNprint()` later places into a `reta` argv.  Rust keeps the same
/// visible pieces and additionally exposes the token vectors so the newer
/// builders do not need to rediscover which branch produced the row selection.
#[allow(non_snake_case)]
pub fn zeiln1234create(
    stextE: &[String],
    bedingungZahl: bool,
    bruch_GanzZahlReziproke: &[String],
    zahlenBereichC: &str,
    maxNum: i64,
    zahlenReiheKeineWteiler: &str,
) -> Zeiln1234CreateResult {
    let token_set = prompt_token_set(stextE);
    let source = if zahlenReiheKeineWteiler.trim().is_empty() {
        zahlenBereichC
    } else {
        zahlenReiheKeineWteiler
    };
    let row_specs = if bedingungZahl {
        row_specs_from_python_text(source)
    } else {
        Vec::new()
    };
    let use_range = token_set.contains("range");
    let use_teiler = token_set.contains("teiler") || token_set.contains("w");
    let einzeln = token_set.contains("einzeln");
    let use_vielfache = !einzeln && (token_set.contains("vielfache") || token_set.contains("v"));
    let invert = token_set.contains("invertieren") || token_set.contains("--invertieren");

    if !bedingungZahl {
        let bruch_zeilen_tokens = if bruch_GanzZahlReziproke.is_empty() {
            vec!["--vorhervonausschnitt=0".to_string()]
        } else {
            build_fractional_prompt_row_section(bruch_GanzZahlReziproke, use_range, false)
                .map(|section| section.tokens)
                .unwrap_or_else(|| vec!["--vorhervonausschnitt=0".to_string()])
        };
        let (zeiln3, zeiln4) = split_zeiln1234_tokens(&bruch_zeilen_tokens);
        return Zeiln1234CreateResult {
            zeiln1: String::new(),
            zeiln2: String::new(),
            zeiln3,
            zeiln4,
            zeilenTokens: Vec::new(),
            bruchZeilenTokens: bruch_zeilen_tokens,
        };
    }

    zeiln1234create_from_row_options(
        &row_specs,
        use_range,
        use_teiler,
        use_vielfache,
        invert,
        bruch_GanzZahlReziproke,
        maxNum,
    )
}

fn push_reta_execute_token(argv: &mut Vec<String>, token: impl Into<String>) {
    let token = token.into();
    if !token.trim().is_empty() {
        argv.push(token);
    }
}

fn cleanup_python_vorhervonausschnitt_zero(argv: &mut Vec<String>) {
    let count = argv
        .iter()
        .filter(|token| token.starts_with("--vorhervonausschnitt="))
        .count();
    if count <= 1 {
        return;
    }
    if let Some(index) = argv
        .iter()
        .position(|token| token == "--vorhervonausschnitt=0")
    {
        argv.remove(index);
    }
}

/// Python `retaPrompt.retaExecuteNprint` up to the point where Python calls
/// `reta.Program(kette, Txt=Txt)`: build the exact `kette`/argv shape once and
/// let the caller decide whether to execute it, batch it, or test it.
#[allow(non_snake_case)]
pub fn retaExecuteNprint(
    ketten: &[String],
    stextE: &[String],
    zeiln1: &str,
    zeiln2: &str,
    welcheSpalten: &[String],
    ErlaubteSpalten: Option<&str>,
) -> Vec<String> {
    let mut kette = vec!["reta".to_string(), "-zeilen".to_string()];
    push_reta_execute_token(&mut kette, zeiln1.to_string());
    push_reta_execute_token(&mut kette, zeiln2.to_string());

    if stextE
        .iter()
        .any(|token| token == "invertieren" || token == "--invertieren")
        && !kette.iter().any(|token| token == "--invertieren")
    {
        kette.push("--invertieren".to_string());
    }

    kette.push("-spalten".to_string());
    push_reta_execute_token(&mut kette, welcheSpalten.join(""));
    kette.push("-ausgabe".to_string());
    kette.push("--breite=0".to_string());

    if let Some(erlaubte_spalten) = ErlaubteSpalten {
        if !erlaubte_spalten.trim().is_empty() {
            kette.push(format!(
                "--spaltenreihenfolgeundnurdiese={}",
                erlaubte_spalten.trim()
            ));
        }
    }

    if stextE
        .iter()
        .any(|token| token == "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar")
        && !kette.iter().any(|token| token == "--keineleereninhalte")
    {
        kette.push("--keineleereninhalte".to_string());
    }
    if stextE
        .iter()
        .any(|token| token == "ee" || token == "--keineueberschriften")
        && !kette.iter().any(|token| token == "--keineueberschriften")
    {
        kette.push("--keineueberschriften".to_string());
    }

    for parameter in returnOnlyParasAsList(stextE) {
        if !kette.iter().any(|token| token == &parameter) {
            kette.push(parameter);
        }
    }
    for token in ketten {
        push_reta_execute_token(&mut kette, token.clone());
    }

    cleanup_python_vorhervonausschnitt_zero(&mut kette);
    kette
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
    let zeiln = if row_specs.is_empty() {
        Zeiln1234CreateResult::default()
    } else {
        zeiln1234create_from_row_options(
            row_specs,
            use_range,
            use_teiler,
            use_vielfache,
            invert,
            &[],
            prompt_python_default_oberesmaximum_seed(),
        )
    };
    let stext_e = prompt_flags_for_reta_execute(invert, suppress_empty, no_headers);
    let mut argv = retaExecuteNprint(
        &[],
        &stext_e,
        &zeiln.zeiln1,
        &zeiln.zeiln2,
        &[para.to_string()],
        cols,
    );
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
    let zeilen_tokens = build_python_row_section_with_custom_oberesmaximum(
        row_specs,
        use_range,
        use_teiler,
        use_vielfache,
        invert,
        Some(python_row_multiple_limit()),
    )
    .map(|section| section.tokens)
    .unwrap_or_else(|| {
        vec![another_oberesmaximum_from_row_specs_with_seed(
            &[],
            python_row_multiple_limit(),
        )]
    });
    let (zeiln1, zeiln2) = split_zeiln1234_tokens(&zeilen_tokens);
    let stext_e = prompt_flags_for_reta_execute(invert, suppress_empty, no_headers);
    let mut argv = retaExecuteNprint(
        &[],
        &stext_e,
        &zeiln1,
        &zeiln2,
        &["--bedeutung=primzahlkreuz".to_string()],
        None,
    );
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
    let section = build_fractional_prompt_row_section(row_specs, use_range, invert)?;
    let (zeiln1, zeiln2) = split_zeiln1234_tokens(&section.tokens);
    let stext_e = prompt_flags_for_reta_execute(invert, suppress_empty, no_headers);
    let mut argv = retaExecuteNprint(
        &[],
        &stext_e,
        &zeiln1,
        &zeiln2,
        &[para.to_string()],
        Some(cols),
    );
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

    let section = build_fractional_prompt_row_section(
        &row_buckets.equal_fraction_row_specs,
        use_range,
        invert,
    )?;
    let (zeiln1, zeiln2) = split_zeiln1234_tokens(&section.tokens);
    let stext_e = prompt_flags_for_reta_execute(invert, suppress_empty, no_headers);
    let mut argv = retaExecuteNprint(
        &[],
        &stext_e,
        &zeiln1,
        &zeiln2,
        &[para.to_string()],
        Some(cols),
    );
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
    let zeilen_tokens = if reciprocal_row_specs.is_empty() {
        let mut tokens = vec!["--vorhervonausschnitt=0".to_string()];
        if invert {
            tokens.push("--invertieren".to_string());
        }
        tokens
    } else {
        build_fractional_prompt_row_section(reciprocal_row_specs, use_range, invert)
            .map(|section| section.tokens)
            .unwrap_or_else(|| vec!["--vorhervonausschnitt=0".to_string()])
    };
    let (zeiln1, zeiln2) = split_zeiln1234_tokens(&zeilen_tokens);
    let stext_e = prompt_flags_for_reta_execute(invert, suppress_empty, no_headers);
    let mut argv = retaExecuteNprint(
        &[],
        &stext_e,
        &zeiln1,
        &zeiln2,
        &[para.to_string()],
        None,
    );
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
                if let Some(value) = semantic_wahl16_value(suffix) {
                    values16.push(value.to_string());
                }
            }
        }
        if token == "16_15" {
            if let Some(value) = semantic_wahl15_value("15") {
                values15.push(value.to_string());
            }
            continue;
        }
        if let Some(suffix) = token.strip_prefix("16_15_") {
            if let Some(value) = semantic_wahl15_value(suffix) {
                values15.push(value.to_string());
            }
            continue;
        }
        if let Some(suffix) = token.strip_prefix("15_") {
            if let Some(value) = semantic_wahl15_value(suffix) {
                values15.push(value.to_string());
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

    let bruch_management = bruch_bereichs_management_from_normalized("", &normalized, &[]);
    let row_specs = &bruch_management.rowSpecs;
    if row_specs.is_empty() {
        return Vec::new();
    }

    let suppress_empty = bruch_management.suppressEmpty;
    let no_headers = bruch_management.noHeaders;
    let use_range = bruch_management.useRange;
    let invert = bruch_management.invertieren;
    let teiler = bruch_management.useTeiler;
    let vielfache = bruch_management.useVielfache;
    let row_buckets = bruch_management.row_buckets();
    let extra_params = bruch_management.extraParams.clone();
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

    let bruch_management = bruch_bereichs_management_from_normalized("", &normalized, &[]);
    let row_buckets = bruch_management.row_buckets();
    let passthrough_params = bruch_management.extraParams.clone();
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

    let use_range = bruch_management.useRange;
    let invert = bruch_management.invertieren;
    let teiler = bruch_management.useTeiler;
    let vielfache = bruch_management.useVielfache;
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

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PromptVorbereitungGrosseAusgabeResult {
    pub IsPureOnlyReTaCmd: bool,
    pub brueche: Vec<String>,
    pub zahlenBereichC: String,
    pub ketten: Vec<String>,
    pub maxNum: i64,
    pub liste: Vec<String>,
    pub zahlenAngaben_: Vec<String>,
    pub ifKurzKurz: bool,
    pub retaCalls: Vec<Vec<String>>,
}

fn prompt_preparation_max_num(tokens: &[String]) -> i64 {
    tokens
        .iter()
        .filter(|token| token.chars().all(|ch| ch.is_ascii_digit()))
        .filter_map(|token| token.parse::<i64>().ok())
        .max()
        .unwrap_or(PYTHON_DEFAULT_OBERESMAXIMUM_FALLBACK)
}

/// Python `retaPrompt.promptVorbereitungGrosseAusgabe` as a testable,
/// side-effect-free phase.  It performs the same high-value prompt preparation
/// steps that feed the big output loop: `TXT` tokenisation, Kurz-Kurz expansion,
/// optional output-mode prefixing, stored-`reta` row overlays, alias/macro
/// replacement and Python-like regex expansion.  The actual row-section rewrite
/// is shared with the dedicated `prepare_prompt_big_output_for_stored_*` helpers
/// below so the interactive and testable paths stay identical.
#[allow(non_snake_case)]
pub fn promptVorbereitungGrosseAusgabe(
    platzhalter: &str,
    promptMode: PromptModus,
    promptMode2: PromptModus,
    promptModeLast: PromptModus,
    text: &str,
    textDazu0: &[String],
) -> PromptVorbereitungGrosseAusgabeResult {
    let raw_txt = TXT::new(text);
    let raw_input_tokens = raw_txt.liste().to_vec();
    let mut reta_calls: Vec<Vec<String>> = Vec::new();
    let mut txt = raw_txt;
    txt.set_platzhalter(platzhalter);

    let mut if_kurz_kurz = false;
    if !txt.liste().is_empty() {
        let (had_kurz_kurz, expanded) = expand_kurz_kurz_befehl(promptMode2, txt.liste());
        if_kurz_kurz = had_kurz_kurz;
        txt.set_liste(&expanded);
    }

    let mut liste = txt.liste().to_vec();
    if promptMode2 == PromptModus::AusgabeSelektiv && promptModeLast == PromptModus::Normal {
        let mut merged = textDazu0.to_vec();
        merged.extend(liste);
        liste = merged;
        txt.set_liste(&liste);
    }

    if promptMode == PromptModus::Normal && !platzhalter.trim().is_empty() {
        let stored_tokens = libreta_prompt_custom_split(platzhalter.trim());
        if let Some(prepared) =
            prepare_prompt_big_output_for_stored_reta(&stored_tokens, &raw_input_tokens)
        {
            if_kurz_kurz |= prepared.had_kurz_kurz;
            liste = prepared.tokens;
            reta_calls = vec![liste.clone()];
        } else if let Some(prepared) =
            prepare_prompt_big_output_for_stored_rows(&stored_tokens, &raw_input_tokens)
        {
            if_kurz_kurz |= prepared.had_kurz_kurz;
            liste = prepared.tokens;
            reta_calls = vec![liste.clone()];
        } else if let Some(calls) = prepare_prompt_big_output_for_stored_reta_prompt_overlay(
            &stored_tokens,
            &raw_input_tokens,
        ) {
            if let Some(first) = calls.first() {
                liste = first.clone();
            }
            reta_calls = calls;
        }
    }

    let max_num = prompt_preparation_max_num(&liste);

    if !matches!(
        liste.first().map(String::as_str),
        Some("shell" | "python" | "abstand")
    ) {
        liste = finalize_prompt_tokens_for_execution(&liste);
    }

    let is_pure_only_reta_cmd = matches!(liste.first().map(String::as_str), Some("reta"));
    let bruch_management = bruch_bereichs_management_from_normalized("", &liste, &[]);

    PromptVorbereitungGrosseAusgabeResult {
        IsPureOnlyReTaCmd: is_pure_only_reta_cmd,
        brueche: bruch_management.rawFractionSpecs.clone(),
        zahlenBereichC: bruch_management.zahlenBereichC.clone(),
        ketten: Vec::new(),
        maxNum: max_num,
        liste,
        zahlenAngaben_: bruch_management.zahlenAngabenMehrere,
        ifKurzKurz: if_kurz_kurz,
        retaCalls: reta_calls,
    }
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
    if split_at > 0 && chars[split_at - 1] == '-' {
        split_at -= 1;
    }

    let prefix = chars[..split_at].iter().collect::<String>();
    let suffix = chars[split_at..].iter().collect::<String>();
    (!suffix.is_empty()).then_some((prefix, suffix))
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use std::collections::BTreeSet;

    use super::{
        anotherOberesMaximum, bruchBereichsManagementAndWbefehl, bruchSpalt,
        build_reta_calls_from_prompt_tokens, createRangesForBruchLists, dictToList,
        expand_kurz_kurz_befehl, findEqualNennerZaehler, findNennerZaehlerMakesWholeNum,
        getDictLimtedByKeyList, grKl, PromptLoescheVorSpeicherungBefehle, TXT,
        expand_python_regex_like_tokens, prepare_prompt_big_output_for_stored_reta,
        prepare_prompt_big_output_for_stored_reta_prompt_overlay,
        prepare_prompt_big_output_for_stored_rows, promptVorbereitungGrosseAusgabe,
        is_15or16_command, is_zeilen_angabe_between_kommas_py, isReTaParameter,
        libreta_prompt_custom_split, libreta_prompt_custom_split2, retaExecuteNprint,
        zeiln1234create,
        libreta_prompt_split_kpattern_commas_py, looks_like_numeric_or_fraction_range,
        python_row_spec_to_numbers, verifyBruchNganzZahlBetweenCommas,
        verifyBruchNganzZahlCommaList, verkuerze_dict, PromptModus,
    };

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    fn string_set(values: &[&str]) -> BTreeSet<String> {
        strings(values).into_iter().collect()
    }

    #[test]
    fn reta_prompt_txt_state_matches_python_property_updates() {
        let mut txt = TXT::new(" a  b (c d) ");
        assert_eq!(txt.text(), "a  b (c d)");
        assert_eq!(txt.liste(), &strings(&["a", "", "b", "(c d)"]));
        assert_eq!(txt.listeS(), txt.liste());
        assert_eq!(txt.menge(), string_set(&["", "a", "b", "(c d)"]));

        txt.set_liste(&strings(&["  x  ", "y z"]));
        assert_eq!(txt.liste(), &strings(&["x", "y z"]));
        assert_eq!(txt.listeS(), &strings(&["x", "y", "z"]));

        txt.set_e(strings(&["ee"]));
        assert_eq!(txt.listeE(), strings(&["x", "y z", "ee"]));
        assert!(txt.has(&string_set(&["x"])));
        assert!(txt.hasWithoutABC(&string_set(&["x"])));
        txt.set_liste(&strings(&["abc", "x"]));
        assert!(!txt.hasWithoutABC(&string_set(&["x"])));
    }

    #[test]
    fn reta_prompt_exact_helpers_keep_python_shapes() {
        let mut dict = IndexMap::new();
        dict.insert("a".to_string(), 1);
        dict.insert("b".to_string(), 2);
        dict.insert("c".to_string(), 3);
        assert_eq!(dictToList(&dict), vec![1, 2, 3]);
        let limited = getDictLimtedByKeyList(&dict, &strings(&["c", "a", "x"]));
        assert_eq!(limited.keys().cloned().collect::<Vec<_>>(), strings(&["c", "a"]));
        assert_eq!(limited.values().copied().collect::<Vec<_>>(), vec![3, 1]);

        let a = BTreeSet::from([1, 5, 9]);
        let b = BTreeSet::from([3, 7]);
        assert_eq!(grKl(&a, &b), (BTreeSet::from([9]), BTreeSet::from([1])));
        assert_eq!(grKl(&a, &BTreeSet::new()), (a.clone(), a));
    }

    #[test]
    fn reta_prompt_fraction_helpers_expose_python_names() {
        let bruch = bruchSpalt("1/2");
        assert_eq!(bruch, vec![vec![], strings(&["1", "2"]), vec![]]);
        assert_eq!(createRangesForBruchLists(&bruch), Some((vec![1], "2".to_string())));

        let bruch_range = bruchSpalt("1/2-3/3");
        assert_eq!(createRangesForBruchLists(&bruch_range), Some((vec![1, 2, 3], "2-3".to_string())));
    }

    #[test]
    fn reta_prompt_number_relation_helpers_follow_python_results() {
        assert_eq!(findEqualNennerZaehler("1-5", "3-7", Vec::new()), strings(&["3", "4", "5"]));
        assert_eq!(
            findNennerZaehlerMakesWholeNum("2-4", "2-8", Vec::new(), Vec::new()),
            (strings(&["1", "1", "2", "1", "3", "2", "4", "2"]), strings(&["1", "2", "1", "1"]))
        );
        assert_eq!(anotherOberesMaximum("3-5", 9, 1024), "--oberesmaximum=1025");
        assert_eq!(anotherOberesMaximum("3-1050", 9, 1024), "--oberesmaximum=1051");
    }


    #[test]
    fn prompt_vorbereitung_grosse_ausgabe_expands_bare_number_defaults() {
        let prepared = promptVorbereitungGrosseAusgabe(
            "",
            PromptModus::Normal,
            PromptModus::Normal,
            PromptModus::Normal,
            "12",
            &[],
        );

        assert!(prepared.ifKurzKurz);
        assert_eq!(prepared.maxNum, 12);
        assert_eq!(prepared.zahlenBereichC, "12".to_string());
        assert_eq!(prepared.zahlenAngaben_, strings(&["12"]));
        assert!(prepared.liste.contains(&"12".to_string()));
        assert!(prepared.liste.contains(&"mulpri".to_string()));
        assert!(prepared.liste.contains(&"multis".to_string()));
        assert!(prepared.liste.contains(&"prim".to_string()));
        assert!(prepared
            .liste
            .contains(&"primfaktorenvergleich".to_string()));
        assert!(prepared.liste.contains(&"absicht".to_string()));
        assert!(prepared.liste.contains(&"thomas".to_string()));
        assert!(prepared.liste.contains(&"teiler".to_string()));
        assert!(prepared
            .liste
            .contains(&"keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar".to_string()));
    }

    #[test]
    fn prompt_vorbereitung_grosse_ausgabe_exact_suffix_adds_no_headers() {
        let prepared = promptVorbereitungGrosseAusgabe(
            "",
            PromptModus::Normal,
            PromptModus::Normal,
            PromptModus::Normal,
            "12 keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar",
            &[],
        );

        assert!(prepared.ifKurzKurz);
        assert!(prepared.liste.contains(&"-ausgabe".to_string()));
        assert!(prepared.liste.contains(&"--keineueberschriften".to_string()));
    }

    #[test]
    fn prompt_vorbereitung_grosse_ausgabe_rewrites_stored_reta_row_overlay() {
        let prepared = promptVorbereitungGrosseAusgabe(
            "reta -zeilen --zeit=heute -spalten --thomas",
            PromptModus::Normal,
            PromptModus::Normal,
            PromptModus::Normal,
            "12-15",
            &[],
        );

        assert!(prepared.IsPureOnlyReTaCmd);
        assert_eq!(
            prepared.liste,
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
    fn prompt_vorbereitung_grosse_ausgabe_rewrites_stored_rows_into_raw_reta() {
        let prepared = promptVorbereitungGrosseAusgabe(
            "12-15 ee",
            PromptModus::Normal,
            PromptModus::Normal,
            PromptModus::Normal,
            "reta -spalten --thomas",
            &[],
        );

        assert!(prepared.IsPureOnlyReTaCmd);
        assert_eq!(prepared.retaCalls.len(), 1);
        assert_eq!(
            prepared.liste,
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
    fn prompt_vorbereitung_grosse_ausgabe_exposes_prompt_overlay_batches() {
        let prepared = promptVorbereitungGrosseAusgabe(
            "reta -ausgabe --nocolor",
            PromptModus::Normal,
            PromptModus::Normal,
            PromptModus::Normal,
            "12 a t",
            &[],
        );

        assert!(prepared.IsPureOnlyReTaCmd);
        assert!(prepared.retaCalls.len() >= 2, "{prepared:?}");
        assert!(prepared
            .retaCalls
            .iter()
            .all(|argv| argv.contains(&"--nocolor".to_string())));
    }

    #[test]
    fn prompt_vorbereitung_grosse_ausgabe_expands_prompt_regex() {
        let prepared = promptVorbereitungGrosseAusgabe(
            "",
            PromptModus::Normal,
            PromptModus::Normal,
            PromptModus::Normal,
            r#"r"absi" 12"#,
            &[],
        );

        assert!(prepared.liste.contains(&"absicht".to_string()));
        assert!(prepared.liste.contains(&"12".to_string()));
        assert_eq!(prepared.zahlenBereichC, "12".to_string());
    }

    #[test]
    fn bruch_bereichs_management_names_python_fraction_buckets() {
        let normal = bruchBereichsManagementAndWbefehl("", &strings(&["2/3"]), &[]);
        assert_eq!(normal.rowSpecs, strings(&["2/3"]));
        assert_eq!(normal.zahlenBereichC, "2/3".to_string());
        assert_eq!(
            normal.nonWholeFractionDenominatorGroups.get(&2),
            Some(&strings(&["3"]))
        );
        assert!(normal.nonWholeFractionNumeratorGroups.is_empty());

        let reverse = bruchBereichsManagementAndWbefehl("", &strings(&["2/5-3/5"]), &[]);
        assert_eq!(
            reverse.nonWholeFractionNumeratorGroups.get(&5),
            Some(&strings(&["2", "3"]))
        );
        assert!(reverse.nonWholeFractionDenominatorGroups.is_empty());
    }

    #[test]
    fn bruch_bereichs_management_tracks_w_and_v_modifiers() {
        let teiler = bruchBereichsManagementAndWbefehl("", &strings(&["12", "w"]), &[]);
        assert_eq!(teiler.rowSpecs, strings(&["12"]));
        assert!(teiler.useTeiler);
        assert!(!teiler.useVielfache);

        let vielfache = bruchBereichsManagementAndWbefehl("", &strings(&["12", "v"]), &[]);
        assert_eq!(vielfache.rowSpecs, strings(&["12"]));
        assert!(vielfache.useVielfache);
        assert!(!vielfache.useTeiler);
    }

    #[test]
    fn zeiln1234create_names_python_row_builder_for_plain_and_vielfache() {
        let plain = zeiln1234create(
            &strings(&[]),
            true,
            &[],
            "12",
            1024,
            "12",
        );
        assert_eq!(plain.zeiln1, "--vorhervonausschnitt=12");
        assert_eq!(plain.zeiln2, "--oberesmaximum=1025");
        assert_eq!(plain.zeiln3, "--vorhervonausschnitt=0");

        let vielfache = zeiln1234create(
            &strings(&["vielfache"]),
            true,
            &[],
            "12",
            1024,
            "12",
        );
        assert_eq!(vielfache.zeiln1, "--vielfachevonzahlen=12");
        assert_eq!(vielfache.zeiln2, "--vorhervonausschnitt=12,v12");
    }

    #[test]
    fn reta_execute_nprint_builds_python_kette_shape() {
        let argv = retaExecuteNprint(
            &strings(&["--nocolor"]),
            &strings(&[
                "invertieren",
                "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar",
                "ee",
            ]),
            "--vorhervonausschnitt=12",
            "--oberesmaximum=1025",
            &strings(&["--grundstrukturen=emotion"]),
            Some("1,2"),
        );
        assert_eq!(argv[0], "reta");
        assert_eq!(argv[1], "-zeilen");
        assert!(argv.iter().any(|token| token == "--invertieren"));
        assert!(argv.iter().any(|token| token == "--grundstrukturen=emotion"));
        assert!(argv.iter().any(|token| token == "--spaltenreihenfolgeundnurdiese=1,2"));
        assert!(argv.iter().any(|token| token == "--keineleereninhalte"));
        assert!(argv.iter().any(|token| token == "--keineueberschriften"));
        assert!(argv.iter().any(|token| token == "--nocolor"));
    }

    #[test]
    fn reta_prompt_delete_before_storage_matches_python_cases() {
        let by_index = PromptLoescheVorSpeicherungBefehle(
            "a b c d",
            PromptModus::Speichern,
            "2-3",
        );
        assert_eq!(by_index.platzhalter, "a d");
        assert_eq!(by_index.promptMode, PromptModus::Normal);
        assert_eq!(by_index.text, "2-3");

        let by_word = PromptLoescheVorSpeicherungBefehle(
            "a b c d",
            PromptModus::Speichern,
            "b d",
        );
        assert_eq!(by_word.platzhalter, "a c");
        assert_eq!(by_word.promptMode, PromptModus::Normal);
        assert_eq!(by_word.text, "");
    }

    #[test]
    fn libreta_prompt_custom_split_keeps_python_empty_and_space_semantics() {
        assert_eq!(
            libreta_prompt_custom_split(" a  b (c d) [e f]"),
            strings(&["", "a", "", "b", "(c d)", "[e f]"])
        );
        assert_eq!(libreta_prompt_custom_split("a "), strings(&["a"]));
    }

    #[test]
    fn libreta_prompt_custom_split2_keeps_python_delimiter_semantics() {
        assert_eq!(
            libreta_prompt_custom_split2("a, b,(c,d),,e", ','),
            strings(&["a", " b", "(c,d)", "", "e"])
        );
        assert_eq!(libreta_prompt_custom_split2("a,", ','), strings(&["a"]));
    }

    #[test]
    fn libreta_prompt_kpattern_split_preserves_python_re_split_edges() {
        assert_eq!(
            libreta_prompt_split_kpattern_commas_py("a,[b,c],d,"),
            strings(&["a", "[b,c]", "d", ""])
        );
        assert_eq!(
            libreta_prompt_split_kpattern_commas_py("a,(b,c),{d,e}"),
            strings(&["a", "(b,c)", "{d,e}"])
        );
    }

    #[test]
    fn libreta_prompt_verify_bruch_ganzzahl_helpers_match_python_lists() {
        assert!(is_zeilen_angabe_between_kommas_py("1-3+5"));
        assert!(is_zeilen_angabe_between_kommas_py("v-3-5+2"));
        assert!(is_zeilen_angabe_between_kommas_py("[n for n in range(3)]"));
        assert!(!is_zeilen_angabe_between_kommas_py("1/2"));
        assert!(!is_zeilen_angabe_between_kommas_py("1,2"));

        let bruch_side = verifyBruchNganzZahlBetweenCommas(
            Vec::new(),
            "1-3",
            Vec::new(),
            strings(&["1/2"]),
            Vec::new(),
            "x",
            Vec::new(),
        );
        assert_eq!(bruch_side.bruchAndGanzZahlEtwaKorrekterBereich, vec![true]);
        assert_eq!(bruch_side.bruchBereichsAngaben, strings(&["1-3"]));
        assert_eq!(bruch_side.bruchRanges, vec![strings(&["1/2"])]);
        assert!(bruch_side.bruchAndGanzZahlEtwaKorrekterBereichAllTrue);

        let ganzzahl_side = verifyBruchNganzZahlBetweenCommas(
            Vec::new(),
            "x",
            Vec::new(),
            Vec::new(),
            Vec::new(),
            "4",
            Vec::new(),
        );
        assert_eq!(ganzzahl_side.zahlenAngaben_, strings(&["4"]));
        assert!(ganzzahl_side.bruchAndGanzZahlEtwaKorrekterBereichAllTrue);

        let both_same_kind = verifyBruchNganzZahlBetweenCommas(
            Vec::new(),
            "1",
            Vec::new(),
            Vec::new(),
            Vec::new(),
            "2",
            Vec::new(),
        );
        assert_eq!(
            both_same_kind.bruchAndGanzZahlEtwaKorrekterBereich,
            vec![false]
        );
        assert!(!both_same_kind.bruchAndGanzZahlEtwaKorrekterBereichAllTrue);

        let comma_result = verifyBruchNganzZahlCommaList(
            Vec::new(),
            "x",
            Vec::new(),
            Vec::new(),
            Vec::new(),
            "4,5",
            Vec::new(),
        );
        assert_eq!(
            comma_result.bruchAndGanzZahlEtwaKorrekterBereich,
            vec![vec![true, true], vec![true, true]]
        );
        assert_eq!(
            comma_result.zahlenAngaben_,
            vec![strings(&["4", "5"]), strings(&["4", "5"])]
        );
        assert!(comma_result.fullBlockIsZahlenbereichAndBruch_Z);

        let python_shared_mutation_shape = verifyBruchNganzZahlCommaList(
            Vec::new(),
            "x",
            Vec::new(),
            Vec::new(),
            Vec::new(),
            "4,x",
            Vec::new(),
        );
        assert_eq!(
            python_shared_mutation_shape.bruchAndGanzZahlEtwaKorrekterBereich,
            vec![vec![true, false], vec![true, false]]
        );
        assert_eq!(
            python_shared_mutation_shape.zahlenAngaben_,
            vec![strings(&["4"]), strings(&["4"])]
        );

        let python_truthy_bug = verifyBruchNganzZahlCommaList(
            Vec::new(),
            "1",
            Vec::new(),
            Vec::new(),
            Vec::new(),
            "2",
            Vec::new(),
        );
        assert_eq!(
            python_truthy_bug.bruchAndGanzZahlEtwaKorrekterBereich,
            vec![vec![false]]
        );
        assert!(python_truthy_bug.fullBlockIsZahlenbereichAndBruch_Z);
    }

    #[test]
    fn libreta_prompt_parameter_detection_matches_python_shape() {
        assert!(isReTaParameter("-zeilen"));
        assert!(isReTaParameter("--zeit=heute"));
        assert!(isReTaParameter("--="));
        assert!(isReTaParameter("--*="));
        assert!(!isReTaParameter("-3"));
        assert!(!isReTaParameter("2/3"));
        assert!(!isReTaParameter("--unbekannt=wert"));
    }

    #[test]
    fn verkuerze_dict_preserves_first_key_per_value_like_python() {
        let reduced = verkuerze_dict(&strings(&["a", "b", "c", "d"])
            .into_iter()
            .zip(strings(&["1", "2", "1", "3"]))
            .collect::<Vec<_>>());
        assert_eq!(
            reduced,
            vec![
                ("a".to_string(), "1".to_string()),
                ("b".to_string(), "2".to_string()),
                ("d".to_string(), "3".to_string()),
            ]
        );
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
    fn semantic_15_16_execution_uses_generated_python_choice_source() {
        assert!(is_15or16_command("16_15"));
        assert!(is_15or16_command("16_15_"));

        let calls = build_reta_calls_from_prompt_tokens(&strings(&[
            "15_15", "15_9_6", "16_15", "12",
        ]));
        assert!(calls.iter().any(|call| call.iter().any(|token| {
            token.starts_with("--grundstrukturen=")
                && token.contains("nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)")
                && token.contains("Größenordnung")
                && !token.contains("pro_contra")
                && !token.contains("strukturgroesse")
        })));
    }

    #[test]
    fn prompt_execution_regex_expands_all_python_main_parameters() {
        let expanded = expand_python_regex_like_tokens(&strings(&[
            "reta",
            "r\"^(h|help|debug|nichts)$\"",
        ]));
        assert_eq!(
            expanded,
            strings(&["reta", "-h", "-help", "-debug", "-nichts"])
        );
    }

    #[test]
    fn prompt_execution_regex_keeps_python_empty_value_parameters_as_flags() {
        let expanded = expand_python_regex_like_tokens(&strings(&[
            "reta",
            "-zeilen",
            "--zaehlung=r\"^1$\"",
        ]));
        assert_eq!(expanded, strings(&["reta", "-zeilen", "--zaehlung"]));
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
    fn oberesmaximum_seed_is_data_backed_but_keeps_python_baseline() {
        assert!(prompt_python_default_oberesmaximum_seed() >= 1024);
        assert!(python_row_multiple_limit() >= prompt_python_default_oberesmaximum_seed() + 4);
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
    fn bare_numeric_row_expands_to_python_default_short_commands() {
        let (_, expanded) = expand_kurz_kurz_befehl(PromptModus::Normal, &strings(&["12"]));
        assert_eq!(
            expanded,
            strings(&[
                "12",
                "mulpri",
                "a",
                "t",
                "w",
                "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar",
            ])
        );

        let calls = build_reta_calls_from_prompt_tokens(&expanded);
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--menschliches=motivation")));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--galaxie=thomas")));
        assert!(calls.iter().all(|call| call
            .iter()
            .any(|token| token == "--vorhervonausschnitt=2,3,4,6,12")));
    }

    #[test]
    fn bare_fraction_row_expands_to_python_default_fraction_commands() {
        let (_, expanded) = expand_kurz_kurz_befehl(PromptModus::Normal, &strings(&["2/3"]));
        for expected in ["2/3", "mulpri", "a", "t", "w", "u", "B", "G", "E", "groesse"] {
            assert!(expanded.contains(&expected.to_string()), "missing {expected} in {expanded:?}");
        }

        let calls = build_reta_calls_from_prompt_tokens(&expanded);
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenuniversum=2")));
        assert!(calls
            .iter()
            .any(|call| call.iter().any(|token| token == "--gebrochenemotion=2")));
    }

    #[test]
    fn exact_mode_single_row_kurz_kurz_adds_python_no_headers_output() {
        let (_, expanded) = expand_kurz_kurz_befehl(
            PromptModus::Normal,
            &strings(&[
                "12",
                "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar",
            ]),
        );
        assert!(expanded.iter().any(|token| token == "-ausgabe"));
        assert!(expanded.iter().any(|token| token == "--keineueberschriften"));

        let calls = build_reta_calls_from_prompt_tokens(&expanded);
        assert!(!calls.is_empty());
        assert!(calls
            .iter()
            .all(|call| call.iter().any(|token| token == "--keineueberschriften")));
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
            Some(vec![2])
        );
    }

    #[test]
    fn python_eval_style_row_collections_accept_concat_union_and_star_unpacking() {
        assert_eq!(python_row_spec_to_numbers("[1,2]+[2,4]"), Some(vec![1, 2, 4]));
        assert_eq!(python_row_spec_to_numbers("{1,2}|{2,5}"), Some(vec![1, 2, 5]));
        assert_eq!(python_row_spec_to_numbers("[*range(1,4),7]"), Some(vec![1, 2, 3, 7]));

        let calls = build_reta_calls_from_prompt_tokens(&strings(&[
            "emotion",
            "[1,2]+[2,4]",
        ]));
        assert_eq!(calls.len(), 1);
        assert!(calls[0]
            .iter()
            .any(|token| token == "--vorhervonausschnitt=1,2,4"));
    }

    #[test]
    fn python_eval_style_row_collections_reject_float_division_like_python() {
        assert_eq!(python_row_spec_to_numbers("[4/2]"), None);
        assert_eq!(python_row_spec_to_numbers("[4//2]"), Some(vec![2]));
    }

    #[test]
    fn python_eval_style_row_collections_accept_common_integer_builtins() {
        assert_eq!(
            python_row_spec_to_numbers("[min(4,2),max(1,3),pow(2,3),abs(-5)]"),
            Some(vec![2, 3, 5, 8])
        );
    }

    #[test]
    fn python_eval_style_row_collections_filter_non_positive_results_like_bereich_to_numbers2() {
        assert_eq!(python_row_spec_to_numbers("[-2,0,3]"), Some(vec![3]));
        assert_eq!(python_row_spec_to_numbers("(1-3,5)"), Some(vec![5]));
        assert_eq!(python_row_spec_to_numbers("([1]+[2])"), None);
        assert!(looks_like_numeric_or_fraction_range("(1-3,5)"));
        assert!(!looks_like_numeric_or_fraction_range("([1]+[2])"));
        assert!(!looks_like_numeric_or_fraction_range("(1-3)+[5]"));
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

    #[test]
    fn python_generator_rows_accept_nested_for_clauses_like_eval() {
        assert_eq!(
            python_row_spec_to_numbers("[i * j for i in range(1,4) for j in range(1,i)]"),
            Some(vec![2, 3, 6])
        );
        assert_eq!(
            python_row_spec_to_numbers("[j for i in range(2,5) for j in range(i, i+2) if j < 5]"),
            Some(vec![2, 3, 4])
        );
    }

    #[test]
    fn python_generator_rows_accept_multiple_filters_and_chained_comparisons() {
        assert_eq!(
            python_row_spec_to_numbers("[n for n in range(10) if 2 < n < 7 if n not in {4}]"),
            Some(vec![3, 5, 6])
        );
        assert_eq!(
            python_row_spec_to_numbers("[n for n in range(10) if not (n < 3 or n > 5)]"),
            Some(vec![3, 4, 5])
        );
    }

    #[test]
    fn python_eval_style_rows_accept_iterable_builtins_inside_expressions() {
        assert_eq!(
            python_row_spec_to_numbers("[sum(range(n)) for n in range(5) if len(range(n)) >= 3]"),
            Some(vec![3, 6])
        );
        assert_eq!(
            python_row_spec_to_numbers("[min(range(2,5)), max(reversed([1,4,3]))]"),
            Some(vec![2, 4])
        );
    }

    #[test]
    fn python_eval_style_rows_accept_dict_keys_like_python_eval_set_conversion() {
        assert_eq!(python_row_spec_to_numbers("{1: 9, 3: 7}"), Some(vec![1, 3]));
        assert_eq!(
            python_row_spec_to_numbers("{n: n * n for n in range(4) if n >= 2}"),
            Some(vec![2, 3])
        );
    }


    #[test]
    fn python_eval_style_rows_follow_python_integer_operator_semantics_deeper() {
        assert_eq!(python_row_spec_to_numbers("[-7//3, -7%3, 7//3, 7%3]"), Some(vec![1, 2]));
        assert_eq!(python_row_spec_to_numbers("[-2**2, (-2)**2]"), Some(vec![4]));
        assert_eq!(python_row_spec_to_numbers("[0b1010 & 0x6, 1_2 << 1, ~-3]"), Some(vec![2, 24]));
    }

    #[test]
    fn python_eval_style_rows_accept_conditional_expressions_and_set_ops_deeper() {
        assert_eq!(
            python_row_spec_to_numbers("[n if n & 1 else 10 for n in range(5) if True]"),
            Some(vec![1, 3, 10])
        );
        assert_eq!(python_row_spec_to_numbers("({1,2,3} - {2}) & {1,3,4}"), Some(vec![1, 3]));
        assert_eq!(python_row_spec_to_numbers("{1,2,3} ^ {3,4}"), Some(vec![1, 2, 4]));
    }

    #[test]
    fn python_eval_style_rows_preserve_sequence_duplicates_before_final_set_conversion_deeper() {
        assert_eq!(python_row_spec_to_numbers("[sum([1,1,2])]"), Some(vec![4]));
        assert_eq!(python_row_spec_to_numbers("[len([1,1,2])]"), Some(vec![3]));
        assert_eq!(python_row_spec_to_numbers("[len(set([1,1,2]))]"), Some(vec![2]));
        assert_eq!(python_row_spec_to_numbers("[sum([n for n in [1,1,2]])]"), Some(vec![4]));
        assert_eq!(python_row_spec_to_numbers("[sum(n for n in [1,1,2])]"), Some(vec![4]));
        assert_eq!(python_row_spec_to_numbers("[sum([1,2,3], 10)]"), Some(vec![16]));
    }

    #[test]
    fn python_eval_style_rows_accept_boolean_generators_for_all_any_deeper() {
        assert_eq!(
            python_row_spec_to_numbers("[7 if all(n < 5 for n in range(5)) else 9]"),
            Some(vec![7])
        );
        assert_eq!(
            python_row_spec_to_numbers("[n for n in range(6) if any(m == n for m in [2,4])]"),
            Some(vec![2, 4])
        );
        assert_eq!(
            python_row_spec_to_numbers("[1 if all([True, 1, not False]) else 2]"),
            Some(vec![1])
        );
    }

    #[test]
    fn python_eval_style_rows_accept_more_iterable_builtins_deeper() {
        assert_eq!(python_row_spec_to_numbers("[]"), Some(Vec::new()));
        assert_eq!(python_row_spec_to_numbers("[0]"), Some(Vec::new()));
        assert_eq!(python_row_spec_to_numbers("5-3"), Some(Vec::new()));
        assert_eq!(python_row_spec_to_numbers("[*divmod(17,5)]"), Some(vec![2, 3]));
        assert_eq!(python_row_spec_to_numbers("[*map(abs, [-2,3])]"), Some(vec![2, 3]));
        assert_eq!(python_row_spec_to_numbers("[*filter(None, [-2,0,3])]"), Some(vec![3]));
        assert_eq!(
            python_row_spec_to_numbers("[n for n in sorted([3,1,2], reverse=True) if n > 1]"),
            Some(vec![2, 3])
        );
        assert_eq!(
            python_row_spec_to_numbers("[7 if all(map(bool, [1,2,3])) else 9]"),
            Some(vec![7])
        );
    }

    #[test]
    fn python_eval_style_rows_accept_math_callables_in_map_filter_deeper() {
        assert_eq!(
            python_row_spec_to_numbers("[*map(math.isqrt, [1,4,9,16])]"),
            Some(vec![1, 2, 3, 4])
        );
        assert_eq!(
            python_row_spec_to_numbers("[*map(math.comb, [5,6], [2,3])]"),
            Some(vec![10, 20])
        );
        assert_eq!(
            python_row_spec_to_numbers("[n for n in filter(math.isqrt, [0,1,4,9])]"),
            Some(vec![1, 4, 9])
        );
        assert_eq!(
            python_row_spec_to_numbers("[*map(math.factorial, [0,3,4])]"),
            Some(vec![1, 6, 24])
        );
    }

    #[test]
    fn python_eval_style_rows_accept_subscripts_slices_and_math_helpers_deeper() {
        assert_eq!(
            python_row_spec_to_numbers("[[10,20,30][1], range(10)[-1], ([1,2]+[3])[2]]"),
            Some(vec![3, 9, 20])
        );
        assert_eq!(python_row_spec_to_numbers("[*range(10)[2:8:2]]"), Some(vec![2, 4, 6]));
        assert_eq!(
            python_row_spec_to_numbers("[math.gcd(84,30), math.isqrt(80), math.comb(5,2)]"),
            Some(vec![6, 8, 10])
        );
    }

    #[test]
    fn python_generator_rows_accept_tuple_unpacking_enumerate_zip_and_items_deeper() {
        assert_eq!(
            python_row_spec_to_numbers("[a + b for a,b in [(1,2),(3,4)]]"),
            Some(vec![3, 7])
        );
        assert_eq!(
            python_row_spec_to_numbers("[i + n for i,n in enumerate([2,4], start=1)]"),
            Some(vec![3, 6])
        );
        assert_eq!(
            python_row_spec_to_numbers("[a * b for a,b in zip([2,3],[5,7])]"),
            Some(vec![10, 21])
        );
        assert_eq!(
            python_row_spec_to_numbers("[n for _, n in enumerate([2,3,4]) if _ > 0]"),
            Some(vec![3, 4])
        );
        assert_eq!(
            python_row_spec_to_numbers("[a * b for a,b in ((n,n+1) for n in range(1,4))]"),
            Some(vec![2, 6, 12])
        );
        assert_eq!(
            python_row_spec_to_numbers("[a + b for a,b in {1:2,3:4}.items()]"),
            Some(vec![3, 7])
        );
    }

    #[test]
    fn python_eval_style_rows_accept_lambda_map_filter_and_builtin_map_deeper() {
        assert_eq!(
            python_row_spec_to_numbers("[*map(lambda n: n*n, range(4))]"),
            Some(vec![1, 4, 9])
        );
        assert_eq!(
            python_row_spec_to_numbers("[n for n in filter(lambda x: x % 2, range(7))]"),
            Some(vec![1, 3, 5])
        );
        assert_eq!(
            python_row_spec_to_numbers("[*map(pow, [2,3], [3,2])]"),
            Some(vec![8, 9])
        );
        assert_eq!(
            python_row_spec_to_numbers("[7 if any(map(lambda n: n == 3, range(5))) else 9]"),
            Some(vec![7])
        );
    }

    #[test]
    fn python_generator_rows_accept_dict_views_and_wrapped_tuple_sources_deeper() {
        assert_eq!(
            python_row_spec_to_numbers("[v for v in {1:2,3:4}.values()]"),
            Some(vec![2, 4])
        );
        assert_eq!(
            python_row_spec_to_numbers("[k + v for k,v in {n:n*n for n in range(1,4)}.items() if v > 1]"),
            Some(vec![6, 12])
        );
        assert_eq!(
            python_row_spec_to_numbers("[a + b for a,b in list(zip([1,2],[3,4]))]"),
            Some(vec![4, 6])
        );
        assert_eq!(
            python_row_spec_to_numbers("[a * b for a,b in reversed([(2,3),(4,5)])]"),
            Some(vec![6, 20])
        );
        assert_eq!(
            python_row_spec_to_numbers("[a + b for a,b in map(lambda n: (n,n+1), range(1,4))]"),
            Some(vec![3, 5, 7])
        );
    }



}
