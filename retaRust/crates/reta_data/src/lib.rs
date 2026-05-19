//! Dynamic split boundary for `libreta_data.so`.
//!
//! This crate now carries real data-side Reta code, not only an ABI anchor.
//! It owns the generated `Words` inventory and Python-source-of-truth alias
//! projections behind C ABI symbols.  The public program still runs through
//! `libreta.so`; these exports are component-level surfaces used by build
//! checks, probes, and future runtime delegation.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[path = "../../../src/shared/words_py.rs"]
pub mod words_py;

pub mod shared {
    pub use crate::words_py;
}

#[path = "../../../src/domain/python_source_of_truth.rs"]
pub mod python_source_of_truth;

const ABI_VERSION: u32 = 1;
const LIBRARY_NAME: &str = "libreta_data.so\0";
const CRATE_NAME: &str = "reta_data\0";
const ROLE_DE: &str = "Daten-, Wörter-, Alias-, CSV- und Kataloggrenze für Reta.\0";
const ROLE_EN: &str = "Data, word, alias, CSV, and catalog boundary for Reta.\0";
const MATH_DE: &str = "Relationale Datenbasis; Objekte sind Wörter, Aliase, Zeilen, Spalten und Katalogeinträge.\0";
const MATH_EN: &str = "Relational data basis; objects are words, aliases, rows, columns, and catalog records.\0";
const MANIFEST_JSON: &str = "{\"abi_version\":1,\"library\":\"libreta_data.so\",\"crate\":\"reta_data\",\"role_de\":\"Daten-, Wörter-, Alias-, CSV- und Kataloggrenze für Reta.\",\"role_en\":\"Data, word, alias, CSV, and catalog boundary for Reta.\",\"real_exports\":[\"reta_data_shared_words_json\",\"reta_data_all_main_alias_groups_json\",\"reta_data_parameter_alias_groups_for_main_json\",\"reta_data_resolve_parameter_main_alias\"]}\0";

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_abi_anchor() -> u64 {
    0xDA7A_0001_0000_0001 ^ (reta_data_words_entry_count() as u64).rotate_left(7)
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_abi_library_name() -> *const c_char {
    LIBRARY_NAME.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_abi_crate_name() -> *const c_char {
    CRATE_NAME.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_abi_role_de() -> *const c_char {
    ROLE_DE.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_abi_role_en() -> *const c_char {
    ROLE_EN.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_abi_math_de() -> *const c_char {
    MATH_DE.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_abi_math_en() -> *const c_char {
    MATH_EN.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_abi_manifest_json() -> *const c_char {
    MANIFEST_JSON.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_words_entry_count() -> usize {
    words_py::Words::new().paraNdataMatrix.len()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_shared_words_json() -> *mut c_char {
    ffi_json_result(|| serde_json::to_string(&words_py::Words::new()))
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_all_main_alias_groups_json() -> *mut c_char {
    ffi_json_result(|| {
        let words = words_py::Words::new();
        serde_json::to_string(&python_source_of_truth::all_main_alias_groups(&words))
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_data_parameter_alias_groups_for_main_json(
    canonical_main: *const c_char,
) -> *mut c_char {
    ffi_json_result(|| {
        let main = unsafe { read_c_string(canonical_main) };
        let words = words_py::Words::new();
        serde_json::to_string(&python_source_of_truth::parameter_alias_groups_for_main(
            &words,
            &main,
        ))
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_data_resolve_parameter_main_alias(
    main_alias: *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let main = unsafe { read_c_string(main_alias) };
        let words = words_py::Words::new();
        python_source_of_truth::resolve_parameter_main_alias(&words, &main).unwrap_or_default()
    })) {
        Ok(value) => into_c_string(value),
        Err(_) => into_c_string(String::new()),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_data_free_string(ptr: *mut c_char) {
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

fn ffi_json_result<F>(build: F) -> *mut c_char
where
    F: FnOnce() -> serde_json::Result<String>,
{
    match catch_unwind(AssertUnwindSafe(build)) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside libreta_data JSON export"),
    }
}

fn json_error_string(message: &str) -> *mut c_char {
    into_c_string(format!(r#"{{"error":"{}"}}"#, message.replace('"', "'")))
}

fn into_c_string(text: String) -> *mut c_char {
    let sanitized = text.replace('\0', "�");
    CString::new(sanitized)
        .unwrap_or_else(|_| CString::new("internal CString error").expect("static CString"))
        .into_raw()
}
