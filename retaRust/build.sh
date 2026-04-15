#!/bin/bash
set -e

cargo clean
cargo build --release

for bin in rp rpl rpe rpb; do
  cc tools/launchers/$bin.c -o target/debug/$bin \
    -Ltarget/debug -lretaprompt_input \
    -Wl,-rpath,'$ORIGIN'
done

echo "Build complete"
