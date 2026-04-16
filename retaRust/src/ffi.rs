use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::{run_reta_from_args_with_runtime, RetaRuntime};

#[repr(C)]
pub struct RetaFfiResponse {
    pub stdout_text: *mut c_char,
    pub stderr_text: *mut c_char,
    pub exit_code: i32,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_run_argv(
    argc: usize,
    argv: *const *const c_char,
    stdin_text: *const c_char,
    terminal_width: usize,
    stdout_is_tty: u8,
    stderr_is_tty: u8,
    stdin_is_tty: u8,
) -> RetaFfiResponse {
    let args = read_argv(argc, argv).unwrap_or_else(|message| vec![format!("--ffi-error={message}")]);
    let stdin_text = read_optional_string(stdin_text).ok().flatten();

    let result = run_reta_from_args_with_runtime(
        args,
        stdin_text,
        RetaRuntime {
            terminal_width: if terminal_width == 0 {
                None
            } else {
                Some(terminal_width)
            },
            stdout_is_tty: Some(stdout_is_tty != 0),
            stderr_is_tty: Some(stderr_is_tty != 0),
            stdin_is_tty: Some(stdin_is_tty != 0),
        },
    );

    let exit_code = result.exit_code;
    let stdout_text = into_c_string(result.stdout);
    let stderr_text = into_c_string(result.stderr);

    RetaFfiResponse {
        stdout_text,
        stderr_text,
        exit_code,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

fn read_argv(argc: usize, argv: *const *const c_char) -> Result<Vec<String>, String> {
    if argc == 0 {
        return Ok(Vec::new());
    }

    if argv.is_null() {
        return Err("argv war null bei argc > 0".to_string());
    }

    let mut args = Vec::with_capacity(argc);
    for index in 0..argc {
        let arg_ptr = unsafe { *argv.add(index) };
        let arg = read_required_string(arg_ptr)
            .map_err(|_| format!("argv[{index}] ist kein valider UTF-8-String"))?;
        args.push(arg);
    }

    Ok(args)
}

fn read_required_string(ptr: *const c_char) -> Result<String, ()> {
    if ptr.is_null() {
        return Ok(String::new());
    }

    let text = unsafe { CStr::from_ptr(ptr) }.to_str().map_err(|_| ())?;
    Ok(text.to_string())
}

fn read_optional_string(ptr: *const c_char) -> Result<Option<String>, ()> {
    if ptr.is_null() {
        return Ok(None);
    }

    let text = unsafe { CStr::from_ptr(ptr) }.to_str().map_err(|_| ())?;
    Ok(Some(text.to_string()))
}

fn into_c_string(text: String) -> *mut c_char {
    let sanitized = text.replace('\0', "�");
    match CString::new(sanitized) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => CString::new("internal error while building CString")
            .expect("static fallback must be a valid CString")
            .into_raw(),
    }
}
