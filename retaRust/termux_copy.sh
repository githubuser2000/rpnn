#!/bin/bash
if [ "${1}" = "debug" ]
then
  cp target/debug/rreta ~/../usr/bin/
  cp target/debug/rrp* ~/../usr/bin/
  cp target/debug/*.so ~/../usr/lib/
else
  cp target/release/rreta ~/../usr/bin/
  cp target/release/rrp* ~/../usr/bin/
  cp target/release/*.so ~/../usr/lib/
fi
