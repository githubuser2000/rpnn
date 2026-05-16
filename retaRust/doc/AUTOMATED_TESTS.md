# Automated tests for the Rust Reta architecture port

The current migration has three useful test layers.

## 1. Critical local suite

```bash
python3 tools/run_automated_tests.py --quick --pretty
```

This runs the most important Python probes and, when Cargo is available, the focused Rust tests for:

- `p1234` and `p12345` prompt regressions
- `mulpri 12345` above the table-row limit
- direct 493/744 materialization
- language sync and coverage for `-language=english -spalten --kontinuum=m`

## 2. Full suite

```bash
python3 tools/run_automated_tests.py --pretty
```

This runs all architecture probes and focused Cargo tests that are cheap enough to keep in the normal loop.

## 3. Binary smoke tests

After a debug build:

```bash
python3 tools/run_automated_tests.py --build-binaries --binary-smoke --pretty
```

The report is written below:

```text
target/reta_arch_tests/<timestamp>/automated_tests_report.json
```

`target/` is ignored by git, so the report does not bloat the repository.

## Direct Cargo tests

```bash
cargo test -p retaprompt_commands --test prompt_math_regressions
cargo test -p reta_architecture --test language_744_regressions
cargo test -p reta --test architecture_binary_smoke
```

These tests are deliberately focused on regressions that already happened in this thread:

- `p1234` must be math-only, not a bogus table call.
- `p12345` must produce math output, not "nichts auszugeben".
- Bare numeric prompts may still create default table calls, but only with valid `-spalten` parameters.
- `-language=english -spalten --kontinuum=m` must keep 493 and 744 direct after the synchronized CSV update.
