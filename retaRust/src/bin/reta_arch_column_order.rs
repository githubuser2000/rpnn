use serde::Serialize;

#[derive(Serialize)]
struct ColumnOrderInspectReport {
    args: Vec<String>,
    selected_columns: Vec<i64>,
    column_order_override: Vec<i64>,
    ordered_selected_columns: Vec<i64>,
    materialized_column_order_legacy: Vec<usize>,
    column_order_override_applied: bool,
    rendered_suppressed_lines: Vec<String>,
    rendered_tag_summary_lines: Vec<String>,
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let runtime = reta_architecture::bootstrap_parameter_runtime();
    let parsed = runtime.parse_cli_args(&args);
    let plan =
        reta_architecture::TableGenerationPlan::from_parameter_command_sets(&parsed.command_sets);
    let materialization = reta_architecture::bootstrap_table_materialization().materialize_plan(
        &plan,
        &reta_architecture::TableMaterializationConfig::from_cli_args(&args),
    );
    let suppressed_view = reta_architecture::table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::from_cli_args(&args),
        &reta_architecture::MaterializedTableViewConfig::default(),
    );
    let tag_summary_view = reta_architecture::table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::from_cli_args(&args),
        &reta_architecture::MaterializedTableViewConfig::default()
            .with_virtual_policy(reta_architecture::VirtualColumnDisplayPolicy::TagSummary),
    );
    let report = ColumnOrderInspectReport {
        args,
        selected_columns: plan.selected_columns.iter().copied().collect(),
        column_order_override: plan.column_order_override.clone(),
        ordered_selected_columns: plan.ordered_selected_columns(),
        materialized_column_order_legacy: materialization.materialized_column_order_legacy,
        column_order_override_applied: materialization.column_order_override_applied,
        rendered_suppressed_lines: suppressed_view.rendered_lines,
        rendered_tag_summary_lines: tag_summary_view.rendered_lines,
    };
    match serde_json::to_string_pretty(&report) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("rreta_arch_column_order could not serialize report: {error}");
            std::process::exit(1);
        }
    }
}
