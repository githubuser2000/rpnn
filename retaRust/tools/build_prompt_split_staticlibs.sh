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
: "${AR:=ar}"
: "${NM:=nm}"

cargo build "${CARGO_FLAGS[@]}" -p reta --lib
cargo build "${CARGO_FLAGS[@]}" -p retaprompt_input --lib
cargo build "${CARGO_FLAGS[@]}" -p retaprompt_commands --lib

mkdir -p "$TARGET_DIR/retaprompt-split-shims"

"$CC" -c crates/retaprompt_input/src/retaprompt_input_shim.c \
  -o "$TARGET_DIR/retaprompt-split-shims/retaprompt_input_shim.o"
"$AR" rcs "$TARGET_DIR/libretaprompt_input.a" \
  "$TARGET_DIR/retaprompt-split-shims/retaprompt_input_shim.o"

"$CC" -c crates/retaprompt_commands/src/retaprompt_commands_shim.c \
  -o "$TARGET_DIR/retaprompt-split-shims/retaprompt_commands_shim.o"
"$AR" rcs "$TARGET_DIR/libretaprompt_commands.a" \
  "$TARGET_DIR/retaprompt-split-shims/retaprompt_commands_shim.o"

verify_archive_has_single_forwarder_object() {
  local archive="$1"
  local expected_object="$2"
  local listed
  listed="$("$AR" t "$archive")"
  if [[ "$listed" != "$expected_object" ]]; then
    echo "archive verification failed for $archive" >&2
    echo "expected only: $expected_object" >&2
    echo "got: $listed" >&2
    exit 1
  fi
}

verify_archive_symbols() {
  local archive="$1"
  shift
  local expected_defined=("$@")
  local nm_out
  nm_out="$("$NM" -g "$archive")"
  for symbol in "${expected_defined[@]}"; do
    if ! grep -Eq "[[:space:]]T[[:space:]]${symbol}$" <<<"$nm_out"; then
      echo "missing exported symbol $symbol in $archive" >&2
      exit 1
    fi
  done
}

verify_archive_has_only_expected_defined_symbols() {
  local archive="$1"
  shift
  local expected_defined=("$@")
  local nm_out
  local actual_defined
  local expected_sorted
  nm_out="$("$NM" -g "$archive")"
  actual_defined="$({ grep -E "[[:space:]]T[[:space:]]" <<<"$nm_out" || true; } | awk '{print $3}' | sort)"
  expected_sorted="$(printf '%s
' "${expected_defined[@]}" | sort)"
  if [[ "$actual_defined" != "$expected_sorted" ]]; then
    echo "defined-symbol verification failed for $archive" >&2
    echo "expected defined symbols:" >&2
    printf '  %s
' "${expected_defined[@]}" >&2
    echo "actual defined symbols:" >&2
    if [[ -n "$actual_defined" ]]; then
      while IFS= read -r line; do
        printf '  %s
' "$line" >&2
      done <<<"$actual_defined"
    else
      echo "  <none>" >&2
    fi
    exit 1
  fi
}

write_split_manifest() {
  local manifest="$1"
  cat >"$manifest" <<EOF
{
  "archives": [
    {
      "path": "$TARGET_DIR/libreta.a",
      "role": "core implementation",
      "contains_forwarder_only": false
    },
    {
      "path": "$TARGET_DIR/libretaprompt_input.a",
      "role": "own command input for rp/rpl/rpe",
      "contains_forwarder_only": true,
      "object": "retaprompt_input_shim.o",
      "defined_symbols": [
        "retaprompt_input_run_rp_from_env",
        "retaprompt_input_run_rpl_from_env",
        "retaprompt_input_run_rpe_from_env"
      ]
    },
    {
      "path": "$TARGET_DIR/libretaprompt_commands.a",
      "role": "command frontend for rp/rpl/rpe/rpb",
      "contains_forwarder_only": true,
      "object": "retaprompt_commands_shim.o",
      "defined_symbols": [
        "retaprompt_commands_run_rp_from_env",
        "retaprompt_commands_run_rpl_from_env",
        "retaprompt_commands_run_rpb_from_env",
        "retaprompt_commands_run_rpe_from_env"
      ]
    }
  ]
}
EOF
}

verify_archive_has_single_forwarder_object \
  "$TARGET_DIR/libretaprompt_input.a" \
  "retaprompt_input_shim.o"
verify_archive_has_single_forwarder_object \
  "$TARGET_DIR/libretaprompt_commands.a" \
  "retaprompt_commands_shim.o"

verify_archive_symbols "$TARGET_DIR/libretaprompt_input.a" \
  retaprompt_input_run_rp_from_env \
  retaprompt_input_run_rpl_from_env \
  retaprompt_input_run_rpe_from_env
verify_archive_symbols "$TARGET_DIR/libretaprompt_commands.a" \
  retaprompt_commands_run_rp_from_env \
  retaprompt_commands_run_rpl_from_env \
  retaprompt_commands_run_rpb_from_env \
  retaprompt_commands_run_rpe_from_env

printf 'built split static archives:\n'
printf '  %s\n' "$TARGET_DIR/libreta.a"
printf '  %s\n' "$TARGET_DIR/libretaprompt_input.a"
printf '  %s\n' "$TARGET_DIR/libretaprompt_commands.a"
printf '\n'
printf 'archive members:\n'
printf '  libreta.a -> Rust implementation from crate reta\n'
printf '  libretaprompt_input.a -> retaprompt_input_shim.o only\n'
printf '  libretaprompt_commands.a -> retaprompt_commands_shim.o only\n'
printf '\n'
printf 'link order example:\n'
printf '  ... libretaprompt_input.a libretaprompt_commands.a libreta.a ...\n'
