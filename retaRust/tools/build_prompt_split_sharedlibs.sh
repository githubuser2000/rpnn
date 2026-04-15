#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PROFILE="${1:-debug}"
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

: "${CC:=cc}"
: "${NM:=nm}"
: "${READELF:=readelf}"

cargo build "${CARGO_FLAGS[@]}" \
  -p reta \
  -p retaprompt_commands \
  -p retaprompt_input \
  -p retaprompt_frontends

mkdir -p "$TARGET_DIR/retaprompt-split-shared"

CORE_SO="$TARGET_DIR/libreta.so"
INPUT_RUST_SO="$TARGET_DIR/libretaprompt_input_rust.so"
INPUT_SO="$TARGET_DIR/libretaprompt_input.so"
COMMANDS_RUST_SO="$TARGET_DIR/libretaprompt_commands_rust.so"
COMMANDS_SO="$TARGET_DIR/libretaprompt_commands.so"

if [[ ! -f "$CORE_SO" ]]; then
  echo "missing core shared library: $CORE_SO" >&2
  exit 1
fi

rename_if_needed() {
  local src="$1"
  local dst="$2"

  [[ -e "$src" ]] || return 0

  local src_real dst_real
  src_real="$(readlink -f "$src" 2>/dev/null || printf '%s\n' "$src")"
  dst_real="$(readlink -f "$dst" 2>/dev/null || printf '%s\n' "$dst")"

  if [[ "$src_real" == "$dst_real" ]]; then
    return 0
  fi

  rm -f "$dst"
  mv -f "$src" "$dst"
}

rename_if_needed "$TARGET_DIR/libretaprompt_input.so" "$INPUT_RUST_SO"
rename_if_needed "$TARGET_DIR/libretaprompt_commands.so" "$COMMANDS_RUST_SO"

build_shared_forwarder() {
  local source="$1"
  local output="$2"
  local soname="$3"
  shift 3
  "$CC" -fPIC -shared "$source" \
    -Wl,-soname,"$soname" \
    -Wl,--no-as-needed \
    -Wl,-rpath,'$ORIGIN' \
    -Wl,-rpath,'$ORIGIN/lib' \
    -Wl,-rpath,'$ORIGIN/../lib' \
    -L "$TARGET_DIR" "$@" \
    -o "$output"
}

build_shared_forwarder \
  crates/retaprompt_commands/src/retaprompt_commands_shim.c \
  "$COMMANDS_SO" \
  libretaprompt_commands.so \
  -lreta

build_shared_forwarder \
  crates/retaprompt_input/src/retaprompt_input_shim.c \
  "$INPUT_SO" \
  libretaprompt_input.so \
  -lretaprompt_commands \
  -lreta

build_launcher() {
  local source="$1"
  local output="$2"
  local libname="$3"
  "$CC" "$source" \
    -Wl,-rpath,'$ORIGIN' \
    -Wl,-rpath,'$ORIGIN/lib' \
    -Wl,-rpath,'$ORIGIN/../lib' \
    -L "$TARGET_DIR" -l"$libname" \
    -o "$output"
}

build_launcher tools/launchers/rp.c  "$TARGET_DIR/rp"  retaprompt_input
build_launcher tools/launchers/rpl.c "$TARGET_DIR/rpl" retaprompt_input
build_launcher tools/launchers/rpe.c "$TARGET_DIR/rpe" retaprompt_input
build_launcher tools/launchers/rpb.c "$TARGET_DIR/rpb" retaprompt_commands 

verify_defined_symbol() {
  local shared="$1"
  local symbol="$2"
  if ! "$NM" -D --defined-only "$shared" | awk '{print $3}' | grep -Fx "$symbol" >/dev/null; then
    echo "missing exported symbol $symbol in $shared" >&2
    exit 1
  fi
}

verify_needed_entry() {
  local binary="$1"
  local needed="$2"
  if ! "$READELF" -d "$binary" | grep -F "Shared library: [$needed]" >/dev/null; then
    echo "missing NEEDED entry $needed in $binary" >&2
    "$READELF" -d "$binary" | egrep 'NEEDED|RPATH|RUNPATH' || true
    exit 1
  fi
}

verify_defined_symbol "$COMMANDS_SO" "retaprompt_commands_run_rp_from_env"
verify_defined_symbol "$COMMANDS_SO" "retaprompt_commands_run_rpl_from_env"
verify_defined_symbol "$COMMANDS_SO" "retaprompt_commands_run_rpb_from_env"
verify_defined_symbol "$COMMANDS_SO" "retaprompt_commands_run_rpe_from_env"

verify_defined_symbol "$INPUT_SO" "retaprompt_input_run_rp_from_env"
verify_defined_symbol "$INPUT_SO" "retaprompt_input_run_rpl_from_env"
verify_defined_symbol "$INPUT_SO" "retaprompt_input_run_rpe_from_env"
verify_defined_symbol "$INPUT_SO" "retaprompt_input_run_launcher_kind_from_env"

verify_needed_entry "$COMMANDS_SO" "libreta.so"
verify_needed_entry "$INPUT_SO" "libretaprompt_commands.so"
verify_needed_entry "$TARGET_DIR/rp"  "libretaprompt_input.so"
verify_needed_entry "$TARGET_DIR/rpl" "libretaprompt_input.so"
verify_needed_entry "$TARGET_DIR/rpe" "libretaprompt_input.so"
verify_needed_entry "$TARGET_DIR/rpb" "libretaprompt_commands.so"

cat > "$TARGET_DIR/retaprompt_split_sharedlibs_manifest.json" <<MANIFEST
{
  "shared_libraries": [
    {
      "path": "$CORE_SO",
      "role": "core implementation"
    },
    {
      "path": "$COMMANDS_SO",
      "role": "command frontend layer",
      "depends_on": ["libreta.so"]
    },
    {
      "path": "$INPUT_SO",
      "role": "launcher/input layer",
      "depends_on": ["libretaprompt_commands.so"]
    }
  ],
  "launchers": [
    {
      "path": "$TARGET_DIR/rp",
      "depends_on": ["libretaprompt_input.so"]
    },
    {
      "path": "$TARGET_DIR/rpl",
      "depends_on": ["libretaprompt_input.so"]
    },
    {
      "path": "$TARGET_DIR/rpe",
      "depends_on": ["libretaprompt_input.so"]
    },
    {
      "path": "$TARGET_DIR/rpb",
      "depends_on": ["libretaprompt_input.so"]
    }
  ]
}
MANIFEST

printf 'built split shared libraries and launchers:\n'
printf '  %s\n' "$CORE_SO"
printf '  %s\n' "$COMMANDS_SO"
printf '  %s\n' "$INPUT_SO"
printf '  %s\n' "$TARGET_DIR/rp"
printf '  %s\n' "$TARGET_DIR/rpl"
printf '  %s\n' "$TARGET_DIR/rpe"
printf '  %s\n' "$TARGET_DIR/rpb"
printf '\nmanifest:\n  %s\n' "$TARGET_DIR/retaprompt_split_sharedlibs_manifest.json"
