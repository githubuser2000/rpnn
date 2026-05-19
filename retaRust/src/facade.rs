//! Thin public C-ABI facade for `libreta.so`.
//!
//! With the `split-facade` feature enabled, this crate keeps only the
//! stable public symbols and forwards the heavy Reta engine work to
//! `libreta_runtime.so`.  The other private core libraries remain linked
//! through `reta_split_abi` so the executable topology is still explicit.

use std::os::raw::c_char;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RetaFfiResponse {
    pub stdout_text: *mut c_char,
    pub stdout_len: usize,
    pub stderr_text: *mut c_char,
    pub stderr_len: usize,
    pub exit_code: i32,
}

unsafe extern "C" {
    fn reta_runtime_core_run_and_print_from_env_ffi() -> i32;
    fn reta_runtime_core_abi_version() -> u32;
    fn reta_runtime_core_run_argv(argc: usize, argv: *const *const c_char, stdin_text: *const c_char, terminal_width: usize, stdout_is_tty: u8, stderr_is_tty: u8, stdin_is_tty: u8) -> RetaFfiResponse;
    fn reta_runtime_core_free_string(ptr: *mut c_char);
    fn reta_runtime_core_shared_words_json() -> *mut c_char;
    fn reta_runtime_core_csv_catalog_snapshot_json() -> *mut c_char;
    fn reta_runtime_core_csv_catalog_assets_json() -> *mut c_char;
    fn reta_runtime_core_html_class_catalog_snapshot_json() -> *mut c_char;
    fn reta_runtime_core_html_class_catalog_records_json() -> *mut c_char;
    fn reta_runtime_core_architecture_table_materialization_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_column_order_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_row_order_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_virtual_columns_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_virtual_parity_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_commit_audit_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_activation_transaction_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_activation_journal_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_activation_replay_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_activation_ledger_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_activation_store_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_activation_persistence_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_activation_file_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_activation_recovery_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_activation_readiness_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_activation_promotion_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_language_parity_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_language_coverage_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_language_sync_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_prompt_language_completion_json(input: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_prompt_language_guard_json(input: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_prompt_activation_readiness_json(input: *const c_char, argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_output_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_numbering_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_layout_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_html_attributes_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_row_styles_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_cell_styles_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_style_composition_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_output_options_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_output_parity_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_shell_styles_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_style_parity_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_snapshot_json() -> *mut c_char;
    fn reta_runtime_core_architecture_cli_plan_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_activation_plan_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_shadow_cli_plan_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_prompt_shadow_plan_json(program_name: *const c_char, prompt_input: *const c_char) -> *mut c_char;
    fn reta_runtime_core_architecture_shadow_commit_policy_json() -> *mut c_char;
    fn reta_runtime_core_architecture_prompt_commit_policy_json() -> *mut c_char;
    fn reta_runtime_core_architecture_table_view_output_commit_policy_json() -> *mut c_char;
    fn reta_runtime_core_architecture_governance_snapshot_json() -> *mut c_char;
    fn reta_runtime_core_execution_network_plan_json(argc: usize, argv: *const *const c_char) -> *mut c_char;
    fn reta_runtime_core_parity_probe_plan_json() -> *mut c_char;
    fn reta_runtime_core_all_main_alias_groups_json() -> *mut c_char;
    fn reta_runtime_core_parameter_alias_groups_for_main_json(canonical_main: *const c_char) -> *mut c_char;
    fn reta_runtime_core_resolve_parameter_main_alias(main_alias: *const c_char) -> *mut c_char;
}

#[inline(always)]
fn preload_private_core_libraries() {
    let _ = crate::reta_split_abi::preload_reta_split_shared_libraries();
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_run_and_print_from_env_ffi() -> i32 {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_run_and_print_from_env_ffi() }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_abi_version() -> u32 {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_abi_version() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_run_argv(argc: usize, argv: *const *const c_char, stdin_text: *const c_char, terminal_width: usize, stdout_is_tty: u8, stderr_is_tty: u8, stdin_is_tty: u8) -> RetaFfiResponse {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_run_argv(argc, argv, stdin_text, terminal_width, stdout_is_tty, stderr_is_tty, stdin_is_tty) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_free_string(ptr: *mut c_char) {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_free_string(ptr) };
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_shared_words_json() -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_shared_words_json() }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_csv_catalog_snapshot_json() -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_csv_catalog_snapshot_json() }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_csv_catalog_assets_json() -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_csv_catalog_assets_json() }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_html_class_catalog_snapshot_json() -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_html_class_catalog_snapshot_json() }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_html_class_catalog_records_json() -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_html_class_catalog_records_json() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_materialization_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_materialization_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_column_order_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_column_order_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_row_order_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_row_order_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_virtual_columns_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_virtual_columns_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_virtual_parity_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_virtual_parity_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_commit_audit_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_commit_audit_json(argc, argv, legacy_text) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_activation_transaction_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_activation_transaction_json(argc, argv, legacy_text) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_activation_journal_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_activation_journal_json(argc, argv, legacy_text) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_activation_replay_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_activation_replay_json(argc, argv, legacy_text) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_activation_ledger_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_activation_ledger_json(argc, argv, legacy_text) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_activation_store_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_activation_store_json(argc, argv, legacy_text) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_activation_persistence_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_activation_persistence_json(argc, argv, legacy_text) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_activation_file_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_activation_file_json(argc, argv, legacy_text) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_activation_recovery_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_activation_recovery_json(argc, argv, legacy_text) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_activation_readiness_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_activation_readiness_json(argc, argv, legacy_text) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_activation_promotion_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_activation_promotion_json(argc, argv, legacy_text) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_language_parity_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_language_parity_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_language_coverage_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_language_coverage_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_language_sync_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_language_sync_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_prompt_language_completion_json(input: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_prompt_language_completion_json(input) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_prompt_language_guard_json(input: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_prompt_language_guard_json(input) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_prompt_activation_readiness_json(input: *const c_char, argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_prompt_activation_readiness_json(input, argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_output_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_output_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_numbering_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_numbering_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_layout_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_layout_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_html_attributes_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_html_attributes_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_row_styles_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_row_styles_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_cell_styles_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_cell_styles_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_style_composition_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_style_composition_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_output_options_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_output_options_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_output_parity_json(argc: usize, argv: *const *const c_char, legacy_text: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_output_parity_json(argc, argv, legacy_text) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_shell_styles_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_shell_styles_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_style_parity_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_style_parity_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_architecture_snapshot_json() -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_snapshot_json() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_cli_plan_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_cli_plan_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_activation_plan_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_activation_plan_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_shadow_cli_plan_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_shadow_cli_plan_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_prompt_shadow_plan_json(program_name: *const c_char, prompt_input: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_prompt_shadow_plan_json(program_name, prompt_input) }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_architecture_shadow_commit_policy_json() -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_shadow_commit_policy_json() }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_architecture_prompt_commit_policy_json() -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_prompt_commit_policy_json() }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_architecture_table_view_output_commit_policy_json() -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_table_view_output_commit_policy_json() }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_architecture_governance_snapshot_json() -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_architecture_governance_snapshot_json() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_execution_network_plan_json(argc: usize, argv: *const *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_execution_network_plan_json(argc, argv) }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parity_probe_plan_json() -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_parity_probe_plan_json() }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_all_main_alias_groups_json() -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_all_main_alias_groups_json() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_parameter_alias_groups_for_main_json(canonical_main: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_parameter_alias_groups_for_main_json(canonical_main) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_resolve_parameter_main_alias(main_alias: *const c_char) -> *mut c_char {
    preload_private_core_libraries();
    unsafe { reta_runtime_core_resolve_parameter_main_alias(main_alias) }
}
