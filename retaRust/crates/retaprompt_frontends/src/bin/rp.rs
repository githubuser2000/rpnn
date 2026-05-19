#[path = "../abi_input_launcher.rs"]
mod abi_input_launcher;

fn main() {
    std::process::exit(abi_input_launcher::run_input_prompt(1));
}
