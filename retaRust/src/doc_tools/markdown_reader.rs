#![allow(non_snake_case)]
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
    let markdown_text = read_doc_file("readme-reta.md")?;
    Ok(preprocess_reta_markdown_for_terminal(&markdown_text))
}

pub fn reta_hilfe_rendered_like_python() -> io::Result<String> {
    let markdown = reta_hilfe_text()?;
    Ok(render_markdown_for_terminal(&markdown))
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

fn preprocess_reta_markdown_for_terminal(markdown_text: &str) -> String {
    let without_anchors = strip_pandoc_anchor_tags(markdown_text);
    let mut out: Vec<String> = Vec::new();
    let mut in_code_fence = false;

    for raw_line in without_anchors.lines() {
        let line = raw_line.replace('\t', "    ");
        let trimmed = line.trim_end();
        let core = trimmed.trim();

        if core.starts_with("```") {
            in_code_fence = !in_code_fence;
            continue;
        }

        if in_code_fence {
            for code_line in wrap_preserving_words(core, 92, 4) {
                out.push(code_line);
            }
            continue;
        }

        if core.is_empty() {
            if out.last().map(|s| !s.is_empty()).unwrap_or(false) {
                out.push(String::new());
            }
            continue;
        }

        if let Some(heading) = core.strip_prefix("### ") {
            if out.last().map(|s| !s.is_empty()).unwrap_or(false) {
                out.push(String::new());
            }
            out.push(heading.to_string());
            continue;
        }
        if let Some(heading) = core.strip_prefix("## ") {
            if out.last().map(|s| !s.is_empty()).unwrap_or(false) {
                out.push(String::new());
            }
            out.push(heading.to_string());
            continue;
        }
        if let Some(heading) = core.strip_prefix("# ") {
            if out.last().map(|s| !s.is_empty()).unwrap_or(false) {
                out.push(String::new());
            }
            out.push(heading.to_string());
            continue;
        }

        let indent_spaces = raw_line.chars().take_while(|c| *c == ' ').count();
        let level = (indent_spaces / 4).min(4);
        let bullet_indent = "  ".repeat(level);

        if let Some(item) = core.strip_prefix("* ") {
            let normalized = normalize_inline_markdown(item);
            let width = if level == 0 { 92 } else { 88 };
            for (idx, part) in wrap_csv_or_text(&normalized, width, bullet_indent.len() + 2).into_iter().enumerate() {
                if idx == 0 {
                    out.push(format!("{}* {}", bullet_indent, part));
                } else {
                    out.push(format!("{}  {}", bullet_indent, part));
                }
            }
            continue;
        }

        let normalized = normalize_inline_markdown(core);
        let width = if normalized.contains(',') && normalized.len() > 70 { 92 } else { 96 };
        for part in wrap_csv_or_text(&normalized, width, bullet_indent.len()) {
            out.push(format!("{}{}", bullet_indent, part));
        }
    }

    while out.last().map(|s| s.is_empty()).unwrap_or(false) {
        out.pop();
    }

    out.join("\n")
}

fn normalize_inline_markdown(input: &str) -> String {
    input
        .replace("**", "")
        .replace("__", "")
        .replace('`', "")
}

fn wrap_csv_or_text(input: &str, width: usize, hanging_indent: usize) -> Vec<String> {
    if input.contains(',') && input.len() > width {
        let csv_wrapped = wrap_csv_like(input, width, hanging_indent);
        if csv_wrapped.len() > 1 {
            return csv_wrapped;
        }
    }
    wrap_preserving_words(input, width, hanging_indent)
}

fn wrap_csv_like(input: &str, width: usize, hanging_indent: usize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut current = String::new();
    let indent = " ".repeat(hanging_indent);

    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let candidate = if current.is_empty() {
            part.to_string()
        } else {
            format!("{}, {}", current, part)
        };

        if !current.is_empty() && candidate.chars().count() > width {
            out.push(current);
            current = format!("{}{}", indent, part);
        } else {
            current = candidate;
        }
    }

    if !current.is_empty() {
        out.push(current);
    }

    if out.is_empty() {
        vec![input.to_string()]
    } else {
        out
    }
}

fn wrap_preserving_words(input: &str, width: usize, hanging_indent: usize) -> Vec<String> {
    let indent = " ".repeat(hanging_indent);
    let words: Vec<&str> = input.split_whitespace().collect();
    if words.is_empty() {
        return vec![String::new()];
    }
    let mut out: Vec<String> = Vec::new();
    let mut current = String::new();

    for word in words {
        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current, word)
        };
        if !current.is_empty() && candidate.chars().count() > width {
            out.push(current);
            current = format!("{}{}", indent, word);
        } else {
            current = candidate;
        }
    }

    if !current.is_empty() {
        out.push(current);
    }
    out
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
