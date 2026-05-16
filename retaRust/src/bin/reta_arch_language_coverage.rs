use serde::Serialize;

#[derive(Serialize)]
struct LanguageCoverageInspect {
    args: Vec<String>,
    policy: reta_architecture::TableViewLanguageCoveragePolicy,
    report: reta_architecture::TableViewLanguageCoverageReport,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let policy = reta_architecture::TableViewLanguageCoveragePolicy::default();
    let report = reta_architecture::language_coverage_for_cli_args(&args, &policy);
    println!(
        "{}",
        serde_json::to_string_pretty(&LanguageCoverageInspect {
            args,
            policy,
            report,
        })?
    );
    Ok(())
}
