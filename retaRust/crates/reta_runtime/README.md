# libreta_runtime.so

Deutsch: Ausführungsnetzwerk mit FIFO, LIFO, Queue, Stack, Duplex-Kanälen, Semaphoren und Cache-Grenzen. Diese Bibliothek trägt außerdem den schweren nicht-interaktiven Reta-Kern, damit `libreta.so` eine kleine stabile Fassade bleiben kann.

English: Execution network with FIFO, LIFO, queue, stack, duplex channels, semaphores, and cache boundaries. This library also carries the heavy non-interactive Reta core so that `libreta.so` can remain a small stable facade.

This crate builds one dynamic `cdylib` and exports a small C ABI:

- `reta_runtime_abi_version`
- `reta_runtime_abi_anchor`
- `reta_runtime_abi_library_name`
- `reta_runtime_abi_crate_name`
- `reta_runtime_abi_role_de` / `reta_runtime_abi_role_en`
- `reta_runtime_abi_math_de` / `reta_runtime_abi_math_en`
- `reta_runtime_abi_manifest_json`

It also exports the private core-forwarding ABI used by `libreta.so`, including:

- `reta_runtime_core_run_and_print_from_env_ffi`
- `reta_runtime_core_run_argv`
- `reta_runtime_core_free_string`
- `reta_runtime_core_shared_words_json`

The detailed German and English documentation is in:

- `doc/shared-libs/de/libreta_runtime.md`
- `doc/shared-libs/en/libreta_runtime.md`
