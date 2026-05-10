#![allow(non_snake_case)]

use retaprompt_input::completion::candidates_for_input_in_mode_with_context;
use retaprompt_input::python_like::{libreta_prompt_custom_split, PromptModus};

fn json_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    for ch in text.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch.is_control() => out.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => out.push(ch),
        }
    }
    out
}

fn json_string_array(items: &[String]) -> String {
    let body = items
        .iter()
        .map(|item| format!("\"{}\"", json_escape(item)))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn prompt_mode_from_name(name: &str) -> Result<PromptModus, String> {
    match name {
        "normal" | "Normal" => Ok(PromptModus::Normal),
        "speichern" | "Speichern" => Ok(PromptModus::Speichern),
        "loeschen-start" | "löschen-start" | "LoeschenStart" => Ok(PromptModus::LoeschenStart),
        "loeschen-select" | "löschen-select" | "LoeschenSelect" => Ok(PromptModus::LoeschenSelect),
        "speicherung-ausgaben" | "SpeicherungAusgaben" => Ok(PromptModus::SpeicherungAusgaben),
        "speicherung-ausgaben-mit-zusatz" | "SpeicherungAusgabenMitZusatz" => {
            Ok(PromptModus::SpeicherungAusgabenMitZusatz)
        }
        "ausgabe-selektiv" | "AusgabeSelektiv" => Ok(PromptModus::AusgabeSelektiv),
        other => Err(format!("unbekannter PromptModus: {other}")),
    }
}

fn help_text(program_name: &str) -> String {
    format!(
        r#"{program_name} - Completion-Probe für retaPrompt/Rust

Aufruf:
  {program_name} [--line <text>] [--mode <modus>] [--context <tokens>]
  {program_name} <text>

Beispiele:
  {program_name} --line "reta -zeilen --zeit=h"
  {program_name} --context "reta -zeilen" --line "--ze"
  {program_name} --mode loeschen-select --line "1-"

Ausgabe:
  JSON-Array der Rust-Completion-Kandidaten. Dieses Werkzeug ändert weder
  Autocomplete noch Autosuggest; es macht nur die vorhandene Completion-Logik
  für Python-vs-Rust-Vergleiche abfragbar.
"#
    )
}

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    let program_name = argv
        .first()
        .cloned()
        .unwrap_or_else(|| "retaprompt_completion_probe".to_string());

    let mut mode = PromptModus::Normal;
    let mut line: Option<String> = None;
    let mut context_tokens: Vec<String> = Vec::new();
    let mut passthrough: Vec<String> = Vec::new();

    let mut index = 1usize;
    while index < argv.len() {
        match argv[index].as_str() {
            "-h" | "--help" | "help" => {
                print!("{}", help_text(&program_name));
                return;
            }
            "--line" => {
                index += 1;
                if index >= argv.len() {
                    eprintln!("--line erwartet einen Text");
                    std::process::exit(2);
                }
                line = Some(argv[index].clone());
            }
            "--mode" => {
                index += 1;
                if index >= argv.len() {
                    eprintln!("--mode erwartet einen Modus");
                    std::process::exit(2);
                }
                mode = match prompt_mode_from_name(&argv[index]) {
                    Ok(mode) => mode,
                    Err(message) => {
                        eprintln!("{message}");
                        std::process::exit(2);
                    }
                };
            }
            "--context" => {
                index += 1;
                if index >= argv.len() {
                    eprintln!("--context erwartet eine Token-Zeile");
                    std::process::exit(2);
                }
                context_tokens = libreta_prompt_custom_split(&argv[index]);
            }
            other => passthrough.push(other.to_string()),
        }
        index += 1;
    }

    let line = line.unwrap_or_else(|| passthrough.join(" "));
    let candidates = candidates_for_input_in_mode_with_context(&line, mode, &context_tokens, &[]);
    println!("{}", json_string_array(&candidates));
}
