#[path = "../abi_command_launcher.rs"]
mod abi_command_launcher;

fn main() {
    std::process::exit(abi_command_launcher::run_command_prompt(3));
}
