use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn candidate_doc_paths(file_name: &str) -> [PathBuf; 4] {
    [
        Path::new("doc").join(file_name),
        Path::new(file_name).to_path_buf(),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("doc").join(file_name),
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
    Ok(strip_front_matter_and_anchor_tags(&markdown_text))
}

pub fn strip_front_matter_and_anchor_tags(markdown_text: &str) -> String {
    let after_front_matter = if let Some(stripped) = markdown_text.strip_prefix("+++") {
        if let Some(end) = stripped.find("+++") {
            stripped[end + 3..].to_string()
        } else {
            markdown_text.to_string()
        }
    } else {
        markdown_text.to_string()
    };

    strip_pandoc_anchor_tags(&after_front_matter)
}

fn strip_pandoc_anchor_tags(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' && i + 1 < bytes.len() && bytes[i + 1] == b'#' {
            i += 2;
            while i < bytes.len() && bytes[i] != b'}' {
                i += 1;
            }
            if i < bytes.len() && bytes[i] == b'}' {
                i += 1;
            }
        } else {
            output.push(bytes[i] as char);
            i += 1;
        }
    }
    output
}
