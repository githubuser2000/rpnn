//! Dynamic split boundary for `libreta_semantics.so`.
//!
//! This crate now carries concrete semantic inventories used by retaPrompt and
//! Reta section/value selection.  The exported symbols keep Rust types inside
//! the component and expose only a stable C ABI.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[path = "../../../src/prompt/semantic_choices.rs"]
pub mod semantic_choices;

const ABI_VERSION: u32 = 1;
const LIBRARY_NAME: &str = "libreta_semantics.so\0";
const CRATE_NAME: &str = "reta_semantics\0";
const ROLE_DE: &str = "Semantische Auswahlgrenze für Spalten, Zeilenfilter, Generatoren, Zahlenlogik und Tags.\0";
const ROLE_EN: &str = "Semantic selection boundary for columns, row filters, generators, number logic, and tags.\0";
const MATH_DE: &str = "Topologische und prägarbenartige Verdichtung lokaler Parameterinformationen.\0";
const MATH_EN: &str = "Topological and presheaf-like condensation of local parameter information.\0";
const MANIFEST_JSON: &str = "{\"abi_version\":1,\"library\":\"libreta_semantics.so\",\"crate\":\"reta_semantics\",\"real_exports\":[\"reta_semantics_choice_counts_json\",\"reta_semantics_wahl15_value\",\"reta_semantics_wahl16_value\"]}\0";

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_abi_anchor() -> u64 {
    0x5E4A_0001_0000_0003
        ^ ((semantic_choices::WAHL15_I18N_ENTRIES.len() as u64) << 11)
        ^ ((semantic_choices::WAHL16_I18N_ENTRIES.len() as u64) << 19)
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_abi_library_name() -> *const c_char {
    LIBRARY_NAME.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_abi_crate_name() -> *const c_char {
    CRATE_NAME.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_abi_role_de() -> *const c_char {
    ROLE_DE.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_abi_role_en() -> *const c_char {
    ROLE_EN.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_abi_math_de() -> *const c_char {
    MATH_DE.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_abi_math_en() -> *const c_char {
    MATH_EN.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_abi_manifest_json() -> *const c_char {
    MANIFEST_JSON.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_choice_counts_json() -> *mut c_char {
    let json = format!(
        "{{\"wahl15\":{},\"wahl16\":{},\"retaprompt_wahl15\":{},\"retaprompt_wahl16\":{},\"main_switches\":{},\"section_switches\":{}}}",
        semantic_choices::WAHL15_I18N_ENTRIES.len(),
        semantic_choices::WAHL16_I18N_ENTRIES.len(),
        semantic_choices::RETAPROMPT_WAHL15_ENTRIES.len(),
        semantic_choices::RETAPROMPT_WAHL16_ENTRIES.len(),
        semantic_choices::RETAPROMPT_RETA_MAIN_SWITCHES.len(),
        semantic_choices::RETAPROMPT_RETA_SECTION_SWITCHES.len(),
    );
    into_c_string(json)
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_wahl15_value(key: *const c_char) -> *mut c_char {
    choice_value_json(key, semantic_choices::semantic_wahl15_value)
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_wahl16_value(key: *const c_char) -> *mut c_char {
    choice_value_json(key, semantic_choices::semantic_wahl16_value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_semantics_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

fn choice_value_json(
    key: *const c_char,
    lookup: fn(&str) -> Option<&'static str>,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let key = unsafe { read_c_string(key) };
        let mut json = String::from("{\"key\":\"");
        push_json_escaped(&mut json, &key);
        json.push_str("\",\"value\":");
        if let Some(value) = lookup(&key) {
            json.push('"');
            push_json_escaped(&mut json, value);
            json.push('"');
        } else {
            json.push_str("null");
        }
        json.push('}');
        json
    })) {
        Ok(json) => into_c_string(json),
        Err(_) => into_c_string("{\"error\":\"panic inside libreta_semantics\"}".to_string()),
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
