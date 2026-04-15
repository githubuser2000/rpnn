# retaprompt_frontends

Additive frontend package for `rp`, `rpl`, `rpb`, and `rpe`.

This package exists for one reason: the visible prompt binaries are built
through a package that depends on the single shared `retaprompt` library package.

That means the layering is now explicit:

- `reta` = core runtime and prompt implementation
- `retaprompt` = the single shared retaPrompt library package that emits `libretaprompt.a`
- `retaprompt_frontends` = thin binary wrappers for `rp`, `rpl`, `rpb`, `rpe`

## Important

This is additive. It does not delete the existing root-package binaries.
Those old binaries remain untouched. This package adds a dedicated path in which
all four prompt frontends sit on top of the same single retaPrompt static
library package.

## Build

Build the shared library:

```bash
cargo build -p retaprompt --lib
```

Build the dedicated frontend binaries that depend on that shared package:

```bash
cargo build -p retaprompt_frontends --bin rp
cargo build -p retaprompt_frontends --bin rpl
cargo build -p retaprompt_frontends --bin rpb
cargo build -p retaprompt_frontends --bin rpe
```

Or build both layers together:

```bash
./tools/build_retaprompt_staticlib.sh debug all
```
