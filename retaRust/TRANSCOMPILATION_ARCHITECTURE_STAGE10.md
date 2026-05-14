# Transcompilation Architecture Stage 10

Stage 10 adds the first operational shadow/adapter activation path.

The previous stages built architecture modules and governance metadata.  This stage connects those structures to the running legacy-compatible `rreta` path without changing visible output.

## Added Rust modules and entry points

- `crates/reta_architecture/src/shadow_pipeline.rs`
  - CLI shadow plan
  - table-adapter shadow renderer
  - prompt-adapter shadow planner
  - deterministic line-diff summary
- `src/reta_arch_shadow.rs`
  - root-crate bridge from legacy `Program` fields into `reta_architecture::ShadowTableInput`
- `src/bin/reta_arch_inspect.rs`
  - JSON architecture/shadow inspection binary
- `tools/architecture_shadow_probe.py`
  - dependency-free parity/shadow smoke runner

## Runtime integration

`run_reta` now still executes the legacy-compatible table path first.  If `--reta-arch=dry-run`, `--reta-arch-adapters`, `--reta-arch-commit`, `--reta-arch-force`, or tracing is active, it additionally builds a Rust shadow table report from:

- `Program.__resultingTable`
- `Program.finallyDisplayLines`
- output mode / width / numbering / color flags

The shadow report is stored as an internal diagnostic.  Visible stdout/stderr stays legacy-controlled.

## New FFI exports

- `reta_architecture_shadow_cli_plan_json`
- `reta_architecture_prompt_shadow_plan_json`

These export the typed activation/shadow plans without requiring a visible behaviour switch.

## Migration and gating changes

The runtime switch now knows these morphisms:

- `shadow_pipeline.table_adapter`
- `shadow_pipeline.prompt_adapter`
- `shadow_pipeline.cli_plan`

Migration control now has activation steps for the table and prompt shadow adapters.

## Universal property

Local shadow sections compare against the legacy output before any commit gate may affect visible behaviour.
