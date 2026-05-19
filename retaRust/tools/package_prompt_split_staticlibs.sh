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
static retaPrompt/Reta packaging is retired in this source tree.

Use the dynamic .so package with the Reta core split and prompt split instead:
  ./tools/package_prompt_split_sharedlibs.sh $PROFILE

This compatibility wrapper delegates to the shared-library package and does not
create or copy any .a archive.
MESSAGE

exec ./tools/package_prompt_split_sharedlibs.sh "$PROFILE"
