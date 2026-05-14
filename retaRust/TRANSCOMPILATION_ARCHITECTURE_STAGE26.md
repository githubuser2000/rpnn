# Transcompilation Architecture Stage 26

Stage 26 moves the Rust materialized table path closer to real Reta output semantics by preserving explicit column order from `--spaltenreihenfolgeundnurdiese`.

## Main change

Before Stage 26, the Rust architecture path parsed the option but the path into `TableGenerationPlan` / `CsvProjectionRequest` used sorted set order. That meant an explicit request like:

```bash
-ausgabe --spaltenreihenfolgeundnurdiese=744,493
```

could collapse back to sorted order. Stage 26 now keeps the requested order:

```text
parameter_runtime -> TableGenerationPlan.column_order_override -> CsvProjectionRequest.column_order_legacy -> MaterializedCsvSection.selected_columns_legacy -> MaterializedTableView
```

## Safety invariant

The important `kontinuum=m` case remains safe:

```text
-spalten --kontinuum=m -> 493, 744
493 -> direct religion.csv column
744 -> virtual / non-direct witness column
```

With the default virtual-column policy, `744` is still suppressed from visible output. With `TagSummary`, the explicit order `744,493` is observable as a diagnostic/renderable view:

```text
744:sternPolygon,keinParaOdMetaP | M Kontinuum (dreizehn)
```

## New files

```text
src/bin/reta_arch_column_order.rs
tools/architecture_column_order_probe.py
```

## Checks

- Isolated `reta_architecture` cargo check with local serde stubs: passed
- Isolated `reta_architecture` cargo test with local serde stubs: 148 passed, 0 failed
- Coverage: 1096 / 1096 functions, 239 / 239 classes
- Strict semantic surface: marker-only 0 functions, missing 0 functions; marker-only 0 classes, missing 0 classes
- Column-order probe: ok

Full workspace cargo check was attempted but blocked by crates.io DNS resolution for `hypher`.
