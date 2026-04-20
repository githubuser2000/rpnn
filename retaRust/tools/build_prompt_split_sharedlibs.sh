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
    echo "the less-invasive path keeps the public Rust modules available without prompt-abi gating" >&2
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
verify_file_exists "$TARGET_DIR/rp"
verify_file_exists "$TARGET_DIR/rpl"
verify_file_exists "$TARGET_DIR/rpe"
verify_file_exists "$TARGET_DIR/rpb"

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

cat > "$MANIFEST" <<MANIFEST_JSON
{
  "build_mode": "plain-cargo-cdylib-plus-c-launchers",
  "artifact_type": "dynamic-shared-libraries",
  "static_archives_intentionally_not_built": true,
  "shared_libraries": [
    {
      "path": "$TARGET_DIR/libreta.so",
      "role": "reta core library in the current source layout",
      "required_symbols": ["reta_run_and_print_from_env_ffi"]
    },
    {
      "path": "$TARGET_DIR/libretaprompt_commands.so",
      "role": "retaPrompt command library with the public commands ABI",
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
      "role": "retaPrompt input/launcher library with the public input ABI",
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
    "This wrapper intentionally does not rebuild shim libraries or static archives.",
    "It preserves the simple cargo-build-plus-launcher-link flow from build.sh.",
    "In the current Rust source layout this build does not remove duplicate code between cdylibs."
  ]
}
MANIFEST_JSON

printf 'built dynamic shared libraries and launchers with the plain build path:\n'
printf '  %s\n' "$TARGET_DIR/libreta.so"
printf '  %s\n' "$TARGET_DIR/libretaprompt_commands.so"
printf '  %s\n' "$TARGET_DIR/libretaprompt_input.so"
printf '  %s\n' "$TARGET_DIR/rp"
printf '  %s\n' "$TARGET_DIR/rpl"
printf '  %s\n' "$TARGET_DIR/rpe"
printf '  %s\n' "$TARGET_DIR/rpb"
printf '\nstatic archives intentionally not built:\n'
printf '  %s\n' "$TARGET_DIR/libreta.a"
printf '  %s\n' "$TARGET_DIR/libretaprompt_commands.a"
printf '  %s\n' "$TARGET_DIR/libretaprompt_input.a"
printf '\nmanifest:\n  %s\n' "$MANIFEST"
