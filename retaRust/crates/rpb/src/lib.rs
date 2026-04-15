#![allow(non_snake_case)]

pub fn run_rpb(argv: Vec<String>) -> i32 {
    retaprompt::run_rpb(argv)
}

pub fn run_rpb_from_env() -> i32 {
    retaprompt::run_rpb_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn rpb_run_from_env() -> i32 {
    run_rpb_from_env()
}
