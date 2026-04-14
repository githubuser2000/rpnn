use std::collections::BTreeSet;
use std::sync::OnceLock;

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
        "15", "2", "5", "7", "8", "10", "12", "13", "17", "18", "6", "9", "3", "16",
        "4", "1", "30", "14", "20", "37", "31", "11", "36", "21", "26", "19", "90",
    ] {
        befehle.push(format!("15_{key}"));
    }
    for key in ["15", "2", "5", "7", "8", "10", "12", "13", "17", "18", "6", "9", "3", "16", "4", "1"] {
        befehle.push(format!("16_15_{key}"));
    }
    for key in ["15", "10", "11"] {
        befehle.push(format!("16_{key}"));
    }

    for cmd in [
        "invertieren", "netzwerk", "komplex", "ee", "groesse", "emotion", "freiheit", "gleichheit",
        "kurzbefehle", "leeren", "kugeln", "kreise", "mond", "reta", "absicht", "motiv", "thomas",
        "universum", "impulse", "motive", "absichten", "primfaktorenvergleich", "vielfache", "einzeln",
        "multis", "multis3", "modulo", "prim", "primfaktorzerlegung", "prim24",
        "primfaktorzerlegungModulo24", "help", "hilfe", "abc", "abcd", "alles", "geist", "a", "R",
        "range", "B", "bewusstsein", "E", "G", "u", "I", "T", "W", "wirklichkeit", "triebe",
        "befehle", "t", "richtung", "r", "v", "h", "p", "primzahlkreuz", "ende", "exit", "quit",
        "q", ":q", "shell", "s", "math", "loggen", "nichtloggen", "mulpri", "python", "w", "teiler",
        "BefehlSpeichernDanach", "S", "BefehlSpeicherungLöschen", "l", "BefehlSpeicherungAusgeben", "o",
        "e", "BefehlSpeichernDavor", "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar", "abstand",
        "abstandPrim",
    ] {
        befehle.push(cmd.to_string());
    }

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
    tokens.iter().map(|token| replace_prompt_alias(token)).collect()
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
    let mut stack: Vec<char> = Vec::new();
    let mut result = Vec::new();
    let mut current = String::new();

    for ch in text.chars() {
        match ch {
            '(' | '[' | '{' => {
                stack.push(ch);
                current.push(ch);
            }
            ')' | ']' | '}' => {
                if !stack.is_empty() {
                    stack.pop();
                }
                current.push(ch);
            }
            c if c.is_whitespace() && stack.is_empty() => {
                if !current.is_empty() {
                    result.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        result.push(current);
    }
    result
}

pub fn custom_split_delim_parenthesized(text: &str, delimiter: char) -> Vec<String> {
    let mut stack: Vec<char> = Vec::new();
    let mut result = Vec::new();
    let mut current = String::new();

    for ch in text.chars() {
        match ch {
            '(' | '[' | '{' => {
                stack.push(ch);
                current.push(ch);
            }
            ')' | ']' | '}' => {
                if !stack.is_empty() {
                    stack.pop();
                }
                current.push(ch);
            }
            c if c == delimiter && stack.is_empty() => {
                result.push(std::mem::take(&mut current));
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        result.push(current);
    }
    result
}

pub fn looks_like_numeric_or_fraction_range(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    let trimmed = text.trim_matches(|c| matches!(c, '(' | ')' | '[' | ']' | '{' | '}'));
    if trimmed.is_empty() {
        return false;
    }
    custom_split_delim_parenthesized(trimmed, ',')
        .into_iter()
        .all(|part| looks_like_single_numeric_or_fraction_part(part.trim()))
}

pub fn is_row_spec_token(text: &str) -> bool {
    looks_like_numeric_or_fraction_range(text)
}

fn looks_like_single_numeric_or_fraction_part(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return false;
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
        let known_direct = is_15or16_command(&s) || words.befehle_set.contains(&s) || first_token_is_reta;

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
                        .filter(|t| *t != "e" && *t != "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar")
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
                        text_dazu.extend([
                            "mulpri", "a", "t", "w", "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar",
                        ].into_iter().map(|s| s.to_string()));
                        if tokens.iter().any(|t| t.contains('/')) {
                            text_dazu.extend(["u", "B", "G", "E", "groesse"].into_iter().map(|s| s.to_string()));
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

    if matches!(tokens.first().map(|s| s.as_str()), Some("reta" | "shell" | "python")) {
        (if_kurz_kurz, tokens.to_vec())
    } else {
        (if_kurz_kurz, stext3)
    }
}

#[derive(Clone, Copy)]
struct SemanticSpec {
    trigger_tokens: &'static [&'static str],
    selector_n: &'static str,
    selector_fraction: Option<&'static str>,
    allowed_n: Option<&'static str>,
    allowed_fraction: Option<&'static str>,
}

fn semantic_specs(universum_simple: bool) -> Vec<SemanticSpec> {
    vec![
        SemanticSpec {
            trigger_tokens: &["absicht", "absichten", "motiv", "motive"],
            selector_n: "--menschliches=motivation",
            selector_fraction: None,
            allowed_n: Some("1"),
            allowed_fraction: Some("3"),
        },
        SemanticSpec {
            trigger_tokens: &["thomas"],
            selector_n: "--galaxie=thomas",
            selector_fraction: None,
            allowed_n: Some("2"),
            allowed_fraction: Some("2"),
        },
        SemanticSpec {
            trigger_tokens: &["universum"],
            selector_n: "--universum=transzendentalien",
            selector_fraction: Some("--universum=transzendentaliereziproke"),
            allowed_n: Some(if universum_simple { "1,4" } else { "1" }),
            allowed_fraction: Some(if universum_simple { "1,2" } else { "1" }),
        },
        SemanticSpec {
            trigger_tokens: &["emotion"],
            selector_n: "--grundstrukturen=emotion",
            selector_fraction: None,
            allowed_n: Some("2,3"),
            allowed_fraction: Some("4,5"),
        },
        SemanticSpec {
            trigger_tokens: &["wirklichkeit"],
            selector_n: "--grundstrukturen=Wirklichkeiten_Wahrheit_Wahrnehmung_(10)",
            selector_fraction: None,
            allowed_n: Some("1,2"),
            allowed_fraction: Some("5"),
        },
        SemanticSpec {
            trigger_tokens: &["triebe"],
            selector_n: "--grundstrukturen=Triebe_und_Bedürfnisse_(6)",
            selector_fraction: None,
            allowed_n: Some("1"),
            allowed_fraction: Some("2"),
        },
        SemanticSpec {
            trigger_tokens: &["impulse"],
            selector_n: "--grundstrukturen=Impulse_(5)",
            selector_fraction: None,
            allowed_n: Some("1,4"),
            allowed_fraction: Some("3"),
        },
        SemanticSpec {
            trigger_tokens: &["bewusstsein"],
            selector_n: "--grundstrukturen=Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity,nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)",
            selector_fraction: None,
            allowed_n: Some("6"),
            allowed_fraction: Some("7"),
        },
        SemanticSpec {
            trigger_tokens: &["geist"],
            selector_n: "--grundstrukturen=geist",
            selector_fraction: None,
            allowed_n: Some("3"),
            allowed_fraction: Some("4"),
        },
        SemanticSpec {
            trigger_tokens: &["freiheit", "gleichheit"],
            selector_n: "--planet=freiheit",
            selector_fraction: None,
            allowed_n: Some("1-4,8"),
            allowed_fraction: Some("5-7"),
        },
        SemanticSpec {
            trigger_tokens: &["groesse"],
            selector_n: "--strukturgroesse=organisation",
            selector_fraction: None,
            allowed_n: Some("1-3"),
            allowed_fraction: Some("99"),
        },
        SemanticSpec {
            trigger_tokens: &["groesse"],
            selector_n: "--strukturgroesse=strukturgroesse",
            selector_fraction: None,
            allowed_n: Some("1,2"),
            allowed_fraction: Some("4"),
        },
        SemanticSpec {
            trigger_tokens: &["kugeln", "kreise"],
            selector_n: "--universum=kugeln",
            selector_fraction: None,
            allowed_n: Some("1-2"),
            allowed_fraction: Some("99"),
        },
        SemanticSpec {
            trigger_tokens: &["netzwerk"],
            selector_n: "--universum=netzwerk",
            selector_fraction: None,
            allowed_n: Some("1-3"),
            allowed_fraction: Some("99"),
        },
        SemanticSpec {
            trigger_tokens: &["komplex"],
            selector_n: "--universum=komplex",
            selector_fraction: None,
            allowed_n: Some("1"),
            allowed_fraction: Some("3"),
        },
        SemanticSpec {
            trigger_tokens: &["mond"],
            selector_n: "--bedeutung=gestirn",
            selector_fraction: None,
            allowed_n: Some("3-6"),
            allowed_fraction: Some("3-6"),
        },
        SemanticSpec {
            trigger_tokens: &["primzahlkreuz"],
            selector_n: "--bedeutung=primzahlkreuz",
            selector_fraction: None,
            allowed_n: None,
            allowed_fraction: None,
        },
        SemanticSpec {
            trigger_tokens: &["richtung"],
            selector_n: "--primzahlwirkung=Galaxieabsicht",
            selector_fraction: None,
            allowed_n: None,
            allowed_fraction: None,
        },
        SemanticSpec {
            trigger_tokens: &["alles"],
            selector_n: "--alles",
            selector_fraction: None,
            allowed_n: None,
            allowed_fraction: None,
        },
    ]
}

fn has_any_token(tokens: &[String], candidates: &[&str]) -> bool {
    candidates.iter().any(|candidate| tokens.iter().any(|token| token == candidate))
}

fn contains_fraction_rows(row_specs: &[String]) -> bool {
    row_specs.iter().any(|row| row.contains('/'))
}

fn line_argument_from_tokens(tokens: &[String], row_specs: &[String]) -> (String, Vec<String>) {
    let joined_rows = row_specs.join(",");
    let mut extra_line_flags: Vec<String> = Vec::new();

    if tokens.iter().any(|t| t == "teiler") {
        extra_line_flags.push("--vorhervonausschnittteiler".to_string());
    }
    if tokens.iter().any(|t| t == "invertieren") {
        extra_line_flags.push("--invertieren".to_string());
    }

    let line_parameter = if tokens.iter().any(|t| t == "vielfache") {
        format!("--vielfachevonzahlen={joined_rows}")
    } else if tokens.iter().any(|t| t == "range") {
        format!("--zaehlung={joined_rows}")
    } else {
        format!("--vorhervonausschnitt={joined_rows}")
    };

    (line_parameter, extra_line_flags)
}

fn generic_passthrough_flags(tokens: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for token in tokens {
        if token.starts_with('-') && token != "-zeilen" && token != "-spalten" && token != "-kombination" && token != "-ausgabe" {
            out.push(token.clone());
        }
    }
    if tokens.iter().any(|t| t == "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar") {
        out.push("--keineleereninhalte".to_string());
    }
    if tokens.iter().any(|t| t == "ee") {
        out.push("--keineueberschriften".to_string());
    }
    out
}

pub fn build_reta_calls_from_prompt_tokens(tokens: &[String]) -> Vec<Vec<String>> {
    let normalized = normalize_prompt_tokens(tokens);
    if normalized.is_empty() {
        return Vec::new();
    }
    if normalized[0] == "reta" || normalized[0].starts_with('-') {
        return Vec::new();
    }

    let row_specs: Vec<String> = normalized.iter().filter(|t| is_row_spec_token(t)).cloned().collect();
    if row_specs.is_empty() {
        return Vec::new();
    }

    if normalized.iter().any(|t| matches!(t.as_str(), "help" | "hilfe" | "befehle" | "kurzbefehle" | "shell" | "python" | "math" | "loggen" | "nichtloggen")) {
        return Vec::new();
    }

    let universum_simple = normalized.len() <= 2
        && !normalized.iter().any(|t| matches!(t.as_str(), "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar" | "ee" | "--keineueberschriften"));
    let specs = semantic_specs(universum_simple);
    let has_fraction = contains_fraction_rows(&row_specs);
    let (line_parameter, extra_line_flags) = line_argument_from_tokens(&normalized, &row_specs);
    let passthrough_flags = generic_passthrough_flags(&normalized);

    let mut calls: Vec<Vec<String>> = Vec::new();
    for spec in specs {
        if !has_any_token(&normalized, spec.trigger_tokens) {
            continue;
        }

        let selector = if has_fraction {
            spec.selector_fraction.unwrap_or(spec.selector_n)
        } else {
            spec.selector_n
        };
        let allowed = if has_fraction {
            spec.allowed_fraction.or(spec.allowed_n)
        } else {
            spec.allowed_n
        };

        let mut argv = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            line_parameter.clone(),
        ];
        argv.extend(extra_line_flags.iter().cloned());
        argv.push("-spalten".to_string());
        argv.push(selector.to_string());
        argv.push("-ausgabe".to_string());
        argv.push("--breite=0".to_string());
        if let Some(allowed_cols) = allowed {
            argv.push(format!("--spaltenreihenfolgeundnurdiese={allowed_cols}"));
        }
        for flag in &passthrough_flags {
            if !argv.contains(flag) {
                argv.push(flag.clone());
            }
        }
        calls.push(argv);
    }

    if calls.is_empty() {
        if let Some(single) = build_reta_argv_from_prompt_tokens(tokens) {
            calls.push(single);
        }
    }

    calls
}

pub fn build_reta_argv_from_prompt_tokens(tokens: &[String]) -> Option<Vec<String>> {
    let normalized = normalize_prompt_tokens(tokens);
    if normalized.is_empty() {
        return None;
    }
    if normalized[0] == "reta" || normalized[0].starts_with('-') {
        return None;
    }

    let mut row_specs: Vec<String> = Vec::new();
    let mut output_commands: Vec<String> = Vec::new();
    let mut output_special_flags: Vec<String> = Vec::new();
    let mut line_flags: Vec<String> = Vec::new();
    let mut output_flags: Vec<String> = Vec::new();

    for token in &normalized {
        if is_row_spec_token(token) {
            row_specs.push(token.clone());
            continue;
        }
        match token.as_str() {
            "vielfache" | "einzeln" => {}
            "teiler" => line_flags.push("--vorhervonausschnittteiler".to_string()),
            "invertieren" => line_flags.push("--invertieren".to_string()),
            "range" => output_special_flags.push("range".to_string()),
            "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar" => output_flags.push("--keineleereninhalte".to_string()),
            "ee" => output_flags.push("--keineueberschriften".to_string()),
            "absicht" | "motiv" | "motive" | "absichten" | "universum" | "thomas" | "geist"
            | "bewusstsein" | "emotion" | "impulse" | "wirklichkeit" | "groesse" | "komplex"
            | "kugeln" | "kreise" | "freiheit" | "gleichheit" | "richtung" | "mond"
            | "alles" | "primzahlkreuz" | "netzwerk" | "triebe" => output_commands.push(token.clone()),
            other if is_15or16_command(other) || other.starts_with(&prompt_words().eig_prefixes.0) || other.starts_with(&prompt_words().eig_prefixes.1) => {
                output_commands.push(other.to_string());
            }
            _ => {}
        }
    }

    if row_specs.is_empty() {
        return None;
    }

    if output_commands.is_empty() {
        output_commands.extend([
            "absicht".to_string(),
            "thomas".to_string(),
        ]);
        if normalized.iter().any(|t| t.contains('/')) {
            output_commands.extend([
                "universum".to_string(),
                "bewusstsein".to_string(),
                "geist".to_string(),
                "emotion".to_string(),
                "groesse".to_string(),
            ]);
        }
        line_flags.push("--vorhervonausschnittteiler".to_string());
    }

    let joined_rows = row_specs.join(",");
    let line_parameter = if normalized.iter().any(|t| t == "vielfache") {
        format!("--vielfachevonzahlen={joined_rows}")
    } else if output_special_flags.iter().any(|flag| flag == "range") {
        format!("--zaehlung={joined_rows}")
    } else {
        format!("--vorhervonausschnitt={joined_rows}")
    };

    let mut argv = vec!["reta".to_string(), "-zeilen".to_string(), line_parameter];
    for flag in line_flags {
        if !argv.contains(&flag) {
            argv.push(flag);
        }
    }

    argv.push("-spalten".to_string());
    for command in output_commands {
        argv.push(format!("--{command}"));
    }

    argv.push("-ausgabe".to_string());
    argv.push("--breite=0".to_string());
    for flag in output_flags {
        if !argv.contains(&flag) {
            argv.push(flag);
        }
    }

    Some(argv)
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
    let mut n = split_at?;
    if n > 0 && chars.get(n.wrapping_sub(1)) == Some(&'-') {
        n -= 1;
    }
    let prefix: String = chars[..n].iter().collect();
    let suffix: String = chars[n..].iter().collect();
    if suffix.is_empty() {
        None
    } else {
        Some((prefix, suffix))
    }
}
