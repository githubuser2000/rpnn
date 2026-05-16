use std::process::Command;

fn run_bin(bin: &str, args: &[&str]) -> String {
    let exe = match bin {
        "rreta_arch_language_sync" => env!("CARGO_BIN_EXE_rreta_arch_language_sync"),
        "rreta_arch_language_coverage" => env!("CARGO_BIN_EXE_rreta_arch_language_coverage"),
        "rreta_arch_language_parity" => env!("CARGO_BIN_EXE_rreta_arch_language_parity"),
        "rreta_arch_prompt_language_guard" => env!("CARGO_BIN_EXE_rreta_arch_prompt_language_guard"),
        "rreta_arch_prompt_activation_readiness" => env!("CARGO_BIN_EXE_rreta_arch_prompt_activation_readiness"),
        other => panic!("unknown test binary {other}"),
    };
    let output = Command::new(exe)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("failed to run {bin}: {error}"));
    assert!(
        output.status.success(),
        "{bin} failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("binary stdout should be utf-8 JSON/text")
}

fn continuum_m_args() -> Vec<&'static str> {
    vec![
        "reta",
        "-language=english",
        "-zeilen",
        "--vorhervonausschnitt=1-1",
        "-spalten",
        "--kontinuum=m",
        "--breite=0",
    ]
}

#[test]
fn language_architecture_binaries_report_synced_744_state() {
    let args = continuum_m_args();

    let sync = run_bin("rreta_arch_language_sync", &args);
    assert!(sync.contains("\"pending_action_count\": 0"), "unexpected sync JSON: {sync}");
    assert!(sync.contains("\"status\": \"ready\""), "unexpected sync status: {sync}");

    let coverage = run_bin("rreta_arch_language_coverage", &args);
    assert!(coverage.contains("\"stale_language_count\": 0"), "unexpected coverage JSON: {coverage}");
    assert!(coverage.contains("\"languages_missing_744\": []"), "unexpected coverage 744 state: {coverage}");

    let parity = run_bin("rreta_arch_language_parity", &args);
    assert!(parity.contains("\"direct_744_materialized\": true"), "unexpected parity JSON: {parity}");
    assert!(parity.contains("\"effective_asset_name\": \"en-religion.csv\""), "unexpected parity asset: {parity}");
}

#[test]
fn prompt_language_binaries_report_ready_for_continuum_m() {
    let prompt_args = [
        "--reta-arch=commit",
        "reta",
        "-language=english",
        "-spalten",
        "--kontinuum=m",
    ];

    let guard = run_bin("rreta_arch_prompt_language_guard", &prompt_args[1..]);
    assert!(guard.contains("\"status\": \"ready\""), "unexpected prompt guard JSON: {guard}");
    assert!(guard.contains("\"direct_744_available_for_prompt_language\": true"), "prompt guard lost direct 744: {guard}");

    let readiness = run_bin("rreta_arch_prompt_activation_readiness", &prompt_args);
    assert!(readiness.contains("\"prompt_language_guard_ready\": true"), "unexpected prompt readiness JSON: {readiness}");
}
