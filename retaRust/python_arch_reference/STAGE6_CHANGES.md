# Stage 6 changes

This continuation stage removes the next major hidden bootstrap from the
existing refactor: `libs/LibRetaPrompt.py` no longer instantiates a full
`reta.Program(...)` at import time.

## New modules / architectural pieces

- `reta_architecture/prompt_runtime.py`
  - `PromptTablesView`
  - `PromptProgramView`
  - `PromptRuntimeBundle`
  - `PromptRuntimeBuilder`
  - `bootstrap_prompt_runtime(...)`
  - `build_main_parameter_commands(...)`

## Modified files

- `reta_architecture/facade.py`
  - adds `RetaArchitecture.bootstrap_prompt_runtime(...)`
- `reta_architecture/__init__.py`
  - exports the new prompt-runtime API
- `libs/LibRetaPrompt.py`
  - replaces import-time `reta.Program(...)` with `bootstrap_prompt_runtime(...)`
  - keeps legacy globals compatible
- `reta_architecture_probe_py.py`
  - adds `prompt-runtime-json`
- `tests/test_architecture_refactor.py`
  - adds regression tests for the explicit prompt-runtime layer

## Main architectural gain

Prompt bootstrap is now derived from the explicit architecture:

- schema
- semantics builder
- input vocabulary builder

instead of being obtained through a full CLI runtime object.

## Checked after this stage

- `py_compile`
- unit tests
- architecture probe for `prompt-runtime-json`
- domain probe for `Religionen/Hinduismus`
- smoke imports of `LibRetaPrompt` and `nestedAlx`
- runtime smoke test of `reta.Program(['reta.py'])`
