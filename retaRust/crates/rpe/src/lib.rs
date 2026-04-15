#![allow(non_snake_case)]

pub fn run_rpe(argv: Vec<String>) -> i32 {
    retaprompt::run_rpe(argv)
}

pub fn run_rpe_from_env() -> i32 {
    retaprompt::run_rpe_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn rpe_run_from_env() -> i32 {
    run_rpe_from_env()
}
