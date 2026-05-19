#!/usr/bin/env bash
# Shell variable documentation: RETA_SHELL_VARIABLES_DE.md and RETA_SHELL_VARIABLES_EN.md
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# Compatibility entry point with a neutral name for the full .so package.
exec ./tools/package_prompt_split_sharedlibs.sh "${1:-release}"
