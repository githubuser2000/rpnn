use reta_shared::shared::words_py::Words;
use reta_shared::shared::reta_py::Program;

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    let words = Words::new();

    let mut program = Program::new(argv);
    program.run(&words);
    program.workflowEverything();
    program.combiTableWorkflow();

    if program.cliErrors.len() > 0 {
        for line in program.cliErrors.iter() {
            println!("{}", line);
        }
    } else {
        println!("{}", program.snapshot());
    }
}
