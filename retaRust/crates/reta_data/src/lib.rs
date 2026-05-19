//! Dynamic split boundary for `libreta_data.so`.
//!
//! This crate is intentionally a `cdylib`: it is part of the Reta runtime
//! shared-library topology and is loaded through `libreta.so`.  The exported
//! ABI symbols are small but stable.  They give the linker and package tests a
//! concrete boundary today, while Rust internals can be moved behind the same
//! boundary without changing launcher contracts.

use std::os::raw::c_char;

const ABI_VERSION: u32 = 1;
const LIBRARY_NAME: &str = "libreta_data.so\0";
const CRATE_NAME: &str = "reta_data\0";
const ROLE_DE: &str = "Daten-, Wörter-, Alias-, CSV- und Kataloggrenze für Reta.\0";
const ROLE_EN: &str = "Data, word, alias, CSV, and catalog boundary for Reta.\0";
const MATH_DE: &str = "Relationale Datenbasis; Objekte sind Wörter, Aliase, Zeilen, Spalten und Katalogeinträge.\0";
const MATH_EN: &str = "Relational data basis; objects are words, aliases, rows, columns, and catalog records.\0";
const MANIFEST_JSON: &str = "{\"abi_version\":1,\"library\":\"libreta_data.so\",\"crate\":\"reta_data\",\"role_de\":\"Daten-, Wörter-, Alias-, CSV- und Kataloggrenze für Reta.\",\"role_en\":\"Data, word, alias, CSV, and catalog boundary for Reta.\",\"math_de\":\"Relationale Datenbasis; Objekte sind Wörter, Aliase, Zeilen, Spalten und Katalogeinträge.\",\"math_en\":\"Relational data basis; objects are words, aliases, rows, columns, and catalog records.\"}\0";

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_data_abi_anchor() -> u64 {
    0xDA7A_0001_0000_0001
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

