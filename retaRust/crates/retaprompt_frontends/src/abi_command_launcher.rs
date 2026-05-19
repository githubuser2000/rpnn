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

pub fn run_command_prompt(kind_value: c_int) -> c_int {
    let commands = match LoadedPromptLibrary::load(COMMANDS_LIBRARY) {
        Ok(library) => library,
        Err(message) => return fail_runtime(&message),
    };
    unsafe { commands.run_kind_argv(kind_value) }.unwrap_or_else(|message| fail_runtime(&message))
}
