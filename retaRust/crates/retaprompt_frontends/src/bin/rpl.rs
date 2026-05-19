#[path = "../abi_launcher.rs"]
mod abi_launcher;

fn main() {
    std::process::exit(abi_launcher::run_input_prompt(2));
}
