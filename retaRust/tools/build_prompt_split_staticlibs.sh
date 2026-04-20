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
static retaPrompt archives are retired in this source tree.

This project now keeps the retaPrompt split on the dynamic .so path:
  ./tools/build_prompt_split_sharedlibs.sh $PROFILE

No libretaprompt_input.a, libretaprompt_commands.a, or libreta.a is built by
this compatibility wrapper. This intentionally prevents the old static archive
path from drifting away from the Python/Rust architecture again.
MESSAGE

exec ./tools/build_prompt_split_sharedlibs.sh "$PROFILE"
