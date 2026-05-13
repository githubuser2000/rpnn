#!/bin/bash
SCRIPT_PATH="${BASH_SOURCE:-$0}"
ABS_SCRIPT_PATH="$(realpath "${SCRIPT_PATH}")"
#ABS_DIRECTORY="$(dirname "${ABS_SCRIPT_PATH}")"
ABS_DIRECTORY="/home/alex/myRepos/reta"
GIT_DIRECTORY="${ABS_DIRECTORY}/.git"
function ctrl_c() {
    cat ${ABS_DIRECTORY}/head1.alx
    cat ${ABS_DIRECTORY}/muell/religionen.before23-0429.js | sed 's/Grundstrukturen/basic_structures/g'
    cat ${ABS_DIRECTORY}/head2.alx
    cat ~/middle.alx
    pypy3 ${ABS_DIRECTORY}/grundStrukHtml.py blank -language=english
    cat ${ABS_DIRECTORY}/footer.alx
}
trap ctrl_c INT
if [ "$1" == 'reta' ]; then
	pypy3 ${ABS_DIRECTORY}/reta -columns --all --width=0 -output --type=html --onetable --nocolor -language=english >  ~/middle.alx
fi
#chown -R alex:alex ${ABS_DIRECTORY}
ctrl_c
