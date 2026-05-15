use serde::Serialize;

#[derive(Serialize)]
struct StyleCompositionInspect {
    args: Vec<String>,
    options: reta_architecture::TableViewOutputCliOptions,
    composition_counts: (usize, usize),
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
    let composition_counts =
        reta_architecture::html_cell_style_composition_counts(&view.rows, &output_config);
    let output = reta_architecture::render_materialized_table_view(&view, &output_config);
    println!(
        "{}",
        serde_json::to_string_pretty(&StyleCompositionInspect {
            args,
            options,
            composition_counts,
            output,
        })?
    );
    Ok(())
}
