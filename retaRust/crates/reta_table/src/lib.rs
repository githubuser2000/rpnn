//! Dynamic split boundary for `libreta_table.so`.
//!
//! This crate now carries concrete table utility logic behind ABI helpers.
//! The heavy full table materialization can be moved here incrementally; the
//! current real exports already prevent this library from being a pure stub.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[path = "../../../src/table_printer/table_utils.rs"]
pub mod table_utils;

const ABI_VERSION: u32 = 1;
const LIBRARY_NAME: &str = "libreta_table.so\0";
const CRATE_NAME: &str = "reta_table\0";
const ROLE_DE: &str = "Tabellen-Materialisierung, Tabellenzustand, View-Aufbau und Verklebung lokaler Sektionen.\0";
const ROLE_EN: &str = "Table materialization, table state, view construction, and gluing of local sections.\0";
const MATH_DE: &str = "Garbe: lokale Spalten-/Zeilen-/Parameter-Sektionen werden zu globalen Tabellen verklebt.\0";
const MATH_EN: &str = "Sheaf: local column, row, and parameter sections are glued into global tables.\0";
const MANIFEST_JSON: &str = "{\"abi_version\":1,\"library\":\"libreta_table.so\",\"crate\":\"reta_table\",\"real_exports\":[\"reta_table_natural_widths_json\",\"reta_table_shrink_widths_json\"]}\0";

#[unsafe(no_mangle)]
pub extern "C" fn reta_table_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_table_abi_anchor() -> u64 {
    let demo = vec![vec!["abc".to_string(), "x".to_string()], vec!["abcdef".to_string()]];
    let width_sum: usize = table_utils::natural_column_widths(&demo).iter().sum();
    0x7A81_E000_0000_0004 ^ ((width_sum as u64) << 5)
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_table_abi_library_name() -> *const c_char {
    LIBRARY_NAME.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_table_abi_crate_name() -> *const c_char {
    CRATE_NAME.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_table_abi_role_de() -> *const c_char {
    ROLE_DE.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_table_abi_role_en() -> *const c_char {
    ROLE_EN.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_table_abi_math_de() -> *const c_char {
    MATH_DE.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_table_abi_math_en() -> *const c_char {
    MATH_EN.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_table_abi_manifest_json() -> *const c_char {
    MANIFEST_JSON.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_table_natural_widths_json(tsv_or_lines: *const c_char) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let input = unsafe { read_c_string(tsv_or_lines) };
        let rows = parse_rows(&input);
        widths_json(&table_utils::natural_column_widths(&rows))
    })) {
        Ok(json) => into_c_string(json),
        Err(_) => into_c_string("{\"error\":\"panic inside libreta_table\"}".to_string()),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_table_shrink_widths_json(tsv_or_lines: *const c_char, budget: usize) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let input = unsafe { read_c_string(tsv_or_lines) };
        let rows = parse_rows(&input);
        let widths = table_utils::natural_column_widths(&rows);
        widths_json(&table_utils::shrink_widths_to_budget(&widths, budget))
    })) {
        Ok(json) => into_c_string(json),
        Err(_) => into_c_string("{\"error\":\"panic inside libreta_table\"}".to_string()),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_table_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

fn parse_rows(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| line.split('\t').map(ToOwned::to_owned).collect::<Vec<_>>())
        .collect()
}

fn widths_json(widths: &[usize]) -> String {
    let body = widths
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"widths\":[{body}]}}")
}

unsafe fn read_c_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned()
}

fn into_c_string(text: String) -> *mut c_char {
    let sanitized = text.replace('\0', "�");
    CString::new(sanitized)
        .unwrap_or_else(|_| CString::new("internal CString error").expect("static CString"))
        .into_raw()
}
