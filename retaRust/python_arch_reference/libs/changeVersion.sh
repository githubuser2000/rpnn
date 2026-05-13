#!/bin/bash
version=`~/Eigene-Dateien/myRepos/reta/libs/version.sh`
#echo $version
sed -i 's/\(version\s*=\s*"\)[^"]*"/\1'${version}'"/g' ~/Eigene-Dateien/myRepos/reta/{setup.py,pyproject.toml} > /dev/null 2> /dev/null
