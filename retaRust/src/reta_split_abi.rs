//! Linker-visible ABI boundary between `libreta.so` and the split Reta core `.so` files.
//!
//! When `build.sh` sets `RETA_LINK_CORE_SPLIT_LIBS=1`, `build.rs` links
//! `libreta.so` against the seven private core libraries.  This module calls
//! one stable anchor symbol from each private library.  That gives `libreta.so`
//! concrete `DT_NEEDED` entries and keeps the public executable contract small:
//! `rreta -> libreta.so -> libreta_{data,parse,semantics,table,render,arch,runtime}.so`.

use std::os::raw::c_char;

const ABI_VERSION: u32 = 1;
const MANIFEST_JSON: &[u8] = b"{\"abi_version\":1,\"facade\":\"libreta.so\",\"private_core_libraries\":[\"libreta_data.so\",\"libreta_parse.so\",\"libreta_semantics.so\",\"libreta_table.so\",\"libreta_render.so\",\"libreta_arch.so\",\"libreta_runtime.so\"],\"dependency_rule\":\"rreta links to libreta.so; libreta.so links to private core split libraries\"}\0";
#[cfg(reta_link_core_split_libs)]
unsafe extern "C" {
    fn reta_data_abi_anchor() -> u64;
    fn reta_parse_abi_anchor() -> u64;
    fn reta_semantics_abi_anchor() -> u64;
    fn reta_table_abi_anchor() -> u64;
    fn reta_render_abi_anchor() -> u64;
    fn reta_arch_abi_anchor() -> u64;
    fn reta_runtime_abi_anchor() -> u64;
}

#[inline(never)]
#[cfg(reta_link_core_split_libs)]
fn linked_core_anchor() -> u64 {
    unsafe {
        reta_data_abi_anchor()
            ^ reta_parse_abi_anchor().rotate_left(1)
            ^ reta_semantics_abi_anchor().rotate_left(2)
            ^ reta_table_abi_anchor().rotate_left(3)
            ^ reta_render_abi_anchor().rotate_left(4)
            ^ reta_arch_abi_anchor().rotate_left(5)
            ^ reta_runtime_abi_anchor().rotate_left(6)
    }
}

#[inline(never)]
#[cfg(not(reta_link_core_split_libs))]
fn linked_core_anchor() -> u64 {
    0xDEAD_0F0F_0000_0001
}

pub fn preload_reta_split_shared_libraries() -> u64 {
    linked_core_anchor()
}

#[cfg_attr(not(reta_runtime_core_carrier), unsafe(no_mangle))]
pub extern "C" fn reta_core_split_abi_version() -> u32 {
    ABI_VERSION
}

#[cfg_attr(not(reta_runtime_core_carrier), unsafe(no_mangle))]
pub extern "C" fn reta_core_split_abi_anchor() -> u64 {
    preload_reta_split_shared_libraries()
}

#[cfg_attr(not(reta_runtime_core_carrier), unsafe(no_mangle))]
pub extern "C" fn reta_core_split_abi_manifest_json() -> *const c_char {
    MANIFEST_JSON.as_ptr().cast()
}

#[cfg_attr(not(reta_runtime_core_carrier), unsafe(no_mangle))]
pub extern "C" fn reta_core_split_abi_is_linked() -> u8 {
    if cfg!(reta_link_core_split_libs) { 1 } else { 0 }
}
