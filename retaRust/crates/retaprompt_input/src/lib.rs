#![allow(non_snake_case)]

use std::path::PathBuf;

use libloading::{library_filename, Library};

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
            "rp" => Some(Self::Rp),
            "rpl" => Some(Self::Rpl),
            "rpe" => Some(Self::Rpe),
            _ => None,
        }
    }

    pub fn from_argv(argv: &[String]) -> Option<Self> {
        argv.first().and_then(|arg0| Self::from_program_name(arg0))
    }

    pub fn abi_value(self) -> i32 {
        match self {
            Self::Rp => 1,
            Self::Rpl => 2,
            Self::Rpe => 4,
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
            "rp" => Some(Self::Rp),
            "rpl" => Some(Self::Rpl),
            "rpb" => Some(Self::Rpb),
            "rpe" => Some(Self::Rpe),
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

type CommandsRunKindFn = unsafe extern "C" fn(i32) -> i32;

pub fn run_kind(argv: Vec<String>, kind: PromptInputFrontendKind) -> i32 {
    run_input_kind_via_commands(argv, kind)
}

pub fn run_kind_from_env(kind: PromptInputFrontendKind) -> i32 {
    run_kind(std::env::args().collect(), kind)
}

pub fn run_current_executable(argv: Vec<String>) -> i32 {
    match PromptInputFrontendKind::from_argv(&argv) {
        Some(kind) => run_kind(argv, kind),
        None => {
            let arg0 = argv.first().cloned().unwrap_or_else(|| "<unknown>".to_string());
            eprintln!(
                "retaprompt_input cannot infer input frontend kind from executable name: {arg0}"
            );
            1
        }
    }
}

pub fn run_current_executable_from_env() -> i32 {
    run_current_executable(std::env::args().collect())
}

pub fn run_launcher_kind(argv: Vec<String>, kind: PromptLauncherKind) -> i32 {
    match kind {
        PromptLauncherKind::Rp => run_input_kind_via_commands(argv, PromptInputFrontendKind::Rp),
        PromptLauncherKind::Rpl => run_input_kind_via_commands(argv, PromptInputFrontendKind::Rpl),
        PromptLauncherKind::Rpb => run_command_kind_via_commands(argv, 3),
        PromptLauncherKind::Rpe => run_input_kind_via_commands(argv, PromptInputFrontendKind::Rpe),
    }
}

pub fn run_launcher_kind_from_env(kind: PromptLauncherKind) -> i32 {
    run_launcher_kind(std::env::args().collect(), kind)
}

pub fn run_any_current_executable(argv: Vec<String>) -> i32 {
    match PromptLauncherKind::from_argv(&argv) {
        Some(kind) => run_launcher_kind(argv, kind),
        None => {
            let arg0 = argv.first().cloned().unwrap_or_else(|| "<unknown>".to_string());
            eprintln!(
                "retaprompt_input cannot infer launcher kind from executable name: {arg0}"
            );
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

fn run_input_kind_via_commands(_argv: Vec<String>, kind: PromptInputFrontendKind) -> i32 {
    with_commands_symbol(b"retaprompt_commands_input_run_kind_from_env", |symbol| unsafe {
        symbol(kind.abi_value())
    })
}

fn run_command_kind_via_commands(_argv: Vec<String>, kind: i32) -> i32 {
    with_commands_symbol(b"retaprompt_commands_run_kind_from_env", |symbol| unsafe {
        symbol(kind)
    })
}

fn with_commands_symbol<F>(symbol_name: &[u8], f: F) -> i32
where
    F: FnOnce(libloading::Symbol<'_, CommandsRunKindFn>) -> i32,
{
    let library = match load_commands_library() {
        Ok(library) => library,
        Err(message) => {
            eprintln!("retaprompt_input failed to load libretaprompt_commands.so: {message}");
            return 127;
        }
    };

    unsafe {
        match library.get::<CommandsRunKindFn>(symbol_name) {
            Ok(symbol) => f(symbol),
            Err(error) => {
                let display = String::from_utf8_lossy(symbol_name);
                eprintln!("retaprompt_input missing symbol {display}: {error}");
                127
            }
        }
    }
}

fn load_commands_library() -> Result<Library, String> {
    let mut errors = Vec::new();
    for candidate in library_candidates("retaprompt_commands", "RETAPROMPT_COMMANDS_LIB_PATH") {
        let display = candidate.display().to_string();
        match unsafe { Library::new(&candidate) } {
            Ok(library) => return Ok(library),
            Err(error) => errors.push(format!("{display}: {error}")),
        }
    }
    Err(format!(
        "could not load libretaprompt_commands.so; tried {}",
        errors.join(" | ")
    ))
}

fn library_candidates(base_name: &str, env_var: &str) -> Vec<PathBuf> {
    let filename = PathBuf::from(library_filename(base_name));
    let mut candidates = Vec::new();

    if let Ok(path) = std::env::var(env_var) {
        let path = path.trim();
        if !path.is_empty() {
            candidates.push(PathBuf::from(path));
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join(&filename));
            candidates.push(dir.join("lib").join(&filename));
            candidates.push(dir.join("..").join("lib").join(&filename));
        }
    }

    candidates.push(filename);
    dedup_paths(candidates)
}

fn dedup_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut deduped = Vec::new();
    for path in paths {
        if !deduped.iter().any(|existing| existing == &path) {
            deduped.push(path);
        }
    }
    deduped
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_kind_from_env(kind: i32) -> i32 {
    run_kind_from_abi_value(kind)
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_current_executable_from_env() -> i32 {
    run_current_executable_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_any_current_executable_from_env() -> i32 {
    run_any_current_executable_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_launcher_kind_from_env(kind: i32) -> i32 {
    run_launcher_kind_from_abi_value(kind)
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_rp_from_env() -> i32 {
    run_rp_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_rpl_from_env() -> i32 {
    run_rpl_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_input_run_rpe_from_env() -> i32 {
    run_rpe_from_env()
}
