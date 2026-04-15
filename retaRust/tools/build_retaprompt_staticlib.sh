#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PROFILE="${1:-debug}"
case "$PROFILE" in
  debug)
    cargo build -p retaprompt --lib
    cargo build -p rp --lib
    cargo build -p rpl --lib
    cargo build -p rpb --lib
    cargo build -p rpe --lib
    ;;
  release)
    cargo build -p retaprompt --release --lib
    cargo build -p rp --release --lib
    cargo build -p rpl --release --lib
    cargo build -p rpb --release --lib
    cargo build -p rpe --release --lib
    ;;
  *)
    echo "usage: $0 [debug|release]" >&2
    exit 1
    ;;
esac
