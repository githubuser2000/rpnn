#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PROFILE="${1:-release}"
case "$PROFILE" in
  debug|release)
    ;;
  *)
    echo "usage: $0 [debug|release]" >&2
    exit 1
    ;;
esac

cat >&2 <<MESSAGE
retaprompt staticlib builds are intentionally disabled.

Use the dynamic shared-library build instead:
  ./tools/build_prompt_split_sharedlibs.sh $PROFILE

The active ABI artifacts are:
  target/$PROFILE/libreta.so
  target/$PROFILE/libretaprompt_commands.so
  target/$PROFILE/libretaprompt_input.so
MESSAGE

exec ./tools/build_prompt_split_sharedlibs.sh "$PROFILE"
