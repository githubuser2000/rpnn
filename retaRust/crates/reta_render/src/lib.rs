//! Dynamic split boundary for `libreta_render.so`.
//!
//! This crate is intentionally a `cdylib`: it is part of the Reta runtime
//! shared-library topology and is loaded through `libreta.so`.  The exported
//! ABI symbols are small but stable.  They give the linker and package tests a
//! concrete boundary today, while Rust internals can be moved behind the same
//! boundary without changing launcher contracts.

use std::os::raw::c_char;

const ABI_VERSION: u32 = 1;
const LIBRARY_NAME: &str = "libreta_render.so\0";
const CRATE_NAME: &str = "reta_render\0";
const ROLE_DE: &str = "Ausgabegrenze für Shell/Text, HTML, BBCode, Layout, Wrapping und Nummerierung.\0";
const ROLE_EN: &str = "Output boundary for shell/text, HTML, BBCode, layout, wrapping, and numbering.\0";
const MATH_DE: &str = "Funktor von semantischen Tabellen in konkrete Darstellungsräume.\0";
const MATH_EN: &str = "Functor from semantic tables into concrete representation spaces.\0";
const MANIFEST_JSON: &str = "{\"abi_version\":1,\"library\":\"libreta_render.so\",\"crate\":\"reta_render\",\"role_de\":\"Ausgabegrenze für Shell/Text, HTML, BBCode, Layout, Wrapping und Nummerierung.\",\"role_en\":\"Output boundary for shell/text, HTML, BBCode, layout, wrapping, and numbering.\",\"math_de\":\"Funktor von semantischen Tabellen in konkrete Darstellungsräume.\",\"math_en\":\"Functor from semantic tables into concrete representation spaces.\"}\0";

#[unsafe(no_mangle)]
pub extern "C" fn reta_render_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_render_abi_anchor() -> u64 {
    0x2E0D_0001_0000_0005
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_render_abi_library_name() -> *const c_char {
    LIBRARY_NAME.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_render_abi_crate_name() -> *const c_char {
    CRATE_NAME.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_render_abi_role_de() -> *const c_char {
    ROLE_DE.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_render_abi_role_en() -> *const c_char {
    ROLE_EN.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_render_abi_math_de() -> *const c_char {
    MATH_DE.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_render_abi_math_en() -> *const c_char {
    MATH_EN.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_render_abi_manifest_json() -> *const c_char {
    MANIFEST_JSON.as_ptr().cast()
}

