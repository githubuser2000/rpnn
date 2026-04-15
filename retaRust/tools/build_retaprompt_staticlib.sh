#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PROFILE="${1:-debug}"
TARGET="${2:-lib}"

case "$PROFILE" in
  debug)
    PROFILE_ARGS=()
    ;;
  release)
    PROFILE_ARGS=(--release)
    ;;
  *)
    echo "usage: $0 [debug|release] [lib|all]" >&2
    exit 1
    ;;
esac

cargo build -p retaprompt --lib "${PROFILE_ARGS[@]}"

case "$TARGET" in
  lib)
    ;;
  all)
    cargo build -p retaprompt_frontends --bin rp  "${PROFILE_ARGS[@]}"
    cargo build -p retaprompt_frontends --bin rpl "${PROFILE_ARGS[@]}"
    cargo build -p retaprompt_frontends --bin rpb "${PROFILE_ARGS[@]}"
    cargo build -p retaprompt_frontends --bin rpe "${PROFILE_ARGS[@]}"
    ;;
  *)
    echo "usage: $0 [debug|release] [lib|all]" >&2
    exit 1
    ;;
esac
