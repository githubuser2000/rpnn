# libreta_render.so — rendering functors

## Purpose

Boundary for shell/text, HTML, BBCode, layout, wrapping, numbering, styles, and output formats.

## Direct integration

Direct user of this library: `libreta.so`.

Direct target dependencies: no direct private mandatory dependency inside this layer.

## Architecture boundary

This `.so` is an intentional ABI boundary. Rust-internal types should not cross it. The outside surface exports stable C symbols, simple integer values, and null-terminated strings. That keeps the topology stable while more Rust code can later move out of `libreta.so` and behind this library.

## Mathematical role

Functor: the same semantic table is mapped into concrete representation spaces without changing the semantics itself.

## Important ABI symbols

- `reta_render_abi_version`
- `reta_render_abi_anchor`
- `reta_render_abi_manifest_json`
- `reta_render_abi_role_de`
- `reta_render_abi_role_en`


## Real code extraction

This library now carries the real `grundStrukHtml` HTML generation code. `rgrundStrukHtml` is therefore built as a tiny C launcher that calls `libreta_render.so` directly instead of embedding the heavy Rust core into the executable.

Additional symbols:

- `reta_render_grundstruk_html`
- `reta_render_grundstruk_html_len`
- `reta_render_free_string`

## Build rule

`build.sh` builds the private core libraries first and then builds `libreta.so` with `RETA_LINK_CORE_SPLIT_LIBS=1`. As a result, `rreta` links directly only to `libreta.so`; the private core libraries appear as `DT_NEEDED` entries of `libreta.so`.

## Non-goal

This library should not become a second public program interface beside `libreta.so`. Public program execution remains stable through the facade.

## Dependency on libreta_semantics.so

`libreta_render.so` is linked against `libreta_semantics.so` in the shared-library build. This keeps the semantic inventory boundary inside the dynamic topology for `rgrundStrukHtml`: `rgrundStrukHtml -> libreta_render.so -> libreta_semantics.so`.
