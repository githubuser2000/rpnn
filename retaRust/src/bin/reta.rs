use reta::shared::reta_py::Program;
use reta::shared::words_py::Words;

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    let words = Words::new();

    let mut program = Program::new(argv);
    program.runAllesLikePythonInit(&words);
    program.run(&words);
    program.combiTableWorkflow();

    if !program.cliErrors.is_empty() {
        for line in &program.cliErrors {
            println!("{}", line);
        }
    } else if !program.finallyDisplayLines.is_empty() {
        for line in &program.finallyDisplayLines {
            println!("{}", line);
        }
    } else {
        println!("{}", program.snapshot());
    }
}
