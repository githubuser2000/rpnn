# Transcompilation Architecture Stage 11

Stage 11 promotes the Stage-10 shadow pipeline from passive diagnostics to a guarded commit-capable path.

The legacy-compatible `rreta` renderer remains the default.  The Rust architecture renderer can now keep the full shadow output, compute a typed commit decision, and replace the visible output only when the runtime switch gate and parity policy both allow it.

## New / changed Rust architecture pieces

- `ShadowTableReport` now stores the complete `rendered_lines`, not only a preview.
- Added `ShadowCommitPolicy`.
- Added `ShadowCommitDecision`.
- Added `evaluate_shadow_table_commit(...)`.
- Added `ShadowPipelineBundle::table_commit_decision(...)`.
- Added runtime-switch gate `shadow_pipeline.table_commit`.
- Added migration-control step `step-shadow-table-commit`.

## Root integration

- `src/reta_arch_shadow.rs` now returns `ShadowTableRuntimeReport { report, commit }`.
- `src/reta_workflow_py.rs` records `ARCH_SHADOW_TABLE` and `ARCH_SHADOW_COMMIT` diagnostics.
- Visible output is still legacy unless the commit decision allows using the Rust shadow lines.
- Commit requires the dedicated `shadow_pipeline.table_commit` gate and, by default, an equal legacy-vs-shadow diff.  `force` can override mismatch explicitly.

## New FFI export

- `reta_architecture_shadow_commit_policy_json()` exports the default commit policy.

## New probe tool

- `tools/architecture_commit_probe.py` compares legacy, dry-run, and commit-gate runs for selected smoke cases.

## Operational invariant

```text
legacy visible lines == rust shadow lines
        +
shadow_pipeline.table_commit gate allowed
        =>
Rust shadow output may be used as visible output
```

Without that invariant, Stage 11 keeps the old visible output.
