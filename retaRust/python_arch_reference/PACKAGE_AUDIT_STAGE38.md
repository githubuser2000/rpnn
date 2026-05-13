# Package Audit Stage 38

Stage 38 adds one required source file:

```text
reta_architecture/arithmetic.py
```

The package manifest now expects this file in addition to the Stage-37
row-range owner.  Runtime artifacts such as `__pycache__`, `.pyc`, `.pyo`,
`.pytest_cache` and `.mypy_cache` must not be included in the release ZIP.
