use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let view = reta_architecture::table_view_for_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::default(),
        &reta_architecture::MaterializedTableViewConfig::default(),
    );
    match serde_json::to_string_pretty(&view) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("failed to serialize table view: {error}");
            std::process::exit(1);
        }
    }
}
