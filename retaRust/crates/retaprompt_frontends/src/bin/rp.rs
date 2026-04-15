#![allow(non_snake_case)]

fn main() {
    std::process::exit(
        retaprompt_input::run_kind_from_env(retaprompt_input::PromptInputFrontendKind::Rp),
    );
}
