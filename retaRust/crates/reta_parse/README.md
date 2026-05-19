# libreta_parse.so

Deutsch: Parsing- und Eingabeauflösungsgrenze für argv, Optionen, Aliase und Prompt-Tokens.

English: Parsing and input-resolution boundary for argv, options, aliases, and prompt tokens.

This crate builds one dynamic `cdylib` and exports a small C ABI:

- `reta_parse_abi_version`
- `reta_parse_abi_anchor`
- `reta_parse_abi_library_name`
- `reta_parse_abi_crate_name`
- `reta_parse_abi_role_de` / `reta_parse_abi_role_en`
- `reta_parse_abi_math_de` / `reta_parse_abi_math_en`
- `reta_parse_abi_manifest_json`

The detailed German and English documentation is in:

- `doc/shared-libs/de/libreta_parse.md`
- `doc/shared-libs/en/libreta_parse.md`
