use serde::Serialize;

#[derive(Serialize)]
struct CellStyleInspect {
    args: Vec<String>,
    options: reta_architecture::TableViewOutputCliOptions,
    cell_styles: reta_architecture::TableViewCellStyleReport,
    output: reta_architecture::TableViewOutputReport,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        &reta_architecture::TableMaterializationConfig::default(),
        &reta_architecture::MaterializedTableViewConfig::default()
            .with_virtual_policy(output_config.virtual_column_policy),
    );
    let prefix_column_count = reta_architecture::output_prefix_column_count(&output_config);
    let cell_styles = reta_architecture::cell_style_report_for_rows(
        &view.rows,
        output_config.mode,
        &output_config.cell_styles,
        output_config.suppress_headers,
        output_config.include_empty_rows,
        prefix_column_count,
    );
    let output = reta_architecture::render_materialized_table_view(&view, &output_config);
    let inspect = CellStyleInspect {
        args,
        options,
        cell_styles,
        output,
    };
    println!("{}", serde_json::to_string_pretty(&inspect)?);
    Ok(())
}
