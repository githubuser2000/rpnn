#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PROFILE="${1:-debug}"
case "$PROFILE" in
  debug)
    cargo build -p retaprompt --lib
    ;;
  release)
    cargo build -p retaprompt --release --lib
    ;;
  *)
    echo "usage: $0 [debug|release]" >&2
    exit 1
    ;;
esac
