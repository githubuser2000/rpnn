#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import os
import sys
from sys import argv

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "libs"))
from center import multiples


def mult3(liste: list):
    o2: set
    for arg in liste:
        if type(arg) is int or arg.isdecimal():
            m3: set = set()
            o3: set = set()
            for m in multiples(int(arg)):
                if m[0] > m[1]:
                    o, n = m[0], m[1]
                else:
                    n, o = m[0], m[1]
                o2 = {tuple(m1) for m1 in multiples(int(o))}
                for a, b in o2:
                    v = [n, a, b]
                    v.sort()
                    o3 |= {tuple(v)}
                o6 = [o5 for o5 in set(o3) if 1 not in o5]
                m3 |= set(o6)
            return arg, m3
            # print(str(arg) + ": " + str(list(m3)))


if __name__ == "__main__":
    mult3(argv[1:])
