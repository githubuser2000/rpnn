//! Dynamic split boundary for `libreta_render.so`.
//!
//! This crate now carries real rendering code.  In particular,
//! `rgrundStrukHtml` is built as a tiny C launcher that calls
//! `reta_render_grundstruk_html()` here instead of embedding the Reta Rust
//! core into its executable.

use std::ffi::CString;
use std::os::raw::c_char;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[path = "../../../src/prompt/semantic_choices.rs"]
pub mod semantic_choices;

#[path = "../../../src/shared/grundstruk_exact.rs"]
pub mod grundstruk_exact;

const ABI_VERSION: u32 = 1;
const LIBRARY_NAME: &str = "libreta_render.so\0";
const CRATE_NAME: &str = "reta_render\0";
const ROLE_DE: &str = "Rendering-Funktoren für Shell/Text, HTML, BBCode, Layout, Wrapping, Nummerierung und Styles.\0";
const ROLE_EN: &str = "Rendering functors for shell/text, HTML, BBCode, layout, wrapping, numbering, and styles.\0";
const MATH_DE: &str = "Funktor: semantische Tabellen und Auswahldaten werden in konkrete Darstellungsräume abgebildet.\0";
const MATH_EN: &str = "Functor: semantic tables and selection data are mapped into concrete representation spaces.\0";
const MANIFEST_JSON: &str = "{\"abi_version\":1,\"library\":\"libreta_render.so\",\"crate\":\"reta_render\",\"real_exports\":[\"reta_render_grundstruk_html\",\"reta_render_grundstruk_html_len\"],\"links_to\":[\"libreta_semantics.so\"]}\0";

#[cfg(reta_render_link_semantics)]
unsafe extern "C" {
    fn reta_semantics_abi_anchor() -> u64;
}

#[inline(never)]
#[cfg(reta_render_link_semantics)]
fn linked_semantics_anchor() -> u64 {
    unsafe { reta_semantics_abi_anchor() }
}

#[inline(never)]
#[cfg(not(reta_render_link_semantics))]
fn linked_semantics_anchor() -> u64 {
    0x5E4A_5E4A_0000_0000
}


#[unsafe(no_mangle)]
pub extern "C" fn reta_render_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_render_abi_anchor() -> u64 {
    0xC0DE_0001_0000_0005 ^ ((reta_render_grundstruk_html_len(0) as u64) << 1) ^ linked_semantics_anchor().rotate_left(7)
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

#[unsafe(no_mangle)]
pub extern "C" fn reta_render_grundstruk_html(blank: u8) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        grundstruk_exact::grundstruk_html_from_i18n(
            &grundstruk_exact::I18nLike::new(),
            blank != 0,
        )
    })) {
        Ok(html) => into_c_string(html),
        Err(_) => into_c_string("".to_string()),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_render_grundstruk_html_len(blank: u8) -> usize {
    catch_unwind(AssertUnwindSafe(|| {
        grundstruk_exact::grundstruk_html_from_i18n(
            &grundstruk_exact::I18nLike::new(),
            blank != 0,
        )
        .len()
    }))
    .unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_render_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

fn into_c_string(text: String) -> *mut c_char {
    let sanitized = text.replace('\0', "�");
    CString::new(sanitized)
        .unwrap_or_else(|_| CString::new("internal CString error").expect("static CString"))
        .into_raw()
}
