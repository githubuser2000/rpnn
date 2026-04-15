# retaPrompt split static libraries

The project now targets three distinct static archives with no code copied from one archive into another:

- `libreta.a` = full shared implementation base
- `libretaprompt_input.a` = tiny ABI forwarding archive for the self-entered prompt input side (`rp`, `rpl`, `rpe`)
- `libretaprompt_commands.a` = tiny ABI forwarding archive for the command-topic side (`rp`, `rpl`, `rpe`, `rpb`)

The two prompt-side archives must **not** be built as Rust `staticlib`, because that would pull `reta` into them again and duplicate code from `libreta.a`.

Instead, `libreta.a` exports the implementation symbols, and the two prompt-side archives are built from small C shim objects that forward into `libreta.a`.

## Functional split

### Input-side library

`libretaprompt_input.a` covers own command input only:

- `rp`
- `rpl`
- `rpe`

Exported public ABI symbols:

- `retaprompt_input_run_rp_from_env`
- `retaprompt_input_run_rpl_from_env`
- `retaprompt_input_run_rpe_from_env`

Forwarded implementation symbols in `libreta.a`:

- `reta_retaprompt_input_run_rp_from_env`
- `reta_retaprompt_input_run_rpl_from_env`
- `reta_retaprompt_input_run_rpe_from_env`

### Command-side library

`libretaprompt_commands.a` covers command-topic entry points only:

- `rp`
- `rpl`
- `rpe`
- `rpb`

Exported public ABI symbols:

- `retaprompt_commands_run_rp_from_env`
- `retaprompt_commands_run_rpl_from_env`
- `retaprompt_commands_run_rpb_from_env`
- `retaprompt_commands_run_rpe_from_env`

Forwarded implementation symbols in `libreta.a`:

- `reta_retaprompt_commands_run_rp_from_env`
- `reta_retaprompt_commands_run_rpl_from_env`
- `reta_retaprompt_commands_run_rpb_from_env`
- `reta_retaprompt_commands_run_rpe_from_env`

## Rust crate layout

The Rust crate split remains additive:

- root crate `reta` = unchanged core implementation
- `crates/retaprompt_input` = Rust input-side facade on top of `reta`
- `crates/retaprompt_commands` = Rust command-side facade on top of `reta`

These two Rust crates depend only on `reta` and not on each other.

## Build

Use the dedicated helper:

```bash
./tools/build_prompt_split_staticlibs.sh debug
./tools/build_prompt_split_staticlibs.sh release
```

This builds:

```text
target/<profile>/libreta.a
target/<profile>/libretaprompt_input.a
target/<profile>/libretaprompt_commands.a
```

## Correct native link model

```text
... libretaprompt_input.a libretaprompt_commands.a libreta.a ...
```

In this model:

- `libretaprompt_input.a` contains only input-side C forwarders
- `libretaprompt_commands.a` contains only command-side C forwarders
- `libreta.a` contains the Rust implementation

So the three archives stay disjoint in contained code.
