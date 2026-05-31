use std::env;
use std::ffi::{c_void, CString};
use std::io::{self, IsTerminal, Read, Write};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::sync::Mutex;

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

type RetaStreamChunkFn = unsafe extern "C" fn(
    kind: u8,
    data: *const u8,
    len: usize,
    user_data: *mut c_void,
) -> i32;

#[repr(C)]
struct RetaFfiStreamResponse {
    exit_code: i32,
    stdout_chunks: usize,
    stderr_chunks: usize,
    stdout_bytes: usize,
    stderr_bytes: usize,
    stdout_lines: usize,
    stderr_lines: usize,
    callback_error: i32,
}

type RetaRunArgvStreamFn = unsafe extern "C" fn(
    argc: usize,
    argv: *const *const c_char,
    stdin_text: *const c_char,
    terminal_width: usize,
    stdout_is_tty: u8,
    stderr_is_tty: u8,
    stdin_is_tty: u8,
    callback: Option<RetaStreamChunkFn>,
    user_data: *mut c_void,
) -> RetaFfiStreamResponse;

type RetaFreeStringFn = unsafe extern "C" fn(ptr: *mut c_char);
type RetaAbiVersionFn = unsafe extern "C" fn() -> u32;

const EXPECTED_RETA_ABI_VERSION: u32 = 2;
const MAX_FFI_RESPONSE_BYTES: usize = 1024 * 1024 * 1024;
const RETA_STREAM_KIND_STDOUT: u8 = 1;
const RETA_STREAM_KIND_STDERR: u8 = 2;

struct LauncherStreamContext {
    stdout: Mutex<io::Stdout>,
    stderr: Mutex<io::Stderr>,
}

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
        drop(abi_version);

        let argv_cstrings = args
            .iter()
            .map(|arg| to_c_string_lossy(arg))
            .collect::<Vec<_>>();
        let argv_ptrs = argv_cstrings
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<_>>();
        let stdin_cstring = stdin_text.as_deref().map(to_c_string_lossy);
        let stdin_ptr = stdin_cstring
            .as_ref()
            .map_or(std::ptr::null(), |text| text.as_ptr());
        let terminal_width = detect_terminal_width().unwrap_or(0);
        let stdout_is_tty = io::stdout().is_terminal() as u8;
        let stderr_is_tty = io::stderr().is_terminal() as u8;
        let stdin_is_tty = io::stdin().is_terminal() as u8;

        if let Ok(run_stream) = library.get::<RetaRunArgvStreamFn>(b"reta_run_argv_stream") {
            let context = LauncherStreamContext {
                stdout: Mutex::new(io::stdout()),
                stderr: Mutex::new(io::stderr()),
            };
            let response = run_stream(
                argv_ptrs.len(),
                argv_ptrs.as_ptr(),
                stdin_ptr,
                terminal_width,
                stdout_is_tty,
                stderr_is_tty,
                stdin_is_tty,
                Some(launcher_stream_chunk),
                &context as *const LauncherStreamContext as *mut c_void,
            );

            let _ = context.stdout.lock().map(|mut stdout| stdout.flush());
            let _ = context.stderr.lock().map(|mut stderr| stderr.flush());

            let exit_code = if response.callback_error != 0 {
                let _ = writeln!(
                    io::stderr(),
                    "reta launcher stream callback failed: {}",
                    response.callback_error
                );
                120
            } else {
                response.exit_code
            };
            drop(run_stream);
            // Do not dlclose the Rust shared library after a successful run.
            // On small Android/Termux systems the late unload/destructor path can
            // segfault after output has already been written correctly.  Let the
            // OS reclaim the mapping at process exit instead.
            std::mem::forget(library);
            return exit_code;
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

        let response = run(
            argv_ptrs.len(),
            argv_ptrs.as_ptr(),
            stdin_ptr,
            terminal_width,
            stdout_is_tty,
            stderr_is_tty,
            stdin_is_tty,
        );

        let stderr_text =
            take_owned_response_string(response.stderr_text, response.stderr_len, &free);
        if !stderr_text.is_empty() {
            let _ = write!(io::stderr().lock(), "{stderr_text}");
        }

        let stdout_text =
            take_owned_response_string(response.stdout_text, response.stdout_len, &free);
        if !stdout_text.is_empty() {
            let _ = write!(io::stdout().lock(), "{stdout_text}");
        }

        let exit_code = response.exit_code;
        drop(free);
        drop(run);
        std::mem::forget(library);
        exit_code
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
    detect_terminal_width_cmd("stty size < /dev/tty 2>/dev/null | awk '{print $2}'")
        .or_else(|| detect_terminal_width_cmd("tput cols 2>/dev/null"))
        .or_else(|| {
            env::var("COLUMNS")
                .ok()
                .and_then(|value| value.parse::<usize>().ok())
                .filter(|width| *width > 0)
        })
}

fn detect_terminal_width_cmd(cmd: &str) -> Option<usize> {
    let output = std::process::Command::new("sh")
        .arg("-lc")
        .arg(cmd)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let txt = String::from_utf8_lossy(&output.stdout).trim().to_string();
    txt.parse::<usize>().ok().filter(|width| *width > 0)
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
            // Cargo can leave the freshly-built cdylib under target/<profile>/deps
            // on Android/Termux.  Check that before falling back to a globally
            // installed libreta, otherwise the launcher may silently execute an
            // older library while the rreta binary itself is new.
            candidates.push(dir.join("deps").join(&filename));
            candidates.push(dir.join("lib").join(&filename));
            candidates.push(dir.join("..").join("lib").join(&filename));
            candidates.push(dir.join("..").join("deps").join(&filename));
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

unsafe extern "C" fn launcher_stream_chunk(
    kind: u8,
    data: *const u8,
    len: usize,
    user_data: *mut c_void,
) -> i32 {
    if user_data.is_null() {
        return -10;
    }
    if data.is_null() && len != 0 {
        return -11;
    }

    let context = unsafe { &*(user_data as *const LauncherStreamContext) };
    let bytes = if len == 0 {
        &[][..]
    } else {
        unsafe { std::slice::from_raw_parts(data, len) }
    };

    let result = match kind {
        RETA_STREAM_KIND_STDOUT => write_stream_chunk(&context.stdout, bytes),
        RETA_STREAM_KIND_STDERR => write_stream_chunk(&context.stderr, bytes),
        _ => return -12,
    };

    match result {
        Ok(()) => 0,
        Err(_) => -13,
    }
}

fn write_stream_chunk<W: Write>(stream: &Mutex<W>, bytes: &[u8]) -> io::Result<()> {
    let mut guard = stream
        .lock()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "stream mutex poisoned"))?;
    guard.write_all(bytes)
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
