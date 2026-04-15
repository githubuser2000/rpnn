# retaPrompt split library layout

This repository keeps `reta` as the unchanged shared core library.
The retaPrompt side is split additively into two independent libraries.
A thin frontend package may stay in the repository as preserved wrapper code,
but it is not part of the active split itself.

## Active layout

1. `reta`
   - unchanged core library
   - single shared lower layer
   - contains the real reusable implementation that the split prompt libraries call

2. `retaprompt_input`
   - only own command input for `rp`, `rpl`, `rpe`
   - depends only on `reta`
   - no dependency on `retaprompt_commands`
   - no copied code from `reta`

3. `retaprompt_commands`
   - only command-topic handling for `rp`, `rpl`, `rpe`, `rpb`
   - depends only on `reta`
   - no dependency on `retaprompt_input`
   - no copied code from `reta`

4. optional preserved wrapper package: `retaprompt_frontends`
   - only thin binaries
   - preserved so old code is not destroyed
   - intentionally inactive in the workspace
   - contains no duplicated `reta` implementation

## Binary-to-library mapping

- `rp` -> `retaprompt_input`
- `rpl` -> `retaprompt_input`
- `rpe` -> `retaprompt_input`
- `rpb` -> `retaprompt_commands`

## Separation rules

- nothing from `reta` is copied into the two prompt libraries
- `retaprompt_input` and `retaprompt_commands` do not depend on each other
- nothing from any of the three libraries is contained inside either of the other two
- `rpb` stays on the command-topic side
- `rp`, `rpl`, `rpe` stay on the own-input side

## Legacy code policy

The older mixed `crates/retaprompt` package is kept in the repository so old
code is not destroyed. It is intentionally inactive and not part of the active
workspace member list.

## Active workspace members

- root package `reta`
- `crates/retaprompt_input`
- `crates/retaprompt_commands`

The preserved packages `crates/retaprompt` and `crates/retaprompt_frontends` stay in the repository but are not active workspace members.
