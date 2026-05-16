use serde::Serialize;

#[derive(Serialize)]
struct RowStyleInspectReport {
    args: Vec<String>,
    options: reta_architecture::TableViewOutputCliOptions,
    row_styles: reta_architecture::TableViewRowStyleReport,
    output: reta_architecture::TableViewOutputReport,
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let options = reta_architecture::parse_table_view_output_cli_options(&args);
    let parsed = reta_architecture::bootstrap_parameter_runtime().parse_cli_args(&args);
    let mode = parsed
        .selected_output_mode
        .unwrap_or(reta_architecture::OutputMode::Html);
    let output_config = reta_architecture::TableViewOutputConfig::default()
        .with_mode(mode)
        .with_cli_options(options.clone());
    let view = reta_architecture::table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::from_cli_args(&args),
        &reta_architecture::MaterializedTableViewConfig::default()
            .with_virtual_policy(output_config.virtual_column_policy),
    );
    let row_styles = reta_architecture::row_style_report_for_rows(
        &view.rows,
        output_config.mode,
        &output_config.row_styles,
        output_config.suppress_headers,
        output_config.include_empty_rows,
    );
    let output = reta_architecture::render_materialized_table_view(&view, &output_config);
    let report = RowStyleInspectReport {
        args,
        options,
        row_styles,
        output,
    };
    match serde_json::to_string_pretty(&report) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("rreta_arch_row_styles could not serialize report: {error}");
            std::process::exit(1);
        }
    }
}
