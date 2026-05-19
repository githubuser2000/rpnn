#!/usr/bin/env sh
set -eu
./build.sh release
for a in rreta rrp rrpl rrpe rrpb
do
  "target/release/$a" -h
done

test -x "target/release/rgrundStrukHtml"
