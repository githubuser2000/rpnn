fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let options = reta_architecture::parse_table_view_output_cli_options(&args);
    let output_config = reta_architecture::TableViewOutputConfig::default()
        .with_mode(reta_architecture::OutputMode::Html)
        .with_cli_options(options.clone());
    let view = reta_architecture::table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::default(),
        &reta_architecture::MaterializedTableViewConfig::default()
            .with_virtual_policy(output_config.virtual_column_policy),
    );
    let html_attributes = reta_architecture::html_attribute_report_for_rows(
        &view.rows,
        &output_config.html_attributes,
        output_config.suppress_headers,
        output_config.include_empty_rows,
    );
    let output = reta_architecture::render_materialized_table_view(&view, &output_config);
    let json = serde_json::json!({
        "args": args,
        "options": options,
        "html_attributes": html_attributes,
        "output": output,
    });
    println!("{}", serde_json::to_string_pretty(&json).expect("serialize html output report"));
}
