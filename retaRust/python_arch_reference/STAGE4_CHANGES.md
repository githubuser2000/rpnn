# Stage 4 continuation changes

This stage continued from the stage-3 repository and focused on the next large
architectural contradiction: the input layer was still mostly implicit in
`center.py` and `LibRetaPrompt.py`.

## Main changes

- added `reta_architecture/input_semantics.py`
  - `RowRangeSyntax`
  - `PromptVocabulary`
  - `PromptVocabularyBuilder`
  - `InputBundle`
- added `reta_architecture/split_i18n.py`
  - builds a legacy-compatible i18n namespace directly from the physically split
    modules instead of routing through `i18n.words`
- updated `reta_architecture/facade.py`
  - architecture now contains an `inputs` layer
- updated `reta_architecture/__init__.py`
  - exports the new input-layer types
- updated `reta_architecture_probe_py.py`
  - added `inputs-json`
- updated `libs/center.py`
  - uses the split i18n proxy
  - delegates row-range grammar to `RowRangeSyntax`
- updated `libs/LibRetaPrompt.py`
  - prompt/completion vocabulary now comes from the explicit architecture input
    layer
  - removed the extra standalone `reta.Program(...)` call for prompt-number
    derivation
- extended `tests/test_architecture_refactor.py`
  - validates the input layer, split i18n proxy, range syntax, and prompt
    vocabulary export compatibility

## Validation

Validated by:

- `py_compile`
- `unittest`
- `reta_architecture_probe_py.py inputs-json`
- `reta_architecture_probe_py.py module-split-json`
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`
- real `reta.Program(['reta.py'])` smoke run

## Semantic check

Observed semantic values remained stable:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `AllSimpleCommandSpalten_len = 554`
- pair lookup `Religionen / Hinduismus -> [217]`
