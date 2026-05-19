#!/usr/bin/env sh
set -eu
./build.sh debug
for a in rreta rrp rrpl rrpe rrpb rgrundStrukHtml
do
  "target/debug/$a" -h
done
