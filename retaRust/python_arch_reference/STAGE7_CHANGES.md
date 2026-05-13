# Stage 7 changes

This continuation stage extracts the next remaining prompt-stack block into an
explicit architectural layer: completion runtime.

## New modules / architectural pieces

- `reta_architecture/completion_runtime.py`
  - `CompletionRuntimeBundle`
  - `CompletionRuntimeBuilder`
  - `bootstrap_completion_runtime(...)`
  - `sort_completion_key(...)`

## Modified files

- `reta_architecture/facade.py`
  - adds `RetaArchitecture.bootstrap_completion_runtime(...)`
- `reta_architecture/__init__.py`
  - exports the completion-runtime API
- `reta_architecture_probe_py.py`
  - adds `completion-runtime-json`
- `libs/LibRetaPrompt.py`
  - bootstraps and exports `completionRuntime`
- `libs/nestedAlx.py`
  - now consumes the explicit `completionRuntime` bundle
  - no longer imports the previous wide spread of prompt globals
- `retaPrompt.py`
  - now consumes `promptRuntime` / `completionRuntime` directly
  - root prompt completer is wired with `completion_runtime=completionRuntime`
- `tests/test_architecture_refactor.py`
  - adds completion-runtime regression coverage

## Main architectural gain

Completion is now a first-class runtime layer instead of an emergent property of
many re-exported globals.

That narrows the gap between:

- prompt semantics
- completion semantics
- interactive shell bootstrap

and makes the remaining prompt stack much easier to reason about.
