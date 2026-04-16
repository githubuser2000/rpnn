use std::env;
use std::io::{self, IsTerminal, Read, Write};

use reta::{run_reta_from_args_with_runtime, RetaRuntime};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let stdin_text = read_stdin_if_piped();

    let result = run_reta_from_args_with_runtime(
        args,
        stdin_text,
        RetaRuntime {
            terminal_width: detect_terminal_width(),
            stdout_is_tty: Some(io::stdout().is_terminal()),
            stderr_is_tty: Some(io::stderr().is_terminal()),
            stdin_is_tty: Some(io::stdin().is_terminal()),
        },
    );

    let mut stderr = io::stderr().lock();
    if !result.stderr.is_empty() {
        let _ = write!(stderr, "{}", result.stderr);
    }

    let mut stdout = io::stdout().lock();
    if !result.stdout.is_empty() {
        let _ = write!(stdout, "{}", result.stdout);
    }

    std::process::exit(result.exit_code);
}

fn read_stdin_if_piped() -> Option<String> {
    if io::stdin().is_terminal() {
        return None;
    }

    let mut buf = String::new();
    match io::stdin().read_to_string(&mut buf) {
        Ok(_) if !buf.is_empty() => Some(buf),
        _ => None,
    }
}

fn detect_terminal_width() -> Option<usize> {
    env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|width| *width > 0)
}
