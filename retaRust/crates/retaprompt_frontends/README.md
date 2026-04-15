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


This package is preserved additively, but it is intentionally not an active workspace member.
The root `reta` package still contains the established binaries, so this wrapper package remains optional and inactive to avoid turning the split into a second active binary layer.
