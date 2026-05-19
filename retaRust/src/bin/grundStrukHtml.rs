#![allow(non_snake_case)]

use std::env;
use std::ffi::CStr;
use std::io::{self, Write};
use std::os::raw::c_char;
use std::path::PathBuf;

use libloading::{library_filename, Library, Symbol};

type RetaRenderAbiVersionFn = unsafe extern "C" fn() -> u32;
type RetaRenderGrundstrukHtmlFn = unsafe extern "C" fn(blank: u8) -> *mut c_char;
type RetaRenderFreeStringFn = unsafe extern "C" fn(ptr: *mut c_char);

const EXPECTED_RETA_RENDER_ABI_VERSION: u32 = 1;
const HELP: &str = "Usage: rgrundStrukHtml [blank]\n\nGenerates the Reta Grundstruktur HTML document.\n\nArguments:\n  blank        generate the blank/table-template variant\n  -h, --help   show this help\n\nThe launcher loads libreta_render.so dynamically. Set RETA_RENDER_LIB_PATH\nto override the shared-library path.\n";

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let blank = match parse_args(&args) {
        Ok(ParsedArgs::Help) => {
            print!("{HELP}");
            return 0;
        }
        Ok(ParsedArgs::Run { blank }) => blank,
        Err(message) => {
            let _ = writeln!(io::stderr(), "rgrundStrukHtml: {message}\n\n{HELP}");
            return 2;
        }
    };

    let library = match load_render_library() {
        Ok(library) => library,
        Err(message) => {
            let _ = writeln!(io::stderr(), "rgrundStrukHtml launcher failed: {message}");
            return 127;
        }
    };

    unsafe {
        let render: Symbol<'_, RetaRenderGrundstrukHtmlFn> = match library.get(b"reta_render_grundstruk_html") {
            Ok(symbol) => symbol,
            Err(error) => {
                let _ = writeln!(
                    io::stderr(),
                    "rgrundStrukHtml launcher failed: missing symbol reta_render_grundstruk_html: {error}"
                );
                return 127;
            }
        };
        let free: Symbol<'_, RetaRenderFreeStringFn> = match library.get(b"reta_render_free_string") {
            Ok(symbol) => symbol,
            Err(error) => {
                let _ = writeln!(
                    io::stderr(),
                    "rgrundStrukHtml launcher failed: missing symbol reta_render_free_string: {error}"
                );
                return 127;
            }
        };

        let ptr = render(blank as u8);
        if ptr.is_null() {
            let _ = writeln!(io::stderr(), "rgrundStrukHtml: libreta_render.so returned null");
            return 1;
        }

        let text = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        free(ptr);
        print!("{text}");
        0
    }
}

enum ParsedArgs {
    Help,
    Run { blank: bool },
}

fn parse_args(args: &[String]) -> Result<ParsedArgs, String> {
    match args {
        [] => Ok(ParsedArgs::Run { blank: false }),
        [arg] if arg == "-h" || arg == "--help" => Ok(ParsedArgs::Help),
        [arg] if arg == "blank" => Ok(ParsedArgs::Run { blank: true }),
        [arg] => Err(format!("unknown argument: {arg}")),
        _ => Err(format!("too many arguments: {}", args.join(" "))),
    }
}

fn load_render_library() -> Result<Library, String> {
    let mut errors = Vec::new();

    for candidate in render_library_candidates() {
        let display = candidate.display().to_string();
        match unsafe { Library::new(&candidate) } {
            Ok(library) => match validate_render_library_abi(&library) {
                Ok(()) => return Ok(library),
                Err(error) => errors.push(format!("{display}: {error}")),
            },
            Err(error) => errors.push(format!("{display}: {error}")),
        }
    }

    let fallback_name = library_filename("reta_render");
    let fallback_display = fallback_name.to_string_lossy().into_owned();
    match unsafe { Library::new(&fallback_name) } {
        Ok(library) => match validate_render_library_abi(&library) {
            Ok(()) => Ok(library),
            Err(error) => {
                errors.push(format!("{fallback_display}: {error}"));
                Err(format!(
                    "could not load compatible libreta_render shared library; tried {}",
                    errors.join(" | ")
                ))
            }
        },
        Err(error) => {
            errors.push(format!("{fallback_display}: {error}"));
            Err(format!(
                "could not load libreta_render shared library; tried {}",
                errors.join(" | ")
            ))
        }
    }
}

fn validate_render_library_abi(library: &Library) -> Result<(), String> {
    let abi_version = unsafe {
        library
            .get::<RetaRenderAbiVersionFn>(b"reta_render_abi_version")
            .map_err(|error| format!("missing symbol reta_render_abi_version: {error}"))?
    };
    let actual = unsafe { abi_version() };
    if actual == EXPECTED_RETA_RENDER_ABI_VERSION {
        Ok(())
    } else {
        Err(format!(
            "incompatible libreta_render ABI {actual}, expected {EXPECTED_RETA_RENDER_ABI_VERSION}"
        ))
    }
}

fn render_library_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(path) = env::var("RETA_RENDER_LIB_PATH") {
        let path = path.trim();
        if !path.is_empty() {
            candidates.push(PathBuf::from(path));
        }
    }

    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            let filename = PathBuf::from(library_filename("reta_render"));
            candidates.push(dir.join(&filename));
            candidates.push(dir.join("deps").join(&filename));
            candidates.push(dir.join("lib").join(&filename));
            candidates.push(dir.join("..").join("lib").join(&filename));
            candidates.push(dir.join("..").join("deps").join(&filename));
        }
    }

    dedup_paths(candidates)
}

fn dedup_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut deduped = Vec::new();
    for path in paths {
        if !deduped.iter().any(|existing| existing == &path) {
            deduped.push(path);
        }
    }
    deduped
}
