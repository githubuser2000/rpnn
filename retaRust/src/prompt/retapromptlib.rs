use std::path::Path;

use super::app::{
    run_prompt_command_frontend_with_profile, run_prompt_frontend,
    run_prompt_frontend_with_profile, run_prompt_input_frontend_with_profile, run_rp_one_shot,
};
use super::frontend_profile::{PromptFrontendKind, PromptFrontendProfile};

fn env_args() -> Vec<String> {
    std::env::args().collect::<Vec<_>>()
}

fn program_name_from_argv(argv: &[String]) -> String {
    argv.first()
        .and_then(|arg0| Path::new(arg0).file_name())
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "rp".to_string())
}

pub fn run_with_kind(argv: Vec<String>, kind: PromptFrontendKind) -> i32 {
    let profile = PromptFrontendProfile::for_kind(kind, true);
    run_prompt_frontend_with_profile(argv, profile)
}

pub fn run_with_profile(argv: Vec<String>, profile: PromptFrontendProfile) -> i32 {
    run_prompt_frontend_with_profile(argv, profile)
}

pub fn run_input_with_kind(argv: Vec<String>, kind: PromptFrontendKind) -> i32 {
    let profile = PromptFrontendProfile::for_kind(kind, true);
    run_prompt_input_frontend_with_profile(argv, profile)
}

pub fn run_command_with_kind(argv: Vec<String>, kind: PromptFrontendKind) -> i32 {
    let profile = PromptFrontendProfile::for_kind(kind, true);
    run_prompt_command_frontend_with_profile(argv, profile)
}

pub fn run_rp(argv: Vec<String>) -> i32 {
    run_with_kind(argv, PromptFrontendKind::Rp)
}

pub fn run_rpl(argv: Vec<String>) -> i32 {
    run_with_kind(argv, PromptFrontendKind::Rpl)
}

pub fn run_rpb(argv: Vec<String>) -> i32 {
    run_with_kind(argv, PromptFrontendKind::Rpb)
}

pub fn run_rpe(argv: Vec<String>) -> i32 {
    run_with_kind(argv, PromptFrontendKind::Rpe)
}

pub fn run_input_rp(argv: Vec<String>) -> i32 {
    run_input_with_kind(argv, PromptFrontendKind::Rp)
}

pub fn run_input_rpl(argv: Vec<String>) -> i32 {
    run_input_with_kind(argv, PromptFrontendKind::Rpl)
}

pub fn run_input_rpe(argv: Vec<String>) -> i32 {
    run_input_with_kind(argv, PromptFrontendKind::Rpe)
}

pub fn run_command_rp(argv: Vec<String>) -> i32 {
    run_command_with_kind(argv, PromptFrontendKind::Rp)
}

pub fn run_command_rpl(argv: Vec<String>) -> i32 {
    run_command_with_kind(argv, PromptFrontendKind::Rpl)
}

pub fn run_command_rpb(argv: Vec<String>) -> i32 {
    run_command_with_kind(argv, PromptFrontendKind::Rpb)
}

pub fn run_command_rpe(argv: Vec<String>) -> i32 {
    run_command_with_kind(argv, PromptFrontendKind::Rpe)
}

pub fn run_rp_from_env() -> i32 {
    run_rp(env_args())
}

pub fn run_rpl_from_env() -> i32 {
    run_rpl(env_args())
}

pub fn run_rpb_from_env() -> i32 {
    run_rpb(env_args())
}

pub fn run_rpe_from_env() -> i32 {
    run_rpe(env_args())
}

pub fn run_input_rp_from_env() -> i32 {
    run_input_rp(env_args())
}

pub fn run_input_rpl_from_env() -> i32 {
    run_input_rpl(env_args())
}

pub fn run_input_rpe_from_env() -> i32 {
    run_input_rpe(env_args())
}

pub fn run_command_rp_from_env() -> i32 {
    run_command_rp(env_args())
}

pub fn run_command_rpl_from_env() -> i32 {
    run_command_rpl(env_args())
}

pub fn run_command_rpb_from_env() -> i32 {
    run_command_rpb(env_args())
}

pub fn run_command_rpe_from_env() -> i32 {
    run_command_rpe(env_args())
}

pub fn run_auto_from_env() -> i32 {
    run_prompt_frontend(env_args(), true)
}

pub fn run_one_shot_direct(argv: Vec<String>) -> i32 {
    run_rp_one_shot(argv, true)
}

pub fn run_retaprompt_with_kind(argv: Vec<String>, kind: PromptFrontendKind) -> i32 {
    run_with_kind(argv, kind)
}

pub fn run_retaprompt_with_profile(argv: Vec<String>, profile: PromptFrontendProfile) -> i32 {
    run_with_profile(argv, profile)
}

pub fn run_retaprompt_rp(argv: Vec<String>) -> i32 {
    run_rp(argv)
}

pub fn run_retaprompt_rpl(argv: Vec<String>) -> i32 {
    run_rpl(argv)
}

pub fn run_retaprompt_rpb(argv: Vec<String>) -> i32 {
    run_rpb(argv)
}

pub fn run_retaprompt_rpe(argv: Vec<String>) -> i32 {
    run_rpe(argv)
}

pub fn run_retaprompt_rp_from_env() -> i32 {
    run_rp_from_env()
}

pub fn run_retaprompt_rpl_from_env() -> i32 {
    run_rpl_from_env()
}

pub fn run_retaprompt_rpb_from_env() -> i32 {
    run_rpb_from_env()
}

pub fn run_retaprompt_rpe_from_env() -> i32 {
    run_rpe_from_env()
}

pub fn run_retaprompt_auto_from_env() -> i32 {
    run_auto_from_env()
}

fn kind_from_abi_value(kind: i32) -> PromptFrontendKind {
    match kind {
        1 => PromptFrontendKind::Rp,
        2 => PromptFrontendKind::Rpl,
        3 => PromptFrontendKind::Rpb,
        4 => PromptFrontendKind::Rpe,
        _ => PromptFrontendKind::Auto,
    }
}

pub fn retaprompt_run_kind_from_env(kind: i32) -> i32 {
    let resolved = kind_from_abi_value(kind);
    if resolved == PromptFrontendKind::Auto {
        run_auto_from_env()
    } else {
        run_with_kind(env_args(), resolved)
    }
}

fn input_kind_from_program_name(program_name: &str) -> Option<PromptFrontendKind> {
    match program_name {
        "rp" => Some(PromptFrontendKind::Rp),
        "rpl" => Some(PromptFrontendKind::Rpl),
        "rpe" => Some(PromptFrontendKind::Rpe),
        _ => None,
    }
}

fn launcher_kind_from_program_name(program_name: &str) -> Option<PromptFrontendKind> {
    match program_name {
        "rp" => Some(PromptFrontendKind::Rp),
        "rpl" => Some(PromptFrontendKind::Rpl),
        "rpb" => Some(PromptFrontendKind::Rpb),
        "rpe" => Some(PromptFrontendKind::Rpe),
        _ => None,
    }
}

pub fn run_input_current_executable_from_env() -> i32 {
    let argv = env_args();
    let program_name = program_name_from_argv(&argv);
    match input_kind_from_program_name(&program_name) {
        Some(kind) => run_input_with_kind(argv, kind),
        None => {
            eprintln!(
                "retaprompt_input cannot infer input frontend kind from executable name: {program_name}"
            );
            1
        }
    }
}

pub fn run_input_any_current_executable_from_env() -> i32 {
    let argv = env_args();
    let program_name = program_name_from_argv(&argv);
    match launcher_kind_from_program_name(&program_name) {
        Some(PromptFrontendKind::Rp) => run_input_with_kind(argv, PromptFrontendKind::Rp),
        Some(PromptFrontendKind::Rpl) => run_input_with_kind(argv, PromptFrontendKind::Rpl),
        Some(PromptFrontendKind::Rpb) => run_command_with_kind(argv, PromptFrontendKind::Rpb),
        Some(PromptFrontendKind::Rpe) => run_input_with_kind(argv, PromptFrontendKind::Rpe),
        Some(PromptFrontendKind::Auto) | None => {
            eprintln!(
                "retaprompt_input cannot infer launcher kind from executable name: {program_name}"
            );
            eprintln!("expected one of: rp, rpl, rpb, rpe");
            1
        }
    }
}

pub fn run_input_launcher_kind_from_env(kind: i32) -> i32 {
    match kind_from_abi_value(kind) {
        PromptFrontendKind::Rp => run_input_rp_from_env(),
        PromptFrontendKind::Rpl => run_input_rpl_from_env(),
        PromptFrontendKind::Rpb => run_command_rpb_from_env(),
        PromptFrontendKind::Rpe => run_input_rpe_from_env(),
        PromptFrontendKind::Auto => {
            eprintln!("invalid retaprompt launcher kind: {kind}");
            1
        }
    }
}

pub fn retaprompt_run_rp_from_env_abi() -> i32 {
    run_rp_from_env()
}

pub fn retaprompt_run_rpl_from_env_abi() -> i32 {
    run_rpl_from_env()
}

pub fn retaprompt_run_rpb_from_env_abi() -> i32 {
    run_rpb_from_env()
}

pub fn retaprompt_run_rpe_from_env_abi() -> i32 {
    run_rpe_from_env()
}

pub fn retaprompt_input_run_rp_from_env_abi() -> i32 {
    run_input_rp_from_env()
}

pub fn retaprompt_input_run_rpl_from_env_abi() -> i32 {
    run_input_rpl_from_env()
}

pub fn retaprompt_input_run_rpe_from_env_abi() -> i32 {
    run_input_rpe_from_env()
}

pub fn retaprompt_commands_run_rp_from_env_abi() -> i32 {
    run_command_rp_from_env()
}

pub fn retaprompt_commands_run_rpl_from_env_abi() -> i32 {
    run_command_rpl_from_env()
}

pub fn retaprompt_commands_run_rpb_from_env_abi() -> i32 {
    run_command_rpb_from_env()
}

pub fn retaprompt_commands_run_rpe_from_env_abi() -> i32 {
    run_command_rpe_from_env()
}

fn ffi_guard_i32<F>(name: &str, f: F) -> i32
where
    F: FnOnce() -> i32,
{
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(exit_code) => exit_code,
        Err(_) => {
            eprintln!("panic inside {name}");
            101
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_run_kind_from_env(kind: i32) -> i32 {
    ffi_guard_i32("reta_retaprompt_run_kind_from_env", || {
        retaprompt_run_kind_from_env(kind)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_run_auto_from_env() -> i32 {
    ffi_guard_i32("reta_retaprompt_run_auto_from_env", || {
        run_retaprompt_auto_from_env()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_run_rp_from_env() -> i32 {
    ffi_guard_i32("reta_retaprompt_run_rp_from_env", || {
        retaprompt_run_rp_from_env_abi()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_run_rpl_from_env() -> i32 {
    ffi_guard_i32("reta_retaprompt_run_rpl_from_env", || {
        retaprompt_run_rpl_from_env_abi()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_run_rpb_from_env() -> i32 {
    ffi_guard_i32("reta_retaprompt_run_rpb_from_env", || {
        retaprompt_run_rpb_from_env_abi()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_run_rpe_from_env() -> i32 {
    ffi_guard_i32("reta_retaprompt_run_rpe_from_env", || {
        retaprompt_run_rpe_from_env_abi()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_input_run_current_executable_from_env() -> i32 {
    ffi_guard_i32(
        "reta_retaprompt_input_run_current_executable_from_env",
        || run_input_current_executable_from_env(),
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_input_run_any_current_executable_from_env() -> i32 {
    ffi_guard_i32(
        "reta_retaprompt_input_run_any_current_executable_from_env",
        || run_input_any_current_executable_from_env(),
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_input_run_launcher_kind_from_env(kind: i32) -> i32 {
    ffi_guard_i32("reta_retaprompt_input_run_launcher_kind_from_env", || {
        run_input_launcher_kind_from_env(kind)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_input_run_rp_from_env() -> i32 {
    ffi_guard_i32("reta_retaprompt_input_run_rp_from_env", || {
        retaprompt_input_run_rp_from_env_abi()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_input_run_rpl_from_env() -> i32 {
    ffi_guard_i32("reta_retaprompt_input_run_rpl_from_env", || {
        retaprompt_input_run_rpl_from_env_abi()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_input_run_rpe_from_env() -> i32 {
    ffi_guard_i32("reta_retaprompt_input_run_rpe_from_env", || {
        retaprompt_input_run_rpe_from_env_abi()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_commands_run_rp_from_env() -> i32 {
    ffi_guard_i32("reta_retaprompt_commands_run_rp_from_env", || {
        retaprompt_commands_run_rp_from_env_abi()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_commands_run_rpl_from_env() -> i32 {
    ffi_guard_i32("reta_retaprompt_commands_run_rpl_from_env", || {
        retaprompt_commands_run_rpl_from_env_abi()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_commands_run_rpb_from_env() -> i32 {
    ffi_guard_i32("reta_retaprompt_commands_run_rpb_from_env", || {
        retaprompt_commands_run_rpb_from_env_abi()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_retaprompt_commands_run_rpe_from_env() -> i32 {
    ffi_guard_i32("reta_retaprompt_commands_run_rpe_from_env", || {
        retaprompt_commands_run_rpe_from_env_abi()
    })
}
