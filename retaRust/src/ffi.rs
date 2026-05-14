use std::collections::BTreeSet;
use std::ffi::CString;
use std::os::raw::c_char;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Mutex, OnceLock};

use serde_json;

use crate::{build_cli_request, run_reta, RetaRuntime};

pub const RETA_ABI_VERSION: u32 = 2;
const MAX_FFI_ARGC: usize = 4096;
const MAX_FFI_STRING_BYTES: usize = 16 * 1024 * 1024;

static FFI_ALLOCATIONS: OnceLock<Mutex<BTreeSet<usize>>> = OnceLock::new();

#[repr(C)]
pub struct RetaFfiResponse {
    pub stdout_text: *mut c_char,
    pub stdout_len: usize,
    pub stderr_text: *mut c_char,
    pub stderr_len: usize,
    pub exit_code: i32,
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_abi_version() -> u32 {
    RETA_ABI_VERSION
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_run_argv(
    argc: usize,
    argv: *const *const c_char,
    stdin_text: *const c_char,
    terminal_width: usize,
    stdout_is_tty: u8,
    stderr_is_tty: u8,
    stdin_is_tty: u8,
) -> RetaFfiResponse {
    match catch_unwind(AssertUnwindSafe(|| unsafe {
        reta_run_argv_impl(
            argc,
            argv,
            stdin_text,
            terminal_width,
            stdout_is_tty,
            stderr_is_tty,
            stdin_is_tty,
        )
    })) {
        Ok(response) => response,
        Err(_) => ffi_error_response(101, "panic inside reta_run_argv\n"),
    }
}

unsafe fn reta_run_argv_impl(
    argc: usize,
    argv: *const *const c_char,
    stdin_text: *const c_char,
    terminal_width: usize,
    stdout_is_tty: u8,
    stderr_is_tty: u8,
    stdin_is_tty: u8,
) -> RetaFfiResponse {
    let args = match unsafe { read_argv(argc, argv) } {
        Ok(args) => args,
        Err(message) => return ffi_error_response(2, format!("reta ffi error: {message}\n")),
    };
    let stdin_text = match unsafe { read_optional_string(stdin_text) } {
        Ok(stdin_text) => stdin_text,
        Err(message) => return ffi_error_response(2, format!("reta ffi stdin error: {message}\n")),
    };

    let request = build_cli_request(
        &args,
        stdin_text,
        RetaRuntime {
            terminal_width: if terminal_width == 0 {
                None
            } else {
                Some(terminal_width)
            },
            stdout_is_tty: Some(stdout_is_tty != 0),
            stderr_is_tty: Some(stderr_is_tty != 0),
            stdin_is_tty: Some(stdin_is_tty != 0),
        },
    );

    match run_reta(request) {
        Ok(response) => {
            let (stdout_text, stdout_len) = into_c_string_with_len(response.rendered_text);
            let (stderr_text, stderr_len) = into_c_string_with_len(response.stderr_text);
            RetaFfiResponse {
                stdout_text,
                stdout_len,
                stderr_text,
                stderr_len,
                exit_code: response.exit_code,
            }
        }
        Err(error) => {
            let (stdout_text, stdout_len) = into_c_string_with_len(String::new());
            let (stderr_text, stderr_len) =
                into_c_string_with_len(format!("reta failed: {error}\n"));
            RetaFfiResponse {
                stdout_text,
                stdout_len,
                stderr_text,
                stderr_len,
                exit_code: error.exit_code(),
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }

    if !unregister_ffi_allocation(ptr) {
        return;
    }

    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_shared_words_json() -> *mut c_char {
    ffi_json_result(|| serde_json::to_string(crate::shared_words()))
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_csv_catalog_snapshot_json() -> *mut c_char {
    ffi_json_result(|| {
        serde_json::to_string(&reta_architecture::bootstrap_csv_catalog().snapshot())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_csv_catalog_assets_json() -> *mut c_char {
    ffi_json_result(|| serde_json::to_string(&reta_architecture::csv_asset_records()))
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_html_class_catalog_snapshot_json() -> *mut c_char {
    ffi_json_result(|| serde_json::to_string(&reta_architecture::html_class_catalog_snapshot()))
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_html_class_catalog_records_json() -> *mut c_char {
    ffi_json_result(|| serde_json::to_string(&reta_architecture::html_class_owned_records()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_materialization_json(
    argc: usize,
    argv: *const *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let args = unsafe { read_argv(argc, argv) }.unwrap_or_default();
        let report = reta_architecture::bootstrap_table_materialization().materialize_cli_args(
            &args,
            &reta_architecture::TableMaterializationConfig::default(),
        );
        serde_json::to_string(&report)
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta_architecture_table_materialization_json"),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_column_order_json(
    argc: usize,
    argv: *const *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let args = unsafe { read_argv(argc, argv) }.unwrap_or_default();
        let parsed = reta_architecture::bootstrap_parameter_runtime().parse_cli_args(&args);
        let plan = reta_architecture::TableGenerationPlan::from_parameter_command_sets(
            &parsed.command_sets,
        );
        let materialization = reta_architecture::bootstrap_table_materialization()
            .materialize_plan(
                &plan,
                &reta_architecture::TableMaterializationConfig::default(),
            );
        serde_json::to_string(&serde_json::json!({
            "args": args,
            "selected_columns": plan.selected_columns.iter().copied().collect::<Vec<_>>(),
            "column_order_override": plan.column_order_override.clone(),
            "ordered_selected_columns": plan.ordered_selected_columns(),
            "materialized_column_order_legacy": materialization.materialized_column_order_legacy,
            "column_order_override_applied": materialization.column_order_override_applied,
        }))
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta_architecture_column_order_json"),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_row_order_json(
    argc: usize,
    argv: *const *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let args = unsafe { read_argv(argc, argv) }.unwrap_or_default();
        let parsed = reta_architecture::bootstrap_parameter_runtime().parse_cli_args(&args);
        let plan = reta_architecture::TableGenerationPlan::from_parameter_command_sets(
            &parsed.command_sets,
        );
        let materialization = reta_architecture::bootstrap_table_materialization()
            .materialize_plan(
                &plan,
                &reta_architecture::TableMaterializationConfig::default(),
            );
        serde_json::to_string(&serde_json::json!({
            "args": args,
            "selected_rows": plan.selected_rows.iter().copied().collect::<Vec<_>>(),
            "row_order_override": plan.row_order_override.clone(),
            "ordered_selected_rows": plan.ordered_selected_rows(),
            "requested_row_order_zero_based": materialization.requested_row_order_zero_based,
            "materialized_row_order_zero_based": materialization.materialized_row_order_zero_based,
            "row_order_override_applied": materialization.row_order_override_applied,
        }))
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta_architecture_row_order_json"),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_json(
    argc: usize,
    argv: *const *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let args = unsafe { read_argv(argc, argv) }.unwrap_or_default();
        let view = reta_architecture::table_view_for_cli_args(
            &args,
            &reta_architecture::TableMaterializationConfig::default(),
            &reta_architecture::MaterializedTableViewConfig::default(),
        );
        serde_json::to_string(&view)
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta_architecture_table_view_json"),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_output_json(
    argc: usize,
    argv: *const *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let args = unsafe { read_argv(argc, argv) }.unwrap_or_default();
        let report = reta_architecture::render_table_view_for_cli_args(
            &args,
            &reta_architecture::TableMaterializationConfig::default(),
            &reta_architecture::TableViewOutputConfig::default(),
        );
        serde_json::to_string(&report)
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta_architecture_table_view_output_json"),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_output_options_json(
    argc: usize,
    argv: *const *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let args = unsafe { read_argv(argc, argv) }.unwrap_or_default();
        let options = reta_architecture::parse_table_view_output_cli_options(&args);
        let report = reta_architecture::render_table_view_for_cli_args(
            &args,
            &reta_architecture::TableMaterializationConfig::default(),
            &reta_architecture::TableViewOutputConfig::default(),
        );
        serde_json::to_string(&serde_json::json!({
            "args": args,
            "options": options,
            "report": report,
        }))
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => {
            json_error_string("panic inside reta_architecture_table_view_output_options_json")
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_table_view_output_parity_json(
    argc: usize,
    argv: *const *const c_char,
    legacy_text: *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let args = unsafe { read_argv(argc, argv) }.unwrap_or_default();
        let legacy_text = unsafe { read_optional_string(legacy_text) }
            .unwrap_or_default()
            .unwrap_or_default();
        let legacy_lines = legacy_text
            .lines()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        let parsed = reta_architecture::bootstrap_parameter_runtime().parse_cli_args(&args);
        let mode = parsed
            .selected_output_mode
            .unwrap_or(reta_architecture::OutputMode::Shell);
        let output_report = reta_architecture::render_table_view_for_cli_args(
            &args,
            &reta_architecture::TableMaterializationConfig::default(),
            &reta_architecture::TableViewOutputConfig::default().with_mode(mode),
        );
        let parity = reta_architecture::compare_table_view_output_to_legacy(
            &output_report,
            &legacy_lines,
            &reta_architecture::TableViewOutputParityConfig::default().with_mode(mode),
        );
        serde_json::to_string(&parity)
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta_architecture_table_view_output_parity_json"),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_architecture_snapshot_json() -> *mut c_char {
    ffi_json_result(|| serde_json::to_string(&crate::shared_architecture().snapshot_ref()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_cli_plan_json(
    argc: usize,
    argv: *const *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let args = unsafe { read_argv(argc, argv) }.unwrap_or_default();
        let plan = reta_architecture::RetaRunArchitecture::from_cli_args(&args);
        serde_json::to_string(&plan)
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta_architecture_cli_plan_json"),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_activation_plan_json(
    argc: usize,
    argv: *const *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let args = unsafe { read_argv(argc, argv) }.unwrap_or_default();
        let (_, switch_config) =
            reta_architecture::extract_architecture_switch_from_argv(&args, None);
        let switch_bundle =
            reta_architecture::bootstrap_runtime_switch(Some(switch_config.clone()));
        let migration_control = reta_architecture::bootstrap_migration_control();
        let units = migration_control.activation_units_for_switch(&switch_bundle, &switch_config);
        serde_json::to_string(&units)
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta_architecture_activation_plan_json"),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_shadow_cli_plan_json(
    argc: usize,
    argv: *const *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let args = unsafe { read_argv(argc, argv) }.unwrap_or_default();
        let (_, switch_config) =
            reta_architecture::extract_architecture_switch_from_argv(&args, None);
        let plan = reta_architecture::bootstrap_shadow_pipeline().cli_plan(&args, &switch_config);
        serde_json::to_string(&plan)
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta_architecture_shadow_cli_plan_json"),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_architecture_prompt_shadow_plan_json(
    program_name: *const c_char,
    prompt_input: *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let program_name =
            unsafe { read_required_string(program_name) }.unwrap_or_else(|_| "rp".to_string());
        let prompt_input = unsafe { read_required_string(prompt_input) }.unwrap_or_default();
        let input = reta_architecture::ShadowPromptInput::new(program_name, prompt_input);
        let config = reta_architecture::ArchitectureSwitchConfig::from_environment();
        let report = reta_architecture::bootstrap_shadow_pipeline().shadow_prompt(&input, &config);
        serde_json::to_string(&report)
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta_architecture_prompt_shadow_plan_json"),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_architecture_shadow_commit_policy_json() -> *mut c_char {
    ffi_json_result(|| serde_json::to_string(&reta_architecture::ShadowCommitPolicy::default()))
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_architecture_prompt_commit_policy_json() -> *mut c_char {
    ffi_json_result(|| {
        serde_json::to_string(&reta_architecture::ShadowPromptCommitPolicy::default())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_architecture_table_view_output_commit_policy_json() -> *mut c_char {
    ffi_json_result(|| {
        serde_json::to_string(&reta_architecture::ShadowTableViewOutputCommitPolicy::default())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_architecture_governance_snapshot_json() -> *mut c_char {
    ffi_json_result(|| {
        let runtime = crate::shared_architecture();
        serde_json::to_string(&(
            runtime.architecture_map.snapshot(),
            runtime.architecture_contracts.snapshot(),
            runtime.architecture_witnesses.snapshot(),
            runtime.architecture_coherence.snapshot(),
            runtime.architecture_validation.snapshot(),
            runtime.architecture_progress.snapshot(),
        ))
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_execution_network_plan_json(
    argc: usize,
    argv: *const *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let args = unsafe { read_argv(argc, argv) }.unwrap_or_default();
        let indices = (0..args.len()).collect::<Vec<_>>();
        let plan = reta_architecture::execution_network_plan_for_indices(
            &indices,
            reta_architecture::DataflowDiscipline::Fifo,
        );
        serde_json::to_string(&plan)
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta_execution_network_plan_json"),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_parity_probe_plan_json() -> *mut c_char {
    ffi_json_result(|| {
        let harness = reta_architecture::bootstrap_parity_harness();
        let config = reta_architecture::ArchitectureSwitchConfig::from_environment();
        serde_json::to_string(&harness.plans_for_switch(&config))
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn reta_all_main_alias_groups_json() -> *mut c_char {
    ffi_json_result(|| {
        serde_json::to_string(
            &crate::domain::python_source_of_truth::all_main_alias_groups(crate::shared_words()),
        )
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_parameter_alias_groups_for_main_json(
    canonical_main: *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let main = unsafe { read_required_string(canonical_main) }.unwrap_or_default();
        serde_json::to_string(
            &crate::domain::python_source_of_truth::parameter_alias_groups_for_main(
                crate::shared_words(),
                &main,
            ),
        )
    })) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta_parameter_alias_groups_for_main_json"),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reta_resolve_parameter_main_alias(
    main_alias: *const c_char,
) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(|| {
        let main = unsafe { read_required_string(main_alias) }.unwrap_or_default();
        crate::domain::python_source_of_truth::resolve_parameter_main_alias(
            crate::shared_words(),
            &main,
        )
    })) {
        Ok(Some(canonical)) => into_c_string(canonical),
        Ok(None) => into_c_string(String::new()),
        Err(_) => into_c_string(String::new()),
    }
}

unsafe fn read_argv(argc: usize, argv: *const *const c_char) -> Result<Vec<String>, String> {
    if argc == 0 {
        return Ok(Vec::new());
    }

    if argc > MAX_FFI_ARGC {
        return Err(format!("argc {argc} ueberschreitet Maximum {MAX_FFI_ARGC}"));
    }

    if argv.is_null() {
        return Err("argv war null bei argc > 0".to_string());
    }

    let mut args = Vec::with_capacity(argc);
    for index in 0..argc {
        let arg_ptr = unsafe { *argv.add(index) };
        let arg = unsafe { read_required_string(arg_ptr) }
            .map_err(|message| format!("argv[{index}] {message}"))?;
        args.push(arg);
    }

    Ok(args)
}

unsafe fn read_required_string(ptr: *const c_char) -> Result<String, String> {
    if ptr.is_null() {
        return Ok(String::new());
    }
    unsafe { read_c_string_bounded(ptr, MAX_FFI_STRING_BYTES) }
}

unsafe fn read_optional_string(ptr: *const c_char) -> Result<Option<String>, String> {
    if ptr.is_null() {
        return Ok(None);
    }
    unsafe { read_c_string_bounded(ptr, MAX_FFI_STRING_BYTES) }.map(Some)
}

unsafe fn read_c_string_bounded(ptr: *const c_char, max_bytes: usize) -> Result<String, String> {
    for len in 0..max_bytes {
        let byte = unsafe { *ptr.add(len) };
        if byte == 0 {
            let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) };
            return std::str::from_utf8(bytes)
                .map(|text| text.to_string())
                .map_err(|_| "ist kein valider UTF-8-String".to_string());
        }
    }
    Err(format!(
        "ist laenger als {max_bytes} Bytes oder nicht NUL-terminiert"
    ))
}

fn ffi_json_result<F>(build: F) -> *mut c_char
where
    F: FnOnce() -> serde_json::Result<String>,
{
    match catch_unwind(AssertUnwindSafe(build)) {
        Ok(Ok(json)) => into_c_string(json),
        Ok(Err(error)) => json_error_string(&error.to_string()),
        Err(_) => json_error_string("panic inside reta JSON FFI export"),
    }
}

fn ffi_error_response<S: Into<String>>(exit_code: i32, stderr_text: S) -> RetaFfiResponse {
    let (stdout_text, stdout_len) = into_c_string_with_len(String::new());
    let (stderr_text, stderr_len) = into_c_string_with_len(stderr_text.into());
    RetaFfiResponse {
        stdout_text,
        stdout_len,
        stderr_text,
        stderr_len,
        exit_code,
    }
}

fn json_error_string(message: &str) -> *mut c_char {
    into_c_string(format!(r#"{{"error":"{}"}}"#, message.replace('"', "'")))
}

fn register_ffi_allocation(ptr: *mut c_char) -> *mut c_char {
    if !ptr.is_null() {
        if let Ok(mut guard) = FFI_ALLOCATIONS
            .get_or_init(|| Mutex::new(BTreeSet::new()))
            .lock()
        {
            guard.insert(ptr as usize);
        }
    }
    ptr
}

fn unregister_ffi_allocation(ptr: *mut c_char) -> bool {
    FFI_ALLOCATIONS
        .get_or_init(|| Mutex::new(BTreeSet::new()))
        .lock()
        .map(|mut guard| guard.remove(&(ptr as usize)))
        .unwrap_or(false)
}

fn into_c_string(text: String) -> *mut c_char {
    into_c_string_with_len(text).0
}

fn into_c_string_with_len(text: String) -> (*mut c_char, usize) {
    let sanitized = text.replace('\0', "�");
    let len = sanitized.len();
    let c_string = CString::new(sanitized).unwrap_or_else(|_| {
        CString::new("internal error while building CString").unwrap_or_else(|_| CString::default())
    });
    (register_ffi_allocation(c_string.into_raw()), len)
}
