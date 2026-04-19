#![allow(non_snake_case)]

use std::ffi::{CStr, CString};
use std::io::{self, IsTerminal};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::sync::OnceLock;

use libloading::{library_filename, Library};

pub mod shared {
    pub mod words_py {
        use indexmap::IndexMap;
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        pub enum PyValue {
            Int(i64),
            Str(String),
            Bool(bool),
            Tuple(Vec<PyValue>),
            NoneValue,
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct StoreParameterEntry {
            pub parameterMainNames: Vec<String>,
            pub parameterNames: Vec<String>,
            pub datas: Vec<Vec<PyValue>>,
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Words {
            pub paraNdataMatrix: Vec<StoreParameterEntry>,
            pub kombiParaNdataMatrix: IndexMap<i64, Vec<String>>,
            pub kombiParaNdataMatrix2: IndexMap<i64, Vec<String>>,
        }

        impl Words {
            pub fn empty() -> Self {
                Self {
                    paraNdataMatrix: Vec::new(),
                    kombiParaNdataMatrix: IndexMap::new(),
                    kombiParaNdataMatrix2: IndexMap::new(),
                }
            }
        }
    }
}

pub mod domain {
    pub mod python_source_of_truth {
        use std::collections::BTreeMap;
        use std::ffi::{CStr, CString};
        use std::os::raw::c_char;
        use std::sync::{Mutex, OnceLock};

        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
        pub struct PythonAliasGroup {
            pub canonical: String,
            pub aliases: Vec<String>,
        }

        type FreeStringFn = unsafe extern "C" fn(*mut c_char);

        static ALL_MAIN_ALIAS_GROUPS_CACHE: OnceLock<Vec<PythonAliasGroup>> = OnceLock::new();
        static PARAMETER_ALIAS_GROUPS_CACHE: OnceLock<Mutex<BTreeMap<String, Vec<PythonAliasGroup>>>> =
            OnceLock::new();
        static MAIN_ALIAS_RESOLUTION_CACHE: OnceLock<BTreeMap<String, String>> = OnceLock::new();

        pub fn all_main_alias_groups(
            _words: &crate::shared::words_py::Words,
        ) -> Vec<PythonAliasGroup> {
            ALL_MAIN_ALIAS_GROUPS_CACHE
                .get_or_init(|| load_alias_groups_from_json(b"reta_all_main_alias_groups_json", None))
                .clone()
        }

        pub fn parameter_alias_groups_for_main(
            _words: &crate::shared::words_py::Words,
            canonical_main: &str,
        ) -> Vec<PythonAliasGroup> {
            let cache = PARAMETER_ALIAS_GROUPS_CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
            let cache_key = normalize_alias_like_python(canonical_main);

            if let Ok(guard) = cache.lock() {
                if let Some(groups) = guard.get(&cache_key) {
                    return groups.clone();
                }
            }

            let groups = load_alias_groups_from_json(
                b"reta_parameter_alias_groups_for_main_json",
                Some(canonical_main),
            );

            if let Ok(mut guard) = cache.lock() {
                guard.entry(cache_key).or_insert_with(|| groups.clone());
            }

            groups
        }

        pub fn resolve_parameter_main_alias(
            _words: &crate::shared::words_py::Words,
            main_alias: &str,
        ) -> Option<String> {
            MAIN_ALIAS_RESOLUTION_CACHE
                .get_or_init(build_main_alias_resolution_cache)
                .get(&normalize_alias_like_python(main_alias))
                .cloned()
        }

        fn load_alias_groups_from_json(
            symbol_name: &[u8],
            canonical_main: Option<&str>,
        ) -> Vec<PythonAliasGroup> {
            let free = match crate::reta_free_string_fn() {
                Ok(free) => free,
                Err(message) => {
                    eprintln!(
                        "retaprompt_commands could not resolve libreta free-string ABI for alias groups: {message}"
                    );
                    return Vec::new();
                }
            };

            let json = if let Some(main) = canonical_main {
                let get_json = match crate::reta_parameter_alias_groups_for_main_json_fn() {
                    Ok(get_json) => get_json,
                    Err(message) => {
                        let display = String::from_utf8_lossy(symbol_name);
                        eprintln!(
                            "retaprompt_commands could not resolve libreta symbol {display}: {message}"
                        );
                        return Vec::new();
                    }
                };
                let main = to_c_string_lossy(main);
                let ptr = unsafe { get_json(main.as_ptr()) };
                unsafe { take_owned_string(ptr, free) }
            } else {
                let get_json = match crate::reta_all_main_alias_groups_json_fn() {
                    Ok(get_json) => get_json,
                    Err(message) => {
                        let display = String::from_utf8_lossy(symbol_name);
                        eprintln!(
                            "retaprompt_commands could not resolve libreta symbol {display}: {message}"
                        );
                        return Vec::new();
                    }
                };
                let ptr = unsafe { get_json() };
                unsafe { take_owned_string(ptr, free) }
            };

            match serde_json::from_str::<Vec<PythonAliasGroup>>(&json) {
                Ok(groups) => groups,
                Err(error) => {
                    eprintln!(
                        "retaprompt_commands could not deserialize alias group metadata: {error}"
                    );
                    Vec::new()
                }
            }
        }

        fn build_main_alias_resolution_cache() -> BTreeMap<String, String> {
            let mut map = BTreeMap::new();
            for group in all_main_alias_groups(&crate::shared::words_py::Words::empty()) {
                for alias in &group.aliases {
                    map.insert(normalize_alias_like_python(alias), group.canonical.clone());
                }
            }
            map
        }

        fn normalize_alias_like_python(txt: &str) -> String {
            txt.trim().replace('ß', "ss").to_lowercase()
        }

        fn to_c_string_lossy(text: &str) -> CString {
            let sanitized = text.replace('\0', "�");
            CString::new(sanitized)
                .expect("sanitized alias string must not contain interior null bytes")
        }

        unsafe fn take_owned_string(ptr: *mut c_char, free: FreeStringFn) -> String {
            if ptr.is_null() {
                return String::new();
            }
            let owned = unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned();
            unsafe { free(ptr) };
            owned
        }
    }
}

#[path = "../../../src/prompt/tokenize.rs"]
pub mod tokenize;
#[path = "../../../src/prompt/semantic_choices.rs"]
pub mod semantic_choices;
#[path = "../../../src/prompt/python_like.rs"]
pub mod python_like;
#[path = "../../../src/prompt/commands.rs"]
pub mod commands;

#[derive(Debug, Clone, Default)]
pub struct RetaRunResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl RetaRunResult {
    pub fn render_text(&self) -> String {
        match (self.stdout.is_empty(), self.stderr.is_empty()) {
            (true, true) => String::new(),
            (false, true) => self.stdout.clone(),
            (true, false) => self.stderr.clone(),
            (false, false) => {
                let mut combined = String::with_capacity(self.stdout.len() + self.stderr.len() + 1);
                combined.push_str(&self.stdout);
                if !combined.ends_with('\n') && !self.stderr.starts_with('\n') {
                    combined.push('\n');
                }
                combined.push_str(&self.stderr);
                combined
            }
        }
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }
}

#[repr(C)]
struct RetaFfiResponse {
    stdout_text: *mut c_char,
    stderr_text: *mut c_char,
    exit_code: i32,
}

type RetaRunArgvFn = unsafe extern "C" fn(
    argc: usize,
    argv: *const *const c_char,
    stdin_text: *const c_char,
    terminal_width: usize,
    stdout_is_tty: u8,
    stderr_is_tty: u8,
    stdin_is_tty: u8,
) -> RetaFfiResponse;

type RetaFreeStringFn = unsafe extern "C" fn(*mut c_char);
type RetaSharedWordsJsonFn = unsafe extern "C" fn() -> *mut c_char;
type RetaAllMainAliasGroupsJsonFn = unsafe extern "C" fn() -> *mut c_char;
type RetaParameterAliasGroupsForMainJsonFn = unsafe extern "C" fn(*const c_char) -> *mut c_char;

static RETA_RUN_ARGV_FN: OnceLock<Result<RetaRunArgvFn, String>> = OnceLock::new();
static RETA_FREE_STRING_FN: OnceLock<Result<RetaFreeStringFn, String>> = OnceLock::new();
static RETA_SHARED_WORDS_JSON_FN: OnceLock<Result<RetaSharedWordsJsonFn, String>> = OnceLock::new();
static RETA_ALL_MAIN_ALIAS_GROUPS_JSON_FN: OnceLock<Result<RetaAllMainAliasGroupsJsonFn, String>> = OnceLock::new();
static RETA_PARAMETER_ALIAS_GROUPS_FOR_MAIN_JSON_FN: OnceLock<
    Result<RetaParameterAliasGroupsForMainJsonFn, String>,
> = OnceLock::new();

pub fn run_reta_from_args<A>(argv: A) -> RetaRunResult
where
    A: AsRef<[String]>,
{
    let args = argv.as_ref();

    let run = match reta_run_argv_fn() {
        Ok(run) => run,
        Err(message) => {
            return RetaRunResult {
                stdout: String::new(),
                stderr: format!("reta ABI lookup failed: {message}\n"),
                exit_code: 127,
            };
        }
    };
    let free = match reta_free_string_fn() {
        Ok(free) => free,
        Err(message) => {
            return RetaRunResult {
                stdout: String::new(),
                stderr: format!("reta ABI lookup failed: {message}\n"),
                exit_code: 127,
            };
        }
    };

    let argv_cstrings = args.iter().map(|arg| to_c_string_lossy(arg)).collect::<Vec<_>>();
    let argv_ptrs = argv_cstrings.iter().map(|arg| arg.as_ptr()).collect::<Vec<_>>();

    let response = unsafe {
        run(
            argv_ptrs.len(),
            argv_ptrs.as_ptr(),
            std::ptr::null(),
            detect_terminal_width().unwrap_or(0),
            io::stdout().is_terminal() as u8,
            io::stderr().is_terminal() as u8,
            io::stdin().is_terminal() as u8,
        )
    };

    let stderr = unsafe { take_owned_string(response.stderr_text, free) };
    let stdout = unsafe { take_owned_string(response.stdout_text, free) };

    RetaRunResult {
        stdout,
        stderr,
        exit_code: response.exit_code,
    }
}

static SHARED_WORDS: OnceLock<shared::words_py::Words> = OnceLock::new();

pub fn shared_words() -> &'static shared::words_py::Words {
    SHARED_WORDS.get_or_init(load_shared_words_snapshot)
}

fn load_shared_words_snapshot() -> shared::words_py::Words {
    let get_json = match reta_shared_words_json_fn() {
        Ok(get_json) => get_json,
        Err(message) => {
            eprintln!("retaprompt_commands could not resolve reta_shared_words_json: {message}");
            return shared::words_py::Words::empty();
        }
    };
    let free = match reta_free_string_fn() {
        Ok(free) => free,
        Err(message) => {
            eprintln!("retaprompt_commands could not resolve reta_free_string while loading words: {message}");
            return shared::words_py::Words::empty();
        }
    };

    let ptr = unsafe { get_json() };
    let json = unsafe { take_owned_string(ptr, free) };
    match serde_json::from_str::<shared::words_py::Words>(&json) {
        Ok(words) => words,
        Err(error) => {
            eprintln!("retaprompt_commands could not deserialize shared words snapshot: {error}");
            shared::words_py::Words::empty()
        }
    }
}

fn detect_terminal_width() -> Option<usize> {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|width| *width > 0)
}

static RETA_LIBRARY: OnceLock<Result<usize, String>> = OnceLock::new();

fn reta_library() -> Result<&'static Library, String> {
    match RETA_LIBRARY.get_or_init(|| {
        load_reta_library().map(|library| Box::leak(Box::new(library)) as *mut Library as usize)
    }) {
        Ok(ptr) => Ok(unsafe { &*(*ptr as *const Library) }),
        Err(message) => Err(message.clone()),
    }
}

fn load_reta_library() -> Result<Library, String> {
    let mut errors = Vec::new();
    for candidate in library_candidates("reta", "RETA_LIB_PATH") {
        let display = candidate.display().to_string();
        match unsafe { Library::new(&candidate) } {
            Ok(library) => return Ok(library),
            Err(error) => errors.push(format!("{display}: {error}")),
        }
    }
    Err(format!(
        "could not load libreta.so; tried {}",
        errors.join(" | ")
    ))
}

fn reta_run_argv_fn() -> Result<RetaRunArgvFn, String> {
    RETA_RUN_ARGV_FN
        .get_or_init(|| {
            let library = reta_library()?;
            unsafe {
                library
                    .get::<RetaRunArgvFn>(b"reta_run_argv")
                    .map(|symbol| *symbol)
                    .map_err(|error| format!("missing symbol reta_run_argv: {error}"))
            }
        })
        .clone()
}

fn reta_free_string_fn() -> Result<RetaFreeStringFn, String> {
    RETA_FREE_STRING_FN
        .get_or_init(|| {
            let library = reta_library()?;
            unsafe {
                library
                    .get::<RetaFreeStringFn>(b"reta_free_string")
                    .map(|symbol| *symbol)
                    .map_err(|error| format!("missing symbol reta_free_string: {error}"))
            }
        })
        .clone()
}

fn reta_shared_words_json_fn() -> Result<RetaSharedWordsJsonFn, String> {
    RETA_SHARED_WORDS_JSON_FN
        .get_or_init(|| {
            let library = reta_library()?;
            unsafe {
                library
                    .get::<RetaSharedWordsJsonFn>(b"reta_shared_words_json")
                    .map(|symbol| *symbol)
                    .map_err(|error| format!("missing symbol reta_shared_words_json: {error}"))
            }
        })
        .clone()
}

fn reta_all_main_alias_groups_json_fn() -> Result<RetaAllMainAliasGroupsJsonFn, String> {
    RETA_ALL_MAIN_ALIAS_GROUPS_JSON_FN
        .get_or_init(|| {
            let library = reta_library()?;
            unsafe {
                library
                    .get::<RetaAllMainAliasGroupsJsonFn>(b"reta_all_main_alias_groups_json")
                    .map(|symbol| *symbol)
                    .map_err(|error| format!("missing symbol reta_all_main_alias_groups_json: {error}"))
            }
        })
        .clone()
}

fn reta_parameter_alias_groups_for_main_json_fn() -> Result<RetaParameterAliasGroupsForMainJsonFn, String> {
    RETA_PARAMETER_ALIAS_GROUPS_FOR_MAIN_JSON_FN
        .get_or_init(|| {
            let library = reta_library()?;
            unsafe {
                library
                    .get::<RetaParameterAliasGroupsForMainJsonFn>(b"reta_parameter_alias_groups_for_main_json")
                    .map(|symbol| *symbol)
                    .map_err(|error| format!("missing symbol reta_parameter_alias_groups_for_main_json: {error}"))
            }
        })
        .clone()
}

pub fn preload_reta_bridge() -> Result<(), String> {
    let _ = reta_library()?;
    let _ = reta_run_argv_fn()?;
    let _ = reta_free_string_fn()?;
    let _ = reta_shared_words_json_fn()?;
    let _ = shared_words();
    Ok(())
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

fn to_c_string_lossy(text: &str) -> CString {
    let sanitized = text.replace('\0', "�");
    CString::new(sanitized).expect("sanitized string must not contain interior null bytes")
}

unsafe fn take_owned_string(ptr: *mut c_char, free: RetaFreeStringFn) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let owned = unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned();
    unsafe { free(ptr) };
    owned
}


pub use commands::{
    commands_text,
    compile_command,
    compile_command_with_state,
    execute_command,
    help_text,
    take_auto_prompt_command,
    EditModeKind,
    PromptCommand,
    PromptOutput,
    SessionState,
};
pub use python_like::PromptModus;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromptCommandFrontendKind {
    Rp,
    Rpl,
    Rpb,
    Rpe,
}

impl PromptCommandFrontendKind {
    pub fn from_abi_value(kind: i32) -> Option<Self> {
        match kind {
            1 => Some(Self::Rp),
            2 => Some(Self::Rpl),
            3 => Some(Self::Rpb),
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
            "rpb" => Some(Self::Rpb),
            "rpe" => Some(Self::Rpe),
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
            Self::Rpb => 3,
            Self::Rpe => 4,
        }
    }

    pub const fn program_name(self) -> &'static str {
        match self {
            Self::Rp => "rp",
            Self::Rpl => "rpl",
            Self::Rpb => "rpb",
            Self::Rpe => "rpe",
        }
    }

    fn default_exact_mode(self, argv: &[String]) -> bool {
        match self {
            Self::Rp => false,
            Self::Rpl => !argv.iter().any(|arg| arg == "-debug"),
            Self::Rpb | Self::Rpe => true,
        }
    }

    const fn emacs_output_mode(self) -> bool {
        matches!(self, Self::Rpe)
    }
}

#[derive(Clone, Debug)]
struct CommandStartupArgs {
    show_help: bool,
    exact_mode: bool,
    command_text: Option<String>,
    trailing_args: Vec<String>,
}

pub fn compile_for_rp(input: &str) -> Result<PromptCommand, String> {
    compile_command(input, PromptModus::Normal)
}

pub fn compile_for_rpl(input: &str) -> Result<PromptCommand, String> {
    compile_command(input, PromptModus::Normal)
}

pub fn compile_for_rpb(input: &str) -> Result<PromptCommand, String> {
    compile_command(input, PromptModus::Normal)
}

pub fn compile_for_rpe(input: &str) -> Result<PromptCommand, String> {
    compile_command(input, PromptModus::Normal)
}

pub fn run_kind(argv: Vec<String>, kind: PromptCommandFrontendKind) -> i32 {
    run_command_one_shot_frontend(argv, kind)
}

pub fn run_kind_from_env(kind: PromptCommandFrontendKind) -> i32 {
    run_kind(std::env::args().collect(), kind)
}

pub fn run_current_executable(argv: Vec<String>) -> i32 {
    match PromptCommandFrontendKind::from_argv(&argv) {
        Some(kind) => run_kind(argv, kind),
        None => {
            let arg0 = argv.first().cloned().unwrap_or_else(|| "<unknown>".to_string());
            eprintln!(
                "retaprompt_commands cannot infer command frontend kind from executable name: {arg0}"
            );
            eprintln!("expected one of: rp, rpl, rpb, rpe");
            1
        }
    }
}

pub fn run_current_executable_from_env() -> i32 {
    run_current_executable(std::env::args().collect())
}

pub fn run_rp(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptCommandFrontendKind::Rp)
}

pub fn run_rpl(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptCommandFrontendKind::Rpl)
}

pub fn run_rpb(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptCommandFrontendKind::Rpb)
}

pub fn run_rpe(argv: Vec<String>) -> i32 {
    run_kind(argv, PromptCommandFrontendKind::Rpe)
}

pub fn run_rp_from_env() -> i32 {
    run_kind_from_env(PromptCommandFrontendKind::Rp)
}

pub fn run_rpl_from_env() -> i32 {
    run_kind_from_env(PromptCommandFrontendKind::Rpl)
}

pub fn run_rpb_from_env() -> i32 {
    run_kind_from_env(PromptCommandFrontendKind::Rpb)
}

pub fn run_rpe_from_env() -> i32 {
    run_kind_from_env(PromptCommandFrontendKind::Rpe)
}

pub fn run_kind_from_abi_value(kind: i32) -> i32 {
    match PromptCommandFrontendKind::from_abi_value(kind) {
        Some(kind) => run_kind_from_env(kind),
        None => {
            eprintln!("invalid retaprompt command kind: {kind}");
            1
        }
    }
}

pub fn run_input_kind_from_abi_value(kind: i32) -> i32 {
    eprintln!(
        "retaprompt input frontends live in libretaprompt_input.so; retaprompt_commands received kind {kind}"
    );
    1
}

fn parse_command_startup_args(argv: &[String], kind: PromptCommandFrontendKind) -> CommandStartupArgs {
    let mut startup = CommandStartupArgs {
        show_help: false,
        exact_mode: kind.default_exact_mode(argv),
        command_text: None,
        trailing_args: Vec::new(),
    };

    let mut index = 1usize;
    while index < argv.len() {
        match argv[index].as_str() {
            "-e" => {
                startup.exact_mode = true;
                index += 1;
            }
            "-h" | "-help" | "--help" => {
                startup.show_help = true;
                index += 1;
            }
            "-debug" | "-vi" | "-log" => {
                index += 1;
            }
            "-befehl" | "-command" => {
                startup.command_text = Some(argv[index + 1..].join(" "));
                return startup;
            }
            _ => {
                startup.trailing_args = argv[index..].to_vec();
                break;
            }
        }
    }

    if startup.command_text.is_none() && !startup.trailing_args.is_empty() {
        startup.command_text = Some(startup.trailing_args.join(" "));
    }

    startup
}

fn run_command_one_shot_frontend(argv: Vec<String>, kind: PromptCommandFrontendKind) -> i32 {
    if let Err(message) = preload_reta_bridge() {
        eprintln!("retaprompt_commands could not preload libreta.so: {message}");
    }

    let startup = parse_command_startup_args(&argv, kind);
    if startup.show_help {
        println!("{}", help_text());
        return 0;
    }

    let program_name = program_name_from_argv(&argv).unwrap_or_else(|| kind.program_name().to_string());
    let mut input = startup
        .command_text
        .unwrap_or_else(|| startup.trailing_args.join(" "));
    if startup.exact_mode {
        input = apply_exact_mode_to_input(&input);
    }

    run_command_one_shot(input, program_name, kind.emacs_output_mode())
}

fn run_command_one_shot(input: String, program_name: String, emacs_output_mode: bool) -> i32 {
    let mut state = SessionState::new(program_name.clone(), true, false);

    if input.trim().is_empty() {
        let output = PromptOutput {
            title: "usage".to_string(),
            text: format!(
                "{program_name} erwartet einen direkten Befehl als Argument, z. B. '{program_name} av12-15' oder '{program_name} reta -zeilen --alles'."
            ),
            exit_code: 1,
        };
        print_command_output(&mut state, output.clone());
        return output.exit_code;
    }

    state.previous_input = state.last_input.clone();
    state.last_input = input.clone();
    state.history_lines.push(input.clone());

    let compiled = match compile_command_with_state(&input, &state) {
        Ok(command) => {
            if emacs_output_mode {
                apply_rpe_emacs_output_to_command(command, &input)
            } else {
                command
            }
        }
        Err(err) => {
            let output = PromptOutput {
                title: "error".to_string(),
                text: err,
                exit_code: 1,
            };
            print_command_output(&mut state, output.clone());
            return output.exit_code;
        }
    };

    if matches!(&compiled, PromptCommand::Exit) {
        return 0;
    }

    if matches!(&compiled, PromptCommand::LaunchUi) {
        let output = PromptOutput {
            title: "ui-error".to_string(),
            text: format!("{program_name} unterstützt keinen interaktiven UI-Start."),
            exit_code: 1,
        };
        print_command_output(&mut state, output.clone());
        return output.exit_code;
    }

    match execute_command(compiled, &mut state) {
        Ok(Some(output)) => {
            print_command_output(&mut state, output.clone());
            output.exit_code
        }
        Ok(None) => {
            if let Some(auto_command) = take_auto_prompt_command(&mut state) {
                match execute_command(auto_command, &mut state) {
                    Ok(Some(output)) => {
                        print_command_output(&mut state, output.clone());
                        output.exit_code
                    }
                    Ok(None) => 0,
                    Err(err) => {
                        let output = PromptOutput {
                            title: "error".to_string(),
                            text: err,
                            exit_code: 1,
                        };
                        print_command_output(&mut state, output.clone());
                        output.exit_code
                    }
                }
            } else {
                0
            }
        }
        Err(err) => {
            let output = PromptOutput {
                title: "error".to_string(),
                text: err,
                exit_code: 1,
            };
            print_command_output(&mut state, output.clone());
            output.exit_code
        }
    }
}

fn program_name_from_argv(argv: &[String]) -> Option<String> {
    argv.first().map(|arg0| {
        std::path::PathBuf::from(arg0)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| arg0.clone())
    })
}

fn apply_exact_mode_to_input(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() || !should_append_exact_suffix(trimmed) {
        return trimmed.to_string();
    }
    format!("{trimmed} keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar")
}

fn should_append_exact_suffix(input: &str) -> bool {
    let tokenized = match tokenize::split_shell_like(input) {
        Ok(tokens) => tokens,
        Err(_) => return false,
    };

    if tokenized.tokens.is_empty() {
        return false;
    }

    let first = tokenized.tokens[0].as_str();
    if first == "reta" || first.starts_with('-') || first.starts_with(':') {
        return false;
    }

    if tokenized.tokens.iter().any(|token| {
        matches!(
            token.as_str(),
            "s"
                | "S"
                | "l"
                | "o"
                | "BefehlSpeichernDavor"
                | "BefehlSpeichernDanach"
                | "BefehlSpeicherungLöschen"
                | "BefehlSpeicherungAusgeben"
        )
    }) {
        return false;
    }

    !matches!(
        first,
        "help"
            | "hilfe"
            | "befehle"
            | "kurzbefehle"
            | "shell"
            | "python"
            | "math"
            | "q"
            | "quit"
            | "exit"
            | "ende"
            | "clear"
            | "leeren"
            | "loggen"
            | "nichtloggen"
    )
}

fn input_starts_with_reta(input: &str) -> bool {
    match tokenize::split_shell_like(input.trim()) {
        Ok(tokenized) => matches!(tokenized.tokens.first(), Some(token) if token == "reta"),
        Err(_) => false,
    }
}

fn rpe_output_group() -> Vec<String> {
    vec![
        "-ausgabe".to_string(),
        "--art=emacs".to_string(),
        "--keineueberschriften".to_string(),
    ]
}

fn apply_rpe_emacs_output_to_argv(
    mut argv: Vec<String>,
    append_after_user_args: bool,
) -> Vec<String> {
    let output_group = rpe_output_group();

    if argv.is_empty() {
        return output_group;
    }

    if append_after_user_args {
        argv.extend(output_group);
        argv
    } else {
        let mut rebuilt = vec![argv[0].clone()];
        rebuilt.extend(output_group);
        rebuilt.extend(argv.into_iter().skip(1));
        rebuilt
    }
}

fn apply_rpe_emacs_output_to_command(command: PromptCommand, input: &str) -> PromptCommand {
    let append_after_user_args = input_starts_with_reta(input);

    match command {
        PromptCommand::Reta(argv) => {
            PromptCommand::Reta(apply_rpe_emacs_output_to_argv(argv, append_after_user_args))
        }
        PromptCommand::RetaBatch(argvs) => PromptCommand::RetaBatch(
            argvs
                .into_iter()
                .map(|argv| apply_rpe_emacs_output_to_argv(argv, append_after_user_args))
                .collect(),
        ),
        PromptCommand::Sequence(commands) => PromptCommand::Sequence(
            commands
                .into_iter()
                .map(|command| apply_rpe_emacs_output_to_command(command, input))
                .collect(),
        ),
        other => other,
    }
}

fn print_command_output(state: &mut SessionState, output: PromptOutput) {
    state.last_output = output.clone();
    if !output.text.is_empty() {
        println!("{}", output.text);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_run_kind_from_env(kind: i32) -> i32 {
    run_kind_from_abi_value(kind)
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_run_current_executable_from_env() -> i32 {
    run_current_executable_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_run_rp_from_env() -> i32 {
    run_rp_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_run_rpl_from_env() -> i32 {
    run_rpl_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_run_rpb_from_env() -> i32 {
    run_rpb_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_run_rpe_from_env() -> i32 {
    run_rpe_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn retaprompt_commands_input_run_kind_from_env(kind: i32) -> i32 {
    run_input_kind_from_abi_value(kind)
}
