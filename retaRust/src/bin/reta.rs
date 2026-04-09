use reta::shared::reta_py::Program;
use reta::shared::words_py::Words;

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
