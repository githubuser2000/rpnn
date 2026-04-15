#![allow(non_snake_case)]

pub fn run_rp(argv: Vec<String>) -> i32 {
    retaprompt::run_rp(argv)
}

pub fn run_rp_from_env() -> i32 {
    retaprompt::run_rp_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn rp_run_from_env() -> i32 {
    run_rp_from_env()
}
