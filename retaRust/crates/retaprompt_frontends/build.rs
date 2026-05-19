fn main() {
    // Public Cargo prompt targets are runtime dlopen launchers.  Do not emit
    // native link instructions here: direct #[link] / DT_NEEDED Cargo frontends
    // are fragile on Android/Termux because Cargo does not reliably control the
    // dynamic-loader search path for sibling cdylibs.  The final packaged
    // executables are still C launchers with explicit DT_NEEDED edges, built and
    // guarded by build.sh.
    println!("cargo:rerun-if-changed=src/abi_launcher.rs");
    println!("cargo:rerun-if-env-changed=RETAPROMPT_INPUT_LIB_PATH");
    println!("cargo:rerun-if-env-changed=RETAPROMPT_COMMANDS_LIB_PATH");
}
