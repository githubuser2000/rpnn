//! Dynamic split boundary for `libreta_arch.so`.
//!
//! This crate is intentionally a `cdylib`: it is part of the Reta runtime
//! shared-library topology and is loaded through `libreta.so`.  The exported
//! ABI symbols are small but stable.  They give the linker and package tests a
//! concrete boundary today, while Rust internals can be moved behind the same
//! boundary without changing launcher contracts.

use std::os::raw::c_char;

const ABI_VERSION: u32 = 1;
const LIBRARY_NAME: &str = "libreta_arch.so\0";
const CRATE_NAME: &str = "reta_arch\0";
const ROLE_DE: &str = "Architektur-, Kategorie-, Topologie-, Morphismus- und universelle-Eigenschaftsgrenze.\0";
const ROLE_EN: &str = "Architecture, category, topology, morphism, and universal-property boundary.\0";
const MATH_DE: &str = "Kategorie- und Funktor-Metadaten; natürliche Transformationen für Parität und Shadow-Pfade.\0";
const MATH_EN: &str = "Category and functor metadata; natural transformations for parity and shadow paths.\0";
const MANIFEST_JSON: &str = "{\"abi_version\":1,\"library\":\"libreta_arch.so\",\"crate\":\"reta_arch\",\"role_de\":\"Architektur-, Kategorie-, Topologie-, Morphismus- und universelle-Eigenschaftsgrenze.\",\"role_en\":\"Architecture, category, topology, morphism, and universal-property boundary.\",\"math_de\":\"Kategorie- und Funktor-Metadaten; natürliche Transformationen für Parität und Shadow-Pfade.\",\"math_en\":\"Category and functor metadata; natural transformations for parity and shadow paths.\"}\0";

#[unsafe(no_mangle)]
pub extern "C" fn reta_arch_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_arch_abi_anchor() -> u64 {
    0xA2C4_0001_0000_0006 ^ (reta_arch_architecture_morphism_count() as u64) ^ ((reta_arch_architecture_csv_asset_count() as u64) << 8)
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_arch_abi_library_name() -> *const c_char {
    LIBRARY_NAME.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_arch_abi_crate_name() -> *const c_char {
    CRATE_NAME.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_arch_abi_role_de() -> *const c_char {
    ROLE_DE.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_arch_abi_role_en() -> *const c_char {
    ROLE_EN.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_arch_abi_math_de() -> *const c_char {
    MATH_DE.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_arch_abi_math_en() -> *const c_char {
    MATH_EN.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_arch_abi_manifest_json() -> *const c_char {
    MANIFEST_JSON.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_arch_architecture_morphism_count() -> usize {
    reta_architecture::bootstrap_architecture_runtime().morphisms.edges.len()
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_arch_architecture_csv_asset_count() -> usize {
    reta_architecture::csv_asset_count()
}

