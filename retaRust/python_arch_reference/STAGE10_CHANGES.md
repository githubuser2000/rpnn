# Stage 10 changes

This continuation stage builds on Stage 9 and extracts the deep prompt-command execution block out of `retaPrompt.py` into an explicit architecture layer. It also fixes two performance/correctness bottlenecks that became visible after the extraction.

## New architectural layer

- `reta_architecture/prompt_execution.py`
  - `PromptExecutionBundle`
  - `bootstrap_prompt_execution(...)`
  - `configure_prompt_execution(...)`
  - moved legacy-compatible command execution helpers:
    - `PromptGrosseAusgabe(...)`
    - `bruchBereichsManagementAndWbefehl(...)`
    - `retaExecuteNprint(...)`
    - `zeiln1234create(...)`
    - `retaCmdAbstraction_n_and_1pron(...)`
    - fraction/range helpers such as `bruchSpalt(...)`, `createRangesForBruchLists(...)`, `findEqualNennerZaehler(...)`

## What moved out of `retaPrompt.py`

The large domain execution block is no longer owned by `retaPrompt.py`.

`retaPrompt.py` now:

- bootstraps `promptExecution` through `architecture.bootstrap_prompt_execution(...)`
- calls `promptExecution.run_grosse_ausgabe(...)`
- keeps only prompt orchestration, regex expansion, input preparation, storage wrappers, and the top-level prompt loop

Concrete line-count effect:

- Stage 9 `retaPrompt.py`: `3159` lines
- Stage 10 `retaPrompt.py`: `815` lines
- new `reta_architecture/prompt_execution.py`: `2544` lines

This is the largest reduction of the legacy prompt file so far.

## Architecture facade / probe tooling

- `reta_architecture/facade.py`
  - adds `RetaArchitecture.bootstrap_prompt_execution(...)`
  - architecture snapshots now include `prompt_execution`
- `reta_architecture_probe_py.py`
  - adds `prompt-execution-json`

## Correctness fix: semantic builder no longer mutates schema sets

`reta_architecture/semantics_builder.py` previously used `set.pop()` when translating one class of local semantic entries. That mutates the set being read. In a layered architecture where the prompt runtime, tests, and real program builds may touch the same schema-derived structures repeatedly, that is poisonous: repeated semantic builds can become order-dependent.

Stage 10 replaces that destructive read with:

```python
next(iter(spalten_nummer_oder_etc))
```

The semantic builder now reads schema/all-values sets without consuming them.

## Performance fix: universal merge avoids repeated full-graph deepcopy

`reta_architecture/universal.py` still preserves defensive copying for newly merged local values, but no longer deep-copies the entire accumulated data-dictionary graph at every merge step.

The new behavior:

- keeps accumulated dictionaries structurally shared during the fold
- deep-copies each newly merged incoming value
- avoids the previous quadratic full-history cloning pattern

This keeps the pushout-like merge semantics but makes prompt-runtime bootstrap and repeated semantic builds much faster.

## Performance fix: empty modal concatenation short-circuits

`libs/lib4tables_concat.py::concatModallogik(...)` now returns immediately when no modal/generated concept rows are selected:

```python
if len(conceptsRowsSetOfTuple) == 0:
    return self.relitable, rowsAsNumbers
```

That avoids doing a deep table copy and modal preparation work when the selected command does not need modal generated columns. This matters for ordinary commands such as a direct religion column lookup.

## Tests / regression coverage

- `tests/test_architecture_refactor.py`
  - adds prompt-execution layer coverage
  - asserts that `retaPrompt.py` delegates to `promptExecution.run_grosse_ausgabe(...)`
  - asserts that `retaPrompt.py` no longer defines `PromptGrosseAusgabe(...)` or `bruchBereichsManagementAndWbefehl(...)`
  - checks the non-mutating semantic-builder read
  - checks the empty modal-concat fast path
  - switches semantic-count tests to the explicit prompt-runtime semantic view instead of forcing an expensive default table render

## Verification summary

Checked in this stage:

- `python3 -S -m py_compile $(find . -name '*.py')`
- `python3 -S reta_architecture_probe_py.py prompt-execution-json`
- `python3 -S reta_domain_probe_py.py pair-json Religionen Hinduismus`
- in-process smoke run of `reta.Program(...)` for:
  - `-zeilen --vorhervonausschnitt=1-9 -spalten --religionen=hinduismus --breite=50`

Observed stable semantic values:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`

## Main architectural gain

The prompt stack is now split into five explicit layers:

1. prompt runtime
2. completion runtime
3. prompt language
4. prompt session / interaction shell
5. prompt execution / deep command semantics

`retaPrompt.py` is now much closer to what it should be: an interactive orchestrator instead of the owner of all prompt state, prompt language, completion, session, and command execution semantics.

## Remaining hard block

The main remaining prompt-side work is not another huge extraction from `LibRetaPrompt.py`; that file is already thin. The remaining work is to turn the still-local orchestration functions in `retaPrompt.py` into smaller explicit bundles, especially:

- `PromptScope(...)`
- `regExReplace(...)`
- `promptVorbereitungGrosseAusgabe(...)`

Those are now much more isolated than before, because the heavy execution block has moved out.
