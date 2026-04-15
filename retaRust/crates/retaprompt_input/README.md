# retaprompt_input

Separate library for the own command input layer of `rp`, `rpl`, and `rpe`.

Rules of this layer:

- depends only on `reta`
- does not embed or copy `reta`
- does not depend on `retaprompt_commands`
- does not contain command-topic logic for `rpb`
- contains only the self-entered frontend/input side for `rp`, `rpl`, `rpe`
