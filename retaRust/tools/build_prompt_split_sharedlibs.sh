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

./build.sh "$PROFILE"

TARGET_DIR="target/$PROFILE"
MANIFEST="$TARGET_DIR/retaprompt_split_sharedlibs_manifest.json"

cat > "$MANIFEST" <<MANIFEST_JSON
{
  "build_mode": "plain-cargo-cdylib-plus-c-launchers",
  "shared_libraries": [
    {
      "path": "$TARGET_DIR/libreta.so",
      "role": "reta core library in the current source layout"
    },
    {
      "path": "$TARGET_DIR/libretaprompt_commands.so",
      "role": "retaPrompt command library with the public commands ABI"
    },
    {
      "path": "$TARGET_DIR/libretaprompt_input.so",
      "role": "retaPrompt input/launcher library with the public input ABI"
    }
  ],
  "launchers": [
    {
      "path": "$TARGET_DIR/rp",
      "links_to": ["libretaprompt_input.so"]
    },
    {
      "path": "$TARGET_DIR/rpl",
      "links_to": ["libretaprompt_input.so"]
    },
    {
      "path": "$TARGET_DIR/rpe",
      "links_to": ["libretaprompt_input.so"]
    },
    {
      "path": "$TARGET_DIR/rpb",
      "links_to": ["libretaprompt_commands.so"]
    }
  ],
  "notes": [
    "This wrapper intentionally does not rebuild shim libraries.",
    "It preserves the simple cargo-build-plus-launcher-link flow from build.sh.",
    "In the current Rust source layout this build does not remove duplicate code between cdylibs."
  ]
}
MANIFEST_JSON

printf 'built shared libraries and launchers with the plain build path:\n'
printf '  %s\n' "$TARGET_DIR/libreta.so"
printf '  %s\n' "$TARGET_DIR/libretaprompt_commands.so"
printf '  %s\n' "$TARGET_DIR/libretaprompt_input.so"
printf '  %s\n' "$TARGET_DIR/rp"
printf '  %s\n' "$TARGET_DIR/rpl"
printf '  %s\n' "$TARGET_DIR/rpe"
printf '  %s\n' "$TARGET_DIR/rpb"
printf '\nmanifest:\n  %s\n' "$MANIFEST"
