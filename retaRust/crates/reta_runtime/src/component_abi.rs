//! Private link anchors from `libreta_runtime.so` to the real split core
//! component libraries.
//!
//! `libreta_runtime.so` is still the heavy engine carrier, but it now records
//! actual DT_NEEDED edges to the data/parse/semantics/table/render/arch
//! components.  That keeps the topology honest while code is moved out in
//! stages.

#[cfg(reta_runtime_link_core_components)]
unsafe extern "C" {
    fn reta_data_abi_anchor() -> u64;
    fn reta_parse_abi_anchor() -> u64;
    fn reta_semantics_abi_anchor() -> u64;
    fn reta_table_abi_anchor() -> u64;
    fn reta_render_abi_anchor() -> u64;
    fn reta_arch_abi_anchor() -> u64;
}

#[inline(never)]
#[cfg(reta_runtime_link_core_components)]
fn linked_component_anchor() -> u64 {
    unsafe {
        reta_data_abi_anchor()
            ^ reta_parse_abi_anchor().rotate_left(1)
            ^ reta_semantics_abi_anchor().rotate_left(2)
            ^ reta_table_abi_anchor().rotate_left(3)
            ^ reta_render_abi_anchor().rotate_left(4)
            ^ reta_arch_abi_anchor().rotate_left(5)
    }
}

#[inline(never)]
#[cfg(not(reta_runtime_link_core_components))]
fn linked_component_anchor() -> u64 {
    0xC0FE_FACA_DE00_0001
}

pub fn preload_reta_runtime_component_libraries() -> u64 {
    linked_component_anchor()
}
