#!/usr/bin/env sh
set -eu
./build.sh release
for a in rreta rrp rrpl rrpe rrpb rgrundStrukHtml
do
  "target/release/$a" -h
done
