#!/bin/bash
if [ "${1}" = "debug" ]
then
  cp target/debug/{rrp,rrpl,rrpe,rrpb,rreta} ~/../usr/bin/
  cp target/debug/*.so ~/../usr/lib/
else
  cp target/release/{rrp,rrpl,rrpe,rrpb,rreta} ~/../usr/bin/
  cp target/release/*.so ~/../usr/lib/
fi
