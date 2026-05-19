# libreta_data.so

Deutsch: Daten-, Wörter-, Alias-, CSV- und Kataloggrenze für Reta.

English: Data, word, alias, CSV, and catalog boundary for Reta.

This crate builds one dynamic `cdylib` and exports a small C ABI:

- `reta_data_abi_version`
- `reta_data_abi_anchor`
- `reta_data_abi_library_name`
- `reta_data_abi_crate_name`
- `reta_data_abi_role_de` / `reta_data_abi_role_en`
- `reta_data_abi_math_de` / `reta_data_abi_math_en`
- `reta_data_abi_manifest_json`

The detailed German and English documentation is in:

- `doc/shared-libs/de/libreta_data.md`
- `doc/shared-libs/en/libreta_data.md`

Real component exports now include:

- `reta_data_shared_words_json`
- `reta_data_all_main_alias_groups_json`
- `reta_data_parameter_alias_groups_for_main_json`
- `reta_data_resolve_parameter_main_alias`
