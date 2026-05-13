# Stage 5 continuation changes

This stage continued from the already refactored repository and focused on the
remaining architectural contradiction in the output/rendering stack.

## Main changes

- Added `reta_architecture/output_semantics.py`
  - `OutputModeSpec`
  - `OutputModeApplication`
  - `RetaOutputSemantics`
  - `bootstrap_output_semantics(...)`
- Updated `reta_architecture/__init__.py`
  - exports the new output-semantics layer
- Updated `reta_architecture/facade.py`
  - `RetaArchitecture` now carries `output_semantics`
  - snapshots include `output_semantics`
  - `sync_tables(...)` auto-detects the current output mode when needed
- Updated `reta_architecture/morphisms.py`
  - renderer morphisms now delegate to the explicit output-semantics registry
  - added canonical output-mode application through the renderer morphisms
- Updated `reta_architecture_probe_py.py`
  - added `output-json`
- Updated `libs/lib4tables.py`
  - output syntax classes now expose architectural metadata (`mode_name`,
    `force_one_table`, `force_zero_width`, `marks_html_or_bbcode`)
- Updated `libs/tableHandling.py`
  - added `OUTPUT_SEMANTICS`
  - added `Tables.outputModeName`
  - output-mode booleans now use the output registry instead of raw class checks
- Updated `reta.py`
  - added `Program.apply_output_mode(...)`
  - removed the hard-coded output selection ladder from CLI parsing
  - now delegates output-mode application to the architecture layer
- Updated executable scripts
  - `grundStrukHtml.py` now uses `build_split_i18n_proxy()`
  - `libs/generate4readme.py` now uses `build_split_i18n_proxy()`
- Extended `tests/test_architecture_refactor.py`
  - verifies output-semantics exposure in the architecture snapshot
  - verifies runtime output-mode detection and application
  - verifies that output dispatch moved out of the old hard-coded branch ladder
  - verifies the remaining executable scripts no longer import `i18n.words`

## Validation

Validated by:

- `py_compile`
- `unittest`
- `reta_architecture_probe_py.py output-json`
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`
- a real `reta.Program(['reta.py'])` run
- runtime output-mode switching checks for `markdown` and `html`

## Architectural result

After this stage, the output stack joins the already extracted schema, input,
and global-semantics layers.

Before stage 5:
- explicit architecture for schema/input/semantics
- implicit output-mode registry hidden across `reta.py`, `tableHandling.py`,
  and syntax classes

After stage 5:
- explicit architecture for schema/input/semantics/output
- canonical output-mode registry
- runtime mode detection and application routed through architecture
- more executable code using split i18n directly instead of the compatibility
  facade
