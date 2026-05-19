# Reta shared-library topology

This documentation describes every built `.so` library in English.

## Target structure

```text
rreta
  -> libreta.so
       -> libreta_data.so
       -> libreta_parse.so
       -> libreta_semantics.so
       -> libreta_table.so
       -> libreta_render.so
       -> libreta_arch.so
       -> libreta_runtime.so

rrp / rrpl / rrpe
  -> libretaprompt_input.so
  -> libretaprompt_commands.so

rrpb
  -> libretaprompt_commands.so
```

## Rule

The executables stay small. Program logic lives in `.so` libraries. `libreta.so` is now deliberately only the stable thin facade; the heavy non-interactive Reta core lives in `libreta_runtime.so` and is reached through private `reta_runtime_core_*` symbols. The other private core libraries form the explicit internal topology and can later be filled with more concrete domain logic.

## Size rule

`libreta.so` must be smaller than `libreta_runtime.so`. The build scripts fail if `libreta.so` becomes the heavy engine carrier again.

## Per-library documents

- [libreta.so](libreta.md)
- [libreta_data.so](libreta_data.md)
- [libreta_parse.so](libreta_parse.md)
- [libreta_semantics.so](libreta_semantics.md)
- [libreta_table.so](libreta_table.md)
- [libreta_render.so](libreta_render.md)
- [libreta_arch.so](libreta_arch.md)
- [libreta_runtime.so](libreta_runtime.md)
- [libretaprompt_commands.so](libretaprompt_commands.md)
- [libretaprompt_input.so](libretaprompt_input.md)
