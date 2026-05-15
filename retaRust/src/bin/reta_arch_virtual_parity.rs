use serde::Serialize;

#[derive(Serialize)]
struct VirtualParityInspect {
    args: Vec<String>,
    config: reta_architecture::TableViewVirtualParityConfig,
    report: reta_architecture::TableViewVirtualParityReport,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let config = reta_architecture::TableViewVirtualParityConfig::default();
    let report = reta_architecture::compare_virtual_column_policies_for_cli_args(&args, &config);
    println!(
        "{}",
        serde_json::to_string_pretty(&VirtualParityInspect {
            args,
            config,
            report,
        })?
    );
    Ok(())
}
