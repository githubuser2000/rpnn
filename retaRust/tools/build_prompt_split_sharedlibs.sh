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

cargo build "${CARGO_FLAGS[@]}" -p reta --lib

mkdir -p "$TARGET_DIR/retaprompt-split-shared"

CORE_SO="$TARGET_DIR/libreta.so"
INPUT_SO="$TARGET_DIR/libretaprompt_input.so"
COMMANDS_SO="$TARGET_DIR/libretaprompt_commands.so"

if [[ ! -f "$CORE_SO" ]]; then
  echo "missing core shared library: $CORE_SO" >&2
  echo "hint: crate reta must build cdylib output libreta.so" >&2
  exit 1
fi

build_shared_forwarder() {
  local source="$1"
  local output="$2"
  local soname="$3"
  shift 3
  "$CC" -fPIC -shared "$source" \
    -Wl,-soname,"$soname" \
    -Wl,-rpath,'$ORIGIN' \
    -L "$TARGET_DIR" -lreta \
    -o "$output"
}

build_shared_forwarder \
  crates/retaprompt_input/src/retaprompt_input_shim.c \
  "$INPUT_SO" \
  libretaprompt_input.so

build_shared_forwarder \
  crates/retaprompt_commands/src/retaprompt_commands_shim.c \
  "$COMMANDS_SO" \
  libretaprompt_commands.so

build_launcher() {
  local source="$1"
  local output="$2"
  local libname="$3"
  "$CC" "$source" \
    -Wl,-rpath,'$ORIGIN' \
    -L "$TARGET_DIR" -l"$libname" \
    -o "$output"
}

build_launcher tools/launchers/rp.c "$TARGET_DIR/rp" retaprompt_input
build_launcher tools/launchers/rpl.c "$TARGET_DIR/rpl" retaprompt_input
build_launcher tools/launchers/rpe.c "$TARGET_DIR/rpe" retaprompt_input
build_launcher tools/launchers/rpb.c "$TARGET_DIR/rpb" retaprompt_commands

verify_shared_defined_symbols() {
  local shared="$1"
  shift
  local expected=("$@")
  local actual
  actual="$($NM -D --defined-only "$shared" | awk '{print $3}' | sort)"
  local wanted
  wanted="$(printf '%s\n' "${expected[@]}" | sort)"
  if [[ "$actual" != "$wanted" ]]; then
    echo "defined-symbol verification failed for $shared" >&2
    echo "expected:" >&2
    printf '  %s\n' "${expected[@]}" >&2
    echo "actual:" >&2
    if [[ -n "$actual" ]]; then
      while IFS= read -r line; do printf '  %s\n' "$line" >&2; done <<<"$actual"
    else
      echo "  <none>" >&2
    fi
    exit 1
  fi
}

verify_needed_entry() {
  local binary="$1"
  local needed="$2"
  if ! "$READELF" -d "$binary" | grep -F "Shared library: [$needed]" >/dev/null; then
    echo "missing NEEDED entry $needed in $binary" >&2
    exit 1
  fi
}

verify_shared_defined_symbols "$INPUT_SO" \
  retaprompt_input_run_rp_from_env \
  retaprompt_input_run_rpl_from_env \
  retaprompt_input_run_rpe_from_env
verify_shared_defined_symbols "$COMMANDS_SO" \
  retaprompt_commands_run_rp_from_env \
  retaprompt_commands_run_rpl_from_env \
  retaprompt_commands_run_rpb_from_env \
  retaprompt_commands_run_rpe_from_env

verify_needed_entry "$INPUT_SO" "libreta.so"
verify_needed_entry "$COMMANDS_SO" "libreta.so"
verify_needed_entry "$TARGET_DIR/rp" "libretaprompt_input.so"
verify_needed_entry "$TARGET_DIR/rpl" "libretaprompt_input.so"
verify_needed_entry "$TARGET_DIR/rpe" "libretaprompt_input.so"
verify_needed_entry "$TARGET_DIR/rpb" "libretaprompt_commands.so"

cat > "$TARGET_DIR/retaprompt_split_sharedlibs_manifest.json" <<MANIFEST
{
  "shared_libraries": [
    {
      "path": "$CORE_SO",
      "role": "core implementation",
      "contains_core_implementation": true
    },
    {
      "path": "$INPUT_SO",
      "role": "own command input for rp/rpl/rpe",
      "depends_on": ["libreta.so"],
      "exports": [
        "retaprompt_input_run_rp_from_env",
        "retaprompt_input_run_rpl_from_env",
        "retaprompt_input_run_rpe_from_env"
      ]
    },
    {
      "path": "$COMMANDS_SO",
      "role": "command frontend for rp/rpl/rpe/rpb",
      "depends_on": ["libreta.so"],
      "exports": [
        "retaprompt_commands_run_rp_from_env",
        "retaprompt_commands_run_rpl_from_env",
        "retaprompt_commands_run_rpb_from_env",
        "retaprompt_commands_run_rpe_from_env"
      ]
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
      "depends_on": ["libretaprompt_commands.so"]
    }
  ]
}
MANIFEST

printf 'built split shared libraries and launchers:\n'
printf '  %s\n' "$CORE_SO"
printf '  %s\n' "$INPUT_SO"
printf '  %s\n' "$COMMANDS_SO"
printf '  %s\n' "$TARGET_DIR/rp"
printf '  %s\n' "$TARGET_DIR/rpl"
printf '  %s\n' "$TARGET_DIR/rpe"
printf '  %s\n' "$TARGET_DIR/rpb"
printf '\nmanifest:\n  %s\n' "$TARGET_DIR/retaprompt_split_sharedlibs_manifest.json"
