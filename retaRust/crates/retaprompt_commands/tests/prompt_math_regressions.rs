use retaprompt_commands::commands::{compile_command_with_state, PromptCommand, SessionState};

fn compile_rp(input: &str) -> PromptCommand {
    let state = SessionState::new("rp".to_string(), false, false);
    compile_command_with_state(input, &state).expect("prompt command should compile")
}

fn immediate_text(command: &PromptCommand) -> Option<String> {
    match command {
        PromptCommand::Immediate(output) => Some(output.text.clone()),
        PromptCommand::Sequence(commands) => commands.iter().find_map(immediate_text),
        _ => None,
    }
}

fn collect_reta_argvs(command: &PromptCommand, out: &mut Vec<Vec<String>>) {
    match command {
        PromptCommand::Reta(argv) => out.push(argv.clone()),
        PromptCommand::RetaBatch(argvs) => out.extend(argvs.iter().cloned()),
        PromptCommand::Sequence(commands) => {
            for command in commands {
                collect_reta_argvs(command, out);
            }
        }
        _ => {}
    }
}

fn reta_argvs(command: &PromptCommand) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    collect_reta_argvs(command, &mut out);
    out
}

fn assert_no_invalid_absicht_thomas_table_argv(command: &PromptCommand) {
    let argvs = reta_argvs(command);
    for argv in argvs {
        assert!(
            !argv.iter().any(|token| token == "--absicht"),
            "invalid bare --absicht leaked into reta argv: {argv:?}"
        );
        assert!(
            !argv.iter().any(|token| token == "--thomas"),
            "invalid bare --thomas leaked into reta argv: {argv:?}"
        );
    }
}

#[test]
fn p1234_is_math_only_and_never_synthesizes_invalid_reta_columns() {
    let command = compile_rp("p1234");
    let text = immediate_text(&command).expect("p1234 should be immediate math output");

    assert!(text.contains("1234"), "unexpected p1234 output: {text}");
    assert!(
        !text.contains("nichts auszugeben"),
        "p1234 must not fall back to the no-output prompt message: {text}"
    );
    assert_no_invalid_absicht_thomas_table_argv(&command);
    assert!(reta_argvs(&command).is_empty(), "p1234 must not synthesize reta table argv");
}

#[test]
fn p12345_is_math_output_above_table_limit_not_no_output_message() {
    let command = compile_rp("p12345");
    let text = immediate_text(&command).expect("p12345 should be immediate math output");

    assert!(text.contains("12345:"), "unexpected p12345 output: {text}");
    assert!(
        text.contains("823") || text.contains("2469") || text.contains("3 * 5"),
        "p12345 output should contain visible factorization evidence: {text}"
    );
    assert!(
        !text.contains("nichts auszugeben"),
        "p12345 must not fall back to the no-output prompt message: {text}"
    );
    assert_no_invalid_absicht_thomas_table_argv(&command);
    assert!(reta_argvs(&command).is_empty(), "p12345 must not synthesize reta table argv");
}

#[test]
fn mulpri_large_number_is_unbounded_math_not_table_row_limited() {
    let command = compile_rp("mulpri 12345");
    let text = immediate_text(&command).expect("mulpri 12345 should be immediate math output");

    assert!(text.contains("12345:"), "unexpected mulpri output: {text}");
    assert!(
        !text.contains("nichts auszugeben"),
        "mulpri 12345 must not be limited by the table row universe: {text}"
    );
    assert_no_invalid_absicht_thomas_table_argv(&command);
}

#[test]
fn prim_integer_list_above_table_limit_renders_each_number() {
    let command = compile_rp("prim 12345,12346");
    let text = immediate_text(&command).expect("prim list should be immediate math output");

    assert!(text.contains("12345"), "missing first number: {text}");
    assert!(text.contains("12346"), "missing second number: {text}");
    assert!(
        !text.contains("nichts auszugeben"),
        "prim integer list must not use fallback no-output text: {text}"
    );
}

#[test]
fn bare_number_still_gets_valid_default_reta_table_columns() {
    let command = compile_rp("1234");
    let argvs = reta_argvs(&command);
    assert!(!argvs.is_empty(), "bare numeric prompt should still generate reta calls");

    assert!(
        argvs.iter().any(|argv| argv.iter().any(|token| token == "--menschliches=motivation")),
        "expected valid absicht/motivation column parameter, got {argvs:?}"
    );
    assert!(
        argvs.iter().any(|argv| argv.iter().any(|token| token == "--galaxie=thomas")),
        "expected valid thomas/galaxie column parameter, got {argvs:?}"
    );
    assert_no_invalid_absicht_thomas_table_argv(&command);
}
