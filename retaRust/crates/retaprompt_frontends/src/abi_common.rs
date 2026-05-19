use std::env;
use std::ffi::{CString, OsString};
use std::io::{self, Write};
use std::os::raw::{c_char, c_int};
use std::path::{Path, PathBuf};

use libloading::{library_filename, Library, Symbol};

pub type RetapromptAbiGeneration = unsafe extern "C" fn() -> u32;
pub type RetapromptRunKindArgv = unsafe extern "C" fn(
    kind: c_int,
    argc: usize,
    argv: *const *const c_char,
) -> c_int;

pub const REQUIRED_PROMPT_ABI_GENERATION: u32 = 2026051902;

#[derive(Clone, Copy, Debug)]
pub struct PromptLibrarySpec {
    logical_name: &'static str,
    display_name: &'static str,
    env_var: &'static str,
    generation_symbol: &'static [u8],
    run_kind_argv_symbol: &'static [u8],
}

impl PromptLibrarySpec {
    pub const fn new(
        logical_name: &'static str,
        display_name: &'static str,
        env_var: &'static str,
        generation_symbol: &'static [u8],
        run_kind_argv_symbol: &'static [u8],
    ) -> Self {
        Self {
            logical_name,
            display_name,
            env_var,
            generation_symbol,
            run_kind_argv_symbol,
        }
    }

    pub const fn logical_name(self) -> &'static str {
        self.logical_name
    }

    pub const fn display_name(self) -> &'static str {
        self.display_name
    }

    pub const fn env_var(self) -> &'static str {
        self.env_var
    }

    pub const fn generation_symbol(self) -> &'static [u8] {
        self.generation_symbol
    }

    pub const fn run_kind_argv_symbol(self) -> &'static [u8] {
        self.run_kind_argv_symbol
    }
}

pub struct LoadedPromptLibrary {
    spec: PromptLibrarySpec,
    path: PathBuf,
    library: Library,
}

impl LoadedPromptLibrary {
    pub fn load(spec: PromptLibrarySpec) -> Result<Self, String> {
        let mut errors = Vec::new();
        for candidate in prompt_library_candidates(spec) {
            let display = candidate.display().to_string();
            match unsafe { Library::new(&candidate) } {
                Ok(library) => match validate_prompt_library(spec, &library) {
                    Ok(()) => {
                        return Ok(Self {
                            spec,
                            path: candidate,
                            library,
                        })
                    }
                    Err(error) => errors.push(format!("{display}: {error}")),
                },
                Err(error) => errors.push(format!("{display}: {error}")),
            }
        }

        let fallback_name = library_filename(spec.logical_name());
        let fallback_display = fallback_name.to_string_lossy().into_owned();
        match unsafe { Library::new(&fallback_name) } {
            Ok(library) => match validate_prompt_library(spec, &library) {
                Ok(()) => Ok(Self {
                    spec,
                    path: PathBuf::from(fallback_name),
                    library,
                }),
                Err(error) => {
                    errors.push(format!("{fallback_display}: {error}"));
                    Err(format!(
                        "could not load compatible {}; tried {}",
                        spec.display_name(),
                        errors.join(" | ")
                    ))
                }
            },
            Err(error) => {
                errors.push(format!("{fallback_display}: {error}"));
                Err(format!(
                    "could not load {}; tried {}",
                    spec.display_name(),
                    errors.join(" | ")
                ))
            }
        }
    }

    pub unsafe fn run_kind_argv(&self, kind_value: c_int) -> Result<c_int, String> {
        let run: Symbol<'_, RetapromptRunKindArgv> = unsafe {
            self.library
                .get(self.spec.run_kind_argv_symbol())
                .map_err(|error| {
                    format!(
                        "{} at {} is missing symbol {}: {error}",
                        self.spec.display_name(),
                        self.path.display(),
                        String::from_utf8_lossy(self.spec.run_kind_argv_symbol())
                    )
                })?
        };

        let argv_cstrings = current_argv_cstrings();
        let argv_ptrs = argv_cstrings
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<_>>();
        Ok(unsafe { run(kind_value, argv_ptrs.len(), argv_ptrs.as_ptr()) })
    }
}

fn validate_prompt_library(spec: PromptLibrarySpec, library: &Library) -> Result<(), String> {
    let generation: Symbol<'_, RetapromptAbiGeneration> = unsafe {
        library
            .get(spec.generation_symbol())
            .map_err(|error| {
                format!(
                    "missing symbol {}: {error}",
                    String::from_utf8_lossy(spec.generation_symbol())
                )
            })?
    };
    let actual = unsafe { generation() };
    if actual == REQUIRED_PROMPT_ABI_GENERATION {
        Ok(())
    } else {
        Err(format!(
            "ABI generation {actual}, expected {REQUIRED_PROMPT_ABI_GENERATION}"
        ))
    }
}

fn prompt_library_candidates(spec: PromptLibrarySpec) -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(path) = env::var(spec.env_var()) {
        push_nonempty_path(&mut candidates, path);
    }

    let filename = PathBuf::from(library_filename(spec.logical_name()));

    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            push_candidate_family(&mut candidates, dir, &filename);
        }
    }

    if let Some(profile_dir) = cargo_profile_dir_from_env() {
        push_candidate_family(&mut candidates, &profile_dir, &filename);
    }

    if let Ok(cwd) = env::current_dir() {
        for profile in ["debug", "release"] {
            push_candidate_family(&mut candidates, &cwd.join("target").join(profile), &filename);
        }
    }

    dedup_paths(candidates)
}

fn push_candidate_family(candidates: &mut Vec<PathBuf>, dir: &Path, filename: &Path) {
    candidates.push(dir.join(filename));
    candidates.push(dir.join("deps").join(filename));
    candidates.push(dir.join("lib").join(filename));
    candidates.push(dir.join("..").join("lib").join(filename));
    candidates.push(dir.join("..").join("deps").join(filename));
}

fn push_nonempty_path(candidates: &mut Vec<PathBuf>, path: String) {
    let path = path.trim();
    if !path.is_empty() {
        candidates.push(PathBuf::from(path));
    }
}

fn cargo_profile_dir_from_env() -> Option<PathBuf> {
    let target_dir = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target"));
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let candidate = target_dir.join(profile);
    if candidate.exists() {
        Some(candidate)
    } else {
        None
    }
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

fn current_argv_cstrings() -> Vec<CString> {
    let args = env::args_os().collect::<Vec<OsString>>();
    let mut out = Vec::with_capacity(args.len().max(1));
    for arg in args {
        out.push(to_c_string_lossy(&arg.to_string_lossy()));
    }
    if out.is_empty() {
        out.push(CString::new("retaprompt").expect("static fallback has no null"));
    }
    out
}

fn to_c_string_lossy(text: &str) -> CString {
    let sanitized = text.replace('\0', "�");
    CString::new(sanitized).expect("sanitized launcher string must not contain interior null bytes")
}

pub fn fail_runtime(message: &str) -> c_int {
    let _ = writeln!(
        io::stderr(),
        "retaprompt launcher failed: {message}\n\nBuild the split prompt shared libraries first with ./build.sh debug or ./build.sh release.\nOverride paths with RETAPROMPT_INPUT_LIB_PATH / RETAPROMPT_COMMANDS_LIB_PATH if needed."
    );
    127
}
