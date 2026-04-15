# retaprompt_frontends

Frontend package for `rp`, `rpl`, `rpb`, and `rpe`.

The layering is explicit and separated:

- `reta` = unchanged core library
- `retaprompt_input` = own command input for `rp`, `rpl`, `rpe`
- `retaprompt_commands` = command-topic layer for `rp`, `rpl`, `rpe`, `rpb`
- `retaprompt_frontends` = thin binaries

The frontend binaries use the split strictly:

- `rp`, `rpl`, `rpe` -> `retaprompt_input`
- `rpb` -> `retaprompt_commands`


This package is now the active binary layer for `rp`, `rpl`, `rpb`, and `rpe`.
The root `reta` package keeps the old source files only as preserved legacy code, but the active Cargo build for these four prompt binaries runs through this package so no dependency cycle is created around the core `reta` library.
