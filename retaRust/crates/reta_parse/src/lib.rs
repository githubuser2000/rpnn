//! Dynamic split boundary for `libreta_parse.so`.
//!
//! This crate now carries actual parser code (`split_shell_like`) behind a C
//! ABI helper.  It remains a private component of the Reta split topology.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[path = "../../../src/prompt/tokenize.rs"]
pub mod tokenize;

const ABI_VERSION: u32 = 1;
const LIBRARY_NAME: &str = "libreta_parse.so\0";
const CRATE_NAME: &str = "reta_parse\0";
const ROLE_DE: &str = "Parsing- und Eingabeauflösungsgrenze für argv, Optionen, Aliase und Prompt-Tokens.\0";
const ROLE_EN: &str = "Parsing and input-resolution boundary for argv, options, aliases, and prompt tokens.\0";
const MATH_DE: &str = "Morphismenfamilie von Rohtext und argv nach kanonischen Reta-Anfragen.\0";
const MATH_EN: &str = "Morphism family from raw text and argv to canonical Reta requests.\0";
const MANIFEST_JSON: &str = "{\"abi_version\":1,\"library\":\"libreta_parse.so\",\"crate\":\"reta_parse\",\"real_exports\":[\"reta_parse_shell_tokens_json\",\"reta_parse_shell_token_count\"]}\0";

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_abi_anchor() -> u64 {
    {
        let demo = b"reta -zeilen --zeit=heute\0";
        0x0A25_0001_0000_0002 ^ ((reta_parse_shell_token_count(demo.as_ptr().cast()) as u64) << 3)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_abi_library_name() -> *const c_char {
    LIBRARY_NAME.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_abi_crate_name() -> *const c_char {
    CRATE_NAME.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_abi_role_de() -> *const c_char {
    ROLE_DE.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_abi_role_en() -> *const c_char {
    ROLE_EN.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_abi_math_de() -> *const c_char {
    MATH_DE.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_abi_math_en() -> *const c_char {
    MATH_EN.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_abi_manifest_json() -> *const c_char {
    MANIFEST_JSON.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_shell_token_count(line: *const c_char) -> usize {
    let input = unsafe { read_c_string(line) };
    tokenize::split_shell_like(&input)
        .map(|tokens| tokens.tokens.len())
        .unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_shell_tokens_json(line: *const c_char) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let input = unsafe { read_c_string(line) };
        match tokenize::split_shell_like(&input) {
            Ok(tokens) => {
                let mut json = String::from("{\"ok\":true,\"tokens\":[");
                for (index, token) in tokens.tokens.iter().enumerate() {
                    if index > 0 {
                        json.push(',');
                    }
                    json.push('"');
                    push_json_escaped(&mut json, token);
                    json.push('"');
                }
                json.push_str("]}");
                json
            }
            Err(error) => {
                let mut json = String::from("{\"ok\":false,\"error\":\"");
                push_json_escaped(&mut json, &error);
                json.push_str("\"}");
                json
            }
        }
    })) {
        Ok(json) => into_c_string(json),
        Err(_) => into_c_string("{\"ok\":false,\"error\":\"panic inside libreta_parse\"}".to_string()),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_parse_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

unsafe fn read_c_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned()
}

fn push_json_escaped(out: &mut String, text: &str) {
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
}

fn into_c_string(text: String) -> *mut c_char {
    let sanitized = text.replace('\0', "�");
    CString::new(sanitized)
        .unwrap_or_else(|_| CString::new("internal CString error").expect("static CString"))
        .into_raw()
}
