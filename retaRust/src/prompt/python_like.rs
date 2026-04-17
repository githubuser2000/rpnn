use std::collections::{BTreeMap, BTreeSet};
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
    befehle.push("15_".to_string());
    for key in ["15", "2", "5", "7", "8", "10", "12", "13", "17", "18", "6", "9", "3", "16", "4", "1"] {
        befehle.push(format!("16_15_{key}"));
    }
    for key in ["15", "10", "11"] {
        befehle.push(format!("16_{key}"));
    }
    befehle.push("16_".to_string());

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
                PromptSemanticSpec { names: &["universum"], integer_para: "--universum=transzendentalien", fraction_para: Some("--universum=transzendentaliereziproke"), integer_cols: "1,4", fraction_cols: "1,2" },
                PromptSemanticSpec { names: &["richtung"], integer_para: "--primzahlwirkung=galaxieabsicht", fraction_para: None, integer_cols: "1", fraction_cols: "1" },
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

pub fn build_reta_calls_from_prompt_tokens(tokens: &[String]) -> Vec<Vec<String>> {
    let normalized = expand_python_prompt_macros(tokens);
    if normalized.is_empty() || normalized[0] == "reta" || normalized[0].starts_with('-') {
        return Vec::new();
    }
    if normalized.iter().any(|t| matches!(t.as_str(), "help" | "hilfe" | "befehle" | "kurzbefehle" | "shell" | "python" | "math" | "loggen" | "nichtloggen")) {
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
    let teiler = normalized.iter().any(|t| t == "teiler");
    let vielfache = normalized.iter().any(|t| t == "vielfache");
    let has_fraction = row_specs.iter().any(|t| t.contains('/'));
    let joined_rows = row_specs.join(",");
    let mut calls: Vec<Vec<String>> = Vec::new();
    let mut seen_labels = BTreeSet::new();

    for token in &normalized {
        for spec in semantic_specs() {
            if spec.names.contains(&token.as_str()) {
                let label = spec.names[0].to_string();
                if seen_labels.insert(label) {
                    calls.push(build_single_semantic_call(
                        spec,
                        &joined_rows,
                        has_fraction,
                        use_range,
                        invert,
                        teiler,
                        vielfache,
                        suppress_empty,
                        no_headers,
                    ));
                }
                break;
            }
        }
    }

    if !contains_blocking_abc(&normalized) {
        append_15_16_calls(&mut calls, &normalized, &joined_rows, use_range, invert, teiler, suppress_empty, no_headers);
    }

    calls
}

fn append_15_16_calls(
    calls: &mut Vec<Vec<String>>,
    normalized: &[String],
    joined_rows: &str,
    use_range: bool,
    invert: bool,
    teiler: bool,
    suppress_empty: bool,
    no_headers: bool,
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
            joined_rows,
            use_range,
            invert,
            teiler,
            false,
            suppress_empty,
            no_headers,
            &format!("--multiversum={}", values16.join(",")),
            None,
        ));
    }
    if !values15.is_empty() {
        calls.push(build_general_semantic_call(
            joined_rows,
            use_range,
            invert,
            teiler,
            false,
            suppress_empty,
            no_headers,
            &format!("--grundstrukturen={}", values15.join(",")),
            None,
        ));
    }
}

fn build_single_semantic_call(
    spec: &PromptSemanticSpec,
    joined_rows: &str,
    has_fraction: bool,
    use_range: bool,
    invert: bool,
    teiler: bool,
    vielfache: bool,
    suppress_empty: bool,
    no_headers: bool,
) -> Vec<String> {
    let para = if has_fraction {
        spec.fraction_para.unwrap_or(spec.integer_para)
    } else {
        spec.integer_para
    };
    let cols = if has_fraction {
        spec.fraction_cols
    } else {
        spec.integer_cols
    };
    build_general_semantic_call(
        joined_rows,
        use_range,
        invert,
        teiler,
        vielfache,
        suppress_empty,
        no_headers,
        para,
        Some(cols),
    )
}

fn build_general_semantic_call(
    joined_rows: &str,
    use_range: bool,
    invert: bool,
    teiler: bool,
    vielfache: bool,
    suppress_empty: bool,
    no_headers: bool,
    para: &str,
    cols: Option<&str>,
) -> Vec<String> {
    let row_parameter = if vielfache {
        format!("--vielfachevonzahlen={joined_rows}")
    } else if use_range {
        format!("--zaehlung={joined_rows}")
    } else {
        format!("--vorhervonausschnitt={joined_rows}")
    };
    let mut argv = vec![
        "reta".to_string(),
        "-zeilen".to_string(),
        row_parameter,
    ];
    if teiler {
        argv.push("--vorhervonausschnittteiler".to_string());
    }
    if invert {
        argv.push("--invertieren".to_string());
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
    argv
}

pub fn build_reta_argv_from_prompt_tokens(tokens: &[String]) -> Option<Vec<String>> {
    let semantic_calls = build_reta_calls_from_prompt_tokens(tokens);
    if semantic_calls.len() == 1 {
        return semantic_calls.into_iter().next();
    }

    let normalized = expand_python_prompt_macros(tokens);
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
            "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar" => {}
            "ee" => output_flags.push("--keineueberschriften".to_string()),
            "absicht" | "motiv" | "motive" | "absichten" | "universum" | "thomas" | "geist"
            | "bewusstsein" | "emotion" | "impulse" | "wirklichkeit" | "groesse" | "komplex"
            | "kugeln" | "kreise" | "freiheit" | "gleichheit" | "richtung" | "mond"
            | "alles" | "primzahlkreuz" => output_commands.push(token.clone()),
            other if is_15or16_command(other) || other.starts_with(&prompt_words().eig_prefixes.0) || other.starts_with(&prompt_words().eig_prefixes.1) => {
                output_commands.push(other.to_string());
            }
            _ => {}
        }
    }

    if row_specs.is_empty() || normalized.iter().any(|t| matches!(t.as_str(), "help" | "hilfe" | "befehle" | "kurzbefehle" | "shell" | "python" | "math" | "loggen" | "nichtloggen")) {
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

    if !output_flags.is_empty() {
        argv.push("-ausgabe".to_string());
        for flag in output_flags {
            if !argv.contains(&flag) {
                argv.push(flag);
            }
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
    effective_input = normalize_prompt_tokens(&effective_input);

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
            if trimmed.contains('/') {
                return None;
            }
            if let Some((start, end)) = parse_integer_range_piece(trimmed) {
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
                numbers.push(trimmed.parse::<i64>().ok()?);
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
    use super::prepare_prompt_big_output_for_stored_reta;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    #[test]
    fn stored_reta_placeholder_replaces_zeilen_section_python_like() {
        let prepared = prepare_prompt_big_output_for_stored_reta(
            &strings(&[
                "reta",
                "-zeilen",
                "--zeit=heute",
                "-spalten",
                "--thomas",
            ]),
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
}
