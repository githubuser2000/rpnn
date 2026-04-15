#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PROFILE="${1:-debug}"
case "$PROFILE" in
  debug)
    CARGO_FLAGS=()
    ;;
  release)
    CARGO_FLAGS=(--release)
    ;;
  *)
    echo "usage: $0 [debug|release]" >&2
    exit 1
    ;;
esac

cargo build "${CARGO_FLAGS[@]}" -p reta --lib
cargo build "${CARGO_FLAGS[@]}" -p retaprompt_input --lib
cargo build "${CARGO_FLAGS[@]}" -p retaprompt_commands --lib
cargo build "${CARGO_FLAGS[@]}" -p retaprompt_frontends --bins

echo "built split prompt layers on top of libreta"
echo "note: there is no mixed retaprompt staticlib in this layout"
