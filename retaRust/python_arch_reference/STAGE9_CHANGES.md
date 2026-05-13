# Stage 9 changes

This continuation stage extracts the prompt-session / interaction shell from
`retaPrompt.py` into an explicit architecture layer and tightens parity checks
around warning noise from the original archive.

## New modules / architectural pieces

- `reta_architecture/prompt_session.py`
  - `PromptTextState`
  - `PromptLoopSetup`
  - `PromptStoreResult`
  - `PromptSessionBundle`
  - `bootstrap_prompt_session(...)`

## What moved out of `retaPrompt.py`

The following concerns are no longer implemented inside `retaPrompt.py` as the
owner module:

- prompt text state / placeholder state
- prompt-toolkit session + history toggle
- prompt-loop bootstrap / debug / `-befehl` / `-e` setup
- storage / placeholder merge orchestration
- deletion-before-storage helper path
- prompt input wrapper logic

`retaPrompt.py` now consumes these through `promptSession` instead of owning the
logic directly.

## Modified files

- `reta_architecture/facade.py`
  - adds `RetaArchitecture.bootstrap_prompt_session(...)`
  - architecture snapshots now include `prompt_session`
- `reta_architecture/__init__.py`
  - exports the prompt-session API
- `reta_architecture_probe_py.py`
  - adds `prompt-session-json`
- `retaPrompt.py`
  - now bootstraps `promptSession`
  - no longer defines the legacy `TXT` class locally
  - delegates session, storage, delete, and input helpers to the prompt-session layer
- `libs/LibRetaPrompt.py`
  - now also exports `promptSession`
- `tests/test_architecture_refactor.py`
  - adds prompt-session regression coverage
  - asserts that `retaPrompt.py` uses the explicit prompt-session layer
- `tests/test_command_parity.py`
  - ignores environment-specific Python warning noise from the original archive
  - sets `PYTHONWARNINGS=ignore` for old-vs-new comparison runs

## Main architectural gain

The prompt stack is now split into four explicit layers instead of one large
historically grown prompt blob:

1. prompt runtime
2. completion runtime
3. prompt language
4. prompt session / interaction shell

That makes `retaPrompt.py` more of a consumer / orchestrator and less of a
semantic owner.

## Concrete effect on file layout

- `retaPrompt.py`
  - Stage 8: `3488` lines
  - Stage 9: `3159` lines
- new extracted module:
  - `reta_architecture/prompt_session.py`: `526` lines

## Verification summary

Checked directly after the refactor:

- `python -m py_compile $(find . -name '*.py')`
- `python reta_architecture_probe_py.py prompt-session-json`
- `python reta_domain_probe_py.py pair-json Religionen Hinduismus`
- `python - <<'PY' ... reta.Program(['reta.py']) ... PY`
  - `paraDict_len = 4155`
  - `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
  - `kombiReverseDict_len = 46`
  - `kombiReverseDict2_len = 51`
  - `AllSimpleCommandSpalten_len = 554`
- targeted unit tests:
  - `test_prompt_session_layer_is_explicit`
  - `test_libretaprompt_is_thin_compatibility_facade`
  - `test_completion_stack_sources_use_explicit_completion_runtime`
- manual old-vs-new command parity checks with warning noise suppressed:
  - shell: religion/babylon command -> equal output
  - markdown: religion/babylon command -> equal output
  - markdown: garben command -> equal output
  - html: religion command -> equal output (after the existing HTML normalization used by the parity test)

## Important note

This stage does **not** yet extract the heavy domain command semantics inside
`PromptGrosseAusgabe(...)` and its related mathematical helper block. The prompt
interaction shell is now explicit, but the deeper command-dispatch semantics are
still largely inside `retaPrompt.py`.
