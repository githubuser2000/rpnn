use std::io::Write;
use std::collections::VecDeque;
use std::path::PathBuf;

use reedline::{
    FileBackedHistory, History, HistoryItem, HistoryItemId, HistorySessionId,
    Result as ReedlineResult, SearchQuery,
};

use super::python_like::libreta_prompt_custom_split;

/// Python `retaPrompt.newSession(history=True)` uses one shared prompt-toolkit
/// history file: `~/.ReTaPromptHistory`.
///
/// Important: prompt_toolkit's `FileHistory` does not store plain command
/// lines.  It writes metadata lines like `# 2026-...` and command lines with a
/// leading `+`.  Rust must parse that format, otherwise arrow-up browsing shows
/// the date metadata and a spurious `+` before every command.
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

/// Reedline appends successful input to its active history by itself.  The
/// custom history below refuses the same lines Python would refuse, so this is
/// only needed for compatibility with older plain-file history users/tests.
pub fn should_scrub_history_string_after_reedline_append_like_python(
    logging_enabled: bool,
    input: &str,
) -> bool {
    !input.trim().is_empty() && !should_append_history_string_like_python(logging_enabled, input)
}

pub fn persistent_prompt_history_enabled(program_name: &str, logging_enabled: bool) -> bool {
    program_name == "rpl" && logging_enabled
}

/// Parse prompt_toolkit FileHistory text.
///
/// Format written by prompt_toolkit:
///
/// ```text
/// # timestamp
/// +command line 1
/// +command line 2
/// ```
///
/// Non-`+` lines terminate the current command and are metadata.  For migration
/// from older Rust builds we also accept clean plain command lines, and we turn
/// old `[timestamp] input: ...` diagnostic-log lines back into commands.
pub fn parse_prompt_toolkit_history_text(raw: &str) -> Vec<String> {
    let mut entries = Vec::new();
    let mut current = String::new();
    let mut in_prompt_toolkit_entry = false;

    let flush_current = |entries: &mut Vec<String>, current: &mut String, active: &mut bool| {
        if *active {
            if current.ends_with('\n') {
                current.pop();
            }
            if !current.trim().is_empty() {
                entries.push(current.clone());
            }
            current.clear();
            *active = false;
        }
    };

    for raw_line in raw.split_inclusive('\n') {
        let line = raw_line.strip_suffix('\n').unwrap_or(raw_line);
        let line = line.strip_suffix('\r').unwrap_or(line);

        if let Some(command_part) = line.strip_prefix('+') {
            current.push_str(command_part);
            current.push('\n');
            in_prompt_toolkit_entry = true;
            continue;
        }

        flush_current(&mut entries, &mut current, &mut in_prompt_toolkit_entry);

        if let Some(command) = command_from_reta_session_log_line(line) {
            if !command.trim().is_empty() {
                entries.push(command);
            }
            continue;
        }

        if should_keep_legacy_plain_history_line(line) {
            entries.push(line.to_string());
        }
    }

    if !raw.ends_with('\n') {
        // split_inclusive already yielded the unterminated last line.  Nothing
        // special is needed here; this branch only documents the case.
    }
    flush_current(&mut entries, &mut current, &mut in_prompt_toolkit_entry);
    entries
}

fn should_keep_legacy_plain_history_line(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return false;
    }
    if looks_like_iso_datetime_metadata(trimmed) || looks_like_numeric_timestamp_metadata(trimmed) {
        return false;
    }
    if looks_like_reta_session_log_line(trimmed) {
        return false;
    }
    true
}

fn looks_like_iso_datetime_metadata(trimmed: &str) -> bool {
    let bytes = trimmed.as_bytes();
    bytes.len() >= 10
        && bytes[0..4].iter().all(|byte| byte.is_ascii_digit())
        && bytes[4] == b'-'
        && bytes[5..7].iter().all(|byte| byte.is_ascii_digit())
        && bytes[7] == b'-'
        && bytes[8..10].iter().all(|byte| byte.is_ascii_digit())
}

fn looks_like_numeric_timestamp_metadata(trimmed: &str) -> bool {
    trimmed.len() >= 9 && trimmed.chars().all(|ch| ch.is_ascii_digit())
}

fn looks_like_reta_session_log_line(trimmed: &str) -> bool {
    command_from_reta_session_log_line(trimmed).is_some()
        || parse_reta_session_log_prefix(trimmed).is_some()
}

fn command_from_reta_session_log_line(line: &str) -> Option<String> {
    let rest = parse_reta_session_log_prefix(line)?;
    rest.strip_prefix("input:")
        .map(str::trim_start)
        .map(str::to_string)
}

fn parse_reta_session_log_prefix(line: &str) -> Option<&str> {
    let rest = line.strip_prefix('[')?;
    let close = rest.find(']')?;
    let timestamp = &rest[..close];
    if timestamp.is_empty() || !timestamp.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    Some(rest[close + 1..].trim_start())
}

pub fn format_prompt_toolkit_history_entry(command: &str) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string());

    let mut out = String::new();
    out.push('\n');
    out.push_str("# ");
    out.push_str(&timestamp);
    out.push('\n');
    for line in command.split('\n') {
        out.push('+');
        out.push_str(line);
        out.push('\n');
    }
    out
}

#[derive(Debug)]
pub struct PromptToolkitFileHistory {
    capacity: usize,
    // Kept as a small mirror for tests and diagnostics.  Navigation/search is
    // delegated to reedline::FileBackedHistory because Reedline's arrow-key
    // browsing uses an internal `not_command_line` filter that external custom
    // History implementations cannot inspect.
    entries: VecDeque<String>,
    inner: FileBackedHistory,
    file: Option<PathBuf>,
    append_enabled: bool,
    memory_enabled: bool,
}

impl PromptToolkitFileHistory {
    pub fn in_memory(capacity: usize) -> ReedlineResult<Self> {
        // rp/rrp should not touch ~/.ReTaPromptHistory, but it may still keep a
        // per-session in-memory history.
        Self::from_entries_with_policy(capacity, Vec::new(), None, false, true)
    }

    pub fn with_file(capacity: usize, file: PathBuf) -> ReedlineResult<Self> {
        Self::with_file_and_append_policy(capacity, file, true)
    }

    pub fn with_file_and_append_policy(
        capacity: usize,
        file: PathBuf,
        append_enabled: bool,
    ) -> ReedlineResult<Self> {
        let entries = std::fs::read_to_string(&file)
            .map(|raw| parse_prompt_toolkit_history_text(&raw))
            .unwrap_or_default();
        // rpl/rrpl always reads the file.  If `nichtloggen` disabled logging,
        // newly typed commands are not added while disabled.
        Self::from_entries_with_policy(capacity, entries, Some(file), append_enabled, append_enabled)
    }

    fn from_entries_with_policy(
        capacity: usize,
        entries: Vec<String>,
        file: Option<PathBuf>,
        append_enabled: bool,
        memory_enabled: bool,
    ) -> ReedlineResult<Self> {
        let mut inner = FileBackedHistory::new(capacity)?;
        let mut retained_entries = VecDeque::new();

        for entry in entries {
            let saved = inner.save(HistoryItem::from_command_line(entry.clone()))?;
            if saved.id.is_some() {
                Self::push_mirror_entry(capacity, &mut retained_entries, saved.command_line);
            }
        }

        Ok(Self {
            capacity,
            entries: retained_entries,
            inner,
            file,
            append_enabled,
            memory_enabled,
        })
    }

    fn push_mirror_entry(capacity: usize, entries: &mut VecDeque<String>, entry: String) {
        if capacity == 0 || entries.back() == Some(&entry) {
            return;
        }
        if entries.len() == capacity {
            entries.pop_front();
        }
        entries.push_back(entry);
    }

    fn remember_saved_entry(&mut self, entry: String) {
        Self::push_mirror_entry(self.capacity, &mut self.entries, entry);
    }

    fn append_entry_to_file(file_path: &PathBuf, entry: &str) -> std::io::Result<()> {
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;
        file.write_all(format_prompt_toolkit_history_entry(entry).as_bytes())?;
        file.flush()
    }
}

impl History for PromptToolkitFileHistory {
    fn save(&mut self, h: HistoryItem) -> ReedlineResult<HistoryItem> {
        let entry = h.command_line.clone();
        if !self.memory_enabled || !should_append_history_string_like_python(true, &entry) {
            return Ok(h);
        }

        let saved = self.inner.save(h)?;
        if saved.id.is_some() {
            self.remember_saved_entry(saved.command_line.clone());
            if self.append_enabled {
                if let Some(file_path) = &self.file {
                    Self::append_entry_to_file(file_path, &saved.command_line)
                        .map_err(reedline::ReedlineError::from)?;
                }
            }
        }
        Ok(saved)
    }

    fn load(&self, id: HistoryItemId) -> ReedlineResult<HistoryItem> {
        self.inner.load(id)
    }

    fn count(&self, query: SearchQuery) -> ReedlineResult<i64> {
        self.inner.count(query)
    }

    fn search(&self, query: SearchQuery) -> ReedlineResult<Vec<HistoryItem>> {
        self.inner.search(query)
    }

    fn update(
        &mut self,
        id: HistoryItemId,
        updater: &dyn Fn(HistoryItem) -> HistoryItem,
    ) -> ReedlineResult<()> {
        self.inner.update(id, updater)
    }

    fn clear(&mut self) -> ReedlineResult<()> {
        self.inner.clear()?;
        self.entries.clear();
        if let Some(file) = &self.file {
            if file.exists() {
                std::fs::remove_file(file).map_err(reedline::ReedlineError::from)?;
            }
        }
        Ok(())
    }

    fn delete(&mut self, h: HistoryItemId) -> ReedlineResult<()> {
        self.inner.delete(h)
    }

    fn sync(&mut self) -> std::io::Result<()> {
        // File writes happen immediately in save().  Keeping sync() a no-op
        // prevents the old drop-time rewrite/truncate behavior and avoids
        // losing commands when the editor is rebuilt repeatedly.
        Ok(())
    }

    fn session(&self) -> Option<HistorySessionId> {
        self.inner.session()
    }
}

impl Drop for PromptToolkitFileHistory {
    fn drop(&mut self) {
        let _ = self.sync();
    }
}

#[cfg(test)]
mod tests {
    use super::{
        contains_history_toggle_token_like_python, default_history_path,
        format_prompt_toolkit_history_entry, parse_prompt_toolkit_history_text,
        persistent_prompt_history_enabled, should_append_history_string_like_python,
        should_scrub_history_string_after_reedline_append_like_python,
    };

    #[test]
    fn default_history_path_matches_python_reta_prompt_history_file() {
        let path = default_history_path("rpb");
        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some(".ReTaPromptHistory")
        );
    }

    #[test]
    fn prompt_toolkit_history_parser_hides_dates_and_plus_prefixes() {
        let raw = "\n# 2026-05-10 08:28:15.000000\n+r6\n\n# 2026-05-10 08:29:00.000000\n+reta -zeilen --alles\n";
        assert_eq!(
            parse_prompt_toolkit_history_text(raw),
            vec!["r6".to_string(), "reta -zeilen --alles".to_string()]
        );
    }

    #[test]
    fn prompt_toolkit_history_parser_supports_multiline_entries() {
        let raw = "# ignored\n+python <<EOF\n+print(1)\n+EOF\n";
        assert_eq!(
            parse_prompt_toolkit_history_text(raw),
            vec!["python <<EOF\nprint(1)\nEOF".to_string()]
        );
    }

    #[test]
    fn prompt_toolkit_history_parser_can_salvage_old_session_input_logs() {
        let raw = "[1710000000] session: start\n[1710000001] input: 12 emotion\n[1710000002] output-meta: ignored\n";
        assert_eq!(
            parse_prompt_toolkit_history_text(raw),
            vec!["12 emotion".to_string()]
        );
    }

    #[test]
    fn prompt_toolkit_history_writer_uses_python_compatible_plus_lines() {
        let formatted = format_prompt_toolkit_history_entry("12\n13");
        assert!(formatted.contains("\n# "));
        assert!(formatted.contains("\n+12\n+13\n"));
        assert_eq!(
            parse_prompt_toolkit_history_text(&formatted),
            vec!["12\n13".to_string()]
        );
    }

    #[test]
    fn rpl_is_the_only_frontend_with_persistent_prompt_history() {
        assert!(persistent_prompt_history_enabled("rpl", true));
        assert!(!persistent_prompt_history_enabled("rpl", false));
        assert!(!persistent_prompt_history_enabled("rp", true));
        assert!(!persistent_prompt_history_enabled("rpb", true));
    }

    #[test]
    fn prompt_toolkit_in_memory_history_does_not_persist() {
        let history = super::PromptToolkitFileHistory::in_memory(32).unwrap();
        assert!(history.file.is_none());
    }

    #[test]
    fn file_history_appends_immediately_when_enabled() {
        use reedline::{History, HistoryItem};

        let path = std::env::temp_dir().join(format!(
            "reta_prompt_history_enabled_{}_{}.txt",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = std::fs::remove_file(&path);
        let mut history =
            super::PromptToolkitFileHistory::with_file_and_append_policy(32, path.clone(), true)
                .unwrap();
        let _ = history
            .save(HistoryItem::from_command_line("r6".to_string()))
            .unwrap();
        let raw = std::fs::read_to_string(&path).unwrap();
        assert!(raw.contains("\n+r6\n"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn file_history_reads_but_does_not_append_when_disabled() {
        use reedline::{History, HistoryItem};

        let path = std::env::temp_dir().join(format!(
            "reta_prompt_history_disabled_{}_{}.txt",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::write(&path, "\n# old\n+r6\n").unwrap();
        let mut history =
            super::PromptToolkitFileHistory::with_file_and_append_policy(32, path.clone(), false)
                .unwrap();
        assert_eq!(history.entries.back().map(String::as_str), Some("r6"));
        let _ = history
            .save(HistoryItem::from_command_line("12".to_string()))
            .unwrap();
        let raw = std::fs::read_to_string(&path).unwrap();
        assert!(raw.contains("\n+r6\n"));
        assert!(!raw.contains("\n+12\n"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn toggle_history_append_rules_match_python_togglehistory() {
        assert!(should_append_history_string_like_python(true, "12 emotion"));
        assert!(!should_append_history_string_like_python(
            false,
            "12 emotion"
        ));
        assert!(!should_append_history_string_like_python(true, ""));
        assert!(!should_append_history_string_like_python(true, "loggen"));
        assert!(!should_append_history_string_like_python(
            true,
            "12 nichtloggen"
        ));
        assert!(contains_history_toggle_token_like_python("12 loggen"));
    }

    #[test]
    fn reedline_scrub_predicate_matches_python_togglehistory_append_rules() {
        assert!(!should_scrub_history_string_after_reedline_append_like_python(true, "12 emotion"));
        assert!(should_scrub_history_string_after_reedline_append_like_python(false, "12 emotion"));
        assert!(should_scrub_history_string_after_reedline_append_like_python(true, "nichtloggen"));
        assert!(!should_scrub_history_string_after_reedline_append_like_python(false, ""));
    }
    #[test]
    fn prompt_toolkit_file_history_browses_the_full_list_in_reverse_order() {
        use reedline::{History, HistoryItem};

        let path = std::env::temp_dir().join(format!(
            "reta_prompt_history_full_list_{}_{}.txt",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = std::fs::remove_file(&path);

        let mut history =
            super::PromptToolkitFileHistory::with_file_and_append_policy(32, path.clone(), true)
                .unwrap();
        history
            .save(HistoryItem::from_command_line("r6".to_string()))
            .unwrap();
        history
            .save(HistoryItem::from_command_line("reta -zeilen --alles".to_string()))
            .unwrap();
        history
            .save(HistoryItem::from_command_line("12".to_string()))
            .unwrap();

        let raw = std::fs::read_to_string(&path).unwrap();
        assert_eq!(
            super::parse_prompt_toolkit_history_text(&raw),
            vec![
                "r6".to_string(),
                "reta -zeilen --alles".to_string(),
                "12".to_string(),
            ]
        );

        let reopened =
            super::PromptToolkitFileHistory::with_file_and_append_policy(32, path.clone(), true)
                .unwrap();
        let browsed = reopened
            .search(reedline::SearchQuery::everything(
                reedline::SearchDirection::Backward,
                None,
            ))
            .unwrap()
            .into_iter()
            .map(|item| item.command_line)
            .collect::<Vec<_>>();
        assert_eq!(
            browsed,
            vec![
                "12".to_string(),
                "reta -zeilen --alles".to_string(),
                "r6".to_string(),
            ]
        );

        let _ = std::fs::remove_file(&path);
    }

}
