use serde::Serialize;

#[derive(Serialize)]
struct RowOrderInspectReport {
    args: Vec<String>,
    selected_rows: Vec<i64>,
    row_order_override: Vec<i64>,
    ordered_selected_rows: Vec<i64>,
    requested_row_order_zero_based: Vec<usize>,
    materialized_row_order_zero_based: Vec<usize>,
    row_order_override_applied: bool,
    rendered_lines: Vec<String>,
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let runtime = reta_architecture::bootstrap_parameter_runtime();
    let parsed = runtime.parse_cli_args(&args);
    let plan =
        reta_architecture::TableGenerationPlan::from_parameter_command_sets(&parsed.command_sets);
    let materialization = reta_architecture::bootstrap_table_materialization().materialize_plan(
        &plan,
        &reta_architecture::TableMaterializationConfig::default(),
    );
    let view = reta_architecture::table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::default(),
        &reta_architecture::MaterializedTableViewConfig::default(),
    );
    let report = RowOrderInspectReport {
        args,
        selected_rows: plan.selected_rows.iter().copied().collect(),
        row_order_override: plan.row_order_override.clone(),
        ordered_selected_rows: plan.ordered_selected_rows(),
        requested_row_order_zero_based: materialization.requested_row_order_zero_based,
        materialized_row_order_zero_based: materialization.materialized_row_order_zero_based,
        row_order_override_applied: materialization.row_order_override_applied,
        rendered_lines: view.rendered_lines,
    };
    match serde_json::to_string_pretty(&report) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("rreta_arch_row_order could not serialize report: {error}");
            std::process::exit(1);
        }
    }
}
