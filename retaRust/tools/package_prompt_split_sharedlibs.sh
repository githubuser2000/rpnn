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
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR/include"

copy_required() {
  local source="$1"
  local dest="$2"
  if [[ ! -f "$source" ]]; then
    echo "missing package input: $source" >&2
    exit 1
  fi
  cp "$source" "$dest"
}

copy_required "$TARGET_DIR/libreta.so" "$OUT_DIR/"
copy_required "$TARGET_DIR/reta" "$OUT_DIR/"
copy_required "$TARGET_DIR/libretaprompt_input.so" "$OUT_DIR/"
copy_required "$TARGET_DIR/libretaprompt_commands.so" "$OUT_DIR/"
copy_required "$TARGET_DIR/rp" "$OUT_DIR/"
copy_required "$TARGET_DIR/rpl" "$OUT_DIR/"
copy_required "$TARGET_DIR/rpe" "$OUT_DIR/"
copy_required "$TARGET_DIR/rpb" "$OUT_DIR/"
copy_required crates/retaprompt_input/include/retaprompt_input.h "$OUT_DIR/include/"
copy_required crates/retaprompt_commands/include/retaprompt_commands.h "$OUT_DIR/include/"
copy_required "$TARGET_DIR/retaprompt_split_sharedlibs_manifest.json" "$OUT_DIR/"
copy_required RETAPROMPT_SHAREDLIB.md "$OUT_DIR/"
copy_required RETAPROMPT_STATICLIB.md "$OUT_DIR/STATIC_ARCHIVES_RETIRED.md"

for archive in \
  "$OUT_DIR/libreta.a" \
  "$OUT_DIR/libretaprompt_input.a" \
  "$OUT_DIR/libretaprompt_commands.a"; do
  if [[ -e "$archive" ]]; then
    echo "unexpected static archive in shared-library package: $archive" >&2
    exit 1
  fi
done

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
- rp/rpl/rpe -> libretaprompt_input.so
- rpb -> libretaprompt_commands.so

Artifact rule:
- This package intentionally contains only dynamic .so libraries and launchers.
- Static archives (.a) are not part of this package.
LAYOUT

printf 'packaged dynamic split shared libraries and launchers in:\n'
printf '  %s\n' "$OUT_DIR"
