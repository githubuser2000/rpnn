# Package Audit Stage 31

Stage 31 adds the validation/coherence source modules and documentation.

## New required architecture files

```text
reta_architecture/architecture_validation.py
reta_architecture/architecture_coherence.py
```

## New probe commands

```bash
python -B -S reta_architecture_probe_py.py architecture-validation-json
python -B -S reta_architecture_probe_py.py architecture-validation-md
python -B -S reta_architecture_probe_py.py architecture-coherence-json
python -B -S reta_architecture_probe_py.py architecture-coherence-md
```

## Final local package check

After removing runtime caches, `package-integrity-json` reported:

```text
file_count: 332
missing_required: []
runtime_artifact_count: 0
suspicious_csvs: []
```

The final ZIP must not contain `__pycache__`, `.pyc`, `.pyo`, `.pytest_cache`, `.mypy_cache` or generated runtime artifacts.

## Test status

```text
py_compile selected architecture/test/probe files: OK
tests.test_architecture_refactor: 52 tests, OK
tests.test_command_parity: 1 test, OK
unittest discover: 53 tests, OK
```
