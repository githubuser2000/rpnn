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

# Build the real Rust libraries and the non-prompt reta binaries.  The prompt
# executables rr{p,pl,pe,pb} are rebuilt below as tiny C launchers so the code
# lives in the shared objects instead of in Rust executable bodies.
if [[ "${RETA_BUILD_RUST_FRONTEND_BINS:-0}" == "1" ]]; then
  cargo build --workspace --features retaprompt_frontends/rust-frontends "${CARGO_FLAGS[@]}"
else
  cargo build \
    -p reta \
    -p reta_architecture \
    -p retaprompt_commands \
    -p retaprompt_input \
    "${CARGO_FLAGS[@]}"
fi

mkdir -p "$TARGET_DIR"

link_launcher() {
  local source="$1"
  local output="$2"
  shift 2

  if [[ "$#" -eq 0 ]]; then
    echo "link_launcher requires at least one library for $output" >&2
    exit 1
  fi

  local library_args=()
  local library
  for library in "$@"; do
    library_args+=("-l${library}")
  done

  cc "$source" \
    -o "$output" \
    -L"$TARGET_DIR" \
    -Wl,--no-as-needed \
    "${library_args[@]}" \
    -Wl,--as-needed \
    -Wl,-rpath,'$ORIGIN' \
    -Wl,-rpath,'$ORIGIN/lib' \
    -Wl,-rpath,'$ORIGIN/../lib'
}

verify_needed() {
  local output="$1"
  shift

  if ! command -v readelf >/dev/null 2>&1; then
    echo "warning: readelf unavailable; skipping DT_NEEDED check for $output" >&2
    return 0
  fi

  local needed
  needed="$(readelf -d "$output" 2>/dev/null | awk '/NEEDED/ {print $0}')"
  local library
  for library in "$@"; do
    if ! grep -Fq "lib${library}.so" <<<"$needed"; then
      echo "launcher dependency check failed: $output does not need lib${library}.so" >&2
      echo "$needed" >&2
      exit 1
    fi
  done
}

verify_not_needed() {
  local output="$1"
  shift

  if ! command -v readelf >/dev/null 2>&1; then
    return 0
  fi

  local needed
  needed="$(readelf -d "$output" 2>/dev/null | awk '/NEEDED/ {print $0}')"
  local library
  for library in "$@"; do
    if grep -Fq "lib${library}.so" <<<"$needed"; then
      echo "launcher dependency check failed: $output unexpectedly needs lib${library}.so" >&2
      echo "$needed" >&2
      exit 1
    fi
  done
}

link_launcher tools/launchers/reta.c "$TARGET_DIR/rreta" reta

# rrpb is command-only.  The interactive prompt launchers need both split
# prompt libraries: libretaprompt_input.so for line input/autocomplete/
# autosuggest, and libretaprompt_commands.so for command parsing/execution.
link_launcher tools/launchers/rp.c  "$TARGET_DIR/rrp"  retaprompt_input retaprompt_commands
link_launcher tools/launchers/rpl.c "$TARGET_DIR/rrpl" retaprompt_input retaprompt_commands
link_launcher tools/launchers/rpe.c "$TARGET_DIR/rrpe" retaprompt_input retaprompt_commands
link_launcher tools/launchers/rpb.c "$TARGET_DIR/rrpb" retaprompt_commands

verify_needed "$TARGET_DIR/rreta" reta
verify_needed "$TARGET_DIR/rrp"  retaprompt_input retaprompt_commands
verify_needed "$TARGET_DIR/rrpl" retaprompt_input retaprompt_commands
verify_needed "$TARGET_DIR/rrpe" retaprompt_input retaprompt_commands
verify_needed "$TARGET_DIR/rrpb" retaprompt_commands
verify_not_needed "$TARGET_DIR/rrpb" retaprompt_input

copy_runtime_data() {
  if [[ -d csv ]]; then
    rm -rf "$TARGET_DIR/csv"
    cp -R csv "$TARGET_DIR/csv"
  fi
}

copy_runtime_data
cat <<BUILD_COMPLETE
Build complete: $TARGET_DIR
  rreta -> libreta.so
  rrp  -> libretaprompt_input.so + libretaprompt_commands.so
  rrpl -> libretaprompt_input.so + libretaprompt_commands.so
  rrpe -> libretaprompt_input.so + libretaprompt_commands.so
  rrpb -> libretaprompt_commands.so
BUILD_COMPLETE
