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

: "${CC:=cc}"
: "${AR:=ar}"

cargo build "${CARGO_FLAGS[@]}" -p reta --lib
cargo build "${CARGO_FLAGS[@]}" -p retaprompt_input --lib
cargo build "${CARGO_FLAGS[@]}" -p retaprompt_commands --lib

mkdir -p "$TARGET_DIR/retaprompt-split-shims"

"$CC" -c crates/retaprompt_input/src/retaprompt_input_shim.c -o "$TARGET_DIR/retaprompt-split-shims/retaprompt_input_shim.o"
"$AR" rcs "$TARGET_DIR/libretaprompt_input.a" "$TARGET_DIR/retaprompt-split-shims/retaprompt_input_shim.o"

"$CC" -c crates/retaprompt_commands/src/retaprompt_commands_shim.c -o "$TARGET_DIR/retaprompt-split-shims/retaprompt_commands_shim.o"
"$AR" rcs "$TARGET_DIR/libretaprompt_commands.a" "$TARGET_DIR/retaprompt-split-shims/retaprompt_commands_shim.o"

echo "built split static archives:"
echo "  $TARGET_DIR/libreta.a"
echo "  $TARGET_DIR/libretaprompt_input.a"
echo "  $TARGET_DIR/libretaprompt_commands.a"
echo
echo "link order example:"
echo "  ... libretaprompt_input.a libretaprompt_commands.a libreta.a ..."
