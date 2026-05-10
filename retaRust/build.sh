#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

PROFILE="${1:-release}"
case "$PROFILE" in
  debug)
    CARGO_FLAGS=()
    TARGET_DIR="target/debug"
    ;;
  release)
    CARGO_FLAGS=(--release)
    TARGET_DIR="target/release"
    ;;
  *)
    echo "usage: $0 [debug|release]" >&2
    exit 1
    ;;
esac

#cargo clean
cargo build --workspace "${CARGO_FLAGS[@]}"

mkdir -p "$TARGET_DIR"

link_launcher() {
  local source="$1"
  local output="$2"
  local library="$3"
  cc "$source" \
    -o "$output" \
    -L"$TARGET_DIR" \
    -l"$library" \
    -Wl,-rpath,'$ORIGIN' \
    -Wl,-rpath,'$ORIGIN/lib' \
    -Wl,-rpath,'$ORIGIN/../lib'
}

link_launcher tools/launchers/reta.c "$TARGET_DIR/rreta" reta
link_launcher tools/launchers/rp.c  "$TARGET_DIR/rrp"  retaprompt_input
link_launcher tools/launchers/rpl.c "$TARGET_DIR/rrpl" retaprompt_input
link_launcher tools/launchers/rpe.c "$TARGET_DIR/rrpe" retaprompt_input
link_launcher tools/launchers/rpb.c "$TARGET_DIR/rrpb" retaprompt_commands

copy_runtime_data() {
  if [[ -d csv ]]; then
    rm -rf "$TARGET_DIR/csv"
    cp -R csv "$TARGET_DIR/csv"
  fi
}

copy_runtime_data

[ "${PROFILE}" = "release" ] && for a in rreta rrp rrpl rrpe rrpb
do
	cargo run --release --bin $a -- -h
done for a in rreta rrp rrpl rrpe rrpb
do
	cargo run --bin $a -- -h

echo "Build complete: $TARGET_DIR"
