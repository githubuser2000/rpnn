# Package Audit Stage 35

Stage 35 ergänzt die Pflichtquelle:

```text
reta_architecture/architecture_rehearsal.py
```

Sie ist im Paketmanifest registriert und wird von `package-integrity-json` geprüft. Runtime-Artefakte wie `__pycache__`, `.pyc`, `.pyo`, `.pytest_cache` und `.mypy_cache` gehören nicht ins Paket.
