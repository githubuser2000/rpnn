use std::path::PathBuf;

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

#[cfg(test)]
mod tests {
    use super::default_history_path;

    #[test]
    fn default_history_path_matches_python_reta_prompt_history_file() {
        let path = default_history_path("rpb");
        assert_eq!(path.file_name().and_then(|name| name.to_str()), Some(".ReTaPromptHistory"));
    }
}
