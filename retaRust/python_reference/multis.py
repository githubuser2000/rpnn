#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import os
import sys
from sys import argv

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "libs"))
from center import multiples


def mult(liste: list):
    for arg in liste:
        if type(arg) is int or arg.isdecimal():
            print(str(arg) + ": " + str(multiples(int(arg))))


def mult2(liste: list):
    ergebnis1: list = []
    ergebnis2: list = []
    for arg in liste:
        if type(arg) is int or arg.isdecimal():
            couples = [
                couple
                for couple in multiples(int(arg))
                if couple[0] != 1 and couple[1] != 1
            ]

            ergebnis1 += [str(arg) + ": " + str(couples)]
            ergebnis2 += [couples]
    return ergebnis1, ergebnis2


if __name__ == "__main__":
    mult(argv[1:])
