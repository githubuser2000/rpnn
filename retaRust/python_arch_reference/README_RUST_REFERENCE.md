# Python architecture reference for Rust transcompilation

This directory is a copy of the current `py reta arch` source supplied for the
Rust transcompilation.  The older `python_reference/` directory is kept as the
legacy monolith/source-freeze used by existing include_str! based parity code.

Rust now has two Python references on purpose:

- `python_reference/`: current exact legacy-data source for generated i18n/code.
- `python_arch_reference/`: modular architecture source for the new typed Rust
  architecture crate and future module-by-module transpilation.
