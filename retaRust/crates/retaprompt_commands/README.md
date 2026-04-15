# retaprompt_commands

Separate library for the command-topic layer of `rp`, `rpl`, `rpe`, and `rpb`.

Rules of this layer:

- depends only on `reta`
- does not embed or copy `reta`
- does not depend on `retaprompt_input`
- does not expose the interactive self-input frontend API
- contains command compilation/execution and the direct `rpb` command entry
