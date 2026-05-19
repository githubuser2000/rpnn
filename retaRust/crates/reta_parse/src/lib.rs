//! Dynamic split boundary for `libreta_parse.so`.
//!
//! This crate is intentionally a `cdylib`: it is part of the Reta runtime
//! shared-library topology and is loaded through `libreta.so`.  The exported
//! ABI symbols are small but stable.  They give the linker and package tests a
//! concrete boundary today, while Rust internals can be moved behind the same
//! boundary without changing launcher contracts.

use std::os::raw::c_char;

const ABI_VERSION: u32 = 1;
const LIBRARY_NAME: &str = "libreta_parse.so\0";
const CRATE_NAME: &str = "reta_parse\0";
const ROLE_DE: &str = "Parsing- und Eingabeauflösungsgrenze für argv, Optionen, Aliase und Prompt-Tokens.\0";
const ROLE_EN: &str = "Parsing and input-resolution boundary for argv, options, aliases, and prompt tokens.\0";
const MATH_DE: &str = "Morphismenfamilie von Rohtext und argv nach kanonischen Reta-Anfragen.\0";
const MATH_EN: &str = "Morphism family from raw text and argv to canonical Reta requests.\0";
const MANIFEST_JSON: &str = "{\"abi_version\":1,\"library\":\"libreta_parse.so\",\"crate\":\"reta_parse\",\"role_de\":\"Parsing- und Eingabeauflösungsgrenze für argv, Optionen, Aliase und Prompt-Tokens.\",\"role_en\":\"Parsing and input-resolution boundary for argv, options, aliases, and prompt tokens.\",\"math_de\":\"Morphismenfamilie von Rohtext und argv nach kanonischen Reta-Anfragen.\",\"math_en\":\"Morphism family from raw text and argv to canonical Reta requests.\"}\0";

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parse_abi_anchor() -> u64 {
    0x0A25_0001_0000_0002
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

