# Stage 66 Checks

Passed:

```text
python3 tools/architecture_prompt_language_commit_guard_probe.py --pretty
python3 tools/architecture_prompt_language_guard_probe.py --pretty
python3 tools/architecture_prompt_language_completion_probe.py --pretty
python3 tools/architecture_language_sync_guard_probe.py --pretty
python3 tools/architecture_language_sync_probe.py --pretty
python3 tools/architecture_language_coverage_guard_probe.py --pretty
python3 tools/architecture_language_coverage_probe.py --pretty
python3 tools/architecture_csv_catalog_probe.py --pretty
python3 tools/architecture_migration_step_arity_probe.py --pretty
python3 tools/architecture_module_coverage.py --pretty
python3 tools/architecture_module_coverage.py --pretty --only-missing
python3 tools/architecture_semantic_surface_audit.py --pretty
python3 tools/architecture_semantic_surface_audit.py --pretty --only-marker-or-missing
python3 -m py_compile tools/architecture_prompt_language_commit_guard_probe.py
```

Static delimiter balance for changed Rust files: passed.

Not run here:

```text
cargo check
cargo test
full workspace build
```

Reason: `cargo` and `rustc` are not available in this container session.
