#!/usr/bin/env bash
# Shell variable documentation: RETA_SHELL_VARIABLES_DE.md and RETA_SHELL_VARIABLES_EN.md
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
static retaPrompt/Reta archives are retired in this source tree.

This project now keeps both splits on the dynamic .so path:
  ./tools/build_prompt_split_sharedlibs.sh $PROFILE

The active ABI artifacts are:
  target/$PROFILE/libreta.so
  target/$PROFILE/libreta_data.so
  target/$PROFILE/libreta_parse.so
  target/$PROFILE/libreta_semantics.so
  target/$PROFILE/libreta_table.so
  target/$PROFILE/libreta_render.so
  target/$PROFILE/libreta_arch.so
  target/$PROFILE/libreta_runtime.so
  target/$PROFILE/rgrundStrukHtml
  target/$PROFILE/libretaprompt_commands.so
  target/$PROFILE/libretaprompt_input.so

No lib*.a archive is built by this compatibility wrapper. This intentionally
prevents the old static archive path from drifting away from the shared-library
architecture again.
MESSAGE

exec ./tools/build_prompt_split_sharedlibs.sh "$PROFILE"
