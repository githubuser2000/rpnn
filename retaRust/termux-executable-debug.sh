#!/usr/bin/env sh
set -eu
./build.sh debug
for a in rreta rrp rrpl rrpe rrpb
do
  "target/debug/$a" -h
done

test -x "target/debug/rgrundStrukHtml"
