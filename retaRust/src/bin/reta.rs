use reta_shared::shared::words_py::Words;
use reta_shared::shared::reta_py::Program;

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    let words = Words::new();

    let mut program = Program::new(argv);
    program.runAllesLikePythonInit(&words);
    program.run(&words);
    program.combiTableWorkflow();

    if program.cliErrors.len() > 0 {
        for line in program.cliErrors.iter() {
            println!("{}", line);
        }
    } else if program.finallyDisplayLines.len() > 0 {
        for line in program.finallyDisplayLines.iter() {
            println!("{}", line);
        }
    } else {
        println!("{}", program.snapshot());
    }
}
