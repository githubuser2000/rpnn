# Continuation refactor changes

This stage continued from the already refactored repository instead of starting over.

## Main changes

- Added `reta_architecture/schema.py`
  - explicit schema extraction layer over `i18n/words.py`
- Added `reta_architecture/semantics_builder.py`
  - extracted parameter-globalisation builder from `reta.py`
- Updated `reta_architecture/facade.py`
  - bootstraps `schema -> topology -> sheaves`
- Updated `reta_architecture/topology.py`
  - now builds from `RetaContextSchema`
- Updated `reta_architecture/sheaves.py`
  - parameter semantics now build from schema
- Updated `reta_architecture/morphisms.py`
  - prompt token splitting now has explicit architecture morphisms
- Updated `reta.py`
  - `storeParamtersForColumns()` now delegates to `ParameterSemanticsBuilder`
- Updated `retaPrompt.py`
  - prompt tokenisation now routes through architecture morphisms
- Updated `reta_architecture_probe_py.py`
  - added `schema-json`
- Updated `reta_domain_probe_py.py`
  - added `schema-json`
- Added `tests/test_architecture_refactor.py`
  - regression coverage for schema extraction and parameter semantics

## Validation

The continuation refactor was validated by:

- `py_compile`
- probe commands (`schema-json`, `mains`)
- a real `reta.Program(['reta.py'])` run
- `unittest` regression tests
- comparison against the previous refactor ZIP for semantic counts


## Stage 3 follow-up

See `STAGE3_CHANGES.md` for the physical split of `i18n/words.py` into bootstrap/context/matrix/runtime modules.
