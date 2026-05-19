#!/bin/bash
set -euo pipefail

PROFILE="${1:-release}"
case "$PROFILE" in
  debug|release) ;;
  *) echo "usage: $0 [debug|release]" >&2; exit 1 ;;
esac

TARGET_DIR="target/$PROFILE"
BIN_DIR="$HOME/../usr/bin"
LIB_DIR="$HOME/../usr/lib"

cp "$TARGET_DIR"/{rrp,rrpl,rrpe,rrpb,rreta} "$BIN_DIR/"
cp "$TARGET_DIR"/*.so "$LIB_DIR/"
