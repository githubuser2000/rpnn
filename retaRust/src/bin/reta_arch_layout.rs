use serde::Serialize;

#[derive(Serialize)]
struct LayoutInspectReport {
    args: Vec<String>,
    options: reta_architecture::TableViewOutputCliOptions,
    layout: reta_architecture::TableViewLayoutReport,
    output: reta_architecture::TableViewOutputReport,
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let options = reta_architecture::parse_table_view_output_cli_options(&args);
    let view = reta_architecture::table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::default(),
        &reta_architecture::MaterializedTableViewConfig::default(),
    );
    let output_config =
        reta_architecture::TableViewOutputConfig::default().with_cli_options(options.clone());
    let layout = reta_architecture::shell_layout_report_for_rows(&view.rows, &output_config);
    let output = reta_architecture::render_table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::default(),
        &output_config,
    );
    let out = LayoutInspectReport {
        args,
        options,
        layout,
        output,
    };
    match serde_json::to_string_pretty(&out) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("rreta_arch_layout could not serialize report: {error}");
            std::process::exit(1);
        }
    }
}
