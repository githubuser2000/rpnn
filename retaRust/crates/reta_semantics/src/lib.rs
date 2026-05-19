//! Dynamic split boundary for `libreta_semantics.so`.
//!
//! This crate is intentionally a `cdylib`: it is part of the Reta runtime
//! shared-library topology and is loaded through `libreta.so`.  The exported
//! ABI symbols are small but stable.  They give the linker and package tests a
//! concrete boundary today, while Rust internals can be moved behind the same
//! boundary without changing launcher contracts.

use std::os::raw::c_char;

const ABI_VERSION: u32 = 1;
const LIBRARY_NAME: &str = "libreta_semantics.so\0";
const CRATE_NAME: &str = "reta_semantics\0";
const ROLE_DE: &str = "Semantische Auswahlgrenze für Spalten, Zeilenfilter, Generatoren, Zahlenlogik und Tags.\0";
const ROLE_EN: &str = "Semantic selection boundary for columns, row filters, generators, number logic, and tags.\0";
const MATH_DE: &str = "Topologische und prägarbenartige Verdichtung lokaler Parameterinformationen.\0";
const MATH_EN: &str = "Topological and presheaf-like condensation of local parameter information.\0";
const MANIFEST_JSON: &str = "{\"abi_version\":1,\"library\":\"libreta_semantics.so\",\"crate\":\"reta_semantics\",\"role_de\":\"Semantische Auswahlgrenze für Spalten, Zeilenfilter, Generatoren, Zahlenlogik und Tags.\",\"role_en\":\"Semantic selection boundary for columns, row filters, generators, number logic, and tags.\",\"math_de\":\"Topologische und prägarbenartige Verdichtung lokaler Parameterinformationen.\",\"math_en\":\"Topological and presheaf-like condensation of local parameter information.\"}\0";

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_semantics_abi_anchor() -> u64 {
    0x5E4A_0001_0000_0003
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

