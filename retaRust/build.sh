#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

PROFILE="${1:-release}"
case "$PROFILE" in
  debug)
    CARGO_FLAGS=()
    TARGET_DIR="${CARGO_TARGET_DIR:-target}/debug"
    ;;
  release)
    CARGO_FLAGS=(--release)
    TARGET_DIR="${CARGO_TARGET_DIR:-target}/release"
    ;;
  *)
    echo "usage: $0 [debug|release]" >&2
    exit 1
    ;;
esac

CORE_SPLIT_PACKAGES=(
  reta_data
  reta_parse
  reta_semantics
  reta_table
  reta_render
  reta_arch
  reta_runtime
)

CORE_SPLIT_LIBRARIES=(
  reta_data
  reta_parse
  reta_semantics
  reta_table
  reta_render
  reta_arch
  reta_runtime
)

PROMPT_SPLIT_PACKAGES=(
  retaprompt_commands
  retaprompt_input
)

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
      echo "launcher/library dependency check failed: $output does not need lib${library}.so" >&2
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
      echo "launcher/library dependency check failed: $output unexpectedly needs lib${library}.so" >&2
      echo "$needed" >&2
      exit 1
    fi
  done
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
  fi
}

verify_facade_smaller_than_runtime() {
  local facade="$TARGET_DIR/libreta.so"
  local runtime="$TARGET_DIR/libreta_runtime.so"

  if [[ ! -f "$facade" || ! -f "$runtime" ]]; then
    return 0
  fi

  local facade_size runtime_size
  facade_size=$(wc -c < "$facade")
  runtime_size=$(wc -c < "$runtime")

  if (( facade_size >= runtime_size )); then
    echo "split-size check failed: libreta.so ($facade_size bytes) should be a thin facade smaller than libreta_runtime.so ($runtime_size bytes)" >&2
    echo "This usually means the facade was built without --features split-facade or the heavy engine moved back into libreta.so." >&2
    exit 1
  fi
}

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

copy_runtime_data() {
  if [[ -d csv ]]; then
    rm -rf "$TARGET_DIR/csv"
    cp -R csv "$TARGET_DIR/csv"
  fi
}

# First build the private Reta core split libraries.  The second cargo call
# links libreta.so against these already existing .so files via build.rs and
# RETA_LINK_CORE_SPLIT_LIBS=1.  This keeps rreta small:
#   rreta -> libreta.so -> libreta_{data,parse,semantics,table,render,arch,runtime}.so
CORE_BUILD_ARGS=()
for package in "${CORE_SPLIT_PACKAGES[@]}"; do
  CORE_BUILD_ARGS+=(-p "$package")
done

cargo build "${CORE_BUILD_ARGS[@]}" "${CARGO_FLAGS[@]}"

# Build only library targets by default.  This keeps Rust application code out
# of the executables: rreta/rrp/rrpl/rrpe/rrpb are linked below as tiny C
# launchers.  Set RETA_BUILD_RUST_TOOL_BINS=1 only when the diagnostic Rust
# binaries are explicitly needed.
RETA_LINK_CORE_SPLIT_LIBS=1 cargo build -p reta --lib --features split-facade "${CARGO_FLAGS[@]}"
cargo build -p reta_architecture --lib "${CARGO_FLAGS[@]}"
cargo build -p retaprompt_commands --lib "${CARGO_FLAGS[@]}"
cargo build -p retaprompt_input --lib "${CARGO_FLAGS[@]}"

if [[ "${RETA_BUILD_RUST_TOOL_BINS:-0}" == "1" ]]; then
  # Diagnostic Rust bins need the full in-crate Rust API.  Build them, then
  # rebuild the public cdylib with split-facade so target/libreta.so is not
  # overwritten by the heavy compatibility build.
  cargo build \
    -p reta \
    --bins \
    "${CARGO_FLAGS[@]}"
  RETA_LINK_CORE_SPLIT_LIBS=1 cargo build -p reta --lib --features split-facade "${CARGO_FLAGS[@]}"
fi

if [[ "${RETA_BUILD_RUST_FRONTEND_BINS:-0}" == "1" ]]; then
  cargo build \
    -p retaprompt_frontends \
    --features retaprompt_frontends/rust-frontends \
    "${CARGO_FLAGS[@]}"
fi

mkdir -p "$TARGET_DIR"

link_launcher tools/launchers/reta.c "$TARGET_DIR/rreta" reta

# rrpb is command-only.  The interactive prompt launchers need both split
# prompt libraries: libretaprompt_input.so for line input/autocomplete/
# autosuggest, and libretaprompt_commands.so for command parsing/execution.
link_launcher tools/launchers/rp.c  "$TARGET_DIR/rrp"  retaprompt_input retaprompt_commands
link_launcher tools/launchers/rpl.c "$TARGET_DIR/rrpl" retaprompt_input retaprompt_commands
link_launcher tools/launchers/rpe.c "$TARGET_DIR/rrpe" retaprompt_input retaprompt_commands
link_launcher tools/launchers/rpb.c "$TARGET_DIR/rrpb" retaprompt_commands

verify_needed "$TARGET_DIR/libreta.so" "${CORE_SPLIT_LIBRARIES[@]}"
verify_facade_smaller_than_runtime
verify_dynamic_symbol_absent "$TARGET_DIR/libreta_runtime.so" reta_run_and_print_from_env_ffi
verify_dynamic_symbol_absent "$TARGET_DIR/libreta_runtime.so" reta_run_argv
verify_dynamic_symbol_absent "$TARGET_DIR/libreta_runtime.so" reta_core_split_abi_anchor
verify_needed "$TARGET_DIR/rreta" reta
verify_not_needed "$TARGET_DIR/rreta" "${CORE_SPLIT_LIBRARIES[@]}"
verify_needed "$TARGET_DIR/rrp"  retaprompt_input retaprompt_commands
verify_needed "$TARGET_DIR/rrpl" retaprompt_input retaprompt_commands
verify_needed "$TARGET_DIR/rrpe" retaprompt_input retaprompt_commands
verify_needed "$TARGET_DIR/rrpb" retaprompt_commands
verify_not_needed "$TARGET_DIR/rrpb" retaprompt_input

copy_runtime_data
cat <<BUILD_COMPLETE
Build complete: $TARGET_DIR
  rreta -> libreta.so
  libreta.so -> libreta_data.so + libreta_parse.so + libreta_semantics.so + libreta_table.so + libreta_render.so + libreta_arch.so + libreta_runtime.so
  libreta_runtime.so carries the heavy Reta engine; libreta.so remains a thin ABI facade
  rrp  -> libretaprompt_input.so + libretaprompt_commands.so
  rrpl -> libretaprompt_input.so + libretaprompt_commands.so
  rrpe -> libretaprompt_input.so + libretaprompt_commands.so
  rrpb -> libretaprompt_commands.so
BUILD_COMPLETE
