use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn candidate_doc_paths(file_name: &str) -> Vec<PathBuf> {
    vec![
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("doc")
            .join(Path::new(file_name).file_name().unwrap_or_default()),
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("doc")
            .join(Path::new(file_name).file_name().unwrap_or_default()),
        Path::new(file_name).to_path_buf(),
        Path::new(env!("CARGO_MANIFEST_DIR")).join(file_name),
    ]
}

pub fn read_doc_file(file_name: &str) -> io::Result<String> {
    for path in candidate_doc_paths(file_name) {
        if path.exists() {
            return fs::read_to_string(path);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Dokument nicht gefunden: {file_name}"),
    ))
}

pub fn reta_hilfe_text() -> io::Result<String> {
    read_doc_file("readme-reta.md")
}

pub fn retaprompt_hilfe_text() -> io::Result<String> {
    let markdown_text = read_doc_file("readme-retaPrompt.md")?;
    Ok(strip_retaprompt_like_center_py(&markdown_text))
}

pub fn retaprompt_hilfe_rendered_like_python() -> io::Result<String> {
    let markdown = retaprompt_hilfe_text()?;
    Ok(render_markdown_for_terminal(&markdown))
}

pub fn strip_retaprompt_like_center_py(markdown_text: &str) -> String {
    let without_anchors = strip_pandoc_anchor_tags(markdown_text);
    let start = without_anchors
        .get(2..)
        .and_then(|tail| tail.find("+++"))
        .map(|idx| idx + 2 + 3)
        .unwrap_or(0);
    without_anchors[start.min(without_anchors.len())..].to_string()
}

pub fn render_markdown_for_terminal(markdown_text: &str) -> String {
    format!("{}", termimad::term_text(markdown_text))
}

fn strip_pandoc_anchor_tags(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '{' && i + 1 < chars.len() && chars[i + 1] == '#' {
            i += 2;
            while i < chars.len() && chars[i] != '}' {
                i += 1;
            }
            if i < chars.len() {
                i += 1;
            }
        } else {
            output.push(chars[i]);
            i += 1;
        }
    }
    output
}
