#!/usr/bin/env bash
set -euo pipefail

TARGET_DIR="${1:-${CARGO_TARGET_DIR:-target}/${2:-release}}"
MAX_BYTES="${RETA_PROMPT_LAUNCHER_MAX_BYTES:-262144}"

fail() {
  echo "prompt launcher topology guard failed: $*" >&2
  exit 1
}

need_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing file: $path"
}

file_size() {
  wc -c < "$1" | tr -d '[:space:]'
}

verify_max_size() {
  local path="$1"
  local size
  size="$(file_size "$path")"
  if (( size > MAX_BYTES )); then
    fail "$path is $size bytes, above RETA_PROMPT_LAUNCHER_MAX_BYTES=$MAX_BYTES. The prompt code has likely moved back into the executable. Keep the executable as a tiny C launcher and put behavior in libretaprompt_*.so."
  fi
}

needed_lines() {
  local path="$1"
  if command -v readelf >/dev/null 2>&1; then
    readelf -d "$path" 2>/dev/null | awk '/NEEDED/ {print $0}'
  fi
}

must_need() {
  local path="$1"
  shift
  if ! command -v readelf >/dev/null 2>&1; then
    echo "warning: readelf unavailable; skipping DT_NEEDED checks for $path" >&2
    return 0
  fi
  local needed library
  needed="$(needed_lines "$path")"
  for library in "$@"; do
    grep -Fq "$library" <<<"$needed" || fail "$path does not have required DT_NEEDED dependency $library; found: $needed"
  done
}

must_not_need() {
  local path="$1"
  shift
  if ! command -v readelf >/dev/null 2>&1; then
    return 0
  fi
  local needed library
  needed="$(needed_lines "$path")"
  for library in "$@"; do
    if grep -Fq "$library" <<<"$needed"; then
      fail "$path has forbidden DT_NEEDED dependency $library; found: $needed"
    fi
  done
}


verify_help_smoke() {
  local path="$1"
  local output
  local ld_path="$TARGET_DIR:$TARGET_DIR/deps:${LD_LIBRARY_PATH:-}"
  if ! output="$(LD_LIBRARY_PATH="$ld_path" DYLD_LIBRARY_PATH="$ld_path" "$path" -h 2>&1)"; then
    fail "$path -h failed. Output: $output"
  fi
  if ! grep -Fq "Prompt-Schicht" <<<"$output"; then
    fail "$path -h did not print prompt help. This usually means a stale libretaprompt_commands.so/libretaprompt_input.so is being used. Output: $output"
  fi
}

verify_no_rust_payload() {
  local path="$1"

  if command -v nm >/dev/null 2>&1; then
    if nm -a "$path" 2>/dev/null | grep -Eq '(^|[[:space:]])(__rust_|rust_begin_unwind|rust_eh_personality|_RNv|_ZN.*(core|std|alloc))'; then
      fail "$path contains Rust runtime/panic symbols. Public prompt executables must be C launchers; Rust prompt code belongs in libretaprompt_input.so/libretaprompt_commands.so."
    fi
  fi

  if command -v strings >/dev/null 2>&1; then
    if strings -a "$path" 2>/dev/null | grep -Eq '(rustc|core::panicking|std::panicking|library/std/src|library/core/src|__rust_)'; then
      fail "$path contains Rust payload strings. Public prompt executables must stay C launchers."
    fi
  fi
}

for exe in rrp rrpl rrpe rrpb; do
  need_file "$TARGET_DIR/$exe"
  verify_max_size "$TARGET_DIR/$exe"
  verify_no_rust_payload "$TARGET_DIR/$exe"
done

must_need "$TARGET_DIR/rrp"  libretaprompt_input.so libretaprompt_commands.so
must_need "$TARGET_DIR/rrpl" libretaprompt_input.so libretaprompt_commands.so
must_need "$TARGET_DIR/rrpe" libretaprompt_input.so libretaprompt_commands.so
must_need "$TARGET_DIR/rrpb" libretaprompt_commands.so

must_not_need "$TARGET_DIR/rrpb" libretaprompt_input.so

for exe in rrp rrpl rrpe rrpb; do
  verify_help_smoke "$TARGET_DIR/$exe"
done

# The prompt launchers must not pull the non-interactive Reta core directly.
for exe in rrp rrpl rrpe rrpb; do
  must_not_need "$TARGET_DIR/$exe" \
    libreta.so \
    libreta_data.so \
    libreta_parse.so \
    libreta_semantics.so \
    libreta_table.so \
    libreta_render.so \
    libreta_arch.so \
    libreta_runtime.so

done

printf 'prompt launcher topology guard passed for %s; max size %s bytes\n' "$TARGET_DIR" "$MAX_BYTES"
