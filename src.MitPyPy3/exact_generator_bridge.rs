#[derive(Debug, Clone)]
pub struct ExactGeneratorRequest {
    pub mode: String,
    pub value: String,
    pub row_range: Option<String>,
}

fn normalize_key(s: &str) -> String {
    s.to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
}

fn take_arg_value(args: &[String], idx: usize) -> Option<String> {
    args.get(idx + 1).cloned().filter(|s| !s.starts_with('-'))
}

fn parse_row_range(args: &[String]) -> Option<String> {
    let mut i = 0usize;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--vorhervonausschnitt" {
            return take_arg_value(args, i);
        }
        if let Some(v) = arg.strip_prefix("--vorhervonausschnitt=") {
            return Some(v.to_string());
        }
        i += 1;
    }
    None
}

fn parse_spaltenname_pairs(args: &[String]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < args.len() {
        if args[i] == "--spaltenname" {
            if let (Some(ober), Some(unter)) = (take_arg_value(args, i), take_arg_value(args, i + 1)) {
                out.push((ober, unter));
            }
        }
        i += 1;
    }
    out
}

fn map_mode_from_oberkategorie(ober: &str) -> Option<&'static str> {
    let n = normalize_key(ober);
    match n.as_str() {
        "universummetakonkret" | "metakonkret" | "meta" | "konkret" => Some("universummetakonkret"),
        "eigenschaft" | "eigenschaften" | "eigenschaftenn" | "eigenschaften1n" | "konzept" | "konzepte" => {
            Some("eigenschaften")
        }
        _ => None,
    }
}

pub fn detect_exact_generator_request(args: &[String]) -> Option<ExactGeneratorRequest> {
    let row_range = parse_row_range(args);
    let pairs = parse_spaltenname_pairs(args);
    if pairs.len() != 1 {
        return None;
    }

    let (ober, unter) = &pairs[0];
    let mode = map_mode_from_oberkategorie(ober)?;

    Some(ExactGeneratorRequest {
        mode: mode.to_string(),
        value: unter.clone(),
        row_range,
    })
}

pub fn try_run_exact_generator_bridge(
    args: &[String],
) -> Result<bool, Box<dyn std::error::Error>> {
    let _ = detect_exact_generator_request(args);

    // Früher wurde hier extern `reta` bzw. `python3 reta.py` gestartet.
    // Das ist genau die Stelle, an der Rust wieder in Python/PyPy3 zurückfiel.
    // Dieser Rückfall ist jetzt absichtlich entfernt.
    //
    // Die Auflösung der gleichen Anfragen läuft stattdessen vollständig intern
    // über die Rust-Pfade:
    // - column_categories_complete.rs
    // - argument_verarbeiter.rs
    // - kategorie_verarbeiter.rs
    // - generated_columns_words_registry.rs
    //
    // Deshalb hier niemals mehr ein externer Prozessaufruf.
    Ok(false)
}
