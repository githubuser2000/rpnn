#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PROFILE="${1:-debug}"
case "$PROFILE" in
  debug)
    CARGO_FLAGS=()
    TARGET_DIR="target/debug"
    ;;
  release)
    CARGO_FLAGS=(--release)
    TARGET_DIR="target/release"
    ;;
  *)
    echo "usage: $0 [debug|release]" >&2
    exit 1
    ;;
esac

cargo build "${CARGO_FLAGS[@]}" -p reta --lib
cargo build "${CARGO_FLAGS[@]}" -p retaprompt --lib

CC_BIN="${CC:-cc}"
AR_BIN="${AR:-ar}"
SHIM_SRC="crates/retaprompt/src/retaprompt_shim.c"
SHIM_OBJ="$TARGET_DIR/retaprompt_shim.o"
OUT_LIB="$TARGET_DIR/libretaprompt.a"

"$CC_BIN" -c "$SHIM_SRC" -o "$SHIM_OBJ"
rm -f "$OUT_LIB"
"$AR_BIN" rcs "$OUT_LIB" "$SHIM_OBJ"

echo "built $OUT_LIB"
echo "note: link libretaprompt.a together with $TARGET_DIR/libreta.a"
