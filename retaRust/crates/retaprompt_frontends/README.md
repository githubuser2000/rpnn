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
