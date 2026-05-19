fn main() {
    println!("cargo:rustc-check-cfg=cfg(reta_link_core_split_libs)");
    println!("cargo:rustc-check-cfg=cfg(reta_runtime_core_carrier)");
    println!("cargo:rustc-cfg=reta_runtime_core_carrier");
}
