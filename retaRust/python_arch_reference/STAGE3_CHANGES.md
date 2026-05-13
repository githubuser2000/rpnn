# Stage 3 continuation changes

This stage continued from the already refactored repository and focused on the
largest remaining architectural contradiction: `i18n/words.py` was still a
single giant file.

## Main changes

- Split `i18n/words.py` into:
  - `i18n/words_bootstrap.py`
  - `i18n/words_context.py`
  - `i18n/words_matrix.py`
  - `i18n/words_runtime.py`
- Replaced `i18n/words.py` with a thin compatibility facade
- Preserved the original monolith as `i18n/words_legacy_monolith.py`
- Extended `reta_architecture/schema.py`
  - added `from_words_parts(...)`
  - added `schema_modules` in schema snapshots
- Updated `reta_architecture/facade.py`
  - bootstraps schema from split words modules instead of scraping the facade
- Updated `reta_architecture_probe_py.py`
  - added `module-split-json`
- Updated `reta_domain_probe_py.py`
  - schema help text now reflects split-module extraction
- Extended `tests/test_architecture_refactor.py`
  - validates that split modules are visible and consistent with the facade

## Validation

Validated by:

- `py_compile`
- `unittest`
- probe commands (`schema-json`, `module-split-json`, `pair-json`)
- a real `reta.Program(['reta.py'])` run
- semantic regression counts matching the previous continuation stage
