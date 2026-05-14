# Stage 25 checks

## Passed

- `python3 -m py_compile tools/architecture_table_view_output_parity_probe.py`
- `python3 tools/architecture_csv_catalog_probe.py --pretty`
- `python3 tools/architecture_html_class_catalog_probe.py --pretty`
- `python3 tools/architecture_table_materialization_probe.py --pretty`
- `python3 tools/architecture_table_view_probe.py --pretty`
- `python3 tools/architecture_table_view_output_probe.py --pretty`
- `python3 tools/architecture_table_view_output_commit_probe.py --pretty`
- `python3 tools/architecture_table_view_output_parity_probe.py --pretty`
- `python3 tools/architecture_module_coverage.py --pretty`
- `python3 tools/architecture_module_coverage.py --pretty --only-missing`
- `python3 tools/architecture_semantic_surface_audit.py --pretty`
- `python3 tools/architecture_semantic_surface_audit.py --pretty --only-marker-or-missing`
- `Cargo.toml` parse via Python `tomllib`
- `crates/reta_architecture/Cargo.toml` parse via Python `tomllib`
- `crates/retaprompt_frontends/Cargo.toml` parse via Python `tomllib`

## Probe result highlights

- Markdown separator rows normalize away.
- HTML table wrappers normalize away.
- CSV quoted separators stay inside cells.
- Shadow pipeline now carries `semantic_diff`.
- Commit decision exposes `semantic_equal`.
- Raw equality remains the normal commit guard.

## Not run

- `cargo check`
- `cargo test`
- full workspace build

Reason: `cargo` and `rustc` are not installed in this container shell.
