use std::env;
use std::process::{Command, Stdio};

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
    let Some(req) = detect_exact_generator_request(args) else {
        return Ok(false);
    };

    let row_range = req.row_range.unwrap_or_else(|| "1-20".to_string());

    let mut cmd = Command::new("reta");
    cmd.arg("-zeilen")
        .arg(format!("--vorhervonausschnitt={row_range}"))
        .arg("-spalten")
        .arg(format!("--{}={}", req.mode, req.value))
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    match cmd.status() {
        Ok(status) if status.success() => return Ok(true),
        Ok(status) => {
            return Err(format!(
                "Exakter Generator-Bridge-Aufruf `reta --{}={}` fehlgeschlagen mit Exit-Status {}",
                req.mode, req.value, status
            )
            .into())
        }
        Err(first_err) => {
            let current_dir = env::current_dir()?;
            let local_candidates = [
                current_dir.join("src/reta_exact/reta.todel/reta.py"),
                current_dir.join("reta.todel/reta.py"),
                current_dir.join("src/reta.py"),
            ];
            for path in local_candidates {
                if !path.exists() {
                    continue;
                }
                let status = Command::new("python3")
                    .arg(&path)
                    .arg("-zeilen")
                    .arg(format!("--vorhervonausschnitt={row_range}"))
                    .arg("-spalten")
                    .arg(format!("--{}={}", req.mode, req.value))
                    .stdin(Stdio::null())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status();
                match status {
                    Ok(s) if s.success() => return Ok(true),
                    Ok(s) => {
                        return Err(format!(
                            "Exakter Generator-Bridge-Aufruf über {} fehlgeschlagen mit Exit-Status {}",
                            path.display(),
                            s
                        )
                        .into())
                    }
                    Err(_) => continue,
                }
            }
            Err(format!(
                "Konnte den exakten Generator weder über `reta` noch über lokale reta.py starten: {}",
                first_err
            )
            .into())
        }
    }
}
