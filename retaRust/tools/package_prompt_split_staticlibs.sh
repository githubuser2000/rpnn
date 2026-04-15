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

./tools/build_prompt_split_staticlibs.sh "$PROFILE"

TARGET_DIR="target/$PROFILE"
OUT_DIR="$TARGET_DIR/retaprompt_split_staticlibs_package"
mkdir -p "$OUT_DIR/include"

cp "$TARGET_DIR/libreta.a" "$OUT_DIR/"
cp "$TARGET_DIR/libretaprompt_input.a" "$OUT_DIR/"
cp "$TARGET_DIR/libretaprompt_commands.a" "$OUT_DIR/"
cp crates/retaprompt_input/include/retaprompt_input.h "$OUT_DIR/include/"
cp crates/retaprompt_commands/include/retaprompt_commands.h "$OUT_DIR/include/"
cp RETAPROMPT_STATICLIB.md "$OUT_DIR/"
cp "$TARGET_DIR/retaprompt_split_staticlibs_manifest.json" "$OUT_DIR/"

if [[ ! -f "$OUT_DIR/retaprompt_split_staticlibs_manifest.json" ]]; then
  echo "missing packaged manifest" >&2
  exit 1
fi

cat > "$OUT_DIR/LINK_ORDER.txt" <<'ORDER'
libretaprompt_input.a
libretaprompt_commands.a
libreta.a
ORDER

printf 'packaged split static libraries in:\n'
printf '  %s\n' "$OUT_DIR"
