# Prompt regression tests

These tests protect the prompt short forms that caused regressions during the
Rust port.

## Main cases

```text
p1234
p12345
mulpri 12345
prim 12345,12346
1234
```

The important distinction is:

* `p1234` and `p12345` are math prompt commands. They expand to `mulpri <n>`
  and must produce immediate math output.
* `p12345` must not be capped by the table row universe. The table row parser
  can reject values above the current row maximum, but math output must still
  accept the number.
* bare numeric prompts such as `1234` may still synthesize the default table
  view, but only with valid `-spalten` parameters:
  `--menschliches=motivation` and `--galaxie=thomas`.
* the old invalid form must never be generated:
  `-spalten --absicht --thomas`.

## Static probes

```bash
python3 tools/architecture_prompt_p1234_probe.py --pretty
python3 tools/architecture_prompt_p12345_probe.py --pretty
```

## Combined regression runner

Without Cargo, this runs static probes:

```bash
python3 tools/run_prompt_regression_tests.py --pretty
```

With Cargo, also run focused Rust unit tests:

```bash
python3 tools/run_prompt_regression_tests.py --cargo --pretty
```

With a built prompt binary, also smoke-test the executable:

```bash
python3 tools/run_prompt_regression_tests.py --cargo --binary-smoke --pretty
```

## Focused cargo tests

```bash
cargo test -p reta --lib p_prefixed_number -- --nocapture
cargo test -p reta --lib p_prefixed_large_number -- --nocapture
cargo test -p reta --lib mulpri_large_number -- --nocapture
cargo test -p reta --lib direct_math_integer_list -- --nocapture
```
