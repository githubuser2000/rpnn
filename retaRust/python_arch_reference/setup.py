#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import os
from distutils.cmd import Command
from distutils.command.build_clib import build_clib
from distutils.command.build_ext import build_ext
from distutils.command.build_py import build_py
from distutils.command.build_scripts import build_scripts

from setuptools import find_packages, setup


class Build(build_py):
    def run(self):
        build_py.run(self)
        self.run_command("build_mo")

    def has_ext_modules(self):
        return True


class BuildExt(build_ext):
    def run(self):
        build_ext.run(self)
        self.run_command("build_mo")


class BuildClib(build_clib):
    def run(self):
        build_clib.run(self)
        self.run_command("build_mo")


class BuildScripts(build_scripts):
    def run(self):
        build_scripts.run(self)
        self.run_command("build_mo")


cmdclass = {
    "build": Build,
    "build_ext": BuildExt,
    "build_clib": BuildClib,
    "build_scripts": BuildScripts,
}


class ExtractMessages(Command):
    """
    A custom command to extract messages to a POT file using the built-in gettext library.
    """

    description = "Extract messages to a POT file"
    user_options = []

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        # Find all Python source files in the current directory and its subdirectories
        py_files = []
        print("ALX")
        print(os.walk("."))
        for root, dirs, files in os.walk("i18n"):
            for file in files:
                if file.endswith(".py"):
                    py_files.append(os.path.join(root, file))
                    print(os.path.join(root, file))

        # Extract messages to a POT file
        os.system(
            f'xgettext --language=Python --output=i18n/messages.pot {" ".join(py_files)}'
        )


setup(
    name="reta",
    version="3.20250507.4591",
    # Version 3 ab retaPrompt, Version 20230901 ist Jahr Monat Tag, Version 4492 ist die Nummer der Anzahlen der Commits des Repos nachdem das alte Repo kaputt gegangen war
    # bis Ende 2023 gab es keine Versionsnummern: Version 2 bedeutet, nun html Datei dazu, Version 3 bedeutet, nun retaPrompt dazu. Es beginnt nun also mit Version 3.
    # So kann jeder nachträglich zu jedem reta selbst eine Versionsnummer bestimmen.
    description="Religions-Tabelle",
    author="Jupiter 3.0 alias trace",
    packages=find_packages(),
    install_requires=[
        "html2text==2020.1.16",
        "bbcode==1.1.0",
        "pyphen==0.9.5",
        "PyHyphen==3.0.1",
        "prompt_toolkit==3.0.19",
        # "orderedset==2.0.3",
        "rich==10.12.0",
        # "numpy"
    ],
    package_data={
        ".": [
            "*.txt",
            "*.csv",
        ],
        "reta": [
            "i18n/*.po",
            "i18n/*.mo",  # How to compile on the fly?
        ],
    },
    cmdclass={
        "extract_messages": ExtractMessages,
    },
    # input_dirs=".",
    # output_file="i18n/locale.pot",
)
