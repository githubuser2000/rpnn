use reta::doc_tools::generate4readme;

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    std::process::exit(generate4readme::main_like_python(&argv));
}
