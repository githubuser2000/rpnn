# libreta_table.so

Deutsch: Tabellen-Materialisierung, Tabellenzustand, View-Aufbau und Verklebung lokaler Sektionen.

English: Table materialization, table state, view construction, and gluing of local sections.

This crate builds one dynamic `cdylib` and exports a small C ABI:

- `reta_table_abi_version`
- `reta_table_abi_anchor`
- `reta_table_abi_library_name`
- `reta_table_abi_crate_name`
- `reta_table_abi_role_de` / `reta_table_abi_role_en`
- `reta_table_abi_math_de` / `reta_table_abi_math_en`
- `reta_table_abi_manifest_json`

The detailed German and English documentation is in:

- `doc/shared-libs/de/libreta_table.md`
- `doc/shared-libs/en/libreta_table.md`
