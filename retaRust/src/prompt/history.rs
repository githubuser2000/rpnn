use std::path::PathBuf;

use super::python_like::libreta_prompt_custom_split;

/// Python `retaPrompt.newSession(history=True)` uses one shared file:
/// `~/.ReTaPromptHistory`.  Keep that path for reedline history as well so
/// `loggen`/`nichtloggen` behaves like Python's ToggleHistory wrapper instead
/// of creating one unrelated file per frontend binary.
pub fn default_history_path(_program_name: &str) -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let mut path = home;
    path.push(".ReTaPromptHistory");
    path
}

pub fn default_log_path(program_name: &str) -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let mut dir = home;
    dir.push(".local");
    dir.push("share");
    dir.push("reta");

    let _ = std::fs::create_dir_all(&dir);
    dir.push(format!("{}_session.log", program_name));
    dir
}

/// Python `ToggleHistory.append_string`: append only while logging/history is
/// enabled, and never append the logging toggle commands themselves.  The split
/// must use the prompt tokenizer because `loggen`/`nichtloggen` can appear next
/// to quoted or bracketed prompt syntax.
pub fn contains_history_toggle_token_like_python(input: &str) -> bool {
    libreta_prompt_custom_split(input.trim())
        .iter()
        .any(|token| token == "loggen" || token == "nichtloggen")
}

pub fn should_append_history_string_like_python(logging_enabled: bool, input: &str) -> bool {
    logging_enabled && !input.trim().is_empty() && !contains_history_toggle_token_like_python(input)
}

/// Reedline appends successful input to its file-backed history by itself.
/// Python `ToggleHistory.append_string` would have refused the same line when
/// logging is disabled or when the line contains `loggen`/`nichtloggen`.
/// The interactive frontend uses this predicate to remove exactly that just
/// appended line again while keeping the readable history file active.
pub fn should_scrub_history_string_after_reedline_append_like_python(
    logging_enabled: bool,
    input: &str,
) -> bool {
    !input.trim().is_empty() && !should_append_history_string_like_python(logging_enabled, input)
}

#[cfg(test)]
mod tests {
    use super::{
        contains_history_toggle_token_like_python, default_history_path,
        should_append_history_string_like_python,
        should_scrub_history_string_after_reedline_append_like_python,
    };

    #[test]
    fn default_history_path_matches_python_reta_prompt_history_file() {
        let path = default_history_path("rpb");
        assert_eq!(path.file_name().and_then(|name| name.to_str()), Some(".ReTaPromptHistory"));
    }

    #[test]
    fn toggle_history_append_rules_match_python_togglehistory() {
        assert!(should_append_history_string_like_python(true, "12 emotion"));
        assert!(!should_append_history_string_like_python(false, "12 emotion"));
        assert!(!should_append_history_string_like_python(true, ""));
        assert!(!should_append_history_string_like_python(true, "loggen"));
        assert!(!should_append_history_string_like_python(true, "12 nichtloggen"));
        assert!(contains_history_toggle_token_like_python("12 loggen"));
    }

    #[test]
    fn reedline_scrub_predicate_matches_python_togglehistory_append_rules() {
        assert!(!should_scrub_history_string_after_reedline_append_like_python(
            true,
            "12 emotion"
        ));
        assert!(should_scrub_history_string_after_reedline_append_like_python(
            false,
            "12 emotion"
        ));
        assert!(should_scrub_history_string_after_reedline_append_like_python(
            true,
            "nichtloggen"
        ));
        assert!(!should_scrub_history_string_after_reedline_append_like_python(
            false,
            ""
        ));
    }
}
