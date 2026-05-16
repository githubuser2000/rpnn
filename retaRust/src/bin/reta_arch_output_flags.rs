use serde::Serialize;

#[derive(Serialize)]
struct OutputFlagsInspectReport {
    args: Vec<String>,
    options: reta_architecture::TableViewOutputCliOptions,
    report: reta_architecture::TableViewOutputReport,
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let options = reta_architecture::parse_table_view_output_cli_options(&args);
    let report = reta_architecture::render_table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::from_cli_args(&args),
        &reta_architecture::TableViewOutputConfig::default(),
    );
    let out = OutputFlagsInspectReport {
        args,
        options,
        report,
    };
    match serde_json::to_string_pretty(&out) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("rreta_arch_output_flags could not serialize report: {error}");
            std::process::exit(1);
        }
    }
}
