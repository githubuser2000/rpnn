use std::path::PathBuf;

use super::app::{run_rp, run_rp_one_shot};

pub fn run_prompt_frontend_from_env() -> i32 {
    let argv = std::env::args().collect::<Vec<_>>();
    run_prompt_frontend(argv)
}

pub fn run_prompt_frontend(argv: Vec<String>) -> i32 {
    let program_name = PathBuf::from(
        argv.first().cloned().unwrap_or_else(|| "rp".to_string()),
    )
    .file_name()
    .map(|s| s.to_string_lossy().to_string())
    .unwrap_or_else(|| "rp".to_string());

    match program_name.as_str() {
        "rpb" => run_rp_one_shot(argv, true),
        "rpe" => run_rp(argv, false),
        "rpl" => run_rp(argv, true),
        _ => run_rp(argv, true),
    }
}
