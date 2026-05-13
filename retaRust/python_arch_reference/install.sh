#!/bin/bash
SCRIPT_PATH="${BASH_SOURCE:-$0}"
ABS_SCRIPT_PATH="$(realpath "${SCRIPT_PATH}")"
ABS_DIRECTORY="$(dirname "${ABS_SCRIPT_PATH}")"
python3 -m venv $ABS_DIRECTORY
pypy3 -m venv $ABS_DIRECTORY
source $ABS_DIRECTORY/bin/activate
python3 -m ensurepip
pypy3 -m ensurepip
# Es genügt wenn nur eins von beiden installiert werden würde: "pyhyphen" oder "pyphen".
# "orderedset" muss nicht installiert werden können.
# Das bedeutet, dass reta auch funktioniert, auch wenn 2 Fehler in dieser Installation eintreten.
pypy3 -m pip install bbcode==1.1.0
pypy3 -m pip install html2text==2020.1.16
pypy3 -m pip install prompt-toolkit==3.0.19
pypy3 -m pip install rich==10.12.0
pypy3 -m pip install pyphen==0.9.5
pypy3 -m pip install pyhyphen==3.0.1
pypy3 -m pip install orderedset==2.0.3
python3 -m pip install bbcode==1.1.0
python3 -m pip install html2text==2020.1.16
python3 -m pip install prompt-toolkit==3.0.19
python3 -m pip install rich==10.12.0
python3 -m pip install pyphen==0.9.5
python3 -m pip install pyhyphen==3.0.1
python3 -m pip install orderedset==2.0.3
