use std::os::raw::c_int;

#[path = "abi_common.rs"]
mod abi_common;

use self::abi_common::{fail_runtime, LoadedPromptLibrary, PromptLibrarySpec};

const COMMANDS_LIBRARY: PromptLibrarySpec = PromptLibrarySpec::new(
    "retaprompt_commands",
    "libretaprompt_commands.so",
    "RETAPROMPT_COMMANDS_LIB_PATH",
    b"retaprompt_commands_abi_generation",
    b"retaprompt_commands_run_kind_argv",
);

const INPUT_LIBRARY: PromptLibrarySpec = PromptLibrarySpec::new(
    "retaprompt_input",
    "libretaprompt_input.so",
    "RETAPROMPT_INPUT_LIB_PATH",
    b"retaprompt_input_abi_generation",
    b"retaprompt_input_run_kind_argv",
);

pub fn run_input_prompt(kind_value: c_int) -> c_int {
    // Load commands first and keep the handle alive.  On Android/Termux this is
    // more reliable than trusting LD_LIBRARY_PATH/RPATH for libretaprompt_input's
    // command dependency, and it preserves the desired two-library topology.
    let commands = match LoadedPromptLibrary::load(COMMANDS_LIBRARY) {
        Ok(library) => library,
        Err(message) => return fail_runtime(&message),
    };
    let input = match LoadedPromptLibrary::load(INPUT_LIBRARY) {
        Ok(library) => library,
        Err(message) => return fail_runtime(&message),
    };

    let _keep_commands_loaded = commands;
    unsafe { input.run_kind_argv(kind_value) }.unwrap_or_else(|message| fail_runtime(&message))
}
