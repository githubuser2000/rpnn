# retaprompt_commands

Separate additive library for the command-focused layer of `rp`, `rpl`, `rpe`, and `rpb`.

Rules of this layer:

- depends on `reta`
- does not embed or copy `reta`
- does not depend on `retaprompt_input`
- keeps command compilation/execution responsibility separate from own input handling

This crate re-exports the command compiler/executor API from `reta::prompt::commands` and offers thin runners for `rp`, `rpl`, `rpb`, and `rpe`.
