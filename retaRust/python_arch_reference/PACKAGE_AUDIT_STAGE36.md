# Package Audit Stage 36

Stage 36 adds one required source file:

```text
reta_architecture/architecture_activation.py
```

`reta_architecture/package_integrity.py` now lists it in the required source
paths. The probe to verify the package remains:

```bash
python -B -S reta_architecture_probe_py.py package-integrity-json
```

Expected state:

```text
missing_required: []
runtime_artifact_count: 0
suspicious_csvs: []
```

The source tree should not contain `__pycache__`, `.pyc`, `.pyo`, `.pytest_cache`
or `.mypy_cache` artifacts in the final package.
