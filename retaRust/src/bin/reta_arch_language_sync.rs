use serde::Serialize;

#[derive(Serialize)]
struct LanguageSyncInspect {
    args: Vec<String>,
    policy: reta_architecture::TableViewLanguageSyncPolicy,
    report: reta_architecture::TableViewLanguageSyncReport,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let policy = reta_architecture::TableViewLanguageSyncPolicy::default();
    let report = reta_architecture::language_sync_for_cli_args(&args, &policy);
    println!(
        "{}",
        serde_json::to_string_pretty(&LanguageSyncInspect {
            args,
            policy,
            report,
        })?
    );
    Ok(())
}
