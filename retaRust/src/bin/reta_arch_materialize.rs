fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let report = reta_architecture::bootstrap_table_materialization().materialize_cli_args(
        &args,
        &reta_architecture::TableMaterializationConfig::from_cli_args(&args),
    );
    match serde_json::to_string_pretty(&report) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("rreta_arch_materialize could not serialize materialization report: {error}");
            std::process::exit(1);
        }
    }
}
