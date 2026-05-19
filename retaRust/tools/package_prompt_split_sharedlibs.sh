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

copy_required_dir() {
  local source="$1"
  local dest="$2"
  if [[ ! -d "$source" ]]; then
    echo "missing package input directory: $source" >&2
    exit 1
  fi
  rm -rf "$dest"
  cp -R "$source" "$dest"
}

copy_required "$TARGET_DIR/libreta.so" "$OUT_DIR/"
copy_required "$TARGET_DIR/rreta" "$OUT_DIR/"
copy_required "$TARGET_DIR/libretaprompt_input.so" "$OUT_DIR/"
copy_required "$TARGET_DIR/libretaprompt_commands.so" "$OUT_DIR/"
copy_required "$TARGET_DIR/rrp" "$OUT_DIR/"
copy_required "$TARGET_DIR/rrpl" "$OUT_DIR/"
copy_required "$TARGET_DIR/rrpe" "$OUT_DIR/"
copy_required "$TARGET_DIR/rrpb" "$OUT_DIR/"
copy_required_dir "$TARGET_DIR/csv" "$OUT_DIR/csv"
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
- rreta
- libreta.so
- libretaprompt_commands.so
- libretaprompt_input.so
- rrp
- rrpl
- rrpe
- rrpb
- csv/religion.csv and the other runtime CSV files

2. Executables in one directory and libraries in ./lib:
- ./rreta ./rrp ./rrpl ./rrpe ./rrpb
- ./lib/libreta.so
- ./lib/libretaprompt_commands.so
- ./lib/libretaprompt_input.so
- ./csv/religion.csv and the other runtime CSV files

3. Executables in bin and libraries in ../lib relative to bin:
- ./bin/rreta ./bin/rrp ./bin/rrpl ./bin/rrpe ./bin/rrpb
- ./lib/libreta.so
- ./lib/libretaprompt_commands.so
- ./lib/libretaprompt_input.so
- ./csv/religion.csv or ./share/reta/csv/religion.csv depending on install layout

Embedded shared-library search paths:
- $ORIGIN
- $ORIGIN/lib
- $ORIGIN/../lib

Runtime CSV search order:
- RETA_CSV_PATH
- executable directory/csv
- executable directory/../csv
- executable directory/share/reta/csv
- executable directory/../share/reta/csv
- executable directory/../../csv
- current working directory/csv

Dependency chain:
- rreta -> libreta.so
- rrp/rrpl/rrpe -> libretaprompt_input.so + libretaprompt_commands.so
- rrpb -> libretaprompt_commands.so

Artifact rule:
- This package intentionally contains only dynamic .so libraries and C launchers.
- Static archives (.a) are not part of this package.
- Rust prompt frontend executables are not built by default in build.sh.
LAYOUT

printf 'packaged dynamic split shared libraries and launchers in:\n'
printf '  %s\n' "$OUT_DIR"
