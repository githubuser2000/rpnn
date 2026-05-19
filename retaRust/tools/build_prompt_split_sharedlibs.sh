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

guard_against_static_regression() {
  if grep -Eq 'crate-type[[:space:]]*=.*staticlib' Cargo.toml crates/*/Cargo.toml 2>/dev/null; then
    echo "dynamic .so build guard failed: staticlib crate-type found in Cargo.toml" >&2
    echo "remove the staticlib crate-type before using the retaPrompt shared-library path" >&2
    exit 1
  fi

  if grep -Eq 'prompt-abi' Cargo.toml 2>/dev/null; then
    echo "dynamic .so build guard failed: prompt-abi gating is present in Cargo.toml" >&2
    echo "the shared-library path keeps the public Rust modules available without prompt-abi gating" >&2
    exit 1
  fi
}

verify_file_exists() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "expected build artifact missing: $path" >&2
    exit 1
  fi
}

verify_dynamic_symbol() {
  local library="$1"
  local symbol="$2"

  if command -v nm >/dev/null 2>&1; then
    if ! nm -D --defined-only "$library" 2>/dev/null | awk '{print $NF}' | grep -Fxq "$symbol"; then
      echo "expected exported symbol missing from $library: $symbol" >&2
      exit 1
    fi
  elif command -v readelf >/dev/null 2>&1; then
    if ! readelf -Ws "$library" 2>/dev/null | awk '{print $8}' | grep -Fxq "$symbol"; then
      echo "expected exported symbol missing from $library: $symbol" >&2
      exit 1
    fi
  else
    echo "warning: neither nm nor readelf is available; exported-symbol check skipped" >&2
  fi
}

verify_dynamic_needed() {
  local executable="$1"
  shift

  if ! command -v readelf >/dev/null 2>&1; then
    echo "warning: readelf unavailable; skipping DT_NEEDED check for $executable" >&2
    return 0
  fi

  local needed
  needed="$(readelf -d "$executable" 2>/dev/null | awk '/NEEDED/ {print $0}')"
  local library
  for library in "$@"; do
    if ! grep -Fq "lib${library}.so" <<<"$needed"; then
      echo "expected DT_NEEDED dependency missing from $executable: lib${library}.so" >&2
      echo "$needed" >&2
      exit 1
    fi
  done
}

verify_dynamic_not_needed() {
  local executable="$1"
  shift

  if ! command -v readelf >/dev/null 2>&1; then
    return 0
  fi

  local needed
  needed="$(readelf -d "$executable" 2>/dev/null | awk '/NEEDED/ {print $0}')"
  local library
  for library in "$@"; do
    if grep -Fq "lib${library}.so" <<<"$needed"; then
      echo "unexpected DT_NEEDED dependency in $executable: lib${library}.so" >&2
      echo "$needed" >&2
      exit 1
    fi
  done
}

verify_no_static_archive() {
  local path="$1"
  if [[ -e "$path" ]]; then
    echo "unexpected static archive produced by shared-library build: $path" >&2
    exit 1
  fi
}

guard_against_static_regression

./build.sh "$PROFILE"

TARGET_DIR="target/$PROFILE"
MANIFEST="$TARGET_DIR/retaprompt_split_sharedlibs_manifest.json"

verify_file_exists "$TARGET_DIR/libreta.so"
verify_file_exists "$TARGET_DIR/libretaprompt_commands.so"
verify_file_exists "$TARGET_DIR/libretaprompt_input.so"
verify_file_exists "$TARGET_DIR/rreta"
verify_file_exists "$TARGET_DIR/csv/religion.csv"
verify_file_exists "$TARGET_DIR/rrp"
verify_file_exists "$TARGET_DIR/rrpl"
verify_file_exists "$TARGET_DIR/rrpe"
verify_file_exists "$TARGET_DIR/rrpb"

verify_no_static_archive "$TARGET_DIR/libreta.a"
verify_no_static_archive "$TARGET_DIR/libretaprompt_commands.a"
verify_no_static_archive "$TARGET_DIR/libretaprompt_input.a"

verify_dynamic_symbol "$TARGET_DIR/libreta.so" reta_run_and_print_from_env_ffi
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_commands.so" retaprompt_commands_run_kind_from_env
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_commands.so" retaprompt_commands_run_current_executable_from_env
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_commands.so" retaprompt_commands_run_rp_from_env
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_commands.so" retaprompt_commands_run_rpl_from_env
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_commands.so" retaprompt_commands_run_rpb_from_env
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_commands.so" retaprompt_commands_run_rpe_from_env
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_input.so" retaprompt_input_run_kind_from_env
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_input.so" retaprompt_input_run_current_executable_from_env
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_input.so" retaprompt_input_run_any_current_executable_from_env
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_input.so" retaprompt_input_run_launcher_kind_from_env
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_input.so" retaprompt_input_run_rp_from_env
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_input.so" retaprompt_input_run_rpl_from_env
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_input.so" retaprompt_input_run_rpe_from_env

verify_dynamic_needed "$TARGET_DIR/rreta" reta
verify_dynamic_needed "$TARGET_DIR/rrp" retaprompt_input retaprompt_commands
verify_dynamic_needed "$TARGET_DIR/rrpl" retaprompt_input retaprompt_commands
verify_dynamic_needed "$TARGET_DIR/rrpe" retaprompt_input retaprompt_commands
verify_dynamic_needed "$TARGET_DIR/rrpb" retaprompt_commands
verify_dynamic_not_needed "$TARGET_DIR/rrpb" retaprompt_input

cat > "$MANIFEST" <<MANIFEST_JSON
{
  "build_mode": "plain-cargo-cdylib-plus-c-launchers-with-split-prompt-dependencies",
  "artifact_type": "dynamic-shared-libraries",
  "static_archives_intentionally_not_built": true,
  "rust_frontend_executables_intentionally_not_built_by_default": true,
  "shared_libraries": [
    {
      "path": "$TARGET_DIR/libreta.so",
      "role": "reta core library in the current source layout",
      "required_symbols": ["reta_run_and_print_from_env_ffi"]
    },
    {
      "path": "$TARGET_DIR/libretaprompt_commands.so",
      "role": "retaPrompt command library for rrpb and the command side of rrp/rrpl/rrpe",
      "required_symbols": [
        "retaprompt_commands_run_kind_from_env",
        "retaprompt_commands_run_current_executable_from_env",
        "retaprompt_commands_run_rp_from_env",
        "retaprompt_commands_run_rpl_from_env",
        "retaprompt_commands_run_rpb_from_env",
        "retaprompt_commands_run_rpe_from_env"
      ]
    },
    {
      "path": "$TARGET_DIR/libretaprompt_input.so",
      "role": "retaPrompt input/autocomplete/autosuggest library for rrp, rrpl and rrpe",
      "required_symbols": [
        "retaprompt_input_run_kind_from_env",
        "retaprompt_input_run_current_executable_from_env",
        "retaprompt_input_run_any_current_executable_from_env",
        "retaprompt_input_run_launcher_kind_from_env",
        "retaprompt_input_run_rp_from_env",
        "retaprompt_input_run_rpl_from_env",
        "retaprompt_input_run_rpe_from_env"
      ]
    }
  ],
  "forbidden_static_archives": [
    "$TARGET_DIR/libreta.a",
    "$TARGET_DIR/libretaprompt_commands.a",
    "$TARGET_DIR/libretaprompt_input.a"
  ],
  "runtime_data": [
    "$TARGET_DIR/csv"
  ],
  "launchers": [
    {
      "path": "$TARGET_DIR/rreta",
      "links_to": ["libreta.so"]
    },
    {
      "path": "$TARGET_DIR/rrp",
      "links_to": ["libretaprompt_input.so", "libretaprompt_commands.so"]
    },
    {
      "path": "$TARGET_DIR/rrpl",
      "links_to": ["libretaprompt_input.so", "libretaprompt_commands.so"]
    },
    {
      "path": "$TARGET_DIR/rrpe",
      "links_to": ["libretaprompt_input.so", "libretaprompt_commands.so"]
    },
    {
      "path": "$TARGET_DIR/rrpb",
      "links_to": ["libretaprompt_commands.so"],
      "must_not_link_to": ["libretaprompt_input.so"]
    }
  ],
  "notes": [
    "The prompt executables are C launchers, not Rust frontend binaries.",
    "rrp, rrpl and rrpe intentionally carry DT_NEEDED entries for both prompt split libraries.",
    "rrpb intentionally carries only the command library dependency.",
    "The current Rust cdylib source layout can still duplicate Rust dependency code inside the shared objects themselves; this build removes that code from the executables and verifies launcher-level dynamic dependencies."
  ]
}
MANIFEST_JSON

printf 'built dynamic shared libraries and launchers with split prompt dependencies:\n'
printf '  %s\n' "$TARGET_DIR/libreta.so"
printf '  %s\n' "$TARGET_DIR/libretaprompt_commands.so"
printf '  %s\n' "$TARGET_DIR/libretaprompt_input.so"
printf '  %s\n' "$TARGET_DIR/rreta"
printf '  %s\n' "$TARGET_DIR/csv"
printf '  %s -> %s + %s\n' "$TARGET_DIR/rrp" "libretaprompt_input.so" "libretaprompt_commands.so"
printf '  %s -> %s + %s\n' "$TARGET_DIR/rrpl" "libretaprompt_input.so" "libretaprompt_commands.so"
printf '  %s -> %s + %s\n' "$TARGET_DIR/rrpe" "libretaprompt_input.so" "libretaprompt_commands.so"
printf '  %s -> %s\n' "$TARGET_DIR/rrpb" "libretaprompt_commands.so"
printf '\nstatic archives intentionally not built:\n'
printf '  %s\n' "$TARGET_DIR/libreta.a"
printf '  %s\n' "$TARGET_DIR/libretaprompt_commands.a"
printf '  %s\n' "$TARGET_DIR/libretaprompt_input.a"
printf '\nmanifest:\n  %s\n' "$MANIFEST"
