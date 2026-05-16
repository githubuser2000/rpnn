# Architecture diagnostic executables

This project now contains many `rreta_arch_*` and `rretaprompt_arch_*` binaries. They are not meant to replace the normal `rreta`/`rretaPrompt` command directly. Most of them are diagnostic, shadow, parity, activation, or migration tools.

The important normal binaries remain:

- `rreta`: current visible `reta` path.
- `rrp`, `rrpl`, `rrpb`, `rrpe`: retaprompt frontends.

The architecture binaries are useful because they produce machine-readable JSON for targeted checks. The most useful script for collecting those outputs is:

```bash
python3 tools/run_architecture_diagnostics.py --build-selected --python-probes --pretty
```

A faster run without prebuilding all selected binaries:

```bash
python3 tools/run_architecture_diagnostics.py --pretty
```

The script writes a directory like:

```text
target/reta_arch_diagnostics/20260516_123456/
```

containing:

```text
binary_catalog.json
diagnostics_summary.json
cases/<case>/legacy_lines.txt
cases/<case>/<binary>.stdout.txt
cases/<case>/<binary>.stderr.txt
cases/<case>/<binary>.meta.json
python_probes/*
```

These outputs are useful to paste or upload when debugging the Rust migration, because they show the exact state of parameter parsing, materialization, language sync, table output, parity, commit readiness, activation, prompt guards, and recovery decisions.

## Recommended cases

The diagnostic runner currently uses these core cases:

```text
continuum_m
continuum_m_en
ordered_744_493
row_order
```

The most important one remains:

```bash
-zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0
```

because it verifies:

```text
493 -> M Kontinuum (dreizehn)
744 -> Neues M (13) Kontinuum
```

## Binary catalog

| Binary | Kategorie | Zweck | Nützliche Ausgabe | Legacy-Lines? |
|---|---|---|---|---:|
| `rreta_arch_activation_file` | activation | Writes/reads an activation store file with read-back validation. | File path, write/read status, digest and parse readiness. | ja |
| `rreta_arch_activation_journal` | activation | Records activation transactions into a replayable journal. | Journal records and replay result. | ja |
| `rreta_arch_activation_ledger` | activation | Turns activation journal records into a hash-chain ledger. | Ledger entries, chain hashes, validation guards. | ja |
| `rreta_arch_activation_persistence` | activation | Persists activation-store text through the architecture persistence layer. | Roundtrip digest, parse readiness, audit/cache status. | ja |
| `rreta_arch_activation_promotion` | activation | Decides whether a case can be promoted from shadow to default visible path. | Promotion ready/blocked status and failed promotion guards. | ja |
| `rreta_arch_activation_readiness` | activation | Folds commit/audit/journal/store/recovery witnesses into readiness. | Ready/blocked status, required guards, promotion-level hint. | ja |
| `rreta_arch_activation_recovery` | activation | Reads an existing activation-store file and validates recovery safety. | Recovery candidate/allowed replay, selected source, failed guards. | ja |
| `rreta_arch_activation_replay` | activation | Checks whether a previous journal can be safely replayed. | Replay guards: transaction id, legacy checksum, rollback decision. | ja |
| `rreta_arch_activation_store` | activation | Encodes/parses activation journal+ledger as a line-oriented store. | Store text metadata, parse report, checksum validation. | ja |
| `rreta_arch_activation_transaction` | activation | Selects visible source via an explicit audited transaction. | Selected source, selected lines checksum, fallback reason. | ja |
| `rreta_arch_commit_audit` | activation | Audits all required TableViewOutput commit guards. | Required/diagnostic guard list, failed guards, language/virtual guard status. | ja |
| `rreta_arch_inspect` | cli-plan | Explains architecture switch, runtime gates and CLI plan. | JSON with parsed architecture mode, gates, selected parameters. | nein |
| `rreta_arch_language_coverage` | language | Reports which language CSVs cover requested direct columns. | Per-language coverage, stale languages, missing columns. | nein |
| `rreta_arch_language_parity` | language | Checks selected/effective language asset and direct 744 safety. | Language parity status, requested/effective asset, failed guards. | nein |
| `rreta_arch_language_sync` | language | Reports pending language CSV sync actions. | Pending languages/columns/assets and sync readiness. | nein |
| `rreta` | legacy-visible | Runs the current visible rreta command path. | Legacy output lines used as comparison input. | nein |
| `rreta_arch_view_output_parity` | parity | Compares Rust TableViewOutput against supplied legacy lines. | Raw and semantic parity JSON. | ja |
| `rreta_arch_view_output_shadow` | parity | Runs shadow TableViewOutput report and commit decision against legacy lines. | Shadow report, commit decision, raw/semantic equality. | ja |
| `rreta_arch_prompt_activation_readiness` | prompt | Folds prompt shadow commit and language guard into prompt readiness. | Prompt activation readiness report and failed guards. | nein |
| `rreta_arch_prompt_language_commit` | prompt | Checks prompt shadow commit with language guard. | Prompt shadow report, commit policy, commit decision. | nein |
| `rreta_arch_prompt_language_completion` | prompt | Shows language parameter/value completions for prompt text. | Prompt completion candidates and language coverage/sync witness. | nein |
| `rreta_arch_prompt_language_guard` | prompt | Checks prompt language completion/coverage/sync safety. | Prompt language guard ready/blocked and failed guards. | nein |
| `rretaprompt_arch_inspect` | prompt | Prompt-frontends inspect binary for retaprompt shadow/commit/readiness. | Cleaned args, shadow report, commit/readiness JSON. | nein |
| `rreta_arch_cell_styles` | style | Tests legacy generateCell/cell-wrapper projection. | Cell-style report and styled output. | nein |
| `rreta_arch_html_classes` | style | Snapshots the generated htmlclassesPy.jsonl catalog. | HTML class catalog counts and records. | nein |
| `rreta_arch_html_output` | style | Renders HTML output with optional htmlclasses witnesses. | HTML output plus attribute report. | nein |
| `rreta_arch_row_styles` | style | Tests legacy row color wrappers for HTML/BBCode. | Row-style report and styled output. | nein |
| `rreta_arch_shell_styles` | style | Tests shell/ANSI color projection and ANSI-strip parity. | ANSI-cell count and styled shell output. | nein |
| `rreta_arch_style_composition` | style | Merges HTML-class attributes with cell-style wrappers. | Composition counts and composed HTML output. | nein |
| `rreta_arch_style_parity` | style-parity | Compares plain vs styled HTML/BBCode semantically. | Style-aware raw/semantic parity report. | nein |
| `rreta_arch_materialize` | table-data | Materializes selected CSV/virtual sections from CLI arguments. | JSON showing direct CSV cells, requested columns, row/column orders. | nein |
| `rreta_arch_view` | table-data | Builds a MaterializedTableView from materialized sections. | JSON with view rows, cells, direct/virtual classification. | nein |
| `rreta_arch_column_order` | table-diagnostics | Verifies --spaltenreihenfolgeundnurdiese ordering. | Requested/materialized column order, especially 744,493 cases. | nein |
| `rreta_arch_row_order` | table-diagnostics | Verifies explicit row ordering. | Requested/materialized row order and header handling. | nein |
| `rreta_arch_layout` | table-output | Explains shell layout, widths and horizontal column pages. | Measured/effective widths, layout pages, rendered lines. | nein |
| `rreta_arch_numbering` | table-output | Explains legacy Zaehlung/Nummerierung prefix projection. | Numbering report and output with numbering enabled. | nein |
| `rreta_arch_output_flags` | table-output | Shows parsed output flags such as headers, empty rows, widths and wrapping. | TableViewOutput options and rendered report. | nein |
| `rreta_arch_view_output` | table-output | Renders a MaterializedTableView as shell/csv/html/bbcode/markdown/etc. | JSON with rendered_lines and output-mode metadata. | nein |
| `rreta_arch_virtual_columns` | virtual-columns | Renders non-direct/virtual columns by policy. | Virtual policy, virtual cells and rendered witnesses. | nein |
| `rreta_arch_virtual_parity` | virtual-columns | Checks virtual-policy changes preserve direct CSV cell identity. | Direct-cell identity and added-virtual-only guards. | nein |
