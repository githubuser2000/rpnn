#![allow(non_snake_case)]

use std::os::raw::c_int;

pub type RetapromptEntrypoint = unsafe extern "C" fn() -> c_int;

#[inline(never)]
pub unsafe fn retain_direct_dependency(anchor: RetapromptEntrypoint) {
    // Force a relocation against the anchor symbol so linkers do not drop the
    // direct DT_NEEDED edge.  This is deliberately tiny launcher code; no
    // retaprompt Rust module may be called from the executable.
    let raw = anchor as usize;
    unsafe {
        std::ptr::read_volatile(&raw);
    }
}

#[inline(never)]
pub unsafe fn run_and_exit(entrypoint: RetapromptEntrypoint) -> ! {
    let code = unsafe { entrypoint() };
    std::process::exit(code);
}
