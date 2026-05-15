# Stage 44 check report

## Passed

```text
python3 -m py_compile tools/architecture_activation_ledger_probe.py
python3 tools/architecture_activation_ledger_probe.py --pretty
python3 tools/architecture_activation_replay_probe.py --pretty
python3 tools/architecture_activation_journal_probe.py --pretty
python3 tools/architecture_activation_transaction_probe.py --pretty
python3 tools/architecture_commit_audit_probe.py --pretty
python3 tools/architecture_virtual_commit_guard_probe.py --pretty
python3 tools/architecture_virtual_parity_probe.py --pretty
python3 tools/architecture_table_view_output_parity_probe.py --pretty
python3 tools/architecture_table_view_output_commit_probe.py --pretty
python3 tools/architecture_module_coverage.py --pretty
python3 tools/architecture_module_coverage.py --pretty --only-missing
python3 tools/architecture_semantic_surface_audit.py --pretty
python3 tools/architecture_semantic_surface_audit.py --pretty --only-marker-or-missing
python TOML parse checks
JSON parse checks for generated reports
archive creation / extraction read check
```

## Coverage

```text
Functions represented: 1096 / 1096
Classes represented:   239 / 239
Missing:               0
```

## Strict semantic-surface audit

```text
Functions declared:    1096 / 1096
Functions marker-only: 0
Functions missing:     0
Classes declared:      239 / 239
Classes marker-only:   0
Classes missing:       0
```

## Not run here

```text
cargo check
cargo test
full workspace build
```

Reason: this container shell currently has no `cargo` / `rustc` available.
