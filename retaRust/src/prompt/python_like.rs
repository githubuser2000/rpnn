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

#[derive(Clone, Debug)]
pub struct PromptSemanticCall {
    pub argv: Vec<String>,
    pub label: String,
}

#[derive(Clone, Debug)]
struct PromptSemanticSpec {
    names: &'static [&'static str],
    integer_para: &'static str,
    fraction_para: Option<&'static str>,
    integer_cols: &'static str,
    fraction_cols: &'static str,
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

pub fn build_reta_calls_from_prompt_tokens(tokens: &[String]) -> Vec<PromptSemanticCall> {
    let normalized = normalize_prompt_tokens(tokens);
    if normalized.is_empty() || normalized[0] == "reta" || normalized[0].starts_with('-') {
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

    let suppress_empty = normalized.iter().any(|t| t == "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar");
    let no_headers = normalized.iter().any(|t| t == "ee" || t == "--keineueberschriften");
    let use_range = normalized.iter().any(|t| t == "range");
    let invert = normalized.iter().any(|t| t == "invertieren");
    let has_fraction = row_specs.iter().any(|t| t.contains('/'));
    let joined_rows = row_specs.join(",");
    let command_count = normalized
        .iter()
        .filter(|token| prompt_words().befehle_set.contains(*token))
        .count();

    let specs = semantic_specs();
    let mut calls = Vec::new();
    let mut seen_labels = BTreeSet::new();

    for token in &normalized {
        for spec in specs {
            if spec.names.contains(&token.as_str()) {
                let label = spec.names[0].to_string();
                if seen_labels.insert(label.clone()) {
                    let argv = build_single_semantic_call(
                        spec,
                        &joined_rows,
                        has_fraction,
                        use_range,
                        invert,
                        suppress_empty,
                        no_headers,
                        command_count,
                    );
                    calls.push(PromptSemanticCall { argv, label });
                }
                break;
            }
        }
    }

    calls
}

fn semantic_specs() -> &'static [PromptSemanticSpec] {
    static SPECS: OnceLock<Vec<PromptSemanticSpec>> = OnceLock::new();
    SPECS.get_or_init(|| {
        vec![
            PromptSemanticSpec { names: &["thomas"], integer_para: "--galaxie=thomas", fraction_para: None, integer_cols: "2", fraction_cols: "2" },
            PromptSemanticSpec { names: &["emotion"], integer_para: "--grundstrukturen=emotion", fraction_para: Some("--gebrochenemotion"), integer_cols: "2,3", fraction_cols: "4,5" },
            PromptSemanticSpec { names: &["wirklichkeit"], integer_para: "--grundstrukturen=wirklichkeit", fraction_para: None, integer_cols: "1,2", fraction_cols: "5" },
            PromptSemanticSpec { names: &["triebe"], integer_para: "--grundstrukturen=triebe", fraction_para: None, integer_cols: "1", fraction_cols: "2" },
            PromptSemanticSpec { names: &["impulse"], integer_para: "--grundstrukturen=impulse", fraction_para: None, integer_cols: "1,4", fraction_cols: "3" },
            PromptSemanticSpec { names: &["bewusstsein"], integer_para: "--grundstrukturen=bewusstsein", fraction_para: None, integer_cols: "6", fraction_cols: "7" },
            PromptSemanticSpec { names: &["geist"], integer_para: "--grundstrukturen=geist", fraction_para: None, integer_cols: "3", fraction_cols: "4" },
            PromptSemanticSpec { names: &["freiheit", "gleichheit"], integer_para: "--planet=freiheit", fraction_para: None, integer_cols: "1-4,8", fraction_cols: "5-7" },
            PromptSemanticSpec { names: &["groesse"], integer_para: "--strukturgroesse=organisation", fraction_para: Some("--gebrochengroesse"), integer_cols: "1-3", fraction_cols: "99" },
            PromptSemanticSpec { names: &["kugeln", "kreise"], integer_para: "--universum=kugeln", fraction_para: None, integer_cols: "1-2", fraction_cols: "99" },
            PromptSemanticSpec { names: &["netzwerk"], integer_para: "--universum=netzwerk", fraction_para: None, integer_cols: "1-3", fraction_cols: "99" },
            PromptSemanticSpec { names: &["komplex"], integer_para: "--universum=komplex", fraction_para: None, integer_cols: "1", fraction_cols: "3" },
            PromptSemanticSpec { names: &["absicht", "absichten", "motiv", "motive"], integer_para: "--menschliches=motivation", fraction_para: None, integer_cols: "1", fraction_cols: "3" },
            PromptSemanticSpec { names: &["universum"], integer_para: "--universum=transzendentalien", fraction_para: Some("--universum=transzendentaliereziproke"), integer_cols: "1", fraction_cols: "1" },
            PromptSemanticSpec { names: &["richtung"], integer_para: "--primzahlwirkung=richtung", fraction_para: None, integer_cols: "1", fraction_cols: "1" },
        ]
    }).as_slice()
}

fn build_single_semantic_call(
    spec: &PromptSemanticSpec,
    joined_rows: &str,
    has_fraction: bool,
    use_range: bool,
    invert: bool,
    suppress_empty: bool,
    no_headers: bool,
    command_count: usize,
) -> Vec<String> {
    let (row_flag_name, row_value) = if use_range {
        ("--zaehlung=", joined_rows)
    } else {
        ("--vorhervonausschnitt=", joined_rows)
    };

    let line_arg = format!("{row_flag_name}{row_value}");
    let mut argv = vec![
        "reta".to_string(),
        "-zeilen".to_string(),
        line_arg,
    ];
    if invert {
        argv.push("--invertieren".to_string());
    }
    argv.push("-spalten".to_string());

    let mut selected_cols = spec.integer_cols.to_string();
    let mut para = spec.integer_para.to_string();
    if has_fraction {
        if let Some(frac_para) = spec.fraction_para {
            para = frac_para.to_string();
        }
        selected_cols = spec.fraction_cols.to_string();
    }
    if spec.names[0] == "universum" && command_count <= 2 && !no_headers && !suppress_empty {
        selected_cols = if has_fraction { "1,2".to_string() } else { "1,4".to_string() };
    }

    argv.push(para);
    argv.push("-ausgabe".to_string());
    argv.push(format!("--spaltenreihenfolgeundnurdiese={selected_cols}"));
    argv.push("--breite=0".to_string());
    if suppress_empty {
        argv.push("--keineleereninhalte".to_string());
    }
    if no_headers {
        argv.push("--keineueberschriften".to_string());
    }
    argv
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
