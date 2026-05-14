# Transcompilation Architecture Stage 12

Stage 12 extends the guarded activation path from `rreta` table output to
`rretaPrompt` prompt execution planning.

## Main additions

- Added typed prompt legacy-command snapshots in `shadow_pipeline.rs`.
- Added `ShadowPromptCommitPolicy` and `ShadowPromptCommitDecision`.
- Added `evaluate_shadow_prompt_commit(...)` and
  `ShadowPipelineBundle::prompt_commit_decision(...)`.
- Added the new guarded gate `shadow_pipeline.prompt_commit`.
- Added `step-shadow-prompt-commit` to the migration-control plan.
- Added prompt architecture flag stripping for both prompt frontend paths:
  - `src/prompt/app.rs`
  - `crates/retaprompt_commands/src/lib.rs`
- Added guarded prompt commit integration.  The prompt shadow plan can replace
  a compiled prompt command only when the legacy compile result is a `Reta`
  command and the Rust prompt plan produces the same argv.
- Added `rretaprompt_arch_inspect` for prompt shadow/commit inspection.
- Added `tools/architecture_prompt_commit_probe.py` for local smoke checks.
- Added a prompt-side `744` regression parity case.

## Safety rule

Stage 12 does not blindly switch prompt behaviour.  The prompt commit rule is:

```text
explicit commit gate
+ legacy command kind == Reta
+ legacy argv == Rust shadow planned argv
=> prompt may use the Rust planned argv
```

For unsupported command kinds, batch commands, shell/python/math commands, and
mismatched argv, the old compiled prompt command remains active.

## Architectural role

This stage makes `rretaPrompt` follow the same guarded migration shape as
`rreta`:

```text
legacy compile -> shadow prompt plan -> argv comparison -> guarded commit
```

That keeps the visible prompt path stable while giving Rust a concrete commit
point for future prompt execution replacement.
