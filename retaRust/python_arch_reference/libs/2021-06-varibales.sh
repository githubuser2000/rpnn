#!/bin/bash
cat *.py | grep -e '\s[a-zA-Z0-9]\+\s=' | grep -v '#' | sed 's/\s*\(.*\)=\s\(.*\)/\1=/g' | grep -ve '^if' -ve '^elif' -ve '^and' | sort | uniq
