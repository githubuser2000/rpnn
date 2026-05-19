# Reta `.so` split — English overview

The structure is now implemented as a real thin facade: `rreta` links directly only to `libreta.so`; `libreta.so` exports the stable public ABI and forwards heavy engine execution to `libreta_runtime.so`. The other private core libraries remain linked as topology and ABI boundaries.

In the split build, `libreta.so` should be small, while `libreta_runtime.so` carries the heavy non-interactive Reta core. The build script checks this size direction and fails if `libreta.so` becomes larger than or equal to `libreta_runtime.so` again.

The prompt programs remain split: `rrpb` uses only `libretaprompt_commands.so`, while `rrp`, `rrpl`, and `rrpe` also use `libretaprompt_input.so` for autocomplete and autosuggest.

See `doc/shared-libs/en/README.md` for the per-library documentation.
