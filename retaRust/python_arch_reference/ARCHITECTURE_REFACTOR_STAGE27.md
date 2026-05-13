# Reta Topology / Morphisms / Universal Properties / Sheaves / Presheaves Refactor

This repository was refactored so that reta no longer hides the intended
architecture only implicitly inside `words.py`, `reta.py`, `retaPrompt.py`, and
`tableHandling.py`.

## What was added in the first refactor stage

### New architecture package

`reta_architecture/`

- `topology.py`
  - symbolic context topology for language, main parameters, sub parameters,
    row semantics, output modes, tags, combination parameters, scopes
- `presheaves.py`
  - raw/local sections for CSVs, translations, assets, and prompt state
- `sheaves.py`
  - canonical parameter semantics sheaf
  - generated columns sheaf
  - table output sheaf
  - HTML reference sheaf
- `morphisms.py`
  - alias, range, prompt, and renderer morphisms
- `universal.py`
  - canonical merge and normalization operators
- `facade.py`
  - `RetaArchitecture.bootstrap(...)`
- `__init__.py`
  - public entry points

### New probe tool

- `reta_architecture_probe_py.py`
  - inspect the architecture bundle directly

## What changed in the continuation refactor stage

### New explicit schema layer

- `reta_architecture/schema.py`
  - extracts the context/schema skeleton from `i18n/words.py`
  - centralises language aliases, main-parameter alias groups, row/output/
    combination domains, scope aliases, tag names, and the parameter/data matrix
  - gives the rest of the architecture a typed schema object instead of direct
    scraping of `words.py`

### New parameter-semantics builder

- `reta_architecture/semantics_builder.py`
  - moves the heavy parameter-globalisation logic out of `reta.py`
  - builds:
    - `paraMainDict`
    - `paraDict`
    - `dataDict`
    - reverse combo lookup dictionaries
    - aggregated "all" values
  - preserves the old semantics while making the glue step reusable and testable

### `facade.py`

- now bootstraps a `RetaContextSchema`
- topology and sheaf layers are now built **from the schema**, not directly from
  `words.py`
- architecture snapshots now include a dedicated `schema` section

### `topology.py`

- now consumes `RetaContextSchema`
- no longer reaches directly into `words.py`

### `sheaves.py`

- parameter semantics sheaf now consumes the schema layer
- repository sheaf bootstrap is now `from_repo(repo_root, schema)`

### `reta.py`

- `storeParamtersForColumns()` was collapsed from its embedded local-cluster
  logic into a thin call to `ParameterSemanticsBuilder`
- the old glue-heavy parameter globalisation logic now lives outside the
  `Program` class
- `Program.paraNdataMatrixAugmented` now exposes the post-"alles" matrix built
  by the architecture layer

### `retaPrompt.py`

- prompt token splitting now routes through `architecture.morphisms.prompt`
- prompt tokenisation is no longer duplicated ad hoc inside the prompt object

### Probe tools

- `reta_architecture_probe_py.py` now supports `schema-json`
- `reta_domain_probe_py.py` now supports `schema-json`

## Practical consequence

reta now has:

1. an explicit **schema layer**
2. an explicit **context topology** over that schema
3. explicit **presheaves / sheaves** over the same context skeleton
4. explicit **morphisms** for prompt, alias, range, and renderer transitions
5. explicit **universal glue** for merge/normalisation
6. an extracted **parameter-semantics builder** instead of burying this inside
   `reta.py`

The old code still exists, but it is now more clearly downstream of a formal
architecture facade.

## Smoke tests performed after the continuation refactor

- `python -m py_compile reta_architecture/*.py reta.py retaPrompt.py reta_domain_probe_py.py reta_architecture_probe_py.py`
- `python reta_architecture_probe_py.py schema-json`
- `python reta_domain_probe_py.py mains`
- `python - <<'PY' ... reta.Program(['reta.py']) ... PY`
- `python -m unittest tests.test_architecture_refactor -v`

## Regression comparison against the previous refactor ZIP

The continuation refactor was checked against the previously generated
`reta_topology_architecture_refactor.zip`.

Compared values matched exactly for a real `reta.Program(['reta.py'])` run:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `kombiReverseDict_len = 46`
- `kombiReverseDict2_len = 51`
- `AllSimpleCommandSpalten_len = 554`

A sample domain-probe result also matched exactly:

- `pair-json Religionen Hinduismus`

So this stage changes architecture placement, not the observed parameter
semantics.

## What changed in the third continuation stage

### `i18n/words.py` was physically split

The previous stages still left one major architectural contradiction in place:
`i18n/words.py` remained a gigantic multi-role file. This stage resolves that by
turning `i18n.words` into a thin compatibility facade and extracting the legacy
monolith into dedicated layers:

- `i18n/words_bootstrap.py`
  - imports, translation bootstrap, debug helpers
- `i18n/words_context.py`
  - language aliases, row/output/combo domains, parameter names, command words,
    and the broader context/schema skeleton
- `i18n/words_matrix.py`
  - `paraNdataMatrix`, `kombiParaNdataMatrix`, `kombiParaNdataMatrix2`
- `i18n/words_runtime.py`
  - runtime/helper classes such as `retapy`, `retaPrompt`, `csvFileNames`,
    `readMeFileNames`, and the final scope/main-command definitions
- `i18n/words_legacy_monolith.py`
  - preserved backup of the old monolithic source for traceability and diffing

The public import path `import i18n.words as i18n` still works. That module now
just re-exports the split modules.

### Schema bootstrap now reads split modules directly

- `reta_architecture/schema.py`
  - now supports `from_words_parts(context_module, matrix_module, runtime_module)`
  - records the module origins inside `schema_modules`
- `reta_architecture/facade.py`
  - now bootstraps the schema from `i18n.words_context`, `i18n.words_matrix`,
    and `i18n.words_runtime`
  - still keeps the compatibility facade metadata from `i18n.words`

This means the architecture layer no longer needs to pretend the monolith is the
source of truth.

### Probe tooling now exposes the split explicitly

- `reta_architecture_probe_py.py`
  - now supports `module-split-json`
- `reta_domain_probe_py.py`
  - schema help text updated to reflect that the schema is extracted from the
    split words modules

### Practical result of stage 3

- `i18n/words.py` shrank from `5431` lines to a `24`-line compatibility facade
- the physical split is real, not merely conceptual
- the architecture facade now consumes the split layers explicitly
- existing callers remain source-compatible

## Smoke tests performed after the third continuation stage

- `python -m py_compile i18n/words.py i18n/words_bootstrap.py i18n/words_context.py i18n/words_matrix.py i18n/words_runtime.py reta_architecture/*.py reta.py retaPrompt.py reta_domain_probe_py.py reta_architecture_probe_py.py tests/test_architecture_refactor.py`
- `python -m unittest tests.test_architecture_refactor -v`
- `python reta_architecture_probe_py.py module-split-json`
- `python reta_domain_probe_py.py pair-json Religionen Hinduismus`
- `python - <<'PY' ... reta.Program(['reta.py']) ... PY`

## Regression comparison after the split

Observed values stayed identical:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `kombiReverseDict_len = 46`
- `kombiReverseDict2_len = 51`
- `AllSimpleCommandSpalten_len = 554`
- `pair-json Religionen Hinduismus -> [217]`

So stage 3 changes the *physical architecture layout* of reta without changing
its observed parameter semantics.

## What changed in the fourth continuation stage

### New explicit input-semantics layer

The next architectural bottleneck after the words split was the still mostly
implicit input layer:

- `libs/center.py` still owned row-range grammar and numeric-selection parsing
- `libs/LibRetaPrompt.py` still rebuilt prompt/completion vocabulary ad hoc
- both modules still behaved as if a single monolithic `i18n.words` object were
  the real source of truth

This stage turns that into an explicit architecture layer:

- `reta_architecture/input_semantics.py`
  - `RowRangeSyntax`
    - explicit representation of row-range grammar such as the multiple-prefix
      (`v`) and the comma-splitting rule for nested bracket expressions
  - `PromptVocabulary`
    - explicit bundle for prompt/completion vocabulary
  - `PromptVocabularyBuilder`
    - derives prompt vocabulary from the architecture schema plus a real
      `reta.Program(...)` semantic state
  - `InputBundle`
    - architecture entry point for input grammar and prompt vocabulary building
- `reta_architecture/split_i18n.py`
  - builds a concrete i18n namespace directly from:
    - `i18n.words_context`
    - `i18n.words_matrix`
    - `i18n.words_runtime`
  - avoids routing these legacy consumers through the `i18n.words`
    compatibility facade

### `RetaArchitecture`

- now contains a dedicated `inputs` layer
- architecture snapshots now expose an `inputs` section
- `reta_architecture_probe_py.py` now supports `inputs-json`

### `libs/center.py`

- no longer imports `i18n.words` directly
- now builds its `i18n` object from the physically split modules through
  `split_i18n.py`
- row-range recognition now delegates to the explicit `RowRangeSyntax` object
  for:
  - integer-range recognition
  - fraction-range recognition
  - comma-splitting / compaction
  - multiple-prefix handling

This means the range grammar is no longer hidden only in regex fragments spread
throughout `center.py`; it now has an explicit architectural home.

### `libs/LibRetaPrompt.py`

- now derives prompt/completion vocabulary through
  `retaProgram.architecture.inputs.build_prompt_vocabulary(...)`
- exports the old globals (`mainParas`, `spalten`, `ausgabeParas`,
  `zeilenParas`, `hauptForNeben`, `befehle`, `befehle2`, ...), but those values
  are now built from an explicit prompt-vocabulary object instead of being
  reconstructed ad hoc in place
- no longer spins up an extra independent `reta.Program(...)` instance just to
  recompute the allowed broken-number set for prompt commands; it now reuses the
  existing `retaProgram`

## Practical result of stage 4

After stage 4, the architecture chain is now:

1. split i18n schema/context modules
2. explicit architecture schema/topology/input layers
3. parameter semantics builder and sheaf layers
4. legacy input modules (`center.py`, `LibRetaPrompt.py`) consuming those layers
   instead of pretending to be the source of truth themselves

The remaining legacy code is still substantial, but the input grammar and prompt
vocabulary are now *downstream* of an explicit architecture layer instead of
being another implicit monolith.

## Smoke tests performed after the fourth continuation stage

- `python -m py_compile reta_architecture/*.py libs/center.py libs/LibRetaPrompt.py reta.py retaPrompt.py reta_domain_probe_py.py reta_architecture_probe_py.py tests/test_architecture_refactor.py`
- `python -m unittest tests.test_architecture_refactor -v`
- `python reta_architecture_probe_py.py inputs-json`
- `python reta_architecture_probe_py.py module-split-json`
- `python reta_domain_probe_py.py pair-json Religionen Hinduismus`
- `python - <<'PY' ... reta.Program(['reta.py']) ... PY`

## Regression comparison after stage 4

Observed values stayed identical:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `AllSimpleCommandSpalten_len = 554`
- `pair-json Religionen Hinduismus -> [217]`

So stage 4 continues the same pattern as the earlier stages: a *real*
architectural move without changing the observed parameter semantics.


## What changed in the fifth continuation stage

### New explicit output-semantics layer

The next remaining architectural contradiction after the input refactor sat in
the output stack:

- `reta.py` still selected output modes through a hard-coded `if/elif` ladder
- `tableHandling.py` still inferred output kinds via direct class comparisons
- the architecture facade had no first-class model of shell/html/bbcode/csv/
  markdown/emacs/nothing output modes

This stage resolves that by introducing:

- `reta_architecture/output_semantics.py`
  - `OutputModeSpec`
  - `OutputModeApplication`
  - `RetaOutputSemantics`
  - `bootstrap_output_semantics(...)`

The new layer centralises:

- canonical output-mode names
- CLI/output aliases
- syntax-class factories
- one-table constraints
- zero-width constraints
- html/bbcode side effects
- runtime detection of the currently active output mode

### `lib4tables.py` now carries output metadata

The syntax classes now declare their architectural identity explicitly:

- `OutputSyntax.mode_name = "shell"`
- `NichtsSyntax.mode_name = "nichts"`
- `csvSyntax.mode_name = "csv"`
- `markdownSyntax.mode_name = "markdown"`
- `emacsSyntax.mode_name = "emacs"`
- `htmlSyntax.mode_name = "html"`
- `bbCodeSyntax.mode_name = "bbcode"`

The table-like output modes also declare whether they force:

- `oneTable = True`
- width `0`

So the runtime no longer needs to rediscover this by repeating ad-hoc branch
logic elsewhere.

### `RetaArchitecture` now exposes output semantics

- `reta_architecture/facade.py`
  - bootstraps `output_semantics`
  - exposes it through `RetaArchitecture.output_semantics`
  - includes it in architecture snapshots
  - auto-detects the current table output mode inside `sync_tables(...)` when no
    explicit output mode is passed

### Renderer morphisms now route through the output layer

- `reta_architecture/morphisms.py`
  - `RendererMorphisms.output_mode_for_tables(...)` now delegates to the
    explicit output-semantics layer
  - `RendererMorphisms.apply_output_mode(...)` was added as the canonical way
    to apply an output mode to a live `Tables` object

### `reta.py` no longer owns the output-mode ladder

- `reta.py`
  - adds `Program.apply_output_mode(...)`
  - removes the old shell/csv/html/bbcode/emacs/markdown selection ladder from
    `parametersToCommandsAndNumbers(...)`
  - delegates mode application to
    `self.architecture.morphisms.renderers.apply_output_mode(...)`

This is the same architectural move used in the previous stages: runtime code
becomes a consumer of an explicit architecture layer instead of being the place
where that architecture is reconstructed ad hoc.

### `tableHandling.py` now uses architectural mode detection

- `libs/tableHandling.py`
  - now bootstraps `OUTPUT_SEMANTICS` once
  - adds `Tables.outputModeName`
  - rewrites:
    - `NichtsOutputYes`
    - `markdownOutputYes`
    - `bbcodeOutputYes`
    - `htmlOutputYes`
    so they depend on the explicit output registry instead of raw `type(...) is ...`
    comparisons

That matters because almost all downstream rendering logic already depends on
those properties. Once they become architectural, a large amount of legacy
rendering code becomes architecturally downstream without needing a full rewrite
in one step.

### Remaining executable scripts no longer need the compatibility facade

The executable stragglers were also cleaned up:

- `grundStrukHtml.py`
- `libs/generate4readme.py`

They now use `reta_architecture.split_i18n.build_split_i18n_proxy()` instead of
importing `i18n.words` directly.

So after stage 5, the executable code paths that still *require* the legacy
compatibility facade are reduced further.

## Practical result of stage 5

After stage 5, reta now has an explicit architecture not only for schema, input
grammar, parameter semantics, and local/global sections, but also for the full
output-mode space.

The important concrete gains are:

1. output-mode identity is explicit and inspectable
2. runtime mode detection is centralised
3. renderer properties now depend on the architecture instead of raw class tests
4. `reta.py` lost one of its bigger hard-coded dispatch ladders
5. two more executable modules no longer depend on `i18n.words`

## Smoke tests performed after the fifth continuation stage

- `python -m py_compile reta_architecture/*.py libs/center.py libs/LibRetaPrompt.py libs/lib4tables.py libs/tableHandling.py libs/generate4readme.py grundStrukHtml.py reta.py reta_architecture_probe_py.py tests/test_architecture_refactor.py`
- `python -m unittest tests.test_architecture_refactor -v`
- `python reta_architecture_probe_py.py output-json`
- `python reta_domain_probe_py.py pair-json Religionen Hinduismus`
- `python - <<'PY' ... reta.Program(['reta.py']) ... program.apply_output_mode('markdown') ... program.apply_output_mode('html') ... PY`

## Regression comparison after stage 5

Observed semantic values stayed identical:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `pair-json Religionen Hinduismus -> [217]`

The new explicit output layer also exposes stable runtime facts:

- `available_modes = ["bbcode", "csv", "emacs", "html", "markdown", "nichts", "shell"]`
- default runtime mode for `reta.Program(['reta.py'])` remains `shell`
- applying `markdown` switches the runtime to `markdown` and forces `oneTable = True`
- applying `html` switches the runtime to `html` and keeps html-specific rendering flags coherent

So stage 5 continues the same pattern as the previous stages: a real
architectural extraction that moves more of reta under explicit topological /
morphism / sheaf-oriented control without changing the observed domain
semantics.

# Continuation stage 6: explicit prompt runtime instead of import-time `reta.Program(...)`

The next remaining architectural knot after stage 5 was the prompt stack.

Even after the schema split, semantics builder extraction, input layer split and
output layer extraction, `libs/LibRetaPrompt.py` still instantiated a full
`reta.Program([sys.argv[0], '-' + nichts])` at import time.

That meant:

- prompt bootstrap still depended on the full runtime program class
- import side effects still contained hidden runtime construction
- `retaPrompt.py` / `nestedAlx.py` still consumed a semantically rich object
  that had been obtained in a historically accidental way instead of through an
  explicit architecture layer

Stage 6 removes that remaining hidden bootstrap and replaces it with an
explicit prompt-runtime architecture.

## New explicit prompt-runtime layer

A new module was added:

- `reta_architecture/prompt_runtime.py`

It introduces:

- `PromptTablesView`
- `PromptProgramView`
- `PromptRuntimeBundle`
- `PromptRuntimeBuilder`
- `bootstrap_prompt_runtime(...)`
- `build_main_parameter_commands(...)`

This layer builds the prompt runtime from the already extracted architecture:

- schema (`RetaContextSchema`)
- input semantics (`PromptVocabularyBuilder`)
- parameter semantics (`ParameterSemanticsBuilder`)

So the prompt layer now consumes the architectural layers created in earlier
stages instead of bypassing them via `reta.Program(...)`.

## What `PromptProgramView` contains

`PromptProgramView` is intentionally small and only contains the pieces that the
prompt/completion stack actually needs:

- `mainParaCmds`
- `paraNdataMatrix`
- `paraNdataMatrixAugmented`
- `paraDict`
- `dataDict`
- `kombiParaNdataMatrix`
- `kombiParaNdataMatrix2`
- `kombiReverseDict`
- `kombiReverseDict2`
- `AllSimpleCommandSpalten`
- `tables.hoechsteZeile`
- `architecture`

So the prompt stack now depends on an explicit semantic view instead of a full
runtime program object.

This is an important shift: the prompt layer is now modelled as an architectural
consumer of a sheaf/morphism/context stack, not as a side effect of normal CLI
execution.

## `RetaArchitecture` now exposes prompt-runtime bootstrap

`reta_architecture/facade.py` gained:

- `RetaArchitecture.bootstrap_prompt_runtime(...)`

That makes prompt bootstrap a first-class architectural operation, parallel to
what earlier stages did for:

- schema
- input semantics
- sheaf synchronisation
- output semantics

## `LibRetaPrompt.py` no longer constructs a full `reta.Program(...)`

`libs/LibRetaPrompt.py` now uses:

- `bootstrap_prompt_runtime(...)`

instead of:

- `reta.Program([sys.argv[0], '-' + i18n.retapy.nichtsWort])`

It still exports the legacy globals (`retaProgram`, `promptVocabulary`,
`mainParas`, `spalten`, `ausgabeParas`, `zeilenParas`, `befehle`, `befehle2`,
...) so the rest of the prompt stack remains compatible.

But the meaning of `retaProgram` has changed:

- before: a full `reta.Program` runtime object created at import time
- now: an explicit `PromptProgramView`

That is the real architectural gain of stage 6.

## Validation logic moved into the prompt-runtime layer

The old `wahl15` consistency check in `LibRetaPrompt.py` depended on the
import-time full program bootstrap.

It now reuses prompt-runtime validation data:

- `promptRuntime.validation['wahl15_missing_values']`
- `promptRuntime.validation['wahl15_valid']`

So even the old integrity assertion now hangs off the explicit prompt-runtime
layer.

## Architecture probe extended

`reta_architecture_probe_py.py` gained:

- `prompt-runtime-json`

This makes the new prompt-runtime layer inspectable in the same style as the
previous architectural stages.

## Tests added for stage 6

The regression suite now additionally checks that:

- the prompt-runtime layer is explicit and inspectable
- `LibRetaPrompt.py` uses `bootstrap_prompt_runtime(...)`
- `LibRetaPrompt.py` no longer contains the old `reta.Program([sys.argv[0], ...])`
  bootstrap
- the exported globals still match the prompt vocabulary
- the prompt runtime matches the previous semantic counts
- `LibRetaPrompt.retaProgram` is now a `PromptProgramView`

## Practical result of stage 6

After stage 6, the prompt stack is no longer bootstrapped by a hidden full CLI
program construction.

The concrete gains are:

1. import-time side effects are reduced
2. prompt semantics are built from the explicit architecture layers
3. `LibRetaPrompt.py` stops being the place where a full runtime program is
   secretly created
4. prompt completion and prompt parsing now depend on an explicit semantic view
   object
5. the prompt layer is now inspectable through architecture tooling

## Smoke tests performed after the sixth continuation stage

- `python -m py_compile reta_architecture/*.py libs/LibRetaPrompt.py reta_architecture_probe_py.py tests/test_architecture_refactor.py`
- `python -m unittest tests.test_architecture_refactor -v`
- `python reta_architecture_probe_py.py prompt-runtime-json`
- `python reta_domain_probe_py.py pair-json Religionen Hinduismus`
- `python - <<'PY' ... import LibRetaPrompt; import nestedAlx ... PY`
- `python - <<'PY' ... reta.Program(['reta.py']) ... architecture.bootstrap_prompt_runtime() ... PY`

## Regression comparison after stage 6

Observed prompt/runtime semantic values stayed identical:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `kombiReverseDict_len = 46`
- `kombiReverseDict2_len = 51`
- `AllSimpleCommandSpalten_len = 554`
- `wahl15_valid = True`
- `pair-json Religionen Hinduismus -> [217]`

So stage 6 continues the same architectural direction as the earlier steps:
remaining runtime bootstrap logic is pulled out of legacy modules and rewritten
as an explicit architectural layer instead of being left as import-time hidden
behaviour.

## What changed in continuation stage 7

### New explicit completion-runtime layer

- `reta_architecture/completion_runtime.py`
  - introduces `CompletionRuntimeBundle`
  - introduces `CompletionRuntimeBuilder`
  - adds `bootstrap_completion_runtime(...)`
  - extracts the completion-/interaction-specific prompt surface from the raw
    prompt runtime into one explicit bundle
  - keeps together:
    - sorted command sets
    - command subsets used by nested completion
    - `hauptForNeben` / parameter-domain command families
    - row/output/combi value suggestions
    - combo-value option maps for galaxy / universe completions

### `facade.py`

- now exposes `RetaArchitecture.bootstrap_completion_runtime(...)`

### `LibRetaPrompt.py`

- now bootstraps `completionRuntime`
- remains the compatibility facade, but no longer has to be the implicit place
  where completion consumers rediscover dozens of separate globals on their own

### `libs/nestedAlx.py`

- no longer imports a long list of completion globals from `LibRetaPrompt`
- now consumes one explicit `completionRuntime` bundle plus the remaining
  shorthand-expansion helper
- recursive nested completers now carry the explicit completion runtime forward
- combo-/spalten-value completion now builds from bundle-owned data instead of
  mutating shared global dictionaries

### `retaPrompt.py`

- no longer imports prompt vocabulary globals from `nestedAlx.py`
- now uses:
  - `promptRuntime.program`
  - `completionRuntime.start_commands(...)`
  - `completionRuntime.befehle2`
- root prompt completion is now instantiated with an explicit
  `completion_runtime=completionRuntime`

### Probe / regression additions

- `reta_architecture_probe_py.py`
  - adds `completion-runtime-json`
- `tests/test_architecture_refactor.py`
  - adds regression coverage for the completion-runtime layer
  - checks that `nestedAlx.py` and `retaPrompt.py` now depend on the explicit
    completion bundle instead of the previous broad cross-module import surface

### Main architectural gain in this stage

The prompt bootstrap from stage 6 is now complemented by an explicit completion
bootstrap:

- stage 6 removed the hidden `reta.Program(...)` bootstrap from `LibRetaPrompt`
- stage 7 removes the next broad implicit layer: the nested completion stack now
  hangs on a typed completion bundle instead of scattered globals

This means that prompt execution and prompt completion are now separate,
inspectable architectural layers.

## What changed in continuation stage 8

### New explicit prompt-language layer

- `reta_architecture/prompt_language.py`
  - introduces `PromptLanguageBundle`
  - introduces `bootstrap_prompt_language(...)`
  - moves the remaining shorthand-/token-/command-language helpers out of
    `libs/LibRetaPrompt.py`
  - now owns:
    - `PromptModus`
    - `custom_split(...)`
    - `custom_split2(...)`
    - `isReTaParameter(...)`
    - `is15or16command(...)`
    - `stextFromKleinKleinKleinBefehl(...)`
    - `verifyBruchNganzZahlCommaList(...)`
    - `verifyBruchNganzZahlBetweenCommas(...)`
    - `verkuerze_dict(...)`

### `LibRetaPrompt.py` is now a real compatibility facade

- old size: `543` lines
- new size: `64` lines
- it now only:
  - bootstraps `promptRuntime`
  - bootstraps `completionRuntime`
  - bootstraps `promptLanguage`
  - re-exports legacy globals for compatibility
- it no longer defines the prompt language itself

### Direct consumers now import prompt language from architecture

- `retaPrompt.py`
  - imports prompt-language helpers from `reta_architecture`
  - keeps only runtime/global compatibility imports from `LibRetaPrompt`
- `libs/nestedAlx.py`
  - imports `PromptModus` and shorthand expansion from `reta_architecture`
  - keeps only `completionRuntime` from `LibRetaPrompt`

### New probe surface

- `reta_architecture_probe_py.py`
  - adds `prompt-language-json`

### Stronger regression / parity verification

- `tests/__init__.py`
  - enables `python -m unittest -v` discovery from repo root
- `tests/test_architecture_refactor.py`
  - adds prompt-language regression coverage
  - checks that `LibRetaPrompt.py` is now a thin compatibility facade
  - checks the new prompt-language import surface in `retaPrompt.py` and
    `nestedAlx.py`
- `tests/test_command_parity.py`
  - runs a representative command matrix against the original archive and the
    refactored repo
  - compares:
    - shell output
    - markdown output
    - html output (with lightweight normalization)
  - uses stub modules for `rich`, `bbcode`, `html2text` so parity can be tested
    even in stripped environments

### Verified in this stage

- `python -m py_compile $(find . -name '*.py')`
- `python -m unittest -v`
  - `Ran 18 tests in 109.699s`
  - `OK`
- `python reta_architecture_probe_py.py prompt-language-json`
- `python reta_domain_probe_py.py pair-json Religionen Hinduismus`
- smoke-run of `reta.Program(['reta.py'])`

### Observed stable semantics

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `AllSimpleCommandSpalten_len = 554`
- `pair-json Religionen Hinduismus -> [217]`

### Main architectural gain in this stage

The remaining prompt language is no longer hidden in a legacy compatibility
module. The stack is now split into three explicit layers:

1. prompt runtime
2. completion runtime
3. prompt language

That is the first point where `LibRetaPrompt.py` stops being a semantic owner
and becomes mostly what it should be: a backward-compatibility export facade.

## What changed in the ninth continuation stage

### New explicit prompt-session layer

The next remaining architectural bottleneck after the prompt-language split was
still inside `retaPrompt.py`: the module still owned prompt text state,
prompt-toolkit session creation, loop bootstrap, placeholder/storage handling,
and deletion/input helper paths.

This stage extracts that shell into:

- `reta_architecture/prompt_session.py`
  - `PromptTextState`
  - `PromptLoopSetup`
  - `PromptStoreResult`
  - `PromptSessionBundle`
  - `bootstrap_prompt_session(...)`

### `retaPrompt.py` is no longer the owner of prompt state/session logic

`retaPrompt.py` now:

- bootstraps `promptSession` from the explicit architecture layer
- uses `PromptTextState` from the architecture package instead of defining the
  old `TXT` class locally
- delegates:
  - session/history creation
  - prompt-loop bootstrap
  - placeholder/storage merge orchestration
  - deletion-before-storage logic
  - prompt input handling

The file still owns the deeper domain command semantics, but no longer the
prompt-session shell.

### `facade.py` / probe tooling

- `RetaArchitecture` now supports `bootstrap_prompt_session(...)`
- architecture snapshots now contain a dedicated `prompt_session` section
- `reta_architecture_probe_py.py` now supports `prompt-session-json`

### `LibRetaPrompt.py`

The compatibility facade now also exports `promptSession`, so external callers
that still depend on the historical prompt facade can reach the new session
layer without re-bootstrap logic.

### Practical result of stage 9

- `retaPrompt.py` shrank from `3488` lines (stage 8) to `3159` lines
- prompt state / session / loop setup / storage shell is now explicit
- the prompt stack is now split into:
  1. prompt runtime
  2. completion runtime
  3. prompt language
  4. prompt session

### Verification after stage 9

Observed values remained stable for a real `reta.Program(['reta.py'])` run:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `kombiReverseDict_len = 46`
- `kombiReverseDict2_len = 51`
- `AllSimpleCommandSpalten_len = 554`
- `pair-json Religionen Hinduismus -> [217]`

Representative old-vs-new command comparisons also matched for shell,
markdown, and HTML outputs after suppressing environment-specific warning noise
from the original archive.

So stage 9 changes the prompt architecture placement without changing the
observed command semantics for the checked cases.

## What changed in the tenth continuation stage

Stage 10 extracts the deep prompt-command execution block from `retaPrompt.py` into a new explicit architecture layer:

- `reta_architecture/prompt_execution.py`
  - `PromptExecutionBundle`
  - `bootstrap_prompt_execution(...)`
  - `configure_prompt_execution(...)`
  - legacy-compatible execution functions such as `PromptGrosseAusgabe(...)`, `bruchBereichsManagementAndWbefehl(...)`, and `retaExecuteNprint(...)`

`retaPrompt.py` now bootstraps `promptExecution` through `RetaArchitecture` and calls `promptExecution.run_grosse_ausgabe(...)`. It no longer defines the large `PromptGrosseAusgabe(...)` function itself.

### Concrete file-size result

- Stage 9 `retaPrompt.py`: `3159` lines
- Stage 10 `retaPrompt.py`: `815` lines
- new `reta_architecture/prompt_execution.py`: `2544` lines

This turns `retaPrompt.py` from a prompt/session/language/execution monolith into a much thinner orchestration module.

### Facade and probe updates

- `RetaArchitecture.bootstrap_prompt_execution(...)` was added
- architecture snapshots now contain `prompt_execution`
- `reta_architecture_probe_py.py` now supports `prompt-execution-json`

### Correctness/performance fixes discovered during extraction

Two lower-level problems became visible once the prompt execution layer was isolated:

1. `reta_architecture/semantics_builder.py` no longer uses `set.pop()` while reading schema/all-values sets. It now uses a non-mutating `next(iter(...))` read, so repeated semantic builds are not order-dependent.
2. `reta_architecture/universal.py` no longer deep-copies the entire accumulated data-dictionary graph on every merge step. It keeps accumulated dictionaries structurally shared during the fold and deep-copies newly merged incoming values.
3. `libs/lib4tables_concat.py::concatModallogik(...)` now short-circuits immediately when no modal/generated concept rows are selected, avoiding unnecessary deep table copying for ordinary commands.

### Verification after stage 10

Checked directly:

- `python3 -S -m py_compile $(find . -name '*.py')`
- `python3 -S reta_architecture_probe_py.py prompt-execution-json`
- `python3 -S reta_domain_probe_py.py pair-json Religionen Hinduismus`
- in-process smoke run of `reta.Program(...)` for a direct `Religionen/Hinduismus` column lookup

Observed stable values:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`

### Stage-10 architecture state

The prompt stack is now split into five explicit layers:

1. prompt runtime
2. completion runtime
3. prompt language
4. prompt session
5. prompt execution

The next remaining hard prompt-side block is the local orchestration still inside `retaPrompt.py`, especially `PromptScope(...)`, `regExReplace(...)`, and `promptVorbereitungGrosseAusgabe(...)`.

## Stage 11 – Prompt-Vorbereitung und Paketintegrität

Stage 11 zieht die Prompt-Vorbereitung aus `retaPrompt.py` in `reta_architecture/prompt_preparation.py` heraus. Damit ist der Prompt-Stack jetzt weiter geschichtet:

- Prompt-Runtime
- Completion-Runtime
- Prompt-Language
- Prompt-Session
- Prompt-Execution
- Prompt-Preparation

Außerdem gibt es mit `reta_architecture/package_integrity.py` jetzt ein explizites Manifest/Audit-Werkzeug, damit Archive nicht mehr nur nach Größe beurteilt werden müssen. Das Manifest ignoriert `__pycache__` und `.pyc`, prüft Pflichtdateien und kann per `reta_architecture_probe_py.py package-integrity-json` ausgegeben werden.

`retaPrompt.py` enthält die großen Vorbereitungsfunktionen `regExReplace(...)` und `promptVorbereitungGrosseAusgabe(...)` nicht mehr selbst, sondern delegiert an `architecture.bootstrap_prompt_preparation(...)`.

---

# Stage 12 – Prompt-Interaktionscontroller

Stage 12 zieht die verbleibende interaktive Prompt-Orchestrierung aus `retaPrompt.py` in eine explizite Architekturschicht:

- `reta_architecture/prompt_interaction.py`
  - `PromptInteractionBundle`
  - `bootstrap_prompt_interaction(...)`

Damit besitzt `retaPrompt.py` den Prompt-Loop nicht mehr selbst. Es bootstrapt nur noch `promptInteraction` und stellt historische Funktionsnamen als dünne Wrapper bereit.

`retaPrompt.py` wurde dadurch von Stage 11 ca. 454 Zeilen auf ca. 124 Zeilen reduziert.

Die neue Schicht koordiniert:

- Prompt-Loop-Setup
- Speichern-/Löschen-/Ausgabe-Modi
- Weiterreichen der Eingabe an Prompt-Preparation und Prompt-Execution
- Synchronisierung der historischen Modul-Globals für alte Starter und Importe

Neue Prüfbefehle:

```bash
python -S reta_architecture_probe_py.py prompt-interaction-json
python -S -m unittest tests.test_architecture_refactor -v
python -S -m unittest tests.test_command_parity -v
python -S -m unittest -v
```

Die gesamte Test-Suite lief in Stage 12 mit 26 Tests erfolgreich durch. Die repräsentative Command-Parity-Matrix gegen das Original lief ebenfalls erfolgreich.

---

# Stage 13 – Column Selection und Table Generation

Stage 13 zieht den nächsten großen Block aus `reta.py`: das Spalten-Bucket-Schema und den Tabellen-/Generated-Column-/Concat-/Kombi-Gluing-Pfad.

Neue Module:

- `reta_architecture/column_selection.py`
- `reta_architecture/table_generation.py`

`ColumnSelectionBundle` besitzt jetzt das Legacy-kompatible `SpaltenTyp`-Schema und erzeugt die 24 positiven/negativen Spalten-Buckets. `TableGenerationBundle` übernimmt das Ankleben externer CSV-Sektionen, generierte Spalten und Kombi-Joins. `reta.py` delegiert an diese Schichten und ist von ca. 1613 auf ca. 1333 Zeilen gefallen.

Neue Probe-Kommandos:

```bash
python reta_architecture_probe_py.py column-selection-json
python reta_architecture_probe_py.py table-generation-json
```

Geprüft:

- `py_compile`: OK
- `tests.test_architecture_refactor`: 28 Tests, OK
- `tests.test_command_parity`: 1 Paritätsmatrix-Test, OK
- `python -m unittest -v`: 29 Tests, OK
- stabile Kernwerte: `paraDict_len = 4155`, `Religionen / Hinduismus -> [217]`

Der Architekturgewinn: `reta.py` orchestriert den Tabellenbau nicht mehr selbst. Die Spaltenauswahl ist jetzt eine explizite lokale Kontextschicht, und der Tabellenaufbau liegt als universelle Gluing-Schicht in `reta_architecture/table_generation.py`.

---

# Stage 14: Parameter-Runtime-Schicht

Stage 14 zieht die bisher größte verbliebene Laufzeitlogik aus `reta.py` heraus: die Shell-/CLI-Parametersemantik.

Neu ist `reta_architecture/parameter_runtime.py`. Diese Schicht besitzt die Transformation von CLI-Argumenten zu Zeilen-Sektionen, Spalten-Buckets, Ausgabeparametern und Obergrenzen. Sie ist bewusst Legacy-kompatibel und arbeitet noch auf dem bestehenden `Program`-Objekt, aber der Program-Kern besitzt die Algorithmen nicht mehr selbst.

Architektonisch ist das die passende Einordnung:

- **Morphismen:** CLI-Text → kanonische Zeilen-/Spalten-/Ausgabesektionen
- **Prägarben:** lokale, noch nicht globalisierte Benutzerauswahl
- **Garben/Gluing:** anschließende Übergabe an `column_selection`, `table_generation` und `prepare4out`
- **Universelle Eigenschaften:** Normalisierung positiver/negativer Spaltenauswahl über `normalize_column_buckets(...)`

`reta.py` ist dadurch auf ca. 548 Zeilen gefallen. Die noch verbliebene Datei ist jetzt wesentlich stärker eine Program-Fassade als ein semantischer Monolith.

---

# Stage 15: Program-Workflow-Schicht

Stage 15 zieht den verbleibenden Top-Level-Ablauf aus `reta.py` in eine eigene Architekturschicht: `reta_architecture/program_workflow.py`.

Neu ist `ProgramWorkflowBundle`. Diese Schicht koordiniert jetzt:

- Haupttabelle laden
- sprachspezifische Motive-Spalte anwenden
- positive und negative Parametersektionen lesen
- Parameter-Semantik globalisieren
- Spaltenauswahl binden
- generierte Tabellen und Zusatzspalten bauen
- `prepare4out(...)` ausführen
- Kombi-Tabellen joinen
- finale Ausgabe erzeugen

`reta.py` ist dadurch von ca. 548 Zeilen auf ca. 200 Zeilen gefallen. Die Datei enthält weiterhin die historische Klasse `Program`, aber die großen Workflow-Algorithmen sind nicht mehr dort zuhause. Sie delegiert an `architecture.bootstrap_program_workflow(...)`.

Architektonisch ist das der bisher sauberste Schnitt für den CLI-Gesamtprozess: Der Program-Workflow ist jetzt der explizite universelle Glue-Knoten zwischen `parameter_runtime`, `column_selection`, `table_generation`, `prepare4out`, Kombi-Joins und Output-Semantik.

Neue Probe-Kommandos:

```bash
python -S reta_architecture_probe_py.py program-workflow-json
python -S reta_architecture_probe_py.py package-integrity-json
```

Geprüft:

- `py_compile`: OK
- `tests.test_architecture_refactor`: 30 Tests, OK
- `python -S -m unittest -v`: 31 Tests, OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- kleine Original-vs-Stage-15-Parität für Shell und Markdown: byte-identisch

Stabile Kernwerte:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`

Der nächste sinnvolle Architekturblock liegt jetzt nicht mehr in `reta.py`, sondern in den großen Tabellen-/Prepare-/Concat-Klassen unter `libs/`.


---

# Stage 16: Table-Preparation-Schicht

Stage 16 zieht die Ausgabevorbereitung aus `libs/lib4tables_prepare.py` in eine eigene Architekturschicht: `reta_architecture/table_preparation.py`.

Neu ist `TablePreparationBundle`. Diese Schicht besitzt jetzt die expliziten Morphismen für:

- globale Zeilenauswahl aus positiven/negativen lokalen Zeilenbedingungen
- Tabellenzeilen-Vorbereitung
- Header-/Tag-Gluing für normale, generierte und Kombi-Spalten
- Zellenumbruch
- Haupttabellen-Vorbereitung
- Kombi-Tabellen-Vorbereitung
- `lastLineNumber`-Ermittlung für die spätere Tabellen-Generation

`libs/lib4tables_prepare.py` bleibt als Kompatibilitätsfassade bestehen, delegiert aber die früher dort eingebetteten Kernstücke an die neue Schicht. `program_workflow.py` und `table_generation.py` verwenden die neue Schicht direkt.

Architektonisch ist das wichtig, weil der alte `prepare4out(...)`-Block genau dort sitzt, wo Reta lokale Auswahlbedingungen in eine globale Ausgabetabelle verklebt. Das ist kein bloßer Renderer, sondern ein echter lokale-zu-globale-Knoten.

Neue Probe-Kommandos:

```bash
python -S reta_architecture_probe_py.py table-preparation-json
python -S reta_architecture_probe_py.py package-integrity-json
```

Geprüft:

- `py_compile`: OK
- `tests.test_architecture_refactor`: 32 Tests, OK
- `tests.test_command_parity`: 1 Paritätsmatrix-Test, OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- echter `reta.py`-Smoke-Test: OK

Stabile Kernwerte:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`

Der nächste große Restblock liegt in `libs/lib4tables_concat.py`.

---

# Stage 17: Generated-Column-Morphismenschicht

Stage 17 beginnt die echte Entflechtung von `libs/lib4tables_concat.py`.

Neu ist `reta_architecture/generated_columns.py`. Diese Schicht besitzt jetzt die einfachen generierten Spaltenmorphismen, die vorher direkt in `Concat` lagen:

- `concatLovePolygon(...)`
- `gleichheitFreiheitVergleich(...)`
- `geistEmotionEnergieMaterieTopologie(...)`
- `concatGleichheitFreiheitDominieren(...)`
- `concatGeistEmotionEnergieMaterieTopologie(...)`
- `concatPrimCreativityType(...)`
- `concatMondExponzierenLogarithmusTyp(...)`

`libs/lib4tables_concat.py` behält kompatible Wrapper, aber die eigentliche Semantik dieser einfachen Generatoren lebt jetzt in der Architekturschicht. Zusätzlich gibt es eine `GeneratedColumnRegistry`, die diese Morphismen mit Trigger-Spalten, Tags und Beschreibung abfragbar macht.

`table_generation.py` nutzt die neue Schicht für die herausgezogenen Generatoren. `RetaArchitecture` besitzt jetzt `generated_columns`, `snapshot()` zeigt diese Schicht, und `reta_architecture_probe_py.py generated-columns-json` macht sie direkt inspizierbar.

Architektonisch ist das der erste saubere Schnitt im Concat-Restblock: Generated Columns werden als eigene Morphismen behandelt, nicht mehr als implizite Methoden eines großen Tabellen-Hilfsobjekts. Sie erzeugen abgeleitete Sektionen und kleben diese wieder in die globale Tabellensektion.

Geprüft:

- `py_compile`: OK
- `python -S -m unittest -v`: 34 Tests, OK
- `reta_architecture_probe_py.py generated-columns-json`: OK
- `reta_architecture_probe_py.py table-generation-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- echter `reta.py`-Smoke-Test: OK

Stabile Kernwerte:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`

Noch offen ist der schwerere Concat-Rest: Modal-Logik, Primzahlkreuz-Pro/Contra, Metakonkret-Theorie-Abstrakt, Bruch-/CSV-Gluing und `readConcatCsv(...)`.

---

# Stage 18 Änderungen

Stage 18 baut direkt auf Stage 17 auf und setzt den Concat-Refactor fort. Der Schwerpunkt lag diesmal auf den schweren Generated-Column- und Meta-Column-Pfaden, ohne neu anzufangen.

## Neu bzw. deutlich erweitert

- `reta_architecture/generated_columns.py`
  - besitzt jetzt die schweren Generated-Column-Morphismen:
    - `concatVervielfacheZeile(...)`
    - `concatModallogik(...)`
    - `concat1RowPrimUniverse2(...)`
    - `concat1PrimzahlkreuzProContra(...)`
  - `GeneratedColumnRegistry` kennt jetzt **9** Generated-Column-Morphismen.
  - Runtime-Abhängigkeiten werden lazy geladen, damit die Architekturschicht importierbar bleibt.

- `reta_architecture/meta_columns.py`
  - neue explizite Schicht für Meta-/Konkret-/Theorie-/Abstrakt-Spaltenmorphismen.
  - enthält u. a.:
    - `spalteMetaKontretTheorieAbstrakt_etc_1(...)`
    - `spalteMetaKontretTheorieAbstrakt_etc(...)`
    - `spalteFuerGegenInnenAussenSeitlichPrim(...)`
    - `readOneCSVAndReturn(...)`
    - `findAllBruecheAndTheirCombinations(...)`
  - `MetaColumnsBundle` macht diese Morphismen abfragbar.

- `libs/lib4tables_concat.py`
  - ist jetzt weitgehend eine Kompatibilitätsfassade für herausgezogene Concat-/Generated-/Meta-Morphismen.
  - Umfang fiel auf ca. **677 Zeilen**.
  - Die Meta-Methoden wurden absichtlich als `*args/**kwargs`-Wrapper angebunden, weil mehrere interne Aufrufe historisch unterschiedliche Signaturen verwenden. Dadurch bleiben alte Aufrufpfade kompatibel, während die Semantik in der neuen Schicht liegt.

- `reta_architecture/facade.py`, `reta_architecture/__init__.py`, `reta_architecture/package_integrity.py`
  - kennen jetzt die Meta-Column-Schicht.
  - `RetaArchitecture.snapshot()` enthält jetzt auch `meta_columns`.

- `reta_architecture_probe_py.py`
  - neuer Befehl:
    - `meta-columns-json`

- `tests/test_architecture_refactor.py`
  - neue Regression für `MetaColumnsBundle`.

## Wichtigste Architekturwirkung

Stage 18 trennt zwei Familien, die vorher im selben Concat-Monolithen lagen:

1. **Generated Columns**: abgeleitete Spalten aus vorhandenen Zahlen-, Struktur- und Konzeptbedingungen.
2. **Meta Columns**: Meta/Konkret/Theorie/Abstrakt- und Prim-Innen/Außen/Seitlich-Klassifikationen.

Damit wird `lib4tables_concat.py` nicht mehr als semantischer Besitzer dieser Logik behandelt, sondern als Legacy-kompatibler Adapter.

## Geprüft

- `py_compile`: OK
- `python -S -m unittest -v`: **35 Tests, OK**
- `reta_architecture_probe_py.py generated-columns-json`: OK, Registry mit **9** Morphismen
- `reta_architecture_probe_py.py meta-columns-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- Smoke-Test `Religionen/sternpolygon`: OK
- Meta-Column-Smoke-Test `--universummetakonkret=meta`: OK

## Stabile Kernwerte

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`

## Noch offen

Der nächste harte Block ist jetzt die verbleibende CSV-/Bruch-/`readConcatCsv(...)`-Logik in `libs/lib4tables_concat.py` sowie die spätere weitere Zerlegung des mittlerweile großen `generated_columns.py`.

---

# Stage 19 – CSV-/Bruch-Gluing-Schicht

Stage 19 zieht die verbliebene CSV-/Bruch-/`readConcatCsv(...)`-Logik aus `libs/lib4tables_concat.py` in eine eigene Architekturschicht.

## Neue Schicht: `reta_architecture/concat_csv.py`

Diese Schicht modelliert CSV-Tabellen als lokale Prägarben-Sektionen, die über `readConcatCsv(...)` in die globale Tabellen-Sektion geklebt werden. Zusätzlich liegen dort die Bruch-/Fraction-Helfer, die Zahl-zu-Paar-Relationen für generierte Spalten erzeugen.

Sie enthält:

- `ConcatCsvSpec`
- `ConcatCsvBundle`
- `bootstrap_concat_csv(...)`
- die `readConcatCsv(...)`-Familie
- Bruch-/Paar-Konverter wie `convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction(...)`
- `combineDicts(...)`

## Legacy-Fassade

`libs/lib4tables_concat.py` bleibt kompatibel, delegiert aber die CSV-/Bruch-Morphismen an `reta_architecture.concat_csv`.

## Integration

- `RetaArchitecture` besitzt jetzt `concat_csv`.
- `RetaArchitecture.snapshot()` enthält jetzt `concat_csv`.
- `TableGenerationBundle` nutzt jetzt `ConcatCsvBundle` explizit.
- `reta_architecture_probe_py.py concat-csv-json` macht die Schicht inspizierbar.

## Zusätzliche Korrektur

Ein alter Fehler in `parameter_runtime.py` wurde sichtbar und repariert: Bruch-/CSV-Parameterpfade referenzierten noch `Program.*`. Diese Stellen nutzen jetzt `type(self).*`, wodurch gebrochen-rationale CSV-Gluing-Kommandos wieder korrekt laufen.

---

# Stage 20 – Tabellen-Ausgabe als explizite Output-Morphismenschicht

Stage 20 baut direkt auf Stage 19 auf. Der nächste große Restblock war die alte verschachtelte Klasse `Tables.Output` in `libs/tableHandling.py`.

## Neue Schicht: `reta_architecture/table_output.py`

Diese Schicht modelliert die Tabellen-Ausgabe als expliziten Renderer-Morphismus über der globalen Tabellen-Sektion. Sie enthält:

- `TableOutput`
- `TableOutputBundle`
- `bootstrap_table_output(...)`

Die bisherige Rendering-Logik aus `Tables.Output` wurde dorthin verschoben. Dadurch ist `tableHandling.py` nicht mehr Besitzer der konkreten Ausgabelogik, sondern delegiert an die Architekturschicht.

## Legacy-Fassade

`libs/tableHandling.py` bleibt kompatibel, erzeugt aber jetzt:

```python
self.getOut = TableOutput(self, Txt)
```

Die verschachtelte Klasse `Output` existiert dort nicht mehr. Der Umfang von `tableHandling.py` fiel auf ca. **881 Zeilen**.

## Integration

- `RetaArchitecture` besitzt jetzt `table_output`.
- `RetaArchitecture.bootstrap_table_output(...)` lädt die Schicht lazy.
- `RetaArchitecture.snapshot()` enthält jetzt `table_output`.
- `reta_architecture_probe_py.py table-output-json` macht die Schicht inspizierbar.
- `reta_architecture/package_integrity.py` behandelt `reta_architecture/table_output.py` als Pflichtdatei.

## Importzyklus-Korrektur

Eine direkte Top-Level-Einbindung hätte diesen Zyklus erzeugt:

```text
center -> reta_architecture -> facade -> table_output -> center
```

Deshalb lädt die Fassade `table_output` bewusst lazy. Das ist kein Workaround, sondern die richtige architektonische Trennung: Die Architektur kennt die Schicht, aber sie erzwingt keinen frühen Runtime-Import.

## Geprüft

- `py_compile`: OK
- `reta_architecture_probe_py.py table-output-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- Smoke-Test `Religionen/sternpolygon`: OK
- `python -B -S -m unittest tests.test_architecture_refactor -q`: **37 Tests, OK**
- `python -B -S -m unittest tests.test_command_parity -q`: **1 Paritätsmatrix-Test, OK**
- `python -B -S -m unittest -q`: **38 Tests, OK**

## Manuelle Parität

- Shell-Ausgabe: byte-identisch
- Markdown-Ausgabe: byte-identisch
- Bruch-/CSV-Gluing-Fall: byte-identisch
- HTML-Ausgabe: identisch nach Normalisierung

## Noch offen

Der nächste harte Block ist weiterhin `libs/tableHandling.py`, diesmal aber unterhalb der Output-Schicht:

- `Tables.Combi` als eigene Kombi-Join-/Relationen-Schicht
- `Tables.Maintable.createSpalteGestirn(...)` als Generated-Column-Morphismus
- `Tables` langfristig als dünne globale Tabellen-Sektionsfassade

---

# Stage 21: Kombi-Join-Schicht und Gestirn-Morphismus

Stage 21 zieht den nächsten Block aus `libs/tableHandling.py` heraus.

## Kombi-Join als eigene Morphismenschicht

Neu ist:

- `reta_architecture/combi_join.py`
- `KombiJoin`
- `KombiJoinBundle`
- `bootstrap_combi_join(...)`

Damit sind die alten Kombi-Operationen nicht mehr als verschachtelte Klasse in `tableHandling.py` versteckt. Die Schicht besitzt die Join-/Relationenmorphismen:

- `prepareTableJoin(...)`
- `removeOneNumber(...)`
- `tableJoin(...)`
- `prepare_kombi(...)`
- `readKombiCsv(...)`
- `kombiNumbersCorrectTestAndSet(...)`

`Tables.Combi` bleibt als Kompatibilitätsalias erhalten:

```python
Combi = KombiJoin
```

Dadurch laufen alte Aufrufpfade weiter, aber die Architektur besitzt die Semantik jetzt explizit.

## `createSpalteGestirn` als Generated-Column-Morphismus

`createSpalteGestirn(...)` wurde aus `Tables.Maintable` herausgelöst und in `reta_architecture/generated_columns.py` überführt.

`GeneratedColumnsBundle` besitzt jetzt:

```python
create_spalte_gestirn(...)
```

Die alte Methode `Tables.Maintable.createSpalteGestirn(...)` ist nur noch ein Wrapper.

## Integration

- `RetaArchitecture` besitzt jetzt `combi_join`.
- `RetaArchitecture.bootstrap_combi_join(...)` bootstrapt die Schicht.
- `RetaArchitecture.snapshot()` enthält jetzt `combi_join`.
- `TableGenerationBundle.snapshot()` enthält jetzt `combi_join`.
- `reta_architecture_probe_py.py combi-join-json` macht die Schicht inspizierbar.
- `reta_architecture/package_integrity.py` behandelt `reta_architecture/combi_join.py` als Pflichtdatei.

## Runtime-Robustheit

Zusätzlich wurden lokale Fallbacks ergänzt:

- `libs/center.py`: Fallbacks für `rich`, `textwrap2`, `pyphen`
- `reta_architecture/prompt_session.py`: Fallbacks für `prompt_toolkit`

Das ist wichtig für Architektur-Probes und CI-artige Tests mit `python -S`, ohne den normalen Projektbetrieb zu verändern.

## Wirkung auf `tableHandling.py`

`libs/tableHandling.py` fiel von ca. **881 Zeilen** auf ca. **275 Zeilen**. Nach Output, Kombi-Join und Gestirn-Morphismus ist `tableHandling.py` deutlich näher an einer reinen Tabellen-Fassade.

## Geprüft

- `py_compile`: OK
- `reta_architecture_probe_py.py combi-join-json`: OK
- `reta_architecture_probe_py.py generated-columns-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- Smoke-Test `Religionen/sternpolygon`: OK
- `python -B -S -m unittest tests.test_architecture_refactor -v`: **39 Tests, OK**
- `python -B -S -m unittest tests.test_command_parity -v`: **1 Paritätsmatrix-Test, OK**
- `python -B -S -m unittest -v`: **40 Tests, OK**

## Noch offen

Der nächste Architekturblock ist die weitere Verdünnung von `Tables` selbst. Danach sollten die verbliebenen tiefen Zellformatierungs- und Vorbereitungspfade in `lib4tables_prepare.py` weiter in die Architektur gezogen werden.

---

# Stage 22: Zeilenfilter- und Bereichsmorphismen

Stage 22 zieht den nächsten tiefen Restblock aus `libs/lib4tables_prepare.py` heraus: die Zeilenfilterlogik.

## Neue Row-Filtering-Schicht

Neu ist:

- `reta_architecture/row_filtering.py`
- `RowFilteringBundle`
- `bootstrap_row_filtering(...)`

Diese Schicht besitzt jetzt die Morphismen, die Parameter-Sektionen in konkrete Zeilenmengen übersetzen:

- `parameters_cmd_with_some_bereich(...)`
- `filter_original_lines(...)`
- `set_zaehlungen(...)`
- `moonsun(...)`
- `delete_doubles_in_sets(...)`
- `from_until(...)`
- `zeile_which_zaehlung(...)`

Damit ist die frühere Zeilenlogik nicht mehr tief in der `Prepare`-Klasse versteckt. Sie ist jetzt als eigener Morphismenblock inspizierbar.

## Semantische Familien

`RowFilteringBundle.snapshot()` macht die wichtigsten Bedingungsfamilien sichtbar:

- absolute Bereiche
- relative Bereiche
- Zeitrelationen `<`, `=`, `>`
- Zählungsgruppen
- Prim-Mod-6-Innen/Außen-Filter
- Mond/Sonne/Planet-Filter
- Prim-Multiple
- Potenzen
- gewöhnliche Vielfache
- Nachbarschaftsinversion
- `z`-/`y`-Positionsfilter

## Wirkung auf `lib4tables_prepare.py`

`libs/lib4tables_prepare.py` bleibt kompatibel, delegiert aber an die neue Schicht.

Der Umfang fiel von ca. **836 Zeilen** auf ca. **362 Zeilen**.

## Integration

- `RetaArchitecture.bootstrap_row_filtering(...)` bootstrapt die Schicht lazy.
- `RetaArchitecture.snapshot()` enthält jetzt `row_filtering`.
- `reta_architecture_probe_py.py row-filtering-json` macht die Schicht inspizierbar.
- `reta_architecture/package_integrity.py` behandelt `reta_architecture/row_filtering.py` als Pflichtdatei.

## Geprüft

- `py_compile`: OK
- `python -B -S -m unittest tests.test_architecture_refactor -v`: **41 Tests, OK**
- `reta_architecture_probe_py.py row-filtering-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- Smoke-Tests: Shell, Markdown, HTML, Bruch-/CSV-Gluing und Meta-Spalten: OK

## Parität gegen Stage 21

- Shell `Religionen/sternpolygon`: byte-identisch
- Markdown `Religionen/sternpolygon`: byte-identisch
- HTML `Religionen/sternpolygon`: identisch nach Normalisierung
- Bruch-/CSV-Gluing `--gebrochenuniversum=5`: byte-identisch
- Meta-Spalten `--universummetakonkret=meta`: byte-identisch

## Noch offen

`lib4tables_prepare.py` enthält noch Text-/Wrapping- und Breitenlogik. Diese kann als nächster Block in eine eigene Text-/Layout-Morphismenschicht überführt werden. Alternativ kann `Tables` weiter zu einer fast reinen globalen Tabellen-Sektionsfassade verdünnt werden.

---

# Stage 23: Zahlen- und Wrapping-Morphismen

Stage 23 baut direkt auf Stage 22 auf und zieht zwei tief verwendete Legacy-Blöcke in eigene Architekturschichten.

## Neue Wrapping-Schicht

Neu ist `reta_architecture/table_wrapping.py` mit:

- `Wraptype`
- `TextWrapRuntime`
- `TableWrappingBundle`
- `bootstrap_table_wrapping(...)`
- Wrapping-, Hyphenation- und Breitenmorphismen wie `alxwrap(...)`, `wrap_cell_text(...)` und `width_for_row(...)`

`libs/lib4tables_prepare.py` delegiert die Wrapping-/Width-Logik jetzt an diese Schicht.

## Neue Number-Theory-Schicht

Neu ist `reta_architecture/number_theory.py` mit:

- `moonNumber(...)`
- `primFak(...)`
- `divisorGenerator(...)`
- `primRepeat(...)`
- `primCreativity(...)`
- `primMultiple(...)`
- `isPrimMultiple(...)`
- Primzahlkreuz-Prädikaten
- `NumberTheoryBundle`

Diese Schicht ist bewusst dependency-light: keine CLI-, Renderer-, i18n- oder Tabellenimporte.

## Wirkung

Architekturmodule wie `row_filtering.py`, `generated_columns.py`, `meta_columns.py` und `table_output.py` hängen für Zahlenlogik jetzt direkt an `number_theory`, nicht mehr an `lib4tables.py`.

## Geprüft

- `py_compile`: OK
- `tests.test_architecture_refactor`: OK
- Probes für `table-wrapping-json` und `number-theory-json`: OK
- Smoke- und Nicht-Regressionsprüfungen gegen Stage 22: OK

---

# Stage 24: Ausgabe-Syntax-Morphismen

Stage 24 zieht die konkreten Ausgabe-Syntaxklassen aus `libs/lib4tables.py` heraus.

## Neue Output-Syntax-Schicht

Neu ist `reta_architecture/output_syntax.py` mit:

- `NichtsSyntax`
- `OutputSyntax`
- `csvSyntax`
- `emacsSyntax`
- `markdownSyntax`
- `bbCodeSyntax`
- `htmlSyntax`
- `OutputSyntaxBundle`
- `bootstrap_output_syntax(...)`

Diese Klasse von Morphismen beschreibt, wie eine globale Tabellensektion in ein konkretes Ausgabeformat übersetzt wird.

## Wirkung auf `lib4tables.py`

`libs/lib4tables.py` ist jetzt nur noch eine Kompatibilitätsfassade.

Der Umfang fiel von ca. **550 Zeilen** auf ca. **59 Zeilen**. Das Modul exportiert nur noch:

- Output-Syntaxklassen aus `reta_architecture.output_syntax`
- Zahlenmorphismen aus `reta_architecture.number_theory`
- `math` für alte Importpfade

## Entkopplung

`reta_architecture/output_semantics.py`, `table_output.py` und `combi_join.py` importieren Output-Syntax jetzt direkt aus der Architekturschicht. Damit ist `lib4tables.py` nicht mehr Teil des Architektur-Hot-Paths.

## Integration

- `RetaArchitecture.bootstrap_output_syntax(...)` bootstrapt die Schicht.
- `RetaArchitecture.snapshot()` enthält jetzt `output_syntax`.
- `reta_architecture_probe_py.py output-syntax-json` macht die Schicht inspizierbar.
- `reta_architecture/package_integrity.py` behandelt `reta_architecture/output_syntax.py` als Pflichtdatei.

## Geprüft

- `py_compile`: OK
- `python -B -S -m unittest tests.test_architecture_refactor -v`: **45 Tests, OK**
- `python -B -S -m unittest tests.test_command_parity -v`: **1 Paritätsmatrix-Test, OK**
- `python -B -S -m unittest -q`: **46 Tests, OK**
- Probes für `output-syntax-json`, `output-json` und `package-integrity-json`: OK

## Noch offen

Der nächste sinnvolle Block ist eine weitere Verdünnung der verbliebenen Legacy-Fassaden in `tableHandling.py`, `lib4tables_prepare.py` und `lib4tables_concat.py`. Besonders naheliegend ist eine saubere Tabellen-Sektionsschicht, die `Tables` selbst als globale Garbe/Sektion modelliert.

---

# Stage 25: Table-Runtime als globale Tabellensektion

Stage 25 zieht die eigentliche `Tables`-Runtimeklasse aus `libs/tableHandling.py` heraus.

## Neue Table-Runtime-Schicht

Neu ist `reta_architecture/table_runtime.py` mit:

- `Tables`
- `BreakoutException`
- `TableRuntimeBundle`
- `bootstrap_table_runtime(...)`

`Tables` ist damit architektonisch die explizite globale Tabellensektion. Sie bindet weiterhin die historischen Runtime-Bausteine zusammen:

- `Prepare`
- `Concat`
- `KombiJoin`
- `TableOutput`
- Generated-Column-Morphismen

Die historischen Komponenten werden lazy geladen, damit kein Importzyklus über `center` entsteht.

## Wirkung auf `tableHandling.py`

`libs/tableHandling.py` ist jetzt nur noch Kompatibilitätsfassade.

- Stage 24: ca. **275 Zeilen**
- Stage 25: ca. **68 Zeilen**

Das Modul exportiert alte Namen weiter, besitzt aber die Tabellenlogik nicht mehr selbst.

## Wirkung auf `reta.py`

`reta.py` nutzt nun direkt die Architektur:

```python
from reta_architecture.table_runtime import Tables
from reta_architecture.output_syntax import OutputSyntax
from reta_architecture.number_theory import primCreativity
```

Damit hängt der Programmlauf nicht mehr an der alten `tableHandling`-Fassade.

## Integration

- `RetaArchitecture.bootstrap_table_runtime(...)` bootstrapt die Schicht.
- `RetaArchitecture.snapshot()` enthält jetzt `table_runtime`.
- `reta_architecture_probe_py.py table-runtime-json` macht die Schicht inspizierbar.
- `reta_architecture/package_integrity.py` behandelt `reta_architecture/table_runtime.py` als Pflichtdatei.

## Geprüft

- `py_compile`: OK
- `tests.test_architecture_refactor`: **45 Tests, OK**
- `tests.test_command_parity`: **1 Paritätsmatrix-Test, OK**
- `python -B -S -m unittest -v`: **46 Tests, OK**
- Probes für `table-runtime-json`, `package-integrity-json` und Domain-Lookup: OK

## Noch offen

`lib4tables_prepare.py` und `lib4tables_concat.py` sind jetzt dünner, aber noch nicht reine Fassaden. Außerdem könnte `TableRuntime` später weiter in stärker typisierte Untersektionen zerlegt werden: Tabellenzustand, Outputzustand, Zeilenzustand, Generated-Column-State und Kombi-State.

---

# Stage 26 – Explizite Tabellenzustands-Sektionen

Stage 26 zieht den mutable Zustand der globalen `Tables`-Sektion in `reta_architecture/table_state.py` heraus. `Tables` bleibt legacy-kompatibel, besitzt seinen Rohzustand aber nicht mehr direkt als unstrukturierte Initialisierung in `__init__`.

Neue Bausteine:

- `GeneratedColumnSection`
- `TableDisplayState`
- `TableStateSections`
- `TableStateBundle`
- `bootstrap_table_state(...)`

`reta.py` erzeugt Tabellen jetzt über `self.architecture.bootstrap_table_runtime().create_tables(...)`, nicht mehr durch direkten `Tables`-Import. Dadurch hängt der Programmlauf konsequenter an der Architektur-Fassade.

Das neue Probe-Kommando lautet:

```bash
python -B -S reta_architecture_probe_py.py table-state-json
```

Geprüft wurden Architekturtests, Probes, Smoke-Test und direkte Nicht-Regression gegen Stage 25 für Shell, Markdown und Bruch-/CSV-Gluing.

# Stage 27 – Kategorien, Funktoren und natürliche Transformationen

Stage 27 baut direkt auf **Stage 26** auf. Es wurde nicht neu angefangen und es wurde kein Laufzeitverhalten bewusst geändert.

## Ausgangslage

Vor den Refactor-Stufen gab es keine eigene `reta_architecture`-Schicht. Die alte Architektur war hauptsächlich in großen, gemischten Dateien versteckt:

- `reta.py`
- `retaPrompt.py`
- `i18n/words.py`
- `libs/tableHandling.py`
- `libs/lib4tables_prepare.py`
- `libs/lib4tables_concat.py`
- `libs/lib4tables.py`

Nach Stage 26 sind Topologie, Prägarben, Garben, Morphismen, universelle Gluing-Knoten, Tabellen-Runtime und Tabellenzustandssektionen bereits explizit vorhanden.

## Geplanter Schritt

Der nächste geplante Paradigmenwechsel war nicht eine weitere große Verhaltensänderung, sondern eine kategoriale Metaschicht:

1. **Kategorien** für die vorhandenen Architekturgebiete benennen.
2. **Funktoren** zwischen diesen Kategorien benennen.
3. **Natürliche Transformationen** ergänzen, weil die Refactor-Invariante nicht nur heißt „A wird auf B abgebildet“, sondern „alternative Architekturpfade kommutieren beobachtbar“.

Funktoren allein reichen nicht. Beispiele:

- Raw CLI/Prompt → kanonische Parametersemantik muss mit Kontext-Restriktion verträglich sein.
- Prägarben → Garben muss mit lokalem Einschränken und globalem Gluing verträglich sein.
- Legacy-Fassade → Architektur-Fassade muss für repräsentative Kommandos dieselbe Ausgabe liefern.
- Mutable `Tables` → explizite `TableStateSections` muss dieselben mutierbaren Objekte spiegeln.

Das sind natürliche Transformationsbedingungen.

## Neue Schicht

### `reta_architecture/category_theory.py`

Neue, bewusst leichte Metadaten-Schicht:

- `CategoryObjectSpec`
- `CategoryMorphismSpec`
- `CategorySpec`
- `FunctorSpec`
- `NaturalTransformationSpec`
- `ParadigmTermSpec`
- `Stage27ArchitecturePlan`
- `CategoryTheoryBundle`
- `bootstrap_category_theory(...)`

Diese Schicht macht die vorhandene Architektur inspizierbar als:

```text
Topologie
Morphismen
universelle Eigenschaften
Prägarben
Garben
math Kategorien
Funktoren
natürliche Transformationen
```

Sie ist keine schwere Framework-Abstraktion und ersetzt keine Runtime-Funktionen. Sie benennt die Struktur, die bereits durch die früheren Stages entstanden ist.

## Registrierte Kategorien

Stage 27 registriert:

- `OpenRetaContextCategory`
- `LocalSectionCategory`
- `CanonicalSemanticSheafCategory`
- `UniversalConstructionCategory`
- `TableSectionCategory`
- `GeneratedColumnEndomorphismCategory`
- `OutputFormatCategory`
- `LegacyFacadeCategory`

## Registrierte Funktoren

Stage 27 registriert unter anderem:

- `SchemaToTopologyFunctor`
- `RawCommandPresheafFunctor`
- `CanonicalParameterSheafFunctor`
- `LocalDataPresheafFunctor`
- `GluedSemanticSheafFunctor`
- `TableGenerationGluingFunctor`
- `GeneratedColumnEndofunctorFamily`
- `OutputRenderingFunctorFamily`
- `NormalizedOutputFunctor`
- `LegacyRuntimeFunctor`
- `ArchitectureRuntimeFunctor`
- `MutableTableRuntimeFunctor`
- `ExplicitTableStateFunctor`

## Registrierte natürliche Transformationen

Stage 27 registriert:

- `RawToCanonicalParameterTransformation`
- `PresheafToSheafGluingTransformation`
- `TableGenerationGluingTransformation`
- `GeneratedColumnsSheafSyncTransformation`
- `TableRuntimeToStateSectionsTransformation`
- `RenderedOutputNormalizationTransformation`
- `LegacyToArchitectureTransformation`

## Architektur-Integration

`RetaArchitecture` besitzt jetzt zusätzlich:

- `category_theory`

Neue Methode:

```python
RetaArchitecture.bootstrap_category_theory(...)
```

`snapshot()` enthält jetzt:

```python
"category_theory": ...
```

## Probe-Werkzeug

Neues Probe-Kommando:

```bash
python -B -S reta_architecture_probe_py.py category-theory-json
```

Das JSON enthält:

- Paradigma
- Anzahl Kategorien/Funktoren/natürlicher Transformationen
- alle Kategorie-Spezifikationen
- alle Funktor-Spezifikationen
- alle natürlichen Transformationen
- Plan: geplant / jetzt umgesetzt / bereits umgesetzt

## Paket-Manifest

`reta_architecture/package_integrity.py` behandelt jetzt zusätzlich als Pflichtdatei:

- `reta_architecture/category_theory.py`

## Tests

Neue Regressionen prüfen:

- `CategoryTheoryBundle` ist explizit vorhanden.
- `RetaArchitecture.snapshot()` enthält `category_theory`.
- Kategorien, Funktoren und natürliche Transformationen sind registriert.
- Das neue Probe-Kommando ist ausführbar.
- Das Manifest enthält `reta_architecture/category_theory.py`.

## Geprüft

- `py_compile`: OK
- `reta_architecture_probe_py.py category-theory-json`: OK
- `tests.test_architecture_refactor`: **47 Tests, OK**
- `tests.test_command_parity`: **1 Paritätsmatrix-Test, OK**
- volle Unittest-Discovery: **48 Tests, OK**

Die Paritätsmatrix nutzt wieder das bereitgestellte Originalarchiv:

```text
/mnt/data/reta.todel.zip
```

Geprüfte repräsentative Fälle:

- Shell `Religionen/sternpolygon`
- Markdown `Religionen/sternpolygon`
- HTML `Religionen/sternpolygon` nach Normalisierung
- Bruch-/CSV-Gluing `--gebrochenuniversum=5`

## Was bereits vor Stage 27 umgesetzt war

| Paradigma | Bereits vorhandene Schicht |
|---|---|
| Topologie | `reta_architecture/topology.py` |
| Prägarben | `reta_architecture/presheaves.py` |
| Garben | `reta_architecture/sheaves.py` |
| Morphismen | `morphisms.py`, `row_filtering.py`, `generated_columns.py`, `concat_csv.py`, `combi_join.py`, `table_output.py`, `table_wrapping.py`, `number_theory.py` |
| Universelle Eigenschaften / Gluing | `universal.py`, `table_generation.py`, `program_workflow.py` |
| Globale Tabellensektion | `table_runtime.py` |
| Explizite Tabellenzustandssektionen | `table_state.py` |

## Architekturgewinn

Vor Stage 27 war die Architektur bereits mathematisch motiviert, aber die kategoriale Ebene war nur implizit.

Nach Stage 27 ist die Struktur explizit:

```text
Open(Context)^op
    -> lokale Prägarben-Sektionen
    -> geklebte Garben
    -> globale Tabellen-Sektion
    -> Renderer-/Output-Sektionen
```

und zusätzlich:

```text
LegacyRuntimeFunctor
    => ArchitectureRuntimeFunctor
```

als natürliche Transformation für beobachtbare Kompatibilität.

Das ist der entscheidende Punkt: Die neue Architektur beschreibt nicht nur, **welche** Schichten existieren, sondern auch, **welche Pfade kommutieren sollen**.
