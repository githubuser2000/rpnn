# Transcompilation Architecture Stage 54

Stage 54 adds a guarded activation promotion layer after the Stage 51-53 readiness work.

## New Rust module

- `crates/reta_architecture/src/table_view_activation_promotion.rs`

This module folds a `TableViewActivationReadinessReport` plus the runtime switch gate into a single promotion decision. It answers whether a specific table-view case is eligible for future default visible Rust output promotion.

## Key safety rule

Promotion is still strict by default:

- readiness must be ready,
- the runtime commit gate must allow `table_view_activation_promotion.default_visible_source`,
- the runtime mode must be commit or force,
- the selected source must be `table_view_output`,
- raw line equality must still hold,
- virtual-column policy must be identity on direct CSV-backed cells.

Semantic equality remains diagnostic only; it does not replace raw parity.

## New public API

- `TableViewActivationPromotionPolicy`
- `TableViewActivationPromotionCheck`
- `TableViewActivationPromotionReport`
- `TableViewActivationPromotionSnapshot`
- `TableViewActivationPromotionBundle`
- `activation_promotion_from_readiness(...)`
- `activation_promotion_for_cli_args(...)`
- `activation_promotion_checks_from_readiness(...)`
- `bootstrap_table_view_activation_promotion(...)`
- `continuum_m_activation_promotion_smoke(...)`

## New CLI / inspect binary

- `rreta_arch_activation_promotion`

Example:

```bash
cargo run --bin rreta_arch_activation_promotion -- \
  --legacy-lines-file legacy.txt \
  --reta-arch=commit \
  -zeilen --vorhervonausschnitt=1-1 \
  -spalten --kontinuum=m \
  --breite=0
```

## New FFI export

- `reta_architecture_table_view_activation_promotion_json(...)`

## New CLI flags stripped before legacy execution

- `--activation-promotion-strict`
- `--activation-promotion-diagnostic`
- `--activation-promotion-allow-force`
- `--activation-promotion-no-force`
- `--activation-promotion-require-commit-mode`
- `--activation-promotion-ignore-commit-mode`
- `--activation-promotion-require-readiness`
- `--activation-promotion-ignore-readiness`
- `--activation-promotion-include-selected-lines`
- `--activation-promotion-no-selected-lines`
- `--activation-promotion-preview=N`

Short aliases with `--promotion-*` are also supported.

## Root bridge

`src/reta_arch_shadow.rs` now computes `view_output_promotion` after readiness. `src/reta_workflow_py.rs` emits a new diagnostic:

- `ARCH_TABLE_VIEW_ACTIVATION_PROMOTION`

Stage 54 does not make visible Rust output the default. It only makes the promotion decision explicit and auditable.

## Additional fix

The root shadow bridge had duplicate internal lines in the extracted Stage 53 tree:

- duplicate `switch_config` extraction,
- duplicate `view_output_replay` field in the struct literal.

Both are removed in Stage 54.
