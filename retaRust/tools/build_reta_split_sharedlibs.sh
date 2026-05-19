#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# Compatibility entry point with a clearer name for the new architecture.
# It builds both the Reta core split and the already split retaPrompt libraries.
exec ./tools/build_prompt_split_sharedlibs.sh "${1:-release}"
