use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use serde_json;

use crate::{build_cli_request, run_reta, RetaRuntime};

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

    let request = build_cli_request(
        &args,
        stdin_text,
        RetaRuntime {
            terminal_width: if terminal_width == 0 { None } else { Some(terminal_width) },
            stdout_is_tty: Some(stdout_is_tty != 0),
            stderr_is_tty: Some(stderr_is_tty != 0),
            stdin_is_tty: Some(stdin_is_tty != 0),
        },
    );

    match run_reta(request) {
        Ok(response) => RetaFfiResponse {
            stdout_text: into_c_string(response.rendered_text),
            stderr_text: into_c_string(response.stderr_text),
            exit_code: response.exit_code,
        },
        Err(error) => RetaFfiResponse {
            stdout_text: into_c_string(String::new()),
            stderr_text: into_c_string(format!("reta failed: {error}\n")),
            exit_code: error.exit_code(),
        },
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

#[unsafe(no_mangle)]
pub extern "C" fn reta_shared_words_json() -> *mut c_char {
    match serde_json::to_string(crate::shared_words()) {
        Ok(json) => into_c_string(json),
        Err(error) => into_c_string(format!(
            r#"{{"error":"{}"}}"#,
            error.to_string().replace('"', "'")
        )),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_all_main_alias_groups_json() -> *mut c_char {
    match serde_json::to_string(&crate::domain::python_source_of_truth::all_main_alias_groups(
        crate::shared_words(),
    )) {
        Ok(json) => into_c_string(json),
        Err(error) => into_c_string(format!(
            r#"{{"error":"{}"}}"#,
            error.to_string().replace('"', "'")
        )),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_parameter_alias_groups_for_main_json(
    canonical_main: *const c_char,
) -> *mut c_char {
    let main = read_required_string(canonical_main).unwrap_or_default();
    match serde_json::to_string(&crate::domain::python_source_of_truth::parameter_alias_groups_for_main(
        crate::shared_words(),
        &main,
    )) {
        Ok(json) => into_c_string(json),
        Err(error) => into_c_string(format!(
            r#"{{"error":"{}"}}"#,
            error.to_string().replace('"', "'")
        )),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_resolve_parameter_main_alias(
    main_alias: *const c_char,
) -> *mut c_char {
    let main = read_required_string(main_alias).unwrap_or_default();
    match crate::domain::python_source_of_truth::resolve_parameter_main_alias(
        crate::shared_words(),
        &main,
    ) {
        Some(canonical) => into_c_string(canonical),
        None => into_c_string(String::new()),
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
