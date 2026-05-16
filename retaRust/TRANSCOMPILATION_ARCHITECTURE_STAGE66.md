# Stage 66 — Prompt Language Commit Guard

Stage 66 folds the `prompt_language_guard` witness into the prompt shadow commit path.

## Main changes

- `ShadowPromptReport` now carries a full `PromptLanguageGuardReport`.
- `ShadowPromptCommitPolicy` now has `require_prompt_language_guard_ready`.
- `ShadowPromptCommitDecision` now reports prompt-language guard status, language, compiled language, and failed guards.
- `shadow_pipeline.prompt_commit` can no longer commit a prompt shadow plan by default when the prompt-language guard is blocked.
- `rretaprompt_arch_inspect` now emits the prompt commit policy and uses `evaluate_shadow_prompt_commit` with CLI-derived policy.
- New inspect binary: `rreta_arch_prompt_language_commit`.
- New migration step: `step-prompt-language-commit-guard`.
- New probe: `tools/architecture_prompt_language_commit_guard_probe.py`.

## Safety invariant

A prompt shadow plan may become commit-eligible only when:

1. the legacy prompt command is a `reta` command,
2. the Rust prompt execution argv equals the legacy compiled argv,
3. the commit gate allows commit,
4. the prompt-language guard is ready.

The important Stage-55/62 language case remains protected:

```text
reta -language=english -spalten --kontinuum=m
493 -> M Kontinuum (dreizehn)
744 -> Neues M (13) Kontinuum
```

## Checks

Static and Python probes passed, including:

- `architecture_prompt_language_commit_guard_probe.py`
- `architecture_prompt_language_guard_probe.py`
- `architecture_prompt_language_completion_probe.py`
- `architecture_language_sync_guard_probe.py`
- `architecture_language_coverage_guard_probe.py`
- `architecture_migration_step_arity_probe.py`
- `architecture_module_coverage.py`
- `architecture_semantic_surface_audit.py`

A full workspace `cargo check` was not run in this container because `cargo`/`rustc` were unavailable here.
