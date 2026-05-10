use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug, Serialize)]
struct HtmlElementMeta {
    tag: String,
    classes: Vec<String>,
    class_string: String,
    attributes: BTreeMap<String, String>,
    html: String,
}

#[derive(Clone, Debug, Serialize)]
struct HeaderCellMeta {
    column_number: i64,
    row_number: Option<i64>,
    tag: String,
    classes: Vec<String>,
    class_string: String,
    class_attributes: Vec<String>,
    extra_class_strings: Vec<String>,
    all_classes: Vec<String>,
    data_attributes: BTreeMap<String, String>,
    attributes: Vec<(String, String)>,
    attributes_first: BTreeMap<String, String>,
    text: String,
    raw_open_tag: String,
    raw_html: String,
    html_elements: Vec<HtmlElementMeta>,
}

#[derive(Clone, Debug)]
struct Config {
    out_path: PathBuf,
    repo_root: Option<PathBuf>,
    stdin_html: bool,
    reta_bin: Option<PathBuf>,
}

fn help_text(program_name: &str) -> String {
    format!(
        r#"{program_name} - extrahiert reta-HTML-Kopfzellen als JSONL

Aufruf:
  {program_name} [out_path] [repo_root]
  {program_name} --stdin-html [out_path]
  {program_name} --reta-bin <pfad> [out_path]

Standard:
  out_path  = ./htmlclassesPy.jsonl
  repo_root = aktuelles Verzeichnis

Das ist das Rust-Gegenstück zu reta_extract_html_classes.py. Es erzeugt pro
Kopfzelle eine kompakte JSON-Zeile mit classes, data-* Attributen, Roh-HTML und
Text. Mit --stdin-html wird kein Programm gestartet; dann wird HTML von stdin
geparst.
"#
    )
}

fn parse_args(argv: &[String]) -> Config {
    let mut out_path: Option<PathBuf> = None;
    let mut repo_root: Option<PathBuf> = None;
    let mut stdin_html = false;
    let mut reta_bin = std::env::var_os("RETA_BIN").map(PathBuf::from);

    let mut positional = Vec::new();
    let mut index = 1usize;
    while index < argv.len() {
        match argv[index].as_str() {
            "-h" | "--help" | "help" => {
                let program_name = argv
                    .first()
                    .map(String::as_str)
                    .unwrap_or("reta_extract_html_classes");
                print!("{}", help_text(program_name));
                std::process::exit(0);
            }
            "--stdin-html" => stdin_html = true,
            "--reta-bin" => {
                index += 1;
                if index >= argv.len() {
                    eprintln!("--reta-bin erwartet einen Pfad");
                    std::process::exit(2);
                }
                reta_bin = Some(PathBuf::from(&argv[index]));
            }
            other => positional.push(PathBuf::from(other)),
        }
        index += 1;
    }

    if let Some(path) = positional.first() {
        out_path = Some(path.clone());
    }
    if let Some(path) = positional.get(1) {
        repo_root = Some(path.clone());
    }

    Config {
        out_path: out_path.unwrap_or_else(|| PathBuf::from("htmlclassesPy.jsonl")),
        repo_root,
        stdin_html,
        reta_bin,
    }
}

fn find_repo_root(explicit: Option<&Path>) -> PathBuf {
    let mut candidates = Vec::new();
    if let Some(path) = explicit {
        candidates.push(path.to_path_buf());
    }
    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd.clone());
        if let Some(parent) = cwd.parent() {
            candidates.push(parent.to_path_buf());
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.to_path_buf());
            if let Some(parent) = dir.parent() {
                candidates.push(parent.to_path_buf());
            }
            if let Some(grandparent) = dir.parent().and_then(Path::parent) {
                candidates.push(grandparent.to_path_buf());
            }
        }
    }

    for candidate in candidates {
        if (candidate.join("Cargo.toml")).exists() || (candidate.join("reta.py")).exists() {
            return candidate;
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn candidate_reta_bins(repo_root: &Path, explicit: Option<&Path>) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(path) = explicit {
        candidates.push(path.to_path_buf());
    }
    candidates.push(repo_root.join("target").join("debug").join("reta"));
    candidates.push(repo_root.join("target").join("release").join("reta"));
    candidates.push(repo_root.join("reta"));
    candidates.push(PathBuf::from("reta"));
    candidates
}

fn run_reta_html(repo_root: &Path, explicit_reta_bin: Option<&Path>) -> Result<String, String> {
    let args = [
        "-zeilen",
        "--vorhervonausschnitt=1",
        "-spalten",
        "--alles",
        "-ausgabe",
        "--art=html",
    ];

    let mut errors = Vec::new();
    for candidate in candidate_reta_bins(repo_root, explicit_reta_bin) {
        let output = Command::new(&candidate)
            .args(args)
            .current_dir(repo_root)
            .output();
        match output {
            Ok(output) if output.status.success() => {
                return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
            }
            Ok(output) => {
                errors.push(format!(
                    "{}: exit {:?}: {}",
                    candidate.display(),
                    output.status.code(),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
            Err(error) => errors.push(format!("{}: {error}", candidate.display())),
        }
    }

    Err(format!(
        "Konnte reta-HTML nicht erzeugen. Setze RETA_BIN oder nutze --reta-bin. Versuche:\n{}",
        errors.join("\n")
    ))
}

fn read_stdin_html() -> Result<String, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("stdin konnte nicht gelesen werden: {error}"))?;
    Ok(input)
}

fn find_first_row(html: &str) -> Result<&str, String> {
    let start = html
        .find("<tr")
        .ok_or_else(|| "Konnte die HTML-Kopfzeile im reta-Output nicht finden.".to_string())?;
    let open_end = html[start..]
        .find('>')
        .map(|offset| start + offset + 1)
        .ok_or_else(|| "Kopfzeilen-<tr> ist unvollständig.".to_string())?;
    let close = html[open_end..]
        .find("</tr>")
        .map(|offset| open_end + offset)
        .ok_or_else(|| "Kopfzeilen-</tr> fehlt.".to_string())?;
    Ok(&html[open_end..close])
}

fn parse_attrs(open_tag: &str) -> Vec<(String, String)> {
    let bytes = open_tag.as_bytes();
    let mut attrs = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        while i < bytes.len() && !is_attr_name_start(bytes[i] as char) {
            i += 1;
        }
        let name_start = i;
        while i < bytes.len() && is_attr_name_char(bytes[i] as char) {
            i += 1;
        }
        if name_start == i {
            continue;
        }
        let name = &open_tag[name_start..i];
        while i < bytes.len() && (bytes[i] as char).is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b'=' {
            continue;
        }
        i += 1;
        while i < bytes.len() && (bytes[i] as char).is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b'"' {
            continue;
        }
        i += 1;
        let value_start = i;
        while i < bytes.len() && bytes[i] != b'"' {
            i += 1;
        }
        if i >= value_start && i <= bytes.len() {
            attrs.push((name.to_string(), open_tag[value_start..i].to_string()));
        }
        if i < bytes.len() {
            i += 1;
        }
    }
    attrs
}

fn is_attr_name_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch == ':'
}

fn is_attr_name_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | ':' | '-' | '.')
}

fn first_attr_map(attrs: &[(String, String)]) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for (key, value) in attrs {
        out.entry(key.clone()).or_insert_with(|| value.clone());
    }
    out
}

fn strip_tags_and_collapse_ws(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    let mut last_was_ws = true;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            ch if in_tag => {}
            ch if ch.is_whitespace() => {
                if !last_was_ws {
                    out.push(' ');
                    last_was_ws = true;
                }
            }
            ch => {
                out.push(ch);
                last_was_ws = false;
            }
        }
    }
    out.trim().to_string()
}

fn class_number(classes: &[String], prefix: &str) -> Option<i64> {
    classes.iter().find_map(|token| {
        token
            .strip_prefix(prefix)
            .and_then(|rest| rest.parse::<i64>().ok())
    })
}

fn extract_header_cells(html: &str) -> Result<Vec<HeaderCellMeta>, String> {
    let row_html = find_first_row(html)?;
    let mut cells = Vec::new();
    let mut search_from = 0usize;

    while let Some(relative_td) = row_html[search_from..].find("<td") {
        let td_start = search_from + relative_td;
        let open_end = row_html[td_start..]
            .find('>')
            .map(|offset| td_start + offset + 1)
            .ok_or_else(|| "Unvollständiges <td>-Tag.".to_string())?;
        let close_start = row_html[open_end..]
            .find("</td>")
            .map(|offset| open_end + offset)
            .ok_or_else(|| "Unvollständiges </td>-Tag.".to_string())?;
        let close_end = close_start + "</td>".len();

        let open_tag = row_html[td_start..open_end].to_string();
        let inner_html = &row_html[open_end..close_start];
        let raw_html = row_html[td_start..close_end].to_string();
        let attrs = parse_attrs(&open_tag);
        let first_attrs = first_attr_map(&attrs);
        let class_attrs = attrs
            .iter()
            .filter_map(|(key, value)| (key == "class").then(|| value.clone()))
            .collect::<Vec<_>>();
        let primary_class_string = class_attrs.first().cloned().unwrap_or_default();
        let classes = primary_class_string
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let extra_class_strings = class_attrs.iter().skip(1).cloned().collect::<Vec<_>>();
        let mut all_classes = classes.clone();
        for extra in &extra_class_strings {
            for class_name in extra.split_whitespace() {
                if !all_classes.iter().any(|existing| existing == class_name) {
                    all_classes.push(class_name.to_string());
                }
            }
        }

        let row_number = class_number(&classes, "z_");
        let column_number = class_number(&classes, "r_")
            .or_else(|| class_number(&classes, "p1_col_"))
            .unwrap_or(cells.len() as i64);
        let data_attributes = first_attrs
            .iter()
            .filter(|(key, _)| key.starts_with("data-"))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<BTreeMap<_, _>>();

        let mut element_attr_map = first_attrs.clone();
        if !primary_class_string.is_empty() {
            element_attr_map.insert("class".to_string(), primary_class_string.clone());
        }

        cells.push(HeaderCellMeta {
            column_number,
            row_number,
            tag: "td".to_string(),
            classes: classes.clone(),
            class_string: primary_class_string.clone(),
            class_attributes: class_attrs,
            extra_class_strings,
            all_classes,
            data_attributes,
            attributes: attrs,
            attributes_first: first_attrs,
            text: strip_tags_and_collapse_ws(inner_html),
            raw_open_tag: open_tag,
            raw_html: raw_html.clone(),
            html_elements: vec![HtmlElementMeta {
                tag: "td".to_string(),
                classes,
                class_string: primary_class_string,
                attributes: element_attr_map,
                html: raw_html,
            }],
        });

        search_from = close_end;
    }

    Ok(cells)
}

fn write_jsonl(path: &Path, cells: &[HeaderCellMeta]) -> Result<(), String> {
    let mut out = String::new();
    for cell in cells {
        let line = serde_json::to_string(cell)
            .map_err(|error| format!("JSON konnte nicht erzeugt werden: {error}"))?;
        out.push_str(&line);
        out.push('\n');
    }
    fs::write(path, out).map_err(|error| {
        format!(
            "{} konnte nicht geschrieben werden: {error}",
            path.display()
        )
    })
}

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    let config = parse_args(&argv);
    let repo_root = find_repo_root(config.repo_root.as_deref());
    let html = if config.stdin_html {
        read_stdin_html()
    } else {
        run_reta_html(&repo_root, config.reta_bin.as_deref())
    };

    let html = match html {
        Ok(html) => html,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(1);
        }
    };

    let cells = match extract_header_cells(&html) {
        Ok(cells) => cells,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(1);
        }
    };

    if let Err(message) = write_jsonl(&config.out_path, &cells) {
        eprintln!("{message}");
        std::process::exit(1);
    }

    println!("geschrieben: {}", config.out_path.display());
    println!("spalten: {}", cells.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_attrs_keeps_duplicate_class_attributes() {
        let attrs = parse_attrs(r#"<td class="a b" data-x="1" class="c">"#);
        assert_eq!(
            attrs,
            vec![
                ("class".to_string(), "a b".to_string()),
                ("data-x".to_string(), "1".to_string()),
                ("class".to_string(), "c".to_string()),
            ]
        );
    }

    #[test]
    fn extract_header_cells_matches_python_shape() {
        let html = r#"<table><tr><td class="z_1 r_2 a" data-k="v">Titel</td><td class="p1_col_3">X</td></tr></table>"#;
        let cells = extract_header_cells(html).expect("cells");
        assert_eq!(cells.len(), 2);
        assert_eq!(cells[0].column_number, 2);
        assert_eq!(cells[0].row_number, Some(1));
        assert_eq!(cells[0].text, "Titel");
        assert_eq!(
            cells[0].data_attributes.get("data-k"),
            Some(&"v".to_string())
        );
        assert_eq!(cells[1].column_number, 3);
    }
}
