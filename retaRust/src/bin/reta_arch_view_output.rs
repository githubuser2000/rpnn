use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let report = reta_architecture::render_table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::default(),
        &reta_architecture::TableViewOutputConfig::default(),
    );
    match serde_json::to_string_pretty(&report) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("failed to serialize table view output: {error}");
            std::process::exit(1);
        }
    }
}
