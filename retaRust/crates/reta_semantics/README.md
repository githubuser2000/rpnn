# libreta_semantics.so

Deutsch: Semantische Auswahlgrenze für Spalten, Zeilenfilter, Generatoren, Zahlenlogik und Tags.

English: Semantic selection boundary for columns, row filters, generators, number logic, and tags.

This crate builds one dynamic `cdylib` and exports a small C ABI:

- `reta_semantics_abi_version`
- `reta_semantics_abi_anchor`
- `reta_semantics_abi_library_name`
- `reta_semantics_abi_crate_name`
- `reta_semantics_abi_role_de` / `reta_semantics_abi_role_en`
- `reta_semantics_abi_math_de` / `reta_semantics_abi_math_en`
- `reta_semantics_abi_manifest_json`

The detailed German and English documentation is in:

- `doc/shared-libs/de/libreta_semantics.md`
- `doc/shared-libs/en/libreta_semantics.md`
