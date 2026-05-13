# Transcompilation architecture stage 2: serde words fix

Status: static fix applied.

## Reason

The previous Rust architecture-stage build failed at `src/ffi.rs` because `reta_shared_words_json()` serializes `crate::shared_words()` with `serde_json::to_string(...)`, but the generated `shared::words_py::Words` type did not implement `serde::Serialize`.

## Changes

- Added `use serde::{Deserialize, Serialize};` to generated word data modules.
- Added `Serialize, Deserialize` derives to:
  - `PyValue`
  - `StoreParameterEntry`
  - `Words`
- Updated `tools/transpile_words.py` so future regeneration keeps those derives instead of reintroducing the same compile error.

## Touched files

- `src/shared/words_py.rs`
- `src/shared/words_python_like.rs`
- `tools/transpile_words.py`

## Build status

Cargo/rustc are still not available in this execution container. Attempts to install via Debian APT failed because the package index was unavailable in the container, and shell network DNS resolution failed. Therefore this stage is not cargo-build-verified here.

The concrete reported error `E0277: Words: serde::Serialize is not satisfied` is addressed by the patch.
