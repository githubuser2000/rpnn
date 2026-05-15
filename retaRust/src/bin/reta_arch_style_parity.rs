use serde::Serialize;

#[derive(Serialize)]
struct StyleParityInspect {
    args: Vec<String>,
    mode: String,
    styled: reta_architecture::TableViewOutputReport,
    plain: reta_architecture::TableViewOutputReport,
    parity: reta_architecture::TableViewOutputParityReport,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let parsed = reta_architecture::bootstrap_parameter_runtime().parse_cli_args(&args);
    let mode = parsed
        .selected_output_mode
        .unwrap_or(reta_architecture::OutputMode::Html);
    let view = reta_architecture::table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::default(),
        &reta_architecture::MaterializedTableViewConfig::default(),
    );
    let options = reta_architecture::parse_table_view_output_cli_options(&args);
    let styled_config = reta_architecture::TableViewOutputConfig::default()
        .with_mode(mode)
        .with_cli_options(options);
    let plain_config = reta_architecture::TableViewOutputConfig::default().with_mode(mode);
    let styled = reta_architecture::render_materialized_table_view(&view, &styled_config);
    let plain = reta_architecture::render_materialized_table_view(&view, &plain_config);
    let parity = reta_architecture::compare_table_view_output_lines(
        &plain.rendered_lines,
        &styled.rendered_lines,
        &reta_architecture::TableViewOutputParityConfig::default().with_mode(mode),
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&StyleParityInspect {
            args,
            mode: mode.canonical_name().to_string(),
            styled,
            plain,
            parity,
        })?
    );
    Ok(())
}
