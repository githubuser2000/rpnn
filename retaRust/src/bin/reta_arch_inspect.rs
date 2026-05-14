fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let (_, switch_config) = reta_architecture::extract_architecture_switch_from_argv(&args, None);
    let plan = reta_architecture::bootstrap_shadow_pipeline().cli_plan(&args, &switch_config);
    match serde_json::to_string_pretty(&plan) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("rreta_arch_inspect could not serialize architecture plan: {error}");
            std::process::exit(1);
        }
    }
}
