use crate::completion::{candidates_for_input, candidates_for_input_in_mode_with_context};
use crate::python_like::PromptModus;

fn assert_contains(values: &[String], expected: &str) {
    assert!(
        values.iter().any(|value| value == expected),
        "expected completion candidate {expected:?}; got first candidates {:?}",
        values.iter().take(30).collect::<Vec<_>>()
    );
}

fn assert_not_contains(values: &[String], forbidden: &str) {
    assert!(
        !values.iter().any(|value| value == forbidden),
        "unexpected completion candidate {forbidden:?}; got first candidates {:?}",
        values.iter().take(30).collect::<Vec<_>>()
    );
}

#[test]
fn python_retaprompt_top_level_completion_contract() {
    // Python NestedCompleter starts with befehle/befehle2, including the
    // uppercase HELP command and the canonical reta command.  Keep this as a
    // conservative contract; it does not change ranking or fuzzy behavior.
    let values = candidates_for_input("");
    assert_contains(&values, "HELP");
    assert_contains(&values, "hilfe");
    assert_contains(&values, "reta");

    let values = candidates_for_input("he");
    assert_contains(&values, "HELP");
    assert_contains(&values, "hilfe");
}

#[test]
fn python_retaprompt_reta_main_parameter_contract() {
    let values = candidates_for_input("reta ");
    for expected in [
        "-zeilen",
        "-spalten",
        "-kombination",
        "-ausgabe",
        "-nichts",
        "-help",
        "-h",
    ] {
        assert_contains(&values, expected);
    }
}

#[test]
fn python_retaprompt_nested_parameter_and_value_contract() {
    let values = candidates_for_input("reta -zeilen --ze");
    assert_contains(&values, "--zeit=");
    assert_contains(&values, "--zaehlung=");
    assert_contains(&values, "--primzahlen=");

    let values = candidates_for_input("reta -zeilen --zeit=h");
    assert_contains(&values, "heute");
    assert_contains(&values, "-heute");

    let values = candidates_for_input("reta -zeilen --zeit=[heute,gestern],m");
    assert_contains(&values, "morgen");
    assert_contains(&values, "-morgen");

    let values = candidates_for_input("reta -ausgabe --art=h");
    assert_contains(&values, "html");
}

#[test]
fn python_retaprompt_spalten_and_kombination_value_contract() {
    let values = candidates_for_input("reta -spalten --menschliches=bew");
    assert_contains(&values, "Bewusstsein_und_Wahrnehmung");
    assert_contains(&values, "bewusstsein");

    let values = candidates_for_input("reta -kombination --galaxie=le");
    assert_contains(&values, "Lebewesen");
    assert_contains(&values, "lebewesen");
}

#[test]
fn python_retaprompt_wahl15_wahl16_nested_contract() {
    let values = candidates_for_input("15_13_");
    assert_contains(&values, "15_13_6");
    assert_contains(&values, "15_13_17");
    assert_contains(&values, "15_13_1pro8");

    let values = candidates_for_input("16_15_1pro");
    assert_contains(&values, "16_15_1pro12");
    assert_contains(&values, "16_15_1pro13");
    assert_contains(&values, "16_15_1pro19");
}

#[test]
fn python_retaprompt_stored_context_and_delete_mode_contract() {
    let values = candidates_for_input_in_mode_with_context(
        "--ze",
        PromptModus::Normal,
        &["reta".to_string(), "-zeilen".to_string()],
        &[],
    );
    assert_contains(&values, "--zeit=");

    let values = candidates_for_input_in_mode_with_context(
        "1-",
        PromptModus::LoeschenSelect,
        &[],
        &[
            "reta".to_string(),
            "-zeilen".to_string(),
            "--zeit=heute".to_string(),
        ],
    );
    assert!(
        values.is_empty(),
        "delete-selection mode must disable completion candidates"
    );
}

#[test]
fn completion_contract_is_not_made_more_aggressive() {
    // This test documents the scope of the parity work: it checks candidates,
    // not automatic acceptance behavior.  In particular, it must not require
    // quick-completion/keybinding changes in reedline.
    let values = candidates_for_input("reta -zeilen --zeit=h");
    assert_contains(&values, "heute");
    assert_not_contains(&values, "--zeit=heute");
}
