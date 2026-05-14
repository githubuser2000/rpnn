# Transcompilation Architecture Stage 20

Stage 20 adds the first concrete CSV-backed table materialization layer.

## Main addition

New module:

```text
crates/reta_architecture/src/table_materialization.rs
```

This module converts the existing Rust architecture data into concrete local CSV sections:

```text
CLI args -> parameter_runtime -> TableGenerationPlan -> CSV projection -> MaterializedCsvSection
```

## Important semantic correction

Reta legacy column numbers are treated as legacy/source column indices for CSV projection, not blindly as one-based CSV coordinates.

For the regression case:

```text
-spalten --kontinuum=m
```

Rust now sees the matrix projection:

```text
493, 744
```

and materializes the CSV-backed part:

```text
493 -> religion.csv source column index 493 -> "M Kontinuum (dreizehn)"
```

while honestly reporting:

```text
744 -> unresolved/missing as a direct religion.csv column
```

This is better than pretending that every matrix projection is already a plain CSV column. It separates:

```text
CSV-backed ordinary data
from
later generated/extra/non-ordinary table sections
```

## New binary

```text
rreta_arch_materialize
```

Example:

```bash
cargo run --bin rreta_arch_materialize -- \
  reta -zeilen --vorhervonausschnitt=1-1 \
  -spalten --kontinuum=m --breite=0
```

## New FFI export

```text
reta_architecture_table_materialization_json(argc, argv)
```

## New probe tool

```text
tools/architecture_table_materialization_probe.py
```

It checks the source-of-truth CSV and generated parameter matrix. It can also compare a built `rreta_arch_materialize` binary when supplied.

## Runtime integration

`ArchitectureRuntime` now contains:

```text
table_materialization
```

`RetaRunArchitecture` now reports:

```text
materialized_csv_section_count
materialized_csv_cell_count
materialized_continuum_m
```

The shadow CLI plan now includes a `materialization_report`.

## Checks

Isolated `reta_architecture` check with local serde stubs passed.

```text
132 tests passed
0 failed
```

Full workspace build remains blocked in this container by crates.io DNS resolution.
