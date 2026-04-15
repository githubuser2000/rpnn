#![allow(non_snake_case)]

fn main() {
    std::process::exit(
        retaprompt_commands::run_kind_from_env(
            retaprompt_commands::PromptCommandFrontendKind::Rpb,
        ),
    );
}
