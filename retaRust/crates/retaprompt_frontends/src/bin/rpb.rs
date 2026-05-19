#![allow(non_snake_case)]

#[path = "../abi_launcher.rs"]
mod abi_launcher;

#[link(name = "retaprompt_commands")]
unsafe extern "C" {
    fn retaprompt_commands_run_rpb_from_env() -> std::os::raw::c_int;
}

fn main() {
    unsafe {
        abi_launcher::run_and_exit(retaprompt_commands_run_rpb_from_env);
    }
}
