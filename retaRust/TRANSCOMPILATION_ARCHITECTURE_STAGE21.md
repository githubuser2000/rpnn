# Transcompilation Architecture Stage 21

Stage 21 fixes the concrete compiler error reported for `csv_catalog.rs` and continues the Rust architecture port by adding an HTML/class witness catalog plus virtual-column witnesses for materialization.

## Key fixes

- `CsvAsset` and `CsvCatalogBundle` are now static borrowed catalog types and derive `Serialize` only.
- New owned catalog types derive `Serialize` and `Deserialize`:
  - `OwnedCsvAsset`
  - `OwnedCsvCatalogBundle`
- `tools/generate_csv_catalog.py` now regenerates this fixed shape, so the lifetime error is not reintroduced.

## New HTML/class catalog

Added:

```text
crates/reta_architecture/src/html_class_catalog.rs
tools/generate_html_class_catalog.py
tools/architecture_html_class_catalog_probe.py
src/bin/reta_arch_html_classes.rs
```

The generated catalog contains:

```text
1372 html-class records
1041 unique columns
1254 records with text
695 records with class strings
1 record for column 744
```

## Materialization improvement

`table_materialization.rs` now distinguishes direct CSV materialization from virtual/non-direct column witnesses.

For the known case:

```text
-spalten --kontinuum=m -> 493, 744
```

Rust now records:

- `493` as a direct `religion.csv` materialized column.
- `744` as a selected but non-direct CSV column with tag/html witnesses.

New structures:

```text
VirtualColumnMaterialization
virtual_columns
virtual_column_count
continuum_m_virtual_column_present
```

## FFI additions

Added:

```text
reta_csv_catalog_assets_json()
reta_html_class_catalog_snapshot_json()
reta_html_class_catalog_records_json()
```

## Checks

The architecture crate was checked in an isolated workspace with local serde stubs:

```text
cargo check -p reta_architecture --offline: passed
cargo test -p reta_architecture --offline --lib: 134 passed, 0 failed
```

A full workspace build remains blocked in this container by missing crates.io dependencies (`indexmap`/network resolution), not by the reported `CsvCatalogBundle` lifetime error.
