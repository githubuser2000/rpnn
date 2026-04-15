# retaPrompt split library layout

Target layout:

1. `reta`
   - unchanged core library
   - single shared lower layer

2. `retaprompt_input`
   - only own command input for `rp`, `rpl`, `rpe`
   - depends only on `reta`

3. `retaprompt_commands`
   - only command-topic handling for `rp`, `rpl`, `rpe`, `rpb`
   - depends only on `reta`

4. `retaprompt_frontends`
   - only thin binaries
   - depends on the two prompt split libraries

Separation rules:

- nothing from `reta` is copied into the two prompt libraries
- `retaprompt_input` and `retaprompt_commands` do not depend on each other
- the mixed `retaprompt` library package is not part of the active workspace layout
- `rpb` stays on the command-topic side
- `rp`, `rpl`, `rpe` stay on the own-input side
