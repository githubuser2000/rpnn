use serde::Serialize;

#[derive(Serialize)]
struct PromptLanguageCompletionInspect {
    input: String,
    policy: reta_architecture::PromptLanguageCompletionPolicy,
    report: reta_architecture::PromptLanguageCompletionReport,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input_parts = Vec::new();
    let mut iter = std::env::args().skip(1).peekable();
    while let Some(arg) = iter.next() {
        if arg == "--prompt-text" || arg == "--input" {
            if let Some(value) = iter.next() {
                input_parts.push(value);
            }
        } else if let Some(value) = arg.strip_prefix("--prompt-text=").or_else(|| arg.strip_prefix("--input=")) {
            input_parts.push(value.to_string());
        } else {
            input_parts.push(arg);
        }
    }
    let input = input_parts.join(" ");
    let input = if input.trim().is_empty() {
        "reta -language=english -spalten --kontinuum=m".to_string()
    } else {
        input
    };
    let policy = reta_architecture::PromptLanguageCompletionPolicy::default();
    let report = reta_architecture::prompt_language_completion_for_text(&input, &policy);
    println!(
        "{}",
        serde_json::to_string_pretty(&PromptLanguageCompletionInspect { input, policy, report })?
    );
    Ok(())
}
