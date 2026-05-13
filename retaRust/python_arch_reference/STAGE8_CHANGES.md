# Stage 8 changes

This continuation stage extracts the remaining prompt-language helper block from
`libs/LibRetaPrompt.py` and adds a real parity matrix against the original
archive.

## New modules / architectural pieces

- `reta_architecture/prompt_language.py`
  - `PromptLanguageBundle`
  - `bootstrap_prompt_language(...)`
  - `PromptModus`
  - prompt-language helper functions formerly defined inside
    `LibRetaPrompt.py`

## Modified files

- `reta_architecture/facade.py`
  - adds `RetaArchitecture.bootstrap_prompt_language(...)`
  - architecture snapshot now includes `prompt_language`
- `reta_architecture/__init__.py`
  - exports the prompt-language API
- `reta_architecture_probe_py.py`
  - adds `prompt-language-json`
- `libs/LibRetaPrompt.py`
  - reduced to a thin compatibility facade
  - now bootstraps `promptRuntime`, `completionRuntime`, `promptLanguage`
- `retaPrompt.py`
  - imports prompt-language helpers from `reta_architecture`
- `libs/nestedAlx.py`
  - imports `PromptModus` and shorthand expansion from `reta_architecture`
- `tests/__init__.py`
  - enables unittest discovery from repo root
- `tests/test_architecture_refactor.py`
  - adds prompt-language regression coverage
- `tests/test_command_parity.py`
  - adds representative old-vs-new command parity checks

## Main architectural gain

The legacy prompt facade is no longer the owner of the prompt language.
It is now just the compatibility export surface.

## Verification summary

- `python -m py_compile $(find . -name '*.py')`
- `python -m unittest -v`
  - `Ran 18 tests in 109.699s`
  - `OK`
- parity matrix against the original archive passes
