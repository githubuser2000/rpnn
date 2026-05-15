use serde::Serialize;

#[derive(Serialize)]
struct VirtualColumnInspect {
    args: Vec<String>,
    options: reta_architecture::TableViewVirtualColumnCliOptions,
    output_options: reta_architecture::TableViewOutputCliOptions,
    config: reta_architecture::TableViewVirtualColumnConfig,
    report: reta_architecture::TableViewVirtualColumnReport,
    output: reta_architecture::TableViewOutputReport,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let options = reta_architecture::parse_table_view_virtual_column_cli_options(&args);
    let output_options = reta_architecture::parse_table_view_output_cli_options(&args);
    let config = reta_architecture::TableViewVirtualColumnConfig::from_cli_options(&options);
    let report = reta_architecture::virtual_column_report_for_cli_args(&args, &config);
    let output = reta_architecture::render_table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::default(),
        &reta_architecture::TableViewOutputConfig::default().with_cli_options(output_options.clone()),
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&VirtualColumnInspect {
            args,
            options,
            output_options,
            config,
            report,
            output,
        })?
    );
    Ok(())
}
