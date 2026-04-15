#![allow(non_snake_case)]

pub fn run_rpl(argv: Vec<String>) -> i32 {
    retaprompt::run_rpl(argv)
}

pub fn run_rpl_from_env() -> i32 {
    retaprompt::run_rpl_from_env()
}

#[unsafe(no_mangle)]
pub extern "C" fn rpl_run_from_env() -> i32 {
    run_rpl_from_env()
}
