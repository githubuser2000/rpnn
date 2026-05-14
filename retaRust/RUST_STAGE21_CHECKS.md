# Rust rreta architecture Stage 21 checks

## Direct compiler fix

Fixed the reported `reta_architecture::csv_catalog` error:

```text
error: lifetime may not live long enough
CsvCatalogBundle { assets: Vec<CsvAsset> }
requires that `'de` must outlive `'static`
```

Root cause: generated static catalog records used `&'static str` and derived `Deserialize`.  Stage 21 keeps static borrowed records `Serialize`-only and adds owned JSON/FFI record types for round trips.

Changed generator too: `tools/generate_csv_catalog.py` now regenerates the fixed shape instead of reintroducing the error.

## Tool versions visible in this container

```text
cargo 1.95.0
rustc 1.95.0
```

## Full workspace build

Not completed in this container because dependency resolution is still blocked before project code is compiled:

```text
error: no matching package named `indexmap` found
location searched: crates.io index
required by package `reta v0.6.0`
```

## Isolated architecture-crate check with local serde stubs

To bypass unavailable crates.io while still checking syntax/types for `reta_architecture`, an isolated workspace was built with local `serde`/`serde_derive` stubs.

```text
cargo check -p reta_architecture --offline: passed
cargo test  -p reta_architecture --offline --lib: 134 passed, 0 failed
```

## Python probes and generators

```text
python3 -m py_compile tools/generate_csv_catalog.py: passed
python3 -m py_compile tools/generate_html_class_catalog.py: passed
python3 -m py_compile tools/architecture_csv_catalog_probe.py: passed
python3 -m py_compile tools/architecture_html_class_catalog_probe.py: passed
python3 -m py_compile tools/architecture_table_materialization_probe.py: passed
python3 tools/generate_csv_catalog.py: passed
python3 tools/generate_html_class_catalog.py: passed
python3 tools/architecture_csv_catalog_probe.py --pretty: status ok
python3 tools/architecture_html_class_catalog_probe.py --pretty: status ok
python3 tools/architecture_table_materialization_probe.py --pretty: status ok
```

## Coverage audits

```text
architecture_module_coverage: 1096 / 1096 functions, 239 / 239 classes
architecture_semantic_surface_audit: 0 marker-only, 0 missing
```
