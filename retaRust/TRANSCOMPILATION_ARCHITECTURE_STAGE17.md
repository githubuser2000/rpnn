# Transcompilation Architecture Stage 17

Stage 17 hardens the Rust architecture from declared surface toward real parameter semantics.

## Main change

A generated Rust parameter matrix was added:

```text
crates/reta_architecture/src/parameter_matrix.rs
```

It is generated from:

```text
python_arch_reference/i18n/words_matrix.py::paraNdataMatrix
```

and stores the integer column projection for each `(parameter main aliases, parameter aliases)` entry.

## Why this matters

Before Stage 17, the Rust parameter runtime could parse CLI tokens but did not resolve the important py-reta-arch alias pair:

```text
--kontinuum=m
```

to the current columns:

```text
493, 744
```

Stage 17 makes that resolution part of the typed Rust architecture.

## Concrete effects

- `bootstrap_schema()` now includes generated `para_n_data_matrix` entries.
- `ParameterSemanticsSheaf` now learns main aliases from the generated matrix, not only top-level CLI groups.
- `ParameterRuntimeBundle::parse_cli_args()` resolves `-spalten --kontinuum=m` to selected columns `493` and `744`.
- Negative values such as `--kontinuum=m,-m` are tracked as excluded columns.
- `CompletionRuntimeBundle` is now filled from input semantics/schema instead of leaving `spalten` completion empty.
- Nested prompt completion can now suggest `--kontinuum=` and the value `m`.
- `SemanticsBuilder` now collects actual matrix column projections instead of only numeric aliases.

## Generated matrix stats

```text
parameter matrix seeds: 429
unique integer column projections: 642
744 regression present: yes
```

## Added regeneration tool

```text
tools/generate_parameter_matrix.py
```

The tool is deterministic and can regenerate `parameter_matrix.rs` from `python_arch_reference`.

## Visibility

Visible output is still not blindly switched. Stage 17 strengthens the architecture/shadow path and parameter semantics so future adapter commits have better Rust-owned data.
