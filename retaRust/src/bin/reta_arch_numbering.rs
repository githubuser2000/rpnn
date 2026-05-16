use serde::Serialize;

#[derive(Serialize)]
struct NumberingInspectReport {
    args: Vec<String>,
    config: reta_architecture::TableViewNumberingConfig,
    report: reta_architecture::TableViewNumberingReport,
    output_with_legacy_numbering: reta_architecture::TableViewOutputReport,
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let view = reta_architecture::table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::from_cli_args(&args),
        &reta_architecture::MaterializedTableViewConfig::default(),
    );
    let config = reta_architecture::TableViewNumberingConfig::legacy_pair();
    let report = reta_architecture::numbering_report_for_rows(&view.rows, &config);
    let output_with_legacy_numbering = reta_architecture::render_table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::from_cli_args(&args),
        &reta_architecture::TableViewOutputConfig::default().with_legacy_numbering(),
    );
    let out = NumberingInspectReport {
        args,
        config,
        report,
        output_with_legacy_numbering,
    };
    match serde_json::to_string_pretty(&out) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("rreta_arch_numbering could not serialize report: {error}");
            std::process::exit(1);
        }
    }
}
