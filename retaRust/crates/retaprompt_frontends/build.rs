use std::env;
use std::path::{Path, PathBuf};

fn profile_target_dir_from_out_dir(out_dir: &Path) -> Option<PathBuf> {
    // OUT_DIR has the form: target/<profile>/build/<pkg-hash>/out
    // Climb: out -> <pkg-hash> -> build -> <profile>
    let hash_dir = out_dir.parent()?;
    let build_dir = hash_dir.parent()?;
    build_dir.parent().map(Path::to_path_buf)
}

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    if let Some(profile_dir) = profile_target_dir_from_out_dir(&out_dir) {
        println!("cargo:rustc-link-search=native={}", profile_dir.display());
    }

    // These Cargo binaries are intentionally thin ABI launchers.  The real
    // prompt code must live in libretaprompt_input.so and/or
    // libretaprompt_commands.so.  Keep the same runtime library search layout
    // as the C launchers built by build.sh.
    for bin in ["rrp", "rrpl", "rrpe", "rrpb"] {
        println!("cargo:rustc-link-arg-bin={bin}=-Wl,-rpath,$ORIGIN");
        println!("cargo:rustc-link-arg-bin={bin}=-Wl,-rpath,$ORIGIN/lib");
        println!("cargo:rustc-link-arg-bin={bin}=-Wl,-rpath,$ORIGIN/../lib");
    }
}
