use std::env;
use std::ffi::CString;
use std::io::{self, IsTerminal, Read, Write};
use std::os::raw::c_char;
use std::path::PathBuf;

use libloading::{library_filename, Library, Symbol};

#[repr(C)]
struct RetaFfiResponse {
    stdout_text: *mut c_char,
    stdout_len: usize,
    stderr_text: *mut c_char,
    stderr_len: usize,
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
type RetaAbiVersionFn = unsafe extern "C" fn() -> u32;

const EXPECTED_RETA_ABI_VERSION: u32 = 2;
const MAX_FFI_RESPONSE_BYTES: usize = 1024 * 1024 * 1024;

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
        let abi_version: Symbol<'_, RetaAbiVersionFn> = match library.get(b"reta_abi_version") {
            Ok(symbol) => symbol,
            Err(error) => {
                let _ = writeln!(
                    io::stderr(),
                    "reta launcher failed: missing symbol reta_abi_version: {error}"
                );
                return 127;
            }
        };
        let actual_abi = abi_version();
        if actual_abi != EXPECTED_RETA_ABI_VERSION {
            let _ = writeln!(
                io::stderr(),
                "reta launcher failed: incompatible libreta ABI {actual_abi}, expected {EXPECTED_RETA_ABI_VERSION}"
            );
            return 127;
        }

        let run: Symbol<'_, RetaRunArgvFn> = match library.get(b"reta_run_argv") {
            Ok(symbol) => symbol,
            Err(error) => {
                let _ = writeln!(
                    io::stderr(),
                    "reta launcher failed: missing symbol reta_run_argv: {error}"
                );
                return 127;
            }
        };

        let free: Symbol<'_, RetaFreeStringFn> = match library.get(b"reta_free_string") {
            Ok(symbol) => symbol,
            Err(error) => {
                let _ = writeln!(
                    io::stderr(),
                    "reta launcher failed: missing symbol reta_free_string: {error}"
                );
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

        let stderr_text = take_owned_response_string(response.stderr_text, response.stderr_len, &free);
        if !stderr_text.is_empty() {
            let _ = write!(io::stderr().lock(), "{stderr_text}");
        }

        let stdout_text = take_owned_response_string(response.stdout_text, response.stdout_len, &free);
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
            Ok(library) => match validate_reta_library_abi(&library) {
                Ok(()) => return Ok(library),
                Err(error) => errors.push(format!("{display}: {error}")),
            },
            Err(error) => errors.push(format!("{display}: {error}")),
        }
    }

    let fallback_name = library_filename("reta");
    let fallback_display = fallback_name.to_string_lossy().into_owned();
    match unsafe { Library::new(&fallback_name) } {
        Ok(library) => match validate_reta_library_abi(&library) {
            Ok(()) => Ok(library),
            Err(error) => {
                errors.push(format!("{fallback_display}: {error}"));
                Err(format!(
                    "could not load compatible libreta shared library; tried {}",
                    errors.join(" | ")
                ))
            }
        },
        Err(error) => {
            errors.push(format!("{fallback_display}: {error}"));
            Err(format!(
                "could not load libreta shared library; tried {}",
                errors.join(" | ")
            ))
        }
    }
}

fn validate_reta_library_abi(library: &Library) -> Result<(), String> {
    let abi_version = unsafe {
        library
            .get::<RetaAbiVersionFn>(b"reta_abi_version")
            .map_err(|error| format!("missing symbol reta_abi_version: {error}"))?
    };
    let actual = unsafe { abi_version() };
    if actual == EXPECTED_RETA_ABI_VERSION {
        Ok(())
    } else {
        Err(format!(
            "incompatible libreta ABI {actual}, expected {EXPECTED_RETA_ABI_VERSION}"
        ))
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

unsafe fn take_owned_response_string(
    ptr: *mut c_char,
    len: usize,
    free: &Symbol<'_, RetaFreeStringFn>,
) -> String {
    if ptr.is_null() {
        return String::new();
    }

    let text = unsafe { read_c_string_lossy_with_known_len(ptr, len, MAX_FFI_RESPONSE_BYTES) }
        .unwrap_or_else(|message| format!("<invalid libreta string: {message}>"));
    unsafe {
        free(ptr);
    }
    text
}

unsafe fn read_c_string_lossy_with_known_len(
    ptr: *const c_char,
    len: usize,
    max_bytes: usize,
) -> Result<String, String> {
    if len > max_bytes {
        return Err(format!(
            "C string response length {len} exceeds maximum {max_bytes} bytes"
        ));
    }
    let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) };
    Ok(String::from_utf8_lossy(bytes).into_owned())
}
