# libreta_render.so

Deutsch: Ausgabegrenze für Shell/Text, HTML, BBCode, Layout, Wrapping und Nummerierung.

English: Output boundary for shell/text, HTML, BBCode, layout, wrapping, and numbering.

This crate builds one dynamic `cdylib` and exports a small C ABI:

- `reta_render_abi_version`
- `reta_render_abi_anchor`
- `reta_render_abi_library_name`
- `reta_render_abi_crate_name`
- `reta_render_abi_role_de` / `reta_render_abi_role_en`
- `reta_render_abi_math_de` / `reta_render_abi_math_en`
- `reta_render_abi_manifest_json`

The detailed German and English documentation is in:

- `doc/shared-libs/de/libreta_render.md`
- `doc/shared-libs/en/libreta_render.md`

Real component exports now include:

- `reta_render_grundstruk_html`
- `reta_render_grundstruk_html_len`

The standard build creates `rgrundStrukHtml` as a tiny C launcher that calls this library directly.

## Split-Abhängigkeit

Im Shared-Library-Build wird `libreta_render.so` gegen `libreta_semantics.so` gelinkt. `rgrundStrukHtml` bleibt ein kleiner Launcher und nutzt die Kette `rgrundStrukHtml -> libreta_render.so -> libreta_semantics.so`.
