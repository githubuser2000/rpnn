#![allow(non_snake_case)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub use reta_architecture;

pub mod shared {
    pub mod words_py {
        pub use retaprompt_commands::shared::words_py::*;
    }
}

pub mod domain {
    pub mod python_source_of_truth {
        pub use retaprompt_commands::domain::python_source_of_truth::*;
    }
}

pub use retaprompt_commands::{run_reta_from_args, shared_words, RetaRunResult};

pub mod semantic_choices {
    pub use retaprompt_commands::semantic_choices::*;
}

pub mod tokenize {
    pub use retaprompt_commands::tokenize::*;
}

pub mod python_like {
    pub use retaprompt_commands::python_like::*;
}

pub mod commands {
    pub use retaprompt_commands::commands::*;
}

#[path = "../../../src/prompt/app.rs"]
pub mod app;
#[path = "../../../src/prompt/completion.rs"]
pub mod completion;
#[path = "../../../src/prompt/frontend_profile.rs"]
pub mod frontend_profile;
#[path = "../../../src/prompt/frontends.rs"]
pub mod frontends;
#[path = "../../../src/prompt/history.rs"]
pub mod history;
#[path = "../../../src/prompt/preset.rs"]
pub mod preset;
#[path = "../../../src/prompt/tui.rs"]
pub mod tui;

#[cfg(test)]
mod completion_python_parity_tests;

pub use app::{
    run_prompt_frontend, run_prompt_frontend_from_env, run_prompt_frontend_with_profile,
    run_prompt_frontend_with_profile_from_env, run_prompt_input_frontend_with_profile,
};
pub use frontend_profile::{PromptFrontendKind, PromptFrontendProfile};
pub use frontends::{
    run_rp_frontend_from_env, run_rpb_frontend_from_env, run_rpe_frontend_from_env,
    run_rpl_frontend_from_env,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromptInputFrontendKind {
    Rp,
    Rpl,
    Rpe,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromptLauncherKind {
    Rp,
    Rpl,
    Rpb,
    Rpe,
}

impl PromptInputFrontendKind {
    pub fn from_abi_value(kind: i32) -> Option<Self> {
        match kind {
            1 => Some(Self::Rp),
            2 => Some(Self::Rpl),
            4 => Some(Self::Rpe),
            _ => None,
        }
    }

    pub fn from_program_name(program_name: &str) -> Option<Self> {
        let base = std::path::Path::new(program_name)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(program_name);
        match base {
            "rp" | "rrp" => Some(Self::Rp),
            "rpl" | "rrpl" => Some(Self::Rpl),
            "rpe" | "rrpe" => Some(Self::Rpe),
            _ => None,
        }
    }

    pub fn from_argv(argv: &[String]) -> Option<Self> {
        argv.first().and_then(|arg0| Self::from_program_name(arg0))
    }

    pub const fn abi_value(self) -> i32 {
        match self {
            Self::Rp => 1,
            Self::Rpl => 2,
            Self::Rpe => 4,
        }
    }

    pub const fn profile(self) -> PromptFrontendProfile {
        match self {
            Self::Rp => PromptFrontendProfile::rp(),
            Self::Rpl => PromptFrontendProfile::rpl(),
            Self::Rpe => PromptFrontendProfile::rpe(),
        }
    }
}

impl PromptLauncherKind {
    pub fn from_program_name(program_name: &str) -> Option<Self> {
        let base = std::path::Path::new(program_name)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(program_name);
        match base {
            "rp" | "rrp" => Some(Self::Rp),
            "rpl" | "rrpl" => Some(Self::Rpl),
            "rpb" | "rrpb" => Some(Self::Rpb),
            "rpe" | "rrpe" => Some(Self::Rpe),
            _ => None,
        }
    }

    pub fn from_argv(argv: &[String]) -> Option<Self> {
        argv.first().and_then(|arg0| Self::from_program_name(arg0))
    }

    pub fn from_abi_value(kind: i32) -> Option<Self> {
        match kind {
            1 => Some(Self::Rp),
            2 => Some(Self::Rpl),
            3 => Some(Self::Rpb),
            4 => Some(Self::Rpe),
            _ => None,
        }
    }
}

pub fn run_kind(argv: Vec<String>, kind: PromptInputFrontendKind) -> i32 {
    run_prompt_input_frontend_with_profile(argv, kind.profile())
}

pub fn run_kind_from_env(kind: PromptInputFrontendKind) -> i32 {
    run_kind(std::env::args().collect(), kind)
}

pub fn run_current_executable(argv: Vec<String>) -> i32 {
    match PromptInputFrontendKind::from_argv(&argv) {
        Some(kind) => run_kind(argv, kind),
        None => {
            let arg0 = argv
                .first()
                .cloned()
                .unwrap_or_else(|| "<unknown>".to_string());
            eprintln!(
                "retaprompt_input cannot infer input frontend kind from executable name: {arg0}"
            );
            eprintln!("expected one of: rp, rpl, rpe");
            1
        }
    }
}

pub fn run_current_executable_from_env() -> i32 {
    run_current_executable(std::env::args().collect())
}

pub fn run_launcher_kind(argv: Vec<String>, kind: PromptLauncherKind) -> i32 {
    match kind {
        PromptLauncherKind::Rp => run_kind(argv, PromptInputFrontendKind::Rp),
        PromptLauncherKind::Rpl => run_kind(argv, PromptInputFrontendKind::Rpl),
        PromptLauncherKind::Rpb => retaprompt_commands::run_rpb(argv),
        PromptLauncherKind::Rpe => run_kind(argv, PromptInputFrontendKind::Rpe),
    }
}

pub fn run_launcher_kind_from_env(kind: PromptLauncherKind) -> i32 {
    run_launcher_kind(std::env::args().collect(), kind)
}

pub fn run_any_current_executable(argv: Vec<String>) -> i32 {
    match PromptLauncherKind::from_argv(&argv) {
        Some(kind) => run_launcher_kind(argv, kind),
        None => {
            let arg0 = argv
                .first()
                .cloned()
                .unwrap_or_else(|| "<unknown>".to_string());
            eprintln!("retaprompt_input cannot infer launcher kind from executable name: {arg0}");
            eprintln!("expected one of: rp, rpl, rpb, rpe");
            1
        }
    }
}

pub fn run_any_current_executable_from_env() -> i32 {
    run_any_current_executable(std::env::args().collect())
}

pub fn run_rp(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptInputFrontendKind::Rp)
}

pub fn run_rpl(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptInputFrontendKind::Rpl)
}

pub fn run_rpe(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptInputFrontendKind::Rpe)
}

pub fn run_rp_from_env() -> i32 {
    run_kind_from_env(PromptInputFrontendKind::Rp)
}

pub fn run_rpl_from_env() -> i32 {
    run_kind_from_env(PromptInputFrontendKind::Rpl)
}

pub fn run_rpe_from_env() -> i32 {
    run_kind_from_env(PromptInputFrontendKind::Rpe)
}

pub fn run_kind_from_abi_value(kind: i32) -> i32 {
    match PromptInputFrontendKind::from_abi_value(kind) {
        Some(kind) => run_kind_from_env(kind),
        None => {
            eprintln!("invalid retaprompt input kind: {kind}");
            1
        }
    }
}

pub fn run_launcher_kind_from_abi_value(kind: i32) -> i32 {
    match PromptLauncherKind::from_abi_value(kind) {
        Some(kind) => run_launcher_kind_from_env(kind),
        None => {
            eprintln!("invalid retaprompt launcher kind: {kind}");
            1
        }
    }
}

fn ffi_guard_i32<F>(name: &str, f: F) -> i32
where
    F: FnOnce() -> i32,
{
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(exit_code) => exit_code,
        Err(_) => {
            eprintln!("panic inside {name}");
            101
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_kind_from_env(kind: i32) -> i32 {
    ffi_guard_i32("retaprompt_input_run_kind_from_env", || {
        run_kind_from_abi_value(kind)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_current_executable_from_env() -> i32 {
    ffi_guard_i32(
        "retaprompt_input_run_current_executable_from_env",
        run_current_executable_from_env,
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_any_current_executable_from_env() -> i32 {
    ffi_guard_i32(
        "retaprompt_input_run_any_current_executable_from_env",
        run_any_current_executable_from_env,
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_launcher_kind_from_env(kind: i32) -> i32 {
    ffi_guard_i32("retaprompt_input_run_launcher_kind_from_env", || {
        run_launcher_kind_from_abi_value(kind)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_rp_from_env() -> i32 {
    ffi_guard_i32("retaprompt_input_run_rp_from_env", run_rp_from_env)
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_rpl_from_env() -> i32 {
    ffi_guard_i32("retaprompt_input_run_rpl_from_env", run_rpl_from_env)
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_rpe_from_env() -> i32 {
    ffi_guard_i32("retaprompt_input_run_rpe_from_env", run_rpe_from_env)
}


fn ffi_string_or_null(text: String) -> *mut c_char {
    match CString::new(text) {
        Ok(value) => value.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

fn json_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 8);
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch.is_control() => out.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => out.push(ch),
        }
    }
    out
}

fn json_string(text: &str) -> String {
    format!("\"{}\"", json_escape(text))
}

fn accept_action_json(action: &completion::RightArrowAcceptAction) -> String {
    match action {
        completion::RightArrowAcceptAction::None => "{\"kind\":\"none\"}".to_string(),
        completion::RightArrowAcceptAction::Insert(text) => {
            format!("{{\"kind\":\"insert\",\"text\":{}}}", json_string(text))
        }
        completion::RightArrowAcceptAction::ReplaceRange {
            replace_start,
            replace_len,
            replacement,
        } => format!(
            "{{\"kind\":\"replace_range\",\"replace_start\":{},\"replace_len\":{},\"replacement\":{}}}",
            replace_start,
            replace_len,
            json_string(replacement)
        ),
    }
}


fn safe_ffi_cursor_position(line: &str, cursor: usize) -> usize {
    let mut pos = cursor.min(line.len());
    while pos > 0 && !line.is_char_boundary(pos) {
        pos -= 1;
    }
    pos
}

fn cursor_autosuggestion_json(line: &str, cursor: usize) -> String {
    let cursor = safe_ffi_cursor_position(line, cursor);
    match completion::autosuggestion_for_input_at_cursor(line, cursor) {
        Some(hint) => format!(
            concat!(
                "{{",
                "\"present\":true,",
                "\"cursor\":{},",
                "\"display\":{},",
                "\"insert\":{},",
                "\"replace_start\":{},",
                "\"replace_len\":{},",
                "\"replacement\":{},",
                "\"cursor_ghost\":{},",
                "\"tail_after_replace\":{},",
                "\"is_cursor_local\":{},",
                "\"accept_action\":{}",
                "}}"
            ),
            hint.cursor,
            json_string(&hint.display),
            json_string(&hint.insert),
            hint.replace_start,
            hint.replace_len,
            json_string(&hint.replacement),
            json_string(&hint.cursor_ghost),
            json_string(&hint.tail_after_replace),
            if hint.is_cursor_local { "true" } else { "false" },
            accept_action_json(&hint.accept_action),
        ),
        None => format!(
            "{{\"present\":false,\"cursor\":{},\"display\":\"\",\"insert\":\"\",\"accept_action\":{{\"kind\":\"none\"}}}}",
            cursor
        ),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_autosuggestion_at_cursor_json(
    line: *const c_char,
    cursor: usize,
) -> *mut c_char {
    if line.is_null() {
        return ffi_string_or_null(
            "{\"present\":false,\"error\":\"line pointer is null\"}".to_string(),
        );
    }

    let text = unsafe { CStr::from_ptr(line) };
    match text.to_str() {
        Ok(line) => ffi_string_or_null(cursor_autosuggestion_json(line, cursor)),
        Err(_) => ffi_string_or_null(
            "{\"present\":false,\"error\":\"line is not valid UTF-8\"}".to_string(),
        ),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
