#!/usr/bin/env bash
# Shell variable documentation: RETA_SHELL_VARIABLES_DE.md and RETA_SHELL_VARIABLES_EN.md
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

TARGET_DIR="${CARGO_TARGET_DIR:-target}/$PROFILE"
MANIFEST="$TARGET_DIR/retaprompt_split_sharedlibs_manifest.json"

CORE_SPLIT_LIBRARIES=(
  reta_data
  reta_parse
  reta_semantics
  reta_table
  reta_render
  reta_arch
  reta_runtime
)

PROMPT_SPLIT_LIBRARIES=(
  retaprompt_commands
  retaprompt_input
)

guard_prompt_frontend_sources() {
  if command -v python3 >/dev/null 2>&1; then
    python3 tools/guard_prompt_frontend_sources.py
  else
    echo "warning: python3 unavailable; skipping prompt frontend source guard" >&2
  fi
}

guard_against_static_regression() {
  if grep -Eq 'crate-type[[:space:]]*=.*staticlib' Cargo.toml crates/*/Cargo.toml 2>/dev/null; then
    echo "dynamic .so build guard failed: staticlib crate-type found in Cargo.toml" >&2
    echo "remove the staticlib crate-type before using the shared-library path" >&2
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

verify_dynamic_symbol_absent() {
  local library="$1"
  local symbol="$2"

  if command -v nm >/dev/null 2>&1; then
    if nm -D --defined-only "$library" 2>/dev/null | awk '{print $NF}' | grep -Fxq "$symbol"; then
      echo "unexpected exported symbol in $library: $symbol" >&2
      exit 1
    fi
  elif command -v readelf >/dev/null 2>&1; then
    if readelf -Ws "$library" 2>/dev/null | awk '{print $8}' | grep -Fxq "$symbol"; then
      echo "unexpected exported symbol in $library: $symbol" >&2
      exit 1
    fi
  else
    echo "warning: neither nm nor readelf is available; exported-symbol absence check skipped" >&2
  fi
}

verify_facade_smaller_than_runtime() {
  local facade="$TARGET_DIR/libreta.so"
  local runtime="$TARGET_DIR/libreta_runtime.so"

  if [[ ! -f "$facade" || ! -f "$runtime" ]]; then
    echo "missing size-check input: $facade or $runtime" >&2
    exit 1
  fi

  local facade_size runtime_size
  facade_size=$(wc -c < "$facade")
  runtime_size=$(wc -c < "$runtime")
  if (( facade_size >= runtime_size )); then
    echo "split-size check failed: libreta.so ($facade_size bytes) is not smaller than libreta_runtime.so ($runtime_size bytes)" >&2
    exit 1
  fi
}

verify_component_size_diversity() {
  local libs=(
    "$TARGET_DIR/libreta_data.so"
    "$TARGET_DIR/libreta_parse.so"
    "$TARGET_DIR/libreta_semantics.so"
    "$TARGET_DIR/libreta_table.so"
    "$TARGET_DIR/libreta_render.so"
  )
  local first_size=""
  local all_same=1
  local lib size
  for lib in "${libs[@]}"; do
    verify_file_exists "$lib"
    size=$(wc -c < "$lib")
    if [[ -z "$first_size" ]]; then
      first_size="$size"
    elif [[ "$size" != "$first_size" ]]; then
      all_same=0
    fi
  done
  if [[ "$all_same" == "1" ]]; then
    echo "split-size check failed: data/parse/semantics/table/render .so files all have the same size ($first_size bytes)." >&2
    echo "The component libraries are expected to export real code, not identical ABI stubs." >&2
    exit 1
  fi
}

verify_dynamic_needed() {
  local binary="$1"
  shift

  if ! command -v readelf >/dev/null 2>&1; then
    echo "warning: readelf unavailable; skipping DT_NEEDED check for $binary" >&2
    return 0
  fi

  local needed
  needed="$(readelf -d "$binary" 2>/dev/null | awk '/NEEDED/ {print $0}')"
  local library
  for library in "$@"; do
    if ! grep -Fq "lib${library}.so" <<<"$needed"; then
      echo "expected DT_NEEDED dependency missing from $binary: lib${library}.so" >&2
      echo "$needed" >&2
      exit 1
    fi
  done
}

verify_dynamic_not_needed() {
  local binary="$1"
  shift

  if ! command -v readelf >/dev/null 2>&1; then
    return 0
  fi

  local needed
  needed="$(readelf -d "$binary" 2>/dev/null | awk '/NEEDED/ {print $0}')"
  local library
  for library in "$@"; do
    if grep -Fq "lib${library}.so" <<<"$needed"; then
      echo "unexpected DT_NEEDED dependency in $binary: lib${library}.so" >&2
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
guard_prompt_frontend_sources

./build.sh "$PROFILE"

verify_file_exists "$TARGET_DIR/libreta.so"
for library in "${CORE_SPLIT_LIBRARIES[@]}"; do
  verify_file_exists "$TARGET_DIR/lib${library}.so"
  verify_no_static_archive "$TARGET_DIR/lib${library}.a"
  verify_dynamic_symbol "$TARGET_DIR/lib${library}.so" "${library}_abi_anchor"
  verify_dynamic_symbol "$TARGET_DIR/lib${library}.so" "${library}_abi_manifest_json"
done

verify_file_exists "$TARGET_DIR/libretaprompt_commands.so"
verify_file_exists "$TARGET_DIR/libretaprompt_input.so"
verify_file_exists "$TARGET_DIR/rreta"
verify_file_exists "$TARGET_DIR/rgrundStrukHtml"
verify_file_exists "$TARGET_DIR/csv/religion.csv"
verify_file_exists "$TARGET_DIR/rrp"
verify_file_exists "$TARGET_DIR/rrpl"
verify_file_exists "$TARGET_DIR/rrpe"
verify_file_exists "$TARGET_DIR/rrpb"

verify_no_static_archive "$TARGET_DIR/libreta.a"
verify_no_static_archive "$TARGET_DIR/libretaprompt_commands.a"
verify_no_static_archive "$TARGET_DIR/libretaprompt_input.a"

verify_dynamic_symbol "$TARGET_DIR/libreta.so" reta_run_and_print_from_env_ffi
verify_dynamic_symbol "$TARGET_DIR/libreta.so" reta_core_split_abi_anchor
verify_dynamic_symbol "$TARGET_DIR/libreta.so" reta_core_split_abi_manifest_json
verify_dynamic_symbol "$TARGET_DIR/libreta_runtime.so" reta_runtime_core_run_and_print_from_env_ffi
verify_dynamic_symbol "$TARGET_DIR/libreta_runtime.so" reta_runtime_core_run_argv
verify_dynamic_symbol "$TARGET_DIR/libreta_runtime.so" reta_runtime_core_free_string
verify_dynamic_symbol "$TARGET_DIR/libreta_runtime.so" reta_runtime_core_shared_words_json
verify_dynamic_symbol "$TARGET_DIR/libreta_data.so" reta_data_shared_words_json
verify_dynamic_symbol "$TARGET_DIR/libreta_parse.so" reta_parse_shell_tokens_json
verify_dynamic_symbol "$TARGET_DIR/libreta_semantics.so" reta_semantics_choice_counts_json
verify_dynamic_symbol "$TARGET_DIR/libreta_table.so" reta_table_natural_widths_json
verify_dynamic_symbol "$TARGET_DIR/libreta_render.so" reta_render_grundstruk_html
verify_component_size_diversity
verify_dynamic_symbol_absent "$TARGET_DIR/libreta_runtime.so" reta_run_and_print_from_env_ffi
verify_dynamic_symbol_absent "$TARGET_DIR/libreta_runtime.so" reta_run_argv
verify_dynamic_symbol_absent "$TARGET_DIR/libreta_runtime.so" reta_core_split_abi_anchor
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
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_input.so" retaprompt_input_autosuggestion_at_cursor_json
verify_dynamic_symbol "$TARGET_DIR/libretaprompt_input.so" retaprompt_input_free_string

verify_dynamic_needed "$TARGET_DIR/libreta.so" "${CORE_SPLIT_LIBRARIES[@]}"
verify_dynamic_needed "$TARGET_DIR/libreta_runtime.so" reta_data reta_parse reta_semantics reta_table reta_render reta_arch
verify_dynamic_needed "$TARGET_DIR/libreta_render.so" reta_semantics
verify_facade_smaller_than_runtime
verify_dynamic_needed "$TARGET_DIR/rreta" reta
verify_dynamic_needed "$TARGET_DIR/rgrundStrukHtml" reta_render
verify_dynamic_not_needed "$TARGET_DIR/rgrundStrukHtml" reta reta_data reta_parse reta_semantics reta_table reta_arch reta_runtime
verify_dynamic_not_needed "$TARGET_DIR/rreta" "${CORE_SPLIT_LIBRARIES[@]}"
verify_dynamic_needed "$TARGET_DIR/rrp"  retaprompt_input retaprompt_commands
verify_dynamic_needed "$TARGET_DIR/rrpl" retaprompt_input retaprompt_commands
verify_dynamic_needed "$TARGET_DIR/rrpe" retaprompt_input retaprompt_commands
verify_dynamic_needed "$TARGET_DIR/rrpb" retaprompt_commands
verify_dynamic_not_needed "$TARGET_DIR/rrpb" retaprompt_input

tools/guard_prompt_launcher_topology.sh "$TARGET_DIR"

cat > "$MANIFEST" <<MANIFEST_JSON
{
  "build_mode": "reta-core-facade-plus-private-core-shared-libraries-and-split-prompt-shared-libraries",
  "artifact_type": "dynamic-shared-libraries",
  "static_archives_intentionally_not_built": true,
  "rust_frontend_executables_intentionally_not_built_by_default": true,
  "core_facade": {
    "path": "$TARGET_DIR/libreta.so",
    "role": "stable public C ABI facade for rreta",
    "required_symbols": [
      "reta_run_and_print_from_env_ffi",
      "reta_core_split_abi_anchor",
      "reta_core_split_abi_manifest_json"
    ],
    "links_to": [
      "libreta_data.so",
      "libreta_parse.so",
      "libreta_semantics.so",
      "libreta_table.so",
      "libreta_render.so",
      "libreta_arch.so",
      "libreta_runtime.so"
    ]
  },
  "core_private_shared_libraries": [
    { "path": "$TARGET_DIR/libreta_data.so", "role": "data, words, aliases, CSV and catalogs" },
    { "path": "$TARGET_DIR/libreta_parse.so", "role": "argv/text parsing and input morphisms" },
    { "path": "$TARGET_DIR/libreta_semantics.so", "role": "semantic selection, topology and presheaf boundary" },
    { "path": "$TARGET_DIR/libreta_table.so", "role": "table materialization, state, views and sheaf gluing" },
    { "path": "$TARGET_DIR/libreta_render.so", "role": "rendering functors for shell/text/html/bbcode output", "links_to": ["libreta_semantics.so"] },
    { "path": "$TARGET_DIR/libreta_arch.so", "role": "category/topology/morphism/universal-property metadata" },
    { "path": "$TARGET_DIR/libreta_runtime.so", "role": "execution network, FIFO/LIFO/queue/stack/duplex/semaphore runtime and heavy Reta engine carrier" }
  ],
  "prompt_shared_libraries": [
    {
      "path": "$TARGET_DIR/libretaprompt_commands.so",
      "role": "retaPrompt command library for rrpb and the command side of rrp/rrpl/rrpe"
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
        "retaprompt_input_run_rpe_from_env",
        "retaprompt_input_autosuggestion_at_cursor_json",
        "retaprompt_input_free_string"
      ],
      "cursor_local_autosuggest": true
    }
  ],
  "forbidden_static_archives": [
    "$TARGET_DIR/libreta.a",
    "$TARGET_DIR/libreta_data.a",
    "$TARGET_DIR/libreta_parse.a",
    "$TARGET_DIR/libreta_semantics.a",
    "$TARGET_DIR/libreta_table.a",
    "$TARGET_DIR/libreta_render.a",
    "$TARGET_DIR/libreta_arch.a",
    "$TARGET_DIR/libreta_runtime.a",
    "$TARGET_DIR/libretaprompt_commands.a",
    "$TARGET_DIR/libretaprompt_input.a"
  ],
  "runtime_data": [
    "$TARGET_DIR/csv"
  ],
  "launchers": [
    {
      "path": "$TARGET_DIR/rreta",
      "links_to": ["libreta.so"],
      "must_not_link_to": [
        "libreta_data.so",
        "libreta_parse.so",
        "libreta_semantics.so",
        "libreta_table.so",
        "libreta_render.so",
        "libreta_arch.so",
        "libreta_runtime.so"
      ]
    },
    {
      "path": "$TARGET_DIR/rgrundStrukHtml",
      "links_to": ["libreta_render.so"],
      "transitive_links_to": ["libreta_semantics.so"],
      "note": "tiny C launcher; HTML rendering code lives in libreta_render.so; semantic inventories are kept behind libreta_semantics.so"
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
    "rreta remains a tiny C launcher that needs only libreta.so directly.",
    "rgrundStrukHtml is also a tiny C launcher and calls libreta_render.so directly; libreta_render.so records its semantic dependency on libreta_semantics.so.",
    "libreta.so is a thin split-facade build and forwards heavy execution to libreta_runtime.so.",
    "rrp, rrpl and rrpe intentionally carry DT_NEEDED entries for both prompt split libraries.",
    "rrpb intentionally carries only the command library dependency.",
    "Prompt launcher size and payload are guarded by tools/guard_prompt_launcher_topology.sh with RETA_PROMPT_LAUNCHER_MAX_BYTES defaulting to 262144.",
    "Prompt frontend sources are guarded by tools/guard_prompt_frontend_sources.py so public bins do not call retaprompt Rust APIs directly.",
    "The heavy non-interactive Reta engine is now outside libreta.so, primarily in libreta_runtime.so; finer distribution into data/parse/semantics/table/render can continue behind the same ABI topology."
  ]
}
MANIFEST_JSON

printf 'built dynamic split shared libraries and launchers:\n'
printf '  %s -> %s\n' "$TARGET_DIR/rreta" "libreta.so"
printf '  %s -> %s -> %s\n' "$TARGET_DIR/rgrundStrukHtml" "libreta_render.so" "libreta_semantics.so"
printf '  %s -> %s\n' "$TARGET_DIR/libreta.so" "libreta_{data,parse,semantics,table,render,arch,runtime}.so"
for library in "${CORE_SPLIT_LIBRARIES[@]}"; do
  printf '  %s\n' "$TARGET_DIR/lib${library}.so"
done
printf '  %s -> %s + %s\n' "$TARGET_DIR/rrp" "libretaprompt_input.so" "libretaprompt_commands.so"
printf '  %s -> %s + %s\n' "$TARGET_DIR/rrpl" "libretaprompt_input.so" "libretaprompt_commands.so"
printf '  %s -> %s + %s\n' "$TARGET_DIR/rrpe" "libretaprompt_input.so" "libretaprompt_commands.so"
printf '  %s -> %s\n' "$TARGET_DIR/rrpb" "libretaprompt_commands.so"
printf '\nmanifest:\n  %s\n' "$MANIFEST"
