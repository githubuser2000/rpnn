# retaprompt_input

Separate additive library for the own command input layer of `rp`, `rpl`, and `rpe`.

Rules of this layer:

- depends on `reta`
- does not embed or copy `reta`
- does not depend on `retaprompt_commands`
- keeps the input/front-end responsibility separate from command semantics

The intent is that interactive/self-entered command handling lives here, while pure command-topic handling for `rp`, `rpl`, `rpe`, `rpb` lives in `retaprompt_commands`.
