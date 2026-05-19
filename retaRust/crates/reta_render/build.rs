use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

fn target_profile_dir() -> PathBuf {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    for ancestor in out_dir.ancestors() {
        if ancestor.file_name() == Some(OsStr::new("build")) {
            if let Some(parent) = ancestor.parent() {
                return parent.to_path_buf();
            }
        }
    }

    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());
    if let Ok(target_dir) = env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(target_dir).join(profile);
    }
    PathBuf::from("target").join(profile)
}

fn main() {
    println!("cargo:rustc-check-cfg=cfg(reta_render_link_semantics)");
    println!("cargo:rerun-if-env-changed=RETA_RENDER_LINK_SEMANTICS");

    if env::var("RETA_RENDER_LINK_SEMANTICS").ok().as_deref() != Some("1") {
        return;
    }

    let target_dir = target_profile_dir();
    println!("cargo:rustc-cfg=reta_render_link_semantics");
    println!("cargo:rustc-link-search=native={}", target_dir.display());
    println!("cargo:rustc-cdylib-link-arg=-Wl,-rpath,$ORIGIN");
    println!("cargo:rustc-cdylib-link-arg=-Wl,-rpath,$ORIGIN/lib");
    println!("cargo:rustc-cdylib-link-arg=-Wl,-rpath,$ORIGIN/../lib");
    println!("cargo:rustc-link-lib=dylib=reta_semantics");
}
