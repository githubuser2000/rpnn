#!/usr/bin/env sh
set -eu
cd "$(dirname "$0")/.."
python3 tools/run_architecture_diagnostics.py --pretty "$@"
