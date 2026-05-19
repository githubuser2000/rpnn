#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# Compatibility entry point with a neutral name for the full .so package.
exec ./tools/package_prompt_split_sharedlibs.sh "${1:-release}"
