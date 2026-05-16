use serde::Serialize;

#[derive(Serialize)]
struct LanguageParityInspect {
    args: Vec<String>,
    policy: reta_architecture::TableViewLanguageParityPolicy,
    report: reta_architecture::TableViewLanguageParityReport,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let policy = reta_architecture::TableViewLanguageParityPolicy::default();
    let report = reta_architecture::language_parity_for_cli_args(&args, &policy);
    println!(
        "{}",
        serde_json::to_string_pretty(&LanguageParityInspect {
            args,
            policy,
            report,
        })?
    );
    Ok(())
}
