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
the prompt frontends can now sit on top of two separate additive layers without mixing input handling and command-topic handling.

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


## Current split

- `reta` = core runtime and prompt implementation
- `retaprompt_input` = own command input layer for `rp`, `rpl`, `rpe`
- `retaprompt_commands` = command-topic layer for `rp`, `rpl`, `rpe`, `rpb`
- `retaprompt_frontends` = thin binaries using the split layers
