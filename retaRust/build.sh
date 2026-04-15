#!/bin/bash
set -e

cargo clean
cargo build --workspace --release

for bin in rp rpl rpe rpb; do
  cc tools/launchers/$bin.c -o target/release/$bin \
    -Ltarget/release -lretaprompt_input \
    -Wl,-rpath,'$ORIGIN'
done

echo "Build complete"
