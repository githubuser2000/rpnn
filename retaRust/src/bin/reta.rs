use std::env;
use std::ffi::{CStr, CString};
use std::io::{self, IsTerminal, Read, Write};
use std::os::raw::c_char;
use std::path::PathBuf;

use libloading::{Library, Symbol, library_filename};

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

type RetaFreeStringFn = unsafe extern "C" fn(ptr: *mut c_char);

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let stdin_text = read_stdin_if_piped();

    let library = match load_reta_library() {
        Ok(library) => library,
        Err(message) => {
            let _ = writeln!(io::stderr(), "reta launcher failed: {message}");
            return 127;
        }
    };

    unsafe {
        let run: Symbol<'_, RetaRunArgvFn> = match library.get(b"reta_run_argv") {
            Ok(symbol) => symbol,
            Err(error) => {
                let _ = writeln!(io::stderr(), "reta launcher failed: missing symbol reta_run_argv: {error}");
                return 127;
            }
        };

        let free: Symbol<'_, RetaFreeStringFn> = match library.get(b"reta_free_string") {
            Ok(symbol) => symbol,
            Err(error) => {
                let _ = writeln!(io::stderr(), "reta launcher failed: missing symbol reta_free_string: {error}");
                return 127;
            }
        };

        let argv_cstrings = args
            .iter()
            .map(|arg| to_c_string_lossy(arg))
            .collect::<Vec<_>>();
        let argv_ptrs = argv_cstrings
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<_>>();
        let stdin_cstring = stdin_text.as_deref().map(to_c_string_lossy);

        let response = run(
            argv_ptrs.len(),
            argv_ptrs.as_ptr(),
            stdin_cstring
                .as_ref()
                .map_or(std::ptr::null(), |text| text.as_ptr()),
            detect_terminal_width().unwrap_or(0),
            io::stdout().is_terminal() as u8,
            io::stderr().is_terminal() as u8,
            io::stdin().is_terminal() as u8,
        );

        let stderr_text = take_owned_string(response.stderr_text, &free);
        if !stderr_text.is_empty() {
            let _ = write!(io::stderr().lock(), "{stderr_text}");
        }

        let stdout_text = take_owned_string(response.stdout_text, &free);
        if !stdout_text.is_empty() {
            let _ = write!(io::stdout().lock(), "{stdout_text}");
        }

        response.exit_code
    }
}

fn read_stdin_if_piped() -> Option<String> {
    if io::stdin().is_terminal() {
        return None;
    }

    let mut buf = String::new();
    match io::stdin().read_to_string(&mut buf) {
        Ok(_) if !buf.is_empty() => Some(buf),
        _ => None,
    }
}

fn detect_terminal_width() -> Option<usize> {
    env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|width| *width > 0)
}

fn load_reta_library() -> Result<Library, String> {
    let mut errors = Vec::new();

    for candidate in reta_library_candidates() {
        let display = candidate.display().to_string();
        match unsafe { Library::new(&candidate) } {
            Ok(library) => return Ok(library),
            Err(error) => errors.push(format!("{display}: {error}")),
        }
    }

    let fallback_name = library_filename("reta");
    let fallback_display = fallback_name.to_string_lossy().into_owned();
    match unsafe { Library::new(&fallback_name) } {
        Ok(library) => Ok(library),
        Err(error) => {
            errors.push(format!("{fallback_display}: {error}"));
            Err(format!(
                "could not load libreta shared library; tried {}",
                errors.join(" | ")
            ))
        }
    }
}

fn reta_library_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(path) = env::var("RETA_LIB_PATH") {
        let path = path.trim();
        if !path.is_empty() {
            candidates.push(PathBuf::from(path));
        }
    }

    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            let filename = PathBuf::from(library_filename("reta"));
            candidates.push(dir.join(&filename));
            candidates.push(dir.join("lib").join(&filename));
            candidates.push(dir.join("..").join("lib").join(&filename));
        }
    }

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
    CString::new(sanitized).expect("sanitized launcher string must not contain interior null bytes")
}

unsafe fn take_owned_string(ptr: *mut c_char, free: &Symbol<'_, RetaFreeStringFn>) -> String {
    if ptr.is_null() {
        return String::new();
    }

    let text = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { free(ptr); }
    text
}
