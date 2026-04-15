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

./tools/build_prompt_split_sharedlibs.sh "$PROFILE"

TARGET_DIR="target/$PROFILE"
OUT_DIR="$TARGET_DIR/retaprompt_split_sharedlibs_package"
mkdir -p "$OUT_DIR/include"

cp "$TARGET_DIR/libreta.so" "$OUT_DIR/"
cp "$TARGET_DIR/reta" "$OUT_DIR/"
cp "$TARGET_DIR/libretaprompt_input.so" "$OUT_DIR/"
cp "$TARGET_DIR/libretaprompt_commands.so" "$OUT_DIR/"
cp "$TARGET_DIR/rp" "$OUT_DIR/"
cp "$TARGET_DIR/rpl" "$OUT_DIR/"
cp "$TARGET_DIR/rpe" "$OUT_DIR/"
cp "$TARGET_DIR/rpb" "$OUT_DIR/"
cp crates/retaprompt_input/include/retaprompt_input.h "$OUT_DIR/include/"
cp crates/retaprompt_commands/include/retaprompt_commands.h "$OUT_DIR/include/"
cp "$TARGET_DIR/retaprompt_split_sharedlibs_manifest.json" "$OUT_DIR/"
cp RETAPROMPT_SHAREDLIB.md "$OUT_DIR/"

cat > "$OUT_DIR/RUN_LAYOUT.txt" <<'LAYOUT'
Supported runtime layouts:

1. All files in one directory:
- reta
- libreta.so
- libretaprompt_commands.so
- libretaprompt_input.so
- rp
- rpl
- rpe
- rpb

2. Executables in one directory and libraries in ./lib:
- ./reta ./rp ./rpl ./rpe ./rpb
- ./lib/libreta.so
- ./lib/libretaprompt_commands.so
- ./lib/libretaprompt_input.so

3. Executables in bin and libraries in ../lib relative to bin:
- ./bin/reta ./bin/rp ./bin/rpl ./bin/rpe ./bin/rpb
- ./lib/libreta.so
- ./lib/libretaprompt_commands.so
- ./lib/libretaprompt_input.so

Embedded search paths:
- $ORIGIN
- $ORIGIN/lib
- $ORIGIN/../lib

Dependency chain:
- reta -> libreta.so
- rp/rpl/rpe -> libretaprompt_input.so -> libretaprompt_commands.so -> libreta.so
- rpb -> libretaprompt_commands.so -> libreta.so
LAYOUT

printf 'packaged split shared libraries and launchers in:\n'
printf '  %s\n' "$OUT_DIR"
