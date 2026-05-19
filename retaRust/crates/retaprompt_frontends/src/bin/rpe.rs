#![allow(non_snake_case)]

#[path = "../abi_launcher.rs"]
mod abi_launcher;

#[link(name = "retaprompt_input")]
unsafe extern "C" {
    fn retaprompt_input_run_rpe_from_env() -> std::os::raw::c_int;
}

#[link(name = "retaprompt_commands")]
unsafe extern "C" {
    fn retaprompt_commands_run_rpe_from_env() -> std::os::raw::c_int;
}

fn main() {
    unsafe {
        abi_launcher::retain_direct_dependency(retaprompt_commands_run_rpe_from_env);
        abi_launcher::run_and_exit(retaprompt_input_run_rpe_from_env);
    }
}
